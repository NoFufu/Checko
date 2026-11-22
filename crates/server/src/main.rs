//! Checko sync server.
//!
//! - `POST /api/register` / `POST /api/login` -> `{ token }`
//! - `POST /api/sync` (Bearer token): push local changes, get everything newer
//!   than the client's `since` cursor. Conflicts: last write (by `updated_at`) wins.
//! - `GET /health` for uptime pings.
//!
//! Configuration via environment:
//! - `DATABASE_URL` (Postgres, e.g. Neon)
//! - `PORT` (default 8080)
//! - `ALLOWED_ORIGINS` comma-separated (default: GitHub Pages + localhost)
//! - `ALLOW_REGISTRATION` = "false" to close sign-ups after creating your account

use std::{net::SocketAddr, sync::Arc};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use tower_http::cors::{AllowOrigin, CorsLayer};
use uuid::Uuid;

const KINDS: [&str; 5] = ["task", "note", "shopping", "event", "profile"];
const MAX_CHANGES: usize = 5000;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    allow_registration: bool,
}

type Shared = Arc<AppState>;

struct ApiError(StatusCode, &'static str);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(serde_json::json!({ "error": self.1 }))).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("database error: {e}");
        ApiError(StatusCode::INTERNAL_SERVER_ERROR, "Datenbankfehler")
    }
}

type ApiResult<T> = Result<Json<T>, ApiError>;

async fn migrate(db: &PgPool) -> Result<(), sqlx::Error> {
    for stmt in [
        "CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now())",
        "CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now())",
        "CREATE SEQUENCE IF NOT EXISTS record_seq",
        "CREATE TABLE IF NOT EXISTS records (
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            kind TEXT NOT NULL,
            id UUID NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            data JSONB,
            seq BIGINT NOT NULL DEFAULT nextval('record_seq'),
            PRIMARY KEY (user_id, kind, id))",
        "CREATE INDEX IF NOT EXISTS records_user_seq ON records (user_id, seq)",
    ] {
        sqlx::query(stmt).execute(db).await?;
    }
    Ok(())
}

// ---------- auth ----------

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
    username: String,
}

fn new_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn clean_username(u: &str) -> Result<String, ApiError> {
    let u = u.trim().to_lowercase();
    if u.len() < 3 || u.len() > 40 || !u.chars().all(|c| c.is_alphanumeric() || "._-".contains(c)) {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "Benutzername: 3–40 Zeichen, nur Buchstaben, Zahlen, . _ -",
        ));
    }
    Ok(u)
}

async fn create_session(db: &PgPool, user_id: Uuid) -> Result<String, ApiError> {
    let token = new_token();
    sqlx::query("INSERT INTO sessions (token, user_id) VALUES ($1, $2)")
        .bind(&token)
        .bind(user_id)
        .execute(db)
        .await?;
    Ok(token)
}

async fn register(State(s): State<Shared>, Json(c): Json<Credentials>) -> ApiResult<TokenResponse> {
    if !s.allow_registration {
        return Err(ApiError(
            StatusCode::FORBIDDEN,
            "Registrierung ist geschlossen",
        ));
    }
    let username = clean_username(&c.username)?;
    if c.password.chars().count() < 8 {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "Passwort: mindestens 8 Zeichen",
        ));
    }
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(c.password.as_bytes(), &salt)
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Speichern"))?
        .to_string();
    let id = Uuid::now_v7();
    let inserted = sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3) ON CONFLICT (username) DO NOTHING")
        .bind(id)
        .bind(&username)
        .bind(&hash)
        .execute(&s.db)
        .await?;
    if inserted.rows_affected() == 0 {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "Benutzername ist schon vergeben",
        ));
    }
    let token = create_session(&s.db, id).await?;
    Ok(Json(TokenResponse { token, username }))
}

async fn login(State(s): State<Shared>, Json(c): Json<Credentials>) -> ApiResult<TokenResponse> {
    let username = c.username.trim().to_lowercase();
    let row = sqlx::query("SELECT id, password_hash FROM users WHERE username = $1")
        .bind(&username)
        .fetch_optional(&s.db)
        .await?;
    let wrong = ApiError(
        StatusCode::UNAUTHORIZED,
        "Benutzername oder Passwort falsch",
    );
    let Some(row) = row else { return Err(wrong) };
    let id: Uuid = row.get("id");
    let hash: String = row.get("password_hash");
    let parsed = PasswordHash::new(&hash)
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "Fehler"))?;
    if Argon2::default()
        .verify_password(c.password.as_bytes(), &parsed)
        .is_err()
    {
        return Err(wrong);
    }
    let token = create_session(&s.db, id).await?;
    Ok(Json(TokenResponse { token, username }))
}

async fn user_from(headers: &HeaderMap, db: &PgPool) -> Result<Uuid, ApiError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError(StatusCode::UNAUTHORIZED, "Nicht angemeldet"))?;
    let row = sqlx::query("SELECT user_id FROM sessions WHERE token = $1")
        .bind(token)
        .fetch_optional(db)
        .await?;
    row.map(|r| r.get("user_id")).ok_or(ApiError(
        StatusCode::UNAUTHORIZED,
        "Sitzung abgelaufen, bitte neu anmelden",
    ))
}

async fn logout(State(s): State<Shared>, headers: HeaderMap) -> Result<StatusCode, ApiError> {
    if let Some(token) = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
    {
        sqlx::query("DELETE FROM sessions WHERE token = $1")
            .bind(token)
            .execute(&s.db)
            .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---------- sync ----------

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Change {
    kind: String,
    id: Uuid,
    updated_at: DateTime<Utc>,
    /// `null` means the record was removed for good.
    data: Option<Value>,
}

#[derive(Deserialize)]
struct SyncRequest {
    /// Server cursor from the last sync (0 = everything).
    #[serde(default)]
    since: i64,
    #[serde(default)]
    changes: Vec<Change>,
}

#[derive(Serialize)]
struct SyncResponse {
    cursor: i64,
    changes: Vec<Change>,
}

async fn sync(
    State(s): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<SyncRequest>,
) -> ApiResult<SyncResponse> {
    let user = user_from(&headers, &s.db).await?;
    if req.changes.len() > MAX_CHANGES {
        return Err(ApiError(
            StatusCode::PAYLOAD_TOO_LARGE,
            "Zu viele Änderungen auf einmal",
        ));
    }
    let mut tx = s.db.begin().await?;
    for c in req
        .changes
        .iter()
        .filter(|c| KINDS.contains(&c.kind.as_str()))
    {
        // Last write wins: only replace when the incoming change is newer.
        sqlx::query(
            "INSERT INTO records (user_id, kind, id, updated_at, data) VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (user_id, kind, id) DO UPDATE
               SET updated_at = EXCLUDED.updated_at, data = EXCLUDED.data, seq = nextval('record_seq')
               WHERE records.updated_at < EXCLUDED.updated_at",
        )
        .bind(user)
        .bind(&c.kind)
        .bind(c.id)
        .bind(c.updated_at)
        .bind(&c.data)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    let rows = sqlx::query(
        "SELECT kind, id, updated_at, data, seq FROM records WHERE user_id = $1 AND seq > $2 ORDER BY seq",
    )
    .bind(user)
    .bind(req.since)
    .fetch_all(&s.db)
    .await?;
    let mut cursor = req.since;
    let changes = rows
        .into_iter()
        .map(|r| {
            cursor = cursor.max(r.get::<i64, _>("seq"));
            Change {
                kind: r.get("kind"),
                id: r.get("id"),
                updated_at: r.get("updated_at"),
                data: r.get("data"),
            }
        })
        .collect();
    Ok(Json(SyncResponse { cursor, changes }))
}

// ---------- widget ----------

#[derive(Deserialize)]
struct WidgetQuery {
    /// Minutes east of UTC on the device (e.g. 60 in winter in Germany).
    #[serde(default)]
    tz: i64,
}

#[derive(Serialize)]
struct WidgetItem {
    title: String,
    when: String,
    color: String,
    urgent: bool,
}

#[derive(Serialize)]
struct WidgetResponse {
    date: String,
    today: Vec<WidgetItem>,
    open_today: usize,
    next: Vec<WidgetItem>,
    streak: u32,
    level: u32,
}

fn records<T: serde::de::DeserializeOwned>(rows: &[(String, Value)], kind: &str) -> Vec<T> {
    rows.iter()
        .filter(|(k, _)| k == kind)
        .filter_map(|(_, v)| serde_json::from_value(v.clone()).ok())
        .collect()
}

fn hhmm(t: Option<chrono::NaiveTime>) -> String {
    t.map(|t| t.format("%H:%M").to_string()).unwrap_or_default()
}

/// Small summary for a home-screen widget (Scriptable on iPhone).
async fn widget(
    State(s): State<Shared>,
    headers: HeaderMap,
    axum::extract::Query(q): axum::extract::Query<WidgetQuery>,
) -> ApiResult<WidgetResponse> {
    use checko_core::{items_between, level_for_xp, today_tasks, Event, ItemKind, Profile, Task};
    let user = user_from(&headers, &s.db).await?;
    let rows: Vec<(String, Value)> = sqlx::query(
        "SELECT kind, data FROM records WHERE user_id = $1 AND data IS NOT NULL AND kind IN ('task','event','profile')",
    )
    .bind(user)
    .fetch_all(&s.db)
    .await?
    .into_iter()
    .map(|r| (r.get("kind"), r.get("data")))
    .collect();
    let tasks: Vec<Task> = records(&rows, "task");
    let events: Vec<Event> = records(&rows, "event");
    let profile: Profile = records(&rows, "profile")
        .into_iter()
        .next()
        .unwrap_or_default();

    let local_now =
        Utc::now().naive_utc() + chrono::Duration::minutes(q.tz.clamp(-14 * 60, 14 * 60));
    let today = local_now.date();
    let color_of = |c: Option<&str>| match c {
        Some("Arbeit") => "#6d4aff",
        Some("Privat") => "#16a34a",
        Some("Gesundheit") => "#f97316",
        Some(_) => "#3b82f6",
        None => "#22c55e",
    };

    let due = today_tasks(&tasks, None, today);
    let open_today = due.len();
    let today_items = due
        .into_iter()
        .take(6)
        .map(|t| WidgetItem {
            title: t.title.clone(),
            when: hhmm(t.due_time),
            color: color_of(t.area.as_deref()).into(),
            urgent: t.priority == checko_core::Priority::P1,
        })
        .collect();
    let next = items_between(&events, &tasks, today, today + chrono::Duration::days(30))
        .into_iter()
        .filter(|i| !i.done && (i.kind == ItemKind::Event || i.start.is_some()))
        .filter(|i| i.date > today || i.start.is_none_or(|t| t >= local_now.time()))
        // tasks due today are already listed under "Heute"
        .filter(|i| !(i.kind == ItemKind::Task && i.date <= today))
        .take(3)
        .map(|i| {
            let days = (i.date - today).num_days();
            let day = match days {
                0 => "Heute".to_string(),
                1 => "Morgen".to_string(),
                n if n < 7 => ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"]
                    [chrono::Datelike::weekday(&i.date).num_days_from_monday() as usize]
                    .to_string(),
                _ => i.date.format("%d.%m.").to_string(),
            };
            let time = hhmm(i.start);
            WidgetItem {
                title: i.title,
                when: if time.is_empty() {
                    day
                } else {
                    format!("{day} {time}")
                },
                color: color_of(i.category.as_deref()).into(),
                urgent: false,
            }
        })
        .collect();
    let weekday = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"]
        [chrono::Datelike::weekday(&today).num_days_from_monday() as usize];
    Ok(Json(WidgetResponse {
        date: format!("{weekday}, {}", today.format("%d.%m.")),
        today: today_items,
        open_today,
        next,
        streak: profile.current_streak(today),
        level: level_for_xp(profile.xp).level,
    }))
}

async fn health(State(s): State<Shared>) -> Result<&'static str, ApiError> {
    sqlx::query("SELECT 1").execute(&s.db).await?;
    Ok("ok")
}

fn cors() -> CorsLayer {
    let origins = std::env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| {
        "https://nofufu.github.io,http://127.0.0.1:8080,http://localhost:8080".into()
    });
    let list: Vec<HeaderValue> = origins
        .split(',')
        .filter_map(|o| HeaderValue::from_str(o.trim()).ok())
        .collect();
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(list))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
}

fn app(state: Shared) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/sync", post(sync))
        .route("/api/widget", get(widget))
        .layer(cors())
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL fehlt");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Datenbank nicht erreichbar");
    migrate(&db)
        .await
        .expect("Tabellen konnten nicht angelegt werden");
    let allow_registration = std::env::var("ALLOW_REGISTRATION")
        .map(|v| v != "false")
        .unwrap_or(true);
    let state = Arc::new(AppState {
        db,
        allow_registration,
    });

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Port belegt");
    println!(
        "Checko-Server läuft auf {addr} (Registrierung {})",
        if allow_registration {
            "offen"
        } else {
            "geschlossen"
        }
    );
    axum::serve(listener, app(state))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .expect("Server-Fehler");
}

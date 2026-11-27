//! Reminders via Web Push, so they arrive even when Checko is closed.
//!
//! - The VAPID key pair is generated on first start and kept in the database
//!   (table `settings`), so there is nothing to configure on Render.
//! - `GET /api/push/key` → public key (base64url) for `pushManager.subscribe`.
//! - `POST /api/push/subscribe` (Bearer) → store the browser's subscription + time zone.
//! - A background loop checks every minute which reminders are due
//!   (`checko_core::reminders_between`) and sends each one once (`push_sent`).

use std::{sync::Arc, time::Duration};

use axum::{extract::State, http::HeaderMap, http::StatusCode, Json};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD as B64, Engine as _};
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use web_push_native::{
    jwt_simple::{algorithms::ES256KeyPair, prelude::ECDSAP256PublicKeyLike},
    p256::PublicKey,
    Auth, WebPushBuilder,
};

use crate::{records, user_from, ApiError, Shared};

/// Shown to push services as the sender (no personal data).
const CONTACT: &str = "https://nofufu.github.io/Checko/";

pub struct Vapid {
    pub keys: ES256KeyPair,
    pub public_b64: String,
}

pub async fn migrate(db: &PgPool) -> Result<(), sqlx::Error> {
    for stmt in [
        "CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
        "CREATE TABLE IF NOT EXISTS push_subs (
            endpoint TEXT PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            p256dh TEXT NOT NULL,
            auth TEXT NOT NULL,
            tz INT NOT NULL DEFAULT 0,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now())",
        "CREATE TABLE IF NOT EXISTS push_sent (
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            key TEXT NOT NULL,
            sent_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            PRIMARY KEY (user_id, key))",
    ] {
        sqlx::query(stmt).execute(db).await?;
    }
    Ok(())
}

/// Load the VAPID key pair, or create and store one on first start.
pub async fn vapid(db: &PgPool) -> Result<Vapid, sqlx::Error> {
    let stored: Option<String> =
        sqlx::query("SELECT value FROM settings WHERE key = 'vapid_private'")
            .fetch_optional(db)
            .await?
            .map(|r| r.get("value"));
    let keys = match stored
        .and_then(|v| B64.decode(v).ok())
        .and_then(|b| ES256KeyPair::from_bytes(&b).ok())
    {
        Some(k) => k,
        None => {
            let k = ES256KeyPair::generate();
            sqlx::query("INSERT INTO settings (key, value) VALUES ('vapid_private', $1) ON CONFLICT (key) DO UPDATE SET value = $1")
                .bind(B64.encode(k.to_bytes()))
                .execute(db)
                .await?;
            k
        }
    };
    let public_b64 = B64.encode(keys.public_key().public_key().to_bytes_uncompressed());
    Ok(Vapid { keys, public_b64 })
}

pub async fn key(State(s): State<Shared>) -> String {
    s.vapid.public_b64.clone()
}

#[derive(Deserialize)]
pub struct SubscribeRequest {
    subscription: Subscription,
    #[serde(default)]
    tz: i32,
}

#[derive(Deserialize)]
struct Subscription {
    endpoint: String,
    keys: SubKeys,
}

#[derive(Deserialize)]
struct SubKeys {
    p256dh: String,
    auth: String,
}

pub async fn subscribe(
    State(s): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<SubscribeRequest>,
) -> Result<StatusCode, ApiError> {
    let user = user_from(&headers, &s.db).await?;
    if !req.subscription.endpoint.starts_with("https://") {
        return Err(ApiError(StatusCode::BAD_REQUEST, "Ungültige Push-Adresse"));
    }
    sqlx::query(
        "INSERT INTO push_subs (endpoint, user_id, p256dh, auth, tz) VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (endpoint) DO UPDATE SET user_id = $2, p256dh = $3, auth = $4, tz = $5",
    )
    .bind(&req.subscription.endpoint)
    .bind(user)
    .bind(&req.subscription.keys.p256dh)
    .bind(&req.subscription.keys.auth)
    .bind(req.tz.clamp(-14 * 60, 14 * 60))
    .execute(&s.db)
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// "Heute · 14:00", "Morgen", "28.11. · 09:00"
fn when(date: NaiveDate, today: NaiveDate, start: Option<chrono::NaiveTime>) -> String {
    let day = match (date - today).num_days() {
        0 => "Heute".to_string(),
        1 => "Morgen".to_string(),
        _ => date.format("%d.%m.").to_string(),
    };
    match start {
        Some(t) => format!("{day} · {}", t.format("%H:%M")),
        None => day,
    }
}

/// Runs forever: every minute send the reminders that became due.
pub fn spawn_scheduler(state: Shared) {
    tokio::spawn(async move {
        let http = reqwest::Client::new();
        let mut tick = tokio::time::interval(Duration::from_secs(60));
        let mut rounds: u64 = 0;
        // Render gives the service its own public address. Calling it every
        // ~10 minutes counts as a visit, so the free server does not fall asleep
        // (asleep it could not send any reminders).
        let own_url = std::env::var("RENDER_EXTERNAL_URL").ok();
        loop {
            tick.tick().await;
            if let (Some(url), true) = (&own_url, rounds.is_multiple_of(10)) {
                let _ = http
                    .get(format!("{}/health", url.trim_end_matches('/')))
                    .timeout(Duration::from_secs(30))
                    .send()
                    .await;
            }
            if let Err(e) = send_due(&state, &http).await {
                eprintln!("push: {e}");
            }
            rounds += 1;
            if rounds.is_multiple_of(1440) {
                let _ =
                    sqlx::query("DELETE FROM push_sent WHERE sent_at < now() - interval '30 days'")
                        .execute(&state.db)
                        .await;
            }
        }
    });
}

/// `POST /api/push/test` (Bearer): send a test reminder through the push
/// service right now and report what each browser's push service answered.
pub async fn test(State(s): State<Shared>, headers: HeaderMap) -> Result<String, ApiError> {
    let user = user_from(&headers, &s.db).await?;
    let subs = sqlx::query("SELECT endpoint, p256dh, auth FROM push_subs WHERE user_id = $1")
        .bind(user)
        .fetch_all(&s.db)
        .await?;
    if subs.is_empty() {
        return Ok("Kein Gerät angemeldet – erst „Erinnerungen aktivieren“ tippen.".into());
    }
    let http = reqwest::Client::new();
    let payload = serde_json::json!({
        "title": "Checko",
        "body": "Test vom Server – Erinnerungen kommen auch bei geschlossener App. 🦎",
        "tag": "checko-server-test",
    })
    .to_string();
    let mut lines = Vec::new();
    for sub in &subs {
        let endpoint: String = sub.get("endpoint");
        let host = endpoint.split('/').nth(2).unwrap_or("?").to_string();
        let r = send_one(
            &s,
            &http,
            &endpoint,
            sub.get("p256dh"),
            sub.get("auth"),
            &payload,
        )
        .await;
        lines.push(match r {
            Ok(201) | Ok(200) | Ok(202) => format!("✓ {host}: gesendet"),
            Ok(404) | Ok(410) => {
                let _ = sqlx::query("DELETE FROM push_subs WHERE endpoint = $1")
                    .bind(&endpoint)
                    .execute(&s.db)
                    .await;
                format!("✗ {host}: Anmeldung abgelaufen – bitte neu aktivieren")
            }
            Ok(code) => format!("✗ {host}: Push-Dienst antwortet {code}"),
            Err(e) => format!("✗ {host}: {e}"),
        });
    }
    Ok(lines.join("\n"))
}

async fn send_due(
    state: &Arc<crate::AppState>,
    http: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use checko_core::{reminders_between, Event, Task};
    let users = sqlx::query("SELECT user_id, max(tz) AS tz FROM push_subs GROUP BY user_id")
        .fetch_all(&state.db)
        .await?;
    for u in users {
        let user: Uuid = u.get("user_id");
        let tz: i32 = u.get("tz");
        let now = (Utc::now() + chrono::Duration::minutes(tz as i64)).naive_utc();
        // 60 minutes back: nothing is lost if the free server was briefly asleep;
        // push_sent makes sure every reminder goes out only once.
        let after = now - chrono::Duration::minutes(60);
        let rows: Vec<(String, Value)> = sqlx::query(
            "SELECT kind, data FROM records WHERE user_id = $1 AND data IS NOT NULL AND kind IN ('task','event')",
        )
        .bind(user)
        .fetch_all(&state.db)
        .await?
        .into_iter()
        .map(|r| (r.get("kind"), r.get("data")))
        .collect();
        let tasks: Vec<Task> = records(&rows, "task");
        let events: Vec<Event> = records(&rows, "event");
        let due = reminders_between(&events, &tasks, after, now);
        if due.is_empty() {
            continue;
        }
        let subs = sqlx::query("SELECT endpoint, p256dh, auth FROM push_subs WHERE user_id = $1")
            .bind(user)
            .fetch_all(&state.db)
            .await?;
        for r in due {
            let fresh = sqlx::query(
                "INSERT INTO push_sent (user_id, key) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(user)
            .bind(&r.key)
            .execute(&state.db)
            .await?
            .rows_affected()
                == 1;
            if !fresh {
                continue;
            }
            let payload = serde_json::json!({
                "title": r.title,
                "body": when(r.date, now.date(), r.start),
                "tag": r.key,
            })
            .to_string();
            for sub in &subs {
                let endpoint: String = sub.get("endpoint");
                match send_one(
                    state,
                    http,
                    &endpoint,
                    sub.get("p256dh"),
                    sub.get("auth"),
                    &payload,
                )
                .await
                {
                    Ok(status) if status == 404 || status == 410 => {
                        // the browser unsubscribed – forget this address
                        let _ = sqlx::query("DELETE FROM push_subs WHERE endpoint = $1")
                            .bind(&endpoint)
                            .execute(&state.db)
                            .await;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("push an {}: {e}", endpoint.split('/').nth(2).unwrap_or("?"))
                    }
                }
            }
        }
    }
    Ok(())
}

async fn send_one(
    state: &Arc<crate::AppState>,
    http: &reqwest::Client,
    endpoint: &str,
    p256dh: String,
    auth: String,
    payload: &str,
) -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
    let ua_public = PublicKey::from_sec1_bytes(&B64.decode(p256dh.trim_end_matches('='))?)?;
    let ua_auth = Auth::clone_from_slice(&B64.decode(auth.trim_end_matches('='))?);
    let req = WebPushBuilder::new(endpoint.parse()?, ua_public, ua_auth)
        .with_vapid(&state.vapid.keys, CONTACT)
        .build(payload.as_bytes().to_vec())?;
    let (parts, body) = req.into_parts();
    let resp = http
        .request(parts.method, parts.uri.to_string())
        .headers(parts.headers)
        .header("Urgency", "high")
        .body(body)
        .send()
        .await?;
    Ok(resp.status().as_u16())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_text() {
        let d = NaiveDate::from_ymd_opt(2026, 11, 26).unwrap();
        let t = chrono::NaiveTime::from_hms_opt(14, 0, 0);
        assert_eq!(when(d, d, t), "Heute · 14:00");
        assert_eq!(when(d.succ_opt().unwrap(), d, None), "Morgen");
        assert_eq!(
            when(NaiveDate::from_ymd_opt(2026, 11, 30).unwrap(), d, t),
            "30.11. · 14:00"
        );
    }

    #[test]
    fn vapid_key_roundtrip() {
        let k = ES256KeyPair::generate();
        let back = ES256KeyPair::from_bytes(&k.to_bytes()).unwrap();
        let a = k.public_key().public_key().to_bytes_uncompressed();
        let b = back.public_key().public_key().to_bytes_uncompressed();
        assert_eq!(a, b);
        assert_eq!(a.len(), 65); // uncompressed P-256 point, what browsers expect
    }
}

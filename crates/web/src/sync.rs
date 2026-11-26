//! Sync with the Checko server (PC ↔ phone).
//!
//! Local-first: everything is saved in the browser first. When signed in,
//! changes since the last push are sent to the server and everything newer
//! on the server is merged back in ("last write wins" by `updated_at`).

use std::time::Duration;

use checko_core::{Event, Note, Profile, ShoppingItem, Task};
use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::store::{load, save, Store};
use crate::time::now;

const CONFIG_KEY: &str = "checko.sync.v1";
const TOMBSTONES_KEY: &str = "checko.tombstones.v1";
const PROFILE_ID: Uuid = Uuid::nil();

/// Server URL baked in at build time (GitHub repo variable), can be changed in the settings.
pub const DEFAULT_SERVER: &str = match option_env!("CHECKO_SERVER_URL") {
    Some(s) => s,
    None => "",
};

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SyncConfig {
    pub server: String,
    pub token: Option<String>,
    pub username: Option<String>,
    /// Server cursor from the last successful sync.
    pub cursor: i64,
    /// Local time of the last successful push.
    pub last_push: Option<DateTime<Utc>>,
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Clone, PartialEq)]
pub enum SyncStatus {
    Off,
    Idle,
    Syncing,
    Error(String),
}

#[derive(Clone, Copy)]
pub struct Sync {
    pub config: RwSignal<SyncConfig>,
    pub status: RwSignal<SyncStatus>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Change {
    kind: String,
    id: Uuid,
    updated_at: DateTime<Utc>,
    data: Option<Value>,
}

#[derive(Deserialize)]
struct SyncResponse {
    cursor: i64,
    changes: Vec<Change>,
}

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
    username: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

/// Remember a record that was removed for good, so other devices remove it too.
pub fn tombstone(kind: &str, id: Uuid) {
    let mut list: Vec<Change> = load(TOMBSTONES_KEY);
    list.push(Change {
        kind: kind.into(),
        id,
        updated_at: now(),
        data: None,
    });
    save(TOMBSTONES_KEY, &list);
}

fn base(server: &str) -> String {
    server.trim().trim_end_matches('/').to_string()
}

async fn error_text(resp: gloo_net::http::Response) -> String {
    match resp.json::<ErrorResponse>().await {
        Ok(e) => e.error,
        Err(_) => format!("Server antwortet mit Fehler {}", resp.status()),
    }
}

fn changes_of<T: Serialize>(
    kind: &str,
    items: &[T],
    id: impl Fn(&T) -> Uuid,
    at: impl Fn(&T) -> DateTime<Utc>,
    since: Option<DateTime<Utc>>,
) -> Vec<Change> {
    items
        .iter()
        .filter(|i| since.is_none_or(|s| at(i) > s))
        .filter_map(|i| {
            Some(Change {
                kind: kind.into(),
                id: id(i),
                updated_at: at(i),
                data: Some(serde_json::to_value(i).ok()?),
            })
        })
        .collect()
}

/// Merge remote records of one kind into a local list (last write wins).
fn merge<T: DeserializeOwned + Clone + Send + std::marker::Sync + 'static>(
    list: RwSignal<Vec<T>>,
    changes: &[Change],
    kind: &str,
    id: impl Fn(&T) -> Uuid,
    at: impl Fn(&T) -> DateTime<Utc>,
) {
    let mine: Vec<&Change> = changes.iter().filter(|c| c.kind == kind).collect();
    if mine.is_empty() {
        return;
    }
    list.update(|items| {
        for c in mine {
            let pos = items.iter().position(|i| id(i) == c.id);
            match (&c.data, pos) {
                (None, Some(p)) => {
                    items.remove(p);
                }
                (None, None) => {}
                (Some(v), pos) => {
                    let Ok(remote) = serde_json::from_value::<T>(v.clone()) else {
                        continue;
                    };
                    match pos {
                        Some(p) if at(&items[p]) >= at(&remote) => {}
                        Some(p) => items[p] = remote,
                        None => items.push(remote),
                    }
                }
            }
        }
    });
}

impl Sync {
    pub fn provide(store: Store) -> Self {
        let mut config: SyncConfig = load(CONFIG_KEY);
        if config.server.is_empty() {
            config.server = DEFAULT_SERVER.to_string();
        }
        let status = if config.token.is_some() {
            SyncStatus::Idle
        } else {
            SyncStatus::Off
        };
        let sync = Sync {
            config: RwSignal::new(config),
            status: RwSignal::new(status),
        };
        Effect::new(move |_| sync.config.with(|c| save(CONFIG_KEY, c)));
        provide_context(sync);

        // Sync on start, every 30 s and shortly after local changes.
        sync.run(store);
        let _ = set_interval_with_handle(move || sync.run(store), Duration::from_secs(30));
        let generation = StoredValue::new(0u32);
        Effect::new(move |prev: Option<()>| {
            store.tasks.track();
            store.notes.track();
            store.shopping.track();
            store.events.track();
            store.profile.track();
            if prev.is_none() {
                return; // initial run
            }
            generation.update_value(|g| *g += 1);
            let mine = generation.get_value();
            set_timeout(
                move || {
                    if generation.get_value() == mine {
                        sync.run(store);
                    }
                },
                Duration::from_millis(1500),
            );
        });
        let focus = window_event_listener(leptos::ev::focus, move |_| sync.run(store));
        on_cleanup(move || focus.remove());
        sync
    }

    pub fn get() -> Self {
        expect_context::<Sync>()
    }

    /// Address of the calendar feed for Outlook (`reset` makes a new one).
    pub async fn feed_url(&self, reset: bool) -> Result<String, String> {
        #[derive(Deserialize)]
        struct FeedResponse {
            token: String,
        }
        let cfg = self.config.get_untracked();
        let token = cfg.token.clone().ok_or("Nicht angemeldet")?;
        let server = base(&cfg.server);
        let resp = Request::post(&format!("{server}/api/feed"))
            .header("Authorization", &format!("Bearer {token}"))
            .json(&serde_json::json!({ "reset": reset }))
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|_| "Server nicht erreichbar – gleich noch einmal versuchen.".to_string())?;
        if !resp.ok() {
            return Err(error_text(resp).await);
        }
        let f: FeedResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(format!("{server}/api/feed/{}.ics", f.token))
    }

    pub fn signed_in(&self) -> bool {
        self.config.with(|c| c.token.is_some())
    }

    /// Register (`create = true`) or log in; on success syncs right away.
    pub fn sign_in(
        &self,
        store: Store,
        server: String,
        username: String,
        password: String,
        create: bool,
    ) {
        let sync = *self;
        sync.status.set(SyncStatus::Syncing);
        spawn_local(async move {
            let url = format!(
                "{}/api/{}",
                base(&server),
                if create { "register" } else { "login" }
            );
            let body = serde_json::json!({ "username": username, "password": password });
            let result = async {
                let resp = Request::post(&url).json(&body).map_err(|e| e.to_string())?.send().await.map_err(|_| {
                    "Server nicht erreichbar. Adresse prüfen – ein schlafender Gratis-Server braucht bis zu einer Minute.".to_string()
                })?;
                if !resp.ok() {
                    return Err(error_text(resp).await);
                }
                resp.json::<TokenResponse>().await.map_err(|e| e.to_string())
            }
            .await;
            match result {
                Ok(t) => {
                    sync.config.update(|c| {
                        c.server = base(&server);
                        c.token = Some(t.token);
                        c.username = Some(t.username);
                        // first sync after login: push everything, pull everything
                        c.cursor = 0;
                        c.last_push = None;
                    });
                    sync.status.set(SyncStatus::Idle);
                    sync.run(store);
                }
                Err(e) => sync.status.set(SyncStatus::Error(e)),
            }
        });
    }

    pub fn sign_out(&self) {
        let c = self.config.get_untracked();
        if let (Some(token), false) = (c.token.clone(), c.server.is_empty()) {
            let url = format!("{}/api/logout", base(&c.server));
            spawn_local(async move {
                let _ = Request::post(&url)
                    .header("Authorization", &format!("Bearer {token}"))
                    .send()
                    .await;
            });
        }
        self.config.update(|c| {
            c.token = None;
            c.username = None;
            c.cursor = 0;
            c.last_push = None;
            c.last_sync = None;
        });
        self.status.set(SyncStatus::Off);
    }

    /// One sync round (does nothing when signed out or already running).
    pub fn run(&self, store: Store) {
        let sync = *self;
        let cfg = sync.config.get_untracked();
        let Some(token) = cfg.token.clone() else {
            return;
        };
        if cfg.server.is_empty() || sync.status.get_untracked() == SyncStatus::Syncing {
            return;
        }
        sync.status.set(SyncStatus::Syncing);
        let started = now();
        let since = cfg.last_push;

        let mut changes = Vec::new();
        store.tasks.with_untracked(|t| {
            changes.extend(changes_of(
                "task",
                t,
                |x: &Task| x.meta.id,
                |x| x.meta.updated_at,
                since,
            ))
        });
        store.notes.with_untracked(|n| {
            changes.extend(changes_of(
                "note",
                n,
                |x: &Note| x.meta.id,
                |x| x.meta.updated_at,
                since,
            ))
        });
        store.shopping.with_untracked(|s| {
            changes.extend(changes_of(
                "shopping",
                s,
                |x: &ShoppingItem| x.meta.id,
                |x| x.meta.updated_at,
                since,
            ))
        });
        store.events.with_untracked(|e| {
            changes.extend(changes_of(
                "event",
                e,
                |x: &Event| x.meta.id,
                |x| x.meta.updated_at,
                since,
            ))
        });
        store.profile.with_untracked(|p| {
            let at = p.updated_at.unwrap_or_default();
            if since.is_none_or(|s| at > s) {
                if let Ok(v) = serde_json::to_value(p) {
                    changes.push(Change {
                        kind: "profile".into(),
                        id: PROFILE_ID,
                        updated_at: at,
                        data: Some(v),
                    });
                }
            }
        });
        let tombstones: Vec<Change> = load(TOMBSTONES_KEY);
        let sent_tombstones = tombstones.len();
        changes.extend(tombstones);

        spawn_local(async move {
            let url = format!("{}/api/sync", base(&cfg.server));
            let body = serde_json::json!({ "since": cfg.cursor, "changes": changes });
            let result = async {
                let resp = Request::post(&url)
                    .header("Authorization", &format!("Bearer {token}"))
                    .json(&body)
                    .map_err(|e| e.to_string())?
                    .send()
                    .await
                    .map_err(|_| {
                        "Server nicht erreichbar – neuer Versuch in 30 Sekunden.".to_string()
                    })?;
                if resp.status() == 401 {
                    return Err("Bitte neu anmelden.".to_string());
                }
                if !resp.ok() {
                    return Err(error_text(resp).await);
                }
                resp.json::<SyncResponse>().await.map_err(|e| e.to_string())
            }
            .await;

            match result {
                Ok(r) => {
                    merge(
                        store.tasks,
                        &r.changes,
                        "task",
                        |x: &Task| x.meta.id,
                        |x| x.meta.updated_at,
                    );
                    merge(
                        store.notes,
                        &r.changes,
                        "note",
                        |x: &Note| x.meta.id,
                        |x| x.meta.updated_at,
                    );
                    merge(
                        store.shopping,
                        &r.changes,
                        "shopping",
                        |x: &ShoppingItem| x.meta.id,
                        |x| x.meta.updated_at,
                    );
                    merge(
                        store.events,
                        &r.changes,
                        "event",
                        |x: &Event| x.meta.id,
                        |x| x.meta.updated_at,
                    );
                    if let Some(remote) = r
                        .changes
                        .iter()
                        .filter(|c| c.kind == "profile")
                        .filter_map(|c| serde_json::from_value::<Profile>(c.data.clone()?).ok())
                        .next_back()
                    {
                        if remote.updated_at > store.profile.with_untracked(|p| p.updated_at) {
                            store.profile.set(remote);
                        }
                    }
                    // drop the tombstones we just sent (new ones may have been added meanwhile)
                    let mut left: Vec<Change> = load(TOMBSTONES_KEY);
                    left.drain(..sent_tombstones.min(left.len()));
                    save(TOMBSTONES_KEY, &left);
                    sync.config.update(|c| {
                        c.cursor = r.cursor;
                        c.last_push = Some(started);
                        c.last_sync = Some(now());
                    });
                    sync.status.set(SyncStatus::Idle);
                }
                Err(e) => {
                    if e == "Bitte neu anmelden." {
                        sync.config.update(|c| c.token = None);
                    }
                    sync.status.set(SyncStatus::Error(e));
                }
            }
        });
    }
}

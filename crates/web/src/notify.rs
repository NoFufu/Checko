//! Reminders as system notifications.
//!
//! Two ways, both use the same tag per reminder so nothing shows twice:
//! - **In the app:** while Checko is open, a timer checks every 30 s and shows
//!   due reminders via the service worker (works on PC and in the open PWA).
//! - **Push:** when signed in to sync, the browser subscribes to Web Push and
//!   the server sends reminders even when Checko is closed (iPhone: only for
//!   the app on the home screen, iOS 16.4+).

use std::time::Duration;

use checko_core::reminders_between;
use chrono::{NaiveDate, NaiveDateTime};
use gloo_net::http::Request;
use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    NotificationOptions, NotificationPermission, PushSubscriptionOptionsInit,
    ServiceWorkerRegistration,
};

use crate::store::Store;
use crate::sync::Sync;
use crate::time::due_label;

const SENT_KEY: &str = "checko.reminded.v1";

/// Is the Notification API available at all (not in every browser / iOS Safari tab)?
pub fn supported() -> bool {
    web_sys::window()
        .map(|w| js_sys::Reflect::has(&w, &JsValue::from_str("Notification")).unwrap_or(false))
        .unwrap_or(false)
}

/// iPhone/iPad in a normal Safari tab: Apple allows notifications only for
/// web apps started from the home screen.
pub fn ios_tab() -> bool {
    let Some(w) = web_sys::window() else {
        return false;
    };
    let ua = w.navigator().user_agent().unwrap_or_default();
    let ios = ua.contains("iPhone") || ua.contains("iPad");
    let standalone = w
        .match_media("(display-mode: standalone)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches());
    ios && !standalone
}

pub fn permission() -> NotificationPermission {
    if supported() {
        web_sys::Notification::permission()
    } else {
        NotificationPermission::Denied
    }
}

/// Ask once (only from a click) – called when a reminder is switched on.
pub fn ensure_permission() {
    if supported() && permission() == NotificationPermission::Default {
        leptos::task::spawn_local(async {
            let _ = request_permission().await;
        });
    }
}

pub async fn request_permission() -> NotificationPermission {
    // already decided: asking again can hang in installed app windows
    if permission() != NotificationPermission::Default {
        return permission();
    }
    if let Ok(p) = web_sys::Notification::request_permission() {
        // a dismissed or quiet prompt can stay open for a long time
        let _ = within(p, 60_000).await;
    }
    permission()
}

fn local_now() -> NaiveDateTime {
    let d = js_sys::Date::new_0();
    NaiveDate::from_ymd_opt(d.get_full_year() as i32, d.get_month() + 1, d.get_date())
        .and_then(|day| day.and_hms_opt(d.get_hours(), d.get_minutes(), d.get_seconds()))
        .unwrap_or_default()
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn sent() -> Vec<String> {
    storage()
        .and_then(|s| s.get_item(SENT_KEY).ok().flatten())
        .and_then(|v| serde_json::from_str(&v).ok())
        .unwrap_or_default()
}

fn mark_sent(key: &str) {
    let mut all = sent();
    all.push(key.to_string());
    let skip = all.len().saturating_sub(300);
    if let (Some(s), Ok(v)) = (storage(), serde_json::to_string(&all[skip..])) {
        let _ = s.set_item(SENT_KEY, &v);
    }
}

/// Wait for a JS promise, but give up after `ms` (some company networks block
/// the browser's push service, then `subscribe()` never answers).
async fn within(p: js_sys::Promise, ms: i32) -> Result<JsValue, &'static str> {
    let timer = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(w) = web_sys::window() {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_1(
                &resolve,
                ms,
                &JsValue::from_str("__timeout__"),
            );
        }
    });
    let race = js_sys::Promise::race(&js_sys::Array::of2(&p, &timer));
    match JsFuture::from(race).await {
        Ok(v) if v.as_string().as_deref() == Some("__timeout__") => Err("timeout"),
        Ok(v) => Ok(v),
        Err(_) => Err("error"),
    }
}

/// Abort signal that fires after `ms` (so a hanging request cannot block forever).
fn deadline(ms: i32) -> Option<web_sys::AbortSignal> {
    let c = web_sys::AbortController::new().ok()?;
    let signal = c.signal();
    let abort = wasm_bindgen::closure::Closure::once_into_js(move || c.abort());
    web_sys::window()?
        .set_timeout_with_callback_and_timeout_and_arguments_0(abort.unchecked_ref(), ms)
        .ok()?;
    Some(signal)
}

const SLOW_SERVER: &str = "Server antwortet nicht. Der Gratis-Server schläft manchmal – in einer Minute noch einmal tippen. \
     Im Firmennetz kann er auch gesperrt sein.";

async fn registration() -> Option<ServiceWorkerRegistration> {
    let sw = web_sys::window()?.navigator().service_worker();
    let reg = within(sw.ready().ok()?, 8000).await.ok()?;
    reg.dyn_into().ok()
}

/// Show one notification (via the service worker so it also works in the PWA).
pub async fn show(title: &str, body: &str, tag: &str) {
    let _ = try_show(title, body, tag).await;
}

/// Like `show`, but says which way worked or why it failed (for the test button).
pub async fn try_show(title: &str, body: &str, tag: &str) -> Result<&'static str, String> {
    if permission() != NotificationPermission::Granted {
        return Err("Benachrichtigungen sind für Checko in diesem Browser nicht erlaubt.".into());
    }
    let opts = NotificationOptions::new();
    opts.set_body(body);
    opts.set_tag(tag);
    opts.set_icon(&crate::asset("brand/icon-192.png"));
    if let Some(reg) = registration().await {
        if let Ok(p) = reg.show_notification_with_options(title, &opts) {
            return match within(p, 5000).await {
                Ok(_) => Ok("über den Service Worker"),
                Err(e) => Err(format!("Service Worker hat abgelehnt ({e}).")),
            };
        }
    }
    web_sys::Notification::new_with_options(title, &opts)
        .map(|_| "direkt")
        .map_err(|_| "Der Browser hat die Benachrichtigung abgelehnt.".to_string())
}

pub fn body_for(date: NaiveDate, start: Option<chrono::NaiveTime>) -> String {
    let mut s = due_label(date, crate::time::today());
    if let Some(t) = start {
        s.push_str(&format!(" · {}", t.format("%H:%M")));
    }
    s
}

/// Start the in-app timer (call once at app start).
pub fn start(store: Store) {
    // reminders up to 10 minutes late are still shown (app just opened)
    let last = StoredValue::new(local_now() - chrono::Duration::minutes(10));
    let tick = move || {
        if permission() != NotificationPermission::Granted {
            return;
        }
        let now = local_now();
        let due = store.events.with_untracked(|e| {
            store
                .tasks
                .with_untracked(|t| reminders_between(e, t, last.get_value(), now))
        });
        last.set_value(now);
        let done = sent();
        for r in due.into_iter().filter(|r| !done.contains(&r.key)) {
            mark_sent(&r.key);
            let body = body_for(r.date, r.start);
            leptos::task::spawn_local(async move { show(&r.title, &body, &r.key).await });
        }
    };
    tick();
    let _ = set_interval_with_handle(tick, Duration::from_secs(30));
}

#[derive(Serialize)]
struct Subscribe {
    subscription: serde_json::Value,
    tz: i32,
}

/// Turn on push (server sends reminders while Checko is closed).
/// Returns a German error text on failure.
/// Turn on push. `step` shows what is happening right now (for the button).
pub async fn enable_push(step: impl Fn(&'static str)) -> Result<(), String> {
    step("Frage nach Erlaubnis …");
    if request_permission().await != NotificationPermission::Granted {
        return Err("Benachrichtigungen wurden nicht erlaubt. Du kannst sie in den Browser-/iPhone-Einstellungen freigeben.".into());
    }
    let sync = Sync::get();
    let cfg = sync.config.get_untracked();
    let Some(token) = cfg.token.clone() else {
        // no account: in-app reminders only
        return Ok(());
    };
    let server = cfg.server.trim_end_matches('/').to_string();
    step("Verbinde mit dem Server …");
    let key = Request::get(&format!("{server}/api/push/key"))
        .abort_signal(deadline(70_000).as_ref())
        .send()
        .await
        .map_err(|_| SLOW_SERVER.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    step("Starte Hintergrund-Dienst …");
    let reg = registration()
        .await
        .ok_or("Kein Service Worker – bitte Seite neu laden (privates Fenster und manche Firmen-Richtlinien blockieren ihn).")?;
    let opts = PushSubscriptionOptionsInit::new();
    opts.set_user_visible_only(true);
    opts.set_application_server_key(&JsValue::from_str(key.trim()));
    let push = reg.push_manager().map_err(|_| {
        "Push wird hier nicht unterstützt (iPhone: Checko zum Home-Bildschirm hinzufügen)."
    })?;
    step("Melde beim Push-Dienst des Browsers an …");
    let sub = match within(
        push.subscribe_with_options(&opts)
            .map_err(|_| "Push wird nicht unterstützt.")?,
        20000,
    )
    .await
    {
        Ok(sub) => sub,
        Err("timeout") => return Err(
            "Der Push-Dienst des Browsers antwortet nicht – im Firmennetz ist er oft gesperrt. \
             Erinnerungen kommen trotzdem, solange Checko geöffnet ist. Zu Hause oder am Handy klappt Push meist."
                .into(),
        ),
        Err(_) => return Err("Push-Anmeldung abgelehnt (Browser-Einstellungen prüfen).".into()),
    };
    let json = js_sys::JSON::stringify(&sub).map_err(|_| "Fehler beim Lesen der Anmeldung.")?;
    let subscription: serde_json::Value =
        serde_json::from_str(&String::from(json)).map_err(|e| e.to_string())?;
    let tz = -(js_sys::Date::new_0().get_timezone_offset() as i32);
    step("Speichere auf dem Server …");
    let resp = Request::post(&format!("{server}/api/push/subscribe"))
        .header("Authorization", &format!("Bearer {token}"))
        .abort_signal(deadline(70_000).as_ref())
        .json(&Subscribe { subscription, tz })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|_| SLOW_SERVER.to_string())?;
    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Server-Fehler {}", resp.status()))
    }
}

/// Ask the server to send a test push right now; returns its report.
pub async fn server_test() -> Result<String, String> {
    server_test_in(0).await
}

/// Test reminder after `secs`: shown by the open app *and* pushed by the server
/// (same tag, so it appears only once). Tests the real path, also with Checko closed.
pub async fn delayed_test(secs: u32) -> String {
    let title = "Checko-Test";
    let body = format!("Test-Erinnerung nach {} Min. – es klappt! 🦎", secs / 60);
    let fire = wasm_bindgen::closure::Closure::once_into_js(move || {
        leptos::task::spawn_local(async move { show(title, &body, "checko-delayed-test").await });
    });
    if let Some(w) = web_sys::window() {
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
            fire.unchecked_ref(),
            secs as i32 * 1000,
        );
    }
    let app = format!(
        "✓ Checko erinnert in {} Min., solange es geöffnet ist.",
        secs / 60
    );
    if Sync::get().config.with_untracked(|c| c.token.is_none()) {
        return app;
    }
    match server_test_in(secs).await {
        Ok(r) => format!("{app}\n{r}"),
        Err(e) => format!("{app}\n✗ Server: {e}"),
    }
}

async fn server_test_in(secs: u32) -> Result<String, String> {
    let cfg = Sync::get().config.get_untracked();
    let token = cfg.token.ok_or("Nicht angemeldet")?;
    let server = cfg.server.trim_end_matches('/').to_string();
    let resp = Request::post(&format!("{server}/api/push/test?in={secs}"))
        .header("Authorization", &format!("Bearer {token}"))
        .abort_signal(deadline(70_000).as_ref())
        .send()
        .await
        .map_err(|_| SLOW_SERVER.to_string())?;
    if !resp.ok() {
        return Err(format!(
            "Server-Fehler {} – ist der Server schon auf dem neuesten Stand?",
            resp.status()
        ));
    }
    resp.text().await.map_err(|e| e.to_string())
}

/// Is this browser already subscribed to push?
pub async fn push_active() -> bool {
    let Some(reg) = registration().await else {
        return false;
    };
    let Ok(pm) = reg.push_manager() else {
        return false;
    };
    let Ok(p) = pm.get_subscription() else {
        return false;
    };
    JsFuture::from(p)
        .await
        .map(|v| !v.is_null() && !v.is_undefined())
        .unwrap_or(false)
}

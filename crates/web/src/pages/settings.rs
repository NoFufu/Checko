use leptos::prelude::*;

use crate::components::{
    icon::{Glyph, Icon},
    topbar::Topbar,
};
use crate::store::Store;
use crate::sync::{Sync, SyncStatus};

#[component]
pub fn SettingsPage() -> impl IntoView {
    let store = Store::get();
    let sync = Sync::get();
    let server = RwSignal::new(sync.config.get_untracked().server);
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let signed_in = move || sync.signed_in();

    let submit = move |create: bool| {
        sync.sign_in(
            store,
            server.get_untracked(),
            username.get_untracked(),
            password.get_untracked(),
            create,
        );
        password.set(String::new());
    };

    let status_text = move || match sync.status.get() {
        SyncStatus::Off => "Nicht verbunden – alles bleibt nur in diesem Browser.".to_string(),
        SyncStatus::Syncing => "Synchronisiere …".to_string(),
        SyncStatus::Idle => match sync.config.with(|c| c.last_sync) {
            Some(t) => format!("Synchronisiert · zuletzt {}", crate::time::local_label(t)),
            None => "Verbunden".to_string(),
        },
        SyncStatus::Error(e) => format!("Fehler: {e}"),
    };
    let is_error = move || matches!(sync.status.get(), SyncStatus::Error(_));

    view! {
        <Topbar title="Einstellungen" icon=Glyph::Settings />
        <div class="page settings">
            <div class="page-head">
                <div>
                    <h1 class="page-h1">"Einstellungen"</h1>
                    <p class="text-muted">"Synchronisierung zwischen PC und Handy."</p>
                </div>
            </div>

            <section class="panel settings-section">
                <h2 class="section-head"><Icon glyph=Glyph::Repeat size=18 />"Synchronisieren"</h2>
                <p class="sync-status" class:error=is_error role="status">{status_text}</p>

                <Show
                    when=signed_in
                    fallback=move || view! {
                        <form class="settings-form" on:submit=move |e| { e.prevent_default(); submit(false); }>
                            <label class="settings-field">
                                <span>"Server-Adresse"</span>
                                <input type="url" placeholder="https://checko-server.onrender.com" bind:value=server required=true />
                            </label>
                            <label class="settings-field">
                                <span>"Benutzername"</span>
                                <input type="text" autocomplete="username" bind:value=username required=true />
                            </label>
                            <label class="settings-field">
                                <span>"Passwort"</span>
                                <input type="password" autocomplete="current-password" bind:value=password required=true minlength="8" />
                            </label>
                            <div class="flex flex-wrap gap-2">
                                <button type="submit" class="btn btn-primary">"Anmelden"</button>
                                <button type="button" class="btn btn-outline" on:click=move |_| submit(true)>"Konto erstellen"</button>
                            </div>
                            <p class="text-sm text-muted">
                                "Auf dem ersten Gerät „Konto erstellen“, auf allen weiteren mit denselben Daten „Anmelden“. "
                                "Was schon in diesem Browser gespeichert ist, wird dabei hochgeladen."
                            </p>
                        </form>
                    }
                >
                    <div class="flex flex-wrap items-center gap-2">
                        <span class="chip">{move || format!("Angemeldet als {}", sync.config.with(|c| c.username.clone().unwrap_or_default()))}</span>
                        <button type="button" class="btn btn-primary" on:click=move |_| sync.run(store)>"Jetzt synchronisieren"</button>
                        <button type="button" class="btn btn-outline" on:click=move |_| sync.sign_out()>"Abmelden"</button>
                    </div>
                    <p class="text-sm text-muted mt-2">{move || format!("Server: {}", sync.config.with(|c| c.server.clone()))}</p>
                </Show>
            </section>

            <Show when=signed_in>
                <WidgetSetup />
            </Show>
        </div>
    }
}

const WIDGET_TEMPLATE: &str = include_str!("../../../../docs/widget/checko-widget.js");

/// Copy-ready Scriptable script with server address and token filled in.
#[component]
fn WidgetSetup() -> impl IntoView {
    let sync = Sync::get();
    let copied = RwSignal::new(false);
    let script = move || {
        let c = sync.config.get();
        let app = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .map(|o| format!("{o}{}/", crate::BASE))
            .unwrap_or_default();
        WIDGET_TEMPLATE
            .replace("__SERVER__", &c.server)
            .replace("__TOKEN__", c.token.as_deref().unwrap_or_default())
            .replace("__APP__", &app)
    };
    let copy = move |_| {
        if let Some(w) = web_sys::window() {
            let _ = w.navigator().clipboard().write_text(&script());
            copied.set(true);
        }
    };
    view! {
        <section class="panel settings-section">
            <h2 class="section-head"><Icon glyph=Glyph::Today size=18 />"iPhone-Widget"</h2>
            <p class="text-sm text-muted">
                "Zeigt heute fällige Aufgaben, den nächsten Termin und deinen Streak auf dem Startbildschirm – ohne die App zu öffnen. "
                "Aktualisiert sich etwa alle 15 Minuten."
            </p>
            <ol class="widget-steps">
                <li>"Die kostenlose App "<b>"Scriptable"</b>" aus dem App Store laden."</li>
                <li>"Hier auf "<b>"„Widget-Skript kopieren“"</b>" tippen."</li>
                <li>"In Scriptable oben rechts "<b>"+"</b>" → Text einfügen → oben den Namen "<b>"Checko"</b>" geben → Fertig."</li>
                <li>"Startbildschirm lange drücken → "<b>"+"</b>" → "<b>"Scriptable"</b>" → mittlere Größe → Widget antippen → bei "<b>"Script"</b>" „Checko“ wählen."</li>
            </ol>
            <div class="flex flex-wrap items-center gap-2">
                <button type="button" class="btn btn-primary" on:click=copy>
                    {move || if copied.get() { "Kopiert ✓" } else { "Widget-Skript kopieren" }}
                </button>
                <span class="text-sm text-muted">"Das Skript enthält deinen Zugangsschlüssel – nicht weitergeben."</span>
            </div>
        </section>
    }
}

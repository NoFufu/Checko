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
        <Topbar />
        <div class="page settings">
            <div class="page-head">
                <div>
                    <h1 class="page-h1">"Einstellungen"</h1>
                    <p class="text-muted">"Konto, Sync, Kategorien, Fokus-Modus und Darstellung."</p>
                </div>
            </div>

            <NotifySetup />

            <section class="panel settings-section">
                <h2 class="section-head"><Icon glyph=Glyph::Calendar size=18 />"Kalender importieren"</h2>
                <p class="text-sm text-muted">"Outlook-Kalender als .ics-Datei oder Outlook-Datendatei (.pst) hier einfügen – die Datei bleibt auf deinem Gerät. Termine erscheinen im Checko-Kalender."</p>
                <crate::components::data_tools::IcsImport />
            </section>

            <section class="panel settings-section">
                <h2 class="section-head"><Icon glyph=Glyph::Notes size=18 />"Datensicherung"</h2>
                <p class="text-sm text-muted">"Alle Aufgaben, Termine, Notizen und Einkäufe als Datei sichern – und bei Bedarf wieder einspielen."</p>
                <crate::components::data_tools::BackupTools />
            </section>

            <section class="panel settings-section">
                <h2 class="section-head"><Icon glyph=Glyph::Pin size=18 />"Kategorien"</h2>
                <p class="text-sm text-muted">"Umbenennen, Emoji und Farbe wählen oder löschen. Gilt für alle Aufgaben und Termine."</p>
                <crate::components::categories::CategoryManager />
            </section>

            <section class="panel settings-section settings-row">
                <span>
                    <span class="block font-semibold">"Fokus-Modus"</span>
                    <span class="block text-sm text-muted">"Blendet XP, Level, Streak, Erfolge und Konfetti aus."</span>
                </span>
                <crate::components::categories::FocusSwitch />
            </section>

            <section class="panel settings-section settings-row">
                <span>
                    <span class="block font-semibold">"Darstellung"</span>
                    <span class="block text-sm text-muted">"Hell oder dunkel"</span>
                </span>
                <crate::components::theme::ThemeSwitch />
            </section>

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
                <OutlookFeed />
            </Show>

            <Show when=signed_in>
                <WidgetSetup />
            </Show>
        </div>
    }
}

/// Calendar feed: show Checko's appointments in Outlook (subscription link).
#[component]
fn OutlookFeed() -> impl IntoView {
    let sync = Sync::get();
    let url = RwSignal::new(None::<String>);
    let busy = RwSignal::new(false);
    let msg = RwSignal::new(None::<String>);
    let fetch = move |reset: bool| {
        busy.set(true);
        msg.set(None);
        leptos::task::spawn_local(async move {
            match sync.feed_url(reset).await {
                Ok(u) => {
                    url.set(Some(u));
                    if reset {
                        msg.set(Some(
                            "Neuer Link erstellt – der alte funktioniert nicht mehr.".into(),
                        ));
                    }
                }
                Err(e) => msg.set(Some(e)),
            }
            busy.set(false);
        });
    };
    let copy = move |_| {
        if let (Some(w), Some(u)) = (web_sys::window(), url.get_untracked()) {
            let _ = w.navigator().clipboard().write_text(&u);
            msg.set(Some("Link kopiert ✓".into()));
        }
    };
    let reset = move |_| {
        let ok = web_sys::window()
            .and_then(|w| {
                w.confirm_with_message("Neuen Link erstellen? Outlook muss dann neu abonnieren.")
                    .ok()
            })
            .unwrap_or(false);
        if ok {
            fetch(true);
        }
    };
    view! {
        <section class="panel settings-section">
            <h2 class="section-head"><Icon glyph=Glyph::Calendar size=18 />"In Outlook anzeigen"</h2>
            <p class="text-sm text-muted">
                "Deine Checko-Termine als Kalender-Abo – Outlook holt Änderungen von selbst (je nach Outlook alle paar Stunden). "
                "Aus Outlook importierte Termine sind nicht dabei, damit nichts doppelt erscheint."
            </p>
            <Show
                when=move || url.get().is_some()
                fallback=move || view! {
                    <div>
                        <button type="button" class="btn btn-primary" disabled=move || busy.get() on:click=move |_| fetch(false)>
                            {move || if busy.get() { "Einen Moment …" } else { "Abo-Link anzeigen" }}
                        </button>
                    </div>
                }
            >
                <input class="feed-url" readonly=true aria-label="Abo-Link" prop:value=move || url.get().unwrap_or_default()
                    on:focus=move |e| { event_target::<web_sys::HtmlInputElement>(&e).select(); } />
                <div class="flex flex-wrap items-center gap-2">
                    <button type="button" class="btn btn-primary" on:click=copy>"Link kopieren"</button>
                    <button type="button" class="btn btn-outline" disabled=move || busy.get() on:click=reset>"Neuen Link erstellen"</button>
                </div>
                <ol class="widget-steps">
                    <li><b>"Outlook im Web / neues Outlook:"</b>" Kalender → "<b>"Kalender hinzufügen"</b>" → "<b>"Aus dem Internet abonnieren"</b>" → Link einfügen, Name „Checko“ → Importieren."</li>
                    <li><b>"Klassisches Outlook:"</b>" Kalender → "<b>"Kalender hinzufügen"</b>" → "<b>"Aus dem Internet …"</b>" → Link einfügen → Ja."</li>
                    <li>"Geht das in der Firma nicht, blockiert die IT externe Kalender – dann bleibt nur Checko im Browser."</li>
                </ol>
                <p class="text-sm text-muted">"Wer den Link hat, sieht deine Termine – nicht weitergeben."</p>
            </Show>
            {move || msg.get().map(|m| view! { <p class="text-sm mt-2" role="status">{m}</p> })}
        </section>
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
                "Zeigt heute fällige Aufgaben, den nächsten Termin und deinen Streak – auf dem Start- und auf dem Sperrbildschirm. "
                "Aktualisiert sich etwa alle 15 Minuten. Das Skript holt sich neue Designs automatisch, du richtest es nur einmal ein."
            </p>
            <ol class="widget-steps">
                <li>"Die kostenlose App "<b>"Scriptable"</b>" aus dem App Store laden."</li>
                <li>"Hier auf "<b>"„Widget-Skript kopieren“"</b>" tippen."</li>
                <li>"In Scriptable oben rechts "<b>"+"</b>" → Text einfügen → oben den Namen "<b>"Checko"</b>" geben → Fertig. Hast du schon ein Checko-Skript: dessen Text komplett ersetzen (einmalig)."</li>
                <li><b>"Startbildschirm:"</b>" lange drücken → "<b>"+"</b>" → "<b>"Scriptable"</b>" → Größe wählen → Widget antippen → bei "<b>"Script"</b>" „Checko“ wählen."</li>
                <li><b>"Sperrbildschirm:"</b>" Sperrbildschirm lange drücken → "<b>"Anpassen"</b>" → Sperrbildschirm → Widget-Feld antippen → "<b>"Scriptable"</b>" → das breite Feld wählen → antippen → „Checko“ wählen."</li>
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

/// Turn on reminders: permission, push subscription (with sync account), test.
#[component]
fn NotifySetup() -> impl IntoView {
    use web_sys::NotificationPermission as P;
    let perm = RwSignal::new(crate::notify::permission());
    let push = RwSignal::new(false);
    let busy = RwSignal::new(false);
    let step = RwSignal::new(None::<&'static str>);
    let msg = RwSignal::new(None::<String>);
    let sync = Sync::get();
    let signed_in = move || sync.config.with(|c| c.token.is_some());
    leptos::task::spawn_local(async move { push.set(crate::notify::push_active().await) });
    let enable = move |_| {
        busy.set(true);
        msg.set(None);
        leptos::task::spawn_local(async move {
            match crate::notify::enable_push(move |t| step.set(Some(t))).await {
                Ok(()) => {
                    push.set(crate::notify::push_active().await);
                    msg.set(Some("Fertig! Erinnerungen sind an.".into()));
                }
                Err(e) => msg.set(Some(e)),
            }
            perm.set(crate::notify::permission());
            step.set(None);
            busy.set(false);
        });
    };
    let test = move |_| {
        leptos::task::spawn_local(async move {
            let r = crate::notify::try_show(
                "Checko",
                "So sehen deine Erinnerungen aus. 🦎",
                "checko-test",
            )
            .await;
            msg.set(Some(match r {
                Ok(how) => format!(
                    "Gesendet ({how}). Nichts gesehen? Dann blockiert Windows/der Browser die Anzeige: \
                     „Nicht stören“ bzw. Fokus aus, und im Browser beim Schloss-Symbol links neben der Adresse \
                     „Benachrichtigungen: Zulassen“ prüfen."
                ),
                Err(e) => e,
            }));
        })
    };
    let ios_tab = crate::notify::ios_tab();
    let status = move || {
        if ios_tab {
            return "Im Safari-Tab erlaubt das iPhone keine Benachrichtigungen. So geht's: unten auf Teilen (□↑) → „Zum Home-Bildschirm“ → Checko über das neue Symbol öffnen und hier auf „Erinnerungen aktivieren“ tippen.";
        }
        match (perm.get(), push.get(), signed_in()) {
        (P::Granted, true, _) => "An – auch wenn Checko geschlossen ist.",
        (P::Granted, false, true) => "An, solange Checko geöffnet ist. Tippe auf „Aktivieren“ für Erinnerungen bei geschlossener App.",
        (P::Granted, false, false) => "An, solange Checko geöffnet ist. Mit Sync-Konto kommen sie auch bei geschlossener App.",
        (P::Denied, _, _) => "Blockiert – in den Browser- bzw. iPhone-Einstellungen für Checko erlauben.",
        _ => "Noch aus.",
    }
    };
    view! {
        <section class="panel settings-section">
            <h2 class="section-head"><Icon glyph=Glyph::Bell size=18 />"Erinnerungen"</h2>
            <p class="text-sm text-muted">
                "Bei Terminen und Aufgaben unter „Erinnerung“ einstellen, wann Checko Bescheid gibt (z. B. 15 Min. vorher). "
                "Auf dem iPhone funktioniert das nur mit Checko auf dem Home-Bildschirm (Teilen → Zum Home-Bildschirm)."
            </p>
            <p class="notify-status" class:on=move || perm.get() == P::Granted>{status}</p>
            <div class="flex flex-wrap items-center gap-2">
                <Show when=move || !ios_tab && !(perm.get() == P::Granted && (push.get() || !signed_in()))>
                    <button type="button" class="btn btn-primary" disabled=move || busy.get() on:click=enable>
                        {move || if busy.get() { step.get().unwrap_or("Einen Moment …") } else { "Erinnerungen aktivieren" }}
                    </button>
                </Show>
                <Show when=move || perm.get() == P::Granted>
                    <button type="button" class="btn btn-outline" on:click=test>"Test-Benachrichtigung"</button>
                </Show>
            </div>
            {move || msg.get().map(|m| view! { <p class="text-sm mt-2">{m}</p> })}
        </section>
    }
}

use leptos::prelude::*;

use super::icon::{Glyph, Icon};

/// Page header: title on the left, notifications and "Neu" on the right.
#[component]
pub fn Topbar(title: &'static str, icon: Glyph) -> impl IntoView {
    view! {
        <header class="topbar">
            <div class="crumb">
                <Icon glyph=icon size=16 />
                <span>{title}</span>
            </div>
            <div class="flex items-center gap-2">
                <button type="button" class="btn-icon" aria-label="Benachrichtigungen" title="Erinnerungen kommen in Phase 6">
                    <Icon glyph=Glyph::Bell size=18 />
                </button>
                <button
                    type="button"
                    class="btn btn-primary"
                    title="Schnell hinzufügen (Strg K)"
                    on:click=move |_| crate::store::Store::get().quick_add_open.set(true)
                >
                    <Icon glyph=Glyph::Add size=16 />
                    "Neu"
                </button>
            </div>
        </header>
    }
}

/// Phone-only header: logo, sync state, search/new and settings.
#[component]
pub fn MobileHeader() -> impl IntoView {
    let store = crate::store::Store::get();
    let sync = crate::sync::Sync::get();
    let dot = move || match sync.status.get() {
        crate::sync::SyncStatus::Off => ("sync-dot off", "Nur lokal gespeichert"),
        crate::sync::SyncStatus::Syncing => ("sync-dot busy", "Synchronisiere …"),
        crate::sync::SyncStatus::Idle => ("sync-dot ok", "Synchronisiert"),
        crate::sync::SyncStatus::Error(_) => ("sync-dot err", "Sync-Fehler"),
    };
    view! {
        <header class="mobile-header">
            <a href=format!("{}/", crate::BASE) class="mobile-logo" aria-label="Checko Startseite">
                <super::logo::Logo />
            </a>
            <div class="flex items-center gap-1">
                <a href=format!("{}/einstellungen", crate::BASE) class="mobile-sync" title=move || dot().1 aria-label=move || dot().1>
                    <span class=move || dot().0></span>
                </a>
                <button type="button" class="btn-icon" aria-label="Suchen" on:click=move |_| store.quick_add_open.set(true)>
                    <Icon glyph=Glyph::Search size=18 />
                </button>
                <a href=format!("{}/einstellungen", crate::BASE) class="btn-icon" aria-label="Einstellungen">
                    <Icon glyph=Glyph::Settings size=18 />
                </a>
            </div>
        </header>
        <button type="button" class="fab" aria-label="Neue Aufgabe" on:click=move |_| store.quick_add_open.set(true)>
            <Icon glyph=Glyph::Add size=26 />
        </button>
    }
}

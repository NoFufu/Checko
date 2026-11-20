use leptos::prelude::*;

use super::placeholder::Placeholder;
use crate::components::icon::Glyph;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <Placeholder
            title="Einstellungen"
            icon=Glyph::Settings
            phase="Kommt später"
            hint="Bereiche verwalten, Daten exportieren, Konto und Synchronisierung."
        />
    }
}

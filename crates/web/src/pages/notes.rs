use leptos::prelude::*;

use super::placeholder::Placeholder;
use crate::components::icon::Glyph;

#[component]
pub fn NotesPage() -> impl IntoView {
    view! {
        <Placeholder
            title="Notizen"
            icon=Glyph::Notes
            phase="Kommt in Phase 3"
            hint="Markdown-Notizen mit Pins, Farben und Verlinkungen."
        />
    }
}

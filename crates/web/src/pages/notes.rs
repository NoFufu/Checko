use leptos::prelude::*;

use super::placeholder::Placeholder;
use crate::components::mascot::Mood;

#[component]
pub fn NotesPage() -> impl IntoView {
    view! {
        <Placeholder
            title="Notizen"
            emoji="📝"
            phase="Kommt in Phase 3"
            hint="Markdown-Notizen mit Pins, Farben und Verlinkungen."
            mood=Mood::Blink
        />
    }
}

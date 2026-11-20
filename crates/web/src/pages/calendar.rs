use leptos::prelude::*;

use super::placeholder::Placeholder;
use crate::components::mascot::Mood;

#[component]
pub fn CalendarPage() -> impl IntoView {
    view! {
        <Placeholder
            title="Kalender"
            emoji="📅"
            phase="Kommt in Phase 6"
            hint="Monat, Woche und Agenda mit Terminen und Wiederholungen."
            mood=Mood::WalkRight
        />
    }
}

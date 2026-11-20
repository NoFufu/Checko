use leptos::prelude::*;

use super::placeholder::Placeholder;
use crate::components::{icon::Glyph, mascot::Mood};

#[component]
pub fn CalendarPage() -> impl IntoView {
    view! {
        <Placeholder
            title="Kalender"
            icon=Glyph::Calendar
            phase="Kommt in Phase 6"
            hint="Monat, Woche und Agenda mit Terminen und Wiederholungen."
            mood=Mood::WalkRight
        />
    }
}

use leptos::prelude::*;

use super::placeholder::Placeholder;
use crate::components::icon::Glyph;

#[component]
pub fn TasksPage() -> impl IntoView {
    view! {
        <Placeholder
            title="Aufgaben"
            icon=Glyph::Tasks
            phase="Kommt in Phase 1"
            hint="Aufgaben anlegen, priorisieren und mit einem federnden Häkchen abhaken – dafür gibt es XP."
        />
    }
}

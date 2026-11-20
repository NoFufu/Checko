use leptos::prelude::*;

use super::placeholder::Placeholder;
use crate::components::{icon::Glyph, mascot::Mood};

#[component]
pub fn ShoppingPage() -> impl IntoView {
    view! {
        <Placeholder
            title="Einkauf"
            icon=Glyph::Shopping
            phase="Kommt in Phase 3"
            hint="Einkaufsliste nach Kategorien – abhaken per Wisch-Geste."
            mood=Mood::Crawl
        />
    }
}

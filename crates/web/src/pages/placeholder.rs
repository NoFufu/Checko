use leptos::prelude::*;

use crate::components::{
    blur_text::BlurText,
    icon::{Glyph, Icon},
    spotlight_card::SpotlightCard,
};

/// Friendly "coming soon" page used until a module is built.
#[component]
pub fn Placeholder(
    title: &'static str,
    icon: Glyph,
    phase: &'static str,
    hint: &'static str,
) -> impl IntoView {
    view! {
        <h1 class="page-title flex items-center gap-3">
            <span class="module-icon"><Icon glyph=icon size=32 /></span>
            <BlurText text=title />
        </h1>
        <SpotlightCard class="mt-8 flex flex-col items-center gap-4 py-14 text-center">
            <span class="module-icon placeholder-icon"><Icon glyph=icon size=48 /></span>
            <p class="text-lg font-bold">"Hier entsteht gerade etwas …"</p>
            <p class="max-w-md text-muted">{hint}</p>
            <span class="chip">{phase}</span>
        </SpotlightCard>
    }
}

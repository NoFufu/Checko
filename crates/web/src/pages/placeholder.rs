use leptos::prelude::*;

use crate::components::{
    blur_text::BlurText,
    icon::{Glyph, Icon},
    mascot::{Mascot, Mood},
    spotlight_card::SpotlightCard,
};

/// Friendly "coming soon" page used until a module is built.
#[component]
pub fn Placeholder(
    title: &'static str,
    icon: Glyph,
    phase: &'static str,
    hint: &'static str,
    mood: Mood,
) -> impl IntoView {
    view! {
        <h1 class="page-title flex items-center gap-3">
            <span class="module-icon"><Icon glyph=icon size=32 /></span>
            <BlurText text=title />
        </h1>
        <SpotlightCard class="mt-8 flex flex-col items-center gap-4 py-14 text-center">
            <Mascot mood=Signal::stored(mood) size=150 />
            <p class="text-lg font-bold">"Checko baut hier noch fleißig …"</p>
            <p class="max-w-md text-muted">{hint}</p>
            <span class="chip">{phase}</span>
        </SpotlightCard>
    }
}

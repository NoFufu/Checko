use leptos::prelude::*;

use crate::components::{
    icon::{Glyph, Icon},
    topbar::Topbar,
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
        <Topbar title=title icon=icon />
        <div class="page">
            <section class="panel empty-panel">
                <span class="empty-icon"><Icon glyph=icon size=28 /></span>
                <h1 class="text-xl font-semibold">{title}</h1>
                <p class="max-w-md text-muted">{hint}</p>
                <span class="chip">{phase}</span>
            </section>
        </div>
    }
}

use leptos::prelude::*;

use super::icon::{Glyph, Icon};

/// Page header: title on the left, notifications and "Neu" on the right.
#[component]
pub fn Topbar(title: &'static str, icon: Glyph) -> impl IntoView {
    view! {
        <header class="topbar">
            <div class="crumb">
                <Icon glyph=icon size=16 />
                <span>{title}</span>
            </div>
            <div class="flex items-center gap-2">
                <button type="button" class="btn-icon" aria-label="Benachrichtigungen" title="Erinnerungen kommen in Phase 6">
                    <Icon glyph=Glyph::Bell size=18 />
                </button>
                <a class="btn btn-primary" href=format!("{}/aufgaben", crate::BASE)>
                    <Icon glyph=Glyph::Add size=16 />
                    "Neu"
                </a>
            </div>
        </header>
    }
}

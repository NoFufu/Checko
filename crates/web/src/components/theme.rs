use leptos::prelude::*;

use super::icon::{Glyph, Icon};

const KEY: &str = "checko-theme";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    fn as_str(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    fn parse(s: &str) -> Self {
        match s {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::System,
        }
    }
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn apply(theme: Theme) {
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let _ = el.set_attribute("data-theme", theme.as_str());
    }
}

/// Light / Dark / System switch. The choice is remembered in this browser.
#[component]
pub fn ThemeSwitch() -> impl IntoView {
    let initial = storage()
        .and_then(|s| s.get_item(KEY).ok().flatten())
        .map(|v| Theme::parse(&v))
        .unwrap_or(Theme::System);
    let theme = RwSignal::new(initial);
    Effect::new(move |_| {
        let t = theme.get();
        apply(t);
        if let Some(s) = storage() {
            let _ = s.set_item(KEY, t.as_str());
        }
    });

    let option = move |t: Theme, glyph: Glyph, label: &'static str| {
        view! {
            <button
                type="button"
                class="theme-option"
                class:active=move || theme.get() == t
                aria-pressed=move || (theme.get() == t).to_string()
                on:click=move |_| theme.set(t)
            >
                <Icon glyph=glyph size=14 />
                {label}
            </button>
        }
    };

    view! {
        <div class="theme-switch" role="group" aria-label="Farbschema">
            {option(Theme::Light, Glyph::Sun, "Hell")}
            {option(Theme::Dark, Glyph::Moon, "Dunkel")}
            {option(Theme::System, Glyph::Monitor, "System")}
        </div>
    }
}

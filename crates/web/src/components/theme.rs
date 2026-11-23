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

fn system_dark() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .is_some_and(|m| m.matches())
}

/// Two small icons (sun / moon). Until you pick one, Checko follows the system.
/// The chosen theme, shared by every switch on the page.
#[derive(Clone, Copy)]
pub struct ThemeState(RwSignal<Theme>);

/// Call once at app start: loads the saved choice and keeps it applied + saved.
pub fn provide_theme() {
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
    provide_context(ThemeState(theme));
}

#[component]
pub fn ThemeSwitch() -> impl IntoView {
    let theme = expect_context::<ThemeState>().0;
    let dark = move || match theme.get() {
        Theme::Dark => true,
        Theme::Light => false,
        Theme::System => system_dark(),
    };

    let option = move |t: Theme, glyph: Glyph, label: &'static str| {
        let active = move || dark() == (t == Theme::Dark);
        view! {
            <button
                type="button"
                class="theme-option"
                class:active=active
                aria-pressed=move || active().to_string()
                aria-label=label
                title=label
                on:click=move |_| theme.set(t)
            >
                <Icon glyph=glyph size=15 />
            </button>
        }
    };

    view! {
        <div class="theme-switch" role="group" aria-label="Farbschema">
            {option(Theme::Light, Glyph::Sun, "Hell")}
            {option(Theme::Dark, Glyph::Moon, "Dunkel")}
        </div>
    }
}

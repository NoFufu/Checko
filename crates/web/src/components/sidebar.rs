use leptos::prelude::*;
use leptos_router::components::A;

use super::icon::{Glyph, Icon};
use super::theme::ThemeSwitch;
use crate::{asset, BASE};

struct Item {
    href: &'static str,
    label: &'static str,
    icon: Glyph,
}

const MAIN: [Item; 5] = [
    Item {
        href: "/",
        label: "Heute",
        icon: Glyph::Today,
    },
    Item {
        href: "/aufgaben",
        label: "Aufgaben",
        icon: Glyph::Tasks,
    },
    Item {
        href: "/kalender",
        label: "Kalender",
        icon: Glyph::Calendar,
    },
    Item {
        href: "/notizen",
        label: "Notizen",
        icon: Glyph::Notes,
    },
    Item {
        href: "/einkauf",
        label: "Einkauf",
        icon: Glyph::Shopping,
    },
];

/// Life areas used to group tasks (colour dot + name). Editable later.
const AREAS: [(&str, &str); 3] = [
    ("Arbeit", "var(--lavender)"),
    ("Privat", "var(--green)"),
    ("Gesundheit", "var(--peach)"),
];

fn nav_link(it: &'static Item) -> impl IntoView {
    view! {
        <A href=format!("{BASE}{}", it.href) exact=true attr:class="nav-item">
            <Icon glyph=it.icon size=18 />
            <span class="nav-label">{it.label}</span>
        </A>
    }
}

/// Main navigation: brand, search, sections, settings and profile card.
/// On small screens only the main links are shown, as a bottom bar.
#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar" aria-label="Seitenleiste">
            <a href=format!("{BASE}/") class="sidebar-brand">
                <span class="brand-mark"><img src=asset("brand/icon.png") alt="" /></span>
                <span>
                    <span class="brand-name">"Checko"</span>
                    <span class="brand-sub">"Organizer"</span>
                </span>
            </a>

            <button type="button" class="sidebar-search" title="Suche kommt in einer späteren Phase">
                <Icon glyph=Glyph::Search size=16 />
                <span>"Suchen"</span>
                <kbd>"Strg K"</kbd>
            </button>

            <nav class="nav-section" aria-label="Hauptnavigation">
                <p class="nav-heading">"Übersicht"</p>
                {MAIN.iter().map(nav_link).collect_view()}
            </nav>

            <details class="nav-section nav-collapsible" open>
                <summary class="nav-heading-row">
                    "Bereiche"
                    <Icon glyph=Glyph::ChevronDown size=16 />
                </summary>
                <ul class="area-list">
                    {AREAS
                        .iter()
                        .map(|(name, color)| {
                            view! {
                                <li class="area-item">
                                    <span class="area-dot" style=format!("background:{color}")></span>
                                    {*name}
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </details>

            <div class="sidebar-bottom">
                <A href=format!("{BASE}/einstellungen") attr:class="nav-item">
                    <Icon glyph=Glyph::Settings size=18 />
                    <span class="nav-label">"Einstellungen"</span>
                </A>
                <div class="profile-card">
                    <div class="profile-row">
                        <span class="avatar">"Du"</span>
                        <span class="min-w-0">
                            <span class="profile-name">"Mein Checko"</span>
                            <span class="profile-sub">"Lokal gespeichert"</span>
                        </span>
                    </div>
                    <ThemeSwitch />
                </div>
            </div>
        </aside>
    }
}

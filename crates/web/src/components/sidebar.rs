use leptos::prelude::*;
use leptos_router::components::A;

use super::icon::{Glyph, Icon};
use super::logo::Logo;
use super::theme::ThemeSwitch;
use crate::store::Store;
use crate::time::today;
use crate::ui::area_color;
use crate::BASE;
use checko_core::{open_tasks, today_tasks};

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

fn nav_link(it: &'static Item, store: Store) -> impl IntoView {
    // Small counters: tasks for today on "Heute", open tasks on "Aufgaben".
    let count = move || -> usize {
        store.tasks.with(|t| match it.href {
            "/" => today_tasks(t, None, today()).len(),
            "/aufgaben" => open_tasks(t, None).len(),
            _ => 0,
        })
    };
    view! {
        <A href=format!("{BASE}{}", it.href) exact=true attr:class="nav-item">
            <Icon glyph=it.icon size=18 />
            <span class="nav-label">{it.label}</span>
            <Show when=move || { count() > 0 }>
                <span class="nav-count">{count}</span>
            </Show>
        </A>
    }
}

/// Main navigation: brand, search, sections, settings and profile card.
/// On small screens only the main links are shown, as a bottom bar.
#[component]
pub fn Sidebar() -> impl IntoView {
    let store = Store::get();
    view! {
        <aside class="sidebar" aria-label="Seitenleiste">
            <a href=format!("{BASE}/") class="sidebar-brand" aria-label="Checko Startseite">
                <Logo />
            </a>

            <button type="button" class="sidebar-search" on:click=move |_| store.quick_add_open.set(true)>
                <Icon glyph=Glyph::Add size=16 />
                <span>"Neue Aufgabe"</span>
                <kbd>"Strg K"</kbd>
            </button>

            <nav class="nav-section" aria-label="Hauptnavigation">
                <p class="nav-heading">"Übersicht"</p>
                {MAIN.iter().map(|it| nav_link(it, store)).collect_view()}
            </nav>

            <details class="nav-section nav-collapsible" open>
                <summary class="nav-heading-row">
                    "Kategorien"
                    <Icon glyph=Glyph::ChevronDown size=16 />
                </summary>
                <ul class="area-list">
                    {move || store
                        .areas()
                        .into_iter()
                        .map(|name| {
                            let n = name.clone();
                            let count = Signal::derive(move || {
                                store.tasks.with(|t| open_tasks(t, Some(&n)).len())
                            });
                            view! {
                                <li class="area-item">
                                    <span class="area-dot" style=format!("background:{}", area_color(&name))></span>
                                    <span class="flex-1">{name.clone()}</span>
                                    <Show when=move || count.get() != 0>
                                        <span class="nav-count">{count}</span>
                                    </Show>
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

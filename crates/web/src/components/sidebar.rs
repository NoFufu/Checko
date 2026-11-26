use leptos::prelude::*;
use leptos_router::components::A;

use super::icon::{Glyph, Icon};
use super::logo::Logo;
use super::task_item::DRAG_TYPE;
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

/// Task id carried by a drag from a task row.
fn dragged_task(e: &leptos::ev::DragEvent) -> Option<uuid::Uuid> {
    e.data_transfer()?.get_data(DRAG_TYPE).ok()?.parse().ok()
}

fn nav_link(it: &'static Item, store: Store) -> impl IntoView {
    // Dropping a task on "Heute" moves it to today.
    let drop_ok = it.href == "/";
    let over = RwSignal::new(false);
    // Small counters: tasks for today on "Heute", open tasks on "Aufgaben".
    let count = move || -> usize {
        store.tasks.with(|t| match it.href {
            "/" => today_tasks(t, None, today()).len(),
            "/aufgaben" => open_tasks(t, None).len(),
            "/einkauf" => store.shopping.with(|s| checko_core::open_count(s)),
            "/notizen" => store
                .notes
                .with(|n| n.iter().filter(|n| n.meta.deleted_at.is_none()).count()),
            _ => 0,
        })
    };
    view! {
        <div
            class="nav-drop"
            class:drop-over=move || over.get()
            on:dragover=move |e| if drop_ok { e.prevent_default(); over.set(true); }
            on:dragleave=move |_| over.set(false)
            on:drop=move |e| {
                over.set(false);
                if let (true, Some(id)) = (drop_ok, dragged_task(&e)) {
                    e.prevent_default();
                    let time = store.tasks.with_untracked(|ts| ts.iter().find(|t| t.meta.id == id).and_then(|t| t.due_time));
                    store.set_due(id, Some(today()), time);
                }
            }
        >
            <A href=format!("{BASE}{}", it.href) exact=true attr:class="nav-item" attr:title=it.label>
                <Icon glyph=it.icon size=20 />
                <span class="nav-label">{it.label}</span>
                <Show when=move || { count() > 0 }>
                    <span class="nav-count">{count}</span>
                </Show>
            </A>
        </div>
    }
}

const OPEN_KEY: &str = "checko-sidebar-open";
const GROUP_KEY: &str = "checko-sidebar-categories-open";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn load_flag(key: &str, default: bool) -> bool {
    storage()
        .and_then(|s| s.get_item(key).ok().flatten())
        .map(|v| v != "false")
        .unwrap_or(default)
}

fn save_flag(key: &str, value: bool) {
    if let Some(s) = storage() {
        let _ = s.set_item(key, if value { "true" } else { "false" });
    }
}

/// Main navigation, after the Aceternity "Grouped Sidebar": a white card on a
/// grey frame, plain links, a divider, collapsible groups with indented items
/// and a small arrow button on the edge that collapses it to a 70 px icon rail.
/// Open/closed and the group state are remembered in this browser.
/// On small screens only the main links are shown, as a bottom bar.
#[component]
pub fn Sidebar() -> impl IntoView {
    let store = Store::get();
    let open = RwSignal::new(load_flag(OPEN_KEY, true));
    let cats_open = RwSignal::new(load_flag(GROUP_KEY, true));
    Effect::new(move |_| save_flag(OPEN_KEY, open.get()));
    Effect::new(move |_| save_flag(GROUP_KEY, cats_open.get()));
    view! {
        <aside class="sidebar" class:open=move || open.get() class:collapsed=move || !open.get() aria-label="Seitenleiste">
            <button
                type="button"
                class="sidebar-toggle"
                aria-label=move || if open.get() { "Seitenleiste einklappen" } else { "Seitenleiste ausklappen" }
                aria-expanded=move || open.get().to_string()
                title=move || if open.get() { "Einklappen" } else { "Ausklappen" }
                on:click=move |_| open.update(|o| *o = !*o)
            >
                <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
                    <path d="M5 12h14M5 12l4 4M5 12l4-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
            <div class="sidebar-scroll">
                <a href=format!("{BASE}/") class="sidebar-brand" aria-label="Checko Startseite">
                    <img class="brand-mark" src=crate::asset("brand/gecko-mark.png") alt="" width="24" height="24" />
                    <Logo />
                </a>

                <nav class="nav-section" aria-label="Hauptnavigation">
                    <button type="button" class="sidebar-search nav-item" title="Suchen / Neu (Strg K)" on:click=move |_| store.quick_add_open.set(true)>
                        <Icon glyph=Glyph::Search size=20 />
                        <span class="nav-label">"Suchen"</span>
                        <kbd>"Strg K"</kbd>
                    </button>
                    {MAIN.iter().map(|it| nav_link(it, store)).collect_view()}
                </nav>

                <div class="sidebar-divider" role="separator"></div>

                <div class="nav-group nav-collapsible">
                    <button
                        type="button"
                        class="group-head"
                        aria-expanded=move || cats_open.get().to_string()
                        title="Kategorien"
                        on:click=move |_| cats_open.update(|o| *o = !*o)
                    >
                        <Icon glyph=Glyph::Pin size=20 />
                        <span class="nav-label">"Kategorien"</span>
                        <span class="group-chevron" class:closed=move || !cats_open.get()><Icon glyph=Glyph::ChevronDown size=16 /></span>
                    </button>
                    <Show when=move || cats_open.get()>
                        <ul class="group-items area-list">
                            {move || store
                                .areas()
                                .into_iter()
                                .map(|name| view! { <CategoryLink name=name /> })
                                .collect_view()}
                        </ul>
                    </Show>
                </div>
            </div>
        </aside>
    }
}

/// One category inside the "Kategorien" group: click to filter the tasks,
/// drop a task onto it to file it there.
#[component]
fn CategoryLink(name: String) -> impl IntoView {
    let store = Store::get();
    let over = RwSignal::new(false);
    let n = name.clone();
    let count = Memo::new(move |_| store.tasks.with(|t| open_tasks(t, Some(&n)).len()));
    let href = format!(
        "{BASE}/aufgaben?kategorie={}",
        js_sys::encode_uri_component(&name)
    );
    let drop_name = name.clone();
    let style = format!("--c:{}", area_color(&name));
    let icon = crate::ui::category_icon(&name);
    let title = name.clone();
    view! {
        <li
            class="folder"
            class:drop-over=move || over.get()
            on:dragover=move |e| { e.prevent_default(); over.set(true); }
            on:dragleave=move |_| over.set(false)
            on:drop=move |e| {
                e.prevent_default();
                over.set(false);
                if let Some(id) = dragged_task(&e) {
                    store.add_category(id, &drop_name);
                }
            }
        >
            <a href=href class="folder-link nav-item" title=title style=style>
                <span class="cat-icon">{icon}</span>
                <span class="nav-label truncate">{name}</span>
                <Show when=move || count.get() != 0>
                    <span class="nav-count">{count}</span>
                </Show>
            </a>
        </li>
    }
}

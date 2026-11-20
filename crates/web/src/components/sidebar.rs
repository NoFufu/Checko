use leptos::prelude::*;
use leptos_router::components::A;

use super::icon::{Glyph, Icon};
use super::logo::Logo;
use super::task_item::DRAG_TYPE;
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
            <A href=format!("{BASE}{}", it.href) exact=true attr:class="nav-item">
                <Icon glyph=it.icon size=18 />
                <span class="nav-label">{it.label}</span>
                <Show when=move || { count() > 0 }>
                    <span class="nav-count">{count}</span>
                </Show>
            </A>
        </div>
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
                <Icon glyph=Glyph::Search size=16 />
                <span>"Suchen / Neu"</span>
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
                        .map(|name| view! { <CategoryFolder name=name /> })
                        .collect_view()}
                </ul>
                <p class="area-hint">"Aufgaben hierher ziehen, um sie einzuordnen."</p>
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

/// One category in the sidebar: a folder you can open to see its tasks,
/// click to filter, and drop tasks onto.
#[component]
fn CategoryFolder(name: String) -> impl IntoView {
    let store = Store::get();
    let open = RwSignal::new(false);
    let over = RwSignal::new(false);
    let n = name.clone();
    let tasks = Memo::new(move |_| {
        store.tasks.with(|t| {
            open_tasks(t, Some(&n))
                .into_iter()
                .map(|t| (t.meta.id, t.title.clone()))
                .collect::<Vec<_>>()
        })
    });
    let count = move || tasks.with(Vec::len);
    let href = format!(
        "{BASE}/aufgaben?kategorie={}",
        js_sys::encode_uri_component(&name)
    );
    let drop_name = name.clone();

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
                    open.set(true);
                }
            }
        >
            <div class="area-item">
                <button
                    type="button"
                    class="folder-toggle"
                    class:open=move || open.get()
                    aria-expanded=move || open.get().to_string()
                    aria-label=format!("{name} auf- oder zuklappen")
                    disabled=move || count() == 0
                    on:click=move |_| open.update(|o| *o = !*o)
                >
                    <Icon glyph=Glyph::ChevronDown size=14 />
                </button>
                <A href=href attr:class="folder-link">
                    <span class="area-dot" style=format!("background:{}", area_color(&name))></span>
                    <span class="flex-1 truncate">{name.clone()}</span>
                </A>
                <Show when=move || count() != 0>
                    <span class="nav-count">{count}</span>
                </Show>
            </div>
            <Show when=move || { open.get() && count() > 0 }>
                <ul class="folder-tasks">
                    {move || tasks
                        .get()
                        .into_iter()
                        .take(8)
                        .map(|(id, title)| view! {
                            <li>
                                <button type="button" class="folder-task" on:click=move |_| store.detail.set(Some(id))>
                                    {title}
                                </button>
                            </li>
                        })
                        .collect_view()}
                </ul>
            </Show>
        </li>
    }
}

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
                <Icon glyph=it.icon size=18 />
                <span class="nav-label">{it.label}</span>
                <Show when=move || { count() > 0 }>
                    <span class="nav-count">{count}</span>
                </Show>
            </A>
        </div>
    }
}

/// Is the keyboard focus still somewhere inside the sidebar?
fn focus_in_sidebar() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element())
        .and_then(|el| el.closest(".sidebar").ok().flatten())
        .is_some()
}

/// Main navigation (port of the Aceternity "Sidebar"): on the desktop it is a
/// 60 px icon rail that opens to 300 px while the mouse is over it (also on
/// keyboard focus and while dragging a task onto it) and pushes the page aside.
/// On small screens only the main links are shown, as a bottom bar.
#[component]
pub fn Sidebar() -> impl IntoView {
    let store = Store::get();
    let open = RwSignal::new(false);
    view! {
        <aside
            class="sidebar"
            class:open=move || open.get()
            aria-label="Seitenleiste"
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| if !focus_in_sidebar() { open.set(false) }
            on:focusin=move |_| open.set(true)
            on:focusout=move |_| {
                // the newly focused element is only known after this event
                set_timeout(move || if !focus_in_sidebar() { open.set(false) }, std::time::Duration::ZERO);
            }
            on:dragenter=move |_| open.set(true)
            on:drop=move |_| open.set(false)
        >
            <a href=format!("{BASE}/") class="sidebar-brand" aria-label="Checko Startseite">
                <img class="brand-mark" src=crate::asset("brand/icon-192.png") alt="" width="28" height="28" />
                <Logo />
            </a>

            <button type="button" class="sidebar-search" title="Suchen / Neu (Strg K)" on:click=move |_| store.quick_add_open.set(true)>
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
    let row_style = format!("--c:{}", area_color(&name));
    let icon = crate::ui::category_icon(&name);
    let toggle_label = format!("{name} auf- oder zuklappen");
    let label = name.clone();

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
            <div class="folder-row" style=row_style>
                <A href=href attr:class="folder-link" attr:title=name.clone()>
                    <span class="cat-icon">{icon}</span>
                    <span class="flex-1 truncate">{label}</span>
                </A>
                <Show when=move || count() != 0>
                    <span class="nav-count">{count}</span>
                </Show>
                <button
                    type="button"
                    class="folder-toggle"
                    class:open=move || open.get()
                    aria-expanded=move || open.get().to_string()
                    aria-label=toggle_label
                    disabled=move || count() == 0
                    on:click=move |_| open.update(|o| *o = !*o)
                >
                    <Icon glyph=Glyph::ChevronDown size=14 />
                </button>
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

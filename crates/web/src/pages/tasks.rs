use checko_core::{done_tasks, open_appointments, open_tasks, open_todos, trashed_tasks};
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::components::{
    area_filter::AreaFilter,
    icon::{Glyph, Icon},
    task_form::TaskForm,
    task_item::TaskItem,
    topbar::Topbar,
};
use crate::store::Store;

#[derive(Clone, Copy, PartialEq)]
enum View {
    Open,
    Done,
    Trash,
}

#[component]
pub fn TasksPage() -> impl IntoView {
    let store = Store::get();
    let view_tab = RwSignal::new(View::Open);
    // None = all areas
    let area = RwSignal::new(None::<String>);
    // "?kategorie=Bad" (from a category block or the sidebar) sets the filter.
    let query = use_query_map();
    Effect::new(move |_| {
        if let Some(k) = query.with(|q| q.get("kategorie")) {
            area.set(Some(k));
            view_tab.set(View::Open);
        }
    });

    let open_count = move || store.tasks.with(|t| open_tasks(t, None).len());
    let done_count = move || store.tasks.with(|t| done_tasks(t, None).len());
    let trash_count = move || store.tasks.with(|t| trashed_tasks(t).len());

    let list = move || {
        let a = area.get();
        store.tasks.with(|ts| {
            let items: Vec<_> = match view_tab.get() {
                View::Open => open_todos(ts, a.as_deref()),
                View::Done => done_tasks(ts, a.as_deref()),
                View::Trash => trashed_tasks(ts),
            }
            .into_iter()
            .cloned()
            .collect();
            items
        })
    };

    let appointments = move || {
        let a = area.get();
        if view_tab.get() != View::Open {
            return Vec::new();
        }
        store.tasks.with(|ts| {
            open_appointments(ts, a.as_deref())
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        })
    };

    let tab = move |v: View, label: &'static str, count: Box<dyn Fn() -> usize + Send + Sync>| {
        view! {
            <button
                type="button"
                role="tab"
                class="segment"
                class:active=move || view_tab.get() == v
                aria-selected=move || (view_tab.get() == v).to_string()
                on:click=move |_| view_tab.set(v)
            >
                {label}
                <span class="count">{count}</span>
            </button>
        }
    };

    view! {
        <Topbar />
        <div class="page page-wide">
            <div class="page-head">
                <div>
                    <h1 class="page-h1">"Aufgaben"</h1>
                    <p class="text-muted">
                        {move || match open_count() {
                            0 => "Alles erledigt. Zeit für etwas Neues!".to_string(),
                            1 => "1 offene Aufgabe".to_string(),
                            n => format!("{n} offene Aufgaben"),
                        }}
                    </p>
                </div>
            </div>

            <section class="panel">
                <TaskForm autofocus=true area=Signal::derive(move || area.get()) />
            </section>

            <div class="toolbar">
                <div class="segmented" role="tablist" aria-label="Ansicht">
                    {tab(View::Open, "Offen", Box::new(open_count))}
                    {tab(View::Done, "Erledigt", Box::new(done_count))}
                    {tab(View::Trash, "Papierkorb", Box::new(trash_count))}
                </div>
                <Show when=move || view_tab.get() != View::Trash>
                    <AreaFilter area=area />
                </Show>
                <Show when=move || { view_tab.get() == View::Trash && trash_count() > 0 }>
                    <button
                        type="button"
                        class="btn btn-outline"
                        on:click=move |_| {
                            let ok = web_sys::window()
                                .and_then(|w| w.confirm_with_message("Papierkorb endgültig leeren?").ok())
                                .unwrap_or(false);
                            if ok {
                                store.empty_trash();
                            }
                        }
                    >
                        <Icon glyph=Glyph::Trash size=16 />
                        "Papierkorb leeren"
                    </button>
                </Show>
            </div>

            <div class="task-columns" class:split=move || view_tab.get() == View::Open>
                <div class="task-col">
                    <Show when=move || view_tab.get() == View::Open>
                        <h2 class="list-heading">
                            <Icon glyph=Glyph::Tasks size=16 />
                            "To-dos"
                        </h2>
                    </Show>
                    <section class="panel list-panel">
                        <Show
                            when=move || !list().is_empty()
                            fallback=move || {
                                let text = match view_tab.get() {
                                    View::Open => "Keine offenen To-dos. Füge oben eins hinzu!",
                                    View::Done => "Noch nichts erledigt. Das erste Häkchen wartet!",
                                    View::Trash => "Der Papierkorb ist leer.",
                                };
                                view! { <p class="empty-line px-3">{text}</p> }
                            }
                        >
                            <ul class="task-list">
                                <For
                                    each=list
                                    key=|t| (t.meta.id, t.meta.updated_at)
                                    children=move |t| view! { <TaskItem task=t trash=view_tab.get_untracked() == View::Trash /> }
                                />
                            </ul>
                        </Show>
                    </section>
                </div>
                <Show when=move || view_tab.get() == View::Open>
                    <div class="task-col">
                        <h2 class="list-heading">
                            <Icon glyph=Glyph::Calendar size=16 />
                            "Termine"
                        </h2>
                        <section class="panel list-panel appt-panel">
                            <Show
                                when=move || !appointments().is_empty()
                                fallback=|| view! { <p class="empty-line px-3">"Keine Termine. Alles mit Uhrzeit landet hier."</p> }
                            >
                                <ul class="task-list">
                                    <For
                                        each=appointments
                                        key=|t| (t.meta.id, t.meta.updated_at)
                                        children=move |t| view! { <TaskItem task=t /> }
                                    />
                                </ul>
                            </Show>
                        </section>
                    </div>
                </Show>
            </div>
        </div>
    }
}

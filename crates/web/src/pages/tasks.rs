use checko_core::{
    done_tasks, my_day_suggestions, my_day_tasks, open_appointments, open_tasks, open_todos,
    planned_groups, trashed_tasks, PlanGroup,
};
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
    MyDay,
    Open,
    Planned,
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
        match query.with(|q| q.get("ansicht")).as_deref() {
            Some("geplant") => view_tab.set(View::Planned),
            Some("meintag") => view_tab.set(View::MyDay),
            _ => {}
        }
        if let Some(k) = query.with(|q| q.get("kategorie")) {
            area.set(Some(k));
            view_tab.set(View::Open);
        }
    });

    let open_count = move || store.tasks.with(|t| open_tasks(t, None).len());
    let done_count = move || store.tasks.with(|t| done_tasks(t, None).len());
    let planned = move || {
        let a = area.get();
        store.tasks.with(|ts| {
            planned_groups(ts, a.as_deref(), crate::time::today())
                .into_iter()
                .map(|(g, v)| (g, v.into_iter().cloned().collect::<Vec<_>>()))
                .collect::<Vec<_>>()
        })
    };
    let planned_count = move || planned().iter().map(|(_, v)| v.len()).sum::<usize>();
    let my_day = move || {
        store.tasks.with(|ts| {
            my_day_tasks(ts, crate::time::today())
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        })
    };
    let my_day_open = move || my_day().iter().filter(|t| t.is_open()).count();
    let suggestions = move || {
        store.tasks.with(|ts| {
            my_day_suggestions(ts, crate::time::today())
                .into_iter()
                .map(|(s, t)| (s, t.clone()))
                .collect::<Vec<_>>()
        })
    };
    let trash_count = move || store.tasks.with(|t| trashed_tasks(t).len());

    let list = move || {
        let a = area.get();
        store.tasks.with(|ts| {
            let items: Vec<_> = match view_tab.get() {
                View::Open => open_todos(ts, a.as_deref()),
                View::Planned | View::MyDay => Vec::new(),
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
                    {tab(View::MyDay, "Mein Tag", Box::new(my_day_open))}
                    {tab(View::Open, "Offen", Box::new(open_count))}
                    {tab(View::Planned, "Geplant", Box::new(planned_count))}
                    {tab(View::Done, "Erledigt", Box::new(done_count))}
                    {tab(View::Trash, "Papierkorb", Box::new(trash_count))}
                </div>
                <Show when=move || !matches!(view_tab.get(), View::Trash | View::MyDay)>
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

            <Show when=move || view_tab.get() == View::MyDay>
                <div class="planned my-day">
                    <section class="plan-group">
                        <h2 class="list-heading">
                            <Icon glyph=Glyph::Sun size=16 />
                            {crate::time::today_label}
                        </h2>
                        <section class="panel list-panel">
                            <Show
                                when=move || !my_day().is_empty()
                                fallback=|| view! { <p class="empty-line px-3">"Dein Tag ist noch leer. Nimm unten Vorschläge dazu oder öffne eine Aufgabe und tippe „Zu Mein Tag“."</p> }
                            >
                                <ul class="task-list">
                                    <For
                                        each=my_day
                                        key=|t| (t.meta.id, t.meta.updated_at)
                                        children=move |t| view! { <TaskItem task=t /> }
                                    />
                                </ul>
                            </Show>
                        </section>
                    </section>
                    <Show when=move || !suggestions().is_empty()>
                        <section class="plan-group">
                            <h2 class="list-heading">
                                "Vorschläge"
                                <span class="count">{move || suggestions().len()}</span>
                            </h2>
                            <section class="panel list-panel">
                                <ul class="suggest-list">
                                    <For
                                        each=suggestions
                                        key=|(s, t)| (*s as u8, t.meta.id, t.meta.updated_at)
                                        children=move |(s, t)| {
                                            let id = t.meta.id;
                                            let title = t.title.clone();
                                            view! {
                                                <li class="suggest-row">
                                                    <button
                                                        type="button"
                                                        class="suggest-title"
                                                        on:click=move |_| store.detail.set(Some(id))
                                                    >
                                                        <span>{t.title.clone()}</span>
                                                        <span class="suggest-why" class:overdue=s == checko_core::Suggestion::Overdue>{s.label()}</span>
                                                    </button>
                                                    <button
                                                        type="button"
                                                        class="btn-icon"
                                                        aria-label=format!("„{title}“ zu Mein Tag")
                                                        title="Zu Mein Tag"
                                                        on:click=move |_| store.toggle_my_day(id)
                                                    >
                                                        <Icon glyph=Glyph::Add size=18 />
                                                    </button>
                                                </li>
                                            }
                                        }
                                    />
                                </ul>
                            </section>
                        </section>
                    </Show>
                </div>
            </Show>

            <Show when=move || view_tab.get() == View::Planned>
                <div class="planned">
                    <Show
                        when=move || planned_count() != 0
                        fallback=|| view! { <section class="panel"><p class="empty-line px-3">"Nichts geplant. Gib einer Aufgabe ein Datum, z. B. „Steuer machen nächste Woche“."</p></section> }
                    >
                        <For
                            each=planned
                            key=|(g, v)| (*g as u8, v.iter().map(|t| (t.meta.id, t.meta.updated_at)).collect::<Vec<_>>())
                            children=move |(g, v)| view! {
                                <section class="plan-group" class:overdue=g == PlanGroup::Overdue>
                                    <h2 class="list-heading">
                                        {g.label()}
                                        <span class="count">{v.len()}</span>
                                    </h2>
                                    <section class="panel list-panel">
                                        <ul class="task-list">
                                            {v.into_iter().map(|t| view! { <TaskItem task=t /> }).collect_view()}
                                        </ul>
                                    </section>
                                </section>
                            }
                        />
                    </Show>
                </div>
            </Show>

            <Show when=move || !matches!(view_tab.get(), View::Planned | View::MyDay)>
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
                                    View::Open | View::Planned | View::MyDay => "Keine offenen To-dos. Füge oben eins hinzu!",
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
            </Show>
        </div>
    }
}

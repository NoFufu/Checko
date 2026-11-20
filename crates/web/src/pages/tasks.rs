use checko_core::{done_tasks, open_tasks, trashed_tasks, AREAS};
use leptos::prelude::*;

use crate::components::{
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

    let open_count = move || store.tasks.with(|t| open_tasks(t, None).len());
    let done_count = move || store.tasks.with(|t| done_tasks(t, None).len());
    let trash_count = move || store.tasks.with(|t| trashed_tasks(t).len());

    let list = move || {
        let a = area.get();
        store.tasks.with(|ts| {
            let items: Vec<_> = match view_tab.get() {
                View::Open => open_tasks(ts, a.as_deref()),
                View::Done => done_tasks(ts, a.as_deref()),
                View::Trash => trashed_tasks(ts),
            }
            .into_iter()
            .cloned()
            .collect();
            items
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
        <Topbar title="Aufgaben" icon=Glyph::Tasks />
        <div class="page">
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
                    <div class="segmented" role="tablist" aria-label="Bereich filtern">
                        <button
                            type="button"
                            class="segment"
                            class:active=move || area.get().is_none()
                            on:click=move |_| area.set(None)
                        >
                            "Alle"
                        </button>
                        {AREAS
                            .iter()
                            .map(|a| {
                                let a = a.to_string();
                                let a2 = a.clone();
                                let a3 = a.clone();
                                view! {
                                    <button
                                        type="button"
                                        class="segment"
                                        class:active=move || area.get().as_deref() == Some(a.as_str())
                                        on:click=move |_| area.set(Some(a2.clone()))
                                    >
                                        {a3}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
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

            <section class="panel list-panel">
                <Show
                    when=move || !list().is_empty()
                    fallback=move || {
                        let text = match view_tab.get() {
                            View::Open => "Keine offenen Aufgaben. Füge oben eine hinzu!",
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
    }
}

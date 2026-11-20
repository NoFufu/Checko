use std::time::Duration;

use checko_core::{Priority, Task, TaskStatus};
use leptos::prelude::*;

use super::icon::{Glyph, Icon};
use crate::store::Store;
use crate::time::{due_label, today};

fn priority_class(p: Priority) -> &'static str {
    match p {
        Priority::P1 => "prio prio-1",
        Priority::P2 => "prio prio-2",
        Priority::P3 => "prio prio-3",
        Priority::P4 => "prio prio-4",
    }
}

fn priority_label(p: Priority) -> &'static str {
    match p {
        Priority::P1 => "P1",
        Priority::P2 => "P2",
        Priority::P3 => "P3",
        Priority::P4 => "P4",
    }
}

fn next_priority(p: Priority) -> Priority {
    match p {
        Priority::P4 => Priority::P3,
        Priority::P3 => Priority::P2,
        Priority::P2 => Priority::P1,
        Priority::P1 => Priority::P4,
    }
}

fn area_color(area: &str) -> &'static str {
    match area {
        "Arbeit" => "var(--lavender)",
        "Privat" => "var(--green)",
        "Gesundheit" => "var(--peach)",
        _ => "var(--muted)",
    }
}

/// One task row. `trash` switches the actions to restore / delete forever.
#[component]
pub fn TaskItem(task: Task, #[prop(optional)] trash: bool) -> impl IntoView {
    let store = Store::get();
    let id = task.meta.id;
    let done = task.status == TaskStatus::Done;
    let started = task.status == TaskStatus::InProgress;
    let priority = task.priority;
    let today = today();
    let overdue = task.is_overdue(today);
    // Briefly show the check animation before the row moves away.
    let checking = RwSignal::new(false);
    let editing = RwSignal::new(false);
    let draft = RwSignal::new(task.title.clone());

    let toggle = move |_| {
        if done {
            store.reopen(id);
        } else {
            checking.set(true);
            set_timeout(move || store.complete(id), Duration::from_millis(420));
        }
    };

    let save_title = move || {
        store.rename(id, draft.get_untracked());
        editing.set(false);
    };

    let due = task.due.map(|d| {
        let mut label = due_label(d, today);
        if let Some(t) = task.due_time {
            label.push_str(&format!(" · {}", t.format("%H:%M")));
        }
        label
    });
    let tags = task.tags.clone();
    let area = task.area.clone();
    let title = task.title.clone();

    view! {
        <li class="task-row" class:done=done class:checking=move || checking.get() class:started=started>
            {(!trash).then(|| view! {
                <button
                    type="button"
                    class="check"
                    class:checked=move || done || checking.get()
                    aria-label=if done { "Wieder öffnen" } else { "Als erledigt markieren" }
                    on:click=toggle
                >
                    <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
                        <path d="M5 12.5l4.5 4.5L19 7.5" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
                    </svg>
                    <span class="spark" aria-hidden="true"></span>
                </button>
            })}

            <div class="task-body">
                <Show
                    when=move || editing.get()
                    fallback=move || {
                        let title = title.clone();
                        view! {
                            <button
                                type="button"
                                class="task-title"
                                title="Zum Umbenennen klicken"
                                disabled=trash || done
                                on:click=move |_| editing.set(true)
                            >
                                {title}
                            </button>
                        }
                    }
                >
                    <input
                        class="task-title-input"
                        aria-label="Titel bearbeiten"
                        autofocus=true
                        bind:value=draft
                        on:keydown=move |ev| match ev.key().as_str() {
                            "Enter" => save_title(),
                            "Escape" => editing.set(false),
                            _ => {}
                        }
                        on:blur=move |_| save_title()
                    />
                </Show>
                <div class="task-meta">
                    {started.then(|| view! { <span class="status-chip">"In Arbeit"</span> })}
                    {due.map(|d| view! {
                        <span class="meta-item" class:overdue=overdue>
                            <Icon glyph=Glyph::Calendar size=13 />
                            {d}
                        </span>
                    })}
                    {tags
                        .into_iter()
                        .map(|t| view! { <span class="meta-item">"#" {t}</span> })
                        .collect_view()}
                    {area.map(|a| view! {
                        <span class="meta-item">
                            <span class="area-dot" style=format!("background:{}", area_color(&a))></span>
                            {a}
                        </span>
                    })}
                </div>
            </div>

            <div class="task-actions">
                {if trash {
                    view! {
                        <button type="button" class="btn btn-outline btn-sm" on:click=move |_| store.restore(id)>
                            "Wiederherstellen"
                        </button>
                        <button
                            type="button"
                            class="btn btn-danger btn-sm"
                            on:click=move |_| {
                                let ok = web_sys::window()
                                    .and_then(|w| w.confirm_with_message("Diese Aufgabe endgültig löschen?").ok())
                                    .unwrap_or(false);
                                if ok {
                                    store.purge(id);
                                }
                            }
                        >
                            "Endgültig löschen"
                        </button>
                    }
                        .into_any()
                } else {
                    view! {
                        <button
                            type="button"
                            class=priority_class(priority)
                            title="Priorität ändern"
                            disabled=done
                            on:click=move |_| store.set_priority(id, next_priority(priority))
                        >
                            {priority_label(priority)}
                        </button>
                        {(!done).then(|| view! {
                            <button
                                type="button"
                                class="row-btn"
                                class:active=started
                                title=if started { "Pausieren" } else { "Beginnen" }
                                aria-label=if started { "Pausieren" } else { "Beginnen" }
                                on:click=move |_| store.toggle_started(id)
                            >
                                <Icon glyph=if started { Glyph::Pause } else { Glyph::Play } size=16 />
                            </button>
                        })}
                        <button
                            type="button"
                            class="row-btn"
                            title="In den Papierkorb"
                            aria-label="In den Papierkorb"
                            on:click=move |_| store.trash(id)
                        >
                            <Icon glyph=Glyph::Trash size=16 />
                        </button>
                    }
                        .into_any()
                }}
            </div>
        </li>
    }
}

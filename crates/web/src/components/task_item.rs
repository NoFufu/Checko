use std::time::Duration;

use checko_core::{Priority, Task, TaskStatus};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use super::icon::{Glyph, Icon};
use crate::store::Store;
use crate::time::{due_label, today};
use crate::ui::{area_color, priority_class, priority_name};

/// MIME type used when dragging a task onto a category or "Heute".
pub const DRAG_TYPE: &str = "application/x-checko-task";

pub fn next_priority(p: Priority) -> Priority {
    match p {
        Priority::P4 => Priority::P3,
        Priority::P3 => Priority::P2,
        Priority::P2 => Priority::P1,
        Priority::P1 => Priority::P4,
    }
}

/// Clickable category block ("#Bad"); opens the task list filtered by it.
#[component]
pub fn CategoryChip(name: String) -> impl IntoView {
    let navigate = use_navigate();
    let target = format!(
        "{}/aufgaben?kategorie={}",
        crate::BASE,
        js_sys::encode_uri_component(&name)
    );
    view! {
        <button
            type="button"
            class="cat-chip"
            style=format!("--c:{}", area_color(&name))
            title=format!("Alle Aufgaben in „{name}“")
            on:click=move |e| {
                e.stop_propagation();
                navigate(&target, Default::default());
            }
        >
            <span class="cat-chip-icon" aria-hidden="true">{crate::ui::category_icon(&name)}</span>
            {name.clone()}
        </button>
    }
}

/// One task row. Click opens the detail panel; drag it onto a category or
/// "Heute" in the sidebar. `trash` switches the actions to restore / delete.
#[component]
pub fn TaskItem(task: Task, #[prop(optional)] trash: bool) -> impl IntoView {
    let store = Store::get();
    let id = task.meta.id;
    let done = task.status == TaskStatus::Done;
    let priority = task.priority;
    let today = today();
    let overdue = task.is_overdue(today);
    // Briefly show the check animation before the row moves away.
    let checking = RwSignal::new(false);

    let toggle = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        if done {
            store.reopen(id);
        } else {
            checking.set(true);
            set_timeout(move || store.complete(id), Duration::from_millis(420));
        }
    };

    let appointment = !trash && task.is_appointment();
    // appointments show their time as a block on the left, to-dos a small date line
    let appt_time = appointment.then(|| {
        let time = task
            .due_time
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_else(|| "Ganzer Tag".into());
        let day = task.due.map(|d| due_label(d, today)).unwrap_or_default();
        (time, day)
    });
    let due = task.due.filter(|_| !appointment).map(|d| {
        let mut label = due_label(d, today);
        if let Some(t) = task.due_time {
            label.push_str(&format!(" · {}", t.format("%H:%M")));
        }
        label
    });
    let categories: Vec<String> = task.categories().into_iter().map(str::to_string).collect();
    let (steps_done, steps_total) = task.step_progress();
    let has_notes = !task.notes.trim().is_empty();
    let repeat = task.recurrence.map(|r| r.label());
    let reminder = task.remind.is_some();
    let title = task.title.clone();

    let on_drag = move |e: leptos::ev::DragEvent| {
        if let Some(dt) = e.data_transfer() {
            let _ = dt.set_data(DRAG_TYPE, &id.to_string());
            let _ = dt.set_data("text/plain", &id.to_string());
            dt.set_effect_allowed("copyMove");
        }
    };

    view! {
        <li
            class="task-row"
            class:appt=appointment
            class:done=done
            class:checking=move || checking.get()
            class:selected=move || store.detail.get() == Some(id)
            draggable=(!trash && crate::ui::fine_pointer()).then_some("true")
            on:dragstart=on_drag
            on:click=move |_| if !trash { store.detail.set(Some(id)) }
        >
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

            {appt_time.map(|(time, day)| view! {
                <span class=format!("appt-time {}", priority_class(priority).replace("urgency ", "")) class:overdue=overdue>
                    <span class="appt-clock">{time}</span>
                    <span class="appt-day">{day}</span>
                </span>
            })}
            <div class="task-body">
                <button
                    type="button"
                    class="task-title"
                    title="Details öffnen"
                    disabled=trash
                >
                    {title}
                </button>
                <div class="task-meta">
                    {due.map(|d| view! {
                        <span class="meta-item" class:overdue=overdue>
                            <Icon glyph=Glyph::Calendar size=13 />
                            {d}
                        </span>
                    })}
                    {repeat.map(|r| view! {
                        <span class="meta-item" title=r.clone()><Icon glyph=Glyph::Repeat size=13 /></span>
                    })}
                    {reminder.then(|| view! {
                        <span class="meta-item" title="Erinnerung an"><Icon glyph=Glyph::Bell size=13 /></span>
                    })}
                    {(steps_total > 0).then(|| view! {
                        <span class="meta-item" class:all-done=steps_done == steps_total>
                            <Icon glyph=Glyph::Tasks size=13 />
                            {format!("{steps_done}/{steps_total}")}
                        </span>
                    })}
                    {has_notes.then(|| view! {
                        <span class="meta-item" title="Hat Notizen"><Icon glyph=Glyph::Notes size=13 /></span>
                    })}
                    {categories.into_iter().map(|c| view! { <CategoryChip name=c /> }).collect_view()}
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
                            title=format!("Dringlichkeit: {} (klicken zum Ändern)", priority_name(priority))
                            aria-label=format!("Dringlichkeit: {}", priority_name(priority))
                            disabled=done
                            on:click=move |e| { e.stop_propagation(); store.set_priority(id, next_priority(priority)) }
                        ></button>
                        <button
                            type="button"
                            class="row-btn row-trash"
                            title="In den Papierkorb"
                            aria-label="In den Papierkorb"
                            on:click=move |e| { e.stop_propagation(); store.trash(id) }
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

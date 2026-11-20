use checko_core::{Priority, Task, TaskStatus};
use chrono::{Datelike, Duration, NaiveDate, NaiveTime};
use leptos::{ev, prelude::*};

use super::icon::{Glyph, Icon};
use super::task_item::CategoryChip;
use crate::store::Store;
use crate::time::{due_label, today};
use crate::ui::{priority_class, priority_name};

const PRIORITIES: [Priority; 4] = [Priority::P4, Priority::P3, Priority::P2, Priority::P1];

/// Side panel with everything about one task (like Microsoft To Do):
/// title, urgency, due date + time with quick moves, categories, steps, notes.
#[component]
pub fn TaskDetail() -> impl IntoView {
    let store = Store::get();
    let task = move || {
        let id = store.detail.get()?;
        store.tasks.with(|ts| {
            ts.iter()
                .find(|t| t.meta.id == id && !t.is_deleted())
                .cloned()
        })
    };
    let close = move || store.detail.set(None);

    let handle = window_event_listener(ev::keydown, move |e| {
        if e.key() == "Escape" && store.detail.get_untracked().is_some() {
            store.detail.set(None);
        }
    });
    on_cleanup(move || handle.remove());

    view! {
        <Show when=move || task().is_some()>
            <div class="drawer-backdrop" on:click=move |_| close()></div>
            <aside class="drawer" role="complementary" aria-label="Aufgabe bearbeiten">
                {move || task().map(|t| view! { <DetailBody task=t /> })}
            </aside>
        </Show>
    }
}

#[component]
fn DetailBody(task: Task) -> impl IntoView {
    let store = Store::get();
    let id = task.meta.id;
    let done = task.status == TaskStatus::Done;
    let today = today();
    let new_cat = RwSignal::new(String::new());
    let new_step = RwSignal::new(String::new());

    let move_to = move |d: Option<NaiveDate>| {
        let time = store
            .tasks
            .with_untracked(|ts| ts.iter().find(|t| t.meta.id == id).and_then(|t| t.due_time));
        store.set_due(id, d, time);
    };
    let next_monday = today + Duration::days(7 - today.weekday().num_days_from_monday() as i64);
    let due_text = task
        .due
        .map(|d| due_label(d, today))
        .unwrap_or_else(|| "Kein Termin".into());
    let current_due = task.due;
    let current_time = task.due_time;

    view! {
        <div class="drawer-head">
            <button
                type="button"
                class="check"
                class:checked=done
                aria-label=if done { "Wieder öffnen" } else { "Als erledigt markieren" }
                on:click=move |_| if done { store.reopen(id) } else { store.complete(id) }
            >
                <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
                    <path d="M5 12.5l4.5 4.5L19 7.5" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
            <input
                class="drawer-title"
                aria-label="Titel"
                prop:value=task.title.clone()
                on:change=move |e| store.rename(id, event_target_value(&e))
                on:keydown=move |e| if e.key() == "Enter" { store.rename(id, event_target_value(&e)) }
            />
            <button type="button" class="row-btn" aria-label="Schließen" title="Schließen (Esc)" on:click=move |_| store.detail.set(None)>
                "✕"
            </button>
        </div>

        <section class="drawer-section">
            <h3 class="drawer-label">"Dringlichkeit"</h3>
            <div class="urgency-picker" role="radiogroup" aria-label="Dringlichkeit">
                {PRIORITIES
                    .iter()
                    .map(|p| {
                        let p = *p;
                        let active = task.priority == p;
                        view! {
                            <button
                                type="button"
                                role="radio"
                                aria-checked=active.to_string()
                                class="urgency-option"
                                class:active=active
                                on:click=move |_| store.set_priority(id, p)
                            >
                                <span class=priority_class(p)></span>
                                {priority_name(p)}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        <section class="drawer-section">
            <h3 class="drawer-label">"Termin · " {due_text}</h3>
            <div class="quick-dates">
                <button type="button" class="btn btn-outline btn-sm" on:click=move |_| move_to(Some(today))>"Heute"</button>
                <button type="button" class="btn btn-outline btn-sm" on:click=move |_| move_to(Some(today + Duration::days(1)))>"Morgen"</button>
                <button type="button" class="btn btn-outline btn-sm" on:click=move |_| move_to(Some(next_monday))>"Nächste Woche"</button>
                {current_due.map(|d| view! {
                    <button type="button" class="btn btn-outline btn-sm" on:click=move |_| move_to(Some(d + Duration::days(1)))>"+1 Tag"</button>
                })}
                {current_due.is_some().then(|| view! {
                    <button type="button" class="btn btn-outline btn-sm" on:click=move |_| move_to(None)>"Entfernen"</button>
                })}
            </div>
            <div class="flex flex-wrap gap-2 mt-2">
                <label class="field">
                    <span class="sr-only">"Datum"</span>
                    <input
                        type="date"
                        aria-label="Datum"
                        prop:value=current_due.map(|d| d.to_string()).unwrap_or_default()
                        on:change=move |e| store.set_due(id, event_target_value(&e).parse().ok(), current_time)
                    />
                </label>
                <label class="field">
                    <span class="sr-only">"Uhrzeit"</span>
                    <input
                        type="time"
                        aria-label="Uhrzeit"
                        prop:value=current_time.map(|t| t.format("%H:%M").to_string()).unwrap_or_default()
                        on:change=move |e| {
                            let t = NaiveTime::parse_from_str(&event_target_value(&e), "%H:%M").ok();
                            store.set_due(id, current_due.or(Some(today)), t);
                        }
                    />
                </label>
            </div>
        </section>

        <section class="drawer-section">
            <h3 class="drawer-label">"Kategorien"</h3>
            <div class="cat-editor">
                {task
                    .categories()
                    .into_iter()
                    .map(|c| {
                        let name = c.to_string();
                        let rm = name.clone();
                        view! {
                            <span class="cat-chip-edit">
                                <CategoryChip name=name.clone() />
                                <button type="button" class="cat-remove" aria-label=format!("{name} entfernen") on:click=move |_| store.remove_category(id, &rm)>
                                    "×"
                                </button>
                            </span>
                        }
                    })
                    .collect_view()}
                <form class="cat-add" on:submit=move |e| {
                    e.prevent_default();
                    store.add_category(id, &new_cat.get_untracked());
                    new_cat.set(String::new());
                }>
                    <input list="checko-categories" placeholder="+ Kategorie" aria-label="Kategorie hinzufügen" bind:value=new_cat />
                    <datalist id="checko-categories">
                        {move || store.areas().into_iter().map(|a| view! { <option value=a></option> }).collect_view()}
                    </datalist>
                </form>
            </div>
        </section>

        <section class="drawer-section">
            <h3 class="drawer-label">
                "Schritte"
                {let (d, n) = task.step_progress(); (n > 0).then(|| format!(" · {d}/{n}"))}
            </h3>
            <ul class="steps">
                {task
                    .steps
                    .iter()
                    .map(|s| {
                        let sid = s.id;
                        view! {
                            <li class="step" class:done=s.done>
                                <input
                                    type="checkbox"
                                    prop:checked=s.done
                                    aria-label=format!("Schritt „{}“ erledigt", s.text)
                                    on:change=move |_| store.edit(id, |t| t.toggle_step(sid, crate::time::now()))
                                />
                                <span class="flex-1">{s.text.clone()}</span>
                                <button type="button" class="cat-remove" aria-label="Schritt löschen" on:click=move |_| store.edit(id, |t| t.remove_step(sid, crate::time::now()))>
                                    "×"
                                </button>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            <form class="step-add" on:submit=move |e| {
                e.prevent_default();
                let text = new_step.get_untracked();
                store.edit(id, |t| t.add_step(&text, crate::time::now()));
                new_step.set(String::new());
            }>
                <Icon glyph=Glyph::Add size=16 />
                <input placeholder="Schritt hinzufügen" aria-label="Schritt hinzufügen" bind:value=new_step />
            </form>
        </section>

        <section class="drawer-section">
            <h3 class="drawer-label">"Notizen"</h3>
            <textarea
                class="note-body drawer-notes"
                placeholder="Details, Links, Telefonnummern …"
                aria-label="Notizen zur Aufgabe"
                prop:value=task.notes.clone()
                on:change=move |e| {
                    let v = event_target_value(&e);
                    store.edit(id, |t| { t.notes = v; t.meta.updated_at = crate::time::now(); });
                }
            ></textarea>
        </section>

        <div class="drawer-foot">
            <span class="text-sm text-muted">
                {format!("Erstellt am {}", task.meta.created_at.format("%d.%m.%Y"))}
            </span>
            <button type="button" class="btn btn-outline btn-sm" on:click=move |_| { store.trash(id); store.detail.set(None); }>
                <Icon glyph=Glyph::Trash size=14 />
                "Löschen"
            </button>
        </div>
    }
}

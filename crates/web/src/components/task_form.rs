use checko_core::{Priority, AREAS};
use leptos::{html, prelude::*};

use super::icon::{Glyph, Icon};
use crate::store::{NewTask, Store};
use crate::time::today;

const PLACEHOLDERS: [&str; 4] = [
    "Zahnarzt anrufen",
    "Milch und Eier einkaufen",
    "Steuererklärung abgeben",
    "Wochenendtrip planen",
];

fn parse_priority(v: &str) -> Priority {
    match v {
        "1" => Priority::P1,
        "2" => Priority::P2,
        "3" => Priority::P3,
        _ => Priority::P4,
    }
}

/// Add-task form: title plus optional priority, area and due date.
/// `due_today` pre-fills the date with today (used on the Today page).
#[component]
pub fn TaskForm(
    #[prop(optional)] due_today: bool,
    #[prop(optional)] autofocus: bool,
    #[prop(optional, into)] area: Signal<Option<String>>,
) -> impl IntoView {
    let store = Store::get();
    let input = NodeRef::<html::Input>::new();
    let title = RwSignal::new(String::new());
    let priority = RwSignal::new("4".to_string());
    let area_sel = RwSignal::new(String::new());
    let due = RwSignal::new(if due_today {
        today().to_string()
    } else {
        String::new()
    });
    // Follow the page filter: adding while "Arbeit" is selected files it under Arbeit.
    Effect::new(move |_| area_sel.set(area.get().unwrap_or_default()));

    let placeholder = PLACEHOLDERS[(js_sys::Math::random() * 4.0) as usize % 4];

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let t = title.get_untracked();
        if t.trim().is_empty() {
            return;
        }
        store.add(NewTask {
            title: t,
            priority: parse_priority(&priority.get_untracked()),
            area: Some(area_sel.get_untracked()).filter(|a| !a.is_empty()),
            due: due.get_untracked().parse().ok(),
        });
        title.set(String::new());
        priority.set("4".into());
        area_sel.set(area.get_untracked().unwrap_or_default());
        if let Some(el) = input.get() {
            let _ = el.focus();
        }
    };

    view! {
        <form class="task-form" on:submit=submit>
            <div class="quick-add">
                <Icon glyph=Glyph::Add size=18 />
                <input
                    node_ref=input
                    type="text"
                    aria-label="Neue Aufgabe"
                    placeholder=placeholder
                    autofocus=autofocus
                    bind:value=title
                />
                <kbd>"Enter"</kbd>
            </div>
            <div class="task-form-options">
                <label class="field">
                    <span class="sr-only">"Priorität"</span>
                    <select bind:value=priority aria-label="Priorität">
                        <option value="1">"P1 · Dringend"</option>
                        <option value="2">"P2 · Hoch"</option>
                        <option value="3">"P3 · Mittel"</option>
                        <option value="4">"P4 · Normal"</option>
                    </select>
                </label>
                <label class="field">
                    <span class="sr-only">"Bereich"</span>
                    <select bind:value=area_sel aria-label="Bereich">
                        <option value="">"Kein Bereich"</option>
                        {AREAS.iter().map(|a| view! { <option value=*a>{*a}</option> }).collect_view()}
                    </select>
                </label>
                <label class="field">
                    <span class="sr-only">"Fällig am"</span>
                    <input type="date" aria-label="Fällig am" bind:value=due />
                </label>
                <button type="submit" class="btn btn-primary" disabled=move || title.get().trim().is_empty()>
                    "Hinzufügen"
                </button>
            </div>
        </form>
    }
}

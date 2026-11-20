use checko_core::{parse_quick_add, Priority, TokenKind};

use crate::ui::priority_name;
use leptos::{html, prelude::*};

use super::icon::{Glyph, Icon};
use crate::store::{NewTask, Store};
use crate::time::{due_label, today};

const PLACEHOLDERS: [&str; 4] = [
    "Zahnarzt morgen 14:00 #Gesundheit !1",
    "Wandleuchte kaufen #Küche",
    "Steuererklärung bis Freitag !2",
    "Wochenendtrip planen nächste Woche",
];

fn parse_priority(v: &str) -> Priority {
    match v {
        "1" => Priority::P1,
        "2" => Priority::P2,
        "3" => Priority::P3,
        _ => Priority::P4,
    }
}

/// Add-task form. Typing "morgen 14:00 #arbeit !1" fills date, time, area and
/// priority automatically; the selects are a fallback for mouse users.
/// `due_today` pre-fills the date with today (used on the Today page).
#[component]
pub fn TaskForm(
    #[prop(optional)] due_today: bool,
    #[prop(optional)] autofocus: bool,
    #[prop(optional, into)] area: Signal<Option<String>>,
    /// Called after a task was added (e.g. to close a dialog).
    #[prop(optional)]
    on_added: Option<Callback<()>>,
    /// Share the typed text with the parent (the Strg+K dialog searches with it).
    #[prop(optional)]
    text: Option<RwSignal<String>>,
) -> impl IntoView {
    let store = Store::get();
    let input = NodeRef::<html::Input>::new();
    let title = text.unwrap_or_else(|| RwSignal::new(String::new()));
    let priority = RwSignal::new("4".to_string());
    let area_sel = RwSignal::new(String::new());
    let default_due = move || {
        if due_today {
            today().to_string()
        } else {
            String::new()
        }
    };
    let due = RwSignal::new(default_due());
    // Follow the page filter: adding while "Arbeit" is selected files it under Arbeit.
    Effect::new(move |_| area_sel.set(area.get().unwrap_or_default()));

    // `autofocus` alone is ignored by browsers once something else has focus
    // (e.g. the button that opened the dialog), so focus explicitly.
    if autofocus {
        Effect::new(move |_| {
            if let Some(el) = input.get() {
                let _ = el.focus();
            }
        });
    }

    let areas = Memo::new(move |_| store.areas());
    let parsed = Memo::new(move |_| areas.with(|a| parse_quick_add(&title.get(), today(), a)));
    let placeholder = PLACEHOLDERS[(js_sys::Math::random() * 4.0) as usize % 4];

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if title.get_untracked().trim().is_empty() {
            return;
        }
        let p = parsed.get_untracked();
        store.add(NewTask {
            title: p.title,
            priority: p
                .priority
                .unwrap_or_else(|| parse_priority(&priority.get_untracked())),
            area: p
                .area
                .or_else(|| Some(area_sel.get_untracked()).filter(|a| !a.is_empty())),
            due: p.due.or_else(|| due.get_untracked().parse().ok()),
            time: p.time,
            tags: p.tags,
        });
        title.set(String::new());
        priority.set("4".into());
        area_sel.set(area.get_untracked().unwrap_or_default());
        due.set(default_due());
        if let Some(cb) = on_added {
            cb.run(());
        } else if let Some(el) = input.get() {
            let _ = el.focus();
        }
    };

    // Chips that show what was recognised, e.g. "📅 Morgen · 14:00".
    let preview = move || {
        let p = parsed.get();
        let today = today();
        let mut chips = Vec::new();
        if let Some(d) = p.due {
            let mut label = due_label(d, today);
            if let Some(t) = p.time {
                label.push_str(&format!(" · {}", t.format("%H:%M")));
            }
            chips.push((Glyph::Calendar, label, "chip-date"));
        }
        if let Some(pr) = p.priority {
            chips.push((Glyph::Level, priority_name(pr).to_string(), "chip-prio"));
        }
        if let Some(a) = p.area.clone() {
            chips.push((Glyph::Pin, format!("Kategorie: {a}"), "chip-area"));
        }
        for t in &p.tags {
            chips.push((Glyph::Notes, format!("#{t}"), "chip-tag"));
        }
        chips
    };
    let has_tokens = move || !parsed.with(|p| p.tokens.is_empty());
    let recognised_words = move || {
        parsed.with(|p| {
            p.tokens
                .iter()
                .filter(|t| t.kind != TokenKind::Tag)
                .map(|t| t.text.clone())
                .collect::<Vec<_>>()
                .join(", ")
        })
    };

    view! {
        <form class="task-form" on:submit=submit>
            <div class="quick-add">
                <Icon glyph=Glyph::Add size=18 />
                <input
                    node_ref=input
                    type="text"
                    aria-label="Neue Aufgabe"
                    aria-describedby="quick-add-hint"
                    placeholder=placeholder
                    autofocus=autofocus
                    bind:value=title
                />
                <kbd>"Enter"</kbd>
            </div>
            <Show
                when=has_tokens
            >
                <div id="quick-add-hint" class="parse-preview" aria-live="polite">
                    <span class="parse-title">
                        {move || parsed.with(|p| p.title.clone())}
                    </span>
                    {move || preview()
                        .into_iter()
                        .map(|(g, label, class)| view! {
                            <span class=format!("parse-chip {class}")>
                                <Icon glyph=g size=13 />
                                {label}
                            </span>
                        })
                        .collect_view()}
                    <span class="sr-only">"Erkannt: " {recognised_words}</span>
                </div>
            </Show>
            <div class="task-form-options">
                <label class="field">
                    <span class="sr-only">"Dringlichkeit"</span>
                    <select bind:value=priority aria-label="Dringlichkeit">
                        <option value="1">"🔴 Dringend"</option>
                        <option value="2">"🟠 Hoch"</option>
                        <option value="3">"🟡 Mittel"</option>
                        <option value="4">"🟢 Normal"</option>
                    </select>
                </label>
                <label class="field">
                    <span class="sr-only">"Kategorie"</span>
                    <select bind:value=area_sel aria-label="Kategorie">
                        <option value="">"Keine Kategorie"</option>
                        {move || areas
                            .get()
                            .into_iter()
                            .map(|a| view! { <option value=a.clone()>{a.clone()}</option> })
                            .collect_view()}
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

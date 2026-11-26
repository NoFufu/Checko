//! Settings: manage categories (rename, emoji, colour, delete) and focus mode.

use leptos::prelude::*;

use crate::store::Store;
use crate::ui::{area_color, category_icon, default_area_color, default_category_icon};

/// Hex value for the colour picker (CSS variables of the defaults resolved).
fn hex_for(color: &str) -> String {
    match color {
        "var(--lavender)" => "#a48cd9".into(),
        "var(--green)" => "#65a982".into(),
        "var(--peach)" => "#f4a587".into(),
        c => c.to_string(),
    }
}

#[component]
pub fn CategoryManager() -> impl IntoView {
    let store = Store::get();
    view! {
        <ul class="cat-manage">
            <For
                each=move || store.areas()
                key=|n| n.clone()
                children=move |name| view! { <CategoryRow name=name /> }
            />
        </ul>
        <p class="text-sm text-muted">
            "Neue Kategorien entstehen, sobald du sie einer Aufgabe gibst (z. B. „#Garten“). "
            "Löschen entfernt nur die Kategorie – die Aufgaben bleiben."
        </p>
    }
}

#[component]
fn CategoryRow(name: String) -> impl IntoView {
    let store = Store::get();
    let n = name.clone();
    let new_name = RwSignal::new(name.clone());
    let emoji = {
        let n = n.clone();
        move || category_icon(&n)
    };
    let color = Signal::derive({
        let n = n.clone();
        move || hex_for(&area_color(&n))
    });
    let custom = {
        let n = n.clone();
        move || {
            store
                .profile
                .with(|p| p.category_style(&n).is_some_and(|s| !s.hidden))
        }
    };
    let current_style = {
        let n = n.clone();
        move || {
            store
                .profile
                .with_untracked(|p| p.category_style(&n).cloned().unwrap_or_default())
        }
    };
    let set_emoji = {
        let (n, cs) = (n.clone(), current_style.clone());
        move |v: String| {
            let v: String = v.trim().chars().take(4).collect();
            let v = Some(v).filter(|v| !v.is_empty() && *v != default_category_icon(&n));
            store.set_category_style(&n, v, cs().color);
        }
    };
    let set_color = {
        let (n, cs) = (n.clone(), current_style.clone());
        move |v: String| {
            let v = Some(v).filter(|v| *v != hex_for(default_area_color(&n)));
            store.set_category_style(&n, cs().emoji, v);
        }
    };
    let reset = {
        let n = n.clone();
        move |_| store.set_category_style(&n, None, None)
    };
    let rename = {
        let n = n.clone();
        move || {
            let v = new_name.get_untracked();
            if v.trim().is_empty() {
                new_name.set(n.clone());
            } else if v.trim() != n {
                store.rename_category_everywhere(&n, &v);
            }
        }
    };
    let delete = {
        let n = n.clone();
        move |_| {
            let ok = web_sys::window()
                .and_then(|w| {
                    w.confirm_with_message(&format!(
                        "Kategorie „{n}“ löschen? Die Aufgaben und Termine bleiben erhalten."
                    ))
                    .ok()
                })
                .unwrap_or(false);
            if ok {
                store.delete_category_everywhere(&n);
            }
        }
    };
    view! {
        <li class="cat-manage-row" style=move || format!("--c:{}", color.get())>
            <label class="cat-emoji-field" title="Emoji ändern">
                <span class="sr-only">{format!("Emoji für {name}")}</span>
                <input
                    class="cat-emoji-input"
                    prop:value=emoji
                    maxlength="8"
                    on:change=move |e| set_emoji(event_target_value(&e))
                />
            </label>
            <label class="cat-color-field" title="Farbe ändern">
                <span class="sr-only">{format!("Farbe für {name}")}</span>
                <input type="color" prop:value=move || color.get() on:change=move |e| set_color(event_target_value(&e)) />
            </label>
            <form class="cat-name-form" on:submit=move |e| { e.prevent_default(); rename(); }>
                <label class="sr-only" for=format!("cat-{name}")>{format!("Name von {name}")}</label>
                <input
                    id=format!("cat-{name}")
                    class="cat-name-input"
                    bind:value=new_name
                    on:blur={
                        let rename = rename.clone();
                        move |_| rename()
                    }
                />
            </form>
            <Show when=custom.clone()>
                <button type="button" class="btn btn-outline btn-sm" on:click=reset.clone()>"Standard"</button>
            </Show>
            <button type="button" class="btn btn-outline btn-sm cat-delete" aria-label=format!("{name} löschen") on:click=delete>
                "Löschen"
            </button>
        </li>
    }
}

/// Switch for focus mode (hides XP, level, streak, achievements, confetti).
#[component]
pub fn FocusSwitch() -> impl IntoView {
    let store = Store::get();
    view! {
        <label class="switch">
            <input
                type="checkbox"
                role="switch"
                prop:checked=move || store.focus()
                on:change=move |e| store.set_focus(event_target_checked(&e))
            />
            <span class="switch-track" aria-hidden="true"><span class="switch-thumb"></span></span>
            <span class="sr-only">"Fokus-Modus"</span>
        </label>
    }
}

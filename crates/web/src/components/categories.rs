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
    let new_cat = RwSignal::new(String::new());
    view! {
        <GroupManager />
        <form class="cat-add" on:submit=move |e| { e.prevent_default(); store.new_category(&new_cat.get_untracked()); new_cat.set(String::new()); }>
            <input class="cat-name-input cat-add-input" placeholder="Neue Kategorie, z. B. Garten" aria-label="Neue Kategorie" bind:value=new_cat />
            <button type="submit" class="btn btn-outline btn-sm">"Hinzufügen"</button>
        </form>
        <ul class="cat-manage">
            <For
                each=move || store.areas()
                key=|n| n.clone()
                children=move |name| view! { <CategoryRow name=name /> }
            />
        </ul>
        <p class="text-sm text-muted">
            "Neue Kategorien legst du oben an – oder einfach mit „#Garten“ in einer Aufgabe. "
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
            store.profile.with(|p| {
                p.category_style(&n)
                    .is_some_and(|s| s.emoji.is_some() || s.color.is_some())
            })
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
            <ParentSelect name=name.clone() />
            <Show when=custom.clone()>
                <button type="button" class="btn btn-outline btn-sm" on:click=reset.clone()>"Standard"</button>
            </Show>
            <button type="button" class="btn btn-outline btn-sm cat-delete" aria-label=format!("{name} löschen") on:click=delete>
                "Löschen"
            </button>
        </li>
    }
}

/// Drop-down: which Überkategorie a category belongs to.
#[component]
fn ParentSelect(name: String) -> impl IntoView {
    let store = Store::get();
    let groups = move || store.profile.with(|p| p.category_groups.clone());
    let n = name.clone();
    let current = move || {
        store
            .profile
            .with(|p| p.category_parent(&n).unwrap_or_default().to_string())
    };
    let n2 = name.clone();
    view! {
        <Show when=move || !groups().is_empty()>
            <select class="cat-parent" aria-label=format!("Überkategorie für {name}") prop:value=current.clone()
                on:change={
                    let n2 = n2.clone();
                    move |e| { let v = event_target_value(&e); store.set_category_parent(&n2, Some(v.as_str()).filter(|v| !v.is_empty())); }
                }>
                <option value="">"Keine Überkategorie"</option>
                {move || groups().into_iter().map(|g| view! { <option value=g.clone()>{g.clone()}</option> }).collect_view()}
            </select>
        </Show>
    }
}

/// Überkategorien: create, rename, delete.
#[component]
fn GroupManager() -> impl IntoView {
    let store = Store::get();
    let new_group = RwSignal::new(String::new());
    let groups = move || store.profile.with(|p| p.category_groups.clone());
    view! {
        <div class="group-manage">
            <p class="text-sm font-semibold">"Überkategorien"</p>
            <ul class="group-manage-list">
                <For each=groups key=|g| g.clone() children=move |g| {
                    let name = RwSignal::new(g.clone());
                    let (old, old2) = (g.clone(), g.clone());
                    view! {
                        <li class="group-chip">
                            <input class="cat-name-input" aria-label=format!("Name der Überkategorie {g}") bind:value=name
                                on:change=move |_| store.rename_group(&old, &name.get_untracked()) />
                            <button type="button" class="btn btn-outline btn-sm" aria-label=format!("Überkategorie {g} löschen")
                                on:click=move |_| {
                                    let ok = web_sys::window().and_then(|w| w.confirm_with_message(&format!("Überkategorie „{old2}“ löschen? Die Kategorien darin bleiben.")).ok()).unwrap_or(false);
                                    if ok { store.delete_group(&old2); }
                                }>"Löschen"</button>
                        </li>
                    }
                } />
            </ul>
            <form class="cat-add" on:submit=move |e| { e.prevent_default(); store.add_group(&new_group.get_untracked()); new_group.set(String::new()); }>
                <input class="cat-name-input cat-add-input" placeholder="Neue Überkategorie, z. B. Zuhause" aria-label="Neue Überkategorie" bind:value=new_group />
                <button type="submit" class="btn btn-outline btn-sm">"Anlegen"</button>
            </form>
            <p class="text-sm text-muted">"Danach bei jeder Kategorie unten die Überkategorie wählen."</p>
        </div>
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

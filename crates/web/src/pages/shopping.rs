use checko_core::{grouped, open_count, suggestions, ShoppingItem};
use leptos::prelude::*;

use crate::components::{
    icon::{Glyph, Icon},
    topbar::Topbar,
};
use crate::store::Store;

#[component]
pub fn ShoppingPage() -> impl IntoView {
    let store = Store::get();
    let input = RwSignal::new(String::new());

    let groups = move || {
        store.shopping.with(|items| {
            grouped(items)
                .into_iter()
                .map(|(cat, v)| (cat, v.into_iter().cloned().collect::<Vec<_>>()))
                .collect::<Vec<_>>()
        })
    };
    let open = move || store.shopping.with(|i| open_count(i));
    let checked = move || {
        store.shopping.with(|i| {
            i.iter()
                .filter(|i| i.checked && i.meta.deleted_at.is_none())
                .count()
        })
    };
    let suggest = move || {
        store.shopping.with(|i| {
            suggestions(i, 8)
                .into_iter()
                .map(|i| i.name.clone())
                .collect::<Vec<_>>()
        })
    };

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        store.add_shopping(&input.get_untracked());
        input.set(String::new());
    };

    view! {
        <Topbar title="Einkauf" icon=Glyph::Shopping />
        <div class="page">
            <div class="page-head">
                <div>
                    <h1 class="page-h1">"Einkaufsliste"</h1>
                    <p class="text-muted">
                        {move || match open() {
                            0 => "Nichts zu besorgen.".to_string(),
                            1 => "1 Sache zu besorgen".to_string(),
                            n => format!("{n} Sachen zu besorgen"),
                        }}
                    </p>
                </div>
                <Show when=move || { checked() > 0 }>
                    <button type="button" class="btn btn-outline" on:click=move |_| store.clear_checked_shopping()>
                        <Icon glyph=Glyph::Trash size=16 />
                        "Erledigte entfernen"
                    </button>
                </Show>
            </div>

            <section class="panel">
                <form class="task-form" on:submit=submit>
                    <div class="quick-add">
                        <Icon glyph=Glyph::Add size=18 />
                        <input
                            type="text"
                            aria-label="Artikel hinzufügen"
                            placeholder="z. B. 2x Milch, Äpfel, Wasser 6l"
                            autofocus=true
                            bind:value=input
                        />
                        <kbd>"Enter"</kbd>
                    </div>
                    <p class="parse-hint">"Checko sortiert automatisch nach Regal: Obst & Gemüse, Kühlregal, Getränke …"</p>
                </form>
                <Show when=move || !suggest().is_empty()>
                    <div class="suggestions">
                        <span class="text-sm text-muted">"Oft gekauft:"</span>
                        {move || suggest()
                            .into_iter()
                            .map(|name| {
                                let n = name.clone();
                                view! {
                                    <button type="button" class="suggestion" on:click=move |_| store.add_shopping(&n)>
                                        <Icon glyph=Glyph::Add size=13 />
                                        {name}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </Show>
            </section>

            <Show
                when=move || !groups().is_empty()
                fallback=|| view! {
                    <section class="panel empty-panel mt-6">
                        <span class="empty-icon"><Icon glyph=Glyph::Shopping size=28 /></span>
                        <p class="font-semibold">"Die Liste ist leer"</p>
                        <p class="text-muted">"Tippe oben ein, was du brauchst."</p>
                    </section>
                }
            >
                <div class="shop-groups">
                    <For
                        each=groups
                        key=|(cat, items)| (cat.to_string(), items.iter().map(|i| (i.meta.id, i.checked)).collect::<Vec<_>>())
                        children=move |(cat, items)| view! {
                            <section class="panel shop-group">
                                <h2 class="shop-cat">{cat}</h2>
                                <ul class="task-list">
                                    {items.into_iter().map(|i| view! { <ShopRow item=i /> }).collect_view()}
                                </ul>
                            </section>
                        }
                    />
                </div>
            </Show>
        </div>
    }
}

#[component]
fn ShopRow(item: ShoppingItem) -> impl IntoView {
    let store = Store::get();
    let id = item.meta.id;
    let checked = item.checked;
    view! {
        <li class="task-row shop-row" class:done=checked>
            <button
                type="button"
                class="check"
                class:checked=checked
                aria-label=if checked { "Wieder auf die Liste" } else { "Abhaken" }
                on:click=move |_| store.toggle_shopping(id)
            >
                <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
                    <path d="M5 12.5l4.5 4.5L19 7.5" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
            <div class="task-body">
                <span class="task-title">{item.name.clone()}</span>
            </div>
            {item.qty.clone().map(|q| view! { <span class="qty">{q}</span> })}
            <div class="task-actions">
                <button type="button" class="row-btn" aria-label="Entfernen" title="Entfernen" on:click=move |_| store.remove_shopping(id)>
                    <Icon glyph=Glyph::Trash size=16 />
                </button>
            </div>
        </li>
    }
}

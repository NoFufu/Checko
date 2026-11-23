use checko_core::{grouped, link_host, open_count, purchase_history, ShoppingItem};
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
    let history_tab = RwSignal::new(false);
    let history = move || store.shopping.with(|i| purchase_history(i));

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
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        store.add_shopping(&input.get_untracked());
        input.set(String::new());
    };

    view! {
        <Topbar />
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
                <div class="segmented" role="tablist" aria-label="Ansicht">
                    <button type="button" role="tab" class="segment" class:active=move || !history_tab.get() aria-selected=move || (!history_tab.get()).to_string() on:click=move |_| history_tab.set(false)>"Liste"</button>
                    <button type="button" role="tab" class="segment" class:active=move || history_tab.get() aria-selected=move || history_tab.get().to_string() on:click=move |_| history_tab.set(true)>"Verlauf"</button>
                </div>
            </div>

            <Show when=move || history_tab.get()>
                <section class="panel list-panel">
                    <Show
                        when=move || !history().is_empty()
                        fallback=|| view! { <p class="empty-line px-3">"Noch nichts gekauft. Abgehakte Sachen landen hier."</p> }
                    >
                        <ul class="task-list">
                            {move || history()
                                .into_iter()
                                .map(|p| {
                                    let again = match &p.url {
                                        Some(u) => format!("{} {u}", p.name),
                                        None => p.name.clone(),
                                    };
                                    let when = crate::time::local_label(p.last);
                                    view! {
                                        <li class="task-row history-row">
                                            <div class="task-body">
                                                <span class="task-title">{p.name.clone()}</span>
                                                <span class="text-sm text-muted">{format!("{}× gekauft · zuletzt {}", p.times, when)}</span>
                                            </div>
                                            {p.url.clone().map(|u| view! {
                                                <a class="shop-link" href=u target="_blank" rel="noopener noreferrer" aria-label="Link öffnen">
                                                    <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
                                                        <path d="M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" />
                                                    </svg>
                                                </a>
                                            })}
                                            {if p.on_list {
                                                view! { <span class="chip">"Auf der Liste"</span> }.into_any()
                                            } else {
                                                view! {
                                                    <button type="button" class="btn btn-outline btn-sm" on:click=move |_| store.add_shopping(&again)>
                                                        <Icon glyph=Glyph::Add size=14 />
                                                        "Nochmal"
                                                    </button>
                                                }.into_any()
                                            }}
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </Show>
                </section>
            </Show>

            <Show when=move || !history_tab.get()>
            <div class="shop-actions">
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
                            placeholder="z. B. 2x Milch oder ein Link"
                            autofocus=true
                            bind:value=input
                        />
                        <kbd>"Enter"</kbd>
                    </div>
                </form>
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
            {item.url.clone().map(|url| {
                let host = link_host(&url);
                let title = format!("Öffnen auf {host}");
                let label = format!("Link öffnen ({host})");
                view! {
                    <a
                        class="shop-link"
                        href=url
                        target="_blank"
                        rel="noopener noreferrer"
                        title=title
                        aria-label=label
                    >
                        <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
                            <path d="M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" />
                        </svg>
                        <span class="shop-link-host">{host}</span>
                    </a>
                }
            })}
            <div class="task-actions">
                <button type="button" class="row-btn" aria-label="Entfernen" title="Entfernen" on:click=move |_| store.remove_shopping(id)>
                    <Icon glyph=Glyph::Trash size=16 />
                </button>
            </div>
        </li>
    }
}

use checko_core::sorted_notes;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use uuid::Uuid;

use crate::components::{
    icon::{Glyph, Icon},
    topbar::Topbar,
};
use crate::store::Store;

#[component]
pub fn NotesPage() -> impl IntoView {
    let store = Store::get();
    let query = use_query_map();
    let selected = RwSignal::new(None::<Uuid>);

    // Open the note from "?id=…" (used by search), otherwise the first one.
    Effect::new(move |_| {
        let from_url = query.with(|q| q.get("id")).and_then(|v| v.parse().ok());
        if from_url.is_some() {
            selected.set(from_url);
        } else if selected.get_untracked().is_none() {
            selected.set(
                store
                    .notes
                    .with_untracked(|n| sorted_notes(n).first().map(|n| n.meta.id)),
            );
        }
    });

    let list = move || {
        store
            .notes
            .with(|n| sorted_notes(n).into_iter().cloned().collect::<Vec<_>>())
    };
    let current = move || {
        let id = selected.get()?;
        store.notes.with(|ns| {
            ns.iter()
                .find(|n| n.meta.id == id && n.meta.deleted_at.is_none())
                .cloned()
        })
    };

    let new_note = move |_| {
        let id = store.new_note();
        selected.set(Some(id));
    };

    view! {
        <Topbar />
        <div class="page">
            <div class="page-head">
                <div>
                    <h1 class="page-h1">"Notizen"</h1>
                </div>
                <button type="button" class="btn btn-primary" on:click=new_note>
                    <Icon glyph=Glyph::Add size=16 />
                    "Neue Notiz"
                </button>
            </div>

            <div class="notes-layout">
                <section class="panel list-panel notes-list" aria-label="Alle Notizen">
                    <Show
                        when=move || !list().is_empty()
                        fallback=|| view! { <p class="empty-line px-3">"Noch keine Notizen."</p> }
                    >
                        <ul class="task-list">
                            <For
                                each=list
                                key=|n| (n.meta.id, n.meta.updated_at)
                                children=move |n| {
                                    let id = n.meta.id;
                                    view! {
                                        <li>
                                            <button
                                                type="button"
                                                class="note-item"
                                                class:active=move || selected.get() == Some(id)
                                                on:click=move |_| selected.set(Some(id))
                                            >
                                                <span class="note-item-title">
                                                    {n.pinned.then(|| view! { <Icon glyph=Glyph::Pin size=13 /> })}
                                                    {n.display_title()}
                                                </span>
                                                <span class="note-item-preview">{n.preview(80)}</span>
                                            </button>
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </Show>
                </section>

                <section class="panel note-editor">
                    {move || match current() {
                        None => view! {
                            <div class="empty-panel">
                                <span class="empty-icon"><Icon glyph=Glyph::Notes size=28 /></span>
                                <p class="font-semibold">"Keine Notiz ausgewählt"</p>
                                <p class="text-muted">"Wähle links eine Notiz oder lege eine neue an."</p>
                            </div>
                        }.into_any(),
                        Some(n) => {
                            let id = n.meta.id;
                            let pinned = n.pinned;
                            let body = n.body_md.clone();
                            view! {
                                <div class="note-toolbar">
                                    <span class="text-sm text-muted">{format!("Geändert {}", crate::time::local_label(n.meta.updated_at))}</span>
                                    <div class="flex items-center gap-1">
                                        <button
                                            type="button"
                                            class="row-btn"
                                            class:active=pinned
                                            title=if pinned { "Lösen" } else { "Anpinnen" }
                                            aria-label=if pinned { "Lösen" } else { "Anpinnen" }
                                            on:click=move |_| store.edit_note(id, |n| n.pinned = !n.pinned)
                                        >
                                            <Icon glyph=Glyph::Pin size=16 />
                                        </button>
                                        <button
                                            type="button"
                                            class="row-btn"
                                            title="Löschen"
                                            aria-label="Notiz löschen"
                                            on:click=move |_| { store.trash_note(id); selected.set(None); }
                                        >
                                            <Icon glyph=Glyph::Trash size=16 />
                                        </button>
                                    </div>
                                </div>
                                <input
                                    class="note-title"
                                    type="text"
                                    placeholder="Titel"
                                    aria-label="Titel"
                                    prop:value=n.title.clone()
                                    on:input=move |ev| { let v = event_target_value(&ev); store.edit_note(id, |n| n.title = v); }
                                />
                                <textarea
                                    class="note-body"
                                    aria-label="Inhalt"
                                    placeholder="Schreib los …"
                                    prop:value=body
                                    on:input=move |ev| { let v = event_target_value(&ev); store.edit_note(id, |n| n.body_md = v); }
                                ></textarea>
                            }.into_any()
                        }
                    }}
                </section>
            </div>
        </div>
    }
}

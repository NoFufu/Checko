use checko_core::{search, HitKind};
use leptos::{ev, prelude::*};
use leptos_router::hooks::use_navigate;

use super::icon::{Glyph, Icon};
use super::task_form::TaskForm;
use crate::store::Store;

/// Strg+K (or Cmd+K) dialog to add a task from anywhere.
#[component]
pub fn QuickAddDialog() -> impl IntoView {
    let store = Store::get();
    let open = store.quick_add_open;
    let text = RwSignal::new(String::new());
    let navigate = use_navigate();
    let go = Callback::new(move |path: String| navigate(&path, Default::default()));

    let hits = move || {
        let q = text.get();
        if q.trim().chars().count() < 2 {
            return Vec::new();
        }
        store.tasks.with(|t| {
            store
                .notes
                .with(|n| store.shopping.with(|s| search(&q, t, n, s, 8)))
        })
    };

    // forget a day picked in the calendar once the dialog closes
    Effect::new(move |_| {
        if !open.get() {
            store.quick_add_date.set(None);
            store.quick_add_until.set(None);
        }
    });

    let handle = window_event_listener(ev::keydown, move |e| {
        let key = e.key().to_lowercase();
        if key == "k" && (e.ctrl_key() || e.meta_key()) {
            e.prevent_default();
            open.update(|o| *o = !*o);
        } else if key == "escape" && open.get_untracked() {
            open.set(false);
        }
    });
    on_cleanup(move || handle.remove());

    view! {
        <Show when=move || open.get()>
            <div class="dialog-backdrop" on:click=move |_| open.set(false)>
                <div
                    class="dialog"
                    role="dialog"
                    aria-modal="true"
                    aria-label="Schnell hinzufügen"
                    on:click=|e| e.stop_propagation()
                >
                    <div class="dialog-head">
                        <h2 class="font-semibold">"Hinzufügen oder suchen"</h2>
                        <span class="text-sm text-muted">"Enter speichert · Esc schließt"</span>
                    </div>
                    <TaskForm
                        full=true
                        autofocus=true
                        text=text
                        on_added=Callback::new(move |_| open.set(false))
                    />
                    <Show when=move || !hits().is_empty()>
                        <div class="search-results">
                            <p class="nav-heading">"Gefunden"</p>
                            <ul>
                                {
                                    move || {
                                        hits()
                                            .into_iter()
                                            .map(|h| {
                                                let (glyph, path) = match h.kind {
                                                    HitKind::Task => (Glyph::Tasks, "/aufgaben".to_string()),
                                                    HitKind::Note => (Glyph::Notes, format!("/notizen?id={}", h.id)),
                                                    HitKind::Shopping => (Glyph::Shopping, "/einkauf".to_string()),
                                                };
                                                view! {
                                                    <li>
                                                        <button
                                                            type="button"
                                                            class="search-hit"
                                                            on:click=move |_| {
                                                                open.set(false);
                                                                text.set(String::new());
                                                                go.run(format!("{}{}", crate::BASE, path));
                                                            }
                                                        >
                                                            <span class="list-icon"><Icon glyph=glyph size=16 /></span>
                                                            <span class="min-w-0">
                                                                <span class="block font-medium truncate">{h.title}</span>
                                                                <span class="block text-sm text-muted truncate">{h.detail}</span>
                                                            </span>
                                                        </button>
                                                    </li>
                                                }
                                            })
                                            .collect_view()
                                    }
                                }
                            </ul>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}

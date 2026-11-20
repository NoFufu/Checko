use leptos::{ev, prelude::*};

use super::task_form::TaskForm;
use crate::store::Store;

/// Strg+K (or Cmd+K) dialog to add a task from anywhere.
#[component]
pub fn QuickAddDialog() -> impl IntoView {
    let store = Store::get();
    let open = store.quick_add_open;

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
                        <h2 class="font-semibold">"Schnell hinzufügen"</h2>
                        <span class="text-sm text-muted">"Esc zum Schließen"</span>
                    </div>
                    <TaskForm autofocus=true on_added=Callback::new(move |_| open.set(false)) />
                </div>
            </div>
        </Show>
    }
}

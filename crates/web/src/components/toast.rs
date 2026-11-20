use leptos::prelude::*;

use super::icon::{Glyph, Icon};
use crate::store::Store;

/// Bottom-centre message with an optional "Rückgängig" button.
#[component]
pub fn ToastHost() -> impl IntoView {
    let store = Store::get();
    view! {
        <div class="toast-host" role="status" aria-live="polite">
            {move || {
                store
                    .toast
                    .get()
                    .map(|t| {
                        let undo = t.undo;
                        view! {
                            <div class="toast">
                                <span>{t.text}</span>
                                {undo.map(|u| view! {
                                    <button type="button" class="toast-undo" on:click=move |_| store.undo(u.clone())>
                                        <Icon glyph=Glyph::Undo size=14 />
                                        "Rückgängig"
                                    </button>
                                })}
                                <button type="button" class="toast-close" aria-label="Schließen" on:click=move |_| store.dismiss_toast()>
                                    "×"
                                </button>
                            </div>
                        }
                    })
            }}
        </div>
    }
}

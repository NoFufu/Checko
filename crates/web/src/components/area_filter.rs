use leptos::prelude::*;

use crate::store::Store;
use crate::ui::area_color;

/// Segmented "Alle | Arbeit | Privat | …" filter, including custom categories.
#[component]
pub fn AreaFilter(area: RwSignal<Option<String>>) -> impl IntoView {
    let store = Store::get();
    view! {
        <div class="segmented" role="tablist" aria-label="Nach Kategorie filtern">
            <button
                type="button"
                role="tab"
                class="segment"
                class:active=move || area.get().is_none()
                aria-selected=move || area.get().is_none().to_string()
                on:click=move |_| area.set(None)
            >
                "Alle"
            </button>
            {move || {
                store
                    .areas()
                    .into_iter()
                    .map(|a| {
                        let is_active = {
                            let a = a.clone();
                            move || area.get().as_deref() == Some(a.as_str())
                        };
                        let pick = a.clone();
                        view! {
                            <button
                                type="button"
                                role="tab"
                                class="segment"
                                class:active=is_active.clone()
                                aria-selected=move || is_active().to_string()
                                on:click=move |_| area.set(Some(pick.clone()))
                            >
                                <span class="area-dot" style=format!("background:{}", area_color(&a))></span>
                                {a.clone()}
                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

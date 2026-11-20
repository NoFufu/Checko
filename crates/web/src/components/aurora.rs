use leptos::prelude::*;

/// Soft, slowly drifting colour blobs behind everything (inspired by "Soft Aurora").
#[component]
pub fn Aurora() -> impl IntoView {
    view! {
        <div class="aurora" aria-hidden="true">
            <span class="blob blob-green"></span>
            <span class="blob blob-lavender"></span>
            <span class="blob blob-peach"></span>
            <div class="grain"></div>
        </div>
    }
}

use leptos::prelude::*;

/// The "checko" wordmark: letters in the text colour (dark in light mode,
/// light in dark mode) and the green check mark forming the arm of the "k".
#[component]
pub fn Logo(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <span class=format!("wordmark {class}") role="img" aria-label="checko">
            <span aria-hidden="true">"chec"</span>
            <svg class="wordmark-k" viewBox="0 0 62 100" aria-hidden="true">
                // stem and lower leg in the text colour
                <path d="M10 6 V94 M30 62 L52 92" fill="none" stroke="currentColor" stroke-width="15" stroke-linecap="round" />
                // the check mark
                <path class="wordmark-check" d="M4 58 L22 76 L58 30" fill="none" stroke-width="15" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
            <span aria-hidden="true">"o"</span>
        </span>
    }
}

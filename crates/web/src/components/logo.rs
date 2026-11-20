use leptos::prelude::*;

/// The "checko" wordmark: letters in the text colour (dark in light mode,
/// light in dark mode) and the green check mark forming the arm of the "k".
#[component]
pub fn Logo(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <span class=format!("wordmark {class}") role="img" aria-label="checko">
            <span aria-hidden="true">"chec"</span>
            <svg class="wordmark-k" viewBox="-16 0 88 100" aria-hidden="true">
                // stem and lower leg in the text colour
                <path d="M12 12 V88 M30 64 L50 88" fill="none" stroke="currentColor" stroke-width="24" stroke-linecap="round" />
                // the check mark: starts left of the stem, ends high above the x-height
                <path class="wordmark-check" d="M-3 56 L19 80 L60 18" fill="none" stroke-width="22" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
            <span aria-hidden="true">"o"</span>
        </span>
    }
}

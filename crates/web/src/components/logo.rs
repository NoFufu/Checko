use leptos::prelude::*;

use crate::asset;

/// The checko logo. Two versions of the original artwork: dark letters for the
/// light theme and light letters for the dark theme (CSS shows the right one).
#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <span class="wordmark" role="img" aria-label="checko">
            <img class="logo-for-light" src=asset("brand/logo-light.webp") alt="" width="900" height="276" />
            <img class="logo-for-dark" src=asset("brand/logo-dark.webp") alt="" width="900" height="276" />
        </span>
    }
}

use leptos::prelude::*;

/// Words fade in from blur one after another (inspired by "Blur Text").
#[component]
pub fn BlurText(
    #[prop(into)] text: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let words: Vec<_> = text
        .split_whitespace()
        .enumerate()
        .map(|(i, w)| {
            let style = format!("animation-delay:{}ms", i * 90);
            view! { <span class="blur-word" style=style>{w.to_string()}</span> }
        })
        .collect();
    view! { <span class=class>{words}</span> }
}

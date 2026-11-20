use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};

/// Glass card with a soft light that follows the cursor (inspired by "Card Spotlight").
#[component]
pub fn SpotlightCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let on_move = |ev: MouseEvent| {
        let Some(el) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let r = el.get_bounding_client_rect();
        let style = el.style();
        let _ = style.set_property("--x", &format!("{}px", ev.client_x() as f64 - r.left()));
        let _ = style.set_property("--y", &format!("{}px", ev.client_y() as f64 - r.top()));
    };
    view! {
        <section class=format!("card spotlight {class}") on:mousemove=on_move>
            {children()}
        </section>
    }
}

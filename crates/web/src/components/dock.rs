use leptos::prelude::*;
use leptos_router::components::A;

use super::icon::{Glyph, Icon};

struct Item {
    href: &'static str,
    label: &'static str,
    icon: Glyph,
}

const ITEMS: [Item; 5] = [
    Item {
        href: "/",
        label: "Heute",
        icon: Glyph::Today,
    },
    Item {
        href: "/aufgaben",
        label: "Aufgaben",
        icon: Glyph::Tasks,
    },
    Item {
        href: "/kalender",
        label: "Kalender",
        icon: Glyph::Calendar,
    },
    Item {
        href: "/notizen",
        label: "Notizen",
        icon: Glyph::Notes,
    },
    Item {
        href: "/einkauf",
        label: "Einkauf",
        icon: Glyph::Shopping,
    },
];

/// Left navigation rail (Outlook-style): icon above label, active item highlighted.
/// On small screens it turns into a bottom bar.
#[component]
pub fn Dock() -> impl IntoView {
    view! {
        <nav class="dock" aria-label="Hauptnavigation">
            <a href=format!("{}/", crate::BASE) class="dock-brand" aria-label="Checko Startseite">
                <img src=crate::asset("brand/icon.png") alt="" />
            </a>
            {ITEMS
                .iter()
                .map(|it| {
                    view! {
                        <A href=format!("{}{}", crate::BASE, it.href) exact=true attr:class="dock-item" attr:aria-label=it.label>
                            <Icon glyph=it.icon size=24 />
                            <span class="dock-label">{it.label}</span>
                        </A>
                    }
                })
                .collect_view()}
        </nav>
    }
}

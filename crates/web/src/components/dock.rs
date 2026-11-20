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

/// Floating dock with macOS-style magnification (CSS only).
#[component]
pub fn Dock() -> impl IntoView {
    view! {
        <nav class="dock" aria-label="Hauptnavigation">
            {ITEMS
                .iter()
                .map(|it| {
                    view! {
                        <A href=format!("{}{}", crate::BASE, it.href) exact=true attr:class="dock-item" attr:aria-label=it.label>
                            <Icon glyph=it.icon />
                            <span class="dock-tooltip">{it.label}</span>
                        </A>
                    }
                })
                .collect_view()}
        </nav>
    }
}

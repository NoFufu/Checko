use leptos::prelude::*;
use leptos_router::components::A;

struct Item {
    href: &'static str,
    label: &'static str,
    icon: &'static str,
}

const ITEMS: [Item; 5] = [
    Item {
        href: "/",
        label: "Heute",
        icon: "☀️",
    },
    Item {
        href: "/aufgaben",
        label: "Aufgaben",
        icon: "✅",
    },
    Item {
        href: "/kalender",
        label: "Kalender",
        icon: "📅",
    },
    Item {
        href: "/notizen",
        label: "Notizen",
        icon: "📝",
    },
    Item {
        href: "/einkauf",
        label: "Einkauf",
        icon: "🛒",
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
                            <span class="dock-icon">{it.icon}</span>
                            <span class="dock-tooltip">{it.label}</span>
                        </A>
                    }
                })
                .collect_view()}
        </nav>
    }
}

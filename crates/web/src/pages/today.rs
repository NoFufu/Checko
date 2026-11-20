use std::time::Duration;

use checko_core::level_for_xp;
use leptos::prelude::*;

use crate::components::{
    blur_text::BlurText,
    icon::{Glyph, Icon},
    spotlight_card::SpotlightCard,
};
use crate::time::{greeting, today_label};

const PLACEHOLDERS: [&str; 4] = [
    "Zahnarzt morgen 14:00 #gesundheit",
    "Milch und Eier einkaufen",
    "Steuererklärung bis Freitag !1",
    "Idee: Wochenendtrip planen",
];

#[component]
pub fn TodayPage() -> impl IntoView {
    // Rotating example placeholder in the quick-add field.
    let ph = RwSignal::new(0usize);
    let _ = set_interval_with_handle(
        move || ph.update(|i| *i = (*i + 1) % PLACEHOLDERS.len()),
        Duration::from_millis(3000),
    );

    let progress = level_for_xp(0);

    view! {
        <section class="flex flex-col-reverse items-start gap-6 sm:flex-row sm:items-center sm:justify-between">
            <div>
                <p class="text-sm font-bold uppercase tracking-widest text-muted">{today_label()}</p>
                <h1 class="page-title mt-2">
                    <BlurText text=format!("{}!", greeting()) />
                </h1>
                <p class="mt-2 text-lg text-muted">"Schön, dass du da bist. Was steht heute an?"</p>
            </div>
        </section>

        <form class="quick-add mt-8" on:submit=|ev| ev.prevent_default()>
            <Icon glyph=Glyph::Add size=22 />
            <input
                type="text"
                aria-label="Schnell hinzufügen"
                placeholder=move || PLACEHOLDERS[ph.get()]
            />
            <kbd>"Strg K"</kbd>
        </form>

        <div class="bento mt-8">
            <SpotlightCard class="bento-wide">
                <CardHead icon=Glyph::Tasks title="Heute fällig" />
                <EmptyState text="Noch keine Aufgaben – ein perfekter Start! Füge oben etwas hinzu." />
            </SpotlightCard>

            <SpotlightCard>
                <CardHead icon=Glyph::Level title="Dein Level" />
                <p class="mt-3 text-4xl font-black">"Level " {progress.level}</p>
                <div class="xp-bar mt-4">
                    <div class="xp-fill" style=format!("width:{}%", progress.percent().max(3.0))></div>
                </div>
                <p class="mt-2 text-sm text-muted">{progress.current} " / " {progress.needed} " XP"</p>
            </SpotlightCard>

            <SpotlightCard>
                <CardHead icon=Glyph::Calendar title="Nächste Termine" />
                <EmptyState text="Dein Kalender ist frei. Zeit für dich!" />
            </SpotlightCard>

            <SpotlightCard>
                <CardHead icon=Glyph::Streak title="Streak" />
                <p class="mt-3 text-4xl font-black">"0 Tage"</p>
                <p class="mt-2 text-sm text-muted">"Erledige heute eine Aufgabe, um deine Serie zu starten."</p>
            </SpotlightCard>

            <SpotlightCard>
                <CardHead icon=Glyph::Shopping title="Einkaufsliste" />
                <EmptyState text="Kühlschrank voll? Liste leer." />
            </SpotlightCard>

            <SpotlightCard>
                <CardHead icon=Glyph::Pin title="Angepinnt" />
                <EmptyState text="Pinne wichtige Notizen an, dann landen sie hier." />
            </SpotlightCard>
        </div>
    }
}

#[component]
fn CardHead(icon: Glyph, title: &'static str) -> impl IntoView {
    view! {
        <h2 class="flex items-center gap-2 text-sm font-extrabold uppercase tracking-wider text-muted">
            <Icon glyph=icon size=18 />
            {title}
        </h2>
    }
}

#[component]
fn EmptyState(text: &'static str) -> impl IntoView {
    view! { <p class="mt-4 text-base text-muted">{text}</p> }
}

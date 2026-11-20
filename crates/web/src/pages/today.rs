use std::time::Duration;

use checko_core::level_for_xp;
use leptos::prelude::*;

use crate::components::{
    icon::{Glyph, Icon},
    topbar::Topbar,
};
use crate::time::{greeting, today_label, weekday_index};

const PLACEHOLDERS: [&str; 4] = [
    "Zahnarzt morgen 14:00 #gesundheit",
    "Milch und Eier einkaufen",
    "Steuererklärung bis Freitag !1",
    "Idee: Wochenendtrip planen",
];

const FILTERS: [&str; 4] = ["Alle", "Arbeit", "Privat", "Gesundheit"];
const WEEK: [&str; 7] = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];

#[component]
pub fn TodayPage() -> impl IntoView {
    // Rotating example placeholder in the quick-add field.
    let ph = RwSignal::new(0usize);
    let _ = set_interval_with_handle(
        move || ph.update(|i| *i = (*i + 1) % PLACEHOLDERS.len()),
        Duration::from_millis(3000),
    );
    let filter = RwSignal::new(0usize);
    let progress = level_for_xp(0);
    let today = weekday_index();

    view! {
        <Topbar title="Heute" icon=Glyph::Today />
        <div class="page">
            <section class="hero">
                <div class="hero-text">
                    <p class="hero-kicker">{today_label()}</p>
                    <h1 class="hero-title">{format!("{}!", greeting())}</h1>
                    <p class="hero-sub">"Dein Tag auf einen Blick: Aufgaben, Termine und Einkäufe."</p>
                    <div class="mt-6 flex flex-wrap gap-3">
                        <a class="btn btn-primary btn-lg" href=format!("{}/aufgaben", crate::BASE)>
                            <Icon glyph=Glyph::Add size=18 />
                            "Aufgabe hinzufügen"
                        </a>
                        <a class="btn btn-outline btn-lg" href=format!("{}/notizen", crate::BASE)>
                            <Icon glyph=Glyph::Notes size=18 />
                            "Notiz schreiben"
                        </a>
                    </div>
                </div>
                <HeroArt />
            </section>

            <div class="toolbar">
                <div class="segmented" role="tablist" aria-label="Bereich filtern">
                    {FILTERS
                        .iter()
                        .enumerate()
                        .map(|(i, name)| {
                            view! {
                                <button
                                    type="button"
                                    role="tab"
                                    class="segment"
                                    class:active=move || filter.get() == i
                                    aria-selected=move || (filter.get() == i).to_string()
                                    on:click=move |_| filter.set(i)
                                >
                                    {*name}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
                <button type="button" class="btn btn-outline" title="Zeitraum wählen kommt später">
                    <Icon glyph=Glyph::Calendar size=16 />
                    "Heute"
                    <Icon glyph=Glyph::ChevronDown size=16 />
                </button>
            </div>

            <div class="dash-grid">
                <div class="dash-main">
                    <SectionHead icon=Glyph::Tasks title="Heute fällig" />
                    <section class="panel">
                        <form class="quick-add" on:submit=|ev| ev.prevent_default()>
                            <Icon glyph=Glyph::Add size=18 />
                            <input
                                type="text"
                                aria-label="Schnell hinzufügen"
                                placeholder=move || PLACEHOLDERS[ph.get()]
                            />
                            <kbd>"Enter"</kbd>
                        </form>
                        <p class="empty-line">"Noch keine Aufgaben für heute. Ein perfekter Start!"</p>
                    </section>

                    <SectionHead icon=Glyph::Level title="Fortschritt" />
                    <div class="metric-grid">
                        <section class="panel">
                            <p class="metric-label">"Erledigt diese Woche"</p>
                            <p class="metric-sub">"Aufgaben pro Tag"</p>
                            <p class="metric-value">"0" <span class="metric-unit">"Aufgaben"</span></p>
                            <div class="week-bars" aria-hidden="true">
                                {WEEK
                                    .iter()
                                    .enumerate()
                                    .map(|(i, d)| {
                                        view! {
                                            <div class="week-col">
                                                <span class="week-bar" class:today=i == today></span>
                                                <span class="week-day" class:today=i == today>{*d}</span>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </section>
                        <section class="panel">
                            <p class="metric-label">"Dein Level"</p>
                            <p class="metric-sub">"XP für erledigte Aufgaben"</p>
                            <p class="metric-value">{progress.level} <span class="metric-unit">"Level"</span></p>
                            <div class="xp-bar mt-5">
                                <div class="xp-fill" style=format!("width:{}%", progress.percent().max(2.0))></div>
                            </div>
                            <p class="metric-foot">{progress.current} " / " {progress.needed} " XP bis Level " {progress.level + 1}</p>
                        </section>
                    </div>
                </div>

                <aside class="dash-side">
                    <SectionHead icon=Glyph::Calendar title="Nächste Termine" />
                    <section class="panel list-panel">
                        <div class="list-empty">
                            <span class="list-icon"><Icon glyph=Glyph::Clock size=18 /></span>
                            <span>
                                <span class="block font-semibold">"Keine Termine"</span>
                                <span class="block text-sm text-muted">"Dein Kalender ist frei."</span>
                            </span>
                        </div>
                    </section>

                    <SectionHead icon=Glyph::Streak title="Schnellstatistik" />
                    <div class="stat-grid">
                        <Stat icon=Glyph::Streak value="0" label="Tage Streak" />
                        <Stat icon=Glyph::Tasks value="0" label="Offen" />
                        <Stat icon=Glyph::Shopping value="0" label="Einkäufe" />
                        <Stat icon=Glyph::Pin value="0" label="Angepinnt" />
                    </div>
                </aside>
            </div>
        </div>
    }
}

#[component]
fn SectionHead(icon: Glyph, title: &'static str) -> impl IntoView {
    view! {
        <h2 class="section-head">
            <Icon glyph=icon size=18 />
            {title}
        </h2>
    }
}

#[component]
fn Stat(icon: Glyph, value: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <section class="panel stat">
            <span class="list-icon"><Icon glyph=icon size=16 /></span>
            <span>
                <span class="stat-value">{value}</span>
                <span class="stat-label">{label}</span>
            </span>
        </section>
    }
}

/// Soft layered hills in the brand colours (decorative).
#[component]
fn HeroArt() -> impl IntoView {
    view! {
        <svg class="hero-art" viewBox="0 0 600 300" preserveAspectRatio="xMidYMid slice" aria-hidden="true">
            <defs>
                <linearGradient id="sky" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0" class="sky-top" />
                    <stop offset="1" class="sky-bottom" />
                </linearGradient>
            </defs>
            <rect width="600" height="300" fill="url(#sky)" />
            <circle class="hero-sun" cx="430" cy="105" r="42" />
            <path class="hill hill-1" d="M0 190 C90 140 170 150 250 175 S420 120 600 150 V300 H0Z" />
            <path class="hill hill-2" d="M0 225 C110 180 210 205 300 215 S470 170 600 200 V300 H0Z" />
            <path class="hill hill-3" d="M0 260 C120 230 240 250 330 255 S500 225 600 245 V300 H0Z" />
        </svg>
    }
}

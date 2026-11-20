use checko_core::{done_this_week, level_for_xp, open_tasks, today_tasks};
use leptos::prelude::*;

use crate::components::{
    area_filter::AreaFilter,
    icon::{Glyph, Icon},
    task_form::TaskForm,
    task_item::TaskItem,
    topbar::Topbar,
};
use crate::store::Store;
use crate::time::{greeting, today, today_label, weekday_index};

const WEEK: [&str; 7] = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];

#[component]
pub fn TodayPage() -> impl IntoView {
    let store = Store::get();
    let filter = RwSignal::new(None::<String>);
    let area = Signal::derive(move || filter.get());
    let today_list = move || {
        let a = area.get();
        store.tasks.with(|t| {
            today_tasks(t, a.as_deref(), today())
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        })
    };
    let week = move || store.tasks.with(|t| done_this_week(t, today()));
    let week_total = move || week().iter().sum::<u32>();
    let progress = move || store.profile.with(|p| level_for_xp(p.xp));
    let streak = move || store.profile.with(|p| p.current_streak(today()));
    let open_count = move || store.tasks.with(|t| open_tasks(t, None).len());
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
                <AreaFilter area=filter />
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
                        <TaskForm due_today=true area=area />
                        <Show
                            when=move || !today_list().is_empty()
                            fallback=|| view! { <p class="empty-line">"Für heute ist nichts fällig. Ein perfekter Start!"</p> }
                        >
                            <ul class="task-list mt-2">
                                <For
                                    each=today_list
                                    key=|t| (t.meta.id, t.meta.updated_at)
                                    children=|t| view! { <TaskItem task=t /> }
                                />
                            </ul>
                        </Show>
                    </section>

                    <SectionHead icon=Glyph::Level title="Fortschritt" />
                    <div class="metric-grid">
                        <section class="panel">
                            <p class="metric-label">"Erledigt diese Woche"</p>
                            <p class="metric-sub">"Aufgaben pro Tag"</p>
                            <p class="metric-value">{week_total} <span class="metric-unit">"Aufgaben"</span></p>
                            <div class="week-bars" aria-hidden="true">
                                {WEEK
                                    .iter()
                                    .enumerate()
                                    .map(|(i, d)| {
                                        let height = move || {
                                            let w = week();
                                            let max = (*w.iter().max().unwrap_or(&0)).max(1) as f32;
                                            format!("height:{}%", 6.0 + w[i] as f32 / max * 94.0)
                                        };
                                        view! {
                                            <div class="week-col" title=move || format!("{}: {} erledigt", d, week()[i])>
                                                <span class="week-bar" class:today=i == today class:filled=move || { week()[i] > 0 } style=height></span>
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
                            <p class="metric-value">{move || progress().level} <span class="metric-unit">"Level"</span></p>
                            <div class="xp-bar mt-5">
                                <div class="xp-fill" style=move || format!("width:{}%", progress().percent().max(2.0))></div>
                            </div>
                            <p class="metric-foot">
                                {move || {
                                    let p = progress();
                                    format!("{} / {} XP bis Level {}", p.current, p.needed, p.level + 1)
                                }}
                            </p>
                        </section>
                    </div>
                </div>

                <aside class="dash-side">
                    <SectionHead icon=Glyph::Calendar title="Nächste Termine" />
                    <UpcomingEvents />

                    <SectionHead icon=Glyph::Pin title="Kategorien" />
                    <CategoryLegend />

                    <SectionHead icon=Glyph::Level title="Erfolge" />
                    <AchievementGrid />

                    <SectionHead icon=Glyph::Streak title="Schnellstatistik" />
                    <div class="stat-grid">
                        <Stat icon=Glyph::Streak value=Signal::derive(move || streak().to_string()) label="Tage Streak" />
                        <Stat icon=Glyph::Tasks value=Signal::derive(move || open_count().to_string()) label="Offen" />
                        <Stat icon=Glyph::Shopping value=Signal::derive(move || store.shopping.with(|s| checko_core::open_count(s)).to_string()) label="Einkäufe" />
                        <Stat icon=Glyph::Pin value=Signal::derive(move || store.notes.with(|n| n.iter().filter(|n| n.pinned && n.meta.deleted_at.is_none()).count()).to_string()) label="Angepinnt" />
                    </div>
                </aside>
            </div>
        </div>
    }
}

/// Next events and dated tasks (14 days), max. 5.
#[component]
fn UpcomingEvents() -> impl IntoView {
    let store = Store::get();
    let items = move || {
        let from = today();
        store.events.with(|e| {
            store.tasks.with(|t| {
                checko_core::items_between(e, t, from, from + chrono::Duration::days(14))
                    .into_iter()
                    .filter(|i| {
                        !i.done && (i.kind == checko_core::ItemKind::Event || i.start.is_some())
                    })
                    .take(5)
                    .collect::<Vec<_>>()
            })
        })
    };
    view! {
        <section class="panel list-panel">
            <Show
                when=move || !items().is_empty()
                fallback=|| view! {
                    <div class="list-empty">
                        <span class="list-icon"><Icon glyph=Glyph::Clock size=18 /></span>
                        <span>
                            <span class="block font-semibold">"Keine Termine"</span>
                            <span class="block text-sm text-muted">"Dein Kalender ist frei."</span>
                        </span>
                    </div>
                }
            >
                <ul class="upcoming">
                    {move || items()
                        .into_iter()
                        .map(|i| {
                            let when = crate::time::due_label(i.date, today());
                            let time = i.start.map(|t| t.format(" · %H:%M").to_string()).unwrap_or_default();
                            let color = i.category.as_deref().map(crate::ui::area_color).unwrap_or("var(--green)");
                            view! {
                                <li>
                                    <a class="upcoming-item" href=format!("{}/kalender", crate::BASE) style=format!("--c:{color}")>
                                        <span class="agenda-bar" aria-hidden="true"></span>
                                        <span class="min-w-0">
                                            <span class="block font-medium truncate">{i.title.clone()}</span>
                                            <span class="block text-sm text-muted">{when}{time}</span>
                                        </span>
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </Show>
        </section>
    }
}

/// All achievements; reached ones in colour, others greyed out.
#[component]
fn AchievementGrid() -> impl IntoView {
    let store = Store::get();
    let unlocked = move |id: &str| {
        store
            .profile
            .with(|p| p.achievements.iter().any(|a| a == id))
    };
    let count = move || store.profile.with(|p| p.achievements.len());
    view! {
        <section class="panel">
            <p class="text-sm text-muted mb-3">
                {move || format!("{} von {} freigeschaltet", count(), checko_core::ACHIEVEMENTS.len())}
            </p>
            <ul class="badge-grid">
                {checko_core::ACHIEVEMENTS
                    .iter()
                    .map(|a| {
                        let id = a.id;
                        view! {
                            <li
                                class="badge"
                                class:locked=move || !unlocked(id)
                                title=format!("{} – {}", a.title, a.description)
                            >
                                <span class="badge-emoji" aria-hidden="true">{a.emoji}</span>
                                <span class="badge-title">{a.title}</span>
                                <span class="sr-only">
                                    {move || if unlocked(id) { "freigeschaltet" } else { "noch gesperrt" }}
                                </span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}

/// Categories with open count and a progress bar (done / all), clickable.
#[component]
fn CategoryLegend() -> impl IntoView {
    let store = Store::get();
    let stats = move || store.tasks.with(|t| checko_core::category_stats(t));
    view! {
        <section class="panel legend">
            <Show
                when=move || !stats().is_empty()
                fallback=|| view! { <p class="text-sm text-muted">"Schreib „#Küche“ hinter eine Aufgabe, dann erscheint die Kategorie hier."</p> }
            >
                <ul>
                    {move || stats()
                        .into_iter()
                        .map(|(name, open, done)| {
                            let total = open + done;
                            let pct = done * 100 / total.max(1);
                            let color = crate::ui::area_color(&name);
                            let href = format!("{}/aufgaben?kategorie={}", crate::BASE, js_sys::encode_uri_component(&name));
                            view! {
                                <li>
                                    <a class="legend-item" href=href>
                                        <span class="legend-row">
                                            <span class="area-dot" style=format!("background:{color}")></span>
                                            <span class="flex-1 font-medium">{name.clone()}</span>
                                            <span class="legend-value">{open} " offen"</span>
                                            <span class="legend-pct">{pct} "%"</span>
                                        </span>
                                        <span class="legend-bar">
                                            <span style=format!("width:{pct}%;background:{color}")></span>
                                        </span>
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </Show>
        </section>
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
fn Stat(icon: Glyph, #[prop(into)] value: Signal<String>, label: &'static str) -> impl IntoView {
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

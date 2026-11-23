use checko_core::{done_last_days, level_for_xp, today_tasks};
use chrono::Datelike;
use leptos::prelude::*;

use crate::components::{
    area_filter::AreaFilter,
    icon::{Glyph, Icon},
    task_form::TaskForm,
    task_item::TaskItem,
    topbar::Topbar,
};
use crate::store::Store;
use crate::time::{greeting, today, today_label};

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
    let today_appts = move || {
        let mut v: Vec<_> = today_list()
            .into_iter()
            .filter(|t| t.is_appointment())
            .collect();
        v.sort_by_key(|t| (t.due, t.due_time.is_some(), t.due_time));
        v
    };
    let today_todos = move || {
        today_list()
            .into_iter()
            .filter(|t| !t.is_appointment())
            .collect::<Vec<_>>()
    };
    let progress = move || store.profile.with(|p| level_for_xp(p.xp));

    view! {
        <Topbar />
        <div class="page">
            <section class="hero">
                <div class="hero-text">
                    <p class="hero-kicker">{today_label()}</p>
                    <h1 class="hero-title">{format!("{}!", greeting())}</h1>
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
                <HeroUpcoming />
            </section>

            <div class="toolbar">
                <AreaFilter area=filter />
            </div>

            <div class="dash-grid">
                <div class="dash-main">
                    <Show when=move || !today_appts().is_empty()>
                        <SectionHead icon=Glyph::Calendar title="Termine heute" />
                        <section class="panel appt-panel">
                            <ul class="task-list">
                                <For
                                    each=today_appts
                                    key=|t| (t.meta.id, t.meta.updated_at)
                                    children=|t| view! { <TaskItem task=t /> }
                                />
                            </ul>
                        </section>
                    </Show>
                    <SectionHead icon=Glyph::Tasks title="Heute zu tun" />
                    <section class="panel">
                        <TaskForm due_today=true area=area />
                        <Show
                            when=move || !today_todos().is_empty()
                            fallback=|| view! { <p class="empty-line">"Für heute ist nichts zu tun. Ein perfekter Start!"</p> }
                        >
                            <ul class="task-list mt-2">
                                <For
                                    each=today_todos
                                    key=|t| (t.meta.id, t.meta.updated_at)
                                    children=|t| view! { <TaskItem task=t /> }
                                />
                            </ul>
                        </Show>
                    </section>
                </div>

                <aside class="dash-side">
                    <SectionHead icon=Glyph::Calendar title="Nächste Termine" />
                    <UpcomingEvents />
                </aside>
            </div>

            <section class="play-zone" aria-label="Fortschritt, Erfolge und Kategorien">
                <div class="play-grid">
                    <div class="play-main">
                        <SectionHead icon=Glyph::Level title="Fortschritt & Erfolge" />
                        <ProgressCard />
                        <div class="play-row">
                            <AchievementGrid />
                        <section class="panel level-card">
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
                    <div class="play-side">
                        <SectionHead icon=Glyph::Pin title="Kategorien" />
                        <CategoryArcs />
                    </div>
                </div>
            </section>
        </div>
    }
}

/// Streak + completions of the last 14 days as a smooth area chart (bklit-style).
#[component]
fn ProgressCard() -> impl IntoView {
    let store = Store::get();
    let streak = move || store.profile.with(|p| p.current_streak(today()));
    let days = move || store.tasks.with(|t| done_last_days(t, today(), 14));
    let week_total = move || days()[7..].iter().sum::<u32>();
    const W: f64 = 300.0;
    const H: f64 = 96.0;
    let points = move || {
        let d = days();
        let max = (*d.iter().max().unwrap_or(&0)).max(1) as f64;
        d.iter()
            .enumerate()
            .map(|(i, v)| {
                let x = i as f64 * W / 13.0;
                let y = H - 8.0 - *v as f64 / max * (H - 22.0);
                (x, y)
            })
            .collect::<Vec<_>>()
    };
    // Catmull-Rom → cubic Bézier for a soft curve
    let line = move || {
        let p = points();
        let mut d = format!("M{:.1} {:.1}", p[0].0, p[0].1);
        for i in 0..p.len() - 1 {
            let p0 = if i == 0 { p[0] } else { p[i - 1] };
            let (p1, p2) = (p[i], p[i + 1]);
            let p3 = if i + 2 < p.len() { p[i + 2] } else { p2 };
            let c1 = (p1.0 + (p2.0 - p0.0) / 6.0, p1.1 + (p2.1 - p0.1) / 6.0);
            let c2 = (p2.0 - (p3.0 - p1.0) / 6.0, p2.1 - (p3.1 - p1.1) / 6.0);
            d.push_str(&format!(
                " C{:.1} {:.1} {:.1} {:.1} {:.1} {:.1}",
                c1.0, c1.1, c2.0, c2.1, p2.0, p2.1
            ));
        }
        d
    };
    let area = move || format!("{} L{W} {H} L0 {H} Z", line());
    let labels = move || {
        let t = today();
        (0..14)
            .map(|i| {
                let d = t - chrono::Duration::days(13 - i);
                let wd = WEEK[d.weekday().num_days_from_monday() as usize];
                (wd, i == 13)
            })
            .collect::<Vec<_>>()
    };
    view! {
        <section class="panel progress-card" class:lit=move || { streak() > 0 }>
            <div class="progress-stats">
                <div class="progress-stat">
                    <span class="streak-flame" aria-hidden="true">"🔥"</span>
                    <span>
                        <span class="streak-num">{streak}</span>
                        <span class="streak-label">{move || if streak() == 1 { "Tag in Folge" } else { "Tage in Folge" }}</span>
                    </span>
                </div>
                <div class="progress-stat">
                    <span>
                        <span class="progress-num">{week_total}</span>
                        <span class="streak-label">"erledigt in 7 Tagen"</span>
                    </span>
                </div>
            </div>
            <svg class="area-chart" viewBox=format!("0 0 {W} {H}") preserveAspectRatio="none" role="img"
                aria-label=move || format!("Erledigte Aufgaben der letzten 14 Tage: {}", days().iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))>
                <defs>
                    <linearGradient id="area-fill" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0" stop-color="#22c55e" stop-opacity="0.45" />
                        <stop offset="1" stop-color="#22c55e" stop-opacity="0" />
                    </linearGradient>
                </defs>
                <line class="area-grid" x1="0" x2=W y1=H - 8.0 y2=H - 8.0 />
                <path d=area fill="url(#area-fill)" />
                <path d=line class="area-line" />
            </svg>
            <div class="area-labels" aria-hidden="true">
                {move || labels().into_iter().map(|(d, is_today)| view! { <span class:today=is_today>{d}</span> }).collect_view()}
            </div>
        </section>
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
                    .filter(|i| !i.done && i.appointment)
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
                                <img class="badge-medal" src=crate::ui::achievement_icon(a.id) alt="" width="128" height="128" loading="lazy" />
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

/// Categories as simple concentric half circles: arc length = share done.
#[component]
fn CategoryArcs() -> impl IntoView {
    let store = Store::get();
    let stats = move || {
        let mut v = store.tasks.with(|t| checko_core::category_stats(t));
        v.retain(|(_, open, _)| *open > 0);
        v.sort_by_key(|(_, open, _)| std::cmp::Reverse(*open));
        v.truncate(4);
        v
    };
    view! {
        <section class="panel arcs-panel">
            <Show
                when=move || !stats().is_empty()
                fallback=|| view! { <p class="text-sm text-muted">"Noch keine Kategorien mit offenen Aufgaben."</p> }
            >
                {move || {
                    let s = stats();
                    let arcs = s
                        .iter()
                        .enumerate()
                        .map(|(i, (name, open, done))| {
                            let r = 92.0 - i as f64 * 15.0;
                            let pct = (*done * 100 / (open + done).max(1)) as f64;
                            let d = format!("M{} 104 A{r} {r} 0 0 1 {} 104", 110.0 - r, 110.0 + r);
                            let color = crate::ui::area_color(name);
                            view! {
                                <path class="arc-track" d=d.clone() />
                                <path
                                    class="arc-bar"
                                    d=d
                                    pathLength="100"
                                    stroke=color
                                    stroke-dasharray=format!("{} 100", pct.max(1.5))
                                    style=format!("animation-delay:{}ms", i * 90)
                                />
                            }
                        })
                        .collect_view();
                    let legend = s
                        .into_iter()
                        .map(|(name, open, done)| {
                            let color = crate::ui::area_color(&name);
                            let pct = done * 100 / (open + done).max(1);
                            let href = format!("{}/aufgaben?kategorie={}", crate::BASE, js_sys::encode_uri_component(&name));
                            view! {
                                <li>
                                    <a class="arc-legend" href=href>
                                        <span class="arc-dot" style=format!("background:{color}")></span>
                                        <span class="flex-1 truncate">{name.clone()}</span>
                                        <span class="text-muted">{open} " offen"</span>
                                        <span class="arc-pct">{pct} "%"</span>
                                    </a>
                                </li>
                            }
                        })
                        .collect_view();
                    view! {
                        <svg viewBox="0 0 220 118" class="arcs-svg" role="img" aria-label="Erledigt pro Kategorie">
                            {arcs}
                            <text x="8" y="116" class="arc-tick">"0%"</text>
                            <text x="110" y="8" class="arc-tick" text-anchor="middle">"50%"</text>
                            <text x="212" y="116" class="arc-tick" text-anchor="end">"100%"</text>
                        </svg>
                        <ul class="arc-list">{legend}</ul>
                    }
                }}
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

/// "Coming up": the next appointment with a countdown and a 14-day strip
/// that shows how many free days lie in between.
#[component]
fn HeroUpcoming() -> impl IntoView {
    let store = Store::get();
    const DAYS: i64 = 14;
    let data = move || {
        let from = today();
        let to = from + chrono::Duration::days(DAYS - 1);
        store.events.with(|e| {
            store.tasks.with(|t| {
                checko_core::items_between(e, t, from, to)
                    .into_iter()
                    .filter(|i| !i.done && i.appointment)
                    .collect::<Vec<_>>()
            })
        })
    };
    let next = move || {
        let now_t = crate::time::now_local_time();
        data()
            .into_iter()
            .find(|i| i.date > today() || i.start.is_none_or(|s| s >= now_t))
    };
    view! {
        <div class="upnext">
            {move || match next() {
                Some(i) => {
                    let days = (i.date - today()).num_days();
                    let when = match days {
                        0 => "Heute".to_string(),
                        1 => "Morgen".to_string(),
                        n => format!("in {n} Tagen"),
                    };
                    let detail = format!(
                        "{}{}",
                        crate::time::due_label(i.date, today()),
                        i.start.map(|t| t.format(" · %H:%M Uhr").to_string()).unwrap_or_default()
                    );
                    let color = i.category.as_deref().map(crate::ui::area_color).unwrap_or("#16a34a");
                    view! {
                        <div class="upnext-head">
                            <span class="upnext-kicker">"Als Nächstes"</span>
                            <span class="upnext-when" style=format!("--c:{color}")>{when}</span>
                        </div>
                        <p class="upnext-title">{i.title.clone()}</p>
                        <p class="upnext-detail">{detail}</p>
                    }
                        .into_any()
                }
                None => view! {
                    <div class="upnext-head"><span class="upnext-kicker">"Als Nächstes"</span></div>
                    <p class="upnext-title">"Die nächsten 14 Tage sind frei"</p>
                    <p class="upnext-detail">"Zeit für dich – oder trag etwas im Kalender ein."</p>
                }
                    .into_any(),
            }}
            <a class="upnext-strip" href=format!("{}/kalender", crate::BASE) aria-label="Zum Kalender">
                {move || {
                    let items = data();
                    let next_date = next().map(|i| i.date);
                    (0..DAYS)
                        .map(|k| {
                            let d = today() + chrono::Duration::days(k);
                            let day_items: Vec<_> = items.iter().filter(|i| i.date == d).collect();
                            let busy = !day_items.is_empty();
                            let tip = day_items.iter().map(|i| i.title.clone()).collect::<Vec<_>>().join(", ");
                            // free days between today and the next appointment
                            let gap = next_date.is_some_and(|n| d > today() && d < n);
                            let dots = day_items
                                .iter()
                                .take(3)
                                .map(|i| {
                                    let c = i.category.as_deref().map(crate::ui::area_color).unwrap_or("#16a34a");
                                    view! { <span class="strip-dot" style=format!("background:{c}")></span> }
                                })
                                .collect_view();
                            let wd = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"][chrono::Datelike::weekday(&d).num_days_from_monday() as usize];
                            view! {
                                <span
                                    class="strip-day"
                                    class:today={ k == 0 }
                                    class:busy=busy
                                    class:gap=gap
                                    class:next={ next_date == Some(d) }
                                    title=tip
                                >
                                    <span class="strip-wd">{wd}</span>
                                    <span class="strip-num">{chrono::Datelike::day(&d)}</span>
                                    <span class="strip-dots">{dots}</span>
                                </span>
                            }
                        })
                        .collect_view()
                }}
            </a>
            {move || {
                let free = next().map(|i| (i.date - today()).num_days() - 1).unwrap_or(0);
                (free > 0).then(|| view! { <p class="upnext-gap">{format!("{free} {} frei bis dahin", if free == 1 { "Tag" } else { "Tage" })}</p> })
            }}
        </div>
    }
}

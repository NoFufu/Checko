use checko_core::{
    items_between, month_grid, parse_quick_add, shift_month, CalendarItem, Event, ItemKind,
};
use chrono::{Datelike, Duration, NaiveDate, NaiveTime};
use leptos::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;

use crate::components::{
    icon::{Glyph, Icon},
    task_detail::repeat_from,
    topbar::Topbar,
};
use crate::store::Store;
use crate::time::{now, today};
use crate::ui::area_color;

const MONTHS: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];
const WEEKDAYS: [&str; 7] = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];
const WEEKDAYS_LONG: [&str; 7] = [
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
    "Sonntag",
];
const DRAG: &str = "application/x-checko-calendar";

fn day_title(d: NaiveDate) -> String {
    format!(
        "{}, {}. {}",
        WEEKDAYS_LONG[d.weekday().num_days_from_monday() as usize],
        d.day(),
        MONTHS[d.month0() as usize]
    )
}

fn time_range(i: &CalendarItem) -> Option<String> {
    let s = i.start?.format("%H:%M").to_string();
    Some(match i.end {
        Some(e) => format!("{s}–{}", e.format("%H:%M")),
        None => s,
    })
}

fn drag_data(i: &CalendarItem) -> String {
    let k = if i.kind == ItemKind::Event { "e" } else { "t" };
    format!("{k}:{}", i.id)
}

fn parse_drag(s: &str) -> Option<(ItemKind, Uuid)> {
    let (k, id) = s.split_once(':')?;
    let kind = if k == "e" {
        ItemKind::Event
    } else {
        ItemKind::Task
    };
    Some((kind, id.parse().ok()?))
}

#[derive(Clone, Copy, PartialEq)]
enum View {
    Month,
    Agenda,
}

#[component]
pub fn CalendarPage() -> impl IntoView {
    let store = Store::get();
    let month = RwSignal::new(shift_month(today(), 0));
    let selected = RwSignal::new(today());
    let view_mode = RwSignal::new(View::Month);
    // days picked by pressing and dragging the mouse across the month
    let range = RwSignal::new(None::<(NaiveDate, NaiveDate)>);
    let finish_range = move |_| {
        if let Some((a, b)) = range.get_untracked() {
            range.set(None);
            if a != b {
                let (from, to) = if a <= b { (a, b) } else { (b, a) };
                selected.set(from);
                store.quick_add_date.set(Some(from));
                store.quick_add_until.set(Some(to));
                store.quick_add_open.set(true);
            }
        }
    };

    let items = move |from: NaiveDate, to: NaiveDate| {
        store
            .events
            .with(|e| store.tasks.with(|t| items_between(e, t, from, to)))
    };
    let grid = move || {
        let m = month.get();
        month_grid(m.year(), m.month())
    };

    view! {
        <Topbar />
        <div class="page page-wide" on:pointerup=finish_range on:pointerleave=move |_| range.set(None)>
            <div class="page-head">
                <div>
                    <h1 class="page-h1">
                        {move || { let m = month.get(); format!("{} {}", MONTHS[m.month0() as usize], m.year()) }}
                    </h1>
                    <p class="text-muted">"Termine und fällige Aufgaben. Einträge lassen sich auf einen anderen Tag ziehen."</p>
                </div>
                <div class="flex flex-wrap items-center gap-2">
                    <div class="segmented" role="tablist" aria-label="Ansicht">
                        <button type="button" role="tab" aria-selected=move || (view_mode.get() == View::Month).to_string() class="segment" class:active=move || view_mode.get() == View::Month on:click=move |_| view_mode.set(View::Month)>"Monat"</button>
                        <button type="button" role="tab" aria-selected=move || (view_mode.get() == View::Agenda).to_string() class="segment" class:active=move || view_mode.get() == View::Agenda on:click=move |_| view_mode.set(View::Agenda)>"Agenda"</button>
                    </div>
                    <button type="button" class="btn-icon" aria-label="Vorheriger Monat" on:click=move |_| month.update(|m| *m = shift_month(*m, -1))>
                        <Icon glyph=Glyph::ChevronLeft size=18 />
                    </button>
                    <button type="button" class="btn btn-outline" on:click=move |_| { month.set(shift_month(today(), 0)); selected.set(today()); }>"Heute"</button>
                    <button type="button" class="btn-icon" aria-label="Nächster Monat" on:click=move |_| month.update(|m| *m = shift_month(*m, 1))>
                        <Icon glyph=Glyph::ChevronRight size=18 />
                    </button>
                </div>
            </div>

            <div class="cal-layout">
                {move || range.get().filter(|(a, b)| a != b).map(|(a, b)| {
                    let (a, b) = if a <= b { (a, b) } else { (b, a) };
                    view! { <p class="cal-range-hint" role="status">{format!("{} – {} · loslassen für Serientermin", a.format("%d.%m."), b.format("%d.%m."))}</p> }
                })}
                <Show
                    when=move || view_mode.get() == View::Month
                    fallback=move || view! { <Agenda selected=selected /> }
                >
                    <section class="panel cal-month" aria-label="Monatsansicht">
                        <div class="cal-weekdays">
                            {WEEKDAYS.iter().map(|d| view! { <span>{*d}</span> }).collect_view()}
                        </div>
                        <div class="cal-grid">
                            {move || {
                                let days = grid();
                                let (from, to) = (days[0], days[41]);
                                let all = items(from, to);
                                let m = month.get();
                                days.into_iter()
                                    .map(|d| {
                                        let day_items: Vec<CalendarItem> = all.iter().filter(|i| i.date == d).cloned().collect();
                                        view! { <DayCell day=d in_month=d.month() == m.month() items=day_items selected=selected range=range /> }
                                    })
                                    .collect_view()
                            }}
                        </div>
                    </section>
                </Show>

                <DayPanel selected=selected />
            </div>
        </div>
    }
}

#[component]
fn DayCell(
    day: NaiveDate,
    in_month: bool,
    items: Vec<CalendarItem>,
    selected: RwSignal<NaiveDate>,
    range: RwSignal<Option<(NaiveDate, NaiveDate)>>,
) -> impl IntoView {
    let in_range = move || {
        range
            .get()
            .is_some_and(|(a, b)| a != b && day >= a.min(b) && day <= a.max(b))
    };
    let store = Store::get();
    let over = RwSignal::new(false);
    let extra = items.len().saturating_sub(3);
    let shown: Vec<CalendarItem> = items.into_iter().take(3).collect();
    let is_today = day == today();

    view! {
        <div
            class="cal-day"
            class:outside=!in_month
            class:today=is_today
            class:selected=move || selected.get() == day
            class:drop-over=move || over.get()
            class:in-range=in_range
            title="Doppelklick: neuer Eintrag · über mehrere Tage ziehen: Serientermin"
            on:pointerdown=move |e| {
                // only a plain press on the day itself (not on a chip) starts a range
                let on_chip = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()).is_some_and(|el| el.closest(".cal-chip").ok().flatten().is_some());
                if e.button() == 0 && e.pointer_type() == "mouse" && !on_chip {
                    range.set(Some((day, day)));
                }
            }
            on:pointerenter=move |_| range.update(|r| if let Some((_, end)) = r { *end = day; })
            on:click=move |_| selected.set(day)
            on:dblclick=move |_| {
                selected.set(day);
                store.quick_add_date.set(Some(day));
                store.quick_add_open.set(true);
            }
            on:dragover=move |e| { e.prevent_default(); over.set(true); }
            on:dragleave=move |_| over.set(false)
            on:drop=move |e| {
                e.prevent_default();
                over.set(false);
                if let Some((kind, id)) = e.data_transfer().and_then(|dt| dt.get_data(DRAG).ok()).as_deref().and_then(parse_drag) {
                    store.move_to_day(kind, id, day);
                    selected.set(day);
                }
            }
        >
            <button type="button" class="cal-num" aria-label=day_title(day) on:click=move |e| { e.stop_propagation(); selected.set(day) }>
                {day.day()}
            </button>
            <ul class="cal-items">
                {shown.into_iter().map(|i| view! { <li><ItemChip item=i /></li> }).collect_view()}
                {(extra > 0).then(|| view! { <li class="cal-more">{format!("+{extra} mehr")}</li> })}
            </ul>
        </div>
    }
}

#[component]
fn ItemChip(item: CalendarItem) -> impl IntoView {
    let data = drag_data(&item);
    let color = item
        .category
        .as_deref()
        .map(area_color)
        .unwrap_or_else(|| "var(--green)".to_string());
    // to-dos look like a little checkbox chip, appointments like a block
    let is_todo = !item.appointment;
    let store = Store::get();
    let (kind, id) = (item.kind, item.id);
    view! {
        <span
            class="cal-chip"
            role="button"
            tabindex="0"
            on:click=move |e| {
                e.stop_propagation();
                if kind == ItemKind::Event { store.event_detail.set(Some(id)) } else { store.detail.set(Some(id)) }
            }
            on:keydown=move |e| if e.key() == "Enter" {
                if kind == ItemKind::Event { store.event_detail.set(Some(id)) } else { store.detail.set(Some(id)) }
            }
            class:task=is_todo
            class:done=item.done
            draggable=crate::ui::fine_pointer().then_some("true")
            title=item.title.clone()
            style=format!("--c:{color}")
            on:dragstart=move |e| {
                if let Some(dt) = e.data_transfer() {
                    let _ = dt.set_data(DRAG, &data);
                    dt.set_effect_allowed("move");
                }
            }
        >
            {item.start.map(|s| view! { <span class="cal-time">{s.format("%H:%M").to_string()}</span> })}
            <span class="cal-title">{item.title.clone()}</span>
        </span>
    }
}

#[component]
fn DayPanel(selected: RwSignal<NaiveDate>) -> impl IntoView {
    let store = Store::get();
    let title = RwSignal::new(String::new());
    let start = RwSignal::new(String::new());
    let end = RwSignal::new(String::new());
    let repeat = RwSignal::new(String::new());

    let day_items = move || {
        let d = selected.get();
        store
            .events
            .with(|e| store.tasks.with(|t| items_between(e, t, d, d)))
    };

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let text = title.get_untracked();
        if text.trim().is_empty() {
            return;
        }
        // "Zahnarzt 14:00 #gesundheit jede Woche" works here too.
        let areas = store.areas();
        let p = parse_quick_add(&text, selected.get_untracked(), &areas);
        let mut e = Event::new(&p.title, p.due.unwrap_or(selected.get_untracked()), now());
        e.start = NaiveTime::parse_from_str(&start.get_untracked(), "%H:%M")
            .ok()
            .or(p.time);
        e.end = NaiveTime::parse_from_str(&end.get_untracked(), "%H:%M").ok();
        e.category = p.area;
        e.recurrence = repeat_from(&repeat.get_untracked()).or(p.recurrence);
        let date = e.date;
        store.add_event(e);
        selected.set(date);
        title.set(String::new());
        start.set(String::new());
        end.set(String::new());
        repeat.set(String::new());
    };

    view! {
        <aside class="panel cal-side" aria-label="Ausgewählter Tag">
            <h2 class="cal-side-title">{move || day_title(selected.get())}</h2>
            <Show
                when=move || !day_items().is_empty()
                fallback=|| view! { <p class="text-sm text-muted">"Nichts geplant."</p> }
            >
                <ul class="cal-list">
                    {move || day_items().into_iter().map(|i| view! { <AgendaRow item=i /> }).collect_view()}
                </ul>
            </Show>

            <form class="cal-form" on:submit=submit>
                <h3 class="drawer-label">"Termin hinzufügen"</h3>
                <div class="quick-add">
                    <Icon glyph=Glyph::Add size=18 />
                    <input type="text" placeholder="z. B. Zahnarzt 14:00 #Gesundheit" aria-label="Termin" bind:value=title />
                </div>
                <div class="flex flex-wrap gap-2">
                    <label class="field"><span class="sr-only">"Von"</span><input type="time" aria-label="Von" bind:value=start /></label>
                    <label class="field"><span class="sr-only">"Bis"</span><input type="time" aria-label="Bis" bind:value=end /></label>
                    <label class="field">
                        <span class="sr-only">"Wiederholen"</span>
                        <select aria-label="Wiederholen" bind:value=repeat>
                            <option value="">"Einmalig"</option>
                            <option value="d">"Jeden Tag"</option>
                            <option value="w">"Jede Woche"</option>
                            <option value="2w">"Alle 2 Wochen"</option>
                            <option value="m">"Jeden Monat"</option>
                            <option value="y">"Jedes Jahr"</option>
                        </select>
                    </label>
                </div>
                <button type="submit" class="btn btn-primary" disabled=move || title.get().trim().is_empty()>"Termin speichern"</button>
            </form>
            <details class="cal-import">
                <summary>"Outlook-Kalender importieren / entfernen"</summary>
                <crate::components::data_tools::IcsImport compact=true />
                <crate::components::data_tools::ImportList />
            </details>
        </aside>
    }
}

/// One line in the day panel / agenda: time, title, repeat, actions.
#[component]
fn AgendaRow(item: CalendarItem) -> impl IntoView {
    let store = Store::get();
    let id = item.id;
    let color = item
        .category
        .as_deref()
        .map(area_color)
        .unwrap_or_else(|| "var(--green)".to_string());
    let is_event = item.kind == ItemKind::Event;
    let time = time_range(&item).unwrap_or_else(|| "Ganztägig".into());
    view! {
        <li
            class="agenda-row"
            class:done=item.done
            class:todo=!item.appointment
            class:clickable=true
            style=format!("--c:{color}")
            title="Details öffnen"
            on:click=move |_| if is_event { store.event_detail.set(Some(id)) } else { store.detail.set(Some(id)) }
        >
            <span class="agenda-bar" aria-hidden="true"></span>
            <span class="agenda-time">{time}</span>
            <span class="flex-1 min-w-0">
                <span class="block font-medium agenda-name">{item.title.clone()}</span>
                <span class="block text-sm text-muted">
                    {if item.appointment { "Termin" } else { "Aufgabe" }}
                    {item.category.clone().map(|c| format!(" · {c}"))}
                    {item.repeats.then_some(" · wiederholt sich")}
                </span>
            </span>
            {if is_event {
                view! { <span class="agenda-open" aria-hidden="true"><Icon glyph=Glyph::ChevronDown size=16 /></span> }
                    .into_any()
            } else {
                let done = item.done;
                view! {
                    <button
                        type="button"
                        class="check agenda-check"
                        class:checked=done
                        aria-label=if done { "Wieder öffnen" } else { "Abhaken" }
                        on:click=move |e| {
                            e.stop_propagation();
                            if done { store.reopen(id) } else { store.complete(id) }
                        }
                    >
                        <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
                            <path d="M5 12.5l4.5 4.5L19 7.5" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
                        </svg>
                    </button>
                }
                    .into_any()
            }}
        </li>
    }
}

/// Next 30 days from the selected day, grouped by date.
#[component]
fn Agenda(selected: RwSignal<NaiveDate>) -> impl IntoView {
    let store = Store::get();
    let groups = move || {
        let from = selected.get();
        let to = from + Duration::days(30);
        let all = store
            .events
            .with(|e| store.tasks.with(|t| items_between(e, t, from, to)));
        let mut out: Vec<(NaiveDate, Vec<CalendarItem>)> = Vec::new();
        for i in all.into_iter().filter(|i| !i.done) {
            match out.last_mut() {
                Some((d, v)) if *d == i.date => v.push(i),
                _ => out.push((i.date, vec![i])),
            }
        }
        out
    };
    view! {
        <section class="panel cal-agenda" aria-label="Agenda">
            <Show
                when=move || !groups().is_empty()
                fallback=|| view! { <p class="empty-line">"In den nächsten 30 Tagen ist nichts geplant."</p> }
            >
                {move || groups()
                    .into_iter()
                    .map(|(d, items)| view! {
                        <div class="agenda-day">
                            <h3 class="agenda-date" class:today=d == today()>{day_title(d)}</h3>
                            <ul class="cal-list">
                                {items.into_iter().map(|i| view! { <AgendaRow item=i /> }).collect_view()}
                            </ul>
                        </div>
                    })
                    .collect_view()}
            </Show>
        </section>
    }
}

//! Calendar: events, simple repeat rules and the combined view of events + tasks.

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Meta, Priority, Task, TaskStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Freq {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// "Every `interval` days/weeks/months/years", optionally until a date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recurrence {
    pub freq: Freq,
    pub interval: u32,
    #[serde(default)]
    pub until: Option<NaiveDate>,
}

fn add_months(d: NaiveDate, months: u32) -> NaiveDate {
    let total = d.year() * 12 + d.month0() as i32 + months as i32;
    let (y, m) = (total / 12, total % 12 + 1);
    // clamp e.g. 31 Jan + 1 month -> 28/29 Feb
    (1..=d.day())
        .rev()
        .find_map(|day| NaiveDate::from_ymd_opt(y, m as u32, day))
        .unwrap_or(d)
}

impl Recurrence {
    pub fn every(freq: Freq) -> Self {
        Self {
            freq,
            interval: 1,
            until: None,
        }
    }

    /// The n-th occurrence after `start` (n = 0 is `start` itself).
    fn nth(&self, start: NaiveDate, n: u32) -> NaiveDate {
        let k = n * self.interval.max(1);
        match self.freq {
            Freq::Daily => start + Duration::days(k as i64),
            Freq::Weekly => start + Duration::weeks(k as i64),
            Freq::Monthly => add_months(start, k),
            Freq::Yearly => add_months(start, k * 12),
        }
    }

    /// First occurrence strictly after `after`.
    pub fn next_after(&self, start: NaiveDate, after: NaiveDate) -> Option<NaiveDate> {
        (0..100_000)
            .map(|n| self.nth(start, n))
            .take_while(|d| self.until.is_none_or(|u| *d <= u))
            .find(|d| *d > after)
    }

    /// All occurrences in `from..=to`.
    pub fn occurrences(&self, start: NaiveDate, from: NaiveDate, to: NaiveDate) -> Vec<NaiveDate> {
        (0..100_000)
            .map(|n| self.nth(start, n))
            .take_while(|d| *d <= to && self.until.is_none_or(|u| *d <= u))
            .filter(|d| *d >= from)
            .collect()
    }

    /// German label, e.g. "Jede Woche", "Alle 2 Monate".
    pub fn label(&self) -> String {
        let (one, many) = match self.freq {
            Freq::Daily => ("Jeden Tag", "Tage"),
            Freq::Weekly => ("Jede Woche", "Wochen"),
            Freq::Monthly => ("Jeden Monat", "Monate"),
            Freq::Yearly => ("Jedes Jahr", "Jahre"),
        };
        if self.interval <= 1 {
            one.to_string()
        } else {
            format!("Alle {} {many}", self.interval)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub meta: Meta,
    pub title: String,
    pub date: NaiveDate,
    #[serde(default)]
    pub start: Option<NaiveTime>,
    #[serde(default)]
    pub end: Option<NaiveTime>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub recurrence: Option<Recurrence>,
}

impl Event {
    pub fn new(title: &str, date: NaiveDate, now: DateTime<Utc>) -> Self {
        Self {
            meta: Meta::new(now),
            title: title.trim().to_string(),
            date,
            start: None,
            end: None,
            category: None,
            notes: String::new(),
            recurrence: None,
        }
    }

    pub fn dates_between(&self, from: NaiveDate, to: NaiveDate) -> Vec<NaiveDate> {
        match self.recurrence {
            Some(r) => r.occurrences(self.date, from, to),
            None if self.date >= from && self.date <= to => vec![self.date],
            None => Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Event,
    Task,
}

/// One entry on a calendar day (an event occurrence or a task due that day).
#[derive(Debug, Clone, PartialEq)]
pub struct CalendarItem {
    pub kind: ItemKind,
    pub id: Uuid,
    pub title: String,
    pub date: NaiveDate,
    pub start: Option<NaiveTime>,
    pub end: Option<NaiveTime>,
    pub category: Option<String>,
    pub done: bool,
    pub priority: Option<Priority>,
    pub repeats: bool,
    /// Time block look (events and appointment tasks) instead of a to-do chip.
    pub appointment: bool,
}

/// Events and due tasks between two dates, sorted by date and time
/// (open before done, all-day items first within a day).
pub fn items_between(
    events: &[Event],
    tasks: &[Task],
    from: NaiveDate,
    to: NaiveDate,
) -> Vec<CalendarItem> {
    let mut out = Vec::new();
    for e in events.iter().filter(|e| e.meta.deleted_at.is_none()) {
        for d in e.dates_between(from, to) {
            out.push(CalendarItem {
                kind: ItemKind::Event,
                id: e.meta.id,
                title: e.title.clone(),
                date: d,
                start: e.start,
                end: e.end,
                category: e.category.clone(),
                done: false,
                priority: None,
                repeats: e.recurrence.is_some(),
                appointment: true,
            });
        }
    }
    for t in tasks.iter().filter(|t| !t.is_deleted()) {
        let Some(d) = t.due else { continue };
        if d < from || d > to {
            continue;
        }
        out.push(CalendarItem {
            kind: ItemKind::Task,
            id: t.meta.id,
            title: t.title.clone(),
            date: d,
            start: t.due_time,
            end: None,
            category: t.area.clone(),
            done: t.status == TaskStatus::Done,
            priority: Some(t.priority),
            repeats: t.recurrence.is_some(),
            appointment: t.is_appointment(),
        });
    }
    // per day: open before done, all-day before timed, then by time
    out.sort_by(|a, b| {
        (
            a.date,
            a.done,
            a.start.is_some(),
            a.start,
            a.kind == ItemKind::Task,
        )
            .cmp(&(
                b.date,
                b.done,
                b.start.is_some(),
                b.start,
                b.kind == ItemKind::Task,
            ))
    });
    out
}

/// 6 weeks × 7 days for a month view, starting on the Monday on/before the 1st.
pub fn month_grid(year: i32, month: u32) -> Vec<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).expect("valid month");
    let start = first - Duration::days(first.weekday().num_days_from_monday() as i64);
    (0..42).map(|i| start + Duration::days(i)).collect()
}

/// Same day in the previous / next month (clamped), for month navigation.
pub fn shift_month(d: NaiveDate, delta: i32) -> NaiveDate {
    let first = NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap_or(d);
    if delta >= 0 {
        add_months(first, delta as u32)
    } else {
        let total = first.year() * 12 + first.month0() as i32 + delta;
        NaiveDate::from_ymd_opt(total.div_euclid(12), (total.rem_euclid(12) + 1) as u32, 1)
            .unwrap_or(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn recurrence_steps() {
        let weekly = Recurrence::every(Freq::Weekly);
        assert_eq!(
            weekly.next_after(d(2026, 11, 2), d(2026, 11, 20)),
            Some(d(2026, 11, 23))
        );
        let monthly = Recurrence::every(Freq::Monthly);
        assert_eq!(
            monthly.next_after(d(2026, 1, 31), d(2026, 1, 31)),
            Some(d(2026, 2, 28))
        );
        let two_days = Recurrence {
            freq: Freq::Daily,
            interval: 2,
            until: Some(d(2026, 11, 6)),
        };
        assert_eq!(
            two_days.occurrences(d(2026, 11, 1), d(2026, 11, 1), d(2026, 11, 30)),
            [d(2026, 11, 1), d(2026, 11, 3), d(2026, 11, 5)]
        );
        assert_eq!(two_days.next_after(d(2026, 11, 1), d(2026, 11, 5)), None);
        assert_eq!(
            Recurrence::every(Freq::Yearly).next_after(d(2024, 2, 29), d(2024, 3, 1)),
            Some(d(2025, 2, 28))
        );
        assert_eq!(
            Recurrence {
                freq: Freq::Weekly,
                interval: 2,
                until: None
            }
            .label(),
            "Alle 2 Wochen"
        );
    }

    #[test]
    fn grid_and_navigation() {
        let g = month_grid(2026, 11);
        assert_eq!(g.len(), 42);
        assert_eq!(g[0], d(2026, 10, 26)); // Monday before 1 Nov 2026 (a Sunday)
        assert_eq!(shift_month(d(2026, 11, 20), 1), d(2026, 12, 1));
        assert_eq!(shift_month(d(2026, 1, 15), -1), d(2025, 12, 1));
    }

    #[test]
    fn combined_items() {
        let now = Utc::now();
        let mut e = Event::new("Yoga", d(2026, 11, 2), now);
        e.start = NaiveTime::from_hms_opt(18, 0, 0);
        e.recurrence = Some(Recurrence::every(Freq::Weekly));
        let mut t = Task::new("Steuer", now);
        t.due = Some(d(2026, 11, 16));
        let items = items_between(&[e], &[t], d(2026, 11, 16), d(2026, 11, 22));
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].kind, ItemKind::Task); // all-day first
        assert!(items[1].repeats);
    }

    #[test]
    fn done_items_come_last() {
        let now = chrono::Utc::now();
        let day = NaiveDate::from_ymd_opt(2026, 11, 23).unwrap();
        let mut done = Task::new("IHK Veranstaltung", now);
        done.due = Some(day);
        done.status = TaskStatus::Done;
        let mut open = Task::new("Neue Mail", now);
        open.due = Some(day);
        open.due_time = NaiveTime::from_hms_opt(7, 30, 0);
        let items = items_between(&[], &[done, open], day, day);
        assert_eq!(items[0].title, "Neue Mail");
        assert!(items[1].done);
    }
}

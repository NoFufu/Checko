//! "Geplant" view: open tasks with a date, grouped like in Microsoft To Do.

use chrono::{Datelike, Duration, NaiveDate};

use crate::{open_tasks, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanGroup {
    Overdue,
    Today,
    Tomorrow,
    ThisWeek,
    NextWeek,
    Later,
}

impl PlanGroup {
    pub fn label(self) -> &'static str {
        match self {
            PlanGroup::Overdue => "Überfällig",
            PlanGroup::Today => "Heute",
            PlanGroup::Tomorrow => "Morgen",
            PlanGroup::ThisWeek => "Diese Woche",
            PlanGroup::NextWeek => "Nächste Woche",
            PlanGroup::Later => "Später",
        }
    }

    /// Group for a due date (weeks start on Monday).
    pub fn of(due: NaiveDate, today: NaiveDate) -> Self {
        let sunday = today + Duration::days(6 - today.weekday().num_days_from_monday() as i64);
        match due {
            d if d < today => PlanGroup::Overdue,
            d if d == today => PlanGroup::Today,
            d if d == today + Duration::days(1) => PlanGroup::Tomorrow,
            d if d <= sunday => PlanGroup::ThisWeek,
            d if d <= sunday + Duration::days(7) => PlanGroup::NextWeek,
            _ => PlanGroup::Later,
        }
    }
}

/// Open tasks with a due date, grouped and sorted by date and time.
/// Empty groups are left out.
pub fn planned_groups<'a>(
    tasks: &'a [Task],
    area: Option<&str>,
    today: NaiveDate,
) -> Vec<(PlanGroup, Vec<&'a Task>)> {
    let mut dated: Vec<&Task> = open_tasks(tasks, area)
        .into_iter()
        .filter(|t| t.due.is_some())
        .collect();
    // stable sort keeps the priority order within one day and time
    dated.sort_by_key(|t| (t.due, t.due_time.is_some(), t.due_time));
    let mut out: Vec<(PlanGroup, Vec<&Task>)> = Vec::new();
    for t in dated {
        let g = PlanGroup::of(t.due.unwrap_or(today), today);
        match out.last_mut() {
            Some((last, v)) if *last == g => v.push(t),
            _ => out.push((g, vec![t])),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveTime, Utc};

    // Thursday, 26.11.2026
    fn day(n: i64) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, 26).unwrap() + Duration::days(n)
    }

    #[test]
    fn groups_by_date() {
        let today = day(0);
        assert_eq!(PlanGroup::of(day(-3), today), PlanGroup::Overdue);
        assert_eq!(PlanGroup::of(day(0), today), PlanGroup::Today);
        assert_eq!(PlanGroup::of(day(1), today), PlanGroup::Tomorrow);
        assert_eq!(PlanGroup::of(day(3), today), PlanGroup::ThisWeek); // Sunday
        assert_eq!(PlanGroup::of(day(4), today), PlanGroup::NextWeek); // Monday
        assert_eq!(PlanGroup::of(day(10), today), PlanGroup::NextWeek); // Sunday
        assert_eq!(PlanGroup::of(day(11), today), PlanGroup::Later);
    }

    #[test]
    fn on_sunday_tomorrow_is_next_week() {
        let sunday = day(3);
        assert_eq!(PlanGroup::of(day(4), sunday), PlanGroup::Tomorrow);
        assert_eq!(PlanGroup::of(day(5), sunday), PlanGroup::NextWeek);
    }

    #[test]
    fn sorted_and_without_undated_or_done() {
        let now = Utc::now();
        let mk = |title: &str, due: Option<i64>, time: Option<u32>| {
            let mut t = Task::new(title, now);
            t.due = due.map(day);
            t.due_time = time.map(|h| NaiveTime::from_hms_opt(h, 0, 0).unwrap());
            t
        };
        let mut done = mk("fertig", Some(0), None);
        done.complete(now, day(0));
        let tasks = vec![
            mk("später", Some(20), None),
            mk("heute 15", Some(0), Some(15)),
            mk("ohne Datum", None, None),
            mk("heute 9", Some(0), Some(9)),
            mk("heute", Some(0), None),
            mk("alt", Some(-2), None),
            done,
        ];
        let g = planned_groups(&tasks, None, day(0));
        let titles: Vec<(PlanGroup, Vec<&str>)> = g
            .iter()
            .map(|(g, v)| (*g, v.iter().map(|t| t.title.as_str()).collect()))
            .collect();
        assert_eq!(
            titles,
            vec![
                (PlanGroup::Overdue, vec!["alt"]),
                (PlanGroup::Today, vec!["heute", "heute 9", "heute 15"]),
                (PlanGroup::Later, vec!["später"]),
            ]
        );
    }
}

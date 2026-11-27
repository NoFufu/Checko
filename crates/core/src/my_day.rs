//! "Mein Tag": a daily list the user picks by hand, plus suggestions
//! (like in Microsoft To Do). The list starts empty every morning.

use chrono::{Duration, NaiveDate};

use crate::{open_tasks, Task, TaskStatus};

/// Why a task is suggested for today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suggestion {
    Overdue,
    DueToday,
    InProgress,
    DueSoon,
}

impl Suggestion {
    pub fn label(self) -> &'static str {
        match self {
            Suggestion::Overdue => "Überfällig",
            Suggestion::DueToday => "Heute fällig",
            Suggestion::InProgress => "In Arbeit",
            Suggestion::DueSoon => "Bald fällig",
        }
    }
}

impl Task {
    pub fn in_my_day(&self, today: NaiveDate) -> bool {
        self.my_day == Some(today) && !self.is_deleted()
    }
}

/// Tasks in "Mein Tag": open ones first (sorted like everywhere), then those
/// already done today.
pub fn my_day_tasks(tasks: &[Task], today: NaiveDate) -> Vec<&Task> {
    let mut v: Vec<&Task> = open_tasks(tasks, None)
        .into_iter()
        .filter(|t| t.in_my_day(today))
        .collect();
    v.extend(
        tasks
            .iter()
            .filter(|t| t.in_my_day(today) && t.status == TaskStatus::Done),
    );
    v
}

/// Open tasks not yet in "Mein Tag" that are worth looking at today:
/// overdue, due today, in progress, due within the next 3 days.
pub fn my_day_suggestions(tasks: &[Task], today: NaiveDate) -> Vec<(Suggestion, &Task)> {
    let soon = today + Duration::days(3);
    let mut v: Vec<(Suggestion, &Task)> = open_tasks(tasks, None)
        .into_iter()
        .filter(|t| !t.in_my_day(today))
        .filter_map(|t| {
            let s = match t.due {
                Some(d) if d < today => Suggestion::Overdue,
                Some(d) if d == today => Suggestion::DueToday,
                _ if t.status == TaskStatus::InProgress => Suggestion::InProgress,
                Some(d) if d <= soon => Suggestion::DueSoon,
                _ => return None,
            };
            Some((s, t))
        })
        .collect();
    v.sort_by_key(|(s, _)| *s as u8);
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn day(n: i64) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, 27).unwrap() + Duration::days(n)
    }

    fn mk(title: &str, due: Option<i64>) -> Task {
        let mut t = Task::new(title, Utc::now());
        t.due = due.map(day);
        t
    }

    #[test]
    fn list_resets_next_day() {
        let mut t = mk("a", None);
        t.my_day = Some(day(0));
        assert!(t.in_my_day(day(0)));
        assert!(!t.in_my_day(day(1)));
    }

    #[test]
    fn open_first_then_done_today() {
        let mut a = mk("erledigt", None);
        a.my_day = Some(day(0));
        a.complete(Utc::now(), day(0));
        let mut b = mk("offen", None);
        b.my_day = Some(day(0));
        let c = mk("nicht drin", None);
        let tasks = vec![a, b, c];
        let titles: Vec<_> = my_day_tasks(&tasks, day(0))
            .iter()
            .map(|t| t.title.as_str())
            .collect();
        assert_eq!(titles, vec!["offen", "erledigt"]);
    }

    #[test]
    fn suggestions_by_reason() {
        let mut started = mk("läuft", None);
        started.status = TaskStatus::InProgress;
        let mut picked = mk("schon drin", Some(0));
        picked.my_day = Some(day(0));
        let tasks = vec![
            mk("bald", Some(2)),
            mk("später", Some(10)),
            mk("heute", Some(0)),
            mk("ohne", None),
            mk("alt", Some(-1)),
            started,
            picked,
        ];
        let got: Vec<_> = my_day_suggestions(&tasks, day(0))
            .into_iter()
            .map(|(s, t)| (s, t.title.as_str()))
            .collect();
        assert_eq!(
            got,
            vec![
                (Suggestion::Overdue, "alt"),
                (Suggestion::DueToday, "heute"),
                (Suggestion::InProgress, "läuft"),
                (Suggestion::DueSoon, "bald"),
            ]
        );
    }
}

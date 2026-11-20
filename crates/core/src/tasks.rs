//! Task life cycle and list helpers (pure, no browser code).

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use uuid::Uuid;

use crate::{xp_for_task, LogEntry, Profile, Task, TaskStatus};

/// Default life areas a task can belong to.
pub const AREAS: [&str; 3] = ["Arbeit", "Privat", "Gesundheit"];

impl Task {
    fn touch(&mut self, now: DateTime<Utc>, text: &str) {
        self.meta.updated_at = now;
        self.log.push(LogEntry {
            at: now,
            text: text.to_string(),
        });
    }

    pub fn is_deleted(&self) -> bool {
        self.meta.deleted_at.is_some()
    }

    pub fn is_open(&self) -> bool {
        !self.is_deleted() && self.status != TaskStatus::Done
    }

    /// Open and due today or earlier.
    pub fn is_due_by(&self, today: NaiveDate) -> bool {
        self.is_open() && self.due.is_some_and(|d| d <= today)
    }

    pub fn is_overdue(&self, today: NaiveDate) -> bool {
        self.is_open() && self.due.is_some_and(|d| d < today)
    }

    pub fn start(&mut self, now: DateTime<Utc>) {
        if self.status == TaskStatus::Planned {
            self.status = TaskStatus::InProgress;
            self.touch(now, "Begonnen");
        }
    }

    pub fn pause(&mut self, now: DateTime<Utc>) {
        if self.status == TaskStatus::InProgress {
            self.status = TaskStatus::Planned;
            self.touch(now, "Pausiert");
        }
    }

    /// Mark as done on the local day `today`. Returns the XP earned.
    pub fn complete(&mut self, now: DateTime<Utc>, today: NaiveDate) -> u32 {
        if self.status == TaskStatus::Done {
            return 0;
        }
        let xp = xp_for_task(self, today);
        self.status = TaskStatus::Done;
        self.done_at = Some(now);
        self.done_on = Some(today);
        self.xp_awarded = xp;
        self.touch(now, "Erledigt");
        xp
    }

    /// Undo completion. Returns the XP that has to be taken back.
    pub fn reopen(&mut self, now: DateTime<Utc>) -> u32 {
        if self.status != TaskStatus::Done {
            return 0;
        }
        let xp = self.xp_awarded;
        self.status = TaskStatus::Planned;
        self.done_at = None;
        self.done_on = None;
        self.xp_awarded = 0;
        self.touch(now, "Wieder geöffnet");
        xp
    }

    /// Move to the trash (soft delete).
    pub fn trash(&mut self, now: DateTime<Utc>) {
        self.meta.deleted_at = Some(now);
        self.touch(now, "In den Papierkorb");
    }

    pub fn restore(&mut self, now: DateTime<Utc>) {
        self.meta.deleted_at = None;
        self.touch(now, "Wiederhergestellt");
    }
}

/// Complete a task in a list and credit the profile. Returns the XP earned.
pub fn complete_task(
    tasks: &mut [Task],
    profile: &mut Profile,
    id: Uuid,
    now: DateTime<Utc>,
    today: NaiveDate,
) -> u32 {
    let Some(t) = tasks.iter_mut().find(|t| t.meta.id == id) else {
        return 0;
    };
    let xp = t.complete(now, today);
    if xp > 0 {
        profile.add_xp(xp, today);
    }
    xp
}

/// Reopen a task in a list and take its XP back.
pub fn reopen_task(tasks: &mut [Task], profile: &mut Profile, id: Uuid, now: DateTime<Utc>) {
    if let Some(t) = tasks.iter_mut().find(|t| t.meta.id == id) {
        let xp = t.reopen(now);
        profile.remove_xp(xp);
    }
}

fn matches_area(t: &Task, area: Option<&str>) -> bool {
    area.is_none_or(|a| t.area.as_deref() == Some(a))
}

/// Open tasks: in progress first, then priority, then due date (none last), then oldest.
pub fn open_tasks<'a>(tasks: &'a [Task], area: Option<&str>) -> Vec<&'a Task> {
    let mut v: Vec<_> = tasks
        .iter()
        .filter(|t| t.is_open() && matches_area(t, area))
        .collect();
    v.sort_by(|a, b| {
        (b.status == TaskStatus::InProgress)
            .cmp(&(a.status == TaskStatus::InProgress))
            .then(a.priority.cmp(&b.priority))
            .then(match (a.due, b.due) {
                (Some(x), Some(y)) => x.cmp(&y),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            })
            .then(a.meta.created_at.cmp(&b.meta.created_at))
    });
    v
}

/// What belongs on the "Today" view: in progress, or due today/overdue.
pub fn today_tasks<'a>(tasks: &'a [Task], area: Option<&str>, today: NaiveDate) -> Vec<&'a Task> {
    open_tasks(tasks, area)
        .into_iter()
        .filter(|t| t.status == TaskStatus::InProgress || t.is_due_by(today))
        .collect()
}

/// Done tasks, most recent first.
pub fn done_tasks<'a>(tasks: &'a [Task], area: Option<&str>) -> Vec<&'a Task> {
    let mut v: Vec<_> = tasks
        .iter()
        .filter(|t| !t.is_deleted() && t.status == TaskStatus::Done && matches_area(t, area))
        .collect();
    v.sort_by_key(|t| std::cmp::Reverse(t.done_at));
    v
}

/// Tasks in the trash, most recently deleted first.
pub fn trashed_tasks(tasks: &[Task]) -> Vec<&Task> {
    let mut v: Vec<_> = tasks.iter().filter(|t| t.is_deleted()).collect();
    v.sort_by_key(|t| std::cmp::Reverse(t.meta.deleted_at));
    v
}

/// Completed tasks per weekday (Mon..Sun) of the week containing `today`.
pub fn done_this_week(tasks: &[Task], today: NaiveDate) -> [u32; 7] {
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let mut out = [0; 7];
    for d in tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .filter_map(|t| t.done_on)
    {
        let offset = (d - monday).num_days();
        if (0..7).contains(&offset) {
            out[offset as usize] += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Priority;

    fn day(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, n).unwrap()
    }

    fn task(title: &str, p: Priority, due: Option<NaiveDate>) -> Task {
        let mut t = Task::new(title, Utc::now());
        t.priority = p;
        t.due = due;
        t
    }

    #[test]
    fn complete_and_reopen_moves_xp() {
        let now = Utc::now();
        let mut tasks = vec![task("a", Priority::P2, Some(day(20)))];
        let id = tasks[0].meta.id;
        let mut p = Profile::default();
        assert_eq!(complete_task(&mut tasks, &mut p, id, now, day(20)), 45);
        assert_eq!(p.xp, 45);
        // completing twice gives nothing
        assert_eq!(complete_task(&mut tasks, &mut p, id, now, day(20)), 0);
        reopen_task(&mut tasks, &mut p, id, now);
        assert_eq!(p.xp, 0);
        assert!(tasks[0].is_open());
        assert_eq!(tasks[0].log.len(), 2);
    }

    #[test]
    fn ordering_and_filters() {
        let now = Utc::now();
        let mut a = task("low", Priority::P4, None);
        let b = task("high", Priority::P1, Some(day(25)));
        let mut c = task("work", Priority::P3, None);
        c.area = Some("Arbeit".into());
        c.start(now);
        a.area = Some("Privat".into());
        let tasks = vec![a, b, c];
        let titles: Vec<_> = open_tasks(&tasks, None)
            .iter()
            .map(|t| t.title.as_str())
            .collect();
        assert_eq!(titles, ["work", "high", "low"]);
        assert_eq!(open_tasks(&tasks, Some("Privat")).len(), 1);
        // today: in-progress + due by today
        assert_eq!(today_tasks(&tasks, None, day(20)).len(), 1);
        assert_eq!(today_tasks(&tasks, None, day(25)).len(), 2);
    }

    #[test]
    fn trash_and_restore() {
        let now = Utc::now();
        let mut tasks = vec![task("x", Priority::P4, None)];
        tasks[0].trash(now);
        assert!(open_tasks(&tasks, None).is_empty());
        assert_eq!(trashed_tasks(&tasks).len(), 1);
        tasks[0].restore(now);
        assert_eq!(open_tasks(&tasks, None).len(), 1);
    }

    #[test]
    fn week_stats() {
        let now = Utc::now();
        // 2026-11-20 is a Friday; Monday is the 16th.
        let mut tasks = vec![
            task("a", Priority::P4, None),
            task("b", Priority::P4, None),
            task("c", Priority::P4, None),
        ];
        tasks[0].complete(now, day(16));
        tasks[1].complete(now, day(20));
        tasks[2].complete(now, day(13)); // last week
        assert_eq!(done_this_week(&tasks, day(20)), [1, 0, 0, 0, 1, 0, 0]);
    }

    #[test]
    fn overdue() {
        let t = task("x", Priority::P4, Some(day(19)));
        assert!(t.is_overdue(day(20)));
        assert!(!t.is_overdue(day(19)));
    }
}

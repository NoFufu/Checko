//! Task life cycle and list helpers (pure, no browser code).

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use uuid::Uuid;

use crate::{xp_for_task, LogEntry, Profile, Step, Task, TaskStatus};

/// Default life areas a task can belong to.
pub const AREAS: [&str; 3] = ["Arbeit", "Privat", "Gesundheit"];

/// All categories: the defaults first, then any custom ones used by tasks.
pub fn all_areas(tasks: &[Task]) -> Vec<String> {
    let mut out: Vec<String> = AREAS.iter().map(|a| a.to_string()).collect();
    let mut custom: Vec<String> = tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .flat_map(|t| {
            t.categories()
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .filter(|a| !out.iter().any(|o| o.eq_ignore_ascii_case(a)))
        .collect();
    custom.sort_by_key(|a| a.to_lowercase());
    custom.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    out.extend(custom);
    out
}

/// Open and done counts per category (for the overview with progress bars).
pub fn category_stats(tasks: &[Task]) -> Vec<(String, usize, usize)> {
    all_areas(tasks)
        .into_iter()
        .map(|c| {
            let mine = tasks
                .iter()
                .filter(|t| !t.is_deleted() && t.has_category(&c));
            let (mut open, mut done) = (0, 0);
            for t in mine {
                if t.status == TaskStatus::Done {
                    done += 1;
                } else {
                    open += 1;
                }
            }
            (c, open, done)
        })
        .filter(|(_, o, d)| o + d > 0)
        .collect()
}

impl Task {
    /// All categories of the task: the main one plus any further #tags.
    pub fn categories(&self) -> Vec<&str> {
        self.area
            .iter()
            .chain(self.tags.iter())
            .map(String::as_str)
            .collect()
    }

    pub fn has_category(&self, c: &str) -> bool {
        self.categories().iter().any(|x| x.eq_ignore_ascii_case(c))
    }

    /// Add a category (the first one becomes the main category).
    pub fn add_category(&mut self, c: &str, now: DateTime<Utc>) {
        let c = c.trim().trim_start_matches('#').trim();
        if c.is_empty() || self.has_category(c) {
            return;
        }
        if self.area.is_none() {
            self.area = Some(c.to_string());
        } else {
            self.tags.push(c.to_string());
        }
        self.meta.updated_at = now;
    }

    pub fn remove_category(&mut self, c: &str, now: DateTime<Utc>) {
        if self
            .area
            .as_deref()
            .is_some_and(|a| a.eq_ignore_ascii_case(c))
        {
            // promote the next tag to main category
            self.area = if self.tags.is_empty() {
                None
            } else {
                Some(self.tags.remove(0))
            };
        } else {
            self.tags.retain(|t| !t.eq_ignore_ascii_case(c));
        }
        self.meta.updated_at = now;
    }

    pub fn add_step(&mut self, text: &str, now: DateTime<Utc>) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }
        self.steps.push(Step {
            id: Uuid::now_v7(),
            text: text.to_string(),
            done: false,
        });
        self.meta.updated_at = now;
    }

    pub fn toggle_step(&mut self, id: Uuid, now: DateTime<Utc>) {
        if let Some(s) = self.steps.iter_mut().find(|s| s.id == id) {
            s.done = !s.done;
            self.meta.updated_at = now;
        }
    }

    pub fn remove_step(&mut self, id: Uuid, now: DateTime<Utc>) {
        self.steps.retain(|s| s.id != id);
        self.meta.updated_at = now;
    }

    /// (done, total) steps.
    pub fn step_progress(&self) -> (usize, usize) {
        (
            self.steps.iter().filter(|s| s.done).count(),
            self.steps.len(),
        )
    }

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

    /// Appointment look (time block in the calendar, "Termine" section) instead of a to-do.
    /// Needs a date; by default everything with a time of day is an appointment.
    pub fn is_appointment(&self) -> bool {
        self.due.is_some() && self.appointment.unwrap_or(self.due_time.is_some())
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
    tasks: &mut Vec<Task>,
    profile: &mut Profile,
    id: Uuid,
    now: DateTime<Utc>,
    today: NaiveDate,
) -> u32 {
    let Some(idx) = tasks.iter().position(|t| t.meta.id == id) else {
        return 0;
    };
    let t = &mut tasks[idx];
    let xp = t.complete(now, today);
    if xp > 0 {
        profile.add_xp(xp, today);
    }
    // Repeating task: create the next one (fresh steps, same settings).
    if let (Some(r), Some(due)) = (t.recurrence, t.due) {
        let after = due.max(today);
        if let Some(next_due) = r.next_after(due, after) {
            let mut next = Task::new(t.title.clone(), now);
            next.notes = t.notes.clone();
            next.priority = t.priority;
            next.area = t.area.clone();
            next.tags = t.tags.clone();
            next.due = Some(next_due);
            next.due_time = t.due_time;
            next.appointment = t.appointment;
            next.remind = t.remind;
            next.recurrence = Some(r);
            next.steps = t
                .steps
                .iter()
                .map(|s| Step {
                    id: Uuid::now_v7(),
                    text: s.text.clone(),
                    done: false,
                })
                .collect();
            // stop the finished one from repeating again on reopen + complete
            t.recurrence = None;
            tasks.push(next);
        }
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
    area.is_none_or(|a| t.has_category(a))
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

/// Open appointments (see [`Task::is_appointment`]) by date and time.
pub fn open_appointments<'a>(tasks: &'a [Task], area: Option<&str>) -> Vec<&'a Task> {
    let mut v: Vec<_> = tasks
        .iter()
        .filter(|t| t.is_open() && t.is_appointment() && matches_area(t, area))
        .collect();
    v.sort_by_key(|t| (t.due, t.due_time.is_some(), t.due_time));
    v
}

/// Open to-dos (everything open that is not an appointment).
pub fn open_todos<'a>(tasks: &'a [Task], area: Option<&str>) -> Vec<&'a Task> {
    open_tasks(tasks, area)
        .into_iter()
        .filter(|t| !t.is_appointment())
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

/// Completed tasks per day for the last `days` days, oldest first (last = today).
pub fn done_last_days(tasks: &[Task], today: NaiveDate, days: usize) -> Vec<u32> {
    let mut out = vec![0; days];
    for d in tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .filter_map(|t| t.done_on)
    {
        let back = (today - d).num_days();
        if (0..days as i64).contains(&back) {
            out[days - 1 - back as usize] += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Meta, Priority};
    use chrono::NaiveTime;

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
    fn areas_include_custom() {
        let mut a = task("x", Priority::P4, None);
        a.area = Some("Küche".into());
        let mut b = task("y", Priority::P4, None);
        b.area = Some("Küche".into());
        let mut c = task("z", Priority::P4, None);
        c.area = Some("Auto".into());
        assert_eq!(
            all_areas(&[a, b, c]),
            ["Arbeit", "Privat", "Gesundheit", "Auto", "Küche"]
        );
    }

    #[test]
    fn multiple_categories() {
        let now = Utc::now();
        let mut t = task("Lampe austauschen", Priority::P4, None);
        t.add_category("#Bad", now);
        t.add_category("Küche", now);
        t.add_category("bad", now); // duplicate
        assert_eq!(t.categories(), ["Bad", "Küche"]);
        let tasks = vec![t.clone()];
        assert_eq!(open_tasks(&tasks, Some("küche")).len(), 1);
        assert_eq!(
            category_stats(&tasks),
            [("Bad".to_string(), 1, 0), ("Küche".to_string(), 1, 0)]
        );
        t.remove_category("Bad", now);
        assert_eq!(t.categories(), ["Küche"]);
        assert_eq!(t.area.as_deref(), Some("Küche"));
    }

    #[test]
    fn steps() {
        let now = Utc::now();
        let mut t = task("Umzug", Priority::P4, None);
        t.add_step("Kartons kaufen", now);
        t.add_step("  ", now);
        t.add_step("Helfer fragen", now);
        let id = t.steps[0].id;
        t.toggle_step(id, now);
        assert_eq!(t.step_progress(), (1, 2));
        t.remove_step(id, now);
        assert_eq!(t.step_progress(), (0, 1));
    }

    #[test]
    fn repeating_task_creates_next() {
        let now = Utc::now();
        let mut t = task("Müll rausbringen", Priority::P4, Some(day(20)));
        t.recurrence = Some(crate::Recurrence::every(crate::Freq::Weekly));
        let id = t.meta.id;
        let mut tasks = vec![t];
        let mut p = Profile::default();
        complete_task(&mut tasks, &mut p, id, now, day(20));
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[1].due, Some(day(27)));
        assert!(tasks[1].is_open());
        assert!(tasks[0].recurrence.is_none());
    }

    #[test]
    fn overdue() {
        let t = task("x", Priority::P4, Some(day(19)));
        assert!(t.is_overdue(day(20)));
        assert!(!t.is_overdue(day(19)));
    }

    #[test]
    fn appointments_and_todos() {
        let now = Utc::now();
        let day = NaiveDate::from_ymd_opt(2026, 11, 24).unwrap();
        let mut meeting = Task::new("Zweigstelle Paderborn", now);
        meeting.due = Some(day);
        meeting.due_time = NaiveTime::from_hms_opt(9, 0, 0);
        let mut early = meeting.clone();
        early.meta = Meta::new(now);
        early.due_time = NaiveTime::from_hms_opt(7, 30, 0);
        let mut wash = Task::new("Wäsche machen", now);
        wash.due = Some(day);
        let mut trash = Task::new("Müll runter bringen", now);
        trash.due = Some(day);
        trash.due_time = NaiveTime::from_hms_opt(18, 0, 0);
        trash.appointment = Some(false); // has a time, but the user wants a to-do
        let mut party = Task::new("Glühweintour", now);
        party.due = Some(day);
        party.appointment = Some(true); // all-day appointment
        let undated = Task::new("Isi antworten", now);

        assert!(meeting.is_appointment());
        assert!(!wash.is_appointment());
        assert!(!trash.is_appointment());
        assert!(party.is_appointment());
        let mut no_date = undated.clone();
        no_date.appointment = Some(true);
        assert!(!no_date.is_appointment(), "an appointment needs a date");

        let all = vec![meeting, early, wash, trash, party, undated];
        let appts: Vec<_> = open_appointments(&all, None)
            .iter()
            .map(|t| t.due_time)
            .collect();
        assert_eq!(
            appts,
            vec![
                None,
                NaiveTime::from_hms_opt(7, 30, 0),
                NaiveTime::from_hms_opt(9, 0, 0)
            ]
        );
        let todos: Vec<_> = open_todos(&all, None)
            .iter()
            .map(|t| t.title.as_str())
            .collect();
        assert_eq!(todos.len(), 3);
        assert!(todos.contains(&"Müll runter bringen"));
    }

    #[test]
    fn done_per_day() {
        let now = Utc::now();
        let today = NaiveDate::from_ymd_opt(2026, 11, 23).unwrap();
        let done = |back: i64| {
            let mut t = Task::new("x", now);
            t.status = TaskStatus::Done;
            t.done_on = Some(today - Duration::days(back));
            t
        };
        let tasks = vec![done(0), done(0), done(3), done(20)];
        let v = done_last_days(&tasks, today, 14);
        assert_eq!(v.len(), 14);
        assert_eq!(v[13], 2);
        assert_eq!(v[10], 1);
        assert_eq!(v.iter().sum::<u32>(), 3);
    }
}

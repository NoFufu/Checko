//! XP and levels. Pure functions so they are easy to test.

use chrono::NaiveDate;

use crate::{Priority, Profile, Task};

/// XP earned for completing a task on `done_on`: priority weight × 10, +50 % if on time.
pub fn xp_for_task(task: &Task, done_on: NaiveDate) -> u32 {
    let base = task.priority.weight() * 10;
    let on_time = task.due.is_some_and(|due| done_on <= due);
    if on_time {
        base + base / 2
    } else {
        base
    }
}

/// XP needed to go from `level` to `level + 1` (level starts at 1).
pub fn xp_to_next(level: u32) -> u32 {
    100 + (level - 1) * 50
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelProgress {
    pub level: u32,
    /// XP collected inside the current level.
    pub current: u32,
    /// XP needed for the next level.
    pub needed: u32,
}

impl LevelProgress {
    pub fn percent(&self) -> f32 {
        self.current as f32 / self.needed as f32 * 100.0
    }
}

pub fn level_for_xp(mut xp: u32) -> LevelProgress {
    let mut level = 1;
    while xp >= xp_to_next(level) {
        xp -= xp_to_next(level);
        level += 1;
    }
    LevelProgress {
        level,
        current: xp,
        needed: xp_to_next(level),
    }
}

impl Profile {
    /// Add XP for something done on `today` and keep the daily streak up to date.
    pub fn add_xp(&mut self, xp: u32, today: NaiveDate) {
        self.xp += xp;
        match self.last_active {
            Some(d) if d == today => {}
            Some(d) if d.succ_opt() == Some(today) => self.streak_days += 1,
            _ => self.streak_days = 1,
        }
        self.last_active = Some(today);
    }

    /// Take XP back (e.g. a task was reopened). The streak stays.
    pub fn remove_xp(&mut self, xp: u32) {
        self.xp = self.xp.saturating_sub(xp);
    }

    /// Streak as it stands today: it breaks if yesterday had no activity.
    pub fn current_streak(&self, today: NaiveDate) -> u32 {
        match self.last_active {
            Some(d) if d == today || d.succ_opt() == Some(today) => self.streak_days,
            _ => 0,
        }
    }
}

/// Small helper used by the UI for previews.
pub fn xp_for_priority(p: Priority) -> u32 {
    p.weight() * 10
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn levels() {
        assert_eq!(level_for_xp(0).level, 1);
        assert_eq!(level_for_xp(99).level, 1);
        let l = level_for_xp(100);
        assert_eq!((l.level, l.current, l.needed), (2, 0, 150));
        assert_eq!(level_for_xp(250).level, 3);
    }

    #[test]
    fn on_time_bonus() {
        let now = Utc::now();
        let today = now.date_naive();
        let mut t = Task::new("x", now);
        t.priority = Priority::P1;
        t.due = Some(today + Duration::days(1));
        assert_eq!(xp_for_task(&t, today), 60);
        t.due = Some(today - Duration::days(1));
        assert_eq!(xp_for_task(&t, today), 40);
        t.due = None;
        assert_eq!(xp_for_task(&t, today), 40);
    }

    #[test]
    fn streaks() {
        let d = |n| NaiveDate::from_ymd_opt(2026, 11, n).unwrap();
        let mut p = Profile::default();
        p.add_xp(10, d(1));
        p.add_xp(10, d(1));
        assert_eq!((p.xp, p.streak_days), (20, 1));
        p.add_xp(10, d(2));
        assert_eq!(p.streak_days, 2);
        assert_eq!(p.current_streak(d(3)), 2);
        assert_eq!(p.current_streak(d(4)), 0);
        p.add_xp(10, d(5));
        assert_eq!(p.streak_days, 1);
        p.remove_xp(100);
        assert_eq!(p.xp, 0);
    }
}

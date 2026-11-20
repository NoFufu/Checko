//! XP and levels. Pure functions so they are easy to test.

use crate::{Priority, Task};

/// XP earned for completing a task: priority weight × 10, +50 % if on time.
pub fn xp_for_task(task: &Task) -> u32 {
    let base = task.priority.weight() * 10;
    let on_time = match (task.due, task.done_at) {
        (Some(due), Some(done)) => done.date_naive() <= due,
        _ => false,
    };
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
        let mut t = Task::new("x", now);
        t.priority = Priority::P1;
        t.due = Some((now + Duration::days(1)).date_naive());
        t.complete(now);
        assert_eq!(xp_for_task(&t), 60);
        t.due = Some((now - Duration::days(1)).date_naive());
        assert_eq!(xp_for_task(&t), 40);
    }
}

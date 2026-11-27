use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Every entity carries these fields so it can be synced later.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Soft delete: items go to the trash first.
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Meta {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::now_v7(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskStatus {
    #[default]
    Planned,
    InProgress,
    Done,
}

/// P1 (most urgent) to P4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum Priority {
    P1,
    P2,
    P3,
    #[default]
    P4,
}

impl Priority {
    pub fn weight(self) -> u32 {
        match self {
            Priority::P1 => 4,
            Priority::P2 => 3,
            Priority::P3 => 2,
            Priority::P4 => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub at: DateTime<Utc>,
    pub text: String,
}

/// A sub-step of a task ("Schritte" in Microsoft To Do).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub id: Uuid,
    pub text: String,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub meta: Meta,
    pub title: String,
    pub notes: String,
    pub status: TaskStatus,
    pub priority: Priority,
    pub area: Option<String>,
    pub tags: Vec<String>,
    pub due: Option<NaiveDate>,
    /// Optional time of day for the due date (local time).
    #[serde(default)]
    pub due_time: Option<NaiveTime>,
    pub remind_at: Option<DateTime<Utc>>,
    pub log: Vec<LogEntry>,
    pub done_at: Option<DateTime<Utc>>,
    /// Local calendar day the task was completed (for streaks and week stats).
    #[serde(default)]
    pub done_on: Option<NaiveDate>,
    /// XP granted on completion, so it can be taken back when reopened.
    #[serde(default)]
    pub xp_awarded: u32,
    #[serde(default)]
    pub steps: Vec<Step>,
    /// Repeat rule: when done, the next occurrence is created automatically.
    #[serde(default)]
    pub recurrence: Option<crate::Recurrence>,
    /// Shown as appointment (time block) or to-do. `None` = automatic:
    /// an entry with a time of day is an appointment.
    #[serde(default)]
    pub appointment: Option<bool>,
    /// Reminder: minutes before the due time (all-day: before 09:00). `None` = off.
    #[serde(default)]
    pub remind: Option<u32>,
}

impl Task {
    pub fn new(title: impl Into<String>, now: DateTime<Utc>) -> Self {
        Self {
            meta: Meta::new(now),
            title: title.into(),
            notes: String::new(),
            status: TaskStatus::Planned,
            priority: Priority::default(),
            area: None,
            tags: Vec::new(),
            due: None,
            due_time: None,
            remind_at: None,
            log: Vec::new(),
            done_at: None,
            done_on: None,
            xp_awarded: 0,
            steps: Vec::new(),
            recurrence: None,
            appointment: None,
            remind: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub meta: Meta,
    pub title: String,
    pub body_md: String,
    pub pinned: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShoppingItem {
    pub meta: Meta,
    pub name: String,
    pub qty: Option<String>,
    pub category: Option<String>,
    pub checked: bool,
    pub times_bought: u32,
    /// Optional product link (e.g. Amazon).
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Profile {
    pub xp: u32,
    pub streak_days: u32,
    pub last_active: Option<NaiveDate>,
    pub achievements: Vec<String>,
    /// Last change (for sync between devices).
    pub updated_at: Option<DateTime<Utc>>,
    /// Own emoji/colour per category, and hidden default categories.
    pub category_styles: Vec<CategoryStyle>,
    /// Überkategorien (groups of categories), in display order.
    pub category_groups: Vec<String>,
    /// Focus mode: XP, level, streak, achievements and confetti are hidden.
    pub focus_mode: bool,
}

/// Look of one category, chosen by the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryStyle {
    pub name: String,
    pub emoji: Option<String>,
    /// CSS colour, e.g. "#65a982".
    pub color: Option<String>,
    /// Hidden from the category list (used for deleted default categories).
    pub hidden: bool,
    /// Überkategorie (group) this category belongs to.
    pub parent: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completing_a_task_sets_status_and_date() {
        let now = Utc::now();
        let today = now.date_naive();
        let mut t = Task::new("Milch kaufen", now);
        assert_eq!(t.status, TaskStatus::Planned);
        t.complete(now, today);
        assert_eq!(t.status, TaskStatus::Done);
        assert_eq!(t.done_at, Some(now));
        assert_eq!(t.done_on, Some(today));
    }

    #[test]
    fn ids_are_unique() {
        let now = Utc::now();
        assert_ne!(Meta::new(now).id, Meta::new(now).id);
    }
}

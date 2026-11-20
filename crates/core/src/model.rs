use chrono::{DateTime, NaiveDate, Utc};
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
    pub remind_at: Option<DateTime<Utc>>,
    pub log: Vec<LogEntry>,
    pub done_at: Option<DateTime<Utc>>,
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
            remind_at: None,
            log: Vec::new(),
            done_at: None,
        }
    }

    pub fn complete(&mut self, now: DateTime<Utc>) {
        self.status = TaskStatus::Done;
        self.done_at = Some(now);
        self.meta.updated_at = now;
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Profile {
    pub xp: u32,
    pub streak_days: u32,
    pub last_active: Option<NaiveDate>,
    pub achievements: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completing_a_task_sets_status_and_date() {
        let now = Utc::now();
        let mut t = Task::new("Milch kaufen", now);
        assert_eq!(t.status, TaskStatus::Planned);
        t.complete(now);
        assert_eq!(t.status, TaskStatus::Done);
        assert_eq!(t.done_at, Some(now));
    }

    #[test]
    fn ids_are_unique() {
        let now = Utc::now();
        assert_ne!(Meta::new(now).id, Meta::new(now).id);
    }
}

//! Checko core: domain models and pure logic (no browser dependencies).

pub mod achievements;
pub mod calendar;
pub mod categories;
pub mod gamification;
pub mod heatmap;
pub mod ics;
pub mod ics_export;
pub mod imports;
pub mod model;
pub mod notes;
pub mod parse;
pub mod planned;
pub mod shopping;
pub mod tasks;

pub use achievements::{achievement, unlock_new, Achievement, Snapshot, ACHIEVEMENTS};
pub use calendar::{
    items_between, month_grid, reminders_between, shift_month, CalendarItem, Event, Freq, ItemKind,
    Recurrence, Reminder, ALL_DAY_REMINDER,
};
pub use categories::*;
pub use gamification::{level_for_xp, xp_for_task, LevelProgress};
pub use heatmap::{heatmap, HeatDay};
pub use ics::{merge_ics, parse_ics, IcsEvent, ImportStats};
pub use ics_export::write_ics;
pub use imports::{delete_import, import_batches, ImportBatch};
pub use model::*;
pub use notes::*;
pub use parse::{parse_quick_add, ParsedTask, Token, TokenKind};
pub use planned::{planned_groups, PlanGroup};
pub use shopping::*;
pub use tasks::*;

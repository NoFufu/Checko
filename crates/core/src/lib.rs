//! Checko core: domain models and pure logic (no browser dependencies).

pub mod achievements;
pub mod gamification;
pub mod model;
pub mod notes;
pub mod parse;
pub mod shopping;
pub mod tasks;

pub use achievements::{achievement, unlock_new, Achievement, Snapshot, ACHIEVEMENTS};
pub use gamification::{level_for_xp, xp_for_task, LevelProgress};
pub use model::*;
pub use notes::*;
pub use parse::{parse_quick_add, ParsedTask, Token, TokenKind};
pub use shopping::*;
pub use tasks::*;

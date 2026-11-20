//! Checko core: domain models and pure logic (no browser dependencies).

pub mod gamification;
pub mod model;
pub mod tasks;

pub use gamification::{level_for_xp, xp_for_task, LevelProgress};
pub use model::*;
pub use tasks::*;

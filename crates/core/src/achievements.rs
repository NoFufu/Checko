//! Achievements ("Erfolge"): small goals that unlock as you use Checko.

use chrono::NaiveDate;

use crate::{level_for_xp, Note, Profile, ShoppingItem, Task, TaskStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Achievement {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub emoji: &'static str,
}

pub const ACHIEVEMENTS: [Achievement; 12] = [
    Achievement {
        id: "first-task",
        title: "Erster Haken",
        description: "Die erste Aufgabe erledigt.",
        emoji: "✅",
    },
    Achievement {
        id: "ten-tasks",
        title: "In Fahrt",
        description: "10 Aufgaben erledigt.",
        emoji: "🚀",
    },
    Achievement {
        id: "fifty-tasks",
        title: "Profi",
        description: "50 Aufgaben erledigt.",
        emoji: "🏆",
    },
    Achievement {
        id: "urgent",
        title: "Feuerwehr",
        description: "Eine dringende Aufgabe erledigt.",
        emoji: "🧯",
    },
    Achievement {
        id: "on-time-5",
        title: "Pünktlich",
        description: "5 Aufgaben vor oder am Fälligkeitstag erledigt.",
        emoji: "⏰",
    },
    Achievement {
        id: "early-bird",
        title: "Frühaufsteher",
        description: "Eine Aufgabe vor 8 Uhr erledigt.",
        emoji: "🌅",
    },
    Achievement {
        id: "streak-3",
        title: "Dranbleiben",
        description: "3 Tage in Folge etwas erledigt.",
        emoji: "🔥",
    },
    Achievement {
        id: "streak-7",
        title: "Eine Woche stark",
        description: "7 Tage in Folge etwas erledigt.",
        emoji: "💪",
    },
    Achievement {
        id: "level-5",
        title: "Level 5",
        description: "Level 5 erreicht.",
        emoji: "⭐",
    },
    Achievement {
        id: "organizer",
        title: "Ordnung muss sein",
        description: "Aufgaben in 3 verschiedenen Kategorien.",
        emoji: "🗂️",
    },
    Achievement {
        id: "shopper",
        title: "Einkaufsheld",
        description: "10 Einkäufe abgehakt.",
        emoji: "🛒",
    },
    Achievement {
        id: "writer",
        title: "Notizbuch",
        description: "5 Notizen geschrieben.",
        emoji: "📝",
    },
];

pub fn achievement(id: &str) -> Option<&'static Achievement> {
    ACHIEVEMENTS.iter().find(|a| a.id == id)
}

/// Everything the achievement rules look at.
pub struct Snapshot<'a> {
    pub tasks: &'a [Task],
    pub notes: &'a [Note],
    pub shopping: &'a [ShoppingItem],
    pub profile: &'a Profile,
    pub today: NaiveDate,
    /// Local hour (0-23) of the moment the check runs.
    pub hour: u32,
}

fn reached(id: &str, s: &Snapshot) -> bool {
    let done: Vec<&Task> = s
        .tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Done)
        .collect();
    match id {
        "first-task" => !done.is_empty(),
        "ten-tasks" => done.len() >= 10,
        "fifty-tasks" => done.len() >= 50,
        "urgent" => done.iter().any(|t| t.priority == crate::Priority::P1),
        "on-time-5" => {
            done.iter()
                .filter(|t| matches!((t.due, t.done_on), (Some(d), Some(on)) if on <= d))
                .count()
                >= 5
        }
        "early-bird" => s.hour < 8 && done.iter().any(|t| t.done_on == Some(s.today)),
        "streak-3" => s.profile.current_streak(s.today) >= 3,
        "streak-7" => s.profile.current_streak(s.today) >= 7,
        "level-5" => level_for_xp(s.profile.xp).level >= 5,
        "organizer" => {
            let mut cats: Vec<String> = s
                .tasks
                .iter()
                .flat_map(|t| {
                    t.categories()
                        .into_iter()
                        .map(str::to_lowercase)
                        .collect::<Vec<_>>()
                })
                .collect();
            cats.sort();
            cats.dedup();
            cats.len() >= 3
        }
        "shopper" => s.shopping.iter().map(|i| i.times_bought).sum::<u32>() >= 10,
        "writer" => s.notes.iter().filter(|n| !n.is_empty()).count() >= 5,
        _ => false,
    }
}

/// Newly reached achievements; they are also added to the profile.
pub fn unlock_new(profile: &mut Profile, s: &Snapshot) -> Vec<&'static Achievement> {
    let new: Vec<&'static Achievement> = ACHIEVEMENTS
        .iter()
        .filter(|a| !profile.achievements.iter().any(|x| x == a.id) && reached(a.id, s))
        .collect();
    profile
        .achievements
        .extend(new.iter().map(|a| a.id.to_string()));
    new
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Priority;
    use chrono::Utc;

    fn day() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, 20).unwrap()
    }

    #[test]
    fn unlocks_once() {
        let now = Utc::now();
        let mut t = Task::new("x", now);
        t.priority = Priority::P1;
        t.complete(now, day());
        let tasks = vec![t];
        let mut profile = Profile::default();
        let p0 = profile.clone();
        let s = Snapshot {
            tasks: &tasks,
            notes: &[],
            shopping: &[],
            profile: &p0,
            today: day(),
            hour: 7,
        };
        let ids: Vec<_> = unlock_new(&mut profile, &s).iter().map(|a| a.id).collect();
        assert_eq!(ids, ["first-task", "urgent", "early-bird"]);
        let p1 = profile.clone();
        let s = Snapshot {
            tasks: &tasks,
            notes: &[],
            shopping: &[],
            profile: &p1,
            today: day(),
            hour: 7,
        };
        assert!(unlock_new(&mut profile, &s).is_empty());
    }

    #[test]
    fn categories_and_level() {
        let now = Utc::now();
        let mut tasks = Vec::new();
        for c in ["Bad", "Küche", "Auto"] {
            let mut t = Task::new("x", now);
            t.add_category(c, now);
            tasks.push(t);
        }
        let mut p = Profile {
            xp: 1000,
            ..Default::default()
        };
        let snap_p = p.clone();
        let s = Snapshot {
            tasks: &tasks,
            notes: &[],
            shopping: &[],
            profile: &snap_p,
            today: day(),
            hour: 12,
        };
        let ids: Vec<_> = unlock_new(&mut p, &s).iter().map(|a| a.id).collect();
        assert_eq!(ids, ["level-5", "organizer"]);
    }
}

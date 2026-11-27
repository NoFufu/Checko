//! Data for the GitHub-style activity grid (done tasks per day).

use chrono::{Datelike, Duration, NaiveDate};

use crate::Task;

#[derive(Debug, Clone, PartialEq)]
pub struct HeatDay {
    pub date: NaiveDate,
    pub count: u32,
    /// Titles of the tasks done that day (for the tooltip).
    pub titles: Vec<String>,
    /// 0 = nothing … 4 = busiest days.
    pub level: u8,
}

/// Days from the Monday `weeks - 1` weeks back up to `today`, column by column
/// (week by week, Monday first).
pub fn heatmap(tasks: &[Task], today: NaiveDate, weeks: u32) -> Vec<HeatDay> {
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let start = monday - Duration::weeks(weeks.saturating_sub(1) as i64);
    let n = (today - start).num_days() as usize + 1;
    let mut days: Vec<HeatDay> = (0..n)
        .map(|i| HeatDay {
            date: start + Duration::days(i as i64),
            count: 0,
            titles: Vec::new(),
            level: 0,
        })
        .collect();
    for t in tasks.iter().filter(|t| !t.is_deleted()) {
        let Some(d) = t.done_on else { continue };
        if d < start || d > today {
            continue;
        }
        let day = &mut days[(d - start).num_days() as usize];
        day.count += 1;
        day.titles.push(t.title.clone());
    }
    let max = days.iter().map(|d| d.count).max().unwrap_or(0);
    for d in &mut days {
        d.level = level(d.count, max);
    }
    days
}

/// Intensity 0–4 relative to the busiest day.
pub fn level(count: u32, max: u32) -> u8 {
    if count == 0 || max == 0 {
        return 0;
    }
    (((count * 4) as f64 / max as f64).ceil() as u8).clamp(1, 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn grid_and_counts() {
        // Friday 27.11.2026
        let today = NaiveDate::from_ymd_opt(2026, 11, 27).unwrap();
        let now = Utc::now();
        let mut tasks = Vec::new();
        for (title, back) in [("a", 0), ("b", 0), ("c", 2), ("zu alt", 400)] {
            let mut t = Task::new(title, now);
            t.complete(now, today - Duration::days(back));
            tasks.push(t);
        }
        let mut open = Task::new("offen", now);
        open.due = Some(today);
        tasks.push(open);
        let h = heatmap(&tasks, today, 26);
        // starts on a Monday, ends today
        assert_eq!(h[0].date.weekday().num_days_from_monday(), 0);
        assert_eq!(h.last().unwrap().date, today);
        assert_eq!(h.len(), 25 * 7 + 5);
        let last = h.last().unwrap();
        assert_eq!((last.count, last.level), (2, 4));
        assert_eq!(last.titles, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(h[h.len() - 3].level, 2);
        assert_eq!(h.iter().map(|d| d.count).sum::<u32>(), 3);
    }

    #[test]
    fn levels() {
        assert_eq!(level(0, 5), 0);
        assert_eq!(level(1, 8), 1);
        assert_eq!(level(8, 8), 4);
        assert_eq!(level(3, 0), 0);
    }
}

//! App state shared by all pages, saved in the browser (localStorage).
//!
//! Later phases swap the storage for IndexedDB and add sync; the pages only
//! talk to `Store`, so they do not need to change.

use std::time::Duration;

use checko_core::{
    all_areas, complete_task, parse_quick_add, reopen_task, Event, Note, Priority, Profile,
    Recurrence, ShoppingItem, Task,
};
use chrono::{NaiveDate, NaiveTime};
use leptos::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::time::{now, today};

const TASKS_KEY: &str = "checko.tasks.v1";
const PROFILE_KEY: &str = "checko.profile.v1";
const NOTES_KEY: &str = "checko.notes.v1";
const SHOPPING_KEY: &str = "checko.shopping.v1";
const EVENTS_KEY: &str = "checko.events.v1";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

pub fn load<T: DeserializeOwned + Default>(key: &str) -> T {
    storage()
        .and_then(|s| s.get_item(key).ok().flatten())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

pub fn save<T: Serialize>(key: &str, value: &T) {
    if let (Some(s), Ok(raw)) = (storage(), serde_json::to_string(value)) {
        let _ = s.set_item(key, &raw);
    }
}

/// A short message at the bottom of the screen, optionally with "Rückgängig".
#[derive(Clone, PartialEq)]
pub struct Toast {
    pub text: String,
    pub undo: Option<Undo>,
    /// Increases with every toast so an old timer does not hide a newer one.
    pub id: u32,
}

#[derive(Clone, PartialEq)]
pub enum Undo {
    Trash(Uuid),
    Complete(Uuid),
    NoteTrash(Uuid),
    EventTrash(Uuid),
    ShoppingClear(Vec<Uuid>),
}

/// A moment worth celebrating (shown as a full-screen overlay).
#[derive(Clone, PartialEq)]
pub enum Celebration {
    LevelUp(u32),
    Achievement(&'static checko_core::Achievement),
}

/// What a new task starts with.
#[derive(Clone)]
pub struct NewTask {
    pub title: String,
    pub priority: Priority,
    pub area: Option<String>,
    pub due: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub tags: Vec<String>,
    pub recurrence: Option<Recurrence>,
    /// Explicit "To-do" / "Termin" choice; `None` = automatic.
    pub appointment: Option<bool>,
    /// Reminder minutes before (see `Task::remind`).
    pub remind: Option<u32>,
}

#[derive(Clone, Copy)]
pub struct Store {
    pub tasks: RwSignal<Vec<Task>>,
    pub profile: RwSignal<Profile>,
    pub notes: RwSignal<Vec<Note>>,
    pub shopping: RwSignal<Vec<ShoppingItem>>,
    pub events: RwSignal<Vec<Event>>,
    pub toast: RwSignal<Option<Toast>>,
    /// The Strg+K quick-add dialog.
    pub quick_add_open: RwSignal<bool>,
    /// Day pre-selected in the add dialog (double-click in the calendar).
    pub quick_add_date: RwSignal<Option<NaiveDate>>,
    /// Task shown in the detail panel.
    pub detail: RwSignal<Option<Uuid>>,
    /// Calendar event shown in the detail panel.
    pub event_detail: RwSignal<Option<Uuid>>,
    /// Celebrations waiting to be shown (first one is visible).
    pub celebrations: RwSignal<Vec<Celebration>>,
}

impl Store {
    /// Create the store, load saved data and keep saving on every change.
    pub fn provide() -> Self {
        let store = Store {
            tasks: RwSignal::new(load(TASKS_KEY)),
            profile: RwSignal::new(load(PROFILE_KEY)),
            notes: RwSignal::new(load(NOTES_KEY)),
            shopping: RwSignal::new(load(SHOPPING_KEY)),
            events: RwSignal::new(load(EVENTS_KEY)),
            toast: RwSignal::new(None),
            quick_add_open: RwSignal::new(false),
            quick_add_date: RwSignal::new(None),
            detail: RwSignal::new(None),
            event_detail: RwSignal::new(None),
            celebrations: RwSignal::new(Vec::new()),
        };
        Effect::new(move |_| store.tasks.with(|t| save(TASKS_KEY, t)));
        Effect::new(move |_| store.profile.with(|p| save(PROFILE_KEY, p)));
        Effect::new(move |_| store.notes.with(|n| save(NOTES_KEY, n)));
        Effect::new(move |_| store.shopping.with(|s| save(SHOPPING_KEY, s)));
        Effect::new(move |_| store.events.with(|e| save(EVENTS_KEY, e)));
        store.watch_progress();
        provide_context(store);
        store
    }

    /// Watch for level-ups and newly reached achievements.
    fn watch_progress(&self) {
        let store = *self;
        // Level-ups: remember the level and celebrate when it grows.
        let last_level = StoredValue::new(
            store
                .profile
                .with_untracked(|p| checko_core::level_for_xp(p.xp).level),
        );
        Effect::new(move |_| {
            let level = store
                .profile
                .with(|p| checko_core::level_for_xp(p.xp).level);
            if level > last_level.get_value() && !store.profile.with_untracked(|p| p.focus_mode) {
                store
                    .celebrations
                    .update(|c| c.push(Celebration::LevelUp(level)));
            }
            last_level.set_value(level);
        });
        // Achievements: check whenever data changes.
        Effect::new(move |_| {
            let hour = js_sys::Date::new_0().get_hours();
            let mut new = Vec::new();
            store.tasks.with(|tasks| {
                store.notes.with(|notes| {
                    store.shopping.with(|shopping| {
                        let snapshot_profile = store.profile.get_untracked();
                        let snap = checko_core::Snapshot {
                            tasks,
                            notes,
                            shopping,
                            profile: &snapshot_profile,
                            today: today(),
                            hour,
                        };
                        let mut p = snapshot_profile.clone();
                        new = checko_core::unlock_new(&mut p, &snap);
                        if !new.is_empty() {
                            p.updated_at = Some(now());
                            store.profile.set(p);
                        }
                    })
                })
            });
            if !new.is_empty() && !store.profile.with_untracked(|p| p.focus_mode) {
                store
                    .celebrations
                    .update(|c| c.extend(new.into_iter().map(Celebration::Achievement)));
            }
        });
    }

    pub fn dismiss_celebration(&self) {
        self.celebrations.update(|c| {
            if !c.is_empty() {
                c.remove(0);
            }
        });
    }

    pub fn get() -> Self {
        expect_context::<Store>()
    }

    pub fn notify(&self, text: impl Into<String>, undo: Option<Undo>) {
        let id = self
            .toast
            .with_untracked(|t| t.as_ref().map_or(0, |t| t.id + 1));
        self.toast.set(Some(Toast {
            text: text.into(),
            undo,
            id,
        }));
        let toast = self.toast;
        set_timeout(
            move || {
                if toast.with_untracked(|t| t.as_ref().is_some_and(|t| t.id == id)) {
                    toast.set(None);
                }
            },
            Duration::from_millis(5000),
        );
    }

    pub fn dismiss_toast(&self) {
        self.toast.set(None);
    }

    /// Change a task in place.
    pub fn edit(&self, id: Uuid, f: impl FnOnce(&mut Task)) {
        self.tasks.update(|ts| {
            if let Some(t) = ts.iter_mut().find(|t| t.meta.id == id) {
                f(t);
            }
        });
    }

    pub fn add(&self, new: NewTask) {
        let title = new.title.trim();
        if title.is_empty() {
            return;
        }
        let mut t = Task::new(title, now());
        t.priority = new.priority;
        t.area = new.area;
        t.due = new.due;
        t.due_time = new.time;
        t.tags = new.tags;
        t.recurrence = new.recurrence;
        t.appointment = new.appointment;
        t.remind = new.remind;
        // Confirm where the task went when it is not for today.
        if let Some(d) = t.due.filter(|d| *d != today()) {
            let mut when = crate::time::due_label(d, today());
            if let Some(time) = t.due_time {
                when.push_str(&format!(" · {}", time.format("%H:%M")));
            }
            self.notify(format!("Eingetragen für {when}"), None);
        }
        self.tasks.update(|ts| ts.push(t));
    }

    /// All categories (defaults + custom ones in use), reactive.
    pub fn areas(&self) -> Vec<String> {
        let mut a = self.tasks.with(|t| all_areas(t));
        self.profile.with(|p| {
            a.retain(|c| !p.category_hidden(c));
            for k in p.kept_categories() {
                if !a.iter().any(|x| x.eq_ignore_ascii_case(k)) {
                    a.push(k.to_string());
                }
            }
        });
        a
    }

    /// Focus mode: no XP, level, streak, achievements or confetti.
    pub fn focus(&self) -> bool {
        self.profile.with(|p| p.focus_mode)
    }

    pub fn set_focus(&self, on: bool) {
        self.profile.update(|p| {
            p.focus_mode = on;
            p.updated_at = Some(now());
        });
    }

    /// Rename a category everywhere (tasks, events, own style).
    pub fn rename_category_everywhere(&self, old: &str, new: &str) {
        let new = new.trim().trim_start_matches('#').trim().to_string();
        if new.is_empty() || new == old {
            return;
        }
        let events = self.events;
        let mut n = 0;
        self.tasks.update(|ts| {
            events.update(|ev| n = checko_core::rename_category(ts, ev, old, &new, now()));
        });
        self.profile
            .update(|p| p.rename_category_style(old, &new, now()));
        self.notify(format!("„{old}“ heißt jetzt „{new}“ ({n} Einträge)"), None);
    }

    /// Remove a category from everything; the tasks themselves stay.
    pub fn delete_category_everywhere(&self, name: &str) {
        let events = self.events;
        let mut n = 0;
        self.tasks.update(|ts| {
            events.update(|ev| n = checko_core::delete_category(ts, ev, name, now()));
        });
        self.profile
            .update(|p| p.delete_category_style(name, now()));
        self.notify(
            format!("Kategorie „{name}“ gelöscht ({n} Einträge behalten)"),
            None,
        );
    }

    pub fn set_category_style(&self, name: &str, emoji: Option<String>, color: Option<String>) {
        self.profile
            .update(|p| p.set_category_style(name, emoji, color, now()));
    }

    /// Rename a task. "#Küche", "morgen", "!1" in the new text are applied too.
    pub fn rename(&self, id: Uuid, text: String) {
        if text.trim().is_empty() {
            return;
        }
        let areas = self.tasks.with_untracked(|t| all_areas(t));
        let p = parse_quick_add(&text, today(), &areas);
        self.edit(id, |t| {
            let before = t.clone();
            t.title = p.title;
            if p.area.is_some() {
                t.area = p.area;
            }
            if let Some(pr) = p.priority {
                t.priority = pr;
            }
            if p.due.is_some() {
                t.due = p.due;
                t.due_time = p.time;
            }
            for tag in p.tags {
                if !t.tags.contains(&tag) {
                    t.tags.push(tag);
                }
            }
            if *t != before {
                t.meta.updated_at = now();
            }
        });
    }

    pub fn add_category(&self, id: Uuid, c: &str) {
        self.profile.update(|p| p.unhide_category(c, now()));
        self.edit(id, |t| t.add_category(c, now()));
    }

    pub fn remove_category(&self, id: Uuid, c: &str) {
        self.edit(id, |t| t.remove_category(c, now()));
    }

    /// Move the due date (None removes it, and the time with it).
    pub fn set_due(&self, id: Uuid, due: Option<NaiveDate>, time: Option<NaiveTime>) {
        self.edit(id, |t| {
            t.due = due;
            t.due_time = if due.is_some() { time } else { None };
            t.meta.updated_at = now();
        });
    }

    pub fn set_priority(&self, id: Uuid, p: Priority) {
        self.edit(id, |t| {
            t.priority = p;
            t.meta.updated_at = now();
        });
    }

    pub fn complete(&self, id: Uuid) {
        let mut xp = 0;
        let profile = self.profile;
        self.tasks.update(|ts| {
            profile.update(|p| {
                xp = complete_task(ts, p, id, now(), today());
                p.updated_at = Some(now());
            });
        });
        if xp > 0 {
            let text = if self.focus() {
                "Erledigt!".to_string()
            } else {
                format!("Erledigt! +{xp} XP")
            };
            self.notify(text, Some(Undo::Complete(id)));
        }
    }

    pub fn reopen(&self, id: Uuid) {
        let profile = self.profile;
        self.tasks.update(|ts| {
            profile.update(|p| {
                reopen_task(ts, p, id, now());
                p.updated_at = Some(now());
            })
        });
    }

    pub fn trash(&self, id: Uuid) {
        self.edit(id, |t| t.trash(now()));
        self.notify("In den Papierkorb verschoben", Some(Undo::Trash(id)));
    }

    pub fn restore(&self, id: Uuid) {
        self.edit(id, |t| t.restore(now()));
    }

    /// Permanently remove a task from the trash.
    pub fn purge(&self, id: Uuid) {
        crate::sync::tombstone("task", id);
        self.tasks.update(|ts| ts.retain(|t| t.meta.id != id));
    }

    pub fn empty_trash(&self) {
        self.tasks.update(|ts| {
            for t in ts.iter().filter(|t| t.is_deleted()) {
                crate::sync::tombstone("task", t.meta.id);
            }
            ts.retain(|t| !t.is_deleted())
        });
    }

    // ---------- Calendar ----------

    pub fn add_event(&self, e: Event) {
        if e.title.is_empty() {
            return;
        }
        self.events.update(|es| es.push(e));
    }

    pub fn edit_event(&self, id: Uuid, f: impl FnOnce(&mut Event)) {
        self.events.update(|es| {
            if let Some(e) = es.iter_mut().find(|e| e.meta.id == id) {
                f(e);
                e.meta.updated_at = now();
            }
        });
    }

    pub fn delete_event(&self, id: Uuid) {
        self.edit_event(id, |e| e.meta.deleted_at = Some(now()));
        self.notify("Termin gelöscht", Some(Undo::EventTrash(id)));
    }

    /// Move a calendar item (event or task) to another day.
    pub fn move_to_day(&self, kind: checko_core::ItemKind, id: Uuid, day: NaiveDate) {
        match kind {
            checko_core::ItemKind::Event => self.edit_event(id, |e| e.date = day),
            checko_core::ItemKind::Task => {
                let time = self.tasks.with_untracked(|ts| {
                    ts.iter().find(|t| t.meta.id == id).and_then(|t| t.due_time)
                });
                self.set_due(id, Some(day), time);
            }
        }
    }

    // ---------- Notes ----------

    /// Create an empty note and return its id.
    pub fn new_note(&self) -> Uuid {
        let n = Note::new(now());
        let id = n.meta.id;
        self.notes.update(|ns| ns.push(n));
        id
    }

    pub fn edit_note(&self, id: Uuid, f: impl FnOnce(&mut Note)) {
        self.notes.update(|ns| {
            if let Some(n) = ns.iter_mut().find(|n| n.meta.id == id) {
                f(n);
                n.meta.updated_at = now();
            }
        });
    }

    pub fn trash_note(&self, id: Uuid) {
        let empty = self.notes.with_untracked(|ns| {
            ns.iter()
                .find(|n| n.meta.id == id)
                .is_some_and(Note::is_empty)
        });
        if empty {
            crate::sync::tombstone("note", id);
            self.notes.update(|ns| ns.retain(|n| n.meta.id != id));
            return;
        }
        self.notes.update(|ns| {
            if let Some(n) = ns.iter_mut().find(|n| n.meta.id == id) {
                n.meta.deleted_at = Some(now());
            }
        });
        self.notify("Notiz gelöscht", Some(Undo::NoteTrash(id)));
    }

    // ---------- Shopping ----------

    pub fn add_shopping(&self, input: &str) {
        let input = input.trim();
        if input.is_empty() {
            return;
        }
        let item = ShoppingItem::new(input, now());
        self.shopping.update(|items| {
            // Re-adding something already on the list (checked) just un-checks it.
            if let Some(existing) = items.iter_mut().find(|i| {
                i.meta.deleted_at.is_none() && i.checked && i.name.eq_ignore_ascii_case(&item.name)
            }) {
                existing.checked = false;
                existing.qty = item.qty.clone();
                if item.url.is_some() {
                    existing.url = item.url.clone();
                }
                existing.meta.updated_at = now();
            } else {
                items.push(item);
            }
        });
    }

    pub fn toggle_shopping(&self, id: Uuid) {
        self.shopping.update(|items| {
            if let Some(i) = items.iter_mut().find(|i| i.meta.id == id) {
                i.toggle(now());
            }
        });
    }

    pub fn remove_shopping(&self, id: Uuid) {
        crate::sync::tombstone("shopping", id);
        self.shopping
            .update(|items| items.retain(|i| i.meta.id != id));
    }

    /// Hide all checked items (they stay in the shopping history).
    pub fn clear_checked_shopping(&self) {
        let mut ids = Vec::new();
        self.shopping.update(|items| {
            for i in items
                .iter_mut()
                .filter(|i| i.checked && i.meta.deleted_at.is_none())
            {
                i.meta.deleted_at = Some(now());
                ids.push(i.meta.id);
            }
        });
        if !ids.is_empty() {
            let n = ids.len();
            self.notify(
                format!("{n} erledigte Einkäufe entfernt"),
                Some(Undo::ShoppingClear(ids)),
            );
        }
    }

    pub fn undo(&self, undo: Undo) {
        match undo {
            Undo::Trash(id) => self.restore(id),
            Undo::Complete(id) => self.reopen(id),
            Undo::EventTrash(id) => self.edit_event(id, |e| e.meta.deleted_at = None),
            Undo::NoteTrash(id) => self.notes.update(|ns| {
                if let Some(n) = ns.iter_mut().find(|n| n.meta.id == id) {
                    n.meta.deleted_at = None;
                }
            }),
            Undo::ShoppingClear(ids) => self.shopping.update(|items| {
                for i in items.iter_mut().filter(|i| ids.contains(&i.meta.id)) {
                    i.meta.deleted_at = None;
                }
            }),
        }
        self.dismiss_toast();
    }
}

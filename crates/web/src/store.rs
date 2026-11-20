//! App state shared by all pages, saved in the browser (localStorage).
//!
//! Later phases swap the storage for IndexedDB and add sync; the pages only
//! talk to `Store`, so they do not need to change.

use std::time::Duration;

use checko_core::{
    all_areas, complete_task, parse_quick_add, reopen_task, Note, Priority, Profile, ShoppingItem,
    Task,
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

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn load<T: DeserializeOwned + Default>(key: &str) -> T {
    storage()
        .and_then(|s| s.get_item(key).ok().flatten())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

fn save<T: Serialize>(key: &str, value: &T) {
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
    ShoppingClear(Vec<Uuid>),
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
}

#[derive(Clone, Copy)]
pub struct Store {
    pub tasks: RwSignal<Vec<Task>>,
    pub profile: RwSignal<Profile>,
    pub notes: RwSignal<Vec<Note>>,
    pub shopping: RwSignal<Vec<ShoppingItem>>,
    pub toast: RwSignal<Option<Toast>>,
    /// The Strg+K quick-add dialog.
    pub quick_add_open: RwSignal<bool>,
}

impl Store {
    /// Create the store, load saved data and keep saving on every change.
    pub fn provide() -> Self {
        let store = Store {
            tasks: RwSignal::new(load(TASKS_KEY)),
            profile: RwSignal::new(load(PROFILE_KEY)),
            notes: RwSignal::new(load(NOTES_KEY)),
            shopping: RwSignal::new(load(SHOPPING_KEY)),
            toast: RwSignal::new(None),
            quick_add_open: RwSignal::new(false),
        };
        Effect::new(move |_| store.tasks.with(|t| save(TASKS_KEY, t)));
        Effect::new(move |_| store.profile.with(|p| save(PROFILE_KEY, p)));
        Effect::new(move |_| store.notes.with(|n| save(NOTES_KEY, n)));
        Effect::new(move |_| store.shopping.with(|s| save(SHOPPING_KEY, s)));
        provide_context(store);
        store
    }

    pub fn get() -> Self {
        expect_context::<Store>()
    }

    fn notify(&self, text: impl Into<String>, undo: Option<Undo>) {
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

    fn edit(&self, id: Uuid, f: impl FnOnce(&mut Task)) {
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
        self.tasks.update(|ts| ts.push(t));
    }

    /// All categories (defaults + custom ones in use), reactive.
    pub fn areas(&self) -> Vec<String> {
        self.tasks.with(|t| all_areas(t))
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
            profile.update(|p| xp = complete_task(ts, p, id, now(), today()));
        });
        if xp > 0 {
            self.notify(format!("Erledigt! +{xp} XP"), Some(Undo::Complete(id)));
        }
    }

    pub fn reopen(&self, id: Uuid) {
        let profile = self.profile;
        self.tasks
            .update(|ts| profile.update(|p| reopen_task(ts, p, id, now())));
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
        self.tasks.update(|ts| ts.retain(|t| t.meta.id != id));
    }

    pub fn empty_trash(&self) {
        self.tasks.update(|ts| ts.retain(|t| !t.is_deleted()));
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
        self.shopping
            .update(|items| items.retain(|i| i.meta.id != id));
    }

    /// Hide all checked items (kept for "Oft gekauft" suggestions).
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

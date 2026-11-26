# Checko – verified feature inventory (code state at commit 14febb2, 2026-11)

All sources are local files in /home/user/checko (paths relative to the repo root). "Verified" = found in code, not only in docs. Status hints for the report writer: HAS / PARTIAL / MISSING.

## 1. Task basics (due date/time, reminders, recurrence, steps, notes, attachments, priorities, tags, status, trash/undo)

### Takeaway
Core task features are solid: due date + optional time, reminders (fixed "minutes before" options) via in-app timer and server Web Push, simple recurrence (daily/weekly/monthly/yearly with interval), steps, plain-text notes, 4 priority levels, multiple categories, soft-delete trash with restore and undo toast. Missing: attachments, visible "In Arbeit" status in the UI, visible activity log, end date for recurrence in the UI, specific-weekday multi-select recurrence.

### Cited Findings
- **Data model**: `Task { title, notes, status (Planned|InProgress|Done), priority (P1–P4), area, tags, due: NaiveDate, due_time, remind_at, log: Vec<LogEntry>, done_at, done_on, xp_awarded, steps, recurrence, appointment: Option<bool>, remind: Option<u32> }`, every entity has UUID v7 + created_at/updated_at/deleted_at — [crates/core/src/model.rs](crates/core/src/model.rs)
- **Due date + time**: date and optional local time of day; detail panel has quick-move chips, a date field, a time field and "Termin entfernen" — [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs) (lines ~121–223)
- **Reminders (HAS, with limits)**: field `remind` = minutes before due time; all-day items remind relative to 09:00 (`ALL_DAY_REMINDER = (9, 0)`) — [crates/core/src/calendar.rs](crates/core/src/calendar.rs). UI choices are fixed: Keine, Zum Zeitpunkt, 5/15/30 Min., 1 Std., 1 Tag vorher (`REMIND_OPTIONS`) — [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs). Only one reminder per item; no custom absolute reminder time in the UI (`remind_at` exists in the model but is not used by the UI). No location-based reminders.
- **Recurrence engine**: `Recurrence { freq: Daily|Weekly|Monthly|Yearly, interval, until }`; month-end clamping (31 Jan + 1 month → 28/29 Feb) — [crates/core/src/calendar.rs](crates/core/src/calendar.rs). Completing a repeating task creates the next occurrence (copies notes, priority, categories, time, reminder, resets steps) — `complete_task` in [crates/core/src/tasks.rs](crates/core/src/tasks.rs)
- **Recurrence in the UI is narrower than the engine**: dropdown offers only Nie / Jeden Tag / Jede Woche / Alle 2 Wochen / Jeden Monat / Jedes Jahr (`repeat_from`/`repeat_value`) — [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs), [crates/web/src/components/task_form.rs](crates/web/src/components/task_form.rs). Arbitrary intervals (e.g. "alle 6 Wochen") only via quick-add text; `until` (end date) never set by parser or UI; no "every Mon+Wed", no "last weekday of month", no "X days after completion".
- **Steps/subtasks (HAS)**: `Step { id, text, done }`, add/toggle/remove, progress "d/n" shown — [crates/core/src/tasks.rs](crates/core/src/tasks.rs), [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs) ("Schritte"). Single level only (no nested subtasks).
- **Notes on a task (HAS, plain text)**: textarea "Notizen" ("Details, Links, Telefonnummern …"), no Markdown rendering — [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs)
- **Attachments (MISSING)**: no file/attachment field in `Task` — [crates/core/src/model.rs](crates/core/src/model.rs)
- **Priorities (HAS)**: 4 levels P1–P4 ("Dringlichkeit" dots); P4 default; click on the priority dot in a list row cycles the priority — [crates/core/src/model.rs](crates/core/src/model.rs), [crates/web/src/components/task_item.rs](crates/web/src/components/task_item.rs)
- **Categories/tags (HAS)**: first category = `area`, further ones = `tags`; add/remove in detail panel with autocomplete of existing categories — [crates/core/src/tasks.rs](crates/core/src/tasks.rs), [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs)
- **Status (PARTIAL)**: core supports Planned → InProgress → Done with `start()`/`pause()` and log entries — [crates/core/src/tasks.rs](crates/core/src/tasks.rs); but no web code calls `start`/`pause` or references `InProgress` (grep over crates/web/src found nothing), so in practice the UI only has open/done.
- **Activity log (PARTIAL/hidden)**: `log` entries ("Begonnen", "Erledigt", "Wieder geöffnet", "In den Papierkorb", "Wiederhergestellt") are written in core — [crates/core/src/tasks.rs](crates/core/src/tasks.rs); no UI displays the log (no reference in crates/web/src).
- **Trash (HAS)**: soft delete, tab "Papierkorb" with restore, purge and "Papierkorb leeren" (with `confirm` dialog) — [crates/web/src/pages/tasks.rs](crates/web/src/pages/tasks.rs), [crates/web/src/store.rs](crates/web/src/store.rs)
- **Undo (HAS, limited)**: toast with "Rückgängig" for: task trashed, task completed, note trashed, event trashed, shopping "clear checked" (`enum Undo`) — [crates/web/src/store.rs](crates/web/src/store.rs). No general multi-step undo history. Deleting a single task asks no confirmation (goes to trash with undo).
- **To-do vs. appointment**: a task with a time is automatically an "appointment" (Termin), switchable via "Anzeigen als" — `is_appointment` in [crates/core/src/tasks.rs](crates/core/src/tasks.rs), [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs)

### Inferences
- Compared to Microsoft To Do / Todoist: comparable on due/remind/steps/priority basics; weaker on recurrence flexibility and reminder customisation; no attachments.

### Gaps
- No evidence of a "start date" / "deadline vs scheduled date" distinction (only one `due`).

## 2. Organization (lists, smart views, filters, sorting, ordering, search, completed)

### Takeaway
Checko organises by "Kategorien" (tags) rather than lists. Smart views: Heute dashboard, Aufgaben page (Offen/Erledigt/Papierkorb; To-dos and Termine side by side), Kalender. Category filter exists; sorting is fixed and automatic; no manual reordering; global search over tasks, notes, shopping in Strg+K.

### Cited Findings
- **Default categories**: Arbeit, Privat, Gesundheit (`AREAS`); custom categories appear automatically once used on a task (`all_areas`) — [crates/core/src/tasks.rs](crates/core/src/tasks.rs)
- **Category management (PARTIAL)**: created implicitly via `#wort` or "+ Neu" on a task; no rename, delete, colour or icon management found (no such store functions in [crates/web/src/store.rs](crates/web/src/store.rs)). Sidebar has a collapsible "Kategorien" group — [crates/web/src/components/sidebar.rs](crates/web/src/components/sidebar.rs)
- **No real lists/projects**: no `List`/`Project` entity in [crates/core/src/model.rs](crates/core/src/model.rs); no nested projects/sections.
- **Routes/views**: `/` (Heute), `/aufgaben`, `/kalender`, `/notizen`, `/einkauf`, `/einstellungen` — [crates/web/src/main.rs](crates/web/src/main.rs)
- **Heute dashboard**: "Als Nächstes" hero, "Termine heute", "Heute zu tun" (in progress or due today/overdue: `today_tasks`), "Fortschritt & Erfolge" (level, XP bar, streak + 14-day area chart), "Kategorien" (half-circle arcs done per category) — [crates/web/src/pages/today.rs](crates/web/src/pages/today.rs), [crates/core/src/tasks.rs](crates/core/src/tasks.rs)
- **Aufgaben page**: tabs Offen / Erledigt / Papierkorb with counts; in "Offen" two columns "To-dos" and "Termine"; category filter chips (`AreaFilter`), also via URL `?kategorie=…` — [crates/web/src/pages/tasks.rs](crates/web/src/pages/tasks.rs), [crates/web/src/components/area_filter.rs](crates/web/src/components/area_filter.rs)
- **Sorting (fixed, no options)**: open tasks sorted in-progress first, then priority, then due date (none last), then creation; appointments by date+time; done by most recent — [crates/core/src/tasks.rs](crates/core/src/tasks.rs). No user-selectable sort, no manual drag reordering within a list.
- **Drag & drop**: task rows are draggable (fine pointer only) onto the sidebar "Heute" entry to set due = today, and onto calendar day cells to move them — [crates/web/src/components/sidebar.rs](crates/web/src/components/sidebar.rs), [crates/web/src/components/task_item.rs](crates/web/src/components/task_item.rs), [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs)
- **Search (HAS)**: `search()` matches all words (case-insensitive substring) in task title/notes/categories, note title/body, shopping item names; excludes trashed items; max 8 hits in the Strg+K dialog; clicking a note hit opens `?id=` — [crates/core/src/notes.rs](crates/core/src/notes.rs), [crates/web/src/components/quick_add.rs](crates/web/src/components/quick_add.rs), [crates/web/src/pages/notes.rs](crates/web/src/pages/notes.rs). Calendar events are NOT searched.
- **Completed tasks (HAS)**: "Erledigt" tab, reopen possible (XP is taken back) — [crates/core/src/tasks.rs](crates/core/src/tasks.rs)
- **Overdue**: `is_overdue` used; overdue tasks are included in "Heute" — [crates/core/src/tasks.rs](crates/core/src/tasks.rs)

### Inferences
- Missing compared to Todoist/TickTick: saved custom filters, sorting options, manual order, project hierarchy, Eisenhower/Kanban boards.

### Gaps
- Did not verify the exact markup of every Heute card beyond section titles.

## 3. Natural-language quick add (exact syntax)

### Takeaway
German NL parser in pure Rust with tests; supports relative/absolute dates, times, priorities, category/tags and simple repeat phrases; recognised tokens are shown as chips while typing. Only the first date and first time are used.

### Cited Findings (all from [crates/core/src/parse.rs](crates/core/src/parse.rs) incl. its tests)
- **Dates**: `heute`, `morgen`, `übermorgen` (also `uebermorgen`), weekdays `montag … sonntag` (today counts if same weekday), `nächste/nächster/nächstes/nächsten/kommende/kommenden + Wochentag` (strictly after today), `nächste woche` (= next Monday), `in N tag/tage/tagen`, `in N woche/wochen`, numeric `24.12.`, `24.12`, `24.12.2026`, `24.12.26`; optional prefix `am`/`bis`/`ab`. A date without year that already passed → next year. Invalid dates (31.2.) ignored. `ae/ue` normalised to ä/ü.
- **Times**: `14:00`, `9:30`, `14 uhr`, `14uhr`, `um 14`, `um 14:30`, `um 8:15 uhr`. Time without date → today. Invalid (25:00) ignored. No "abends/morgens/mittags", no "14:00-15:00" range.
- **Priority**: `!1`…`!4`, `p1`…`p4`; `!!`/`!!!` = P1; single `!` = P2.
- **Category/tags**: first `#wort` = category (matches existing case-insensitively, otherwise new, capitalised); further `#wörter` = tags; after connector words (für, im, am, zum …) the word stays in the title ("Lampe für #Küche kaufen" → "Lampe für Küche kaufen").
- **Repeat**: `täglich`, `wöchentlich`, `monatlich`, `jährlich`; `jeden/jede/jedes + Tag/Woche/Monat/Jahr`; `jeden Montag` (weekly, first date = next Monday); `alle N Tage/Wochen/Monate/Jahre`. Repeat without date starts today. "Jede Menge Arbeit" stays a title.
- **Not parsed**: reminders, durations/end times, English words, "Ende des Monats", "in 2 Stunden".
- Parser is used in the Strg+K/add dialog with live token chips — [crates/web/src/components/task_form.rs](crates/web/src/components/task_form.rs) — and in the calendar "Termin hinzufügen" field — [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs)
- **Shortcut**: Strg+K / Cmd+K opens the dialog, Esc closes; also via sidebar search button, top bar, mobile FAB "+" — [crates/web/src/components/quick_add.rs](crates/web/src/components/quick_add.rs), [crates/web/src/components/topbar.rs](crates/web/src/components/topbar.rs)

### Inferences
- Comparable to Todoist's quick add for German basics; no NL reminder syntax.

### Gaps
- None significant.

## 4. Calendar

### Takeaway
Month grid and 30-day agenda; separate Event entity plus tasks with due dates; drag & drop between days (mouse only); double-click a day to open the add dialog; recurrence and reminders for events. No week/day view, no external calendar sync.

### Cited Findings
- **Views**: `View::Month` and `View::Agenda` only ("Monat"/"Agenda" segmented control); month navigation + "Heute" button; agenda shows next 30 days — [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs). Month grid = 6×7 starting Monday — `month_grid` in [crates/core/src/calendar.rs](crates/core/src/calendar.rs). **Week/day view: MISSING** (PLAN phase 6 marked 🟡 — [docs/PLAN.md](docs/PLAN.md)).
- **Event model**: `Event { title, date, start, end, category, notes, recurrence, remind }` (single-day; no multi-day events, no location, no attendees, no colour field) — [crates/core/src/calendar.rs](crates/core/src/calendar.rs)
- **Combined view**: `items_between` merges event occurrences and tasks with due dates; appointment tasks shown as time blocks, to-dos as checkable chips; "+N mehr" overflow — [crates/core/src/calendar.rs](crates/core/src/calendar.rs), [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs)
- **Drag & drop (HAS, desktop only)**: chips `draggable` only when `fine_pointer()`; drop on day calls `move_to_day` — [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs), [crates/web/src/store.rs](crates/web/src/store.rs). No resizing of time blocks.
- **Double-click** on a day opens the add dialog with that date (`quick_add_date`) — [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs), [crates/web/src/store.rs](crates/web/src/store.rs)
- **Side panel**: selected day's items + form "Termin hinzufügen" (text with NL parsing, Von/Bis times, Wiederholen: Einmalig/Jeden Tag/Jede Woche/Alle 2 Wochen/Jeden Monat/Jedes Jahr) — [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs)
- **Event detail panel** (`EventDetail`): title, date, from/to, Wiederholen, Erinnerung, Kategorie, Notizen, Löschen — [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs)
- **No external calendar integration**: no ICS/CalDAV/Google/Outlook code found (server routes only register/login/logout/sync/widget/push — [crates/server/src/main.rs](crates/server/src/main.rs)).
- Editing one occurrence of a repeating event separately: not supported (recurrence is on the whole event; no exceptions field) — [crates/core/src/calendar.rs](crates/core/src/calendar.rs)

### Gaps
- None beyond noted.

## 5. Notes, shopping list, gamification

### Takeaway
Notes are simple (title/body, pin, delete) with no Markdown rendering or [[links]]. Shopping list is relatively rich (auto aisle categories, quantities, product links with title extraction, purchase history, suggestions, clear checked with undo) but no swipe gestures or sharing. Gamification: XP, levels, streak, 12 achievements, level-up/achievement confetti overlays; themes per level, mascot and focus mode missing.

### Cited Findings
- **Notes**: `Note { title, body_md, pinned, tags }`; list sorted pinned first then last edited; pin/unpin, delete (trash + undo) — [crates/core/src/model.rs](crates/core/src/model.rs), [crates/core/src/notes.rs](crates/core/src/notes.rs), [crates/web/src/pages/notes.rs](crates/web/src/pages/notes.rs). No Markdown renderer dependency in [crates/web/Cargo.toml](crates/web/Cargo.toml) and no `[[` handling in web code → **Markdown rendering & [[Links]]: MISSING**; note tags not editable in UI (no reference found). No notes trash view to restore later (only undo toast).
- **Shopping**: 8 aisle categories in supermarket order (Obst & Gemüse … Sonstiges), auto-guessed from keywords (`guess_category`); quantity parsing "2x Milch", "Milch 2l", "3 Äpfel" (`parse_item`); first URL extracted as product link, readable title from Amazon-style URLs (`extract_link`, `title_from_url`, `link_host`); `times_bought`, `suggestions`, `purchase_history` — [crates/core/src/shopping.rs](crates/core/src/shopping.rs). UI: tabs "Liste"/"Verlauf", re-add from history, open link, "clear checked", remove — [crates/web/src/pages/shopping.rs](crates/web/src/pages/shopping.rs). Category of an item not manually changeable (no UI found). **Swipe gestures: MISSING** (no touch/pointer swipe code in crates/web/src). **Shared shopping lists: MISSING** (PLAN "Später" — [docs/PLAN.md](docs/PLAN.md)).
- **XP**: priority weight (P1=4 … P4=1) × 10, +50 % if done on/before due date (P1 on time = 60 XP) — [crates/core/src/gamification.rs](crates/core/src/gamification.rs). Reopen removes XP.
- **Levels**: XP to next level = 100 + (level−1)×50 — [crates/core/src/gamification.rs](crates/core/src/gamification.rs)
- **Streak**: consecutive days with at least one completion; breaks if yesterday had none — [crates/core/src/gamification.rs](crates/core/src/gamification.rs)
- **Achievements (12)**: Erster Haken, In Fahrt (10), Profi (50), Feuerwehr (urgent), Pünktlich (5 on time), Frühaufsteher (before 8:00), Dranbleiben (3-day streak), Eine Woche stark (7-day), Level 5, Ordnung muss sein (3 categories), Einkaufsheld (10 shopping items), Notizbuch (5 notes) — [crates/core/src/achievements.rs](crates/core/src/achievements.rs)
- **Celebrations**: full-screen overlay with 28 confetti pieces for LevelUp/Achievement; spark animation on check — [crates/web/src/components/celebrate.rs](crates/web/src/components/celebrate.rs), [crates/web/src/components/task_item.rs](crates/web/src/components/task_item.rs), `Celebration` enum in [crates/web/src/store.rs](crates/web/src/store.rs)
- **Mascot: MISSING (paused)** — "Mascot is paused for now (kept for later)" — [crates/web/src/components/mod.rs](crates/web/src/components/mod.rs); **themes per level, focus mode: MISSING** ("Zurückgestellt" — [CLAUDE.md](CLAUDE.md)); gamification cannot be switched off (no setting found in [crates/web/src/pages/settings.rs](crates/web/src/pages/settings.rs)).

### Gaps
- Did not verify whether achievements list is displayed as a gallery or only via celebration overlay (Heute shows "Fortschritt & Erfolge" section).

## 6. Widgets, sync, offline/PWA, notifications, themes, shortcuts, collaboration, import/export, platforms, AI

### Takeaway
Web app (Leptos/WASM) on GitHub Pages, installable PWA with network-first service worker; data local-first in localStorage; optional account-based sync via own Axum + Postgres (Neon) server, last-write-wins; Web Push reminders from server; iPhone widgets via Scriptable (home + lock screen). No collaboration, no import/export, no native apps, no AI, no Android widget.

### Cited Findings
- **Storage**: localStorage keys `checko.tasks.v1`, `.profile.v1`, `.notes.v1`, `.shopping.v1`, `.events.v1`, saved on every change — [crates/web/src/store.rs](crates/web/src/store.rs). IndexedDB not used (planned phase 7 — [docs/PLAN.md](docs/PLAN.md)).
- **Sync (HAS)**: username/password accounts (Argon2 hashes, bearer session tokens), `POST /api/sync` with cursor; server upsert only if `records.updated_at < EXCLUDED.updated_at` → **last write wins per record** (no field-level merge, no conflict UI); deletions via tombstones — [crates/server/src/main.rs](crates/server/src/main.rs), [crates/web/src/sync.rs](crates/web/src/sync.rs). Client syncs every 30 s and on window focus, plus "Jetzt synchronisieren" button; settings: server address, Anmelden / Konto erstellen / Abmelden — [crates/web/src/sync.rs](crates/web/src/sync.rs), [crates/web/src/pages/settings.rs](crates/web/src/pages/settings.rs). Registration can be closed (`ALLOW_REGISTRATION=false`) — [crates/server/src/main.rs](crates/server/src/main.rs). Single-user per account; no sharing.
- **PWA (HAS)**: `manifest.webmanifest` (standalone, icons incl. maskable) and `sw.js` (network first, cache fallback offline) registered in index.html — [crates/web/manifest.webmanifest](crates/web/manifest.webmanifest), [crates/web/sw.js](crates/web/sw.js), [crates/web/index.html](crates/web/index.html)
- **Notifications (HAS)**: in-app timer every 30 s shows due reminders via service worker; Web Push when signed in — server VAPID keys stored in DB, scheduler every 60 s using `reminders_between`, 10-minute look-back for a sleeping free server, `push_sent` dedupe; notification click focuses/opens app. iOS: only for the home-screen web app, iOS 16.4+. Settings has enable + "Test-Benachrichtigung" — [crates/web/src/notify.rs](crates/web/src/notify.rs), [crates/server/src/push.rs](crates/server/src/push.rs), [crates/web/sw.js](crates/web/sw.js), [crates/web/src/pages/settings.rs](crates/web/src/pages/settings.rs). No snooze/complete actions in notifications (sw.js only has click → open).
- **Widgets (HAS, iPhone via Scriptable only)**: server `GET /api/widget` returns date, today's tasks, open count, next items (30 days), streak, level — [crates/server/src/main.rs](crates/server/src/main.rs). Scriptable script supports home screen small/medium/large and lock screen accessoryRectangular/Circular/Inline, offline cache in Keychain, self-updating loader fetched from GitHub Pages — [docs/widget/checko-widget-core.js](docs/widget/checko-widget-core.js), [docs/widget/checko-widget.js](docs/widget/checko-widget.js); setup instructions + "Widget-Skript kopieren" in settings — [crates/web/src/pages/settings.rs](crates/web/src/pages/settings.rs). Read-only (no interactive check-off); requires the third-party Scriptable app and sync account.
- **Themes/dark mode (HAS)**: Light / Dark / System (prefers-color-scheme) switch — [crates/web/src/components/theme.rs](crates/web/src/components/theme.rs). `prefers-reduced-motion` respected in CSS — [crates/web/style/input.css](crates/web/style/input.css). No colour themes.
- **Keyboard**: Strg/Cmd+K (add/search), Esc closes dialogs/detail, Enter in fields; no further shortcut set found — [crates/web/src/components/quick_add.rs](crates/web/src/components/quick_add.rs), [crates/web/src/components/task_detail.rs](crates/web/src/components/task_detail.rs)
- **Collaboration/sharing: MISSING** (no shared lists, assignment, comments; server stores per `user_id` only — [crates/server/src/main.rs](crates/server/src/main.rs)).
- **Import/export: MISSING** — PLAN vision promises "Export/Import als JSON & Markdown" ([docs/PLAN.md](docs/PLAN.md)) but no export/import code in web or server (no such routes/settings).
- **Platforms**: browser/PWA only (desktop + mobile responsive, mobile top bar + FAB — [crates/web/src/components/topbar.rs](crates/web/src/components/topbar.rs)); Tauri desktop/mobile apps not started ("Später" — [docs/PLAN.md](docs/PLAN.md)); no Apple Watch, no browser extension, no email-to-task, no Siri/assistant integration.
- **AI assistant: MISSING** (phase 8 not started; no Claude API code in server) — [docs/PLAN.md](docs/PLAN.md), [crates/server/src/main.rs](crates/server/src/main.rs)

### Inferences
- Sync conflict handling is simple: concurrent edits of the same task on two devices → whole-record overwrite by the later `updated_at`; offline edits are kept locally and pushed later.

### Gaps
- Not verified whether the service worker pre-caches the WASM on first load (it caches on fetch, so offline works only after a visit).

## 7. Planned but NOT implemented (summary list for "missing")

### Takeaway
Plan items still open, verified absent in code.

### Cited Findings
- Calendar week/day views (only Monat/Agenda) — [crates/web/src/pages/calendar.rs](crates/web/src/pages/calendar.rs); PLAN phase 6 🟡 — [docs/PLAN.md](docs/PLAN.md)
- "In Arbeit" status UI and visible per-task history log (core only) — [crates/core/src/tasks.rs](crates/core/src/tasks.rs)
- Markdown rendering and `[[Links]]` in notes; note tags/colour — [crates/web/src/pages/notes.rs](crates/web/src/pages/notes.rs)
- Swipe gestures (shopping/tasks) — no touch code in crates/web/src
- Themes per level, focus mode, axolotl mascot — [CLAUDE.md](CLAUDE.md), [crates/web/src/components/mod.rs](crates/web/src/components/mod.rs)
- Morgen-Plan, Abend-Check-in, Wochenrückblick ("folgen später") — [docs/PLAN.md](docs/PLAN.md)
- IndexedDB storage, JSON/Markdown export/import — [docs/PLAN.md](docs/PLAN.md), [crates/web/src/store.rs](crates/web/src/store.rs)
- KI-Assistent (Claude API), Tauri apps, shared shopping lists — [docs/PLAN.md](docs/PLAN.md)
- Recurrence end date (`until`) and custom intervals in UI; category rename/delete/colour; attachments; manual sort; calendar event search.

### Inferences
- Overall Checko is a single-user personal organizer with strong German NL input and gamification, at feature level roughly between Microsoft To Do (basics) and a lightweight planner, lacking collaboration and import/export.

### Gaps
- None.

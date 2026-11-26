# Todoist & TickTick – Feature Inventory (as of Nov 2026)

Research date: 2026-11-26. Primary sources: official pricing/feature pages and the Todoist 2026 changelog. Third-party reviews (2sync, techharry, temporal.day etc.) were used where official data was missing; they disagree on several points, and those disagreements are flagged below.

## Pricing and plans (free vs paid)

### Takeaway
Todoist: Beginner (free), Pro at $60/yr or $7/mo, Business at $96/user/yr or $10/user/mo. TickTick: Free, plus Premium at US$49.99/yr according to the official page. No monthly price is listed there, and there is no team plan. Several 2025-era reviews still quote the older $35.99/yr.

### Cited Findings
- Todoist Pro: $5/mo billed yearly ($60/yr) or $7/mo monthly. Business: $8/user/mo yearly ($96) or $10/user/mo monthly. — [Todoist Pricing](https://todoist.com/pricing)
- Todoist limits Beginner / Pro / Business: personal projects 5 / 300 / 300 per member; custom filters 3 / 150 / 150; connected calendars 1 / 3 / 3; file upload 5 MB / 100 MB / 100 MB; reporting history 1 week / full / full; collaborators per personal project 5 on all plans. — [Todoist Pricing](https://todoist.com/pricing)
- Todoist Pro-only (not in Beginner): custom reminders, task deadlines, task duration, auto backups, task capture from email, calendar and table layouts, board layout in label and filter views, project insights, Automations (beta). Business adds team workspace, roles, folders, shared templates, team activity reporting and advanced insights. — [Todoist Pricing](https://todoist.com/pricing)
- Todoist on all plans: dates and times, priorities, sub-tasks, list layout, board layout (in projects), comments, assigned tasks, recurring dates, labels, descriptions, productivity visualizations, 120+ integrations, "filters in your own words", Todoist Comms (beta), MCP server. AI task capture (incl. Ramble) is limited to 10 sessions/month on Beginner and unlimited on paid plans. — [Todoist Pricing](https://todoist.com/pricing)
- Since 19 Nov 2026, all color themes are free on every plan (Web/Desktop), and all themes and app icons are free on Android. — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- TickTick Premium costs US$49.99/year ("less than US$4.17/month"). — [TickTick Upgrade](https://ticktick.com/about/upgrade); contradicted by older/third-party figures of $35.99/yr or $2.99–3.99/mo at [techharry 2026 review](https://www.techharry.com/2026/09/tick-tick-review.html). Use the official $49.99 figure.
- TickTick limits Free / Premium: lists 9 / 299; tasks and notes per list 99 / 999; attachments 1/day / 199/day; checklist items 19 / 199 per task; reminders per task 2 / 5; members per shared list 2 / 29; habits 5 / 299; countdowns 5 / 299. — [TickTick Upgrade](https://ticktick.com/about/upgrade). The page text was partly merged, so some values are inferred from the column order. The fetch of [ticktick.com/upgrade](https://ticktick.com/upgrade) read the same numbers.
- TickTick Free includes list and Kanban views, NLP, task reminders and cross-platform sync. Premium adds multiple calendar views, task duration, email reminders, AI features and themes. Calendar views, Eisenhower Matrix, widgets, themes and statistics are marked "Basic" on Free and "Unlimited" on Premium. — [TickTick Upgrade](https://ticktick.com/upgrade)
- TickTick has no team/family plan and no lifetime option. — [techharry](https://www.techharry.com/2026/09/tick-tick-review.html)

### Inferences
- Paid tiers cost roughly the same: TickTick Premium is about $10/yr cheaper than Todoist Pro. TickTick puts more into the free tier (Kanban, Eisenhower basic, habits, countdowns, Pomodoro). Todoist's strengths are integrations and teams.

### Gaps
- The official comparison table's tier markers for individual TickTick rows (Timeline, Pomodoro, Comments, Templates, Constant Reminder, Filter, MCP, AI Voice Capture, Recording Summary) could not be read. Whether Pomodoro and Timeline are free is disputed in reviews (see below).
- Regional or EUR pricing was not checked.

## Task basics (dates, reminders, recurrence, subtasks, attachments, priorities, labels, deadlines)

### Takeaway
Both apps cover the basics. Todoist has the richer documented recurrence grammar, and it separates "date" from "deadline" (deadline is Pro only). TickTick offers constant ("annoying") reminders, email reminders and checklist-item reminders. Its location reminders are iOS only.

### Cited Findings
- Todoist recurrence syntax (from the official help):
  - basic: `every day/weekday/workday/week/month/year`
  - ranges: `starting`, `until`, `for 3 weeks`, `from … until …`
  - hourly: `every hour`, `every 12 hours starting at 9pm`
  - multiple days: `every mon, fri at 20:00`, `every 2, 15, 27`
  - workday positions: `every last workday`, `every 15th workday, first workday`
  - ordinal weekdays: `every 1st wed jan, 3rd thu jul`
  - `every 3 workday`, `every quarter`
  - after completion: `every! 3 months`, `every! 3 hours`, `after 10 days` (becomes `every! 10 days`)
  - `every other day/week/fri`
  - holidays: `new year day`, `valentine`, `halloween`, `new year eve`

  — [Todoist: Introduction to recurring dates](https://www.todoist.com/help/articles/introduction-to-recurring-dates-YUYVJJAV)
- Todoist deadlines are a separate field `{march 30}` (Pro/Business). They have keyboard shortcuts `d` / `Shift+D` (5 Mar 2026). — [Todoist Quick Add](https://www.todoist.com/help/articles/use-task-quick-add-in-todoist-va4Lhpzz), [Todoist Pricing](https://todoist.com/pricing), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- Todoist task duration is Pro only. Since 1 Oct 2026 a duration works without a date (Quick Add "Write paper for 1h"). Android Quick Add parses ranges like "Meet tomorrow 9-10" (24 Sep 2026, paid). — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- Todoist reminders: basic task reminders on all plans, "custom reminders" Pro only. Location reminders are set on mobile and fire on arrival or departure. "Urgent reminders" previewed on iOS in Jan 2026 and reached Android on 20 Aug 2026. — [Todoist Pricing](https://todoist.com/pricing), [Todoist help search results / Karma & reminders](https://www.todoist.com/help/todoist/features/introduction-to-karma-OgWkWy), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd). One review says Todoist location reminders are Pro/Business only — [2sync](https://2sync.com/blog/ticktick-vs-todoist).
- Todoist: sub-tasks, descriptions, comments, labels and four priority levels p1–p4 are on all plans. Attachments go in comments; uploads are capped at 5 MB free and 100 MB paid. Markdown text formatting in tasks and comments came to iOS on 12 Nov 2026, and Markdown tables render on Android (24 Sep 2026). — [Todoist Pricing](https://todoist.com/pricing), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- TickTick reminders:
  - multiple alert times and end-time alerts
  - lock-screen pinning (iOS only)
  - Constant Reminder (keeps ringing until the task is done)
  - email reminders (Premium)
  - repeat weekly, monthly, yearly or custom
  - location reminders (iOS only)
  - a daily planning reminder

  — [TickTick Features](https://ticktick.com/features), [TickTick Upgrade](https://ticktick.com/upgrade)
- In mid-2026 TickTick extended constant reminders to tasks, habits and countdowns. — [WebSearch summary of TickTick changelog/help](https://help.ticktick.com/articles/7082552170989486080) (secondary summary; the page itself could not be read)
- TickTick tasks have priority, tags, subtasks, checklist items (19 per task free / 199 Premium), checklist-item reminders, comments, templates, notes with images and Markdown, and list/task activity history. — [TickTick Upgrade](https://ticktick.com/upgrade), [TickTick Features](https://ticktick.com/features)
- TickTick recurrence has trouble with patterns like "2nd Tuesday of the month" (anecdotal Reddit report, possibly outdated). — [Reddit r/ticktick (mirror)](https://axebps-redlib.hf.space/r/ticktick/comments/1im1vkt/smart_date_parsing_help). The techharry review also says its parsing is weaker than Todoist's — [techharry](https://www.techharry.com/2026/09/tick-tick-review.html)

### Inferences
- For Checko's own recurrence engine, Todoist's grammar is the best reference: `every!` for after-completion, workday ordinals, "every other", ranges.
- Neither official TickTick page documents a completion-based recurrence rule. From memory, TickTick has a "repeat from completion date" option, but this is unverified.

### Gaps
- An official TickTick list of recurrence options (after-completion, workday rules, Chinese lunar calendar, etc.) was not found.
- TickTick's per-task attachment size limit was not found.

## Natural-language Quick Add syntax

### Takeaway
Todoist: `#project /section %label` (the old `@` is being retired by the end of 2026), `p1–p3`, `!reminder`, `{deadline}`, `+assignee`, plus durations. TickTick: smart date parsing with `#tag`, `^list`, `!priority` and `@user`, documented mainly by third parties.

### Cited Findings
- Todoist Quick Add:
  - natural-language dates, including recurring ones ("every other Tuesday starting March 3")
  - `{date}` for a deadline
  - `#Project`, `/Section`
  - `%label`; `@label` still works but is "planned to be retired by end of 2026"
  - `p1`/`p2`/`p3`
  - `!14:00` or `!30 min before` for a reminder
  - `+Name` for an assignee (shared projects only)
  - description field, attachments in Quick Add, and multi-task capture from text, images or documents
  - smart date recognition can be toggled
  - dates are not supported in Czech or Turkish

  — [Todoist Quick Add help](https://www.todoist.com/help/articles/use-task-quick-add-in-todoist-va4Lhpzz)
- Todoist Quick Add redesign and multi-task capture for all (8 Oct 2026). "Extract tasks…" from text, images or documents on all platforms (22 Oct 2026). Draggable global Quick Add window (22 Oct 2026). — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- TickTick smart date parsing ("buy ticket tomorrow 8 am" sets the date and reminder) can be turned off, and the date text can be hidden from the title. — [TickTick blog 2.7.0](https://blog.ticktick.com/2016/04/12/ticktick-270-a-new-breakthrough-with-city/) (2016, old but the feature persists; NLP is listed on the [Free plan](https://ticktick.com/about/upgrade))
- TickTick quick-add tokens: `^listname`, `#tag`, `!priority`, `@user` (assign), with dates parsed or `*duedate`. — [curtismchale.ca (third-party, 2020)](https://curtismchale.ca/2020/08/10/ticktick-quick-add-syntax). Unverified officially and may be outdated.

### Inferences
- Todoist uses `#` for project, while TickTick uses `#` for tag. Checko's planned syntax (`#gesundheit !1`) follows the TickTick `#tag` convention, with `!` meaning priority as in TickTick (`!` is a reminder in Todoist).

### Gaps
- No official TickTick help article on quick-add tokens was found.
- German-language date-word support for either app was not verified.

## Organization (projects, folders, sections, filters, views, sorting, completed, search)

### Takeaway
Todoist has projects, sections, sub-projects, folders (Business), labels, query filters ("filters in your own words"), and list/board/calendar/table layouts (calendar and table are Pro). TickTick has lists, folders, tags and smart filters, plus list, Kanban, timeline and calendar views (day/week/month/year/multi-day/multi-week/agenda).

### Cited Findings
- Todoist layouts: list (all plans); board (personal projects on all plans, label/filter views on Pro, team projects on Business); calendar and table (Pro in personal/label/filter views, Business in team projects). Table layout came to Android on 19 Nov 2026. — [Todoist Pricing](https://todoist.com/pricing), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- Other 2026 Todoist organization changes (all from the [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)):
  - drag to reorder or nest projects (5 Nov)
  - project and section descriptions (9 Jul)
  - filters in the calendar Plan sidebar (10 Sep)
  - completed recurring occurrences shown in date views (10 Sep)
  - reorder tasks in Upcoming while a filter is on (30 Oct)
  - uncomplete multiple completed tasks (iOS, 30 Oct)
  - Quick Find by task ID or link (15 Oct)
  - comment search grouped by project (19 Nov)
  - collapse/expand view shortcuts (1 Oct)
- Todoist folders, roles and team workspace are Business only. — [Todoist Pricing](https://todoist.com/pricing)
- TickTick: lists, smart filters and tags; three-column list view; Kanban with custom grouping; Timeline as a "lighter alternative to Gantt" with drag-adjustable durations; calendar views yearly/monthly/weekly/agenda/multi-day/multi-week. — [TickTick Features](https://ticktick.com/features)
- TickTick 8.0 (Jan 2026) brought a redesign, Suggested Tasks, a Year View with heatmap, a "Modern" calendar style and more Month View options. Search filters were added mid-2026. — [techharry](https://www.techharry.com/2026/09/tick-tick-review.html), [WebSearch summary of TickTick "What's New"](https://help.ticktick.com/articles/7082552170989486080)
- Timeline and calendar views are Premium only. — [TechRepublic review](https://www.techrepublic.com/de/article/ticktick-review/) (via search snippet). The official page lists "multiple calendar views" under Premium and "Calendar views: Basic" under Free.

### Inferences
- In TickTick, Free probably gets a limited calendar while Premium unlocks the full set of views (multi-day, multi-week, timeline).

### Gaps
- Exact Todoist sort/group options and TickTick folder ("list group") limits were not documented in the sources read.
- The Todoist "Logbook"/completed-view details were not fetched.

## Planning (Today/Upcoming, calendar sync, time blocking, Eisenhower, Pomodoro, habits, countdowns, reviews, stats, AI)

### Takeaway
TickTick bundles planning tools that Todoist lacks: Eisenhower Matrix, Pomodoro/focus timer with white noise, habit tracker, countdowns and a statistics dashboard. Todoist has Today/Upcoming, true two-way Google/Outlook task sync, calendar time-blocking (Pro), Karma and productivity goals, and a broad AI suite (Ramble voice, Task/Email/Filter Assist, Task Capture, Automations with AI steps).

### Cited Findings
- Todoist calendar integration (Google or Outlook):
  - accounts: 1 on Beginner/Pro Legacy, 3 on Pro/Business
  - iCloud only via subscription in Google or Outlook
  - events appear read-only in Today and Upcoming
  - "Sync tasks to calendar" creates a "Todoist" calendar
  - two-way for name/date/time/duration/recurrence; completion adds a checkmark
  - completion-based recurrences and single-instance edits don't sync
  - the calendar layout for time-blocking is Pro/Business

  — [Todoist: Use the Calendar integration](https://www.todoist.com/help/articles/use-the-calendar-integration-rCqwLCt3G)
- 2026 calendar-layout improvements: drag to pick a time (Android, 15 Oct), drag a new task block (iOS, 17 Sep), color tasks by priority/project/label (20 Aug), edge auto-paging while dragging (19 Nov). — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- TickTick can subscribe to external calendars and integrate with apps like Notion. — [TickTick Features](https://ticktick.com/features). Its Notion integration with two-way sync arrived in Feb 2025 — [2sync](https://2sync.com/blog/ticktick-vs-todoist).
- Whether TickTick syncs Google/Outlook calendars two-way is disputed. The App Store says changes are reflected in both apps. Reviews variously say Google, iCloud, Outlook and CalDAV are supported, or that sync is read-only. — [temporal.day](https://temporal.day/blog/ticktick-vs-todoist-2026), [App Store listing](https://apps.apple.com/app/ticktick-todo-task-list/id626144601) (via search snippets; unverified)
- TickTick: Eisenhower Matrix (Basic on Free / Unlimited on Premium), Pomodoro focus timer, Habit Tracker with logs and stats (5 habits free / 299 Premium), Countdown (5 / 299), Statistics (tasks completed, focus duration, habit logs; Basic / Unlimited). — [TickTick Features](https://ticktick.com/features), [TickTick Upgrade](https://ticktick.com/upgrade)
- Whether Pomodoro is on TickTick Free is disputed: some reviews say it is included, techharry says it is Premium only. — [techharry](https://www.techharry.com/2026/09/tick-tick-review.html); see the search summary listing [nathanojaokomo](https://nathanojaokomo.com/blog/ticktick-review) and [capterra](https://www.capterra.com/p/170641/TickTick/)
- TickTick's focus timer logs distractions and has white noise. — [techharry](https://www.techharry.com/2026/09/tick-tick-review.html)
- Todoist has no habit tracker, Pomodoro or Eisenhower Matrix. — [2sync](https://2sync.com/blog/ticktick-vs-todoist) (third-party, consistent with the official pricing page, which lists none)
- Todoist productivity: daily and weekly goals (defaults 5 and 25), streaks, vacation mode, days off, Karma. Productivity visualizations are on all plans. — [Todoist Karma help](https://www.todoist.com/help/todoist/features/introduction-to-karma-OgWkWy), [Productivity view](https://www.todoist.com/help/todoist/features/use-the-productivity-view-in-todoist-6S63uAa9), [Todoist Pricing](https://todoist.com/pricing)
- Todoist AI in 2026 (dates from the [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd) unless noted):
  - Ramble voice-to-task (LLM-based, part of "Todoist Assist") went GA on 21–22 Jan 2026 — [TechCrunch](https://techcrunch.com/2026/01/21/todoists-app-now-lets-you-add-tasks-to-your-to-do-list-by-speaking-to-its-ai/)
  - Ramble later gained sections and view context (6 Feb), Wear OS beta (19 Mar), assigning to members (16 Apr) and widget buttons (Sep)
  - Email Assist on paid plans (29 Jan)
  - Task Capture from notes, photos and files (beta Apr; Assist out of beta 22 Oct)
  - Automations (beta, Pro) with AI steps — [Todoist Pricing](https://todoist.com/pricing)
  - the Assist suite also includes Task Assist (breaks work into subtasks) and Filter Assist — [2sync](https://2sync.com/blog/ticktick-vs-todoist)
  - Ramble works in 38 languages — [2sync](https://2sync.com/blog/ticktick-vs-todoist)
- TickTick AI in 2026: MCP support for ChatGPT, Claude etc. (spring 2026); AI Mode / AI Voice Capture that splits spoken input into multiple tasks; Recording Summary (transcription plus summary); AI Assistant that creates tasks, organizes lists and breaks down projects (8 Oct 2026); Siri AI on iOS 27. Per the techharry review these are Premium. — [techharry](https://www.techharry.com/2026/09/tick-tick-review.html), [WebSearch summary of TickTick changelog](https://ticktick.com/public/changelog/en.html), [TickTick Upgrade](https://ticktick.com/upgrade)
- TickTick 8.0 "Suggested Tasks" helps review unfinished tasks and pick today's focus, which acts as a daily review. — [WebSearch summary](https://help.ticktick.com/articles/7082552170989486080)

### Inferences
- In Todoist, the Today summary ("5 tasks · 2h 30m", iOS 26+) together with goals and streaks amounts to a light daily review. Neither app documents a formal "weekly review" workflow in the sources read.

### Gaps
- TickTick's two-way Google/Outlook sync, and whether it is Premium, is not confirmed from an official page.
- Whether a dedicated Todoist weekly review exists was not found.

## Collaboration

### Takeaway
Todoist is stronger for teams: shared projects, assignees, comments, a Business workspace with roles, folders and shared templates, team activity reporting, People tab, Insights, and Todoist Comms (beta chat). TickTick allows shared lists (2 members free / 29 Premium), assignment, comments and an activity log. It has no team plan.

### Cited Findings
- Todoist: comments and assigned tasks on all plans; 5 collaborators per personal project; teams up to 1,000 people (5 guests on non-Business). Business adds roles, folders, centralized billing, shared templates and team activity reporting. Comms message history is 1 month (full on Business). — [Todoist Pricing](https://todoist.com/pricing)
- Todoist 2026: People tab, Insights and reworked Reporting (23 Apr); Reporting gained 15 more event types (30 Apr); Project Insights extended to Pro (16 Jul); "Last edited by" on shared templates (15 Oct); team templates gallery (1 Oct). — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd), [Todoist Apr 23 update](https://www.todoist.com/help/todoist/product-updates/a-clearer-picture-of-your-teams-work-apr-23-2026-tUh7Nsstf)
- TickTick: shared lists, task assignment, comments, list/task activities; members 2 free / 29 Premium. — [TickTick Features](https://ticktick.com/features), [TickTick Upgrade](https://ticktick.com/upgrade)

### Inferences
- For a personal organizer like Checko, TickTick-level collaboration (shared lists, assign, comment) is the relevant bar.

### Gaps
- TickTick activity-log retention period: not found.

## Platforms, integrations and extras (widgets, wearables, extensions, email-to-task, templates, offline, themes, shortcuts, import/export, undo, trash)

### Takeaway
Both run on iOS, Android, macOS, Windows, Linux and Web, with browser extensions and offline support. Todoist has 120–150+ integrations, Wear OS and Apple Watch, email forwarding (Pro), auto backups, CSV import/export and an MCP server. TickTick has fewer integrations (Notion, Google Calendar, Telegram, Gmail/Outlook add-ons), 40+ themes, list backgrounds and MCP.

### Cited Findings
- Platform availability (from [2sync](https://2sync.com/blog/ticktick-vs-todoist)):
  - Todoist: iOS, Android, macOS, Windows, Linux, Web, Apple Watch, Wear OS; Chrome, Firefox, Safari and Edge extensions
  - TickTick: iOS, Android, macOS, Windows, Linux, Web, Apple Watch; Chrome, Firefox and Edge extensions; Gmail and Outlook add-ons
  - both work offline
- TickTick Wear OS support is mentioned by [techharry](https://www.techharry.com/2026/09/tick-tick-review.html).
- Todoist integrations: "120+ integrations" (official). — [Todoist Pricing](https://todoist.com/pricing); "150+" — [2sync](https://2sync.com/blog/ticktick-vs-todoist)
- Todoist email to task: Pro and Business. Forwarding creates a task with the email and attachments in its comments (Oct 2026 update); email forwarding on iOS and Android (29 Jan 2026). — [Todoist Pricing](https://todoist.com/pricing), [Todoist Oct 2026 update](https://www.todoist.com/help/todoist/product-updates/october-2026-capture-tasks-wherever-they-find-you-reUlb5uSI), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- Todoist auto backups (Pro). "Manage data" was renamed "Import & export" (30 Oct 2026). CSV import with confirmation (8 Oct 2026). — [Todoist Pricing](https://todoist.com/pricing), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- Todoist keyboard: Vim-style h/j/k/l for keyboard dragging, a keyboard-friendly emoji picker, copy-view-link and collapse shortcuts, and a global Quick Add shortcut on the macOS/Windows desktop apps. — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd), [Todoist Quick Add](https://www.todoist.com/help/articles/use-task-quick-add-in-todoist-va4Lhpzz)
- Todoist widgets and wearables: iOS widgets with a Ramble button (Sep 2026); Wear OS Day Progress widget (15 Oct 2026) and a Wear OS project picker with team projects; iOS App Shortcuts and Siri on-screen awareness. — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- Todoist templates: shared templates are Business; there is a team templates gallery page. — [Todoist Pricing](https://todoist.com/pricing), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- TickTick capture and integrations:
  - quick capture by email plugin, right-click in the browser and a global desktop shortcut
  - home-screen widget, voice input, Google Calendar add-on, Notion integration
  - Telegram (create a task from a message, mid-2026)
  - MCP servers

  — [TickTick Features](https://ticktick.com/features), [TickTick Upgrade](https://ticktick.com/upgrade), [WebSearch summary](https://help.ticktick.com/articles/7082552170989486080)
- TickTick personalization: 40+ themes and per-list backgrounds. Themes are "Basic" on Free and full on Premium. — [TickTick Features](https://ticktick.com/features), [TickTick Upgrade](https://ticktick.com/upgrade)
- TickTick keyboard shortcuts and command menus. — [TickTick Features](https://ticktick.com/features)

### Inferences
- Todoist's free themes from Nov 2026 remove a former Pro perk. TickTick still gates themes.

### Gaps
- Undo and trash behavior (retention period, restoring deleted tasks) for both apps was not verified. From memory, both have undo toasts and TickTick has a Trash, but this is unsourced.
- TickTick import/export (CSV backup, Todoist import) was not verified from official sources.
- The TickTick Apple Watch complication and widget feature set was not detailed.

## Gamification

### Takeaway
Todoist: Karma points and levels from Beginner to Enlightened, daily and weekly goals, streaks, vacation mode. TickTick: statistics with an "achievement score", streaks in the habit tracker, and focus stats. Its gamification is less formalized than Karma.

### Cited Findings
- Todoist Karma levels: Beginner (0–499), Novice, Intermediate, Professional, Expert, Master, Grand Master, Enlightened (50,000+). Points are earned by completing tasks and using features such as reminders. — [Todoist: Introduction to Karma](https://www.todoist.com/help/todoist/features/introduction-to-karma-OgWkWy), [Todoist Karma](https://www.todoist.com/karma)
- Todoist goals (5 daily / 25 weekly by default), streaks (daily and weekly; current, last and max exposed in the API), vacation mode and days off. — [Productivity view](https://www.todoist.com/help/todoist/features/use-the-productivity-view-in-todoist-6S63uAa9), [Todoist API](https://developer.todoist.com/api/v1/)
- A refreshed setup card shows Karma points (19 Nov 2026), and the productivity profile shows the join date (Oct 2026). — [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd)
- TickTick has "Achievement scores tied to the Pomodoro timer" (third-party). — [2sync](https://2sync.com/blog/ticktick-vs-todoist). The habit tracker has streaks and reminders — [2sync](https://2sync.com/blog/ticktick-vs-todoist). Statistics cover tasks completed, focus duration and habit logs — [TickTick Features](https://ticktick.com/features)

### Inferences
- Karma (points, levels, goals, streaks, vacation mode) is the closer template for Checko's planned XP, level and streak system.

### Gaps
- No official TickTick documentation on how the achievement score is calculated or what levels it has was found.
- Whether Todoist Karma is fully available on Beginner is not explicit, though "productivity visualizations" are listed on all plans.

## Notable features added 2024–2026

### Takeaway
Todoist's 2026 work centered on AI capture, team insights and calendar or layout polish. TickTick's centered on the 8.0 redesign, AI and MCP.

### Cited Findings
- Todoist 2026 (from the [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd) unless noted):
  - Ramble GA (Jan) — [TechCrunch](https://techcrunch.com/2026/01/21/todoists-app-now-lets-you-add-tasks-to-your-to-do-list-by-speaking-to-its-ai/)
  - Email Assist (Jan)
  - Task Capture from images and documents (Apr–Oct)
  - People tab, Insights and Reporting (Apr)
  - Android Material 3 Expressive redesign (Jun)
  - project and section descriptions (Jul)
  - Quick Add redesign (Aug/Oct)
  - urgent reminders (Jan iOS preview, Aug Android)
  - durations without dates (Oct)
  - sub-tasks on boards (Oct)
  - table layout on Android and all themes free (Nov)
  - `%` replacing `@` for labels by end of 2026 — [Quick Add help](https://www.todoist.com/help/articles/use-task-quick-add-in-todoist-va4Lhpzz)
  - Automations (beta), Comms (beta) and MCP server on the pricing page — [Todoist Pricing](https://todoist.com/pricing)
  - continuous calendar scrolling (Mar 2026) — [2sync](https://2sync.com/blog/ticktick-vs-todoist)
- TickTick:
  - Weekly View major update (Oct 2024) — [AlternativeTo](https://alternativeto.net/news/2024/10/ticktick-unveils-major-update-with-new-weekly-view)
  - Notion two-way integration (Feb 2025) — [2sync](https://2sync.com/blog/ticktick-vs-todoist)
  - v8.0 redesign with Suggested Tasks, Year View and Modern calendar (Jan 2026)
  - MCP and AI Mode voice input (spring 2026)
  - search filters, Telegram integration, constant reminders for habits and countdowns (mid-2026)
  - editor hides formatting marks (late 2026)
  - AI Assistant and iOS 27 Siri AI (8 Oct 2026)

  — [techharry](https://www.techharry.com/2026/09/tick-tick-review.html), [WebSearch summary of TickTick changelog/help](https://ticktick.com/public/changelog/en.html). Exact TickTick dates are unverified because the official changelog page did not render for the fetcher.

### Inferences
- Both apps are converging on AI capture (voice, documents) and MCP access for external AI agents. Checko's planned Phase 8 (Claude API) fits this trend.

### Gaps
- The TickTick official changelog (JS-rendered) could not be read directly, so TickTick 2024–2026 dates rely on secondary summaries.
- The Todoist 2024–2025 changelog (e.g. deadlines launch, Assist launch, workspace launch dates) was not fetched. Only 2026 was read.

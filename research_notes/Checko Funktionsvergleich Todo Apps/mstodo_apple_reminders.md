# Microsoft To Do and Apple Reminders: feature inventory (as of Nov 2026)

Scope: dense has/missing inventory for a comparison table. Status date: 2026-11-26. Items not backed by a fetched source are listed under "Gaps" (sometimes with an unverified hint from background knowledge, clearly marked).

**Headline news (important for the table):** Microsoft announced on 18 Nov 2026 that the **To Do apps will retire on June 30, 2027**. Work and school users go to Planner. Personal accounts, including Microsoft 365 Personal and Family subscribers, get **no replacement**. See question 6.

---

## 1. Task basics (due date/time, reminders, recurrence, subtasks, notes, attachments, priority, tags)

### Takeaway
Apple Reminders has the richer task model: date plus time, early reminders, location and "When Messaging" triggers, flag and 3-level priority, tags, images and document scans, URL, per-reminder time zone, and since iOS 26.2 "Urgent" alarm reminders. Microsoft To Do has due date, a separate "Remind me" date/time, recurrence (daily, weekdays, weekly, monthly, yearly, custom), steps (subtasks), notes, file attachments and an Important star. Its docs show no location reminders and no priority levels beyond Important.

### Cited Findings
**Apple Reminders**
- Date and time per reminder, set through the Date/Time buttons. A reminder with a due date also appears as an all-day event in Calendar. — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- Repeat: turn on Date or Time, tap Repeat, choose a frequency. — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- Custom repeat: monthly by date or by position (first/second/third... plus weekday), and "first weekend day of month" via Monthly drop-downs. — [MacMost](https://macmost.com/creating-regular-repeating-reminders.html), [Mac Observer](https://www.macobserver.com/tips/macos-setting-custom-reminders/). An older iOS 15-era report says an Outlook "first weekday of month" recurrence imported wrongly as "first Monday". — [Apple Community](https://discussions.apple.com/thread/253422474)
- Early Reminder, for example "5 minutes before" or "2 days before". — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- Location reminders: arriving at or leaving a place, or getting into a car (Bluetooth). Requires Precise Location. — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- "When Messaging": the reminder pops up the next time you chat with a chosen contact in Messages. — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- Flag, and Priority with Low/Medium/High. — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- Notes, URL, and images (take a photo, use the library, scan a document, scan text). — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- Tags (#hashtags) work across lists and can be filtered in Smart Lists, including combining or excluding tags. — [9to5Mac power-user features (Feb 2025)](https://9to5mac.com/2025/02/14/apples-reminders-app-seven-power-user-features/)
- **Urgent** (iOS 26.2): a reminder with a due time triggers an alarm with 9-minute snooze plus a Live Activity. If no date/time is set, it defaults to the next hour. — [9to5Mac (Feb 2026)](https://9to5mac.com/2026/02/27/ios-26-4-gives-reminders-best-new-feature-the-one-thing-it-was-missing/), [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- **Time zones** (iOS 26): a time zone per reminder below the due time, plus an app-wide "Set Manually" display time zone under Settings > Apps > Reminders. — [9to5Mac (Dec 2025)](https://9to5mac.com/2025/12/22/ios-26s-reminders-app-got-a-new-feature-inspired-by-calendar/)

**Microsoft To Do**
- Due date: Today, Tomorrow, Next week, or a picked date. — [MS Support: Due dates and reminders](https://support.microsoft.com/en-us/office/add-due-dates-and-reminders-in-microsoft-to-do-064d9696-08d1-4433-bfdd-f661dc97491f)
- "Remind me": Later today, Tomorrow, Next week, or a picked date **and time**. The reminder carries the time; the docs mention no due time. — [MS Support](https://support.microsoft.com/en-us/office/add-due-dates-and-reminders-in-microsoft-to-do-064d9696-08d1-4433-bfdd-f661dc97491f)
- Repeat: Daily, Weekdays, Weekly, Monthly, Yearly, Custom. It needs a due date first. — [MS Support](https://support.microsoft.com/en-us/office/add-due-dates-and-reminders-in-microsoft-to-do-064d9696-08d1-4433-bfdd-f661dc97491f), [MS Support: Outlook To Do](https://support.microsoft.com/en-us/outlook/calendar/manage-tasks-with-to-do-in-outlook)
- A user in a Tech Community thread could not create a "last weekday of month" custom repeat in To Do. — [MS Tech Community](https://techcommunity.microsoft.com/t5/microsoft-to-do/can-t-create-a-custom-repeat-action-every-last-day-of-weekdays/m-p/3443140)
- Steps (subtasks) and files in the task detail view. Several files per task, added one at a time. — [MS Support: Manage tasks with To Do in Outlook](https://support.microsoft.com/en-us/outlook/calendar/manage-tasks-with-to-do-in-outlook)
- Search covers tasks, steps, **tags** and notes, so tags exist in To Do. — [MS Support: Sort and search](https://support.microsoft.com/en-us/office/sort-and-search-in-lists-133fa637-3f47-4633-8d78-7289059fa630)
- Sort by "Importance" (the Important star). — [MS Support: Sort and search](https://support.microsoft.com/en-us/office/sort-and-search-in-lists-133fa637-3f47-4633-8d78-7289059fa630)

### Inferences
- MS To Do priority is binary (Important yes/no), while Apple has Flag plus None/Low/Medium/High.
- Location-based reminders: Apple yes, MS To Do apparently no (not mentioned on the official reminder page).
- Both have custom recurrence. Apple's "nth weekday of month" is documented by third parties. MS custom options (every N days/weeks, selected weekdays) are likely but not confirmed by a fetched source.

### Gaps
- MS To Do custom repeat details (every N, specific weekdays, nth weekday) are not described on the support page. Unverified hint: the UI offers "every N day/week/month/year" plus weekday selection for weekly.
- Whether MS To Do supports a due *time* separate from the reminder: the docs say no.
- MS To Do tag syntax (#tag in the title) is not verified beyond the search page.
- Apple: subtasks (indent, or drag onto another reminder) are not covered by a fetched page. Background knowledge says they exist since iOS 13 and are one level deep. Treat as likely, but unverified here.

---

## 2. Organization (lists, groups, sections, smart lists, filters, sorting, ordering, completed, search)

### Takeaway
Apple has lists, list groups, sections, column (Kanban) view, templates, 8 built-in Smart Lists, and custom Smart Lists with any/all filters. MS To Do has lists, list groups, fixed smart lists (My Day, Important, Planned, Assigned to me, Flagged email, Tasks), 5 sort options and global search, but no user-defined smart filters.

### Cited Findings
- Apple built-in Smart Lists (each can be toggled): Today (incl. overdue), Scheduled, All, Flagged, Completed, Assigned to Me, Groceries, Siri Suggestions (from Mail). — [Apple Support: Use Smart Lists](https://support.apple.com/guide/iphone/use-smart-lists-iphe882772ed/ios)
- Apple iOS 26.4 adds an "Urgent" Smart List. — [9to5Mac](https://9to5mac.com/2026/02/27/ios-26-4-gives-reminders-best-new-feature-the-one-thing-it-was-missing/)
- Apple custom Smart Lists filter by tags (several, include or exclude), date, time, location, flag, priority and lists, with an any/all match. — [Apple Support](https://support.apple.com/guide/iphone/use-smart-lists-iphe882772ed/ios), [9to5Mac](https://9to5mac.com/2025/02/14/apples-reminders-app-seven-power-user-features/)
- Apple sections, drag-and-drop between sections, "View as Columns" (side by side, Trello-like), and templates ("Save as Template"). — [9to5Mac](https://9to5mac.com/2025/02/14/apples-reminders-app-seven-power-user-features/)
- Apple support guide has topics "Organize multiple lists" (list groups), "Search your lists", "Delete and recover reminders" and "Print a list". — [Apple iPhone User Guide TOC](https://support.apple.com/guide/iphone/set-up-reminders-iph5e10c6d82/ios)
- MS To Do sorting: Importance, Due date, Added to My Day, Alphabetically, Creation date. Search covers tasks, steps, tags and notes. — [MS Support: Sort and search](https://support.microsoft.com/en-us/office/sort-and-search-in-lists-133fa637-3f47-4633-8d78-7289059fa630)
- MS To Do has list groups, which Planner has no equivalent for. — [office365itpros (Nov 2026)](https://office365itpros.com/2026/11/20/to-do-apps-retirement/)
- MS To Do "Assigned to me" list (Planner) and Flagged email integration. — [MS Support: Planner tasks in To Do](https://support.microsoft.com/en-us/office/see-planner-tasks-in-microsoft-to-do-4cac0115-4e4c-44f7-80ae-d1e25ec0527e), [MS Planner blog](https://techcommunity.microsoft.com/t5/Microsoft-To-Do-Blog/Planner-and-To-Do-integration-bringing-you-a-more-cohesive-task/ba-p/552740)

### Inferences
- Custom smart lists/filters: Apple yes, MS To Do no (only fixed smart lists).
- Sections/columns/templates: Apple yes. MS To Do has no documented equivalent.
- Undo/trash: Apple has "Recently Deleted" recovery (support topic "Delete and recover reminders"). No equivalent was confirmed for MS To Do.

### Gaps
- MS To Do sections and grouping are not documented. Unverified hint: none exist. Its "Planned" view groups by Earlier/Today/Tomorrow/Later.
- Completed handling: MS has "show/hide completed" (unverified). Apple has a Completed Smart List and a show/hide option (Completed list cited).
- Manual drag-and-drop ordering: both very likely support it, but no fetched source states it explicitly. MS My Day says "arrange tasks in the order you want" (cited in Q3).

---

## 3. Planning (My Day, suggestions, calendar, natural language, AI)

### Takeaway
MS To Do's distinctive planning feature is **My Day** with Suggestions and a nightly reset, plus smart date recognition while typing. Apple relies on Today/Scheduled, calendar display of dated reminders, and natural-language entry through Siri. In iOS 26, Apple Intelligence adds Suggested Reminders and auto-categorized sections.

### Cited Findings
- My Day: a focus list for today. It clears every night, and unfinished tasks return to their source list. Tasks can be ordered manually. — [MS Support: Plan your day with My Day](https://support.microsoft.com/en-us/office/plan-your-day-with-my-day-4cafa6fe-8891-4ad5-985f-65912146a73e)
- My Day Suggestions: tasks due today or later, overdue tasks, and tasks from earlier My Days that weren't completed. Add them with "+ Add to My Day". Daily repeating tasks show up as suggestions each morning. — [MS Support: Outlook To Do](https://support.microsoft.com/en-us/outlook/calendar/manage-tasks-with-to-do-in-outlook), [MS Support: Daily habits](https://support.microsoft.com/en-us/office/creating-daily-habits-with-microsoft-to-do-cf2a37ea-8f7b-450c-a401-b131365728b1)
- MS smart due date recognition: type "tomorrow", "Friday", "tomorrow 5pm" or "every Monday" in the title, and it is highlighted and applied. Available on Windows 2.66+ (English only) and iOS. It doesn't work in the details view. — [MS Support: Smart due date](https://support.microsoft.com/en-us/office/smart-due-date-reminder-recognition-in-microsoft-to-do-b8cde14b-28da-42e1-9ce8-d75cc3717993)
- Apple: dated reminders appear as all-day events in Calendar. — [Apple Support: Add details](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)
- Apple Intelligence (iOS 26): **Suggested Reminders** from emails, websites or shared content, and **Auto-categorize** a list into sections (needs consent). — [9to5Mac (Oct 2025)](https://9to5mac.com/2025/10/13/heres-everything-new-in-reminders-with-ios-26/). Requires Apple Intelligence hardware, iPhone 15 Pro or later. — [9to5Mac search summary / Neowin](https://www.neowin.net/news/exploring-ios-26-5-new-features-you-can-find-in-the-reminders-app/)
- Apple "Siri Suggestions" Smart List with items detected in Mail. — [Apple Support](https://support.apple.com/guide/iphone/use-smart-lists-iphe882772ed/ios)
- iOS 26 Phone app: "Call Back Reminder" from recent calls (1 hour, later today, tomorrow, custom). — [9to5Mac](https://9to5mac.com/2025/10/13/heres-everything-new-in-reminders-with-ios-26/)
- MS direction: tasks are being unified into Planner and Copilot ("unified tasks and plans for Copilot and Microsoft 365"). Tasks can be connected to Copilot and agents. — [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/), [The Next Web](https://thenextweb.com/news/microsoft-to-do-retiring-planner-personal-accounts-wunderlist)

### Inferences
- An explicit daily-plan ritual (My Day): MS yes, Apple no (only the automatic Today list).
- MS To Do itself got no Copilot AI features; the AI investment goes into Planner.

### Gaps
- Apple in-app natural-language entry: confirmed for iOS 27 ("describe" a reminder; see question 6). Pre-iOS 27 typing-time date suggestions are not verified.
- No calendar grid view in either app (inference from the absence of documentation, unverified).

---

## 4. Collaboration

### Takeaway
Both have free shared lists with assignment. Apple adds per-list notifications (items added/completed) and Messages activity. Neither has comments.

### Cited Findings
- Apple: share a list with iCloud users. Participants can add, edit and complete items. "Allow others to invite" can be turned off. — [Apple Support: Share and collaborate](https://support.apple.com/guide/iphone/share-and-collaborate-iph2a8f9121e/ios)
- Apple: assign with @name or the Person button, which feeds the "Assigned to Me" Smart List. Notify When covers Adding Items and Completing Items. Sharing via Messages shows activity in the chat. Requires an Apple Account and updated iCloud Reminders; sharing works only with iCloud users. — [Apple Support](https://support.apple.com/guide/iphone/share-and-collaborate-iph2a8f9121e/ios)
- MS To Do: list sharing since June 2018. — [Wikipedia](https://en.wikipedia.org/wiki/Microsoft_To_Do)
- MS To Do: "Assign to" a member of a shared list. Moving a task to another list removes the assignment. — [MS Support: Assign tasks in shared lists](https://support.microsoft.com/en-us/todo/assign-tasks-in-shared-lists)
- MS: shared-list membership and assignments won't transfer to Planner at retirement. — [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/)

### Gaps
- Comments: no source found for either app. Most likely neither has comments.
- MS cross-account sharing limits (personal vs work account) are not verified.

---

## 5. Platforms, integrations, extras

### Takeaway
MS To Do runs on Windows, macOS, iOS, Android and the web, and integrates with Outlook (flagged email, Tasks), Planner and Teams. Apple Reminders runs on iPhone, iPad, Mac, Apple Watch and Vision, with only a reduced iCloud.com web app (also usable on Windows/Android in a browser). Apple has deep OS integration: Siri, widgets, Lock Screen, Control Center, Action Button, Calendar, Messages, Phone and Mail.

### Cited Findings
- MS To Do platforms: Windows 10+, macOS Ventura+, Android 8+, iOS 17+, and web. — [Wikipedia](https://en.wikipedia.org/wiki/Microsoft_To_Do), [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/)
- MS To Do is free (freeware). — [Wikipedia](https://en.wikipedia.org/wiki/Microsoft_To_Do)
- MS: a flagged Outlook email becomes a To Do task. Planner tasks appear in "Assigned to me". To Do is also used inside Outlook (My Day pane, Tasks). Teams Tasks app "My tasks" lacks My Day and Flagged email. — [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/), [MS Support: Planner in To Do](https://support.microsoft.com/en-us/office/see-planner-tasks-in-microsoft-to-do-4cac0115-4e4c-44f7-80ae-d1e25ec0527e), [MS Support: Teams Tasks](https://support.microsoft.com/en-us/office/use-the%20tasks-app%20in%20teams-e32639f3-2e07-4b62-9a8c-fd706c12c070)
- MS: To Do Graph API exists. Offline task management is available in the native desktop apps (to be lost). — [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/)
- MS: a screen-reader guide exists for To Do (accessibility support). — [MS Support: screen reader](https://support.microsoft.com/en-us/accessibility/todo/use-a-screen-reader-to-work-with-tasks-in-to-do)
- Apple iOS 26: a "New Reminder" control for Control Center, Lock Screen and Action Button (iPhone 15 Pro+) opens a compact popup to set list, note, date, location and flag. Plus the Liquid Glass redesign. — [9to5Mac](https://9to5mac.com/2025/10/13/heres-everything-new-in-reminders-with-ios-26/)
- Apple: Siri can create location reminders. — [9to5Mac](https://9to5mac.com/2025/02/14/apples-reminders-app-seven-power-user-features/)
- Apple Groceries list type: auto-sorts items into aisle categories. It isn't always accurate, but it learns manual corrections. — [The Kitchn](https://www.thekitchn.com/grocery-shopping-reminders-app-review-23768909), [Apple Support TOC "Make a grocery list"](https://support.apple.com/guide/iphone/make-a-grocery-list-iph80ba26e1f/ios)
- Apple: Print a list, and other accounts (for example Exchange or IMAP, with reduced features). — [Apple Support TOC](https://support.apple.com/guide/iphone/set-up-reminders-iph5e10c6d82/ios), [Apple Support: Share](https://support.apple.com/guide/iphone/share-and-collaborate-iph2a8f9121e/ios)
- Apple iCloud.com web gaps (2024 user reports, possibly outdated): you can't group or move lists, and can't add URL, location, tags or images. — [Apple Community](https://discussions.apple.com/thread/255706034)
- Apple Reminders is free with an Apple device, and sharing is free. — [9to5Mac](https://9to5mac.com/2025/02/14/apples-reminders-app-seven-power-user-features/)

### Inferences
- Native Android/Windows apps: MS yes, Apple no (web only).
- Email to task: MS yes (Outlook flag). Apple only has Mail-detected "Siri Suggestions" and Share Sheet (Apple Intelligence Suggested Reminders).

### Gaps (not verified in this session; background knowledge only, mark as uncertain)
- MS To Do themes/backgrounds per list: long-standing, but not verified here.
- MS To Do Apple Watch app: believed removed around 2020. Wear OS: none. Unverified.
- MS To Do widgets (Android/iOS/Windows): believed to exist, unverified.
- MS printing (via the web/Windows app print list) and keyboard shortcuts: unverified.
- Import: MS had a Wunderlist/Todoist importer (Wunderlist ended May 2020, cited by [Wikipedia](https://en.wikipedia.org/wiki/Microsoft_To_Do)). Export is "promised before retirement", which implies no general export today. — [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/)
- Apple export: no official export (only print or share). Unverified.
- Apple Watch Reminders app and widgets: certain from background knowledge, but no fetched source this session.

---

## 6. Notable changes 2024–2026

### Takeaway
Apple keeps expanding Reminders (iOS 26 Liquid Glass, AI suggestions, auto-categorize, time zones, Urgent alarms, Urgent Smart List). Microsoft is **ending To Do apps on 30 June 2027** with no consumer replacement.

### Cited Findings
- 18 Nov 2026: Microsoft blog "Introducing unified tasks and plans for Copilot and Microsoft 365" says it "will phase out To Do. The To Do apps will retire on June 30, 2027." The service, Graph API and sync retire later (dates TBA). — [office365itpros (20 Nov 2026)](https://office365itpros.com/2026/11/20/to-do-apps-retirement/), [The Next Web](https://thenextweb.com/news/microsoft-to-do-retiring-planner-personal-accounts-wunderlist), [Topedia](https://blog.topedia.com/posts/microsoft-to-do-will-be-retired-as-planner-consolidates-into-unified-plans)
- Work/school accounts move to a private Planner plan. Existing tasks aren't migrated, only viewable in Planner until 30 Jun 2027. Personal accounts get no replacement. No native Windows or Mac Planner apps are planned. Planner lacks list groups and attachments in the private plan, and its notifications aren't a full replacement for reminders. Classic Outlook Tasks continue. — [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/)
- An earlier conflicting report (around Aug 2026) said "To Do isn't retiring", but it is superseded. — [WindowsForum](https://windowsforum.com/news/microsoft-to-do-is-not-retiring-but-planner-and-copilot-take-priority.441339/)
- Apple iOS 26 (Sept 2025) brought Liquid Glass, the New Reminder control, Apple Intelligence Suggested Reminders and Auto-categorize, Call Back Reminders and time zones. — [9to5Mac](https://9to5mac.com/2025/10/13/heres-everything-new-in-reminders-with-ios-26/), [9to5Mac](https://9to5mac.com/2025/12/22/ios-26s-reminders-app-got-a-new-feature-inspired-by-calendar/)
- iOS 26.2 brought Urgent alarm reminders with snooze and a Live Activity. iOS 26.4 brought the Urgent Smart List and two more ways to mark Urgent. — [9to5Mac](https://9to5mac.com/2026/02/27/ios-26-4-gives-reminders-best-new-feature-the-one-thing-it-was-missing/), [9to5Mac (Apr 2026)](https://9to5mac.com/2026/04/01/ios-26-4-makes-apples-best-new-reminders-feature-easier-to-use/)
- iOS 18 (2024) put dated reminders into the Calendar app; the Add details page cites the all-day event display. — [Apple Support](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios)

### Inferences
- For Checko, MS To Do is a "legacy" reference: My Day, Suggestions and smart date parsing are worth copying. Apple Reminders is the current reference for smart lists, sections/columns, grocery auto-categories and urgent alarms.

### iOS 27 (announced WWDC June 2026, shipped fall 2026)
- **Natural-language entry ("describe")**: type, for example, "get the groceries at 6pm tonight" and date, time and location are filled in. — [MacRumors iOS 27 guide (Oct 2026)](https://www.macrumors.com/guide/ios-27-calendar-reminders/), [MacRumors (Jun 2026)](https://www.macrumors.com/2026/06/08/ios-27-calendar-reminders/), [9to5Mac (Sep 2026)](https://9to5mac.com/2026/09/29/heres-everything-new-for-reminders-in-ios-27/)
- New metadata box around the active reminder (date, time, repeat, location, tag, flag). The bottom quick-add bar was removed, and image and flag moved into Details. — [9to5Mac](https://9to5mac.com/2026/09/29/heres-everything-new-for-reminders-in-ios-27/), [MacRumors](https://www.macrumors.com/guide/ios-27-calendar-reminders/)
- Improved grocery sorting with more languages. — [MacRumors](https://www.macrumors.com/guide/ios-27-calendar-reminders/)
- Shortcuts actions to create or delete groups, lists and sections, plus "Get What's On Screen". — [MacRumors](https://www.macrumors.com/guide/ios-27-calendar-reminders/)
- New extra-large widget size. Siri can search Reminders and Calendar. — [MacRumors](https://www.macrumors.com/guide/ios-27-calendar-reminders/)
- Apple Intelligence features require iPhone 15 Pro or later. It is unclear whether the natural-language entry itself needs Apple Intelligence. One report says the system-wide Siri reminder creation is delayed at launch. — [MacRumors](https://www.macrumors.com/guide/ios-27-calendar-reminders/), search summary of [Gadget Hacks](https://apple.gadgethacks.com/news/ios-27-reminders-new-features-smarter-ai-input-and-editing/)
- Reviewers call the update fairly minor. — [MacStories iOS 27 review](https://www.macstories.net/stories/ios-and-ipados-27-review/12/) (search snippet only, not fetched)

### Gaps
- iOS 27 details beyond the MacRumors guide (for example, any new recurrence options) were not verified.
- The exact iOS 18 Reminders/Calendar integration (creating and editing reminders inside Calendar) is not confirmed by a fetched source.

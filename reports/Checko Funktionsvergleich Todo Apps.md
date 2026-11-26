# Checko kann To-Do-Umsteiger jetzt abholen

Bei den Grundlagen hält Checko mit den vier großen Aufgaben-Apps mit: Fälligkeit mit Uhrzeit, Erinnerungen, Wiederholung, Schritte, vier Prioritäten, Kategorien, Papierkorb und Rückgängig. In drei Punkten ist Checko sogar vorn. Der **Kalender ist kostenlos** (bei Todoist und TickTick kostet er extra, Microsoft To Do hat gar keinen). Die **deutsche Schnell-Eingabe** zeigt beim Tippen, was sie erkannt hat. Und Checko bündelt **Aufgaben, Termine, Notizen, Einkauf und Gamification in einer App**. Die größten Lücken liegen woanders: Es gibt keine echten Listen oder Projekte, keinen „Mein Tag“-Plan, kein Teilen, keinen Import oder Export, keine Woche-/Tag-Ansicht im Kalender und nur einfache Wiederholungs- und Erinnerungsoptionen. Am meisten vermissen Nutzer laut Reviews und Foren zweierlei: **Aufgaben und Kalender in einer Ansicht, möglichst gratis**, und **flexible Wiederholungen**. Den ersten Wunsch erfüllt Checko schon halb. Dazu kommt eine Gelegenheit mit festem Termin: Microsoft hat am 18.11.2026 angekündigt, dass die **To-Do-Apps am 30.06.2027 eingestellt werden**. Privatkunden bekommen keinen Ersatz ([office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/)). Deshalb sollten ganz oben auf der Bauliste ein Import aus dem To-Do-Export, Listen und ein „Mein Tag“ stehen. Danach folgen Wiederholungen, die Wochenansicht und bessere Erinnerungen.

**Datenlage:** Checkos Funktionen sind im Code geprüft (Commit 14febb2, Stand Nov. 2026). Die Angaben zu den anderen Apps stützen sich auf offizielle Hilfe- und Preisseiten und, wo die fehlten, auf Reviews. Einträge mit „(?)“ sind nicht belegt oder in den Quellen widersprüchlich. Für die Nutzerwünsche gibt es **keine Stimmenzahlen**: Feedback-Portale und Reddit waren nicht erreichbar. Die Rangfolge ist daher eine Einschätzung danach, wie oft ein Thema in unabhängigen Quellen auftaucht.

**Legende:** hat · teilw. = teilweise · fehlt · (?) = unsicher oder nicht belegt

---

## Funktionsvergleich nach Bereichen

### Aufgaben-Grundlagen: Checko ist solide, aber es fehlen Anhänge und flexible Wiederholungen

| Funktion | Checko | MS To Do | Todoist | TickTick | Apple Erinnerungen |
|---|---|---|---|---|---|
| Fälligkeit mit Datum + Uhrzeit | hat | teilw. (Uhrzeit nur über „Erinnern“) | hat | hat | hat |
| Getrennte Deadline neben dem geplanten Datum | fehlt | fehlt (?) | hat (Pro) | (?) | fehlt (?) |
| Dauer / Zeitspanne | teilw. (nur Kalender-Termine: von–bis) | fehlt (?) | hat (Pro) | hat (Premium) | (?) |
| Einfache Wiederholung (täglich … jährlich) | hat | hat | hat | hat | hat |
| Flexible Wiederholung (Mo+Mi, n-ter Wochentag, „nach Erledigung“, Enddatum) | teilw. (nur in der Engine bzw. per Texteingabe) | teilw. | hat (umfangreichste Regeln) | teilw. (?) | teilw. (n-ter Wochentag ja, „X Tage nach Erledigung“ nein) |
| Unteraufgaben / Schritte | hat (eine Ebene) | hat | hat | hat (+ Checklisten) | hat (?) |
| Notizen / Beschreibung | hat (nur Text) | hat | hat (Markdown) | hat (Markdown) | hat |
| Anhänge / Bilder | fehlt | hat | hat (5 MB / 100 MB) | hat (1/Tag gratis) | hat (Fotos, Scans) |
| Prioritäten | hat (P1–P4) | teilw. (nur Stern „Wichtig“) | hat (p1–p4) | hat | hat (3 Stufen + Markierung) |
| Tags / Kategorien | hat | teilw. (Tags nur über die Suche) | hat (Labels) | hat | hat |
| Status „In Arbeit“ | teilw. (nur im Core, nicht in der Oberfläche) | fehlt (?) | teilw. (Board-Spalten) | teilw. (Kanban) | teilw. (Spalten) |
| Papierkorb + Rückgängig | hat | (?) | (?) | hat (?) | hat („Zuletzt gelöscht“) |
| Sichtbarer Verlauf pro Aufgabe | teilw. (wird gespeichert, aber nicht angezeigt) | fehlt (?) | hat (Aktivität/Reporting) | hat | fehlt (?) |

Checkos Wiederholungs-Engine kann mehr, als die Oberfläche zeigt. Das Modell kennt Intervalle und ein Enddatum (`until`). Das Dropdown bietet aber nur „Nie / Jeden Tag / Jede Woche / Alle 2 Wochen / Jeden Monat / Jedes Jahr“. „Jeden Montag und Mittwoch“ oder „letzter Werktag im Monat“ gehen gar nicht (`crates/core/src/calendar.rs`, `crates/web/src/components/task_detail.rs`). Den Maßstab setzt Todoist: `every!` wiederholt ab Erledigung, außerdem gibt es Werktags-Positionen, `every other` und Zeiträume mit `until` ([Todoist Hilfe](https://www.todoist.com/help/articles/introduction-to-recurring-dates-YUYVJJAV)). Microsoft To Do kann laut einem Forenbeitrag kein „letzter Wochentag im Monat“ ([MS Tech Community](https://techcommunity.microsoft.com/t5/microsoft-to-do/can-t-create-a-custom-repeat-action-every-last-day-of-weekdays/m-p/3443140)).

### Organisation & Ansichten: Checko fehlen echte Listen

| Funktion | Checko | MS To Do | Todoist | TickTick | Apple Erinnerungen |
|---|---|---|---|---|---|
| Listen / Projekte | fehlt (nur Kategorien) | hat | hat (+ Unterprojekte) | hat | hat |
| Gruppen / Ordner / Abschnitte | fehlt | teilw. (Listengruppen, keine Abschnitte) | hat (Ordner nur Business) | hat | hat (Gruppen + Abschnitte) |
| Kategorien verwalten (umbenennen, Farbe, löschen) | fehlt | – | hat | hat | hat |
| Heute-Ansicht | hat (Bento-Dashboard) | hat („Mein Tag“ mit Vorschlägen) | hat (Heute/Demnächst) | hat (Vorgeschlagene Aufgaben) | hat (Smart List „Heute“) |
| Eigene, gespeicherte Filter / Smart Lists | teilw. (Kategorie-Chips, nicht speicherbar) | fehlt (nur feste Listen) | hat (3 gratis / 150 Pro) | hat | hat (beliebig mit UND/ODER) |
| Wählbare Sortierung | fehlt (feste automatische Sortierung) | hat (5 Optionen) | hat (?) | (?) | (?) |
| Manuelle Reihenfolge per Ziehen | fehlt | hat | hat | hat (?) | hat (?) |
| Kanban / Board | fehlt | fehlt | hat | hat | hat („Als Spalten“) |
| Eisenhower-Matrix | fehlt | fehlt | fehlt | hat | fehlt |
| Vorlagen | fehlt | fehlt (?) | hat | hat | hat |
| Globale Suche | hat (Aufgaben, Notizen, Einkauf; keine Termine) | hat | hat | hat (+ Suchfilter) | hat |

Quellen: Apple [Smart Lists](https://support.apple.com/guide/iphone/use-smart-lists-iphe882772ed/ios) und [Abschnitte/Spalten/Vorlagen](https://9to5mac.com/2025/02/14/apples-reminders-app-seven-power-user-features/), MS [Sortieren und Suchen](https://support.microsoft.com/en-us/office/sort-and-search-in-lists-133fa637-3f47-4633-8d78-7289059fa630), [Todoist Preise](https://todoist.com/pricing), [TickTick Funktionen](https://ticktick.com/features).

### Planung & Kalender: Checkos stärkster Bereich, aber Woche/Tag fehlt

| Funktion | Checko | MS To Do | Todoist | TickTick | Apple Erinnerungen |
|---|---|---|---|---|---|
| Monatskalender mit Aufgaben | **hat (gratis)** | fehlt | teilw. (nur Pro) | teilw. (Basis gratis, voll in Premium) | teilw. (in der Kalender-App, als ganztägige Einträge) |
| Woche / Tag / Timeline (Time-Blocking) | fehlt (nur Monat + Agenda) | fehlt | hat (Pro) | hat (Premium) | teilw. (Kalender-App) |
| Drag & Drop im Kalender | teilw. (nur mit Maus, zwischen Tagen) | fehlt | hat (Pro) | hat | (?) |
| Externe Kalender (Google, Outlook, ICS) | fehlt | teilw. (Outlook, kein Google) | hat (2-Wege, 1 bzw. 3 Konten) | teilw. (Abo; ob 2-Wege, ist umstritten) | hat (iCloud-Kalender) |
| Natürliche Sprache bei der Eingabe | **hat (Deutsch, Live-Chips)** | teilw. (nur Englisch, Windows/iOS) | hat (Deutsch nicht geprüft) | hat | teilw. (erst ab iOS 27) |
| Gewohnheiten (Habits) | fehlt | fehlt | fehlt | hat (5 gratis) | fehlt |
| Fokus-Timer / Pomodoro | fehlt | fehlt | fehlt | hat (ob gratis, ist umstritten) | fehlt |
| Statistik / Rückblick | teilw. (14-Tage-Diagramm, Kategorie-Bögen) | fehlt | hat (Produktivitätsansicht) | hat | fehlt |
| KI-Assistent | fehlt (Phase 8) | fehlt (KI geht nach Planner/Copilot) | hat (Ramble, Assist, Task Capture) | hat (Premium, MCP) | teilw. (Vorschläge, Auto-Abschnitte; ab iPhone 15 Pro) |

Quellen: [Todoist Kalender-Integration](https://www.todoist.com/help/articles/use-the-calendar-integration-rCqwLCt3G), [TickTick Upgrade](https://ticktick.com/upgrade), [MS Smart Due Date](https://support.microsoft.com/en-us/office/smart-due-date-reminder-recognition-in-microsoft-to-do-b8cde14b-28da-42e1-9ce8-d75cc3717993), [iOS 27 Erinnerungen](https://www.macrumors.com/guide/ios-27-calendar-reminders/), [iOS 26 Apple Intelligence](https://9to5mac.com/2025/10/13/heres-everything-new-in-reminders-with-ios-26/). Ob TickTick Google- und Outlook-Kalender in beide Richtungen synchronisiert, bleibt offen: Die Quellen widersprechen sich ([temporal.day](https://temporal.day/blog/ticktick-vs-todoist-2026)).

### Erinnerungen: Checkos Push funktioniert, ist aber starr

| Funktion | Checko | MS To Do | Todoist | TickTick | Apple Erinnerungen |
|---|---|---|---|---|---|
| Erinnerung relativ zur Fälligkeit | hat (0/5/15/30 Min., 1 Std., 1 Tag) | teilw. | hat | hat | hat („Früher erinnern“) |
| Frei wählbarer Erinnerungszeitpunkt | fehlt (`remind_at` steht im Modell, wird aber nicht genutzt) | hat | hat (Pro, `!14:00`) | hat | hat |
| Mehrere Erinnerungen pro Aufgabe | fehlt (eine) | fehlt (?) | hat (Pro) | hat (2 gratis / 5) | teilw. |
| Ortsbezogen | fehlt | fehlt | hat (mobil; laut einer Review nur Pro) | teilw. (nur iOS) | hat (+ Auto, „Beim Schreiben mit …“) |
| Hartnäckig / Alarm | fehlt | fehlt | hat (Urgent, 2026) | hat (Constant Reminder) | hat (Dringend, iOS 26.2) |
| Schlummern / Aktionen in der Benachrichtigung | fehlt (Klick öffnet nur die App) | (?) | (?) | (?) | teilw. (9-Min.-Schlummern bei „Dringend“) |
| Push ohne geöffnete App | hat (Web Push vom Server; iOS nur als Home-Bildschirm-App ab 16.4) | hat (native; Android-Akku-Probleme) | hat (native; Android-Akku-Probleme) | hat | hat |
| Erinnerung per E-Mail | fehlt | fehlt | fehlt (?) | hat (Premium) | fehlt |

Quellen: [Apple Details hinzufügen](https://support.apple.com/guide/iphone/add-details-iphec7a1de82/ios), [Apple „Dringend“](https://9to5mac.com/2026/02/27/ios-26-4-gives-reminders-best-new-feature-the-one-thing-it-was-missing/), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd), [TickTick Funktionen](https://ticktick.com/features), [MS Erinnerungen](https://support.microsoft.com/en-us/office/add-due-dates-and-reminders-in-microsoft-to-do-064d9696-08d1-4433-bfdd-f661dc97491f).

### Zusammenarbeit: Checko hat keine, alle anderen schon

| Funktion | Checko | MS To Do | Todoist | TickTick | Apple Erinnerungen |
|---|---|---|---|---|---|
| Geteilte Listen | fehlt | hat | hat (5 pro Projekt) | hat (2 gratis / 29) | hat (nur mit iCloud-Nutzern) |
| Aufgaben zuweisen | fehlt | hat | hat | hat | hat (@Name) |
| Kommentare | fehlt | fehlt (?) | hat | hat | fehlt (?) |
| Team-Bereich, Rollen, Berichte | fehlt | fehlt (Teams nutzen Planner) | hat (Business) | fehlt | fehlt |
| Geteilte Einkaufsliste | fehlt (geplant für „später“) | teilw. (geteilte Liste, ohne Einkaufslogik) | teilw. | teilw. | hat (Einkaufsliste teilbar) |

Quellen: [Apple Teilen](https://support.apple.com/guide/iphone/share-and-collaborate-iph2a8f9121e/ios), [MS Zuweisen](https://support.microsoft.com/en-us/todo/assign-tasks-in-shared-lists), [Todoist Preise](https://todoist.com/pricing), [TickTick Upgrade](https://ticktick.com/about/upgrade).

### Plattformen & Extras: Checko punktet mit Web, Offline und Einkauf, ihm fehlen native Apps und Integrationen

| Funktion | Checko | MS To Do | Todoist | TickTick | Apple Erinnerungen |
|---|---|---|---|---|---|
| Web-App | **hat (Hauptplattform, PWA)** | hat | hat | hat | teilw. (iCloud.com stark eingeschränkt) |
| Native iOS/Android-App | fehlt (PWA installierbar) | hat (bis 30.06.2027) | hat | hat | teilw. (nur Apple, kein Android) |
| Desktop | teilw. (PWA) | hat (Win/Mac) | hat (+ Linux) | hat (+ Linux) | teilw. (nur Mac) |
| Offline nutzbar | hat (local-first, ab dem zweiten Besuch) | hat (native Desktop-Apps) | hat | hat | hat (?) |
| Sync zwischen Geräten | hat (eigener Server, „letzte Änderung gewinnt“) | hat | hat | hat (Berichte über Sync-Fehler) | hat |
| Widgets | teilw. (iPhone über Scriptable, nur Anzeige, kein Android) | hat (?) | hat | hat | hat |
| Smartwatch | fehlt | fehlt (?) | hat (Apple Watch, Wear OS) | hat | hat (?) |
| Sprachassistent / Spracheingabe | fehlt | fehlt (wird gewünscht) | hat (Ramble, Siri) | hat | hat (Siri) |
| E-Mail → Aufgabe | fehlt | hat (Outlook-Markierung) | hat (Pro) | hat (Add-on) | teilw. (Siri-Vorschläge aus Mail) |
| Browser-Erweiterung | fehlt | (?) | hat | hat | fehlt |
| Integrationen / API / MCP | fehlt | teilw. (Graph API, Outlook, Teams; läuft aus) | hat (120+, MCP) | teilw. (Notion, Telegram, MCP) | teilw. (Kurzbefehle) |
| Import / Export | fehlt | teilw. (Export erst „vor der Abschaltung“ zugesagt) | hat (CSV, Auto-Backup Pro) | (?) | fehlt (?) (nur Drucken/Teilen) |
| Hell/Dunkel, Farbthemen | teilw. (Hell/Dunkel/System, keine Farbthemen) | hat (?) | hat (alle Themen gratis seit 19.11.2026) | hat (40+, voll nur Premium) | teilw. (folgt dem System) |
| Tastaturkürzel | teilw. (Strg+K, Esc) | (?) | hat (umfangreich) | hat | (?) |
| Notizen-Modul | hat (einfach, ohne Markdown und `[[Links]]`) | fehlt | fehlt (nur Beschreibungen) | hat (Markdown) | fehlt (separate Notizen-App) |
| Einkaufsliste mit Gängen | **hat (8 Gänge, Mengen, Produktlinks, Verlauf, Vorschläge)** | fehlt | fehlt | fehlt (?) | hat (lernt aus Korrekturen) |

Quellen: [2sync Plattformvergleich](https://2sync.com/blog/ticktick-vs-todoist), [office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/), [Apple iCloud-Web-Lücken](https://discussions.apple.com/thread/255706034), [Apple Einkaufsliste](https://www.thekitchn.com/grocery-shopping-reminders-app-review-23768909), [Todoist 2026 Changelog](https://www.todoist.com/help/articles/2026-changelog-HD3jJAtLd).

### Spielerisches: Nur Todoist kommt an Checko heran, aber Todoist lässt sich abschalten

| Funktion | Checko | MS To Do | Todoist | TickTick | Apple Erinnerungen |
|---|---|---|---|---|---|
| Punkte / XP | hat (Priorität × Pünktlichkeit) | fehlt | hat (Karma) | teilw. („Achievement Score“) | fehlt |
| Level | hat | fehlt | hat (8 Stufen bis „Enlightened“) | (?) | fehlt |
| Streaks | hat (Tage) | fehlt | hat (Tag + Woche) | teilw. (bei Habits) | fehlt |
| Erfolge / Achievements | hat (12, mit Konfetti) | fehlt | teilw. (Karma-Stufen) | teilw. | fehlt |
| Tages-/Wochenziele | fehlt | fehlt | hat (5 / 25) | fehlt (?) | fehlt |
| Abschaltbar, Urlaubsmodus, Streak-Rettung | fehlt | – | hat (aus, Urlaub, freie Tage, 3× Streak retten) | (?) | – |

Quellen: [Todoist Karma](https://www.todoist.com/help/todoist/features/introduction-to-karma-OgWkWy), [Todoist Produktivität](https://www.todoist.com/help/todoist/features/use-the-productivity-view-in-todoist-6S63uAa9), [2sync](https://2sync.com/blog/ticktick-vs-todoist). Checko: `crates/core/src/gamification.rs`, `crates/core/src/achievements.rs`.

### Preise: Checko und Apple kosten nichts, Todoist und TickTick sperren den Kalender hinter Abos

| App | Gratis | Bezahlt | Wichtigste Bezahlschranke |
|---|---|---|---|
| Checko | komplett | – | keine (der eigene Server läuft auf Render + Neon) |
| MS To Do | komplett | – | keine, **aber die Apps enden am 30.06.2027** |
| Todoist | Beginner | Pro 60 $/Jahr oder 7 $/Monat (seit 10.12.2025, vorher 48 $); Business 96 $/Nutzer/Jahr | Kalender-Layout, eigene Erinnerungen, Deadlines, Dauer, E-Mail → Aufgabe; gratis nur 5 Projekte und 3 Filter |
| TickTick | Free | Premium **49,99 $/Jahr** laut offizieller Seite; Reviews nennen noch 35,99 $ (**widersprüchlich**) | volle Kalender-Ansichten, Timeline, Dauer, KI; gratis nur 9 Listen und 2 Erinnerungen pro Aufgabe |
| Apple Erinnerungen | komplett (mit Apple-Gerät) | – | KI-Funktionen erst ab iPhone 15 Pro |

Quellen: [Todoist Preise](https://todoist.com/pricing), [Todoist Preiserhöhung 2025](https://www.todoist.com/help/articles/todoist-pro-pricing-update-in-2025), [TickTick Upgrade](https://ticktick.com/about/upgrade), [techharry](https://www.techharry.com/2026/09/tick-tick-review.html). Euro-Preise wurden nicht geprüft.

---

## Was Nutzer vermissen: an erster Stelle Kalender und Aufgaben zusammen

Mit Abstand am häufigsten wünschen sich Nutzer **Aufgaben und Kalender in einer Ansicht**. Bei Microsoft To Do ist das die Hauptbeschwerde. Auf AlternativeTo schrieb ein Nutzer, die App sei ohne Kalenderansicht „unusable“, ein anderer vermisste die Google-Kalender-Anbindung ([AlternativeTo](https://www.alternativeto.net/software/microsoft-to-do/about/), [Software Advice](https://www.softwareadvice.com/project-management/microsoft-to-do-profile/)). Bei Todoist und TickTick gibt es die Ansicht, aber nur gegen Geld. Bei TickTick ist die Bezahlschranke vor dem Kalender in einer Capterra-Bewertung die Hauptbeschwerde ([Capterra](https://www.capterra.com/p/170641/TickTick/reviews/?page=2)). Wer von Todoist zu TickTick wechselt, nennt dafür vor allem drei Gründe: die kombinierte Ansicht, Habits und Pomodoro an einem Ort sowie den Preis. Die **Todoist-Preiserhöhung vom Dezember 2025** war für viele der Anlass, Alternativen zu vergleichen ([Asian Efficiency](https://www.asianefficiency.com/task-management/todoist-vs-ticktick/), [Rework](https://resources.rework.com/tools/productivity/todoist-vs-ticktick)).

| # | Was fehlt (qualitative Rangfolge) | Bei welchen Apps | Wie steht Checko da? |
|---|---|---|---|
| 1 | Aufgaben im Kalender, ohne Abo | To Do (fehlt), Todoist und TickTick (Bezahlschranke) | **gut**: Monat gratis; Woche/Tag fehlt |
| 2 | Flexible Wiederholungen (z. B. „X Tage nach Erledigung“, jeden Werktag, Wiederholung springt nicht auf vergangene Daten) | To Do ([Forum](https://techcommunity.microsoft.com/discussions/to-do/todo-could-be-much-more---bugs--basic-features-missing/2149615)), Apple ([9to5Mac](https://9to5mac.com/2024/06/11/reminders-in-ios-18-can-now-live-inside-the-calendar-app)), TickTick | teilweise |
| 3 | Auf allen Plattformen inkl. Web und Android | Apple ([Apple Discussions](https://discussions.apple.com/thread/252660132)) | **gut** (PWA) |
| 4 | Faire Preise, keine Abo-Falle | Todoist (+25 %), TickTick (Preise unklar) | **gut** (gratis) |
| 5 | Habits + Fokus-Timer im selben Tool | Todoist („doesn't do habits. At all.“) | fehlt |
| 6 | Zuverlässige Erinnerungen und Schlummern | To Do und Todoist auf Android ([MS Support](https://support.microsoft.com/en-us/todo/troubleshoot-reminder-issues-in-microsoft-to-do-for-android), [Todoist Hilfe](https://www.todoist.com/help/articles/troubleshoot-notifications-not-received-from-todoist-OKO3ezsK)); Apple (Schlummern, ältere Wünsche) | teilweise (Server-Push, aber starr) |
| 7 | Geteilte Listen, die zuverlässig synchronisieren (vor allem Einkauf) | To Do (Sync nur in eine Richtung, [Forum](https://techcommunity.microsoft.com/discussions/to-do/synchronization-issue/4445162/replies/4448705)), Apple ([MacRumors](https://forums.macrumors.com/threads/question-about-shared-reminders-the-new-grocery-list.2406641)) | fehlt |
| 8 | Richtige Tags, Sortierung nach Liste | To Do ([Microsoft Q&A](https://learn.microsoft.com/en-us/answers/questions/5618421/why-can-you-not-sort-by-list-in-to-do-my-day-or-pl)) | teilweise (Tags ja, Sortierung nein) |
| 9 | Spracheingabe zum schnellen Erfassen | To Do ([Forum 2025](https://techcommunity.microsoft.com/discussions/to-do/features-suggestions/4464263)) | fehlt |
| 10 | Aufgeräumte Oberfläche statt Überladung | TickTick („cluttered“, „childish“ Themes) ([G2](https://www.g2.com/products/ticktick)) | gut, solange Checko ruhig bleibt |

Zwei Warnungen kommen aus den Quellen. Erstens zahlen All-in-one-Apps mit Überladung: Das ist der häufigste Kritikpunkt an TickTick ([Capterra](https://www.capterra.com/p/170641/TickTick/reviews/)). Zweitens kann Gamification Druck erzeugen. Todoist bietet deshalb an, Karma auszuschalten, kennt Urlaubsmodus und freie Tage und lässt einen gerissenen Streak bis zu dreimal retten ([Todoist Karma](https://todoist.com/help/articles/introduction-to-karma-OgWkWy)). Ein Essay vergleicht Karma mit dem Schuldgefühl bei Duolingo-Streaks ([Hulry](https://hulry.com/firesides/gamification/)). Bei Microsoft To Do nannte Microsoft die Listengruppen einst „one of the most requested features ever“ ([Windows Central](https://www.windowscentral.com/microsoft-do-now-has-list-groups-all-users)). Das ist wichtig für Umsteiger, denn Checko hat noch nicht einmal Listen.

---

## Was Checko schon besser macht

**Kalender ohne Bezahlschranke.** Checko zeigt Termine und To-dos in einem Monatsraster. To-dos lassen sich dort direkt abhaken, ein Doppelklick legt einen Eintrag an, und Einträge lassen sich zwischen Tagen verschieben (`crates/web/src/pages/calendar.rs`). Genau das ist Nutzerwunsch Nr. 1. Todoist verlangt dafür Pro, TickTick schaltet nur einen Teil gratis frei, und To Do hat gar keinen Kalender.

**Deutsche Schnell-Eingabe mit sichtbarer Erkennung.** „Zahnarzt morgen 14:00 #gesundheit !1“, „nächsten Freitag“, „alle 3 Wochen“: Erkannte Teile erscheinen beim Tippen als Chips (`crates/core/src/parse.rs`, mit Tests). Microsoft erkennt Datumswörter nur auf Englisch ([MS Support](https://support.microsoft.com/en-us/office/smart-due-date-reminder-recognition-in-microsoft-to-do-b8cde14b-28da-42e1-9ce8-d75cc3717993)), Apple erst ab iOS 27. Ob Todoist und TickTick deutsche Datumswörter verstehen, wurde nicht geprüft.

**Einkaufsliste, die mitdenkt.** Checko sortiert automatisch nach Gängen, erkennt Mengen („2x Milch“), liest Produktlinks samt Titel aus, führt einen Kaufverlauf und macht Vorschläge (`crates/core/src/shopping.rs`). Vergleichbar ist nur Apples Einkaufsliste. To Do, Todoist und TickTick haben nichts Eigenes dafür.

**Alles in einer App, ohne Abo.** Aufgaben, Termine, Notizen, Einkauf und Fortschritt gibt es sonst nur verteilt auf mehrere Apps oder gegen Geld. Bei der Gamification (XP nach Priorität und Pünktlichkeit, Level, Streak, 12 Erfolge mit Konfetti) ist Todoists Karma die einzige ernsthafte Konkurrenz.

**Local-first, unabhängig von einer Plattform, mit eigenem Server.** Checko läuft offline im Browser, auf iPhone, Android und PC gleich. Der Sync läuft über einen eigenen Server, die Daten bleiben unter eigener Kontrolle. Das ist das Gegenmodell zu Apple (nur Apple-Geräte) und zu Microsoft (die App wird eingestellt). Papierkorb und Rückgängig sind ebenfalls eingebaut.

Ehrlich bleibt: Push-Nachrichten kommen auf dem iPhone nur an, wenn Checko als Home-Bildschirm-App installiert ist (ab iOS 16.4). Das Widget braucht die Dritt-App Scriptable und kann nur anzeigen, nicht abhaken. Beim Sync gewinnt die zuletzt gespeicherte Version eines ganzen Eintrags, es gibt also keine Zusammenführung einzelner Felder (`crates/server/src/main.rs`).

---

## Das Aus von Microsoft To Do öffnet ein Zeitfenster bis Mitte 2027

Microsoft hat am **18.11.2026** angekündigt, To Do auslaufen zu lassen: „The To Do apps will retire on June 30, 2027.“ Dienst, Graph API und Sync sollen später folgen, Termine dafür stehen noch nicht fest. Konten von Arbeit und Schule wechseln in einen privaten Planner-Plan. Ihre bestehenden Aufgaben werden **nicht migriert**, sondern sind dort nur bis zum 30.06.2027 sichtbar. **Privatkonten, auch mit Microsoft 365 Personal/Family, bekommen keinen Ersatz.** Planner hat keine Listengruppen und im privaten Plan keine Anhänge. Geteilte Listen und Zuweisungen werden nicht übernommen. Ein Export ist „vor der Abschaltung“ zugesagt ([office365itpros](https://office365itpros.com/2026/11/20/to-do-apps-retirement/), [The Next Web](https://thenextweb.com/news/microsoft-to-do-retiring-planner-personal-accounts-wunderlist), [Topedia](https://blog.topedia.com/posts/microsoft-to-do-will-be-retired-as-planner-consolidates-into-unified-plans)). Eine frühere Meldung vom August 2026 („To Do isn't retiring“) ist damit überholt ([WindowsForum](https://windowsforum.com/news/microsoft-to-do-is-not-retiring-but-planner-and-copilot-take-priority.441339/)).

Damit suchen in den nächsten sieben Monaten viele Privatnutzer einer kostenlosen, schlichten App eine neue Heimat. Was sie erwarten, ist klar: Listen und Listengruppen, „Mein Tag“ mit Vorschlägen, Schritte, „Wichtig“, Erinnerung mit Uhrzeit, Wiederholung, Anhänge und geteilte Listen. Schritte, Erinnerungen, Wiederholung und Prioritäten hat Checko schon. Listen, Gruppen, „Mein Tag“, Anhänge und Teilen fehlen noch.

**Unsicher ist:** In welchem Format der zugesagte Export kommt (JSON, CSV, Outlook-PST?), ist noch nicht bekannt. Ein Importer lässt sich daher erst fertigstellen, wenn Microsoft das Format veröffentlicht. Bis dahin gibt es einen Weg über die Graph API, solange sie läuft. Dafür bräuchte der Checko-Server allerdings eine Microsoft-Anmeldung per OAuth, und das ist deutlich aufwendiger. Außerdem ist Checko bisher ein persönlicher Organizer, der Server erlaubt das Schließen der Registrierung. **Ob Checko überhaupt fremde Umsteiger ansprechen soll, ist eine offene Entscheidung für NoFufu.** Ein Import lohnt sich aber auch für die eigene Migration und als Vertrauenssignal.

---

## Bauliste: Top 10 für Checko

Die Reihenfolge folgt drei Kriterien: Was öffnet das To-Do-Zeitfenster, was behebt die häufigsten Nutzerbeschwerden, was ist im Verhältnis zum Aufwand am wirkungsvollsten.

| # | Funktion | Begründung | Aufwand | Bezug zu Nutzerwünschen |
|---|---|---|---|---|
| 1 | **Import & Export**: JSON-Export/Backup in den Einstellungen, Import des MS-To-Do-Exports (sobald das Format bekannt ist) und von Todoist-CSV | Steht in PLAN.md, fehlt aber im Code. Ohne Import kein Umstieg, ohne Export kein Vertrauen. Zeitkritisch: Stichtag 30.06.2027 | mittel | To-Do-Aus, Privatnutzer ohne Ersatz; Todoist hat CSV-Import/Export |
| 2 | **Listen statt nur Kategorien**: zuerst Kategorien verwalten (umbenennen, Farbe, löschen), dann optional Listen mit Gruppen | Damit To-Do-Listen beim Import einen Platz haben; Grundlage für Teilen und Sortieren | mittel (Verwaltung) bis groß (Listen + Gruppen) | Listengruppen bei To Do „most requested“; alle vier Apps haben Listen |
| 3 | **„Mein Tag“ / Morgen-Plan**: morgens Vorschläge (überfällig, heute fällig, gestern liegen geblieben) übernehmen, abends zurücksetzen | Das Markenzeichen von To Do; Umsteiger suchen genau das. Morgen-Plan und Abend-Check-in stehen schon in PLAN.md | mittel | To-Do-„My Day“; TickTick „Suggested Tasks“ |
| 4 | **Flexible Wiederholung in der Oberfläche**: Wochentage mehrfach wählen, n-ter/letzter Wochentag, Enddatum (Engine hat `until`), „X Tage nach Erledigung“, Parser um „jeden Mo und Mi“ erweitern | Engine ist zum Teil schon da; reine Core-Logik, gut testbar | mittel | Schmerzpunkt bei To Do und Apple; Todoist ist hier Maßstab |
| 5 | **Erinnerungen ausbauen**: frei wählbarer Zeitpunkt (`remind_at` nutzen), mehrere Erinnerungen, Aktionen „Erledigt“ und „10 Min. später“ in der Benachrichtigung (Service Worker), Parser-Syntax für Erinnerungen | Kleine Lücken mit großer Wirkung im Alltag; Server-Push steht schon | klein bis mittel | Schlummern (Apple), Erinnerungsfrust (Android); Todoist `!30 min before` |
| 6 | **Kalender-Wochen-/Tagesansicht** mit Zeitblöcken, Drag & Drop auch per Touch | Macht Checkos stärkstes Argument (Kalender gratis) vollständig; offen aus Phase 6 | groß | Wunsch Nr. 1; bei Todoist und TickTick kostenpflichtig |
| 7 | **Fokus-Modus + nachsichtige Gamification**: alles abschaltbar, Ruhetage oder Streak-Schutz, kein Abzug fürs Verschieben | Kleiner Aufwand, nimmt Druck raus; steht schon in CLAUDE.md | klein | Karma-Kritik; Todoist bietet aus, Urlaubsmodus und Streak-Rettung |
| 8 | **Geteilte Einkaufsliste** (erste geteilte Liste): Einladung per Link, Änderungen fast in Echtzeit, Anzeige „zuletzt synchronisiert“ | Einkauf ist Checkos Paradedisziplin; Teilen ist die größte Lücke gegenüber allen vier Apps | groß (Rechte, Server, Konflikte) | Geteilte Listen bei allen vier Apps; Sync-Frust bei To Do und Apple |
| 9 | **Gewohnheiten (Habits)**, optional mit Fokus-Timer | Passt direkt zu XP und Streaks; das wichtigste Argument von TickTick | mittel | „Todoist doesn't do habits“; Wechselgrund zu TickTick |
| 10 | **Kalender-Abo (ICS-Feed) vom Server**, nur lesend, für Google, Apple und Outlook | Bringt Checko-Termine ohne OAuth-Aufwand in andere Kalender; Zwei-Wege-Sync kann später kommen | klein bis mittel | Fehlende Google-Anbindung bei To Do; Todoist hat Kalender-Sync |

**Knapp dahinter** liegt der KI-Assistent aus Phase 8: Schnell-Erfassen per Sprache oder Text, Aufgaben zerlegen. Todoist, TickTick und Apple investieren 2026 stark in diese Richtung ([TechCrunch](https://techcrunch.com/2026/01/21/todoists-app-now-lets-you-add-tasks-to-your-to-do-list-by-speaking-to-its-ai/)). Er kostet aber laufend API-Geld und löst keinen der häufigsten Schmerzpunkte. **Schnelle Gewinne** mit kleinem Aufwand: Status „In Arbeit“ und den Verlauf pro Aufgabe in der Oberfläche zeigen (beides gibt es im Core schon), Termine in die Suche aufnehmen, eine wählbare Sortierung, Markdown in Notizen. Anhänge bleiben mittelfristig wichtig für To-Do-Umsteiger. Sie brauchen aber einen Dateispeicher auf dem Server oder IndexedDB und sind daher groß.

---

## Fazit

Checko muss nicht in die Breite von Todoist oder TickTick wachsen. Es fehlen vor allem die Dinge, mit denen Nutzer überhaupt erst einziehen: Daten mitbringen (Import), sie wie gewohnt ordnen (Listen) und den Tag planen („Mein Tag“). Bei Kalender, deutscher Eingabe, Einkauf und Preis ist Checko schon da, wo Nutzer der Konkurrenz hinwollen. Das ist eine seltene Lage. Die Wünsche, die Todoist und TickTick hinter Abos verstecken, erfüllt Checko kostenlos, und die größte kostenlose Alternative verschwindet im Juni 2027.

Das zweite Ergebnis: Checkos eigene Pläne passen gut zu den Nutzerwünschen. Morgen-Plan, Fokus-Modus, Wochenansicht, Export und geteilte Einkaufsliste stehen schon in PLAN.md oder CLAUDE.md. Neu ist vor allem die Reihenfolge, die das To-Do-Aus vorgibt. Vorher sollte NoFufu eine Grundsatzfrage klären: Soll Checko auch für andere da sein? Davon hängt ab, ob Import und Teilen Vorrang vor Komfort-Funktionen bekommen.

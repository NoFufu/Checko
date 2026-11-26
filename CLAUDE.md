# 🧠 Master‑Prompt – „Checko“ Organizer

> Diesen Prompt am Anfang jeder neuen Claude‑Code‑Session einfügen (oder als `CLAUDE.md` ins Repo legen). Danach die gewünschte Phase nennen, z. B. „Starte Phase 1, Modul To‑Do“.

---

Du bist ein erfahrener **Rust‑Fullstack‑Entwickler und UI/UX‑Designer**. Wir bauen gemeinsam **„Checko“**, einen persönlichen Organizer (To‑Dos, Kalender, Notizen, Einkaufsliste, Erinnerungen), der im Browser (PWA), auf dem Desktop und auf Android/iOS läuft.

## Pflicht‑Technologien
- **Sprache: ausschließlich Rust** (außer minimalem CSS/HTML und Build‑Config). Ausnahme nur für Test‑Werkzeuge: der Check‑up in `e2e/` nutzt Playwright (JS); die App selbst bleibt 100 % Rust.
- **Jetzt: reine Webanwendung.** Leptos 0.8 (CSR, WASM) + `leptos_router`, Build mit Trunk, Tailwind CSS v4. Später als PWA installierbar; Tauri (Desktop/Mobile) erst in einer späteren Phase – Code so schreiben, dass er wiederverwendbar bleibt.
- **Architektur:** Cargo‑Workspace mit `crates/core` (pure Rust, keine Web‑Abhängigkeiten) und `crates/web` (Leptos‑App).
- **Sync PC ↔ Handy ist Pflicht** (Axum‑Server, Phase 5). KI über Claude API auf dem Server (Phase 8).
- **Daten:** local‑first im Browser. Trait `Storage`, zuerst `localStorage`, später IndexedDB. IDs = UUID v7, jede Entität hat `created_at`, `updated_at`, `deleted_at` (Soft‑Delete, sync‑fähig).
- **Zeit:** `chrono`, Wiederholungen mit eigener Regel in `core/calendar.rs` (kein `rrule`).

## Design‑Vorgaben
- Orientiere dich an **reactbits.dev** (z. B. Soft Aurora, Dock, Magic Bento, Animated List, Spring Check, Swipe Row, Click Spark, Blur Text, Counter) und **ui.aceternity.com** (Bento Grid, Card Spotlight, Glowing Effect, Floating Dock, Placeholders‑And‑Vanish‑Input, Animated Modal, Timeline). Siehe Tabelle in `docs/PLAN.md`.
- Allgemein: Orientiere dich an **reactbits.dev** und **ui.aceternity.com**, portiere die Effekte aber nach Leptos/CSS (kein React): Aurora‑Hintergrund, Spotlight‑Cards, Bento‑Grid‑Dashboard, Floating Dock, SplitText/BlurText‑Überschriften, ClickSpark/Konfetti beim Abhaken, Tilt/Magnet‑Hover.
- Stil: freundlich, rund, Glas‑Karten, Grün `#65a982`, Lavendel `#a48cd9`, Pfirsich `#f4a587`, Tinte `#251e2e`, Light + Dark Mode, Schrift Nunito/Plus Jakarta Sans.
- Animationen 150–300 ms mit Spring‑Easing, `prefers-reduced-motion` respektieren, WCAG AA, volle Tastaturbedienung, mobil‑first & responsiv.

## Spielerisch (Gamification)
- XP pro erledigter Aufgabe (Priorität × Pünktlichkeit), Level, Streaks, Achievements.
- Maskottchen: **rosa Axolotl mit grünem Schal** (Sprite‑Sheet `assets/mascot/axolotl-sprites.webp`, Zustände siehe `docs/PLAN.md`), reagiert auf Fortschritt. Name noch offen.
- Themes werden mit Levels freigeschaltet. Alles abschaltbar („Fokus‑Modus“).
- Gamification‑Logik liegt als reine, getestete Funktionen in `core`.

## Kernfunktionen
1. Heute‑Dashboard (Bento‑Grid). 2. To‑Dos mit Projekten, Prioritäten, Unteraufgaben, Tags, Wiederholung. 3. Kalender (Monat/Woche/Tag, Drag & Drop). 4. Markdown‑Notizen mit `[[Links]]` und Suche. 5. Einkaufsliste mit Kategorien und Wisch‑Gesten. 6. Erinnerungen mit nativen Notifications. 7. Quick‑Add/Command‑Palette (`Strg+K`) mit Syntax wie `Zahnarzt morgen 14:00 #gesundheit !1` (deutsche Datumswörter: heute, morgen, übermorgen, Montag, nächste Woche …).

## Workflow‑Ideen (aus der Recherche)
- Status Geplant → In Arbeit → Erledigt, datierter Verlauf pro Aufgabe.
- Morgen‑Plan, Abend‑Check‑in, Wochenrückblick.
- Aktivitäts‑Log + Undo, Papierkorb statt hartem Löschen, Bestätigung vor Löschen.
- Globale Suche über alle Module.

## Zusammenarbeit
- Claude ist für das gesamte Projekt zuständig (Code, Optik, CSS) und darf frühere Arbeit von Codex ändern oder ersetzen.
- Codex liefert nur noch Grafiken (`assets/**`, siehe `AGENTS.md`). NoFufu kann Grafiken auch direkt an Claude schicken, Claude legt sie ab und pusht sie.
- Fertige Phasen landen in `main`; GitHub Pages veröffentlicht automatisch unter https://nofufu.github.io/Checko/.
- Der Link zur App steht immer ganz oben in der `README.md` und muss dort bleiben.

## Arbeitsweise
- Arbeite in **kleinen, lauffähigen Schritten**; nach jedem Schritt müssen `cargo test -p checko-core`, `cargo clippy -p checko-web --target wasm32-unknown-unknown -- -D warnings` und `cargo clippy -p checko-core -p checko-server --all-targets -- -D warnings` grün sein.
- Schreibe Tests für jede Logik im `core` (vor allem Parser, XP, Wiederholungen).
- Erkläre kurz auf **Deutsch**, was du gemacht hast und wie ich es starte (`trunk serve`).
- Keine Platzhalter‑„TODO“-Implementierungen ohne Hinweis; keine unnötigen Abhängigkeiten.
- Halte dich an den Plan in `docs/PLAN.md`; frage nach, wenn eine Entscheidung offen ist, statt zu raten.
- Commits klein und beschreibend (Conventional Commits).
- **Sparsam arbeiten:** Prüf‑Screenshots nur bei sichtbaren Design‑Änderungen, klein (Ausschnitt/verkleinert), möglichst mehrere Ansichten in einem Bild. Keine Webseiten wiederholt abrufen.
- **Eine Aufgabe = eine Sitzung:** Nach einer erledigten Aufgabe kann NoFufu eine neue Sitzung starten; der Stand steht unten unter „Aktueller Stand“ – dort nach größeren Schritten aktualisieren.

## Check‑up & Bewertungsbogen
- `npm run checkup` (nach `trunk build --release`): klickt alle Kernabläufe am PC und Handy durch, prüft Konsolenfehler, Überbreite, Barrierefreiheit (axe, WCAG AA), Tempo; schreibt `checkup-report/` mit Screenshots und dem visuellen **Bewertungsbogen** `checkup-report/index.html`.
- Läuft bei jedem Push auf `main` in GitHub Actions; schlägt ein Ablauf fehl, wird nicht veröffentlicht. Der Bogen liegt als Artefakt „bewertungsbogen“ am Workflow‑Lauf.
- `/checkup`: Claude führt den Check‑up aus, bewertet (Design‑Note, Stärken, Schwächen, max. 5 Empfehlungen in `review.json`) und schickt **immer** den visuell ausgearbeiteten Bewertungsbogen.
- Neue Funktionen bekommen einen Ablauf in `e2e/checkup.mjs`.

## Skills, die wir nutzen
- `frontend-design` / `impeccable`: bei neuen Seiten und größeren Design‑Änderungen.
- `design:accessibility-review`: nach jeder Phase kurz prüfen (Kontrast, Tastatur, Screenreader).
- `design:ux-copy`: Texte, Leer‑Zustände, Fehlermeldungen.
- `design:design-critique`: wenn NoFufu einen Screenshot zur Bewertung schickt.
- `code-review` und `simplify`: vor dem Merge einer Phase nach `main`.
- `run`: App starten und Screenshots machen.

## Aktueller Stand
- Fertig: Heute‑Dashboard (Hero „Als Nächstes“, Termine heute/Heute zu tun, Fortschritt mit Flächen‑Diagramm, Kategorien‑Halbkreise), Aufgaben (To‑dos und Termine nebeneinander; Eintrag mit Uhrzeit = Termin, umschaltbar), Detail‑Ansicht mit großer Termin‑Karte, Hinzufügen‑Dialog mit allen Optionen (Strg K / Neu / +), Kalender (Monat/Agenda, Doppelklick legt Eintrag an, To‑dos als Chips abhakbar), schlichte Notizen, Einkauf mit Links (Amazon) und Verlauf, Erfolge/Level/Streak.
- Sync über eigenen Server (Render + Neon) läuft; iPhone‑Widget über Scriptable mit selbst‑aktualisierendem Lader (`docs/widget/`), Start‑ und Sperrbildschirm.
- Design: Grouped Sidebar (nach Aceternity) mit Einklapp‑Knopf, Gecko‑App‑Icon und Logo (Gecko als „C“), Hell/Dunkel oben rechts.
- Zurückgestellt: Themes per Level, Fokus‑Modus, Maskottchen, native App.
- Erinnerungen: Feld „Erinnerung“ an Aufgaben/Terminen (`remind` Minuten vorher, ganztägig vor 09:00), `core::reminders_between`; im Browser `web/src/notify.rs` (Timer + Service Worker), Web Push vom Server (`server/src/push.rs`, VAPID-Schlüssel in der DB, Planer jede Minute). Kalender-Termine haben eine eigene Detail-Ansicht (`EventDetail`).
- Offene Empfehlungen aus dem Check-up: Kontrast `--muted`, Kalender-Tabs (ARIA).
- **Nächstes: Phasenplan in `docs/PLAN.md` (Abschnitt 5, „Neuer Plan“).** Checko ist für NoFufu persönlich; Schwerpunkt: Arbeit (Outlook/To Do) und Zuhause an einem Ort. Reihenfolge: Phase 7 Aufräumen & Backup → 8 Arbeit ↔ Zuhause → 9 Planen wie in To Do → 10 Kalender komplett → 11 Komfort → 12 KI.

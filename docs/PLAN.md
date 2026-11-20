# 🦀 Projekt „Checko“ – Persönlicher Organizer in Rust (Web‑First)

> Arbeitstitel. **Phase 1 = Webanwendung im Browser** (installierbar als PWA). Desktop/Mobile‑App mit Tauri kommt später – der Code ist so aufgebaut, dass wir ihn 1:1 weiterverwenden.

---

## 1. Vision
- **Eine Inbox für alles**: „Milch kaufen morgen 18 Uhr“ eintippen → die App sortiert ein.
- **Spielerisch, nicht kindisch**: XP, Streaks, Axolotl‑Maskottchen, befriedigende Micro‑Interactions.
- **Local‑first**: läuft offline im Browser, Daten bleiben bei dir; Export/Import als JSON & Markdown.

## Entscheidungen (Stand jetzt)
- **Name:** offen, **Checko** (Logo: grüner Gecko, dessen Körper ein Häkchen bildet).
- **Maskottchen:** rosa **Axolotl mit grünem Schal** (Sprite‑Sheet `assets/mascot/axolotl-sprites.webp`, einzelne Streifen pro Zustand in `assets/mascot/`).
- **Daten:** local‑first, aber **Sync PC ↔ Handy ist Pflicht** → eigener Rust‑Server (Axum) früh einplanen.
- **KI:** ja, als eigene Phase (Claude API, Schlüssel bleibt auf dem Server).
- **Reihenfolge:** erst Desktop‑Browser, dann Handy (responsive/PWA).

### Maskottchen‑Zustände (aus dem Sprite‑Sheet)
| Reihe | Bilder | Einsatz in der App |
|---|---|---|
| 1 Stehen/Lächeln | 6 | Ruhezustand, freut sich bei erledigter Aufgabe |
| 2 Laufen rechts | 8 | läuft über den XP‑Balken bei Fortschritt |
| 3 Laufen links | 8 | Seitenwechsel‑Animation |
| 4 Winken | 4 | Begrüßung morgens |
| 5 Kriechen/Schwimmen | 5 | Ladezustand |
| 6 Überrascht → zufrieden | 8 | überfällige Aufgabe / Erinnerung |
| 7 Blinzeln | 6 | Idle‑Animation |
| 8 Am Tablet | 6 | „arbeitet mit dir“ bei Aufgaben in Arbeit, KI denkt nach |
| 9 Tablet + Daumen hoch | 6 | Level‑Up / Achievement |

Umsetzung: Sprites einzeln freistellen (gleiche Rahmengröße), als PNG/WebP‑Streifen pro Zustand, Animation per CSS `steps()`.

## 2. Erkenntnisse aus den Quellen
### Personal‑Organizer‑Skill (mcpmarket)
- Wiederholungen (RRULE) und Zeitzonen sind **eigene, exakt geprüfte Felder** – nie raten.
- Jede Änderung ist ein klar begrenzter Schreibvorgang mit **Protokoll („Receipt“)** → wir bauen ein **Aktivitäts‑Log** und **Rückgängig**.

### Savvy‑Sloth (AI‑Task‑Management)
- Status als Spalten: **Geplant → In Arbeit → Erledigt** (mit Erledigt‑Datum).
- Pro Aufgabe ein **datierter Notiz‑Verlauf** („12.03.: Angebot angefragt“).
- **Morgen‑Plan** (Tagesliste: erst „in Arbeit“, dann Top‑Prioritäten) und **Abend‑Check‑in** („Was hast du geschafft?“), dazu ein **Wochenrückblick**.
- Kategorien/Bereiche (z. B. Arbeit, Privat, Gesundheit), Querverweise zwischen Aufgaben.
- Lehre: Systeme sterben, wenn man sie nicht füttert → **Capture muss in 2 Sekunden gehen**, und die App soll freundlich erinnern (genau hier hilft Gamification).

### Claude Academy (Dateien nach Inhalt ordnen)
- Inhaltsbasiertes Einsortieren mit **Vorher/Nachher‑Zusammenfassung**.
- **Bestätigung vor dem Löschen**, Papierkorb statt endgültig löschen.
- „Wo ist X hin?“ → **globale Suche** über alle Module.

### reactbits.dev & ui.aceternity.com (Design)
Beide sind React‑Bibliotheken (Tailwind, Motion, teils WebGL). Wir **bauen die passenden Effekte in Leptos + CSS nach**:

| Bereich | Vorbild‑Komponenten | Umsetzung bei uns |
|---|---|---|
| Hintergrund | Soft Aurora, Aurora Background, Dot Grid, Sparkles | CSS‑Gradient‑Blobs mit Blur + Animation; optional Canvas‑Dots |
| Navigation | Floating Dock, Pill Nav, Gooey Nav | Dock mit Vergrößerung beim Hover (Desktop), Bottom‑Bar (Handy) |
| Dashboard | Bento Grid, Magic Bento, Card Spotlight, Glowing Effect | CSS‑Grid + Spotlight via CSS‑Variable `--x/--y` aus Mausposition |
| Aufgaben | Animated List, Spring Check, Swipe Row, Swipe Toast / Fuse Button | Einfliegen der Zeilen, federndes Häkchen, Wischen zum Erledigen, Undo‑Toast |
| Eingabe | Placeholders And Vanish Input, Command‑Palette | Quick‑Add‑Feld mit wechselnden Beispiel‑Platzhaltern, „verpufft“ beim Absenden |
| Kalender | Timeline, Animated Modal, Animated Tooltip | Agenda‑Timeline, Termin‑Dialog mit Spring‑Animation |
| Notizen | Folder / Folder Float, Card Stack, Tilted Card | Notiz‑Karten mit leichtem Tilt, Ordner mit herausspringenden Notizen |
| Einkauf | Shredder, Glass Icons, Spring Check | Kategorie‑Icons, „Erledigtes wegschreddern“ |
| Text | Split Text, Blur Text, Rotating Text, Count Up, Counter | Begrüßung mit Blur‑Reveal, XP‑Zähler als Odometer |
| Belohnung | Click Spark, Sparkles | Konfetti/Funken beim Abhaken, Level‑Up‑Moment |

> Regel: Effekte **dezent** hinter dichten Listen, `prefers-reduced-motion` respektieren, Effekte im „Fokus‑Modus“ abschaltbar.

## 3. Tech‑Stack (Web‑First)
| Schicht | Wahl |
|---|---|
| UI | **Leptos 0.7** (CSR, Rust → WebAssembly) + `leptos_router` |
| Build/Dev‑Server | **Trunk** (`trunk serve`) |
| Styling | **Tailwind CSS v4** + eigene `animations.css` |
| Speicher | Browser‑lokal: zuerst `localStorage` (gloo‑storage), dann **IndexedDB** (`rexie`) hinter einem `Storage`‑Trait |
| Kernlogik | Crate `core` (pure Rust, getestet): Modelle, Quick‑Add‑Parser, XP, Wiederholungen |
| Zeit | `chrono`, später `rrule` |
| Offline/Installierbar | PWA: Manifest + Service Worker |
| Später | Tauri 2 (Desktop/Mobile) · Axum‑Sync‑Server · optional Claude‑KI |

### Struktur
```
checko/
├── Cargo.toml          # Workspace
├── crates/
│   ├── core/           # Modelle, Parser, Gamification (+ Tests)
│   └── web/            # Leptos-App: Komponenten, Seiten, Storage
│       ├── index.html
│       ├── style/      # tailwind.css, animations.css
│       └── src/
└── assets/             # Icons, Maskottchen-SVGs, Sounds
```

## 4. Datenmodell (Kern)
- `Task { id, title, notes, status(Planned|InProgress|Done), priority(1–4), area, tags, due, remind_at, recurrence, subtasks, log: Vec<LogEntry>, done_at, created_at, updated_at, deleted_at }`
- `Event { id, title, start, end, all_day, recurrence, color }`
- `Note { id, title, body_md, pinned, color, tags, links }`
- `ShoppingItem { id, name, qty, category, checked, times_bought }`
- `Profile { xp, level, streak, last_active, achievements, theme }`
- IDs: UUID v7 (sync‑fähig für später), Soft‑Delete → Papierkorb.

## 5. Roadmap
| Phase | Inhalt | Ergebnis |
|---|---|---|
| ✅ **0 – Setup** | Workspace, Leptos + Trunk + Tailwind, Aurora‑Hintergrund, Dock‑Navigation, leere Seiten | `trunk serve` zeigt die App |
| ✅ **1 – To‑Do** | Aufgaben anlegen/erledigen/löschen, Prioritäten, Bereiche, Spring‑Check, Undo, localStorage | Erste nutzbare App |
| ✅ **2 – Quick‑Add & Heute** | Parser („morgen 14:00 #arbeit !1“), Strg+K‑Fenster, Heute‑Dashboard (Morgen‑Plan/Abend‑Check‑in folgen später) | Tägliche Nutzung möglich |
| **3 – Einkauf & Notizen** | Einkaufsliste mit Kategorien, Markdown‑Notizen, globale Suche | Alle Hauptmodule |
| **4 – Gamification** | XP, Level, Streaks, Achievements, Axolotl‑Reaktionen, Konfetti, Themes | Spielerisch 🎉 |
| **5 – Sync‑Server** | Axum + SQLite, Login, Änderungs‑Sync (UUID v7, `updated_at`, Soft‑Delete), Hosting (z. B. kleiner VPS/Fly.io) | PC‑Eingabe sofort am Handy |
| **6 – Kalender & Erinnerungen** | Monat/Woche/Agenda, Wiederholungen, Benachrichtigungen (Web Push) | Kalender komplett |
| **7 – Handy & PWA** | Mobile Layout, Gesten, installierbar, offline mit IndexedDB | App‑Gefühl am Handy |
| **8 – KI‑Assistent** | Claude über den Server: natürliche Sprache, Morgen‑Briefing, Auto‑Kategorien, Wochenrückblick | „Sag's einfach“ |
| Später | Tauri‑Desktop/Mobile‑App, geteilte Einkaufslisten | |

## 6. Qualität
- `cargo fmt`, `cargo clippy -D warnings`, `cargo test` (core) bei jedem Schritt.
- Playwright‑Screenshots zur Sichtprüfung der UI.
- Git‑Repo auf GitHub, kleine Commits.

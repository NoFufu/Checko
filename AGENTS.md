# Übergabe an Codex – Checko

> Codex liest diese Datei automatisch. Sie legt fest, **wer was im Projekt bearbeitet**, damit sich Codex und Claude nicht gegenseitig den Code überschreiben.

## Rollen

| Wer | Verantwortlich für |
|---|---|
| **Claude** | Code und Logik: Datenmodelle, Speicherung, Zustand, Routing, Funktionen der Seiten, Sync, KI, Build und CI |
| **Codex** | Optik: CSS und Animationen, Icons, Grafiken, Farben, Abstände, rein visuelle Komponenten |

Der Besitzer des Projekts (NoFufu) entscheidet bei Unklarheiten. Im Zweifel: **nicht ändern, sondern im Pull Request nachfragen.**

## Was Codex ändern darf ✅

- `crates/web/style/**` (Tailwind‑Eingabe, eigene CSS‑Klassen, Animationen)
- `assets/**` (Logo, Icons, Maskottchen‑Bilder)
- Rein visuelle Komponenten:
  - `crates/web/src/components/icon.rs`
  - `crates/web/src/components/aurora.rs`
  - `crates/web/src/components/blur_text.rs`
  - `crates/web/src/components/spotlight_card.rs`
  - `crates/web/src/components/mascot.rs` (nur Darstellung, die `Mood`‑Varianten bleiben)
- In allen anderen `.rs`‑Dateien **nur**:
  - Werte von `class=`‑Attributen
  - sichtbare Texte (Beschriftungen, Hinweise)
  - Icon‑Namen beim Aufruf von `<Icon …/>`
- In `crates/web/index.html`: Schriftarten, Meta‑Tags, Favicon

## Was Codex NICHT ändern darf ⛔

- `crates/core/**` (Modelle, XP/Level, Parser, Tests)
- Logik in `crates/web/src/**`: Signale, Event‑Handler (`on:…`), Speicherung, Routen, Funktionen, Props, Modulstruktur, neue oder gelöschte Seiten
- `crates/web/src/main.rs`
- `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`, `crates/web/Trunk.toml`
- `.github/**` (CI und Veröffentlichung auf GitHub Pages)
- `CLAUDE.md`, `docs/PLAN.md`, diese Datei `AGENTS.md`
- Keine neuen Abhängigkeiten (Crates oder npm‑Pakete)

Braucht eine Design‑Idee neue Logik (z. B. Wisch‑Geste, Drag & Drop, Konfetti beim Abhaken):
→ im Pull Request beschreiben oder in `docs/design-requests.md` eintragen. Claude baut die Logik, Codex stylt sie danach.

## CSS‑Klassen, auf die sich der Code verlässt

Diese Klassen dürfen **neu gestaltet, aber nicht umbenannt oder gelöscht** werden:

`card`, `spotlight`, `bento`, `bento-wide`, `quick-add`, `chip`, `page-title`, `text-muted`, `brand`, `blur-word`, `mascot`, `mascot-button`, `xp-bar`, `xp-fill`, `dock`, `dock-item`, `dock-tooltip`, `checko-icon`, `module-icon`, `aurora`, `blob`, `blob-green`, `blob-lavender`, `blob-peach`, `grain`

Die CSS‑Variablen `--x`/`--y` (Spotlight) und `--size`/`--frames`/`--dur` (Maskottchen) werden aus Rust gesetzt und müssen erhalten bleiben. Neue Klassen dürfen jederzeit dazukommen.

## Arbeitsablauf

1. **Vor dem Start** die offenen Branches prüfen. Gibt es einen `claude/*`‑ oder `phase-*`‑Branch, der dieselben Dateien ändert: warten oder nur CSS/Assets anfassen.
2. Immer auf einem eigenen Branch `codex/<thema>` arbeiten, nie direkt auf `main`.
3. Kleine Pull Requests mit **einem** Thema. Screenshots von vorher/nachher anhängen.
4. Vor dem Pull Request muss alles grün sein:
   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --target wasm32-unknown-unknown -- -D warnings
   cd crates/web && trunk build
   ```
5. Den eigenen Pull Request nicht selbst mergen. Das macht NoFufu.

## Design‑Vorgaben (Kurzfassung)

- Farben aus dem Logo: Grün `#65a982`, Lavendel `#a48cd9`, Pfirsich `#f4a587`, Tinte `#251e2e`. Als CSS‑Variablen in `crates/web/style/input.css`.
- Hell‑ und Dunkelmodus müssen beide funktionieren.
- Freundlich und rund: Glas‑Karten, Schrift Nunito.
- Animationen 150–300 ms mit Federwirkung. `prefers-reduced-motion` respektieren.
- Kontrast mindestens WCAG AA. Fokus‑Rahmen für die Tastaturbedienung nicht entfernen.
- Vorbilder: reactbits.dev und ui.aceternity.com, nachgebaut in CSS (kein React). Details stehen in `docs/PLAN.md`.

## Projektstand

- Phase 0 ist fertig: Grundgerüst, Heute‑Seite, Menüleiste, Platzhalter‑Seiten, Outline‑Icons.
- **Als Nächstes baut Claude Phase 1: Aufgaben** (anlegen, abhaken, speichern, XP). Während Phase 1 läuft, ändert Codex `crates/web/src/pages/tasks.rs` und `crates/web/src/pages/today.rs` bitte gar nicht.

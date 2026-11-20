# Übergabe an Codex – Checko

> Codex liest diese Datei automatisch. Sie legt fest, **wer was im Projekt bearbeitet**.

## Rollen

| Wer | Verantwortlich für |
|---|---|
| **Claude** | Das komplette Projekt: Code, Logik, Optik/CSS, Komponenten, Build, CI. Claude darf auch frühere Arbeit von Codex ändern oder ersetzen. |
| **Codex** | Nur **Grafiken**: Logo, Icons als Bilddateien, Illustrationen, Maskottchen‑Bilder |

Der Besitzer des Projekts (NoFufu) entscheidet bei Unklarheiten.

## Was Codex ändern darf ✅

- Nur Dateien unter `assets/**` (Bilder: SVG, PNG, WebP).
- Neue Grafiken dürfen dazukommen. Bestehende Dateinamen bitte nicht umbenennen oder löschen, weil der Code sie lädt:
  - `assets/brand/logo-light.webp` (dunkle Schrift, für den hellen Modus), `assets/brand/logo-dark.webp` (helle Schrift, für den Dunkelmodus), `assets/brand/favicon.svg` (Browser‑Icon). Transparenter Hintergrund.
  - `assets/mascot/*.webp` (Sprite‑Streifen, je Bild 160 × 160 px pro Frame, nebeneinander)

## Was Codex NICHT ändern darf ⛔

Alles außerhalb von `assets/**`, insbesondere:

- `crates/**` (Rust‑Code, CSS in `crates/web/style/`, `index.html`)
- `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`, `Trunk.toml`
- `.github/**`, `docs/**`, `CLAUDE.md`, `README.md`, diese Datei

Soll eine neue Grafik irgendwo in der App erscheinen: Datei ablegen und im Pull Request beschreiben, wo sie hin soll. Claude baut sie dann ein.

## Arbeitsablauf

1. Immer auf einem eigenen Branch `codex/<thema>` arbeiten, nie direkt auf `main`.
2. Kleine Pull Requests mit einem Thema, mit Vorschaubild der Grafik.
3. Den eigenen Pull Request nicht selbst mergen. Das macht NoFufu.

## Stil der Grafiken

- Markenfarben aus dem Logo: Grün `#65a982`, Lavendel `#a48cd9`, Pfirsich `#f4a587`, Tinte `#251e2e`.
- Freundlich, rund, klare Formen. Die Grafiken müssen auf hellem und dunklem Hintergrund gut aussehen (transparenter Hintergrund).
- Icons in der App sind Linien‑Icons (24 × 24, Strichstärke 1,75, runde Enden).

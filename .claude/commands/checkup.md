---
description: App testen und einen visuellen Bewertungsbogen erstellen
---

Führe einen Check-up von Checko durch und liefere NoFufu einen visuell ausgearbeiteten **Bewertungsbogen**.

1. Release-Build erstellen: `cd crates/web && trunk build --release` (realistische Tempo-Werte).
2. Check-up starten: `npm run checkup` (im Repo-Wurzelordner). Das klickt alle Kernabläufe am PC und Handy durch, prüft Konsolenfehler, Überbreite, Barrierefreiheit (axe, WCAG AA) und Tempo und schreibt `checkup-report/report.json`, Screenshots und `checkup-report/index.html`.
3. `checkup-report/report.json` lesen. Screenshots **sparsam** ansehen: höchstens eine kleine Kontaktübersicht (4 Ansichten verkleinert zu einem Bild), keine einzelnen Vollbilder.
4. Einschätzung als `checkup-report/review.json` schreiben (Format oben in `e2e/report.mjs`): `summary`, `scores.design` (0–100), `strengths`, `weaknesses`, `recommendations` (max. 5, mit `priority` hoch/mittel/niedrig, konkret und umsetzbar), `notes` pro Bereich. Ehrlich bewerten, nichts schönreden.
5. Bogen neu bauen: `npm run checkup:report`.
6. `checkup-report/index.html` mit SendUserFile (display: render) schicken und auf Deutsch kurz zusammenfassen: Gesamtnote, die wichtigsten Probleme, Frage welche Empfehlungen umgesetzt werden sollen. Nichts ungefragt ändern.

Wenn ein Ablauf fehlschlägt: zuerst prüfen, ob die App oder der Test kaputt ist, und das im Bericht klar sagen.

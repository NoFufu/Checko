# Synchronisieren einrichten (PC ↔ Handy)

Einmalig, ca. 10 Minuten. Alles ist kostenlos (Gratis‑Stufen von Neon und Render).

## 1. Datenbank bei Neon anlegen

1. Auf **https://neon.tech** ein Konto anlegen (Anmeldung mit GitHub geht am schnellsten).
2. **Create project** → Name z. B. `checko`, Region **Europe (Frankfurt)** → **Create**.
3. Im Projekt auf **Connect** klicken und die **Connection string** kopieren.
   Sie sieht ungefähr so aus:
   `postgresql://benutzer:passwort@ep-irgendwas.eu-central-1.aws.neon.tech/neondb?sslmode=require`
4. Diese Adresse ist geheim – nirgendwo öffentlich einfügen, nur gleich bei Render.

## 2. Server bei Render starten

1. Auf **https://render.com** ein Konto anlegen (am besten mit GitHub anmelden).
2. Oben **New +** → **Blueprint**.
3. Das Repository **NoFufu/Checko** auswählen → Render findet die Datei `render.yaml`.
4. Bei **DATABASE_URL** die kopierte Neon‑Adresse einfügen → **Apply**.
5. Der erste Build dauert ca. 5–10 Minuten (Rust wird übersetzt). Danach steht oben die Adresse des Servers,
   z. B. `https://checko-server.onrender.com`. Diese Adresse kopieren.
6. Test: `https://checko-server.onrender.com/health` im Browser öffnen → dort steht `ok`.

## 3. Adresse bei GitHub hinterlegen

Damit Checko die Adresse kennt und der Server wach bleibt:

1. Im Repository **Settings** → **Secrets and variables** → **Actions** → Reiter **Variables** → **New repository variable**.
2. Name: `CHECKO_SERVER_URL`, Wert: die Render‑Adresse (ohne `/` am Ende) → **Add variable**.
3. Reiter **Actions** → **Deploy to GitHub Pages** → **Run workflow** (damit die App die Adresse übernimmt).

Ab jetzt pingt der Workflow **Keep sync server awake** den Server alle 10 Minuten an, damit er nicht einschläft.

## 4. In Checko anmelden

1. **Am PC:** Checko öffnen → **Einstellungen** → Server‑Adresse prüfen → Benutzername + Passwort (mind. 8 Zeichen) → **Konto erstellen**.
   Alles, was schon im Browser gespeichert ist, wird hochgeladen.
2. **Am Handy:** Checko öffnen → **Einstellungen** → dieselben Daten → **Anmelden**.
3. Fertig: Änderungen erscheinen nach wenigen Sekunden auf dem anderen Gerät
   (Checko gleicht nach jeder Änderung, alle 30 Sekunden und beim Wechsel ins Fenster ab).

## 5. Registrierung schließen (empfohlen)

Wenn dein Konto angelegt ist, kann sich sonst jeder mit der Server‑Adresse ein eigenes Konto anlegen
(er sieht deine Daten aber nicht). So schließt du die Registrierung:

Render → Dienst **checko-server** → **Environment** → `ALLOW_REGISTRATION` auf `false` → **Save** (Server startet neu).

## Gut zu wissen

- **Offline:** Checko funktioniert ohne Internet weiter. Sobald wieder Verbindung da ist, wird abgeglichen.
- **Konflikte:** Wird dieselbe Sache auf zwei Geräten geändert, gewinnt die neuere Änderung.
- **Gratis‑Grenzen:** Render‑Gratis‑Server können nach Pausen kurz brauchen; Neon‑Gratis hat 0,5 GB Speicher
  (reicht für sehr viele Aufgaben). GitHub pausiert geplante Workflows, wenn 60 Tage niemand etwas im Repository ändert –
  dann unter **Actions** den Workflow einmal wieder aktivieren.
- **Umzug:** Der Server ist ein normales Docker‑Programm. Er kann jederzeit auf einen eigenen Server umziehen;
  dafür nur `DATABASE_URL` mitnehmen.

// Checko – Widget für die iPhone-App "Scriptable"
// Zeigt: heute fällige Aufgaben, den nächsten Termin und deinen Streak.
// Tippen auf das Widget öffnet Checko.
// SERVER und TOKEN werden von Checko (Einstellungen → iPhone-Widget) eingesetzt.

const SERVER = "__SERVER__";
const TOKEN = "__TOKEN__";
const APP = "__APP__";
const CACHE_KEY = "checko-widget-cache";

const C = {
  white: new Color("#ffffff"),
  soft: new Color("#ffffff", 0.75),
  faint: new Color("#ffffff", 0.18),
};

async function load() {
  const tz = -new Date().getTimezoneOffset();
  const req = new Request(`${SERVER}/api/widget?tz=${tz}`);
  req.headers = { Authorization: `Bearer ${TOKEN}` };
  req.timeoutInterval = 60; // ein schlafender Gratis-Server braucht etwas
  return await req.loadJSON();
}

let data = null;
let offline = false;
try {
  data = await load();
  if (data && data.today) Keychain.set(CACHE_KEY, JSON.stringify(data));
  else throw new Error("leer");
} catch (e) {
  offline = true;
  if (Keychain.contains(CACHE_KEY)) data = JSON.parse(Keychain.get(CACHE_KEY));
}

const family = config.widgetFamily || "medium";
const w = new ListWidget();
w.url = APP;
w.setPadding(14, 14, 12, 14);
const g = new LinearGradient();
g.colors = [new Color("#5b3df5"), new Color("#6d4aff"), new Color("#1f9d63")];
g.locations = [0, 0.45, 1];
g.startPoint = new Point(0, 0);
g.endPoint = new Point(1, 1);
w.backgroundGradient = g;

function text(stack, value, size, color = C.white, weight = "regular") {
  const t = stack.addText(value);
  t.font = weight === "bold" ? Font.boldSystemFont(size) : weight === "semibold" ? Font.semiboldSystemFont(size) : Font.systemFont(size);
  t.textColor = color;
  t.lineLimit = 1;
  return t;
}

// Kopfzeile: checko ✓ · 🔥 Streak
const head = w.addStack();
head.centerAlignContent();
text(head, "checko", 15, C.white, "bold");
text(head, " ✓", 15, new Color("#4ade80"), "bold");
head.addSpacer();
if (data) text(head, `🔥 ${data.streak}`, 13, C.white, "semibold");
w.addSpacer(8);

if (!data) {
  text(w, "Keine Verbindung", 14, C.white, "semibold");
  text(w, "Tippen, um Checko zu öffnen", 11, C.soft);
} else {
  const max = family === "small" ? 3 : family === "large" ? 8 : 4;
  const body = w.addStack();
  body.layoutHorizontally();
  body.topAlignContent();

  // Heute
  const left = body.addStack();
  left.layoutVertically();
  text(left, `HEUTE · ${data.open_today}`, 10, C.soft, "bold");
  left.addSpacer(4);
  if (data.today.length === 0) text(left, "Alles erledigt 🎉", 13, C.white, "semibold");
  for (const it of data.today.slice(0, max)) {
    const row = left.addStack();
    row.centerAlignContent();
    text(row, "● ", 10, new Color(it.urgent ? "#ff6b6b" : it.color));
    text(row, it.title, 13, C.white, "semibold");
    if (it.when && family !== "small") text(row, `  ${it.when}`, 11, C.soft);
    left.addSpacer(2);
  }

  // Als Nächstes (nicht beim kleinen Widget)
  if (family !== "small") {
    body.addSpacer(12);
    const right = body.addStack();
    right.layoutVertically();
    right.size = new Size(family === "large" ? 140 : 120, 0);
    right.backgroundColor = C.faint;
    right.cornerRadius = 12;
    right.setPadding(8, 10, 8, 10);
    text(right, "ALS NÄCHSTES", 9, C.soft, "bold");
    right.addSpacer(3);
    if (data.next.length === 0) text(right, "Nichts geplant", 12, C.white, "semibold");
    for (const it of data.next.slice(0, family === "large" ? 3 : 2)) {
      text(right, it.title, 12, C.white, "semibold");
      text(right, it.when, 10, C.soft);
      right.addSpacer(3);
    }
  }
}

w.addSpacer();
const foot = w.addStack();
foot.addSpacer();
const stamp = new DateFormatter();
stamp.dateFormat = "HH:mm";
text(foot, offline ? `offline · ${stamp.string(new Date())}` : `aktualisiert ${stamp.string(new Date())}`, 8, C.soft);

w.refreshAfterDate = new Date(Date.now() + 15 * 60 * 1000);
if (config.runsInWidget) Script.setWidget(w);
else await w.presentMedium();
Script.complete();

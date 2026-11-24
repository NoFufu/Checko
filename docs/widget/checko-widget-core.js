// Checko – Widget-Design für Scriptable (wird vom kleinen Lade-Skript automatisch geholt).
// Startbildschirm (klein/mittel/groß) und Sperrbildschirm (rechteckig/rund/eine Zeile).
// Diese Datei liegt auf GitHub Pages; Änderungen hier kommen ohne neues Kopieren aufs Handy.

module.exports.run = async ({ server, token, app }) => {
const SERVER = server;
const TOKEN = token;
const APP = app;
const CACHE_KEY = "checko-widget-cache";

const WHITE = new Color("#ffffff");
const SOFT = new Color("#ffffff", 0.72);
const PANEL = new Color("#ffffff", 0.16);
const URGENT = new Color("#ff6b6b");

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

// ---------- lock screen (iOS 16+): monochrome, no background, system tint ----------
if (family.startsWith("accessory")) {
  const next = data && data.next && data.next[0];
  const firstToday = data && data.today && data.today[0];
  const font = (size, weight) =>
    weight === "bold" ? Font.boldSystemFont(size) : weight === "semibold" ? Font.semiboldSystemFont(size) : Font.systemFont(size);
  const line = (stack, value, size, weight = "regular", opacity = 1) => {
    const t = stack.addText(value);
    t.font = font(size, weight);
    t.textColor = new Color("#ffffff", opacity);
    t.lineLimit = 1;
    t.minimumScaleFactor = 0.6;
    return t;
  };
  if (family === "accessoryInline") {
    // one line next to the date above the clock
    const value = !data ? "Checko offline"
      : next ? `✓ ${next.title} · ${next.when}`
      : `✓ ${data.open_today} heute offen`;
    line(w, value, 12);
  } else if (family === "accessoryCircular") {
    w.addAccessoryWidgetBackground = true;
    w.addSpacer();
    const c = w.addStack(); c.addSpacer();
    line(c, data ? `${data.open_today}` : "–", 20, "bold");
    c.addSpacer();
    const l = w.addStack(); l.addSpacer();
    line(l, "heute", 9, "semibold", 0.8);
    l.addSpacer();
    w.addSpacer();
  } else {
    // accessoryRectangular: header, next appointment (or first task), when
    w.setPadding(0, 0, 0, 0);
    const head = w.addStack();
    head.centerAlignContent();
    line(head, "✓ CHECKO", 11, "bold", 0.85);
    head.addSpacer();
    if (data) line(head, `${data.open_today} heute · 🔥${data.streak}`, 11, "semibold", 0.85);
    w.addSpacer(2);
    if (!data) {
      line(w, "Keine Verbindung", 15, "semibold");
    } else if (next) {
      line(w, next.title, 16, "bold");
      line(w, next.when, 13, "regular", 0.85);
    } else if (firstToday) {
      line(w, firstToday.title, 16, "bold");
      line(w, firstToday.when ? `Heute · ${firstToday.when}` : "Heute", 13, "regular", 0.85);
    } else {
      line(w, "Alles erledigt 🎉", 16, "bold");
    }
  }
  w.refreshAfterDate = new Date(Date.now() + 15 * 60 * 1000);
  if (config.runsInWidget) Script.setWidget(w);
  else await w.presentAccessoryRectangular();
  Script.complete();
  return;
}

w.setPadding(16, 16, 14, 16);
const g = new LinearGradient();
g.colors = [new Color("#5b3df5"), new Color("#6d4aff"), new Color("#1f9d63")];
g.locations = [0, 0.5, 1];
g.startPoint = new Point(0, 0);
g.endPoint = new Point(1, 1);
w.backgroundGradient = g;

function text(stack, value, size, { color = WHITE, weight = "regular", lines = 1, scale = 1 } = {}) {
  const t = stack.addText(value);
  t.font =
    weight === "bold" ? Font.boldSystemFont(size)
    : weight === "semibold" ? Font.semiboldSystemFont(size)
    : weight === "medium" ? Font.mediumSystemFont(size)
    : Font.systemFont(size);
  t.textColor = color;
  t.lineLimit = lines;
  if (scale < 1) t.minimumScaleFactor = scale;
  return t;
}

function dot(stack, color) {
  const d = stack.addStack();
  d.size = new Size(8, 8);
  d.cornerRadius = 4;
  d.backgroundColor = color;
}

function pill(stack, value) {
  const p = stack.addStack();
  p.setPadding(2, 7, 2, 7);
  p.cornerRadius = 8;
  p.backgroundColor = PANEL;
  text(p, value, 11, { weight: "semibold" });
}

// ---------- header: checko ✓ · date · 🔥 streak ----------
const head = w.addStack();
head.centerAlignContent();
text(head, "checko", 16, { weight: "bold" });
text(head, " ✓", 16, { color: new Color("#4ade80"), weight: "bold" });
head.addSpacer();
if (data) {
  if (family !== "small") {
    text(head, data.date, 11, { color: SOFT, weight: "medium" });
    head.addSpacer(8);
  }
  pill(head, `🔥 ${data.streak}`);
}

if (!data) {
  w.addSpacer();
  text(w, "Keine Verbindung", 15, { weight: "semibold" });
  w.addSpacer(2);
  text(w, "Tippen, um Checko zu öffnen", 12, { color: SOFT });
  w.addSpacer();
} else if (family === "small") {
  // ---------- small: big count + two tasks ----------
  w.addSpacer(10);
  const count = w.addStack();
  count.bottomAlignContent();
  text(count, `${data.open_today}`, 30, { weight: "bold" });
  count.addSpacer(5);
  text(count, data.open_today === 1 ? "Aufgabe heute" : "Aufgaben heute", 11, { color: SOFT, weight: "medium" });
  w.addSpacer(8);
  if (data.today.length === 0) text(w, "Alles erledigt 🎉", 13, { weight: "semibold" });
  for (const it of data.today.slice(0, 2)) {
    const row = w.addStack();
    row.centerAlignContent();
    dot(row, it.urgent ? URGENT : new Color(it.color));
    row.addSpacer(6);
    text(row, it.title, 12, { weight: "semibold", scale: 0.8 });
    w.addSpacer(4);
  }
  w.addSpacer();
} else {
  // ---------- medium / large: Heute | Als Nächstes ----------
  w.addSpacer(family === "large" ? 14 : 10);
  const max = family === "large" ? 7 : 3;
  const body = w.addStack();
  body.layoutHorizontally();
  body.topAlignContent();

  const left = body.addStack();
  left.layoutVertically();
  text(left, `HEUTE · ${data.open_today}`, 10, { color: SOFT, weight: "bold" });
  left.addSpacer(7);
  if (data.today.length === 0) text(left, "Alles erledigt 🎉", 14, { weight: "semibold" });
  for (const it of data.today.slice(0, max)) {
    const row = left.addStack();
    row.centerAlignContent();
    dot(row, it.urgent ? URGENT : new Color(it.color));
    row.addSpacer(7);
    text(row, it.title, 13, { weight: "semibold", scale: 0.75 });
    if (it.when) {
      row.addSpacer(6);
      row.addSpacer();
      text(row, it.when, 11, { color: SOFT, weight: "medium" });
    }
    left.addSpacer(7);
  }
  const more = data.open_today - Math.min(data.today.length, max);
  if (more > 0) text(left, `+ ${more} weitere`, 11, { color: SOFT, weight: "medium" });

  body.addSpacer(12);

  const right = body.addStack();
  right.layoutVertically();
  right.size = new Size(family === "large" ? 140 : 122, 0);
  right.backgroundColor = PANEL;
  right.cornerRadius = 14;
  right.setPadding(10, 11, 10, 11);
  text(right, "ALS NÄCHSTES", 9, { color: SOFT, weight: "bold" });
  right.addSpacer(6);
  const next = data.next.slice(0, family === "large" ? 3 : 2);
  if (next.length === 0) text(right, "Nichts geplant", 13, { weight: "semibold" });
  next.forEach((it, i) => {
    if (i > 0) right.addSpacer(8);
    text(right, it.title, i === 0 ? 14 : 12, { weight: "semibold", lines: i === 0 ? 2 : 1, scale: 0.8 });
    right.addSpacer(2);
    text(right, it.when, 11, { color: SOFT, weight: "medium" });
  });
  right.addSpacer();
  w.addSpacer();
}

if (offline && data) {
  const foot = w.addStack();
  foot.addSpacer();
  text(foot, "offline – letzter Stand", 9, { color: SOFT });
}

w.refreshAfterDate = new Date(Date.now() + 15 * 60 * 1000);
if (config.runsInWidget) Script.setWidget(w);
else await (family === "small" ? w.presentSmall() : family === "large" ? w.presentLarge() : w.presentMedium());
Script.complete();
};

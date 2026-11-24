// Checko – Widget für die iPhone-App "Scriptable"
// Dieses kleine Skript lädt das aktuelle Widget-Design automatisch von Checko,
// du musst es also nur einmal einrichten. Funktioniert auf Start- und Sperrbildschirm.
// SERVER und TOKEN hat Checko (Einstellungen → iPhone-Widget) eingesetzt – nicht weitergeben.

const SERVER = "__SERVER__";
const TOKEN = "__TOKEN__";
const APP = "__APP__";

const fm = FileManager.local();
const file = fm.joinPath(fm.documentsDirectory(), "checko-widget-core.js");
try {
  const req = new Request(`${APP}checko-widget-core.js`);
  req.timeoutInterval = 20;
  const code = await req.loadString();
  if (code.includes("module.exports.run")) fm.writeString(file, code);
} catch (e) {
  // offline: use the last downloaded design
}

if (fm.fileExists(file)) {
  await importModule(file).run({ server: SERVER, token: TOKEN, app: APP });
} else {
  const w = new ListWidget();
  w.addText("Checko: einmal mit Internet öffnen");
  Script.setWidget(w);
  Script.complete();
}

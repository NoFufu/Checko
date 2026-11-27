// Checko check-up: drives the built app in a real browser, checks the main
// flows, console errors, layout overflow, accessibility (axe-core) and speed,
// takes screenshots and writes checkup-report/report.json + the evaluation
// sheet (index.html, see report.mjs).
//
// Usage:  (cd crates/web && trunk build)  then  npm run checkup
// Env:    CHECKUP_DIST  (default crates/web/dist)
//         CHECKUP_BASE  (default "" – set to "/Checko" for the Pages build)
//
// Test tooling only – the app itself stays 100 % Rust.

import { chromium, devices } from 'playwright';
import { createServer } from 'node:http';
import { readFile, mkdir, rm, writeFile, stat } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { join, extname, resolve } from 'node:path';
import { execSync } from 'node:child_process';
import { createRequire } from 'node:module';
import { buildReport } from './report.mjs';

const require = createRequire(import.meta.url);
const ROOT = resolve(new URL('..', import.meta.url).pathname);
const DIST = resolve(ROOT, process.env.CHECKUP_DIST || 'crates/web/dist');
const BASE = (process.env.CHECKUP_BASE || '').replace(/\/$/, '');
const OUT = join(ROOT, 'checkup-report');
const PORT = 8099;
const ORIGIN = `http://127.0.0.1:${PORT}`;
const APP = `${ORIGIN}${BASE}/`;

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm', '.css': 'text/css', '.webp': 'image/webp', '.png': 'image/png', '.svg': 'image/svg+xml', '.json': 'application/json', '.webmanifest': 'application/manifest+json' };

// --- static server with SPA fallback (like GitHub Pages' 404.html) ---------
function serve() {
  const server = createServer(async (req, res) => {
    let path = decodeURIComponent(new URL(req.url, ORIGIN).pathname);
    if (BASE && path.startsWith(BASE)) path = path.slice(BASE.length) || '/';
    let file = join(DIST, path);
    try {
      if (!file.startsWith(DIST) || !(await stat(file)).isFile()) throw 0;
    } catch {
      file = join(DIST, 'index.html');
    }
    const body = await readFile(file);
    res.writeHead(200, { 'content-type': TYPES[extname(file)] || 'application/octet-stream' });
    res.end(body);
  });
  return new Promise((ok) => server.listen(PORT, '127.0.0.1', () => ok(server)));
}

// --- helpers ---------------------------------------------------------------
const results = { flows: [], pages: [], a11y: {}, consoleErrors: [], perf: {}, meta: {} };
const AXE = require.resolve('axe-core/axe.min.js');

async function dismissPopups(page) {
  // achievement celebrations ("Erfolg freigeschaltet") cover the middle
  for (let i = 0; i < 3; i++) {
    const btn = page.locator('.celebration-card button');
    if (await btn.count()) { await btn.first().click().catch(() => {}); await page.waitForTimeout(250); } else break;
  }
}

async function nav(page, label) {
  await dismissPopups(page);
  await page.locator(`.nav-item:has-text("${label}")`).first().click();
  await page.waitForTimeout(400);
}

async function flow(name, fn) {
  const t0 = Date.now();
  try {
    await fn();
    results.flows.push({ name, ok: true, ms: Date.now() - t0 });
    console.log(`  ✓ ${name}`);
  } catch (e) {
    results.flows.push({ name, ok: false, ms: Date.now() - t0, error: String(e.message || e).split('\n')[0].slice(0, 240) });
    console.log(`  ✗ ${name}: ${String(e.message || e).split('\n')[0]}`);
  }
}

function expect(cond, msg) { if (!cond) throw new Error(msg); }

async function newPage(browser, opts, tag) {
  const ctx = await browser.newContext({ locale: 'de-DE', timezoneId: 'Europe/Berlin', ...opts });
  const page = await ctx.newPage();
  page.on('console', (m) => { if (m.type() === 'error') results.consoleErrors.push({ where: tag, text: m.text().slice(0, 300) }); });
  page.on('pageerror', (e) => results.consoleErrors.push({ where: tag, text: String(e.message).slice(0, 300) }));
  page.setDefaultTimeout(8000);
  return { ctx, page };
}

async function auditPage(page, { route, label, device, scheme }) {
  await page.waitForTimeout(350);
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
  const shot = `${device}-${scheme}-${route.replace(/\W+/g, '') || 'heute'}.jpg`;
  await page.screenshot({ path: join(OUT, shot), type: 'jpeg', quality: 72 });
  await page.addScriptTag({ path: AXE });
  const axe = await page.evaluate(async () => {
    const r = await window.axe.run(document, { runOnly: { type: 'tag', values: ['wcag2a', 'wcag2aa'] } });
    return r.violations.map((v) => ({ id: v.id, impact: v.impact, help: v.help, url: v.helpUrl, nodes: v.nodes.length, examples: v.nodes.slice(0, 3).map((n) => ({ target: n.target.join(' '), html: n.html.slice(0, 140), why: (n.failureSummary || '').split('\n').slice(1, 2).join(' ').trim() })) }));
  });
  for (const v of axe) {
    const a = (results.a11y[v.id] ||= { ...v, pages: [] });
    a.pages.push(`${label} (${device}, ${scheme})`);
    a.nodes = Math.max(a.nodes, v.nodes);
    for (const ex of v.examples || []) if (a.examples.length < 8 && !a.examples.some((x) => x.html === ex.html)) a.examples.push(ex);
  }
  results.pages.push({ route, label, device, scheme, overflow: Math.max(0, overflow), shot, axe: axe.length });
}

// --- main ------------------------------------------------------------------
async function main() {
  expect(existsSync(join(DIST, 'index.html')), `Kein Build in ${DIST} – erst "trunk build" ausführen.`);
  await rm(OUT, { recursive: true, force: true });
  await mkdir(OUT, { recursive: true });
  const server = await serve();
  const browser = await chromium.launch();
  try {
    results.meta = {
      date: new Date().toISOString(),
      commit: (() => { try { return execSync('git rev-parse --short HEAD', { cwd: ROOT }).toString().trim(); } catch { return '–'; } })(),
      base: BASE || '/',
    };

    // ---------- desktop flows ----------
    console.log('Abläufe (PC):');
    const { ctx, page } = await newPage(browser, { viewport: { width: 1440, height: 900 }, colorScheme: 'light' }, 'PC');

    await flow('App startet', async () => {
      await page.goto(APP, { waitUntil: 'load' });
      await page.waitForSelector('.app-shell', { timeout: 15000 });
      results.perf = await page.evaluate(async () => {
        const nav = performance.getEntriesByType('navigation')[0];
        const lcp = await new Promise((ok) => {
          let v = 0;
          new PerformanceObserver((l) => { for (const e of l.getEntries()) v = e.startTime; }).observe({ type: 'largest-contentful-paint', buffered: true });
          setTimeout(() => ok(v), 600);
        });
        const res = performance.getEntriesByType('resource');
        const size = (re) => res.filter((r) => re.test(r.name)).reduce((s, r) => s + (r.encodedBodySize || r.transferSize || 0), 0);
        return { domReady: Math.round(nav.domContentLoadedEventEnd), load: Math.round(nav.loadEventEnd), lcp: Math.round(lcp), wasm: size(/\.wasm/), js: size(/\.js($|\?)/), css: size(/\.css/) };
      });
    });

    await flow('Schnell-Eingabe erkennt Datum, Uhrzeit und Kategorie', async () => {
      await page.keyboard.press('Control+k');
      await page.locator('.dialog input[aria-label="Neue Aufgabe"]').fill('Zahnarzt morgen 14:00 #Gesundheit');
      await page.keyboard.press('Enter');
      await page.waitForTimeout(300);
      await nav(page, 'Aufgaben');
      const row = page.locator('.task-row', { hasText: 'Zahnarzt' });
      await row.waitFor();
      expect((await row.locator('.appt-time').innerText()).includes('14:00'), 'Uhrzeit 14:00 fehlt im Termin-Block');
      expect(await row.locator('.cat-chip, .chip, [class*="cat"]').filter({ hasText: 'Gesundheit' }).count(), 'Kategorie Gesundheit fehlt');
    });

    await flow('To-do anlegen und abhaken', async () => {
      const input = page.locator('.page input[aria-label="Neue Aufgabe"]').first();
      await input.fill('Wäsche machen');
      await input.press('Enter');
      const row = page.locator('.task-row', { hasText: 'Wäsche machen' });
      await row.waitFor();
      await row.locator('.check').click();
      await page.waitForTimeout(900);
      await dismissPopups(page);
      await page.locator('.segment', { hasText: 'Erledigt' }).click();
      await page.locator('.task-row', { hasText: 'Wäsche machen' }).waitFor();
      await page.locator('.segment', { hasText: 'Offen' }).click();
    });

    await flow('Details öffnen und Termin ↔ To-do umschalten', async () => {
      await page.locator('.task-row', { hasText: 'Zahnarzt' }).click();
      await page.locator('.drawer').waitFor();
      expect(await page.locator('.when-big').innerText() !== '', 'Termin-Karte leer');
      await page.locator('.seg-mini button', { hasText: 'To-do' }).click();
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
      expect(!(await page.locator('.task-row', { hasText: 'Zahnarzt' }).locator('.appt-time').count()), 'Ist nach Umschalten noch ein Termin');
    });

    await flow('Geplant: Aufgaben nach Datum gruppiert', async () => {
      await nav(page, 'Aufgaben');
      const input = page.locator('.task-form input').first();
      await input.fill('Checkup geplant übermorgen');
      await input.press('Enter');
      await page.waitForTimeout(300);
      await page.locator('.segment', { hasText: 'Geplant' }).click();
      const group = page.locator('.plan-group', { hasText: 'Checkup geplant' });
      await group.waitFor();
      const head = await group.locator('.list-heading').innerText();
      expect(/Diese Woche|Nächste Woche/.test(head), `falsche Gruppe: ${head}`);
      await page.locator('.segment', { hasText: 'Offen' }).click();
    });

    await flow('Mein Tag: Vorschlag übernehmen', async () => {
      await nav(page, 'Aufgaben');
      const input = page.locator('.task-form input').first();
      await input.fill('Checkup mein Tag heute');
      await input.press('Enter');
      await page.waitForTimeout(300);
      await page.locator('.segment', { hasText: 'Mein Tag' }).click();
      const row = page.locator('.suggest-row', { hasText: 'Checkup mein Tag' });
      await row.waitFor();
      await row.locator('.btn-icon').click();
      await page.locator('.my-day .task-list', { hasText: 'Checkup mein Tag' }).waitFor();
      expect(!(await row.count()), 'Vorschlag nach Übernahme noch da');
      await page.locator('.segment', { hasText: 'Offen' }).click();
    });

    await flow('Kalender: Doppelklick legt Eintrag an', async () => {
      await nav(page, 'Kalender');
      await page.locator('.cal-day').nth(17).dblclick();
      const input = page.locator('.dialog input[aria-label="Neue Aufgabe"]');
      await input.waitFor();
      expect(await page.locator('.add-options input[type="date"]').inputValue() !== '', 'Datum nicht vorbelegt');
      await input.fill('Team-Meeting 10:00');
      await page.keyboard.press('Enter');
      await page.locator('.cal-chip', { hasText: 'Team-Meeting' }).waitFor();
    });

    await flow('Kalender: Termin anklicken, bearbeiten und Erinnerung setzen', async () => {
      const form = page.locator('.cal-form input[aria-label="Termin"]');
      await form.fill('Elternabend in der Grundschule am Marktplatz 18:00');
      await page.locator('.cal-form button[type="submit"]').click();
      const chip = page.locator('.cal-chip', { hasText: 'Elternabend' }).first();
      await chip.waitFor();
      await chip.click();
      const drawer = page.locator('.drawer[aria-label="Termin bearbeiten"]');
      await drawer.waitFor();
      await drawer.locator('textarea.drawer-title').fill('Elternabend Grundschule');
      await drawer.locator('textarea.drawer-title').press('Enter');
      await drawer.locator('select[aria-label="Erinnerung"]').selectOption('15');
      await page.keyboard.press('Escape');
      await page.locator('.cal-chip', { hasText: 'Elternabend Grundschule' }).first().waitFor();
      const remind = await page.evaluate(() => JSON.parse(localStorage.getItem('checko.events.v1') || '[]').find((e) => e.title === 'Elternabend Grundschule')?.remind);
      expect(remind === 15, `Erinnerung nicht gespeichert (${remind})`);
    });

    await flow('Kalender-Datei (.ics) importieren, erneuter Import ohne Doppelungen', async () => {
      await page.locator('.cal-import summary').click();
      const ics = ['BEGIN:VCALENDAR','VERSION:2.0','BEGIN:VEVENT','UID:checkup-1','SUMMARY:Outlook Teammeeting',
        'DTSTART;TZID=W. Europe Standard Time:' + new Date().toISOString().slice(0,10).replace(/-/g,'') + 'T100000',
        'DTEND;TZID=W. Europe Standard Time:' + new Date().toISOString().slice(0,10).replace(/-/g,'') + 'T110000',
        'END:VEVENT','END:VCALENDAR'].join('\r\n');
      const file = { name: 'outlook.ics', mimeType: 'text/calendar', buffer: Buffer.from(ics) };
      await page.locator('.cal-import input[type="file"]').setInputFiles(file);
      const res = page.locator('.cal-import .ics-result');
      await res.waitFor();
      expect((await res.innerText()).includes('1 neu'), `Import-Meldung: ${await res.innerText()}`);
      await page.locator('.cal-import input[type="file"]').setInputFiles(file);
      await page.waitForTimeout(400);
      const n = await page.evaluate(() => JSON.parse(localStorage.getItem('checko.events.v1') || '[]').filter((e) => e.title === 'Outlook Teammeeting').length);
      expect(n === 1, `Termin ${n}× vorhanden`);
    });

    await flow('Outlook-Datei (.pst): ungültige Datei wird freundlich abgelehnt', async () => {
      await page.locator('.cal-import input[type="file"]').setInputFiles({ name: 'kaputt.pst', mimeType: 'application/octet-stream', buffer: Buffer.from('keine pst') });
      const res = page.locator('.cal-import .ics-result');
      await page.waitForFunction(() => /gültige Outlook/.test(document.querySelector('.cal-import .ics-result')?.textContent || ''), null, { timeout: 5000 });
      expect((await res.innerText()).includes('Outlook'), `Meldung: ${await res.innerText()}`);
    });

    await flow('Einkauf: Amazon-Link wird zum Eintrag mit Namen', async () => {
      await nav(page, 'Einkauf');
      const input = page.locator('input[aria-label="Artikel hinzufügen"]');
      await input.fill('https://www.amazon.de/Sony-WH-1000XM5-Kopfh%C3%B6rer/dp/B09Y2MYL5C/ref=sr_1_1');
      await input.press('Enter');
      const row = page.locator('.shop-row', { hasText: 'Sony WH 1000XM5 Kopfhörer' });
      await row.waitFor();
      expect(await row.locator('.shop-link').count(), 'Link-Knopf fehlt');
      await row.locator('.check').click();
      await page.locator('.segment', { hasText: 'Verlauf' }).click();
      await page.locator('.history-row', { hasText: 'Sony' }).waitFor();
      await page.locator('.segment', { hasText: 'Liste' }).click();
    });

    await flow('Notiz schreiben bleibt gespeichert', async () => {
      await nav(page, 'Notizen');
      await page.locator('button', { hasText: 'Neue Notiz' }).click();
      await page.locator('.note-body').click();
      await page.keyboard.type('Check-up Notiz');
      await nav(page, 'Heute');
      await nav(page, 'Notizen');
      expect((await page.locator('.note-item').first().innerText()).includes('Check-up Notiz'), 'Notiz nicht gespeichert');
    });

    await flow('Kategorie umbenennen, Emoji ändern und Fokus-Modus', async () => {
      await dismissPopups(page); await page.locator('.topbar a[aria-label="Einstellungen"]').first().click(); await page.waitForTimeout(400);
      await page.waitForTimeout(400);
      const name = page.locator('.cat-name-input[id="cat-Privat"]');
      await name.fill('Zuhause');
      await name.press('Enter');
      await page.locator('.cat-name-input[id="cat-Zuhause"]').waitFor();
      const emoji = page.locator('.cat-manage-row', { has: page.locator('[id="cat-Zuhause"]') }).locator('.cat-emoji-input');
      await emoji.fill('🏡');
      await emoji.press('Tab');
      await page.locator('.folder-link', { hasText: 'Zuhause' }).locator('.cat-icon', { hasText: '🏡' }).waitFor();
      await page.locator('label.switch').click();
      await nav(page, 'Heute');
      await page.waitForTimeout(400);
      expect(!(await page.locator('.xp-strip').count()), 'Level trotz Fokus-Modus sichtbar');
      await dismissPopups(page); await page.locator('.topbar a[aria-label="Einstellungen"]').first().click(); await page.waitForTimeout(400);
      await page.locator('label.switch').click();
      await nav(page, 'Heute');
      await page.locator('.xp-strip').waitFor();
      await page.locator('.xp-strip').click();
      await page.locator('.progress-dialog .level-card').waitFor();
      await page.keyboard.press('Escape');
    });

    await flow('Kalender: über mehrere Tage ziehen öffnet Serientermin', async () => {
      await nav(page, 'Kalender');
      const days = page.locator('.cal-day');
      const a = await days.nth(15).boundingBox(); const c = await days.nth(17).boundingBox();
      await page.mouse.move(a.x + a.width / 2, a.y + a.height - 6); await page.mouse.down();
      await page.mouse.move(c.x + c.width / 2, c.y + c.height - 6, { steps: 6 }); await page.mouse.up();
      await page.locator('.dialog').waitFor({ timeout: 3000 });
      expect((await page.locator('.dialog').innerText()).includes('bis '), 'Serie bis … fehlt');
      await page.keyboard.press('Escape');
    });

    await flow('Kategorie per Ziehen in Überkategorie und wieder heraus', async () => {
      await dismissPopups(page);
      await page.locator('.topbar a[aria-label="Einstellungen"]').first().click(); await page.waitForTimeout(300);
      await page.locator('input[aria-label="Neue Überkategorie"]').fill('Checkup-Gruppe'); await page.keyboard.press('Enter');
      await page.locator('.folder-link', { hasText: 'Gesundheit' }).dragTo(page.locator('.cat-group-head', { hasText: 'Checkup-Gruppe' }));
      await page.locator('.cat-group .folder-link', { hasText: 'Gesundheit' }).waitFor({ timeout: 3000 });
      await page.locator('.cat-group .folder-link', { hasText: 'Gesundheit' }).dragTo(page.locator('.group-head').first());
      await page.waitForTimeout(300);
      expect(!(await page.locator('.cat-group .folder-link', { hasText: 'Gesundheit' }).count()), 'nicht wieder herausgezogen');
    });

    await flow('Hell/Dunkel umschalten', async () => {
      await page.locator('.topbar .theme-option[aria-label="Dunkel"]').click();
      expect(await page.evaluate(() => document.documentElement.dataset.theme) === 'dark', 'Dunkelmodus nicht aktiv');
      await page.locator('.topbar .theme-option[aria-label="Hell"]').click();
    });

    await flow('Seitenleiste ein- und ausklappen (bleibt gespeichert)', async () => {
      await page.hover('.sidebar');
      await page.locator('.sidebar-toggle').click();
      await page.waitForTimeout(450);
      expect(Math.round(await page.$eval('.sidebar', (e) => e.getBoundingClientRect().width)) === 70, 'eingeklappt nicht 70 px');
      await page.goto(APP); await page.waitForSelector('.app-shell');
      expect(Math.round(await page.$eval('.sidebar', (e) => e.getBoundingClientRect().width)) === 70, 'Zustand nicht gespeichert');
      await page.hover('.sidebar');
      await page.locator('.sidebar-toggle').click();
      await page.waitForTimeout(450);
    });

    // ---------- desktop pages: light + dark ----------
    const ROUTES = [['', 'Heute'], ['aufgaben', 'Aufgaben'], ['kalender', 'Kalender'], ['notizen', 'Notizen'], ['einkauf', 'Einkauf']];
    console.log('Seiten prüfen (PC, Handy · hell, dunkel) …');
    for (const scheme of ['light', 'dark']) {
      await page.emulateMedia({ colorScheme: scheme });
      await page.evaluate((s) => document.documentElement.setAttribute('data-theme', s), scheme);
      for (const [route, label] of ROUTES) {
        await nav(page, label);
        await auditPage(page, { route, label, device: 'pc', scheme });
      }
    }
    const state = await ctx.storageState();
    await ctx.close();

    // ---------- phone ----------
    for (const scheme of ['light', 'dark']) {
      const { ctx: pctx, page: phone } = await newPage(browser, { ...devices['iPhone 13'], colorScheme: scheme, storageState: state }, `Handy ${scheme}`);
      await phone.goto(APP); await phone.waitForSelector('.app-shell');
      await phone.evaluate((s) => document.documentElement.setAttribute('data-theme', s), scheme);
      if (scheme === 'light') {
        await flow('Handy: Tab-Leiste und +-Knopf', async () => {
          expect(await phone.locator('.sidebar .nav-drop .nav-item').first().isVisible(), 'Tab-Leiste nicht sichtbar');
          await phone.locator('.fab').click();
          await phone.locator('.dialog input[aria-label="Neue Aufgabe"]').waitFor();
          await phone.keyboard.press('Escape');
        });
      }
      for (const [route, label] of ROUTES) {
        await nav(phone, label);
        await auditPage(phone, { route, label, device: 'handy', scheme });
      }
      await pctx.close();
    }
  } finally {
    await browser.close();
    server.close();
  }

  await writeFile(join(OUT, 'report.json'), JSON.stringify(results, null, 2));
  const html = await buildReport(OUT);
  const failed = results.flows.filter((f) => !f.ok).length;
  console.log(`\nAbläufe: ${results.flows.length - failed}/${results.flows.length} ok · Konsolenfehler: ${results.consoleErrors.length} · Barrierefreiheit: ${Object.keys(results.a11y).length} Regelverstöße`);
  console.log(`Bewertungsbogen: ${html}`);
  if (failed) process.exitCode = 1;
}

main().catch((e) => { console.error(e); process.exit(1); });

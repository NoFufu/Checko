// Builds the evaluation sheet ("Bewertungsbogen") from checkup-report/report.json
// and, if present, Claude's review in checkup-report/review.json.
// Output: checkup-report/index.html – one self-contained page (images embedded).
//
// review.json (optional, written by /checkup):
// { "summary": "…", "scores": { "design": 84 },
//   "strengths": ["…"], "weaknesses": ["…"],
//   "recommendations": [{ "title": "…", "detail": "…", "priority": "hoch|mittel|niedrig" }],
//   "notes": { "funktion": "…", "design": "…" } }

import { readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { join, resolve } from 'node:path';

const ROOT = resolve(new URL('..', import.meta.url).pathname);

const esc = (s) => String(s ?? '').replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
const clamp = (n) => Math.max(0, Math.min(100, Math.round(n)));

export function grade(score) {
  if (score == null) return { n: '–', label: 'nicht bewertet' };
  if (score >= 92) return { n: '1', label: 'sehr gut' };
  if (score >= 81) return { n: '2', label: 'gut' };
  if (score >= 67) return { n: '3', label: 'befriedigend' };
  if (score >= 50) return { n: '4', label: 'ausreichend' };
  if (score >= 30) return { n: '5', label: 'mangelhaft' };
  return { n: '6', label: 'ungenügend' };
}

export function scoreAll(r, review) {
  const flows = r.flows || [];
  const ok = flows.filter((f) => f.ok).length;
  const phonePages = (r.pages || []).filter((p) => p.device === 'handy');
  const phoneFlowsFailed = flows.filter((f) => !f.ok && f.name.startsWith('Handy')).length;
  const weights = { critical: 15, serious: 8, moderate: 4, minor: 2 };
  const a11y = Object.values(r.a11y || {});
  const uniqueErrors = new Set((r.consoleErrors || []).map((e) => e.text)).size;
  const lcp = r.perf?.lcp || r.perf?.load || 0;
  let tempo = lcp <= 1500 ? 100 : lcp <= 2500 ? 85 : lcp <= 4000 ? 65 : 45;
  if ((r.perf?.wasm || 0) > 2.5e6) tempo -= 10;
  return {
    funktion: { label: 'Funktion', icon: '✓', score: flows.length ? clamp((ok / flows.length) * 100) : null, detail: `${ok} von ${flows.length} Abläufen klappen` },
    stabil: { label: 'Stabilität', icon: '◆', score: clamp(100 - uniqueErrors * 15), detail: uniqueErrors ? `${uniqueErrors} verschiedene Konsolenfehler` : 'keine Konsolenfehler' },
    handy: { label: 'Handy', icon: '▯', score: clamp(100 - phonePages.filter((p) => p.overflow > 1).length * 20 - phoneFlowsFailed * 25), detail: `${phonePages.filter((p) => p.overflow > 1).length} Seiten zu breit` },
    a11y: { label: 'Barrierefreiheit', icon: '◎', score: clamp(100 - a11y.reduce((s, v) => s + (weights[v.impact] || 2), 0)), detail: a11y.length ? `${a11y.length} Regelverstöße (WCAG AA)` : 'keine Verstöße gefunden' },
    tempo: { label: 'Tempo', icon: '⚡', score: clamp(tempo), detail: `größtes Element nach ${(lcp / 1000).toFixed(1)} s` },
    design: { label: 'Design', icon: '✦', score: review?.scores?.design ?? null, detail: review?.scores?.design != null ? 'Bewertung von Claude' : 'noch nicht bewertet (/checkup)' },
  };
}

const kb = (b) => (b > 1e6 ? `${(b / 1e6).toFixed(1)} MB` : `${Math.round(b / 1e3)} KB`);

export async function buildReport(dir = join(ROOT, 'checkup-report')) {
  const r = JSON.parse(await readFile(join(dir, 'report.json'), 'utf8'));
  const review = existsSync(join(dir, 'review.json')) ? JSON.parse(await readFile(join(dir, 'review.json'), 'utf8')) : null;
  const cats = scoreAll(r, review);
  const rated = Object.values(cats).filter((c) => c.score != null);
  const total = rated.length ? clamp(rated.reduce((s, c) => s + c.score, 0) / rated.length) : 0;
  const g = grade(total);
  const logo = existsSync(join(ROOT, 'assets/brand/logo-light.webp'))
    ? `data:image/webp;base64,${(await readFile(join(ROOT, 'assets/brand/logo-light.webp'))).toString('base64')}` : '';
  const img = async (f) => `data:image/jpeg;base64,${(await readFile(join(dir, f))).toString('base64')}`;
  const date = new Date(r.meta?.date || Date.now()).toLocaleString('de-DE', { dateStyle: 'long', timeStyle: 'short', timeZone: 'Europe/Berlin' });
  const color = (s) => (s == null ? 'var(--muted)' : s >= 81 ? 'var(--good)' : s >= 50 ? 'var(--warn)' : 'var(--bad)');
  const ring = (s, size = 132) => {
    const rr = 52, c = 2 * Math.PI * rr;
    return `<svg viewBox="0 0 120 120" width="${size}" height="${size}" aria-hidden="true"><circle cx="60" cy="60" r="${rr}" class="ring-track"/><circle cx="60" cy="60" r="${rr}" class="ring-bar" stroke="${color(s)}" stroke-dasharray="${(c * (s ?? 0)) / 100} ${c}" transform="rotate(-90 60 60)"/></svg>`;
  };

  const catCards = Object.entries(cats).map(([key, c]) => {
    const gg = grade(c.score);
    const note = review?.notes?.[key];
    return `<article class="cat">
      <header><span class="cat-icon">${c.icon}</span><h3>${esc(c.label)}</h3><span class="cat-grade" style="--c:${color(c.score)}">${gg.n}</span></header>
      <div class="bar"><span style="width:${c.score ?? 0}%;background:${color(c.score)}"></span></div>
      <p class="cat-score"><b>${c.score ?? '–'}</b>${c.score != null ? ' / 100' : ''} · ${esc(gg.label)}</p>
      <p class="cat-detail">${esc(c.detail)}</p>
      ${note ? `<p class="cat-note">${esc(note)}</p>` : ''}
    </article>`;
  }).join('');

  const flowRows = (r.flows || []).map((f) => `<tr class="${f.ok ? 'ok' : 'fail'}"><td><span class="pill ${f.ok ? 'pill-ok' : 'pill-bad'}">${f.ok ? '✓ ok' : '✗ Fehler'}</span></td><td>${esc(f.name)}${f.error ? `<div class="err">${esc(f.error)}</div>` : ''}</td><td class="num">${(f.ms / 1000).toFixed(1)} s</td></tr>`).join('');

  const ruleDe = {
    'color-contrast': 'Text hat zu wenig Kontrast zum Hintergrund',
    'aria-required-children': 'ARIA-Rolle enthält nicht erlaubte Kind-Elemente',
    'aria-required-parent': 'ARIA-Rolle steht ohne passendes Eltern-Element',
    'button-name': 'Knopf hat keinen lesbaren Namen',
    'link-name': 'Link hat keinen lesbaren Namen',
    'label': 'Eingabefeld hat keine Beschriftung',
    'image-alt': 'Bild hat keinen Alternativtext',
    'nested-interactive': 'Klickbare Elemente sind ineinander verschachtelt',
    'aria-allowed-attr': 'Nicht erlaubtes ARIA-Attribut',
    'aria-valid-attr-value': 'Ungültiger ARIA-Wert',
    'list': 'Liste enthält nicht erlaubte Elemente',
    'listitem': 'Listenpunkt steht außerhalb einer Liste',
    'select-name': 'Auswahlfeld hat keine Beschriftung',
    'scrollable-region-focusable': 'Scrollbarer Bereich ist per Tastatur nicht erreichbar',
  };
  const impactDe = { critical: 'kritisch', serious: 'ernst', moderate: 'mittel', minor: 'gering' };
  const a11yRows = Object.values(r.a11y || {}).sort((a, b) => ['critical', 'serious', 'moderate', 'minor'].indexOf(a.impact) - ['critical', 'serious', 'moderate', 'minor'].indexOf(b.impact))
    .map((v) => `<li><span class="pill imp-${v.impact}">${impactDe[v.impact] || v.impact}</span><div><b>${esc(ruleDe[v.id] || v.help)}</b><small>${esc(v.id)} · ${v.nodes} Stellen · ${esc([...new Set(v.pages.map((p) => p.split(' (')[0]))].join(', '))}</small>${(v.examples || []).map((x) => `<code class="ex">${esc(x.html)}</code>${x.why ? `<small>${esc(x.why)}</small>` : ''}`).join('')}</div></li>`).join('');

  const recs = (review?.recommendations || []).map((x, i) => `<li class="rec prio-${esc(x.priority || 'mittel')}"><span class="rec-n">${i + 1}</span><div><b>${esc(x.title)}</b><p>${esc(x.detail || '')}</p></div><span class="pill prio">${esc(x.priority || 'mittel')}</span></li>`).join('');
  const list = (arr) => (arr || []).map((s) => `<li>${esc(s)}</li>`).join('');

  const groups = [['pc', 'light', 'PC · hell'], ['pc', 'dark', 'PC · dunkel'], ['handy', 'light', 'Handy · hell'], ['handy', 'dark', 'Handy · dunkel']];
  let gallery = '';
  for (const [d, s, title] of groups) {
    const ps = (r.pages || []).filter((p) => p.device === d && p.scheme === s);
    if (!ps.length) continue;
    const shots = await Promise.all(ps.map(async (p) => `<figure class="${d}"><img loading="lazy" src="${await img(p.shot)}" alt="${esc(p.label)} – ${esc(title)}"><figcaption>${esc(p.label)}${p.overflow > 1 ? ' <span class="pill pill-bad">zu breit</span>' : ''}</figcaption></figure>`));
    gallery += `<h3 class="gal-title">${title}</h3><div class="gallery gallery-${d}">${shots.join('')}</div>`;
  }

  const p = r.perf || {};
  const html = `<!doctype html>
<html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Checko Bewertungsbogen</title>
<link rel="preconnect" href="https://fonts.googleapis.com"><link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700;800&display=swap" rel="stylesheet">
<style>
:root{--bg:#f6f5f8;--card:#fff;--text:#251e2e;--muted:#6b6475;--line:#e8e4ee;--good:#1fa463;--warn:#e6a10b;--bad:#e5484d;--lav:#a48cd9;--peach:#f4a587;--soft:#f3f0f7}
@media (prefers-color-scheme:dark){:root{--bg:#131016;--card:#1c1820;--text:#f4f1f6;--muted:#a69fb0;--line:#2e2834;--soft:#241f29}}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:15px/1.5 "Plus Jakarta Sans",system-ui,sans-serif}
.wrap{max-width:1080px;margin:0 auto;padding:32px 20px 64px}
.hero{position:relative;overflow:hidden;display:grid;grid-template-columns:1fr auto;gap:24px;align-items:center;padding:28px;border-radius:24px;color:#fff;background:radial-gradient(120% 140% at 100% 0%,#22c55e 0%,transparent 55%),linear-gradient(135deg,#5b3df5 0%,#6d4aff 45%,#1f9d63 100%);box-shadow:0 24px 48px -24px rgb(91 61 245/.7)}
.hero::after{content:"";position:absolute;right:-80px;bottom:-140px;width:340px;height:340px;border-radius:50%;background:rgb(255 255 255/.08)}
.logo{display:inline-block;padding:8px 14px;border-radius:14px;background:#fff}.logo img{display:block;height:34px}
.kicker{margin:18px 0 4px;font-size:12px;font-weight:800;letter-spacing:.12em;text-transform:uppercase;opacity:.85}
h1{margin:0;font-size:38px;line-height:1.1;letter-spacing:-.02em}
.meta{margin-top:10px;font-size:13px;opacity:.85}
.verdict{position:relative;z-index:1;display:grid;place-items:center;width:168px;height:168px;border-radius:50%;background:rgb(255 255 255/.14);backdrop-filter:blur(6px)}
.verdict svg{position:absolute}.ring-track{fill:none;stroke:rgb(255 255 255/.25);stroke-width:10}.ring-bar{fill:none;stroke-width:10;stroke-linecap:round}
.verdict .big{font-size:52px;font-weight:800;line-height:1}.verdict small{display:block;text-align:center;font-size:12px;font-weight:700;opacity:.9}
.summary{margin:20px 0 0;padding:18px 20px;border-radius:18px;background:var(--card);border:1px solid var(--line);font-size:16px}
h2{margin:40px 0 14px;font-size:20px;letter-spacing:-.01em;display:flex;align-items:center;gap:10px}h2::before{content:"";width:10px;height:10px;border-radius:3px;background:var(--lav)}
.cats{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:14px}
.cat{padding:18px;border-radius:18px;background:var(--card);border:1px solid var(--line);box-shadow:0 8px 20px -16px rgb(37 30 46/.35)}
.cat header{display:flex;align-items:center;gap:10px}.cat h3{margin:0;flex:1;font-size:15px}
.cat-icon{display:grid;place-items:center;width:30px;height:30px;border-radius:9px;background:var(--soft);font-weight:800}
.cat-grade{display:grid;place-items:center;width:34px;height:34px;border-radius:50%;font-weight:800;color:#fff;background:var(--c)}
.bar{height:8px;margin:14px 0 8px;border-radius:99px;background:var(--soft);overflow:hidden}.bar span{display:block;height:100%;border-radius:99px}
.cat-score{margin:0;font-size:13px;color:var(--muted)}.cat-score b{font-size:20px;color:var(--text)}
.cat-detail{margin:4px 0 0;font-size:13px}.cat-note{margin:10px 0 0;padding-top:10px;border-top:1px dashed var(--line);font-size:13px;color:var(--muted)}
.two{display:grid;grid-template-columns:1fr 1fr;gap:14px}@media(max-width:760px){.two{grid-template-columns:1fr}.hero{grid-template-columns:1fr}.verdict{margin:auto}}
.box{padding:18px 20px;border-radius:18px;background:var(--card);border:1px solid var(--line)}.box h3{margin:0 0 8px;font-size:15px}
.box ul{margin:0;padding-left:18px}.box li{margin:4px 0}
.good h3{color:var(--good)}.weak h3{color:var(--bad)}
table{width:100%;border-collapse:collapse;background:var(--card);border:1px solid var(--line);border-radius:18px;overflow:hidden}
td:first-child{width:1%}td{padding:11px 14px;border-top:1px solid var(--line);vertical-align:top}tr:first-child td{border-top:0}.num{text-align:right;color:var(--muted);white-space:nowrap}
.err{margin-top:4px;font-size:12px;color:var(--bad)}
.pill{display:inline-block;padding:2px 9px;border-radius:99px;font-size:12px;font-weight:700;white-space:nowrap;background:var(--soft);color:var(--muted)}
.pill-ok{background:color-mix(in srgb,var(--good) 15%,transparent);color:var(--good)}.pill-bad{background:color-mix(in srgb,var(--bad) 15%,transparent);color:var(--bad)}
.imp-critical,.imp-serious{background:color-mix(in srgb,var(--bad) 15%,transparent);color:var(--bad)}.imp-moderate{background:color-mix(in srgb,var(--warn) 18%,transparent);color:#b07800}
.a11y{list-style:none;margin:0;padding:0;display:grid;gap:8px}.a11y li{display:flex;gap:12px;align-items:flex-start;padding:12px 14px;border-radius:14px;background:var(--card);border:1px solid var(--line)}.a11y small{display:block;color:var(--muted)}.ex{display:block;margin-top:6px;padding:4px 8px;border-radius:8px;background:var(--soft);font-size:11px;overflow-wrap:anywhere}
.recs{list-style:none;margin:0;padding:0;display:grid;gap:10px}.rec{display:flex;gap:14px;align-items:flex-start;padding:14px 16px;border-radius:16px;background:var(--card);border:1px solid var(--line);border-left:5px solid var(--lav)}
.rec p{margin:2px 0 0;color:var(--muted);font-size:14px}.rec>div{flex:1}.rec-n{display:grid;place-items:center;flex:none;width:28px;height:28px;border-radius:50%;background:var(--soft);font-weight:800}
.prio-hoch{border-left-color:var(--bad)}.prio-mittel{border-left-color:var(--warn)}.prio-niedrig{border-left-color:var(--good)}
.tiles{display:grid;grid-template-columns:repeat(auto-fill,minmax(150px,1fr));gap:12px}.tile{padding:16px;border-radius:16px;background:var(--card);border:1px solid var(--line)}.tile b{display:block;font-size:22px}.tile span{font-size:12px;color:var(--muted)}
.gal-title{margin:22px 0 10px;font-size:14px;color:var(--muted)}
.gallery{display:grid;gap:12px}.gallery-pc{grid-template-columns:repeat(auto-fill,minmax(300px,1fr))}.gallery-handy{grid-template-columns:repeat(auto-fill,minmax(150px,1fr))}
figure{margin:0;border-radius:14px;overflow:hidden;background:var(--card);border:1px solid var(--line)}figure img{display:block;width:100%}figcaption{padding:8px 10px;font-size:13px;font-weight:600}
footer{margin-top:40px;font-size:12px;color:var(--muted);text-align:center}
@media print{body{background:#fff}.hero{box-shadow:none}.gallery-pc{grid-template-columns:1fr 1fr}}
</style></head><body><div class="wrap">
<section class="hero">
  <div>${logo ? `<span class="logo"><img src="${logo}" alt="checko"></span>` : ''}
    <p class="kicker">Check-up · Bewertungsbogen</p>
    <h1>Gesamtnote ${g.n} – ${esc(g.label)}</h1>
    <p class="meta">${esc(date)} · Version ${esc(r.meta?.commit)} · ${(r.flows || []).length} Abläufe · ${(r.pages || []).length} Ansichten geprüft</p>
  </div>
  <div class="verdict">${ring(total, 168)}<div><div class="big">${total}</div><small>von 100</small></div></div>
</section>
${review?.summary ? `<p class="summary">${esc(review.summary)}</p>` : ''}

<h2>Bewertung nach Bereichen</h2>
<div class="cats">${catCards}</div>

${review ? `<h2>Einschätzung</h2><div class="two"><div class="box good"><h3>Stärken</h3><ul>${list(review.strengths)}</ul></div><div class="box weak"><h3>Schwächen</h3><ul>${list(review.weaknesses)}</ul></div></div>` : ''}
${recs ? `<h2>Empfehlungen</h2><ol class="recs">${recs}</ol>` : ''}

<h2>Abläufe</h2>
<table>${flowRows}</table>

<h2>Barrierefreiheit (WCAG 2 AA)</h2>
${a11yRows ? `<ul class="a11y">${a11yRows}</ul>` : '<p class="box">Keine automatisch erkennbaren Verstöße. 🎉</p>'}

<h2>Tempo</h2>
<div class="tiles">
  <div class="tile"><b>${((p.lcp || 0) / 1000).toFixed(2)} s</b><span>größtes Element sichtbar</span></div>
  <div class="tile"><b>${((p.domReady || 0) / 1000).toFixed(2)} s</b><span>Seite bereit</span></div>
  <div class="tile"><b>${kb(p.wasm || 0)}</b><span>App (WebAssembly)</span></div>
  <div class="tile"><b>${kb((p.js || 0) + (p.css || 0))}</b><span>JS + CSS</span></div>
</div>
${(r.consoleErrors || []).length ? `<h2>Konsolenfehler</h2><ul class="a11y">${[...new Map(r.consoleErrors.map((e) => [e.text, e])).values()].map((e) => `<li><span class="pill pill-bad">${esc(e.where)}</span><div><small>${esc(e.text)}</small></div></li>`).join('')}</ul>` : ''}

<h2>Ansichten</h2>
${gallery}
<footer>Automatisch erstellt mit <code>npm run checkup</code>${review ? ' · Einschätzung von Claude' : ''} · Checko</footer>
</div></body></html>`;
  const out = join(dir, 'index.html');
  await writeFile(out, html);
  return out;
}

// run directly: node e2e/report.mjs
if (import.meta.url === `file://${process.argv[1]}`) {
  buildReport().then((f) => console.log(`Bewertungsbogen: ${f}`)).catch((e) => { console.error(e); process.exit(1); });
}

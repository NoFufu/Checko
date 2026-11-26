// Checko service worker: network first, cached copy when offline.
// (Network first so a new version always shows up right away.)
const CACHE = "checko-v1";

self.addEventListener("install", () => self.skipWaiting());
self.addEventListener("activate", (e) => e.waitUntil(self.clients.claim()));

self.addEventListener("fetch", (e) => {
  const req = e.request;
  if (req.method !== "GET" || new URL(req.url).origin !== location.origin) return;
  e.respondWith(
    fetch(req)
      .then((res) => {
        const copy = res.clone();
        caches.open(CACHE).then((c) => c.put(req, copy));
        return res;
      })
      .catch(async () => {
        const hit = await caches.match(req);
        if (hit) return hit;
        if (req.mode === "navigate") return caches.match(new URL("./", self.registration.scope));
        return Response.error();
      })
  );
});

// Reminders from the server (Web Push) – same tag as in-app reminders, so no doubles.
self.addEventListener("push", (e) => {
  let d = {};
  try { d = e.data ? e.data.json() : {}; } catch (_) { d = { title: e.data && e.data.text() }; }
  e.waitUntil(
    self.registration.showNotification(d.title || "Checko", {
      body: d.body || "",
      tag: d.tag || undefined,
      icon: new URL("assets/brand/icon-192.png", self.registration.scope).href,
      badge: new URL("assets/brand/favicon-64.png", self.registration.scope).href,
      data: { url: self.registration.scope },
    })
  );
});

// Tap on a notification: focus an open Checko window or open one.
self.addEventListener("notificationclick", (e) => {
  e.notification.close();
  const url = (e.notification.data && e.notification.data.url) || self.registration.scope;
  e.waitUntil(
    self.clients.matchAll({ type: "window", includeUncontrolled: true }).then((list) => {
      for (const c of list) if (c.url.startsWith(self.registration.scope) && "focus" in c) return c.focus();
      return self.clients.openWindow(url);
    })
  );
});

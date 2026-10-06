const CACHE_PREFIX = 'pixel-terminal-';
const CACHE_NAME = `${CACHE_PREFIX}__RELEASE_ID__`;
const SCOPE = new URL(self.registration.scope);
const OFFLINE_ASSETS = [
  './',
  './screen.css',
  './manifest.webmanifest',
  './icon.svg',
  './icon-192.png',
  './icon-512.png',
  './apple-touch-icon.png',
  './pkg/pixel_ssh_web_client.js',
  './pkg/pixel_ssh_web_client_bg.wasm',
].map(path => new URL(path, SCOPE).href);

self.addEventListener('install', event => {
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE_NAME);
    await cache.addAll(OFFLINE_ASSETS);
    await self.skipWaiting();
  })());
});

self.addEventListener('activate', event => {
  event.waitUntil((async () => {
    const names = await caches.keys();
    await Promise.all(names
      .filter(name => name.startsWith(CACHE_PREFIX) && name !== CACHE_NAME)
      .map(name => caches.delete(name)));
    await self.clients.claim();
  })());
});

self.addEventListener('fetch', event => {
  const request = event.request;
  const url = new URL(request.url);
  if (request.method !== 'GET' || url.origin !== SCOPE.origin ||
      !url.pathname.startsWith(SCOPE.pathname)) return;

  event.respondWith((async () => {
    const cache = await caches.open(CACHE_NAME);
    try {
      const response = await fetch(request);
      // Preserve the visible URL change for retired HTML routes. Fetch follows
      // redirects internally; returning its final HTML would leave /catalog/
      // in the address bar and lose the native view's query parameters.
      if (request.mode === 'navigate' && response.redirected) {
        return Response.redirect(response.url, 302);
      }
      if (response.ok) await cache.put(request, response.clone());
      return response;
    } catch {
      const cached = await cache.match(request, { ignoreSearch: true });
      if (cached) return cached;
      if (request.mode === 'navigate') {
        const shell = await cache.match(new URL('./', SCOPE).href);
        if (shell) return shell;
      }
      return Response.error();
    }
  })());
});

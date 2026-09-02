'use strict';
/* ShadowCrypt Online — service worker: full offline play via cache-first app shell */
var CACHE = 'shadowcrypt-v2';
var ASSETS = [
  './',
  'index.html',
  'manifest.webmanifest',
  'css/style.css',
  'icons/icon.svg',
  'icons/icon-192.png',
  'icons/icon-512.png',
  'js/data/data_classes.js',
  'js/data/data_species.js',
  'js/data/data_enemies.js',
  'js/data/data_items.js',
  'js/data/data_quests.js',
  'js/data/data_social.js',
  'js/data/data_companions.js',
  'js/data/data_crafting.js',
  'js/data/data_world.js',
  'js/data/data_mmo.js',
  'js/util.js',
  'js/audio.js',
  'js/worldgen.js',
  'js/entities.js',
  'js/combat.js',
  'js/ai.js',
  'js/systems.js',
  'js/haven.js',
  'js/arena.js',
  'js/siege.js',
  'js/net.js',
  'js/render.js',
  'js/view3d.js',
  'js/input.js',
  'js/ui.js',
  'js/game.js',
  'js/main.js'
];

self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(CACHE).then(function (cache) {
      return cache.addAll(ASSETS);
    }).then(function () { return self.skipWaiting(); })
  );
});

self.addEventListener('activate', function (e) {
  e.waitUntil(
    caches.keys().then(function (keys) {
      return Promise.all(keys.map(function (k) {
        if (k !== CACHE) return caches.delete(k);
      }));
    }).then(function () { return self.clients.claim(); })
  );
});

self.addEventListener('fetch', function (e) {
  var url = new URL(e.request.url);
  if (e.request.method !== 'GET') return;
  if (url.pathname.endsWith('/ws')) return; // never touch the websocket upgrade path
  // cache-first for same-origin app shell, network-first fallback-to-cache for the rest
  if (url.origin === location.origin) {
    e.respondWith(
      caches.match(e.request, { ignoreSearch: true }).then(function (hit) {
        if (hit) return hit;
        return fetch(e.request).then(function (res) {
          if (res && res.ok) {
            var clone = res.clone();
            caches.open(CACHE).then(function (cache) { cache.put(e.request, clone); });
          }
          return res;
        }).catch(function () {
          return caches.match('index.html');
        });
      })
    );
  }
});

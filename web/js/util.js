'use strict';
/* ShadowCrypt Online — utilities: RNG, math, events, storage */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.util = (function () {
  // Deterministic RNG (mulberry32) so dungeon floors can be seed-shared in co-op.
  function mulberry32(seed) {
    var a = seed >>> 0;
    return function () {
      a |= 0; a = (a + 0x6D2B79F5) | 0;
      var t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }

  function Rng(seed) {
    this.next = mulberry32(seed === undefined ? ((Math.random() * 0xffffffff) >>> 0) : seed);
  }
  Rng.prototype.float = function () { return this.next(); };
  Rng.prototype.int = function (min, max) { // inclusive
    return min + Math.floor(this.next() * (max - min + 1));
  };
  Rng.prototype.chance = function (p) { return this.next() < p; };
  Rng.prototype.pick = function (arr) { return arr[Math.floor(this.next() * arr.length)]; };
  Rng.prototype.shuffle = function (arr) {
    for (var i = arr.length - 1; i > 0; i--) {
      var j = Math.floor(this.next() * (i + 1));
      var t = arr[i]; arr[i] = arr[j]; arr[j] = t;
    }
    return arr;
  };
  Rng.prototype.weighted = function (entries) { // [{w: number, v: any}]
    var total = 0, i;
    for (i = 0; i < entries.length; i++) total += entries[i].w;
    var roll = this.next() * total;
    for (i = 0; i < entries.length; i++) {
      roll -= entries[i].w;
      if (roll <= 0) return entries[i].v;
    }
    return entries[entries.length - 1].v;
  };

  function clamp(v, lo, hi) { return v < lo ? lo : (v > hi ? hi : v); }
  function lerp(a, b, t) { return a + (b - a) * t; }
  function dist(x1, y1, x2, y2) { var dx = x2 - x1, dy = y2 - y1; return Math.sqrt(dx * dx + dy * dy); }
  function cheb(x1, y1, x2, y2) { return Math.max(Math.abs(x2 - x1), Math.abs(y2 - y1)); }
  function manhattan(x1, y1, x2, y2) { return Math.abs(x2 - x1) + Math.abs(y2 - y1); }

  // Simple event bus
  var listeners = {};
  function on(evt, fn) { (listeners[evt] = listeners[evt] || []).push(fn); }
  function off(evt, fn) {
    var l = listeners[evt]; if (!l) return;
    var i = l.indexOf(fn); if (i >= 0) l.splice(i, 1);
  }
  function emit(evt) {
    var l = listeners[evt]; if (!l) return;
    var args = Array.prototype.slice.call(arguments, 1);
    for (var i = 0; i < l.length; i++) {
      try { l[i].apply(null, args); } catch (e) { console.error('event handler error', evt, e); }
    }
  }

  // Storage (guarded — private mode etc.)
  function storeSet(key, value) {
    try { localStorage.setItem(key, JSON.stringify(value)); return true; }
    catch (e) { console.warn('storage write failed', e); return false; }
  }
  function storeGet(key, fallback) {
    try {
      var raw = localStorage.getItem(key);
      return raw === null ? fallback : JSON.parse(raw);
    } catch (e) { return fallback; }
  }
  function storeDel(key) { try { localStorage.removeItem(key); } catch (e) { /* noop */ } }

  function uid() {
    return Date.now().toString(36) + '-' + Math.floor(Math.random() * 0xffffff).toString(36);
  }

  function fmt(n) {
    if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
    if (n >= 10000) return (n / 1000).toFixed(1) + 'k';
    return String(Math.floor(n));
  }

  function fmtTime(ms) {
    var s = Math.max(0, Math.ceil(ms / 1000));
    if (s < 60) return s + 's';
    var m = Math.floor(s / 60); s = s % 60;
    if (m < 60) return m + 'm ' + s + 's';
    var h = Math.floor(m / 60); m = m % 60;
    return h + 'h ' + m + 'm';
  }

  function esc(s) {
    return String(s == null ? '' : s)
      .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;').replace(/'/g, '&#39;');
  }

  function deepClone(o) { return JSON.parse(JSON.stringify(o)); }

  // Bresenham line — used for line of sight
  function line(x0, y0, x1, y1) {
    var pts = [];
    var dx = Math.abs(x1 - x0), dy = Math.abs(y1 - y0);
    var sx = x0 < x1 ? 1 : -1, sy = y0 < y1 ? 1 : -1;
    var err = dx - dy;
    for (;;) {
      pts.push([x0, y0]);
      if (x0 === x1 && y0 === y1) break;
      var e2 = 2 * err;
      if (e2 > -dy) { err -= dy; x0 += sx; }
      if (e2 < dx) { err += dx; y0 += sy; }
    }
    return pts;
  }

  var DIRS8 = [[0, -1], [0, 1], [-1, 0], [1, 0], [-1, -1], [1, -1], [-1, 1], [1, 1]];
  var DIRS4 = [[0, -1], [0, 1], [-1, 0], [1, 0]];

  function hashStr(s) {
    var h = 2166136261;
    for (var i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
      h = Math.imul(h, 16777619);
    }
    return h >>> 0;
  }

  return {
    Rng: Rng, clamp: clamp, lerp: lerp, dist: dist, cheb: cheb, manhattan: manhattan,
    on: on, off: off, emit: emit,
    storeSet: storeSet, storeGet: storeGet, storeDel: storeDel,
    uid: uid, fmt: fmt, fmtTime: fmtTime, esc: esc, deepClone: deepClone,
    line: line, DIRS8: DIRS8, DIRS4: DIRS4, hashStr: hashStr
  };
})();

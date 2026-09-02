'use strict';
/* ShadowCrypt Online — sprite atlas loader.
 * Art: Dungeon Crawl Stone Soup rltiles (public domain) — see assets/LICENSE-art.txt */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.assets = (function () {
  var img = null, rects = null, ready = false;
  var loadCbs = [];

  function load() {
    if (typeof Image === 'undefined') return;
    var gotImg = false, gotJson = false;
    function maybeDone() {
      if (gotImg && gotJson) {
        ready = true;
        loadCbs.forEach(function (cb) { try { cb(); } catch (e) { /* noop */ } });
        loadCbs = [];
        if (SC.util) SC.util.emit('assets:ready');
      }
    }
    img = new Image();
    img.onload = function () { gotImg = true; maybeDone(); };
    img.onerror = function () { console.warn('atlas.png failed to load — using vector fallback art'); };
    img.src = 'assets/atlas.png';
    fetch('assets/atlas.json').then(function (r) { return r.json(); }).then(function (j) {
      rects = j; gotJson = true; maybeDone();
    }).catch(function () { console.warn('atlas.json failed to load'); });
  }

  function isReady() { return ready; }
  function onReady(cb) { if (ready) cb(); else loadCbs.push(cb); }
  function rect(key) { return (rects && rects[key]) || null; }
  function has(key) { return !!(ready && rects && rects[key]); }

  // Draw sprite `key` centered at (cx, cy) scaled so its longer side == size px.
  function draw(ctx, key, cx, cy, size, alpha) {
    var r = rect(key);
    if (!r) return false;
    var scale = size / Math.max(r[2], r[3]);
    var w = r[2] * scale, h = r[3] * scale;
    var prevSmooth = ctx.imageSmoothingEnabled;
    ctx.imageSmoothingEnabled = false;
    if (alpha !== undefined) { var pa = ctx.globalAlpha; ctx.globalAlpha = alpha; }
    ctx.drawImage(img, r[0], r[1], r[2], r[3], Math.round(cx - w / 2), Math.round(cy - h / 2), Math.round(w), Math.round(h));
    if (alpha !== undefined) ctx.globalAlpha = pa;
    ctx.imageSmoothingEnabled = prevSmooth;
    return true;
  }

  // Draw sprite stretched to exactly fill a rect (tiles).
  function drawRect(ctx, key, x, y, w, h) {
    var r = rect(key);
    if (!r) return false;
    var prevSmooth = ctx.imageSmoothingEnabled;
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(img, r[0], r[1], r[2], r[3], x, y, w, h);
    ctx.imageSmoothingEnabled = prevSmooth;
    return true;
  }

  function image() { return img; }
  function map() { return (SC.DATA && SC.DATA.sprites) || {}; }

  // Convenience lookups against the generated mapping
  function enemyKey(id) { var m = map(); return (m.enemies && m.enemies[id]) || null; }
  function itemKey(id) { var m = map(); return (m.items && m.items[id]) || null; }
  function layerKey(id) { var m = map(); return (m.layers && m.layers[id]) || null; }
  function tileKey(name) { var m = map(); return (m.tiles && m.tiles[name]) || null; }
  function buildingKey(id) { var m = map(); return (m.buildings && m.buildings[id]) || null; }
  function effectKey(name) { var m = map(); return (m.effects && m.effects[name]) || null; }
  function classLook(classId) { var m = map(); return (m.classes && m.classes[classId]) || null; }
  function themeTiles(themeId) { var m = map(); return (m.themes && m.themes[themeId]) || null; }
  function skillIconKey(cat) { var m = map(); return (m.skillIcons && m.skillIcons[cat]) || null; }
  function cropKey(id) { var m = map(); return (m.cropIcon && m.cropIcon[id]) || null; }

  // Small standalone canvas with one sprite (for DOM UI: inventory, buttons)
  function iconCanvas(key, size) {
    size = size || 32;
    var c = document.createElement('canvas');
    c.width = size; c.height = size;
    c.className = 'sprite-icon';
    var g = c.getContext('2d');
    if (!draw(g, key, size / 2, size / 2, size)) return null;
    return c;
  }

  return {
    load: load, isReady: isReady, onReady: onReady,
    rect: rect, has: has, draw: draw, drawRect: drawRect, image: image,
    enemyKey: enemyKey, itemKey: itemKey, layerKey: layerKey, tileKey: tileKey,
    buildingKey: buildingKey, effectKey: effectKey, classLook: classLook,
    themeTiles: themeTiles, skillIconKey: skillIconKey, cropKey: cropKey,
    iconCanvas: iconCanvas
  };
})();

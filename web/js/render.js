'use strict';
/* ShadowCrypt Online — renderer v2
 * Pre-rendered theme tile atlases (textured floors, depth-shaded walls, animated liquids),
 * procedural monster sprite families, class-specific hero sprites with swing animation,
 * dynamic lighting + torch flicker + vignette, screen shake, hit flashes, boss bar,
 * ability telegraphs, elite auras, objective arrow, minimap. */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.render = (function () {
  var U = SC.util, T = SC.TILE;

  var canvas = null, ctx = null;
  var dpr = 1, vw = 0, vh = 0;
  var cam = { x: 0, y: 0 };
  var TILE = 40;
  var floats = [], sparks = [];
  var shakeMag = 0, shakeX = 0, shakeY = 0;
  var vignette = null;
  var atlases = {}; // themeId:size -> atlas
  var settingsFx = U.storeGet('sc_fx', { shake: true });

  // ------------------------------------------------------------------ core
  function init(el) {
    canvas = el;
    ctx = canvas.getContext('2d');
    resize();
    if (typeof window !== 'undefined') window.addEventListener('resize', resize);
  }

  function resize() {
    dpr = (typeof window !== 'undefined' && window.devicePixelRatio) || 1;
    vw = canvas.clientWidth || (typeof window !== 'undefined' ? window.innerWidth : 800);
    vh = canvas.clientHeight || (typeof window !== 'undefined' ? window.innerHeight : 600);
    canvas.width = Math.round(vw * dpr);
    canvas.height = Math.round(vh * dpr);
    TILE = vw < 480 ? 36 : (vw < 900 ? 42 : 48);
    vignette = null;
  }

  function shake(mag) { if (settingsFx.shake) shakeMag = Math.max(shakeMag, mag); }
  function setShakeEnabled(on) { settingsFx.shake = !!on; U.storeSet('sc_fx', settingsFx); }

  function floatText(x, y, txt, color) {
    floats.push({ x: x, y: y, txt: txt, color: color || '#fff', ttl: 950, vy: -1.5, vx: (Math.random() - 0.5) * 0.6 });
  }
  function burst(x, y, color, n) {
    for (var i = 0; i < (n || 8); i++) {
      var a = Math.random() * Math.PI * 2, s = 0.6 + Math.random() * 2.4;
      sparks.push({ x: x + 0.5, y: y + 0.5, vx: Math.cos(a) * s, vy: Math.sin(a) * s - 0.8, ttl: 380 + Math.random() * 320, color: color || '#fff', g: 3 });
    }
  }

  function tickFx(dtMs) {
    var i, dt = dtMs / 1000;
    for (i = floats.length - 1; i >= 0; i--) {
      var f = floats[i];
      f.ttl -= dtMs; f.y += f.vy * dt; f.x += (f.vx || 0) * dt; f.vy *= 0.97;
      if (f.ttl <= 0) floats.splice(i, 1);
    }
    for (i = sparks.length - 1; i >= 0; i--) {
      var s = sparks[i];
      s.ttl -= dtMs; s.x += s.vx * dt; s.y += s.vy * dt; s.vy += (s.g || 0) * dt;
      if (s.ttl <= 0) sparks.splice(i, 1);
    }
    // camera shake decay
    if (shakeMag > 0.2) {
      shakeX = (Math.random() - 0.5) * shakeMag;
      shakeY = (Math.random() - 0.5) * shakeMag;
      shakeMag *= Math.pow(0.0015, dt); // fast decay
    } else { shakeMag = 0; shakeX = 0; shakeY = 0; }
  }

  // smooth grid→pixel interpolation for entity movement
  function interp(e, dtMs) {
    var k = Math.min(1, dtMs / 110);
    e.fx = e.fx === undefined ? e.x : U.lerp(e.fx, e.x, k);
    e.fy = e.fy === undefined ? e.y : U.lerp(e.fy, e.y, k);
  }

  // ----------------------------------------------------------- tile atlas
  function hashXY(x, y, salt) { return U.hashStr(x + ':' + y + ':' + (salt || 0)); }

  function shade(hex, amt) {
    var n = parseInt(hex.slice(1), 16);
    var r = U.clamp(((n >> 16) & 255) + amt, 0, 255);
    var g = U.clamp(((n >> 8) & 255) + amt, 0, 255);
    var b = U.clamp((n & 255) + amt, 0, 255);
    return 'rgb(' + r + ',' + g + ',' + b + ')';
  }

  function mkCanvas(s) {
    var c = document.createElement('canvas');
    c.width = s; c.height = s;
    return c;
  }

  function buildAtlas(theme, size) {
    var pal = theme.palette || { wall: '#3a3f4d', floor: '#20242e', accent: '#556' };
    var a = { floors: [], water: [], lava: [] };
    var i, c, g, x, y;
    // floor variants with speckle texture
    for (i = 0; i < 4; i++) {
      c = mkCanvas(size); g = c.getContext('2d');
      g.fillStyle = pal.floor; g.fillRect(0, 0, size, size);
      var rng = new U.Rng(U.hashStr(theme.id + ':floor:' + i));
      for (var sp = 0; sp < 14; sp++) {
        g.fillStyle = rng.chance(0.5) ? shade(pal.floor, 7) : shade(pal.floor, -8);
        var ss = rng.int(1, 3);
        g.fillRect(rng.int(0, size - 3), rng.int(0, size - 3), ss, ss);
      }
      // crack on variant 3
      if (i === 3) {
        g.strokeStyle = shade(pal.floor, -14); g.lineWidth = 1;
        g.beginPath(); g.moveTo(size * 0.2, size * 0.8);
        g.lineTo(size * 0.45, size * 0.55); g.lineTo(size * 0.4, size * 0.3); g.stroke();
      }
      g.strokeStyle = 'rgba(0,0,0,0.18)'; g.strokeRect(0.5, 0.5, size - 1, size - 1);
      a.floors.push(c);
    }
    // wall top (seen from above) + wall front face (depth)
    c = mkCanvas(size); g = c.getContext('2d');
    g.fillStyle = shade(pal.wall, 8); g.fillRect(0, 0, size, size);
    g.fillStyle = pal.wall; g.fillRect(2, 2, size - 4, size - 4);
    // brick lines
    g.strokeStyle = 'rgba(0,0,0,0.28)'; g.lineWidth = 1;
    for (y = size / 3; y < size; y += size / 3) { g.beginPath(); g.moveTo(0, y); g.lineTo(size, y); g.stroke(); }
    g.beginPath(); g.moveTo(size / 2, 0); g.lineTo(size / 2, size / 3); g.stroke();
    g.beginPath(); g.moveTo(size / 4, size / 3); g.lineTo(size / 4, size * 2 / 3); g.stroke();
    g.beginPath(); g.moveTo(size * 3 / 4, size / 3); g.lineTo(size * 3 / 4, size * 2 / 3); g.stroke();
    g.beginPath(); g.moveTo(size / 2, size * 2 / 3); g.lineTo(size / 2, size); g.stroke();
    a.wallTop = c;
    c = mkCanvas(size); g = c.getContext('2d');
    var grad = g.createLinearGradient(0, 0, 0, size);
    grad.addColorStop(0, shade(pal.wall, -18));
    grad.addColorStop(1, shade(pal.wall, -42));
    g.fillStyle = grad; g.fillRect(0, 0, size, size);
    g.strokeStyle = 'rgba(0,0,0,0.3)';
    for (y = size / 4; y < size; y += size / 4) { g.beginPath(); g.moveTo(0, y); g.lineTo(size, y); g.stroke(); }
    a.wallFront = c;
    // animated water & lava frames
    for (i = 0; i < 2; i++) {
      c = mkCanvas(size); g = c.getContext('2d');
      g.fillStyle = '#123a5e'; g.fillRect(0, 0, size, size);
      g.strokeStyle = 'rgba(120,190,255,0.35)'; g.lineWidth = 1.5;
      for (var w = 0; w < 3; w++) {
        g.beginPath();
        var wy = size * (0.25 + w * 0.25) + (i === 0 ? 0 : 3);
        g.moveTo(0, wy);
        g.quadraticCurveTo(size * 0.25, wy - 3, size * 0.5, wy);
        g.quadraticCurveTo(size * 0.75, wy + 3, size, wy);
        g.stroke();
      }
      a.water.push(c);
      c = mkCanvas(size); g = c.getContext('2d');
      g.fillStyle = '#6e1a06'; g.fillRect(0, 0, size, size);
      var lrng = new U.Rng(U.hashStr('lava' + i));
      for (var b = 0; b < 5; b++) {
        g.fillStyle = 'rgba(255,' + lrng.int(100, 170) + ',30,' + (0.5 + i * 0.2) + ')';
        var br = lrng.int(2, 5) + i;
        g.beginPath(); g.arc(lrng.int(4, size - 4), lrng.int(4, size - 4), br, 0, Math.PI * 2); g.fill();
      }
      g.fillStyle = 'rgba(255,220,120,' + (0.25 + i * 0.15) + ')';
      g.fillRect(0, i === 0 ? size * 0.3 : size * 0.6, size, 2);
      a.lava.push(c);
    }
    return a;
  }

  function atlasFor(theme) {
    var key = (theme.id || 'x') + ':' + TILE;
    if (!atlases[key]) atlases[key] = buildAtlas(theme, TILE);
    return atlases[key];
  }

  // ------------------------------------------------------------- lighting
  function lightAt(px, py, x, y, radius, flicker) {
    var d = U.dist(px, py, x, y);
    var b = 1 - Math.pow(Math.min(1, d / radius), 1.7);
    return U.clamp(b * flicker + 0.12, 0.1, 1);
  }

  function drawVignette() {
    if (!vignette) {
      vignette = document.createElement('canvas');
      vignette.width = Math.max(2, Math.round(vw / 2)); vignette.height = Math.max(2, Math.round(vh / 2));
      var g = vignette.getContext('2d');
      var grad = g.createRadialGradient(vignette.width / 2, vignette.height / 2, Math.min(vignette.width, vignette.height) * 0.36,
        vignette.width / 2, vignette.height / 2, Math.max(vignette.width, vignette.height) * 0.72);
      grad.addColorStop(0, 'rgba(0,0,0,0)');
      grad.addColorStop(1, 'rgba(2,3,8,0.55)');
      g.fillStyle = grad;
      g.fillRect(0, 0, vignette.width, vignette.height);
    }
    ctx.drawImage(vignette, 0, 0, vw, vh);
  }

  // ---------------------------------------------------------------- crypt
  function renderCrypt(st, dtMs) {
    tickFx(dtMs);
    var map = st.map, p = st.player;
    var theme = map.theme || {};
    var pal = theme.palette || { wall: '#3a3f4d', floor: '#20242e', accent: '#556' };
    var atlas = atlasFor(theme);
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.fillStyle = '#04060b';
    ctx.fillRect(0, 0, vw, vh);

    var targX = p.fx * TILE - vw / 2 + TILE / 2;
    var targY = p.fy * TILE - vh / 2 + TILE / 2;
    cam.x = U.lerp(cam.x, targX, Math.min(1, dtMs / 120));
    cam.y = U.lerp(cam.y, targY, Math.min(1, dtMs / 120));
    var camX = cam.x + shakeX, camY = cam.y + shakeY;

    var x0 = Math.max(0, Math.floor(camX / TILE) - 1);
    var y0 = Math.max(0, Math.floor(camY / TILE) - 1);
    var x1 = Math.min(map.w - 1, Math.ceil((camX + vw) / TILE) + 1);
    var y1 = Math.min(map.h - 1, Math.ceil((camY + vh) / TILE) + 1);

    var t = Date.now() / 1000;
    var flicker = 0.93 + 0.05 * Math.sin(t * 11) + 0.02 * Math.sin(t * 27);
    var fovR = SC.entities.hasEffect(p, 'torchlight') ? 13 : 10;
    var liquidFrame = Math.floor(t * 2) % 2;

    var x, y, key, sx, sy;
    for (y = y0; y <= y1; y++) {
      for (x = x0; x <= x1; x++) {
        key = x + ',' + y;
        var visible = !!map.visible[key];
        var explored = !!map.explored[key];
        if (!visible && !explored) continue;
        var tile = map.get(x, y);
        sx = x * TILE - camX; sy = y * TILE - camY;
        drawTile(atlas, pal, tile, sx, sy, t, map, x, y, liquidFrame);
        // lighting / memory fog
        if (visible) {
          var b = lightAt(p.fx, p.fy, x, y, fovR, flicker);
          if (b < 0.98) {
            ctx.fillStyle = 'rgba(3,5,12,' + (1 - b).toFixed(3) + ')';
            ctx.fillRect(sx, sy, TILE, TILE);
          }
        } else {
          ctx.fillStyle = 'rgba(6,9,20,0.78)';
          ctx.fillRect(sx, sy, TILE, TILE);
        }
      }
    }

    // telegraphs (boss ability warnings)
    var now = Date.now();
    if (st.telegraphs) {
      for (var tg = 0; tg < st.telegraphs.length; tg++) {
        var tel = st.telegraphs[tg];
        var prog = U.clamp((now - tel.at) / tel.duration, 0, 1);
        var tx = tel.x * TILE - camX + TILE / 2, ty = tel.y * TILE - camY + TILE / 2;
        ctx.strokeStyle = tel.color || 'rgba(255,80,60,0.9)';
        ctx.lineWidth = 2;
        ctx.beginPath(); ctx.arc(tx, ty, tel.radius * TILE, 0, Math.PI * 2); ctx.stroke();
        ctx.fillStyle = (tel.color || 'rgba(255,80,60,1)').replace(/[\d.]+\)$/, (0.14 + prog * 0.2).toFixed(2) + ')');
        ctx.beginPath(); ctx.arc(tx, ty, tel.radius * TILE * prog, 0, Math.PI * 2); ctx.fill();
      }
    }

    // breakables (urns, crates, golden chests)
    if (map.breakables) {
      for (var bk = 0; bk < map.breakables.length; bk++) {
        var br = map.breakables[bk];
        if (br.broken || !map.visible[br.x + ',' + br.y]) continue;
        drawBreakable(br, br.x * TILE - camX, br.y * TILE - camY, t);
      }
    }

    // ground items
    for (var gi = 0; gi < st.groundItems.length; gi++) {
      var g2 = st.groundItems[gi];
      if (!map.visible[g2.x + ',' + g2.y]) continue;
      var gsx = g2.x * TILE - camX, gsy = g2.y * TILE - camY;
      var bob = Math.sin(t * 3 + g2.x + g2.y) * 2;
      drawItemGlyph(g2, gsx + TILE / 2, gsy + TILE / 2 + bob, t);
    }

    // co-op ghosts
    if (SC.net) {
      var gl = SC.net.cryptGhosts(p.floor);
      for (var gg = 0; gg < gl.length; gg++) {
        var gh = gl[gg];
        var ghx = gh.x * TILE - camX + TILE / 2, ghy = gh.y * TILE - camY + TILE / 2;
        ctx.globalAlpha = 0.5;
        drawHero(ghx, ghy, gh.classId || 'warrior', t, { dirX: 0, dirY: 1 });
        ctx.globalAlpha = 1;
        label(gh.name, ghx, ghy - TILE * 0.8, '#7ecbff');
      }
    }

    // allies
    for (var ai = 0; ai < st.allies.length; ai++) {
      var al = st.allies[ai];
      if (al.hp <= 0) continue;
      interp(al, dtMs);
      var alx = al.fx * TILE - camX + TILE / 2, aly = al.fy * TILE - camY + TILE / 2;
      drawMonster({ id: al.id || 'skeleton', name: al.name, glyph: al.glyph, color: al.cid ? '#6fd88f' : '#cfd8ea' }, alx, aly, t, 0.85);
      hpBar(alx, aly - TILE * 0.62, al.hp / al.maxHp, TILE * 0.7);
    }

    // monsters
    var bossOnScreen = null;
    for (var mi = 0; mi < st.monsters.length; mi++) {
      var m = st.monsters[mi];
      if (m.hp <= 0) continue;
      if (!map.visible[m.x + ',' + m.y]) continue;
      interp(m, dtMs);
      var mx = m.fx * TILE - camX + TILE / 2, my = m.fy * TILE - camY + TILE / 2;
      var size = m.boss ? 1.6 : (m.miniBoss ? 1.25 : 1);
      // elite aura
      if (m.affix) {
        ctx.strokeStyle = m.affix.color;
        ctx.lineWidth = 2;
        ctx.globalAlpha = 0.5 + 0.3 * Math.sin(t * 5);
        ctx.beginPath(); ctx.arc(mx, my, TILE * 0.44 * size, 0, Math.PI * 2); ctx.stroke();
        ctx.globalAlpha = 1;
      }
      if (m.enraged) {
        ctx.fillStyle = 'rgba(255,60,40,' + (0.1 + 0.08 * Math.sin(t * 8)) + ')';
        ctx.beginPath(); ctx.arc(mx, my, TILE * 0.55 * size, 0, Math.PI * 2); ctx.fill();
      }
      drawMonster(m, mx, my, t, size);
      // hit flash
      if (m.flashUntil && now < m.flashUntil) {
        ctx.globalAlpha = 0.55;
        ctx.fillStyle = '#fff';
        ctx.beginPath(); ctx.arc(mx, my - TILE * 0.1, TILE * 0.36 * size, 0, Math.PI * 2); ctx.fill();
        ctx.globalAlpha = 1;
      }
      hpBar(mx, my - TILE * 0.6 * size, m.hp / m.maxHp, TILE * 0.72 * size);
      if (m.affix && !m.boss) label(m.affix.name + ' ' + m.name, mx, my - TILE * 0.74 * size, m.affix.color);
      if (m.boss) bossOnScreen = m;
      else if (m.miniBoss) label(m.name, mx, my - TILE * 0.78 * size, '#ffd35c');
    }

    // projectiles
    for (var pi = 0; pi < st.projectiles.length; pi++) {
      var pr = st.projectiles[pi];
      var prx = pr.x * TILE - camX, pry = pr.y * TILE - camY;
      ctx.fillStyle = pr.color || '#ffd35c';
      ctx.beginPath(); ctx.arc(prx, pry, 4, 0, Math.PI * 2); ctx.fill();
      ctx.globalAlpha = 0.35;
      ctx.beginPath(); ctx.arc(prx - pr.vx * 0.02 * TILE, pry - pr.vy * 0.02 * TILE, 3, 0, Math.PI * 2); ctx.fill();
      ctx.globalAlpha = 1;
    }

    // player
    interp(p, dtMs);
    var px = p.fx * TILE - camX + TILE / 2, py = p.fy * TILE - camY + TILE / 2;
    if (p.dashUntil && now < p.dashUntil) {
      ctx.globalAlpha = 0.35;
      drawHero(px - p.dirX * TILE * 0.5, py - p.dirY * TILE * 0.5, p.classId, t, p);
      ctx.globalAlpha = 1;
    }
    if (SC.entities.hasEffect(p, 'invisible')) ctx.globalAlpha = 0.45;
    drawHero(px, py, p.classId, t, p);
    ctx.globalAlpha = 1;

    drawSparksAt(camX, camY, TILE);
    drawFloatsAt(camX, camY, TILE);
    drawVignette();
    if (bossOnScreen) drawBossBar(bossOnScreen);
    drawObjectiveArrow(map, p, camX, camY);
    drawMinimap(map, p, st.monsters);
  }

  function drawTile(atlas, pal, tile, sx, sy, t, map, x, y, liquidFrame) {
    var below;
    switch (tile) {
      case T.WALL:
        below = map.get(x, y + 1);
        if (below !== T.WALL && map.inb(x, y + 1)) ctx.drawImage(atlas.wallFront, sx, sy);
        else ctx.drawImage(atlas.wallTop, sx, sy);
        break;
      case T.WATER:
        ctx.drawImage(atlas.water[liquidFrame], sx, sy);
        break;
      case T.LAVA:
        ctx.drawImage(atlas.lava[liquidFrame], sx, sy);
        break;
      default: {
        ctx.drawImage(atlas.floors[hashXY(x, y, map.floor) % 4], sx, sy);
        // deterministic decor
        var h = hashXY(x, y, map.floor * 7);
        if ((h % 23) === 0) { // bones
          ctx.strokeStyle = 'rgba(215,215,205,0.32)'; ctx.lineWidth = 2;
          ctx.beginPath(); ctx.moveTo(sx + TILE * 0.3, sy + TILE * 0.6); ctx.lineTo(sx + TILE * 0.55, sy + TILE * 0.72); ctx.stroke();
          ctx.beginPath(); ctx.arc(sx + TILE * 0.62, sy + TILE * 0.74, 2.4, 0, Math.PI * 2); ctx.stroke();
        } else if ((h % 19) === 1) { // moss / theme accent tuft
          ctx.fillStyle = (pal.accent || '#586') + '';
          ctx.globalAlpha = 0.22;
          ctx.beginPath(); ctx.arc(sx + (h % 30), sy + (h % 26) + 6, 3.5, 0, Math.PI * 2); ctx.fill();
          ctx.globalAlpha = 1;
        }
        // overlays
        if (tile === T.TRAP && map.trapRevealed[x + ',' + y]) {
          ctx.strokeStyle = '#e74c3c'; ctx.lineWidth = 2;
          ctx.strokeRect(sx + TILE * 0.28, sy + TILE * 0.28, TILE * 0.44, TILE * 0.44);
          ctx.beginPath(); ctx.moveTo(sx + TILE * 0.28, sy + TILE * 0.28); ctx.lineTo(sx + TILE * 0.72, sy + TILE * 0.72); ctx.stroke();
        } else if (tile === T.DOOR_CLOSED || tile === T.DOOR_OPEN) {
          ctx.fillStyle = '#164';
          ctx.fillStyle = tile === T.DOOR_CLOSED ? '#7a5a3a' : 'rgba(122,90,58,0.45)';
          if (tile === T.DOOR_CLOSED) {
            ctx.fillRect(sx + 4, sy + 2, TILE - 8, TILE - 4);
            ctx.strokeStyle = 'rgba(0,0,0,0.4)'; ctx.lineWidth = 1;
            ctx.strokeRect(sx + 4, sy + 2, TILE - 8, TILE - 4);
            ctx.beginPath(); ctx.moveTo(sx + TILE / 2, sy + 2); ctx.lineTo(sx + TILE / 2, sy + TILE - 2); ctx.stroke();
            ctx.fillStyle = '#d9b36a';
            ctx.beginPath(); ctx.arc(sx + TILE * 0.62, sy + TILE * 0.5, 2.2, 0, Math.PI * 2); ctx.fill();
          } else {
            ctx.fillRect(sx + 2, sy + 2, 6, TILE - 4);
            ctx.fillRect(sx + TILE - 8, sy + 2, 6, TILE - 4);
          }
        } else if (tile === T.STAIRS_DOWN || tile === T.STAIRS_UP) {
          var col = tile === T.STAIRS_DOWN ? '#f1c40f' : '#9ad1ff';
          ctx.fillStyle = 'rgba(0,0,0,0.5)';
          ctx.fillRect(sx + 3, sy + 3, TILE - 6, TILE - 6);
          ctx.fillStyle = col;
          for (var st3 = 0; st3 < 3; st3++) {
            var stw = TILE * (0.66 - st3 * 0.17);
            ctx.fillRect(sx + (TILE - stw) / 2, sy + TILE * 0.26 + st3 * TILE * 0.19, stw, TILE * 0.1);
          }
          ctx.globalAlpha = 0.25 + 0.15 * Math.sin(t * 3);
          ctx.fillRect(sx + 3, sy + 3, TILE - 6, TILE - 6);
          ctx.globalAlpha = 1;
        } else if (tile === T.CHEST || tile === T.CHEST_OPEN) {
          var open = tile === T.CHEST_OPEN;
          ctx.fillStyle = open ? '#5b451a' : '#8a6420';
          ctx.fillRect(sx + TILE * 0.2, sy + TILE * 0.38, TILE * 0.6, TILE * 0.36);
          ctx.fillStyle = open ? '#6d5210' : '#b8860b';
          ctx.fillRect(sx + TILE * 0.2, sy + TILE * (open ? 0.26 : 0.3), TILE * 0.6, TILE * 0.12);
          ctx.fillStyle = open ? '#333' : '#f1c40f';
          ctx.fillRect(sx + TILE * 0.46, sy + TILE * 0.42, TILE * 0.08, TILE * 0.12);
          if (!open) {
            ctx.globalAlpha = 0.2 + 0.15 * Math.sin(t * 4 + x);
            ctx.fillStyle = '#ffe680';
            ctx.fillRect(sx + TILE * 0.16, sy + TILE * 0.24, TILE * 0.68, TILE * 0.54);
            ctx.globalAlpha = 1;
          }
        } else if (tile === T.SHRINE || tile === T.SHRINE_USED) {
          var used = tile === T.SHRINE_USED;
          ctx.fillStyle = used ? '#4a3d5c' : '#8e5fb8';
          ctx.beginPath();
          ctx.moveTo(sx + TILE / 2, sy + TILE * 0.12);
          ctx.lineTo(sx + TILE * 0.74, sy + TILE * 0.82);
          ctx.lineTo(sx + TILE * 0.26, sy + TILE * 0.82);
          ctx.closePath(); ctx.fill();
          ctx.fillStyle = used ? '#382e46' : '#c99df0';
          ctx.beginPath(); ctx.arc(sx + TILE / 2, sy + TILE * 0.4, TILE * 0.09, 0, Math.PI * 2); ctx.fill();
          if (!used) {
            ctx.fillStyle = 'rgba(176,106,224,' + (0.28 + Math.sin(t * 4) * 0.18) + ')';
            ctx.beginPath(); ctx.arc(sx + TILE / 2, sy + TILE * 0.45, TILE * 0.36, 0, Math.PI * 2); ctx.fill();
          }
        } else if (tile === T.BOSS_GATE) {
          ctx.fillStyle = '#2c0b0b';
          ctx.fillRect(sx + 2, sy + 2, TILE - 4, TILE - 4);
          ctx.strokeStyle = '#e74c3c'; ctx.lineWidth = 3;
          ctx.strokeRect(sx + 4, sy + 4, TILE - 8, TILE - 8);
          ctx.fillStyle = '#e74c3c';
          ctx.font = 'bold ' + (TILE * 0.5) + 'px serif';
          ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
          ctx.globalAlpha = 0.7 + 0.3 * Math.sin(t * 5);
          ctx.fillText('☠', sx + TILE / 2, sy + TILE / 2 + 2);
          ctx.globalAlpha = 1;
        } else if (tile === T.PILLAR) {
          ctx.fillStyle = 'rgba(0,0,0,0.35)';
          ctx.beginPath(); ctx.ellipse(sx + TILE / 2, sy + TILE * 0.78, TILE * 0.3, TILE * 0.12, 0, 0, Math.PI * 2); ctx.fill();
          var pg = ctx.createLinearGradient(sx, 0, sx + TILE, 0);
          pg.addColorStop(0, shade(pal.wall, -20));
          pg.addColorStop(0.5, shade(pal.wall, 25));
          pg.addColorStop(1, shade(pal.wall, -25));
          ctx.fillStyle = pg;
          ctx.fillRect(sx + TILE * 0.32, sy + TILE * 0.12, TILE * 0.36, TILE * 0.66);
          ctx.fillStyle = shade(pal.wall, 12);
          ctx.fillRect(sx + TILE * 0.26, sy + TILE * 0.06, TILE * 0.48, TILE * 0.1);
          ctx.fillRect(sx + TILE * 0.26, sy + TILE * 0.74, TILE * 0.48, TILE * 0.1);
        }
      }
    }
  }

  function drawBreakable(br, sx, sy, t) {
    if (br.kind === 'urn') {
      ctx.fillStyle = '#8a6a4a';
      ctx.beginPath();
      ctx.ellipse(sx + TILE / 2, sy + TILE * 0.62, TILE * 0.2, TILE * 0.24, 0, 0, Math.PI * 2);
      ctx.fill();
      ctx.fillStyle = '#6d5238';
      ctx.fillRect(sx + TILE * 0.42, sy + TILE * 0.3, TILE * 0.16, TILE * 0.12);
      ctx.strokeStyle = 'rgba(0,0,0,0.3)';
      ctx.beginPath(); ctx.ellipse(sx + TILE / 2, sy + TILE * 0.62, TILE * 0.13, TILE * 0.16, 0, 0, Math.PI * 2); ctx.stroke();
    } else if (br.kind === 'crate') {
      ctx.fillStyle = '#7a5a35';
      ctx.fillRect(sx + TILE * 0.22, sy + TILE * 0.3, TILE * 0.56, TILE * 0.5);
      ctx.strokeStyle = '#4d3820'; ctx.lineWidth = 2;
      ctx.strokeRect(sx + TILE * 0.22, sy + TILE * 0.3, TILE * 0.56, TILE * 0.5);
      ctx.beginPath(); ctx.moveTo(sx + TILE * 0.22, sy + TILE * 0.3); ctx.lineTo(sx + TILE * 0.78, sy + TILE * 0.8); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(sx + TILE * 0.78, sy + TILE * 0.3); ctx.lineTo(sx + TILE * 0.22, sy + TILE * 0.8); ctx.stroke();
    } else if (br.kind === 'goldenChest') {
      ctx.fillStyle = 'rgba(255,215,80,' + (0.16 + 0.12 * Math.sin(t * 3)) + ')';
      ctx.beginPath(); ctx.arc(sx + TILE / 2, sy + TILE / 2, TILE * 0.5, 0, Math.PI * 2); ctx.fill();
      ctx.fillStyle = '#c9a227';
      ctx.fillRect(sx + TILE * 0.16, sy + TILE * 0.36, TILE * 0.68, TILE * 0.4);
      ctx.fillStyle = '#ffd75e';
      ctx.fillRect(sx + TILE * 0.16, sy + TILE * 0.28, TILE * 0.68, TILE * 0.14);
      ctx.fillStyle = '#7a5c10';
      ctx.fillRect(sx + TILE * 0.44, sy + TILE * 0.42, TILE * 0.12, TILE * 0.16);
      label('🔒', sx + TILE / 2, sy + TILE * 0.24, '#ffd75e');
    }
  }

  // ------------------------------------------------------- hero rendering
  var CLASS_COLORS = {
    warrior: '#e67e22', mage: '#3498db', rogue: '#95a5a6',
    paladin: '#f1c40f', ranger: '#2ecc71', necromancer: '#9b59b6'
  };
  function classColor(classId) { return CLASS_COLORS[classId] || '#b06ae0'; }

  // Anime-styled hero: cel-shaded flat colors, bold outline, spiky class-colored hair, big eyes.
  var HAIR_COLORS = {
    warrior: '#ff8c3a', mage: '#5db9ff', rogue: '#b8c4d6',
    paladin: '#ffe066', ranger: '#5ee08a', necromancer: '#c37aff'
  };
  function drawHero(x, y, classId, t, p) {
    var color = classColor(classId);
    var hair = HAIR_COLORS[classId] || '#d78aff';
    var r = TILE * 0.36;
    var now = Date.now();
    var moving = p && p.fx !== undefined && (Math.abs(p.fx - p.x) > 0.02 || Math.abs(p.fy - p.y) > 0.02);
    var bounce = moving ? Math.abs(Math.sin(t * 14)) * r * 0.12 : Math.sin(t * 2) * r * 0.04;
    y -= bounce;
    ctx.lineJoin = 'round';
    // shadow
    ctx.fillStyle = 'rgba(0,0,0,0.42)';
    ctx.beginPath(); ctx.ellipse(x, y + r * 0.95 + bounce, r * 0.72, r * 0.26, 0, 0, Math.PI * 2); ctx.fill();
    var dirY = p ? p.dirY : 1;
    var dirX = p ? (p.dirX || 0) : 0;
    var swinging = p && p.swingUntil && now < p.swingUntil;
    if (dirY < 0) drawWeapon(x, y, classId, t, p, swinging);
    // cel-shaded tunic body (anime proportions: small body, big head)
    ctx.fillStyle = color;
    ctx.strokeStyle = '#131722'; ctx.lineWidth = 2.5;
    ctx.beginPath();
    ctx.moveTo(x, y - r * 0.4);
    ctx.quadraticCurveTo(x + r * 0.72, y - r * 0.05, x + r * 0.55, y + r * 0.85);
    ctx.lineTo(x - r * 0.55, y + r * 0.85);
    ctx.quadraticCurveTo(x - r * 0.72, y - r * 0.05, x, y - r * 0.4);
    ctx.fill(); ctx.stroke();
    // flat cel shade on body's right
    ctx.fillStyle = 'rgba(0,0,0,0.22)';
    ctx.beginPath();
    ctx.moveTo(x + r * 0.1, y - r * 0.3);
    ctx.quadraticCurveTo(x + r * 0.66, y, x + r * 0.5, y + r * 0.82);
    ctx.lineTo(x + r * 0.12, y + r * 0.82);
    ctx.closePath(); ctx.fill();
    // belt
    ctx.fillStyle = '#2b2333';
    ctx.fillRect(x - r * 0.5, y + r * 0.22, r * 1.0, r * 0.14);
    ctx.fillStyle = '#ffd75e';
    ctx.fillRect(x - r * 0.08, y + r * 0.22, r * 0.16, r * 0.14);
    // big anime head
    var hx = x, hy = y - r * 0.78;
    ctx.fillStyle = '#ffe3c2';
    ctx.strokeStyle = '#131722'; ctx.lineWidth = 2.5;
    ctx.beginPath(); ctx.arc(hx, hy, r * 0.5, 0, Math.PI * 2); ctx.fill(); ctx.stroke();
    // big expressive eyes (track facing)
    var ex = dirX * r * 0.12;
    if (dirY >= 0) {
      ctx.fillStyle = '#fff';
      ctx.beginPath(); ctx.ellipse(hx - r * 0.2 + ex, hy + r * 0.05, r * 0.13, r * 0.17, 0, 0, Math.PI * 2); ctx.fill();
      ctx.beginPath(); ctx.ellipse(hx + r * 0.2 + ex, hy + r * 0.05, r * 0.13, r * 0.17, 0, 0, Math.PI * 2); ctx.fill();
      var iris = { warrior: '#c96a1e', mage: '#2f7fd6', rogue: '#5b6a80', paladin: '#c9a227', ranger: '#2f9e57', necromancer: '#8b46c9' }[classId] || '#7a4ac9';
      ctx.fillStyle = iris;
      ctx.beginPath(); ctx.ellipse(hx - r * 0.19 + ex * 1.4, hy + r * 0.07, r * 0.08, r * 0.11, 0, 0, Math.PI * 2); ctx.fill();
      ctx.beginPath(); ctx.ellipse(hx + r * 0.21 + ex * 1.4, hy + r * 0.07, r * 0.08, r * 0.11, 0, 0, Math.PI * 2); ctx.fill();
      ctx.fillStyle = '#131722';
      ctx.beginPath(); ctx.ellipse(hx - r * 0.19 + ex * 1.4, hy + r * 0.08, r * 0.045, r * 0.06, 0, 0, Math.PI * 2); ctx.fill();
      ctx.beginPath(); ctx.ellipse(hx + r * 0.21 + ex * 1.4, hy + r * 0.08, r * 0.045, r * 0.06, 0, 0, Math.PI * 2); ctx.fill();
      // sparkle highlights
      ctx.fillStyle = '#fff';
      ctx.beginPath(); ctx.arc(hx - r * 0.23 + ex, hy + r * 0.0, r * 0.035, 0, Math.PI * 2); ctx.fill();
      ctx.beginPath(); ctx.arc(hx + r * 0.17 + ex, hy + r * 0.0, r * 0.035, 0, Math.PI * 2); ctx.fill();
      // tiny mouth
      ctx.strokeStyle = '#8a5a3a'; ctx.lineWidth = 1.4;
      ctx.beginPath(); ctx.arc(hx + ex * 0.5, hy + r * 0.26, r * 0.08, 0.2, Math.PI - 0.2); ctx.stroke();
    }
    // spiky anime hair
    ctx.fillStyle = hair;
    ctx.strokeStyle = '#131722'; ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(hx - r * 0.52, hy + r * 0.1);
    ctx.quadraticCurveTo(hx - r * 0.58, hy - r * 0.42, hx - r * 0.3, hy - r * 0.4);
    ctx.lineTo(hx - r * 0.34, hy - r * 0.66);
    ctx.lineTo(hx - r * 0.12, hy - r * 0.44);
    ctx.lineTo(hx - r * 0.02, hy - r * 0.78);
    ctx.lineTo(hx + r * 0.14, hy - r * 0.44);
    ctx.lineTo(hx + r * 0.34, hy - r * 0.62);
    ctx.lineTo(hx + r * 0.32, hy - r * 0.38);
    ctx.quadraticCurveTo(hx + r * 0.58, hy - r * 0.4, hx + r * 0.52, hy + r * 0.1);
    ctx.quadraticCurveTo(hx + r * 0.3, hy - r * 0.18, hx, hy - r * 0.16);
    ctx.quadraticCurveTo(hx - r * 0.3, hy - r * 0.18, hx - r * 0.52, hy + r * 0.1);
    ctx.closePath();
    ctx.fill(); ctx.stroke();
    // hair shine
    ctx.strokeStyle = 'rgba(255,255,255,0.55)';
    ctx.lineWidth = 2;
    ctx.beginPath(); ctx.arc(hx, hy - r * 0.18, r * 0.34, Math.PI * 1.15, Math.PI * 1.6); ctx.stroke();
    if (dirY >= 0) drawWeapon(x, y, classId, t, p, swinging);
  }

  function colorHex(c) { return c[0] === '#' ? c : '#b06ae0'; }

  function drawWeapon(x, y, classId, t, p, swinging) {
    var r = TILE * 0.34;
    var now = Date.now();
    var dirX = p ? (p.dirX || 0) : 0;
    var side = dirX >= 0 ? 1 : -1;
    var baseAng = side > 0 ? -0.5 : Math.PI + 0.5;
    var ang = baseAng;
    if (swinging && p.swingUntil) {
      var prog = 1 - (p.swingUntil - now) / 170;
      ang = baseAng + side * (prog * 2.2 - 0.9);
    }
    var wx = x + side * r * 0.72, wy = y + r * 0.1;
    ctx.save();
    ctx.translate(wx, wy);
    ctx.rotate(ang);
    switch (classId) {
      case 'warrior': // sword
        ctx.fillStyle = '#cfd6e4'; ctx.fillRect(-1.5, -r * 1.15, 3, r * 1.05);
        ctx.fillStyle = '#f1c40f'; ctx.fillRect(-4.5, -r * 0.14, 9, 3);
        ctx.fillStyle = '#7a4a1e'; ctx.fillRect(-1.5, -r * 0.1, 3, r * 0.3);
        break;
      case 'mage': // staff with orb
        ctx.fillStyle = '#7a5a3a'; ctx.fillRect(-1.5, -r * 1.2, 3, r * 1.5);
        ctx.fillStyle = '#7ec8ff';
        ctx.beginPath(); ctx.arc(0, -r * 1.24, 4.2, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = 'rgba(126,200,255,' + (0.3 + 0.2 * Math.sin(t * 6)) + ')';
        ctx.beginPath(); ctx.arc(0, -r * 1.24, 7, 0, Math.PI * 2); ctx.fill();
        break;
      case 'rogue': // dagger
        ctx.fillStyle = '#cfd6e4'; ctx.fillRect(-1.2, -r * 0.7, 2.4, r * 0.62);
        ctx.fillStyle = '#444'; ctx.fillRect(-3.2, -r * 0.1, 6.4, 2.4);
        break;
      case 'paladin': // warhammer
        ctx.fillStyle = '#8a7a5a'; ctx.fillRect(-1.5, -r * 1.05, 3, r * 1.3);
        ctx.fillStyle = '#e8e2cf'; ctx.fillRect(-6, -r * 1.2, 12, r * 0.34);
        break;
      case 'ranger': // bow
        ctx.strokeStyle = '#8a6a3a'; ctx.lineWidth = 2.5;
        ctx.beginPath(); ctx.arc(0, -r * 0.4, r * 0.55, -1.2, 1.2); ctx.stroke();
        ctx.strokeStyle = '#ddd'; ctx.lineWidth = 1;
        ctx.beginPath(); ctx.moveTo(Math.cos(-1.2) * r * 0.55, -r * 0.4 + Math.sin(-1.2) * r * 0.55);
        ctx.lineTo(Math.cos(1.2) * r * 0.55, -r * 0.4 + Math.sin(1.2) * r * 0.55); ctx.stroke();
        break;
      case 'necromancer': // scythe
        ctx.fillStyle = '#4a3a5a'; ctx.fillRect(-1.5, -r * 1.2, 3, r * 1.5);
        ctx.strokeStyle = '#b8ffc8'; ctx.lineWidth = 3;
        ctx.beginPath(); ctx.arc(r * 0.28, -r * 1.1, r * 0.3, Math.PI * 0.8, Math.PI * 1.9); ctx.stroke();
        break;
      default:
        ctx.fillStyle = '#cfd6e4'; ctx.fillRect(-1.5, -r * 0.9, 3, r * 0.9);
    }
    ctx.restore();
    // swing trail
    if (swinging) {
      ctx.strokeStyle = 'rgba(255,255,255,0.35)';
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.arc(x, y, r * 1.15, baseAng - 0.9 * side, ang, side < 0);
      ctx.stroke();
    }
  }

  // ---------------------------------------------------- monster rendering
  function monsterFamily(m) {
    var id = (m.id || '') + ' ' + (m.name || '');
    id = id.toLowerCase();
    if (/slime|ooze/.test(id)) return 'slime';
    if (/\brat\b|giant_rat|rodent/.test(id)) return 'rat';
    if (/bat\b/.test(id)) return 'bat';
    if (/spider/.test(id)) return 'spider';
    if (/wasp|crawler|beetle|scorpion|insect/.test(id)) return 'insect';
    if (/wolf|bear|boar|hound|fox|frost_wolf|dire/.test(id)) return 'beast';
    if (/ghost|wraith|banshee|spirit|shade|specter|phantom/.test(id)) return 'ghost';
    if (/skeleton|bone/.test(id)) return 'skeleton';
    if (/zombie|ghoul|mummy|wendigo/.test(id)) return 'zombie';
    if (/vampire/.test(id)) return 'vampire';
    if (/dragon|drake|wyrm|wyvern/.test(id)) return 'dragon';
    if (/demon|balrog|fiend|succubus|doom|abyssal|infernal|imp\b/.test(id)) return 'demon';
    if (/golem|elemental|gargoyle|statue|guardian|colossus|rock|magma|frozen_knight/.test(id)) return 'golem';
    if (/ent\b|tree|vine|mushroom|shroom|plant|druid/.test(id)) return 'plant';
    if (/lich|sphinx|witch|shaman|necro/.test(id)) return 'lich';
    if (/eye|horror/.test(id)) return 'eye';
    return 'humanoid';
  }

  function drawMonster(m, x, y, t, size) {
    size = size || 1;
    var r = TILE * 0.33 * size;
    var color = m.color || '#c0c0c0';
    var wob = Math.sin(t * 3.2 + (m.x || 0) * 0.7) * r * 0.07;
    // shadow
    ctx.fillStyle = 'rgba(0,0,0,0.4)';
    ctx.beginPath(); ctx.ellipse(x, y + r * 0.92, r * 0.82, r * 0.28, 0, 0, Math.PI * 2); ctx.fill();
    var fam = m._fam || (m._fam = monsterFamily(m));
    ctx.save();
    ctx.translate(0, wob);
    switch (fam) {
      case 'slime': {
        var squish = 1 + Math.sin(t * 5 + x) * 0.12;
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.ellipse(x, y + r * 0.2, r * squish, r * (2 - squish) * 0.62, 0, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = 'rgba(255,255,255,0.35)';
        ctx.beginPath(); ctx.ellipse(x - r * 0.3, y - r * 0.12, r * 0.22, r * 0.14, -0.5, 0, Math.PI * 2); ctx.fill();
        eyes(x, y + r * 0.05, r, 0.3, '#111');
        break;
      }
      case 'rat':
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.ellipse(x, y + r * 0.2, r * 0.85, r * 0.55, 0, 0, Math.PI * 2); ctx.fill();
        ctx.beginPath(); ctx.arc(x + r * 0.62, y, r * 0.36, 0, Math.PI * 2); ctx.fill(); // head
        ctx.beginPath(); ctx.arc(x + r * 0.5, y - r * 0.32, r * 0.16, 0, Math.PI * 2); ctx.fill(); // ear
        ctx.beginPath(); ctx.arc(x + r * 0.78, y - r * 0.3, r * 0.16, 0, Math.PI * 2); ctx.fill();
        ctx.strokeStyle = color; ctx.lineWidth = 2;
        ctx.beginPath(); ctx.moveTo(x - r * 0.8, y + r * 0.2);
        ctx.quadraticCurveTo(x - r * 1.4, y + r * (0.1 + Math.sin(t * 6) * 0.15), x - r * 1.5, y - r * 0.2); ctx.stroke();
        eyes(x + r * 0.66, y - r * 0.05, r * 0.8, 0.25, '#e33');
        break;
      case 'bat': {
        var flap = Math.sin(t * 12 + x) * 0.6;
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.arc(x, y, r * 0.42, 0, Math.PI * 2); ctx.fill();
        [[-1, 1], [1, -1]].forEach(function (sgn) {
          ctx.beginPath();
          ctx.moveTo(x, y);
          ctx.quadraticCurveTo(x + sgn[0] * r * 0.9, y - r * (0.6 + flap * sgn[1] * 0), x + sgn[0] * r * 1.2, y - r * flap * 0.5);
          ctx.quadraticCurveTo(x + sgn[0] * r * 0.7, y + r * 0.2, x, y + r * 0.15);
          ctx.fill();
        });
        eyes(x, y - r * 0.06, r, 0.28, '#ff5');
        break;
      }
      case 'spider':
        ctx.strokeStyle = color; ctx.lineWidth = 2;
        for (var li = 0; li < 4; li++) {
          var la = (li - 1.5) * 0.5 + Math.sin(t * 8 + li) * 0.1;
          [[1], [-1]].forEach(function (sgn) {
            ctx.beginPath();
            ctx.moveTo(x, y);
            ctx.lineTo(x + sgn[0] * Math.cos(la) * r * 1.1, y + Math.sin(la) * r * 0.6 + r * 0.35);
            ctx.stroke();
          });
        }
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.arc(x, y + r * 0.1, r * 0.52, 0, Math.PI * 2); ctx.fill();
        ctx.beginPath(); ctx.arc(x, y - r * 0.4, r * 0.3, 0, Math.PI * 2); ctx.fill();
        eyes(x, y - r * 0.42, r * 0.9, 0.2, '#e33');
        break;
      case 'insect':
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.ellipse(x, y + r * 0.15, r * 0.4, r * 0.62, 0, 0, Math.PI * 2); ctx.fill();
        ctx.globalAlpha = 0.45;
        var wf = Math.sin(t * 20) * 0.5;
        ctx.beginPath(); ctx.ellipse(x - r * 0.5, y - r * 0.2, r * 0.5, r * 0.2, -0.6 - wf, 0, Math.PI * 2); ctx.fill();
        ctx.beginPath(); ctx.ellipse(x + r * 0.5, y - r * 0.2, r * 0.5, r * 0.2, 0.6 + wf, 0, Math.PI * 2); ctx.fill();
        ctx.globalAlpha = 1;
        ctx.fillStyle = '#222';
        for (var st2 = 0; st2 < 3; st2++) {
          ctx.fillRect(x - r * 0.4, y - r * 0.1 + st2 * r * 0.28, r * 0.8, 2);
        }
        eyes(x, y - r * 0.4, r, 0.24, '#111');
        break;
      case 'beast':
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.ellipse(x - r * 0.15, y + r * 0.15, r * 0.72, r * 0.5, 0, 0, Math.PI * 2); ctx.fill();
        ctx.beginPath(); ctx.arc(x + r * 0.55, y - r * 0.15, r * 0.38, 0, Math.PI * 2); ctx.fill();
        // snout + ears
        ctx.beginPath(); ctx.ellipse(x + r * 0.85, y - r * 0.05, r * 0.22, r * 0.14, 0, 0, Math.PI * 2); ctx.fill();
        ctx.beginPath(); ctx.moveTo(x + r * 0.3, y - r * 0.42); ctx.lineTo(x + r * 0.42, y - r * 0.72); ctx.lineTo(x + r * 0.55, y - r * 0.42); ctx.fill();
        ctx.beginPath(); ctx.moveTo(x + r * 0.58, y - r * 0.45); ctx.lineTo(x + r * 0.72, y - r * 0.7); ctx.lineTo(x + r * 0.82, y - r * 0.4); ctx.fill();
        // legs
        ctx.fillRect(x - r * 0.65, y + r * 0.4, r * 0.16, r * 0.5);
        ctx.fillRect(x + r * 0.2, y + r * 0.4, r * 0.16, r * 0.5);
        eyes(x + r * 0.6, y - r * 0.22, r * 0.8, 0.22, '#fd3');
        break;
      case 'ghost': {
        ctx.globalAlpha = 0.75;
        ctx.fillStyle = color;
        ctx.beginPath();
        ctx.arc(x, y - r * 0.2, r * 0.55, Math.PI, 0);
        var wave = t * 6;
        for (var gx = 3; gx >= -3; gx--) {
          ctx.lineTo(x + gx * r * 0.18, y + r * 0.55 + Math.sin(wave + gx) * r * 0.12);
        }
        ctx.closePath(); ctx.fill();
        ctx.globalAlpha = 1;
        eyes(x, y - r * 0.25, r, 0.28, '#0ff');
        break;
      }
      case 'skeleton':
        ctx.fillStyle = '#e8e4d8';
        ctx.beginPath(); ctx.arc(x, y - r * 0.45, r * 0.36, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = '#111';
        ctx.fillRect(x - r * 0.18, y - r * 0.52, r * 0.13, r * 0.14);
        ctx.fillRect(x + r * 0.05, y - r * 0.52, r * 0.13, r * 0.14);
        ctx.fillStyle = '#e8e4d8';
        ctx.fillRect(x - r * 0.06, y - r * 0.1, r * 0.12, r * 0.6); // spine
        for (var rb = 0; rb < 3; rb++) {
          ctx.fillRect(x - r * 0.4, y - r * 0.02 + rb * r * 0.2, r * 0.8, r * 0.07);
        }
        ctx.strokeStyle = '#e8e4d8'; ctx.lineWidth = 2.5;
        ctx.beginPath(); ctx.moveTo(x - r * 0.4, y + r * 0.05); ctx.lineTo(x - r * 0.7, y + r * 0.45); ctx.stroke();
        ctx.beginPath(); ctx.moveTo(x + r * 0.4, y + r * 0.05); ctx.lineTo(x + r * 0.7, y + r * 0.45); ctx.stroke();
        break;
      case 'zombie':
        ctx.fillStyle = color;
        ctx.fillRect(x - r * 0.4, y - r * 0.25, r * 0.8, r * 1.0);
        ctx.beginPath(); ctx.arc(x, y - r * 0.5, r * 0.36, 0, Math.PI * 2); ctx.fill();
        // lurching arms
        ctx.strokeStyle = color; ctx.lineWidth = 4;
        var lurch = Math.sin(t * 3) * 0.1;
        ctx.beginPath(); ctx.moveTo(x - r * 0.4, y - r * 0.1); ctx.lineTo(x - r * 1.0, y - r * (0.15 + lurch)); ctx.stroke();
        ctx.beginPath(); ctx.moveTo(x + r * 0.4, y - r * 0.1); ctx.lineTo(x + r * 1.0, y - r * (0.2 - lurch)); ctx.stroke();
        ctx.fillStyle = 'rgba(0,0,0,0.25)';
        ctx.fillRect(x - r * 0.4, y + r * 0.3, r * 0.8, r * 0.1);
        eyes(x, y - r * 0.52, r, 0.24, '#9f9');
        break;
      case 'vampire':
        ctx.fillStyle = '#1a1a2e';
        ctx.beginPath();
        ctx.moveTo(x, y - r * 0.6);
        ctx.lineTo(x + r * 0.85, y + r * 0.85);
        ctx.lineTo(x - r * 0.85, y + r * 0.85);
        ctx.closePath(); ctx.fill();
        ctx.fillStyle = color;
        ctx.beginPath();
        ctx.moveTo(x, y - r * 0.5);
        ctx.lineTo(x + r * 0.5, y + r * 0.8);
        ctx.lineTo(x - r * 0.5, y + r * 0.8);
        ctx.closePath(); ctx.fill();
        ctx.fillStyle = '#efe0d0';
        ctx.beginPath(); ctx.arc(x, y - r * 0.55, r * 0.3, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = '#fff';
        ctx.fillRect(x - r * 0.1, y - r * 0.42, 2, 4);
        ctx.fillRect(x + r * 0.05, y - r * 0.42, 2, 4);
        eyes(x, y - r * 0.6, r, 0.2, '#e33');
        break;
      case 'dragon': {
        var dflap = Math.sin(t * 6) * 0.3;
        ctx.fillStyle = shade(colorHex(color), -25);
        [[-1], [1]].forEach(function (sgn) {
          ctx.beginPath();
          ctx.moveTo(x, y - r * 0.1);
          ctx.quadraticCurveTo(x + sgn[0] * r * 1.1, y - r * (0.9 + dflap), x + sgn[0] * r * 1.4, y - r * 0.1);
          ctx.quadraticCurveTo(x + sgn[0] * r * 0.8, y + r * 0.15, x, y + r * 0.2);
          ctx.fill();
        });
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.ellipse(x, y + r * 0.1, r * 0.55, r * 0.62, 0, 0, Math.PI * 2); ctx.fill();
        ctx.beginPath(); ctx.arc(x + r * 0.1, y - r * 0.55, r * 0.32, 0, Math.PI * 2); ctx.fill();
        ctx.beginPath(); ctx.ellipse(x + r * 0.42, y - r * 0.5, r * 0.22, r * 0.13, 0.2, 0, Math.PI * 2); ctx.fill();
        // horns
        ctx.strokeStyle = '#e8e0c8'; ctx.lineWidth = 2;
        ctx.beginPath(); ctx.moveTo(x - r * 0.1, y - r * 0.8); ctx.lineTo(x - r * 0.25, y - r * 1.05); ctx.stroke();
        eyes(x + r * 0.18, y - r * 0.6, r, 0.2, '#ff0');
        break;
      }
      case 'demon':
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.ellipse(x, y + r * 0.05, r * 0.58, r * 0.7, 0, 0, Math.PI * 2); ctx.fill();
        // horns
        ctx.strokeStyle = '#2a2a2a'; ctx.lineWidth = 3;
        ctx.beginPath(); ctx.moveTo(x - r * 0.3, y - r * 0.55);
        ctx.quadraticCurveTo(x - r * 0.55, y - r * 0.95, x - r * 0.35, y - r * 1.1); ctx.stroke();
        ctx.beginPath(); ctx.moveTo(x + r * 0.3, y - r * 0.55);
        ctx.quadraticCurveTo(x + r * 0.55, y - r * 0.95, x + r * 0.35, y - r * 1.1); ctx.stroke();
        // wings hint
        ctx.globalAlpha = 0.5;
        ctx.fillStyle = shade(colorHex(color), -40);
        ctx.beginPath(); ctx.moveTo(x - r * 0.4, y); ctx.lineTo(x - r * 1.1, y - r * 0.5); ctx.lineTo(x - r * 0.9, y + r * 0.3); ctx.fill();
        ctx.beginPath(); ctx.moveTo(x + r * 0.4, y); ctx.lineTo(x + r * 1.1, y - r * 0.5); ctx.lineTo(x + r * 0.9, y + r * 0.3); ctx.fill();
        ctx.globalAlpha = 1;
        eyes(x, y - r * 0.25, r, 0.28, '#ff3');
        break;
      case 'golem': {
        ctx.fillStyle = color;
        roundRectPath(x - r * 0.55, y - r * 0.5, r * 1.1, r * 1.15, r * 0.18);
        ctx.fill();
        ctx.fillStyle = shade(colorHex(color), -30);
        roundRectPath(x - r * 0.72, y - r * 0.15, r * 0.28, r * 0.7, r * 0.1); ctx.fill();
        roundRectPath(x + r * 0.44, y - r * 0.15, r * 0.28, r * 0.7, r * 0.1); ctx.fill();
        ctx.strokeStyle = 'rgba(0,0,0,0.35)'; ctx.lineWidth = 1.5;
        ctx.beginPath(); ctx.moveTo(x - r * 0.3, y - r * 0.2); ctx.lineTo(x - r * 0.05, y + r * 0.1); ctx.lineTo(x - r * 0.2, y + r * 0.45); ctx.stroke();
        eyes(x, y - r * 0.22, r, 0.26, '#8ef');
        break;
      }
      case 'plant':
        ctx.strokeStyle = shade(colorHex(color), -20); ctx.lineWidth = 3;
        for (var vi = -1; vi <= 1; vi++) {
          ctx.beginPath();
          ctx.moveTo(x + vi * r * 0.25, y + r * 0.6);
          ctx.quadraticCurveTo(x + vi * r * (0.6 + Math.sin(t * 3 + vi) * 0.2), y - r * 0.2, x + vi * r * 0.45, y - r * (0.7 + Math.sin(t * 2.5 + vi) * 0.15));
          ctx.stroke();
        }
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.ellipse(x, y + r * 0.35, r * 0.5, r * 0.4, 0, 0, Math.PI * 2); ctx.fill();
        eyes(x, y + r * 0.25, r, 0.24, '#ff6');
        break;
      case 'lich':
        ctx.fillStyle = shade(colorHex(color), -25);
        ctx.beginPath();
        ctx.moveTo(x, y - r * 0.55);
        ctx.lineTo(x + r * 0.6, y + r * 0.85);
        ctx.lineTo(x - r * 0.6, y + r * 0.85);
        ctx.closePath(); ctx.fill();
        ctx.fillStyle = '#dcd6c8';
        ctx.beginPath(); ctx.arc(x, y - r * 0.55, r * 0.32, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = '#111';
        ctx.fillRect(x - r * 0.16, y - r * 0.62, r * 0.12, r * 0.13);
        ctx.fillRect(x + r * 0.05, y - r * 0.62, r * 0.12, r * 0.13);
        // floating orbs
        for (var oi = 0; oi < 2; oi++) {
          var oa = t * 2.4 + oi * Math.PI;
          ctx.fillStyle = 'rgba(140,255,190,0.8)';
          ctx.beginPath(); ctx.arc(x + Math.cos(oa) * r * 0.9, y - r * 0.1 + Math.sin(oa) * r * 0.4, 3, 0, Math.PI * 2); ctx.fill();
        }
        break;
      case 'eye':
        ctx.fillStyle = color;
        ctx.beginPath(); ctx.arc(x, y, r * 0.6, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = '#fff';
        ctx.beginPath(); ctx.ellipse(x, y, r * 0.36, r * 0.28, 0, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = '#8a1c1c';
        var lookX = Math.sin(t * 1.7) * r * 0.12;
        ctx.beginPath(); ctx.arc(x + lookX, y, r * 0.14, 0, Math.PI * 2); ctx.fill();
        // tendrils
        ctx.strokeStyle = color; ctx.lineWidth = 2;
        for (var ti = 0; ti < 4; ti++) {
          var ta = ti * 1.6 + t;
          ctx.beginPath();
          ctx.moveTo(x + Math.cos(ta) * r * 0.55, y + Math.sin(ta) * r * 0.55);
          ctx.lineTo(x + Math.cos(ta) * r * 0.95, y + Math.sin(ta) * r * 0.95);
          ctx.stroke();
        }
        break;
      default: { // humanoid: body + head + weapon arm
        ctx.fillStyle = color;
        ctx.beginPath();
        ctx.moveTo(x, y - r * 0.55);
        ctx.quadraticCurveTo(x + r * 0.62, y - r * 0.1, x + r * 0.5, y + r * 0.8);
        ctx.lineTo(x - r * 0.5, y + r * 0.8);
        ctx.quadraticCurveTo(x - r * 0.62, y - r * 0.1, x, y - r * 0.55);
        ctx.fill();
        ctx.fillStyle = shade(colorHex(color), 30);
        ctx.beginPath(); ctx.arc(x, y - r * 0.6, r * 0.32, 0, Math.PI * 2); ctx.fill();
        // crude club/blade
        ctx.strokeStyle = shade(colorHex(color), -45); ctx.lineWidth = 3;
        ctx.beginPath(); ctx.moveTo(x + r * 0.5, y); ctx.lineTo(x + r * 0.95, y - r * 0.5); ctx.stroke();
        eyes(x, y - r * 0.62, r, 0.22, '#111');
      }
    }
    ctx.restore();
  }

  function eyes(x, y, r, spread, color) {
    ctx.fillStyle = color;
    ctx.beginPath(); ctx.arc(x - r * spread, y, Math.max(1.4, r * 0.07), 0, Math.PI * 2); ctx.fill();
    ctx.beginPath(); ctx.arc(x + r * spread, y, Math.max(1.4, r * 0.07), 0, Math.PI * 2); ctx.fill();
  }

  function roundRectPath(x, y, w, h, r) {
    ctx.beginPath();
    ctx.moveTo(x + r, y);
    ctx.arcTo(x + w, y, x + w, y + h, r);
    ctx.arcTo(x + w, y + h, x, y + h, r);
    ctx.arcTo(x, y + h, x, y, r);
    ctx.arcTo(x, y, x + w, y, r);
    ctx.closePath();
  }

  // ------------------------------------------------------- item / ui bits
  function drawItemGlyph(g2, x, y, t) {
    var def = SC.entities.itemDef(g2.id) || {};
    var kindGlyphs = { potion: '!', scroll: '?', weapon: '/', shield: ')', armor: '[', helmet: '^', gloves: '{', boots: 'b', ring: 'o', amulet: '"', food: '%', special: '*', material: '◆' };
    var rc = rarityColor(g2.rarity);
    // glow for rare+
    if (g2.rarity && ['epic', 'legendary', 'mythic'].indexOf(g2.rarity) >= 0) {
      ctx.fillStyle = rc;
      ctx.globalAlpha = 0.18 + 0.1 * Math.sin(t * 4);
      ctx.beginPath(); ctx.arc(x, y, TILE * 0.42, 0, Math.PI * 2); ctx.fill();
      ctx.globalAlpha = 1;
    }
    ctx.fillStyle = 'rgba(0,0,0,0.55)';
    ctx.beginPath(); ctx.arc(x, y, TILE * 0.26, 0, Math.PI * 2); ctx.fill();
    ctx.strokeStyle = rc; ctx.lineWidth = 2;
    ctx.beginPath(); ctx.arc(x, y, TILE * 0.26, 0, Math.PI * 2); ctx.stroke();
    ctx.fillStyle = g2.gold ? '#f1c40f' : (def.color || '#f1c40f');
    ctx.font = 'bold ' + Math.round(TILE * 0.34) + 'px monospace';
    ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
    ctx.fillText(g2.gold ? '$' : (def.glyph || kindGlyphs[def.kind] || '*'), x, y + 1);
  }

  function rarityColor(r) {
    return { common: '#95a5a6', uncommon: '#2ecc71', rare: '#3498db', epic: '#d35ded', legendary: '#f1c40f', mythic: '#e74c3c' }[r || 'common'] || '#95a5a6';
  }

  function hpBar(x, y, frac, w) {
    frac = U.clamp(frac, 0, 1);
    if (frac >= 1) return;
    ctx.fillStyle = 'rgba(0,0,0,0.6)';
    ctx.fillRect(x - w / 2, y, w, 4);
    ctx.fillStyle = frac > 0.5 ? '#2ecc71' : (frac > 0.25 ? '#f1c40f' : '#e74c3c');
    ctx.fillRect(x - w / 2, y, w * frac, 4);
  }

  function label(txt, x, y, color) {
    ctx.font = 'bold 11px sans-serif';
    ctx.textAlign = 'center'; ctx.textBaseline = 'bottom';
    ctx.fillStyle = 'rgba(0,0,0,0.6)';
    var tw = ctx.measureText(txt).width;
    ctx.fillRect(x - tw / 2 - 4, y - 13, tw + 8, 14);
    ctx.fillStyle = color || '#fff';
    ctx.fillText(txt, x, y);
  }

  function drawBossBar(boss) {
    var w = Math.min(420, vw * 0.7);
    var x = vw / 2 - w / 2, y = 58;
    ctx.fillStyle = 'rgba(5,7,12,0.8)';
    ctx.fillRect(x - 4, y - 20, w + 8, 40);
    ctx.strokeStyle = boss.enraged ? '#ff5040' : '#8a2be2';
    ctx.lineWidth = 1.5;
    ctx.strokeRect(x - 4, y - 20, w + 8, 40);
    ctx.font = 'bold 13px serif';
    ctx.textAlign = 'center'; ctx.textBaseline = 'bottom';
    ctx.fillStyle = '#ffd35c';
    ctx.fillText('☠ ' + boss.name + (boss.enraged ? ' — ENRAGED' : ''), vw / 2, y - 2);
    var frac = U.clamp(boss.hp / boss.maxHp, 0, 1);
    ctx.fillStyle = '#3a0d0d';
    ctx.fillRect(x, y + 2, w, 12);
    var grad = ctx.createLinearGradient(x, 0, x + w, 0);
    grad.addColorStop(0, '#ff7060'); grad.addColorStop(1, '#c0392b');
    ctx.fillStyle = grad;
    ctx.fillRect(x, y + 2, w * frac, 12);
    ctx.fillStyle = '#fff';
    ctx.font = '10px monospace';
    ctx.textBaseline = 'middle';
    ctx.fillText(Math.max(0, Math.round(boss.hp)) + ' / ' + boss.maxHp, vw / 2, y + 8);
  }

  function drawObjectiveArrow(map, p, camX, camY) {
    if (!map.stairsDown || !map.explored[map.stairsDown.x + ',' + map.stairsDown.y]) return;
    var sx = map.stairsDown.x * TILE - camX + TILE / 2, sy = map.stairsDown.y * TILE - camY + TILE / 2;
    if (sx > 40 && sx < vw - 40 && sy > 40 && sy < vh - 40) return; // on screen
    var cx = vw / 2, cy = vh / 2;
    var ang = Math.atan2(sy - cy, sx - cx);
    var ex = cx + Math.cos(ang) * (Math.min(vw, vh) * 0.38);
    var ey = cy + Math.sin(ang) * (Math.min(vw, vh) * 0.38);
    ctx.save();
    ctx.translate(ex, ey);
    ctx.rotate(ang);
    ctx.fillStyle = 'rgba(241,196,15,0.85)';
    ctx.beginPath();
    ctx.moveTo(10, 0); ctx.lineTo(-6, -7); ctx.lineTo(-2, 0); ctx.lineTo(-6, 7);
    ctx.closePath(); ctx.fill();
    ctx.restore();
  }

  function drawSparksAt(camX, camY, scale) {
    for (var i = 0; i < sparks.length; i++) {
      var s = sparks[i];
      ctx.globalAlpha = Math.min(1, s.ttl / 300);
      ctx.fillStyle = s.color;
      ctx.fillRect(s.x * scale - camX - 2, s.y * scale - camY - 2, 4, 4);
    }
    ctx.globalAlpha = 1;
  }

  function drawFloatsAt(camX, camY, scale) {
    ctx.font = 'bold 15px sans-serif';
    ctx.textAlign = 'center';
    for (var i = 0; i < floats.length; i++) {
      var f = floats[i];
      var fx = f.x * scale - camX + scale / 2, fy = f.y * scale - camY;
      ctx.globalAlpha = Math.min(1, f.ttl / 400);
      ctx.fillStyle = '#000';
      ctx.fillText(f.txt, fx + 1, fy + 1);
      ctx.fillStyle = f.color;
      ctx.fillText(f.txt, fx, fy);
    }
    ctx.globalAlpha = 1;
  }

  function drawMinimap(map, p, monsters) {
    var mw = Math.min(120, map.w * 2), s = mw / map.w;
    var mh = map.h * s;
    var mx = vw - mw - 10, my = 104;
    ctx.fillStyle = 'rgba(5,7,12,0.75)';
    ctx.fillRect(mx - 3, my - 3, mw + 6, mh + 6);
    for (var y = 0; y < map.h; y++) {
      for (var x = 0; x < map.w; x++) {
        var key = x + ',' + y;
        if (!map.explored[key]) continue;
        var tile = map.get(x, y);
        if (tile === T.WALL) continue;
        ctx.fillStyle = tile === T.STAIRS_DOWN ? '#f1c40f' :
          (tile === T.LAVA ? '#a33' : (tile === T.WATER ? '#369' : 'rgba(160,170,200,0.5)'));
        ctx.fillRect(mx + x * s, my + y * s, Math.max(1, s), Math.max(1, s));
      }
    }
    for (var mi = 0; mi < monsters.length; mi++) {
      var m = monsters[mi];
      if (m.hp <= 0 || !map.visible[m.x + ',' + m.y]) continue;
      ctx.fillStyle = m.boss ? '#ff4030' : '#e08050';
      ctx.fillRect(mx + m.x * s - 1, my + m.y * s - 1, 2.5, 2.5);
    }
    ctx.fillStyle = '#fff';
    ctx.fillRect(mx + p.x * s - 1.5, my + p.y * s - 1.5, 3.5, 3.5);
  }

  // ---------------------------------------------------------------- haven
  function renderHaven(st, dtMs) {
    tickFx(dtMs);
    var h = st.haven;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    var grad = ctx.createLinearGradient(0, 0, 0, vh);
    grad.addColorStop(0, '#151329');
    grad.addColorStop(0.5, '#101c14');
    grad.addColorStop(1, '#0a140d');
    ctx.fillStyle = grad;
    ctx.fillRect(0, 0, vw, vh);
    // stars
    var t = Date.now() / 1000;
    for (var si = 0; si < 24; si++) {
      var sh = U.hashStr('star' + si);
      ctx.fillStyle = 'rgba(255,255,255,' + (0.15 + 0.12 * Math.sin(t + si)) + ')';
      ctx.fillRect((sh % vw), ((sh >> 8) % Math.max(1, Math.round(vh * 0.22))), 2, 2);
    }

    var gw = SC.haven.GRID_W, gh = SC.haven.GRID_H;
    var cell = Math.min((vw - 20) / gw, (vh - 190) / gh);
    var ox = (vw - gw * cell) / 2, oy = 76;
    st._havenLayout = { ox: ox, oy: oy, cell: cell };

    var x, y;
    for (y = 0; y < gh; y++) {
      for (x = 0; x < gw; x++) {
        ctx.fillStyle = ((x + y) % 2 === 0) ? '#182a1c' : '#152518';
        ctx.fillRect(ox + x * cell, oy + y * cell, cell - 1, cell - 1);
        var dh = hashXY(x, y, 99);
        if (dh % 11 === 0) {
          ctx.fillStyle = 'rgba(90,140,90,0.25)';
          ctx.fillRect(ox + x * cell + (dh % 20), oy + y * cell + ((dh >> 4) % 20), 3, 3);
        }
      }
    }
    if (st.buildPlacing) {
      var chk = SC.haven.canPlace(h, st.buildPlacing.type, st.buildPlacing.x, st.buildPlacing.y);
      var d = SC.haven.bdef(st.buildPlacing.type) || { size: 1 };
      ctx.fillStyle = chk.ok ? 'rgba(46,204,113,0.35)' : 'rgba(231,76,60,0.35)';
      ctx.fillRect(ox + st.buildPlacing.x * cell, oy + st.buildPlacing.y * cell, cell * d.size, cell * d.size);
    }
    var now = Date.now();
    for (var i = 0; i < h.buildings.length; i++) {
      var b = h.buildings[i];
      var d2 = SC.haven.bdef(b.type) || { size: 1, icon: '❓' };
      var bx = ox + b.x * cell, by = oy + b.y * cell, bs = cell * d2.size;
      ctx.fillStyle = st.selectedBuilding === b ? '#2c3b57' : '#1d2740';
      roundRectPath(bx + 2, by + 2, bs - 4, bs - 4, 8);
      ctx.fill();
      ctx.strokeStyle = st.selectedBuilding === b ? '#f1c40f' : 'rgba(0,0,0,0.4)';
      ctx.lineWidth = st.selectedBuilding === b ? 2 : 1;
      roundRectPath(bx + 2, by + 2, bs - 4, bs - 4, 8);
      ctx.stroke();
      ctx.font = Math.round(bs * 0.44) + 'px serif';
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
      ctx.fillText(d2.icon, bx + bs / 2, by + bs / 2 - bs * 0.06);
      ctx.fillStyle = '#0b0e14';
      ctx.fillRect(bx + 4, by + 4, 22, 13);
      ctx.fillStyle = '#f1c40f';
      ctx.font = 'bold 10px sans-serif';
      ctx.textAlign = 'left'; ctx.textBaseline = 'top';
      ctx.fillText('L' + b.level, bx + 6, by + 6);
      var pend = SC.haven.pendingProduction(b, now);
      if (pend > 0) {
        ctx.fillStyle = '#f1c40f';
        ctx.beginPath(); ctx.arc(bx + bs - 9, by + 9, 5 + Math.sin(t * 4) * 1.5, 0, Math.PI * 2); ctx.fill();
      }
      if (b.type === 'farmPlot' && b.crop) {
        var prog = SC.haven.cropProgress(b, now);
        var cd = SC.haven.cropDef(b.crop.id) || { icon: '🌱' };
        ctx.font = Math.round(bs * (0.2 + prog * 0.22)) + 'px serif';
        ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
        ctx.fillText(prog >= 1 ? cd.icon : (prog > 0.5 ? '🌿' : '🌱'), bx + bs / 2, by + bs * 0.62);
        ctx.strokeStyle = prog >= 1 ? '#2ecc71' : '#f1c40f';
        ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.arc(bx + bs / 2, by + bs / 2, bs * 0.42, -Math.PI / 2, -Math.PI / 2 + prog * Math.PI * 2);
        ctx.stroke();
        if (!b.crop.watered && prog < 1) {
          ctx.font = Math.round(bs * 0.22) + 'px serif';
          ctx.fillText('💧', bx + bs * 0.78, by + bs * 0.24);
        }
      }
    }
    drawFloatsAt(-ox, -oy, cell);
  }

  // ---------------------------------------------------------------- siege
  function renderSiege(st, dtMs) {
    renderHaven(st, dtMs); // base view underneath
    var sg = st.siege;
    if (!sg) return;
    var lay = st._havenLayout;
    var ox = lay.ox + shakeX, oy = lay.oy + shakeY, cell = lay.cell;
    var t = Date.now() / 1000;

    // dark battle tint
    ctx.fillStyle = 'rgba(40,5,10,0.16)';
    ctx.fillRect(0, 0, vw, vh);

    // attackers
    for (var i = 0; i < sg.attackers.length; i++) {
      var a = sg.attackers[i];
      if (a.hp <= 0) continue;
      var ax = ox + a.x * cell + cell / 2, ay = oy + a.y * cell + cell / 2;
      drawMonster(a, ax, ay, t, 0.8);
      hpBar(ax, ay - cell * 0.5, a.hp / a.maxHp, cell * 0.6);
    }
    // tower shots
    for (var s2 = 0; s2 < sg.shots.length; s2++) {
      var sh = sg.shots[s2];
      ctx.strokeStyle = 'rgba(140,220,255,' + (sh.ttl / 200) + ')';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(ox + sh.x1 * cell + cell / 2, oy + sh.y1 * cell + cell / 2);
      ctx.lineTo(ox + sh.x2 * cell + cell / 2, oy + sh.y2 * cell + cell / 2);
      ctx.stroke();
    }
    // player hero on the field
    var p = st.player;
    var hx = ox + sg.hero.x * cell + cell / 2, hy = oy + sg.hero.y * cell + cell / 2;
    drawHero(hx, hy, p.classId, t, { dirX: sg.hero.dirX, dirY: sg.hero.dirY, swingUntil: sg.hero.swingUntil, fx: sg.hero.x, fy: sg.hero.y, x: sg.hero.x, y: sg.hero.y });

    drawSparksAt(-ox, -oy, cell);
    drawFloatsAt(-ox, -oy, cell);

    // keep HP + wave banner
    var w = Math.min(380, vw * 0.7);
    var bx = vw / 2 - w / 2, by = 58;
    ctx.fillStyle = 'rgba(5,7,12,0.8)';
    ctx.fillRect(bx - 4, by - 18, w + 8, 36);
    ctx.font = 'bold 12px sans-serif';
    ctx.textAlign = 'center'; ctx.textBaseline = 'bottom';
    ctx.fillStyle = '#ffd35c';
    ctx.fillText('🏰 SIEGE — Wave ' + sg.wave + '/' + sg.maxWaves + ' · enemies: ' + sg.attackers.filter(function (a2) { return a2.hp > 0; }).length, vw / 2, by - 1);
    var frac = U.clamp(sg.keepHp / sg.keepMaxHp, 0, 1);
    ctx.fillStyle = '#3a0d0d';
    ctx.fillRect(bx, by + 2, w, 10);
    ctx.fillStyle = frac > 0.5 ? '#2ecc71' : (frac > 0.25 ? '#f1c40f' : '#e74c3c');
    ctx.fillRect(bx, by + 2, w * frac, 10);
  }

  // ---------------------------------------------------------------- arena
  function renderArena(dtMs) {
    tickFx(dtMs);
    var A = SC.arena.state();
    if (!A) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    var bgrad = ctx.createLinearGradient(0, 0, 0, vh);
    bgrad.addColorStop(0, '#140b1e');
    bgrad.addColorStop(1, '#0a0712');
    ctx.fillStyle = bgrad;
    ctx.fillRect(0, 0, vw, vh);
    var cell = Math.min(vw / A.map.w, (vh - 140) / A.map.h);
    var ox = (vw - A.map.w * cell) / 2 + shakeX, oy = 70 + shakeY;
    var t = Date.now() / 1000;

    for (var y = 0; y < A.map.h; y++) {
      for (var x = 0; x < A.map.w; x++) {
        if (A.map.solid[y * A.map.w + x]) {
          ctx.fillStyle = '#332752';
          ctx.fillRect(ox + x * cell, oy + y * cell, cell, cell);
          ctx.fillStyle = 'rgba(255,255,255,0.06)';
          ctx.fillRect(ox + x * cell, oy + y * cell, cell, 3);
          ctx.fillStyle = 'rgba(0,0,0,0.35)';
          ctx.fillRect(ox + x * cell, oy + (y + 1) * cell - 4, cell, 4);
        } else {
          ctx.fillStyle = ((x + y) % 2 === 0) ? '#191228' : '#161024';
          ctx.fillRect(ox + x * cell, oy + y * cell, cell, cell);
        }
      }
    }

    for (var pi = 0; pi < A.powerups.length; pi++) {
      var pw = A.powerups[pi];
      ctx.fillStyle = 'rgba(255,255,255,0.1)';
      ctx.beginPath(); ctx.arc(ox + pw.x * cell, oy + pw.y * cell, cell * 0.5, 0, Math.PI * 2); ctx.fill();
      ctx.font = Math.round(cell * 0.7) + 'px serif';
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
      ctx.fillText(pw.def.icon, ox + pw.x * cell, oy + pw.y * cell + Math.sin(t * 4 + pi) * 3);
    }

    var now = Date.now();
    for (var bi = 0; bi < A.bombs.length; bi++) {
      var b = A.bombs[bi];
      var flash = (b.at - now) < 500 && Math.floor(now / 100) % 2 === 0;
      ctx.font = Math.round(cell * (b.mega ? 0.9 : 0.65)) + 'px serif';
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
      ctx.globalAlpha = flash ? 0.5 : 1;
      ctx.fillText('💣', ox + b.x * cell, oy + b.y * cell);
      ctx.globalAlpha = 1;
    }

    var all = [A.me];
    for (var id in A.fighters) all.push(A.fighters[id]);
    for (var fi = 0; fi < all.length; fi++) {
      var f = all[fi];
      if (f.hp <= 0 || now < f.respawnAt) continue;
      var fx = ox + f.x * cell, fy = oy + f.y * cell;
      if (now < f.shieldUntil) {
        ctx.strokeStyle = 'rgba(130,200,255,0.8)';
        ctx.lineWidth = 2;
        ctx.beginPath(); ctx.arc(fx, fy, cell * 0.5, 0, Math.PI * 2); ctx.stroke();
      }
      // mini hero
      var savedTile = TILE; TILE = cell;
      drawHero(fx, fy, f === A.me ? (SC.game && SC.game.state.player ? SC.game.state.player.classId : 'warrior') : (f.bot ? 'necromancer' : 'rogue'), t, { dirX: f.aimX, dirY: f.aimY, fx: f.x, fy: f.y, x: f.x, y: f.y });
      TILE = savedTile;
      ctx.strokeStyle = 'rgba(255,255,255,0.5)';
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.moveTo(fx + f.aimX * cell * 0.4, fy + f.aimY * cell * 0.4);
      ctx.lineTo(fx + f.aimX * cell * 0.6, fy + f.aimY * cell * 0.6);
      ctx.stroke();
      label(f.name + ' · ' + f.score, fx, fy - cell * 0.62, f === A.me ? '#ffd35c' : '#dfe6f5');
      hpBar(fx, fy - cell * 0.55, f.hp / ((SC.DATA.arena && SC.DATA.arena.hp) || 100), cell * 0.8);
    }

    for (var pri = 0; pri < A.projectiles.length; pri++) {
      var pr = A.projectiles[pri];
      ctx.fillStyle = pr.color || '#ffd35c';
      ctx.beginPath(); ctx.arc(ox + pr.x * cell, oy + pr.y * cell, cell * 0.1, 0, Math.PI * 2); ctx.fill();
    }
    for (var pa = 0; pa < A.particles.length; pa++) {
      var pt = A.particles[pa];
      ctx.globalAlpha = Math.min(1, pt.ttl / 300);
      ctx.fillStyle = pt.color;
      ctx.fillRect(ox + pt.x * cell - 2, oy + pt.y * cell - 2, 4, 4);
    }
    ctx.globalAlpha = 1;

    var remain = Math.max(0, A.endAt - now);
    ctx.fillStyle = 'rgba(5,7,12,0.7)';
    ctx.fillRect(vw / 2 - 60, oy - 34 - shakeY, 120, 26);
    ctx.fillStyle = remain < 15000 ? '#e74c3c' : '#f1c40f';
    ctx.font = 'bold 16px monospace';
    ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
    ctx.fillText(A.over ? 'MATCH OVER' : U.fmtTime(remain), vw / 2, oy - 21 - shakeY);
    A._layout = { ox: ox, oy: oy, cell: cell };
  }

  // ------------------------------------------------- sprite baking (for 3D billboards)
  var spriteCache = {};
  function bakeSprite(key, sizePx, drawFn) {
    if (spriteCache[key]) return spriteCache[key];
    var c = mkCanvas(sizePx);
    var g = c.getContext('2d');
    var saved = ctx, savedTile = TILE;
    ctx = g; TILE = sizePx * 0.9;
    try { drawFn(g, sizePx); } finally { ctx = saved; TILE = savedTile; }
    // cel outline: stamp dark silhouette at 8 offsets beneath the sprite
    var sil = mkCanvas(sizePx);
    var sg = sil.getContext('2d');
    sg.drawImage(c, 0, 0);
    sg.globalCompositeOperation = 'source-in';
    sg.fillStyle = '#10131d';
    sg.fillRect(0, 0, sizePx, sizePx);
    var outlined = mkCanvas(sizePx);
    var og = outlined.getContext('2d');
    var o = Math.max(1, Math.round(sizePx / 48));
    [[o, 0], [-o, 0], [0, o], [0, -o], [o, o], [-o, -o], [o, -o], [-o, o]].forEach(function (d) {
      og.drawImage(sil, d[0], d[1]);
    });
    og.drawImage(c, 0, 0);
    spriteCache[key] = outlined;
    return outlined;
  }

  function spriteMonster(m, frame) {
    var fam = m._fam || monsterFamily(m);
    var key = 'm:' + fam + ':' + (m.color || '') + ':' + (m.boss ? 'b' : m.miniBoss ? 'mb' : '') + ':' + frame;
    return bakeSprite(key, 144, function (g, s) {
      drawMonster(m, s / 2, s / 2, frame * 0.9 + 0.3, m.boss ? 1.15 : 1);
    });
  }

  function spriteHero(classId, frame) {
    return bakeSprite('h:' + classId + ':' + frame, 144, function (g, s) {
      drawHero(s / 2, s / 2 + s * 0.08, classId, frame * 0.8 + 0.2, { dirX: 0, dirY: 1, x: 0, y: 0, fx: 0, fy: 0 });
    });
  }

  function spriteGlyph(key, emoji, sizePx) {
    return bakeSprite('g:' + key, sizePx || 72, function (g, s) {
      g.font = Math.round(s * 0.7) + 'px serif';
      g.textAlign = 'center'; g.textBaseline = 'middle';
      g.fillText(emoji, s / 2, s / 2);
    });
  }

  return {
    init: init, resize: resize,
    renderCrypt: renderCrypt, renderHaven: renderHaven, renderArena: renderArena, renderSiege: renderSiege,
    bakeSprite: bakeSprite, spriteMonster: spriteMonster, spriteHero: spriteHero, spriteGlyph: spriteGlyph,
    shadeColor: shade,
    floatText: floatText, burst: burst, shake: shake, setShakeEnabled: setShakeEnabled,
    fxSettings: function () { return settingsFx; },
    tileSize: function () { return TILE; },
    camera: function () { return cam; },
    viewport: function () { return { w: vw, h: vh }; },
    rarityColor: rarityColor,
    classColor: classColor,
    drawMonster: drawMonster
  };
})();

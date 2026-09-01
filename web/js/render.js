'use strict';
/* ShadowCrypt Online — canvas renderer: crypt, haven, farm and arena views */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.render = (function () {
  var U = SC.util, T = SC.TILE;

  var canvas = null, ctx = null;
  var dpr = 1, vw = 0, vh = 0;
  var cam = { x: 0, y: 0 };
  var TILE = 40;               // css pixels per tile (crypt)
  var floats = [];             // floating combat text
  var sparks = [];             // particles

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
    // scale tiles for small screens
    TILE = vw < 480 ? 34 : (vw < 900 ? 40 : 46);
  }

  function floatText(x, y, txt, color) {
    floats.push({ x: x, y: y, txt: txt, color: color || '#fff', ttl: 900, vy: -1.4 });
  }
  function burst(x, y, color, n) {
    for (var i = 0; i < (n || 8); i++) {
      var a = Math.random() * Math.PI * 2, s = 0.5 + Math.random() * 2;
      sparks.push({ x: x + 0.5, y: y + 0.5, vx: Math.cos(a) * s, vy: Math.sin(a) * s, ttl: 400 + Math.random() * 300, color: color || '#fff' });
    }
  }

  function tickFx(dtMs) {
    var i;
    for (i = floats.length - 1; i >= 0; i--) {
      var f = floats[i];
      f.ttl -= dtMs; f.y += f.vy * dtMs / 1000;
      if (f.ttl <= 0) floats.splice(i, 1);
    }
    for (i = sparks.length - 1; i >= 0; i--) {
      var s = sparks[i];
      s.ttl -= dtMs; s.x += s.vx * dtMs / 1000; s.y += s.vy * dtMs / 1000;
      if (s.ttl <= 0) sparks.splice(i, 1);
    }
  }

  // ---------------- Crypt -------------------------------------------------
  function renderCrypt(st, dtMs) {
    tickFx(dtMs);
    var map = st.map, p = st.player;
    var pal = (map.theme && map.theme.palette) || { wall: '#3a3f4d', floor: '#20242e', accent: '#556' };
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.fillStyle = '#05070c';
    ctx.fillRect(0, 0, vw, vh);

    // camera follows player smoothly (uses interpolated position)
    var targX = p.fx * TILE - vw / 2 + TILE / 2;
    var targY = p.fy * TILE - vh / 2 + TILE / 2;
    cam.x = U.lerp(cam.x, targX, Math.min(1, dtMs / 120));
    cam.y = U.lerp(cam.y, targY, Math.min(1, dtMs / 120));

    var x0 = Math.max(0, Math.floor(cam.x / TILE) - 1);
    var y0 = Math.max(0, Math.floor(cam.y / TILE) - 1);
    var x1 = Math.min(map.w - 1, Math.ceil((cam.x + vw) / TILE) + 1);
    var y1 = Math.min(map.h - 1, Math.ceil((cam.y + vh) / TILE) + 1);

    var t = Date.now() / 1000;
    for (var y = y0; y <= y1; y++) {
      for (var x = x0; x <= x1; x++) {
        var key = x + ',' + y;
        var visible = !!map.visible[key];
        var explored = !!map.explored[key];
        if (!visible && !explored) continue;
        var tile = map.get(x, y);
        var sx = x * TILE - cam.x, sy = y * TILE - cam.y;
        drawTile(tile, sx, sy, pal, t, map, x, y);
        if (!visible) { // memory fog
          ctx.fillStyle = 'rgba(4,6,12,0.62)';
          ctx.fillRect(sx, sy, TILE, TILE);
        }
      }
    }

    // ground items
    for (var gi = 0; gi < st.groundItems.length; gi++) {
      var g = st.groundItems[gi];
      if (!map.visible[g.x + ',' + g.y]) continue;
      var gsx = g.x * TILE - cam.x, gsy = g.y * TILE - cam.y;
      var bob = Math.sin(t * 3 + g.x + g.y) * 2;
      drawItemGlyph(g, gsx + TILE / 2, gsy + TILE / 2 + bob);
    }

    // co-op ghosts (other online players on this floor)
    if (SC.net) {
      var gl = SC.net.cryptGhosts(p.floor);
      for (var gg = 0; gg < gl.length; gg++) {
        var gh = gl[gg];
        var ghx = gh.x * TILE - cam.x + TILE / 2, ghy = gh.y * TILE - cam.y + TILE / 2;
        ctx.globalAlpha = 0.55;
        drawHero(ghx, ghy, '#7ecbff', t);
        ctx.globalAlpha = 1;
        label(gh.name, ghx, ghy - TILE * 0.72, '#7ecbff');
      }
    }

    // allies
    for (var ai = 0; ai < st.allies.length; ai++) {
      var al = st.allies[ai];
      if (al.hp <= 0) continue;
      interp(al, dtMs);
      var alx = al.fx * TILE - cam.x + TILE / 2, aly = al.fy * TILE - cam.y + TILE / 2;
      drawBeast(alx, aly, '#6fd88f', al.glyph || 'c', t);
      hpBar(alx, aly - TILE * 0.55, al.hp / al.maxHp, TILE * 0.7);
    }

    // monsters
    for (var mi = 0; mi < st.monsters.length; mi++) {
      var m = st.monsters[mi];
      if (m.hp <= 0) continue;
      if (!map.visible[m.x + ',' + m.y]) continue;
      interp(m, dtMs);
      var mx = m.fx * TILE - cam.x + TILE / 2, my = m.fy * TILE - cam.y + TILE / 2;
      var size = m.boss ? 1.5 : (m.miniBoss ? 1.2 : 1);
      drawBeast(mx, my, m.color, m.glyph, t, size);
      hpBar(mx, my - TILE * 0.55 * size, m.hp / m.maxHp, TILE * 0.7 * size);
      if (m.boss || m.miniBoss) label(m.name, mx, my - TILE * 0.75 * size, '#ffd35c');
    }

    // projectiles
    for (var pi = 0; pi < st.projectiles.length; pi++) {
      var pr = st.projectiles[pi];
      var prx = pr.x * TILE - cam.x, pry = pr.y * TILE - cam.y;
      ctx.beginPath();
      ctx.fillStyle = pr.color || '#ffd35c';
      ctx.arc(prx, pry, 4, 0, Math.PI * 2);
      ctx.fill();
      ctx.beginPath();
      ctx.globalAlpha = 0.4;
      ctx.arc(prx - pr.vx * 0.02 * TILE, pry - pr.vy * 0.02 * TILE, 3, 0, Math.PI * 2);
      ctx.fill();
      ctx.globalAlpha = 1;
    }

    // player
    interp(p, dtMs);
    var px = p.fx * TILE - cam.x + TILE / 2, py = p.fy * TILE - cam.y + TILE / 2;
    if (SC.entities.hasEffect(p, 'invisible')) ctx.globalAlpha = 0.45;
    drawHero(px, py, classColor(p.classId), t);
    ctx.globalAlpha = 1;

    drawSparks();
    drawFloats();
    drawMinimap(map, p);
  }

  function interp(e, dtMs) {
    var k = Math.min(1, dtMs / 110);
    e.fx = e.fx === undefined ? e.x : U.lerp(e.fx, e.x, k);
    e.fy = e.fy === undefined ? e.y : U.lerp(e.fy, e.y, k);
  }

  function drawTile(tile, sx, sy, pal, t, map, x, y) {
    switch (tile) {
      case T.WALL:
        ctx.fillStyle = pal.wall;
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = 'rgba(0,0,0,0.25)';
        ctx.fillRect(sx, sy + TILE - 5, TILE, 5);
        break;
      case T.FLOOR: case T.TRAP:
        ctx.fillStyle = pal.floor;
        ctx.fillRect(sx, sy, TILE, TILE);
        if (((x * 7 + y * 13) % 9) === 0) { // subtle floor variation
          ctx.fillStyle = 'rgba(255,255,255,0.025)';
          ctx.fillRect(sx + 4, sy + 4, TILE - 8, TILE - 8);
        }
        if (tile === T.TRAP && map.trapRevealed[x + ',' + y]) {
          ctx.strokeStyle = '#e74c3c';
          ctx.lineWidth = 2;
          ctx.beginPath();
          ctx.moveTo(sx + TILE * 0.3, sy + TILE * 0.7);
          ctx.lineTo(sx + TILE * 0.5, sy + TILE * 0.3);
          ctx.lineTo(sx + TILE * 0.7, sy + TILE * 0.7);
          ctx.stroke();
        }
        break;
      case T.DOOR_CLOSED: case T.DOOR_OPEN:
        ctx.fillStyle = pal.floor;
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = tile === T.DOOR_CLOSED ? '#7a5a3a' : 'rgba(122,90,58,0.4)';
        if (tile === T.DOOR_CLOSED) ctx.fillRect(sx + 4, sy + 2, TILE - 8, TILE - 4);
        else { ctx.fillRect(sx + 2, sy + 2, 6, TILE - 4); ctx.fillRect(sx + TILE - 8, sy + 2, 6, TILE - 4); }
        break;
      case T.STAIRS_DOWN: case T.STAIRS_UP:
        ctx.fillStyle = pal.floor;
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = tile === T.STAIRS_DOWN ? '#f1c40f' : '#9ad1ff';
        for (var st3 = 0; st3 < 3; st3++) {
          var stw = TILE * (0.7 - st3 * 0.18);
          ctx.fillRect(sx + (TILE - stw) / 2, sy + TILE * 0.25 + st3 * TILE * 0.2, stw, TILE * 0.12);
        }
        break;
      case T.WATER:
        ctx.fillStyle = '#123a5e';
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = 'rgba(120,190,255,0.25)';
        ctx.fillRect(sx, sy + Math.sin(t * 2 + x + y) * 3 + TILE * 0.4, TILE, 3);
        break;
      case T.LAVA:
        ctx.fillStyle = '#7e1e08';
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = 'rgba(255,140,40,' + (0.35 + Math.sin(t * 3 + x * 2 + y) * 0.2) + ')';
        ctx.fillRect(sx + 3, sy + 3, TILE - 6, TILE - 6);
        break;
      case T.CHEST: case T.CHEST_OPEN:
        ctx.fillStyle = pal.floor;
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = tile === T.CHEST ? '#b8860b' : '#6d5210';
        ctx.fillRect(sx + TILE * 0.2, sy + TILE * 0.35, TILE * 0.6, TILE * 0.4);
        ctx.fillStyle = tile === T.CHEST ? '#f1c40f' : '#8a6d1a';
        ctx.fillRect(sx + TILE * 0.2, sy + TILE * 0.3, TILE * 0.6, TILE * 0.12);
        break;
      case T.SHRINE: case T.SHRINE_USED:
        ctx.fillStyle = pal.floor;
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = tile === T.SHRINE ? '#b06ae0' : '#5d4a70';
        ctx.beginPath();
        ctx.moveTo(sx + TILE / 2, sy + TILE * 0.15);
        ctx.lineTo(sx + TILE * 0.75, sy + TILE * 0.8);
        ctx.lineTo(sx + TILE * 0.25, sy + TILE * 0.8);
        ctx.closePath();
        ctx.fill();
        if (tile === T.SHRINE) {
          ctx.fillStyle = 'rgba(176,106,224,' + (0.3 + Math.sin(t * 4) * 0.2) + ')';
          ctx.beginPath(); ctx.arc(sx + TILE / 2, sy + TILE * 0.45, TILE * 0.3, 0, Math.PI * 2); ctx.fill();
        }
        break;
      case T.BOSS_GATE:
        ctx.fillStyle = '#3d0f0f';
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.strokeStyle = '#e74c3c';
        ctx.lineWidth = 3;
        ctx.strokeRect(sx + 4, sy + 4, TILE - 8, TILE - 8);
        ctx.fillStyle = '#e74c3c';
        ctx.font = 'bold ' + (TILE * 0.5) + 'px serif';
        ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
        ctx.fillText('☠', sx + TILE / 2, sy + TILE / 2 + 2);
        break;
      case T.PILLAR:
        ctx.fillStyle = pal.floor;
        ctx.fillRect(sx, sy, TILE, TILE);
        ctx.fillStyle = pal.accent || '#556';
        ctx.beginPath();
        ctx.arc(sx + TILE / 2, sy + TILE / 2, TILE * 0.32, 0, Math.PI * 2);
        ctx.fill();
        ctx.fillStyle = 'rgba(255,255,255,0.12)';
        ctx.beginPath();
        ctx.arc(sx + TILE / 2 - 3, sy + TILE / 2 - 3, TILE * 0.14, 0, Math.PI * 2);
        ctx.fill();
        break;
      default:
        ctx.fillStyle = pal.floor;
        ctx.fillRect(sx, sy, TILE, TILE);
    }
  }

  function classColor(classId) {
    return {
      warrior: '#e67e22', mage: '#3498db', rogue: '#95a5a6',
      paladin: '#f1c40f', ranger: '#2ecc71', necromancer: '#9b59b6'
    }[classId] || '#b06ae0';
  }

  function drawHero(x, y, color, t) {
    var r = TILE * 0.34;
    // shadow
    ctx.fillStyle = 'rgba(0,0,0,0.4)';
    ctx.beginPath(); ctx.ellipse(x, y + r * 0.9, r * 0.8, r * 0.35, 0, 0, Math.PI * 2); ctx.fill();
    // cloak body
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.moveTo(x, y - r);
    ctx.quadraticCurveTo(x + r, y - r * 0.2, x + r * 0.7, y + r * 0.8);
    ctx.lineTo(x - r * 0.7, y + r * 0.8);
    ctx.quadraticCurveTo(x - r, y - r * 0.2, x, y - r);
    ctx.fill();
    // head
    ctx.fillStyle = '#f2d5b1';
    ctx.beginPath(); ctx.arc(x, y - r * 0.75, r * 0.42, 0, Math.PI * 2); ctx.fill();
    // hood rim
    ctx.strokeStyle = 'rgba(0,0,0,0.35)';
    ctx.lineWidth = 2;
    ctx.beginPath(); ctx.arc(x, y - r * 0.75, r * 0.45, Math.PI * 0.9, Math.PI * 2.1); ctx.stroke();
    // idle bob glint
    ctx.fillStyle = 'rgba(255,255,255,' + (0.15 + Math.sin(t * 2) * 0.08) + ')';
    ctx.beginPath(); ctx.arc(x - r * 0.3, y - r * 0.2, r * 0.16, 0, Math.PI * 2); ctx.fill();
  }

  function drawBeast(x, y, color, glyph, t, size) {
    size = size || 1;
    var r = TILE * 0.32 * size;
    ctx.fillStyle = 'rgba(0,0,0,0.4)';
    ctx.beginPath(); ctx.ellipse(x, y + r * 0.9, r * 0.85, r * 0.32, 0, 0, Math.PI * 2); ctx.fill();
    var bob = Math.sin(t * 3 + x * 0.1) * r * 0.08;
    ctx.fillStyle = color;
    ctx.beginPath(); ctx.arc(x, y + bob, r, 0, Math.PI * 2); ctx.fill();
    ctx.fillStyle = 'rgba(255,255,255,0.15)';
    ctx.beginPath(); ctx.arc(x - r * 0.3, y - r * 0.3 + bob, r * 0.35, 0, Math.PI * 2); ctx.fill();
    ctx.fillStyle = '#0b0e14';
    ctx.font = 'bold ' + Math.round(r * 1.1) + 'px monospace';
    ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
    ctx.fillText(glyph || '?', x, y + bob + 1);
  }

  function drawItemGlyph(g, x, y) {
    var def = SC.entities.itemDef(g.id) || {};
    var kindGlyphs = { potion: '!', scroll: '?', weapon: '/', shield: ')', armor: '[', helmet: '^', gloves: '{', boots: 'b', ring: 'o', amulet: '"', food: '%', special: '*', material: '◆' };
    var color = def.color || '#f1c40f';
    ctx.fillStyle = 'rgba(0,0,0,0.5)';
    ctx.beginPath(); ctx.arc(x, y, TILE * 0.26, 0, Math.PI * 2); ctx.fill();
    ctx.strokeStyle = rarityColor(g.rarity);
    ctx.lineWidth = 2;
    ctx.beginPath(); ctx.arc(x, y, TILE * 0.26, 0, Math.PI * 2); ctx.stroke();
    ctx.fillStyle = color;
    ctx.font = 'bold ' + Math.round(TILE * 0.34) + 'px monospace';
    ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
    ctx.fillText(g.gold ? '$' : (def.glyph || kindGlyphs[def.kind] || '*'), x, y + 1);
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

  function drawSparks() {
    for (var i = 0; i < sparks.length; i++) {
      var s = sparks[i];
      ctx.globalAlpha = Math.min(1, s.ttl / 300);
      ctx.fillStyle = s.color;
      ctx.fillRect(s.x * TILE - cam.x - 2, s.y * TILE - cam.y - 2, 4, 4);
    }
    ctx.globalAlpha = 1;
  }

  function drawFloats() {
    ctx.font = 'bold 15px sans-serif';
    ctx.textAlign = 'center';
    for (var i = 0; i < floats.length; i++) {
      var f = floats[i];
      ctx.globalAlpha = Math.min(1, f.ttl / 400);
      ctx.fillStyle = '#000';
      ctx.fillText(f.txt, f.x * TILE - cam.x + TILE / 2 + 1, f.y * TILE - cam.y + 1);
      ctx.fillStyle = f.color;
      ctx.fillText(f.txt, f.x * TILE - cam.x + TILE / 2, f.y * TILE - cam.y);
    }
    ctx.globalAlpha = 1;
  }

  function drawMinimap(map, p) {
    var mw = Math.min(120, map.w * 2), s = mw / map.w;
    var mh = map.h * s;
    var mx = vw - mw - 10, my = 64;
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
    ctx.fillStyle = '#fff';
    ctx.fillRect(mx + p.x * s - 1, my + p.y * s - 1, 3, 3);
  }

  // ---------------- Haven / Farm ------------------------------------------
  function renderHaven(st, dtMs, farmFocus) {
    tickFx(dtMs);
    var h = st.haven;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    // sky gradient
    var grad = ctx.createLinearGradient(0, 0, 0, vh);
    grad.addColorStop(0, '#141327');
    grad.addColorStop(1, '#0b1a12');
    ctx.fillStyle = grad;
    ctx.fillRect(0, 0, vw, vh);

    var gw = SC.haven.GRID_W, gh = SC.haven.GRID_H;
    var cell = Math.min((vw - 20) / gw, (vh - 190) / gh);
    var ox = (vw - gw * cell) / 2, oy = 76;
    st._havenLayout = { ox: ox, oy: oy, cell: cell };

    var t = Date.now() / 1000;
    // ground
    for (var y = 0; y < gh; y++) {
      for (var x = 0; x < gw; x++) {
        ctx.fillStyle = ((x + y) % 2 === 0) ? '#17251a' : '#152218';
        ctx.fillRect(ox + x * cell, oy + y * cell, cell - 1, cell - 1);
      }
    }
    // placement preview
    if (st.buildPlacing) {
      var chk = SC.haven.canPlace(h, st.buildPlacing.type, st.buildPlacing.x, st.buildPlacing.y);
      var d = SC.haven.bdef(st.buildPlacing.type) || { size: 1 };
      ctx.fillStyle = chk.ok ? 'rgba(46,204,113,0.35)' : 'rgba(231,76,60,0.35)';
      ctx.fillRect(ox + st.buildPlacing.x * cell, oy + st.buildPlacing.y * cell, cell * d.size, cell * d.size);
    }
    // buildings
    var now = Date.now();
    for (var i = 0; i < h.buildings.length; i++) {
      var b = h.buildings[i];
      var d2 = SC.haven.bdef(b.type) || { size: 1, icon: '❓' };
      var bx = ox + b.x * cell, by = oy + b.y * cell, bs = cell * d2.size;
      ctx.fillStyle = st.selectedBuilding === b ? '#2c3b57' : '#1d2740';
      roundRect(bx + 2, by + 2, bs - 4, bs - 4, 8);
      ctx.fill();
      if (st.selectedBuilding === b) {
        ctx.strokeStyle = '#f1c40f'; ctx.lineWidth = 2;
        roundRect(bx + 2, by + 2, bs - 4, bs - 4, 8);
        ctx.stroke();
      }
      ctx.font = Math.round(bs * 0.44) + 'px serif';
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
      ctx.fillText(d2.icon, bx + bs / 2, by + bs / 2 - bs * 0.06);
      // level chip
      ctx.fillStyle = '#0b0e14';
      ctx.fillRect(bx + 4, by + 4, 22, 13);
      ctx.fillStyle = '#f1c40f';
      ctx.font = 'bold 10px sans-serif';
      ctx.textAlign = 'left'; ctx.textBaseline = 'top';
      ctx.fillText('L' + b.level, bx + 6, by + 6);
      // production ready dot / crop ring
      var pend = SC.haven.pendingProduction(b, now);
      if (pend > 0) {
        ctx.fillStyle = '#f1c40f';
        ctx.beginPath(); ctx.arc(bx + bs - 9, by + 9, 5 + Math.sin(t * 4) * 1.5, 0, Math.PI * 2); ctx.fill();
      }
      if (b.type === 'farmPlot') {
        if (b.crop) {
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
    }
    drawFloats2(ox, oy, cell);
  }

  function drawFloats2(ox, oy, cell) {
    ctx.font = 'bold 14px sans-serif';
    ctx.textAlign = 'center';
    for (var i = 0; i < floats.length; i++) {
      var f = floats[i];
      ctx.globalAlpha = Math.min(1, f.ttl / 400);
      ctx.fillStyle = f.color;
      ctx.fillText(f.txt, ox + f.x * cell + cell / 2, oy + f.y * cell);
    }
    ctx.globalAlpha = 1;
  }

  function roundRect(x, y, w, h, r) {
    ctx.beginPath();
    ctx.moveTo(x + r, y);
    ctx.arcTo(x + w, y, x + w, y + h, r);
    ctx.arcTo(x + w, y + h, x, y + h, r);
    ctx.arcTo(x, y + h, x, y, r);
    ctx.arcTo(x, y, x + w, y, r);
    ctx.closePath();
  }

  // ---------------- Arena -------------------------------------------------
  function renderArena(dtMs) {
    var A = SC.arena.state();
    if (!A) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.fillStyle = '#0d0912';
    ctx.fillRect(0, 0, vw, vh);
    var cell = Math.min(vw / A.map.w, (vh - 140) / A.map.h);
    var ox = (vw - A.map.w * cell) / 2, oy = 70;
    var t = Date.now() / 1000;

    for (var y = 0; y < A.map.h; y++) {
      for (var x = 0; x < A.map.w; x++) {
        if (A.map.solid[y * A.map.w + x]) {
          ctx.fillStyle = '#2c2440';
          ctx.fillRect(ox + x * cell, oy + y * cell, cell, cell);
          ctx.fillStyle = 'rgba(0,0,0,0.3)';
          ctx.fillRect(ox + x * cell, oy + (y + 1) * cell - 4, cell, 4);
        } else {
          ctx.fillStyle = ((x + y) % 2 === 0) ? '#171226' : '#151023';
          ctx.fillRect(ox + x * cell, oy + y * cell, cell, cell);
        }
      }
    }

    // powerups
    for (var pi = 0; pi < A.powerups.length; pi++) {
      var pw = A.powerups[pi];
      ctx.font = Math.round(cell * 0.7) + 'px serif';
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
      var bob = Math.sin(t * 4 + pi) * 3;
      ctx.fillText(pw.def.icon, ox + pw.x * cell, oy + pw.y * cell + bob);
    }

    // bombs
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

    // fighters
    var all = [A.me];
    for (var id in A.fighters) all.push(A.fighters[id]);
    for (var fi = 0; fi < all.length; fi++) {
      var f = all[fi];
      if (f.hp <= 0 || now < f.respawnAt) continue;
      var fx = ox + f.x * cell, fy = oy + f.y * cell;
      ctx.fillStyle = 'rgba(0,0,0,0.4)';
      ctx.beginPath(); ctx.ellipse(fx, fy + cell * 0.32, cell * 0.3, cell * 0.12, 0, 0, Math.PI * 2); ctx.fill();
      ctx.fillStyle = f.color;
      ctx.beginPath(); ctx.arc(fx, fy, cell * 0.34, 0, Math.PI * 2); ctx.fill();
      if (now < f.shieldUntil) {
        ctx.strokeStyle = 'rgba(130,200,255,0.8)';
        ctx.lineWidth = 2;
        ctx.beginPath(); ctx.arc(fx, fy, cell * 0.45, 0, Math.PI * 2); ctx.stroke();
      }
      // aim indicator
      ctx.strokeStyle = 'rgba(255,255,255,0.5)';
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.moveTo(fx + f.aimX * cell * 0.36, fy + f.aimY * cell * 0.36);
      ctx.lineTo(fx + f.aimX * cell * 0.55, fy + f.aimY * cell * 0.55);
      ctx.stroke();
      label(f.name + ' · ' + f.score, fx, fy - cell * 0.5, f === A.me ? '#ffd35c' : '#dfe6f5');
      hpBarPx(fx, fy - cell * 0.45, f.hp / ((SC.DATA.arena && SC.DATA.arena.hp) || 100), cell * 0.8);
    }

    // projectiles
    for (var pri = 0; pri < A.projectiles.length; pri++) {
      var pr = A.projectiles[pri];
      ctx.fillStyle = pr.color || '#ffd35c';
      ctx.beginPath(); ctx.arc(ox + pr.x * cell, oy + pr.y * cell, cell * 0.1, 0, Math.PI * 2); ctx.fill();
    }

    // particles
    for (var pa = 0; pa < A.particles.length; pa++) {
      var pt = A.particles[pa];
      ctx.globalAlpha = Math.min(1, pt.ttl / 300);
      ctx.fillStyle = pt.color;
      ctx.fillRect(ox + pt.x * cell - 2, oy + pt.y * cell - 2, 4, 4);
    }
    ctx.globalAlpha = 1;

    // timer + scores header
    var remain = Math.max(0, A.endAt - now);
    ctx.fillStyle = 'rgba(5,7,12,0.7)';
    ctx.fillRect(vw / 2 - 60, oy - 34, 120, 26);
    ctx.fillStyle = remain < 15000 ? '#e74c3c' : '#f1c40f';
    ctx.font = 'bold 16px monospace';
    ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
    ctx.fillText(A.over ? 'MATCH OVER' : U.fmtTime(remain), vw / 2, oy - 21);

    // store layout for input
    A._layout = { ox: ox, oy: oy, cell: cell };
  }

  function hpBarPx(x, y, frac, w) {
    frac = U.clamp(frac, 0, 1);
    ctx.fillStyle = 'rgba(0,0,0,0.6)';
    ctx.fillRect(x - w / 2, y, w, 4);
    ctx.fillStyle = frac > 0.5 ? '#2ecc71' : (frac > 0.25 ? '#f1c40f' : '#e74c3c');
    ctx.fillRect(x - w / 2, y, w * frac, 4);
  }

  return {
    init: init, resize: resize,
    renderCrypt: renderCrypt, renderHaven: renderHaven, renderArena: renderArena,
    floatText: floatText, burst: burst,
    tileSize: function () { return TILE; },
    camera: function () { return cam; },
    viewport: function () { return { w: vw, h: vh }; },
    rarityColor: rarityColor,
    classColor: classColor
  };
})();

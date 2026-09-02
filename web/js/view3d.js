'use strict';
/* ShadowCrypt Online — 3D view: raycast renderer for first-person (FPP) and
 * third-person over-the-shoulder (TPP) camera modes. Same dungeons, same game
 * state — walls raised into 3D via DDA raycasting, entities as depth-sorted,
 * z-buffered billboard sprites with cel outlines, distance fog, theme palettes,
 * head-bob, FPP weapon viewmodel and crosshair. */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.view3d = (function () {
  var U = SC.util, T = SC.TILE;
  var FOV = 66 * Math.PI / 180;
  var COL_W = 2;               // css px per ray column
  var canvas = null, ctx = null, dpr = 1, vw = 0, vh = 0;
  var zbuf = [];
  var bobPhase = 0;
  var smoothAng = null;

  function ensure() {
    canvas = document.getElementById('game-canvas');
    ctx = canvas.getContext('2d');
    dpr = window.devicePixelRatio || 1;
    vw = canvas.clientWidth || window.innerWidth;
    vh = canvas.clientHeight || window.innerHeight;
  }

  function shade(hex, amt) { return SC.render.shadeColor(hex, amt); }

  var texCache = { themeId: null, wall: null, wall2: null };
  function texFor(themeId) {
    if (texCache.themeId !== themeId) {
      texCache.themeId = themeId;
      var tt = SC.assets && SC.assets.isReady() && SC.assets.themeTiles(themeId);
      texCache.wall = tt && tt.walls.length ? tt.walls[0] : null;
      texCache.wall2 = tt && tt.walls.length > 1 ? tt.walls[1] : texCache.wall;
    }
    return texCache;
  }

  function wallInfo(map, tile, pal) {
    var tex = texFor((map.theme && map.theme.id) || 'dungeon');
    switch (tile) {
      case T.WALL: return { color: pal.wall, texKey: tex.wall };
      case T.PILLAR: return { color: shade(pal.wall, 18), texKey: tex.wall2 };
      case T.DOOR_CLOSED: return { color: '#7a5a3a', door: true, texKey: SC.assets && SC.assets.isReady() ? SC.assets.tileKey('doorClosed') : null };
      case T.BOSS_GATE: return { color: '#5c1414', gate: true, texKey: SC.assets && SC.assets.isReady() ? SC.assets.tileKey('bossGate') : null };
      default: return null;
    }
  }

  function isWallTile(map, x, y, pal) {
    var t = map.get(x, y);
    return wallInfo(map, t, pal);
  }

  // ---------------------------------------------------------------- render
  function interp(e, dtMs) {
    var k = Math.min(1, dtMs / 110);
    e.fx = e.fx === undefined ? e.x : e.fx + (e.x - e.fx) * k;
    e.fy = e.fy === undefined ? e.y : e.fy + (e.y - e.fy) * k;
  }

  function render(st, dtMs) {
    ensure();
    var p = st.player;
    var map = st.map;
    interp(p, dtMs);
    for (var ii = 0; ii < st.monsters.length; ii++) interp(st.monsters[ii], dtMs);
    for (var aj = 0; aj < st.allies.length; aj++) interp(st.allies[aj], dtMs);
    var theme = map.theme || {};
    var pal = theme.palette || { wall: '#3a3f4d', floor: '#20242e', accent: '#556' };
    var now = Date.now();
    var t = now / 1000;

    // camera
    var ang = p.ang !== undefined ? p.ang : Math.atan2(p.dirY || 1, p.dirX || 0);
    if (smoothAng === null) smoothAng = ang;
    var da = ((ang - smoothAng + Math.PI * 3) % (Math.PI * 2)) - Math.PI;
    smoothAng += da * Math.min(1, dtMs / 90);
    ang = smoothAng;

    var px = p.fx + 0.5, py = p.fy + 0.5;
    var tpp = st.camera === 'tpp';
    var camDist = 0;
    var cx = px, cy = py;
    if (tpp) {
      camDist = 3.0;
      // pull camera back until it would clip a wall
      var reach = 0.2;
      for (var cd = 0.2; cd <= 3.0; cd += 0.1) {
        var tx2 = px - Math.cos(ang) * cd, ty2 = py - Math.sin(ang) * cd;
        if (isWallTile(map, Math.floor(tx2), Math.floor(ty2), pal)) { break; }
        reach = cd;
      }
      camDist = reach;
      cx = px - Math.cos(ang) * camDist;
      cy = py - Math.sin(ang) * camDist;
    }

    // movement head-bob (FPP)
    var moving = Math.abs(p.fx - p.x) > 0.02 || Math.abs(p.fy - p.y) > 0.02;
    bobPhase += dtMs * (moving ? 0.011 : 0.003);
    var bobY = tpp ? 0 : Math.sin(bobPhase) * 6;
    var horizon = vh / 2 + bobY + (tpp ? -vh * 0.11 : 0);

    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    // ceiling & floor gradients
    var cg = ctx.createLinearGradient(0, 0, 0, horizon);
    cg.addColorStop(0, '#05070d');
    cg.addColorStop(1, shade(pal.wall, -46));
    ctx.fillStyle = cg;
    ctx.fillRect(0, 0, vw, horizon);
    var fg = ctx.createLinearGradient(0, horizon, 0, vh);
    fg.addColorStop(0, shade(pal.floor, -34));
    fg.addColorStop(1, shade(pal.floor, 6));
    ctx.fillStyle = fg;
    ctx.fillRect(0, horizon, vw, vh - horizon);

    // raycast walls
    var nCols = Math.ceil(vw / COL_W);
    if (zbuf.length !== nCols) zbuf = new Array(nCols);
    var focalH = (vh * 1.1);
    var flicker = 0.92 + 0.06 * Math.sin(t * 11) + 0.02 * Math.sin(t * 23);
    var maxDepth = 18;
    var atlasImg = SC.assets && SC.assets.isReady() ? SC.assets.image() : null;
    var prevSmoothWalls = ctx.imageSmoothingEnabled;
    ctx.imageSmoothingEnabled = false;

    for (var col = 0; col < nCols; col++) {
      var relCol = (col * COL_W + COL_W / 2) / vw - 0.5;
      var rayAng = ang + Math.atan(relCol * 2 * Math.tan(FOV / 2));
      var cosDiff = Math.cos(rayAng - ang);
      var rdx = Math.cos(rayAng), rdy = Math.sin(rayAng);
      // DDA
      var mapX = Math.floor(cx), mapY = Math.floor(cy);
      var dDistX = Math.abs(1 / (rdx || 1e-9)), dDistY = Math.abs(1 / (rdy || 1e-9));
      var stepX = rdx < 0 ? -1 : 1, stepY = rdy < 0 ? -1 : 1;
      var sideDistX = rdx < 0 ? (cx - mapX) * dDistX : (mapX + 1 - cx) * dDistX;
      var sideDistY = rdy < 0 ? (cy - mapY) * dDistY : (mapY + 1 - cy) * dDistY;
      var side = 0, hit = null, dist = maxDepth, guard = 0;
      while (guard++ < 64) {
        if (sideDistX < sideDistY) { sideDistX += dDistX; mapX += stepX; side = 0; }
        else { sideDistY += dDistY; mapY += stepY; side = 1; }
        if (!map.inb(mapX, mapY)) { hit = { color: '#05070d' }; dist = maxDepth; break; }
        var info = isWallTile(map, mapX, mapY, pal);
        if (info) {
          hit = info;
          dist = side === 0 ? (sideDistX - dDistX) : (sideDistY - dDistY);
          break;
        }
        if ((side === 0 ? sideDistX : sideDistY) > maxDepth) { break; }
      }
      var perp = Math.max(0.08, dist * cosDiff);
      zbuf[col] = perp;
      if (!hit) continue;
      var colH = focalH / perp;
      var y0 = horizon - colH / 2, y1 = horizon + colH / 2;
      // wall u coordinate
      var wallX = side === 0 ? cy + dist * rdy : cx + dist * rdx;
      wallX -= Math.floor(wallX);
      var bright = U.clamp(1.35 - perp / 6.2, 0.05, 1.12) * flicker;
      if (side === 1) bright *= 0.72;
      // TEXTURED column: sample a vertical strip from the real wall sprite
      var texRect = hit.texKey && atlasImg ? SC.assets.rect(hit.texKey) : null;
      if (texRect) {
        var su = texRect[0] + Math.min(texRect[2] - 1, Math.floor(wallX * texRect[2]));
        ctx.drawImage(atlasImg, su, texRect[1], 1, texRect[3], col * COL_W, y0, COL_W, colH);
        var dark = U.clamp(1 - bright, 0, 0.94);
        if (dark > 0.02) {
          ctx.fillStyle = 'rgba(3,4,10,' + dark.toFixed(3) + ')';
          ctx.fillRect(col * COL_W, y0, COL_W, colH);
        }
      } else {
        var stripe = (Math.floor(wallX * 5) % 2 === 0) ? 4 : -10;
        ctx.fillStyle = shade(hit.color, Math.round((bright - 1) * 110) + stripe);
        ctx.fillRect(col * COL_W, y0, COL_W, colH);
        ctx.fillStyle = 'rgba(0,0,0,' + (0.28 * bright).toFixed(2) + ')';
        ctx.fillRect(col * COL_W, y0 + colH * 0.33, COL_W, Math.max(1, colH * 0.015));
        ctx.fillRect(col * COL_W, y0 + colH * 0.66, COL_W, Math.max(1, colH * 0.015));
      }
      // torch light bounce near the floor
      ctx.fillStyle = 'rgba(255,190,110,' + (0.08 * bright).toFixed(3) + ')';
      ctx.fillRect(col * COL_W, y0 + colH * 0.82, COL_W, colH * 0.1);
      if (hit.gate) {
        ctx.fillStyle = 'rgba(231,76,60,' + (0.4 * bright * (0.7 + 0.3 * Math.sin(t * 5))) + ')';
        if ((Math.floor(wallX * 8) % 2) === 0) ctx.fillRect(col * COL_W, y0 + colH * 0.2, COL_W, colH * 0.6);
      }
      // hard edge shading top/bottom
      ctx.fillStyle = 'rgba(0,0,0,0.4)';
      ctx.fillRect(col * COL_W, y0, COL_W, Math.min(4, colH * 0.05));
      ctx.fillRect(col * COL_W, y1 - Math.min(4, colH * 0.05), COL_W, Math.min(4, colH * 0.05));
    }

    // depth-space ambient shading: ceiling falls to black, floor gains contact shadow at horizon
    var shadeTop = ctx.createLinearGradient(0, 0, 0, horizon);
    shadeTop.addColorStop(0, 'rgba(0,0,0,0.55)');
    shadeTop.addColorStop(1, 'rgba(0,0,0,0)');
    ctx.fillStyle = shadeTop;
    ctx.fillRect(0, 0, vw, horizon);
    var shadeMid = ctx.createLinearGradient(0, horizon - vh * 0.09, 0, horizon + vh * 0.14);
    shadeMid.addColorStop(0, 'rgba(0,0,0,0)');
    shadeMid.addColorStop(0.5, 'rgba(0,0,0,0.42)');
    shadeMid.addColorStop(1, 'rgba(0,0,0,0)');
    ctx.fillStyle = shadeMid;
    ctx.fillRect(0, horizon - vh * 0.09, vw, vh * 0.23);

    // ---------------- billboards --------------------------------------
    var sprites = [];
    function addSprite(wx, wy, img, scale, yOff, glow) {
      var dx = wx - cx, dy = wy - cy;
      var depth = dx * Math.cos(ang) + dy * Math.sin(ang);
      if (depth < 0.4 || depth > maxDepth) return;
      var lat = -dx * Math.sin(ang) + dy * Math.cos(ang);
      var focalW = (vw / 2) / Math.tan(FOV / 2);
      var sx = vw / 2 + (lat / depth) * focalW;
      var size = (focalH / depth) * (scale || 0.9);
      size = Math.min(size, vh * 0.82); // never let a close-up sprite swallow the screen
      if (sx + size / 2 < 0 || sx - size / 2 > vw) return;
      sprites.push({ x: sx, depth: depth, size: size, img: img, yOff: yOff || 0, glow: glow });
    }
    function groundY(depth) { return horizon + (focalH / depth) / 2; }

    // telegraphs as glowing ground discs
    for (var tg = 0; tg < (st.telegraphs || []).length; tg++) {
      var tel = st.telegraphs[tg];
      var tdx = tel.x + 0.5 - cx, tdy = tel.y + 0.5 - cy;
      var tDepth = tdx * Math.cos(ang) + tdy * Math.sin(ang);
      if (tDepth < 0.2 || tDepth > maxDepth) continue;
      var tLat = -tdx * Math.sin(ang) + tdy * Math.cos(ang);
      var focalW2 = (vw / 2) / Math.tan(FOV / 2);
      var tsx = vw / 2 + (tLat / tDepth) * focalW2;
      var trw = (tel.radius * focalW2) / tDepth;
      var prog = U.clamp((now - tel.at) / tel.duration, 0, 1);
      ctx.strokeStyle = tel.color || 'rgba(255,80,60,0.9)';
      ctx.lineWidth = 2;
      ctx.beginPath(); ctx.ellipse(tsx, groundY(tDepth), trw, trw * 0.32, 0, 0, Math.PI * 2); ctx.stroke();
      ctx.fillStyle = (tel.color || 'rgba(255,80,60,1)').replace(/[\d.]+\)$/, (0.16 + prog * 0.22).toFixed(2) + ')');
      ctx.beginPath(); ctx.ellipse(tsx, groundY(tDepth), trw * prog, trw * 0.32 * prog, 0, 0, Math.PI * 2); ctx.fill();
    }

    var frame = Math.floor(t * 3) % 2;
    // tiles that become billboards (stairs, chests, shrines)
    var scanR = 12;
    for (var sy2 = Math.max(0, p.y - scanR); sy2 <= Math.min(map.h - 1, p.y + scanR); sy2++) {
      for (var sx2 = Math.max(0, p.x - scanR); sx2 <= Math.min(map.w - 1, p.x + scanR); sx2++) {
        var tile2 = map.get(sx2, sy2);
        var img2 = null, sc2 = 0.8;
        if (tile2 === T.STAIRS_DOWN) { img2 = SC.render.spriteGlyph('sd', '🕳️'); sc2 = 0.7; }
        else if (tile2 === T.STAIRS_UP) { img2 = SC.render.spriteGlyph('su', '🪜'); sc2 = 0.7; }
        else if (tile2 === T.CHEST) { img2 = SC.render.spriteGlyph('ch', '🧰'); sc2 = 0.62; }
        else if (tile2 === T.SHRINE) { img2 = SC.render.spriteGlyph('sh', '⛩️'); sc2 = 0.85; }
        else if (tile2 === T.LAVA) {
          // lava glow disc
          var ldx = sx2 + 0.5 - cx, ldy = sy2 + 0.5 - cy;
          var lDepth = ldx * Math.cos(ang) + ldy * Math.sin(ang);
          if (lDepth > 0.3 && lDepth < 12) {
            var lLat = -ldx * Math.sin(ang) + ldy * Math.cos(ang);
            var fW = (vw / 2) / Math.tan(FOV / 2);
            var lsx = vw / 2 + (lLat / lDepth) * fW;
            var lw = (0.55 * fW) / lDepth;
            ctx.fillStyle = 'rgba(255,110,30,' + U.clamp(0.5 - lDepth * 0.04, 0.06, 0.5) * (0.7 + 0.3 * Math.sin(t * 4 + sx2)) + ')';
            ctx.beginPath(); ctx.ellipse(lsx, groundY(lDepth), lw, lw * 0.32, 0, 0, Math.PI * 2); ctx.fill();
          }
        }
        if (img2 && map.explored[sx2 + ',' + sy2]) addSprite(sx2 + 0.5, sy2 + 0.5, img2, sc2, 0.12);
      }
    }
    // breakables
    (map.breakables || []).forEach(function (br) {
      if (br.broken) return;
      var em = br.kind === 'urn' ? '🏺' : (br.kind === 'crate' ? '📦' : '🗝️');
      addSprite(br.x + 0.5, br.y + 0.5, SC.render.spriteGlyph('br' + br.kind, em), 0.55, 0.18);
    });
    // ground items
    st.groundItems.forEach(function (g2) {
      if (!map.visible[g2.x + ',' + g2.y]) return;
      var img3 = g2.gold ? SC.render.spriteGlyph('gold', '💰', 48)
        : SC.render.spriteGlyph('it:' + (SC.entities.itemDef(g2.id) || { kind: 'x' }).kind,
          { potion: '🧪', scroll: '📜', weapon: '⚔️', shield: '🛡️', armor: '🥋', helmet: '🪖', gloves: '🧤', boots: '🥾', ring: '💍', amulet: '📿', food: '🍖', special: '✨', material: '🪨' }[(SC.entities.itemDef(g2.id) || { kind: 'special' }).kind] || '✨', 48);
      addSprite(g2.x + 0.5, g2.y + 0.5, img3, 0.34, 0.28 + Math.sin(t * 3 + g2.x) * 0.02);
    });
    // monsters
    st.monsters.forEach(function (m) {
      if (m.hp <= 0 || !map.visible[m.x + ',' + m.y]) return;
      addSprite(m.fx + 0.5, m.fy + 0.5, SC.render.spriteMonster(m, frame), m.boss ? 1.05 : (m.miniBoss ? 0.8 : 0.62), 0, m.affix ? m.affix.color : null);
    });
    // allies
    st.allies.forEach(function (al) {
      if (al.hp <= 0) return;
      addSprite(al.fx + 0.5, al.fy + 0.5, SC.render.spriteMonster({ id: al.id || 'skeleton', name: al.name, glyph: al.glyph, color: al.cid ? '#6fd88f' : '#cfd8ea' }, frame), 0.5, 0);
    });
    // co-op ghosts
    if (SC.net) {
      SC.net.cryptGhosts(p.floor).forEach(function (gh) {
        addSprite(gh.x + 0.5, gh.y + 0.5, SC.render.spriteHero(gh.classId || 'warrior', frame), 0.62, 0);
      });
    }
    // projectiles
    st.projectiles.forEach(function (pr) {
      addSprite(pr.x, pr.y, SC.render.spriteGlyph('proj' + (pr.from === 'player' ? 'p' : 'm'), pr.from === 'player' ? '✦' : '✴️', 32), 0.18, 0.35);
    });
    // player body in TPP (paper-doll: shows equipped gear)
    if (tpp) addSprite(px, py, SC.render.spriteHero(p.classId, frame, p), 0.66, 0);

    // depth sort far→near, draw with z-buffer slices (crisp pixel scaling — HD-2D look)
    ctx.imageSmoothingEnabled = false;
    sprites.sort(function (a, b) { return b.depth - a.depth; });
    for (var si = 0; si < sprites.length; si++) {
      var sp = sprites[si];
      var sy0 = groundY(sp.depth) - sp.size * (1 - (sp.yOff || 0));
      var x0 = Math.round(sp.x - sp.size / 2);
      var fog = U.clamp(1.3 - sp.depth / 10, 0.1, 1);
      ctx.globalAlpha = fog;
      if (sp.glow) {
        ctx.save();
        ctx.shadowColor = sp.glow;
        ctx.shadowBlur = Math.min(12, 4 + sp.depth * 2);
      }
      // draw in vertical slices, skipping columns hidden by nearer walls
      var slice = Math.max(2, COL_W);
      for (var sxp = 0; sxp < sp.size; sxp += slice) {
        var scrX = x0 + sxp;
        var colIdx = Math.floor(scrX / COL_W);
        if (colIdx < 0 || colIdx >= zbuf.length) continue;
        if (zbuf[colIdx] !== undefined && zbuf[colIdx] < sp.depth) continue;
        var u = sxp / sp.size;
        ctx.drawImage(sp.img, u * sp.img.width, 0, slice * sp.img.width / sp.size, sp.img.height,
          scrX, sy0, slice, sp.size);
      }
      if (sp.glow) ctx.restore();
      ctx.globalAlpha = 1;
    }
    ctx.imageSmoothingEnabled = true;

    // ------------- FPP weapon viewmodel + crosshair --------------------
    if (!tpp) {
      var swing = p.swingUntil && now < p.swingUntil ? 1 - (p.swingUntil - now) / 170 : 0;
      drawViewmodel(p.classId, t, swing, moving);
    }
    // crosshair
    ctx.strokeStyle = 'rgba(255,255,255,0.65)';
    ctx.lineWidth = 1.5;
    var chY = tpp ? horizon - vh * 0.04 : horizon;
    ctx.beginPath(); ctx.moveTo(vw / 2 - 8, chY); ctx.lineTo(vw / 2 - 3, chY); ctx.stroke();
    ctx.beginPath(); ctx.moveTo(vw / 2 + 3, chY); ctx.lineTo(vw / 2 + 8, chY); ctx.stroke();
    ctx.beginPath(); ctx.moveTo(vw / 2, chY - 8); ctx.lineTo(vw / 2, chY - 3); ctx.stroke();
    ctx.beginPath(); ctx.moveTo(vw / 2, chY + 3); ctx.lineTo(vw / 2, chY + 8); ctx.stroke();

    // damage vignette when hurt recently
    var eff = SC.entities.effective(p);
    if (p.hp < eff.maxHp * 0.3) {
      ctx.fillStyle = 'rgba(180,20,20,' + (0.12 + 0.08 * Math.sin(t * 6)) + ')';
      ctx.fillRect(0, 0, vw, vh);
    }

    // boss bar via top-down renderer helpers is DOM-free; reuse minimap for orientation
    drawCompassMinimap(map, p, ang);
    var boss = null;
    st.monsters.forEach(function (m) { if (m.boss && m.hp > 0 && map.visible[m.x + ',' + m.y]) boss = m; });
    if (boss) drawBossBar3d(boss);
  }

  function drawViewmodel(classId, t, swing, moving) {
    var bob = Math.sin(t * (moving ? 9 : 2.4)) * (moving ? 8 : 3);
    var baseX = vw * 0.72, baseY = vh - vh * 0.16 + bob;
    var s = Math.min(vw, vh) * 0.24;
    ctx.save();
    ctx.translate(baseX, baseY);
    if (swing > 0) ctx.rotate(-0.9 + swing * 1.6);
    ctx.lineJoin = 'round';
    ctx.strokeStyle = '#131722';
    switch (classId) {
      case 'mage':
        ctx.fillStyle = '#7a5a3a'; ctx.lineWidth = 3;
        ctx.fillRect(-s * 0.06, -s * 1.05, s * 0.12, s * 1.3);
        ctx.strokeRect(-s * 0.06, -s * 1.05, s * 0.12, s * 1.3);
        ctx.fillStyle = '#7ec8ff';
        ctx.beginPath(); ctx.arc(0, -s * 1.12, s * 0.14, 0, Math.PI * 2); ctx.fill(); ctx.stroke();
        ctx.fillStyle = 'rgba(126,200,255,' + (0.35 + 0.2 * Math.sin(t * 6)) + ')';
        ctx.beginPath(); ctx.arc(0, -s * 1.12, s * 0.24, 0, Math.PI * 2); ctx.fill();
        break;
      case 'ranger':
        ctx.strokeStyle = '#8a6a3a'; ctx.lineWidth = 6;
        ctx.beginPath(); ctx.arc(-s * 0.2, -s * 0.5, s * 0.6, -1.1, 1.1); ctx.stroke();
        ctx.strokeStyle = '#ddd'; ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(-s * 0.2 + Math.cos(-1.1) * s * 0.6, -s * 0.5 + Math.sin(-1.1) * s * 0.6);
        ctx.lineTo(-s * 0.2 + Math.cos(1.1) * s * 0.6, -s * 0.5 + Math.sin(1.1) * s * 0.6);
        ctx.stroke();
        ctx.strokeStyle = '#c8b088'; ctx.lineWidth = 3;
        ctx.beginPath(); ctx.moveTo(-s * 0.2, -s * 0.5); ctx.lineTo(s * 0.5, -s * 0.5); ctx.stroke();
        break;
      case 'necromancer':
        ctx.fillStyle = '#4a3a5a'; ctx.lineWidth = 3;
        ctx.fillRect(-s * 0.06, -s * 1.1, s * 0.12, s * 1.35);
        ctx.strokeRect(-s * 0.06, -s * 1.1, s * 0.12, s * 1.35);
        ctx.strokeStyle = '#b8ffc8'; ctx.lineWidth = 6;
        ctx.beginPath(); ctx.arc(s * 0.22, -s * 1.02, s * 0.26, Math.PI * 0.75, Math.PI * 1.9); ctx.stroke();
        break;
      case 'rogue':
        ctx.fillStyle = '#cfd6e4'; ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.moveTo(0, -s * 0.95); ctx.lineTo(s * 0.1, -s * 0.2); ctx.lineTo(-s * 0.1, -s * 0.2);
        ctx.closePath(); ctx.fill(); ctx.stroke();
        ctx.fillStyle = '#444';
        ctx.fillRect(-s * 0.16, -s * 0.2, s * 0.32, s * 0.1);
        break;
      case 'paladin':
        ctx.fillStyle = '#8a7a5a'; ctx.lineWidth = 3;
        ctx.fillRect(-s * 0.06, -s * 1.0, s * 0.12, s * 1.25);
        ctx.strokeRect(-s * 0.06, -s * 1.0, s * 0.12, s * 1.25);
        ctx.fillStyle = '#e8e2cf';
        ctx.fillRect(-s * 0.3, -s * 1.16, s * 0.6, s * 0.26);
        ctx.strokeRect(-s * 0.3, -s * 1.16, s * 0.6, s * 0.26);
        break;
      default: // warrior sword
        ctx.fillStyle = '#cfd6e4'; ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.moveTo(0, -s * 1.25); ctx.lineTo(s * 0.09, -s * 0.25); ctx.lineTo(-s * 0.09, -s * 0.25);
        ctx.closePath(); ctx.fill(); ctx.stroke();
        ctx.fillStyle = '#f1c40f';
        ctx.fillRect(-s * 0.22, -s * 0.26, s * 0.44, s * 0.09);
        ctx.fillStyle = '#7a4a1e';
        ctx.fillRect(-s * 0.05, -s * 0.18, s * 0.1, s * 0.3);
    }
    // gauntlet fist
    ctx.fillStyle = '#caa27a';
    ctx.beginPath(); ctx.arc(0, s * 0.06, s * 0.16, 0, Math.PI * 2); ctx.fill();
    ctx.strokeStyle = '#131722'; ctx.lineWidth = 2.5; ctx.stroke();
    ctx.restore();
  }

  function drawCompassMinimap(map, p, ang) {
    var R = 46;
    var mx = vw - R - 14, my = R + 68;
    ctx.save();
    ctx.beginPath(); ctx.arc(mx, my, R, 0, Math.PI * 2); ctx.clip();
    ctx.fillStyle = 'rgba(5,7,12,0.72)';
    ctx.fillRect(mx - R, my - R, R * 2, R * 2);
    var s = 5;
    // rotate map so "up" = facing
    ctx.translate(mx, my);
    ctx.rotate(-ang - Math.PI / 2);
    for (var y = Math.max(0, p.y - 10); y <= Math.min(map.h - 1, p.y + 10); y++) {
      for (var x = Math.max(0, p.x - 10); x <= Math.min(map.w - 1, p.x + 10); x++) {
        if (!map.explored[x + ',' + y]) continue;
        var tile = map.get(x, y);
        if (tile === T.WALL) continue;
        ctx.fillStyle = tile === T.STAIRS_DOWN ? '#f1c40f' : 'rgba(150,165,200,0.4)';
        ctx.fillRect((x - p.x) * s - s / 2, (y - p.y) * s - s / 2, s - 1, s - 1);
      }
    }
    ctx.restore();
    // player arrow
    ctx.fillStyle = '#fff';
    ctx.beginPath();
    ctx.moveTo(mx, my - 5); ctx.lineTo(mx - 4, my + 4); ctx.lineTo(mx + 4, my + 4);
    ctx.closePath(); ctx.fill();
    ctx.strokeStyle = 'rgba(255,255,255,0.25)';
    ctx.lineWidth = 1.5;
    ctx.beginPath(); ctx.arc(mx, my, R, 0, Math.PI * 2); ctx.stroke();
  }

  function drawBossBar3d(boss) {
    var w = Math.min(420, vw * 0.7);
    var x = vw / 2 - w / 2, y = 58;
    ctx.fillStyle = 'rgba(5,7,12,0.8)';
    ctx.fillRect(x - 4, y - 20, w + 8, 40);
    ctx.strokeStyle = boss.enraged ? '#ff5040' : '#8a2be2';
    ctx.strokeRect(x - 4, y - 20, w + 8, 40);
    ctx.font = 'bold 13px serif';
    ctx.textAlign = 'center'; ctx.textBaseline = 'bottom';
    ctx.fillStyle = '#ffd35c';
    ctx.fillText('☠ ' + boss.name + (boss.enraged ? ' — ENRAGED' : ''), vw / 2, y - 2);
    var frac = U.clamp(boss.hp / boss.maxHp, 0, 1);
    ctx.fillStyle = '#3a0d0d';
    ctx.fillRect(x, y + 2, w, 12);
    ctx.fillStyle = '#e74c3c';
    ctx.fillRect(x, y + 2, w * frac, 12);
  }

  return { render: render };
})();

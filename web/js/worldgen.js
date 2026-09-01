'use strict';
/* ShadowCrypt Online — procedural dungeon generation (ported from the Rust roguelike)
 * 30 floors, 8 themed areas, rooms+corridors, doors, traps, water/lava, chests,
 * shrines, pillars, boss arenas, field-of-view. */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.TILE = {
  WALL: 0, FLOOR: 1, DOOR_CLOSED: 2, DOOR_OPEN: 3, STAIRS_DOWN: 4, STAIRS_UP: 5,
  TRAP: 6, WATER: 7, LAVA: 8, CHEST: 9, SHRINE: 10, BOSS_GATE: 11, PILLAR: 12,
  CHEST_OPEN: 13, SHRINE_USED: 14
};

SC.worldgen = (function () {
  var T = SC.TILE;
  var U = SC.util;

  function themeForFloor(floor) {
    var themes = (SC.DATA && SC.DATA.floorThemes) || [];
    for (var i = 0; i < themes.length; i++) {
      var th = themes[i];
      var lo = th.floors ? th.floors[0] : 1, hi = th.floors ? th.floors[1] : 30;
      if (floor >= lo && floor <= hi) return th;
    }
    return themes[0] || { id: 'dungeon', name: 'Dark Dungeon', enemyTier: 1, palette: { wall: '#3a3f4d', floor: '#20242e', accent: '#556' } };
  }

  function isBossFloor(floor) { return floor % 5 === 0; }
  function isMiniBossFloor(floor) { return floor % 5 === 3; } // mini-boss mid-area

  function GameMap(w, h) {
    this.w = w; this.h = h;
    this.tiles = new Uint8Array(w * h); // all WALL
    this.trapRevealed = {};
    this.visible = {};
    this.explored = {};
  }
  GameMap.prototype.idx = function (x, y) { return y * this.w + x; };
  GameMap.prototype.inb = function (x, y) { return x >= 0 && y >= 0 && x < this.w && y < this.h; };
  GameMap.prototype.get = function (x, y) { return this.inb(x, y) ? this.tiles[this.idx(x, y)] : T.WALL; };
  GameMap.prototype.set = function (x, y, t) { if (this.inb(x, y)) this.tiles[this.idx(x, y)] = t; };
  GameMap.prototype.isWalkable = function (x, y) {
    var t = this.get(x, y);
    return t !== T.WALL && t !== T.PILLAR && t !== T.CHEST && t !== T.SHRINE &&
           t !== T.CHEST_OPEN && t !== T.SHRINE_USED && t !== T.DOOR_CLOSED && t !== T.BOSS_GATE;
  };
  GameMap.prototype.blocksSight = function (x, y) {
    var t = this.get(x, y);
    return t === T.WALL || t === T.PILLAR || t === T.DOOR_CLOSED;
  };

  function carveRoom(map, r) {
    for (var y = r.y; y < r.y + r.h; y++)
      for (var x = r.x; x < r.x + r.w; x++)
        map.set(x, y, T.FLOOR);
  }

  function carveTunnel(map, x1, y1, x2, y2, rng) {
    var x = x1, y = y1;
    var horizFirst = rng.chance(0.5);
    function step(fromX, toX, fy) { for (var i = Math.min(fromX, toX); i <= Math.max(fromX, toX); i++) map.set(i, fy, T.FLOOR); }
    function stepV(fromY, toY, fx) { for (var i = Math.min(fromY, toY); i <= Math.max(fromY, toY); i++) map.set(fx, i, T.FLOOR); }
    if (horizFirst) { step(x, x2, y); stepV(y, y2, x2); }
    else { stepV(y, y2, x); step(x, x2, y2); }
  }

  function roomsOverlap(a, b, pad) {
    return a.x - pad < b.x + b.w && a.x + a.w + pad > b.x &&
           a.y - pad < b.y + b.h && a.y + a.h + pad > b.y;
  }

  function center(r) { return { x: Math.floor(r.x + r.w / 2), y: Math.floor(r.y + r.h / 2) }; }

  function randomFloorIn(map, room, rng, taken) {
    for (var tries = 0; tries < 60; tries++) {
      var x = rng.int(room.x, room.x + room.w - 1);
      var y = rng.int(room.y, room.y + room.h - 1);
      if (map.get(x, y) === T.FLOOR && !taken[x + ',' + y]) { taken[x + ',' + y] = 1; return { x: x, y: y }; }
    }
    return null;
  }

  function placeDoors(map, rooms, rng) {
    // A door candidate: a floor tile on a room's boundary ring whose neighbors form a corridor entrance
    for (var r = 0; r < rooms.length; r++) {
      var room = rooms[r];
      var edges = [];
      var x, y;
      for (x = room.x; x < room.x + room.w; x++) { edges.push([x, room.y - 1]); edges.push([x, room.y + room.h]); }
      for (y = room.y; y < room.y + room.h; y++) { edges.push([room.x - 1, y]); edges.push([room.x + room.w, y]); }
      for (var i = 0; i < edges.length; i++) {
        x = edges[i][0]; y = edges[i][1];
        if (map.get(x, y) !== T.FLOOR) continue;
        var horizWalls = map.blocksSight(x - 1, y) && map.blocksSight(x + 1, y);
        var vertWalls = map.blocksSight(x, y - 1) && map.blocksSight(x, y + 1);
        if ((horizWalls || vertWalls) && rng.chance(0.55)) map.set(x, y, T.DOOR_CLOSED);
      }
    }
  }

  function scatterLiquid(map, rng, tile, blobs, blobSize) {
    for (var b = 0; b < blobs; b++) {
      var sx = rng.int(2, map.w - 3), sy = rng.int(2, map.h - 3);
      if (map.get(sx, sy) !== T.FLOOR) continue;
      var frontier = [[sx, sy]];
      var placed = 0;
      while (frontier.length && placed < blobSize) {
        var p = frontier.splice(rng.int(0, frontier.length - 1), 1)[0];
        if (map.get(p[0], p[1]) !== T.FLOOR) continue;
        map.set(p[0], p[1], tile);
        placed++;
        for (var d = 0; d < 4; d++) {
          var nx = p[0] + U.DIRS4[d][0], ny = p[1] + U.DIRS4[d][1];
          if (map.get(nx, ny) === T.FLOOR && rng.chance(0.6)) frontier.push([nx, ny]);
        }
      }
    }
  }

  function generate(floor, seed) {
    var rng = new U.Rng(seed);
    var theme = themeForFloor(floor);
    var boss = isBossFloor(floor);

    var w = boss ? 44 : Math.min(70, 46 + floor), h = boss ? 34 : Math.min(48, 34 + Math.floor(floor / 2));
    var map = new GameMap(w, h);
    map.floor = floor;
    map.seed = seed;
    map.theme = theme;
    var rooms = [];
    var maxRooms = boss ? 5 : (10 + Math.floor(floor / 3));

    if (boss) {
      // Boss arena: big central chamber + entry room + side alcoves
      var arena = { x: Math.floor(w / 2) - 9, y: 4, w: 18, h: 16, isArena: true };
      var entry = { x: Math.floor(w / 2) - 3, y: h - 9, w: 7, h: 5 };
      carveRoom(map, arena); carveRoom(map, entry);
      carveTunnel(map, center(entry).x, center(entry).y, center(arena).x, center(arena).y + 7, rng);
      rooms.push(arena, entry);
      // pillars in arena
      for (var px = arena.x + 3; px < arena.x + arena.w - 2; px += 5)
        for (var py = arena.y + 3; py < arena.y + arena.h - 2; py += 5)
          if (rng.chance(0.7)) map.set(px, py, T.PILLAR);
      // gate at arena mouth
      map.set(center(arena).x, arena.y + arena.h, T.BOSS_GATE);
    } else {
      for (var attempt = 0; attempt < maxRooms * 8 && rooms.length < maxRooms; attempt++) {
        var rw = rng.int(5, 11), rh = rng.int(4, 9);
        var room = { x: rng.int(1, w - rw - 2), y: rng.int(1, h - rh - 2), w: rw, h: rh };
        var ok = true;
        for (var i = 0; i < rooms.length; i++) if (roomsOverlap(room, rooms[i], 1)) { ok = false; break; }
        if (!ok) continue;
        carveRoom(map, room);
        if (rooms.length > 0) {
          var prev = center(rooms[rooms.length - 1]), cur = center(room);
          carveTunnel(map, prev.x, prev.y, cur.x, cur.y, rng);
        }
        rooms.push(room);
      }
      placeDoors(map, rooms, rng);
      // pillars in large rooms
      for (var ri = 0; ri < rooms.length; ri++) {
        var rr = rooms[ri];
        if (rr.w >= 8 && rr.h >= 6 && rng.chance(0.5)) {
          map.set(rr.x + 2, rr.y + 2, T.PILLAR);
          map.set(rr.x + rr.w - 3, rr.y + 2, T.PILLAR);
          map.set(rr.x + 2, rr.y + rr.h - 3, T.PILLAR);
          map.set(rr.x + rr.w - 3, rr.y + rr.h - 3, T.PILLAR);
        }
      }
    }

    // Theme liquids: water in caves/forest, lava in volcanic/demon
    var tid = theme.id || '';
    if (/cave|forest|frozen/.test(tid)) scatterLiquid(map, rng, T.WATER, rng.int(1, 3), rng.int(4, 10));
    if (/volcan|demon/.test(tid)) scatterLiquid(map, rng, T.LAVA, rng.int(2, 4), rng.int(4, 9));

    map.rooms = rooms;
    var taken = {};

    // Spawn point: first room (entry room on boss floors is rooms[1])
    var spawnRoom = boss ? rooms[1] : rooms[0];
    var spawn = randomFloorIn(map, spawnRoom, rng, taken) || center(spawnRoom);
    map.spawn = spawn;
    map.set(spawn.x, spawn.y, T.FLOOR);
    taken[spawn.x + ',' + spawn.y] = 1;

    // Stairs
    if (floor < 30) {
      var lastRoom = boss ? rooms[0] : rooms[rooms.length - 1];
      var sd = randomFloorIn(map, lastRoom, rng, taken) || center(lastRoom);
      map.set(sd.x, sd.y, T.STAIRS_DOWN);
      map.stairsDown = sd;
    }
    if (floor > 1) {
      var su = { x: spawn.x + 1, y: spawn.y };
      if (map.get(su.x, su.y) !== T.FLOOR) su = { x: spawn.x, y: spawn.y + 1 };
      if (map.get(su.x, su.y) === T.FLOOR) { map.set(su.x, su.y, T.STAIRS_UP); map.stairsUp = su; taken[su.x + ',' + su.y] = 1; }
    }

    // Chests, shrines, traps (not on boss floors' arena)
    var normalRooms = rooms.filter(function (r) { return !r.isArena; });
    var chestCount = boss ? 2 : rng.int(1, 3);
    var pos, c;
    for (c = 0; c < chestCount; c++) {
      var roomC = rng.pick(normalRooms.length ? normalRooms : rooms);
      pos = randomFloorIn(map, roomC, rng, taken);
      if (pos) map.set(pos.x, pos.y, T.CHEST);
    }
    if (!boss && rng.chance(0.45)) {
      pos = randomFloorIn(map, rng.pick(rooms), rng, taken);
      if (pos) map.set(pos.x, pos.y, T.SHRINE);
    }
    var trapCount = boss ? 0 : rng.int(2, 4 + Math.floor(floor / 4));
    for (c = 0; c < trapCount; c++) {
      pos = randomFloorIn(map, rng.pick(rooms), rng, taken);
      if (pos && map.get(pos.x, pos.y) === T.FLOOR) map.set(pos.x, pos.y, T.TRAP);
    }

    // Monster + item spawn locations (resolved to actual entities by game.js using SC.DATA)
    map.monsterSpawns = [];
    map.itemSpawns = [];
    if (!boss) {
      var mcount = rng.int(6, 10 + Math.floor(floor / 2));
      for (c = 0; c < mcount; c++) {
        var mr = rng.pick(rooms);
        if (mr === rooms[0]) continue; // keep spawn room safe
        pos = randomFloorIn(map, mr, rng, taken);
        if (pos) map.monsterSpawns.push(pos);
      }
      var icount = rng.int(3, 6);
      for (c = 0; c < icount; c++) {
        pos = randomFloorIn(map, rng.pick(rooms), rng, taken);
        if (pos) map.itemSpawns.push(pos);
      }
    } else {
      // boss floor: boss center arena + a few adds
      map.bossSpawn = center(rooms[0]);
      var addc = rng.int(2, 4);
      for (c = 0; c < addc; c++) {
        pos = randomFloorIn(map, rooms[0], rng, taken);
        if (pos) map.monsterSpawns.push(pos);
      }
    }
    if (isMiniBossFloor(floor) && map.monsterSpawns.length) {
      map.miniBossSpawn = map.monsterSpawns.pop();
    }

    return map;
  }

  // Field of view: raycast within radius, walls block.
  function computeFov(map, ox, oy, radius) {
    var vis = {};
    vis[ox + ',' + oy] = true;
    map.explored[ox + ',' + oy] = true;
    for (var y = oy - radius; y <= oy + radius; y++) {
      for (var x = ox - radius; x <= ox + radius; x++) {
        if (!map.inb(x, y)) continue;
        var dx = x - ox, dy = y - oy;
        if (dx * dx + dy * dy > radius * radius) continue;
        var pts = U.line(ox, oy, x, y);
        for (var i = 0; i < pts.length; i++) {
          var px = pts[i][0], py = pts[i][1];
          var key = px + ',' + py;
          vis[key] = true;
          map.explored[key] = true;
          if (map.blocksSight(px, py) && !(px === ox && py === oy)) break;
        }
      }
    }
    map.visible = vis;
    return vis;
  }

  // BFS pathability check / next-step pathfinding for AI (bounded)
  function findPath(map, sx, sy, tx, ty, maxNodes, extraBlock) {
    if (sx === tx && sy === ty) return [];
    var open = [[sx, sy]];
    var came = {};
    came[sx + ',' + sy] = null;
    var nodes = 0;
    while (open.length && nodes < (maxNodes || 900)) {
      var cur = open.shift();
      nodes++;
      for (var d = 0; d < 8; d++) {
        var nx = cur[0] + U.DIRS8[d][0], ny = cur[1] + U.DIRS8[d][1];
        var key = nx + ',' + ny;
        if (came[key] !== undefined) continue;
        if (nx === tx && ny === ty) {
          came[key] = cur;
          // walk back
          var path = [];
          var node = [nx, ny];
          while (node && !(node[0] === sx && node[1] === sy)) {
            path.push({ x: node[0], y: node[1] });
            node = came[node[0] + ',' + node[1]];
          }
          path.reverse();
          return path;
        }
        // closed doors and boss gates are traversable for planning — walkers open them on contact
        var nt = map.get(nx, ny);
        if (!map.isWalkable(nx, ny) && nt !== T.DOOR_CLOSED && nt !== T.BOSS_GATE) continue;
        if (extraBlock && extraBlock(nx, ny)) continue;
        came[key] = cur;
        open.push([nx, ny]);
      }
    }
    return null;
  }

  return {
    generate: generate,
    computeFov: computeFov,
    findPath: findPath,
    themeForFloor: themeForFloor,
    isBossFloor: isBossFloor,
    isMiniBossFloor: isMiniBossFloor,
    GameMap: GameMap
  };
})();

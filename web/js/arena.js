'use strict';
/* ShadowCrypt Online — Arena: real-time PvP deathmatch (Mini-Militia / BombSquad style).
 * Offline: fight bots. Online: state relayed via WebSocket rooms; each client owns its
 * fighter and applies damage to itself from remote projectiles/bombs (victim-authoritative). */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.arena = (function () {
  var U = SC.util;

  var A = null; // active arena state

  function cfg() { return (SC.DATA && SC.DATA.arena) || {}; }

  var BOT_NAMES = ['Gravekeeper', 'Bonechewer', 'Wraithling', 'Cryptbat', 'Gloomfang'];

  function makeMap(seed) {
    var rng = new U.Rng(seed);
    var w = 26, h = 16;
    var solid = new Uint8Array(w * h);
    var x, y;
    for (x = 0; x < w; x++) { solid[x] = 1; solid[(h - 1) * w + x] = 1; }
    for (y = 0; y < h; y++) { solid[y * w] = 1; solid[y * w + w - 1] = 1; }
    // symmetric obstacles
    for (var i = 0; i < 14; i++) {
      x = rng.int(2, Math.floor(w / 2) - 1); y = rng.int(2, h - 3);
      solid[y * w + x] = 1;
      solid[y * w + (w - 1 - x)] = 1; // mirror
    }
    return { w: w, h: h, solid: solid, seed: seed };
  }

  function isSolid(map, x, y) {
    var tx = Math.floor(x), ty = Math.floor(y);
    if (tx < 0 || ty < 0 || tx >= map.w || ty >= map.h) return true;
    return !!map.solid[ty * map.w + tx];
  }

  function spawnPoint(map, rng) {
    for (var t = 0; t < 200; t++) {
      var x = rng.int(1, map.w - 2) + 0.5, y = rng.int(1, map.h - 2) + 0.5;
      if (!isSolid(map, x, y)) return { x: x, y: y };
    }
    return { x: map.w / 2, y: map.h / 2 };
  }

  function makeFighter(id, name, color, isBot) {
    return {
      id: id, name: name, color: color || '#b06ae0', bot: !!isBot,
      x: 2, y: 2, vx: 0, vy: 0, aimX: 1, aimY: 0,
      hp: cfg().hp || 100, score: 0, deaths: 0,
      fireCd: 0, bombCd: 0, respawnAt: 0,
      speedBoost: 0, shieldUntil: 0, tripleUntil: 0, megaUntil: 0,
      remote: false, lastUpdate: Date.now()
    };
  }

  function enter(playerName, playerColor, online) {
    var seed = online && online.seed ? online.seed : ((Date.now() / 60000) | 0);
    var rng = new U.Rng(seed + 7);
    A = {
      map: makeMap(seed),
      me: makeFighter('me', playerName, playerColor, false),
      fighters: {},   // remote/bot fighters by id
      projectiles: [], bombs: [], powerups: [], particles: [],
      rng: rng,
      endAt: Date.now() + (cfg().matchSeconds || 120) * 1000,
      online: !!online,
      over: false,
      nextPowerup: Date.now() + 4000,
      netAcc: 0,
      startedAt: Date.now(),
      botsFilled: false
    };
    var sp = spawnPoint(A.map, rng);
    A.me.x = sp.x; A.me.y = sp.y;
    if (!A.online) {
      for (var i = 0; i < 3; i++) {
        var b = makeFighter('bot' + i, BOT_NAMES[i % BOT_NAMES.length], ['#e74c3c', '#2ecc71', '#f1c40f'][i % 3], true);
        var bsp = spawnPoint(A.map, rng);
        b.x = bsp.x; b.y = bsp.y;
        b.thinkCd = 0;
        A.fighters[b.id] = b;
      }
    }
    return A;
  }

  function leave() { A = null; }
  function state() { return A; }

  // ---- Actions ------------------------------------------------------------
  function fire(f) {
    var now = Date.now();
    if (now < f.respawnAt || f.hp <= 0) return;
    if (f.fireCd > 0) return;
    f.fireCd = cfg().fireCooldownMs || 380;
    var dirs = [[f.aimX, f.aimY]];
    if (now < f.tripleUntil) {
      var ang = Math.atan2(f.aimY, f.aimX);
      dirs = [0, -0.28, 0.28].map(function (da) { return [Math.cos(ang + da), Math.sin(ang + da)]; });
    }
    dirs.forEach(function (d) {
      var len = Math.sqrt(d[0] * d[0] + d[1] * d[1]) || 1;
      A.projectiles.push({
        owner: f.id, x: f.x, y: f.y,
        vx: d[0] / len * (cfg().projSpeed || 11), vy: d[1] / len * (cfg().projSpeed || 11),
        ttl: 1400, color: f.color
      });
    });
    if (f === A.me && A.online && SC.net) SC.net.sendArena({ t: 'fire', dirs: dirs, x: f.x, y: f.y });
  }

  function dropBomb(f) {
    var now = Date.now();
    if (now < f.respawnAt || f.hp <= 0) return;
    if (f.bombCd > 0) return;
    f.bombCd = cfg().bombCooldownMs || 2600;
    var mega = now < f.megaUntil;
    A.bombs.push({ owner: f.id, x: f.x, y: f.y, at: now + (cfg().bombFuseMs || 1400), mega: mega });
    if (f === A.me && A.online && SC.net) SC.net.sendArena({ t: 'bomb', x: f.x, y: f.y, mega: mega });
  }

  function hurt(f, dmg, byId) {
    var now = Date.now();
    if (f.hp <= 0 || now < f.respawnAt) return;
    if (now < f.shieldUntil) dmg = Math.round(dmg * (1 - ((cfg().powerups || []).find(function (p) { return p.effect === 'shield'; }) || { value: 0.5 }).value));
    f.hp -= dmg;
    if (f.hp <= 0) {
      f.deaths++;
      f.respawnAt = now + (cfg().respawnMs || 2500);
      spawnBurst(f.x, f.y, f.color, 18);
      var killer = byId === 'me' ? A.me : A.fighters[byId];
      if (killer) killer.score++;
      if (f === A.me) {
        U.emit('arena:death', killer ? killer.name : 'the arena');
        if (A.online && SC.net) SC.net.sendArena({ t: 'died', by: byId });
      } else if (byId === 'me') {
        U.emit('arena:kill', f.name);
      }
    }
  }

  function respawn(f) {
    f.hp = cfg().hp || 100;
    var sp = spawnPoint(A.map, A.rng);
    f.x = sp.x; f.y = sp.y; f.vx = 0; f.vy = 0;
    f.respawnAt = 0;
  }

  function spawnBurst(x, y, color, n) {
    for (var i = 0; i < n; i++) {
      var a = Math.random() * Math.PI * 2, s = 2 + Math.random() * 4;
      A.particles.push({ x: x, y: y, vx: Math.cos(a) * s, vy: Math.sin(a) * s, ttl: 500 + Math.random() * 300, color: color });
    }
  }

  // ---- Tick ---------------------------------------------------------------
  // input: {mx, my (move dir -1..1), fire:boolean, bomb:boolean}
  function tick(dtMs, input) {
    if (!A) return;
    var dt = dtMs / 1000;
    var now = Date.now();
    var me = A.me;

    if (!A.over && now > A.endAt) {
      A.over = true;
      U.emit('arena:over', results());
    }

    // my movement
    if (me.hp > 0 && now >= me.respawnAt) {
      var spd = (cfg().moveSpeed || 5.2) * (now < me.speedBoost ? 1.5 : 1);
      moveFighter(me, (input.mx || 0) * spd, (input.my || 0) * spd, dt);
      if (input.mx || input.my) {
        var l = Math.sqrt(input.mx * input.mx + input.my * input.my) || 1;
        me.aimX = input.mx / l; me.aimY = input.my / l;
      }
      if (input.fire) fire(me);
      if (input.bomb) dropBomb(me);
    } else if (me.hp <= 0 && me.respawnAt && now >= me.respawnAt) {
      respawn(me);
    }
    me.fireCd = Math.max(0, me.fireCd - dtMs);
    me.bombCd = Math.max(0, me.bombCd - dtMs);

    // an online room with nobody else in it gets training bots until real players show up
    if (A.online && !A.botsFilled && now - A.startedAt > 4000 && !A.over) {
      var hasRemote = false;
      for (var chk in A.fighters) { if (A.fighters[chk].remote) { hasRemote = true; break; } }
      if (!hasRemote) {
        A.botsFilled = true;
        for (var nb = 0; nb < 2; nb++) {
          var fill = makeFighter('bot' + nb, BOT_NAMES[nb], ['#e74c3c', '#2ecc71'][nb], true);
          var fsp = spawnPoint(A.map, A.rng);
          fill.x = fsp.x; fill.y = fsp.y; fill.thinkCd = 0;
          A.fighters[fill.id] = fill;
        }
        U.emit('msg', 'No rivals yet — training bots enter the arena!');
      }
    }

    // bots (run in both offline and lonely-online rooms); prune stale remote fighters
    for (var id in A.fighters) {
      var b = A.fighters[id];
      if (b.bot) {
        if (b.hp <= 0 && b.respawnAt && now >= b.respawnAt) respawn(b);
        if (b.hp <= 0) continue;
        b.fireCd = Math.max(0, b.fireCd - dtMs);
        b.bombCd = Math.max(0, b.bombCd - dtMs);
        b.thinkCd -= dtMs;
        if (b.thinkCd <= 0) {
          b.thinkCd = 200 + Math.random() * 250;
          botThink(b, now);
        }
        moveFighter(b, b.vx, b.vy, dt);
      } else if (A.online && now - b.lastUpdate > 6000) {
        delete A.fighters[id];
      }
    }

    // projectiles
    for (var i = A.projectiles.length - 1; i >= 0; i--) {
      var pr = A.projectiles[i];
      pr.x += pr.vx * dt; pr.y += pr.vy * dt; pr.ttl -= dtMs;
      var dead = pr.ttl <= 0 || isSolid(A.map, pr.x, pr.y);
      if (!dead) {
        // hit checks: I am authoritative for my own damage; bots are local so I resolve them too
        if (pr.owner !== 'me' && me.hp > 0 && now >= me.respawnAt && U.dist(pr.x, pr.y, me.x, me.y) < 0.45) {
          hurt(me, cfg().projDamage || 14, pr.owner);
          if (A.online && SC.net) SC.net.sendArena({ t: 'hp', hp: me.hp });
          dead = true;
        }
        if (!dead) {
          // bots are resolved locally even online; remote players resolve their own hits
          for (var bid in A.fighters) {
            var bf = A.fighters[bid];
            if (A.online && !bf.bot) continue;
            if (pr.owner !== bf.id && bf.hp > 0 && now >= bf.respawnAt && U.dist(pr.x, pr.y, bf.x, bf.y) < 0.45) {
              hurt(bf, cfg().projDamage || 14, pr.owner);
              dead = true; break;
            }
          }
        }
      }
      if (dead) {
        spawnBurst(pr.x, pr.y, pr.color, 3);
        A.projectiles.splice(i, 1);
      }
    }

    // bombs
    for (var bi = A.bombs.length - 1; bi >= 0; bi--) {
      var bomb = A.bombs[bi];
      if (now >= bomb.at) {
        var radius = (cfg().bombRadius || 2.4) * (bomb.mega ? 1.6 : 1);
        var dmg = (cfg().bombDamage || 42) * (bomb.mega ? 1.5 : 1);
        spawnBurst(bomb.x, bomb.y, '#ffb15c', 26);
        if (me.hp > 0 && U.dist(bomb.x, bomb.y, me.x, me.y) < radius) {
          hurt(me, Math.round(dmg), bomb.owner);
          if (A.online && SC.net) SC.net.sendArena({ t: 'hp', hp: me.hp });
        }
        for (var bbid in A.fighters) {
          var bbf = A.fighters[bbid];
          if (A.online && !bbf.bot) continue;
          if (bbf.hp > 0 && bomb.owner !== bbf.id && U.dist(bomb.x, bomb.y, bbf.x, bbf.y) < radius) {
            hurt(bbf, Math.round(dmg), bomb.owner);
          }
        }
        A.bombs.splice(bi, 1);
      }
    }

    // powerups spawn + pickup
    if (now >= A.nextPowerup && A.powerups.length < 4 && !A.over) {
      A.nextPowerup = now + 6000 + Math.random() * 5000;
      var pus = cfg().powerups || [];
      if (pus.length) {
        var pu = A.rng.weighted(pus.map(function (p) { return { w: p.weight || 1, v: p }; }));
        var psp = spawnPoint(A.map, A.rng);
        A.powerups.push({ def: pu, x: psp.x, y: psp.y });
      }
    }
    for (var pi = A.powerups.length - 1; pi >= 0; pi--) {
      var pw = A.powerups[pi];
      if (me.hp > 0 && U.dist(pw.x, pw.y, me.x, me.y) < 0.6) {
        applyPowerup(me, pw.def);
        A.powerups.splice(pi, 1);
        U.emit('arena:powerup', pw.def);
      } else {
        var grabbed = false;
        for (var gid in A.fighters) {
          var gf = A.fighters[gid];
          if (A.online && !gf.bot) continue;
          if (gf.hp > 0 && U.dist(pw.x, pw.y, gf.x, gf.y) < 0.6) { applyPowerup(gf, pw.def); grabbed = true; break; }
        }
        if (grabbed) A.powerups.splice(pi, 1);
      }
    }

    // particles
    for (var pa = A.particles.length - 1; pa >= 0; pa--) {
      var pt = A.particles[pa];
      pt.x += pt.vx * dt; pt.y += pt.vy * dt; pt.ttl -= dtMs;
      pt.vx *= 0.92; pt.vy *= 0.92;
      if (pt.ttl <= 0) A.particles.splice(pa, 1);
    }

    // network state broadcast @10Hz
    if (A.online && SC.net) {
      A.netAcc += dtMs;
      if (A.netAcc >= 100) {
        A.netAcc = 0;
        SC.net.sendArena({ t: 's', x: me.x, y: me.y, ax: me.aimX, ay: me.aimY, hp: me.hp, sc: me.score, nm: me.name, cl: me.color });
      }
    }
  }

  function applyPowerup(f, def) {
    var now = Date.now();
    if (def.effect === 'heal') f.hp = Math.min(cfg().hp || 100, f.hp + def.value);
    else if (def.effect === 'speed') f.speedBoost = now + (def.durMs || 6000);
    else if (def.effect === 'shield') f.shieldUntil = now + (def.durMs || 6000);
    else if (def.effect === 'triple') f.tripleUntil = now + (def.durMs || 7000);
    else if (def.effect === 'megabomb') f.megaUntil = now + (def.durMs || 8000);
  }

  function moveFighter(f, vx, vy, dt) {
    var nx = f.x + vx * dt, ny = f.y + vy * dt;
    if (!isSolid(A.map, nx, f.y)) f.x = nx;
    if (!isSolid(A.map, f.x, ny)) f.y = ny;
  }

  function botThink(b, now) {
    var me = A.me;
    var d = U.dist(b.x, b.y, me.x, me.y);
    var toMe = { x: me.x - b.x, y: me.y - b.y };
    var l = Math.sqrt(toMe.x * toMe.x + toMe.y * toMe.y) || 1;
    b.aimX = toMe.x / l; b.aimY = toMe.y / l;
    var spd = cfg().moveSpeed || 5.2;
    if (me.hp <= 0 || now < me.respawnAt) { // wander
      b.vx = (Math.random() - 0.5) * spd; b.vy = (Math.random() - 0.5) * spd;
      return;
    }
    if (d > 5) { b.vx = b.aimX * spd * 0.9; b.vy = b.aimY * spd * 0.9; }
    else if (d < 2.5) { b.vx = -b.aimX * spd * 0.8; b.vy = -b.aimY * spd * 0.8; }
    else { b.vx = -b.aimY * spd * 0.6; b.vy = b.aimX * spd * 0.6; } // strafe
    if (d < 8 && b.fireCd <= 0 && Math.random() < 0.75) fire(b);
    if (d < 3 && b.bombCd <= 0 && Math.random() < 0.3) dropBomb(b);
  }

  // ---- Networking ---------------------------------------------------------
  function handleNet(fromId, fromName, msg) {
    if (!A || !A.online) return;
    var f = A.fighters[fromId];
    if (!f) {
      f = makeFighter(fromId, fromName || 'Rival', '#e0655c', false);
      f.remote = true;
      A.fighters[fromId] = f;
      // a real rival arrived — training bots bow out
      if (A.botsFilled) {
        for (var bid in A.fighters) { if (A.fighters[bid].bot) delete A.fighters[bid]; }
        U.emit('msg', f.name + ' enters the arena — the bots withdraw!');
      }
    }
    f.lastUpdate = Date.now();
    if (msg.t === 's') {
      f.x = msg.x; f.y = msg.y; f.aimX = msg.ax; f.aimY = msg.ay;
      f.hp = msg.hp; f.score = msg.sc || 0;
      if (msg.nm) f.name = msg.nm;
      if (msg.cl) f.color = msg.cl;
    } else if (msg.t === 'fire') {
      (msg.dirs || []).forEach(function (d) {
        var l = Math.sqrt(d[0] * d[0] + d[1] * d[1]) || 1;
        A.projectiles.push({ owner: fromId, x: msg.x, y: msg.y, vx: d[0] / l * (cfg().projSpeed || 11), vy: d[1] / l * (cfg().projSpeed || 11), ttl: 1400, color: f.color });
      });
    } else if (msg.t === 'bomb') {
      A.bombs.push({ owner: fromId, x: msg.x, y: msg.y, at: Date.now() + (cfg().bombFuseMs || 1400), mega: msg.mega });
    } else if (msg.t === 'hp') {
      f.hp = msg.hp;
    } else if (msg.t === 'died') {
      if (msg.by === (SC.net && SC.net.myId()) || msg.by === 'me') {
        A.me.score++;
        U.emit('arena:kill', f.name);
      }
      f.deaths++;
      spawnBurst(f.x, f.y, f.color, 18);
    }
  }

  function results() {
    var list = [{ name: A.me.name, score: A.me.score, me: true }];
    for (var id in A.fighters) list.push({ name: A.fighters[id].name, score: A.fighters[id].score, me: false });
    list.sort(function (a, b) { return b.score - a.score; });
    return list;
  }

  return {
    enter: enter, leave: leave, state: state, tick: tick,
    fire: fire, dropBomb: dropBomb, handleNet: handleNet, results: results,
    isSolid: isSolid
  };
})();

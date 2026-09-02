'use strict';
/* ShadowCrypt Online — Siege Defense: playable Clash-style base defense.
 * Shadow creatures storm your haven grid in waves; your towers auto-fire,
 * bone walls soak damage, and YOUR HERO fights on the field (move + arc
 * attacks + dash). Win all waves to earn gold, materials and XP. */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.siege = (function () {
  var U = SC.util, E = SC.entities;

  var COOLDOWN_MS = 4 * 3600000;

  function canStart(h) {
    var last = h.lastSiegePlayed || 0;
    return Date.now() - last >= COOLDOWN_MS;
  }
  function nextIn(h) { return Math.max(0, (h.lastSiegePlayed || 0) + COOLDOWN_MS - Date.now()); }

  function keepOf(h) { return h.buildings.find(function (b) { return b.type === 'keep'; }); }

  function start(st) {
    var p = st.player, h = st.haven;
    var keep = keepOf(h);
    var kl = SC.haven.keepLevel(h);
    st.siege = {
      wave: 0,
      maxWaves: 2 + Math.min(4, Math.ceil(kl / 2)),
      keepHp: 200 + kl * 120,
      keepMaxHp: 200 + kl * 120,
      keepX: keep.x + 1, keepY: keep.y + 1,
      attackers: [],
      shots: [],
      walls: {},           // 'x,y' -> {hp}
      towers: [],
      hero: {
        x: keep.x + 1, y: keep.y + 2.5,
        dirX: 0, dirY: -1, atkCd: 0, dashCd: 0, swingUntil: 0
      },
      nextWaveAt: Date.now() + 2500,
      over: false, won: false,
      kills: 0
    };
    // snapshot defensive structures
    h.buildings.forEach(function (b) {
      if (b.type === 'wall') st.siege.walls[b.x + ',' + b.y] = { hp: 30 + b.level * 25, max: 30 + b.level * 25, x: b.x, y: b.y };
      if (b.type === 'tower') st.siege.towers.push({ x: b.x, y: b.y, level: b.level, cd: 0 });
    });
    U.emit('msg', '🏰 SIEGE! Shadow creatures approach — defend your Keep!');
    U.emit('sfx', 'wave');
  }

  function abort(st) {
    if (st.siege && !st.siege.over) U.emit('msg', 'You abandoned the defense…');
    st.siege = null;
  }

  function spawnWave(st) {
    var sg = st.siege, h = st.haven;
    sg.wave++;
    var kl = SC.haven.keepLevel(h);
    var tier = Math.min(8, Math.ceil(kl / 1.5) + Math.floor(sg.wave / 2));
    var pool = E.enemyPool(tier);
    var count = 3 + sg.wave * 2;
    var rng = new U.Rng((Date.now() & 0xffffff) ^ sg.wave);
    for (var i = 0; i < count; i++) {
      var edge = rng.int(0, 3);
      var x, y;
      if (edge === 0) { x = rng.float() * SC.haven.GRID_W; y = -0.5; }
      else if (edge === 1) { x = rng.float() * SC.haven.GRID_W; y = SC.haven.GRID_H + 0.5; }
      else if (edge === 2) { x = -0.5; y = rng.float() * SC.haven.GRID_H; }
      else { x = SC.haven.GRID_W + 0.5; y = rng.float() * SC.haven.GRID_H; }
      var tpl = pool.length ? rng.pick(pool) : { id: 'shade', name: 'Shade', hp: 15, atk: 4, def: 0, spd: 8, xp: 5, color: '#7a6a9a', glyph: 's' };
      var m = E.spawnMonster(tpl, 0, 0, kl * 2, { eliteChance: sg.wave >= 3 ? 0.2 : 0 });
      m.x = x; m.y = y; m.fx = x; m.fy = y;
      m.hp = Math.round(m.hp * 0.8); m.maxHp = m.hp; // siege pacing
      m.atkTimer = 0;
      sg.attackers.push(m);
    }
    U.emit('toast', { text: '⚔ Wave ' + sg.wave + '/' + sg.maxWaves + '!', cls: 'bad' });
    U.emit('sfx', 'wave');
  }

  function tick(st, dt) {
    var sg = st.siege;
    if (!sg || sg.over) return;
    var p = st.player;
    var now = Date.now();
    var dts = dt / 1000;

    // waves
    var alive = sg.attackers.filter(function (a) { return a.hp > 0; });
    if (!alive.length && sg.wave >= sg.maxWaves && sg.wave > 0) return finish(st, true);
    if (!alive.length && now >= sg.nextWaveAt) {
      if (sg.wave < sg.maxWaves) { spawnWave(st); sg.nextWaveAt = now + 3000; }
    }

    // hero movement (free float on the grid)
    var hero = sg.hero;
    hero.atkCd -= dt; hero.dashCd -= dt;
    var mv = SC.input.moveVector();
    var spd = 4.6;
    if (mv.x || mv.y) {
      hero.x = U.clamp(hero.x + mv.x * spd * dts, 0.2, SC.haven.GRID_W - 0.2);
      hero.y = U.clamp(hero.y + mv.y * spd * dts, 0.2, SC.haven.GRID_H - 0.2);
      hero.dirX = mv.x; hero.dirY = mv.y;
    }
    if (SC.input.isAttackHeld() && hero.atkCd <= 0) heroAttack(st);

    // attackers advance toward keep, blocked by walls
    for (var i = 0; i < sg.attackers.length; i++) {
      var a = sg.attackers[i];
      if (a.hp <= 0) continue;
      a.atkTimer -= dt;
      var dx = sg.keepX - a.x, dy = sg.keepY - a.y;
      var dd = Math.sqrt(dx * dx + dy * dy) || 1;
      // reached keep?
      if (dd < 0.9) {
        if (a.atkTimer <= 0) {
          a.atkTimer = 900;
          sg.keepHp -= a.atk;
          SC.render.floatText(sg.keepX, sg.keepY, '-' + a.atk, '#ff8060');
          SC.render.shake(2);
          if (sg.keepHp <= 0) return finish(st, false);
        }
        continue;
      }
      // wall in the way?
      var nx = a.x + (dx / dd) * a.spd * 0.32 * dts * 3;
      var ny = a.y + (dy / dd) * a.spd * 0.32 * dts * 3;
      var wk = Math.floor(nx) + ',' + Math.floor(ny);
      var wall = sg.walls[wk];
      if (wall && wall.hp > 0) {
        if (a.atkTimer <= 0) {
          a.atkTimer = 800;
          wall.hp -= a.atk;
          SC.render.floatText(wall.x, wall.y, '-' + a.atk, '#d0c8b8');
          if (wall.hp <= 0) { U.emit('msg', 'A bone wall crumbles!'); U.emit('sfx', 'hit'); }
        }
        continue;
      }
      a.x = nx; a.y = ny; a.fx = nx; a.fy = ny;
      // attack hero if close
      var hd = U.dist(a.x, a.y, hero.x, hero.y);
      if (hd < 0.8 && a.atkTimer <= 0) {
        a.atkTimer = 1000;
        var eff = E.effective(p);
        var dmg = Math.max(1, Math.round(a.atk * 0.9 - eff.def * 0.4));
        p.hp -= dmg;
        SC.render.floatText(hero.x, hero.y, '-' + dmg, '#ff6b6b');
        U.emit('sfx', 'hurt');
        SC.render.shake(3);
        if (p.hp <= 0) {
          p.hp = 1; // hero staggers but the siege continues; keep is what matters
          U.emit('msg', 'You stagger back, bloodied!');
        }
      }
    }

    // towers auto-fire
    for (var ti = 0; ti < sg.towers.length; ti++) {
      var tw = sg.towers[ti];
      tw.cd -= dt;
      if (tw.cd > 0) continue;
      var best = null, bd = 4.2;
      for (var ai = 0; ai < sg.attackers.length; ai++) {
        var at = sg.attackers[ai];
        if (at.hp <= 0) continue;
        var td = U.dist(tw.x, tw.y, at.x, at.y);
        if (td < bd) { bd = td; best = at; }
      }
      if (best) {
        tw.cd = Math.max(400, 1300 - tw.level * 120);
        var tdmg = 4 + tw.level * 3;
        best.hp -= tdmg;
        sg.shots.push({ x1: tw.x, y1: tw.y, x2: best.x, y2: best.y, ttl: 160 });
        SC.render.floatText(best.x, best.y, '-' + tdmg, '#9fd8ff');
        if (best.hp <= 0) onKilled(st, best);
      }
    }
    for (var s2 = sg.shots.length - 1; s2 >= 0; s2--) {
      sg.shots[s2].ttl -= dt;
      if (sg.shots[s2].ttl <= 0) sg.shots.splice(s2, 1);
    }
    U.emit('hud:update');
  }

  function heroAttack(st) {
    var sg = st.siege;
    if (!sg || sg.over) return;
    var p = st.player, hero = sg.hero;
    if (hero.atkCd > 0) return;
    var eff = E.effective(p);
    hero.atkCd = U.clamp(900 - eff.spd * 22, 260, 900);
    hero.swingUntil = Date.now() + 170;
    U.emit('sfx', 'swing');
    var hits = 0;
    for (var i = 0; i < sg.attackers.length; i++) {
      var a = sg.attackers[i];
      if (a.hp <= 0) continue;
      if (U.dist(a.x, a.y, hero.x, hero.y) <= 1.6) {
        var dmg = SC.combat.computeDamage(eff.atk, a.def, 1);
        a.hp -= dmg;
        hits++;
        SC.render.floatText(a.x, a.y, '-' + dmg, '#ffd35c');
        SC.render.burst(a.x, a.y, a.color, 4);
        if (a.hp <= 0) onKilled(st, a);
      }
    }
    if (hits) U.emit('sfx', 'hit');
  }

  function heroDash(st) {
    var sg = st.siege;
    if (!sg || sg.over) return;
    var hero = sg.hero;
    if (hero.dashCd > 0) return;
    hero.dashCd = 1600;
    hero.x = U.clamp(hero.x + hero.dirX * 2.2, 0.2, SC.haven.GRID_W - 0.2);
    hero.y = U.clamp(hero.y + hero.dirY * 2.2, 0.2, SC.haven.GRID_H - 0.2);
    SC.render.burst(hero.x, hero.y, '#9fd8ff', 6);
    U.emit('sfx', 'dash');
  }

  function onKilled(st, a) {
    var sg = st.siege;
    sg.kills++;
    SC.render.burst(a.x, a.y, a.color, 10);
    U.emit('sfx', 'kill');
    E.gainXp(st.player, Math.round(a.xp * 0.6));
  }

  function finish(st, won) {
    var sg = st.siege, p = st.player, h = st.haven;
    sg.over = true; sg.won = won;
    h.lastSiegePlayed = Date.now();
    var kl = SC.haven.keepLevel(h);
    if (won) {
      var gold = 120 * kl + sg.kills * 8;
      p.gold += gold;
      E.gainXp(p, 80 * kl);
      var mats = Object.keys(SC.DATA.materials || {});
      var lootLines = ['💰 ' + gold + ' gold', '✨ ' + (80 * kl) + ' XP'];
      if (mats.length) {
        var rng = new U.Rng((Date.now() & 0xffffff));
        for (var i = 0; i < 2; i++) {
          var mid = rng.pick(mats);
          E.addItem(p, mid, rng.int(1, 3));
          var mdef = E.lookup('materials', mid) || { name: mid };
          lootLines.push('🪨 ' + mdef.name);
        }
      }
      p.stats.siegesWon = (p.stats.siegesWon || 0) + 1;
      U.emit('sfx', 'victory');
      U.emit('siege:over', { won: true, loot: lootLines, kills: sg.kills });
    } else {
      var loss = Math.min(p.gold, 30 * kl);
      p.gold -= loss;
      U.emit('sfx', 'death');
      U.emit('siege:over', { won: false, loss: loss, kills: sg.kills });
    }
    SC.systems.checkAchievements(p);
    SC.game.save();
    return true;
  }

  return {
    start: start, abort: abort, tick: tick,
    heroAttack: heroAttack, heroDash: heroDash,
    canStart: canStart, nextIn: nextIn, COOLDOWN_MS: COOLDOWN_MS
  };
})();

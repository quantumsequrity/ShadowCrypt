'use strict';
/* ShadowCrypt Online — core game state, main loop, crypt simulation, saves */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.game = (function () {
  var U = SC.util, E = SC.entities, W = SC.worldgen, C = SC.combat;

  var SAVE_KEY = 'shadowcrypt_save_v1';
  var MOVE_MS = 140;

  var st = {
    started: false,
    mode: 'crypt',            // crypt | haven | farm | arena
    player: null,
    haven: null,
    map: null,
    floors: {},               // floor number -> {map, monsters, groundItems}
    monsters: [],
    allies: [],               // companions + summons (crypt instances)
    groundItems: [],          // {x,y,id?,qty,rarity} or {x,y,gold}
    projectiles: [],
    moveCd: 0,
    atkCd: 0,
    path: null,               // tap-to-move path
    runSeed: 0,
    buildPlacing: null,       // {type,x,y} while placing a building
    selectedBuilding: null,
    lastSaveAt: 0,
    cloudDirty: false,
    blessedRun: null
  };

  // ------------------------------------------------------------------ boot
  function newGame(opts) {
    st.player = E.createPlayer(opts);
    st.haven = SC.haven.createHaven();
    st.player.haven = st.haven;
    st.floors = {};
    st.runSeed = (Date.now() & 0xffffff);
    enterFloor(1, true);
    st.started = true;
    save();
    U.emit('game:started');
    U.emit('msg', 'Welcome to ShadowCrypt, ' + st.player.name + '. Descend if you dare.');
  }

  function loadGame() {
    var data = U.storeGet(SAVE_KEY, null);
    if (!data || !data.player) return false;
    return applySave(data);
  }

  function applySave(data) {
    try {
      st.player = data.player;
      st.haven = data.player.haven || SC.haven.createHaven();
      st.player.haven = st.haven;
      st.player.effects = st.player.effects || [];
      st.player.cooldowns = {};
      st.floors = {};
      st.runSeed = (Date.now() & 0xffffff);
      // offline progress
      var siegeEvents = SC.haven.resolveSieges(st.player, st.haven);
      enterFloor(Math.min(data.floor || 1, 1) === 1 ? 1 : 1, true); // always resume at floor 1... unless portal
      var depth = SC.haven.portalDepth(st.haven);
      if ((data.floor || 1) > 1 && depth > 1) enterFloor(Math.min(data.floor, depth), true);
      st.started = true;
      U.emit('game:started');
      siegeEvents.forEach(function (ev) { U.emit('msg', ev); });
      U.emit('msg', 'Welcome back, ' + st.player.name + '.');
      return true;
    } catch (e) {
      console.error('save corrupt', e);
      return false;
    }
  }

  function hasSave() { return !!U.storeGet(SAVE_KEY, null); }

  function save() {
    if (!st.player) return;
    var data = { player: st.player, floor: st.player.floor, savedAt: Date.now(), v: 1 };
    U.storeSet(SAVE_KEY, data);
    st.lastSaveAt = Date.now();
    if (SC.net && SC.net.isConnected()) SC.net.cloudSave(data);
  }

  function wipeSave() { U.storeDel(SAVE_KEY); }

  // --------------------------------------------------------------- floors
  function floorSeed(floor) {
    // co-op: same seed for everyone in the same hour+floor when online, personal otherwise
    return U.hashStr('crypt:' + floor + ':' + st.runSeed);
  }

  function enterFloor(floor, fresh) {
    var p = st.player;
    p.floor = floor;
    var cached = st.floors[floor];
    if (cached && !fresh) {
      st.map = cached.map;
      st.monsters = cached.monsters;
      st.groundItems = cached.groundItems;
    } else {
      var map = W.generate(floor, floorSeed(floor));
      st.map = map;
      st.monsters = [];
      st.groundItems = [];
      populateFloor(map, floor);
      st.floors[floor] = { map: map, monsters: st.monsters, groundItems: st.groundItems };
    }
    p.x = st.map.spawn.x; p.y = st.map.spawn.y;
    p.fx = p.x; p.fy = p.y;
    st.projectiles = [];
    st.path = null;
    // companions come along
    st.allies = [];
    (p.companions || []).forEach(function (comp) {
      comp.x = p.x; comp.y = p.y; comp.fx = p.x; comp.fy = p.y;
      comp.hp = comp.maxHp;
      st.allies.push(comp);
    });
    W.computeFov(st.map, p.x, p.y, fovRadius());
    p.stats.deepestFloor = Math.max(p.stats.deepestFloor, floor);
    SC.systems.questProgress(p, 'explore', floor, 1);
    SC.systems.checkAchievements(p);
    var theme = st.map.theme || {};
    U.emit('floor:entered', { floor: floor, theme: theme.name });
    U.emit('msg', '— Floor ' + floor + ': ' + (theme.name || 'Unknown depths') + ' —');
  }

  function populateFloor(map, floor) {
    var rng = new U.Rng(floorSeed(floor) ^ 0x9e3779b9);
    var tier = Math.min(8, Math.ceil(floor / 4));
    var pool = E.enemyPool(tier);
    map.monsterSpawns.forEach(function (pos) {
      if (!pool.length) return;
      var tpl = rng.pick(pool);
      st.monsters.push(E.spawnMonster(tpl, pos.x, pos.y, floor));
    });
    if (map.bossSpawn) {
      var boss = E.bossFor(floor);
      if (boss) {
        var bm = E.spawnMonster(Object.assign({}, boss, { flags: Object.assign({}, boss.flags, { boss: true }) }), map.bossSpawn.x, map.bossSpawn.y, floor);
        st.monsters.push(bm);
      }
    }
    if (map.miniBossSpawn) {
      var mini = E.miniBossFor(floor);
      if (mini) {
        var mm = E.spawnMonster(Object.assign({}, mini, { flags: Object.assign({}, mini.flags, { miniBoss: true }) }), map.miniBossSpawn.x, map.miniBossSpawn.y, floor);
        st.monsters.push(mm);
      }
    }
    map.itemSpawns.forEach(function (pos) {
      var drops = C.rollLoot(rng, floor, 2);
      drops.forEach(function (d) {
        if (d.gold) st.groundItems.push({ x: pos.x, y: pos.y, gold: d.gold });
        else st.groundItems.push({ x: pos.x, y: pos.y, id: d.id, qty: d.qty, rarity: d.rarity });
      });
    });
  }

  function fovRadius() {
    var r = 10;
    if (E.hasEffect(st.player, 'torchlight')) r += 3;
    if (E.hasEffect(st.player, 'blind')) r = 3;
    return r;
  }

  // ---------------------------------------------------------------- loop
  var lastT = 0;
  function frame(t) {
    var dt = Math.min(100, t - lastT || 16);
    lastT = t;
    if (st.started) {
      if (st.mode === 'crypt') {
        tickCrypt(dt);
        SC.render.renderCrypt(st, dt);
      } else if (st.mode === 'haven' || st.mode === 'farm') {
        SC.render.renderHaven(st, dt, st.mode === 'farm');
      } else if (st.mode === 'arena') {
        var input = SC.input.moveVector();
        SC.arena.tick(dt, { mx: input.x, my: input.y, fire: SC.input.isAttackHeld(), bomb: false });
        SC.render.renderArena(dt);
      }
      // autosave
      if (Date.now() - st.lastSaveAt > 20000) save();
    }
    U.emit('frame', dt);
    if (typeof requestAnimationFrame !== 'undefined') requestAnimationFrame(frame);
  }

  var netPosAcc = 0;

  function tickCrypt(dt) {
    var p = st.player;
    if (!p || p.hp <= 0) return;

    st.moveCd -= dt; st.atkCd -= dt;

    // movement
    var mv = SC.input.moveVector();
    if ((mv.x || mv.y) && st.moveCd <= 0) {
      st.path = null;
      var dx = Math.abs(mv.x) > 0.35 ? Math.sign(mv.x) : 0;
      var dy = Math.abs(mv.y) > 0.35 ? Math.sign(mv.y) : 0;
      if (dx || dy) tryMove(dx, dy);
    } else if (st.path && st.path.length && st.moveCd <= 0) {
      var next = st.path[0];
      var pdx = Math.sign(next.x - p.x), pdy = Math.sign(next.y - p.y);
      if (tryMove(pdx, pdy)) {
        if (p.x === next.x && p.y === next.y) st.path.shift();
      } else st.path = null;
    }

    // held attack auto-swings
    if (SC.input.isAttackHeld() && st.atkCd <= 0) doAttack();

    // monsters
    var aictx = monsterCtx();
    for (var i = 0; i < st.monsters.length; i++) {
      SC.ai.tickMonster(aictx, st.monsters[i], dt);
      var mfx = E.tickEffects(st.monsters[i], dt);
      mfx.forEach(function (ev) {
        if (ev.dmg > 0) {
          st.monsters[i].hp -= ev.dmg;
          if (st.monsters[i].hp <= 0) onMonsterKilled(st.monsters[i], false);
        }
      });
    }
    // clean dead
    for (var d = st.monsters.length - 1; d >= 0; d--) {
      if (st.monsters[d].hp <= 0 && !st.monsters[d]._looted) onMonsterKilled(st.monsters[d], false);
      if (st.monsters[d].hp <= 0) st.monsters.splice(d, 1);
    }

    // allies
    for (var a = 0; a < st.allies.length; a++) SC.ai.tickAlly(aictx, st.allies[a], dt);
    st.allies = st.allies.filter(function (al) { return al.hp > 0 || al.cid; }); // companions persist at 0 (revive at floor change)

    // projectiles
    tickProjectiles(dt);

    // player effects + hunger
    var pev = E.tickEffects(p, dt);
    var eff = E.effective(p);
    pev.forEach(function (ev) {
      if (ev.dmg > 0) {
        p.hp -= ev.dmg;
        SC.render.floatText(p.x, p.y, '-' + ev.dmg, '#7ddc5c');
      } else if (ev.dmg < 0) {
        p.hp = Math.min(eff.maxHp, p.hp - ev.dmg);
      }
    });
    var hungerMsg = SC.systems.tickHunger(p, dt, true);
    if (hungerMsg) U.emit('msg', hungerMsg);
    if (p.hunger <= 0) {
      p._starveAcc = (p._starveAcc || 0) + dt;
      if (p._starveAcc > 2000) { p._starveAcc = 0; p.hp -= 2; SC.render.floatText(p.x, p.y, '-2', '#c88'); }
    }
    // slow mana regen
    p._mpAcc = (p._mpAcc || 0) + dt;
    if (p._mpAcc > 1500) { p._mpAcc = 0; p.mp = Math.min(eff.maxMp, p.mp + 1 + Math.floor(p.level / 5)); }

    // standing on lava
    if (st.map.get(p.x, p.y) === SC.TILE.LAVA) {
      p._lavaAcc = (p._lavaAcc || 0) + dt;
      if (p._lavaAcc > 600) { p._lavaAcc = 0; p.hp -= 5; E.addEffect(p, 'burning', 3); SC.render.floatText(p.x, p.y, '-5 LAVA', '#ff7a3c'); }
    }

    if (p.hp <= 0) return die();
    if (p.hp === 1 && !p.stats.closeCalls) { p.stats.closeCalls = 1; SC.systems.checkAchievements(p); }

    // online presence
    netPosAcc += dt;
    if (netPosAcc > 2000) {
      netPosAcc = 0;
      if (SC.net) SC.net.reportPos(p);
    }
    U.emit('hud:update');
  }

  function monsterCtx() {
    return {
      map: st.map,
      player: st.player,
      monsters: st.monsters,
      allies: st.allies,
      occupied: function (x, y, self) {
        if (st.player.x === x && st.player.y === y) return true;
        for (var i = 0; i < st.monsters.length; i++) {
          var m = st.monsters[i];
          if (m !== self && m.hp > 0 && m.x === x && m.y === y) return true;
        }
        for (var j = 0; j < st.allies.length; j++) {
          var al = st.allies[j];
          if (al !== self && al.hp > 0 && al.x === x && al.y === y) return true;
        }
        return false;
      },
      monsterAttackPlayer: function (m) {
        var res = C.monsterAttack(m, st.player);
        SC.render.floatText(st.player.x, st.player.y, '-' + res.dmg, '#ff6b6b');
        SC.render.burst(st.player.x, st.player.y, '#e74c3c', 5);
        // boss specials
        if (m.boss && Math.random() < 0.25) {
          var fx = ['burning', 'frozen', 'weakened', 'poisoned'][Math.floor(Math.random() * 4)];
          E.addEffect(st.player, fx, 4);
          U.emit('msg', m.name + ' afflicts you with ' + fx + '!');
        }
      },
      monsterAttackAlly: function (m, al) {
        var dmg = C.computeDamage(m.atk, al.def, 1);
        al.hp -= dmg;
        SC.render.floatText(al.x, al.y, '-' + dmg, '#ffb0a0');
      },
      fireMonsterProjectile: function (m, target) {
        var dx = target.x - m.x, dy = target.y - m.y;
        var len = Math.sqrt(dx * dx + dy * dy) || 1;
        st.projectiles.push({
          x: m.x + 0.5, y: m.y + 0.5, vx: dx / len * 7, vy: dy / len * 7,
          ttl: 1600, from: 'monster', dmg: m.atk, color: m.color
        });
      },
      onMonsterKilled: onMonsterKilled,
      spawnFloatText: function (x, y, txt, color) { SC.render.floatText(x, y, txt, color); }
    };
  }

  function tickProjectiles(dt) {
    var p = st.player;
    for (var i = st.projectiles.length - 1; i >= 0; i--) {
      var pr = st.projectiles[i];
      pr.x += pr.vx * dt / 1000; pr.y += pr.vy * dt / 1000; pr.ttl -= dt;
      var tx = Math.floor(pr.x), ty = Math.floor(pr.y);
      var dead = pr.ttl <= 0 || st.map.blocksSight(tx, ty);
      if (!dead && pr.from === 'player') {
        for (var mi = 0; mi < st.monsters.length; mi++) {
          var m = st.monsters[mi];
          if (m.hp > 0 && U.dist(pr.x - 0.5, pr.y - 0.5, m.x, m.y) < 0.6) {
            var eff = E.effective(p);
            var dmg = C.computeDamage(eff.atk * (pr.mult || 1), m.def, 1);
            m.hp -= dmg; m.aiState = 'chase';
            SC.render.floatText(m.x, m.y, '-' + dmg, '#ffd35c');
            if (m.hp <= 0) onMonsterKilled(m, false);
            dead = true;
            break;
          }
        }
      } else if (!dead && pr.from === 'monster') {
        if (U.dist(pr.x - 0.5, pr.y - 0.5, p.x, p.y) < 0.6) {
          var eff2 = E.effective(p);
          var dmg2 = Math.max(1, Math.round(pr.dmg * (0.85 + Math.random() * 0.3) - eff2.def));
          p.hp -= dmg2;
          SC.render.floatText(p.x, p.y, '-' + dmg2, '#ff6b6b');
          dead = true;
        }
      }
      if (dead) {
        SC.render.burst(pr.x - 0.5, pr.y - 0.5, pr.color || '#ffd35c', 3);
        st.projectiles.splice(i, 1);
      }
    }
  }

  // ------------------------------------------------------------- movement
  function tryMove(dx, dy) {
    var p = st.player;
    if (E.hasEffect(p, 'frozen') || E.hasEffect(p, 'stunned')) return false;
    if (E.hasEffect(p, 'confused') && Math.random() < 0.4) {
      var rd = U.DIRS8[Math.floor(Math.random() * 8)];
      dx = rd[0]; dy = rd[1];
    }
    var nx = p.x + dx, ny = p.y + dy;
    p.dirX = dx; p.dirY = dy;

    // bump attack
    var m = monsterAt(nx, ny);
    if (m) { doAttack(m); return false; }

    var tile = st.map.get(nx, ny);
    if (tile === SC.TILE.DOOR_CLOSED) {
      st.map.set(nx, ny, SC.TILE.DOOR_OPEN);
      U.emit('msg', 'You open the door.');
      st.moveCd = MOVE_MS;
      afterMoveFov();
      return true;
    }
    if (tile === SC.TILE.CHEST) { openChest(nx, ny); return false; }
    if (tile === SC.TILE.SHRINE) { useShrine(nx, ny); return false; }
    if (tile === SC.TILE.BOSS_GATE) {
      st.map.set(nx, ny, SC.TILE.FLOOR);
      U.emit('msg', 'The boss gate grinds open… something stirs.');
      return true;
    }
    if (!st.map.isWalkable(nx, ny)) return false;

    var eff = E.effective(p);
    var speedMs = U.clamp(MOVE_MS * (10 / Math.max(4, eff.spd)), 70, 260);
    if (tile === SC.TILE.WATER) speedMs *= 1.7;
    p.x = nx; p.y = ny;
    st.moveCd = speedMs;
    p.hunger = Math.max(0, p.hunger - 0.02);

    if (tile === SC.TILE.TRAP) triggerTrap(nx, ny);
    pickupHere();
    afterMoveFov();
    return true;
  }

  function afterMoveFov() {
    W.computeFov(st.map, st.player.x, st.player.y, fovRadius());
  }

  function monsterAt(x, y) {
    for (var i = 0; i < st.monsters.length; i++) {
      var m = st.monsters[i];
      if (m.hp > 0 && m.x === x && m.y === y) return m;
    }
    return null;
  }

  // ------------------------------------------------------------- actions
  function doAttack(targetMonster) {
    if (st.atkCd > 0) return;
    var p = st.player;
    var eff = E.effective(p);
    st.atkCd = U.clamp(900 - eff.spd * 22, 260, 900);

    var isRangedClass = ['ranger', 'mage', 'necromancer'].indexOf(p.classId) >= 0;
    var weapon = p.equipment.weapon && E.itemDef(p.equipment.weapon.id);
    var ranged = (weapon && weapon.ranged) || (!weapon && isRangedClass);

    var m = targetMonster;
    if (!m) {
      // nearest visible monster
      var best = null, bd = 1e9;
      for (var i = 0; i < st.monsters.length; i++) {
        var mm = st.monsters[i];
        if (mm.hp <= 0) continue;
        if (!st.map.visible[mm.x + ',' + mm.y]) continue;
        var dd = U.dist(mm.x, mm.y, p.x, p.y);
        if (dd < bd) { bd = dd; best = mm; }
      }
      m = best;
      if (m && !ranged && bd > 1.6) m = null;
      if (m && ranged && bd > 8) m = null;
    }
    if (!m) {
      SC.render.burst(p.x + p.dirX * 0.5, p.y + p.dirY * 0.5, '#8ea2c9', 3);
      return;
    }

    if (ranged && U.cheb(p.x, p.y, m.x, m.y) > 1) {
      var dx = m.x - p.x, dy = m.y - p.y;
      var len = Math.sqrt(dx * dx + dy * dy) || 1;
      st.projectiles.push({
        x: p.x + 0.5, y: p.y + 0.5, vx: dx / len * 10, vy: dy / len * 10,
        ttl: 1200, from: 'player', color: p.classId === 'mage' ? '#7ec8ff' : '#e8d9a0'
      });
      return;
    }

    // behind check for rogue: attacking in the monster's rear半
    var behind = (m.x - p.x) * p.dirX + (m.y - p.y) * p.dirY > 0 && Math.random() < 0.4;
    var res = C.playerAttack(p, m, { behind: behind });
    SC.render.floatText(m.x, m.y, '-' + res.dmg + (res.crit ? '!' : ''), res.crit ? '#ffec6b' : '#ffd35c');
    SC.render.burst(m.x, m.y, m.color, 4);
    if (res.killed) onMonsterKilled(m, false);
  }

  function onMonsterKilled(m, byAlly) {
    if (m._looted) return;
    m._looted = true;
    var p = st.player;
    p.stats.kills++;
    if (m.boss) p.stats.bossKills++;
    var xpMult = st.blessedRun === 'bl_xp' ? 1.5 : 1;
    var leveled = E.gainXp(p, Math.round(m.xp * xpMult));
    SC.render.floatText(m.x, m.y, '+' + Math.round(m.xp * xpMult) + 'xp', '#c88ae8');
    SC.render.burst(m.x, m.y, m.color, 10);

    var rng = new U.Rng((Date.now() & 0xffffff) ^ U.hashStr(m.mid));
    var luck = (st.blessedRun === 'bl_luck' ? 4 : 0) + (E.hasEffect(p, 'lucky') ? 3 : 0);
    var drops = C.rollLoot(rng, p.floor, luck);
    if (m.boss || m.miniBoss) drops = drops.concat(C.chestLoot(rng, p.floor));
    drops.forEach(function (d) {
      if (d.gold) st.groundItems.push({ x: m.x, y: m.y, gold: d.gold });
      else st.groundItems.push({ x: m.x, y: m.y, id: d.id, qty: d.qty, rarity: d.rarity });
    });

    SC.systems.questProgress(p, 'kill', m.id, 1);
    if (m.boss) {
      SC.systems.questProgress(p, 'boss', m.id, 1);
      U.emit('toast', { text: '☠ ' + m.name + ' defeated!', cls: 'gold' });
      U.emit('msg', 'The ' + m.name + ' falls! The way deeper is open.');
      if (p.floor >= 30) winGame();
    }
    SC.systems.checkAchievements(p);
    if (leveled) U.emit('toast', { text: '⬆ Level ' + p.level + '!', cls: 'gold' });
  }

  function pickupHere() {
    var p = st.player;
    for (var i = st.groundItems.length - 1; i >= 0; i--) {
      var g = st.groundItems[i];
      if (g.x !== p.x || g.y !== p.y) continue;
      if (g.gold) {
        p.gold += g.gold;
        SC.render.floatText(p.x, p.y, '+' + g.gold + 'g', '#f1c40f');
        st.groundItems.splice(i, 1);
      } else {
        if (E.addItem(p, g.id, g.qty || 1, g.rarity)) {
          var def = E.itemDef(g.id) || E.lookup('materials', g.id) || { name: g.id };
          U.emit('msg', 'Picked up: ' + def.name + ((g.qty || 1) > 1 ? ' ×' + g.qty : ''));
          SC.systems.questProgress(p, 'collect', g.id, g.qty || 1);
          st.groundItems.splice(i, 1);
        }
      }
    }
  }

  function openChest(x, y) {
    var p = st.player;
    st.map.set(x, y, SC.TILE.CHEST_OPEN);
    p.stats.chestsOpened++;
    var rng = new U.Rng(U.hashStr('chest:' + p.floor + ':' + x + ':' + y));
    var drops = C.chestLoot(rng, p.floor);
    drops.forEach(function (d) {
      if (d.gold) { p.gold += d.gold; SC.render.floatText(x, y, '+' + d.gold + 'g', '#f1c40f'); }
      else st.groundItems.push({ x: x, y: y, id: d.id, qty: d.qty, rarity: d.rarity });
    });
    U.emit('msg', 'The chest creaks open…');
    SC.systems.checkAchievements(p);
  }

  function useShrine(x, y) {
    var p = st.player;
    st.map.set(x, y, SC.TILE.SHRINE_USED);
    var eff = E.effective(p);
    var roll = Math.random();
    if (roll < 0.3) { p.hp = eff.maxHp; p.mp = eff.maxMp; U.emit('msg', '✨ The shrine restores you fully!'); }
    else if (roll < 0.5) { E.addEffect(p, 'strengthened', 60); U.emit('msg', '✨ You feel a surge of strength!'); }
    else if (roll < 0.7) { E.addEffect(p, 'shielded', 60); U.emit('msg', '✨ A protective aura surrounds you!'); }
    else if (roll < 0.85) { E.addEffect(p, 'regenerating', 60); U.emit('msg', '✨ Your wounds begin to knit themselves!'); }
    else { E.gainXp(p, E.xpForLevel(p.level) / 3 | 0); U.emit('msg', '✨ Ancient knowledge fills your mind!'); }
  }

  function triggerTrap(x, y) {
    var p = st.player;
    st.map.trapRevealed[x + ',' + y] = true;
    var dmg = 3 + p.floor * 2;
    var eff = E.effective(p);
    dmg = Math.max(1, dmg - Math.floor(eff.def / 3));
    p.hp -= dmg;
    SC.render.floatText(x, y, '-' + dmg + ' TRAP', '#ff8c5c');
    var roll = Math.random();
    if (roll < 0.25) E.addEffect(p, 'poisoned', 6);
    else if (roll < 0.4) E.addEffect(p, 'bleeding', 5);
    U.emit('msg', '⚠ You stepped on a trap!');
    if (p.hp <= 0) die();
  }

  function interact() {
    var p = st.player;
    var tile = st.map.get(p.x, p.y);
    if (tile === SC.TILE.STAIRS_DOWN) return descend();
    if (tile === SC.TILE.STAIRS_UP) return ascend();
    pickupHere();
    // adjacent interactables
    for (var d = 0; d < 8; d++) {
      var nx = p.x + U.DIRS8[d][0], ny = p.y + U.DIRS8[d][1];
      var t = st.map.get(nx, ny);
      if (t === SC.TILE.CHEST) return openChest(nx, ny);
      if (t === SC.TILE.SHRINE) return useShrine(nx, ny);
      if (t === SC.TILE.DOOR_CLOSED) { st.map.set(nx, ny, SC.TILE.DOOR_OPEN); afterMoveFov(); return U.emit('msg', 'You open the door.'); }
    }
    U.emit('msg', 'Nothing to interact with here.');
  }

  function descend() {
    var p = st.player;
    if (st.map.get(p.x, p.y) !== SC.TILE.STAIRS_DOWN) return U.emit('msg', 'Find the stairs down (>) first.');
    if (p.floor >= 30) return U.emit('msg', 'This is the deepest floor.');
    enterFloor(p.floor + 1, false);
    save();
  }

  function ascend() {
    var p = st.player;
    if (st.map.get(p.x, p.y) !== SC.TILE.STAIRS_UP) return U.emit('msg', 'Find the stairs up (<) first.');
    if (p.floor <= 1) { switchMode('haven'); U.emit('msg', 'You climb out into your haven.'); return; }
    enterFloor(p.floor - 1, false);
  }

  function useSkillSlot(idx) {
    var p = st.player;
    if (st.mode !== 'crypt') return;
    var skillId = p.skillSlots[idx];
    if (!skillId) return U.emit('msg', 'No skill in that slot yet.');
    var res = C.useSkill(skillCtx(), skillId);
    if (res.ok) { if (res.msg) U.emit('msg', res.msg); }
    else if (res.reason === 'mana') U.emit('msg', 'Not enough mana!');
    else if (res.reason === 'cooldown') U.emit('msg', 'Still on cooldown.');
    else if (res.reason === 'notarget') U.emit('msg', 'No target in range.');
    U.emit('hud:update');
  }

  function skillCtx() {
    var p = st.player;
    return {
      player: p,
      monsters: st.monsters,
      map: st.map,
      spawnFloatText: function (x, y, txt, c) { SC.render.floatText(x, y, txt, c); },
      fireProjectile: function (from, target, mult) {
        var dx = target.x - from.x, dy = target.y - from.y;
        var len = Math.sqrt(dx * dx + dy * dy) || 1;
        st.projectiles.push({ x: from.x + 0.5, y: from.y + 0.5, vx: dx / len * 10, vy: dy / len * 10, ttl: 1300, from: 'player', mult: mult || 1, color: '#c8ffb0' });
      },
      summonAlly: function (skillId) {
        if (st.allies.filter(function (a) { return !a.cid; }).length >= 3) { U.emit('msg', 'You cannot control more summons.'); return; }
        var isDemon = /imp|demon/.test(skillId);
        var s = {
          sid: U.uid(), name: isDemon ? 'Imp' : 'Skeleton', glyph: isDemon ? 'i' : 's',
          x: p.x, y: p.y, fx: p.x, fy: p.y,
          hp: 15 + p.level * 3, maxHp: 15 + p.level * 3,
          atk: 4 + p.level, def: 1 + Math.floor(p.level / 3), spd: 10,
          effects: []
        };
        st.allies.push(s);
        SC.render.burst(p.x, p.y, isDemon ? '#e74c3c' : '#cfd8ea', 8);
      },
      teleportRandom: function () {
        var rng = new U.Rng((Date.now() & 0xffffff));
        for (var t = 0; t < 200; t++) {
          var x = rng.int(1, st.map.w - 2), y = rng.int(1, st.map.h - 2);
          if (st.map.isWalkable(x, y) && !monsterAt(x, y)) {
            p.x = x; p.y = y; p.fx = x; p.fy = y;
            afterMoveFov();
            return;
          }
        }
      },
      revealMap: function () {
        for (var y = 0; y < st.map.h; y++)
          for (var x = 0; x < st.map.w; x++)
            st.map.explored[x + ',' + y] = true;
      }
    };
  }

  function useInventoryItem(invIndex) {
    var p = st.player;
    var stk = p.inventory[invIndex];
    if (!stk) return;
    var def = E.itemDef(stk.id);
    if (!def) return U.emit('msg', 'You cannot use that.');
    var equipKinds = ['weapon', 'shield', 'armor', 'helmet', 'gloves', 'boots', 'ring', 'amulet'];
    if (equipKinds.indexOf(def.kind) >= 0) {
      if (E.equipItem(p, invIndex)) U.emit('msg', 'Equipped: ' + def.name);
      return;
    }
    var res = C.useConsumable(skillCtx(), invIndex);
    if (res.ok && res.msg) U.emit('msg', res.msg);
    else if (!res.ok) U.emit('msg', 'You cannot use that here.');
    U.emit('hud:update');
  }

  function die() {
    var p = st.player;
    p.stats.deaths++;
    SC.systems.checkAchievements(p);
    var lost = Math.floor(p.gold * 0.1);
    p.gold -= lost;
    U.emit('toast', { text: '💀 You died! Lost ' + lost + ' gold.', cls: 'bad' });
    U.emit('msg', 'Darkness takes you… but your haven calls you back.');
    p.effects = [];
    var eff = E.effective(p);
    p.hp = Math.max(1, Math.floor(eff.maxHp * 0.5));
    p.mp = Math.floor(eff.maxMp * 0.5);
    p.hunger = Math.max(p.hunger, 40);
    st.floors = {}; // dungeon reshuffles on death
    st.runSeed = (Date.now() & 0xffffff);
    switchMode('haven');
    save();
    U.emit('player:died');
  }

  function winGame() {
    U.emit('toast', { text: '👑 THE DEMON KING IS SLAIN!', cls: 'gold' });
    U.emit('game:won');
    var p = st.player;
    p.gold += 5000;
    E.gainXp(p, 10000);
    save();
  }

  // --------------------------------------------------------------- modes
  function switchMode(mode) {
    if (mode === st.mode) return;
    if (st.mode === 'arena') { SC.arena.leave(); if (SC.net) SC.net.leaveArena(); }
    st.mode = mode;
    st.buildPlacing = null;
    st.selectedBuilding = null;
    if (mode === 'arena') {
      var p = st.player;
      var online = SC.net && SC.net.isConnected();
      if (online) {
        SC.net.joinArena();
        // enter local arena immediately with shared seed once joined; start bots-less
        SC.arena.enter(p.name, SC.render.classColor(p.classId), { seed: (Date.now() / 60000) | 0 });
      } else {
        SC.arena.enter(p.name, SC.render.classColor(p.classId), null);
        U.emit('msg', 'Offline arena: fight the training bots!');
      }
    }
    if (mode === 'crypt' && st.map) {
      // returning underground
      afterMoveFov();
    }
    U.emit('mode:changed', mode);
  }

  function startDescent(fromFloor) {
    st.floors = {};
    st.runSeed = (Date.now() & 0xffffff);
    var bl = SC.haven.activeBlessing(st.haven);
    st.blessedRun = bl ? bl.id : null;
    if (st.blessedRun === 'bl_iron') E.addEffect(st.player, 'shielded', 300);
    if (st.blessedRun === 'bl_fury') E.addEffect(st.player, 'strengthened', 300);
    enterFloor(fromFloor || 1, true);
    switchMode('crypt');
  }

  // --------------------------------------------------------------- taps
  function onCanvasTap(pt) {
    var p = st.player;
    if (!st.started) return;
    if (st.mode === 'crypt') {
      var TILE = SC.render.tileSize();
      var cam = SC.render.camera();
      var tx = Math.floor((pt.x + cam.x) / TILE), ty = Math.floor((pt.y + cam.y) / TILE);
      if (!st.map.inb(tx, ty)) return;
      var m = monsterAt(tx, ty);
      if (m && U.cheb(p.x, p.y, tx, ty) <= 1) { doAttack(m); return; }
      if (m && st.map.visible[tx + ',' + ty]) {
        // path to it then attack
        var pth = W.findPath(st.map, p.x, p.y, tx, ty, 1200);
        if (pth) { pth.pop(); st.path = pth; }
        return;
      }
      if (st.map.explored[tx + ',' + ty]) {
        var path = W.findPath(st.map, p.x, p.y, tx, ty, 1600);
        if (path) st.path = path;
      }
    } else if (st.mode === 'haven' || st.mode === 'farm') {
      havenTap(pt);
    }
  }

  function havenTap(pt) {
    var lay = st._havenLayout;
    if (!lay) return;
    var gx = Math.floor((pt.x - lay.ox) / lay.cell), gy = Math.floor((pt.y - lay.oy) / lay.cell);
    if (gx < 0 || gy < 0 || gx >= SC.haven.GRID_W || gy >= SC.haven.GRID_H) return;
    if (st.buildPlacing) {
      st.buildPlacing.x = gx; st.buildPlacing.y = gy;
      U.emit('haven:place-moved');
      return;
    }
    var cells = SC.haven.occupiedCells(st.haven);
    var b = cells[gx + ',' + gy];
    st.selectedBuilding = b || null;
    U.emit('haven:selected', b || null);
  }

  // --------------------------------------------------------------- wiring
  function init() {
    U.on('action:attack', function () { if (st.mode === 'crypt') doAttack(); else if (st.mode === 'arena') { /* handled by held check */ } });
    U.on('action:interact', function () {
      if (st.mode === 'crypt') interact();
      else if (st.mode === 'arena') { var A = SC.arena.state(); if (A) SC.arena.dropBomb(A.me); }
    });
    U.on('action:skill', useSkillSlot);
    U.on('action:descend', descend);
    U.on('action:ascend', ascend);
    U.on('canvas:tap', onCanvasTap);
    U.on('net:loaded', function (data) {
      // adopt cloud save only if newer than local
      var local = U.storeGet(SAVE_KEY, null);
      if (data && (!local || (data.savedAt || 0) > (local.savedAt || 0))) {
        U.storeSet(SAVE_KEY, data);
        if (!st.started) U.emit('boot:cloud-save-ready');
      }
    });
    U.on('arena:net-joined', function (room) {
      if (st.mode === 'arena') {
        var p = st.player;
        SC.arena.enter(p.name, SC.render.classColor(p.classId), { seed: room.seed });
      }
    });
    if (typeof document !== 'undefined') {
      document.addEventListener('visibilitychange', function () {
        if (document.visibilityState === 'hidden' && st.started) save();
      });
    }
    if (typeof requestAnimationFrame !== 'undefined') requestAnimationFrame(frame);
  }

  return {
    state: st,
    init: init,
    newGame: newGame, loadGame: loadGame, hasSave: hasSave, save: save, wipeSave: wipeSave,
    switchMode: switchMode, startDescent: startDescent,
    interact: interact, descend: descend, ascend: ascend,
    useSkillSlot: useSkillSlot, useInventoryItem: useInventoryItem,
    enterFloor: enterFloor
  };
})();

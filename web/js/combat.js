'use strict';
/* ShadowCrypt Online — combat: damage, skills, projectiles, loot */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.combat = (function () {
  var U = SC.util, E = SC.entities;

  function variance(rng) { return 0.85 + (rng ? rng.float() : Math.random()) * 0.3; }

  function computeDamage(atk, def, mult) {
    var raw = atk * (mult || 1) * variance() - def;
    return Math.max(1, Math.round(raw));
  }

  // Player attacks a monster (melee)
  function playerAttack(p, m, opts) {
    opts = opts || {};
    var eff = E.effective(p);
    var mult = opts.mult || 1;
    // Rogue backstab: attacking from monster's rear arc
    if (p.classId === 'rogue' && opts.behind) mult *= 3;
    // crit chance from speed
    var crit = Math.random() < Math.min(0.35, 0.05 + eff.spd * 0.005);
    if (crit) mult *= 1.6;
    // holy bonus vs undead (paladin theme)
    if (m.undead && (p.classId === 'paladin' || opts.holy)) mult *= 1.5;
    var dmg = computeDamage(eff.atk, m.def, mult);
    m.hp -= dmg;
    m.aiState = 'chase';
    m.flashUntil = Date.now() + 90;
    // gear affix procs
    var procs = E.affixProcs(p);
    var procApplied = null;
    for (var i = 0; i < procs.length; i++) {
      var a = procs[i];
      if (a.proc && Math.random() < (a.procChance || 0.15)) {
        E.addEffect(m, a.proc, 4);
        procApplied = a.proc;
      }
    }
    if (eff.lifesteal > 0) {
      p.hp = Math.min(eff.maxHp, p.hp + Math.max(1, Math.round(dmg * eff.lifesteal)));
    }
    return { dmg: dmg, crit: crit, killed: m.hp <= 0, proc: procApplied };
  }

  function monsterAttack(m, p) {
    var eff = E.effective(p);
    var mult = 1;
    if (E.hasEffect(p, 'berserk')) mult *= 1.5; // berserk takes extra damage
    var dmg = computeDamage(m.atk, eff.def, mult);
    if (E.hasEffect(p, 'shielded')) dmg = Math.max(1, Math.round(dmg * 0.6));
    p.hp -= dmg;
    // elite on-hit effects
    if (m.affix) {
      if (m.affix.onHit && Math.random() < 0.35) E.addEffect(p, m.affix.onHit, 4);
      if (m.affix.lifesteal) m.hp = Math.min(m.maxHp, m.hp + Math.round(dmg * m.affix.lifesteal));
    }
    return { dmg: dmg };
  }

  // ---- Loot ---------------------------------------------------------------
  var RARITY_WEIGHTS = [
    { w: 55, v: 'common' }, { w: 24, v: 'uncommon' }, { w: 12, v: 'rare' },
    { w: 6, v: 'epic' }, { w: 2.4, v: 'legendary' }, { w: 0.6, v: 'mythic' }
  ];

  function rollRarity(rng, floorBonus) {
    var weights = RARITY_WEIGHTS.map(function (e, i) {
      return { w: e.w * (i >= 3 ? (1 + (floorBonus || 0) * 0.08) : 1), v: e.v };
    });
    return rng.weighted(weights);
  }

  function lootableItems() {
    var items = (SC.DATA && SC.DATA.items) || {};
    var out = [];
    for (var id in items) {
      var it = items[id];
      if (it.kind === 'special' && it.id !== 'bomb' && it.id !== 'torch') continue;
      out.push(it);
    }
    return out;
  }

  var EQUIP_KINDS = ['weapon', 'shield', 'armor', 'helmet', 'gloves', 'boots', 'ring', 'amulet'];

  // Affix count scales with rarity: rare 35%/1, epic 1, legendary 1-2, mythic 2
  function rollAffixes(rng, rarity) {
    var count = 0;
    if (rarity === 'rare') count = rng.chance(0.35) ? 1 : 0;
    else if (rarity === 'epic') count = 1;
    else if (rarity === 'legendary') count = rng.chance(0.5) ? 2 : 1;
    else if (rarity === 'mythic') count = 2;
    if (!count) return undefined;
    var ids = ['bear', 'viper', 'power', 'warding', 'wisdom', 'flames', 'frost', 'venom', 'leech', 'fortune', 'titans'];
    var out = [];
    rng.shuffle(ids);
    for (var i = 0; i < count; i++) out.push(ids[i]);
    return out;
  }

  function rollLoot(rng, floor, luck) {
    var drops = [];
    var goldAmt = rng.int(3, 8 + floor * 2);
    drops.push({ gold: goldAmt });
    var dropChance = 0.35 + (luck || 0) * 0.05;
    if (rng.chance(dropChance)) {
      var pool = lootableItems();
      if (pool.length) {
        // weight toward consumables, tier-gate equipment
        var filtered = pool.filter(function (it) {
          var minF = it.minFloor || (it.tier ? (it.tier - 1) * 4 : 0);
          return minF <= floor + 2;
        });
        var pick = rng.pick(filtered.length ? filtered : pool);
        var rar = 'common', affixes;
        if (EQUIP_KINDS.indexOf(pick.kind) >= 0) {
          rar = rollRarity(rng, floor);
          affixes = rollAffixes(rng, rar);
        }
        drops.push({ id: pick.id, qty: 1, rarity: rar, affixes: affixes });
      }
    }
    return drops;
  }

  function chestLoot(rng, floor) {
    var drops = [{ gold: rng.int(15, 30 + floor * 4) }];
    var n = rng.int(1, 3);
    var pool = lootableItems();
    for (var i = 0; i < n && pool.length; i++) {
      var filtered = pool.filter(function (it) {
        var minF = it.minFloor || (it.tier ? (it.tier - 1) * 4 : 0);
        return minF <= floor + 4;
      });
      var pick = rng.pick(filtered.length ? filtered : pool);
      var isEquip = EQUIP_KINDS.indexOf(pick.kind) >= 0;
      var rar = isEquip ? rollRarity(rng, floor + 3) : 'common';
      drops.push({ id: pick.id, qty: 1, rarity: rar, affixes: isEquip ? rollAffixes(rng, rar) : undefined });
    }
    // crafting material chance
    var mats = (SC.DATA && SC.DATA.materials) || {};
    var matIds = Object.keys(mats);
    if (matIds.length && rng.chance(0.5)) {
      drops.push({ id: rng.pick(matIds), qty: rng.int(1, 3), rarity: 'common' });
    }
    return drops;
  }

  // ---- Skills -------------------------------------------------------------
  // Generic skill executor: interprets skill data + hand-tuned behaviors for known ids.
  // ctx: { player, monsters, map, projectiles, spawnFloatText(x,y,txt,color), summon(fn) }
  var SKILL_BEHAVIORS = {
    // id: {cost, cd, run(ctx) -> msg or null }
    default: { cost: 10, cd: 5 }
  };

  function skillInfo(skillId) {
    var def = E.lookup('skills', skillId) || {};
    return {
      id: skillId,
      name: def.name || skillId,
      cost: def.manaCost != null ? def.manaCost : 12,
      cd: def.cooldown != null ? def.cooldown : 6,
      description: def.description || ''
    };
  }

  function aoeDamage(ctx, cx, cy, radius, atkMult, effectId, effectSecs) {
    var eff = E.effective(ctx.player);
    var hits = 0;
    for (var i = 0; i < ctx.monsters.length; i++) {
      var m = ctx.monsters[i];
      if (m.hp <= 0) continue;
      if (U.dist(m.x, m.y, cx, cy) <= radius) {
        var dmg = computeDamage(eff.atk, m.def, atkMult);
        m.hp -= dmg;
        m.aiState = 'chase';
        hits++;
        if (effectId) E.addEffect(m, effectId, effectSecs || 4);
        if (ctx.spawnFloatText) ctx.spawnFloatText(m.x, m.y, '-' + dmg, '#ff9d5c');
      }
    }
    return hits;
  }

  function nearestMonsters(ctx, count, maxRange) {
    var p = ctx.player;
    var list = ctx.monsters.filter(function (m) { return m.hp > 0 && U.dist(m.x, m.y, p.x, p.y) <= (maxRange || 8); });
    list.sort(function (a, b) { return U.dist(a.x, a.y, p.x, p.y) - U.dist(b.x, b.y, p.x, p.y); });
    return list.slice(0, count);
  }

  function singleHit(ctx, m, mult, effectId, secs) {
    var eff = E.effective(ctx.player);
    var dmg = computeDamage(eff.atk, m.def, mult);
    m.hp -= dmg;
    m.aiState = 'chase';
    if (effectId) E.addEffect(m, effectId, secs || 4);
    if (ctx.spawnFloatText) ctx.spawnFloatText(m.x, m.y, '-' + dmg, '#ffd35c');
    return dmg;
  }

  function useSkill(ctx, skillId) {
    var p = ctx.player;
    var info = skillInfo(skillId);
    var now = Date.now();
    if ((p.cooldowns[skillId] || 0) > now) return { ok: false, reason: 'cooldown' };
    if (p.mp < info.cost) return { ok: false, reason: 'mana' };
    var eff = E.effective(p);
    var msg = null;
    var id = skillId.toLowerCase();

    // Categorize by name — covers the full generated skill list generically.
    if (/fire|meteor|hellfire|blast|nova|flame/.test(id) && !/frost/.test(id)) {
      var hits = aoeDamage(ctx, p.x, p.y, 3.2, 1.6, 'burning', 4);
      msg = info.name + ' scorches ' + hits + ' enemies!';
    } else if (/frost|ice|blizzard/.test(id)) {
      var h2 = aoeDamage(ctx, p.x, p.y, 3.0, 1.3, 'frozen', 3);
      msg = info.name + ' freezes ' + h2 + ' enemies!';
    } else if (/lightning|chain|thunder|storm/.test(id)) {
      var targets = nearestMonsters(ctx, 4, 7);
      if (!targets.length) return { ok: false, reason: 'notarget' };
      for (var t = 0; t < targets.length; t++) singleHit(ctx, targets[t], 1.8 - t * 0.2);
      msg = info.name + ' arcs through ' + targets.length + ' enemies!';
    } else if (/heal|holy light|rejuven|miracle|redemption/.test(id) && !/smite|strike|wrath/.test(id)) {
      var amount = Math.round(eff.maxHp * 0.35);
      p.hp = Math.min(eff.maxHp, p.hp + amount);
      if (ctx.spawnFloatText) ctx.spawnFloatText(p.x, p.y, '+' + amount, '#7dff9b');
      // holy: bonus damage to nearby undead (paladin flavor)
      ctx.monsters.forEach(function (m) {
        if (m.hp > 0 && m.undead && U.dist(m.x, m.y, p.x, p.y) <= 2.5) singleHit(ctx, m, 1.4, null);
      });
      msg = info.name + ' restores ' + amount + ' HP!';
    } else if (/smite|holy|judgment|exorcism|divine|crusader|wrath/.test(id)) {
      var tg = nearestMonsters(ctx, 1, 5)[0];
      if (!tg) return { ok: false, reason: 'notarget' };
      singleHit(ctx, tg, tg.undead ? 2.6 : 1.7);
      msg = info.name + ' strikes with holy power!';
    } else if (/multi ?shot|rapidfire|arrow|shot|snipe|marks/.test(id)) {
      var tgs = nearestMonsters(ctx, 3, 9);
      if (!tgs.length) return { ok: false, reason: 'notarget' };
      for (var a = 0; a < tgs.length; a++) {
        if (ctx.fireProjectile) ctx.fireProjectile(p, tgs[a], 1.4);
        else singleHit(ctx, tgs[a], 1.4);
      }
      msg = info.name + ' hits ' + tgs.length + ' targets!';
    } else if (/raise|skeleton|summon|army|imp|demonic|pet|call/.test(id)) {
      if (ctx.summonAlly) { ctx.summonAlly(skillId); msg = info.name + '!'; }
      else return { ok: false, reason: 'nocontext' };
    } else if (/rage|berserk|frenzy|reckless/.test(id)) {
      E.addEffect(p, 'berserk', 10);
      msg = 'You fly into a rage! (2x damage, +50% taken)';
    } else if (/shield ?wall|fortify|bone armor|barrier|magic shield|blood shield/.test(id)) {
      E.addEffect(p, 'shielded', 12);
      msg = info.name + ' hardens your defenses!';
    } else if (/haste|swift|time ?warp/.test(id)) {
      E.addEffect(p, 'haste', 10);
      msg = 'You feel lightning fast!';
    } else if (/empower|bless|battle ?cry|rally|inner|enlighten/.test(id)) {
      E.addEffect(p, 'strengthened', 12);
      msg = info.name + ' empowers you!';
    } else if (/vanish|invis|shadow ?meld|smoke/.test(id)) {
      E.addEffect(p, 'invisible', 8);
      msg = 'You melt into the shadows…';
    } else if (/poison|venom|dagger/.test(id)) {
      var pt = nearestMonsters(ctx, 1, 2.5)[0];
      if (!pt) return { ok: false, reason: 'notarget' };
      singleHit(ctx, pt, 1.5, 'poisoned', 6);
      msg = info.name + ' poisons the enemy!';
    } else if (/hex|curse|doom|weak/.test(id)) {
      var ct = nearestMonsters(ctx, 1, 6)[0];
      if (!ct) return { ok: false, reason: 'notarget' };
      singleHit(ctx, ct, 1.2, 'weakened', 8);
      msg = info.name + ' curses the enemy!';
    } else if (/blood|exsanguinate|life ?tap|soul|reap|death ?coil|drain/.test(id)) {
      var bt = nearestMonsters(ctx, 1, 5)[0];
      if (!bt) return { ok: false, reason: 'notarget' };
      var d = singleHit(ctx, bt, 1.6);
      var heal = Math.round(d * 0.5);
      p.hp = Math.min(eff.maxHp, p.hp + heal);
      if (ctx.spawnFloatText) ctx.spawnFloatText(p.x, p.y, '+' + heal, '#ff5c8a');
      msg = info.name + ' drains life!';
    } else {
      // default: heavy single-target strike
      var dt = nearestMonsters(ctx, 1, 2.5)[0];
      if (!dt) return { ok: false, reason: 'notarget' };
      singleHit(ctx, dt, 2.0);
      msg = info.name + '!';
    }

    p.mp -= info.cost;
    p.cooldowns[skillId] = now + info.cd * 1000;
    return { ok: true, msg: msg };
  }

  // ---- Consumables --------------------------------------------------------
  function useConsumable(ctx, invIndex) {
    var p = ctx.player;
    var st = p.inventory[invIndex];
    if (!st) return { ok: false };
    var def = E.itemDef(st.id);
    if (!def) return { ok: false };
    var eff = E.effective(p);
    var msg = null;
    var id = def.id;

    if (def.kind === 'food') {
      var hr = def.hungerRestore || 15;
      p.hunger = Math.min(100, p.hunger + hr);
      p.hp = Math.min(eff.maxHp, p.hp + Math.round(hr / 4));
      msg = 'You eat the ' + def.name + '. (+' + hr + ' hunger)';
    } else if (def.kind === 'potion') {
      if (/health|restore|cure ?all|ultimate/.test(id)) { p.hp = /full|ultimate|cure/.test(id) ? eff.maxHp : Math.min(eff.maxHp, p.hp + 30 + p.level * 4); msg = 'You feel restored!'; }
      if (/mana|restore|full|ultimate/.test(id)) { p.mp = /full|ultimate/.test(id) ? eff.maxMp : Math.min(eff.maxMp, p.mp + 25 + p.level * 3); if (!msg) msg = 'Mana surges back!'; }
      if (/strength|giant/.test(id)) { E.addEffect(p, 'strengthened', 30); msg = 'You feel mighty!'; }
      if (/defense/.test(id)) { E.addEffect(p, 'shielded', 30); msg = 'Your skin hardens!'; }
      if (/speed/.test(id)) { E.addEffect(p, 'haste', 20); msg = 'You feel quick!'; }
      if (/invisibility/.test(id)) { E.addEffect(p, 'invisible', 12); msg = 'You fade from sight…'; }
      if (/regen/.test(id)) { E.addEffect(p, 'regenerating', 30); msg = 'Wounds begin to close.'; }
      if (/berserk/.test(id)) { E.addEffect(p, 'berserk', 15); msg = 'RAGE!'; }
      if (/antidote/.test(id)) { E.removeEffect(p, 'poisoned'); msg = 'The poison fades.'; }
      if (/cure ?all/.test(id)) { p.effects = p.effects.filter(function (e) { return ['poisoned', 'burning', 'bleeding', 'weakened', 'confused', 'blind', 'frozen', 'stunned'].indexOf(e.id) < 0; }); msg = 'You feel cleansed!'; }
      if (/experience/.test(id)) { E.gainXp(p, E.xpForLevel(p.level) / 2 | 0); msg = 'Knowledge floods your mind!'; }
      if (/luck/.test(id)) { E.addEffect(p, 'lucky', 60); msg = 'You feel lucky!'; }
      if (!msg) { p.hp = Math.min(eff.maxHp, p.hp + 20); msg = 'You drink the ' + def.name + '.'; }
    } else if (def.kind === 'scroll') {
      if (/teleport/.test(id) && ctx.teleportRandom) { ctx.teleportRandom(); msg = 'Reality bends!'; }
      else if (/mapping/.test(id) && ctx.revealMap) { ctx.revealMap(); msg = 'The floor layout burns into your mind!'; }
      else if (/fireball|meteor/.test(id)) { aoeDamage(ctx, p.x, p.y, 3.5, 2.2, 'burning', 5); msg = 'The scroll erupts in flame!'; }
      else if (/ice|blizzard/.test(id)) { aoeDamage(ctx, p.x, p.y, 3.5, 1.8, 'frozen', 4); msg = 'A blizzard tears through!'; }
      else if (/lightning/.test(id)) { var lt = nearestMonsters(ctx, 4, 8); lt.forEach(function (m) { singleHit(ctx, m, 2.2); }); msg = 'Lightning strikes!'; }
      else if (/earthquake/.test(id)) { aoeDamage(ctx, p.x, p.y, 5, 1.6, 'stunned', 2); msg = 'The ground shakes violently!'; }
      else if (/divine|wrath|death/.test(id)) { aoeDamage(ctx, p.x, p.y, 4.5, 2.6); msg = 'Devastating power is unleashed!'; }
      else if (/heal/.test(id)) { p.hp = eff.maxHp; msg = 'Holy light restores you!'; }
      else if (/time ?stop/.test(id)) { ctx.monsters.forEach(function (m) { E.addEffect(m, 'stunned', 6); }); msg = 'Time freezes for your foes!'; }
      else { aoeDamage(ctx, p.x, p.y, 3, 1.5); msg = 'The scroll crackles with power!'; }
    } else if (def.kind === 'special' && id === 'bomb') {
      aoeDamage(ctx, p.x, p.y, 2.5, 3.0, 'burning', 3);
      msg = 'BOOM!';
    } else if (def.kind === 'special' && id === 'torch') {
      E.addEffect(p, 'torchlight', 60);
      msg = 'The torch blazes to life.';
    } else {
      return { ok: false, reason: 'notusable' };
    }
    E.removeItem(p, st.id, 1, st.rarity || 'common');
    return { ok: true, msg: msg };
  }

  return {
    computeDamage: computeDamage,
    playerAttack: playerAttack,
    monsterAttack: monsterAttack,
    rollLoot: rollLoot,
    chestLoot: chestLoot,
    rollRarity: rollRarity,
    useSkill: useSkill,
    skillInfo: skillInfo,
    useConsumable: useConsumable,
    aoeDamage: aoeDamage,
    nearestMonsters: nearestMonsters
  };
})();

'use strict';
/* ShadowCrypt Online — entities: player, monsters, companions, status effects, leveling */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.entities = (function () {
  var U = SC.util;

  function D() { return SC.DATA || {}; }
  function classById(id) { return (D().classes || []).find ? (D().classes || []).find(function (c) { return c.id === id; }) : null; }

  function lookup(collection, id) {
    var col = D()[collection];
    if (!col) return null;
    if (Array.isArray(col)) {
      for (var i = 0; i < col.length; i++) if (col[i].id === id) return col[i];
      return null;
    }
    return col[id] || null;
  }

  function speciesBonuses(speciesId, subspeciesId) {
    var all = D().species || [];
    for (var i = 0; i < all.length; i++) {
      var sp = all[i];
      if (sp.id !== speciesId) continue;
      var subs = sp.subspecies || [];
      for (var j = 0; j < subs.length; j++) {
        if (subs[j].id === subspeciesId) return subs[j].bonuses || {};
      }
      return (subs[0] && subs[0].bonuses) || {};
    }
    return {};
  }

  function xpForLevel(level) {
    return Math.floor(40 * Math.pow(level, 1.65));
  }

  // ---- Player -------------------------------------------------------------
  function createPlayer(opts) {
    var cls = lookup('classes', opts.classId) || { id: 'warrior', name: 'Warrior', hp: 50, atk: 8, def: 5, mana: 10, spd: 10 };
    var spB = speciesBonuses(opts.speciesId, opts.subspeciesId);
    var p = {
      id: U.uid(),
      name: (opts.name || 'Adventurer').slice(0, 16),
      classId: cls.id,
      speciesId: opts.speciesId || 'human',
      subspeciesId: opts.subspeciesId || null,
      subclassId: null,
      level: 1, xp: 0, gold: 50, gems: 0,
      base: {
        hp: (cls.hp || 40) + (spB.hp || 0),
        atk: (cls.atk || 5) + (spB.atk || 0),
        def: (cls.def || 3) + (spB.def || 0),
        mana: (cls.mana || 20) + (spB.mana || 0),
        spd: (cls.spd || 10) + (spB.spd || 0)
      },
      hp: 0, mp: 0, hunger: 100,
      x: 0, y: 0, floor: 1, fx: 0, fy: 0, dirX: 0, dirY: 1,
      inventory: [],
      equipment: { weapon: null, shield: null, helmet: null, armor: null, gloves: null, boots: null, ring1: null, ring2: null, amulet: null },
      effects: [],
      skillSlots: [],
      cooldowns: {},
      companions: [],
      quests: { active: [], completed: [] },
      achievements: [],
      factionRep: {},
      knownRecipes: [],
      stats: { kills: 0, bossKills: 0, deaths: 0, deepestFloor: 1, cropsHarvested: 0, itemsCrafted: 0, pvpWins: 0, questsDone: 0, chestsOpened: 0 },
      createdAt: Date.now()
    };
    var eff = effective(p);
    p.hp = eff.maxHp; p.mp = eff.maxMp;
    assignSkills(p);
    // starter gear
    addItem(p, 'health_potion', 3);
    addItem(p, 'bread', 2);
    addItem(p, 'torch', 1);
    return p;
  }

  function classSkillIds(p) {
    var ids = [];
    var cls = lookup('classes', p.classId);
    var skills = D().skills || {};
    // base class special: map to a skill whose id/name matches, else first 1-2 generic
    var baseSkillByClass = {
      warrior: ['rage', 'berserk'], mage: ['fireBlast', 'fireball'], rogue: ['deadlyStrike', 'backstab'],
      paladin: ['heal', 'holyStrike', 'holySmite'], ranger: ['multiShot', 'aimedShot'], necromancer: ['raiseSkeleton', 'deathCoil']
    };
    var cand = baseSkillByClass[p.classId] || [];
    for (var i = 0; i < cand.length; i++) {
      if (lookup('skills', cand[i])) { ids.push(cand[i]); }
    }
    if (p.subclassId) {
      var sub = lookup('subclasses', p.subclassId);
      if (sub && sub.skills) {
        for (var j = 0; j < sub.skills.length; j++) {
          if (ids.indexOf(sub.skills[j]) < 0 && lookup('skills', sub.skills[j])) ids.push(sub.skills[j]);
        }
      }
    }
    return ids;
  }

  function assignSkills(p) {
    var ids = classSkillIds(p);
    p.skillSlots = ids.slice(0, 4);
  }

  function subclassBonuses(p) {
    if (!p.subclassId) return {};
    var sub = lookup('subclasses', p.subclassId);
    return (sub && sub.bonuses) || {};
  }

  function rarityMult(rarity) {
    var rs = D().rarities || [];
    for (var i = 0; i < rs.length; i++) if (rs[i].id === rarity) return rs[i].multiplier || 1;
    return 1;
  }

  function itemDef(id) { return lookup('items', id); }

  function equipStat(p, stat) {
    var total = 0;
    var eq = p.equipment;
    for (var slot in eq) {
      var it = eq[slot];
      if (!it) continue;
      var def = itemDef(it.id);
      if (!def) continue;
      var mult = rarityMult(it.rarity || 'common');
      var v = 0;
      if (stat === 'atk') v = def.atk || 0;
      else if (stat === 'def') v = def.def || 0;
      if (def.bonuses && def.bonuses[stat]) v += def.bonuses[stat];
      total += Math.round(v * mult);
    }
    return total;
  }

  // Effective (fully derived) stats
  function effective(p) {
    var sb = subclassBonuses(p);
    var lvl = p.level - 1;
    var growth = classGrowth(p.classId);
    var e = {
      maxHp: p.base.hp + (sb.hp || 0) + lvl * growth.hp + equipStat(p, 'hp'),
      maxMp: p.base.mana + (sb.mana || 0) + lvl * growth.mana + equipStat(p, 'mana'),
      atk: p.base.atk + (sb.atk || 0) + lvl * growth.atk + equipStat(p, 'atk'),
      def: p.base.def + (sb.def || 0) + lvl * growth.def + equipStat(p, 'def'),
      spd: p.base.spd + (sb.spd || 0) + Math.floor(lvl / 4) + equipStat(p, 'spd')
    };
    // status effect modifiers
    for (var i = 0; i < p.effects.length; i++) {
      var fx = p.effects[i];
      if (fx.id === 'strengthened') e.atk = Math.round(e.atk * 1.4);
      if (fx.id === 'weakened') e.atk = Math.round(e.atk * 0.6);
      if (fx.id === 'haste') e.spd = Math.round(e.spd * 1.5);
      if (fx.id === 'shielded') e.def = Math.round(e.def * 1.5);
      if (fx.id === 'berserk') { e.atk = Math.round(e.atk * 2); }
    }
    if (p.hunger <= 0) { e.atk = Math.max(1, Math.round(e.atk * 0.7)); }
    return e;
  }

  function classGrowth(classId) {
    var table = {
      warrior: { hp: 9, atk: 2.2, def: 1.4, mana: 2 },
      mage: { hp: 5, atk: 1.2, def: 0.8, mana: 8 },
      rogue: { hp: 6, atk: 2.0, def: 1.0, mana: 3 },
      paladin: { hp: 8, atk: 1.8, def: 1.6, mana: 4 },
      ranger: { hp: 7, atk: 2.0, def: 1.0, mana: 4 },
      necromancer: { hp: 5.5, atk: 1.4, def: 0.8, mana: 7 }
    };
    var g = table[classId] || { hp: 7, atk: 1.8, def: 1.1, mana: 4 };
    return { hp: Math.round(g.hp), atk: Math.round(g.atk * 10) / 10 | 0 || 1, def: Math.max(1, Math.round(g.def)), mana: Math.round(g.mana) };
  }

  function gainXp(p, amount) {
    p.xp += amount;
    var leveled = false;
    while (p.xp >= xpForLevel(p.level)) {
      p.xp -= xpForLevel(p.level);
      p.level++;
      leveled = true;
      var eff = effective(p);
      p.hp = eff.maxHp; p.mp = eff.maxMp;
      U.emit('player:levelup', p.level);
    }
    if (leveled) assignSkills(p);
    return leveled;
  }

  // ---- Inventory ----------------------------------------------------------
  var STACKABLE = { potion: true, scroll: true, food: true, special: true, material: true, seed: true };

  function addItem(p, id, qty, rarity) {
    qty = qty || 1;
    var def = itemDef(id) || (lookup('materials', id) ? Object.assign({ kind: 'material' }, lookup('materials', id)) : null);
    var kind = def ? def.kind : 'special';
    if (STACKABLE[kind]) {
      for (var i = 0; i < p.inventory.length; i++) {
        if (p.inventory[i].id === id && (p.inventory[i].rarity || 'common') === (rarity || 'common')) {
          p.inventory[i].qty += qty;
          U.emit('inv:changed');
          return true;
        }
      }
    }
    if (p.inventory.length >= 48 && !STACKABLE[kind]) { U.emit('msg', 'Inventory full!'); return false; }
    p.inventory.push({ id: id, qty: qty, rarity: rarity || 'common' });
    U.emit('inv:changed');
    return true;
  }

  function removeItem(p, id, qty, rarity) {
    qty = qty || 1;
    for (var i = 0; i < p.inventory.length; i++) {
      var st = p.inventory[i];
      if (st.id === id && (rarity === undefined || (st.rarity || 'common') === rarity)) {
        if (st.qty > qty) { st.qty -= qty; U.emit('inv:changed'); return true; }
        if (st.qty === qty) { p.inventory.splice(i, 1); U.emit('inv:changed'); return true; }
        return false;
      }
    }
    return false;
  }

  function countItem(p, id) {
    var n = 0;
    for (var i = 0; i < p.inventory.length; i++) if (p.inventory[i].id === id) n += p.inventory[i].qty;
    return n;
  }

  function equipItem(p, invIndex) {
    var st = p.inventory[invIndex];
    if (!st) return false;
    var def = itemDef(st.id);
    if (!def) return false;
    var slot = def.slot || def.kind;
    var slotMap = { weapon: 'weapon', shield: 'shield', helmet: 'helmet', armor: 'armor', gloves: 'gloves', boots: 'boots', amulet: 'amulet' };
    var target = slotMap[slot];
    if (slot === 'ring') target = p.equipment.ring1 ? (p.equipment.ring2 ? 'ring1' : 'ring2') : 'ring1';
    if (!target) return false;
    var prev = p.equipment[target];
    p.equipment[target] = { id: st.id, rarity: st.rarity || 'common' };
    p.inventory.splice(invIndex, 1);
    if (prev) p.inventory.push({ id: prev.id, qty: 1, rarity: prev.rarity || 'common' });
    var eff = effective(p);
    p.hp = Math.min(p.hp, eff.maxHp); p.mp = Math.min(p.mp, eff.maxMp);
    U.emit('inv:changed');
    return true;
  }

  function unequip(p, slot) {
    var it = p.equipment[slot];
    if (!it) return false;
    if (p.inventory.length >= 48) { U.emit('msg', 'Inventory full!'); return false; }
    p.equipment[slot] = null;
    p.inventory.push({ id: it.id, qty: 1, rarity: it.rarity || 'common' });
    U.emit('inv:changed');
    return true;
  }

  // ---- Status effects -----------------------------------------------------
  function addEffect(ent, id, seconds, power) {
    for (var i = 0; i < ent.effects.length; i++) {
      if (ent.effects[i].id === id) {
        ent.effects[i].ttl = Math.max(ent.effects[i].ttl, seconds * 1000);
        ent.effects[i].power = Math.max(ent.effects[i].power || 1, power || 1);
        return;
      }
    }
    ent.effects.push({ id: id, ttl: seconds * 1000, power: power || 1, acc: 0 });
  }
  function hasEffect(ent, id) {
    for (var i = 0; i < ent.effects.length; i++) if (ent.effects[i].id === id) return true;
    return false;
  }
  function removeEffect(ent, id) {
    ent.effects = ent.effects.filter(function (e) { return e.id !== id; });
  }

  // returns array of {dmg, effectId} damage events applied this tick
  function tickEffects(ent, dtMs) {
    var events = [];
    for (var i = ent.effects.length - 1; i >= 0; i--) {
      var fx = ent.effects[i];
      fx.ttl -= dtMs;
      fx.acc = (fx.acc || 0) + dtMs;
      if (fx.acc >= 1000) {
        fx.acc -= 1000;
        var dot = { poisoned: 2, burning: 3, bleeding: 2 }[fx.id];
        if (dot) {
          var dmg = dot * (fx.power || 1);
          events.push({ dmg: dmg, effectId: fx.id });
        }
        if (fx.id === 'regenerating') events.push({ dmg: -(3 * (fx.power || 1)), effectId: fx.id });
      }
      if (fx.ttl <= 0) ent.effects.splice(i, 1);
    }
    return events;
  }

  // ---- Monsters -----------------------------------------------------------
  function enemyPool(tier) {
    var all = D().enemies || {};
    var out = [];
    for (var id in all) {
      var e = all[id];
      var flags = e.flags || {};
      if (flags.boss || flags.miniBoss) continue;
      if ((e.tier || 1) === tier) out.push(e);
    }
    if (!out.length) { for (var id2 in all) { var e2 = all[id2]; if (!(e2.flags && (e2.flags.boss || e2.flags.miniBoss))) out.push(e2); } }
    return out;
  }

  function bossFor(floor) {
    var bosses = D().bosses || [];
    var arr = Array.isArray(bosses) ? bosses : Object.keys(bosses).map(function (k) { return bosses[k]; });
    for (var i = 0; i < arr.length; i++) {
      if (arr[i].floor === floor && !(arr[i].miniBoss || (arr[i].flags && arr[i].flags.miniBoss))) return arr[i];
    }
    // fallback: any boss flagged in enemies
    return null;
  }

  function miniBossFor(floor) {
    var bosses = D().bosses || [];
    var arr = Array.isArray(bosses) ? bosses : Object.keys(bosses).map(function (k) { return bosses[k]; });
    var tier = Math.min(8, Math.ceil(floor / 4));
    var minis = arr.filter(function (b) { return b.miniBoss || (b.flags && b.flags.miniBoss); });
    if (!minis.length) return null;
    return minis[Math.min(minis.length - 1, tier - 1)];
  }

  var monsterSeq = 1;
  function spawnMonster(template, x, y, floor) {
    var scale = 1 + Math.max(0, floor - 1) * 0.06;
    var flags = template.flags || {};
    var m = {
      mid: 'm' + (monsterSeq++),
      id: template.id,
      name: template.name,
      glyph: template.glyph || (template.name || '?')[0].toLowerCase(),
      color: template.color || '#c0c0c0',
      x: x, y: y, fx: x, fy: y,
      hp: Math.round((template.hp || 10) * scale),
      maxHp: Math.round((template.hp || 10) * scale),
      atk: Math.round((template.atk || 3) * scale),
      def: Math.round((template.def || 1) * scale),
      spd: template.spd || 8,
      xp: Math.round((template.xp || 5) * scale),
      undead: !!flags.undead,
      ranged: !!flags.ranged,
      boss: !!flags.boss || !!template.boss,
      miniBoss: !!flags.miniBoss || !!template.miniBoss,
      effects: [],
      aiState: 'idle',
      moveCd: 0, atkCd: 0,
      lastSeenPlayer: null
    };
    if (m.boss) { m.hp = Math.round(m.hp * 1.2); m.maxHp = m.hp; }
    return m;
  }

  // ---- Companions ---------------------------------------------------------
  function createCompanion(defId) {
    var def = lookup('companions', defId);
    if (!def) return null;
    var st = def.stats || {};
    return {
      cid: U.uid(),
      id: def.id, name: def.name,
      level: 1, xp: 0, bond: 0,
      hp: st.hp || 20, maxHp: st.hp || 20,
      atk: st.atk || 4, def: st.def || 1, spd: st.spd || 10,
      x: 0, y: 0, fx: 0, fy: 0,
      abilities: def.abilities || [],
      effects: []
    };
  }

  return {
    createPlayer: createPlayer,
    effective: effective,
    xpForLevel: xpForLevel,
    gainXp: gainXp,
    assignSkills: assignSkills,
    classSkillIds: classSkillIds,
    lookup: lookup,
    itemDef: itemDef,
    rarityMult: rarityMult,
    addItem: addItem, removeItem: removeItem, countItem: countItem,
    equipItem: equipItem, unequip: unequip,
    addEffect: addEffect, hasEffect: hasEffect, removeEffect: removeEffect, tickEffects: tickEffects,
    enemyPool: enemyPool, bossFor: bossFor, miniBossFor: miniBossFor, spawnMonster: spawnMonster,
    createCompanion: createCompanion
  };
})();

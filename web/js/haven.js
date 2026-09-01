'use strict';
/* ShadowCrypt Online — Haven: base building, resource production, farming, companions, sieges */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.haven = (function () {
  var U = SC.util, E = SC.entities;

  var GRID_W = 14, GRID_H = 11;

  function bdef(type) { return (SC.DATA.buildings || {})[type]; }

  function createHaven() {
    var h = {
      grid: { w: GRID_W, h: GRID_H },
      buildings: [],
      lastSiege: Date.now(),
      blessing: null,       // {id, expiresAt}
      lastBlessing: 0,
      seq: 1
    };
    // starter keep in the middle
    h.buildings.push({
      bid: 'b0', type: 'keep', level: 1,
      x: Math.floor(GRID_W / 2) - 1, y: Math.floor(GRID_H / 2) - 1,
      lastCollect: Date.now(), stored: 0, crop: null
    });
    return h;
  }

  function keepLevel(h) {
    var k = h.buildings.find(function (b) { return b.type === 'keep'; });
    return k ? k.level : 1;
  }

  function countType(h, type) {
    return h.buildings.filter(function (b) { return b.type === type; }).length;
  }

  function occupiedCells(h, except) {
    var cells = {};
    h.buildings.forEach(function (b) {
      if (b === except) return;
      var d = bdef(b.type) || { size: 1 };
      for (var dy = 0; dy < d.size; dy++)
        for (var dx = 0; dx < d.size; dx++)
          cells[(b.x + dx) + ',' + (b.y + dy)] = b;
    });
    return cells;
  }

  function canPlace(h, type, x, y) {
    var d = bdef(type);
    if (!d) return { ok: false, msg: 'Unknown building' };
    if (d.unique && countType(h, type) >= 1) return { ok: false, msg: 'Already built' };
    if (d.maxCount && countType(h, type) >= d.maxCount) return { ok: false, msg: 'Limit reached' };
    var cells = occupiedCells(h);
    for (var dy = 0; dy < d.size; dy++) {
      for (var dx = 0; dx < d.size; dx++) {
        var cx = x + dx, cy = y + dy;
        if (cx < 0 || cy < 0 || cx >= GRID_W || cy >= GRID_H) return { ok: false, msg: 'Out of bounds' };
        if (cells[cx + ',' + cy]) return { ok: false, msg: 'Occupied' };
      }
    }
    return { ok: true };
  }

  function buildCost(type, level) {
    var d = bdef(type);
    if (!d) return { gold: 999999 };
    if (level <= 1) return { gold: (d.cost && d.cost.gold) || 0 };
    var base = (d.upgradeBase && d.upgradeBase.gold) || 100;
    return { gold: Math.round(base * Math.pow(d.costMult || 1.7, level - 2)) };
  }

  function build(p, h, type, x, y) {
    var chk = canPlace(h, type, x, y);
    if (!chk.ok) return chk;
    var d = bdef(type);
    // non-keep buildings limited by keep level: total buildings <= keep*4
    if (type !== 'keep' && h.buildings.length >= keepLevel(h) * 4 + 1) {
      return { ok: false, msg: 'Upgrade your Shadow Keep to build more (max ' + (keepLevel(h) * 4) + ')' };
    }
    var cost = buildCost(type, 1);
    if (p.gold < cost.gold) return { ok: false, msg: 'Need ' + cost.gold + ' gold' };
    p.gold -= cost.gold;
    h.buildings.push({
      bid: 'b' + (h.seq++), type: type, level: 1, x: x, y: y,
      lastCollect: Date.now(), stored: 0, crop: null
    });
    U.emit('haven:changed');
    return { ok: true, msg: d.name + ' built!' };
  }

  function upgrade(p, h, b) {
    var d = bdef(b.type);
    if (!d) return { ok: false, msg: 'Unknown building' };
    if (b.level >= d.maxLevel) return { ok: false, msg: 'Max level' };
    if (b.type !== 'keep' && b.level >= keepLevel(h)) return { ok: false, msg: 'Upgrade your Shadow Keep first' };
    var cost = buildCost(b.type, b.level + 1);
    if (p.gold < cost.gold) return { ok: false, msg: 'Need ' + cost.gold + ' gold' };
    p.gold -= cost.gold;
    collectFrom(p, h, b); // bank pending production at old rate first
    b.level++;
    U.emit('haven:changed');
    return { ok: true, msg: d.name + ' → level ' + b.level };
  }

  function demolish(p, h, b) {
    if (b.type === 'keep') return { ok: false, msg: 'The Keep cannot be demolished' };
    h.buildings = h.buildings.filter(function (x) { return x !== b; });
    p.gold += Math.floor(buildCost(b.type, b.level).gold * 0.3);
    U.emit('haven:changed');
    return { ok: true, msg: 'Demolished (30% refund)' };
  }

  // ---- Production ---------------------------------------------------------
  function productionPerHour(b) {
    var d = bdef(b.type);
    if (!d || !d.produce) return null;
    var out = {};
    for (var res in d.produce) {
      out[res] = Math.round(d.produce[res] * Math.pow(d.produceMult || 1.5, b.level - 1));
    }
    return out;
  }

  function storeCap(b) {
    var d = bdef(b.type);
    return d && d.storeCap ? Math.round(d.storeCap * Math.pow(1.4, b.level - 1)) : 0;
  }

  function pendingProduction(b, now) {
    var per = productionPerHour(b);
    if (!per) return 0;
    var hours = (now - b.lastCollect) / 3600000;
    var res = Object.keys(per)[0];
    return Math.min(storeCap(b), Math.floor(per[res] * hours));
  }

  function collectFrom(p, h, b) {
    var now = Date.now();
    var amt = pendingProduction(b, now);
    if (amt <= 0) return { ok: false, amount: 0 };
    var per = productionPerHour(b);
    var res = Object.keys(per)[0];
    b.lastCollect = now;
    if (res === 'gold') p.gold += amt;
    else if (res === 'mana') {
      // mana well: bottled as mana potions per 40 mana
      var bottles = Math.floor(amt / 40);
      if (bottles > 0) E.addItem(p, 'mana_potion', bottles);
      else p.mp = Math.min(E.effective(p).maxMp, p.mp + amt);
    }
    U.emit('haven:changed');
    return { ok: true, amount: amt, res: res };
  }

  function collectAll(p, h) {
    var total = { gold: 0, mana: 0 };
    h.buildings.forEach(function (b) {
      var r = collectFrom(p, h, b);
      if (r.ok) total[r.res] = (total[r.res] || 0) + r.amount;
    });
    return total;
  }

  // ---- Farming ------------------------------------------------------------
  function cropDef(id) { return (SC.DATA.crops || {})[id]; }

  function farmPlots(h) {
    return h.buildings.filter(function (b) { return b.type === 'farmPlot'; });
  }

  function plant(p, h, plot, cropId) {
    if (plot.type !== 'farmPlot') return { ok: false, msg: 'Not a farm plot' };
    if (plot.crop) return { ok: false, msg: 'Already planted' };
    var c = cropDef(cropId);
    if (!c) return { ok: false, msg: 'Unknown seed' };
    if (p.gold < c.cost) return { ok: false, msg: 'Need ' + c.cost + ' gold for seeds' };
    p.gold -= c.cost;
    plot.crop = { id: cropId, plantedAt: Date.now(), watered: false };
    U.emit('haven:changed');
    return { ok: true, msg: c.name + ' planted!' };
  }

  function water(plot) {
    if (!plot.crop || plot.crop.watered) return { ok: false };
    plot.crop.watered = true;
    U.emit('haven:changed');
    return { ok: true, msg: 'Watered — grows twice as fast now.' };
  }

  function cropProgress(plot, now) {
    if (!plot.crop) return 0;
    var c = cropDef(plot.crop.id);
    if (!c) return 0;
    var grow = c.growMs / (plot.crop.watered ? 2 : 1);
    // higher plot level speeds growth 10%/level
    grow = grow / (1 + (plot.level - 1) * 0.1);
    return U.clamp((now - plot.crop.plantedAt) / grow, 0, 1);
  }

  function harvest(p, h, plot) {
    if (!plot.crop) return { ok: false, msg: 'Nothing planted' };
    if (cropProgress(plot, Date.now()) < 1) return { ok: false, msg: 'Not ready yet' };
    var c = cropDef(plot.crop.id);
    var got = [];
    (c.yield || []).forEach(function (y) {
      var qty = y.qty + (plot.level - 1); // higher level plots yield more
      E.addItem(p, y.id, qty);
      var def = E.itemDef(y.id) || { name: y.id };
      got.push(qty + '× ' + def.name);
    });
    E.gainXp(p, c.xp || 5);
    p.stats.cropsHarvested++;
    plot.crop = null;
    SC.systems.questProgress(p, 'harvest', c.id, 1);
    SC.systems.checkAchievements(p);
    U.emit('haven:changed');
    return { ok: true, msg: 'Harvested ' + got.join(', ') + ' (+' + (c.xp || 5) + ' XP)' };
  }

  // ---- Companions (Companion Den) ----------------------------------------
  function recruitableCompanions(h) {
    var den = h.buildings.find(function (b) { return b.type === 'barracks'; });
    if (!den) return [];
    var all = SC.systems.asArray(SC.DATA.companions);
    // den level gates the roster
    return all.slice(0, Math.min(all.length, den.level * 3));
  }

  function recruit(p, h, compId) {
    var pool = recruitableCompanions(h);
    var def = pool.find(function (c) { return c.id === compId; });
    if (!def) return { ok: false, msg: 'Not available — upgrade your Companion Den' };
    if (p.companions.length >= 3) return { ok: false, msg: 'Max 3 companions' };
    var cost = def.cost || 200;
    if (p.gold < cost) return { ok: false, msg: 'Need ' + cost + ' gold' };
    p.gold -= cost;
    var comp = E.createCompanion(compId);
    p.companions.push(comp);
    U.emit('haven:changed');
    return { ok: true, msg: def.name + ' joins you!' };
  }

  // ---- Shrine blessing ----------------------------------------------------
  function claimBlessing(p, h) {
    var shrine = h.buildings.find(function (b) { return b.type === 'shrineB'; });
    if (!shrine) return { ok: false, msg: 'Build an Ancient Shrine first' };
    var now = Date.now();
    if (now - h.lastBlessing < 20 * 3600000) {
      return { ok: false, msg: 'Next blessing in ' + U.fmtTime(20 * 3600000 - (now - h.lastBlessing)) };
    }
    var rng = new U.Rng((now / 1000) | 0);
    var bl = rng.pick(SC.DATA.blessings || [{ id: 'bl_xp', name: 'Blessing of Wisdom' }]);
    h.blessing = { id: bl.id, expiresAt: now + 2 * 3600000 };
    h.lastBlessing = now;
    U.emit('haven:changed');
    return { ok: true, msg: bl.icon + ' ' + bl.name + ' — active for 2h!' };
  }

  function activeBlessing(h) {
    if (!h.blessing || h.blessing.expiresAt < Date.now()) return null;
    return (SC.DATA.blessings || []).find(function (b) { return b.id === h.blessing.id; }) || null;
  }

  // ---- Shadow sieges (offline event) -------------------------------------
  // Every ~8h a shadow siege tests your towers; resolved deterministically on load.
  function resolveSieges(p, h) {
    var now = Date.now();
    var events = [];
    var SIEGE_MS = 8 * 3600000;
    var guard = 0;
    while (now - h.lastSiege > SIEGE_MS && guard++ < 10) {
      h.lastSiege += SIEGE_MS;
      var towers = h.buildings.filter(function (b) { return b.type === 'tower'; });
      var walls = h.buildings.filter(function (b) { return b.type === 'wall'; });
      var defense = towers.reduce(function (s, t) { return s + t.level * 3; }, 0) +
                    walls.reduce(function (s, w) { return s + w.level; }, 0);
      var rng = new U.Rng((h.lastSiege / 1000) | 0);
      var strength = rng.int(3, 8 + keepLevel(h) * 2);
      if (defense >= strength) {
        var loot = rng.int(10, 30) * keepLevel(h);
        p.gold += loot;
        events.push('🗼 Your defenses repelled a shadow siege! Salvaged ' + loot + ' gold.');
      } else {
        var loss = Math.min(p.gold, rng.int(10, 25) * keepLevel(h));
        p.gold -= loss;
        events.push('💀 A shadow siege breached your haven — lost ' + loss + ' gold. Build towers and walls!');
      }
    }
    if (now - h.lastSiege > SIEGE_MS) h.lastSiege = now; // clamp huge offline gaps
    return events;
  }

  // Portal: deepest starting floor
  function portalDepth(h) {
    var portal = h.buildings.find(function (b) { return b.type === 'portal'; });
    return portal ? Math.min(26, 1 + portal.level * 5) : 1;
  }

  return {
    GRID_W: GRID_W, GRID_H: GRID_H,
    createHaven: createHaven, bdef: bdef, keepLevel: keepLevel,
    canPlace: canPlace, buildCost: buildCost, build: build, upgrade: upgrade, demolish: demolish,
    productionPerHour: productionPerHour, pendingProduction: pendingProduction, storeCap: storeCap,
    collectFrom: collectFrom, collectAll: collectAll,
    cropDef: cropDef, farmPlots: farmPlots, plant: plant, water: water,
    cropProgress: cropProgress, harvest: harvest,
    recruitableCompanions: recruitableCompanions, recruit: recruit,
    claimBlessing: claimBlessing, activeBlessing: activeBlessing,
    resolveSieges: resolveSieges, portalDepth: portalDepth,
    occupiedCells: occupiedCells
  };
})();

'use strict';
/* ShadowCrypt Online — meta systems: quests, achievements, crafting, factions, hunger, shops */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.systems = (function () {
  var U = SC.util, E = SC.entities;

  function D() { return SC.DATA || {}; }
  function asArray(col) {
    if (!col) return [];
    return Array.isArray(col) ? col : Object.keys(col).map(function (k) { return col[k]; });
  }

  // ---- Quests -------------------------------------------------------------
  function allQuests() { return asArray(D().quests); }

  function availableQuests(p) {
    var done = p.quests.completed;
    var activeIds = p.quests.active.map(function (q) { return q.id; });
    return allQuests().filter(function (q) {
      if (done.indexOf(q.id) >= 0 || activeIds.indexOf(q.id) >= 0) return false;
      if (q.minLevel && p.level < q.minLevel) return false;
      if (q.requires && done.indexOf(q.requires) < 0) return false;
      return true;
    });
  }

  function acceptQuest(p, questId) {
    var q = allQuests().find(function (x) { return x.id === questId; });
    if (!q) return false;
    if (p.quests.active.length >= 8) { U.emit('msg', 'Quest log full.'); return false; }
    var objectives = (q.objectives || [{ kind: 'kill', target: 'any', count: 5 }]).map(function (o) {
      return { kind: o.kind || 'kill', target: o.target || 'any', count: o.count || 1, progress: 0 };
    });
    p.quests.active.push({ id: q.id, objectives: objectives });
    U.emit('msg', 'Quest accepted: ' + q.name);
    U.emit('quests:changed');
    return true;
  }

  function questProgress(p, kind, target, amount) {
    amount = amount || 1;
    var completedAny = false;
    for (var i = 0; i < p.quests.active.length; i++) {
      var aq = p.quests.active[i];
      var qdef = allQuests().find(function (x) { return x.id === aq.id; });
      var allDone = true;
      for (var j = 0; j < aq.objectives.length; j++) {
        var o = aq.objectives[j];
        if (o.progress < o.count && o.kind === kind &&
            (o.target === 'any' || o.target === target || (kind === 'explore' && String(o.target) === String(target)))) {
          o.progress = Math.min(o.count, o.progress + amount);
        }
        if (o.progress < o.count) allDone = false;
      }
      if (allDone && qdef) {
        completeQuest(p, aq, qdef);
        i--;
        completedAny = true;
      }
    }
    if (completedAny) U.emit('quests:changed');
  }

  function completeQuest(p, activeQuest, qdef) {
    p.quests.active = p.quests.active.filter(function (q) { return q.id !== activeQuest.id; });
    p.quests.completed.push(qdef.id);
    p.stats.questsDone++;
    var r = qdef.rewards || {};
    if (r.gold) p.gold += r.gold;
    if (r.xp) E.gainXp(p, r.xp);
    if (r.items) {
      var items = Array.isArray(r.items) ? r.items : [r.items];
      items.forEach(function (it) {
        if (typeof it === 'string') E.addItem(p, it, 1);
        else if (it && it.id) E.addItem(p, it.id, it.count || it.qty || 1);
      });
    }
    if (r.reputation) {
      for (var f in r.reputation) addReputation(p, f, r.reputation[f]);
    }
    U.emit('msg', '✔ Quest complete: ' + qdef.name + (r.gold ? ' (+' + r.gold + 'g)' : ''));
    U.emit('toast', { text: 'Quest complete: ' + qdef.name, cls: 'gold' });
    checkAchievements(p);
  }

  // ---- Achievements -------------------------------------------------------
  function checkAchievements(p) {
    var defs = asArray(D().achievements);
    for (var i = 0; i < defs.length; i++) {
      var a = defs[i];
      if (p.achievements.indexOf(a.id) >= 0) continue;
      if (achievementMet(p, a)) {
        p.achievements.push(a.id);
        if (a.reward) {
          if (a.reward.gold) p.gold += a.reward.gold;
          if (a.reward.gems) p.gems = (p.gems || 0) + a.reward.gems;
          if (a.reward.xp) E.gainXp(p, a.reward.xp);
        }
        U.emit('toast', { text: '🏆 Achievement: ' + a.name, cls: 'gold' });
      }
    }
  }

  function achievementMet(p, a) {
    var c = a.condition || {};
    var s = p.stats;
    if (c.kills && s.kills < c.kills) return false;
    if (c.bossKills && s.bossKills < c.bossKills) return false;
    if (c.level && p.level < c.level) return false;
    if (c.floor && s.deepestFloor < c.floor) return false;
    if (c.gold && p.gold < c.gold) return false;
    if (c.quests && s.questsDone < c.quests) return false;
    if (c.crafted && s.itemsCrafted < c.crafted) return false;
    if (c.harvested && s.cropsHarvested < c.harvested) return false;
    if (c.pvpWins && s.pvpWins < c.pvpWins) return false;
    if (c.chests && s.chestsOpened < c.chests) return false;
    if (c.deaths && s.deaths < c.deaths) return false;
    if (c.closeCalls && (s.closeCalls || 0) < c.closeCalls) return false;
    // every condition key must be one we track (and have a real threshold) — unknown keys never auto-unlock
    var tracked = ['kills', 'bossKills', 'level', 'floor', 'gold', 'quests', 'crafted', 'harvested', 'pvpWins', 'chests', 'deaths', 'closeCalls'];
    var keys = Object.keys(c);
    return keys.length > 0 && keys.every(function (k) { return tracked.indexOf(k) >= 0 && c[k] > 0; });
  }

  // ---- Factions -----------------------------------------------------------
  function addReputation(p, factionId, amount) {
    p.factionRep[factionId] = (p.factionRep[factionId] || 0) + amount;
    var f = asArray(D().factions).find(function (x) { return x.id === factionId; });
    if (f) U.emit('msg', (amount >= 0 ? '+' : '') + amount + ' reputation with ' + f.name);
  }

  function reputationRank(p, factionId) {
    var rep = p.factionRep[factionId] || 0;
    var f = asArray(D().factions).find(function (x) { return x.id === factionId; });
    var ranks = (f && (f.ranks || f.reputationTiers)) || [
      { name: 'Stranger', min: 0 }, { name: 'Friendly', min: 100 },
      { name: 'Honored', min: 400 }, { name: 'Exalted', min: 1000 }
    ];
    var current = ranks[0];
    for (var i = 0; i < ranks.length; i++) {
      var min = ranks[i].min != null ? ranks[i].min : (ranks[i].reputation || 0);
      if (rep >= min) current = ranks[i];
    }
    return current;
  }

  // ---- Crafting -----------------------------------------------------------
  function allRecipes() { return asArray(D().recipes); }

  function canCraft(p, recipe) {
    var ings = recipe.ingredients || [];
    for (var i = 0; i < ings.length; i++) {
      var need = ings[i].count || ings[i].qty || 1;
      if (E.countItem(p, ings[i].id) < need) return false;
    }
    return true;
  }

  function craft(p, recipeId) {
    var r = allRecipes().find(function (x) { return x.id === recipeId; });
    if (!r) return { ok: false, msg: 'Unknown recipe' };
    if (!canCraft(p, r)) return { ok: false, msg: 'Missing ingredients' };
    var ings = r.ingredients || [];
    for (var i = 0; i < ings.length; i++) E.removeItem(p, ings[i].id, ings[i].count || ings[i].qty || 1);
    var resultId = r.result || r.resultId || r.output;
    var qty = r.resultCount || 1;
    E.addItem(p, resultId, qty);
    p.stats.itemsCrafted++;
    checkAchievements(p);
    var def = E.itemDef(resultId) || E.lookup('materials', resultId) || { name: resultId };
    U.emit('toast', { text: 'Crafted: ' + def.name, cls: 'gold' });
    return { ok: true, msg: 'Crafted ' + def.name };
  }

  // ---- Hunger -------------------------------------------------------------
  // Real-time hunger: depletes slowly in the crypt; starvation hurts.
  function tickHunger(p, dtMs, inCrypt) {
    if (!inCrypt) return null;
    p._hungerAcc = (p._hungerAcc || 0) + dtMs;
    if (p._hungerAcc < 4000) return null;
    p._hungerAcc -= 4000;
    p.hunger = Math.max(0, p.hunger - 1);
    if (p.hunger === 50) return 'You feel hungry.';
    if (p.hunger === 25) return 'You are starving!';
    if (p.hunger === 0) return 'You are dying of starvation!';
    if (p.hunger <= 0) { p.hp -= 2; return null; }
    return null;
  }

  // ---- Shops --------------------------------------------------------------
  function shopStock(npc, p) {
    // NPC-defined inventory if present, else generated by role
    var stock = [];
    var inv = npc && (npc.inventory || npc.services);
    if (Array.isArray(inv) && inv.length && typeof inv[0] !== 'string') {
      inv.forEach(function (s) { if (s.id) stock.push({ id: s.id, price: s.price }); });
    }
    if (!stock.length) {
      var items = D().items || {};
      var wanted;
      var role = (npc && npc.role) || 'merchant';
      if (/blacksmith|smith/.test(role)) wanted = ['weapon', 'shield', 'armor', 'helmet', 'gloves', 'boots'];
      else if (/alchemist|healer/.test(role)) wanted = ['potion'];
      else if (/mage|scribe/.test(role)) wanted = ['scroll'];
      else wanted = ['potion', 'food', 'scroll', 'special'];
      var pool = [];
      for (var id in items) {
        if (wanted.indexOf(items[id].kind) >= 0) pool.push(items[id]);
      }
      var rng = new U.Rng(U.hashStr((npc ? npc.id : 'shop') + ':' + Math.floor(Date.now() / 3600000)));
      rng.shuffle(pool);
      pool.slice(0, 8).forEach(function (it) { stock.push({ id: it.id, price: null }); });
    }
    return stock.map(function (s) {
      var def = E.itemDef(s.id) || { name: s.id, value: 10 };
      return { id: s.id, name: def.name, price: s.price || Math.max(5, (def.value || 10) * 2), def: def };
    });
  }

  function buyItem(p, stockEntry) {
    if (p.gold < stockEntry.price) return { ok: false, msg: 'Not enough gold' };
    if (!E.addItem(p, stockEntry.id, 1)) return { ok: false, msg: 'Inventory full' };
    p.gold -= stockEntry.price;
    return { ok: true, msg: 'Bought ' + stockEntry.name };
  }

  function sellItem(p, invIndex) {
    var st = p.inventory[invIndex];
    if (!st) return { ok: false };
    var def = E.itemDef(st.id) || E.lookup('materials', st.id);
    var value = Math.max(1, Math.floor(((def && def.value) || 5) * E.rarityMult(st.rarity || 'common') * 0.5));
    E.removeItem(p, st.id, 1, st.rarity || 'common');
    p.gold += value;
    return { ok: true, msg: 'Sold for ' + value + 'g' };
  }

  return {
    allQuests: allQuests, availableQuests: availableQuests, acceptQuest: acceptQuest,
    questProgress: questProgress, checkAchievements: checkAchievements,
    addReputation: addReputation, reputationRank: reputationRank,
    allRecipes: allRecipes, canCraft: canCraft, craft: craft,
    tickHunger: tickHunger,
    shopStock: shopStock, buyItem: buyItem, sellItem: sellItem,
    asArray: asArray
  };
})();

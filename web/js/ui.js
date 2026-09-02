'use strict';
/* ShadowCrypt Online — DOM UI: HUD, panels, chat, toasts, modals */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.ui = (function () {
  var U = SC.util, E = SC.entities, G = null; // G set at init to SC.game

  var $ = function (id) { return document.getElementById(id); };
  var currentPanel = null;

  var KIND_ICONS = {
    weapon: '🗡️', shield: '🛡️', armor: '🥋', helmet: '🪖', gloves: '🧤', boots: '🥾',
    ring: '💍', amulet: '📿', potion: '🧪', scroll: '📜', food: '🍖', special: '✨', material: '🪨', seed: '🌰'
  };

  function itemIcon(id) {
    var def = E.itemDef(id) || E.lookup('materials', id);
    if (!def) return '❓';
    return KIND_ICONS[def.kind] || '✨';
  }

  // Real sprite icon element for an item id (falls back to emoji span)
  function itemIconEl(id, size) {
    if (SC.assets && SC.assets.isReady()) {
      var key = SC.assets.itemKey(id);
      if (key) {
        var c = SC.assets.iconCanvas(key, size || 30);
        if (c) return c;
      }
    }
    var span = document.createElement('span');
    span.textContent = itemIcon(id);
    span.style.fontSize = ((size || 30) * 0.8) + 'px';
    return span;
  }

  function skillCategory(skillId) {
    var id = skillId.toLowerCase();
    if (/fire|meteor|hellfire|blast|flame/.test(id) && !/frost/.test(id)) return 'fire';
    if (/frost|ice|blizzard/.test(id)) return 'ice';
    if (/lightning|chain|thunder|storm/.test(id)) return 'bolt';
    if (/heal|holy light|rejuven|miracle|redemption/.test(id) && !/smite|strike|wrath/.test(id)) return 'heal';
    if (/smite|holy|judgment|exorcism|divine|crusader|wrath/.test(id)) return 'holy';
    if (/shot|arrow|snipe|marks|rapidfire/.test(id)) return 'shot';
    if (/raise|skeleton|summon|army|imp|demonic|pet|call/.test(id)) return 'summon';
    if (/rage|berserk|frenzy|reckless/.test(id)) return 'rage';
    if (/shield|fortify|bone armor|barrier/.test(id)) return 'shield';
    if (/haste|swift|time/.test(id)) return 'haste';
    if (/empower|bless|cry|rally|inner|enlighten/.test(id)) return 'buff';
    if (/vanish|invis|meld|smoke/.test(id)) return 'invis';
    if (/poison|venom/.test(id)) return 'poison';
    if (/hex|curse|doom|weak/.test(id)) return 'curse';
    if (/blood|exsanguinate|tap|soul|reap|coil|drain/.test(id)) return 'blood';
    return 'strike';
  }

  // ------------------------------------------------------------------ HUD
  var lastHudAt = 0;
  function updateHud(force) {
    var p = G.state.player;
    if (!p) return;
    // called every frame from the game loop — throttle the DOM churn
    var now = Date.now();
    if (!force && now - lastHudAt < 200) return;
    lastHudAt = now;
    var eff = E.effective(p);
    setBar('bar-hp', 'txt-hp', p.hp, eff.maxHp);
    setBar('bar-mp', 'txt-mp', p.mp, eff.maxMp);
    setBar('bar-xp', 'txt-xp', p.xp, E.xpForLevel(p.level), ' ');
    $('hud-floor').textContent = 'Lv ' + p.level + ' · ' + (G.state.mode === 'crypt' ? ('Floor ' + p.floor) : modeTitle());
    $('hud-gold').textContent = '💰 ' + U.fmt(p.gold);
    $('hud-hunger').textContent = '🍗 ' + Math.round(p.hunger) + '%';
    $('hud-hunger').style.color = p.hunger > 50 ? 'var(--green)' : (p.hunger > 25 ? 'var(--gold)' : 'var(--hp)');
    // paper-doll portrait: your actual hero with equipped gear
    var portrait = $('hud-portrait');
    if (SC.assets && SC.assets.isReady() && SC.render.spriteHero) {
      var heroC = SC.render.spriteHero(p.classId, 0, p);
      var eqSig = Object.keys(p.equipment).map(function (k) { var e = p.equipment[k]; return e ? e.id : '-'; }).join(',');
      if (portrait._sig !== p.classId + eqSig) {
        portrait._sig = p.classId + eqSig;
        portrait.innerHTML = '';
        var pc = document.createElement('canvas');
        pc.width = 40; pc.height = 40;
        var pg = pc.getContext('2d');
        pg.imageSmoothingEnabled = false;
        pg.drawImage(heroC, 0, 0, 40, 40);
        portrait.appendChild(pc);
      }
    } else {
      portrait.textContent = { warrior: '⚔️', mage: '🔮', rogue: '🗡️', paladin: '✝️', ranger: '🏹', necromancer: '💀' }[p.classId] || '🎭';
    }

    // effects
    var fxHost = $('hud-effects');
    fxHost.innerHTML = '';
    p.effects.forEach(function (fx) {
      var chip = document.createElement('div');
      chip.className = 'fx-chip';
      var def = (SC.DATA.statusEffects || []).find(function (s) { return s.id === fx.id; });
      chip.textContent = (def ? def.name : fx.id) + ' ' + Math.ceil(fx.ttl / 1000) + 's';
      chip.style.borderColor = def && def.color ? def.color : 'var(--line)';
      fxHost.appendChild(chip);
    });

    // skill buttons
    for (var i = 0; i < 4; i++) updateSkillBtn(i);
  }

  function modeTitle() {
    return { haven: '🏰 Haven', farm: '🌾 Farm', arena: '💥 Arena', siege: '⚔ SIEGE' }[G.state.mode] || '';
  }

  function setBar(barId, txtId, cur, max, labelOverride) {
    var el = $(barId), txt = $(txtId);
    if (!el) return;
    var frac = max > 0 ? U.clamp(cur / max, 0, 1) : 0;
    el.style.width = (frac * 100) + '%';
    if (txt) txt.textContent = labelOverride || (Math.max(0, Math.round(cur)) + '/' + Math.round(max));
  }

  function updateSkillBtn(i) {
    var p = G.state.player;
    var btn = $('act-skill' + (i + 1));
    if (!btn) return;
    var skillId = p.skillSlots[i];
    var old = btn.querySelector('.cd'); if (old) old.remove();
    var oldC = btn.querySelector('.cost'); if (oldC) oldC.remove();
    if (!skillId) { btn.style.opacity = 0.3; btn.textContent = String(i + 1); return; }
    btn.style.opacity = 1;
    var info = SC.combat.skillInfo(skillId);
    var iconKey = SC.assets && SC.assets.isReady() && SC.assets.skillIconKey(skillCategory(skillId));
    btn.textContent = '';
    if (iconKey) {
      var ic = SC.assets.iconCanvas(iconKey, 34);
      if (ic) { ic.style.borderRadius = '50%'; btn.appendChild(ic); }
      else btn.textContent = skillEmoji(skillId);
    } else btn.textContent = skillEmoji(skillId);
    btn.title = info.name;
    var cost = document.createElement('span');
    cost.className = 'cost'; cost.textContent = info.cost;
    btn.appendChild(cost);
    var until = p.cooldowns[skillId] || 0;
    var remain = until - Date.now();
    if (remain > 0) {
      var cd = document.createElement('span');
      cd.className = 'cd';
      cd.textContent = Math.ceil(remain / 1000);
      btn.appendChild(cd);
    }
  }

  function skillEmoji(skillId) {
    var id = skillId.toLowerCase();
    if (/fire|meteor|hellfire|blast|flame/.test(id) && !/frost/.test(id)) return '🔥';
    if (/frost|ice|blizzard/.test(id)) return '❄️';
    if (/lightning|chain|thunder|storm/.test(id)) return '⚡';
    if (/heal|holy light|rejuven|miracle|redemption/.test(id) && !/smite|strike|wrath/.test(id)) return '💚';
    if (/smite|holy|judgment|exorcism|divine|crusader|wrath/.test(id)) return '✨';
    if (/shot|arrow|snipe|marks|rapidfire/.test(id)) return '🏹';
    if (/raise|skeleton|summon|army|imp|demonic|pet|call/.test(id)) return '💀';
    if (/rage|berserk|frenzy|reckless/.test(id)) return '😡';
    if (/shield|fortify|bone armor|barrier/.test(id)) return '🛡️';
    if (/haste|swift|time/.test(id)) return '💨';
    if (/empower|bless|cry|rally|inner|enlighten/.test(id)) return '⬆️';
    if (/vanish|invis|meld|smoke/.test(id)) return '👻';
    if (/poison|venom/.test(id)) return '☠️';
    if (/hex|curse|doom|weak/.test(id)) return '🧿';
    if (/blood|exsanguinate|tap|soul|reap|coil|drain/.test(id)) return '🩸';
    return '✦';
  }

  // ------------------------------------------------------------ messages
  function pushMsg(text) {
    var host = $('hud-msg');
    var line = document.createElement('div');
    line.className = 'msg-line';
    line.textContent = text;
    host.insertBefore(line, host.firstChild);
    while (host.children.length > 4) host.removeChild(host.lastChild);
    setTimeout(function () { line.style.opacity = '0'; }, 4200);
    setTimeout(function () { if (line.parentNode) line.parentNode.removeChild(line); }, 5400);
  }

  function toast(opts) {
    var host = $('toast-host');
    var t = document.createElement('div');
    t.className = 'toast' + (opts.cls ? ' ' + opts.cls : '');
    t.textContent = opts.text;
    host.appendChild(t);
    setTimeout(function () { t.style.opacity = '0'; t.style.transition = 'opacity .5s'; }, 2600);
    setTimeout(function () { if (t.parentNode) t.parentNode.removeChild(t); }, 3200);
  }

  function modal(title, bodyHtml, actions) {
    var host = $('modal-host'), card = $('modal-card');
    card.innerHTML = '<h3>' + U.esc(title) + '</h3><div class="modal-body">' + bodyHtml + '</div><div class="modal-actions"></div>';
    var act = card.querySelector('.modal-actions');
    (actions || [{ label: 'OK' }]).forEach(function (a) {
      var b = document.createElement('button');
      b.className = 'btn' + (a.primary ? ' btn-primary' : '') + (a.danger ? ' btn-danger' : '');
      b.textContent = a.label;
      b.onclick = function () {
        host.classList.add('hidden');
        if (a.onClick) a.onClick();
      };
      act.appendChild(b);
    });
    host.classList.remove('hidden');
  }

  // ------------------------------------------------------------- panels
  var PANELS = {
    inventory: { title: '🎒 Bag', render: renderInventory },
    equipment: { title: '🛡 Gear', render: renderEquipment },
    character: { title: '📊 Hero', render: renderCharacter },
    skills: { title: '✨ Skills', render: renderSkills },
    quests: { title: '📜 Quests', render: renderQuests },
    crafting: { title: '⚒ Craft', render: renderCrafting },
    companions: { title: '🐺 Allies', render: renderCompanions },
    achievements: { title: '🏆 Feats', render: renderAchievements },
    ranks: { title: '👑 Ranks', render: renderRanks },
    factions: { title: '🏛 Factions', render: renderFactions },
    build: { title: '🏗 Build', render: renderBuild },
    shop: { title: '🛒 Shop', render: renderShop },
    settings: { title: '⚙ Menu', render: renderSettings },
    help: { title: '❓ Help', render: renderHelp }
  };

  var TAB_SETS = {
    menu: ['character', 'skills', 'quests', 'achievements', 'ranks', 'factions', 'settings', 'help'],
    bag: ['inventory', 'equipment', 'crafting'],
    haven: ['build', 'companions', 'shop']
  };

  function openPanel(name) {
    var host = $('panel-host');
    var def = PANELS[name];
    if (!def) return;
    currentPanel = name;
    // tabs: find the tab set containing this panel
    var tabs = null;
    for (var setName in TAB_SETS) {
      if (TAB_SETS[setName].indexOf(name) >= 0) { tabs = TAB_SETS[setName]; break; }
    }
    var tabHost = $('panel-tabs');
    tabHost.innerHTML = '';
    (tabs || [name]).forEach(function (tn) {
      var b = document.createElement('button');
      b.className = 'ptab' + (tn === name ? ' active' : '');
      b.textContent = PANELS[tn].title;
      b.onclick = function () { openPanel(tn); };
      tabHost.appendChild(b);
    });
    var body = $('panel-body');
    body.innerHTML = '';
    def.render(body);
    host.classList.remove('hidden');
  }

  function closePanel() {
    $('panel-host').classList.add('hidden');
    currentPanel = null;
  }

  function refreshPanel() {
    if (currentPanel && !$('panel-host').classList.contains('hidden')) openPanel(currentPanel);
  }

  // ---- Inventory
  function renderInventory(body) {
    var p = G.state.player;
    var grid = document.createElement('div');
    grid.className = 'inv-grid';
    if (!p.inventory.length) body.innerHTML = '<p style="color:var(--dim)">Your bag is empty. Loot the crypt!</p>';
    p.inventory.forEach(function (stk, idx) {
      var def = E.itemDef(stk.id) || E.lookup('materials', stk.id) || { name: stk.id, kind: 'special' };
      var slot = document.createElement('div');
      slot.className = 'inv-slot rar-' + (stk.rarity || 'common');
      var ico = document.createElement('div');
      ico.className = 'ico';
      ico.appendChild(itemIconEl(stk.id, 34));
      slot.appendChild(ico);
      var nm = document.createElement('div');
      nm.className = 'nm';
      nm.textContent = def.name;
      slot.appendChild(nm);
      if (stk.qty > 1) {
        var q = document.createElement('div');
        q.className = 'qty';
        q.textContent = stk.qty;
        slot.appendChild(q);
      }
      slot.onclick = function () { itemActions(idx, stk, def); };
      grid.appendChild(slot);
    });
    body.appendChild(grid);
  }

  function itemActions(idx, stk, def) {
    var rar = (SC.DATA.rarities || []).find(function (r) { return r.id === (stk.rarity || 'common'); });
    var statLine = [];
    var mult = E.rarityMult(stk.rarity || 'common');
    if (def.atk) statLine.push('ATK ' + Math.round(def.atk * mult));
    if (def.def) statLine.push('DEF ' + Math.round(def.def * mult));
    if (def.bonuses) for (var b in def.bonuses) statLine.push(b.toUpperCase() + ' +' + Math.round(def.bonuses[b] * mult));
    if (def.hungerRestore) statLine.push('Hunger +' + def.hungerRestore);
    var affixHtml = '';
    if (stk.affixes && stk.affixes.length) {
      affixHtml = '<br>' + stk.affixes.map(function (aid) {
        var a = E.affixDef(aid);
        if (!a) return '';
        var descr = [];
        ['hp', 'atk', 'def', 'mana', 'spd'].forEach(function (s2) { if (a[s2]) descr.push('+' + a[s2] + ' ' + s2.toUpperCase()); });
        if (a.proc) descr.push(Math.round((a.procChance || 0.15) * 100) + '% ' + a.proc + ' on hit');
        if (a.lifesteal) descr.push(Math.round(a.lifesteal * 100) + '% lifesteal');
        if (a.goldFind) descr.push('+' + Math.round(a.goldFind * 100) + '% gold');
        return '<span style="color:#c9a4ff">✨ ' + U.esc(a.name) + '</span> <span style="color:var(--dim);font-size:11px">(' + descr.join(', ') + ')</span>';
      }).join('<br>');
    }
    var html = (rar && rar.id !== 'common' ? '<span class="pill" style="border-color:' + rar.color + ';color:' + rar.color + '">' + rar.name + '</span> ' : '') +
      U.esc(def.description || '') +
      (statLine.length ? '<br><b>' + statLine.join(' · ') + '</b>' : '') +
      affixHtml +
      '<br><span style="color:var(--dim)">Value: ' + Math.floor((def.value || 5) * mult) + 'g</span>';
    var equipKinds = ['weapon', 'shield', 'armor', 'helmet', 'gloves', 'boots', 'ring', 'amulet'];
    var actions = [];
    if (equipKinds.indexOf(def.kind) >= 0) actions.push({ label: 'Equip', primary: true, onClick: function () { G.useInventoryItem(idx); refreshPanel(); } });
    else if (['potion', 'scroll', 'food', 'special'].indexOf(def.kind) >= 0) actions.push({ label: 'Use', primary: true, onClick: function () { G.useInventoryItem(idx); refreshPanel(); updateHud(); } });
    actions.push({
      label: 'Sell', onClick: function () {
        var r = SC.systems.sellItem(G.state.player, idx);
        if (r.ok) pushMsg(r.msg);
        refreshPanel(); updateHud();
      }
    });
    actions.push({ label: 'Close' });
    modal(def.name, html, actions);
  }

  // ---- Equipment
  function renderEquipment(body) {
    var p = G.state.player;
    var eff = E.effective(p);
    var slots = [['weapon', 'Weapon'], ['shield', 'Off-hand'], ['helmet', 'Head'], ['armor', 'Body'], ['gloves', 'Hands'], ['boots', 'Feet'], ['ring1', 'Ring 1'], ['ring2', 'Ring 2'], ['amulet', 'Neck']];
    var html = '<table class="stat-table"><tr><td>⚔ Attack</td><td>' + eff.atk + '</td></tr>' +
      '<tr><td>🛡 Defense</td><td>' + eff.def + '</td></tr>' +
      '<tr><td>💨 Speed</td><td>' + eff.spd + '</td></tr>' +
      '<tr><td>❤️ Max HP</td><td>' + eff.maxHp + '</td></tr>' +
      '<tr><td>🔮 Max MP</td><td>' + eff.maxMp + '</td></tr></table><br>';
    body.innerHTML = html;
    slots.forEach(function (s) {
      var row = document.createElement('div');
      row.className = 'equip-row';
      var it = p.equipment[s[0]];
      var def = it ? E.itemDef(it.id) : null;
      row.innerHTML = '<span class="slot-name">' + s[1] + '</span>';
      var mid = document.createElement('span');
      mid.style.cssText = 'flex:1;display:flex;align-items:center;gap:8px';
      if (def) {
        mid.appendChild(itemIconEl(it.id, 28));
        var nm2 = document.createElement('span');
        nm2.innerHTML = U.esc(def.name) + ' <span class="pill" style="color:' + SC.render.rarityColor(it.rarity) + '">' + (it.rarity || 'common') + '</span>';
        mid.appendChild(nm2);
      } else {
        mid.innerHTML = '<span style="color:var(--dim)">— empty —</span>';
      }
      row.appendChild(mid);
      if (def) {
        var btn = document.createElement('button');
        btn.className = 'btn'; btn.style.padding = '4px 10px'; btn.textContent = 'Unequip';
        btn.onclick = function () { E.unequip(p, s[0]); refreshPanel(); updateHud(); };
        row.appendChild(btn);
      }
      body.appendChild(row);
    });
  }

  // ---- Character
  function renderCharacter(body) {
    var p = G.state.player;
    var cls = E.lookup('classes', p.classId) || { name: p.classId };
    var sub = p.subclassId ? E.lookup('subclasses', p.subclassId) : null;
    var sp = (SC.DATA.species || []).find(function (s) { return s.id === p.speciesId; });
    var subsp = sp && (sp.subspecies || []).find(function (s) { return s.id === p.subspeciesId; });
    var s = p.stats;
    body.innerHTML =
      '<div class="list-card"><div class="lc-title">' + U.esc(p.name) + '</div>' +
      '<div class="lc-sub">Level ' + p.level + ' ' + (subsp ? U.esc(subsp.name) + ' ' : '') + U.esc(sub ? sub.name : cls.name) + '</div>' +
      '<div class="lc-meta">XP: ' + p.xp + ' / ' + E.xpForLevel(p.level) + ' · Gold: ' + U.fmt(p.gold) + '</div></div>' +
      '<table class="stat-table">' +
      '<tr><td>Kills</td><td>' + s.kills + '</td></tr>' +
      '<tr><td>Boss kills</td><td>' + s.bossKills + '</td></tr>' +
      '<tr><td>Deepest floor</td><td>' + s.deepestFloor + '</td></tr>' +
      '<tr><td>Quests done</td><td>' + s.questsDone + '</td></tr>' +
      '<tr><td>Chests opened</td><td>' + s.chestsOpened + '</td></tr>' +
      '<tr><td>Crops harvested</td><td>' + s.cropsHarvested + '</td></tr>' +
      '<tr><td>Items crafted</td><td>' + s.itemsCrafted + '</td></tr>' +
      '<tr><td>Arena wins</td><td>' + s.pvpWins + '</td></tr>' +
      '<tr><td>Sieges won</td><td>' + (s.siegesWon || 0) + '</td></tr>' +
      '<tr><td>Deaths</td><td>' + s.deaths + '</td></tr></table>';
    // talent allocation
    p.talents = p.talents || { atk: 0, hp: 0, mana: 0, spd: 0, def: 0 };
    var tp = p.talentPoints || 0;
    var tHead = document.createElement('h3');
    tHead.style.color = 'var(--gold)';
    tHead.textContent = '💪 Talents' + (tp > 0 ? ' — ' + tp + ' points to spend!' : '');
    body.appendChild(tHead);
    [['atk', '⚔ Might', '+1 ATK'], ['hp', '❤️ Vitality', '+4 HP'], ['def', '🛡 Bulwark', '+1 DEF'],
     ['mana', '🔮 Spirit', '+3 MP'], ['spd', '💨 Agility', '+1 SPD /2pts']].forEach(function (row) {
      var div = document.createElement('div');
      div.className = 'equip-row';
      div.innerHTML = '<span class="slot-name">' + row[1] + '</span><span style="flex:1;color:var(--dim);font-size:12px">' + row[2] + ' · rank <b style="color:var(--gold)">' + (p.talents[row[0]] || 0) + '</b></span>';
      if (tp > 0) {
        var btn = document.createElement('button');
        btn.className = 'btn btn-primary';
        btn.style.padding = '4px 14px';
        btn.textContent = '+';
        btn.onclick = function () {
          if ((p.talentPoints || 0) <= 0) return;
          p.talents[row[0]] = (p.talents[row[0]] || 0) + 1;
          p.talentPoints--;
          U.emit('sfx', 'ui');
          refreshPanel(); updateHud(true);
        };
        div.appendChild(btn);
      }
      body.appendChild(div);
    });
    // subclass advancement
    if (!p.subclassId && p.level >= 10) {
      var head = document.createElement('h3');
      head.textContent = '⭐ Choose your Subclass!';
      head.style.color = 'var(--gold)';
      body.appendChild(head);
      var cls2 = E.lookup('classes', p.classId);
      var subs = (SC.DATA.subclasses || []).filter(function (sc2) {
        return !sc2.isAdvanced && (!cls2 || !cls2.subclasses || cls2.subclasses.indexOf(sc2.id) >= 0);
      });
      subs.slice(0, 12).forEach(function (sc2) {
        var card = document.createElement('div');
        card.className = 'list-card';
        var b = sc2.bonuses || {};
        card.innerHTML = '<div class="lc-title">' + U.esc(sc2.name) + '</div><div class="lc-sub">' + U.esc(sc2.description || '') + '</div>' +
          '<div class="lc-meta">HP+' + (b.hp || 0) + ' ATK+' + (b.atk || 0) + ' DEF+' + (b.def || 0) + ' MP+' + (b.mana || 0) + ' SPD+' + (b.spd || 0) + '</div>';
        var btn = document.createElement('button');
        btn.className = 'btn btn-primary'; btn.textContent = 'Choose';
        btn.onclick = function () {
          p.subclassId = sc2.id;
          E.assignSkills(p);
          var eff = E.effective(p);
          p.hp = eff.maxHp; p.mp = eff.maxMp;
          toast({ text: 'You are now a ' + sc2.name + '!', cls: 'gold' });
          refreshPanel(); updateHud();
          G.save();
        };
        var act = document.createElement('div');
        act.className = 'lc-actions';
        act.appendChild(btn);
        card.appendChild(act);
        body.appendChild(card);
      });
    }
  }

  // ---- Skills
  function renderSkills(body) {
    var p = G.state.player;
    var known = E.classSkillIds(p);
    if (!known.length) { body.innerHTML = '<p style="color:var(--dim)">No skills yet. Reach level 10 and choose a subclass to unlock more.</p>'; return; }
    body.innerHTML = '<p style="color:var(--dim);font-size:12px">Tap a skill to place it in your 4 action slots.</p>';
    known.forEach(function (id) {
      var info = SC.combat.skillInfo(id);
      var slotIdx = p.skillSlots.indexOf(id);
      var card = document.createElement('div');
      card.className = 'list-card';
      card.innerHTML = '<div class="lc-title">' + U.esc(info.name) + (slotIdx >= 0 ? ' <span class="pill">slot ' + (slotIdx + 1) + '</span>' : '') + '</div>' +
        '<div class="lc-sub">' + U.esc(info.description || '') + '</div>' +
        '<div class="lc-meta">💧 ' + info.cost + ' mana · ⏱ ' + info.cd + 's cooldown</div>';
      var act = document.createElement('div');
      act.className = 'lc-actions';
      for (var i = 0; i < 4; i++) {
        (function (n) {
          var b = document.createElement('button');
          b.className = 'btn'; b.textContent = 'Slot ' + (n + 1);
          b.onclick = function () {
            var old = p.skillSlots.indexOf(id);
            if (old >= 0) p.skillSlots[old] = null;
            p.skillSlots[n] = id;
            refreshPanel(); updateHud();
          };
          act.appendChild(b);
        })(i);
      }
      card.appendChild(act);
      body.appendChild(card);
    });
  }

  // ---- Quests
  function renderQuests(body) {
    var p = G.state.player;
    body.innerHTML = '<h3 style="color:var(--gold);margin:4px 0">Active</h3>';
    if (!p.quests.active.length) body.innerHTML += '<p style="color:var(--dim)">No active quests. Accept some below!</p>';
    p.quests.active.forEach(function (aq) {
      var qdef = SC.systems.allQuests().find(function (q) { return q.id === aq.id; }) || { name: aq.id };
      var card = document.createElement('div');
      card.className = 'list-card';
      var objHtml = aq.objectives.map(function (o) {
        var done = o.progress >= o.count;
        return '<div style="color:' + (done ? 'var(--green)' : '#aab8d8') + '">' + (done ? '✔' : '▫') + ' ' +
          U.esc(objLabel(o)) + ' — ' + o.progress + '/' + o.count + '</div>';
      }).join('');
      card.innerHTML = '<div class="lc-title">' + U.esc(qdef.name) + '</div><div class="lc-sub">' + U.esc(qdef.description || '') + '</div>' + objHtml +
        rewardLine(qdef.rewards);
      body.appendChild(card);
    });
    var avail = SC.systems.availableQuests(p).slice(0, 10);
    var h = document.createElement('h3');
    h.textContent = 'Available'; h.style.color = 'var(--gold)';
    body.appendChild(h);
    if (!avail.length) body.innerHTML += '<p style="color:var(--dim)">Nothing new right now — level up or finish current quests.</p>';
    avail.forEach(function (q) {
      var card = document.createElement('div');
      card.className = 'list-card';
      card.innerHTML = '<div class="lc-title">' + U.esc(q.name) + '</div><div class="lc-sub">' + U.esc(q.description || '') + '</div>' + rewardLine(q.rewards);
      var btn = document.createElement('button');
      btn.className = 'btn btn-primary'; btn.textContent = 'Accept';
      btn.onclick = function () { SC.systems.acceptQuest(p, q.id); refreshPanel(); };
      var act = document.createElement('div');
      act.className = 'lc-actions'; act.appendChild(btn);
      card.appendChild(act);
      body.appendChild(card);
    });
    body.innerHTML += '<p style="color:var(--dim);font-size:11px;margin-top:10px">' + p.quests.completed.length + ' quests completed</p>';
  }

  function objLabel(o) {
    var target = o.target === 'any' ? '' : String(o.target).replace(/_/g, ' ');
    if (o.kind === 'kill') return 'Slay ' + o.count + ' ' + (target || 'monsters');
    if (o.kind === 'boss') return 'Defeat ' + (target || 'the boss');
    if (o.kind === 'collect') return 'Collect ' + o.count + ' ' + (target || 'items');
    if (o.kind === 'explore') return 'Reach floor ' + o.target;
    if (o.kind === 'harvest') return 'Harvest ' + o.count + ' ' + (target || 'crops');
    if (o.kind === 'craft') return 'Craft ' + o.count + ' ' + (target || 'items');
    return o.kind + ' ' + o.count;
  }

  function rewardLine(r) {
    if (!r) return '';
    var parts = [];
    if (r.gold) parts.push('💰 ' + r.gold);
    if (r.xp) parts.push('✨ ' + r.xp + ' XP');
    if (r.items) (Array.isArray(r.items) ? r.items : [r.items]).forEach(function (it) {
      var id = typeof it === 'string' ? it : it.id;
      var def = E.itemDef(id) || { name: id };
      parts.push('🎁 ' + def.name);
    });
    return parts.length ? '<div class="lc-meta">Reward: ' + U.esc(parts.join(' · ')) + '</div>' : '';
  }

  // ---- Crafting
  function renderCrafting(body) {
    var p = G.state.player;
    var recipes = SC.systems.allRecipes();
    if (!recipes.length) { body.innerHTML = '<p style="color:var(--dim)">No recipes known.</p>'; return; }
    var inHaven = G.state.mode === 'haven' || G.state.mode === 'farm';
    var hasForge = (G.state.haven.buildings || []).some(function (b) { return b.type === 'forge'; });
    var hasApoth = (G.state.haven.buildings || []).some(function (b) { return b.type === 'apothecary'; });
    body.innerHTML = inHaven ? '' : '<p style="color:var(--dim);font-size:12px">⚠ Weapon/armor and potion recipes need your haven Forge / Apothecary.</p>';
    recipes.forEach(function (r) {
      var ok = SC.systems.canCraft(p, r);
      var stationOk = true;
      if (r.station === 'forge') stationOk = inHaven && hasForge;
      if (r.station === 'apothecary') stationOk = inHaven && hasApoth;
      var resultDef = E.itemDef(r.result || r.resultId || r.output) || E.lookup('materials', r.result) || { name: r.name || r.id };
      var card = document.createElement('div');
      card.className = 'list-card' + (ok && stationOk ? '' : ' done');
      var ing = (r.ingredients || []).map(function (i2) {
        var d = E.itemDef(i2.id) || E.lookup('materials', i2.id) || { name: i2.id };
        var have = E.countItem(p, i2.id);
        var need = i2.count || i2.qty || 1;
        return '<span style="color:' + (have >= need ? 'var(--green)' : 'var(--hp)') + '">' + U.esc(d.name) + ' ' + have + '/' + need + '</span>';
      }).join(' · ');
      card.innerHTML = '<div class="lc-title">' + U.esc(resultDef.name) + (r.station ? ' <span class="pill">' + r.station + '</span>' : '') + '</div>' +
        '<div class="lc-sub">' + ing + '</div>';
      var ct2 = card.querySelector('.lc-title');
      var cic = itemIconEl(r.result || r.resultId || r.output || '', 26);
      cic.style.cssText = 'vertical-align:middle;margin-right:6px';
      ct2.insertBefore(cic, ct2.firstChild);
      if (ok && stationOk) {
        var btn = document.createElement('button');
        btn.className = 'btn btn-primary'; btn.textContent = 'Craft';
        btn.onclick = function () {
          var res = SC.systems.craft(p, r.id);
          pushMsg(res.msg);
          refreshPanel(); updateHud();
        };
        var act = document.createElement('div');
        act.className = 'lc-actions'; act.appendChild(btn);
        card.appendChild(act);
      }
      body.appendChild(card);
    });
  }

  // ---- Companions
  function renderCompanions(body) {
    var p = G.state.player;
    body.innerHTML = '<h3 style="color:var(--gold);margin:4px 0">Your party (' + p.companions.length + '/3)</h3>';
    p.companions.forEach(function (c, idx) {
      var card = document.createElement('div');
      card.className = 'list-card';
      card.innerHTML = '<div class="lc-title">🐾 ' + U.esc(c.name) + ' <span class="pill">Lv ' + c.level + '</span></div>' +
        '<div class="lc-meta">HP ' + c.hp + '/' + c.maxHp + ' · ATK ' + c.atk + ' · DEF ' + c.def + '</div>';
      var btn = document.createElement('button');
      btn.className = 'btn btn-danger'; btn.textContent = 'Release';
      btn.onclick = function () {
        modal('Release ' + c.name + '?', 'They will return to the wild.', [
          { label: 'Release', danger: true, onClick: function () { p.companions.splice(idx, 1); refreshPanel(); } },
          { label: 'Keep' }
        ]);
      };
      var act = document.createElement('div');
      act.className = 'lc-actions'; act.appendChild(btn);
      card.appendChild(act);
      body.appendChild(card);
    });
    var pool = SC.haven.recruitableCompanions(G.state.haven);
    var h = document.createElement('h3');
    h.textContent = 'Recruit (Companion Den)'; h.style.color = 'var(--gold)';
    body.appendChild(h);
    if (!pool.length) body.innerHTML += '<p style="color:var(--dim)">Build a Companion Den in your haven to recruit allies.</p>';
    pool.forEach(function (cdef) {
      if (p.companions.some(function (c) { return c.id === cdef.id; })) return;
      var stz = cdef.stats || {};
      var card = document.createElement('div');
      card.className = 'list-card';
      card.innerHTML = '<div class="lc-title">🐾 ' + U.esc(cdef.name) + '</div>' +
        '<div class="lc-sub">' + U.esc(cdef.description || '') + '</div>' +
        '<div class="lc-meta">HP ' + (stz.hp || '?') + ' · ATK ' + (stz.atk || '?') + ' · 💰 ' + (cdef.cost || 200) + 'g</div>';
      var btn = document.createElement('button');
      btn.className = 'btn btn-primary'; btn.textContent = 'Recruit';
      btn.onclick = function () {
        var res = SC.haven.recruit(p, G.state.haven, cdef.id);
        pushMsg(res.msg);
        refreshPanel(); updateHud();
      };
      var act = document.createElement('div');
      act.className = 'lc-actions'; act.appendChild(btn);
      card.appendChild(act);
      body.appendChild(card);
    });
  }

  // ---- Achievements / factions
  function renderAchievements(body) {
    var p = G.state.player;
    var defs = SC.systems.asArray(SC.DATA.achievements);
    body.innerHTML = '<p style="color:var(--dim);font-size:12px">' + p.achievements.length + ' / ' + defs.length + ' unlocked</p>';
    defs.forEach(function (a) {
      var got = p.achievements.indexOf(a.id) >= 0;
      var card = document.createElement('div');
      card.className = 'list-card' + (got ? '' : ' done');
      card.innerHTML = '<div class="lc-title">' + (got ? '🏆' : '🔒') + ' ' + U.esc(a.name) + '</div>' +
        '<div class="lc-sub">' + U.esc(a.description || '') + '</div>';
      body.appendChild(card);
    });
  }

  function renderRanks(body) {
    if (!SC.net || !SC.net.isConnected()) {
      body.innerHTML = '<p style="color:var(--dim)">⚫ Leaderboards need an online connection.</p>';
      return;
    }
    body.innerHTML = '<p style="color:var(--dim)">Fetching the deepest delvers…</p>';
    var handler = function (top) {
      U.off('net:leaderboard', handler);
      if (currentPanel !== 'ranks') return;
      body.innerHTML = '<p style="color:var(--dim);font-size:12px">Ranked by deepest floor reached.</p>';
      if (!top.length) body.innerHTML += '<p style="color:var(--dim)">No heroes ranked yet — be the first!</p>';
      var me = G.state.player;
      top.forEach(function (r, i) {
        var isMe = r.name === me.name && r.level === me.level;
        var card = document.createElement('div');
        card.className = 'list-card';
        if (isMe) card.style.borderColor = 'var(--gold)';
        var medal = i === 0 ? '🥇' : i === 1 ? '🥈' : i === 2 ? '🥉' : (i + 1) + '.';
        card.innerHTML = '<div class="lc-title">' + medal + ' ' + U.esc(r.name) +
          ' <span class="pill">Lv ' + r.level + ' ' + U.esc(r.classId || '') + '</span></div>' +
          '<div class="lc-meta">🕳 Floor ' + r.floor + ' · ⚔ ' + r.kills + ' kills · ☠ ' + r.bossKills + ' bosses · 💥 ' + r.pvpWins + ' PvP wins</div>';
        body.appendChild(card);
      });
    };
    U.on('net:leaderboard', handler);
    SC.net.requestLeaderboard();
  }

  function renderFactions(body) {
    var p = G.state.player;
    var defs = SC.systems.asArray(SC.DATA.factions);
    if (!defs.length) { body.innerHTML = '<p style="color:var(--dim)">No factions discovered yet.</p>'; return; }
    defs.forEach(function (f) {
      var rep = p.factionRep[f.id] || 0;
      var rank = SC.systems.reputationRank(p, f.id);
      var card = document.createElement('div');
      card.className = 'list-card';
      card.innerHTML = '<div class="lc-title">' + U.esc(f.name) + ' <span class="pill">' + U.esc(rank.name || '') + '</span></div>' +
        '<div class="lc-sub">' + U.esc(f.description || '') + '</div>' +
        '<div class="lc-meta">Reputation: ' + rep + '</div>';
      body.appendChild(card);
    });
  }

  // ---- Build (haven)
  function renderBuild(body) {
    var p = G.state.player, h = G.state.haven;
    var res = document.createElement('div');
    res.className = 'res-row';
    res.innerHTML = '<span>💰 <b>' + U.fmt(p.gold) + '</b></span><span>🏰 Keep Lv <b>' + SC.haven.keepLevel(h) + '</b></span>' +
      '<span>🏗 <b>' + h.buildings.length + '</b>/' + (SC.haven.keepLevel(h) * 4 + 1) + '</span>';
    body.appendChild(res);
    var collectBtn = document.createElement('button');
    collectBtn.className = 'btn btn-primary';
    collectBtn.textContent = '⛏ Collect all resources';
    collectBtn.onclick = function () {
      var got = SC.haven.collectAll(p, h);
      pushMsg('Collected: ' + (got.gold ? got.gold + ' gold ' : '') + (got.mana ? got.mana + ' mana' : '') || 'nothing yet');
      refreshPanel(); updateHud();
    };
    body.appendChild(collectBtn);
    var blessBtn = document.createElement('button');
    blessBtn.className = 'btn';
    blessBtn.textContent = '⛩ Claim daily blessing';
    blessBtn.onclick = function () {
      var r = SC.haven.claimBlessing(p, h);
      pushMsg(r.msg);
      refreshPanel();
    };
    body.appendChild(blessBtn);
    // playable siege defense
    if (SC.siege) {
      var siegeBtn = document.createElement('button');
      var ready = SC.siege.canStart(h);
      siegeBtn.className = 'btn' + (ready ? ' btn-danger' : '');
      siegeBtn.textContent = ready ? '⚔ REPEL SIEGE — defend for loot!' : '⚔ Next siege in ' + U.fmtTime(SC.siege.nextIn(h));
      siegeBtn.disabled = !ready;
      siegeBtn.onclick = function () {
        closePanel();
        G.switchMode('siege');
        pushMsg('Defend the Keep! Move + attack like the crypt. Towers and walls fight with you.');
      };
      body.appendChild(siegeBtn);
    }
    var hint = document.createElement('p');
    hint.style.cssText = 'color:var(--dim);font-size:12px';
    hint.textContent = 'Tap a building on the haven grid to select it (upgrade/collect). Choose one below to place a new one.';
    body.appendChild(hint);
    var bar = document.createElement('div');
    bar.className = 'build-bar';
    var defs = SC.DATA.buildings || {};
    for (var id in defs) {
      (function (d) {
        var cost = SC.haven.buildCost(d.id, 1);
        var card = document.createElement('div');
        card.className = 'list-card';
        card.style.marginBottom = '0';
        card.innerHTML = '<div class="lc-title">' + d.icon + ' ' + U.esc(d.name) + '</div>' +
          '<div class="lc-sub">' + U.esc(d.desc || '') + '</div>' +
          '<div class="lc-meta">💰 ' + cost.gold + '</div>';
        var btn = document.createElement('button');
        btn.className = 'btn btn-primary'; btn.textContent = 'Place';
        btn.onclick = function () {
          G.state.buildPlacing = { type: d.id, x: 3, y: 3 };
          closePanel();
          G.switchMode('haven');
          pushMsg('Tap the grid to position, then ✔ to confirm.');
          showPlacementConfirm();
        };
        var act = document.createElement('div');
        act.className = 'lc-actions'; act.appendChild(btn);
        card.appendChild(act);
        bar.appendChild(card);
      })(defs[id]);
    }
    body.appendChild(bar);
  }

  function showPlacementConfirm() {
    var host = $('toast-host');
    var bar = document.createElement('div');
    bar.className = 'toast';
    bar.style.pointerEvents = 'auto';
    var ok = document.createElement('button');
    ok.className = 'btn btn-primary'; ok.textContent = '✔ Build here';
    ok.onclick = function () {
      var bp = G.state.buildPlacing;
      if (!bp) return cleanup();
      var res = SC.haven.build(G.state.player, G.state.haven, bp.type, bp.x, bp.y);
      pushMsg(res.msg || '');
      if (res.ok) { G.state.buildPlacing = null; cleanup(); updateHud(); }
    };
    var cancel = document.createElement('button');
    cancel.className = 'btn'; cancel.textContent = '✕';
    cancel.onclick = function () { G.state.buildPlacing = null; cleanup(); };
    bar.appendChild(ok); bar.appendChild(cancel);
    host.appendChild(bar);
    function cleanup() { if (bar.parentNode) bar.parentNode.removeChild(bar); }
  }

  function buildingSelected(b) {
    if (!b) return;
    var p = G.state.player, h = G.state.haven;
    var d = SC.haven.bdef(b.type) || { name: b.type };
    var pend = SC.haven.pendingProduction(b, Date.now());
    var upCost = SC.haven.buildCost(b.type, b.level + 1);
    var html = U.esc(d.desc || '') + '<br><b>Level ' + b.level + '/' + (d.maxLevel || 1) + '</b>';
    var prod = SC.haven.productionPerHour(b);
    if (prod) {
      var rk = Object.keys(prod)[0];
      html += '<br>Produces ' + prod[rk] + ' ' + rk + '/hour (stored: ' + pend + ')';
    }
    var actions = [];
    if (pend > 0) actions.push({
      label: '⛏ Collect ' + pend, primary: true, onClick: function () {
        SC.haven.collectFrom(p, h, b); updateHud();
      }
    });
    if (b.type === 'farmPlot') {
      if (!b.crop) actions.push({ label: '🌱 Plant', primary: true, onClick: function () { plantMenu(b); } });
      else {
        var prog = SC.haven.cropProgress(b, Date.now());
        if (prog >= 1) actions.push({ label: '🌾 Harvest', primary: true, onClick: function () { var r = SC.haven.harvest(p, h, b); pushMsg(r.msg); updateHud(); } });
        else if (!b.crop.watered) actions.push({ label: '💧 Water', primary: true, onClick: function () { var r = SC.haven.water(b); if (r.msg) pushMsg(r.msg); } });
      }
    }
    if (b.level < (d.maxLevel || 1)) actions.push({
      label: '⬆ Upgrade (' + upCost.gold + 'g)', onClick: function () {
        var r = SC.haven.upgrade(p, h, b);
        pushMsg(r.msg); updateHud();
      }
    });
    if (b.type !== 'keep') actions.push({ label: 'Demolish', danger: true, onClick: function () { var r = SC.haven.demolish(p, h, b); pushMsg(r.msg); } });
    actions.push({ label: 'Close' });
    modal(d.icon + ' ' + d.name, html, actions);
  }

  function plantMenu(plot) {
    var p = G.state.player;
    var crops = SC.DATA.crops || {};
    var html = '<div class="build-bar">';
    modal('🌱 Plant seeds', 'Pick a crop:', [{ label: 'Cancel' }]);
    var bodyEl = document.querySelector('#modal-card .modal-body');
    bodyEl.innerHTML = '';
    for (var id in crops) {
      (function (c) {
        var card = document.createElement('div');
        card.className = 'list-card';
        card.innerHTML = '<div class="lc-title">' + c.icon + ' ' + U.esc(c.name) + '</div>' +
          '<div class="lc-sub">' + U.esc(c.desc || '') + '</div>' +
          '<div class="lc-meta">💰 ' + c.cost + ' · ⏱ ' + U.fmtTime(c.growMs) + '</div>';
        var btn = document.createElement('button');
        btn.className = 'btn btn-primary'; btn.textContent = 'Plant';
        btn.onclick = function () {
          var r = SC.haven.plant(p, G.state.haven, plot, c.id);
          pushMsg(r.msg);
          $('modal-host').classList.add('hidden');
          updateHud();
        };
        var act = document.createElement('div');
        act.className = 'lc-actions'; act.appendChild(btn);
        card.appendChild(act);
        bodyEl.appendChild(card);
      })(crops[id]);
    }
  }

  // ---- Shop
  function renderShop(body) {
    var p = G.state.player;
    var npcs = SC.systems.asArray(SC.DATA.npcs);
    var npc = npcs.length ? npcs[Math.floor(Date.now() / 3600000) % npcs.length] : null;
    body.innerHTML = npc ? '<div class="list-card"><div class="lc-title">🧙 ' + U.esc(npc.name) + '</div><div class="lc-sub">' +
      U.esc((npc.dialogue && npc.dialogue[0]) || 'What are you buying?') + '</div></div>' : '';
    var stock = SC.systems.shopStock(npc, p);
    stock.forEach(function (s2) {
      var card = document.createElement('div');
      card.className = 'list-card';
      card.innerHTML = '<div class="lc-title">' + U.esc(s2.name) + '</div>' +
        '<div class="lc-sub">' + U.esc(s2.def.description || '') + '</div>' +
        '<div class="lc-meta">💰 ' + s2.price + '</div>';
      var st2 = card.querySelector('.lc-title');
      var sic = itemIconEl(s2.id, 26);
      sic.style.cssText = 'vertical-align:middle;margin-right:6px';
      st2.insertBefore(sic, st2.firstChild);
      var btn = document.createElement('button');
      btn.className = 'btn btn-primary'; btn.textContent = 'Buy';
      btn.onclick = function () {
        var r = SC.systems.buyItem(p, s2);
        pushMsg(r.msg);
        updateHud();
      };
      var act = document.createElement('div');
      act.className = 'lc-actions'; act.appendChild(btn);
      card.appendChild(act);
      body.appendChild(card);
    });
    var sellHint = document.createElement('p');
    sellHint.style.cssText = 'color:var(--dim);font-size:12px';
    sellHint.textContent = 'To sell, open your Bag and tap an item.';
    body.appendChild(sellHint);
  }

  // ---- Settings & help
  function renderSettings(body) {
    var online = SC.net && SC.net.isConnected();
    body.innerHTML = '<div class="list-card"><div class="lc-title">' + (online ? '🟢 Online' : '⚫ Offline') + '</div>' +
      '<div class="lc-sub">' + (online ? SC.net.onlineCount() + ' adventurers online. Chat, co-op ghosts, arena PvP, leaderboards and cloud saves active.' :
        'Playing offline — everything works locally. Connect to a server for multiplayer.') + '</div></div>';
    // audio / fx toggles
    var au = SC.audio ? SC.audio.settings() : { sfx: true, music: true };
    var fx = SC.render.fxSettings();
    [['🔊 Sound effects', au.sfx, function (on) { SC.audio.setSfx(on); }],
     ['🎵 Music', au.music, function (on) { SC.audio.setMusic(on); }],
     ['📳 Screen shake', fx.shake, function (on) { SC.render.setShakeEnabled(on); }]].forEach(function (row) {
      var div = document.createElement('div');
      div.className = 'equip-row';
      div.innerHTML = '<span style="flex:1">' + row[0] + '</span>';
      var btn = document.createElement('button');
      btn.className = 'btn' + (row[1] ? ' btn-primary' : '');
      btn.textContent = row[1] ? 'ON' : 'OFF';
      btn.onclick = function () { row[2](!row[1]); U.emit('sfx', 'ui'); refreshPanel(); };
      div.appendChild(btn);
      body.appendChild(div);
    });
    // camera mode
    var camDiv = document.createElement('div');
    camDiv.className = 'equip-row';
    var camNames = { top: '🗺 Top-down', tpp: '🎮 Third-person', fpp: '👁 First-person' };
    camDiv.innerHTML = '<span style="flex:1">🎥 Crypt camera: <b style="color:var(--gold)">' + camNames[G.state.camera] + '</b></span>';
    var camBtn = document.createElement('button');
    camBtn.className = 'btn';
    camBtn.textContent = 'Switch (V)';
    camBtn.onclick = function () { G.cycleCamera(); refreshPanel(); };
    camDiv.appendChild(camBtn);
    body.appendChild(camDiv);
    var saveBtn = document.createElement('button');
    saveBtn.className = 'btn btn-primary'; saveBtn.textContent = '💾 Save now';
    saveBtn.onclick = function () { G.save(); toast({ text: 'Saved!', cls: 'gold' }); };
    body.appendChild(saveBtn);
    var abandonBtn = document.createElement('button');
    abandonBtn.className = 'btn btn-danger'; abandonBtn.textContent = '☠ Abandon character';
    abandonBtn.onclick = function () {
      modal('Abandon character?', 'This permanently deletes ' + U.esc(G.state.player.name) + ' and their haven. There is no undo.', [
        { label: 'Delete forever', danger: true, onClick: function () { G.wipeSave(); location.reload(); } },
        { label: 'Cancel', primary: true }
      ]);
    };
    body.appendChild(abandonBtn);
  }

  function renderHelp(body) {
    body.innerHTML =
      '<div class="list-card"><div class="lc-title">🗡 Crypt</div><div class="lc-sub">Move: WASD/arrows/joystick, or tap a tile. Attack: Space/⚔️ (hold to auto-attack — melee sweeps a 120° arc, crits knock back). Dash with i-frames: Shift/💨. Skills: 1-4. Interact: G/F/✋. Smash urns and crates for loot; mini-bosses drop 🗝️ Keys for golden chests. Elite monsters glow — extra loot. Descend all 30 floors and slay the Demon King.</div></div>' +
      '<div class="list-card"><div class="lc-title">🎥 Camera — Top-down / TPP / FPP</div><div class="lc-sub">Press V or 🎥 to cycle views: classic top-down, third-person behind your hero, or full first-person. In TPP/FPP: joystick-up walks forward, drag the RIGHT side of the screen (or mouse-drag / Q & E) to look around — PUBG style. Tap the screen to attack.</div></div>' +
      '<div class="list-card"><div class="lc-title">🏰 Haven</div><div class="lc-sub">Your base. Place buildings, collect gold from mines over time (even while away), upgrade the Keep to unlock more. Towers and walls defend against shadow sieges every 8 hours.</div></div>' +
      '<div class="list-card"><div class="lc-title">🌾 Farm</div><div class="lc-sub">Build farm plots, plant seeds, water them (2× speed), harvest food and potion ingredients. Crops grow in real time — even while you are offline.</div></div>' +
      '<div class="list-card"><div class="lc-title">💥 Arena</div><div class="lc-sub">Real-time PvP! Move with joystick/WASD, hold ⚔ to shoot, ✦ drops a bomb. Grab power-ups. Online: fight other players; offline: training bots.</div></div>' +
      '<div class="list-card"><div class="lc-title">📱 Install</div><div class="lc-sub">This game is a PWA — use your browser\'s "Add to Home Screen" / "Install" to play it like a native app, fully offline.</div></div>' +
      '<div class="list-card"><div class="lc-title">⌨ Hotkeys</div><div class="lc-sub">I bag · Tab gear · J quests · K hero · M map · Enter chat · Esc close · &gt; descend · &lt; ascend</div></div>';
  }

  // ------------------------------------------------------------ chat
  function initChat() {
    var drawer = $('chat-drawer');
    $('btn-chat-toggle').onclick = function () { drawer.classList.toggle('hidden'); };
    function sendChat() {
      var inp = $('chat-input');
      var text = inp.value.trim();
      if (!text) return;
      inp.value = '';
      SC.net.chat(text);
      addChatLine({ name: G.state.player ? G.state.player.name : 'You', text: text, self: true });
    }
    $('chat-send').onclick = sendChat;
    $('chat-input').addEventListener('keydown', function (e) {
      if (e.key === 'Enter') sendChat();
      e.stopPropagation();
    });
    U.on('chat', function (m) {
      if (m.system) addChatLine({ sys: true, text: m.text });
      else addChatLine({ name: m.name, text: m.text });
    });
    U.on('ui:chat-focus', function () {
      drawer.classList.remove('hidden');
      $('chat-input').focus();
    });
  }

  function addChatLine(m) {
    var log = $('chat-log');
    var line = document.createElement('div');
    if (m.sys) line.innerHTML = '<span class="c-sys">' + U.esc(m.text) + '</span>';
    else line.innerHTML = '<span class="c-name">' + U.esc(m.name || '?') + ':</span> ' + U.esc(m.text);
    log.appendChild(line);
    while (log.children.length > 120) log.removeChild(log.firstChild);
    log.scrollTop = log.scrollHeight;
  }

  // ------------------------------------------------------------ nav
  function initNav() {
    var btns = document.querySelectorAll('.nav-btn');
    btns.forEach(function (b) {
      b.addEventListener('click', function () {
        var mode = b.getAttribute('data-mode');
        if (mode === 'menu') { openPanel('character'); return; }
        btns.forEach(function (x) { x.classList.remove('active'); });
        b.classList.add('active');
        if (mode === 'crypt') {
          if (G.state.mode !== 'crypt') {
            // start/resume a descent from haven
            if (G.state.map && G.state.player.hp > 0) G.switchMode('crypt');
            else G.startDescent(1);
          }
        } else {
          G.switchMode(mode);
        }
        if (mode === 'haven') openPanel('build');
        if (mode === 'farm') pushMsg('Tap your farm plots to plant, water and harvest. Build more in 🏰 Haven → Build.');
        updateHud();
      });
    });
  }

  function setNavActive(mode) {
    document.querySelectorAll('.nav-btn').forEach(function (b) {
      b.classList.toggle('active', b.getAttribute('data-mode') === mode);
    });
  }

  // ------------------------------------------------------------ init
  function init() {
    G = SC.game;
    $('panel-close').onclick = closePanel;
    $('panel-host').addEventListener('click', function (e) { if (e.target === $('panel-host')) closePanel(); });
    $('modal-host').addEventListener('click', function (e) { if (e.target === $('modal-host')) $('modal-host').classList.add('hidden'); });
    initNav();
    initChat();

    U.on('msg', pushMsg);
    U.on('toast', toast);
    U.on('hud:update', updateHud);
    U.on('ui:panel', openPanel);
    U.on('ui:escape', function () {
      if (!$('modal-host').classList.contains('hidden')) $('modal-host').classList.add('hidden');
      else if (currentPanel) closePanel();
      else openPanel('settings');
    });
    U.on('haven:selected', buildingSelected);
    U.on('mode:changed', function (mode) { setNavActive(mode === 'farm' ? 'farm' : mode); updateHud(); });
    U.on('player:levelup', function () { updateHud(); });
    U.on('quests:changed', function () { if (currentPanel === 'quests') refreshPanel(); });
    U.on('inv:changed', function () { if (currentPanel === 'inventory' || currentPanel === 'equipment') refreshPanel(); });
    U.on('net:up', function (info) {
      $('hud-net').className = 'net-on';
      addChatLine({ sys: true, text: 'Connected — ' + info.online + ' adventurers online.' });
    });
    U.on('net:down', function () {
      $('hud-net').className = 'net-off';
      addChatLine({ sys: true, text: 'Connection lost — playing offline.' });
    });
    U.on('net:online', function (n) { /* count shown in settings */ });
    U.on('arena:kill', function (name) { toast({ text: '💥 Eliminated ' + name + '!', cls: 'gold' }); });
    U.on('arena:death', function (name) { toast({ text: '☠ Fragged by ' + name, cls: 'bad' }); });
    U.on('arena:powerup', function (def) { pushMsg('Picked up ' + def.icon + ' ' + def.id + '!'); });
    U.on('arena:over', function (results) {
      var me = results.find(function (r) { return r.me; });
      var won = results[0] && results[0].me;
      if (won && G.state.player) { G.state.player.stats.pvpWins++; SC.systems.checkAchievements(G.state.player); }
      var html = results.map(function (r, i) {
        return '<div style="color:' + (r.me ? 'var(--gold)' : 'var(--text)') + '">' + (i + 1) + '. ' + U.esc(r.name) + ' — ' + r.score + ' kills</div>';
      }).join('');
      modal(won ? '🏆 VICTORY!' : '💥 Match over', html, [
        { label: 'Rematch', primary: true, onClick: function () { G.switchMode('haven'); G.switchMode('arena'); } },
        { label: 'Back to Haven', onClick: function () { G.switchMode('haven'); setNavActive('haven'); } }
      ]);
    });
    U.on('siege:over', function (res) {
      if (res.won) {
        modal('🏆 SIEGE REPELLED!', 'Your haven stands! ' + res.kills + ' shadow creatures destroyed.<br><br>' + res.loot.map(U.esc).join('<br>'), [
          { label: 'Glory!', primary: true, onClick: function () { G.switchMode('haven'); setNavActive('haven'); } }
        ]);
      } else {
        modal('💀 The Keep has fallen…', 'The shadows looted ' + res.loss + ' gold. Build more towers and bone walls, then try again.<br>You slew ' + res.kills + ' attackers.', [
          { label: 'Rebuild', onClick: function () { G.switchMode('haven'); setNavActive('haven'); } }
        ]);
      }
    });
    U.on('player:levelup', function (lvl) {
      U.emit('sfx', 'levelup');
      if (G.state.player && (G.state.player.talentPoints || 0) > 0) {
        toast({ text: '💪 ' + G.state.player.talentPoints + ' talent points — open Menu ▸ Hero', cls: 'gold' });
      }
    });
    U.on('camera:changed', function () { updateHud(true); });
    U.on('assets:ready', function () {
      updateHud(true);
      refreshPanel();
      pushMsg('✨ High-definition art loaded.');
    });
    U.on('game:won', function () {
      modal('👑 THE DEMON KING IS SLAIN!',
        'You have conquered all 30 floors of ShadowCrypt. The realm is free… for now.<br><br>+5000 gold, +10000 XP.<br>The crypt reshuffles for your next descent — try a deeper portal, a new class, or the arena!',
        [{ label: 'Glorious!', primary: true }]);
    });
    U.on('floor:entered', function () { updateHud(); });
  }

  return { init: init, openPanel: openPanel, closePanel: closePanel, updateHud: updateHud, pushMsg: pushMsg, toast: toast, modal: modal };
})();

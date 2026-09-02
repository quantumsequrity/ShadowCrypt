'use strict';
/* ShadowCrypt Online — boot: title screen, character creation, service worker */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

(function () {
  var U = SC.util, E = SC.entities;
  var $ = function (id) { return document.getElementById(id); };

  var chosen = { speciesId: null, subspeciesId: null, classId: null };

  function bootUI() {
    var hasSave = SC.game.hasSave();
    $('btn-continue').classList.toggle('hidden', !hasSave);
    $('btn-continue').onclick = function () {
      if (SC.game.loadGame()) startPlaying();
      else { SC.ui.toast({ text: 'Save was corrupt — starting fresh.', cls: 'bad' }); showCreate(); }
    };
    $('btn-newgame').onclick = function () {
      if (hasSave) {
        SC.ui.modal('Start over?', 'You already have a hero. Starting a new adventure deletes them and their haven.', [
          { label: 'Delete & restart', danger: true, onClick: function () { SC.game.wipeSave(); showCreate(); } },
          { label: 'Cancel', primary: true }
        ]);
      } else showCreate();
    };
    $('boot-status').textContent = (navigator.onLine ? 'online' : 'offline') + ' · works offline · v1.0';
  }

  function showCreate() {
    $('boot-screen').classList.add('hidden');
    $('create-screen').classList.remove('hidden');
    renderSpecies();
    renderClasses();
    updateSummary();
    $('btn-create-back').onclick = function () {
      $('create-screen').classList.add('hidden');
      $('boot-screen').classList.remove('hidden');
    };
    $('btn-create-go').onclick = function () {
      var name = $('hero-name').value.trim() || 'Adventurer';
      if (!chosen.classId) { SC.ui.toast({ text: 'Pick a class first!', cls: 'bad' }); return; }
      $('create-screen').classList.add('hidden');
      SC.game.newGame({
        name: name,
        classId: chosen.classId,
        speciesId: chosen.speciesId || 'human',
        subspeciesId: chosen.subspeciesId
      });
      startPlaying();
    };
  }

  function renderSpecies() {
    var host = $('species-list');
    host.innerHTML = '';
    var all = SC.DATA.species || [];
    if (!all.length) {
      chosen.speciesId = 'human';
      host.innerHTML = '<p style="color:var(--dim)">Human</p>';
      return;
    }
    all.forEach(function (sp) {
      var card = document.createElement('div');
      card.className = 'pick-card' + (chosen.speciesId === sp.id ? ' sel' : '');
      card.innerHTML = '<div class="pc-name">' + U.esc(sp.name) + '</div><div class="pc-desc">' + U.esc((sp.description || '').slice(0, 90)) + '</div>';
      card.onclick = function () {
        chosen.speciesId = sp.id;
        chosen.subspeciesId = (sp.subspecies && sp.subspecies[0]) ? sp.subspecies[0].id : null;
        renderSpecies();
        renderSubspecies(sp);
        updateSummary();
      };
      host.appendChild(card);
    });
    if (!chosen.speciesId && all.length) {
      chosen.speciesId = all[0].id;
      chosen.subspeciesId = (all[0].subspecies && all[0].subspecies[0]) ? all[0].subspecies[0].id : null;
      renderSpecies();
      renderSubspecies(all[0]);
    } else {
      var cur = all.find(function (s) { return s.id === chosen.speciesId; });
      if (cur) renderSubspecies(cur);
    }
  }

  function renderSubspecies(sp) {
    var host = $('subspecies-list');
    host.innerHTML = '';
    (sp.subspecies || []).forEach(function (ss) {
      var b = ss.bonuses || {};
      var card = document.createElement('div');
      card.className = 'pick-card' + (chosen.subspeciesId === ss.id ? ' sel' : '');
      card.innerHTML = '<div class="pc-name">' + U.esc(ss.name) + '</div>' +
        '<div class="pc-stats">HP' + sign(b.hp) + ' ATK' + sign(b.atk) + ' DEF' + sign(b.def) + ' SPD' + sign(b.spd) + ' MP' + sign(b.mana) + '</div>';
      card.onclick = function () {
        chosen.subspeciesId = ss.id;
        renderSubspecies(sp);
        updateSummary();
      };
      host.appendChild(card);
    });
  }

  function sign(v) { v = v || 0; return (v >= 0 ? '+' : '') + v; }

  function renderClasses() {
    var host = $('class-list');
    host.innerHTML = '';
    (SC.DATA.classes || []).forEach(function (c) {
      var card = document.createElement('div');
      card.className = 'pick-card' + (chosen.classId === c.id ? ' sel' : '');
      card.innerHTML = '<div class="pc-name">' + classIcon(c.id) + ' ' + U.esc(c.name) + '</div>' +
        '<div class="pc-desc">' + U.esc(c.specialAbility || '') + '</div>' +
        '<div class="pc-stats">HP ' + c.hp + ' · ATK ' + c.atk + ' · DEF ' + c.def + ' · MP ' + c.mana + ' · SPD ' + c.spd + '</div>';
      card.onclick = function () {
        chosen.classId = c.id;
        renderClasses();
        updateSummary();
      };
      host.appendChild(card);
    });
  }

  function classIcon(id) {
    return { warrior: '⚔️', mage: '🔮', rogue: '🗡️', paladin: '✝️', ranger: '🏹', necromancer: '💀' }[id] || '🎭';
  }

  function updateSummary() {
    var el = $('create-summary');
    var cls = (SC.DATA.classes || []).find(function (c) { return c.id === chosen.classId; });
    var sp = (SC.DATA.species || []).find(function (s) { return s.id === chosen.speciesId; });
    var ss = sp && (sp.subspecies || []).find(function (x) { return x.id === chosen.subspeciesId; });
    if (!cls) { el.innerHTML = 'Pick a species and class. Every choice changes your stats, skills and playstyle — 6 classes, 37 subclasses at level 10, and deeper evolutions await.'; return; }
    var b = (ss && ss.bonuses) || {};
    el.innerHTML = '<b>' + U.esc((ss ? ss.name + ' ' : '') + cls.name) + '</b> — ' +
      'HP <b>' + (cls.hp + (b.hp || 0)) + '</b> · ATK <b>' + (cls.atk + (b.atk || 0)) + '</b> · DEF <b>' + (cls.def + (b.def || 0)) + '</b> · ' +
      'MP <b>' + (cls.mana + (b.mana || 0)) + '</b> · SPD <b>' + (cls.spd + (b.spd || 0)) + '</b><br>' +
      U.esc(cls.specialAbility || '');
  }

  function startPlaying() {
    $('boot-screen').classList.add('hidden');
    $('create-screen').classList.add('hidden');
    $('hud').classList.remove('hidden');
    SC.ui.updateHud(true);
    SC.audio.setMood(SC.game.state.mode === 'crypt' ? 'crypt' : SC.game.state.mode);
    if (SC.net) SC.net.connect(SC.game.state.player.name);
  }

  function registerSW() {
    if ('serviceWorker' in navigator && location.protocol !== 'file:') {
      navigator.serviceWorker.register('sw.js').catch(function (e) {
        console.warn('SW registration failed', e);
      });
    }
  }

  function init() {
    SC.assets.load();
    SC.render.init($('game-canvas'));
    SC.audio.init();
    SC.input.init();
    SC.game.init();
    SC.ui.init();
    bootUI();
    registerSW();
  }

  if (typeof document !== 'undefined') {
    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', init);
    else init();
  }
})();

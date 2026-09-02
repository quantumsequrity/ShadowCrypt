'use strict';
/* ShadowCrypt Online — procedural audio: WebAudio SFX synthesis + adaptive generative music.
 * No audio assets: every sound is synthesized. Unlocks on first user gesture. */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.audio = (function () {
  var U = SC.util;
  var ctx = null, master = null, sfxBus = null, musicBus = null;
  var settings = U.storeGet('sc_audio', { sfx: true, music: true });
  var musicTimer = null, currentMood = null, beat = 0;

  function ensureCtx() {
    if (ctx) return true;
    try {
      var AC = window.AudioContext || window.webkitAudioContext;
      if (!AC) return false;
      ctx = new AC();
      master = ctx.createGain(); master.gain.value = 0.5; master.connect(ctx.destination);
      sfxBus = ctx.createGain(); sfxBus.gain.value = settings.sfx ? 0.9 : 0; sfxBus.connect(master);
      musicBus = ctx.createGain(); musicBus.gain.value = settings.music ? 0.32 : 0; musicBus.connect(master);
      return true;
    } catch (e) { return false; }
  }

  function resume() {
    if (!ensureCtx()) return;
    if (ctx.state === 'suspended') ctx.resume();
  }

  // ---- synth primitives ---------------------------------------------------
  function tone(opts) {
    if (!ctx || !settings.sfx) return;
    var t0 = ctx.currentTime + (opts.delay || 0);
    var osc = ctx.createOscillator();
    var g = ctx.createGain();
    osc.type = opts.type || 'square';
    osc.frequency.setValueAtTime(opts.f0 || 440, t0);
    if (opts.f1) osc.frequency.exponentialRampToValueAtTime(Math.max(20, opts.f1), t0 + (opts.dur || 0.15));
    var vol = (opts.vol || 0.25);
    g.gain.setValueAtTime(0.0001, t0);
    g.gain.exponentialRampToValueAtTime(vol, t0 + (opts.attack || 0.005));
    g.gain.exponentialRampToValueAtTime(0.0001, t0 + (opts.dur || 0.15));
    osc.connect(g); g.connect(opts.bus || sfxBus);
    osc.start(t0); osc.stop(t0 + (opts.dur || 0.15) + 0.05);
  }

  function noise(opts) {
    if (!ctx || !settings.sfx) return;
    var t0 = ctx.currentTime + (opts.delay || 0);
    var dur = opts.dur || 0.2;
    var len = Math.max(1, Math.floor(ctx.sampleRate * dur));
    var buf = ctx.createBuffer(1, len, ctx.sampleRate);
    var data = buf.getChannelData(0);
    for (var i = 0; i < len; i++) data[i] = (Math.random() * 2 - 1) * (1 - i / len);
    var src = ctx.createBufferSource(); src.buffer = buf;
    var filter = ctx.createBiquadFilter();
    filter.type = opts.filter || 'lowpass';
    filter.frequency.setValueAtTime(opts.cutoff || 1200, t0);
    if (opts.cutoff1) filter.frequency.exponentialRampToValueAtTime(opts.cutoff1, t0 + dur);
    var g = ctx.createGain();
    g.gain.setValueAtTime(opts.vol || 0.3, t0);
    g.gain.exponentialRampToValueAtTime(0.0001, t0 + dur);
    src.connect(filter); filter.connect(g); g.connect(opts.bus || sfxBus);
    src.start(t0);
  }

  // ---- sound effects ------------------------------------------------------
  var SFX = {
    hit: function () { noise({ dur: 0.08, cutoff: 2500, vol: 0.28 }); tone({ type: 'square', f0: 180, f1: 60, dur: 0.08, vol: 0.16 }); },
    crit: function () { noise({ dur: 0.12, cutoff: 3600, vol: 0.36 }); tone({ type: 'sawtooth', f0: 520, f1: 90, dur: 0.16, vol: 0.24 }); },
    swing: function () { noise({ dur: 0.09, filter: 'bandpass', cutoff: 900, cutoff1: 2400, vol: 0.12 }); },
    hurt: function () { tone({ type: 'sawtooth', f0: 220, f1: 70, dur: 0.2, vol: 0.28 }); noise({ dur: 0.1, cutoff: 800, vol: 0.2 }); },
    death: function () { tone({ type: 'sawtooth', f0: 300, f1: 40, dur: 0.5, vol: 0.3 }); noise({ dur: 0.4, cutoff: 600, vol: 0.25 }); },
    kill: function () { tone({ type: 'triangle', f0: 300, f1: 500, dur: 0.1, vol: 0.14 }); noise({ dur: 0.12, cutoff: 1500, vol: 0.16 }); },
    gold: function () { tone({ type: 'sine', f0: 1320, dur: 0.07, vol: 0.16 }); tone({ type: 'sine', f0: 1760, dur: 0.1, vol: 0.14, delay: 0.05 }); },
    pickup: function () { tone({ type: 'triangle', f0: 660, f1: 990, dur: 0.12, vol: 0.18 }); },
    potion: function () { tone({ type: 'sine', f0: 440, f1: 880, dur: 0.25, vol: 0.2 }); tone({ type: 'sine', f0: 550, f1: 1100, dur: 0.25, vol: 0.12, delay: 0.06 }); },
    door: function () { noise({ dur: 0.18, cutoff: 500, vol: 0.22 }); tone({ type: 'square', f0: 90, f1: 60, dur: 0.15, vol: 0.1 }); },
    stairs: function () { [0, 1, 2].forEach(function (i) { tone({ type: 'triangle', f0: 330 - i * 60, dur: 0.12, vol: 0.16, delay: i * 0.09 }); }); },
    levelup: function () { [523, 659, 784, 1047].forEach(function (f, i) { tone({ type: 'triangle', f0: f, dur: 0.22, vol: 0.2, delay: i * 0.09 }); }); },
    quest: function () { [659, 880].forEach(function (f, i) { tone({ type: 'sine', f0: f, dur: 0.18, vol: 0.18, delay: i * 0.1 }); }); },
    achievement: function () { [784, 988, 1175, 1568].forEach(function (f, i) { tone({ type: 'triangle', f0: f, dur: 0.2, vol: 0.16, delay: i * 0.07 }); }); },
    skill_fire: function () { noise({ dur: 0.35, cutoff: 900, cutoff1: 200, vol: 0.3 }); tone({ type: 'sawtooth', f0: 140, f1: 50, dur: 0.3, vol: 0.2 }); },
    skill_ice: function () { tone({ type: 'sine', f0: 1200, f1: 400, dur: 0.3, vol: 0.2 }); noise({ dur: 0.25, filter: 'highpass', cutoff: 2500, vol: 0.14 }); },
    skill_bolt: function () { noise({ dur: 0.15, filter: 'highpass', cutoff: 1800, vol: 0.3 }); tone({ type: 'square', f0: 1400, f1: 200, dur: 0.12, vol: 0.2 }); },
    skill_holy: function () { [880, 1108, 1318].forEach(function (f, i) { tone({ type: 'sine', f0: f, dur: 0.4, vol: 0.12, delay: i * 0.04 }); }); },
    skill_dark: function () { tone({ type: 'sawtooth', f0: 110, f1: 55, dur: 0.45, vol: 0.24 }); tone({ type: 'sine', f0: 220, f1: 110, dur: 0.4, vol: 0.12 }); },
    heal: function () { tone({ type: 'sine', f0: 523, f1: 784, dur: 0.3, vol: 0.18 }); tone({ type: 'sine', f0: 659, f1: 988, dur: 0.3, vol: 0.12, delay: 0.08 }); },
    explosion: function () { noise({ dur: 0.5, cutoff: 1400, cutoff1: 100, vol: 0.42 }); tone({ type: 'square', f0: 80, f1: 30, dur: 0.4, vol: 0.3 }); },
    shoot: function () { tone({ type: 'square', f0: 700, f1: 250, dur: 0.09, vol: 0.14 }); },
    bow: function () { noise({ dur: 0.07, filter: 'bandpass', cutoff: 1600, vol: 0.14 }); tone({ type: 'triangle', f0: 900, f1: 400, dur: 0.08, vol: 0.1 }); },
    dash: function () { noise({ dur: 0.14, filter: 'bandpass', cutoff: 700, cutoff1: 2600, vol: 0.2 }); },
    trap: function () { tone({ type: 'square', f0: 150, f1: 70, dur: 0.2, vol: 0.26 }); noise({ dur: 0.12, cutoff: 3000, vol: 0.2 }); },
    chest: function () { tone({ type: 'triangle', f0: 392, dur: 0.1, vol: 0.16 }); [523, 659, 784].forEach(function (f, i) { tone({ type: 'sine', f0: f, dur: 0.14, vol: 0.14, delay: 0.1 + i * 0.06 }); }); },
    shrine: function () { [440, 554, 659, 880].forEach(function (f, i) { tone({ type: 'sine', f0: f, dur: 0.5, vol: 0.1, delay: i * 0.1 }); }); },
    build: function () { noise({ dur: 0.12, cutoff: 900, vol: 0.22 }); tone({ type: 'square', f0: 200, f1: 300, dur: 0.1, vol: 0.12, delay: 0.1 }); noise({ dur: 0.1, cutoff: 1100, vol: 0.18, delay: 0.18 }); },
    plant: function () { noise({ dur: 0.12, cutoff: 700, vol: 0.14 }); tone({ type: 'sine', f0: 300, f1: 420, dur: 0.14, vol: 0.1, delay: 0.05 }); },
    harvest: function () { tone({ type: 'triangle', f0: 587, f1: 880, dur: 0.15, vol: 0.16 }); tone({ type: 'triangle', f0: 880, dur: 0.12, vol: 0.12, delay: 0.1 }); },
    boss: function () { tone({ type: 'sawtooth', f0: 65, f1: 45, dur: 1.1, vol: 0.34 }); tone({ type: 'sawtooth', f0: 98, f1: 60, dur: 1.0, vol: 0.2, delay: 0.1 }); noise({ dur: 0.8, cutoff: 300, vol: 0.2 }); },
    telegraph: function () { tone({ type: 'sine', f0: 220, f1: 440, dur: 0.5, vol: 0.14 }); },
    ui: function () { tone({ type: 'sine', f0: 700, dur: 0.05, vol: 0.08 }); },
    error: function () { tone({ type: 'square', f0: 200, f1: 150, dur: 0.15, vol: 0.14 }); },
    wave: function () { tone({ type: 'sawtooth', f0: 130, f1: 260, dur: 0.5, vol: 0.2 }); tone({ type: 'sawtooth', f0: 196, f1: 392, dur: 0.5, vol: 0.14, delay: 0.15 }); },
    victory: function () { [523, 659, 784, 1047, 1318].forEach(function (f, i) { tone({ type: 'triangle', f0: f, dur: 0.3, vol: 0.18, delay: i * 0.11 }); }); }
  };

  function play(name) {
    if (!ctx || !settings.sfx) return;
    var fn = SFX[name];
    if (fn) { try { fn(); } catch (e) { /* audio must never break the game */ } }
  }

  // ---- generative music ---------------------------------------------------
  // Minor-scale generative sequencer; mood per game mode.
  var MOODS = {
    crypt: { root: 55, scale: [0, 2, 3, 5, 7, 8, 10], bpm: 60, padVol: 0.9, pluckChance: 0.30, bassEvery: 4, dark: true },
    haven: { root: 65.4, scale: [0, 2, 4, 7, 9], bpm: 72, padVol: 0.7, pluckChance: 0.45, bassEvery: 8, dark: false },
    farm: { root: 65.4, scale: [0, 2, 4, 7, 9], bpm: 72, padVol: 0.7, pluckChance: 0.45, bassEvery: 8, dark: false },
    arena: { root: 55, scale: [0, 2, 3, 5, 7, 8, 10], bpm: 128, padVol: 0.4, pluckChance: 0.6, bassEvery: 2, dark: true, drums: true },
    siege: { root: 49, scale: [0, 2, 3, 5, 7, 8, 10], bpm: 110, padVol: 0.6, pluckChance: 0.5, bassEvery: 2, dark: true, drums: true },
    boss: { root: 46.2, scale: [0, 1, 3, 5, 6, 8, 10], bpm: 96, padVol: 1.0, pluckChance: 0.5, bassEvery: 2, dark: true, drums: true }
  };

  function note(freq, t0, dur, type, vol, bus) {
    var osc = ctx.createOscillator();
    var g = ctx.createGain();
    osc.type = type; osc.frequency.value = freq;
    g.gain.setValueAtTime(0.0001, t0);
    g.gain.linearRampToValueAtTime(vol, t0 + Math.min(0.08, dur * 0.3));
    g.gain.exponentialRampToValueAtTime(0.0001, t0 + dur);
    osc.connect(g); g.connect(bus || musicBus);
    osc.start(t0); osc.stop(t0 + dur + 0.1);
  }

  function musicStep() {
    if (!ctx || !settings.music || !currentMood) return;
    var m = MOODS[currentMood] || MOODS.crypt;
    var stepDur = 60 / m.bpm / 2; // eighth notes
    var t0 = ctx.currentTime + 0.05;
    beat++;
    var deg = m.scale[Math.floor(Math.random() * m.scale.length)];
    // pad drone every 8 steps
    if (beat % 8 === 1) {
      var padF = m.root * Math.pow(2, m.scale[0] / 12);
      note(padF, t0, stepDur * 8, 'sine', 0.05 * m.padVol);
      note(padF * Math.pow(2, m.scale[2] / 12), t0, stepDur * 8, 'sine', 0.035 * m.padVol);
      if (m.dark) note(padF * 0.5, t0, stepDur * 8, 'triangle', 0.05 * m.padVol);
    }
    // bass
    if (beat % m.bassEvery === 0) {
      note(m.root, t0, stepDur * 1.2, 'triangle', 0.09);
    }
    // pluck melody
    if (Math.random() < m.pluckChance) {
      var f = m.root * 2 * Math.pow(2, deg / 12) * (Math.random() < 0.3 ? 2 : 1);
      note(f, t0, stepDur * (Math.random() < 0.3 ? 2 : 0.9), 'triangle', 0.05);
    }
    // drums (arena/siege/boss)
    if (m.drums) {
      if (beat % 4 === 0) { // kick
        var osc = ctx.createOscillator(); var g = ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(120, t0);
        osc.frequency.exponentialRampToValueAtTime(45, t0 + 0.12);
        g.gain.setValueAtTime(0.22, t0);
        g.gain.exponentialRampToValueAtTime(0.0001, t0 + 0.14);
        osc.connect(g); g.connect(musicBus);
        osc.start(t0); osc.stop(t0 + 0.2);
      }
      if (beat % 4 === 2 && settings.music) { // hat via short noise
        var len = Math.floor(ctx.sampleRate * 0.03);
        var buf = ctx.createBuffer(1, len, ctx.sampleRate);
        var d = buf.getChannelData(0);
        for (var i = 0; i < len; i++) d[i] = (Math.random() * 2 - 1) * (1 - i / len);
        var src = ctx.createBufferSource(); src.buffer = buf;
        var hf = ctx.createBiquadFilter(); hf.type = 'highpass'; hf.frequency.value = 6000;
        var hg = ctx.createGain(); hg.gain.value = 0.07;
        src.connect(hf); hf.connect(hg); hg.connect(musicBus);
        src.start(t0);
      }
    }
    musicTimer = setTimeout(musicStep, stepDur * 1000);
  }

  function setMood(mood) {
    if (currentMood === mood) return;
    currentMood = mood;
    beat = 0;
    if (musicTimer) { clearTimeout(musicTimer); musicTimer = null; }
    if (ctx && settings.music && mood) musicStep();
  }

  // ---- settings -----------------------------------------------------------
  function setSfx(on) {
    settings.sfx = !!on;
    U.storeSet('sc_audio', settings);
    if (sfxBus) sfxBus.gain.value = on ? 0.9 : 0;
  }
  function setMusic(on) {
    settings.music = !!on;
    U.storeSet('sc_audio', settings);
    if (musicBus) musicBus.gain.value = on ? 0.32 : 0;
    if (on && currentMood && !musicTimer && ctx) musicStep();
    if (!on && musicTimer) { clearTimeout(musicTimer); musicTimer = null; }
  }

  function init() {
    if (typeof document === 'undefined') return;
    // unlock on first gesture
    var unlock = function () {
      resume();
      if (currentMood && settings.music && !musicTimer && ctx) musicStep();
      document.removeEventListener('pointerdown', unlock);
      document.removeEventListener('keydown', unlock);
    };
    document.addEventListener('pointerdown', unlock);
    document.addEventListener('keydown', unlock);
    document.addEventListener('visibilitychange', function () {
      if (!ctx) return;
      if (document.visibilityState === 'hidden') { if (ctx.state === 'running') ctx.suspend(); }
      else if (ctx.state === 'suspended') ctx.resume();
    });
    U.on('sfx', play);
    U.on('music', setMood);
  }

  return { init: init, play: play, setMood: setMood, setSfx: setSfx, setMusic: setMusic, settings: function () { return settings; } };
})();

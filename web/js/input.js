'use strict';
/* ShadowCrypt Online — input: keyboard (PC), touch joystick + buttons (mobile/tablet), tap-to-move */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.input = (function () {
  var U = SC.util;

  var keys = {};
  var joy = { active: false, id: null, cx: 0, cy: 0, dx: 0, dy: 0 };
  var tapTarget = null; // {x,y} world tile tapped (crypt tap-to-move)
  var attackHeld = false;
  var lookAccum = 0;                 // radians of camera turn accumulated since last consume
  var lookTouch = { id: null, lastX: 0 };
  var mouseLook = { down: false, lastX: 0, moved: 0 };

  function init() {
    if (typeof window === 'undefined') return;

    window.addEventListener('keydown', function (e) {
      if (e.target && (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA')) return;
      keys[e.key.toLowerCase()] = true;
      var k = e.key.toLowerCase();
      if (k === 'i') U.emit('ui:panel', 'inventory');
      if (k === 'tab') { e.preventDefault(); U.emit('ui:panel', 'equipment'); }
      if (k === 'j') U.emit('ui:panel', 'quests');
      if (k === 'k') U.emit('ui:panel', 'character');
      if (k === 'm') U.emit('ui:panel', 'map');
      if (k === '?') U.emit('ui:panel', 'help');
      if (k === 'escape') U.emit('ui:escape');
      if (k === 'g') U.emit('action:interact');
      if (k === 'f') U.emit('action:interact');
      if (k === 'shift') U.emit('action:dash');
      if (k === 'v') U.emit('action:camera');
      if (k === ' ') { e.preventDefault(); U.emit('action:attack'); }
      if (k === 'enter') U.emit('ui:chat-focus');
      if (k >= '1' && k <= '4') U.emit('action:skill', parseInt(k, 10) - 1);
      if (k === '>' || k === '.') U.emit('action:descend');
      if (k === '<' || k === ',') U.emit('action:ascend');
    });
    window.addEventListener('keyup', function (e) {
      keys[e.key.toLowerCase()] = false;
    });
    window.addEventListener('blur', function () { keys = {}; });

    // detect touch → show joystick
    window.addEventListener('touchstart', function onFirstTouch() {
      document.body.classList.add('touch');
      window.removeEventListener('touchstart', onFirstTouch);
    }, { passive: true });

    initJoystick();
    initButtons();
    initCanvasTaps();
    initLook();
  }

  // Camera look: touch-drag on the right 55% of the screen, or mouse drag, turns the view (TPP/FPP)
  function initLook() {
    var canvas = document.getElementById('game-canvas');
    if (!canvas) return;
    var SENS = 0.0075;
    canvas.addEventListener('touchstart', function (e) {
      for (var i = 0; i < e.changedTouches.length; i++) {
        var t = e.changedTouches[i];
        if (t.identifier === joy.id) continue;
        if (t.clientX > window.innerWidth * 0.45 && lookTouch.id === null) {
          lookTouch.id = t.identifier;
          lookTouch.lastX = t.clientX;
        }
      }
    }, { passive: true });
    window.addEventListener('touchmove', function (e) {
      if (lookTouch.id === null) return;
      for (var i = 0; i < e.changedTouches.length; i++) {
        var t = e.changedTouches[i];
        if (t.identifier === lookTouch.id) {
          lookAccum += (t.clientX - lookTouch.lastX) * SENS;
          lookTouch.lastX = t.clientX;
        }
      }
    }, { passive: true });
    function endLook(e) {
      for (var i = 0; i < e.changedTouches.length; i++) {
        if (e.changedTouches[i].identifier === lookTouch.id) lookTouch.id = null;
      }
    }
    window.addEventListener('touchend', endLook);
    window.addEventListener('touchcancel', endLook);
    // mouse drag look
    canvas.addEventListener('mousedown', function (e) {
      mouseLook.down = true; mouseLook.lastX = e.clientX; mouseLook.moved = 0;
    });
    window.addEventListener('mousemove', function (e) {
      if (!mouseLook.down) return;
      var dx = e.clientX - mouseLook.lastX;
      mouseLook.moved += Math.abs(dx);
      lookAccum += dx * SENS;
      mouseLook.lastX = e.clientX;
    });
    window.addEventListener('mouseup', function () { mouseLook.down = false; });
  }

  function consumeLook() {
    var v = lookAccum;
    lookAccum = 0;
    // keyboard turn keys
    if (keys['q']) v -= 0.045;
    if (keys['e']) v += 0.045;
    return v;
  }

  // did the last mouse interaction turn the camera (suppress tap-to-move)?
  function wasDragging() { return mouseLook.moved > 8; }

  function initJoystick() {
    var el = document.getElementById('joystick');
    var knob = document.getElementById('joystick-knob');
    if (!el || !knob) return;
    var R = 46;

    function setKnob() {
      knob.style.transform = 'translate(calc(-50% + ' + (joy.dx * R) + 'px), calc(-50% + ' + (joy.dy * R) + 'px))';
    }
    el.addEventListener('touchstart', function (e) {
      e.preventDefault();
      var t = e.changedTouches[0];
      joy.active = true; joy.id = t.identifier;
      var r = el.getBoundingClientRect();
      joy.cx = r.left + r.width / 2; joy.cy = r.top + r.height / 2;
      moveJoy(t.clientX, t.clientY);
    }, { passive: false });
    window.addEventListener('touchmove', function (e) {
      if (!joy.active) return;
      for (var i = 0; i < e.changedTouches.length; i++) {
        var t = e.changedTouches[i];
        if (t.identifier === joy.id) { moveJoy(t.clientX, t.clientY); e.preventDefault(); }
      }
    }, { passive: false });
    function endJoy(e) {
      for (var i = 0; i < e.changedTouches.length; i++) {
        if (e.changedTouches[i].identifier === joy.id) {
          joy.active = false; joy.dx = 0; joy.dy = 0;
          setKnob();
        }
      }
    }
    window.addEventListener('touchend', endJoy);
    window.addEventListener('touchcancel', endJoy);

    function moveJoy(x, y) {
      var dx = (x - joy.cx) / R, dy = (y - joy.cy) / R;
      var len = Math.sqrt(dx * dx + dy * dy);
      if (len > 1) { dx /= len; dy /= len; }
      joy.dx = dx; joy.dy = dy;
      setKnob();
    }
  }

  function initButtons() {
    function bindHold(id, downFn, upFn) {
      var el = document.getElementById(id);
      if (!el) return;
      var down = function (e) { e.preventDefault(); downFn(); };
      var up = function (e) { e.preventDefault(); if (upFn) upFn(); };
      el.addEventListener('touchstart', down, { passive: false });
      el.addEventListener('touchend', up, { passive: false });
      el.addEventListener('mousedown', down);
      el.addEventListener('mouseup', up);
      el.addEventListener('mouseleave', function () { if (upFn) upFn(); });
    }
    bindHold('act-attack', function () { attackHeld = true; U.emit('action:attack'); }, function () { attackHeld = false; });
    bindHold('act-interact', function () { U.emit('action:interact'); });
    bindHold('act-dash', function () { U.emit('action:dash'); });
    bindHold('btn-camera', function () { U.emit('action:camera'); });
    for (var i = 1; i <= 4; i++) {
      (function (n) {
        bindHold('act-skill' + n, function () { U.emit('action:skill', n - 1); });
      })(i);
    }
  }

  function initCanvasTaps() {
    var canvas = document.getElementById('game-canvas');
    if (!canvas) return;
    function onTap(cx, cy) { U.emit('canvas:tap', { x: cx, y: cy }); }
    canvas.addEventListener('click', function (e) { onTap(e.clientX, e.clientY); });
    canvas.addEventListener('touchend', function (e) {
      if (joy.active) return;
      if (e.changedTouches.length) {
        var t = e.changedTouches[0];
        onTap(t.clientX, t.clientY);
      }
    });
  }

  // Analog movement vector from keyboard + joystick, normalized
  function moveVector() {
    var mx = 0, my = 0;
    if (keys['w'] || keys['arrowup']) my -= 1;
    if (keys['s'] || keys['arrowdown']) my += 1;
    if (keys['a'] || keys['arrowleft']) mx -= 1;
    if (keys['d'] || keys['arrowright']) mx += 1;
    if (keys['z']) { mx -= 1; my += 1; }
    if (keys['c']) { mx += 1; my += 1; }
    var len = Math.sqrt(mx * mx + my * my);
    if (len > 1) { mx /= len; my /= len; }
    mx += joy.dx; my += joy.dy;
    len = Math.sqrt(mx * mx + my * my);
    if (len > 1) { mx /= len; my /= len; }
    return { x: mx, y: my };
  }

  function setTapTarget(t) { tapTarget = t; }
  function getTapTarget() { return tapTarget; }
  function clearTapTarget() { tapTarget = null; }
  function isAttackHeld() { return attackHeld || !!keys[' ']; }
  function isKey(k) { return !!keys[k]; }

  return {
    init: init,
    moveVector: moveVector,
    consumeLook: consumeLook,
    wasDragging: wasDragging,
    setTapTarget: setTapTarget, getTapTarget: getTapTarget, clearTapTarget: clearTapTarget,
    isAttackHeld: isAttackHeld,
    isKey: isKey,
    joystick: function () { return joy; }
  };
})();

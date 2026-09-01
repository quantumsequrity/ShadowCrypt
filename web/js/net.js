'use strict';
/* ShadowCrypt Online — network client: presence, chat, co-op ghosts, arena rooms, cloud saves.
 * Offline-first: everything works without a server; when reachable, online features light up. */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.net = (function () {
  var U = SC.util;

  var ws = null;
  var connected = false;
  var myId = null;
  var onlineCount = 0;
  var reconnectDelay = 2000;
  var wantConnection = true;
  var pending = [];
  var arenaRoom = null;
  var ghosts = {}; // other players in the crypt: id -> {name,floor,x,y,classId,at}
  var token = U.storeGet('sc_token', null);

  function url() {
    if (typeof location === 'undefined') return null;
    if (location.protocol === 'file:') return null;
    var proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    return proto + '//' + location.host + '/ws';
  }

  function connect(playerName) {
    var u = url();
    if (!u || !wantConnection) return;
    if (ws && (ws.readyState === 0 || ws.readyState === 1)) return;
    try {
      ws = new WebSocket(u);
    } catch (e) {
      scheduleReconnect(playerName);
      return;
    }
    ws.onopen = function () {
      reconnectDelay = 2000;
      send({ t: 'hello', name: playerName || 'Adventurer', token: token });
    };
    ws.onmessage = function (ev) {
      var msg;
      try { msg = JSON.parse(ev.data); } catch (e) { return; }
      handle(msg);
    };
    ws.onclose = function () {
      var was = connected;
      connected = false;
      arenaRoom = null;
      ghosts = {};
      if (was) U.emit('net:down');
      scheduleReconnect(playerName);
    };
    ws.onerror = function () { /* onclose follows */ };
  }

  function scheduleReconnect(playerName) {
    if (!wantConnection) return;
    setTimeout(function () { connect(playerName); }, reconnectDelay);
    reconnectDelay = Math.min(30000, reconnectDelay * 1.6);
  }

  function send(obj) {
    if (ws && ws.readyState === 1) {
      try { ws.send(JSON.stringify(obj)); return true; } catch (e) { return false; }
    }
    return false;
  }

  function handle(msg) {
    switch (msg.t) {
      case 'welcome':
        connected = true;
        myId = msg.id;
        onlineCount = msg.online || 1;
        if (msg.token) { token = msg.token; U.storeSet('sc_token', token); }
        U.emit('net:up', { online: onlineCount });
        while (pending.length) send(pending.shift());
        break;
      case 'online':
        onlineCount = msg.n;
        U.emit('net:online', msg.n);
        break;
      case 'chat':
        U.emit('chat', { from: msg.id, name: msg.name, text: msg.text, system: !!msg.system });
        break;
      case 'pos':
        if (msg.id !== myId) {
          ghosts[msg.id] = { name: msg.name, floor: msg.floor, x: msg.x, y: msg.y, classId: msg.classId, at: Date.now() };
        }
        break;
      case 'bye':
        delete ghosts[msg.id];
        break;
      case 'arena_joined':
        arenaRoom = { room: msg.room, seed: msg.seed };
        U.emit('arena:net-joined', arenaRoom);
        break;
      case 'arena':
        if (SC.arena) SC.arena.handleNet(msg.from, msg.name, msg.d || {});
        break;
      case 'loaded':
        U.emit('net:loaded', msg.data || null);
        break;
      case 'saved':
        U.emit('net:saved');
        break;
      case 'error':
        U.emit('msg', '⚠ ' + (msg.text || 'server error'));
        break;
    }
  }

  // ---- Public API ---------------------------------------------------------
  function chat(text) {
    if (!send({ t: 'chat', text: String(text).slice(0, 200) })) {
      U.emit('chat', { from: 'sys', name: '', text: 'You are offline — no one hears you.', system: true });
    }
  }

  function reportPos(p) {
    if (!connected) return;
    send({ t: 'pos', floor: p.floor, x: p.x, y: p.y, classId: p.classId, name: p.name });
  }

  function cryptGhosts(floor) {
    var out = [];
    var now = Date.now();
    for (var id in ghosts) {
      var g = ghosts[id];
      if (now - g.at > 15000) { delete ghosts[id]; continue; }
      if (g.floor === floor) out.push(g);
    }
    return out;
  }

  function joinArena() {
    if (!connected) return false;
    return send({ t: 'arena_join' });
  }
  function leaveArena() {
    arenaRoom = null;
    send({ t: 'arena_leave' });
  }
  function sendArena(d) {
    if (!arenaRoom) return;
    send({ t: 'arena', d: d });
  }

  function cloudSave(data) {
    if (!connected) return false;
    return send({ t: 'save', data: data });
  }
  function cloudLoad() {
    if (!connected) return false;
    return send({ t: 'load' });
  }

  return {
    connect: connect,
    isConnected: function () { return connected; },
    onlineCount: function () { return onlineCount; },
    myId: function () { return myId; },
    chat: chat,
    reportPos: reportPos,
    cryptGhosts: cryptGhosts,
    joinArena: joinArena, leaveArena: leaveArena, sendArena: sendArena,
    arenaRoom: function () { return arenaRoom; },
    cloudSave: cloudSave, cloudLoad: cloudLoad,
    stop: function () { wantConnection = false; if (ws) try { ws.close(); } catch (e) { /* noop */ } }
  };
})();

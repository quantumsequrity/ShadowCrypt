'use strict';
/* ShadowCrypt Online — multiplayer server
 * - serves the web client (../web)
 * - WebSocket at /ws: chat, crypt presence, arena PvP rooms, cloud saves
 * Run: npm install && npm start  (PORT env var, default 8080) */

const http = require('http');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const WebSocket = require('ws');

const PORT = parseInt(process.env.PORT || '8080', 10);
const WEB_ROOT = path.join(__dirname, '..', 'web');
const SAVE_DIR = path.join(__dirname, 'data', 'saves');
fs.mkdirSync(SAVE_DIR, { recursive: true });

// ---------------------------------------------------------------- static
const MIME = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.json': 'application/json',
  '.webmanifest': 'application/manifest+json',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.ico': 'image/x-icon',
  '.woff2': 'font/woff2'
};

const server = http.createServer((req, res) => {
  let urlPath;
  try {
    urlPath = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  } catch (e) {
    res.writeHead(400); res.end('bad request'); return;
  }
  if (urlPath === '/') urlPath = '/index.html';
  if (urlPath === '/health') { res.writeHead(200, { 'Content-Type': 'application/json' }); res.end(JSON.stringify({ ok: true, online: clients.size })); return; }
  const filePath = path.normalize(path.join(WEB_ROOT, urlPath));
  if (filePath !== WEB_ROOT && !filePath.startsWith(WEB_ROOT + path.sep)) { res.writeHead(403); res.end('forbidden'); return; }
  fs.readFile(filePath, (err, data) => {
    if (err) {
      // SPA-ish fallback to the shell for unknown GETs (PWA deep links)
      fs.readFile(path.join(WEB_ROOT, 'index.html'), (err2, shell) => {
        if (err2) { res.writeHead(404); res.end('not found'); return; }
        res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
        res.end(shell);
      });
      return;
    }
    const ext = path.extname(filePath).toLowerCase();
    res.writeHead(200, {
      'Content-Type': MIME[ext] || 'application/octet-stream',
      'Cache-Control': ext === '.html' ? 'no-cache' : 'public, max-age=300'
    });
    res.end(data);
  });
});

// ---------------------------------------------------------------- ws
const wss = new WebSocket.Server({ server, path: '/ws' });
const clients = new Map(); // ws -> {id, name, token, floor, arenaRoom}
let seq = 1;

// ---------------------------------------------------------------- leaderboard
const LB_PATH = path.join(__dirname, 'data', 'leaderboard.json');
let leaderboard = {};
try { leaderboard = JSON.parse(fs.readFileSync(LB_PATH, 'utf8')); } catch (e) { leaderboard = {}; }
let lbDirty = false;
setInterval(() => {
  if (!lbDirty) return;
  lbDirty = false;
  fs.writeFile(LB_PATH, JSON.stringify(leaderboard), () => {});
}, 15000);

function updateLeaderboard(token, save) {
  try {
    const p = save && save.player;
    if (!p || !p.name) return;
    const key = crypto.createHash('sha256').update(String(token)).digest('hex').slice(0, 16);
    leaderboard[key] = {
      name: String(p.name).slice(0, 16),
      level: p.level | 0,
      floor: (p.stats && p.stats.deepestFloor) | 0,
      kills: (p.stats && p.stats.kills) | 0,
      bossKills: (p.stats && p.stats.bossKills) | 0,
      pvpWins: (p.stats && p.stats.pvpWins) | 0,
      classId: String(p.classId || '').slice(0, 20),
      at: Date.now()
    };
    lbDirty = true;
  } catch (e) { /* never let stats break saves */ }
}

function topPlayers(n) {
  return Object.values(leaderboard)
    .sort((a, b) => (b.floor - a.floor) || (b.level - a.level) || (b.kills - a.kills))
    .slice(0, n);
}

const ARENA_ROOM_MAX = 6;
const arenaRooms = new Map(); // roomId -> {seed, members:Set<ws>}

function send(ws, obj) {
  if (ws.readyState === WebSocket.OPEN) {
    try { ws.send(JSON.stringify(obj)); } catch (e) { /* noop */ }
  }
}

function broadcast(obj, exceptWs) {
  const raw = JSON.stringify(obj);
  for (const [ws] of clients) {
    if (ws !== exceptWs && ws.readyState === WebSocket.OPEN) {
      try { ws.send(raw); } catch (e) { /* noop */ }
    }
  }
}

function broadcastOnline() {
  broadcast({ t: 'online', n: clients.size });
}

function sanitizeName(name) {
  return String(name || 'Adventurer').replace(/[^\w \-']/g, '').slice(0, 16) || 'Adventurer';
}

function savePathFor(token) {
  const hash = crypto.createHash('sha256').update(String(token)).digest('hex');
  return path.join(SAVE_DIR, hash + '.json');
}

function findArenaRoom() {
  for (const [id, room] of arenaRooms) {
    if (room.members.size < ARENA_ROOM_MAX) return id;
  }
  const id = 'ar' + (seq++);
  arenaRooms.set(id, { seed: (Date.now() / 60000) | 0, members: new Set() });
  return id;
}

function leaveArenaRoom(ws) {
  const c = clients.get(ws);
  if (!c || !c.arenaRoom) return;
  const room = arenaRooms.get(c.arenaRoom);
  if (room) {
    room.members.delete(ws);
    if (room.members.size === 0) arenaRooms.delete(c.arenaRoom);
  }
  c.arenaRoom = null;
}

wss.on('connection', (ws) => {
  ws.isAlive = true;
  ws.on('pong', () => { ws.isAlive = true; });

  ws.on('message', (raw) => {
    if (raw.length > 256 * 1024) return; // hard cap
    let msg;
    try { msg = JSON.parse(raw); } catch (e) { return; }
    const c = clients.get(ws);

    if (msg.t === 'hello') {
      const token = (typeof msg.token === 'string' && msg.token.length >= 16 && msg.token.length <= 64)
        ? msg.token
        : crypto.randomBytes(24).toString('hex');
      const info = {
        id: 'p' + (seq++),
        name: sanitizeName(msg.name),
        token,
        floor: 0,
        arenaRoom: null,
        lastChat: 0
      };
      clients.set(ws, info);
      send(ws, { t: 'welcome', id: info.id, online: clients.size, token });
      broadcastOnline();
      return;
    }
    if (!c) return; // must hello first

    switch (msg.t) {
      case 'chat': {
        const now = Date.now();
        if (now - c.lastChat < 500) return; // rate limit
        c.lastChat = now;
        const text = String(msg.text || '').slice(0, 200);
        if (!text.trim()) return;
        broadcast({ t: 'chat', id: c.id, name: c.name, text }, ws);
        break;
      }
      case 'pos': {
        c.floor = msg.floor | 0;
        if (typeof msg.name === 'string') c.name = sanitizeName(msg.name);
        broadcast({
          t: 'pos', id: c.id, name: c.name,
          floor: c.floor, x: msg.x | 0, y: msg.y | 0,
          classId: String(msg.classId || '').slice(0, 24)
        }, ws);
        break;
      }
      case 'arena_join': {
        leaveArenaRoom(ws);
        const roomId = findArenaRoom();
        const room = arenaRooms.get(roomId);
        room.members.add(ws);
        c.arenaRoom = roomId;
        send(ws, { t: 'arena_joined', room: roomId, seed: room.seed });
        break;
      }
      case 'arena_leave':
        leaveArenaRoom(ws);
        break;
      case 'arena': {
        if (!c.arenaRoom) return;
        const room = arenaRooms.get(c.arenaRoom);
        if (!room) return;
        const out = JSON.stringify({ t: 'arena', from: c.id, name: c.name, d: msg.d || {} });
        for (const member of room.members) {
          if (member !== ws && member.readyState === WebSocket.OPEN) {
            try { member.send(out); } catch (e) { /* noop */ }
          }
        }
        break;
      }
      case 'save': {
        if (!msg.data || typeof msg.data !== 'object') return;
        const json = JSON.stringify(msg.data);
        if (json.length > 512 * 1024) { send(ws, { t: 'error', text: 'save too large' }); return; }
        fs.writeFile(savePathFor(c.token), json, (err) => {
          send(ws, err ? { t: 'error', text: 'save failed' } : { t: 'saved' });
        });
        updateLeaderboard(c.token, msg.data);
        break;
      }
      case 'leaderboard': {
        send(ws, { t: 'leaderboard', top: topPlayers(20) });
        break;
      }
      case 'load': {
        fs.readFile(savePathFor(c.token), 'utf8', (err, data) => {
          if (err) { send(ws, { t: 'loaded', data: null }); return; }
          try { send(ws, { t: 'loaded', data: JSON.parse(data) }); }
          catch (e) { send(ws, { t: 'loaded', data: null }); }
        });
        break;
      }
    }
  });

  ws.on('close', () => {
    const c = clients.get(ws);
    leaveArenaRoom(ws);
    clients.delete(ws);
    if (c) broadcast({ t: 'bye', id: c.id });
    broadcastOnline();
  });
  ws.on('error', () => { /* close follows */ });
});

// heartbeat: drop dead sockets
const heartbeat = setInterval(() => {
  for (const ws of wss.clients) {
    if (ws.isAlive === false) { ws.terminate(); continue; }
    ws.isAlive = false;
    try { ws.ping(); } catch (e) { /* noop */ }
  }
}, 30000);
wss.on('close', () => clearInterval(heartbeat));

server.listen(PORT, () => {
  console.log('ShadowCrypt Online server listening on http://localhost:' + PORT);
  console.log('Serving client from ' + WEB_ROOT);
});

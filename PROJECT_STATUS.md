# ShadowCrypt Online — Project Status, Progress & Roadmap

> **The single source of truth for the terminal-to-MMORPG conversion project.**
> All game code lives on the branch [`claude/terminal-to-mmorpg-conversion-tzv53z`](https://github.com/quantumsequrity/ShadowCrypt/tree/claude/terminal-to-mmorpg-conversion-tzv53z) — open as [Pull Request #1](https://github.com/quantumsequrity/ShadowCrypt/pull/1).
> `main` still holds the original Rust terminal roguelike, untouched.

---

## 1. The End Goal (the vision)

Convert ShadowCrypt from a Rust **terminal roguelike** into a full **cross-platform action MMORPG** that is:

- **Playable everywhere** — phone, tablet, and PC, in any browser, with touch controls (virtual joystick, buttons, look-drag) and keyboard/mouse.
- **Playable offline AND online** — installable as a PWA that runs with zero connection; online it becomes a shared world (chat, seeing other players, PvP, leaderboards, cloud saves).
- **A blend of the games that inspired it**:
  - *Genshin Impact* → real-time action combat, third-person exploration
  - *PUBG / Free Fire* → first-person & third-person camera modes with shooter-style look controls
  - *Clash of Clans* → buildable/upgradable base with offline resource production and playable siege defense
  - *Mini Militia / BombSquad* → fast real-time PvP arena with bombs and power-ups
  - *Farming games* → real-time crops you plant, water, and harvest
- **Beautiful and detailed** — professional game art, not programmer shapes: real sprites, textured 3D, polished UI, sound and music.
- **Faithful** — every class, species, enemy, item, quest, faction, companion, and recipe from the original Rust game preserved.

---

## 2. What Has Been Built (current progress)

### ✅ Phase 1 — Data port (100% of Rust content extracted to JS)

Parallel extraction agents read the entire Rust source and produced verbatim JS data modules (`web/js/data/`):

| Domain | Count |
|---|---|
| Classes / subclasses / advanced classes / skills | 6 / 37 / 18 / 132 |
| Species (with subspecies, incl. dragon-form evolution) | 11 |
| Enemies + bosses & mini-bosses | 148 + 12 |
| Items + rarity tiers | 156 + 6 |
| Quests | 89 |
| Factions / NPCs / achievements | 16 / 7 / 34 |
| Companions (with bond & leveling tables) | 33 |
| Crafting materials / recipes | 47 / 226 |
| Status effects / floor themes / spells | 13 / 8 / 150 |

Every generated data file carries `*_notes` documenting any value the Rust source didn't express.

### ✅ Phase 2 — The game itself (v1 commit `f0f9b5a`)

- **🗡 Crypt** — the original 30-floor descent as a real-time action RPG: procedural floors (rooms, corridors, doors, traps, water/lava, chests, shrines, boss arenas), FOV & lighting, leveling, subclass choice at level 10, loot with rarities, hunger, companions fighting beside you. Death = respawn at Haven with 10% gold loss (no more permadeath).
- **🏰 Haven** — Clash-style base: 11 building types, gold/mana produced *while you're away*, walls & towers, daily blessings, a portal to start descents deeper.
- **🌾 Farm** — 7 crops growing in real time (watering doubles speed), yields food & potions.
- **💥 Arena** — real-time PvP deathmatch: projectiles, bombs, 5 power-up types; online rooms of up to 6, training bots fill empty rooms.
- **🌐 Online layer** — Node.js server (`server/`, only dependency: `ws`): static hosting, global chat, crypt "ghost" presence of other players, arena rooms with shared map seeds, token-keyed cloud saves.
- **📱 PWA** — service worker caches the whole game (41 assets) for full offline play; installable to home screen.

### ✅ Phase 3 — Sophistication (v2 commit `2e9432d`)

- **🎥 Three camera modes** (key **V**): top-down, third-person, first-person — a raycasting 3D engine with billboard sprites, fog, weapon viewmodels, crosshair, compass minimap, shooter-style look controls.
- **🔊 Fully synthesized audio** — 35+ WebAudio sound effects + adaptive generative music per mode (crypt ambient, boss dread theme, arena/siege drums). Zero audio files.
- **⚔️ Combat feel** — 120° melee arcs, crit knockback, dodge-dash with i-frames (Shift), screen shake, hit flashes, telegraphed boss abilities (slam / nova / summon / volley) with dodge rings, boss enrage at 50% HP.
- **💎 Deep loot** — elite monsters with 6 affixes (auras, on-hit procs, double drops); 11 item affixes ("of the Leech", "of Flames"…) with procs, lifesteal, gold-find; locked golden chests opened with keys from mini-bosses; smashable urns/crates.
- **💪 Talents** — 3 points per level across Might / Vitality / Bulwark / Spirit / Agility.
- **🏰 Playable siege defense** — every 4h, fight raid waves in person beside your towers and bone walls for gold/materials/XP.
- **👑 Server leaderboards** — deepest-floor rankings, in-game Ranks panel.

### ✅ Phase 4 — Professional art (v3 commit `aa8f5f7`)

- **633 real pixel-art sprites** integrated from the open-source *Dungeon Crawl Stone Soup* tileset (public domain — attribution in `web/assets/LICENSE-art.txt`): every monster, item, themed wall/floor set, feature, building, spell icon and effect. An automated fuzzy-matching + Chromium compositing pipeline packed them into one 612 KB texture atlas.
- **Paper-doll hero** — the character sprite is composited live from equipment layers: equip a new weapon/shield/helmet/armor/gloves/boots and it **visibly changes** on your hero (world, 3D, HUD portrait).
- **True textured raycasting** — FPP/TPP render real per-theme wall textures column-by-column with distance lighting and torch bounce.
- Sprite icons across the whole UI (inventory, equipment, shop, crafting, skill buttons); graceful fallback to procedural vector art if the atlas hasn't loaded.

### ✅ Phase 5 — Final polish (commit `2b36cfa`)

- Bundled "Press Start 2P" pixel font (OFL-licensed, offline-cached) for title/HUD identity.
- Title-screen animated parade of the six class heroes; live hero sprite previews on class-selection cards.

### 🧪 Verification (all green, zero console errors)

- `node --check` on all 28 client JS files + server
- Simulation suite: all 30 floors generated & stairs reachable, all 6 classes create/level/fight, haven build→plant→water→harvest→collect→upgrade, quests, crafting end-to-end, arena bot sim, all cross-data references resolve
- 28-check + 19-check Playwright browser suites (run twice for stability): creation → combat → dash → TPP/FPP → elites → affixes → talents → telegraphs → siege victory → leaderboard → save/reload persistence
- 9 WebSocket protocol tests; offline-PWA boot-and-play test (41 cached assets)

---

## 3. Work That Was In Progress When Paused

A "completion pass" was underway (drafted but **not committed** — the branch is clean without it):

1. **Difficulty balance** — gate elite monsters off floors 1–2 and scale their chance by depth, so fresh level-1 heroes aren't deleted by a Colossal rat. *(One-line change in `web/js/entities.js`, drafted and reverted per stop request.)*
2. **Advanced class evolution at level 25** — the data is already extracted (18 advanced forms, e.g. Berserker → Warlord); needs a small UI block in the Hero panel mirroring the existing level-10 subclass picker.
3. **Screenshots in the repo** — copy the promo screenshots into `docs/screenshots/` and embed them in `PLAY_ONLINE.md` / the PR so GitHub shows the game visually.
4. **Flip PR #1 from draft to “Ready for review.”**

---

## 4. Roadmap — What We Intend To Do Next

### Near term (hours of work)
- Finish the completion pass above (items 1–4).
- Sound/music volume sliders; more SFX variety per weapon type.
- Mobile ergonomics pass: button sizing options, left-handed mode.
- Difficulty curve tuning across all 30 floors; XP/gold economy balance.

### Mid term (the big MMO features)
- **True co-op dungeon runs** — party system with a shared floor seed exists (ghost presence works); add synced monster state (host-authoritative) so parties fight the same monsters together.
- **PvP base raiding** — attack *other players'* havens (the siege engine already simulates attackers/walls/towers; point it at a downloaded snapshot of another player's base — the real Clash of Clans loop).
- **Guilds/clans** — shared chat channel, guild leaderboard, cooperative siege events.
- **Trading / auction house** — player-to-player item exchange via the server.
- **Accounts** — optional username/password or OAuth on top of the current token-based cloud saves; server-side validation & anti-cheat for leaderboards.
- **More camera polish** — floor/ceiling texture casting in 3D, sprite direction facing (front/back/side), weapon swing viewmodel animations per class.

### Long term (the end-state)
- Hosted public server (Render/Fly/Railway guide, or Dockerfile) so anyone can join one shared world.
- Seasonal leaderboards & events; daily quests.
- A Clash-Royale-style card skirmish mini-mode using the companion roster.
- Optional richer art packs (the atlas pipeline makes swapping/adding open-licensed art packs trivial).
- App-store wrappers (Capacitor/TWA) for Play Store / App Store distribution.

---

## 5. Architecture Map

```
web/                       ← the entire game client (no build step, no frameworks)
  index.html               ← shell: screens, HUD, touch controls, panels
  css/style.css            ← responsive dark-fantasy UI, glassmorphism, pixel font
  fonts/                   ← Press Start 2P (OFL) bundled for offline
  assets/atlas.png|json    ← 633-sprite texture atlas + coordinates (public-domain art)
  js/data/*.js             ← ALL game data ported from Rust (11 modules) + sprite mapping
  js/util.js               ← seeded RNG, math, event bus, storage
  js/assets.js             ← atlas loader with vector-art fallback
  js/audio.js              ← synthesized SFX + generative music (WebAudio)
  js/worldgen.js           ← procedural 30-floor generator, FOV, pathfinding
  js/entities.js           ← player/monsters/companions, stats, affixes, talents
  js/combat.js             ← damage model, 132-skill executor, loot & affix rolls
  js/ai.js                 ← monster AI, boss ability phases, ally AI
  js/systems.js            ← quests, achievements, factions, crafting, hunger, shops
  js/haven.js              ← base building, production, farming, blessings, sieges
  js/siege.js              ← playable wave-defense mode
  js/arena.js              ← real-time PvP (online rooms + offline bots)
  js/net.js                ← WebSocket client: chat, presence, arena, cloud saves, ranks
  js/render.js             ← 2D renderer: sprites, paper-doll hero, lighting, shake, minimap
  js/view3d.js             ← textured raycasting engine (FPP/TPP)
  js/input.js              ← keyboard/mouse/touch + look controls
  js/ui.js                 ← HUD, panels, chat, modals, toasts
  js/game.js               ← state, main loop, saves, mode routing
  js/main.js               ← boot, character creation, title screen
  sw.js + manifest         ← offline PWA (cache v4, 41 assets)
server/
  server.js                ← static hosting + WebSocket world (chat/presence/arena/saves/leaderboard)
PLAY_ONLINE.md             ← how to play/run/deploy
```

**How to run:** `cd server && npm install && npm start` → open `http://localhost:8080` (phones on the same Wi-Fi: `http://<your-PC-IP>:8080`; install via "Add to Home Screen"). Press **V** in the crypt to cycle top-down / third-person / first-person.

---

## 6. Licenses & Credits

- Game code: this repository (MIT, per the original project).
- Sprite art: *Dungeon Crawl Stone Soup* rltiles, public domain — `web/assets/LICENSE-art.txt`.
- Font: *Press Start 2P*, SIL Open Font License 1.1 — `web/fonts/LICENSE-fonts.txt`.
- Audio & music: 100% synthesized in code, no third-party assets.

---

*Last updated: 2026-09-02 · Branch head at this writing: `2b36cfa` (5 commits, ~26,000 lines added) · All test suites passing.*

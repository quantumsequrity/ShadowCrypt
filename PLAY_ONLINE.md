# ShadowCrypt Online — Play Guide

ShadowCrypt is now a **cross-platform action MMORPG** that runs in any browser — phone, tablet, or PC — **fully playable offline** and with online multiplayer when a server is available. The original terminal roguelike still lives in this repo (see `HOW_TO_PLAY.md`); this guide covers the web game in `web/`.

## ✨ What makes it special

- **Professional game art** — 633 hand-crafted pixel-art sprites from the legendary open-source roguelike *Dungeon Crawl Stone Soup* (public domain, see `web/assets/LICENSE-art.txt`): every one of the 148 monsters, 199 items, themed wall/floor tilesets for all 8 dungeon regions, spell icons, effects. Packed into a single texture atlas for instant loading, cached offline.
- **A paper-doll hero** — your character's sprite is composited live from equipment layers: equip a new sword, shield, helmet, armor, gloves or boots and you SEE it on your hero, in the HUD portrait, in top-down, and in third-person 3D.
- **Textured 3D dungeon crawling** — the first-person and third-person modes render real wall textures per theme via column-sampled raycasting, with distance lighting, torch bounce and billboard sprite monsters.

- **Three camera modes** — press **V** (or 🎥): classic **top-down**, cinematic **third-person** behind your hero, and full **first-person** with a weapon viewmodel and crosshair. In TPP/FPP, joystick-up walks forward and dragging the right side of the screen (or mouse-drag / Q & E) looks around — shooter-style controls.
- **Anime cel-shaded art** — heroes with spiky class-colored hair and big expressive eyes, bold-outlined sprite monsters in 16 body families (slimes wobble, bats flap, ghosts wave, dragons beat their wings), all drawn procedurally — zero downloaded assets.
- **Fully synthesized audio** — 35+ sound effects and adaptive generative music (dark ambient in the crypt, drums in the arena and sieges, a dread theme on boss floors), created live with WebAudio. No audio files.
- **Combat that feels good** — 120° melee arcs, crit knockback, a dodge-dash with i-frames (Shift/💨), screen shake, hit-flashes, and telegraphed boss abilities: dodge the glowing rings before the slam lands. Bosses enrage at half health.
- **Deep loot** — elite monsters (Frenzied, Vampiric, Colossal…) with auras and bonus drops; item affixes ("Flame Sword of the Leech") with procs, lifesteal and gold-find; locked golden chests opened by keys from mini-bosses; smashable urns and crates.
- **Talents** — every level grants 3 points to spend on Might, Vitality, Bulwark, Spirit or Agility.
- **Playable siege defense** — every 4 hours your haven can be attacked: fight waves *yourself* alongside your towers and bone walls, Clash-style, for gold, materials and XP.
- **Server leaderboards** — deepest-floor rankings across everyone on your server (Menu ▸ Ranks).

---

## Quick Start

### Play offline (no server, no install)

Any static file host works. The quickest local option:

```bash
cd web
python3 -m http.server 8000        # or: npx serve
# open http://localhost:8000
```

> Opening `index.html` directly with `file://` also works for a quick look, but the service worker (offline install) needs http(s).

### Play online (multiplayer)

```bash
cd server
npm install
npm start                          # PORT=8080 by default
# open http://localhost:8080
```

The server hosts the game **and** the multiplayer backend: global chat, co-op ghost presence in the crypt, real-time PvP arena rooms, and cloud saves. Deploy it to any Node host (Render, Fly, Railway, a VPS…) and share the URL — everyone who opens it plays together.

### Install as an app (mobile / tablet / desktop)

The game is a **PWA**. Open it in a browser and use *Add to Home Screen* (Android/iOS) or the install icon in the address bar (desktop Chrome/Edge). It then launches fullscreen like a native app and **works with no connection at all** — the whole game is cached on your device.

---

## The Four Ways to Play

### 🗡 The Crypt — action roguelike descent
The heart of the game: 30 procedurally generated floors across 8 themed areas (Dark Dungeon → Demon Realm), 148 enemy types, 12 bosses and mini-bosses, traps, chests, shrines, water and lava — everything from the original terminal game, now in real-time action combat with smooth movement, projectiles, skills, and companions fighting at your side.

- Move with **WASD/arrows**, the **virtual joystick**, or **tap a tile** to path there
- **Space / ⚔️** attacks (hold to auto-attack); **1–4** cast your skill slots
- **G / E / ✦** interacts: open doors and chests, use shrines, take stairs
- Death is no longer permadeath: you wake in your Haven, keeping gear and XP, losing 10% gold — the crypt reshuffles

When online, you see other players on your floor as blue ghosts and share a global chat.

### 🏰 The Haven — build your base
Clash-style base building: place and upgrade 11 building types on your haven grid. Gold Mines and Mana Wells **produce while you're away**, the Companion Den recruits from 33 companions, the Dark Forge and Apothecary unlock 226 crafting recipes, the Crypt Portal lets you start deeper descents, and Towers + Bone Walls defend against **shadow sieges** that strike every 8 hours.

### 🌾 The Farm — grow your supplies
Plant 7 crop types in real-time farm plots — water them for 2× speed, harvest food, potions and legendary fruit. Crops keep growing while you're offline.

### 💥 The Arena — real-time PvP
Mini-Militia-style deathmatch: joystick movement, hold-to-shoot, bombs, and power-ups (hearts, speed boots, shields, triple-shot, mega-bombs). Online you're matched into rooms of up to 6 players; offline (or in an empty room) training bots keep it hot.

---

## Progression

- **7 species** with 40+ subspecies (Humans, Elves, Dwarves, Fairies, Angels, Dragonians, Undead) — each with stat bonuses
- **6 classes** (Warrior, Mage, Rogue, Paladin, Ranger, Necromancer) → at level 10 choose one of **37 subclasses**, with advanced evolutions beyond
- **156 items** in 6 rarity tiers (Common → Mythic), 9 equipment slots
- **89 quests**, **34 achievements**, **16 factions** with reputation ranks
- **13 status effects**, hunger survival, and the Demon King waiting on floor 30

## Saves

Your hero saves automatically to your device (localStorage). When online, saves also sync to the server keyed to a private token — reconnect from the same browser and continue anywhere the server can see.

---

*May your blade stay sharp and your torches never dim.*

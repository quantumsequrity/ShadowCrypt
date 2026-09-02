'use strict';
/* ShadowCrypt Online — MMO-layer data: haven buildings, crops, arena. (New content for the web MMORPG.) */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

/* Haven buildings — Clash-of-Clans-style base. Costs in gold (g), shards (s = crypt shards).
 * produce: resource per hour. Upgrade cost scales by costMult^level. */
SC.DATA.buildings = {
  keep: {
    id: 'keep', name: 'Shadow Keep', icon: '🏰', size: 2, maxLevel: 10,
    desc: 'The heart of your haven. Its level caps every other building.',
    cost: { gold: 0 }, upgradeBase: { gold: 400 }, costMult: 1.9,
    unique: true, starter: true
  },
  goldMine: {
    id: 'goldMine', name: 'Gold Mine', icon: '⛏️', size: 1, maxLevel: 10,
    desc: 'Digs gold from the crypt walls over time.',
    cost: { gold: 120 }, upgradeBase: { gold: 200 }, costMult: 1.7,
    produce: { gold: 60 }, produceMult: 1.55, storeCap: 600
  },
  manaWell: {
    id: 'manaWell', name: 'Mana Well', icon: '🔮', size: 1, maxLevel: 10,
    desc: 'Condenses ambient shadow-mana. Collect to restore potions of mana… or bottle it.',
    cost: { gold: 150 }, upgradeBase: { gold: 240 }, costMult: 1.7,
    produce: { mana: 40 }, produceMult: 1.5, storeCap: 400
  },
  farmPlot: {
    id: 'farmPlot', name: 'Farm Plot', icon: '🌱', size: 1, maxLevel: 5,
    desc: 'Fertile crypt soil. Plant seeds, water them, harvest food and reagents.',
    cost: { gold: 100 }, upgradeBase: { gold: 180 }, costMult: 1.6,
    maxCount: 8
  },
  barracks: {
    id: 'barracks', name: 'Companion Den', icon: '🐺', size: 2, maxLevel: 8,
    desc: 'Recruit and train companions to fight beside you in the crypt.',
    cost: { gold: 300 }, upgradeBase: { gold: 350 }, costMult: 1.8,
    unique: true
  },
  forge: {
    id: 'forge', name: 'Dark Forge', icon: '⚒️', size: 2, maxLevel: 8,
    desc: 'Craft weapons, armor and trinkets from crypt materials.',
    cost: { gold: 250 }, upgradeBase: { gold: 300 }, costMult: 1.8,
    unique: true
  },
  apothecary: {
    id: 'apothecary', name: 'Apothecary', icon: '⚗️', size: 1, maxLevel: 8,
    desc: 'Brew potions and elixirs from harvested herbs.',
    cost: { gold: 220 }, upgradeBase: { gold: 260 }, costMult: 1.75,
    unique: true
  },
  wall: {
    id: 'wall', name: 'Bone Wall', icon: '🧱', size: 1, maxLevel: 6,
    desc: 'Defends your haven during shadow sieges.',
    cost: { gold: 30 }, upgradeBase: { gold: 50 }, costMult: 1.5,
    maxCount: 40
  },
  tower: {
    id: 'tower', name: 'Watch Tower', icon: '🗼', size: 1, maxLevel: 8,
    desc: 'Automatically repels shadow sieges — higher levels drive off stronger raids.',
    cost: { gold: 200 }, upgradeBase: { gold: 280 }, costMult: 1.7,
    maxCount: 6
  },
  shrineB: {
    id: 'shrineB', name: 'Ancient Shrine', icon: '⛩️', size: 1, maxLevel: 5,
    desc: 'Grants a daily blessing: bonus XP, luck, or protection.',
    cost: { gold: 500 }, upgradeBase: { gold: 600 }, costMult: 2.0,
    unique: true
  },
  portal: {
    id: 'portal', name: 'Crypt Portal', icon: '🌀', size: 1, maxLevel: 6,
    desc: 'Attunes to deeper floors — each level lets you start your descent 5 floors lower.',
    cost: { gold: 800 }, upgradeBase: { gold: 900 }, costMult: 2.1,
    unique: true
  }
};

/* Crops — farming layer. growMs = real time to mature (halved when watered).
 * yields item ids from the crypt item/material tables. */
SC.DATA.crops = {
  mossberry: {
    id: 'mossberry', name: 'Mossberry', icon: '🫐', seedName: 'Mossberry Seeds',
    cost: 10, growMs: 90000, yield: [{ id: 'apple', qty: 2 }], xp: 4,
    desc: 'A hardy berry that thrives in crypt gloom.'
  },
  cryptWheat: {
    id: 'cryptWheat', name: 'Crypt Wheat', icon: '🌾', seedName: 'Crypt Wheat Seeds',
    cost: 18, growMs: 180000, yield: [{ id: 'bread', qty: 2 }], xp: 8,
    desc: 'Pale wheat that needs no sun. Bakes into hearty bread.'
  },
  gloomshroom: {
    id: 'gloomshroom', name: 'Gloomshroom', icon: '🍄', seedName: 'Gloomshroom Spores',
    cost: 25, growMs: 240000, yield: [{ id: 'cheese', qty: 1 }, { id: 'meat', qty: 1 }], xp: 12,
    desc: 'Meaty fungus. Tastes better than it looks.'
  },
  bloodroot: {
    id: 'bloodroot', name: 'Bloodroot', icon: '🥀', seedName: 'Bloodroot Bulb',
    cost: 40, growMs: 420000, yield: [{ id: 'health_potion', qty: 1 }], xp: 20,
    desc: 'Crimson tuber used in healing draughts.'
  },
  manaLotus: {
    id: 'manaLotus', name: 'Mana Lotus', icon: '🪷', seedName: 'Mana Lotus Seed',
    cost: 55, growMs: 540000, yield: [{ id: 'mana_potion', qty: 1 }], xp: 26,
    desc: 'Blooms with condensed shadow-mana.'
  },
  goldenApple: {
    id: 'goldenApple', name: 'Golden Apple Tree', icon: '🍎', seedName: 'Golden Sapling',
    cost: 120, growMs: 900000, yield: [{ id: 'golden_apple', qty: 1 }], xp: 60,
    desc: 'A legendary fruit said to extend life itself.'
  },
  dragonFruit: {
    id: 'dragonFruit', name: 'Dragonfruit Vine', icon: '🐉', seedName: 'Dragonfruit Cutting',
    cost: 90, growMs: 720000, yield: [{ id: 'dragon_fruit', qty: 1 }], xp: 45,
    desc: 'Scaly fruit crackling with warmth.'
  }
};

/* Arena — Mini-Militia / BombSquad-style real-time PvP. */
SC.DATA.arena = {
  tickMs: 100,
  moveSpeed: 5.2,        // tiles per second
  projSpeed: 11,
  projDamage: 14,
  bombDamage: 42,
  bombRadius: 2.4,
  bombFuseMs: 1400,
  fireCooldownMs: 380,
  bombCooldownMs: 2600,
  respawnMs: 2500,
  matchSeconds: 120,
  maxPlayers: 6,
  hp: 100,
  powerups: [
    { id: 'heart', icon: '❤️', effect: 'heal', value: 35, weight: 3 },
    { id: 'boots', icon: '👟', effect: 'speed', value: 1.5, durMs: 6000, weight: 2 },
    { id: 'shield', icon: '🛡️', effect: 'shield', value: 0.5, durMs: 6000, weight: 2 },
    { id: 'triple', icon: '🔱', effect: 'triple', durMs: 7000, weight: 2 },
    { id: 'mega', icon: '💣', effect: 'megabomb', durMs: 8000, weight: 1 }
  ]
};

/* Daily blessings from the Ancient Shrine building */
SC.DATA.blessings = [
  { id: 'bl_xp', name: 'Blessing of Wisdom', desc: '+50% XP for this crypt run', icon: '📖' },
  { id: 'bl_luck', name: 'Blessing of Fortune', desc: 'Much better loot this run', icon: '🍀' },
  { id: 'bl_iron', name: 'Blessing of Iron', desc: '+25% defense this run', icon: '🛡️' },
  { id: 'bl_fury', name: 'Blessing of Fury', desc: '+20% attack this run', icon: '⚔️' }
];

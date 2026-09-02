'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// ============================================================================
// WORLD DATA — status effects, floor themes, spells, tiles, shrines, traps.
// Ported from src/main.rs (StatusEffect ~line 49, Tile ~line 1297,
// DungeonTheme ~line 1397, SPELL COMPENDIUM ~line 5680, trigger_trap ~line
// 7162, use_shrine ~line 7227), cross-referenced with core/src/world.rs,
// core/src/magic.rs and the README.md floor table.
// See SC.DATA.world_notes for porting decisions.
// ============================================================================

// ----------------------------------------------------------------------------
// STATUS EFFECTS — src/main.rs enum StatusEffect (13 variants), name()/color()
// verbatim. dmgPerTick is the player-facing tick (Player::tick_status_effects).
// ----------------------------------------------------------------------------
SC.DATA.statusEffects = [
  { id: 'poisoned',     name: 'Poisoned',     color: '#2ecc71', kind: 'debuff', dmgPerTick: 2, description: 'Toxins course through your veins, dealing 2 damage each turn (enemies take 3).' },
  { id: 'burning',      name: 'Burning',      color: '#e74c3c', kind: 'debuff', dmgPerTick: 3, description: 'Engulfed in flames, taking 3 damage each turn (enemies take 5).' },
  { id: 'frozen',       name: 'Frozen',       color: '#1abc9c', kind: 'debuff', description: 'Encased in ice — a frozen creature cannot act until it thaws.' },
  { id: 'bleeding',     name: 'Bleeding',     color: '#922b21', kind: 'debuff', dmgPerTick: 1, description: 'An open wound drains 1 HP each turn (enemies take 2).' },
  { id: 'stunned',      name: 'Stunned',      color: '#f1c40f', kind: 'debuff', description: 'Dazed and reeling — the stunned cannot act this turn.' },
  { id: 'blind',        name: 'Blind',        color: '#5d6d7e', kind: 'debuff', description: 'Vision fails; the darkness of the crypt closes in.' },
  { id: 'haste',        name: 'Haste',        color: '#3498db', kind: 'buff',   description: 'Supernatural speed quickens every movement.' },
  { id: 'shielded',     name: 'Shielded',     color: '#ecf0f1', kind: 'buff',   description: 'A protective barrier grants +10 defense and absorbs the next enemy attack entirely, then shatters.' },
  { id: 'regenerating', name: 'Regenerating', color: '#d35ded', kind: 'buff',   description: 'Wounds knit closed, restoring 3 HP each turn.' },
  { id: 'strengthened', name: 'Strengthened', color: '#f1c40f', kind: 'buff',   description: 'Empowered muscles multiply attack power by 1.5x.' },
  { id: 'weakened',     name: 'Weakened',     color: '#76448a', kind: 'debuff', description: 'Sapped strength halves attack power (0.5x).' },
  { id: 'invisible',    name: 'Invisible',    color: '#95a5a6', kind: 'buff',   description: 'Unseen by enemies — they cannot track or attack you.' },
  { id: 'confused',     name: 'Confused',     color: '#b7950b', kind: 'debuff', description: 'A scrambled mind — 30% chance each turn to stumble in a random direction.' }
];

// ----------------------------------------------------------------------------
// FLOOR THEMES — src/main.rs / core/src/world.rs enum DungeonTheme,
// from_level() ranges and name() verbatim; descriptions from README.md
// floor table. Palettes invented (see world_notes).
// ----------------------------------------------------------------------------
SC.DATA.floorThemes = [
  {
    id: 'dungeon', name: 'Dark Dungeon', floors: [1, 4],
    description: 'Classic stone corridors filled with vermin and goblins',
    palette: { wall: '#3a3f44', floor: '#23272b', accent: '#95a5a6' },
    enemyTier: 1, boss: 'bossGoblinKing'
  },
  {
    id: 'caves', name: 'Twisted Caves', floors: [5, 8],
    description: 'Natural caverns with trolls, orcs, and elemental creatures',
    palette: { wall: '#4a3b2a', floor: '#2b241a', accent: '#b7950b' },
    enemyTier: 2, boss: 'bossOrcWarlord'
  },
  {
    id: 'crypt', name: 'Haunted Crypt', floors: [9, 12],
    description: 'Undead-infested tombs with ghosts, vampires, and wraiths',
    palette: { wall: '#3d3450', floor: '#241f30', accent: '#76448a' },
    enemyTier: 3, boss: 'bossVampireLord'
  },
  {
    id: 'forest', name: 'Cursed Forest', floors: [13, 16],
    description: 'Underground forest with wolves, ents, and forest spirits',
    palette: { wall: '#2a3d2a', floor: '#1a2b1a', accent: '#1e8449' },
    enemyTier: 4, boss: 'bossForestGuardian'
  },
  {
    id: 'frozen', name: 'Frozen Caverns', floors: [17, 20],
    description: 'Ice-covered tunnels with frost giants and yetis',
    palette: { wall: '#2e4a5a', floor: '#1b2f3a', accent: '#1abc9c' },
    enemyTier: 5, boss: 'bossIceDragon'
  },
  {
    id: 'volcanic', name: 'Volcanic Depths', floors: [21, 24],
    description: 'Lava-filled chambers with fire elementals and hellhounds',
    palette: { wall: '#4a2520', floor: '#2b1512', accent: '#e74c3c' },
    enemyTier: 6, boss: 'ancientDragon'
  },
  {
    id: 'ruins', name: 'Ancient Ruins', floors: [25, 28],
    description: 'Forgotten temples guarded by golems and sphinxes',
    palette: { wall: '#4d4633', floor: '#2e2a1e', accent: '#f1c40f' },
    enemyTier: 7, boss: 'colossus'
  },
  {
    id: 'demon', name: 'Demon Realm', floors: [29, 30],
    description: 'The final hellish domain of the Demon King',
    palette: { wall: '#451f2c', floor: '#260f16', accent: '#922b21' },
    enemyTier: 8, boss: 'bossDemonKing'
  }
];

// ----------------------------------------------------------------------------
// SPELLS — src/main.rs SPELL COMPENDIUM (create_*_spells, ids 1-150 in
// source order). Spell::new(id, name, school, tier, mana, cd, dmg, range,
// area, cast, desc): manaCost/damage/description verbatim; element = magic
// school; aoe = area radius when > 0; damage omitted when 0.
// ----------------------------------------------------------------------------
SC.DATA.spells = [
  // Fire (1-10)
  { id: 'ember',              name: 'Ember',                manaCost: 5,   damage: 8,   element: 'fire',        description: 'A small flame projectile' },
  { id: 'flameBolt',          name: 'Flame Bolt',           manaCost: 12,  damage: 18,  element: 'fire',        description: 'Concentrated bolt of fire' },
  { id: 'burningHands',       name: 'Burning Hands',        manaCost: 15,  damage: 15,  element: 'fire',        aoe: 2,  description: 'Cone of flames' },
  { id: 'fireball',           name: 'Fireball',             manaCost: 25,  damage: 35,  element: 'fire',        aoe: 3,  description: 'Explosive fireball' },
  { id: 'scorch',             name: 'Scorch',               manaCost: 20,  damage: 25,  element: 'fire',        description: 'Intense heat ignites target' },
  { id: 'fireWall',           name: 'Fire Wall',            manaCost: 35,  damage: 20,  element: 'fire',        aoe: 4,  description: 'Wall of flames' },
  { id: 'immolate',           name: 'Immolate',             manaCost: 45,  damage: 50,  element: 'fire',        description: 'Engulfs target in flames' },
  { id: 'meteorStrike',       name: 'Meteor Strike',        manaCost: 80,  damage: 120, element: 'fire',        aoe: 5,  description: 'Meteor from the sky' },
  { id: 'inferno',            name: 'Inferno',              manaCost: 100, damage: 150, element: 'fire',        aoe: 6,  description: 'Massive firestorm' },
  { id: 'phoenixFlame',       name: 'Phoenix Flame',        manaCost: 150, damage: 200, element: 'fire',        aoe: 8,  description: 'Legendary fire that resurrects' },
  // Ice (11-20)
  { id: 'frostTouch',         name: 'Frost Touch',          manaCost: 5,   damage: 6,   element: 'ice',         description: 'Chilling touch' },
  { id: 'iceShard',           name: 'Ice Shard',            manaCost: 10,  damage: 15,  element: 'ice',         description: 'Sharp ice projectile' },
  { id: 'frostNova',          name: 'Frost Nova',           manaCost: 25,  damage: 25,  element: 'ice',         aoe: 4,  description: 'Frost explosion around caster' },
  { id: 'iceSpear',           name: 'Ice Spear',            manaCost: 22,  damage: 30,  element: 'ice',         description: 'Piercing ice lance' },
  { id: 'frozenArmor',        name: 'Frozen Armor',         manaCost: 30,  element: 'ice',         description: 'Protective ice shell' },
  { id: 'blizzard',           name: 'Blizzard',             manaCost: 50,  damage: 40,  element: 'ice',         aoe: 6,  description: 'Raging snowstorm' },
  { id: 'flashFreeze',        name: 'Flash Freeze',         manaCost: 60,  damage: 60,  element: 'ice',         description: 'Instantly freeze target' },
  { id: 'glacialSpike',       name: 'Glacial Spike',        manaCost: 75,  damage: 100, element: 'ice',         aoe: 2,  description: 'Massive ice spike' },
  { id: 'iceAge',             name: 'Ice Age',              manaCost: 120, damage: 180, element: 'ice',         aoe: 10, description: 'Freezes entire battlefield' },
  { id: 'absoluteZero',       name: 'Absolute Zero',        manaCost: 160, damage: 250, element: 'ice',         aoe: 4,  description: 'Ultimate cold stops all' },
  // Lightning (21-30)
  { id: 'spark',              name: 'Spark',                manaCost: 4,   damage: 7,   element: 'lightning',   description: 'Small discharge' },
  { id: 'shock',              name: 'Shock',                manaCost: 10,  damage: 16,  element: 'lightning',   description: 'Stunning shock' },
  { id: 'lightningBolt',      name: 'Lightning Bolt',       manaCost: 22,  damage: 35,  element: 'lightning',   description: 'Classic lightning strike' },
  { id: 'chainLightning',     name: 'Chain Lightning',      manaCost: 35,  damage: 40,  element: 'lightning',   description: 'Bounces between enemies' },
  { id: 'thunderclap',        name: 'Thunderclap',          manaCost: 38,  damage: 35,  element: 'lightning',   aoe: 5,  description: 'Explosive thunder' },
  { id: 'staticField',        name: 'Static Field',         manaCost: 45,  damage: 30,  element: 'lightning',   aoe: 4,  description: 'Electrified zone' },
  { id: 'ballLightning',      name: 'Ball Lightning',       manaCost: 55,  damage: 70,  element: 'lightning',   aoe: 3,  description: 'Floating electric orb' },
  { id: 'thunderstorm',       name: 'Thunderstorm',         manaCost: 80,  damage: 90,  element: 'lightning',   aoe: 8,  description: 'Multiple lightning strikes' },
  { id: 'rideTheLightning',   name: 'Ride the Lightning',   manaCost: 90,  damage: 110, element: 'lightning',   aoe: 2,  description: 'Become lightning' },
  { id: 'mjolnirsWrath',      name: "Mjolnir's Wrath",      manaCost: 170, damage: 280, element: 'lightning',   aoe: 6,  description: 'Divine lightning' },
  // Earth (31-40)
  { id: 'stoneThrow',         name: 'Stone Throw',          manaCost: 5,   damage: 9,   element: 'earth',       description: 'Hurl a rock' },
  { id: 'tremor',             name: 'Tremor',               manaCost: 12,  damage: 14,  element: 'earth',       aoe: 3,  description: 'Minor earthquake' },
  { id: 'stoneSkin',          name: 'Stone Skin',           manaCost: 15,  element: 'earth',       description: 'Harden skin' },
  { id: 'rockSpike',          name: 'Rock Spike',           manaCost: 22,  damage: 32,  element: 'earth',       description: 'Sharp stone eruption' },
  { id: 'earthWall',          name: 'Earth Wall',           manaCost: 30,  element: 'earth',       aoe: 3,  description: 'Protective barrier' },
  { id: 'quicksand',          name: 'Quicksand',            manaCost: 35,  damage: 25,  element: 'earth',       aoe: 2,  description: 'Trapping sands' },
  { id: 'boulderCrush',       name: 'Boulder Crush',        manaCost: 45,  damage: 65,  element: 'earth',       aoe: 2,  description: 'Massive boulder' },
  { id: 'earthquake',         name: 'Earthquake',           manaCost: 75,  damage: 85,  element: 'earth',       aoe: 10, description: 'Massive tremor' },
  { id: 'meteorSwarm',        name: 'Meteor Swarm',         manaCost: 130, damage: 160, element: 'earth',       aoe: 8,  description: 'Rain of meteors' },
  { id: 'worldBreaker',       name: 'World Breaker',        manaCost: 180, damage: 300, element: 'earth',       aoe: 12, description: 'Cataclysmic event' },
  // Wind (41-50)
  { id: 'gust',               name: 'Gust',                 manaCost: 4,   damage: 6,   element: 'wind',        description: 'Blast of air' },
  { id: 'windSlash',          name: 'Wind Slash',           manaCost: 10,  damage: 18,  element: 'wind',        description: 'Cutting air blade' },
  { id: 'tailwind',           name: 'Tailwind',             manaCost: 12,  element: 'wind',        aoe: 3,  description: 'Speed boost' },
  { id: 'cyclone',            name: 'Cyclone',              manaCost: 25,  damage: 28,  element: 'wind',        aoe: 4,  description: 'Swirling vortex' },
  { id: 'airShield',          name: 'Air Shield',           manaCost: 20,  element: 'wind',        description: 'Deflecting barrier' },
  { id: 'whirlwind',          name: 'Whirlwind',            manaCost: 35,  damage: 40,  element: 'wind',        aoe: 5,  description: 'Spinning wind' },
  { id: 'vacuum',             name: 'Vacuum',               manaCost: 45,  damage: 50,  element: 'wind',        aoe: 3,  description: 'Remove air' },
  { id: 'tornado',            name: 'Tornado',              manaCost: 80,  damage: 100, element: 'wind',        aoe: 6,  description: 'Devastating twister' },
  { id: 'flight',             name: 'Flight',               manaCost: 50,  element: 'wind',        description: 'Grants flight' },
  { id: 'hurricane',          name: 'Hurricane',            manaCost: 160, damage: 220, element: 'wind',        aoe: 15, description: 'Ultimate storm' },
  // Water (51-60)
  { id: 'waterJet',           name: 'Water Jet',            manaCost: 5,   damage: 7,   element: 'water',       description: 'Pressurized water' },
  { id: 'healingSpring',      name: 'Healing Spring',       manaCost: 15,  element: 'water',       aoe: 2,  description: 'Restorative waters' },
  { id: 'tidalWave',          name: 'Tidal Wave',           manaCost: 25,  damage: 30,  element: 'water',       aoe: 4,  description: 'Crashing wave' },
  { id: 'purify',             name: 'Purify',               manaCost: 20,  element: 'water',       description: 'Cleanse ailments' },
  { id: 'waterBreathing',     name: 'Water Breathing',      manaCost: 10,  element: 'water',       description: 'Breathe underwater' },
  { id: 'hydroPump',          name: 'Hydro Pump',           manaCost: 45,  damage: 65,  element: 'water',       description: 'High pressure cannon' },
  { id: 'healingRain',        name: 'Healing Rain',         manaCost: 50,  element: 'water',       aoe: 6,  description: 'Widespread healing' },
  { id: 'maelstrom',          name: 'Maelstrom',            manaCost: 80,  damage: 95,  element: 'water',       aoe: 7,  description: 'Churning whirlpool' },
  { id: 'tsunami',            name: 'Tsunami',              manaCost: 130, damage: 170, element: 'water',       aoe: 10, description: 'Massive tidal wave' },
  { id: 'leviathansEmbrace',  name: "Leviathan's Embrace",  manaCost: 170, damage: 240, element: 'water',       aoe: 12, description: "Ocean's power" },
  // Light (61-70)
  { id: 'holyLight',          name: 'Holy Light',           manaCost: 8,   element: 'light',       description: 'Basic healing' },
  { id: 'smite',              name: 'Smite',                manaCost: 12,  damage: 20,  element: 'light',       description: 'Divine damage' },
  { id: 'blessing',           name: 'Blessing',             manaCost: 15,  element: 'light',       description: 'Divine protection' },
  { id: 'purifyingLight',     name: 'Purifying Light',      manaCost: 25,  damage: 35,  element: 'light',       description: 'Damages evil heals allies' },
  { id: 'divineShield',       name: 'Divine Shield',        manaCost: 40,  element: 'light',       description: 'Invulnerability' },
  { id: 'consecration',       name: 'Consecration',         manaCost: 35,  damage: 25,  element: 'light',       aoe: 5,  description: 'Holy ground' },
  { id: 'resurrection',       name: 'Resurrection',         manaCost: 100, element: 'light',       description: 'Bring back the dead' },
  { id: 'solarFlare',         name: 'Solar Flare',          manaCost: 60,  damage: 80,  element: 'light',       aoe: 4,  description: 'Blinding light' },
  { id: 'judgment',           name: 'Judgment',             manaCost: 100, damage: 150, element: 'light',       description: 'Divine judgment' },
  { id: 'avatarOfLight',      name: 'Avatar of Light',      manaCost: 180, damage: 200, element: 'light',       aoe: 10, description: 'Divine avatar' },
  // Dark (71-80)
  { id: 'shadowBolt',         name: 'Shadow Bolt',          manaCost: 6,   damage: 10,  element: 'dark',        description: 'Bolt of darkness' },
  { id: 'lifeDrain',          name: 'Life Drain',           manaCost: 15,  damage: 18,  element: 'dark',        description: 'Steal life force' },
  { id: 'curse',              name: 'Curse',                manaCost: 12,  element: 'dark',        description: 'Weaken target' },
  { id: 'fear',               name: 'Fear',                 manaCost: 25,  element: 'dark',        description: 'Terrify enemies' },
  { id: 'shadowStep',         name: 'Shadow Step',          manaCost: 20,  element: 'dark',        description: 'Teleport through shadows' },
  { id: 'corruption',         name: 'Corruption',           manaCost: 35,  damage: 40,  element: 'dark',        description: 'Corrupt target' },
  { id: 'darkPact',           name: 'Dark Pact',            manaCost: 0,   damage: 80,  element: 'dark',        description: 'Sacrifice HP for damage' },
  { id: 'soulRend',           name: 'Soul Rend',            manaCost: 60,  damage: 90,  element: 'dark',        description: 'Tear at the soul' },
  { id: 'voidZone',           name: 'Void Zone',            manaCost: 100, damage: 120, element: 'dark',        aoe: 6,  description: 'Area of darkness' },
  { id: 'apocalypse',         name: 'Apocalypse',           manaCost: 200, damage: 350, element: 'dark',        aoe: 10, description: 'Ultimate destruction' },
  // Arcane (81-90)
  { id: 'arcaneBolt',         name: 'Arcane Bolt',          manaCost: 5,   damage: 9,   element: 'arcane',      description: 'Pure magical energy' },
  { id: 'manaShield',         name: 'Mana Shield',          manaCost: 20,  element: 'arcane',      description: 'Shield from mana' },
  { id: 'dispelMagic',        name: 'Dispel Magic',         manaCost: 25,  element: 'arcane',      description: 'Remove magical effects' },
  { id: 'arcaneMissiles',     name: 'Arcane Missiles',      manaCost: 28,  damage: 45,  element: 'arcane',      description: 'Multiple missiles' },
  { id: 'counterspell',       name: 'Counterspell',         manaCost: 30,  element: 'arcane',      description: 'Interrupt and silence' },
  { id: 'arcaneExplosion',    name: 'Arcane Explosion',     manaCost: 40,  damage: 55,  element: 'arcane',      aoe: 5,  description: 'Arcane burst' },
  { id: 'spellSteal',         name: 'Spell Steal',          manaCost: 45,  element: 'arcane',      description: 'Steal enemy buffs' },
  { id: 'arcaneTorrent',      name: 'Arcane Torrent',       manaCost: 60,  damage: 85,  element: 'arcane',      aoe: 3,  description: 'Stream of power' },
  { id: 'prismaticSpray',     name: 'Prismatic Spray',      manaCost: 100, damage: 140, element: 'arcane',      aoe: 6,  description: 'Multi-element burst' },
  { id: 'arcaneSingularity',  name: 'Arcane Singularity',   manaCost: 180, damage: 280, element: 'arcane',      aoe: 8,  description: 'Magical collapse' },
  // Nature (91-100)
  { id: 'thorn',              name: 'Thorn',                manaCost: 4,   damage: 8,   element: 'nature',      description: 'Sharp thorn' },
  { id: 'rejuvenation',       name: 'Rejuvenation',         manaCost: 15,  element: 'nature',      description: 'Healing over time' },
  { id: 'entangle',           name: 'Entangle',             manaCost: 12,  damage: 10,  element: 'nature',      aoe: 3,  description: 'Vines hold enemies' },
  { id: 'poisonSpores',       name: 'Poison Spores',        manaCost: 22,  damage: 20,  element: 'nature',      aoe: 3,  description: 'Toxic cloud' },
  { id: 'summonBeast',        name: 'Summon Beast',         manaCost: 40,  element: 'nature',      description: 'Call wild beast' },
  { id: 'barkskin',           name: 'Barkskin',             manaCost: 25,  element: 'nature',      description: 'Tough bark armor' },
  { id: 'wildGrowth',         name: 'Wild Growth',          manaCost: 50,  element: 'nature',      aoe: 5,  description: 'Area heal' },
  { id: 'swarm',              name: 'Swarm',                manaCost: 55,  damage: 70,  element: 'nature',      aoe: 4,  description: 'Insect swarm' },
  { id: 'wrathOfNature',      name: 'Wrath of Nature',      manaCost: 100, damage: 150, element: 'nature',      aoe: 8,  description: "Nature's fury" },
  { id: 'avatarOfNature',     name: 'Avatar of Nature',     manaCost: 180, damage: 200, element: 'nature',      aoe: 10, description: 'Become one with nature' },
  // Blood (101-110)
  { id: 'bloodBolt',          name: 'Blood Bolt',           manaCost: 0,   damage: 12,  element: 'blood',       description: 'Costs HP not mana' },
  { id: 'bloodShield',        name: 'Blood Shield',         manaCost: 0,   element: 'blood',       description: 'Shield from blood' },
  { id: 'sanguineStrike',     name: 'Sanguine Strike',      manaCost: 15,  damage: 35,  element: 'blood',       description: 'Attack that heals' },
  { id: 'bloodBoil',          name: 'Blood Boil',           manaCost: 20,  damage: 40,  element: 'blood',       description: 'Boil enemy blood' },
  { id: 'transfusion',        name: 'Transfusion',          manaCost: 0,   element: 'blood',       description: 'Transfer HP to ally' },
  { id: 'hemorrhage',         name: 'Hemorrhage',           manaCost: 35,  damage: 55,  element: 'blood',       description: 'Severe bleeding' },
  { id: 'bloodFrenzy',        name: 'Blood Frenzy',         manaCost: 0,   element: 'blood',       description: 'Sacrifice HP for power' },
  { id: 'exsanguinate',       name: 'Exsanguinate',         manaCost: 60,  damage: 120, element: 'blood',       description: 'Drain all blood' },
  { id: 'bloodNova',          name: 'Blood Nova',           manaCost: 0,   damage: 150, element: 'blood',       aoe: 6,  description: 'Explosive blood' },
  { id: 'crimsonApocalypse',  name: 'Crimson Apocalypse',   manaCost: 100, damage: 300, element: 'blood',       aoe: 8,  description: 'Ultimate blood magic' },
  // Necromancy (111-120)
  { id: 'touchOfDeath',       name: 'Touch of Death',       manaCost: 8,   damage: 12,  element: 'necromancy',  description: 'Deathly touch' },
  { id: 'raiseSkeleton',      name: 'Raise Skeleton',       manaCost: 25,  element: 'necromancy',  description: 'Raise skeleton minion' },
  { id: 'boneArmor',          name: 'Bone Armor',           manaCost: 20,  element: 'necromancy',  description: 'Shield of bones' },
  { id: 'deathCoil',          name: 'Death Coil',           manaCost: 25,  damage: 35,  element: 'necromancy',  description: 'Coil of death' },
  { id: 'corpseExplosion',    name: 'Corpse Explosion',     manaCost: 30,  damage: 60,  element: 'necromancy',  aoe: 4,  description: 'Detonate corpses' },
  { id: 'armyOfDead',         name: 'Army of Dead',         manaCost: 80,  element: 'necromancy',  description: 'Skeleton army' },
  { id: 'deathGrip',          name: 'Death Grip',           manaCost: 35,  damage: 40,  element: 'necromancy',  description: 'Pull and damage' },
  { id: 'plague',             name: 'Plague',               manaCost: 50,  damage: 30,  element: 'necromancy',  aoe: 5,  description: 'Spreading disease' },
  { id: 'lichform',           name: 'Lichform',             manaCost: 150, element: 'necromancy',  description: 'Become a lich' },
  { id: 'apocalypseOfUndeath', name: 'Apocalypse of Undeath', manaCost: 200, damage: 250, element: 'necromancy', aoe: 10, description: 'Ultimate necromancy' },
  // Time (121-130)
  { id: 'slow',               name: 'Slow',                 manaCost: 10,  element: 'time',        description: 'Slow target' },
  { id: 'haste',              name: 'Haste',                manaCost: 20,  element: 'time',        description: 'Speed up ally' },
  { id: 'timeWarp',           name: 'Time Warp',            manaCost: 30,  element: 'time',        aoe: 4,  description: 'Area haste' },
  { id: 'temporalShield',     name: 'Temporal Shield',      manaCost: 40,  element: 'time',        description: 'Rewinds damage' },
  { id: 'age',                name: 'Age',                  manaCost: 35,  damage: 50,  element: 'time',        description: 'Rapidly age target' },
  { id: 'rewind',             name: 'Rewind',               manaCost: 60,  element: 'time',        description: 'Rewind damage' },
  { id: 'temporalStasis',     name: 'Temporal Stasis',      manaCost: 70,  element: 'time',        description: 'Freeze in time' },
  { id: 'glimpseOfEternity',  name: 'Glimpse of Eternity',  manaCost: 90,  damage: 100, element: 'time',        description: 'Show their death' },
  { id: 'timeStop',           name: 'Time Stop',            manaCost: 150, element: 'time',        description: 'Stop time' },
  { id: 'temporalParadox',    name: 'Temporal Paradox',     manaCost: 200, damage: 300, element: 'time',        aoe: 8,  description: 'Create time paradox' },
  // Space (131-140)
  { id: 'blink',              name: 'Blink',                manaCost: 10,  element: 'space',       description: 'Short teleport' },
  { id: 'spatialRift',        name: 'Spatial Rift',         manaCost: 15,  damage: 20,  element: 'space',       aoe: 2,  description: 'Tear in space' },
  { id: 'teleport',           name: 'Teleport',             manaCost: 30,  element: 'space',       description: 'Long range teleport' },
  { id: 'dimensionalAnchor',  name: 'Dimensional Anchor',   manaCost: 25,  element: 'space',       description: 'Prevent teleportation' },
  { id: 'gravityWell',        name: 'Gravity Well',         manaCost: 40,  damage: 45,  element: 'space',       aoe: 4,  description: 'Crushing gravity' },
  { id: 'phaseShift',         name: 'Phase Shift',          manaCost: 35,  element: 'space',       description: 'Become intangible' },
  { id: 'wormhole',           name: 'Wormhole',             manaCost: 60,  element: 'space',       description: 'Create portal' },
  { id: 'banish',             name: 'Banish',               manaCost: 80,  damage: 100, element: 'space',       description: 'Send to another dimension' },
  { id: 'dimensionalCollapse', name: 'Dimensional Collapse', manaCost: 140, damage: 200, element: 'space',      aoe: 6,  description: 'Collapse space' },
  { id: 'realityTear',        name: 'Reality Tear',         manaCost: 200, damage: 350, element: 'space',       aoe: 10, description: 'Tear reality' },
  // Enchantment (141-150)
  { id: 'minorEnchant',       name: 'Minor Enchant',        manaCost: 10,  element: 'enchantment', description: 'Small buff' },
  { id: 'charm',              name: 'Charm',                manaCost: 20,  element: 'enchantment', description: 'Charm enemy' },
  { id: 'empower',            name: 'Empower',              manaCost: 30,  element: 'enchantment', description: 'Boost stats' },
  { id: 'enchantWeapon',      name: 'Enchant Weapon',       manaCost: 35,  element: 'enchantment', description: 'Magic weapon' },
  { id: 'massCharm',          name: 'Mass Charm',           manaCost: 60,  element: 'enchantment', aoe: 4,  description: 'Charm multiple' },
  { id: 'dominate',           name: 'Dominate',             manaCost: 70,  element: 'enchantment', description: 'Control enemy' },
  { id: 'heroism',            name: 'Heroism',              manaCost: 50,  element: 'enchantment', aoe: 5,  description: 'Inspire allies' },
  { id: 'polymorph',          name: 'Polymorph',            manaCost: 80,  element: 'enchantment', description: 'Transform enemy' },
  { id: 'powerWordStun',      name: 'Power Word: Stun',     manaCost: 100, damage: 80,  element: 'enchantment', description: 'Instant stun' },
  { id: 'absoluteCommand',    name: 'Absolute Command',     manaCost: 180, element: 'enchantment', aoe: 8,  description: 'Control all minds' }
];

// ----------------------------------------------------------------------------
// TILES — src/main.rs / core/src/world.rs enum Tile. glyph() and walkable()
// verbatim (all 19 variants ported; contract's 13 core ids plus source extras).
// ----------------------------------------------------------------------------
SC.DATA.tiles = [
  { id: 'wall',         glyph: '#',  passable: false, description: 'Solid stone wall. Blocks movement and line of sight.' },
  { id: 'floor',        glyph: '.',  passable: true,  description: 'Bare dungeon floor.' },
  { id: 'doorClosed',   glyph: '+',  passable: false, description: 'A closed door. Blocks sight until opened by bumping into it.' },
  { id: 'doorOpen',     glyph: "'",  passable: true,  description: 'An open doorway.' },
  { id: 'stairsDown',   glyph: '>',  passable: true,  description: 'Stairs descending to the next floor.', effect: 'descend' },
  { id: 'stairsUp',     glyph: '<',  passable: true,  description: 'Stairs ascending to the previous floor (present on floors 2+).', effect: 'ascend' },
  { id: 'trap',         glyph: '^',  passable: true,  description: 'A hidden mechanism. Stepping on it triggers one of five random trap effects, then it becomes disarmed.', effect: 'triggerRandomTrap' },
  { id: 'disarmedTrap', glyph: '_',  passable: true,  description: 'A sprung trap, now harmless.' },
  { id: 'water',        glyph: '~',  passable: true,  description: 'A pool of dark water. Found in dungeon and cave themes.' },
  { id: 'lava',         glyph: '~',  passable: false, description: 'Molten rock in volcanic and demon floors. Contact burns for 5 + floor damage and sets you burning for 3 turns.', effect: 'burning' },
  { id: 'chest',        glyph: '=',  passable: false, description: 'A treasure chest. Bump to open: 1-3 items (30% chance of upgraded rarity) plus 10-50 gold per floor level.', effect: 'openChest' },
  { id: 'openChest',    glyph: '-',  passable: true,  description: 'An emptied chest.' },
  { id: 'shrine',       glyph: '&',  passable: false, description: 'A glowing shrine. Bump to receive one of six random blessings; it then goes dark.', effect: 'randomBlessing' },
  { id: 'usedShrine',   glyph: '.',  passable: true,  description: 'A spent shrine, its magic exhausted.' },
  { id: 'pillar',       glyph: 'O',  passable: false, description: 'A stone pillar. Blocks movement and line of sight; found in large rooms.' },
  { id: 'grass',        glyph: '"',  passable: true,  description: 'Underground grass, the floor of the Cursed Forest.' },
  { id: 'ice',          glyph: '.',  passable: true,  description: 'Slick ice, the floor of the Frozen Caverns.' },
  { id: 'sand',         glyph: '.',  passable: true,  description: 'Ancient sand, the floor of the Ancient Ruins.' },
  { id: 'bossGate',     glyph: '8',  passable: true,  description: 'The gate beyond the boss chamber, replacing the stairs on boss floors (5, 10, 15, 20, 25, 30). The way onward opens only when the boss falls.', effect: 'bossFight' }
];

// ----------------------------------------------------------------------------
// SHRINES — src/main.rs use_shrine(): six equally likely blessings,
// messages and values verbatim.
// ----------------------------------------------------------------------------
SC.DATA.shrines = [
  { id: 'restoration', name: 'Shrine of Restoration', description: 'Fully healed!',   effect: 'Restores HP and mana to their maximums.' },
  { id: 'vitality',    name: 'Shrine of Vitality',    description: '+10 Max HP!',     effect: 'Permanently raises max HP by 10 (and heals 10).' },
  { id: 'power',       name: 'Shrine of Power',       description: '+3 Attack!',      effect: 'Permanently raises base attack by 3.' },
  { id: 'protection',  name: 'Shrine of Protection',  description: '+2 Defense!',     effect: 'Permanently raises base defense by 2.' },
  { id: 'wisdom',      name: 'Shrine of Wisdom',      description: '+15 Max Mana!',   effect: 'Permanently raises max mana by 15 (and restores 15).' },
  { id: 'experience',  name: 'Shrine of Experience',  description: '+XP!',            effect: 'Grants 50 XP per dungeon floor (50 x floor).' }
];

// ----------------------------------------------------------------------------
// TRAPS — src/main.rs trigger_trap(): five equally likely trap types,
// damage formulas and status durations verbatim.
// ----------------------------------------------------------------------------
SC.DATA.traps = [
  { id: 'spike',      name: 'Spike Trap',       damageRange: [5, 20],           description: 'Iron spikes burst from the floor for 5 + floor/2 damage (5 on floor 1 up to 20 on floor 30).' },
  { id: 'poisonDart', name: 'Poison Dart Trap', damage: 0, effect: 'poisoned',  description: 'A dart coated in venom - poisons you for 5 turns.' },
  { id: 'teleport',   name: 'Teleport Trap',    damage: 0,                      description: 'Arcane runes flare and transport you to a random walkable spot on the floor.' },
  { id: 'flash',      name: 'Flash Trap',       damage: 0, effect: 'blind',     description: 'A blinding burst of light - you are blinded for 10 turns.' },
  { id: 'alarm',      name: 'Alarm Trap',       damage: 0,                      description: 'A shrieking alarm summons 2 enemies from the current floor pool to spots within 2 tiles.' }
];

// ----------------------------------------------------------------------------
// PORTING NOTES
// ----------------------------------------------------------------------------
SC.DATA.world_notes = [
  "statusEffects: all 13 variants of src/main.rs enum StatusEffect, name()/color() verbatim (crossterm -> hex per the agreed table). No berserk/lucky/torchlight effects exist in the source StatusEffect enum (Berserk/Luck appear only as potions, skills and enchants). dmgPerTick uses the player-facing tick (Player::tick_status_effects: poison 2, burn 3, bleed 1, regeneration heals 3/turn); the enemy tick differs (poison 3, burn 5, bleed 2) and is noted in each description.",
  "statusEffects mechanics from source: shielded grants +10 defense and fully absorbs one enemy attack then is removed; strengthened multiplies total attack x1.5; weakened x0.5; frozen/stunned creatures skip their turn; confused gives a 30% chance to move randomly; invisible makes enemies ignore the player. Enemy-inflicted statuses last 5 turns.",
  "floorThemes: ranges, names and floor tiles verbatim from DungeonTheme (main.rs ~1397 / core/src/world.rs), matching the authoritative README.md floor table. Theme slugs shortened per contract (dungeon, caves, crypt, forest, frozen, volcanic, ruins, demon). enemyTier 1-8 follows the README 'Enemies by Theme' tier numbering.",
  "floorThemes palettes: INVENTED - the terminal source has only per-tile crossterm colors, so wall/floor/accent hex values are new, chosen for a dark dungeon aesthetic with accents echoing each theme's source tile colors (e.g. frozen uses Cyan #1abc9c, volcanic Red #e74c3c).",
  "floorThemes bosses: boss ids for dungeon/caves/crypt/forest/frozen/demon are the exact SC.DATA.bosses ids from data_enemies.js, paired per the README Boss Floors table (each boss caps its theme's tier: Goblin King floor 5, Orc Warlord 10, Vampire Lord 15, Forest Guardian 20, Ice Dragon 25, Demon King 30). Volcanic Depths and Ancient Ruins have no entry in SC.DATA.bosses, so they use the expanded-roster boss encounters from SC.DATA.enemies: ancientDragon (bossFloor 22) and colossus (bossFloor 27); other expanded encounters (dwarfKing 12, archfey 18, archDemon 28) remain documented in data_enemies.js.",
  "Note: mechanically DungeonTheme::from_level makes boss floors 5/10/15/20/25 generate with the NEXT theme's tileset (floor 5 is Twisted Caves terrain), but the README boss table's theme pairing is followed here as the design intent.",
  "spells: all 150 spells of the src/main.rs SPELL COMPENDIUM (create_fire_spells .. create_enchantment_spells), array order = source ids 1-150. name, mana_cost (manaCost), base damage and description are verbatim; element is the MagicSchool lowercased; aoe is the Spell::new area radius when > 0; damage omitted when the source base damage is 0 (utility/heal/buff spells). Per contract, tier, cooldown, range, cast_time, mana_type and the SpellEffect rider lists (DoT, stun, summon, etc.) were not ported; spell combinations, mastery scaling and the separate class Skill list (core/src/magic.rs, already in data_classes.js) are also out of scope here.",
  "tiles: all 19 Tile variants ported with glyph() and walkable() verbatim; contract ids used for wall/floor/doorClosed(Door)/doorOpen(OpenDoor)/stairs/trap/water/lava/chest/shrine/bossGate/pillar, plus source extras disarmedTrap, openChest, usedShrine, grass, ice, sand. effect strings are invented labels summarizing source behavior.",
  "tiles/lava: in the source Tile::walkable() does NOT include Lava (passable false is faithful), yet main.rs also contains a contact-damage branch (5 + floor damage, burning 3 turns) that this data documents; a web port that makes lava walkable should apply that effect.",
  "tiles/doors: chests and shrines are activated by bumping (move into them); closed doors open on bump. Chest loot: 1-3 items, 30% chance each is upgraded one rarity step, plus (10..=50) x floor gold - verbatim from open_chest().",
  "generation context (core/src/world.rs, not part of the contract data): map 100x45, up to 20 rooms (5-15 tiles), traps 20%/room, chests 15%/room, shrine chance 10% + 1% per floor, water pools 10% in dungeon/caves, lava pools 15% in volcanic/demon, pillars 30% in large rooms; boss floors are [5,10,15,20,25,30].",
  "shrines: six blessings verbatim from use_shrine(), equally likely (rng 0..6). ids invented from the shrine message names.",
  "traps: five types verbatim from trigger_trap(), equally likely (rng 0..5). Trap type names invented from the source messages. spike damageRange [5,20] is the computed span of '5 + floor/2' over floors 1-30; non-damaging traps carry damage 0."
];

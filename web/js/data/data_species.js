'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// Ported from /home/user/ShadowCrypt/src/main.rs (Species enum + subspecies impls).
// Stat bonus tuple order in the Rust source is (hp, atk, def, spd, mana) — see main.rs:562.
SC.DATA.species = [
  {
    id: 'human',
    name: 'Human',
    description: 'Adaptable mortals from every corner of the world',
    subspecies: [
      {
        id: 'northerner',
        name: 'Northerner',
        description: 'Hardy folk from frozen lands. +STR, +DEF, Cold Resist',
        bonuses: { hp: 20, atk: 10, def: 5, spd: 0, mana: 0 },
        ability: 'Cold Resist - Resistant to cold and frozen environments'
      },
      {
        id: 'southerner',
        name: 'Southerner',
        description: 'Agile desert dwellers. +DEX, +SPD, Heat Resist',
        bonuses: { hp: 0, atk: 5, def: 0, spd: 10, mana: 5 },
        ability: 'Heat Resist - Resistant to heat and burning environments'
      },
      {
        id: 'imperial',
        name: 'Imperial',
        description: 'Noble bloodline. Balanced stats, +Gold Find',
        bonuses: { hp: 10, atk: 5, def: 5, spd: 5, mana: 10 },
        ability: 'Gold Find - Find extra gold'
      },
      {
        id: 'nomad',
        name: 'Nomad',
        description: 'Wandering survivors. High speed, survival skills',
        bonuses: { hp: 5, atk: 5, def: 0, spd: 15, mana: 0 },
        ability: 'Survivalist - Survival bonuses while exploring'
      },
      {
        id: 'islander',
        name: 'Islander',
        description: 'Sea-faring people. +DEX, can breathe underwater',
        bonuses: { hp: 10, atk: 5, def: 5, spd: 10, mana: 0 },
        ability: 'Water Breathing - Can breathe underwater'
      }
    ]
  },
  {
    id: 'elf',
    name: 'Elf',
    description: 'Long-lived and graceful, deeply attuned to magic',
    subspecies: [
      {
        id: 'highElf',
        name: 'High Elf',
        description: 'Ancient arcane masters. High mana, magic affinity',
        bonuses: { hp: 0, atk: 5, def: 0, spd: 5, mana: 30 },
        ability: 'Arcane Mastery - Spells cost 20% less mana'
      },
      {
        id: 'woodElf',
        name: 'Wood Elf',
        description: 'Forest guardians. Agile, nature magic',
        bonuses: { hp: 10, atk: 10, def: 5, spd: 15, mana: 10 },
        ability: "Nature's Grace - Heal in forests, tame beasts"
      },
      {
        id: 'darkElf',
        name: 'Dark Elf',
        description: 'Shadow dwellers. Strong attacks, stealth',
        bonuses: { hp: 0, atk: 10, def: 5, spd: 10, mana: 15 },
        ability: 'Shadow Step - Short range teleport'
      },
      {
        id: 'bloodElf',
        name: 'Blood Elf',
        description: 'Blood mages. Lifesteal, powerful magic',
        bonuses: { hp: -10, atk: 15, def: 0, spd: 5, mana: 20 },
        ability: 'Blood Drain - Steal HP with attacks'
      },
      {
        id: 'seaElf',
        name: 'Sea Elf',
        description: 'Ocean dwellers. Water breathing, swim mastery',
        bonuses: { hp: 15, atk: 5, def: 10, spd: 10, mana: 10 },
        ability: 'Aquatic - Breathe underwater, water magic boost'
      },
      {
        id: 'moonElf',
        name: 'Moon Elf',
        description: 'Lunar elves. Stronger at night, magic power',
        bonuses: { hp: 5, atk: 5, def: 5, spd: 5, mana: 25 },
        ability: 'Lunar Power - Stronger at night'
      },
      {
        id: 'sunElf',
        name: 'Sun Elf',
        description: 'Light wielders. Holy magic, day strength',
        bonuses: { hp: 10, atk: 15, def: 5, spd: 5, mana: 15 },
        ability: 'Solar Blessing - Stronger during day, holy resist'
      },
      {
        id: 'wildElf',
        name: 'Wild Elf',
        description: 'Feral elves. Beast taming, savage combat',
        bonuses: { hp: 15, atk: 15, def: 0, spd: 20, mana: 0 },
        ability: 'Feral Rage - Berserk mode when low HP'
      }
    ]
  },
  {
    id: 'dwarf',
    name: 'Dwarf',
    description: 'Stout mountain folk, master smiths and miners',
    subspecies: [
      {
        id: 'mountainDwarf',
        name: 'Mountain Dwarf',
        description: 'Hardy dwarf from the mountain halls',
        bonuses: { hp: 25, atk: 20, def: 20, spd: -5, mana: 0 },
        ability: 'Stone Skin - Take 25% less physical damage'
      },
      {
        id: 'deepDwarf',
        name: 'Deep Dwarf',
        description: 'Dwarf adapted to deep underground living',
        bonuses: { hp: 20, atk: 10, def: 15, spd: 0, mana: 5 },
        ability: 'Dark Vision - See in complete darkness'
      },
      {
        id: 'goldDwarf',
        name: 'Gold Dwarf',
        description: 'Dwarf with innate gold-finding abilities',
        bonuses: { hp: 15, atk: 10, def: 10, spd: 5, mana: 10 },
        ability: 'Treasure Sense - Find hidden gold and items'
      },
      {
        id: 'ironDwarf',
        name: 'Iron Dwarf',
        description: 'Dwarf with unbreakable resolve',
        bonuses: { hp: 30, atk: 15, def: 30, spd: -10, mana: 0 },
        ability: 'Immovable - Cannot be pushed or knocked back'
      },
      {
        id: 'runeDwarf',
        name: 'Rune Dwarf',
        description: 'Dwarf with ancient rune magic',
        bonuses: { hp: 10, atk: 5, def: 10, spd: 0, mana: 25 },
        ability: 'Rune Craft - Enchant weapons and armor'
      },
      {
        id: 'frostDwarf',
        name: 'Frost Dwarf',
        description: 'Dwarf from the frozen peaks',
        bonuses: { hp: 20, atk: 15, def: 20, spd: 0, mana: 5 },
        ability: 'Frost Aura - Slow nearby enemies'
      },
      {
        id: 'fireDwarf',
        name: 'Fire Dwarf',
        description: 'Dwarf master of forge and flame',
        bonuses: { hp: 15, atk: 25, def: 15, spd: 0, mana: 5 },
        ability: 'Forge Master - Weapons deal fire damage'
      },
      {
        id: 'hillDwarf',
        name: 'Hill Dwarf',
        description: 'Sturdy dwarf from the hills',
        bonuses: { hp: 40, atk: 10, def: 15, spd: 5, mana: 0 },
        ability: 'Dwarven Resilience - Poison/Disease immunity'
      }
    ]
  },
  {
    id: 'dragonian',
    name: 'Dragonian',
    description: 'Dragon-blooded humanoids who evolve into true dragons',
    subspecies: [
      {
        id: 'redDragonian',
        name: 'Red Dragonian',
        description: 'Flame-scaled warriors. Fire breath, immune to fire',
        bonuses: { hp: 20, atk: 25, def: 10, spd: 5, mana: 15 },
        ability: 'Fire Breath - Cone of flames'
      },
      {
        id: 'blackDragonian',
        name: 'Black Dragonian',
        description: 'Acid dragons. Corrosive attacks, dark magic',
        bonuses: { hp: 15, atk: 20, def: 15, spd: 10, mana: 20 },
        ability: 'Acid Spray - Melts armor'
      },
      {
        id: 'blueDragonian',
        name: 'Blue Dragonian',
        description: 'Storm dragons. Lightning attacks, high speed',
        bonuses: { hp: 10, atk: 20, def: 10, spd: 15, mana: 25 },
        ability: 'Lightning Bolt - Chain lightning'
      },
      {
        id: 'whiteDragonian',
        name: 'White Dragonian',
        description: 'Frost dragons. Ice magic, immune to cold',
        bonuses: { hp: 25, atk: 15, def: 20, spd: 5, mana: 15 },
        ability: 'Frost Breath - Freezes enemies'
      },
      {
        id: 'goldDragonian',
        name: 'Gold Dragonian',
        description: 'Holy dragons. Divine power, all resists',
        bonuses: { hp: 20, atk: 20, def: 20, spd: 10, mana: 30 },
        ability: 'Holy Fire - Burns evil'
      },
      {
        id: 'greenDragonian',
        name: 'Green Dragonian',
        description: 'Poison dragons. Toxic attacks, nature magic',
        bonuses: { hp: 15, atk: 15, def: 15, spd: 15, mana: 20 },
        ability: 'Poison Cloud - AoE poison'
      },
      {
        id: 'silverDragonian',
        name: 'Silver Dragonian',
        description: 'Wind dragons. Fast, flight mastery',
        bonuses: { hp: 10, atk: 15, def: 10, spd: 25, mana: 20 },
        ability: 'Wind Blast - Knockback'
      },
      {
        id: 'bronzeDragonian',
        name: 'Bronze Dragonian',
        description: 'Earth dragons. Incredible defense, earthquakes',
        bonuses: { hp: 30, atk: 20, def: 30, spd: -5, mana: 10 },
        ability: 'Earth Spike - Stuns'
      }
    ],
    dragonForms: [
      {
        id: 'dragonian',
        name: 'Dragonian',
        description: 'Starting form - humanoid with dragon features',
        evolvesAtLevel: 0,
        evolvesTo: 'drake',
        evolvesToAtLevel: 10,
        bonuses: { hp: 0, atk: 0, def: 0, spd: 0, mana: 0 }
      },
      {
        id: 'drake',
        name: 'Drake',
        description: 'Level 10 - larger, wings',
        evolvesAtLevel: 10,
        evolvesTo: 'wyrm',
        evolvesToAtLevel: 20,
        bonuses: { hp: 50, atk: 20, def: 20, spd: 10, mana: 30 }
      },
      {
        id: 'wyrm',
        name: 'Wyrm',
        description: 'Level 20 - serpentine, powerful magic',
        evolvesAtLevel: 20,
        evolvesTo: 'trueDragon',
        evolvesToAtLevel: 30,
        bonuses: { hp: 100, atk: 40, def: 40, spd: 20, mana: 60 }
      },
      {
        id: 'trueDragon',
        name: 'True Dragon',
        description: 'Level 30 - full dragon form, ultimate power',
        evolvesAtLevel: 30,
        evolvesTo: 'elderDragon',
        evolvesToAtLevel: 40,
        bonuses: { hp: 200, atk: 80, def: 80, spd: 30, mana: 100 }
      },
      {
        id: 'elderDragon',
        name: 'Elder Dragon',
        description: 'Level 40 - ancient power, reality warping',
        evolvesAtLevel: 40,
        evolvesTo: 'dragonGod',
        evolvesToAtLevel: 50,
        bonuses: { hp: 400, atk: 150, def: 150, spd: 50, mana: 200 }
      },
      {
        id: 'dragonGod',
        name: 'Dragon God',
        description: 'Level 50 - godlike, near invincible',
        evolvesAtLevel: 50,
        evolvesTo: null,
        evolvesToAtLevel: null,
        bonuses: { hp: 1000, atk: 300, def: 300, spd: 100, mana: 500 }
      }
    ]
  },
  {
    id: 'demon',
    name: 'Demon',
    description: 'Hell-born beings of dark power',
    subspecies: [
      {
        id: 'infernal',
        name: 'Infernal Demon',
        description: 'Hell-born destroyers. Devastating fire magic',
        bonuses: { hp: 10, atk: 25, def: 5, spd: 0, mana: 15 },
        ability: 'Fire Magic - Devastating fire spells'
      },
      {
        id: 'abyssal',
        name: 'Abyssal Demon',
        description: 'Void creatures. Tough, immune to darkness',
        bonuses: { hp: 20, atk: 10, def: 20, spd: -5, mana: 0 },
        ability: 'Darkness Immunity - Immune to darkness'
      },
      {
        id: 'succubus',
        name: 'Succubus',
        description: 'Seductive demons. Charm magic, high mana',
        bonuses: { hp: 0, atk: 5, def: 0, spd: 15, mana: 25 },
        ability: 'Charm - Charm enemies with seductive magic'
      },
      {
        id: 'imp',
        name: 'Imp',
        description: 'Minor demons. Fast, can fly',
        bonuses: { hp: 0, atk: 10, def: 0, spd: 25, mana: 10 },
        ability: 'Flight - Can fly over terrain'
      },
      {
        id: 'balor',
        name: 'Balor',
        description: 'Greater demons. Extreme power, slow berserker',
        bonuses: { hp: 25, atk: 30, def: 10, spd: -15, mana: 0 },
        ability: 'Berserker - Extreme power in battle'
      }
    ]
  },
  {
    id: 'undead',
    name: 'Undead',
    description: 'The risen dead, feared by the living and burned by holy light',
    subspecies: [
      {
        id: 'skeleton',
        name: 'Skeleton',
        description: 'Animated bones. Immune to poison and bleeding',
        bonuses: { hp: 0, atk: 15, def: 5, spd: 15, mana: 10 },
        ability: 'Boneless - No hunger, immune to poison/bleed',
        holy_vulnerability: 150
      },
      {
        id: 'zombie',
        name: 'Zombie',
        description: 'Risen dead. Very tough, regenerates, but slow',
        bonuses: { hp: 60, atk: 10, def: 15, spd: -15, mana: 0 },
        ability: 'Relentless - Regen HP from killing',
        holy_vulnerability: 150
      },
      {
        id: 'vampire',
        name: 'Vampire',
        description: 'Blood drinkers. Lifesteal, weak to sunlight',
        bonuses: { hp: 20, atk: 20, def: 10, spd: 20, mana: 30 },
        ability: 'Blood Thirst - Must drain blood, powerful at night',
        holy_vulnerability: 200
      },
      {
        id: 'lich',
        name: 'Lich',
        description: 'Undead mages. Immense magic power, phylactery',
        bonuses: { hp: -20, atk: 10, def: 0, spd: 5, mana: 80 },
        ability: 'Phylactery - Respawn once per floor if killed',
        holy_vulnerability: 200
      },
      {
        id: 'wraith',
        name: 'Wraith',
        description: 'Spectral beings. Phase through walls, ethereal',
        bonuses: { hp: -30, atk: 15, def: -10, spd: 25, mana: 40 },
        ability: 'Incorporeal - Phase through walls, 50% physical resist',
        holy_vulnerability: 300
      },
      {
        id: 'deathKnight',
        name: 'Death Knight',
        description: 'Undead warriors. Heavy armor, dark aura',
        bonuses: { hp: 40, atk: 25, def: 30, spd: 0, mana: 20 },
        ability: 'Death Aura - Weaken nearby living enemies',
        holy_vulnerability: 150
      },
      {
        id: 'mummy',
        name: 'Mummy',
        description: 'Ancient dead. Curses, sand magic',
        bonuses: { hp: 30, atk: 15, def: 20, spd: -5, mana: 25 },
        ability: 'Ancient Curse - Curse attackers',
        holy_vulnerability: 150
      },
      {
        id: 'revenant',
        name: 'Revenant',
        description: 'Vengeful spirits. Cannot die until revenge',
        bonuses: { hp: 20, atk: 30, def: 20, spd: 10, mana: 10 },
        ability: 'Undying Rage - Cannot die while enemies remain',
        holy_vulnerability: 100
      }
    ],
    tiers: [
      { id: 'risen', name: 'Risen', description: 'Freshly undead', minLevel: 0, maxLevel: 14, powerMultiplier: 1.0 },
      { id: 'greater', name: 'Greater', description: 'Level 15 - stronger undead powers', minLevel: 15, maxLevel: 29, powerMultiplier: 1.5 },
      { id: 'ancient', name: 'Ancient', description: 'Level 30 - powerful undead lord', minLevel: 30, maxLevel: 44, powerMultiplier: 2.0 },
      { id: 'eternal', name: 'Eternal', description: 'Level 45 - death incarnate', minLevel: 45, maxLevel: null, powerMultiplier: 3.0 }
    ]
  },
  {
    id: 'beastkin',
    name: 'Beastkin',
    description: 'Humanoids with the blood of beasts, able to transform at high level',
    subspecies: [
      {
        id: 'wolfkin',
        name: 'Wolfkin',
        description: 'Wolf-blooded beastkin with pack instincts',
        bonuses: { hp: 15, atk: 20, def: 10, spd: 20, mana: 5 },
        ability: 'Pack Howl - Buff allies, fear enemies',
        transformation: { level: 25, name: 'Alpha Wolf - Lead pack of wolves' }
      },
      {
        id: 'catkin',
        name: 'Catkin',
        description: 'Feline beastkin with nine lives',
        bonuses: { hp: 5, atk: 15, def: 5, spd: 30, mana: 10 },
        ability: 'Nine Lives - Survive fatal blow once per floor',
        transformation: { level: 25, name: 'Shadow Cat - Permanent stealth' }
      },
      {
        id: 'bearkin',
        name: 'Bearkin',
        description: 'Bear-blooded beastkin with immense strength',
        bonuses: { hp: 50, atk: 30, def: 25, spd: -10, mana: 0 },
        ability: 'Hibernate - Full heal but skip turns',
        transformation: { level: 25, name: 'Werebear - Massive size and power' }
      },
      {
        id: 'foxkin',
        name: 'Foxkin',
        description: 'Fox-blooded beastkin with illusion magic',
        bonuses: { hp: 0, atk: 10, def: 5, spd: 20, mana: 30 },
        ability: 'Fox Fire - Create illusion decoys',
        transformation: { level: 25, name: 'Nine-Tail - Ultimate illusion mastery' }
      },
      {
        id: 'lionkin',
        name: 'Lionkin',
        description: 'Lion-blooded beastkin with royal presence',
        bonuses: { hp: 25, atk: 25, def: 15, spd: 10, mana: 10 },
        ability: 'Roar - Stun and fear all enemies',
        transformation: { level: 25, name: 'King of Beasts - Command all beasts' }
      },
      {
        id: 'tigerkin',
        name: 'Tigerkin',
        description: 'Tiger-blooded beastkin with deadly ambush',
        bonuses: { hp: 20, atk: 35, def: 10, spd: 20, mana: 5 },
        ability: 'Ambush - First attack deals 3x damage',
        transformation: { level: 25, name: 'White Tiger - Divine beast form' }
      },
      {
        id: 'serpentkin',
        name: 'Serpentkin',
        description: 'Serpent-blooded beastkin with constricting grip',
        bonuses: { hp: 15, atk: 20, def: 15, spd: 15, mana: 15 },
        ability: 'Constrict - Immobilize and crush enemy',
        transformation: { level: 25, name: 'Naga - Half-serpent spellcaster' }
      },
      {
        id: 'eaglekin',
        name: 'Eaglekin',
        description: 'Eagle-blooded beastkin with flight abilities',
        bonuses: { hp: 10, atk: 20, def: 5, spd: 25, mana: 10 },
        ability: 'Dive - Flying attack, ignore terrain',
        transformation: { level: 25, name: 'Thunderbird - Storm powers' }
      },
      {
        id: 'ratkin',
        name: 'Ratkin',
        description: 'Rat-blooded beastkin with plague abilities',
        bonuses: { hp: 10, atk: 15, def: 5, spd: 25, mana: 15 },
        ability: 'Plague Carrier - Spread disease',
        transformation: { level: 25, name: 'Rat King - Control rat swarms' }
      },
      {
        id: 'sharkkin',
        name: 'Sharkkin',
        description: 'Shark-blooded beastkin with blood frenzy',
        bonuses: { hp: 30, atk: 35, def: 20, spd: 15, mana: 0 },
        ability: 'Blood Frenzy - +damage vs wounded enemies',
        transformation: { level: 25, name: 'Megalodon Form - Massive power' }
      }
    ]
  },
  {
    id: 'orc',
    name: 'Orc',
    description: 'Powerful warrior tribes fueled by battle rage',
    subspecies: [
      {
        id: 'greenOrc',
        name: 'Green Orc',
        description: 'Common orcs. Strong berserkers',
        bonuses: { hp: 30, atk: 25, def: 15, spd: 5, mana: 5 },
        ability: 'WAAAGH! - Battle cry buffs attack'
      },
      {
        id: 'blackOrc',
        name: 'Black Orc',
        description: 'Elite orcs. Devastating attackers, slow',
        bonuses: { hp: 50, atk: 35, def: 25, spd: -10, mana: 0 },
        ability: 'Unstoppable - Immune to CC when raging'
      },
      {
        id: 'greyOrc',
        name: 'Grey Orc',
        description: 'Tactical orcs. Balanced warriors with magic',
        bonuses: { hp: 20, atk: 15, def: 10, spd: 10, mana: 25 },
        ability: 'Cunning - Can cast while raging'
      },
      {
        id: 'redOrc',
        name: 'Red Orc',
        description: 'Frenzied orcs. Fast, fire damage',
        bonuses: { hp: 25, atk: 40, def: 10, spd: 10, mana: 10 },
        ability: 'Blood Rage - Fire damage while raging'
      },
      {
        id: 'paleOrc',
        name: 'Pale Orc',
        description: 'Cave orcs. Stealthy ambushers',
        bonuses: { hp: 20, atk: 20, def: 15, spd: 15, mana: 10 },
        ability: 'Silent Rage - Stealth while raging'
      },
      {
        id: 'seaOrc',
        name: 'Sea Orc',
        description: 'Aquatic orcs. Water combat masters',
        bonuses: { hp: 30, atk: 25, def: 20, spd: 10, mana: 5 },
        ability: 'Sea Fury - Water combat mastery'
      }
    ]
  },
  {
    id: 'goblin',
    name: 'Goblin',
    description: 'Small, cunning tricksters with a knack for traps and gadgets',
    subspecies: [
      {
        id: 'commonGoblin',
        name: 'Common Goblin',
        description: 'Stealthy scouts. Fast, good at traps',
        bonuses: { hp: -10, atk: 10, def: -5, spd: 25, mana: 10 },
        ability: 'Trap Master - Set deadly traps'
      },
      {
        id: 'hobgoblin',
        name: 'Hobgoblin',
        description: 'Warrior goblins. Disciplined fighters',
        bonuses: { hp: 20, atk: 20, def: 15, spd: 10, mana: 5 },
        ability: 'Formation - Tactical combat bonuses'
      },
      {
        id: 'bugbear',
        name: 'Bugbear',
        description: 'Large goblins. Strong ambushers',
        bonuses: { hp: 25, atk: 25, def: 15, spd: 15, mana: 0 },
        ability: 'Ambush Expert - Surprise attack damage'
      },
      {
        id: 'nilbog',
        name: 'Nilbog',
        description: 'Chaos goblins. Powerful wild magic',
        bonuses: { hp: 0, atk: 15, def: 5, spd: 20, mana: 25 },
        ability: 'Chaos Touch - Random magical effects'
      },
      {
        id: 'goblinShaman',
        name: 'Goblin Shaman',
        description: 'Spirit callers. Summon ancestral spirits',
        bonuses: { hp: -5, atk: 5, def: 0, spd: 15, mana: 35 },
        ability: 'Spirit Call - Summon ancestral spirits'
      },
      {
        id: 'goblinTinker',
        name: 'Goblin Tinker',
        description: 'Inventors. Create explosives and machines',
        bonuses: { hp: 10, atk: 15, def: 10, spd: 20, mana: 20 },
        ability: 'Bomb Craft - Create explosive devices'
      }
    ]
  },
  {
    id: 'fairy',
    name: 'Fairy',
    description: 'Tiny magical beings of incredible speed and fragile bodies',
    subspecies: [
      {
        id: 'pixie',
        name: 'Pixie',
        description: 'Tiny fairy with incredible speed',
        bonuses: { hp: -30, atk: 5, def: -20, spd: 40, mana: 30 },
        ability: 'Shrink - Become tiny, dodge everything'
      },
      {
        id: 'sprite',
        name: 'Sprite',
        description: 'Light fairy with healing powers',
        bonuses: { hp: -20, atk: 5, def: -15, spd: 35, mana: 40 },
        ability: 'Healing Light - Powerful healing aura'
      },
      {
        id: 'dryad',
        name: 'Dryad',
        description: 'Tree spirit with nature magic',
        bonuses: { hp: 10, atk: 10, def: 15, spd: 10, mana: 30 },
        ability: 'Tree Form - Root in place, massive regen'
      },
      {
        id: 'nymph',
        name: 'Nymph',
        description: 'Water fairy with charm abilities',
        bonuses: { hp: 0, atk: 10, def: 5, spd: 25, mana: 35 },
        ability: 'Allure - Charm enemies to fight for you'
      },
      {
        id: 'sylph',
        name: 'Sylph',
        description: 'Air spirit with wind magic',
        bonuses: { hp: -15, atk: 10, def: -10, spd: 45, mana: 25 },
        ability: 'Wind Walk - Fly over all terrain'
      },
      {
        id: 'willOWisp',
        name: "Will-o'-Wisp",
        description: 'Ghost light with confusion powers',
        bonuses: { hp: -25, atk: 15, def: -20, spd: 30, mana: 40 },
        ability: 'Hypnotic Light - Confuse all enemies'
      },
      {
        id: 'darkFairy',
        name: 'Dark Fairy',
        description: 'Corrupted fairy with curse magic',
        bonuses: { hp: -20, atk: 20, def: -10, spd: 35, mana: 35 },
        ability: 'Fairy Curse - Powerful debuffs'
      },
      {
        id: 'leprechaun',
        name: 'Leprechaun',
        description: 'Lucky fairy with gold-finding abilities',
        bonuses: { hp: -10, atk: 15, def: 0, spd: 30, mana: 25 },
        ability: 'Pot of Gold - Massive luck bonus'
      }
    ]
  },
  {
    id: 'angel',
    name: 'Angel',
    description: 'Divine beings of the heavenly host wielding holy power',
    subspecies: [
      {
        id: 'seraph',
        name: 'Seraph',
        description: 'Six-winged angel of fire and purification',
        bonuses: { hp: 30, atk: 30, def: 20, spd: 20, mana: 50 },
        ability: 'Holy Fire - Burn evil with divine flames'
      },
      {
        id: 'cherub',
        name: 'Cherub',
        description: 'Guardian angel with protection magic',
        bonuses: { hp: 40, atk: 15, def: 35, spd: 15, mana: 30 },
        ability: 'Divine Shield - Protect self and allies'
      },
      {
        id: 'throne',
        name: 'Throne',
        description: 'Angel of justice that smites evil',
        bonuses: { hp: 25, atk: 35, def: 25, spd: 15, mana: 35 },
        ability: 'Judgment - Execute low HP enemies'
      },
      {
        id: 'dominion',
        name: 'Dominion',
        description: 'Angelic leader that empowers allies',
        bonuses: { hp: 30, atk: 20, def: 25, spd: 20, mana: 40 },
        ability: 'Command - Buff all allies significantly'
      },
      {
        id: 'virtue',
        name: 'Virtue',
        description: 'Angel of miracles and healing',
        bonuses: { hp: 20, atk: 10, def: 20, spd: 20, mana: 60 },
        ability: 'Miracle - Full heal and cure all'
      },
      {
        id: 'power',
        name: 'Power',
        description: 'Warrior angel wielding holy weapons',
        bonuses: { hp: 35, atk: 40, def: 30, spd: 15, mana: 25 },
        ability: 'Holy Weapon - Summon divine armaments'
      },
      {
        id: 'fallenAngel',
        name: 'Fallen Angel',
        description: 'Corrupted angel mixing dark and holy powers',
        bonuses: { hp: 25, atk: 35, def: 20, spd: 25, mana: 45 },
        ability: 'Twilight - Mix of holy and dark powers'
      },
      {
        id: 'nephilim',
        name: 'Nephilim',
        description: 'Half-angel with balanced mortal and divine powers',
        bonuses: { hp: 30, atk: 25, def: 25, spd: 20, mana: 30 },
        ability: 'Hybrid Power - Use both mortal and divine abilities'
      }
    ]
  }
];

SC.DATA.species_notes = [
  'Stat bonus order follows the Rust tuple (hp, atk, def, spd, mana) — main.rs:562, :743, :951, :1015, :1162.',
  'Species-level description strings do not exist in the Rust source (only base_species_name); they were written here to summarize the source comments.',
  'HumanSubspecies and DemonSubspecies have no ability method in the Rust source; their ability strings were derived from the enum variant comments (main.rs:553-559, :1153-1159) and description text.',
  'AngelSubspecies (main.rs:870-908) is defined in the source but is NOT a variant of the Species enum (main.rs:539-550) and has no name()/description() methods; names come from the enum variants, descriptions from the source comments, abilities verbatim from divine_power(). It may be an unreleased/locked species.',
  'Subspecies ability strings map to the per-species source methods: Elf special_ability(), Dwarf special_ability(), Fairy fairy_magic(), Angel divine_power(), Dragonian breath_weapon(), Undead undead_trait(), Beastkin beast_ability(), Orc orc_rage(), Goblin goblin_trick().',
  'Dragonian dragonForms: evolvesAtLevel is the level at which that form is reached (DragonForm::can_evolve, main.rs:1003-1012); bonuses are the evolution_bonus() tuple for that form (main.rs:1014-1024), applied per current form (not cumulative increments).',
  'Undead holy_vulnerability is percent holy damage taken (100 = normal, 150 = 1.5x, 200 = 2x, 300 = 3x) from UndeadSubspecies::holy_vulnerability (main.rs:1078-1086).',
  'Undead tiers come from UndeadTier::from_level and power_multiplier (main.rs:1116-1146); maxLevel null on Eternal means no upper bound.',
  'Beastkin transformation objects come from BeastkinSubspecies::transformation (main.rs:1272-1289); it unlocks at level >= 25 for every subspecies.'
];

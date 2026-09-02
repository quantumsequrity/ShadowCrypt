'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// ============================================================================
// BASE CHARACTER CLASSES
// Ported from src/main.rs: enum CharacterClass (~line 108).
// base_stats tuple order: (hp, attack, defense, mana, speed) — verified
// against source comment (line 130) and README.md class tables.
// ============================================================================
SC.DATA.classes = [
  {
    id: 'warrior',
    name: 'Warrior',
    hp: 50, atk: 8, def: 5, mana: 10, spd: 10,
    specialAbility: 'Berserk (2x damage, take 50% more)',
    description: 'Ideal for players who prefer direct combat and high survivability.',
    skills: ['berserk', 'cleave', 'shieldBash', 'whirlwind'],
    subclasses: ['berserker', 'knight', 'gladiator'],
    evolutionPaths: ['adv_berserker', 'adv_guardian']
  },
  {
    id: 'mage',
    name: 'Mage',
    hp: 30, atk: 3, def: 2, mana: 50, spd: 10,
    specialAbility: 'Fireball (AoE damage)',
    description: 'Ideal for players who enjoy spellcasting and dealing with groups of enemies.',
    skills: ['fireball', 'iceSpear', 'lightning', 'teleport'],
    subclasses: ['elementalist', 'necromancer', 'enchanter'],
    evolutionPaths: ['adv_archmage', 'adv_elementalist']
  },
  {
    id: 'rogue',
    name: 'Rogue',
    hp: 35, atk: 6, def: 3, mana: 20, spd: 15,
    specialAbility: 'Backstab (3x damage from behind)',
    description: 'Ideal for players who prefer stealth, positioning, and critical strikes.',
    skills: ['backstab', 'shadowStep', 'poisonBlade', 'vanish'],
    subclasses: ['assassin', 'shadow', 'trickster'],
    evolutionPaths: ['adv_assassin', 'adv_shadowdancer']
  },
  {
    id: 'paladin',
    name: 'Paladin',
    hp: 45, atk: 6, def: 6, mana: 30, spd: 8,
    specialAbility: 'Holy Light (heal + damage undead)',
    description: 'Ideal for balanced gameplay with both offensive and defensive capabilities.',
    skills: ['holyLight', 'divineShield', 'smite', 'consecrate'],
    subclasses: ['priest', 'inquisitor', 'monk'],
    evolutionPaths: ['adv_crusader', 'adv_templar']
  },
  {
    id: 'ranger',
    name: 'Ranger',
    hp: 38, atk: 7, def: 3, mana: 25, spd: 12,
    specialAbility: 'Multi-shot (hit 3 enemies)',
    description: 'Ideal for players who like ranged combat and crowd control.',
    skills: ['multiShot', 'poisonArrow', 'trapSet', 'eagleEye'],
    subclasses: ['beastmaster', 'archer', 'druid'],
    evolutionPaths: ['adv_beastmaster', 'adv_marksman']
  },
  {
    id: 'necromancer',
    name: 'Necromancer',
    hp: 32, atk: 4, def: 2, mana: 45, spd: 9,
    specialAbility: 'Raise Dead (summon skeleton)',
    description: 'Ideal for players who enjoy summoning minions and dark magic.',
    skills: ['raiseDead', 'lifeDrain', 'curse', 'darkPact'],
    subclasses: ['demonologist', 'bloodMage', 'hexer'],
    evolutionPaths: ['adv_lich', 'adv_reaper']
  }
];

SC.DATA.classes_notes = [
  'base stats ported verbatim from CharacterClass::base_stats; tuple order (hp, atk, def, mana, spd) confirmed by the source comment and the README class tables.',
  'specialAbility strings are verbatim from CharacterClass::special_ability.',
  'CharacterClass has no description() in the Rust source; descriptions are taken from the "Ideal for" lines in README.md class sections.',
  'Progression constants from the source: subclass_upgrade_level() = 10 (subclass unlock), advanced_subclass_level() = 25 (advanced subclass unlock).',
  'evolutionPaths lists the Tier 2 AdvancedClass ids available from this base class (AdvancedClass::evolution_paths); ids reference SC.DATA.advancedClasses.',
  'subclasses lists the Subclass ids available at level 10 (CharacterClass::available_subclasses); ids reference SC.DATA.subclasses.'
];

// ============================================================================
// SUBCLASSES
// Ported from src/main.rs: enum Subclass (~line 165).
// stat_bonuses tuple order: (hp, atk, def, mana, spd) — cross-checked:
// Knight (30,15,25,0,0) = hp30/atk15/def25; mana-heavy entries (Archmage 80,
// Sage 90, HighPriest 80) carry the large value in slot 4.
// ============================================================================
SC.DATA.subclasses = [
  {
    id: 'berserker', name: 'Berserker', description: 'Unleash primal rage',
    bonuses: { hp: 20, atk: 30, def: 0, mana: 10, spd: 5 },
    skills: ['rage', 'reckless', 'bloodFrenzy'],
    advancedForm: 'warlord', alternativeAdvanced: null, isAdvanced: false, color: '#e74c3c'
  },
  {
    id: 'knight', name: 'Knight', description: 'Stalwart defender',
    bonuses: { hp: 30, atk: 15, def: 25, mana: 0, spd: 0 },
    skills: ['shieldWall', 'fortify', 'rally'],
    advancedForm: 'paladin', alternativeAdvanced: 'darkKnight', isAdvanced: false, color: '#ecf0f1'
  },
  {
    id: 'gladiator', name: 'Gladiator', description: 'Swift duelist',
    bonuses: { hp: 15, atk: 25, def: 10, mana: 5, spd: 20 },
    skills: ['dualStrike', 'riposte', 'flurryOfBlows'],
    advancedForm: 'champion', alternativeAdvanced: null, isAdvanced: false, color: '#f1c40f'
  },
  {
    id: 'warlord', name: 'Warlord', description: 'Command armies',
    bonuses: { hp: 40, atk: 50, def: 20, mana: 15, spd: 10 },
    skills: ['rage', 'battleCry', 'devastate', 'conqueror'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#922b21'
  },
  {
    id: 'paladin', name: 'Paladin', description: 'Holy warrior',
    bonuses: { hp: 50, atk: 30, def: 40, mana: 30, spd: 5 },
    skills: ['shieldWall', 'holySmite', 'divineAura', 'redemption'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#f1c40f'
  },
  {
    id: 'darkKnight', name: 'Dark Knight', description: 'Dark powers',
    bonuses: { hp: 35, atk: 45, def: 30, mana: 25, spd: 10 },
    skills: ['shieldWall', 'darkSlash', 'soulReap', 'corruption'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#76448a'
  },
  {
    id: 'champion', name: 'Champion', description: 'Arena master',
    bonuses: { hp: 30, atk: 40, def: 20, mana: 10, spd: 35 },
    skills: ['dualStrike', 'execute', 'gloryStrike', 'unstoppable'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#f1c40f'
  },
  {
    id: 'elementalist', name: 'Elementalist', description: 'Master elements',
    bonuses: { hp: 10, atk: 20, def: 5, mana: 40, spd: 5 },
    skills: ['fireBlast', 'frostNova', 'chainLightning'],
    advancedForm: 'archmage', alternativeAdvanced: null, isAdvanced: false, color: '#1abc9c'
  },
  {
    id: 'necromancer', name: 'Necromancer', description: 'Death magic',
    bonuses: { hp: 15, atk: 15, def: 10, mana: 35, spd: 5 },
    skills: ['raiseSkeleton', 'deathCoil', 'boneArmor'],
    advancedForm: 'lichLord', alternativeAdvanced: null, isAdvanced: false, color: '#5d6d7e'
  },
  {
    id: 'enchanter', name: 'Enchanter', description: 'Powerful buffs',
    bonuses: { hp: 10, atk: 10, def: 15, mana: 45, spd: 5 },
    skills: ['empower', 'magicShield', 'haste'],
    advancedForm: 'sage', alternativeAdvanced: null, isAdvanced: false, color: '#d35ded'
  },
  {
    id: 'archmage', name: 'Archmage', description: 'Supreme mage',
    bonuses: { hp: 20, atk: 35, def: 15, mana: 80, spd: 10 },
    skills: ['fireBlast', 'meteor', 'elementalMastery', 'arcaneNova'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#3498db'
  },
  {
    id: 'lichLord', name: 'Lich Lord', description: 'Undead lord',
    bonuses: { hp: 50, atk: 30, def: 25, mana: 60, spd: 5 },
    skills: ['raiseSkeleton', 'armyOfDead', 'deathGrip', 'lichform'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#5d6d7e'
  },
  {
    id: 'sage', name: 'Sage', description: 'Ultimate wisdom',
    bonuses: { hp: 25, atk: 20, def: 30, mana: 90, spd: 10 },
    skills: ['empower', 'timeWarp', 'omniscience', 'transcendence'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#ecf0f1'
  },
  {
    id: 'assassin', name: 'Assassin', description: 'Deadly strikes',
    bonuses: { hp: 10, atk: 35, def: 5, mana: 15, spd: 20 },
    skills: ['deadlyStrike', 'ambush', 'poisonDagger'],
    advancedForm: 'shadowBlade', alternativeAdvanced: null, isAdvanced: false, color: '#5d6d7e'
  },
  {
    id: 'shadow', name: 'Shadow', description: 'One with darkness',
    bonuses: { hp: 15, atk: 20, def: 10, mana: 20, spd: 25 },
    skills: ['shadowMeld', 'vanish', 'shadowStrike'],
    advancedForm: 'nightstalker', alternativeAdvanced: null, isAdvanced: false, color: '#95a5a6'
  },
  {
    id: 'trickster', name: 'Trickster', description: 'Traps and deception',
    bonuses: { hp: 15, atk: 25, def: 10, mana: 25, spd: 15 },
    skills: ['throwKnife', 'smokeScreen', 'trapMaster'],
    advancedForm: 'masterThief', alternativeAdvanced: null, isAdvanced: false, color: '#b7950b'
  },
  {
    id: 'shadowBlade', name: 'Shadow Blade', description: 'Shadow weapons',
    bonuses: { hp: 20, atk: 55, def: 15, mana: 30, spd: 30 },
    skills: ['deadlyStrike', 'shadowDance', 'deathMark', 'eviscerate'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#76448a'
  },
  {
    id: 'nightstalker', name: 'Nightstalker', description: 'Invisible killer',
    bonuses: { hp: 25, atk: 40, def: 20, mana: 35, spd: 40 },
    skills: ['shadowMeld', 'phantomStrike', 'assassination', 'voidStep'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#5d6d7e'
  },
  {
    id: 'masterThief', name: 'Master Thief', description: 'Steal anything',
    bonuses: { hp: 30, atk: 45, def: 25, mana: 40, spd: 35 },
    skills: ['throwKnife', 'pickpocket', 'grandHeist', 'luckOfThief'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#b7950b'
  },
  {
    id: 'priest', name: 'Priest', description: 'Divine healing',
    bonuses: { hp: 25, atk: 10, def: 15, mana: 40, spd: 5 },
    skills: ['heal', 'blessing', 'purify'],
    advancedForm: 'highPriest', alternativeAdvanced: null, isAdvanced: false, color: '#ecf0f1'
  },
  {
    id: 'inquisitor', name: 'Inquisitor', description: 'Holy wrath',
    bonuses: { hp: 20, atk: 25, def: 20, mana: 25, spd: 5 },
    skills: ['holyStrike', 'judgment', 'exorcism'],
    advancedForm: 'templar', alternativeAdvanced: null, isAdvanced: false, color: '#f1c40f'
  },
  {
    id: 'monk', name: 'Monk', description: 'Martial mastery',
    bonuses: { hp: 20, atk: 30, def: 20, mana: 15, spd: 25 },
    skills: ['palmStrike', 'flurryOfBlows', 'innerPeace'],
    advancedForm: 'grandmaster', alternativeAdvanced: null, isAdvanced: false, color: '#b7950b'
  },
  {
    id: 'highPriest', name: 'High Priest', description: 'Miracle worker',
    bonuses: { hp: 50, atk: 20, def: 30, mana: 80, spd: 10 },
    skills: ['heal', 'miracle', 'divineIntervention', 'resurrection'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#ecf0f1'
  },
  {
    id: 'templar', name: 'Templar', description: 'Holy crusader',
    bonuses: { hp: 40, atk: 45, def: 40, mana: 40, spd: 15 },
    skills: ['holyStrike', 'crusaderStrike', 'holyWrath', 'zealot'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#f1c40f'
  },
  {
    id: 'grandmaster', name: 'Grandmaster', description: 'Martial perfection',
    bonuses: { hp: 40, atk: 50, def: 35, mana: 25, spd: 45 },
    skills: ['palmStrike', 'quiveringPalm', 'enlightenment', 'perfectForm'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#1abc9c'
  },
  {
    id: 'beastmaster', name: 'Beastmaster', description: 'Animal bond',
    bonuses: { hp: 20, atk: 20, def: 15, mana: 20, spd: 15 },
    skills: ['callPet', 'beastBond', 'packTactics'],
    advancedForm: 'alpha', alternativeAdvanced: null, isAdvanced: false, color: '#1e8449'
  },
  {
    id: 'archer', name: 'Archer', description: 'Ranged precision',
    bonuses: { hp: 15, atk: 30, def: 10, mana: 15, spd: 20 },
    skills: ['aimedShot', 'multiShot', 'piercingArrow'],
    advancedForm: 'marksman', alternativeAdvanced: null, isAdvanced: false, color: '#2ecc71'
  },
  {
    id: 'druid', name: 'Druid', description: "Nature's forces",
    bonuses: { hp: 20, atk: 15, def: 15, mana: 35, spd: 10 },
    skills: ['entangle', 'natureFury', 'rejuvenate'],
    advancedForm: 'archdruid', alternativeAdvanced: null, isAdvanced: false, color: '#2ecc71'
  },
  {
    id: 'alpha', name: 'Alpha', description: 'Pack leader',
    bonuses: { hp: 40, atk: 35, def: 30, mana: 30, spd: 25 },
    skills: ['callPet', 'alphaRoar', 'beastMaster', 'stampede'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#1e8449'
  },
  {
    id: 'marksman', name: 'Marksman', description: 'Perfect accuracy',
    bonuses: { hp: 25, atk: 55, def: 20, mana: 25, spd: 30 },
    skills: ['aimedShot', 'headshot', 'rapidFire', 'killShot'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#2ecc71'
  },
  {
    id: 'archdruid', name: 'Archdruid', description: 'Nature incarnate',
    bonuses: { hp: 45, atk: 30, def: 30, mana: 70, spd: 15 },
    skills: ['entangle', 'natureWrath', 'treeForm', 'forceOfNature'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#2ecc71'
  },
  {
    id: 'demonologist', name: 'Demonologist', description: 'Summon demons',
    bonuses: { hp: 15, atk: 25, def: 10, mana: 35, spd: 5 },
    skills: ['summonImp', 'demonBolt', 'hellfire'],
    advancedForm: 'demonLord', alternativeAdvanced: null, isAdvanced: false, color: '#922b21'
  },
  {
    id: 'bloodMage', name: 'Blood Mage', description: 'Life for power',
    bonuses: { hp: 30, atk: 30, def: 5, mana: 30, spd: 5 },
    skills: ['bloodBolt', 'lifeTap', 'bloodShield'],
    advancedForm: 'bloodLord', alternativeAdvanced: null, isAdvanced: false, color: '#e74c3c'
  },
  {
    id: 'hexer', name: 'Hexer', description: 'Devastating curses',
    bonuses: { hp: 15, atk: 20, def: 15, mana: 40, spd: 5 },
    skills: ['hex', 'doomCurse', 'weakness'],
    advancedForm: 'curseWeaver', alternativeAdvanced: null, isAdvanced: false, color: '#76448a'
  },
  {
    id: 'demonLord', name: 'Demon Lord', description: 'Demonic legions',
    bonuses: { hp: 35, atk: 45, def: 25, mana: 60, spd: 15 },
    skills: ['summonImp', 'summonDemon', 'infernalPact', 'demonicForm'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#922b21'
  },
  {
    id: 'bloodLord', name: 'Blood Lord', description: 'Crimson arts',
    bonuses: { hp: 60, atk: 50, def: 15, mana: 50, spd: 10 },
    skills: ['bloodBolt', 'exsanguinate', 'crimsonPact', 'bloodNova'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#e74c3c'
  },
  {
    id: 'curseWeaver', name: 'Curse Weaver', description: 'Inescapable curses',
    bonuses: { hp: 30, atk: 35, def: 30, mana: 80, spd: 10 },
    skills: ['hex', 'doom', 'curseOfAgony', 'pandemonium'],
    advancedForm: null, alternativeAdvanced: null, isAdvanced: true, color: '#76448a'
  }
];

SC.DATA.subclasses_notes = [
  'bonuses ported verbatim from Subclass::stat_bonuses; tuple order interpreted as (hp, atk, def, mana, spd), cross-checked against Knight (30,15,25,0,0) and the mana-heavy caster entries (Archmage 80, Sage 90, HighPriest 80 in slot 4).',
  'skills lists ported verbatim from Subclass::unique_skills; ids reference SC.DATA.skills.',
  'advancedForm/alternativeAdvanced ported from Subclass::advanced_form and Subclass::alternative_advanced (only Knight has an alternative: Dark Knight).',
  'colors converted from crossterm Color enums per the fixed hex mapping (Subclass::color).',
  'Base subclasses unlock at level 10, advanced forms at level 25 (CharacterClass::subclass_upgrade_level / advanced_subclass_level).'
];

// ============================================================================
// ADVANCED CLASSES (CLASS EVOLUTION SYSTEM)
// Ported from src/main.rs: enum AdvancedClass (~line 193) + ClassRequirements.
// stat_bonuses and statRequirements tuple order: (hp, atk, def, mana, spd)
// per source comments (lines 232-233, 333).
// Ids are prefixed adv_ to keep them distinct from subclass ids.
// ============================================================================
SC.DATA.advancedClasses = [
  {
    id: 'adv_berserker', name: 'Berserker', tier: 2, baseClass: 'warrior',
    description: 'Fury incarnate. Massive damage, rage abilities, low defense',
    bonuses: { hp: 30, atk: 25, def: -5, mana: 0, spd: 10 },
    requirements: {
      minLevel: 15, minKills: 100, requiredItem: 'Rage Crystal', requiredBossKills: 2,
      statRequirements: { hp: 80, atk: 15, def: 0, mana: 0, spd: 0 }
    },
    uniqueAbility: 'Blood Rage - Enter berserk state: +100% damage, lifesteal, but lose 5 HP/turn',
    evolutionPaths: ['adv_warlord']
  },
  {
    id: 'adv_guardian', name: 'Guardian', tier: 2, baseClass: 'warrior',
    description: 'Unbreakable wall. High defense, party protection, taunts',
    bonuses: { hp: 50, atk: 5, def: 30, mana: 10, spd: -5 },
    requirements: {
      minLevel: 15, minKills: 75, requiredItem: 'Aegis Shield', requiredBossKills: 2,
      statRequirements: { hp: 100, atk: 0, def: 15, mana: 0, spd: 0 }
    },
    uniqueAbility: 'Fortress Stance - Become immovable: 75% damage reduction, taunt all enemies',
    evolutionPaths: ['adv_warlord']
  },
  {
    id: 'adv_warlord', name: 'Warlord', tier: 3, baseClass: 'warrior',
    description: 'Supreme commander. Army buffs, devastating charges, fear aura',
    bonuses: { hp: 60, atk: 30, def: 25, mana: 20, spd: 15 },
    requirements: {
      minLevel: 30, minKills: 300, requiredItem: "Conqueror's Crown", requiredBossKills: 10,
      statRequirements: { hp: 150, atk: 25, def: 20, mana: 0, spd: 0 }
    },
    uniqueAbility: 'Rally the Troops - All allies gain +50% stats, enemies feared for 5 turns',
    evolutionPaths: []
  },
  {
    id: 'adv_archmage', name: 'Archmage', tier: 2, baseClass: 'mage',
    description: 'Master of all magic. Reduced mana costs, spell amplification',
    bonuses: { hp: 10, atk: 10, def: 5, mana: 60, spd: 5 },
    requirements: {
      minLevel: 15, minKills: 60, requiredItem: 'Tome of Infinite Knowledge', requiredBossKills: 2,
      statRequirements: { hp: 0, atk: 0, def: 0, mana: 100, spd: 0 }
    },
    uniqueAbility: 'Arcane Overload - Next 3 spells cost no mana and deal double damage',
    evolutionPaths: ['adv_sage']
  },
  {
    id: 'adv_elementalist', name: 'Elementalist', tier: 2, baseClass: 'mage',
    description: 'Elemental fury. Fire, ice, lightning mastery, combos',
    bonuses: { hp: 15, atk: 20, def: 5, mana: 45, spd: 10 },
    requirements: {
      minLevel: 15, minKills: 80, requiredItem: 'Elemental Core', requiredBossKills: 2,
      statRequirements: { hp: 0, atk: 10, def: 0, mana: 80, spd: 0 }
    },
    uniqueAbility: 'Elemental Convergence - Unleash all elements at once, massive AoE',
    evolutionPaths: ['adv_sage']
  },
  {
    id: 'adv_sage', name: 'Sage', tier: 3, baseClass: 'mage',
    description: 'Ultimate wisdom. Reality manipulation, time magic, omniscience',
    bonuses: { hp: 30, atk: 25, def: 20, mana: 80, spd: 20 },
    requirements: {
      minLevel: 30, minKills: 200, requiredItem: 'Orb of Omniscience', requiredBossKills: 10,
      statRequirements: { hp: 0, atk: 15, def: 0, mana: 200, spd: 0 }
    },
    uniqueAbility: 'Time Stop - Freeze time for 3 turns, act freely while enemies frozen',
    evolutionPaths: []
  },
  {
    id: 'adv_assassin', name: 'Assassin', tier: 2, baseClass: 'rogue',
    description: 'Death from shadows. Instant kills, poison mastery, stealth',
    bonuses: { hp: 10, atk: 30, def: 0, mana: 15, spd: 25 },
    requirements: {
      minLevel: 15, minKills: 120, requiredItem: 'Venom Fang Dagger', requiredBossKills: 2,
      statRequirements: { hp: 0, atk: 15, def: 0, mana: 0, spd: 20 }
    },
    uniqueAbility: 'Death Mark - Mark target for instant kill on next hit from behind',
    evolutionPaths: ['adv_phantom']
  },
  {
    id: 'adv_shadowdancer', name: 'Shadowdancer', tier: 2, baseClass: 'rogue',
    description: 'Shadow and magic. Teleportation, illusions, dark magic',
    bonuses: { hp: 15, atk: 20, def: 5, mana: 35, spd: 20 },
    requirements: {
      minLevel: 15, minKills: 90, requiredItem: 'Shadow Essence', requiredBossKills: 2,
      statRequirements: { hp: 0, atk: 10, def: 0, mana: 40, spd: 20 }
    },
    uniqueAbility: 'Shadow Realm - Enter shadow plane, invisible and can phase through walls',
    evolutionPaths: ['adv_phantom']
  },
  {
    id: 'adv_phantom', name: 'Phantom', tier: 3, baseClass: 'rogue',
    description: 'Beyond death. Ethereal form, soul rend, untouchable',
    bonuses: { hp: 25, atk: 35, def: 15, mana: 40, spd: 35 },
    requirements: {
      minLevel: 30, minKills: 350, requiredItem: 'Soul of the Void', requiredBossKills: 10,
      statRequirements: { hp: 0, atk: 30, def: 0, mana: 60, spd: 30 }
    },
    uniqueAbility: 'Soul Rend - Become ethereal, attacks hit souls directly ignoring armor',
    evolutionPaths: []
  },
  {
    id: 'adv_crusader', name: 'Crusader', tier: 2, baseClass: 'paladin',
    description: 'Holy warrior. Smite evil, divine strikes, zealous charge',
    bonuses: { hp: 35, atk: 25, def: 15, mana: 30, spd: 10 },
    requirements: {
      minLevel: 15, minKills: 100, requiredItem: 'Holy Avenger Sword', requiredBossKills: 3,
      statRequirements: { hp: 80, atk: 15, def: 10, mana: 50, spd: 0 }
    },
    uniqueAbility: 'Divine Judgment - Smite all enemies in sight, bonus damage to undead/demons',
    evolutionPaths: ['adv_avatar']
  },
  {
    id: 'adv_templar', name: 'Templar', tier: 2, baseClass: 'paladin',
    description: 'Divine shield. Holy barriers, healing aura, sanctuary',
    bonuses: { hp: 45, atk: 15, def: 30, mana: 35, spd: 5 },
    requirements: {
      minLevel: 15, minKills: 80, requiredItem: 'Divine Relic', requiredBossKills: 3,
      statRequirements: { hp: 100, atk: 10, def: 15, mana: 60, spd: 0 }
    },
    uniqueAbility: 'Sanctuary - Create holy barrier, party invulnerable for 3 turns',
    evolutionPaths: ['adv_avatar']
  },
  {
    id: 'adv_avatar', name: 'Avatar', tier: 3, baseClass: 'paladin',
    description: 'Divine vessel. Godly power, resurrection, judgment',
    bonuses: { hp: 60, atk: 30, def: 35, mana: 50, spd: 20 },
    requirements: {
      minLevel: 30, minKills: 250, requiredItem: 'Blessing of the Gods', requiredBossKills: 12,
      statRequirements: { hp: 150, atk: 25, def: 25, mana: 100, spd: 0 }
    },
    uniqueAbility: 'Divine Descent - Transform into godly form, +200% all stats for 10 turns',
    evolutionPaths: []
  },
  {
    id: 'adv_beastmaster', name: 'Beastmaster', tier: 2, baseClass: 'ranger',
    description: 'Beast lord. Multiple pets, animal forms, pack tactics',
    bonuses: { hp: 30, atk: 20, def: 15, mana: 25, spd: 15 },
    requirements: {
      minLevel: 15, minKills: 90, requiredItem: 'Beast Soul Totem', requiredBossKills: 2,
      statRequirements: { hp: 60, atk: 15, def: 10, mana: 40, spd: 15 }
    },
    uniqueAbility: 'Pack Alpha - Summon 3 legendary beasts to fight alongside you',
    evolutionPaths: ['adv_warden']
  },
  {
    id: 'adv_marksman', name: 'Marksman', tier: 2, baseClass: 'ranger',
    description: 'Perfect aim. Critical shots, piercing arrows, sniper',
    bonuses: { hp: 15, atk: 35, def: 5, mana: 20, spd: 20 },
    requirements: {
      minLevel: 15, minKills: 100, requiredItem: 'Legendary Bow', requiredBossKills: 2,
      statRequirements: { hp: 50, atk: 20, def: 5, mana: 30, spd: 18 }
    },
    uniqueAbility: 'Perfect Shot - Guaranteed critical hit that pierces all enemies in line',
    evolutionPaths: ['adv_warden']
  },
  {
    id: 'adv_warden', name: 'Warden', tier: 3, baseClass: 'ranger',
    description: "Nature's champion. Elemental beasts, terrain control, primal fury",
    bonuses: { hp: 45, atk: 30, def: 25, mana: 35, spd: 25 },
    requirements: {
      minLevel: 30, minKills: 280, requiredItem: 'Heart of the Wild', requiredBossKills: 10,
      statRequirements: { hp: 100, atk: 30, def: 20, mana: 60, spd: 25 }
    },
    uniqueAbility: "Primal Storm - Summon nature's wrath, elemental beasts rain destruction",
    evolutionPaths: []
  },
  {
    id: 'adv_lich', name: 'Lich', tier: 2, baseClass: 'necromancer',
    description: 'Undead mage. Immortal phylactery, ice magic, curse mastery',
    bonuses: { hp: 0, atk: 15, def: 10, mana: 70, spd: 5 },
    requirements: {
      minLevel: 15, minKills: 80, requiredItem: 'Phylactery', requiredBossKills: 3,
      statRequirements: { hp: 0, atk: 10, def: 5, mana: 100, spd: 0 }
    },
    uniqueAbility: 'Phylactery Bond - On death, resurrect at full power after 3 turns',
    evolutionPaths: ['adv_deathLord']
  },
  {
    id: 'adv_reaper', name: 'Reaper', tier: 2, baseClass: 'necromancer',
    description: "Death's hand. Soul harvest, instant death, fear aura",
    bonuses: { hp: 20, atk: 30, def: 5, mana: 50, spd: 15 },
    requirements: {
      minLevel: 15, minKills: 100, requiredItem: "Death's Scythe", requiredBossKills: 3,
      statRequirements: { hp: 40, atk: 15, def: 5, mana: 80, spd: 10 }
    },
    uniqueAbility: 'Soul Harvest - Kill all enemies below 25% HP, heal for their max HP',
    evolutionPaths: ['adv_deathLord']
  },
  {
    id: 'adv_deathLord', name: 'Death Lord', tier: 3, baseClass: 'necromancer',
    description: 'Master of death. Massive undead army, death knight form',
    bonuses: { hp: 40, atk: 35, def: 25, mana: 65, spd: 20 },
    requirements: {
      minLevel: 30, minKills: 300, requiredItem: 'Crown of the Dead King', requiredBossKills: 12,
      statRequirements: { hp: 80, atk: 25, def: 15, mana: 150, spd: 15 }
    },
    uniqueAbility: 'Army of the Damned - Raise all corpses on floor as permanent undead army',
    evolutionPaths: []
  }
];

SC.DATA.advancedClasses_notes = [
  'Ids carry an adv_ prefix because the AdvancedClass evolution system is separate from the Subclass system and several names collide (Berserker, Archmage, Templar, Marksman, ...).',
  'tier 2 = first evolution (level 15+), tier 3 = final evolution (level 30+), per the ClassTier enum comments; the Base tier is represented by SC.DATA.classes.',
  'bonuses and requirements.statRequirements ported verbatim; tuple order (hp, atk, def, mana, spd) per source comments on ClassRequirements and AdvancedClass::stat_bonuses.',
  'Lich intentionally has hp bonus 0 — the source comments "Lich trades HP for power".',
  'evolutionPaths on each entry is the AdvancedClass::tier3_evolution target (empty for tier 3); the tier-2 paths from each base class live on SC.DATA.classes[].evolutionPaths.',
  'can_evolve gating in the source: player level >= minLevel, kills >= minKills, boss kills >= requiredBossKills, every stat >= statRequirements, and the requiredItem must be held.'
];

// ============================================================================
// SKILLS
// Ported from src/main.rs: enum Skill (~line 5381) and impl Skill.
// The Rust impl defines name() and mana_cost() only; subclass/advanced skills
// fall through to name "Unknown Skill" and mana cost 30, and their in-game
// behavior is the generic strike in use_skill's default arm.
// effect objects for base skills mirror the use_skill implementation.
// ============================================================================
SC.DATA.skills = [
  // --- Base Class Skills - Warrior ---
  {
    id: 'berserk', name: 'Berserk', manaCost: 10, cooldown: 0, class: 'warrior',
    description: 'Enter a rage that empowers your attacks (Strength for 10 turns).',
    effect: { type: 'buffSelf', status: 'Strength', duration: 10 }
  },
  {
    id: 'cleave', name: 'Cleave', manaCost: 10, cooldown: 0, class: 'warrior',
    description: 'Strike every adjacent enemy with your full attack.',
    effect: { type: 'meleeAoe', radius: 1, damageStat: 'attack', multiplier: 1 }
  },
  {
    id: 'shieldBash', name: 'Shield Bash', manaCost: 10, cooldown: 0, class: 'warrior',
    description: 'Bash an adjacent enemy with your shield, stunning it for 3 turns and dealing damage equal to your defense.',
    effect: { type: 'strikeAdjacent', damageStat: 'defense', multiplier: 1, status: 'Stun', statusDuration: 3, targets: 1 }
  },
  {
    id: 'whirlwind', name: 'Whirlwind', manaCost: 25, cooldown: 0, class: 'warrior',
    description: 'Spin and strike every adjacent enemy with your full attack.',
    effect: { type: 'meleeAoe', radius: 1, damageStat: 'attack', multiplier: 1 }
  },
  // --- Base Class Skills - Mage ---
  {
    id: 'fireball', name: 'Fireball', manaCost: 20, cooldown: 0, class: 'mage',
    description: 'Explosive fire in a radius of 3 that deals 20 damage and burns enemies for 5 turns.',
    effect: { type: 'aoeSpell', damage: 20, radius: 3, status: 'Burn', statusDuration: 5 }
  },
  {
    id: 'iceSpear', name: 'Ice Spear', manaCost: 20, cooldown: 0, class: 'mage',
    description: 'Freezing burst in a radius of 2 that deals 15 damage and freezes enemies for 5 turns.',
    effect: { type: 'aoeSpell', damage: 15, radius: 2, status: 'Freeze', statusDuration: 5 }
  },
  {
    id: 'lightning', name: 'Lightning', manaCost: 20, cooldown: 0, class: 'mage',
    description: 'Lightning strikes up to 3 random visible enemies for 25 damage each.',
    effect: { type: 'randomStrikes', damage: 25, targets: 3, requiresVisible: true }
  },
  {
    id: 'teleport', name: 'Teleport', manaCost: 30, cooldown: 0, class: 'mage',
    description: 'Teleport to a random room on the floor.',
    effect: { type: 'teleportRandom' }
  },
  // --- Base Class Skills - Rogue ---
  {
    id: 'backstab', name: 'Backstab', manaCost: 15, cooldown: 0, class: 'rogue',
    description: 'Strike the nearest enemy for triple attack damage.',
    effect: { type: 'strikeNearest', damageStat: 'attack', multiplier: 3 }
  },
  {
    id: 'shadowStep', name: 'Shadow Step', manaCost: 15, cooldown: 0, class: 'rogue',
    description: 'Step through shadows to appear beside a visible enemy.',
    effect: { type: 'teleportBehindEnemy' }
  },
  {
    id: 'poisonBlade', name: 'Poison Blade', manaCost: 15, cooldown: 0, class: 'rogue',
    description: 'Strike the nearest enemy with a poisoned blade (attack damage + Poison for 10 turns).',
    effect: { type: 'strikeNearest', damageStat: 'attack', multiplier: 1, status: 'Poison', statusDuration: 10 }
  },
  {
    id: 'vanish', name: 'Vanish', manaCost: 25, cooldown: 0, class: 'rogue',
    description: 'Vanish into the shadows, becoming invisible for 10 turns.',
    effect: { type: 'buffSelf', status: 'Invisibility', duration: 10 }
  },
  // --- Base Class Skills - Paladin ---
  {
    id: 'holyLight', name: 'Holy Light', manaCost: 20, cooldown: 0, class: 'paladin',
    description: 'Heal yourself for 20 + 2 per level and burn all visible undead for 30 damage.',
    effect: { type: 'healAndSmiteUndead', healBase: 20, healPerLevel: 2, undeadDamage: 30 }
  },
  {
    id: 'divineShield', name: 'Divine Shield', manaCost: 20, cooldown: 0, class: 'paladin',
    description: 'A divine shield protects you for 5 turns.',
    effect: { type: 'buffSelf', status: 'Shield', duration: 5 }
  },
  {
    id: 'smite', name: 'Smite', manaCost: 20, cooldown: 0, class: 'paladin',
    description: 'Smite a visible enemy for double attack damage (doubled again against undead).',
    effect: { type: 'strikeFirstVisible', damageStat: 'attack', multiplier: 2, undeadMultiplier: 2 }
  },
  {
    id: 'consecrate', name: 'Consecrate', manaCost: 35, cooldown: 0, class: 'paladin',
    description: 'Consecrate the ground beneath you, creating a shrine tile.',
    effect: { type: 'createTile', tile: 'Shrine' }
  },
  // --- Base Class Skills - Ranger ---
  {
    id: 'multiShot', name: 'Multi-Shot', manaCost: 15, cooldown: 0, class: 'ranger',
    description: 'Fire arrows at up to 3 visible enemies for full attack damage each.',
    effect: { type: 'multiStrike', damageStat: 'attack', multiplier: 1, targets: 3, requiresVisible: true }
  },
  {
    id: 'poisonArrow', name: 'Poison Arrow', manaCost: 15, cooldown: 0, class: 'ranger',
    description: 'Shoot the nearest enemy with a poisoned arrow (attack damage + Poison for 10 turns).',
    effect: { type: 'strikeNearest', damageStat: 'attack', multiplier: 1, status: 'Poison', statusDuration: 10 }
  },
  {
    id: 'trapSet', name: 'Set Trap', manaCost: 15, cooldown: 0, class: 'ranger',
    description: 'Set a trap on the tile beneath you.',
    effect: { type: 'createTile', tile: 'Trap' }
  },
  {
    id: 'eagleEye', name: 'Eagle Eye', manaCost: 10, cooldown: 0, class: 'ranger',
    description: 'Reveal the entire floor.',
    effect: { type: 'revealMap' }
  },
  // --- Base Class Skills - Necromancer ---
  {
    id: 'raiseDead', name: 'Raise Dead', manaCost: 40, cooldown: 0, class: 'necromancer',
    description: 'Raise a skeleton minion beside you with attack equal to half of yours.',
    effect: { type: 'summon', creature: 'Skeleton', attackStat: 'attack', attackMultiplier: 0.5 }
  },
  {
    id: 'lifeDrain', name: 'Life Drain', manaCost: 20, cooldown: 0, class: 'necromancer',
    description: 'Drain 15 HP from a visible enemy, healing yourself for the damage dealt.',
    effect: { type: 'drain', damage: 15, healEqualsDamage: true }
  },
  {
    id: 'curse', name: 'Curse', manaCost: 20, cooldown: 0, class: 'necromancer',
    description: 'Curse all visible enemies with Weakness for 10 turns.',
    effect: { type: 'debuffAllVisible', status: 'Weakness', duration: 10 }
  },
  {
    id: 'darkPact', name: 'Dark Pact', manaCost: 50, cooldown: 0, class: 'necromancer',
    description: 'Sacrifice a quarter of your HP to restore full mana and gain Strength for 15 turns.',
    effect: { type: 'sacrifice', hpCostFraction: 0.25, restoreManaFull: true, status: 'Strength', statusDuration: 15 }
  },

  // --- Subclass Skills - Warrior: Berserker ---
  {
    id: 'rage', name: 'Rage', manaCost: 30, cooldown: 0, class: null,
    description: 'Channel primal fury into a powerful blow.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'reckless', name: 'Reckless', manaCost: 30, cooldown: 0, class: null,
    description: 'Abandon caution for a devastating attack.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'bloodFrenzy', name: 'Blood Frenzy', manaCost: 30, cooldown: 0, class: null,
    description: 'A frenzied strike fueled by the scent of blood.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warrior: Knight ---
  {
    id: 'shieldWall', name: 'Shield Wall', manaCost: 30, cooldown: 0, class: null,
    description: 'Slam forward behind your raised shield.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'fortify', name: 'Fortify', manaCost: 30, cooldown: 0, class: null,
    description: 'Brace and strike with fortified resolve.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'rally', name: 'Rally', manaCost: 30, cooldown: 0, class: null,
    description: 'A rallying blow that inspires the charge.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warrior: Gladiator ---
  {
    id: 'dualStrike', name: 'Dual Strike', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike with both weapons in one motion.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'riposte', name: 'Riposte', manaCost: 30, cooldown: 0, class: null,
    description: 'A duelist counter-thrust.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'flurryOfBlows', name: 'Flurry of Blows', manaCost: 30, cooldown: 0, class: null,
    description: 'A rapid flurry of strikes.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warrior: Warlord ---
  {
    id: 'battleCry', name: 'Battle Cry', manaCost: 30, cooldown: 0, class: null,
    description: 'A commanding shout that precedes a crushing blow.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'devastate', name: 'Devastate', manaCost: 30, cooldown: 0, class: null,
    description: 'A devastating strike meant to break lines.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'conqueror', name: 'Conqueror', manaCost: 30, cooldown: 0, class: null,
    description: "The conqueror's finishing blow.",
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warrior: Paladin (subclass) ---
  {
    id: 'holySmite', name: 'Holy Smite', manaCost: 30, cooldown: 0, class: null,
    description: 'A smite charged with holy power.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'divineAura', name: 'Divine Aura', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike wreathed in a divine aura.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'redemption', name: 'Redemption', manaCost: 30, cooldown: 0, class: null,
    description: 'A redeeming blow against the wicked.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warrior: Dark Knight ---
  {
    id: 'darkSlash', name: 'Dark Slash', manaCost: 30, cooldown: 0, class: null,
    description: 'A slash infused with dark power.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'soulReap', name: 'Soul Reap', manaCost: 30, cooldown: 0, class: null,
    description: 'Reap at the soul of your foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'corruption', name: 'Corruption', manaCost: 30, cooldown: 0, class: null,
    description: 'Corrupting darkness lashes the target.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warrior: Champion ---
  {
    id: 'execute', name: 'Execute', manaCost: 30, cooldown: 0, class: null,
    description: "An executioner's strike.",
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'gloryStrike', name: 'Glory Strike', manaCost: 30, cooldown: 0, class: null,
    description: 'A glorious strike for the arena crowd.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'unstoppable', name: 'Unstoppable', manaCost: 30, cooldown: 0, class: null,
    description: 'An unstoppable charge and blow.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Mage: Elementalist ---
  {
    id: 'fireBlast', name: 'Fire Blast', manaCost: 30, cooldown: 0, class: null,
    description: 'A concentrated blast of fire.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'frostNova', name: 'Frost Nova', manaCost: 30, cooldown: 0, class: null,
    description: 'A nova of biting frost.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'chainLightning', name: 'Chain Lightning', manaCost: 30, cooldown: 0, class: null,
    description: 'Lightning that leaps toward your foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Mage: Necromancer (subclass) ---
  {
    id: 'raiseSkeleton', name: 'Raise Skeleton', manaCost: 30, cooldown: 0, class: null,
    description: 'Call bones to strike at your enemy.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'deathCoil', name: 'Death Coil', manaCost: 30, cooldown: 0, class: null,
    description: 'A coil of death energy.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'boneArmor', name: 'Bone Armor', manaCost: 30, cooldown: 0, class: null,
    description: 'Shards of bone lash outward.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Mage: Enchanter ---
  {
    id: 'empower', name: 'Empower', manaCost: 30, cooldown: 0, class: null,
    description: 'An empowered arcane strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'magicShield', name: 'Magic Shield', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike from behind a shimmering ward.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'haste', name: 'Haste', manaCost: 30, cooldown: 0, class: null,
    description: 'A hastened, blurring attack.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Mage: Archmage ---
  {
    id: 'meteor', name: 'Meteor', manaCost: 30, cooldown: 0, class: null,
    description: 'Call a meteor down upon your foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'elementalMastery', name: 'Elemental Mastery', manaCost: 30, cooldown: 0, class: null,
    description: 'Masterful elemental force unleashed.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'arcaneNova', name: 'Arcane Nova', manaCost: 30, cooldown: 0, class: null,
    description: 'A nova of pure arcane power.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Mage: Lich Lord ---
  {
    id: 'armyOfDead', name: 'Army of Dead', manaCost: 30, cooldown: 0, class: null,
    description: 'The dead surge forward at your command.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'deathGrip', name: 'Death Grip', manaCost: 30, cooldown: 0, class: null,
    description: "Death's grip crushes your foe.",
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'lichform', name: 'Lichform', manaCost: 30, cooldown: 0, class: null,
    description: 'Channel the power of your undead form.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Mage: Sage ---
  {
    id: 'timeWarp', name: 'Time Warp', manaCost: 30, cooldown: 0, class: null,
    description: 'Warp time to land an impossible blow.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'omniscience', name: 'Omniscience', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike with perfect foresight.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'transcendence', name: 'Transcendence', manaCost: 30, cooldown: 0, class: null,
    description: 'A transcendent surge of power.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Rogue: Assassin ---
  {
    id: 'deadlyStrike', name: 'Deadly Strike', manaCost: 30, cooldown: 0, class: null,
    description: 'A strike aimed at a vital point.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'ambush', name: 'Ambush', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike from ambush.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'poisonDagger', name: 'Poison Dagger', manaCost: 30, cooldown: 0, class: null,
    description: 'A dagger coated in lethal venom.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Rogue: Shadow ---
  {
    id: 'shadowMeld', name: 'Shadow Meld', manaCost: 30, cooldown: 0, class: null,
    description: 'Meld with shadow and strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'shadowStrike', name: 'Shadow Strike', manaCost: 30, cooldown: 0, class: null,
    description: 'A strike launched from darkness.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Rogue: Trickster ---
  {
    id: 'throwKnife', name: 'Throw Knife', manaCost: 30, cooldown: 0, class: null,
    description: 'Hurl a knife at your target.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'smokeScreen', name: 'Smoke Screen', manaCost: 30, cooldown: 0, class: null,
    description: 'Attack under cover of smoke.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'trapMaster', name: 'Trap Master', manaCost: 30, cooldown: 0, class: null,
    description: 'A cunning, trap-laced assault.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Rogue: Shadow Blade ---
  {
    id: 'shadowDance', name: 'Shadow Dance', manaCost: 30, cooldown: 0, class: null,
    description: 'Dance between shadows, blades flashing.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'deathMark', name: 'Death Mark', manaCost: 30, cooldown: 0, class: null,
    description: 'Mark your prey for death.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'eviscerate', name: 'Eviscerate', manaCost: 30, cooldown: 0, class: null,
    description: 'A vicious eviscerating slash.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Rogue: Nightstalker ---
  {
    id: 'phantomStrike', name: 'Phantom Strike', manaCost: 30, cooldown: 0, class: null,
    description: 'A strike no one sees coming.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'assassination', name: 'Assassination', manaCost: 30, cooldown: 0, class: null,
    description: 'A perfectly executed kill attempt.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'voidStep', name: 'Void Step', manaCost: 30, cooldown: 0, class: null,
    description: 'Step through the void to strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Rogue: Master Thief ---
  {
    id: 'pickpocket', name: 'Pickpocket', manaCost: 30, cooldown: 0, class: null,
    description: 'A quick strike with light fingers.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'grandHeist', name: 'Grand Heist', manaCost: 30, cooldown: 0, class: null,
    description: 'The boldest strike of all.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'luckOfThief', name: 'Luck of Thief', manaCost: 30, cooldown: 0, class: null,
    description: "A lucky thief's opportunistic blow.",
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Cleric/Paladin: Priest ---
  {
    id: 'heal', name: 'Heal', manaCost: 30, cooldown: 0, class: null,
    description: 'Channel healing light against your foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'blessing', name: 'Blessing', manaCost: 30, cooldown: 0, class: null,
    description: 'A blessed strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'purify', name: 'Purify', manaCost: 30, cooldown: 0, class: null,
    description: 'Purifying light sears the target.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Cleric/Paladin: Inquisitor ---
  {
    id: 'holyStrike', name: 'Holy Strike', manaCost: 30, cooldown: 0, class: null,
    description: 'A strike of holy retribution.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'judgment', name: 'Judgment', manaCost: 30, cooldown: 0, class: null,
    description: 'Pass judgment upon the wicked.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'exorcism', name: 'Exorcism', manaCost: 30, cooldown: 0, class: null,
    description: 'An exorcising burst of holy wrath.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Cleric/Paladin: Monk ---
  {
    id: 'palmStrike', name: 'Palm Strike', manaCost: 30, cooldown: 0, class: null,
    description: 'A focused open-palm strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'innerPeace', name: 'Inner Peace', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike from a place of perfect calm.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Cleric/Paladin: High Priest ---
  {
    id: 'miracle', name: 'Miracle', manaCost: 30, cooldown: 0, class: null,
    description: 'A miraculous surge of divine power.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'divineIntervention', name: 'Divine Intervention', manaCost: 30, cooldown: 0, class: null,
    description: 'The divine intervenes on your behalf.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'resurrection', name: 'Resurrection', manaCost: 30, cooldown: 0, class: null,
    description: 'Life-giving power turned against the foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Cleric/Paladin: Templar ---
  {
    id: 'crusaderStrike', name: 'Crusader Strike', manaCost: 30, cooldown: 0, class: null,
    description: "A crusader's righteous blow.",
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'holyWrath', name: 'Holy Wrath', manaCost: 30, cooldown: 0, class: null,
    description: 'Unleash holy wrath.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'zealot', name: 'Zealot', manaCost: 30, cooldown: 0, class: null,
    description: 'A zealous, unrelenting assault.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Cleric/Paladin: Grandmaster ---
  {
    id: 'quiveringPalm', name: 'Quivering Palm', manaCost: 30, cooldown: 0, class: null,
    description: 'The legendary quivering palm technique.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'enlightenment', name: 'Enlightenment', manaCost: 30, cooldown: 0, class: null,
    description: 'An enlightened, flawless strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'perfectForm', name: 'Perfect Form', manaCost: 30, cooldown: 0, class: null,
    description: 'A strike of perfect martial form.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Ranger: Beastmaster ---
  {
    id: 'callPet', name: 'Call Pet', manaCost: 30, cooldown: 0, class: null,
    description: 'Your companion lunges at the foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'beastBond', name: 'Beast Bond', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike as one with your beast.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'packTactics', name: 'Pack Tactics', manaCost: 30, cooldown: 0, class: null,
    description: 'Coordinated pack assault.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Ranger: Archer ---
  {
    id: 'aimedShot', name: 'Aimed Shot', manaCost: 30, cooldown: 0, class: null,
    description: 'A carefully aimed shot.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'piercingArrow', name: 'Piercing Arrow', manaCost: 30, cooldown: 0, class: null,
    description: 'An arrow that punches through armor.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Ranger: Druid ---
  {
    id: 'entangle', name: 'Entangle', manaCost: 30, cooldown: 0, class: null,
    description: 'Grasping vines batter the target.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'natureFury', name: 'Nature Fury', manaCost: 30, cooldown: 0, class: null,
    description: "Nature's fury lashes out.",
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'rejuvenate', name: 'Rejuvenate', manaCost: 30, cooldown: 0, class: null,
    description: 'Vital energy surges into a strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Ranger: Alpha ---
  {
    id: 'alphaRoar', name: 'Alpha Roar', manaCost: 30, cooldown: 0, class: null,
    description: "The alpha's roar precedes the pounce.",
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'beastMaster', name: 'Beast Master', manaCost: 30, cooldown: 0, class: null,
    description: 'Command every beast to strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'stampede', name: 'Stampede', manaCost: 30, cooldown: 0, class: null,
    description: 'A thundering stampede tramples the foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Ranger: Marksman ---
  {
    id: 'headshot', name: 'Headshot', manaCost: 30, cooldown: 0, class: null,
    description: 'A precise shot to the head.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'rapidFire', name: 'Rapid Fire', manaCost: 30, cooldown: 0, class: null,
    description: 'A rapid volley of arrows.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'killShot', name: 'Kill Shot', manaCost: 30, cooldown: 0, class: null,
    description: 'The shot meant to end the fight.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Ranger: Archdruid ---
  {
    id: 'natureWrath', name: 'Nature Wrath', manaCost: 30, cooldown: 0, class: null,
    description: 'The full wrath of the wild.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'treeForm', name: 'Tree Form', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike with limbs of living wood.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'forceOfNature', name: 'Force of Nature', manaCost: 30, cooldown: 0, class: null,
    description: 'An unstoppable force of nature.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warlock: Demonologist ---
  {
    id: 'summonImp', name: 'Summon Imp', manaCost: 30, cooldown: 0, class: null,
    description: 'An imp harries your target.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'demonBolt', name: 'Demon Bolt', manaCost: 30, cooldown: 0, class: null,
    description: 'A bolt of demonic energy.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'hellfire', name: 'Hellfire', manaCost: 30, cooldown: 0, class: null,
    description: 'Hellfire scorches the target.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warlock: Blood Mage ---
  {
    id: 'bloodBolt', name: 'Blood Bolt', manaCost: 30, cooldown: 0, class: null,
    description: 'A bolt of crystallized blood.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'lifeTap', name: 'Life Tap', manaCost: 30, cooldown: 0, class: null,
    description: 'Tap life essence to fuel a strike.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'bloodShield', name: 'Blood Shield', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike from behind a shield of blood.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warlock: Hexer ---
  {
    id: 'hex', name: 'Hex', manaCost: 30, cooldown: 0, class: null,
    description: 'A vicious hex wracks the target.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'doomCurse', name: 'Doom Curse', manaCost: 30, cooldown: 0, class: null,
    description: 'A curse of impending doom.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'weakness', name: 'Weakness', manaCost: 30, cooldown: 0, class: null,
    description: 'Sap the strength from your foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warlock: Demon Lord ---
  {
    id: 'summonDemon', name: 'Summon Demon', manaCost: 30, cooldown: 0, class: null,
    description: 'A greater demon answers your call.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'infernalPact', name: 'Infernal Pact', manaCost: 30, cooldown: 0, class: null,
    description: 'Infernal power sealed in a pact.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'demonicForm', name: 'Demonic Form', manaCost: 30, cooldown: 0, class: null,
    description: 'Strike in your demonic aspect.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warlock: Blood Lord ---
  {
    id: 'exsanguinate', name: 'Exsanguinate', manaCost: 30, cooldown: 0, class: null,
    description: 'Drain the blood from your victim.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'crimsonPact', name: 'Crimson Pact', manaCost: 30, cooldown: 0, class: null,
    description: 'A crimson pact fuels the blow.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'bloodNova', name: 'Blood Nova', manaCost: 30, cooldown: 0, class: null,
    description: 'A nova of boiling blood.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  // --- Subclass Skills - Warlock: Curse Weaver ---
  {
    id: 'doom', name: 'Doom', manaCost: 30, cooldown: 0, class: null,
    description: 'Pronounce doom upon your foe.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'curseOfAgony', name: 'Curse of Agony', manaCost: 30, cooldown: 0, class: null,
    description: 'A curse of pure agony.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  },
  {
    id: 'pandemonium', name: 'Pandemonium', manaCost: 30, cooldown: 0, class: null,
    description: 'Unleash utter pandemonium.',
    effect: { type: 'genericStrike', damageStat: 'attack', bonusDamage: 10, targets: 1, requiresVisible: true }
  }
];

SC.DATA.skills_notes = [
  'All 132 Skill enum variants are ported. manaCost values are verbatim from Skill::mana_cost: the 24 base-class skills have explicit costs; every subclass/advanced skill uses the source fallback arm (_ => 30).',
  'The Rust Skill impl has no cooldown() function and the game imposes no skill cooldowns (skills are gated by mana only), so cooldown is 0 for every skill; the fn cooldown at main.rs:3152 belongs to WeaponAbility, a different system.',
  'Names for the 24 base skills are verbatim from Skill::name; the Rust name() returns "Unknown Skill" for all subclass skills, so their display names were derived mechanically from the enum variant identifiers (e.g. BloodFrenzy -> "Blood Frenzy").',
  'The Rust source has no Skill::description; base-skill descriptions were derived from the actual use_skill implementation (main.rs ~7475), and subclass-skill descriptions are short flavor lines derived from the skill/subclass names.',
  'effect objects for the 24 base skills encode the exact use_skill behavior (damage numbers, status effects and durations verbatim). All subclass/advanced skills share the source default arm: a single strike on the first visible enemy for total_attack + 10 (encoded as type genericStrike, bonusDamage 10).',
  'class is the base CharacterClass that learns the skill via Skill::for_class, or null for subclass skills (subclass ownership is in SC.DATA.subclasses[].skills; note some skills are shared, e.g. rage, shieldWall, dualStrike, fireBlast, multiShot appear in multiple lists).',
  'Status effect names (Strength, Stun, Burn, Freeze, Poison, Invisibility, Shield, Weakness) are the StatusEffect enum variants used by use_skill.'
];

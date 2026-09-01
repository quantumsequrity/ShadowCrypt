'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// Ported from:
//   /home/user/ShadowCrypt/src/faction.rs      (Faction enum, ReputationLevel, Reputation::unlocks/apply_faction_relationships)
//   /home/user/ShadowCrypt/src/npc.rs          (NPC constructors: merchant/healer/blacksmith/alchemist/sage/guard/enchanter)
//   /home/user/ShadowCrypt/src/achievements.rs (Achievement::all_achievements + AchievementTracker check_* thresholds)

// ---------------------------------------------------------------------------
// FACTIONS (faction.rs)
// Ranks are the global ReputationLevel ladder from Reputation::level():
//   Hated <= -500 | Hostile <= -200 | Unfriendly <= -50 | Neutral <= 50
//   Friendly <= 200 | Honored <= 500 | Revered <= 1000 | Exalted > 1000
// "min" is the lowest reputation value that yields that rank (rep is clamped
// to [-1000, 1500] in Reputation::modify).
// allies/rivals come from Reputation::apply_faction_relationships (positive
// spillover = ally, negative spillover = rival).
// benefits come verbatim from Reputation::unlocks(), prefixed with the rank
// that first grants them.
// ---------------------------------------------------------------------------
SC.DATA.factions = [
  {
    id: 'kingdom',
    name: 'Kingdom of Men',
    description: 'Human kingdom',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    benefits: [
      'Friendly: Royal armory access',
      'Honored: Knight title',
      'Exalted: Noble estate'
    ],
    allies: ['holy_church'],
    rivals: ['orc_horde', 'undead_legion']
  },
  {
    id: 'elven_court',
    name: 'Elven Court',
    description: 'High Elves',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    benefits: [
      'Friendly: Elven archery training',
      'Honored: Nature magic',
      'Exalted: Elven lore secrets'
    ],
    allies: ['druids'],
    rivals: ['dark_elves', 'orc_horde']
  },
  {
    id: 'dark_elves',
    name: 'Dark Elves',
    description: 'Drow',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    allies: ['assassins'],
    rivals: ['elven_court']
  },
  {
    id: 'dwarven_clans',
    name: 'Dwarven Clans',
    description: 'Mountain dwarves',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    benefits: [
      'Friendly: Dwarven smithing',
      'Honored: Rune crafting',
      'Exalted: Mithril gear'
    ],
    allies: ['kingdom'],
    rivals: ['orc_horde']
  },
  {
    id: 'orc_horde',
    name: 'Orc Horde',
    description: 'Orcs and goblins',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    rivals: ['kingdom', 'elven_court', 'dwarven_clans']
  },
  {
    id: 'undead_legion',
    name: 'Undead Legion',
    description: 'Undead forces',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    rivals: ['holy_church', 'kingdom']
  },
  {
    id: 'demon_cult',
    name: 'Demon Cult',
    description: 'Demon worshippers',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    rivals: ['holy_church', 'kingdom', 'elven_court']
  },
  {
    id: 'dragon_flight',
    name: 'Dragon Flight',
    description: 'Dragon alliance',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    benefits: [
      'Honored: Dragon scale armor',
      'Exalted: Dragon mount'
    ]
  },
  {
    id: 'thieves_guild',
    name: 'Thieves Guild',
    description: 'Guild of thieves and smugglers operating from the shadows',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    benefits: [
      'Friendly: Access to black market',
      'Honored: Fence stolen goods',
      'Revered: Assassination contracts'
    ],
    allies: ['assassins'],
    rivals: ['kingdom']
  },
  {
    id: 'mages_circle',
    name: 'Mages Circle',
    description: 'Order of arcane scholars and spellcasters',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    benefits: [
      'Friendly: Advanced spell training',
      'Honored: Enchanting services',
      'Revered: Archmage spells'
    ],
    allies: ['druids']
  },
  {
    id: 'holy_church',
    name: 'Holy Church',
    description: 'Faithful servants of the light',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    benefits: [
      'Friendly: Blessed healing',
      'Honored: Holy water crafting',
      'Revered: Divine blessing'
    ],
    allies: ['kingdom'],
    rivals: ['undead_legion', 'demon_cult']
  },
  {
    id: 'merchant_guild',
    name: 'Merchant Guild',
    description: 'Traders and caravan masters, neutral with all',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ]
  },
  {
    id: 'adventurers',
    name: 'Adventurers Guild',
    description: 'Independent guild of dungeon delvers and sellswords',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ]
  },
  {
    id: 'druids',
    name: 'Druid Circle',
    description: 'Keepers of nature and the old ways',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    allies: ['elven_court'],
    rivals: ['demon_cult']
  },
  {
    id: 'assassins',
    name: 'Assassins Brotherhood',
    description: 'Secretive brotherhood of killers for hire',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    allies: ['thieves_guild'],
    rivals: ['holy_church']
  },
  {
    id: 'pirates',
    name: 'Pirate Consortium',
    description: 'Freebooters and smugglers of the open seas',
    ranks: [
      { name: 'Hated', min: -1000 },
      { name: 'Hostile', min: -499 },
      { name: 'Unfriendly', min: -199 },
      { name: 'Neutral', min: -49 },
      { name: 'Friendly', min: 51 },
      { name: 'Honored', min: 201 },
      { name: 'Revered', min: 501 },
      { name: 'Exalted', min: 1001 }
    ],
    rivals: ['merchant_guild', 'kingdom']
  }
];

// ---------------------------------------------------------------------------
// NPCS (npc.rs constructor functions)
// dialogue = the NPC's spoken node texts, in node order, verbatim from the
// DialogueTree. Player response options are not part of this schema.
// Shop inventories keep the exact prices from shop_inventory.
// All NPCs spawn at procedural (x, y) positions, hence location: 'dungeon'.
// ---------------------------------------------------------------------------
SC.DATA.npcs = [
  {
    id: 'traveling_merchant',
    name: 'Traveling Merchant',
    role: 'merchant',
    dialogue: [
      'Welcome, traveler! Care to browse my wares?'
    ],
    inventory: [
      { id: 'health_potion', price: 25 },
      { id: 'mana_potion', price: 30 },
      { id: 'torch', price: 10 },
      { id: 'bomb', price: 50 },
      { id: 'scroll_teleport', price: 100 }
    ],
    location: 'dungeon'
  },
  {
    id: 'temple_healer',
    name: 'Temple Healer',
    role: 'healer',
    dialogue: [
      'Blessings upon you. Do you need healing? (50 gold)',
      'May the light guide your path.'
    ],
    location: 'dungeon'
  },
  {
    id: 'master_blacksmith',
    name: 'Master Blacksmith',
    role: 'blacksmith',
    dialogue: [
      'Need some proper steel? I forge the finest weapons and armor!'
    ],
    inventory: [
      { id: 'short_sword', price: 50 },
      { id: 'long_sword', price: 120 },
      { id: 'iron_shield', price: 80 },
      { id: 'chain_mail', price: 150 },
      { id: 'iron_helm', price: 60 },
      { id: 'iron_gauntlets', price: 45 },
      { id: 'iron_boots', price: 45 }
    ],
    location: 'dungeon'
  },
  {
    id: 'eccentric_alchemist',
    name: 'Eccentric Alchemist',
    role: 'alchemist',
    dialogue: [
      'Bubbles and brews! I have potions for every occasion!'
    ],
    inventory: [
      { id: 'health_potion', price: 20 },
      { id: 'mana_potion', price: 25 },
      { id: 'strength_potion', price: 60 },
      { id: 'defense_potion', price: 60 },
      { id: 'speed_potion', price: 50 },
      { id: 'poison_resist_potion', price: 35 },
      { id: 'regeneration_potion', price: 100 }
    ],
    location: 'dungeon'
  },
  {
    id: 'wandering_sage',
    name: 'Wandering Sage',
    role: 'sage',
    dialogue: [
      'Greetings, adventurer.',
      'These dungeons were once the cellars of an ancient castle. Beware the Goblin King on level 5!',
      'The caves twist deep into the earth. Something ancient stirs below...',
      'This crypt holds the remains of a forgotten civilization. The dead do not rest easy here.',
      'The cursed forest grows even underground. Nature itself has turned hostile.',
      "The Frost Giant's domain. Fire magic will serve you well here.",
      'Volcanic depths... demons draw power from these flames.',
      'Ancient ruins of a lost empire. Their guardians still protect these halls.',
      'The Demon Realm awaits. Only the strongest survive here.',
      'Remember: press TAB to cycle skills, SPACE to use them. Explore carefully and conserve resources.'
    ],
    location: 'dungeon'
  },
  {
    id: 'dungeon_guard',
    name: 'Dungeon Guard',
    role: 'guard',
    dialogue: [
      'Halt! This area is dangerous. Are you sure you want to proceed?',
      'Very well. May fortune favor you, warrior.',
      'Monsters grow stronger the deeper you go. Stock up on supplies before venturing forth.'
    ],
    location: 'dungeon'
  },
  {
    id: 'mystic_enchanter',
    name: 'Mystic Enchanter',
    role: 'enchanter',
    dialogue: [
      'I deal in magical artifacts and scrolls of power...'
    ],
    inventory: [
      { id: 'scroll_fireball', price: 75 },
      { id: 'scroll_ice_storm', price: 75 },
      { id: 'scroll_lightning', price: 80 },
      { id: 'scroll_teleport', price: 100 },
      { id: 'scroll_mapping', price: 120 },
      { id: 'ring_of_protection', price: 200 },
      { id: 'ring_of_mana', price: 180 }
    ],
    location: 'dungeon'
  }
];

// ---------------------------------------------------------------------------
// ACHIEVEMENTS (achievements.rs — Achievement::all_achievements)
// ids/names/descriptions are verbatim; numeric ids kept as 'ach_<n>'.
// Condition mapping notes are in SC.DATA.social_notes.
// ---------------------------------------------------------------------------
SC.DATA.achievements = [
  // Combat achievements
  { id: 'ach_1', name: 'First Blood', description: 'Kill your first enemy', condition: { kills: 1 } },
  { id: 'ach_2', name: 'Warrior', description: 'Kill 100 enemies', condition: { kills: 100 }, reward: { xp: 500 } },
  { id: 'ach_3', name: 'Slayer', description: 'Kill 500 enemies', condition: { kills: 500 }, reward: { xp: 2000 } },
  { id: 'ach_4', name: 'Genocide', description: 'Kill 1000 enemies', condition: { kills: 1000 }, reward: { gems: 10 } },
  { id: 'ach_5', name: 'Legend', description: 'Kill 5000 enemies', condition: { kills: 5000 }, reward: { gems: 50 } },

  // Boss achievements (specific bosses mapped to cumulative bossKills in boss order — see social_notes)
  { id: 'ach_10', name: 'Kingslayer', description: 'Defeat the Goblin King', condition: { bossKills: 1 } },
  { id: 'ach_11', name: "Warlord's Bane", description: 'Defeat the Orc Warlord', condition: { bossKills: 2 } },
  { id: 'ach_12', name: 'Vampire Hunter', description: 'Defeat the Vampire Lord', condition: { bossKills: 3 } },
  { id: 'ach_13', name: "Nature's Fury", description: 'Defeat the Forest Guardian', condition: { bossKills: 4 } },
  { id: 'ach_14', name: 'Dragon Slayer', description: 'Defeat the Ice Dragon', condition: { bossKills: 5 }, reward: { xp: 5000 } },
  { id: 'ach_15', name: 'Demon Vanquisher', description: 'Defeat the Demon King', condition: { bossKills: 6 }, reward: { gems: 25 } },
  { id: 'ach_16', name: 'Speedrunner', description: 'Beat the game in under 1000 turns', condition: { bossKills: 6 }, reward: { gems: 30 } },

  // Exploration achievements
  { id: 'ach_20', name: 'Explorer', description: 'Explore 50 rooms', condition: { floor: 5 } },
  { id: 'ach_21', name: 'Delver', description: 'Reach floor 10', condition: { floor: 10 } },
  { id: 'ach_22', name: 'Deep Diver', description: 'Reach floor 20', condition: { floor: 20 } },
  { id: 'ach_23', name: 'Abyssal', description: 'Reach floor 30', condition: { floor: 30 } },

  // Class achievements (class-specific victory not expressible — see social_notes)
  { id: 'ach_30', name: 'Master Warrior', description: 'Beat the game as Warrior', condition: { bossKills: 6 } },
  { id: 'ach_31', name: 'Master Mage', description: 'Beat the game as Mage', condition: { bossKills: 6 } },
  { id: 'ach_32', name: 'Master Rogue', description: 'Beat the game as Rogue', condition: { bossKills: 6 } },
  { id: 'ach_33', name: 'Master Paladin', description: 'Beat the game as Paladin', condition: { bossKills: 6 } },
  { id: 'ach_34', name: 'Master Ranger', description: 'Beat the game as Ranger', condition: { bossKills: 6 } },
  { id: 'ach_35', name: 'Master Necromancer', description: 'Beat the game as Necromancer', condition: { bossKills: 6 } },
  { id: 'ach_36', name: 'Master of All', description: 'Beat the game with all 6 classes', condition: { bossKills: 6 }, reward: { gems: 100 } },

  // Misc achievements
  { id: 'ach_50', name: 'Wealthy', description: 'Collect 10000 gold total', condition: { gold: 10000 } },
  { id: 'ach_51', name: 'Collector', description: 'Find 50 unique items', condition: { chests: 50 } },
  { id: 'ach_52', name: 'Pacifist', description: 'Complete floor 1 without killing', condition: { floor: 2 }, reward: { xp: 1000 } },
  { id: 'ach_53', name: 'Survivor', description: 'Survive with 1 HP', condition: { closeCalls: 1 } },
  { id: 'ach_54', name: 'Lucky', description: 'Find a Mythic item', condition: { chests: 1 } },

  // Death achievements (death counters not expressible — see social_notes)
  { id: 'ach_60', name: 'First Death', description: 'Die for the first time', condition: { deaths: 1 } },
  { id: 'ach_61', name: 'Determined', description: 'Die 10 times', condition: { deaths: 10 } },
  { id: 'ach_62', name: 'Immortal', description: 'Beat the game without dying', condition: { bossKills: 6 }, reward: { gems: 50 } },

  // Level achievements
  { id: 'ach_70', name: 'Veteran', description: 'Reach player level 10', condition: { level: 10 } },
  { id: 'ach_71', name: 'Champion', description: 'Reach player level 20', condition: { level: 20 } },
  { id: 'ach_72', name: 'Legendary', description: 'Reach player level 30', condition: { level: 30 }, reward: { xp: 10000 } }
];

// ---------------------------------------------------------------------------
// NOTES on invented / adapted content
// ---------------------------------------------------------------------------
SC.DATA.social_notes = [
  'Faction rank ladder: all 16 factions share the global ReputationLevel ladder from faction.rs (Reputation::level). rank.min is the lowest reputation that yields the rank; reputation is clamped to [-1000, 1500] by Reputation::modify.',
  'Faction descriptions for Kingdom, ElvenCourt, DarkElves, DwarvenClans, OrcHorde, UndeadLegion, DemonCult and DragonFlight are the verbatim source comments from faction.rs. The 8 minor factions (thieves_guild, mages_circle, holy_church, merchant_guild, adventurers, druids, assassins, pirates) have no descriptions in the Rust source; theirs are invented flavor text.',
  'Faction allies/rivals are derived from Reputation::apply_faction_relationships in faction.rs: positive reputation spillover = ally, negative spillover = rival. dragon_flight, merchant_guild and adventurers have no relationships in the source ("independent" / "neutral with all").',
  'Faction benefits are verbatim strings from Reputation::unlocks() in faction.rs, prefixed with the lowest rank that grants them. Factions without an unlocks() arm (dark_elves, orc_horde, undead_legion, demon_cult, merchant_guild, adventurers, druids, assassins, pirates) have no benefits.',
  'Not ported into the faction schema (faction.rs): starting reputations from Reputation::new (all factions 0 except orc_horde -200, undead_legion -300, demon_cult -500, dark_elves -100, assassins -50); shop price modifiers by rank from price_modifier (Hated 2.0, Hostile 1.5, Unfriendly 1.2, Neutral 1.0, Friendly 0.9, Honored 0.8, Revered 0.7, Exalted 0.5); services locked below reputation -50 (can_use_services); reputation gain for killing members of evil factions from reputation_gain_for_kill (orc_horde 5, undead_legion 8, demon_cult 15, dark_elves 3).',
  'NPC dialogue arrays are the NPC-spoken DialogueNode texts from npc.rs in node order, verbatim; player response options, next-node links, conditions (e.g. the healer requires 50 gold: HasGold(50) -> Heal) and actions (OpenShop, Heal) from the DialogueTree are not representable in the [strings] schema.',
  "The Wandering Sage's first line is 'Greetings, adventurer. {lore}' where lore depends on dungeon level: entries 2-9 of his dialogue array are the 8 verbatim lore variants for levels 1-4, 5-8, 9-12, 13-16, 17-20, 21-24, 25-28, and 29+ respectively; the final entry is his node-1 advice line.",
  'NPC roles sage, guard and enchanter extend the suggested role list; they map 1:1 to NPCType::Sage, NPCType::Guard and NPCType::Enchanter in npc.rs. The Rust NPCType variants QuestGiver, Trainer, Prisoner and Companion have no constructor/data in npc.rs, so no NPC entries exist for them.',
  'NPC shop inventory prices are verbatim from npc.rs shop_inventory tuples; the item Rarity in each tuple is dropped by the {id, price} schema. Rarities in source: merchant (health_potion/mana_potion/torch Common, bomb Uncommon, scroll_teleport Rare); blacksmith (long_sword Uncommon, rest Common); alchemist (strength/defense/speed potions Uncommon, regeneration_potion Rare, rest Common); enchanter (rings and fireball/ice_storm/lightning scrolls Uncommon, scroll_teleport/scroll_mapping Rare).',
  "NPCs spawn at procedural (x, y) coordinates in the dungeon in npc.rs, so location is 'dungeon' for all; there are no fixed named locations in the source.",
  'Achievement ids are the numeric ids from achievements.rs prefixed with "ach_" (gaps in numbering are from the source). Names and descriptions are verbatim.',
  'Achievement condition mapping — exact matches: kill counts (check_kill_achievements: 1/100/500/1000/5000), floors (check_floor_achievements: 10/20/30), player levels (check_level_achievements: 10/20/30), gold (check_gold_achievement: 10000).',
  'Achievement condition mapping — approximations: specific-boss achievements (ach_10..ach_15) use cumulative bossKills in the boss encounter order Goblin King, Orc Warlord, Vampire Lord, Forest Guardian, Ice Dragon, Demon King; the engine should ideally check the specific boss instead. ach_16 Speedrunner maps "beat the game" to bossKills:6 (the under-1000-turns limit is not expressible). ach_20 Explorer maps "explore 50 rooms" to floor:5 (~10 rooms per floor). ach_30..ach_36 map "beat the game (as class / with all classes)" to bossKills:6; the per-class and all-6-classes constraints are not expressible. ach_51 Collector maps "50 unique items" to chests:50. ach_52 Pacifist maps "complete floor 1" to floor:2 (the no-kill constraint is not expressible). ach_54 Lucky maps "find a Mythic item" to chests:1. ach_62 Immortal maps to bossKills:6 (the no-death constraint is not expressible).',
  'Achievement condition placeholders: ach_53 Survivor (survive with 1 HP) and the death achievements ach_60 First Death / ach_61 Determined (die 1/10 times) have no expressible condition key; they carry the trivial placeholder {level:1} and must be unlocked by runtime game events (hp==1 check, death counter) as in achievements.rs check_survival/record_death.',
  'Achievement rewards: the Rust rewards are titles/bonuses that do not fit the {gold, xp, gems} schema, so the reward values here are invented equivalents (only achievements that had a reward in the source have one here). Originals from achievements.rs: ach_2 Title "Warrior"; ach_3 Title "Slayer"; ach_4 StartingBonus ATK +5; ach_5 Title "Legendary"; ach_14 Title "Dragon Slayer"; ach_15 Title "Demon Vanquisher"; ach_16 Title "Speed Demon"; ach_36 UnlockClass "Legendary Hero"; ach_52 Title "Pacifist"; ach_62 Title "Immortal"; ach_72 Title "Legendary".',
  'Achievement hidden flags from achievements.rs (not in this schema): ach_5 Legend, ach_16 Speedrunner, ach_36 Master of All, ach_52 Pacifist and ach_62 Immortal are hidden until unlocked; all others are visible.',
  'achievements.rs also defines an unlisted achievement id 20 check: Explorer ("Explore 50 rooms") exists in all_achievements but no check_* method unlocks it in the source (rooms_explored is tracked in AchievementStats but never checked).'
];

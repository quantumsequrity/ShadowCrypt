'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// ============================================================================
// ShadowCrypt quest catalog - ported from src/quest.rs (create_all_quests(),
// 89 predefined quests). Values verbatim from source. Enemy target ids match
// the keys of SC.DATA.enemies in web/js/data/data_enemies.js.
// See SC.DATA.quests_notes for all porting decisions and adaptations.
// ============================================================================

SC.DATA.quests = [
  // ========== MAIN STORY QUESTS (IDs 1-13) ==========
  {
    "id": 1,
    "name": "The Descent Begins",
    "description": "Enter the dungeon and survive your first encounter with the darkness.",
    "type": "story",
    "difficulty": "trivial",
    "lore": "The ancient ShadowCrypt has stood sealed for a thousand years. Now, dark energy seeps from its depths, and you have been chosen to investigate.",
    "objectives": [
      { "kind": "explore", "target": 2, "count": 1 },
      { "kind": "kill", "target": "any", "count": 3 }
    ],
    "rewards": { "xp": 100, "gold": 50 },
    "next": 2,
    "chain": { "id": 1, "position": 1 }
  },
  {
    "id": 2,
    "name": "Whispers in the Dark",
    "description": "The walls seem to whisper. Find the source of the mysterious voices.",
    "type": "story",
    "difficulty": "easy",
    "lore": "As you delve deeper, you hear whispers that shouldn't exist. Ancient voices speak of a great evil awakening.",
    "objectives": [
      { "kind": "explore", "target": 5, "count": 1 },
      { "kind": "boss", "target": "bossGoblinKing", "count": 1 }
    ],
    "rewards": { "xp": 300, "gold": 150, "items": [{ "id": "health_potion", "count": 1, "rarity": "uncommon" }] },
    "requires": 1,
    "next": 3,
    "chain": { "id": 1, "position": 2 }
  },
  {
    "id": 3,
    "name": "The Goblin Menace",
    "description": "The Goblin King has fallen, but his army remains. Thin their numbers.",
    "type": "story",
    "difficulty": "easy",
    "objectives": [
      { "kind": "kill", "target": "goblin", "count": 15 },
      { "kind": "kill", "target": "any", "count": 10, "special": "source_target_goblin_archer" }
    ],
    "rewards": { "xp": 400, "gold": 200 },
    "requires": 2,
    "next": 4,
    "chain": { "id": 1, "position": 3 }
  },
  {
    "id": 4,
    "name": "Into the Catacombs",
    "description": "The path leads deeper, into ancient burial grounds where the dead do not rest.",
    "type": "story",
    "difficulty": "normal",
    "objectives": [
      { "kind": "explore", "target": 10, "count": 1 },
      { "kind": "kill", "target": "skeleton", "count": 20 }
    ],
    "rewards": { "xp": 600, "gold": 300, "items": [{ "id": "long_sword", "count": 1, "rarity": "rare" }] },
    "requires": 3,
    "next": 5,
    "chain": { "id": 1, "position": 4 }
  },
  {
    "id": 5,
    "name": "The Necromancer's Sanctum",
    "description": "A powerful necromancer has taken residence in the catacombs. End his dark rituals.",
    "type": "story",
    "difficulty": "normal",
    "objectives": [
      { "kind": "boss", "target": "bossOrcWarlord", "count": 1 }
    ],
    "rewards": { "xp": 800, "gold": 500, "items": [{ "id": "mana_potion", "count": 1, "rarity": "rare" }] },
    "requires": 4,
    "next": 6,
    "chain": { "id": 1, "position": 5 }
  },
  {
    "id": 6,
    "name": "Echoes of the Fallen",
    "description": "The necromancer's death has disturbed something ancient. Investigate the sealed chamber.",
    "type": "story",
    "difficulty": "hard",
    "objectives": [
      { "kind": "explore", "target": 15, "count": 1 },
      { "kind": "kill", "target": "wraith", "count": 10 }
    ],
    "rewards": { "xp": 1000, "gold": 600 },
    "requires": 5,
    "next": 7,
    "chain": { "id": 1, "position": 6 }
  },
  {
    "id": 7,
    "name": "The Frozen Depths",
    "description": "Ice and frost pervade these halls. Something cold and ancient awaits.",
    "type": "story",
    "difficulty": "hard",
    "objectives": [
      { "kind": "boss", "target": "bossVampireLord", "count": 1 }
    ],
    "rewards": { "xp": 1500, "gold": 800, "items": [{ "id": "frost_blade", "count": 1, "rarity": "epic" }] },
    "requires": 6,
    "next": 8,
    "chain": { "id": 1, "position": 7 }
  },
  {
    "id": 8,
    "name": "Heart of Ice",
    "description": "The Frost Giant's lair holds secrets of the demon's origin. Find the frozen tome.",
    "type": "story",
    "difficulty": "hard",
    "objectives": [
      { "kind": "explore", "target": 20, "count": 1 },
      { "kind": "collect", "target": "any", "count": 10 }
    ],
    "rewards": { "xp": 1800, "gold": 1000 },
    "requires": 7,
    "next": 9,
    "chain": { "id": 1, "position": 8 }
  },
  {
    "id": 9,
    "name": "The Dragon's Lair",
    "description": "An ancient dragon guards the path to the demon realm. Prove your worth.",
    "type": "story",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "boss", "target": "bossForestGuardian", "count": 1 }
    ],
    "rewards": { "xp": 2500, "gold": 1500, "items": [{ "id": "dragon_armor", "count": 1, "rarity": "epic" }] },
    "requires": 8,
    "next": 10,
    "chain": { "id": 1, "position": 9 }
  },
  {
    "id": 10,
    "name": "Gates of the Abyss",
    "description": "The demon realm's entrance lies before you. Steel yourself for what awaits.",
    "type": "story",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "explore", "target": 25, "count": 1 },
      { "kind": "kill", "target": "demon", "count": 15 }
    ],
    "rewards": { "xp": 3000, "gold": 2000 },
    "requires": 9,
    "next": 11,
    "chain": { "id": 1, "position": 10 }
  },
  {
    "id": 11,
    "name": "The Demon Lord's Challenge",
    "description": "A Demon Lord blocks your path. Defeat this lieutenant of darkness.",
    "type": "story",
    "difficulty": "nightmare",
    "objectives": [
      { "kind": "boss", "target": "bossIceDragon", "count": 1 }
    ],
    "rewards": { "xp": 4000, "gold": 2500, "items": [{ "id": "demon_slayer", "count": 1, "rarity": "legendary" }] },
    "requires": 10,
    "next": 12,
    "chain": { "id": 1, "position": 11 }
  },
  {
    "id": 12,
    "name": "The Final Descent",
    "description": "Floor 30 awaits. The Demon King's throne room lies at the bottom.",
    "type": "story",
    "difficulty": "nightmare",
    "objectives": [
      { "kind": "explore", "target": 30, "count": 1 }
    ],
    "rewards": { "xp": 5000, "gold": 3000 },
    "requires": 11,
    "next": 13,
    "chain": { "id": 1, "position": 12 }
  },
  {
    "id": 13,
    "name": "End of the Shadow",
    "description": "The Demon King must fall. The fate of the world rests on your shoulders.",
    "type": "story",
    "difficulty": "legendary",
    "objectives": [
      { "kind": "boss", "target": "bossDemonKing", "count": 1 }
    ],
    "rewards": { "xp": 10000, "gold": 10000, "title": "Savior of the Realm" },
    "requires": 12,
    "chain": { "id": 1, "position": 13 }
  },

  // ========== BOUNTY QUESTS (IDs 101-115) ==========
  {
    "id": 101,
    "name": "Rat Infestation",
    "description": "The dungeon's lower levels are overrun with rats. Clear them out.",
    "giver": "Wandering Sage",
    "type": "kill",
    "difficulty": "trivial",
    "objectives": [
      { "kind": "kill", "target": "rat", "count": 10 }
    ],
    "rewards": { "xp": 50, "gold": 25 }
  },
  {
    "id": 102,
    "name": "Spider Extermination",
    "description": "Giant spiders have nested in the caves. Destroy their webs.",
    "type": "kill",
    "difficulty": "trivial",
    "objectives": [
      { "kind": "kill", "target": "spider", "count": 8 }
    ],
    "rewards": { "xp": 75, "gold": 40 }
  },
  {
    "id": 103,
    "name": "Bat Swarm",
    "description": "A colony of bats is terrorizing travelers. Put an end to their menace.",
    "type": "kill",
    "difficulty": "trivial",
    "objectives": [
      { "kind": "kill", "target": "bat", "count": 12 }
    ],
    "rewards": { "xp": 60, "gold": 30 }
  },
  {
    "id": 104,
    "name": "Goblin Scouts",
    "description": "Goblin scouts have been spotted. Eliminate them before they report back.",
    "type": "kill",
    "difficulty": "easy",
    "objectives": [
      { "kind": "kill", "target": "goblin", "count": 10 }
    ],
    "rewards": { "xp": 100, "gold": 60 }
  },
  {
    "id": 105,
    "name": "Skeleton Patrol",
    "description": "Undead patrols roam the catacombs. Break their ranks.",
    "type": "kill",
    "difficulty": "easy",
    "objectives": [
      { "kind": "kill", "target": "skeleton", "count": 12 }
    ],
    "rewards": { "xp": 120, "gold": 75 },
    "minFloor": 5
  },
  {
    "id": 106,
    "name": "Zombie Uprising",
    "description": "The dead are rising. Put them back to rest permanently.",
    "type": "kill",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "zombie", "count": 15 }
    ],
    "rewards": { "xp": 150, "gold": 100 },
    "minFloor": 5
  },
  {
    "id": 107,
    "name": "Orc War Party",
    "description": "An orc war party has entered the dungeon. Stop their advance.",
    "type": "kill",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "orc", "count": 10 },
      { "kind": "kill", "target": "any", "count": 5, "special": "source_target_orc_warrior" }
    ],
    "rewards": { "xp": 200, "gold": 150 },
    "minFloor": 8
  },
  {
    "id": 108,
    "name": "Ghostly Presence",
    "description": "Ghosts haunt the lower halls. Dispel their spirits.",
    "type": "kill",
    "difficulty": "hard",
    "objectives": [
      { "kind": "kill", "target": "ghost", "count": 8 }
    ],
    "rewards": { "xp": 250, "gold": 200 },
    "minFloor": 10
  },
  {
    "id": 109,
    "name": "Wraith Hunters",
    "description": "Wraiths are particularly dangerous. Destroy them with care.",
    "type": "kill",
    "difficulty": "hard",
    "objectives": [
      { "kind": "kill", "target": "wraith", "count": 6 }
    ],
    "rewards": { "xp": 300, "gold": 250 },
    "minFloor": 12
  },
  {
    "id": 110,
    "name": "Vampire Hunt",
    "description": "A vampire coven threatens the region. End their bloodlust.",
    "type": "kill",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "kill", "target": "vampire", "count": 5 }
    ],
    "rewards": { "xp": 400, "gold": 350 },
    "minFloor": 15
  },
  {
    "id": 111,
    "name": "Troll Trouble",
    "description": "Trolls have made the caves their home. Evict them permanently.",
    "type": "kill",
    "difficulty": "hard",
    "objectives": [
      { "kind": "kill", "target": "troll", "count": 4 }
    ],
    "rewards": { "xp": 350, "gold": 300 },
    "minFloor": 13
  },
  {
    "id": 112,
    "name": "Dark Elf Assassins",
    "description": "Dark elf assassins have infiltrated the dungeon. Hunt them down.",
    "type": "kill",
    "difficulty": "hard",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 8, "special": "source_target_dark_elf" }
    ],
    "rewards": { "xp": 350, "gold": 275 },
    "minFloor": 12
  },
  {
    "id": 113,
    "name": "Werewolf Pack",
    "description": "A pack of werewolves prowls the night. Silver your weapons.",
    "type": "kill",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 4, "special": "source_target_werewolf" }
    ],
    "rewards": { "xp": 450, "gold": 400 },
    "minFloor": 16
  },
  {
    "id": 114,
    "name": "Demon Slayer",
    "description": "Demons have crossed into our realm. Send them back to the abyss.",
    "type": "kill",
    "difficulty": "nightmare",
    "objectives": [
      { "kind": "kill", "target": "demon", "count": 10 }
    ],
    "rewards": { "xp": 600, "gold": 500 },
    "minFloor": 20
  },
  {
    "id": 115,
    "name": "Lich's Bane",
    "description": "A lich threatens to raise an army of undead. Destroy its phylactery.",
    "type": "kill",
    "difficulty": "nightmare",
    "objectives": [
      { "kind": "kill", "target": "lich", "count": 2 }
    ],
    "rewards": { "xp": 800, "gold": 600 },
    "minFloor": 22
  },

  // ========== EXPLORATION QUESTS (IDs 201-207) ==========
  {
    "id": 201,
    "name": "Dungeon Delver",
    "description": "Explore the first few floors of the dungeon.",
    "type": "explore",
    "difficulty": "trivial",
    "objectives": [
      { "kind": "explore", "target": "any", "count": 10, "special": "explore_rooms" }
    ],
    "rewards": { "xp": 100, "gold": 50 }
  },
  {
    "id": 202,
    "name": "Cartographer",
    "description": "Map out a significant portion of the dungeon.",
    "type": "explore",
    "difficulty": "easy",
    "objectives": [
      { "kind": "explore", "target": "any", "count": 25, "special": "explore_rooms" }
    ],
    "rewards": { "xp": 200, "gold": 100, "items": [{ "id": "scroll_mapping", "count": 1, "rarity": "common" }] }
  },
  {
    "id": 203,
    "name": "Secret Seeker",
    "description": "Find hidden rooms within the dungeon.",
    "type": "explore",
    "difficulty": "normal",
    "objectives": [
      { "kind": "explore", "target": "any", "count": 1, "special": "find_secret_room" }
    ],
    "rewards": { "xp": 300, "gold": 200 }
  },
  {
    "id": 204,
    "name": "Deep Explorer",
    "description": "Venture deep into the dungeon's lower levels.",
    "type": "explore",
    "difficulty": "hard",
    "objectives": [
      { "kind": "explore", "target": 15, "count": 1 }
    ],
    "rewards": { "xp": 500, "gold": 300 }
  },
  {
    "id": 205,
    "name": "Abyss Walker",
    "description": "Reach the deepest known levels of the dungeon.",
    "type": "explore",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "explore", "target": 25, "count": 1 }
    ],
    "rewards": { "xp": 1000, "gold": 600 }
  },
  {
    "id": 206,
    "name": "Room by Room",
    "description": "Methodically explore every corner of the dungeon.",
    "type": "explore",
    "difficulty": "normal",
    "objectives": [
      { "kind": "explore", "target": "any", "count": 50, "special": "explore_rooms" }
    ],
    "rewards": { "xp": 400, "gold": 250 }
  },
  {
    "id": 207,
    "name": "Lost Passages",
    "description": "Discover the hidden pathways between floors.",
    "type": "explore",
    "difficulty": "hard",
    "objectives": [
      { "kind": "explore", "target": "any", "count": 3, "special": "find_all_secrets" }
    ],
    "rewards": { "xp": 600, "gold": 400 }
  },

  // ========== COLLECTION QUESTS (IDs 301-310) ==========
  {
    "id": 301,
    "name": "Potion Hoarder",
    "description": "Collect health potions for the infirmary.",
    "giver": "Temple Healer",
    "type": "collect",
    "difficulty": "easy",
    "objectives": [
      { "kind": "collect", "target": "health_potion", "count": 5 }
    ],
    "rewards": { "xp": 100, "gold": 75 }
  },
  {
    "id": 302,
    "name": "Mana Crystal Gathering",
    "description": "The Enchanters Guild needs mana potions for their experiments.",
    "giver": "Mystic Enchanter",
    "type": "collect",
    "difficulty": "easy",
    "objectives": [
      { "kind": "collect", "target": "mana_potion", "count": 5 }
    ],
    "rewards": { "xp": 100, "gold": 80 }
  },
  {
    "id": 303,
    "name": "Gold Rush",
    "description": "Accumulate wealth for the merchant's guild.",
    "giver": "Traveling Merchant",
    "type": "collect",
    "difficulty": "normal",
    "objectives": [
      { "kind": "collect", "target": "gold", "count": 500 }
    ],
    "rewards": { "xp": 200, "items": [{ "id": "ring_of_protection", "count": 1, "rarity": "uncommon" }] }
  },
  {
    "id": 304,
    "name": "Treasure Hunter",
    "description": "Find valuable items throughout the dungeon.",
    "type": "collect",
    "difficulty": "normal",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 15 }
    ],
    "rewards": { "xp": 250, "gold": 150 }
  },
  {
    "id": 305,
    "name": "Rare Finds",
    "description": "Locate rare quality items.",
    "type": "collect",
    "difficulty": "hard",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 3, "special": "rarity_rare" }
    ],
    "rewards": { "xp": 400, "gold": 300 }
  },
  {
    "id": 306,
    "name": "Epic Discovery",
    "description": "Find epic quality equipment.",
    "type": "collect",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 2, "special": "rarity_epic" }
    ],
    "rewards": { "xp": 800, "gold": 500 }
  },
  {
    "id": 307,
    "name": "Legendary Acquisition",
    "description": "Discover a legendary artifact.",
    "type": "collect",
    "difficulty": "nightmare",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 1, "special": "rarity_legendary" }
    ],
    "rewards": { "xp": 1500, "gold": 1000 }
  },
  {
    "id": 308,
    "name": "Scroll Collector",
    "description": "Gather magical scrolls for study.",
    "type": "collect",
    "difficulty": "normal",
    "objectives": [
      { "kind": "collect", "target": "scroll_fireball", "count": 2 },
      { "kind": "collect", "target": "scroll_ice_storm", "count": 2 }
    ],
    "rewards": { "xp": 300, "gold": 200 }
  },
  {
    "id": 309,
    "name": "Torch Bearer",
    "description": "Collect torches to light the dark passages.",
    "type": "collect",
    "difficulty": "trivial",
    "objectives": [
      { "kind": "collect", "target": "torch", "count": 10 }
    ],
    "rewards": { "xp": 50, "gold": 30 }
  },
  {
    "id": 310,
    "name": "Wealthy Adventurer",
    "description": "Amass a significant fortune.",
    "type": "collect",
    "difficulty": "hard",
    "objectives": [
      { "kind": "collect", "target": "gold", "count": 2000 }
    ],
    "rewards": { "xp": 600, "items": [{ "id": "ring_of_mana", "count": 1, "rarity": "rare" }] }
  },

  // ========== SKILL/CRAFTING QUESTS (IDs 401-407) ==========
  {
    "id": 401,
    "name": "Combat Training",
    "description": "Use your combat skills in battle.",
    "type": "side",
    "difficulty": "easy",
    "objectives": [
      { "kind": "skill", "target": "any", "count": 10, "special": "use_skills" }
    ],
    "rewards": { "xp": 150, "gold": 75 }
  },
  {
    "id": 402,
    "name": "Skill Master",
    "description": "Demonstrate mastery of your abilities.",
    "type": "side",
    "difficulty": "normal",
    "objectives": [
      { "kind": "skill", "target": "any", "count": 50, "special": "use_skills" }
    ],
    "rewards": { "xp": 400, "gold": 200 }
  },
  {
    "id": 403,
    "name": "Level Up",
    "description": "Gain experience and reach a new level.",
    "type": "side",
    "difficulty": "easy",
    "objectives": [
      { "kind": "level", "target": 5, "count": 5, "special": "reach_level" }
    ],
    "rewards": { "xp": 200, "gold": 100 }
  },
  {
    "id": 404,
    "name": "Power Growth",
    "description": "Continue growing stronger.",
    "type": "side",
    "difficulty": "normal",
    "objectives": [
      { "kind": "level", "target": 10, "count": 10, "special": "reach_level" }
    ],
    "rewards": { "xp": 500, "gold": 250 }
  },
  {
    "id": 405,
    "name": "Veteran Status",
    "description": "Become a seasoned adventurer.",
    "type": "side",
    "difficulty": "hard",
    "objectives": [
      { "kind": "level", "target": 15, "count": 15, "special": "reach_level" }
    ],
    "rewards": { "xp": 1000, "gold": 500 }
  },
  {
    "id": 406,
    "name": "Elite Warrior",
    "description": "Reach the pinnacle of power.",
    "type": "side",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "level", "target": 20, "count": 20, "special": "reach_level" }
    ],
    "rewards": { "xp": 2000, "gold": 1000 }
  },
  {
    "id": 407,
    "name": "Legendary Hero",
    "description": "Become a living legend.",
    "type": "side",
    "difficulty": "legendary",
    "objectives": [
      { "kind": "level", "target": 25, "count": 25, "special": "reach_level" }
    ],
    "rewards": { "xp": 5000, "gold": 2500, "title": "Living Legend" }
  },

  // ========== SURVIVAL QUESTS (IDs 501-505) ==========
  {
    "id": 501,
    "name": "Survivor",
    "description": "Survive in the dungeon for an extended period.",
    "type": "side",
    "difficulty": "normal",
    "objectives": [
      { "kind": "survive", "target": "any", "count": 200, "special": "survive_turns" }
    ],
    "rewards": { "xp": 300, "gold": 150 }
  },
  {
    "id": 502,
    "name": "Endurance Test",
    "description": "Test your limits of survival.",
    "type": "side",
    "difficulty": "hard",
    "objectives": [
      { "kind": "survive", "target": "any", "count": 500, "special": "survive_turns" }
    ],
    "rewards": { "xp": 600, "gold": 300 }
  },
  {
    "id": 503,
    "name": "Iron Will",
    "description": "Survive without healing for an extended period.",
    "type": "side",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "survive", "target": "any", "count": 100, "special": "survive_turns_without_healing" }
    ],
    "rewards": { "xp": 800, "gold": 400, "stats": { "hp": 10, "atk": 0, "def": 5, "mana": 0, "spd": 0 } }
  },
  {
    "id": 504,
    "name": "Unbreakable",
    "description": "Reach deep floors while maintaining high health.",
    "type": "side",
    "difficulty": "nightmare",
    "objectives": [
      { "kind": "explore", "target": 15, "count": 1, "special": "min_hp_percent_75" }
    ],
    "rewards": { "xp": 1200, "gold": 600 }
  },
  {
    "id": 505,
    "name": "Perfect Run",
    "description": "Reach the lower floors in peak condition.",
    "type": "side",
    "difficulty": "legendary",
    "objectives": [
      { "kind": "explore", "target": 20, "count": 1, "special": "min_hp_percent_90" }
    ],
    "rewards": { "xp": 2000, "gold": 1000, "title": "Untouchable" }
  },

  // ========== ARENA/COMBAT QUESTS (IDs 601-610) ==========
  {
    "id": 601,
    "name": "First Blood",
    "description": "Slay your first enemies.",
    "type": "arena",
    "difficulty": "trivial",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 5 }
    ],
    "rewards": { "xp": 50, "gold": 25 }
  },
  {
    "id": 602,
    "name": "Blood Bath",
    "description": "Prove yourself in combat.",
    "type": "arena",
    "difficulty": "easy",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 25 }
    ],
    "rewards": { "xp": 200, "gold": 100 }
  },
  {
    "id": 603,
    "name": "Warrior's Path",
    "description": "Walk the path of the warrior.",
    "type": "arena",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 50 }
    ],
    "rewards": { "xp": 400, "gold": 200 }
  },
  {
    "id": 604,
    "name": "Champion's Trial",
    "description": "Prove your worth as a champion.",
    "type": "arena",
    "difficulty": "hard",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 100 }
    ],
    "rewards": { "xp": 800, "gold": 400, "items": [{ "id": "greatsword", "count": 1, "rarity": "rare" }] }
  },
  {
    "id": 605,
    "name": "Slayer Supreme",
    "description": "Become a master of death.",
    "type": "arena",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 200 }
    ],
    "rewards": { "xp": 1500, "gold": 800 }
  },
  {
    "id": 606,
    "name": "Death Incarnate",
    "description": "Leave none standing in your wake.",
    "type": "arena",
    "difficulty": "nightmare",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 500 }
    ],
    "rewards": { "xp": 3000, "gold": 1500, "title": "Death Incarnate" }
  },
  {
    "id": 607,
    "name": "Boss Hunter",
    "description": "Defeat the dungeon's bosses.",
    "type": "arena",
    "difficulty": "hard",
    "objectives": [
      { "kind": "boss", "target": "any", "count": 1 }
    ],
    "rewards": { "xp": 500, "gold": 300 }
  },
  {
    "id": 608,
    "name": "Boss Slayer",
    "description": "Defeat multiple bosses.",
    "type": "arena",
    "difficulty": "veryHard",
    "objectives": [
      { "kind": "boss", "target": "any", "count": 3 }
    ],
    "rewards": { "xp": 1500, "gold": 800 }
  },
  {
    "id": 609,
    "name": "Flawless Victory",
    "description": "Kill enemies without taking damage.",
    "type": "arena",
    "difficulty": "hard",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 5, "special": "without_taking_damage" }
    ],
    "rewards": { "xp": 600, "gold": 350 }
  },
  {
    "id": 610,
    "name": "Speed Demon",
    "description": "Kill enemies quickly.",
    "type": "arena",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 10, "special": "within_50_turns" }
    ],
    "rewards": { "xp": 400, "gold": 200 }
  },

  // ========== NPC/GUILD QUESTS (IDs 701-707) ==========
  {
    "id": 701,
    "name": "Merchant's Request",
    "description": "Help the traveling merchant with their inventory problems.",
    "giver": "Traveling Merchant",
    "type": "guild",
    "difficulty": "easy",
    "objectives": [
      { "kind": "collect", "target": "gold", "count": 200 }
    ],
    "rewards": { "xp": 150, "reputation": { "Merchants Guild": 10 } }
  },
  {
    "id": 702,
    "name": "Healer's Herbs",
    "description": "Gather healing supplies for the temple.",
    "giver": "Temple Healer",
    "type": "guild",
    "difficulty": "easy",
    "objectives": [
      { "kind": "collect", "target": "health_potion", "count": 3 }
    ],
    "rewards": { "xp": 100, "reputation": { "Temple": 10 } }
  },
  {
    "id": 703,
    "name": "Blacksmith's Test",
    "description": "Prove your worth to the blacksmith.",
    "giver": "Master Blacksmith",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 20 }
    ],
    "rewards": { "xp": 200, "reputation": { "Blacksmith Guild": 15 }, "items": [{ "id": "iron_helm", "count": 1, "rarity": "uncommon" }] }
  },
  {
    "id": 704,
    "name": "Alchemist's Ingredients",
    "description": "Gather rare ingredients from the dungeon.",
    "giver": "Eccentric Alchemist",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 10 }
    ],
    "rewards": { "xp": 250, "reputation": { "Alchemist Guild": 15 }, "items": [{ "id": "strength_potion", "count": 1, "rarity": "rare" }] }
  },
  {
    "id": 705,
    "name": "Sage's Wisdom",
    "description": "Prove your understanding of the dungeon.",
    "giver": "Wandering Sage",
    "type": "guild",
    "difficulty": "hard",
    "objectives": [
      { "kind": "explore", "target": 10, "count": 1 },
      { "kind": "explore", "target": "any", "count": 30, "special": "explore_rooms" }
    ],
    "rewards": { "xp": 400, "reputation": { "Sages Council": 20 } }
  },
  {
    "id": 706,
    "name": "Enchanter's Challenge",
    "description": "Gather magical items for enchantment study.",
    "giver": "Mystic Enchanter",
    "type": "guild",
    "difficulty": "hard",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 2, "special": "rarity_rare" }
    ],
    "rewards": { "xp": 500, "reputation": { "Enchanters Circle": 25 }, "items": [{ "id": "scroll_enchant", "count": 1, "rarity": "rare" }] }
  },
  {
    "id": 707,
    "name": "Guard's Duty",
    "description": "Help the dungeon guard clear threats.",
    "giver": "Dungeon Guard",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 30 }
    ],
    "rewards": { "xp": 300, "reputation": { "Guards": 20 } }
  },

  // ========== LEGENDARY QUESTS (IDs 801-805) ==========
  {
    "id": 801,
    "name": "The Demon King's Bane",
    "description": "Collect artifacts of power to stand against the Demon King.",
    "type": "legendary",
    "difficulty": "legendary",
    "lore": "Ancient prophecies speak of five artifacts that, when combined, grant power to defeat the Demon King.",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 3, "special": "rarity_legendary" },
      { "kind": "boss", "target": "any", "count": 5 },
      { "kind": "explore", "target": 30, "count": 1 }
    ],
    "rewards": { "xp": 10000, "gold": 5000, "title": "Demon's Bane", "stats": { "hp": 20, "atk": 10, "def": 10, "mana": 20, "spd": 5 } },
    "chain": { "id": 1, "position": 1 }
  },
  {
    "id": 802,
    "name": "Keeper of Secrets",
    "description": "Uncover all the hidden secrets of ShadowCrypt.",
    "type": "legendary",
    "difficulty": "legendary",
    "objectives": [
      { "kind": "explore", "target": "any", "count": 10, "special": "find_all_secrets" },
      { "kind": "explore", "target": "any", "count": 100, "special": "explore_rooms" }
    ],
    "rewards": { "xp": 8000, "gold": 4000, "title": "Keeper of Secrets" }
  },
  {
    "id": 803,
    "name": "The Thousand Slain",
    "description": "Become a legend through sheer combat prowess.",
    "type": "legendary",
    "difficulty": "legendary",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 1000 }
    ],
    "rewards": { "xp": 15000, "gold": 7500, "title": "The Thousand Slayer", "stats": { "hp": 0, "atk": 25, "def": 0, "mana": 0, "spd": 10 } }
  },
  {
    "id": 804,
    "name": "Master of All",
    "description": "Reach the pinnacle of power and mastery.",
    "type": "legendary",
    "difficulty": "legendary",
    "objectives": [
      { "kind": "level", "target": 30, "count": 30, "special": "reach_level" },
      { "kind": "skill", "target": "any", "count": 500, "special": "use_skills" }
    ],
    "rewards": { "xp": 20000, "gold": 10000, "title": "Master of All" }
  },
  {
    "id": 805,
    "name": "Wealthy Beyond Measure",
    "description": "Amass legendary wealth.",
    "type": "legendary",
    "difficulty": "legendary",
    "objectives": [
      { "kind": "collect", "target": "gold", "count": 50000 }
    ],
    "rewards": { "xp": 12000, "title": "Dragon's Hoard", "items": [{ "id": "crown_of_kings", "count": 1, "rarity": "legendary" }] }
  },

  // ========== TIME-LIMITED DAILY QUESTS (IDs 901-904) ==========
  {
    "id": 901,
    "name": "Timed Extermination",
    "description": "Clear enemies quickly before time runs out.",
    "type": "daily",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 15 }
    ],
    "rewards": { "xp": 200, "gold": 100 },
    "deadline": 100,
    "repeatable": true
  },
  {
    "id": 902,
    "name": "Speed Run",
    "description": "Reach the next floor quickly.",
    "type": "daily",
    "difficulty": "hard",
    "objectives": [
      { "kind": "explore", "target": 3, "count": 1 }
    ],
    "rewards": { "xp": 300, "gold": 150 },
    "deadline": 150,
    "repeatable": true
  },
  {
    "id": 903,
    "name": "Quick Loot",
    "description": "Gather items before time expires.",
    "type": "daily",
    "difficulty": "normal",
    "objectives": [
      { "kind": "collect", "target": "any", "count": 5 }
    ],
    "rewards": { "xp": 150, "gold": 75 },
    "deadline": 75,
    "repeatable": true
  },
  {
    "id": 904,
    "name": "Rapid Gold",
    "description": "Collect gold quickly.",
    "type": "daily",
    "difficulty": "easy",
    "objectives": [
      { "kind": "collect", "target": "gold", "count": 100 }
    ],
    "rewards": { "xp": 100, "gold": 50 },
    "deadline": 100,
    "repeatable": true
  },

  // ========== CLASS-SPECIFIC QUESTS (IDs 1001-1006) ==========
  {
    "id": 1001,
    "name": "Warrior's Honor",
    "description": "Prove yourself as a true warrior.",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "any", "count": 30 },
      { "kind": "kill", "target": "any", "count": 3, "special": "without_taking_damage" }
    ],
    "rewards": { "xp": 400, "gold": 200, "items": [{ "id": "greatsword", "count": 1, "rarity": "rare" }] },
    "requiresClass": "warrior"
  },
  {
    "id": 1002,
    "name": "Mage's Study",
    "description": "Master the arcane arts through practice.",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "skill", "target": "any", "count": 30, "special": "use_skills" },
      { "kind": "collect", "target": "mana_potion", "count": 5 }
    ],
    "rewards": { "xp": 400, "gold": 200, "items": [{ "id": "void_staff", "count": 1, "rarity": "rare" }] },
    "requiresClass": "mage"
  },
  {
    "id": 1003,
    "name": "Rogue's Cunning",
    "description": "Use stealth and skill to overcome your foes.",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "goblin", "count": 10 },
      { "kind": "collect", "target": "gold", "count": 300 }
    ],
    "rewards": { "xp": 400, "gold": 300, "items": [{ "id": "dagger", "count": 1, "rarity": "rare" }] },
    "requiresClass": "rogue"
  },
  {
    "id": 1004,
    "name": "Paladin's Virtue",
    "description": "Smite evil and protect the innocent.",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "skeleton", "count": 15 },
      { "kind": "kill", "target": "zombie", "count": 15 }
    ],
    "rewards": { "xp": 400, "gold": 200, "items": [{ "id": "holy_armor", "count": 1, "rarity": "rare" }] },
    "requiresClass": "paladin"
  },
  {
    "id": 1005,
    "name": "Ranger's Hunt",
    "description": "Track and hunt the beasts of the dungeon.",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "wolf", "count": 10 },
      { "kind": "kill", "target": "spider", "count": 10 }
    ],
    "rewards": { "xp": 400, "gold": 200, "items": [{ "id": "bow", "count": 1, "rarity": "rare" }] },
    "requiresClass": "ranger"
  },
  {
    "id": 1006,
    "name": "Necromancer's Dominion",
    "description": "Master the powers of death and undeath.",
    "type": "guild",
    "difficulty": "normal",
    "objectives": [
      { "kind": "kill", "target": "ghost", "count": 5 },
      { "kind": "skill", "target": "any", "count": 25, "special": "use_skills" }
    ],
    "rewards": { "xp": 400, "gold": 200, "items": [{ "id": "scythe", "count": 1, "rarity": "rare" }] },
    "requiresClass": "necromancer"
  }
];

SC.DATA.quests_notes = [
  "Source: src/quest.rs create_all_quests() - all 89 predefined quests ported; ids, names, descriptions, lore, counts, XP, gold, item rewards, reputation values, NPC givers, prerequisites, deadlines and repeatable flags are verbatim from source.",
  "Procedural quest generators (ProceduralQuestGenerator::generate_kill_quest/collection/exploration/survival/daily/weekly, source ids 50000-99999) are code, not data, so they are NOT included here; only the static quest catalog is ported.",
  "Enemy target ids use the exact keys of SC.DATA.enemies in web/js/data/data_enemies.js, which are camelCase (e.g. 'bossGoblinKing', 'goblin'), not snake_case - the contract said to use that file's ids, so its camelCase convention wins for enemy targets. Item ids are snake_case of the ItemKind name (e.g. HealthPotion -> 'health_potion') since there is no item data file to match against.",
  "Quest 3 objective 2: source targets EnemyKind::GoblinArcher, which does not exist in data_enemies.js (or anywhere else in the repo's EnemyKind enums) - target set to 'any' with special 'source_target_goblin_archer'.",
  "Quest 107 objective 2: source targets EnemyKind::OrcWarrior, not present in data_enemies.js - target set to 'any' with special 'source_target_orc_warrior'. (Closest existing enemy is 'orcBerserker'.)",
  "Quest 112: source targets EnemyKind::DarkElf, not present in data_enemies.js - target set to 'any' with special 'source_target_dark_elf'. (Closest existing enemy is 'darkElfAssassin'.)",
  "Quest 113: source targets EnemyKind::Werewolf, not present in data_enemies.js - target set to 'any' with special 'source_target_werewolf'.",
  "QuestType mapping to the 'type' field: Main -> 'story', Bounty -> 'kill', Exploration -> 'explore', Collection -> 'collect', Side -> 'side', Arena -> 'arena', Guild -> 'guild', Legendary -> 'legendary', Daily -> 'daily'.",
  "Objective kind mapping: KillEnemyType/KillAnyEnemy/KillWithoutDamage/KillInTime -> 'kill'; KillBoss -> 'boss' (target 'any' for KillBoss(None)); CollectItem/CollectGold/CollectRarity/CollectAnyItems -> 'collect' (gold uses target 'gold', matching ItemKind::Gold); ReachFloor/ExploreRooms/FindSecretRoom/FindAllSecrets/ReachFloorWithHP -> 'explore' (floor number as target for ReachFloor, else target 'any').",
  "Invented objective kinds beyond the contract's kill/collect/explore/boss/craft/harvest list, because the source objectives have no counterpart there: 'skill' (ObjectiveType::UseSkill), 'level' (ObjectiveType::LevelUp - target and count are the level to reach), 'survive' (ObjectiveType::SurviveTurns / SurviveWithoutHealing - count is turns). No predefined quest uses a craft or harvest objective.",
  "Invented optional objective field 'special' to preserve source mechanics the {kind,target,count} shape cannot express: 'without_taking_damage' (KillWithoutDamage), 'within_50_turns' (KillInTime(10,50)), 'rarity_rare'/'rarity_epic'/'rarity_legendary' (CollectRarity), 'explore_rooms' (ExploreRooms), 'find_secret_room' (FindSecretRoom), 'find_all_secrets' (FindAllSecrets), 'min_hp_percent_75'/'min_hp_percent_90' (ReachFloorWithHP), 'use_skills' (UseSkill), 'reach_level' (LevelUp), 'survive_turns'/'survive_turns_without_healing' (SurviveTurns/SurviveWithoutHealing), and 'source_target_*' markers for the four missing enemy kinds above.",
  "Invented optional quest fields to avoid losing verbatim source data: 'difficulty' (QuestDifficulty, camelCase), 'lore' (with_lore), 'minFloor' (requires_floor - floor requirement, distinct from the schema's minLevel; no predefined quest uses requires_level so minLevel never appears), 'deadline' (with_deadline, in turns), 'repeatable', 'requiresClass' (requires_class, lowercased class name), 'next' (then_quest - the quest id unlocked on completion; inverse of 'requires'), and 'chain' {id, position} (in_chain).",
  "Invented optional reward fields: 'title' (QuestReward::title) and 'stats' {hp,atk,def,mana,spd} (QuestReward::stat_bonuses). Reward items carry count:1 (source rewards are always a single item) plus an extra 'rarity' field verbatim from source, since the source stores (ItemKind, Rarity) pairs.",
  "Quests 1-13 belong to chain 1 ('The Descent') per create_quest_chains(); the source only tags them via chain membership, and only quest 801 calls .in_chain(1, 1) directly (verbatim - yes, the source puts 801 into chain id 1 at position 1, which looks like a source bug but is preserved). Chain positions 1-13 for quests 1-13 were derived from the chain's quest order. Quest chains themselves (final rewards for chains 1-4) are aggregate data not part of the per-quest schema and are recorded here: chain 1 'The Descent' (25000 XP, 15000 gold, title 'Conqueror of ShadowCrypt'), chain 2 'The Hunter' (5000 XP, 3000 gold, title 'Master Hunter'), chain 3 'Explorer's Guild' (4000 XP, 2000 gold, title 'Master Cartographer'), chain 4 'Path of Power' (8000 XP, 4000 gold, title 'Avatar of War').",
  "Quest ids are numbers, verbatim from the Rust u32 ids. 'requires' holds the single prerequisite quest id (no source quest has more than one prerequisite_quests entry).",
  "Objective 'count' is the required_progress value passed to QuestObjective::new; for ReachFloor-style single-completion objectives the source passes 1 and the floor lives in 'target'."
];

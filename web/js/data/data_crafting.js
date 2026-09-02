'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// ============================================================================
// ShadowCrypt crafting catalog - ported from src/crafting.rs and src/recipes.rs.
// 47 materials (25 ItemKind crafting materials + 4 special components +
// 18 HerbType alchemy herbs) and 226 recipes across smithing, alchemy,
// cooking, enchanting, tailoring, jewelcrafting, runecraft, and rare loot
// recipes. Generated data file; see SC.DATA.crafting_notes for porting decisions.
// ============================================================================

SC.DATA.materials = {
  "iron_ore": {
    "id": "iron_ore",
    "name": "Iron Ore",
    "tier": 1,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Raw iron ore for smithing",
    "value": 5,
    "sources": [
      "dungeon loot",
      "salvage: iron weapons, chain mail, iron shields (default salvage result)"
    ]
  },
  "steel_ingot": {
    "id": "steel_ingot",
    "name": "Steel Ingot",
    "tier": 2,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Refined steel",
    "value": 15,
    "sources": [
      "crafted: Steel Ingot (Forge)",
      "salvage: greatswords, battle axes, katanas, plate mail, tower shields"
    ]
  },
  "leather_strip": {
    "id": "leather_strip",
    "name": "Leather Strip",
    "tier": 1,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Basic leather",
    "value": 3,
    "sources": [
      "crafted: Leather Strip (Loom)",
      "salvage: leather armor, wooden shields, katanas"
    ]
  },
  "dragon_blood": {
    "id": "dragon_blood",
    "name": "Dragon Blood",
    "tier": 4,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Purified dragon blood",
    "value": 200,
    "sources": [
      "crafted: Dragon Blood Extract (Alchemy Table)"
    ]
  },
  "red_herb": {
    "id": "red_herb",
    "name": "Red Herb",
    "tier": 1,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "A common healing herb",
    "value": 4,
    "sources": [
      "dungeon loot"
    ]
  },
  "empty_vial": {
    "id": "empty_vial",
    "name": "Empty Vial",
    "tier": 1,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "A glass container",
    "value": 2,
    "sources": [
      "crafted: Empty Vial (Forge)",
      "salvage: health and mana potions"
    ]
  },
  "moon_flower": {
    "id": "moon_flower",
    "name": "Moon Flower",
    "tier": 1,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "A mana-rich night blossom",
    "value": 5,
    "sources": [
      "dungeon loot"
    ]
  },
  "phoenix_feather": {
    "id": "phoenix_feather",
    "name": "Phoenix Feather",
    "tier": 5,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "A feather that burns without being consumed",
    "value": 400,
    "sources": [
      "dungeon loot"
    ]
  },
  "unicorn_horn": {
    "id": "unicorn_horn",
    "name": "Unicorn Horn",
    "tier": 5,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "A purifying horn of legend",
    "value": 500,
    "sources": [
      "dungeon loot"
    ]
  },
  "elixir_of_life": {
    "id": "elixir_of_life",
    "name": "Elixir of Life",
    "tier": 5,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Revive from death once",
    "value": 1000,
    "sources": [
      "crafted: Elixir of Life (Alchemy Table)"
    ]
  },
  "manacrystal_i": {
    "id": "manacrystal_i",
    "name": "Mana Crystal I",
    "tier": 2,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Basic mana storage",
    "value": 30,
    "sources": [
      "crafted: Mana Crystal I (Alchemy Table)"
    ]
  },
  "manacrystal_ii": {
    "id": "manacrystal_ii",
    "name": "Mana Crystal II",
    "tier": 3,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Improved mana storage",
    "value": 90,
    "sources": [
      "crafted: Mana Crystal II (Alchemy Table)"
    ]
  },
  "manacrystal_iii": {
    "id": "manacrystal_iii",
    "name": "Mana Crystal III",
    "tier": 4,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Superior mana storage",
    "value": 250,
    "sources": [
      "crafted: Mana Crystal III (Alchemy Table)"
    ]
  },
  "blank_scroll": {
    "id": "blank_scroll",
    "name": "Blank Scroll",
    "tier": 1,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "For writing spells",
    "value": 6,
    "sources": [
      "crafted: Blank Scroll (Workbench)"
    ]
  },
  "raw_meat": {
    "id": "raw_meat",
    "name": "Raw Meat",
    "tier": 1,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Uncooked meat",
    "value": 3,
    "sources": [
      "dungeon loot"
    ]
  },
  "gold_bar": {
    "id": "gold_bar",
    "name": "Gold Bar",
    "tier": 2,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Pure gold",
    "value": 60,
    "sources": [
      "crafted: Gold Bar (Forge)"
    ]
  },
  "silver_bar": {
    "id": "silver_bar",
    "name": "Silver Bar",
    "tier": 2,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Pure silver",
    "value": 40,
    "sources": [
      "crafted: Silver Bar (Forge)"
    ]
  },
  "mithril_ore": {
    "id": "mithril_ore",
    "name": "Mithril Ore",
    "tier": 3,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Rare elven silver ore",
    "value": 120,
    "sources": [
      "dungeon loot",
      "crafted: Mithril Ingot (Forge)"
    ]
  },
  "rune_stone": {
    "id": "rune_stone",
    "name": "Rune Stone",
    "tier": 2,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Basic rune material",
    "value": 25,
    "sources": [
      "crafted: Rune Stone (Enchanting Altar)"
    ]
  },
  "enchanted_gem": {
    "id": "enchanted_gem",
    "name": "Enchanted Gem",
    "tier": 3,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "A magical gemstone",
    "value": 80,
    "sources": [
      "crafted: Enchanted Gem (Enchanting Altar)",
      "rare salvage: most items"
    ]
  },
  "frost_essence": {
    "id": "frost_essence",
    "name": "Frost Essence",
    "tier": 3,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Concentrated cold",
    "value": 45,
    "sources": [
      "crafted: Frost Essence (Alchemy Table)",
      "rare salvage: frost weapons and gauntlets"
    ]
  },
  "fire_essence": {
    "id": "fire_essence",
    "name": "Fire Essence",
    "tier": 3,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Concentrated flame",
    "value": 45,
    "sources": [
      "crafted: Fire Essence (Alchemy Table)",
      "rare salvage: flame weapons and gauntlets"
    ]
  },
  "void_essence": {
    "id": "void_essence",
    "name": "Void Essence",
    "tier": 4,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Concentrated void",
    "value": 110,
    "sources": [
      "crafted: Void Essence (Alchemy Table)",
      "rare salvage: Void Staff"
    ]
  },
  "ancient_bone": {
    "id": "ancient_bone",
    "name": "Ancient Bone",
    "tier": 2,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "A bone from an ancient creature",
    "value": 35,
    "sources": [
      "dungeon loot"
    ]
  },
  "cursed_fabric": {
    "id": "cursed_fabric",
    "name": "Cursed Fabric",
    "tier": 2,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Dark cloth",
    "value": 40,
    "sources": [
      "crafted: Cursed Fabric (Loom)"
    ]
  },
  "dragon_scale": {
    "id": "dragon_scale",
    "name": "Dragon Scale",
    "tier": 3,
    "glyph": "s",
    "color": "#ecf0f1",
    "description": "A scale shed from a dragon",
    "value": 150,
    "sources": [
      "dungeon loot",
      "salvage: dragon armor, helm, and shield"
    ]
  },
  "demon_heart": {
    "id": "demon_heart",
    "name": "Demon Heart",
    "tier": 4,
    "glyph": "h",
    "color": "#ecf0f1",
    "description": "The still-beating heart of a demon",
    "value": 300,
    "sources": [
      "dungeon loot"
    ]
  },
  "soul_gem": {
    "id": "soul_gem",
    "name": "Soul Gem",
    "tier": 4,
    "glyph": "o",
    "color": "#ecf0f1",
    "description": "Contains captured souls",
    "value": 220,
    "sources": [
      "dungeon loot",
      "crafted: Soul Gem (Alchemy Table)"
    ]
  },
  "ancient_relic": {
    "id": "ancient_relic",
    "name": "Ancient Relic",
    "tier": 4,
    "glyph": "*",
    "color": "#ecf0f1",
    "description": "Mysterious power",
    "value": 350,
    "sources": [
      "dungeon loot",
      "crafted: Ancient Relic (Enchanting Altar)"
    ]
  },
  "bloodroot": {
    "id": "bloodroot",
    "name": "Bloodroot",
    "tier": 1,
    "glyph": "\"",
    "color": "#e74c3c",
    "description": "Health restoration",
    "value": 5,
    "rarity": "common",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "moonpetal": {
    "id": "moonpetal",
    "name": "Moonpetal",
    "tier": 1,
    "glyph": "\"",
    "color": "#3498db",
    "description": "Mana restoration",
    "value": 5,
    "rarity": "common",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "sunleaf": {
    "id": "sunleaf",
    "name": "Sunleaf",
    "tier": 1,
    "glyph": "\"",
    "color": "#b7950b",
    "description": "Energy and stamina",
    "value": 5,
    "rarity": "common",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "nightshade": {
    "id": "nightshade",
    "name": "Nightshade",
    "tier": 1,
    "glyph": "\"",
    "color": "#1e8449",
    "description": "Poison base",
    "value": 5,
    "rarity": "common",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "feverfew": {
    "id": "feverfew",
    "name": "Feverfew",
    "tier": 1,
    "glyph": "\"",
    "color": "#2ecc71",
    "description": "Cures disease",
    "value": 5,
    "rarity": "common",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "dragontongue": {
    "id": "dragontongue",
    "name": "Dragontongue",
    "tier": 2,
    "glyph": "\"",
    "color": "#922b21",
    "description": "Fire properties",
    "value": 15,
    "rarity": "uncommon",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "frostbloom": {
    "id": "frostbloom",
    "name": "Frostbloom",
    "tier": 2,
    "glyph": "\"",
    "color": "#1abc9c",
    "description": "Ice properties",
    "value": 15,
    "rarity": "uncommon",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "stormweed": {
    "id": "stormweed",
    "name": "Stormweed",
    "tier": 2,
    "glyph": "\"",
    "color": "#f1c40f",
    "description": "Lightning properties",
    "value": 15,
    "rarity": "uncommon",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "shadowmoss": {
    "id": "shadowmoss",
    "name": "Shadowmoss",
    "tier": 2,
    "glyph": "\"",
    "color": "#5d6d7e",
    "description": "Stealth and invisibility",
    "value": 15,
    "rarity": "uncommon",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "glowcap": {
    "id": "glowcap",
    "name": "Glowcap",
    "tier": 2,
    "glyph": "\"",
    "color": "#ecf0f1",
    "description": "Light and vision",
    "value": 15,
    "rarity": "uncommon",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "voidroot": {
    "id": "voidroot",
    "name": "Voidroot",
    "tier": 3,
    "glyph": "\"",
    "color": "#76448a",
    "description": "Void and teleportation",
    "value": 60,
    "rarity": "rare",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "phoenixwort": {
    "id": "phoenixwort",
    "name": "Phoenixwort",
    "tier": 3,
    "glyph": "\"",
    "color": "#d35ded",
    "description": "Resurrection",
    "value": 60,
    "rarity": "rare",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "unicornhair": {
    "id": "unicornhair",
    "name": "Unicornhair",
    "tier": 3,
    "glyph": "\"",
    "color": "#ecf0f1",
    "description": "Purification",
    "value": 60,
    "rarity": "rare",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "mandrake": {
    "id": "mandrake",
    "name": "Mandrake",
    "tier": 3,
    "glyph": "\"",
    "color": "#b7950b",
    "description": "Transformation",
    "value": 60,
    "rarity": "rare",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "ghostorchid": {
    "id": "ghostorchid",
    "name": "Ghost Orchid",
    "tier": 3,
    "glyph": "\"",
    "color": "#95a5a6",
    "description": "Spirit properties",
    "value": 60,
    "rarity": "rare",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "worldtree_leaf": {
    "id": "worldtree_leaf",
    "name": "Worldtree Leaf",
    "tier": 5,
    "glyph": "\"",
    "color": "#2ecc71",
    "description": "Ultimate healing",
    "value": 300,
    "rarity": "legendary",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "dragonheart": {
    "id": "dragonheart",
    "name": "Dragonheart Flower",
    "tier": 5,
    "glyph": "\"",
    "color": "#922b21",
    "description": "Ultimate power",
    "value": 300,
    "rarity": "legendary",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  },
  "starfall_dust": {
    "id": "starfall_dust",
    "name": "Starfall Dust",
    "tier": 5,
    "glyph": "\"",
    "color": "#d35ded",
    "description": "Cosmic power",
    "value": 300,
    "rarity": "legendary",
    "sources": [
      "herbalism: gathered in the dungeon"
    ]
  }
};

SC.DATA.recipes = [
  {
    "id": "iron_dagger",
    "name": "Iron Dagger",
    "description": "A simple iron dagger",
    "result": "dagger",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 1,
    "xp": 10,
    "category": "weapon"
  },
  {
    "id": "iron_short_sword",
    "name": "Iron Short Sword",
    "description": "A basic iron blade",
    "result": "short_sword",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 2,
    "xp": 15,
    "category": "weapon"
  },
  {
    "id": "iron_axe",
    "name": "Iron Axe",
    "description": "A sturdy chopping axe",
    "result": "axe",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 3
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 3,
    "xp": 18,
    "category": "weapon"
  },
  {
    "id": "iron_mace",
    "name": "Iron Mace",
    "description": "A heavy iron mace",
    "result": "mace",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 4
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 4,
    "xp": 20,
    "category": "weapon"
  },
  {
    "id": "iron_spear",
    "name": "Iron Spear",
    "description": "A long iron-tipped spear",
    "result": "spear",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 2
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 5,
    "xp": 22,
    "category": "weapon"
  },
  {
    "id": "hunting_bow",
    "name": "Hunting Bow",
    "description": "A simple wooden bow",
    "result": "bow",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 3
      }
    ],
    "station": "workbench",
    "skill": "blacksmithing",
    "skillLevel": 3,
    "xp": 15,
    "category": "weapon"
  },
  {
    "id": "light_crossbow",
    "name": "Light Crossbow",
    "description": "A compact crossbow",
    "result": "crossbow",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 2
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "blacksmithing",
    "skillLevel": 6,
    "xp": 25,
    "category": "weapon"
  },
  {
    "id": "wooden_staff",
    "name": "Wooden Staff",
    "description": "A basic magical focus",
    "result": "staff",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "blacksmithing",
    "skillLevel": 2,
    "xp": 12,
    "category": "weapon"
  },
  {
    "id": "iron_warhammer",
    "name": "Iron Warhammer",
    "description": "A crushing iron hammer",
    "result": "war_hammer",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 5
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 7,
    "xp": 28,
    "category": "weapon"
  },
  {
    "id": "iron_flail",
    "name": "Iron Flail",
    "description": "A chain weapon with iron ball",
    "result": "flail",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 4
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 8,
    "xp": 30,
    "category": "weapon"
  },
  {
    "id": "steel_long_sword",
    "name": "Steel Long Sword",
    "description": "A refined steel blade",
    "result": "long_sword",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 3
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 10,
    "xp": 35,
    "category": "weapon"
  },
  {
    "id": "steel_greatsword",
    "name": "Steel Greatsword",
    "description": "A massive two-handed blade",
    "result": "greatsword",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 5
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 12,
    "xp": 45,
    "category": "weapon"
  },
  {
    "id": "steel_battle_axe",
    "name": "Steel Battle Axe",
    "description": "A fearsome war axe",
    "result": "battle_axe",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 11,
    "xp": 40,
    "category": "weapon"
  },
  {
    "id": "steel_halberd",
    "name": "Steel Halberd",
    "description": "A polearm with axe blade",
    "result": "halberd",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "leather_strip",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 13,
    "xp": 48,
    "category": "weapon"
  },
  {
    "id": "steel_rapier",
    "name": "Steel Rapier",
    "description": "An elegant thrusting sword",
    "result": "rapier",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 2
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 14,
    "xp": 42,
    "category": "weapon"
  },
  {
    "id": "steel_morningstar",
    "name": "Steel Morningstar",
    "description": "A spiked mace",
    "result": "morningstar",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 15,
    "xp": 50,
    "category": "weapon"
  },
  {
    "id": "war_trident",
    "name": "War Trident",
    "description": "A three-pronged spear",
    "result": "trident",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 16,
    "xp": 55,
    "category": "weapon"
  },
  {
    "id": "reapers_scythe",
    "name": "Reaper's Scythe",
    "description": "A curved blade of death",
    "result": "scythe",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 5
      },
      {
        "id": "leather_strip",
        "count": 2
      },
      {
        "id": "ancient_bone",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 18,
    "xp": 65,
    "category": "weapon"
  },
  {
    "id": "katana",
    "name": "Katana",
    "description": "A curved eastern blade",
    "result": "katana",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 20,
    "xp": 70,
    "category": "weapon",
    "craftingTime": 3
  },
  {
    "id": "mithril_blade",
    "name": "Mithril Blade",
    "description": "A blade of elven silver",
    "result": "long_sword",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 3
      },
      {
        "id": "steel_ingot",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 22,
    "xp": 80,
    "category": "weapon"
  },
  {
    "id": "mithril_greatsword",
    "name": "Mithril Greatsword",
    "description": "A massive mithril blade",
    "result": "greatsword",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 5
      },
      {
        "id": "steel_ingot",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 25,
    "xp": 100,
    "category": "weapon"
  },
  {
    "id": "mithril_axe",
    "name": "Mithril Axe",
    "description": "An axe of shimmering metal",
    "result": "battle_axe",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 4
      },
      {
        "id": "steel_ingot",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 23,
    "xp": 85,
    "category": "weapon"
  },
  {
    "id": "mithril_spear",
    "name": "Mithril Spear",
    "description": "A lightweight spear",
    "result": "halberd",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 3
      },
      {
        "id": "steel_ingot",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 24,
    "xp": 90,
    "category": "weapon"
  },
  {
    "id": "flame_sword",
    "name": "Flame Sword",
    "description": "A blade wreathed in fire",
    "result": "flame_sword",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "fire_essence",
        "count": 3
      },
      {
        "id": "manacrystal_ii",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 28,
    "xp": 120,
    "category": "weapon",
    "enchantment": "fire"
  },
  {
    "id": "frost_blade",
    "name": "Frost Blade",
    "description": "A sword of eternal ice",
    "result": "frost_blade",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "frost_essence",
        "count": 3
      },
      {
        "id": "manacrystal_ii",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 28,
    "xp": 120,
    "category": "weapon",
    "enchantment": "frost"
  },
  {
    "id": "thunder_axe",
    "name": "Thunder Axe",
    "description": "An axe crackling with lightning",
    "result": "thunder_axe",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 5
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "gold_bar",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 30,
    "xp": 130,
    "category": "weapon",
    "enchantment": "lightning"
  },
  {
    "id": "void_staff",
    "name": "Void Staff",
    "description": "A staff of dark power",
    "result": "void_staff",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "staff",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 3
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 35,
    "xp": 150,
    "category": "weapon",
    "craftingTime": 5
  },
  {
    "id": "demon_slayer",
    "name": "Demon Slayer",
    "description": "Bane of all demons",
    "result": "demon_slayer",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 5
      },
      {
        "id": "demon_heart",
        "count": 1
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      },
      {
        "id": "phoenix_feather",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 40,
    "xp": 200,
    "category": "weapon",
    "enchantment": "holy",
    "craftingTime": 8
  },
  {
    "id": "leather_cap",
    "name": "Leather Cap",
    "description": "A simple leather helmet",
    "result": "leather_cap",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 1,
    "xp": 8,
    "category": "armor"
  },
  {
    "id": "leather_armor",
    "name": "Leather Armor",
    "description": "Basic leather protection",
    "result": "leather_armor",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 4
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 2,
    "xp": 12,
    "category": "armor"
  },
  {
    "id": "leather_gloves",
    "name": "Leather Gloves",
    "description": "Simple hand protection",
    "result": "leather_gloves",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 1,
    "xp": 8,
    "category": "armor"
  },
  {
    "id": "leather_boots",
    "name": "Leather Boots",
    "description": "Basic footwear",
    "result": "leather_boots",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 1,
    "xp": 8,
    "category": "armor"
  },
  {
    "id": "iron_helm",
    "name": "Iron Helm",
    "description": "A protective iron helmet",
    "result": "iron_helm",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 5,
    "xp": 18,
    "category": "armor"
  },
  {
    "id": "chain_mail",
    "name": "Chain Mail",
    "description": "Interlocking iron rings",
    "result": "chain_mail",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 6
      },
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 8,
    "xp": 28,
    "category": "armor"
  },
  {
    "id": "iron_gauntlets",
    "name": "Iron Gauntlets",
    "description": "Armored hand protection",
    "result": "iron_gauntlets",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 3
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 6,
    "xp": 20,
    "category": "armor"
  },
  {
    "id": "iron_boots",
    "name": "Iron Boots",
    "description": "Heavy iron boots",
    "result": "iron_boots",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 3
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 6,
    "xp": 20,
    "category": "armor"
  },
  {
    "id": "steel_helm",
    "name": "Steel Helm",
    "description": "A sturdy steel helmet",
    "result": "steel_helm",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 12,
    "xp": 40,
    "category": "armor"
  },
  {
    "id": "scale_mail",
    "name": "Scale Mail",
    "description": "Overlapping metal scales",
    "result": "scale_mail",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 5
      },
      {
        "id": "leather_strip",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 14,
    "xp": 50,
    "category": "armor"
  },
  {
    "id": "plate_mail",
    "name": "Plate Mail",
    "description": "Full plate armor",
    "result": "plate_mail",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 8
      },
      {
        "id": "leather_strip",
        "count": 4
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 18,
    "xp": 70,
    "category": "armor",
    "craftingTime": 4
  },
  {
    "id": "buckler",
    "name": "Buckler",
    "description": "A small round shield",
    "result": "buckler",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 2
      },
      {
        "id": "leather_strip",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 2,
    "xp": 10,
    "category": "armor"
  },
  {
    "id": "wooden_shield",
    "name": "Wooden Shield",
    "description": "A basic wooden shield",
    "result": "wooden_shield",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 3
      }
    ],
    "station": "workbench",
    "skill": "blacksmithing",
    "skillLevel": 1,
    "xp": 8,
    "category": "armor"
  },
  {
    "id": "iron_shield",
    "name": "Iron Shield",
    "description": "A sturdy iron shield",
    "result": "iron_shield",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 4
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 6,
    "xp": 22,
    "category": "armor"
  },
  {
    "id": "tower_shield",
    "name": "Tower Shield",
    "description": "A massive defensive shield",
    "result": "tower_shield",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 6
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 15,
    "xp": 55,
    "category": "armor"
  },
  {
    "id": "spiked_shield",
    "name": "Spiked Shield",
    "description": "An offensive shield",
    "result": "spiked_shield",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 5
      },
      {
        "id": "iron_ore",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 16,
    "xp": 58,
    "category": "armor"
  },
  {
    "id": "dragon_helm",
    "name": "Dragon Helm",
    "description": "Helmet of dragon scales",
    "result": "dragon_helm",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "dragon_scale",
        "count": 2
      },
      {
        "id": "steel_ingot",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 35,
    "xp": 140,
    "category": "armor"
  },
  {
    "id": "dragon_armor",
    "name": "Dragon Armor",
    "description": "Armor of dragon scales",
    "result": "dragon_armor",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "dragon_scale",
        "count": 5
      },
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "leather_strip",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 38,
    "xp": 180,
    "category": "armor",
    "craftingTime": 6
  },
  {
    "id": "dragon_gauntlets",
    "name": "Dragon Gauntlets",
    "description": "Gauntlets of dragon scales",
    "result": "dragon_gauntlets",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "dragon_scale",
        "count": 2
      },
      {
        "id": "steel_ingot",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 36,
    "xp": 150,
    "category": "armor"
  },
  {
    "id": "dragon_shield",
    "name": "Dragon Shield",
    "description": "Shield of dragon scales",
    "result": "dragon_shield",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "dragon_scale",
        "count": 3
      },
      {
        "id": "steel_ingot",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 37,
    "xp": 160,
    "category": "armor"
  },
  {
    "id": "crystal_armor",
    "name": "Crystal Armor",
    "description": "Armor of magical crystal",
    "result": "crystal_armor",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "manacrystal_iii",
        "count": 5
      },
      {
        "id": "mithril_ore",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 45,
    "xp": 220,
    "category": "armor",
    "craftingTime": 8
  },
  {
    "id": "titan_plate",
    "name": "Titan Plate",
    "description": "Armor of the titans",
    "result": "titan_plate",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 6
      },
      {
        "id": "steel_ingot",
        "count": 6
      },
      {
        "id": "dragon_scale",
        "count": 2
      },
      {
        "id": "ancient_relic",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 50,
    "xp": 280,
    "category": "armor",
    "craftingTime": 10
  },
  {
    "id": "steel_ingot",
    "name": "Steel Ingot",
    "description": "Refined steel",
    "result": "steel_ingot",
    "resultCount": 2,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 3
      }
    ],
    "station": "forge",
    "skill": "blacksmithing",
    "skillLevel": 5,
    "xp": 15,
    "category": "misc"
  },
  {
    "id": "gold_bar",
    "name": "Gold Bar",
    "description": "Pure gold",
    "result": "gold_bar",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 5
      }
    ],
    "station": "forge",
    "skill": "blacksmithing",
    "skillLevel": 10,
    "xp": 25,
    "category": "misc"
  },
  {
    "id": "silver_bar",
    "name": "Silver Bar",
    "description": "Pure silver",
    "result": "silver_bar",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 4
      }
    ],
    "station": "forge",
    "skill": "blacksmithing",
    "skillLevel": 8,
    "xp": 20,
    "category": "misc"
  },
  {
    "id": "mithril_ingot",
    "name": "Mithril Ingot",
    "description": "Refined mithril",
    "result": "mithril_ore",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 2
      },
      {
        "id": "steel_ingot",
        "count": 1
      }
    ],
    "station": "forge",
    "skill": "blacksmithing",
    "skillLevel": 20,
    "xp": 50,
    "category": "misc"
  },
  {
    "id": "magic_shield",
    "name": "Magic Shield",
    "description": "An enchanted shield",
    "result": "magic_shield",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "iron_shield",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "blacksmithing",
    "skillLevel": 25,
    "xp": 100,
    "category": "armor"
  },
  {
    "id": "mirror_shield",
    "name": "Mirror Shield",
    "description": "Reflects magic",
    "result": "mirror_shield",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 4
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      },
      {
        "id": "silver_bar",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 42,
    "xp": 180,
    "category": "armor"
  },
  {
    "id": "phoenix_shield",
    "name": "Phoenix Shield",
    "description": "Burns attackers",
    "result": "phoenix_shield",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "dragon_shield",
        "count": 1
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "fire_essence",
        "count": 5
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 48,
    "xp": 250,
    "category": "armor",
    "craftingTime": 6
  },
  {
    "id": "abyssal_shield",
    "name": "Abyssal Shield",
    "description": "Shield from the void",
    "result": "abyssal_shield",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "tower_shield",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 5
      },
      {
        "id": "demon_heart",
        "count": 1
      },
      {
        "id": "cursed_fabric",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 55,
    "xp": 320,
    "category": "armor",
    "craftingTime": 8
  },
  {
    "id": "boots_of_speed",
    "name": "Boots of Speed",
    "description": "Swift as the wind",
    "result": "boots_of_speed",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "leather_boots",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "silver_bar",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 30,
    "xp": 110,
    "category": "armor",
    "enchantment": "swiftness"
  },
  {
    "id": "boots_of_leaping",
    "name": "Boots of Leaping",
    "description": "Jump incredible heights",
    "result": "boots_of_leaping",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "leather_boots",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 1
      },
      {
        "id": "leather_strip",
        "count": 5
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 32,
    "xp": 115,
    "category": "armor"
  },
  {
    "id": "winged_boots",
    "name": "Winged Boots",
    "description": "Grants flight",
    "result": "winged_boots",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "leather_boots",
        "count": 1
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "manacrystal_iii",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 40,
    "xp": 160,
    "category": "armor"
  },
  {
    "id": "shadow_boots",
    "name": "Shadow Boots",
    "description": "Walk in shadows",
    "result": "shadow_boots",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "leather_boots",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 2
      },
      {
        "id": "cursed_fabric",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 38,
    "xp": 145,
    "category": "armor",
    "enchantment": "stealth"
  },
  {
    "id": "lava_walkers",
    "name": "Lava Walkers",
    "description": "Walk on fire",
    "result": "lava_walkers",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "iron_boots",
        "count": 1
      },
      {
        "id": "fire_essence",
        "count": 4
      },
      {
        "id": "dragon_scale",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 42,
    "xp": 170,
    "category": "armor",
    "enchantment": "resistance"
  },
  {
    "id": "boots_of_the_wind",
    "name": "Boots of the Wind",
    "description": "Control the wind",
    "result": "boots_of_the_wind",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "winged_boots",
        "count": 1
      },
      {
        "id": "manacrystal_iii",
        "count": 3
      },
      {
        "id": "phoenix_feather",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 50,
    "xp": 240,
    "category": "armor",
    "craftingTime": 5
  },
  {
    "id": "gloves_of_power",
    "name": "Gloves of Power",
    "description": "Grants incredible strength",
    "result": "gloves_of_power",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "iron_gauntlets",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "steel_ingot",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 30,
    "xp": 110,
    "category": "armor"
  },
  {
    "id": "thieves_gloves",
    "name": "Thieves Gloves",
    "description": "For nimble fingers",
    "result": "thieves_gloves",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "leather_gloves",
        "count": 1
      },
      {
        "id": "silver_bar",
        "count": 2
      },
      {
        "id": "void_essence",
        "count": 1
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 28,
    "xp": 100,
    "category": "armor"
  },
  {
    "id": "frost_gauntlets",
    "name": "Frost Gauntlets",
    "description": "Freeze on touch",
    "result": "frost_gauntlets",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "iron_gauntlets",
        "count": 1
      },
      {
        "id": "frost_essence",
        "count": 4
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 35,
    "xp": 140,
    "category": "armor",
    "enchantment": "frost"
  },
  {
    "id": "flame_gauntlets",
    "name": "Flame Gauntlets",
    "description": "Burn on touch",
    "result": "flame_gauntlets",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "iron_gauntlets",
        "count": 1
      },
      {
        "id": "fire_essence",
        "count": 4
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 35,
    "xp": 140,
    "category": "armor",
    "enchantment": "fire"
  },
  {
    "id": "gauntlets_of_might",
    "name": "Gauntlets of Might",
    "description": "Ultimate power",
    "result": "gauntlets_of_might",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "dragon_gauntlets",
        "count": 1
      },
      {
        "id": "manacrystal_iii",
        "count": 3
      },
      {
        "id": "dragon_blood",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 48,
    "xp": 230,
    "category": "armor",
    "craftingTime": 5
  },
  {
    "id": "wizard_hat",
    "name": "Wizard Hat",
    "description": "Amplifies magic",
    "result": "wizard_hat",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "cursed_fabric",
        "count": 3
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 25,
    "xp": 90,
    "category": "armor",
    "enchantment": "wisdom"
  },
  {
    "id": "crown_of_kings",
    "name": "Crown of Kings",
    "description": "Symbol of royalty",
    "result": "crown_of_kings",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "enchanted_gem",
        "count": 3
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 40,
    "xp": 180,
    "category": "armor"
  },
  {
    "id": "demon_skull",
    "name": "Demon Skull",
    "description": "Helm of a demon",
    "result": "demon_skull",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "ancient_bone",
        "count": 3
      },
      {
        "id": "demon_heart",
        "count": 1
      },
      {
        "id": "cursed_fabric",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 42,
    "xp": 190,
    "category": "armor"
  },
  {
    "id": "crystal_crown",
    "name": "Crystal Crown",
    "description": "Crown of pure crystal",
    "result": "crystal_crown",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "manacrystal_iii",
        "count": 4
      },
      {
        "id": "enchanted_gem",
        "count": 4
      },
      {
        "id": "silver_bar",
        "count": 3
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 50,
    "xp": 260,
    "category": "armor",
    "craftingTime": 6
  },
  {
    "id": "hood_of_shadows",
    "name": "Hood of Shadows",
    "description": "Become one with darkness",
    "result": "hood_of_shadows",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "cursed_fabric",
        "count": 4
      },
      {
        "id": "void_essence",
        "count": 3
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 38,
    "xp": 155,
    "category": "armor",
    "enchantment": "stealth"
  },
  {
    "id": "helm_of_valor",
    "name": "Helm of Valor",
    "description": "Inspires courage",
    "result": "helm_of_valor",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 4
      },
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "gold_bar",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 52,
    "xp": 270,
    "category": "armor",
    "enchantment": "bravery",
    "craftingTime": 5
  },
  {
    "id": "mage_robes",
    "name": "Mage Robes",
    "description": "Robes of power",
    "result": "mage_robes",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "cursed_fabric",
        "count": 5
      },
      {
        "id": "manacrystal_ii",
        "count": 3
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 28,
    "xp": 95,
    "category": "armor",
    "enchantment": "wisdom"
  },
  {
    "id": "assassin_garb",
    "name": "Assassin Garb",
    "description": "Silent killer attire",
    "result": "assassin_garb",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 6
      },
      {
        "id": "cursed_fabric",
        "count": 2
      },
      {
        "id": "void_essence",
        "count": 1
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 30,
    "xp": 105,
    "category": "armor",
    "enchantment": "stealth"
  },
  {
    "id": "holy_armor",
    "name": "Holy Armor",
    "description": "Blessed by the gods",
    "result": "holy_armor",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 4
      },
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 45,
    "xp": 200,
    "category": "armor",
    "enchantment": "blessed",
    "craftingTime": 6
  },
  {
    "id": "demon_armor",
    "name": "Demon Armor",
    "description": "Forged in hellfire",
    "result": "demon_armor",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "steel_ingot",
        "count": 6
      },
      {
        "id": "demon_heart",
        "count": 2
      },
      {
        "id": "fire_essence",
        "count": 4
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 46,
    "xp": 210,
    "category": "armor",
    "craftingTime": 6
  },
  {
    "id": "shadow_cloak",
    "name": "Shadow Cloak",
    "description": "Wrap yourself in darkness",
    "result": "shadow_cloak",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "cursed_fabric",
        "count": 6
      },
      {
        "id": "void_essence",
        "count": 4
      }
    ],
    "station": "loom",
    "skill": "blacksmithing",
    "skillLevel": 44,
    "xp": 195,
    "category": "armor",
    "enchantment": "evasion"
  },
  {
    "id": "minor_health_potion",
    "name": "Minor Health Potion",
    "description": "Restores 25 HP",
    "result": "health_potion",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 1,
    "xp": 10,
    "category": "potion"
  },
  {
    "id": "minor_mana_potion",
    "name": "Minor Mana Potion",
    "description": "Restores 20 mana",
    "result": "mana_potion",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 2,
    "xp": 12,
    "category": "potion"
  },
  {
    "id": "antidote",
    "name": "Antidote",
    "description": "Cures poison",
    "result": "cure_all_potion",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 1
      },
      {
        "id": "moon_flower",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 3,
    "xp": 15,
    "category": "potion"
  },
  {
    "id": "empty_vial",
    "name": "Empty Vial",
    "description": "A glass container",
    "result": "empty_vial",
    "resultCount": 3,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 1
      }
    ],
    "station": "forge",
    "skill": "alchemy",
    "skillLevel": 1,
    "xp": 5,
    "category": "misc"
  },
  {
    "id": "health_potion",
    "name": "Health Potion",
    "description": "Restores 50 HP",
    "result": "health_potion",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 3
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 5,
    "xp": 18,
    "category": "potion"
  },
  {
    "id": "mana_potion",
    "name": "Mana Potion",
    "description": "Restores 40 mana",
    "result": "mana_potion",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 6,
    "xp": 20,
    "category": "potion"
  },
  {
    "id": "strength_potion",
    "name": "Strength Potion",
    "description": "Increases attack temporarily",
    "result": "strength_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 2
      },
      {
        "id": "moon_flower",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 8,
    "xp": 28,
    "category": "potion"
  },
  {
    "id": "defense_potion",
    "name": "Defense Potion",
    "description": "Increases defense temporarily",
    "result": "defense_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 1
      },
      {
        "id": "moon_flower",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 8,
    "xp": 28,
    "category": "potion"
  },
  {
    "id": "speed_potion",
    "name": "Speed Potion",
    "description": "Increases speed temporarily",
    "result": "speed_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 10,
    "xp": 32,
    "category": "potion"
  },
  {
    "id": "regeneration_potion",
    "name": "Regeneration Potion",
    "description": "Slowly restores HP",
    "result": "regeneration_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 4
      },
      {
        "id": "moon_flower",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 12,
    "xp": 38,
    "category": "potion"
  },
  {
    "id": "fire_resistance_potion",
    "name": "Fire Resistance Potion",
    "description": "Resist fire damage",
    "result": "fire_resist_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "fire_essence",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 10,
    "xp": 30,
    "category": "potion"
  },
  {
    "id": "ice_resistance_potion",
    "name": "Ice Resistance Potion",
    "description": "Resist ice damage",
    "result": "ice_resist_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "frost_essence",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 10,
    "xp": 30,
    "category": "potion"
  },
  {
    "id": "poison_resistance_potion",
    "name": "Poison Resistance Potion",
    "description": "Resist poison",
    "result": "poison_resist_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 3
      },
      {
        "id": "moon_flower",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 12,
    "xp": 35,
    "category": "potion"
  },
  {
    "id": "greater_health_potion",
    "name": "Greater Health Potion",
    "description": "Restores 100 HP",
    "result": "health_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "moon_flower",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 15,
    "xp": 45,
    "category": "potion"
  },
  {
    "id": "greater_mana_potion",
    "name": "Greater Mana Potion",
    "description": "Restores 80 mana",
    "result": "mana_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "red_herb",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 16,
    "xp": 48,
    "category": "potion"
  },
  {
    "id": "invisibility_potion",
    "name": "Invisibility Potion",
    "description": "Become invisible",
    "result": "invisibility_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "void_essence",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 18,
    "xp": 55,
    "category": "potion"
  },
  {
    "id": "berserk_potion",
    "name": "Berserk Potion",
    "description": "Enter a rage state",
    "result": "berserk_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 4
      },
      {
        "id": "fire_essence",
        "count": 2
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 20,
    "xp": 60,
    "category": "potion"
  },
  {
    "id": "giant_potion",
    "name": "Giant Potion",
    "description": "Grow in size and power",
    "result": "giant_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 22,
    "xp": 70,
    "category": "potion"
  },
  {
    "id": "levitation_potion",
    "name": "Levitation Potion",
    "description": "Float in the air",
    "result": "levitation_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 4
      },
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 24,
    "xp": 75,
    "category": "potion"
  },
  {
    "id": "experience_potion",
    "name": "Experience Potion",
    "description": "Double XP gain",
    "result": "xp_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "enchanted_gem",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 25,
    "xp": 80,
    "category": "potion"
  },
  {
    "id": "vision_potion",
    "name": "Vision Potion",
    "description": "See in darkness",
    "result": "vision_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "fire_essence",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 14,
    "xp": 40,
    "category": "potion"
  },
  {
    "id": "full_restore_potion",
    "name": "Full Restore Potion",
    "description": "Fully restores HP and Mana",
    "result": "full_restore_potion",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 28,
    "xp": 100,
    "category": "potion"
  },
  {
    "id": "luck_potion",
    "name": "Luck Potion",
    "description": "Increases luck",
    "result": "luck_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 4
      },
      {
        "id": "gold_bar",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 26,
    "xp": 85,
    "category": "potion"
  },
  {
    "id": "critical_potion",
    "name": "Critical Potion",
    "description": "Increases crit chance",
    "result": "critical_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 4
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 27,
    "xp": 88,
    "category": "potion"
  },
  {
    "id": "cure_all_potion",
    "name": "Cure All Potion",
    "description": "Removes all debuffs",
    "result": "cure_all_potion",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 3
      },
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "unicorn_horn",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 30,
    "xp": 110,
    "category": "potion"
  },
  {
    "id": "ultimate_power_potion",
    "name": "Ultimate Power Potion",
    "description": "Massively boosts all stats",
    "result": "ultimate_power_potion",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "dragon_blood",
        "count": 2
      },
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 40,
    "xp": 180,
    "category": "potion",
    "craftingTime": 5
  },
  {
    "id": "elixir_of_life",
    "name": "Elixir of Life",
    "description": "Revive from death once",
    "result": "elixir_of_life",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "unicorn_horn",
        "count": 1
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "full_restore_potion",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 50,
    "xp": 300,
    "category": "potion",
    "craftingTime": 10
  },
  {
    "id": "fire_essence",
    "name": "Fire Essence",
    "description": "Concentrated flame",
    "result": "fire_essence",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "manacrystal_i",
        "count": 2
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 20,
    "xp": 55,
    "category": "misc"
  },
  {
    "id": "frost_essence",
    "name": "Frost Essence",
    "description": "Concentrated cold",
    "result": "frost_essence",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "manacrystal_i",
        "count": 2
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 20,
    "xp": 55,
    "category": "misc"
  },
  {
    "id": "void_essence",
    "name": "Void Essence",
    "description": "Concentrated void",
    "result": "void_essence",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "ancient_bone",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 28,
    "xp": 85,
    "category": "misc"
  },
  {
    "id": "mana_crystal_i",
    "name": "Mana Crystal I",
    "description": "Basic mana storage",
    "result": "manacrystal_i",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 15,
    "xp": 40,
    "category": "misc"
  },
  {
    "id": "mana_crystal_ii",
    "name": "Mana Crystal II",
    "description": "Improved mana storage",
    "result": "manacrystal_ii",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "manacrystal_i",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 25,
    "xp": 65,
    "category": "misc"
  },
  {
    "id": "mana_crystal_iii",
    "name": "Mana Crystal III",
    "description": "Superior mana storage",
    "result": "manacrystal_iii",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "manacrystal_ii",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 2
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 35,
    "xp": 100,
    "category": "misc"
  },
  {
    "id": "poison_vial",
    "name": "Poison Vial",
    "description": "Coat weapons with poison",
    "result": "health_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 2
      },
      {
        "id": "ancient_bone",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 10,
    "xp": 30,
    "category": "potion"
  },
  {
    "id": "fire_bomb",
    "name": "Fire Bomb",
    "description": "Explodes in flames",
    "result": "bomb",
    "resultCount": 3,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "fire_essence",
        "count": 1
      },
      {
        "id": "iron_ore",
        "count": 2
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 12,
    "xp": 35,
    "category": "misc"
  },
  {
    "id": "frost_bomb",
    "name": "Frost Bomb",
    "description": "Freezing explosion",
    "result": "bomb",
    "resultCount": 3,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "frost_essence",
        "count": 1
      },
      {
        "id": "iron_ore",
        "count": 2
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 12,
    "xp": 35,
    "category": "misc"
  },
  {
    "id": "void_bomb",
    "name": "Void Bomb",
    "description": "Tears reality",
    "result": "bomb",
    "resultCount": 2,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "void_essence",
        "count": 1
      },
      {
        "id": "iron_ore",
        "count": 3
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 25,
    "xp": 70,
    "category": "misc"
  },
  {
    "id": "dragon_blood_extract",
    "name": "Dragon Blood Extract",
    "description": "Purified dragon blood",
    "result": "dragon_blood",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "dragon_scale",
        "count": 2
      },
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 30,
    "xp": 95,
    "category": "misc"
  },
  {
    "id": "soul_gem",
    "name": "Soul Gem",
    "description": "Contains captured souls",
    "result": "soul_gem",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "enchanted_gem",
        "count": 2
      },
      {
        "id": "void_essence",
        "count": 2
      },
      {
        "id": "ancient_bone",
        "count": 3
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 35,
    "xp": 130,
    "category": "misc"
  },
  {
    "id": "teleport_crystal",
    "name": "Teleport Crystal",
    "description": "One-time teleport",
    "result": "teleport_crystal",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "void_essence",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 28,
    "xp": 85,
    "category": "misc"
  },
  {
    "id": "potion_of_stone_skin",
    "name": "Potion of Stone Skin",
    "description": "Hardens your skin",
    "result": "defense_potion",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 3
      },
      {
        "id": "red_herb",
        "count": 3
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 22,
    "xp": 65,
    "category": "potion"
  },
  {
    "id": "potion_of_eagle_eye",
    "name": "Potion of Eagle Eye",
    "description": "Enhanced vision",
    "result": "vision_potion",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 4
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 16,
    "xp": 48,
    "category": "potion"
  },
  {
    "id": "potion_of_the_bear",
    "name": "Potion of the Bear",
    "description": "Massive strength boost",
    "result": "strength_potion",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 6
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 32,
    "xp": 110,
    "category": "potion"
  },
  {
    "id": "potion_of_the_hawk",
    "name": "Potion of the Hawk",
    "description": "Extreme speed",
    "result": "speed_potion",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 6
      },
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 34,
    "xp": 120,
    "category": "potion"
  },
  {
    "id": "titans_brew",
    "name": "Titan's Brew",
    "description": "Temporary invincibility",
    "result": "defense_potion",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "dragon_scale",
        "count": 2
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "empty_vial",
        "count": 1
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 45,
    "xp": 200,
    "category": "potion",
    "craftingTime": 5
  },
  {
    "id": "cooked_meat",
    "name": "Cooked Meat",
    "description": "Simple roasted meat",
    "result": "meat",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 1,
    "xp": 5,
    "category": "food",
    "foodBuff": "well_fed"
  },
  {
    "id": "hearty_bread",
    "name": "Hearty Bread",
    "description": "Fresh baked bread",
    "result": "bread",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 2,
    "xp": 8,
    "category": "food",
    "foodBuff": "well_fed"
  },
  {
    "id": "fresh_cheese",
    "name": "Fresh Cheese",
    "description": "Aged to perfection",
    "result": "cheese",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 3,
    "xp": 10,
    "category": "food",
    "foodBuff": "nourished"
  },
  {
    "id": "baked_apple",
    "name": "Baked Apple",
    "description": "A warm treat",
    "result": "apple",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "apple",
        "count": 1
      },
      {
        "id": "red_herb",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 4,
    "xp": 12,
    "category": "food",
    "foodBuff": "energized"
  },
  {
    "id": "meat_stew",
    "name": "Meat Stew",
    "description": "Hearty and filling",
    "result": "meat",
    "resultCount": 2,
    "rarity": "common",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      },
      {
        "id": "red_herb",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 5,
    "xp": 18,
    "category": "food",
    "foodBuff": "well_fed"
  },
  {
    "id": "warriors_ration",
    "name": "Warrior's Ration",
    "description": "Boosts attack",
    "result": "meat",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      },
      {
        "id": "red_herb",
        "count": 3
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 10,
    "xp": 30,
    "category": "food",
    "foodBuff": "strengthened"
  },
  {
    "id": "defenders_meal",
    "name": "Defender's Meal",
    "description": "Boosts defense",
    "result": "meat",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      },
      {
        "id": "moon_flower",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 10,
    "xp": 30,
    "category": "food",
    "foodBuff": "fortified"
  },
  {
    "id": "swift_bread",
    "name": "Swift Bread",
    "description": "Speed enhancing",
    "result": "bread",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 4
      },
      {
        "id": "moon_flower",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 12,
    "xp": 35,
    "category": "food",
    "foodBuff": "hastened"
  },
  {
    "id": "lucky_dumpling",
    "name": "Lucky Dumpling",
    "description": "Increases luck",
    "result": "bread",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 3
      },
      {
        "id": "gold_bar",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 14,
    "xp": 40,
    "category": "food",
    "foodBuff": "lucky"
  },
  {
    "id": "scholars_porridge",
    "name": "Scholar's Porridge",
    "description": "Increases XP gain",
    "result": "bread",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 4
      },
      {
        "id": "red_herb",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 15,
    "xp": 45,
    "category": "food",
    "foodBuff": "focused"
  },
  {
    "id": "healing_soup",
    "name": "Healing Soup",
    "description": "Regenerates HP",
    "result": "meat",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      },
      {
        "id": "red_herb",
        "count": 4
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 16,
    "xp": 48,
    "category": "food",
    "foodBuff": "nourished"
  },
  {
    "id": "mana_biscuits",
    "name": "Mana Biscuits",
    "description": "Regenerates mana",
    "result": "bread",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "red_herb",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 18,
    "xp": 52,
    "category": "food",
    "foodBuff": "enlightened"
  },
  {
    "id": "dragon_fruit_salad",
    "name": "Dragon Fruit Salad",
    "description": "Exotic and powerful",
    "result": "dragon_fruit",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "apple",
        "count": 3
      },
      {
        "id": "dragon_scale",
        "count": 1
      },
      {
        "id": "moon_flower",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 20,
    "xp": 60,
    "category": "food",
    "foodBuff": "heroic"
  },
  {
    "id": "heros_feast",
    "name": "Hero's Feast",
    "description": "Complete meal for warriors",
    "result": "feast",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "meat",
        "count": 2
      },
      {
        "id": "bread",
        "count": 2
      },
      {
        "id": "cheese",
        "count": 1
      },
      {
        "id": "apple",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 25,
    "xp": 80,
    "category": "food",
    "foodBuff": "heroic"
  },
  {
    "id": "kings_banquet",
    "name": "King's Banquet",
    "description": "Fit for royalty",
    "result": "feast",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "meat",
        "count": 3
      },
      {
        "id": "bread",
        "count": 3
      },
      {
        "id": "cheese",
        "count": 2
      },
      {
        "id": "apple",
        "count": 3
      },
      {
        "id": "gold_bar",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 30,
    "xp": 110,
    "category": "food",
    "foodBuff": "heroic",
    "craftingTime": 3
  },
  {
    "id": "dragon_steak",
    "name": "Dragon Steak",
    "description": "Meat from a dragon",
    "result": "meat",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 3
      },
      {
        "id": "dragon_scale",
        "count": 2
      },
      {
        "id": "fire_essence",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 32,
    "xp": 120,
    "category": "food",
    "foodBuff": "strengthened"
  },
  {
    "id": "phoenix_pie",
    "name": "Phoenix Pie",
    "description": "Legendary pastry",
    "result": "feast",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "apple",
        "count": 5
      },
      {
        "id": "red_herb",
        "count": 3
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 35,
    "xp": 140,
    "category": "food",
    "foodBuff": "survivors_will"
  },
  {
    "id": "ancient_wine",
    "name": "Ancient Wine",
    "description": "Aged for centuries",
    "result": "ancient_wine",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "apple",
        "count": 5
      },
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "red_herb",
        "count": 3
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 28,
    "xp": 95,
    "category": "food",
    "foodBuff": "heroic",
    "craftingTime": 5
  },
  {
    "id": "golden_apple",
    "name": "Golden Apple",
    "description": "A divine fruit",
    "result": "golden_apple",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "apple",
        "count": 3
      },
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "moon_flower",
        "count": 3
      },
      {
        "id": "phoenix_feather",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 40,
    "xp": 180,
    "category": "food",
    "foodBuff": "legendary_feast",
    "craftingTime": 5
  },
  {
    "id": "ambrosia",
    "name": "Ambrosia",
    "description": "Food of the gods",
    "result": "feast",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "golden_apple",
        "count": 1
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "unicorn_horn",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 50,
    "xp": 300,
    "category": "food",
    "foodBuff": "legendary_feast",
    "craftingTime": 10
  },
  {
    "id": "survivors_hardtack",
    "name": "Survivor's Hardtack",
    "description": "Never give up",
    "result": "bread",
    "resultCount": 2,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "red_herb",
        "count": 5
      },
      {
        "id": "moon_flower",
        "count": 5
      },
      {
        "id": "steel_ingot",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 38,
    "xp": 150,
    "category": "food",
    "foodBuff": "survivors_will"
  },
  {
    "id": "spicy_stew",
    "name": "Spicy Stew",
    "description": "Fire resistance food",
    "result": "meat",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      },
      {
        "id": "fire_essence",
        "count": 1
      },
      {
        "id": "red_herb",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 15,
    "xp": 45,
    "category": "food"
  },
  {
    "id": "frozen_treat",
    "name": "Frozen Treat",
    "description": "Ice resistance food",
    "result": "apple",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "apple",
        "count": 2
      },
      {
        "id": "frost_essence",
        "count": 1
      },
      {
        "id": "moon_flower",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 15,
    "xp": 45,
    "category": "food"
  },
  {
    "id": "dungeon_rations",
    "name": "Dungeon Rations",
    "description": "Long lasting food",
    "result": "bread",
    "resultCount": 3,
    "rarity": "common",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      },
      {
        "id": "red_herb",
        "count": 2
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 8,
    "xp": 25,
    "category": "food",
    "foodBuff": "well_fed"
  },
  {
    "id": "mages_delight",
    "name": "Mage's Delight",
    "description": "Mana focused meal",
    "result": "bread",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "moon_flower",
        "count": 6
      },
      {
        "id": "manacrystal_i",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 22,
    "xp": 70,
    "category": "food",
    "foodBuff": "energized"
  },
  {
    "id": "berserkers_blood_pudding",
    "name": "Berserker's Blood Pudding",
    "description": "Rage inducing",
    "result": "meat",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 3
      },
      {
        "id": "dragon_blood",
        "count": 1
      }
    ],
    "station": "cooking_fire",
    "skill": "cooking",
    "skillLevel": 24,
    "xp": 75,
    "category": "food",
    "foodBuff": "strengthened"
  },
  {
    "id": "scroll_of_mapping",
    "name": "Scroll of Mapping",
    "description": "Reveals the map",
    "result": "scroll_mapping",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "manacrystal_i",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 2,
    "xp": 15,
    "category": "misc"
  },
  {
    "id": "scroll_of_identify",
    "name": "Scroll of Identify",
    "description": "Identifies items",
    "result": "scroll_identify",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "manacrystal_i",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 3,
    "xp": 18,
    "category": "misc"
  },
  {
    "id": "scroll_of_fireball",
    "name": "Scroll of Fireball",
    "description": "Casts fireball",
    "result": "scroll_fireball",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "fire_essence",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 5,
    "xp": 25,
    "category": "misc"
  },
  {
    "id": "scroll_of_ice_storm",
    "name": "Scroll of Ice Storm",
    "description": "Freezing attack",
    "result": "scroll_ice_storm",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "frost_essence",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 5,
    "xp": 25,
    "category": "misc"
  },
  {
    "id": "scroll_of_lightning",
    "name": "Scroll of Lightning",
    "description": "Electric attack",
    "result": "scroll_lightning",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "manacrystal_i",
        "count": 2
      },
      {
        "id": "gold_bar",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 6,
    "xp": 28,
    "category": "misc"
  },
  {
    "id": "scroll_of_teleport",
    "name": "Scroll of Teleport",
    "description": "Short range teleport",
    "result": "scroll_teleport",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 10,
    "xp": 40,
    "category": "misc"
  },
  {
    "id": "scroll_of_enchant",
    "name": "Scroll of Enchant",
    "description": "Enchant equipment",
    "result": "scroll_enchant",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 15,
    "xp": 55,
    "category": "misc"
  },
  {
    "id": "scroll_of_summon",
    "name": "Scroll of Summon",
    "description": "Summon ally",
    "result": "scroll_summon",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "soul_gem",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 18,
    "xp": 65,
    "category": "misc"
  },
  {
    "id": "scroll_of_banish",
    "name": "Scroll of Banish",
    "description": "Banish enemies",
    "result": "scroll_banish",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 2
      },
      {
        "id": "manacrystal_ii",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 20,
    "xp": 70,
    "category": "misc"
  },
  {
    "id": "scroll_of_mass_heal",
    "name": "Scroll of Mass Heal",
    "description": "Heal all allies",
    "result": "scroll_mass_heal",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "phoenix_feather",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 22,
    "xp": 80,
    "category": "misc"
  },
  {
    "id": "scroll_of_chain_lightning",
    "name": "Scroll of Chain Lightning",
    "description": "Bouncing lightning",
    "result": "scroll_chain_lightning",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 3
      },
      {
        "id": "gold_bar",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 24,
    "xp": 85,
    "category": "misc"
  },
  {
    "id": "scroll_of_blizzard",
    "name": "Scroll of Blizzard",
    "description": "Massive ice storm",
    "result": "scroll_blizzard",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "frost_essence",
        "count": 4
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 26,
    "xp": 95,
    "category": "misc"
  },
  {
    "id": "scroll_of_meteor",
    "name": "Scroll of Meteor",
    "description": "Summon meteors",
    "result": "scroll_meteor",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "fire_essence",
        "count": 4
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 28,
    "xp": 100,
    "category": "misc"
  },
  {
    "id": "scroll_of_earthquake",
    "name": "Scroll of Earthquake",
    "description": "Shake the ground",
    "result": "scroll_earthquake",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "iron_ore",
        "count": 5
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 25,
    "xp": 90,
    "category": "misc"
  },
  {
    "id": "scroll_of_time_stop",
    "name": "Scroll of Time Stop",
    "description": "Freeze time",
    "result": "scroll_time_stop",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 3
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      },
      {
        "id": "ancient_relic",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 40,
    "xp": 180,
    "category": "misc",
    "craftingTime": 5
  },
  {
    "id": "scroll_of_death",
    "name": "Scroll of Death",
    "description": "Instant death chance",
    "result": "scroll_death",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "demon_heart",
        "count": 1
      },
      {
        "id": "soul_gem",
        "count": 2
      },
      {
        "id": "ancient_bone",
        "count": 3
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 45,
    "xp": 200,
    "category": "misc",
    "craftingTime": 5
  },
  {
    "id": "scroll_of_divine_wrath",
    "name": "Scroll of Divine Wrath",
    "description": "Holy devastation",
    "result": "scroll_divine_wrath",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "manacrystal_iii",
        "count": 3
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 48,
    "xp": 220,
    "category": "misc",
    "craftingTime": 5
  },
  {
    "id": "scroll_of_darkness",
    "name": "Scroll of Darkness",
    "description": "Consume in shadow",
    "result": "scroll_darkness",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "blank_scroll",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 4
      },
      {
        "id": "cursed_fabric",
        "count": 3
      },
      {
        "id": "demon_heart",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 50,
    "xp": 250,
    "category": "misc",
    "craftingTime": 6
  },
  {
    "id": "wand_of_fire",
    "name": "Wand of Fire",
    "description": "Shoots fireballs",
    "result": "wand",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "staff",
        "count": 1
      },
      {
        "id": "fire_essence",
        "count": 3
      },
      {
        "id": "manacrystal_i",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 15,
    "xp": 55,
    "category": "weapon",
    "enchantment": "fire"
  },
  {
    "id": "wand_of_frost",
    "name": "Wand of Frost",
    "description": "Shoots ice bolts",
    "result": "wand",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "staff",
        "count": 1
      },
      {
        "id": "frost_essence",
        "count": 3
      },
      {
        "id": "manacrystal_i",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 15,
    "xp": 55,
    "category": "weapon",
    "enchantment": "frost"
  },
  {
    "id": "wand_of_lightning",
    "name": "Wand of Lightning",
    "description": "Shoots lightning",
    "result": "wand",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "staff",
        "count": 1
      },
      {
        "id": "manacrystal_ii",
        "count": 3
      },
      {
        "id": "gold_bar",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 20,
    "xp": 75,
    "category": "weapon",
    "enchantment": "lightning"
  },
  {
    "id": "wand_of_the_void",
    "name": "Wand of the Void",
    "description": "Shoots void bolts",
    "result": "wand",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "staff",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 4
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 35,
    "xp": 140,
    "category": "weapon",
    "enchantment": "chaos"
  },
  {
    "id": "enchanted_gem",
    "name": "Enchanted Gem",
    "description": "A magical gemstone",
    "result": "enchanted_gem",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "manacrystal_i",
        "count": 2
      },
      {
        "id": "gold_bar",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 10,
    "xp": 35,
    "category": "misc"
  },
  {
    "id": "greater_enchanted_gem",
    "name": "Greater Enchanted Gem",
    "description": "Powerful gem",
    "result": "enchanted_gem",
    "resultCount": 2,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 25,
    "xp": 80,
    "category": "misc"
  },
  {
    "id": "supreme_enchanted_gem",
    "name": "Supreme Enchanted Gem",
    "description": "Ultimate gem",
    "result": "enchanted_gem",
    "resultCount": 3,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "manacrystal_iii",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "enchanting",
    "skillLevel": 35,
    "xp": 130,
    "category": "misc"
  },
  {
    "id": "torch",
    "name": "Torch",
    "description": "Illuminates darkness",
    "result": "torch",
    "resultCount": 5,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 1
      },
      {
        "id": "fire_essence",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "enchanting",
    "skillLevel": 5,
    "xp": 15,
    "category": "misc"
  },
  {
    "id": "compass",
    "name": "Compass",
    "description": "Points to exit",
    "result": "compass",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 2
      },
      {
        "id": "manacrystal_i",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "enchanting",
    "skillLevel": 8,
    "xp": 28,
    "category": "misc"
  },
  {
    "id": "blank_scroll",
    "name": "Blank Scroll",
    "description": "For writing spells",
    "result": "blank_scroll",
    "resultCount": 3,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "enchanting",
    "skillLevel": 1,
    "xp": 8,
    "category": "misc"
  },
  {
    "id": "leather_strip",
    "name": "Leather Strip",
    "description": "Basic leather",
    "result": "leather_strip",
    "resultCount": 3,
    "rarity": "common",
    "ingredients": [
      {
        "id": "raw_meat",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 1,
    "xp": 5,
    "category": "misc"
  },
  {
    "id": "leather_armor_tailoring",
    "name": "Leather Armor",
    "description": "Basic protection",
    "result": "leather_armor",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 5
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 5,
    "xp": 20,
    "category": "armor"
  },
  {
    "id": "leather_cap_tailoring",
    "name": "Leather Cap",
    "description": "Head protection",
    "result": "leather_cap",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 3
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 3,
    "xp": 12,
    "category": "armor"
  },
  {
    "id": "leather_gloves_tailoring",
    "name": "Leather Gloves",
    "description": "Hand protection",
    "result": "leather_gloves",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 2,
    "xp": 8,
    "category": "armor"
  },
  {
    "id": "leather_boots_tailoring",
    "name": "Leather Boots",
    "description": "Foot protection",
    "result": "leather_boots",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 3
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 4,
    "xp": 15,
    "category": "armor"
  },
  {
    "id": "cursed_fabric",
    "name": "Cursed Fabric",
    "description": "Dark cloth",
    "result": "cursed_fabric",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 3
      },
      {
        "id": "void_essence",
        "count": 1
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 15,
    "xp": 50,
    "category": "misc"
  },
  {
    "id": "mage_robes_tailoring",
    "name": "Mage Robes",
    "description": "Magical clothing",
    "result": "mage_robes",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "cursed_fabric",
        "count": 4
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 25,
    "xp": 90,
    "category": "armor",
    "enchantment": "wisdom"
  },
  {
    "id": "assassin_garb_tailoring",
    "name": "Assassin Garb",
    "description": "Stealthy attire",
    "result": "assassin_garb",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "leather_strip",
        "count": 5
      },
      {
        "id": "cursed_fabric",
        "count": 3
      },
      {
        "id": "void_essence",
        "count": 2
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 28,
    "xp": 100,
    "category": "armor",
    "enchantment": "stealth"
  },
  {
    "id": "shadow_cloak_tailoring",
    "name": "Shadow Cloak",
    "description": "Embrace darkness",
    "result": "shadow_cloak",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "cursed_fabric",
        "count": 6
      },
      {
        "id": "void_essence",
        "count": 4
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 35,
    "xp": 150,
    "category": "armor",
    "enchantment": "evasion"
  },
  {
    "id": "hood_of_shadows_tailoring",
    "name": "Hood of Shadows",
    "description": "Hide in darkness",
    "result": "hood_of_shadows",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "cursed_fabric",
        "count": 4
      },
      {
        "id": "void_essence",
        "count": 3
      }
    ],
    "station": "loom",
    "skill": "tailoring",
    "skillLevel": 32,
    "xp": 130,
    "category": "armor",
    "enchantment": "stealth"
  },
  {
    "id": "ring_of_strength",
    "name": "Ring of Strength",
    "description": "+Attack ring",
    "result": "ring_of_strength",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "silver_bar",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 5,
    "xp": 30,
    "category": "misc"
  },
  {
    "id": "ring_of_protection",
    "name": "Ring of Protection",
    "description": "+Defense ring",
    "result": "ring_of_protection",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "silver_bar",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 5,
    "xp": 30,
    "category": "misc"
  },
  {
    "id": "ring_of_speed",
    "name": "Ring of Speed",
    "description": "+Speed ring",
    "result": "ring_of_speed",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "silver_bar",
        "count": 2
      },
      {
        "id": "manacrystal_i",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 8,
    "xp": 35,
    "category": "misc"
  },
  {
    "id": "ring_of_regeneration",
    "name": "Ring of Regeneration",
    "description": "HP regen ring",
    "result": "ring_of_regeneration",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 1
      },
      {
        "id": "red_herb",
        "count": 5
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 12,
    "xp": 50,
    "category": "misc"
  },
  {
    "id": "ring_of_mana",
    "name": "Ring of Mana",
    "description": "+Mana ring",
    "result": "ring_of_mana",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 2
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 15,
    "xp": 60,
    "category": "misc"
  },
  {
    "id": "ring_of_luck",
    "name": "Ring of Luck",
    "description": "Better drops",
    "result": "ring_of_luck",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 18,
    "xp": 70,
    "category": "misc"
  },
  {
    "id": "ring_of_fireball",
    "name": "Ring of Fireball",
    "description": "Cast fireballs",
    "result": "ring_of_fireball",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 2
      },
      {
        "id": "fire_essence",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 20,
    "xp": 80,
    "category": "misc",
    "enchantment": "fire"
  },
  {
    "id": "ring_of_invisibility",
    "name": "Ring of Invisibility",
    "description": "Turn invisible",
    "result": "ring_of_invisibility",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 3
      },
      {
        "id": "void_essence",
        "count": 2
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 25,
    "xp": 100,
    "category": "misc",
    "enchantment": "stealth"
  },
  {
    "id": "ring_of_the_vampire",
    "name": "Ring of the Vampire",
    "description": "Lifesteal",
    "result": "ring_of_the_vampire",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 3
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "enchanted_gem",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 28,
    "xp": 110,
    "category": "misc",
    "enchantment": "vampiric"
  },
  {
    "id": "ring_of_frost",
    "name": "Ring of Frost",
    "description": "Freeze enemies",
    "result": "ring_of_frost",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "silver_bar",
        "count": 3
      },
      {
        "id": "frost_essence",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 22,
    "xp": 85,
    "category": "misc",
    "enchantment": "frost"
  },
  {
    "id": "ring_of_flame",
    "name": "Ring of Flame",
    "description": "Burn enemies",
    "result": "ring_of_flame",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 2
      },
      {
        "id": "fire_essence",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 22,
    "xp": 85,
    "category": "misc",
    "enchantment": "fire"
  },
  {
    "id": "ring_of_thunder",
    "name": "Ring of Thunder",
    "description": "Shock enemies",
    "result": "ring_of_thunder",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 4
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 24,
    "xp": 90,
    "category": "misc",
    "enchantment": "lightning"
  },
  {
    "id": "ring_of_shadows",
    "name": "Ring of Shadows",
    "description": "Embrace shadows",
    "result": "ring_of_shadows",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "silver_bar",
        "count": 3
      },
      {
        "id": "void_essence",
        "count": 3
      },
      {
        "id": "cursed_fabric",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 30,
    "xp": 120,
    "category": "misc",
    "enchantment": "stealth"
  },
  {
    "id": "ring_of_death",
    "name": "Ring of Death",
    "description": "Instant kill chance",
    "result": "ring_of_death",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "demon_heart",
        "count": 1
      },
      {
        "id": "soul_gem",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 40,
    "xp": 180,
    "category": "misc",
    "enchantment": "oblivion",
    "craftingTime": 5
  },
  {
    "id": "ring_of_the_ancients",
    "name": "Ring of the Ancients",
    "description": "Ancient power",
    "result": "ring_of_the_ancients",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 4
      },
      {
        "id": "ancient_bone",
        "count": 3
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      },
      {
        "id": "ancient_relic",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 45,
    "xp": 200,
    "category": "misc",
    "craftingTime": 5
  },
  {
    "id": "amulet_of_health",
    "name": "Amulet of Health",
    "description": "+HP amulet",
    "result": "amulet_of_health",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 3
      },
      {
        "id": "enchanted_gem",
        "count": 2
      },
      {
        "id": "red_herb",
        "count": 5
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 15,
    "xp": 60,
    "category": "misc",
    "enchantment": "vitality"
  },
  {
    "id": "amulet_of_mana",
    "name": "Amulet of Mana",
    "description": "+Mana amulet",
    "result": "amulet_of_mana",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 3
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      },
      {
        "id": "moon_flower",
        "count": 5
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 18,
    "xp": 70,
    "category": "misc",
    "enchantment": "wisdom"
  },
  {
    "id": "amulet_of_protection",
    "name": "Amulet of Protection",
    "description": "+Defense amulet",
    "result": "amulet_of_protection",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 3
      },
      {
        "id": "steel_ingot",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 20,
    "xp": 75,
    "category": "misc",
    "enchantment": "protection"
  },
  {
    "id": "amulet_of_power",
    "name": "Amulet of Power",
    "description": "+Attack amulet",
    "result": "amulet_of_power",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 4
      },
      {
        "id": "dragon_blood",
        "count": 1
      },
      {
        "id": "enchanted_gem",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 25,
    "xp": 100,
    "category": "misc",
    "enchantment": "sharpness"
  },
  {
    "id": "amulet_of_wisdom",
    "name": "Amulet of Wisdom",
    "description": "+XP amulet",
    "result": "amulet_of_wisdom",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 4
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      },
      {
        "id": "enchanted_gem",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 28,
    "xp": 110,
    "category": "misc",
    "enchantment": "experience"
  },
  {
    "id": "amulet_of_life",
    "name": "Amulet of Life",
    "description": "Revive once",
    "result": "amulet_of_life",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 35,
    "xp": 160,
    "category": "misc",
    "enchantment": "resurrection",
    "craftingTime": 5
  },
  {
    "id": "amulet_of_death",
    "name": "Amulet of Death",
    "description": "Death aura",
    "result": "amulet_of_death",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "demon_heart",
        "count": 2
      },
      {
        "id": "soul_gem",
        "count": 2
      },
      {
        "id": "ancient_bone",
        "count": 3
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 40,
    "xp": 180,
    "category": "misc",
    "craftingTime": 5
  },
  {
    "id": "amulet_of_dragons",
    "name": "Amulet of Dragons",
    "description": "Dragon power",
    "result": "amulet_of_dragons",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "dragon_scale",
        "count": 3
      },
      {
        "id": "dragon_blood",
        "count": 2
      },
      {
        "id": "manacrystal_iii",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 42,
    "xp": 190,
    "category": "misc",
    "craftingTime": 5
  },
  {
    "id": "amulet_of_chaos",
    "name": "Amulet of Chaos",
    "description": "Random effects",
    "result": "amulet_of_chaos",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "void_essence",
        "count": 4
      },
      {
        "id": "demon_heart",
        "count": 1
      },
      {
        "id": "phoenix_feather",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 45,
    "xp": 200,
    "category": "misc",
    "enchantment": "chaos",
    "craftingTime": 5
  },
  {
    "id": "amulet_of_order",
    "name": "Amulet of Order",
    "description": "Stability",
    "result": "amulet_of_order",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 5
      },
      {
        "id": "manacrystal_iii",
        "count": 4
      },
      {
        "id": "enchanted_gem",
        "count": 4
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 45,
    "xp": 200,
    "category": "misc",
    "enchantment": "fortification",
    "craftingTime": 5
  },
  {
    "id": "amulet_of_balance",
    "name": "Amulet of Balance",
    "description": "Perfect harmony",
    "result": "amulet_of_balance",
    "resultCount": 1,
    "rarity": "legendary",
    "ingredients": [
      {
        "id": "amulet_of_chaos",
        "count": 1
      },
      {
        "id": "amulet_of_order",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 50,
    "xp": 300,
    "category": "misc",
    "craftingTime": 8
  },
  {
    "id": "amulet_of_the_gods",
    "name": "Amulet of the Gods",
    "description": "Divine power",
    "result": "amulet_of_the_gods",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 8
      },
      {
        "id": "manacrystal_iii",
        "count": 5
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "dragon_scale",
        "count": 3
      },
      {
        "id": "unicorn_horn",
        "count": 1
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 60,
    "xp": 400,
    "category": "misc",
    "enchantment": "blessed",
    "craftingTime": 10
  },
  {
    "id": "rune_stone",
    "name": "Rune Stone",
    "description": "Basic rune material",
    "result": "rune_stone",
    "resultCount": 1,
    "rarity": "common",
    "ingredients": [
      {
        "id": "iron_ore",
        "count": 2
      },
      {
        "id": "manacrystal_i",
        "count": 1
      }
    ],
    "station": "enchanting_altar",
    "skill": "runecraft",
    "skillLevel": 1,
    "xp": 10,
    "category": "misc"
  },
  {
    "id": "fire_rune",
    "name": "Fire Rune",
    "description": "Imbue with fire",
    "result": "fire_essence",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "rune_stone",
        "count": 1
      },
      {
        "id": "fire_essence",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "runecraft",
    "skillLevel": 10,
    "xp": 35,
    "category": "misc"
  },
  {
    "id": "frost_rune",
    "name": "Frost Rune",
    "description": "Imbue with frost",
    "result": "frost_essence",
    "resultCount": 1,
    "rarity": "uncommon",
    "ingredients": [
      {
        "id": "rune_stone",
        "count": 1
      },
      {
        "id": "frost_essence",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "runecraft",
    "skillLevel": 10,
    "xp": 35,
    "category": "misc"
  },
  {
    "id": "void_rune",
    "name": "Void Rune",
    "description": "Imbue with void",
    "result": "void_essence",
    "resultCount": 1,
    "rarity": "rare",
    "ingredients": [
      {
        "id": "rune_stone",
        "count": 1
      },
      {
        "id": "void_essence",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "runecraft",
    "skillLevel": 20,
    "xp": 65,
    "category": "misc"
  },
  {
    "id": "ancient_relic",
    "name": "Ancient Relic",
    "description": "Mysterious power",
    "result": "ancient_relic",
    "resultCount": 1,
    "rarity": "epic",
    "ingredients": [
      {
        "id": "rune_stone",
        "count": 3
      },
      {
        "id": "ancient_bone",
        "count": 2
      },
      {
        "id": "manacrystal_ii",
        "count": 2
      }
    ],
    "station": "enchanting_altar",
    "skill": "runecraft",
    "skillLevel": 30,
    "xp": 120,
    "category": "misc"
  },
  {
    "id": "godslayer_blade",
    "name": "Godslayer Blade",
    "description": "Bane of the divine",
    "result": "demon_slayer",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 8
      },
      {
        "id": "dragon_blood",
        "count": 3
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "demon_heart",
        "count": 2
      },
      {
        "id": "ancient_relic",
        "count": 1
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 60,
    "xp": 500,
    "category": "weapon",
    "enchantment": "godslayer",
    "rareRecipe": true,
    "craftingTime": 15
  },
  {
    "id": "armor_of_eternity",
    "name": "Armor of Eternity",
    "description": "Timeless protection",
    "result": "titan_plate",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "mithril_ore",
        "count": 10
      },
      {
        "id": "dragon_scale",
        "count": 5
      },
      {
        "id": "phoenix_feather",
        "count": 2
      },
      {
        "id": "manacrystal_iii",
        "count": 5
      }
    ],
    "station": "anvil",
    "skill": "blacksmithing",
    "skillLevel": 65,
    "xp": 600,
    "category": "armor",
    "enchantment": "indestructible",
    "rareRecipe": true,
    "craftingTime": 20
  },
  {
    "id": "crown_of_the_universe",
    "name": "Crown of the Universe",
    "description": "Cosmic power",
    "result": "crystal_crown",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 10
      },
      {
        "id": "manacrystal_iii",
        "count": 8
      },
      {
        "id": "phoenix_feather",
        "count": 3
      },
      {
        "id": "unicorn_horn",
        "count": 2
      }
    ],
    "station": "anvil",
    "skill": "jewelcrafting",
    "skillLevel": 70,
    "xp": 700,
    "category": "armor",
    "enchantment": "omniscience",
    "rareRecipe": true,
    "craftingTime": 25
  },
  {
    "id": "potion_of_immortality",
    "name": "Potion of Immortality",
    "description": "Cheat death forever",
    "result": "elixir_of_life",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "elixir_of_life",
        "count": 3
      },
      {
        "id": "phoenix_feather",
        "count": 5
      },
      {
        "id": "unicorn_horn",
        "count": 3
      },
      {
        "id": "dragon_blood",
        "count": 3
      }
    ],
    "station": "alchemy_table",
    "skill": "alchemy",
    "skillLevel": 70,
    "xp": 800,
    "category": "potion",
    "rareRecipe": true,
    "craftingTime": 30
  },
  {
    "id": "timewarp_ring",
    "name": "Timewarp Ring",
    "description": "Bend time itself",
    "result": "ring_of_the_ancients",
    "resultCount": 1,
    "rarity": "mythic",
    "ingredients": [
      {
        "id": "gold_bar",
        "count": 8
      },
      {
        "id": "void_essence",
        "count": 6
      },
      {
        "id": "manacrystal_iii",
        "count": 5
      },
      {
        "id": "ancient_relic",
        "count": 2
      }
    ],
    "station": "workbench",
    "skill": "jewelcrafting",
    "skillLevel": 75,
    "xp": 900,
    "category": "misc",
    "enchantment": "timewarp",
    "rareRecipe": true,
    "craftingTime": 20
  }
];

SC.DATA.crafting_notes = [
  "Ported from src/crafting.rs and src/recipes.rs; recipe names, descriptions, results, counts, rarities, ingredient counts, stations, skill requirements, XP rewards, enchantments, food buffs, crafting times, and rare-recipe flags are verbatim from the Rust source.",
  "Ids are snake_case of the Rust ItemKind variant names (e.g. manacrystal_ii, xp_potion, thieves_gloves); recipe ids are snake_case of the recipe names.",
  "Station ids follow the Rust CraftingStation enum: forge, anvil, workbench, alchemy_table, enchanting_altar, cooking_fire, loom (instead of the generic apothecary/kitchen examples).",
  "Recipe categories are collapsed to the five contract buckets; rings, amulets, scrolls, bombs, tools, and crafted materials fall under \"misc\".",
  "Extra recipe fields beyond the base contract (description, rarity, skill, xp, enchantment, foodBuff, craftingTime, rareRecipe) preserve verbatim data from recipes.rs; craftingTime is omitted when it is the default 1 turn, and enchantment/foodBuff ids are snake_case of the Rust enum variants.",
  "Material glyphs and colors are verbatim from main.rs ItemKind: crafting materials fall through to glyph \"*\" and color White (#ecf0f1), except dragon_scale \"s\", demon_heart \"h\", and soul_gem \"o\".",
  "Material gold values are invented (the Rust source assigns no gold value to crafting materials); they are scaled by tier for a web shop UI.",
  "Material tiers are inferred (1 = basic gatherables up to 5 = mythic-grade reagents); the Rust source defines no material tiers.",
  "The 18 alchemy herbs (HerbType in crafting.rs) are defined in the source but not consumed by any recipe; their descriptions come from source comments, rarity is verbatim (mapped to tier 1/2/3/5), and their glyph (\") and colors are invented since HerbType defines neither.",
  "Recipes duplicated between Blacksmithing and Tailoring (leather cap/armor/gloves/boots, mage robes, assassin garb, shadow cloak, hood of shadows) keep the smithing id; the tailoring variant gets a \"_tailoring\" suffix.",
  "Ring of Thunder lists Gold Bar twice in recipes.rs (3 + 1); merged here into a single ingredient entry of 4.",
  "Poison Vial's result is ItemKind::HealthPotion in recipes.rs (an apparent source placeholder); ported verbatim as health_potion.",
  "Several themed recipes share a result id with a base recipe (e.g. Greater Health Potion -> health_potion, Fire Rune -> fire_essence, Mithril Ingot -> mithril_ore) because the Rust ItemKind enum has no distinct variant for them; ported verbatim.",
  "Crafting stations, skills, material quality tiers, and the enchantment catalog in crafting.rs are engine/system data (with functions), not item data, so they are not exported here per the single-file data contract."
];

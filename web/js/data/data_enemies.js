'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// ============================================================================
// ShadowCrypt enemy catalog - ported from src/main.rs (EnemyKind, 148 kinds)
// cross-checked against core/src/entities.rs. Generated data file; see
// SC.DATA.enemies_notes / SC.DATA.bosses_notes for porting decisions.
// ============================================================================

SC.DATA.enemies = {
  "rat": {
    "id": "rat",
    "name": "Rat",
    "glyph": "r",
    "color": "#5d6d7e",
    "tier": 1,
    "hp": 8,
    "atk": 3,
    "def": 0,
    "spd": 10,
    "xp": 5,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Rats",
    "spawnFloors": [
      1,
      4
    ]
  },
  "bat": {
    "id": "bat",
    "name": "Bat",
    "glyph": "b",
    "color": "#5d6d7e",
    "tier": 1,
    "hp": 6,
    "atk": 2,
    "def": 0,
    "spd": 10,
    "xp": 4,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Bats",
    "spawnFloors": [
      1,
      4
    ]
  },
  "spider": {
    "id": "spider",
    "name": "Spider",
    "glyph": "s",
    "color": "#b7950b",
    "tier": 1,
    "hp": 10,
    "atk": 4,
    "def": 1,
    "spd": 10,
    "xp": 8,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "pluralName": "Spiders",
    "spawnFloors": [
      1,
      4
    ]
  },
  "goblin": {
    "id": "goblin",
    "name": "Goblin",
    "glyph": "g",
    "color": "#2ecc71",
    "tier": 1,
    "hp": 15,
    "atk": 5,
    "def": 2,
    "spd": 10,
    "xp": 12,
    "flags": {},
    "statusInflicts": [],
    "faction": "orcHorde",
    "pluralName": "Goblins",
    "spawnFloors": [
      1,
      4
    ]
  },
  "skeleton": {
    "id": "skeleton",
    "name": "Skeleton",
    "glyph": "k",
    "color": "#ecf0f1",
    "tier": 1,
    "hp": 12,
    "atk": 6,
    "def": 1,
    "spd": 10,
    "xp": 10,
    "flags": {
      "undead": true
    },
    "statusInflicts": [
      "bleed"
    ],
    "faction": "undeadLegion",
    "pluralName": "Skeletons",
    "spawnFloors": [
      1,
      4
    ]
  },
  "kobold": {
    "id": "kobold",
    "name": "Kobold",
    "glyph": "k",
    "color": "#2ecc71",
    "tier": 1,
    "hp": 10,
    "atk": 4,
    "def": 1,
    "spd": 10,
    "xp": 7,
    "flags": {},
    "statusInflicts": [],
    "faction": "orcHorde",
    "pluralName": "Kobolds",
    "spawnFloors": [
      1,
      4
    ]
  },
  "giantRat": {
    "id": "giantRat",
    "name": "Giant Rat",
    "glyph": "r",
    "color": "#5d6d7e",
    "tier": 1,
    "hp": 14,
    "atk": 5,
    "def": 1,
    "spd": 10,
    "xp": 10,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Giant Rats",
    "spawnFloors": [
      1,
      4
    ]
  },
  "caveCrawler": {
    "id": "caveCrawler",
    "name": "Cave Crawler",
    "glyph": "c",
    "color": "#5d6d7e",
    "tier": 1,
    "hp": 18,
    "atk": 6,
    "def": 2,
    "spd": 10,
    "xp": 15,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Cave Crawlers",
    "spawnFloors": [
      1,
      4
    ]
  },
  "giantSpider": {
    "id": "giantSpider",
    "name": "Giant Spider",
    "glyph": "S",
    "color": "#b7950b",
    "tier": 2,
    "hp": 25,
    "atk": 8,
    "def": 3,
    "spd": 10,
    "xp": 25,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "pluralName": "Giant Spiders",
    "spawnFloors": [
      5,
      8
    ]
  },
  "orc": {
    "id": "orc",
    "name": "Orc",
    "glyph": "o",
    "color": "#1e8449",
    "tier": 2,
    "hp": 35,
    "atk": 10,
    "def": 4,
    "spd": 10,
    "xp": 30,
    "flags": {},
    "statusInflicts": [],
    "faction": "orcHorde",
    "pluralName": "Orcs",
    "spawnFloors": [
      5,
      8
    ]
  },
  "troll": {
    "id": "troll",
    "name": "Troll",
    "glyph": "t",
    "color": "#148f77",
    "tier": 2,
    "hp": 50,
    "atk": 8,
    "def": 6,
    "spd": 10,
    "xp": 40,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Trolls",
    "spawnFloors": [
      5,
      8
    ]
  },
  "caveOgre": {
    "id": "caveOgre",
    "name": "Cave Ogre",
    "glyph": "O",
    "color": "#b7950b",
    "tier": 2,
    "hp": 60,
    "atk": 12,
    "def": 5,
    "spd": 10,
    "xp": 50,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Cave Ogres",
    "spawnFloors": [
      5,
      8
    ]
  },
  "slime": {
    "id": "slime",
    "name": "Slime",
    "glyph": "j",
    "color": "#2ecc71",
    "tier": 2,
    "hp": 40,
    "atk": 6,
    "def": 8,
    "spd": 10,
    "xp": 35,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "pluralName": "Slimes",
    "spawnFloors": [
      5,
      8
    ]
  },
  "hobgoblin": {
    "id": "hobgoblin",
    "name": "Hobgoblin",
    "glyph": "h",
    "color": "#1e8449",
    "tier": 2,
    "hp": 30,
    "atk": 9,
    "def": 4,
    "spd": 10,
    "xp": 28,
    "flags": {},
    "statusInflicts": [],
    "faction": "orcHorde",
    "pluralName": "Hobgoblins",
    "spawnFloors": [
      5,
      8
    ]
  },
  "caveBear": {
    "id": "caveBear",
    "name": "Cave Bear",
    "glyph": "B",
    "color": "#b7950b",
    "tier": 2,
    "hp": 55,
    "atk": 14,
    "def": 6,
    "spd": 10,
    "xp": 45,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Cave Bears",
    "spawnFloors": [
      5,
      8
    ]
  },
  "mushroom": {
    "id": "mushroom",
    "name": "Toxic Mushroom",
    "glyph": "m",
    "color": "#2ecc71",
    "tier": 2,
    "hp": 20,
    "atk": 5,
    "def": 2,
    "spd": 10,
    "xp": 20,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "pluralName": "Toxic Mushrooms",
    "spawnFloors": [
      5,
      8
    ]
  },
  "rockElemental": {
    "id": "rockElemental",
    "name": "Rock Elemental",
    "glyph": "R",
    "color": "#95a5a6",
    "tier": 2,
    "hp": 70,
    "atk": 10,
    "def": 12,
    "spd": 10,
    "xp": 55,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Rock Elementals",
    "spawnFloors": [
      5,
      8
    ]
  },
  "zombie": {
    "id": "zombie",
    "name": "Zombie",
    "glyph": "z",
    "color": "#1e8449",
    "tier": 3,
    "hp": 45,
    "atk": 10,
    "def": 4,
    "spd": 10,
    "xp": 40,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Zombies",
    "spawnFloors": [
      9,
      12
    ]
  },
  "ghost": {
    "id": "ghost",
    "name": "Ghost",
    "glyph": "G",
    "color": "#95a5a6",
    "tier": 3,
    "hp": 30,
    "atk": 12,
    "def": 2,
    "spd": 10,
    "xp": 45,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Ghosts",
    "spawnFloors": [
      9,
      12
    ]
  },
  "wraith": {
    "id": "wraith",
    "name": "Wraith",
    "glyph": "W",
    "color": "#95a5a6",
    "tier": 3,
    "hp": 35,
    "atk": 15,
    "def": 3,
    "spd": 10,
    "xp": 55,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Wraiths",
    "spawnFloors": [
      9,
      12
    ]
  },
  "vampire": {
    "id": "vampire",
    "name": "Vampire",
    "glyph": "V",
    "color": "#922b21",
    "tier": 3,
    "hp": 55,
    "atk": 14,
    "def": 6,
    "spd": 10,
    "xp": 70,
    "flags": {
      "undead": true
    },
    "statusInflicts": [
      "bleed"
    ],
    "faction": "undeadLegion",
    "pluralName": "Vampires",
    "spawnFloors": [
      9,
      12
    ]
  },
  "mummy": {
    "id": "mummy",
    "name": "Mummy",
    "glyph": "M",
    "color": "#ecf0f1",
    "tier": 3,
    "hp": 50,
    "atk": 11,
    "def": 8,
    "spd": 10,
    "xp": 60,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Mummies",
    "spawnFloors": [
      9,
      12
    ]
  },
  "ghoul": {
    "id": "ghoul",
    "name": "Ghoul",
    "glyph": "g",
    "color": "#1e8449",
    "tier": 3,
    "hp": 40,
    "atk": 12,
    "def": 3,
    "spd": 10,
    "xp": 50,
    "flags": {
      "undead": true
    },
    "statusInflicts": [
      "bleed"
    ],
    "faction": "undeadLegion",
    "pluralName": "Ghouls",
    "spawnFloors": [
      9,
      12
    ]
  },
  "banshee": {
    "id": "banshee",
    "name": "Banshee",
    "glyph": "B",
    "color": "#95a5a6",
    "tier": 3,
    "hp": 35,
    "atk": 16,
    "def": 2,
    "spd": 10,
    "xp": 65,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Banshees",
    "spawnFloors": [
      9,
      12
    ]
  },
  "deathKnight": {
    "id": "deathKnight",
    "name": "Death Knight",
    "glyph": "K",
    "color": "#76448a",
    "tier": 3,
    "hp": 75,
    "atk": 18,
    "def": 10,
    "spd": 10,
    "xp": 85,
    "flags": {
      "undead": true
    },
    "statusInflicts": [
      "bleed"
    ],
    "faction": "undeadLegion",
    "pluralName": "Death Knights",
    "spawnFloors": [
      9,
      12
    ]
  },
  "boneGolem": {
    "id": "boneGolem",
    "name": "Bone Golem",
    "glyph": "G",
    "color": "#ecf0f1",
    "tier": 3,
    "hp": 90,
    "atk": 14,
    "def": 12,
    "spd": 10,
    "xp": 80,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Bone Golems",
    "spawnFloors": [
      9,
      12
    ]
  },
  "wolf": {
    "id": "wolf",
    "name": "Wolf",
    "glyph": "w",
    "color": "#95a5a6",
    "tier": 4,
    "hp": 40,
    "atk": 12,
    "def": 3,
    "spd": 10,
    "xp": 50,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "pluralName": "Wolves",
    "spawnFloors": [
      13,
      16
    ]
  },
  "direWolf": {
    "id": "direWolf",
    "name": "Dire Wolf",
    "glyph": "W",
    "color": "#95a5a6",
    "tier": 4,
    "hp": 60,
    "atk": 16,
    "def": 5,
    "spd": 10,
    "xp": 75,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "pluralName": "Dire Wolves",
    "spawnFloors": [
      13,
      16
    ]
  },
  "treeEnt": {
    "id": "treeEnt",
    "name": "Tree Ent",
    "glyph": "T",
    "color": "#2ecc71",
    "tier": 4,
    "hp": 100,
    "atk": 14,
    "def": 12,
    "spd": 10,
    "xp": 100,
    "flags": {},
    "statusInflicts": [],
    "faction": "druids",
    "pluralName": "Tree Ents",
    "spawnFloors": [
      13,
      16
    ]
  },
  "forestTroll": {
    "id": "forestTroll",
    "name": "Forest Troll",
    "glyph": "t",
    "color": "#148f77",
    "tier": 4,
    "hp": 80,
    "atk": 15,
    "def": 8,
    "spd": 10,
    "xp": 90,
    "flags": {},
    "statusInflicts": [],
    "faction": "druids",
    "pluralName": "Forest Trolls",
    "spawnFloors": [
      13,
      16
    ]
  },
  "druid": {
    "id": "druid",
    "name": "Corrupted Druid",
    "glyph": "d",
    "color": "#1e8449",
    "tier": 4,
    "hp": 50,
    "atk": 18,
    "def": 4,
    "spd": 10,
    "xp": 80,
    "flags": {},
    "statusInflicts": [],
    "faction": "druids",
    "pluralName": "Corrupted Druids",
    "spawnFloors": [
      13,
      16
    ]
  },
  "wildBoar": {
    "id": "wildBoar",
    "name": "Wild Boar",
    "glyph": "b",
    "color": "#b7950b",
    "tier": 4,
    "hp": 55,
    "atk": 14,
    "def": 5,
    "spd": 10,
    "xp": 60,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Wild Boars",
    "spawnFloors": [
      13,
      16
    ]
  },
  "giantWasp": {
    "id": "giantWasp",
    "name": "Giant Wasp",
    "glyph": "w",
    "color": "#f1c40f",
    "tier": 4,
    "hp": 35,
    "atk": 16,
    "def": 2,
    "spd": 10,
    "xp": 55,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "pluralName": "Giant Wasps",
    "spawnFloors": [
      13,
      16
    ]
  },
  "venomousVine": {
    "id": "venomousVine",
    "name": "Venomous Vine",
    "glyph": "v",
    "color": "#2ecc71",
    "tier": 4,
    "hp": 45,
    "atk": 12,
    "def": 6,
    "spd": 10,
    "xp": 65,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": "druids",
    "pluralName": "Venomous Vines",
    "spawnFloors": [
      13,
      16
    ]
  },
  "forestSpirit": {
    "id": "forestSpirit",
    "name": "Forest Spirit",
    "glyph": "S",
    "color": "#1e8449",
    "tier": 4,
    "hp": 40,
    "atk": 20,
    "def": 3,
    "spd": 10,
    "xp": 75,
    "flags": {},
    "statusInflicts": [],
    "faction": "druids",
    "pluralName": "Forest Spirits",
    "spawnFloors": [
      13,
      16
    ]
  },
  "iceElemental": {
    "id": "iceElemental",
    "name": "Ice Elemental",
    "glyph": "E",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 70,
    "atk": 18,
    "def": 8,
    "spd": 10,
    "xp": 110,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "pluralName": "Ice Elementals",
    "spawnFloors": [
      17,
      20
    ]
  },
  "frostGiant": {
    "id": "frostGiant",
    "name": "Frost Giant",
    "glyph": "F",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 120,
    "atk": 22,
    "def": 12,
    "spd": 10,
    "xp": 150,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "pluralName": "Frost Giants",
    "spawnFloors": [
      17,
      20
    ]
  },
  "yetiWarrior": {
    "id": "yetiWarrior",
    "name": "Yeti Warrior",
    "glyph": "Y",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 90,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 130,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "pluralName": "Yeti Warriors",
    "spawnFloors": [
      17,
      20
    ]
  },
  "iceWraith": {
    "id": "iceWraith",
    "name": "Ice Wraith",
    "glyph": "w",
    "color": "#95a5a6",
    "tier": 5,
    "hp": 60,
    "atk": 24,
    "def": 6,
    "spd": 10,
    "xp": 120,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": "undeadLegion",
    "pluralName": "Ice Wraiths",
    "spawnFloors": [
      17,
      20
    ]
  },
  "frostWolf": {
    "id": "frostWolf",
    "name": "Frost Wolf",
    "glyph": "w",
    "color": "#95a5a6",
    "tier": 5,
    "hp": 55,
    "atk": 18,
    "def": 6,
    "spd": 10,
    "xp": 100,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "pluralName": "Frost Wolves",
    "spawnFloors": [
      17,
      20
    ]
  },
  "iceSpider": {
    "id": "iceSpider",
    "name": "Ice Spider",
    "glyph": "S",
    "color": "#b7950b",
    "tier": 5,
    "hp": 50,
    "atk": 16,
    "def": 5,
    "spd": 10,
    "xp": 95,
    "flags": {},
    "statusInflicts": [
      "poison",
      "freeze"
    ],
    "faction": null,
    "pluralName": "Ice Spiders",
    "spawnFloors": [
      17,
      20
    ]
  },
  "frozenKnight": {
    "id": "frozenKnight",
    "name": "Frozen Knight",
    "glyph": "K",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 100,
    "atk": 22,
    "def": 14,
    "spd": 10,
    "xp": 140,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "pluralName": "Frozen Knights",
    "spawnFloors": [
      17,
      20
    ]
  },
  "wendigo": {
    "id": "wendigo",
    "name": "Wendigo",
    "glyph": "W",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 85,
    "atk": 26,
    "def": 8,
    "spd": 10,
    "xp": 160,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "pluralName": "Wendigos",
    "spawnFloors": [
      17,
      20
    ]
  },
  "fireElemental": {
    "id": "fireElemental",
    "name": "Fire Elemental",
    "glyph": "E",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 80,
    "atk": 22,
    "def": 8,
    "spd": 10,
    "xp": 140,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "pluralName": "Fire Elementals",
    "spawnFloors": [
      21,
      24
    ]
  },
  "lavaGolem": {
    "id": "lavaGolem",
    "name": "Lava Golem",
    "glyph": "L",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 150,
    "atk": 20,
    "def": 18,
    "spd": 10,
    "xp": 180,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "pluralName": "Lava Golems",
    "spawnFloors": [
      21,
      24
    ]
  },
  "hellhound": {
    "id": "hellhound",
    "name": "Hellhound",
    "glyph": "H",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 70,
    "atk": 25,
    "def": 8,
    "spd": 10,
    "xp": 150,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "pluralName": "Hellhounds",
    "spawnFloors": [
      21,
      24
    ]
  },
  "fireDrake": {
    "id": "fireDrake",
    "name": "Fire Drake",
    "glyph": "D",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 100,
    "atk": 28,
    "def": 12,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "dragonFlight",
    "pluralName": "Fire Drakes",
    "spawnFloors": [
      21,
      24
    ]
  },
  "magmaSlime": {
    "id": "magmaSlime",
    "name": "Magma Slime",
    "glyph": "j",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 65,
    "atk": 18,
    "def": 10,
    "spd": 10,
    "xp": 130,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "pluralName": "Magma Slimes",
    "spawnFloors": [
      21,
      24
    ]
  },
  "salamander": {
    "id": "salamander",
    "name": "Salamander",
    "glyph": "s",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 75,
    "atk": 24,
    "def": 7,
    "spd": 10,
    "xp": 160,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "pluralName": "Salamanders",
    "spawnFloors": [
      21,
      24
    ]
  },
  "cinderWraith": {
    "id": "cinderWraith",
    "name": "Cinder Wraith",
    "glyph": "W",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 55,
    "atk": 28,
    "def": 4,
    "spd": 10,
    "xp": 170,
    "flags": {
      "undead": true
    },
    "statusInflicts": [
      "burn"
    ],
    "faction": "undeadLegion",
    "pluralName": "Cinder Wraiths",
    "spawnFloors": [
      21,
      24
    ]
  },
  "infernalImp": {
    "id": "infernalImp",
    "name": "Infernal Imp",
    "glyph": "i",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 45,
    "atk": 22,
    "def": 5,
    "spd": 10,
    "xp": 120,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "pluralName": "Infernal Imps",
    "spawnFloors": [
      21,
      24
    ]
  },
  "golem": {
    "id": "golem",
    "name": "Stone Golem",
    "glyph": "G",
    "color": "#f1c40f",
    "tier": 7,
    "hp": 180,
    "atk": 22,
    "def": 20,
    "spd": 10,
    "xp": 220,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Stone Golems",
    "spawnFloors": [
      25,
      28
    ]
  },
  "ancientGuardian": {
    "id": "ancientGuardian",
    "name": "Ancient Guardian",
    "glyph": "A",
    "color": "#f1c40f",
    "tier": 7,
    "hp": 200,
    "atk": 25,
    "def": 22,
    "spd": 10,
    "xp": 250,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Ancient Guardians",
    "spawnFloors": [
      25,
      28
    ]
  },
  "sphinx": {
    "id": "sphinx",
    "name": "Sphinx",
    "glyph": "X",
    "color": "#f1c40f",
    "tier": 7,
    "hp": 150,
    "atk": 30,
    "def": 15,
    "spd": 10,
    "xp": 280,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Sphinxes",
    "spawnFloors": [
      25,
      28
    ]
  },
  "lich": {
    "id": "lich",
    "name": "Lich",
    "glyph": "L",
    "color": "#d35ded",
    "tier": 7,
    "hp": 120,
    "atk": 35,
    "def": 12,
    "spd": 10,
    "xp": 300,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Liches",
    "spawnFloors": [
      25,
      28
    ]
  },
  "gargoyle": {
    "id": "gargoyle",
    "name": "Gargoyle",
    "glyph": "G",
    "color": "#f1c40f",
    "tier": 7,
    "hp": 130,
    "atk": 24,
    "def": 18,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Gargoyles",
    "spawnFloors": [
      25,
      28
    ]
  },
  "mummyLord": {
    "id": "mummyLord",
    "name": "Mummy Lord",
    "glyph": "M",
    "color": "#d35ded",
    "tier": 7,
    "hp": 160,
    "atk": 28,
    "def": 16,
    "spd": 10,
    "xp": 260,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "pluralName": "Mummy Lords",
    "spawnFloors": [
      25,
      28
    ]
  },
  "cursedStatue": {
    "id": "cursedStatue",
    "name": "Cursed Statue",
    "glyph": "S",
    "color": "#f1c40f",
    "tier": 7,
    "hp": 220,
    "atk": 20,
    "def": 25,
    "spd": 10,
    "xp": 240,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "pluralName": "Cursed Statues",
    "spawnFloors": [
      25,
      28
    ]
  },
  "shadowAssassin": {
    "id": "shadowAssassin",
    "name": "Shadow Assassin",
    "glyph": "a",
    "color": "#5d6d7e",
    "tier": 7,
    "hp": 80,
    "atk": 40,
    "def": 8,
    "spd": 10,
    "xp": 280,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "pluralName": "Shadow Assassins",
    "spawnFloors": [
      25,
      28
    ]
  },
  "demon": {
    "id": "demon",
    "name": "Demon",
    "glyph": "D",
    "color": "#e74c3c",
    "tier": 8,
    "hp": 140,
    "atk": 30,
    "def": 15,
    "spd": 10,
    "xp": 280,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "pluralName": "Demons",
    "spawnFloors": [
      29,
      30
    ]
  },
  "demonLord": {
    "id": "demonLord",
    "name": "Demon Lord",
    "glyph": "&",
    "color": "#e74c3c",
    "tier": 8,
    "hp": 200,
    "atk": 35,
    "def": 20,
    "spd": 10,
    "xp": 350,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "pluralName": "Demon Lords",
    "spawnFloors": [
      29,
      30
    ]
  },
  "succubus": {
    "id": "succubus",
    "name": "Succubus",
    "glyph": "s",
    "color": "#d35ded",
    "tier": 8,
    "hp": 100,
    "atk": 28,
    "def": 10,
    "spd": 10,
    "xp": 250,
    "flags": {},
    "statusInflicts": [],
    "faction": "demonCult",
    "pluralName": "Succubi",
    "spawnFloors": [
      29,
      30
    ]
  },
  "balrog": {
    "id": "balrog",
    "name": "Balrog",
    "glyph": "B",
    "color": "#e74c3c",
    "tier": 8,
    "hp": 250,
    "atk": 40,
    "def": 25,
    "spd": 10,
    "xp": 400,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "pluralName": "Balrogs",
    "spawnFloors": [
      29,
      30
    ]
  },
  "pitFiend": {
    "id": "pitFiend",
    "name": "Pit Fiend",
    "glyph": "P",
    "color": "#e74c3c",
    "tier": 8,
    "hp": 180,
    "atk": 38,
    "def": 18,
    "spd": 10,
    "xp": 380,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "pluralName": "Pit Fiends",
    "spawnFloors": [
      29,
      30
    ]
  },
  "shadowDemon": {
    "id": "shadowDemon",
    "name": "Shadow Demon",
    "glyph": "S",
    "color": "#d35ded",
    "tier": 8,
    "hp": 120,
    "atk": 35,
    "def": 12,
    "spd": 10,
    "xp": 320,
    "flags": {},
    "statusInflicts": [],
    "faction": "demonCult",
    "pluralName": "Shadow Demons",
    "spawnFloors": [
      29,
      30
    ]
  },
  "abyssalHorror": {
    "id": "abyssalHorror",
    "name": "Abyssal Horror",
    "glyph": "H",
    "color": "#922b21",
    "tier": 8,
    "hp": 300,
    "atk": 45,
    "def": 22,
    "spd": 10,
    "xp": 450,
    "flags": {},
    "statusInflicts": [],
    "faction": "demonCult",
    "pluralName": "Abyssal Horrors",
    "spawnFloors": [
      29,
      30
    ]
  },
  "doomGuard": {
    "id": "doomGuard",
    "name": "Doom Guard",
    "glyph": "D",
    "color": "#e74c3c",
    "tier": 8,
    "hp": 220,
    "atk": 42,
    "def": 20,
    "spd": 10,
    "xp": 420,
    "flags": {},
    "statusInflicts": [],
    "faction": "demonCult",
    "pluralName": "Doom Guards",
    "spawnFloors": [
      29,
      30
    ]
  },
  "bossGoblinKing": {
    "id": "bossGoblinKing",
    "name": "GOBLIN KING",
    "glyph": "K",
    "color": "#2ecc71",
    "tier": 1,
    "hp": 200,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 500,
    "flags": {
      "boss": true
    },
    "statusInflicts": [],
    "faction": "orcHorde",
    "spawnFloors": null,
    "bossFloor": 5
  },
  "bossOrcWarlord": {
    "id": "bossOrcWarlord",
    "name": "ORC WARLORD",
    "glyph": "W",
    "color": "#1e8449",
    "tier": 2,
    "hp": 400,
    "atk": 30,
    "def": 15,
    "spd": 10,
    "xp": 1000,
    "flags": {
      "boss": true
    },
    "statusInflicts": [
      "bleed"
    ],
    "faction": "orcHorde",
    "spawnFloors": null,
    "bossFloor": 10
  },
  "bossVampireLord": {
    "id": "bossVampireLord",
    "name": "VAMPIRE LORD",
    "glyph": "V",
    "color": "#922b21",
    "tier": 3,
    "hp": 600,
    "atk": 40,
    "def": 20,
    "spd": 10,
    "xp": 2000,
    "flags": {
      "undead": true,
      "boss": true
    },
    "statusInflicts": [
      "bleed"
    ],
    "faction": "undeadLegion",
    "spawnFloors": null,
    "bossFloor": 15
  },
  "bossForestGuardian": {
    "id": "bossForestGuardian",
    "name": "FOREST GUARDIAN",
    "glyph": "G",
    "color": "#2ecc71",
    "tier": 4,
    "hp": 800,
    "atk": 45,
    "def": 25,
    "spd": 10,
    "xp": 3000,
    "flags": {
      "boss": true
    },
    "statusInflicts": [],
    "faction": "druids",
    "spawnFloors": null,
    "bossFloor": 20
  },
  "bossIceDragon": {
    "id": "bossIceDragon",
    "name": "ICE DRAGON",
    "glyph": "D",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 1200,
    "atk": 55,
    "def": 30,
    "spd": 10,
    "xp": 5000,
    "flags": {
      "boss": true
    },
    "statusInflicts": [
      "freeze"
    ],
    "faction": "dragonFlight",
    "spawnFloors": null,
    "bossFloor": 25
  },
  "bossDemonKing": {
    "id": "bossDemonKing",
    "name": "DEMON KING",
    "glyph": "&",
    "color": "#e74c3c",
    "tier": 8,
    "hp": 2000,
    "atk": 70,
    "def": 40,
    "spd": 10,
    "xp": 10000,
    "flags": {
      "boss": true
    },
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "spawnFloors": null,
    "bossFloor": 30
  },
  "goblinChampion": {
    "id": "goblinChampion",
    "name": "Goblin Champion",
    "glyph": "K",
    "color": "#2ecc71",
    "tier": 1,
    "hp": 150,
    "atk": 18,
    "def": 8,
    "spd": 10,
    "xp": 300,
    "flags": {
      "miniBoss": true
    },
    "statusInflicts": [],
    "faction": "orcHorde",
    "spawnFloors": [
      5,
      8
    ]
  },
  "orcBerserker": {
    "id": "orcBerserker",
    "name": "Orc Berserker",
    "glyph": "W",
    "color": "#1e8449",
    "tier": 2,
    "hp": 250,
    "atk": 28,
    "def": 12,
    "spd": 10,
    "xp": 600,
    "flags": {
      "miniBoss": true
    },
    "statusInflicts": [],
    "faction": "orcHorde",
    "spawnFloors": [
      9,
      12
    ]
  },
  "vampireElite": {
    "id": "vampireElite",
    "name": "Vampire Elite",
    "glyph": "V",
    "color": "#922b21",
    "tier": 3,
    "hp": 350,
    "atk": 35,
    "def": 18,
    "spd": 10,
    "xp": 1200,
    "flags": {
      "undead": true,
      "miniBoss": true
    },
    "statusInflicts": [],
    "faction": "undeadLegion",
    "spawnFloors": [
      13,
      16
    ]
  },
  "ancientWyrm": {
    "id": "ancientWyrm",
    "name": "Ancient Wyrm",
    "glyph": "G",
    "color": "#2ecc71",
    "tier": 4,
    "hp": 500,
    "atk": 40,
    "def": 22,
    "spd": 10,
    "xp": 1800,
    "flags": {
      "miniBoss": true
    },
    "statusInflicts": [],
    "faction": "dragonFlight",
    "spawnFloors": [
      17,
      20
    ]
  },
  "frostLord": {
    "id": "frostLord",
    "name": "Frost Lord",
    "glyph": "D",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 700,
    "atk": 48,
    "def": 28,
    "spd": 10,
    "xp": 3500,
    "flags": {
      "miniBoss": true
    },
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "infernalLord": {
    "id": "infernalLord",
    "name": "Infernal Lord",
    "glyph": "&",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 900,
    "atk": 55,
    "def": 32,
    "spd": 10,
    "xp": 5500,
    "flags": {
      "miniBoss": true
    },
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "spawnFloors": [
      25,
      28
    ]
  },
  "highElfMage": {
    "id": "highElfMage",
    "name": "High Elf Mage",
    "glyph": "e",
    "color": "#1abc9c",
    "tier": 2,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "ranged": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      5,
      8
    ]
  },
  "woodElfRanger": {
    "id": "woodElfRanger",
    "name": "Wood Elf Ranger",
    "glyph": "e",
    "color": "#2ecc71",
    "tier": 2,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "ranged": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      5,
      8
    ]
  },
  "darkElfAssassin": {
    "id": "darkElfAssassin",
    "name": "Dark Elf Assassin",
    "glyph": "e",
    "color": "#76448a",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "bloodElfWarlock": {
    "id": "bloodElfWarlock",
    "name": "Blood Elf Warlock",
    "glyph": "e",
    "color": "#922b21",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "elfBladeDancer": {
    "id": "elfBladeDancer",
    "name": "Elf Blade Dancer",
    "glyph": "e",
    "color": "#ecf0f1",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "elvenSentinel": {
    "id": "elvenSentinel",
    "name": "Elven Sentinel",
    "glyph": "E",
    "color": "#f1c40f",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "dwarfWarrior": {
    "id": "dwarfWarrior",
    "name": "Dwarf Warrior",
    "glyph": "d",
    "color": "#b7950b",
    "tier": 2,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      5,
      8
    ]
  },
  "dwarfRunesmith": {
    "id": "dwarfRunesmith",
    "name": "Dwarf Runesmith",
    "glyph": "d",
    "color": "#1abc9c",
    "tier": 2,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      5,
      8
    ]
  },
  "dwarfBerserker": {
    "id": "dwarfBerserker",
    "name": "Dwarf Berserker",
    "glyph": "d",
    "color": "#e74c3c",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "ironGolem": {
    "id": "ironGolem",
    "name": "Iron Golem",
    "glyph": "I",
    "color": "#95a5a6",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "dwarfKing": {
    "id": "dwarfKing",
    "name": "DWARF KING",
    "glyph": "K",
    "color": "#f1c40f",
    "tier": 8,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      29,
      30
    ],
    "bossFloor": 12
  },
  "wyrmling": {
    "id": "wyrmling",
    "name": "Wyrmling",
    "glyph": "d",
    "color": "#2ecc71",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "youngDragon": {
    "id": "youngDragon",
    "name": "Young Dragon",
    "glyph": "D",
    "color": "#f1c40f",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "adultDragon": {
    "id": "adultDragon",
    "name": "Adult Dragon",
    "glyph": "D",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "ancientDragon": {
    "id": "ancientDragon",
    "name": "ANCIENT DRAGON",
    "glyph": "D",
    "color": "#d35ded",
    "tier": 8,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "dragonFlight",
    "spawnFloors": [
      29,
      30
    ],
    "bossFloor": 22
  },
  "dragonPriest": {
    "id": "dragonPriest",
    "name": "Dragon Priest",
    "glyph": "p",
    "color": "#922b21",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "dragonborn": {
    "id": "dragonborn",
    "name": "Dragonborn",
    "glyph": "D",
    "color": "#b7950b",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "impSwarm": {
    "id": "impSwarm",
    "name": "Imp Swarm",
    "glyph": "i",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "demonHound": {
    "id": "demonHound",
    "name": "Demon Hound",
    "glyph": "h",
    "color": "#922b21",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "chainDevil": {
    "id": "chainDevil",
    "name": "Chain Devil",
    "glyph": "C",
    "color": "#5d6d7e",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "boneDemon": {
    "id": "boneDemon",
    "name": "Bone Demon",
    "glyph": "B",
    "color": "#ecf0f1",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "plagueDemon": {
    "id": "plagueDemon",
    "name": "Plague Demon",
    "glyph": "P",
    "color": "#1e8449",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "dreamDemon": {
    "id": "dreamDemon",
    "name": "Dream Demon",
    "glyph": "D",
    "color": "#d35ded",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "archDemon": {
    "id": "archDemon",
    "name": "ARCHDEMON",
    "glyph": "&",
    "color": "#e74c3c",
    "tier": 8,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": "demonCult",
    "spawnFloors": [
      29,
      30
    ],
    "bossFloor": 28
  },
  "skeletonWarrior": {
    "id": "skeletonWarrior",
    "name": "Skeleton Warrior",
    "glyph": "k",
    "color": "#ecf0f1",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "skeletonArcher": {
    "id": "skeletonArcher",
    "name": "Skeleton Archer",
    "glyph": "k",
    "color": "#ecf0f1",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true,
      "ranged": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "skeletonMage": {
    "id": "skeletonMage",
    "name": "Skeleton Mage",
    "glyph": "k",
    "color": "#ecf0f1",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true,
      "ranged": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "zombieBrute": {
    "id": "zombieBrute",
    "name": "Zombie Brute",
    "glyph": "Z",
    "color": "#1e8449",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "zombieSpitter": {
    "id": "zombieSpitter",
    "name": "Zombie Spitter",
    "glyph": "z",
    "color": "#1e8449",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true,
      "ranged": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "vampireSpawn": {
    "id": "vampireSpawn",
    "name": "Vampire Spawn",
    "glyph": "v",
    "color": "#922b21",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "vampireNoble": {
    "id": "vampireNoble",
    "name": "Vampire Noble",
    "glyph": "V",
    "color": "#922b21",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "wight": {
    "id": "wight",
    "name": "Wight",
    "glyph": "W",
    "color": "#148f77",
    "tier": 3,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      9,
      12
    ]
  },
  "specter": {
    "id": "specter",
    "name": "Specter",
    "glyph": "S",
    "color": "#95a5a6",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "poltergeist": {
    "id": "poltergeist",
    "name": "Poltergeist",
    "glyph": "p",
    "color": "#95a5a6",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "revenantKnight": {
    "id": "revenantKnight",
    "name": "Revenant Knight",
    "glyph": "R",
    "color": "#76448a",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {
      "undead": true
    },
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "alphaWolf": {
    "id": "alphaWolf",
    "name": "Alpha Wolf",
    "glyph": "W",
    "color": "#95a5a6",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "direBoar": {
    "id": "direBoar",
    "name": "Dire Boar",
    "glyph": "B",
    "color": "#b7950b",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "giantEagle": {
    "id": "giantEagle",
    "name": "Giant Eagle",
    "glyph": "E",
    "color": "#f1c40f",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "manticore": {
    "id": "manticore",
    "name": "Manticore",
    "glyph": "M",
    "color": "#b7950b",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "chimera": {
    "id": "chimera",
    "name": "Chimera",
    "glyph": "C",
    "color": "#922b21",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "bleed"
    ],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "griffon": {
    "id": "griffon",
    "name": "Griffon",
    "glyph": "G",
    "color": "#f1c40f",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "hydra": {
    "id": "hydra",
    "name": "Hydra",
    "glyph": "H",
    "color": "#1e8449",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "basilisk": {
    "id": "basilisk",
    "name": "Basilisk",
    "glyph": "B",
    "color": "#b7950b",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "cockatrice": {
    "id": "cockatrice",
    "name": "Cockatrice",
    "glyph": "c",
    "color": "#f1c40f",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "stormElemental": {
    "id": "stormElemental",
    "name": "Storm Elemental",
    "glyph": "E",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "magmaElemental": {
    "id": "magmaElemental",
    "name": "Magma Elemental",
    "glyph": "E",
    "color": "#e74c3c",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "burn"
    ],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "mudElemental": {
    "id": "mudElemental",
    "name": "Mud Elemental",
    "glyph": "E",
    "color": "#b7950b",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "lightElemental": {
    "id": "lightElemental",
    "name": "Light Elemental",
    "glyph": "E",
    "color": "#ecf0f1",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "darkElemental": {
    "id": "darkElemental",
    "name": "Dark Elemental",
    "glyph": "E",
    "color": "#5d6d7e",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "voidElemental": {
    "id": "voidElemental",
    "name": "Void Elemental",
    "glyph": "V",
    "color": "#d35ded",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "fallenSeraph": {
    "id": "fallenSeraph",
    "name": "Fallen Seraph",
    "glyph": "A",
    "color": "#76448a",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "corruptedCherub": {
    "id": "corruptedCherub",
    "name": "Corrupted Cherub",
    "glyph": "a",
    "color": "#922b21",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "angelicGuard": {
    "id": "angelicGuard",
    "name": "Angelic Guard",
    "glyph": "A",
    "color": "#ecf0f1",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "pixie": {
    "id": "pixie",
    "name": "Pixie",
    "glyph": "p",
    "color": "#d35ded",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "satyr": {
    "id": "satyr",
    "name": "Satyr",
    "glyph": "s",
    "color": "#b7950b",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "dryad": {
    "id": "dryad",
    "name": "Dryad",
    "glyph": "d",
    "color": "#2ecc71",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "treant": {
    "id": "treant",
    "name": "Treant",
    "glyph": "T",
    "color": "#1e8449",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "unicorn": {
    "id": "unicorn",
    "name": "Unicorn",
    "glyph": "U",
    "color": "#ecf0f1",
    "tier": 4,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      13,
      16
    ]
  },
  "feyDragon": {
    "id": "feyDragon",
    "name": "Fey Dragon",
    "glyph": "F",
    "color": "#1abc9c",
    "tier": 5,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "freeze"
    ],
    "faction": null,
    "spawnFloors": [
      17,
      20
    ]
  },
  "archfey": {
    "id": "archfey",
    "name": "ARCHFEY",
    "glyph": "A",
    "color": "#d35ded",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ],
    "bossFloor": 18
  },
  "clockworkSoldier": {
    "id": "clockworkSoldier",
    "name": "Clockwork Soldier",
    "glyph": "C",
    "color": "#b7950b",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "animatedArmor": {
    "id": "animatedArmor",
    "name": "Animated Armor",
    "glyph": "A",
    "color": "#95a5a6",
    "tier": 6,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      21,
      24
    ]
  },
  "shieldGuardian": {
    "id": "shieldGuardian",
    "name": "Shield Guardian",
    "glyph": "S",
    "color": "#f1c40f",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "colossus": {
    "id": "colossus",
    "name": "COLOSSUS",
    "glyph": "C",
    "color": "#5d6d7e",
    "tier": 8,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      29,
      30
    ],
    "bossFloor": 27
  },
  "beholder": {
    "id": "beholder",
    "name": "Beholder",
    "glyph": "@",
    "color": "#d35ded",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "mindFlayer": {
    "id": "mindFlayer",
    "name": "Mind Flayer",
    "glyph": "M",
    "color": "#76448a",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "aboleth": {
    "id": "aboleth",
    "name": "Aboleth",
    "glyph": "A",
    "color": "#148f77",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "gibbering": {
    "id": "gibbering",
    "name": "Gibbering Mouther",
    "glyph": "g",
    "color": "#b7950b",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  },
  "otyugh": {
    "id": "otyugh",
    "name": "Otyugh",
    "glyph": "O",
    "color": "#1e8449",
    "tier": 7,
    "hp": 100,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 200,
    "flags": {},
    "statusInflicts": [
      "poison"
    ],
    "faction": null,
    "spawnFloors": [
      25,
      28
    ]
  }
};

SC.DATA.bosses = {
  "bossGoblinKing": {
    "id": "bossGoblinKing",
    "name": "GOBLIN KING",
    "type": "boss",
    "floor": 5,
    "spawnFloors": null,
    "tier": 1,
    "glyph": "K",
    "color": "#2ecc71",
    "hp": 200,
    "atk": 20,
    "def": 10,
    "spd": 10,
    "xp": 500,
    "flags": {
      "boss": true
    },
    "specialAttacks": [],
    "faction": "orcHorde"
  },
  "bossOrcWarlord": {
    "id": "bossOrcWarlord",
    "name": "ORC WARLORD",
    "type": "boss",
    "floor": 10,
    "spawnFloors": null,
    "tier": 2,
    "glyph": "W",
    "color": "#1e8449",
    "hp": 400,
    "atk": 30,
    "def": 15,
    "spd": 10,
    "xp": 1000,
    "flags": {
      "boss": true
    },
    "specialAttacks": [
      "bleed"
    ],
    "faction": "orcHorde"
  },
  "bossVampireLord": {
    "id": "bossVampireLord",
    "name": "VAMPIRE LORD",
    "type": "boss",
    "floor": 15,
    "spawnFloors": null,
    "tier": 3,
    "glyph": "V",
    "color": "#922b21",
    "hp": 600,
    "atk": 40,
    "def": 20,
    "spd": 10,
    "xp": 2000,
    "flags": {
      "undead": true,
      "boss": true
    },
    "specialAttacks": [
      "bleed"
    ],
    "faction": "undeadLegion"
  },
  "bossForestGuardian": {
    "id": "bossForestGuardian",
    "name": "FOREST GUARDIAN",
    "type": "boss",
    "floor": 20,
    "spawnFloors": null,
    "tier": 4,
    "glyph": "G",
    "color": "#2ecc71",
    "hp": 800,
    "atk": 45,
    "def": 25,
    "spd": 10,
    "xp": 3000,
    "flags": {
      "boss": true
    },
    "specialAttacks": [],
    "faction": "druids"
  },
  "bossIceDragon": {
    "id": "bossIceDragon",
    "name": "ICE DRAGON",
    "type": "boss",
    "floor": 25,
    "spawnFloors": null,
    "tier": 5,
    "glyph": "D",
    "color": "#1abc9c",
    "hp": 1200,
    "atk": 55,
    "def": 30,
    "spd": 10,
    "xp": 5000,
    "flags": {
      "boss": true
    },
    "specialAttacks": [
      "freeze"
    ],
    "faction": "dragonFlight"
  },
  "bossDemonKing": {
    "id": "bossDemonKing",
    "name": "DEMON KING",
    "type": "boss",
    "floor": 30,
    "spawnFloors": null,
    "tier": 8,
    "glyph": "&",
    "color": "#e74c3c",
    "hp": 2000,
    "atk": 70,
    "def": 40,
    "spd": 10,
    "xp": 10000,
    "flags": {
      "boss": true
    },
    "specialAttacks": [
      "burn"
    ],
    "faction": "demonCult"
  },
  "goblinChampion": {
    "id": "goblinChampion",
    "name": "Goblin Champion",
    "type": "miniBoss",
    "floor": null,
    "spawnFloors": [
      5,
      8
    ],
    "tier": 1,
    "glyph": "K",
    "color": "#2ecc71",
    "hp": 150,
    "atk": 18,
    "def": 8,
    "spd": 10,
    "xp": 300,
    "flags": {
      "miniBoss": true
    },
    "specialAttacks": [],
    "faction": "orcHorde"
  },
  "orcBerserker": {
    "id": "orcBerserker",
    "name": "Orc Berserker",
    "type": "miniBoss",
    "floor": null,
    "spawnFloors": [
      9,
      12
    ],
    "tier": 2,
    "glyph": "W",
    "color": "#1e8449",
    "hp": 250,
    "atk": 28,
    "def": 12,
    "spd": 10,
    "xp": 600,
    "flags": {
      "miniBoss": true
    },
    "specialAttacks": [],
    "faction": "orcHorde"
  },
  "vampireElite": {
    "id": "vampireElite",
    "name": "Vampire Elite",
    "type": "miniBoss",
    "floor": null,
    "spawnFloors": [
      13,
      16
    ],
    "tier": 3,
    "glyph": "V",
    "color": "#922b21",
    "hp": 350,
    "atk": 35,
    "def": 18,
    "spd": 10,
    "xp": 1200,
    "flags": {
      "undead": true,
      "miniBoss": true
    },
    "specialAttacks": [],
    "faction": "undeadLegion"
  },
  "ancientWyrm": {
    "id": "ancientWyrm",
    "name": "Ancient Wyrm",
    "type": "miniBoss",
    "floor": null,
    "spawnFloors": [
      17,
      20
    ],
    "tier": 4,
    "glyph": "G",
    "color": "#2ecc71",
    "hp": 500,
    "atk": 40,
    "def": 22,
    "spd": 10,
    "xp": 1800,
    "flags": {
      "miniBoss": true
    },
    "specialAttacks": [],
    "faction": "dragonFlight"
  },
  "frostLord": {
    "id": "frostLord",
    "name": "Frost Lord",
    "type": "miniBoss",
    "floor": null,
    "spawnFloors": [
      21,
      24
    ],
    "tier": 5,
    "glyph": "D",
    "color": "#1abc9c",
    "hp": 700,
    "atk": 48,
    "def": 28,
    "spd": 10,
    "xp": 3500,
    "flags": {
      "miniBoss": true
    },
    "specialAttacks": [
      "freeze"
    ],
    "faction": null
  },
  "infernalLord": {
    "id": "infernalLord",
    "name": "Infernal Lord",
    "type": "miniBoss",
    "floor": null,
    "spawnFloors": [
      25,
      28
    ],
    "tier": 6,
    "glyph": "&",
    "color": "#e74c3c",
    "hp": 900,
    "atk": 55,
    "def": 32,
    "spd": 10,
    "xp": 5500,
    "flags": {
      "miniBoss": true
    },
    "specialAttacks": [
      "burn"
    ],
    "faction": "demonCult"
  }
};

SC.DATA.enemies_notes = [
  "Source: src/main.rs EnemyKind enum + impl (148 variants), cross-checked against core/src/entities.rs (older 72-variant catalog; stats identical where both define them).",
  "hp/atk/def/xp are BASE values; the Rust engine scales them at spawn by (1.0 + dungeonLevel * 0.1) in Enemy::new (truncated to integer).",
  "The 69 expanded-roster enemies (elf, dwarf, dragon, expanded demon/undead, beast, elemental, celestial, fey, construct, aberration) all hit the Rust base_stats() fallback arm `_ => (100, 20, 10, 200)`; those values are ported verbatim (hp 100, atk 20, def 10, xp 200).",
  "spd: the Rust source defines no per-enemy speed (every actor acts once per turn); spd is set to 10 for all enemies to match the player-class baseline speed (Warrior/Mage base speed = 10 in core/src/classes.rs).",
  "flags.ranged: no ranged-attack mechanic exists in the Rust source; ranged=true was assigned as a sensible archetype pick for skeletonArcher, zombieSpitter, skeletonMage, highElfMage, woodElfRanger only.",
  "tier: derived from the enum section comments (tiers 1-8 = floors 1-4, 5-8, 9-12, 13-16, 17-20, 21-24, 25-28, 29-30) and, for expanded-roster enemies, the first for_level() spawn pool they appear in. Bosses and mini-bosses use the README.md area labels (Goblin King/Champion tier 1 ... Demon King tier 8 / Infernal Lord tier 6).",
  "spawnFloors: the [min,max] floor range of the for_level() pool the enemy belongs to in src/main.rs; note the code spawns each mini-boss one area later than its README tier label (e.g. Goblin Champion spawns on floors 5-8). The 6 main bosses never spawn from pools (spawnFloors null).",
  "bossFloor: from boss_for_level() in src/main.rs. Besides the 6 classic bosses it also assigns dwarfKing (floor 12), archfey (18), ancientDragon (22), colossus (27) and archDemon (28) as boss encounters; Rust is_boss() does NOT include these five, so their flags carry no boss/miniBoss - use bossFloor to detect them.",
  "statusInflicts: from can_poison()/can_burn()/can_freeze()/can_bleed() in src/main.rs (extended lists). Status tick damage to the player per turn in the source: poison 2, burn 3, bleed 1; to enemies: poison 3, burn 5, bleed 2.",
  "faction: from EnemyKind::faction() (orcHorde, undeadLegion, demonCult, dragonFlight, druids; null = no faction).",
  "pluralName: only present where plural_name() has an explicit arm in src/main.rs (the generic fallback \"Enemies\" is omitted)."
];

SC.DATA.bosses_notes = [
  "The 6 bosses (type \"boss\") + 6 mini-bosses (type \"miniBoss\") from src/main.rs is_boss(). floor is the boss_for_level() floor; mini-bosses have no boss floor (floor null) - they spawn in regular for_level() pools (spawnFloors).",
  "The Rust source has no boss phase system and no scripted special attacks; specialAttacks lists the status effects the boss can inflict on hit (from can_burn/can_freeze/can_bleed/can_poison). Empty array = plain melee.",
  "boss_for_level() also places five expanded-roster enemies as boss encounters: dwarfKing (floor 12), archfey (18), ancientDragon (22), colossus (27), archDemon (28). They are kept in SC.DATA.enemies with a bossFloor field since Rust is_boss() excludes them.",
  "Boss stats are base values and are scaled by (1.0 + dungeonLevel * 0.1) at spawn, exactly like regular enemies."
];

'use strict';
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});
SC.DATA = SC.DATA || {};

// ============================================================================
// COMPANION DATA — ported from src/companion.rs (create_all_companions et al.)
// Stats are the exact values Companion::new() computes at each companion's
// starting level:
//   hp    = classBaseHp   + speciesHpBonus   + level * 8
//   mana  = classBaseMana + speciesManaBonus + level * 5
//   atk   = classBaseAtk  + speciesAtkBonus  + level * 2
//   def   = classBaseDef  + speciesDefBonus  + level * 1
//   spd   = classBaseSpd  + speciesSpdBonus
// (class base stats from CharacterClass::base_stats in src/main.rs,
//  species bonuses from CompanionSpecies::stat_bonuses in src/companion.rs)
// ============================================================================

SC.DATA.companions = [
  {
    "id": 1,
    "name": "Thorin",
    "title": "the Unbreakable",
    "kind": "Dwarf",
    "class": "Warrior",
    "level": 5,
    "personality": "Brave",
    "description": "A legendary dwarven warrior who lost his clan to a dragon. Seeks vengeance and glory.",
    "stats": { "hp": 110, "atk": 23, "def": 25, "spd": 5, "mana": 35 },
    "abilities": ["Cleave", "Taunt", "Shield Ally"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Hail, adventurer! My axe thirsts for battle!",
      "battleCry": "FOR KHAZAD-DUM!",
      "victory": "Another victory for the dwarves!",
      "death": "The mountain... calls me home..."
    },
    "personalQuest": {
      "type": "Revenge",
      "targetName": "Dragon",
      "target": 1,
      "description": "Help Thorin slay the dragon that destroyed his clan"
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 2,
    "name": "Valeria",
    "title": "the Lioness",
    "kind": "Human",
    "class": "Warrior",
    "level": 6,
    "personality": "Protective",
    "description": "Former captain of the Royal Guard, disgraced by a false accusation. Fights to restore her honor.",
    "stats": { "hp": 108, "atk": 25, "def": 16, "spd": 15, "mana": 45 },
    "abilities": ["Shield Ally", "Battle Cry", "Execute"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "My sword is yours, if your cause is just.",
      "battleCry": "By my honor!",
      "victory": "Justice prevails!",
      "death": "My... honor... is restored..."
    },
    "personalQuest": {
      "type": "Redemption",
      "goodDeedsNeeded": 20,
      "target": 20,
      "description": "Help Valeria restore her honor through noble deeds"
    },
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 3,
    "name": "Grimjaw",
    "title": "the Scarred",
    "kind": "Orc",
    "class": "Warrior",
    "level": 7,
    "personality": "Vengeful",
    "description": "An orc chieftain's son, scarred by humans. Seeks to prove orcs can be honorable.",
    "stats": { "hp": 121, "atk": 37, "def": 17, "spd": 10, "mana": 40 },
    "abilities": ["Flurry", "Execute", "Battle Cry"],
    "cost": 350,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "You... not afraid of orc? Good. Grimjaw respect that.",
      "battleCry": "BLOOD FOR HONOR!",
      "victory": "Grimjaw strong! Grimjaw win!",
      "death": "Grimjaw... die... standing..."
    },
    "personalQuest": {
      "type": "ProveWorth",
      "killsNeeded": 50,
      "target": 50,
      "description": "Help Grimjaw prove his worth through combat"
    },
    "howObtained": "Random recruitment offer on dungeon levels 4-12 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 4,
    "name": "Seraphina",
    "title": "the Starweaver",
    "kind": "Elf",
    "class": "Mage",
    "level": 6,
    "personality": "Wise",
    "description": "An elven archmage who has lived for 500 years. Seeks a forbidden spell that could save her dying homeland.",
    "stats": { "hp": 78, "atk": 20, "def": 8, "spd": 20, "mana": 95 },
    "abilities": ["Fireball", "Ice Storm", "Time Stop"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "The stars have foretold our meeting, young one.",
      "battleCry": "By the ancient powers!",
      "victory": "As the prophecy foretold.",
      "death": "My magic... returns to the stars..."
    },
    "romanceDialogue": [
      "In five centuries, I have never felt... this.",
      "Our souls dance like twin stars.",
      "Time means nothing when I am with you."
    ],
    "personalQuest": {
      "type": "AncientArtifact",
      "itemName": "Tome of Infinite Knowledge",
      "target": 1,
      "description": "Help Seraphina find the Tome of Infinite Knowledge"
    },
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 5,
    "name": "Mordecai",
    "title": "the Flame",
    "kind": "Tiefling",
    "class": "Mage",
    "level": 5,
    "personality": "Ambitious",
    "description": "A tiefling pyromancer shunned for his demonic heritage. Craves power to show the world his worth.",
    "stats": { "hp": 75, "atk": 23, "def": 12, "spd": 15, "mana": 90 },
    "abilities": ["Fireball", "Dragon Breath", "Unholy Might"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Do not fear the flames within me... fear what I do with them.",
      "battleCry": "BURN IN HELLFIRE!",
      "victory": "See what I can do? Imagine what's next!",
      "death": "The flames... consume me at last..."
    },
    "personalQuest": {
      "type": "MasterSkill",
      "skillUses": 100,
      "target": 100,
      "description": "Help Mordecai master his fire magic"
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 6,
    "name": "Whisper",
    "title": "the Void",
    "kind": "Spirit",
    "class": "Mage",
    "level": 8,
    "personality": "Mysterious",
    "description": "A spirit trapped between worlds. Seeks the anchor that binds it to this realm.",
    "stats": { "hp": 74, "atk": 24, "def": 5, "spd": 25, "mana": 120 },
    "abilities": ["Teleport", "Time Stop", "Vanish"],
    "cost": 400,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I am... here. And not here. Can you... hear me?",
      "battleCry": "From the void, I strike!",
      "victory": "The living... are so fragile.",
      "death": "At last... peace..."
    },
    "personalQuest": {
      "type": "LostFamily",
      "dungeonLevel": 25,
      "target": 1,
      "description": "Help Whisper find its anchor in the depths"
    },
    "howObtained": "Random recruitment offer on dungeon levels 5-13 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 7,
    "name": "Shadow",
    "title": "the Silent",
    "kind": "Half-Elf",
    "class": "Rogue",
    "level": 5,
    "personality": "Stoic",
    "description": "An assassin who abandoned her guild after they ordered her to kill a child. Now hunted by her former allies.",
    "stats": { "hp": 80, "atk": 21, "def": 13, "spd": 22, "mana": 55 },
    "abilities": ["Backstab", "Vanish", "Poison"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I work in silence. Don't ask questions.",
      "battleCry": "You won't see me coming.",
      "victory": "Target eliminated.",
      "death": "The shadows... welcome me..."
    },
    "romanceDialogue": [
      "I've never let anyone get close. You're... different.",
      "For you, I would come out of the shadows.",
      "I was death. You gave me life."
    ],
    "personalQuest": {
      "type": "Revenge",
      "targetName": "Guild Master",
      "target": 1,
      "description": "Help Shadow defeat her former guild"
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 8,
    "name": "Felix",
    "title": "the Lucky",
    "kind": "Goblin",
    "class": "Rogue",
    "level": 4,
    "personality": "Cheerful",
    "description": "A goblin who was kicked out of his tribe for being 'too nice'. Believes luck will guide him to fortune.",
    "stats": { "hp": 67, "atk": 19, "def": 7, "spd": 30, "mana": 45 },
    "abilities": ["Steal", "Treasure Hunter", "Ambush"],
    "cost": 200,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Hey-hey! Felix is best friend you ever have! Felix promise!",
      "battleCry": "Shiny things for Felix!",
      "victory": "Felix win! Felix ALWAYS win! Hehehehe!",
      "death": "Felix... not so lucky after all..."
    },
    "personalQuest": {
      "type": "AncientArtifact",
      "itemName": "Lucky Coin",
      "target": 1,
      "description": "Help Felix find the legendary Lucky Coin"
    },
    "howObtained": "Random recruitment offer on dungeon levels 1-9 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 9,
    "name": "Raven",
    "title": "the Blade Dancer",
    "kind": "Human",
    "class": "Rogue",
    "level": 6,
    "personality": "Romantic",
    "description": "A notorious jewel thief who steals only from the corrupt. Leaves a black feather at every heist.",
    "stats": { "hp": 93, "atk": 23, "def": 14, "spd": 20, "mana": 55 },
    "abilities": ["Disarm", "Flurry", "Vanish"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Care to dance with danger, darling?",
      "battleCry": "Like poetry in motion!",
      "victory": "Another masterpiece complete!",
      "death": "One last... dance..."
    },
    "romanceDialogue": [
      "I steal many things, but you've stolen something from me.",
      "For you, I'd give up all the jewels in the world.",
      "Let's write our story together, my love."
    ],
    "personalQuest": {
      "type": "AncientArtifact",
      "itemName": "Heart of the Ocean",
      "target": 1,
      "description": "Help Raven steal the legendary Heart of the Ocean"
    },
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 10,
    "name": "Sir Aldric",
    "title": "the Dawn",
    "kind": "Human",
    "class": "Paladin",
    "level": 6,
    "personality": "Kind",
    "description": "A holy knight who lost his faith after his temple was destroyed. Seeks to believe again.",
    "stats": { "hp": 103, "atk": 23, "def": 17, "spd": 13, "mana": 65 },
    "abilities": ["Healing Touch", "Divine Smite", "Sanctuary"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "The light fades, but I still remember its warmth.",
      "battleCry": "May the dawn come again!",
      "victory": "Perhaps... there is still hope.",
      "death": "I see... the light... at last..."
    },
    "personalQuest": {
      "type": "Homecoming",
      "destinationLevel": 30,
      "target": 1,
      "description": "Help Sir Aldric find the source of divine light in the depths"
    },
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 11,
    "name": "Isolde",
    "title": "the Redeemer",
    "kind": "Half-Elf",
    "class": "Paladin",
    "level": 7,
    "personality": "Protective",
    "description": "A fallen paladin seeking redemption after accidentally causing innocents' deaths in righteous fury.",
    "stats": { "hp": 106, "atk": 25, "def": 18, "spd": 15, "mana": 75 },
    "abilities": ["Shield Ally", "Mass Heal", "Resurrect"],
    "cost": 350,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I will protect you, as I failed to protect them.",
      "battleCry": "Not again! Never again!",
      "victory": "Perhaps I can atone...",
      "death": "Forgive me... I tried..."
    },
    "romanceDialogue": [
      "You see past my sins. How?",
      "With you, I feel worthy of love again.",
      "My heart beats only for you."
    ],
    "personalQuest": {
      "type": "Redemption",
      "goodDeedsNeeded": 30,
      "target": 30,
      "description": "Help Isolde atone through saving lives"
    },
    "howObtained": "Random recruitment offer on dungeon levels 4-12 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 12,
    "name": "Brother Marcus",
    "title": "the Heretic",
    "kind": "Human",
    "class": "Paladin",
    "level": 5,
    "personality": "Foolish",
    "description": "An eccentric monk who believes the gods speak to him through cheese. Surprisingly effective healer.",
    "stats": { "hp": 95, "atk": 21, "def": 16, "spd": 13, "mana": 60 },
    "abilities": ["Healing Touch", "Bless", "Purify"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "The cheddar has spoken! We must journey forth!",
      "battleCry": "In the name of the sacred brie!",
      "victory": "The gouda was right all along!",
      "death": "The... cheese... was silent..."
    },
    "personalQuest": {
      "type": "AncientArtifact",
      "itemName": "Divine Cheese Wheel",
      "target": 1,
      "description": "Help Brother Marcus find the legendary Divine Cheese"
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 13,
    "name": "Kira",
    "title": "Wolfheart",
    "kind": "Beastkin",
    "class": "Ranger",
    "level": 5,
    "personality": "Loyal",
    "description": "A wolf-kin ranger whose pack was killed by hunters. Her wolf companion Ghost is her only family.",
    "stats": { "hp": 88, "atk": 27, "def": 13, "spd": 27, "mana": 50 },
    "abilities": ["Summon", "Beast Form", "Scout"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "The pack is everything. Will you be pack?",
      "battleCry": "For the pack!",
      "victory": "The hunt... is good.",
      "death": "Ghost... stay with them..."
    },
    "romanceDialogue": [
      "I've never chosen a mate before. My heart chooses you.",
      "In wolf terms, we are bonded for life now.",
      "You are my alpha. My everything."
    ],
    "personalQuest": {
      "type": "Revenge",
      "targetName": "Hunter",
      "target": 1,
      "description": "Help Kira avenge her fallen pack"
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 14,
    "name": "Ember",
    "title": "the Wild",
    "kind": "Fairy",
    "class": "Ranger",
    "level": 4,
    "personality": "Cheerful",
    "description": "A mischievous forest fairy who got bored with her grove. Everything is an adventure!",
    "stats": { "hp": 60, "atk": 15, "def": 2, "spd": 32, "mana": 70 },
    "abilities": ["Scout", "Nature's Wrath", "Teleport"],
    "cost": 200,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Ooooh, you look like FUN! Can I come? Pleeeease?",
      "battleCry": "Wheeeee! Pew pew pew!",
      "victory": "That was AMAZING! Again! Again!",
      "death": "Oh no... the light is... pretty..."
    },
    "personalQuest": {
      "type": "ProveWorth",
      "killsNeeded": 25,
      "target": 25,
      "description": "Help Ember prove she can be a real adventurer"
    },
    "howObtained": "Random recruitment offer on dungeon levels 1-9 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 15,
    "name": "Hawk",
    "title": "the Silent Arrow",
    "kind": "Elf",
    "class": "Ranger",
    "level": 7,
    "personality": "Stoic",
    "description": "An elven master archer who took a vow of silence after failing to save his love. Speaks through actions.",
    "stats": { "hp": 94, "atk": 26, "def": 10, "spd": 22, "mana": 75 },
    "abilities": ["Ambush", "Disarm", "Scout"],
    "cost": 350,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "...",
      "battleCry": "*Draws bow with grim determination*",
      "victory": "*A rare, subtle nod of approval*",
      "death": "*Closes eyes peacefully*"
    },
    "personalQuest": {
      "type": "LostFamily",
      "dungeonLevel": 20,
      "target": 1,
      "description": "Help Hawk find what happened to his lost love"
    },
    "howObtained": "Random recruitment offer on dungeon levels 4-12 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 16,
    "name": "Morticia",
    "title": "the Grave Whisperer",
    "kind": "Vampire",
    "class": "Necromancer",
    "level": 6,
    "personality": "Kind",
    "description": "A vampire necromancer who only raises the willing dead. She sees death as another form of life.",
    "stats": { "hp": 95, "atk": 31, "def": 13, "spd": 19, "mana": 90 },
    "abilities": ["Summon", "Unholy Might", "Regeneration"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "The dead are not gone - merely quiet. I give them voice.",
      "battleCry": "Rise, friends! One more dance!",
      "victory": "See? Death need not be cruel.",
      "death": "At last... I understand... the silence..."
    },
    "romanceDialogue": [
      "I am cold, yet you warm me.",
      "An eternity alone... and then I found you.",
      "I will love you beyond death itself."
    ],
    "personalQuest": {
      "type": "LostFamily",
      "dungeonLevel": 15,
      "target": 1,
      "description": "Help Morticia find her husband's spirit"
    },
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 17,
    "name": "Bones",
    "title": "the Eternal",
    "kind": "Undead",
    "class": "Necromancer",
    "level": 8,
    "personality": "Humble",
    "description": "An ancient lich who has forgotten why he became undead. Helps adventurers hoping to remember.",
    "stats": { "hp": 121, "atk": 30, "def": 20, "spd": 4, "mana": 90 },
    "abilities": ["Summon", "Resurrect", "Arcane Mastery"],
    "cost": 400,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I have... forgotten much. Perhaps you can help me remember?",
      "battleCry": "Death is... familiar.",
      "victory": "Yes... this feels... right.",
      "death": "Finally... I remember... goodbye..."
    },
    "personalQuest": {
      "type": "Homecoming",
      "destinationLevel": 28,
      "target": 1,
      "description": "Help Bones find his phylactery and his memories"
    },
    "howObtained": "Random recruitment offer on dungeon levels 5-13 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 18,
    "name": "Lilith",
    "title": "the Blood Witch",
    "kind": "Demon",
    "class": "Necromancer",
    "level": 7,
    "personality": "Cruel",
    "description": "A demon who rebelled against the Hells. Uses dark magic for her own twisted sense of justice.",
    "stats": { "hp": 98, "atk": 33, "def": 14, "spd": 14, "mana": 90 },
    "abilities": ["Unholy Might", "Poison", "Soul Link"],
    "cost": 350,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Don't trust me. I won't trust you. Let's work together.",
      "battleCry": "Scream for me!",
      "victory": "Delicious suffering!",
      "death": "Even demons... can feel... pain..."
    },
    "personalQuest": {
      "type": "Revenge",
      "targetName": "Demon Lord",
      "target": 1,
      "description": "Help Lilith destroy her former master"
    },
    "howObtained": "Random recruitment offer on dungeon levels 4-12 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 19,
    "name": "Granite",
    "title": "the Living Mountain",
    "kind": "Golem",
    "class": "Warrior",
    "level": 10,
    "personality": "Protective",
    "description": "An ancient golem awakened to protect. Speaks little, shields much.",
    "stats": { "hp": 170, "atk": 38, "def": 40, "spd": -5, "mana": 50 },
    "abilities": ["Shield Ally", "Taunt", "Sanctuary"],
    "cost": 500,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Granite... protect.",
      "battleCry": "STONE ENDURES!",
      "victory": "All... safe.",
      "death": "Granite... crumbles..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 7-15 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 20,
    "name": "Celeste",
    "title": "the Oracle",
    "kind": "Human",
    "class": "Mage",
    "level": 6,
    "personality": "Mysterious",
    "description": "A blind seer who sees the future. Her prophecies are always true, but often misunderstood.",
    "stats": { "hp": 88, "atk": 20, "def": 13, "spd": 15, "mana": 85 },
    "abilities": ["Time Stop", "Scout", "Bless"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I have seen your fate... and chosen to walk beside you.",
      "battleCry": "As I foresaw!",
      "victory": "The future... shifts.",
      "death": "I see... everything now... beautiful..."
    },
    "romanceDialogue": [
      "I foresaw loving you. I did not foresee how deeply.",
      "Every future I see has you in it.",
      "Our fates are intertwined eternally."
    ],
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 21,
    "name": "Patches",
    "title": "the Survivor",
    "kind": "Human",
    "class": "Rogue",
    "level": 3,
    "personality": "Greedy",
    "description": "A scoundrel who has survived everything. Knows every dirty trick in the book.",
    "stats": { "hp": 69, "atk": 17, "def": 11, "spd": 20, "mana": 40 },
    "abilities": ["Steal", "Treasure Hunter", "Vanish"],
    "cost": 150,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "You look like you could use someone with... flexible morals.",
      "battleCry": "It's not cowardice, it's TACTICS!",
      "victory": "I'll take my cut now, thanks.",
      "death": "Should've... seen this coming..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 1-8 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 22,
    "name": "Aurora",
    "title": "the Dragon Princess",
    "kind": "Dragonian",
    "class": "Mage",
    "level": 8,
    "personality": "Ambitious",
    "description": "A dragonborn princess in exile. Seeks the throne that was stolen from her.",
    "stats": { "hp": 114, "atk": 34, "def": 20, "spd": 15, "mana": 100 },
    "abilities": ["Dragon Breath", "Fireball", "Battle Cry"],
    "cost": 400,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Bow not to me - but know that I am royalty.",
      "battleCry": "FEEL MY DRAGON'S WRATH!",
      "victory": "As it should be.",
      "death": "My kingdom... will rise..."
    },
    "romanceDialogue": [
      "You dare to court a princess? ...I like your courage.",
      "You would make a worthy consort.",
      "My heart and my kingdom - both yours."
    ],
    "howObtained": "Random recruitment offer on dungeon levels 5-13 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 23,
    "name": "Pip",
    "title": "the Brave",
    "kind": "Goblin",
    "class": "Warrior",
    "level": 2,
    "personality": "Brave",
    "description": "The smallest goblin in his tribe. Dreams of being a hero and proving size doesn't matter.",
    "stats": { "hp": 66, "atk": 17, "def": 7, "spd": 25, "mana": 25 },
    "abilities": ["Ambush", "Flurry"],
    "cost": 100,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "P-Pip is brave! Pip will help!",
      "battleCry": "FOR GLORY! *squeak*",
      "victory": "Pip... Pip did it!?",
      "death": "Pip... was brave... right?"
    },
    "personalQuest": {
      "type": "ProveWorth",
      "killsNeeded": 30,
      "target": 30,
      "description": "Help Pip become a true hero"
    },
    "howObtained": "Random recruitment offer on dungeon levels 1-7 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 24,
    "name": "Tempest",
    "title": "Storm's Daughter",
    "kind": "Half-Elf",
    "class": "Mage",
    "level": 6,
    "personality": "Brave",
    "description": "Child of a mortal and a storm spirit. Lightning courses through her veins.",
    "stats": { "hp": 83, "atk": 20, "def": 13, "spd": 17, "mana": 90 },
    "abilities": ["Lightning", "Teleport", "Ice Storm"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "The storm is always with me. Can you handle the thunder?",
      "battleCry": "LIGHTNING STRIKES!",
      "victory": "The storm passes, victorious!",
      "death": "Return... to the storm..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 25,
    "name": "Jack",
    "title": "the Wanderer",
    "kind": "Human",
    "class": "Ranger",
    "level": 4,
    "personality": "Humble",
    "description": "A simple farmer who lost everything to monsters. Now wanders, helping others avoid his fate.",
    "stats": { "hp": 80, "atk": 20, "def": 12, "spd": 17, "mana": 50 },
    "abilities": ["Scout", "Ambush", "Treasure Hunter"],
    "cost": 200,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I'm just a farmer with a bow. But I won't let them hurt anyone else.",
      "battleCry": "This is for my family!",
      "victory": "We did it. They're safe now.",
      "death": "Tell them... I tried..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 1-9 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 26,
    "name": "Nyx",
    "title": "the Nightmare",
    "kind": "Demon",
    "class": "Rogue",
    "level": 7,
    "personality": "Mysterious",
    "description": "A nightmare demon who feeds on fear. Grew tired of tormenting sleepers and seeks real challenges.",
    "stats": { "hp": 101, "atk": 35, "def": 15, "spd": 20, "mana": 65 },
    "abilities": ["Vanish", "Poison", "Unholy Might"],
    "cost": 350,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I've walked your nightmares. Now I'll walk beside you... if you dare.",
      "battleCry": "Fear me!",
      "victory": "Your fear... it tastes like victory.",
      "death": "Even nightmares... end..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 4-12 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 27,
    "name": "Brother Sun",
    "title": "the Radiant",
    "kind": "Human",
    "class": "Paladin",
    "level": 5,
    "personality": "Cheerful",
    "description": "A sun priest who believes joy is the greatest weapon against darkness. Annoyingly optimistic.",
    "stats": { "hp": 95, "atk": 21, "def": 16, "spd": 13, "mana": 60 },
    "abilities": ["Healing Touch", "Bless", "Divine Smite"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Praise the sun! What a glorious day for adventure!",
      "battleCry": "LET THE LIGHT SHINE!",
      "victory": "Haha! Magnificent! Simply magnificent!",
      "death": "The sun... it's so... warm..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 28,
    "name": "Vera",
    "title": "the Iron Maiden",
    "kind": "Human",
    "class": "Warrior",
    "level": 7,
    "personality": "Stoic",
    "description": "A legendary gladiator who won her freedom. Now fights for those who cannot fight for themselves.",
    "stats": { "hp": 116, "atk": 27, "def": 17, "spd": 15, "mana": 50 },
    "abilities": ["Execute", "Flurry", "Taunt"],
    "cost": 350,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Save your words. Show me what you can do.",
      "battleCry": "No retreat.",
      "victory": "Another victory.",
      "death": "A good... death..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 4-12 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 29,
    "name": "Sage Yuki",
    "title": "the Frost Flower",
    "kind": "Elf",
    "class": "Mage",
    "level": 6,
    "personality": "Cautious",
    "description": "A snow elf mage who has never left her frozen homeland. Seeks to understand the world below.",
    "stats": { "hp": 78, "atk": 20, "def": 8, "spd": 20, "mana": 95 },
    "abilities": ["Ice Storm", "Sanctuary", "Regeneration"],
    "cost": 300,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "The world... is much warmer than I expected.",
      "battleCry": "Winter's embrace!",
      "victory": "Like snow settling peacefully.",
      "death": "I return... to the eternal... winter..."
    },
    "romanceDialogue": [
      "My heart was ice. You melted it.",
      "I never knew warmth could feel so... right.",
      "Stay with me. Forever."
    ],
    "howObtained": "Random recruitment offer on dungeon levels 3-11 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 30,
    "name": "Rex",
    "title": "the Hound",
    "kind": "Beastkin",
    "class": "Ranger",
    "level": 5,
    "personality": "Loyal",
    "description": "A loyal dog-kin who was abandoned by his master. Seeks a new person to serve.",
    "stats": { "hp": 88, "atk": 27, "def": 13, "spd": 27, "mana": 50 },
    "abilities": ["Scout", "Flurry", "Treasure Hunter"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Rex will follow! Rex is good boy! ...Rex IS good boy, right?",
      "battleCry": "Bad people! Rex bite!",
      "victory": "Rex help! Rex GOOD BOY!",
      "death": "Rex... sorry... Rex tried..."
    },
    "personalQuest": {
      "type": "Confession",
      "relationshipNeeded": "Loyal",
      "target": 1,
      "description": "Show Rex he truly is a good boy"
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 31,
    "name": "Obsidian",
    "title": "the Fallen Angel",
    "kind": "Spirit",
    "class": "Paladin",
    "level": 9,
    "personality": "Stoic",
    "description": "An angel who fell from grace to save mortals. Now serves penance in the mortal realm.",
    "stats": { "hp": 97, "atk": 29, "def": 10, "spd": 23, "mana": 105 },
    "abilities": ["Divine Smite", "Healing Touch", "Resurrect"],
    "cost": 450,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "I gave up heaven. I have no regrets.",
      "battleCry": "By my fallen grace!",
      "victory": "Perhaps... redemption is possible.",
      "death": "I return... to the light..."
    },
    "romanceDialogue": [
      "Angels do not love. But I am no longer an angel.",
      "For you, I fell. For you, I would fall again.",
      "My eternity belongs to you."
    ],
    "howObtained": "Random recruitment offer on dungeon levels 6-14 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 32,
    "name": "Zara",
    "title": "the Chain Breaker",
    "kind": "Human",
    "class": "Rogue",
    "level": 5,
    "personality": "Vengeful",
    "description": "A former slave who escaped and now frees others. Burns with hatred for slavers.",
    "stats": { "hp": 85, "atk": 21, "def": 13, "spd": 20, "mana": 45 },
    "abilities": ["Disarm", "Backstab", "Ambush"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "No one should be in chains. NO ONE.",
      "battleCry": "FREEDOM!",
      "victory": "Another cage... broken.",
      "death": "I die... free..."
    },
    "personalQuest": {
      "type": "Revenge",
      "targetName": "Slaver",
      "target": 1,
      "description": "Help Zara destroy the slave trade"
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  },
  {
    "id": 33,
    "name": "Echo",
    "title": "the Mirror",
    "kind": "Spirit",
    "class": "Mage",
    "level": 5,
    "personality": "Foolish",
    "description": "A reflection that gained sentience. Struggles to understand reality and its own existence.",
    "stats": { "hp": 50, "atk": 18, "def": 2, "spd": 25, "mana": 105 },
    "abilities": ["Teleport", "Vanish", "Time Stop"],
    "cost": 250,
    "isRomanceable": true,
    "dialogue": {
      "greeting": "Am I real? Are you? What is 'real' anyway?",
      "battleCry": "Reflect this!",
      "victory": "Did I do that? Or did you?",
      "death": "I was... just a reflection... after all..."
    },
    "howObtained": "Random recruitment offer on dungeon levels 2-10 (1-3 candidates rolled per level; requires open party slot and not already recruited)"
  }
];

// ============================================================================
// LEVELING, BOND (RELATIONSHIP), ROMANCE, AND ABILITY TABLES
// Ported verbatim from src/companion.rs mechanics.
// ============================================================================

SC.DATA.companionLeveling = {
  "xpCurve": {
    "initialXpToLevelFormula": "100 + level * 50",
    "initialXpToLevelBase": 100,
    "initialXpToLevelPerLevel": 50,
    "xpToLevelGrowthMultiplier": 1.3,
    "wisePersonalityXpMultiplier": 1.25
  },
  "levelUpGains": {
    "maxHp": 8,
    "maxMana": 5,
    "attack": 2,
    "defense": 1,
    "restoresHpToFull": true,
    "restoresManaToFull": true
  },
  "statFormulas": {
    "hp": "classBaseHp + speciesHpBonus + level * 8",
    "mana": "classBaseMana + speciesManaBonus + level * 5",
    "attack": "classBaseAtk + speciesAtkBonus + level * 2",
    "defense": "classBaseDef + speciesDefBonus + level * 1",
    "speed": "classBaseSpd + speciesSpdBonus"
  },
  "classBaseStats": {
    "Warrior":     { "hp": 50, "atk": 8, "def": 5, "mana": 10, "spd": 10 },
    "Mage":        { "hp": 30, "atk": 3, "def": 2, "mana": 50, "spd": 10 },
    "Rogue":       { "hp": 35, "atk": 6, "def": 3, "mana": 20, "spd": 15 },
    "Paladin":     { "hp": 45, "atk": 6, "def": 6, "mana": 30, "spd": 8 },
    "Ranger":      { "hp": 38, "atk": 7, "def": 3, "mana": 25, "spd": 12 },
    "Necromancer": { "hp": 32, "atk": 4, "def": 2, "mana": 45, "spd": 9 }
  },
  "speciesStatBonuses": {
    "Human":     { "hp": 10,  "atk": 5,  "def": 5,   "spd": 5,   "mana": 5 },
    "Elf":       { "hp": 0,   "atk": 5,  "def": 0,   "spd": 10,  "mana": 15 },
    "Dwarf":     { "hp": 20,  "atk": 5,  "def": 15,  "spd": -5,  "mana": 0 },
    "Orc":       { "hp": 15,  "atk": 15, "def": 5,   "spd": 0,   "mana": -5 },
    "Goblin":    { "hp": 0,   "atk": 5,  "def": 0,   "spd": 15,  "mana": 5 },
    "Demon":     { "hp": 10,  "atk": 15, "def": 5,   "spd": 5,   "mana": 10 },
    "Undead":    { "hp": 25,  "atk": 10, "def": 10,  "spd": -5,  "mana": 5 },
    "Dragonian": { "hp": 20,  "atk": 15, "def": 10,  "spd": 5,   "mana": 10 },
    "Beastkin":  { "hp": 10,  "atk": 10, "def": 5,   "spd": 15,  "mana": 0 },
    "Fairy":     { "hp": -10, "atk": 0,  "def": -5,  "spd": 20,  "mana": 25 },
    "Half-Elf":  { "hp": 5,   "atk": 5,  "def": 5,   "spd": 7,   "mana": 10 },
    "Tiefling":  { "hp": 5,   "atk": 10, "def": 5,   "spd": 5,   "mana": 15 },
    "Golem":     { "hp": 40,  "atk": 10, "def": 25,  "spd": -15, "mana": -10 },
    "Spirit":    { "hp": -20, "atk": 5,  "def": -5,  "spd": 15,  "mana": 30 },
    "Vampire":   { "hp": 15,  "atk": 15, "def": 5,   "spd": 10,  "mana": 15 }
  },
  "relationshipTiers": [
    { "id": "Hostile",    "value": -2, "statBonusMultiplier": 0.5,  "xpToNextTier": 100,  "note": "Will leave or betray you" },
    { "id": "Unfriendly", "value": -1, "statBonusMultiplier": 0.75, "xpToNextTier": 200,  "note": "Reluctant, may refuse orders" },
    { "id": "Neutral",    "value": 0,  "statBonusMultiplier": 1.0,  "xpToNextTier": 500,  "note": "Basic cooperation" },
    { "id": "Friendly",   "value": 1,  "statBonusMultiplier": 1.15, "xpToNextTier": 1000, "note": "Good relations, bonus effectiveness" },
    { "id": "Loyal",      "value": 2,  "statBonusMultiplier": 1.3,  "xpToNextTier": 2500, "note": "Very strong bond, will sacrifice for you" },
    { "id": "Bonded",     "value": 3,  "statBonusMultiplier": 1.5,  "xpToNextTier": null, "note": "Soulbound, romance available, unique abilities (max tier)" }
  ],
  "relationshipXpModifiers": {
    "loyalPersonalityMultiplier": 1.5
  },
  "romanceStages": [
    { "stage": 0, "name": "Not Started", "advanceRequirement": "relationship >= Friendly" },
    { "stage": 1, "name": "Interested",  "advanceRequirement": "giftsGiven >= 3 AND relationship >= Friendly" },
    { "stage": 2, "name": "Courting",    "advanceRequirement": "datesCompleted >= 2 AND relationship >= Loyal" },
    { "stage": 3, "name": "Devoted",     "advanceRequirement": "confessionDone AND relationship >= Loyal" },
    { "stage": 4, "name": "In Love",     "advanceRequirement": "relationship >= Bonded" },
    { "stage": 5, "name": "Soulbound",   "advanceRequirement": null }
  ],
  "startingRelationship": "Neutral",
  "abilityCatalog": {
    "Shield Ally":     { "manaCost": 10,  "cooldown": 3,  "passive": false, "category": "Combat" },
    "Battle Cry":      { "manaCost": 15,  "cooldown": 3,  "passive": false, "category": "Combat" },
    "Taunt":           { "manaCost": 10,  "cooldown": 3,  "passive": false, "category": "Combat" },
    "Flurry":          { "manaCost": 15,  "cooldown": 3,  "passive": false, "category": "Combat" },
    "Execute":         { "manaCost": 25,  "cooldown": 5,  "passive": false, "category": "Combat" },
    "Cleave":          { "manaCost": 15,  "cooldown": 3,  "passive": false, "category": "Combat" },
    "Healing Touch":   { "manaCost": 20,  "cooldown": 3,  "passive": false, "category": "Support" },
    "Mass Heal":       { "manaCost": 35,  "cooldown": 10, "passive": false, "category": "Support" },
    "Resurrect":       { "manaCost": 50,  "cooldown": 20, "passive": false, "category": "Support" },
    "Bless":           { "manaCost": 20,  "cooldown": 3,  "passive": false, "category": "Support" },
    "Purify":          { "manaCost": 20,  "cooldown": 3,  "passive": false, "category": "Support" },
    "Sanctuary":       { "manaCost": 35,  "cooldown": 8,  "passive": false, "category": "Support" },
    "Fireball":        { "manaCost": 25,  "cooldown": 3,  "passive": false, "category": "Magic" },
    "Ice Storm":       { "manaCost": 35,  "cooldown": 5,  "passive": false, "category": "Magic" },
    "Lightning":       { "manaCost": 25,  "cooldown": 3,  "passive": false, "category": "Magic" },
    "Teleport":        { "manaCost": 35,  "cooldown": 3,  "passive": false, "category": "Magic" },
    "Summon":          { "manaCost": 50,  "cooldown": 10, "passive": false, "category": "Magic" },
    "Time Stop":       { "manaCost": 60,  "cooldown": 20, "passive": false, "category": "Magic" },
    "Backstab":        { "manaCost": 10,  "cooldown": 3,  "passive": false, "category": "Stealth" },
    "Vanish":          { "manaCost": 20,  "cooldown": 3,  "passive": false, "category": "Stealth" },
    "Poison":          { "manaCost": 10,  "cooldown": 3,  "passive": false, "category": "Stealth" },
    "Steal":           { "manaCost": 20,  "cooldown": 3,  "passive": false, "category": "Stealth" },
    "Disarm":          { "manaCost": 15,  "cooldown": 3,  "passive": false, "category": "Stealth" },
    "Ambush":          { "manaCost": 15,  "cooldown": 3,  "passive": false, "category": "Stealth" },
    "Dragon Breath":   { "manaCost": 50,  "cooldown": 10, "passive": false, "category": "Unique" },
    "Beast Form":      { "manaCost": 50,  "cooldown": 10, "passive": false, "category": "Unique" },
    "Unholy Might":    { "manaCost": 60,  "cooldown": 8,  "passive": false, "category": "Unique" },
    "Divine Smite":    { "manaCost": 60,  "cooldown": 8,  "passive": false, "category": "Unique" },
    "Nature's Wrath":  { "manaCost": 60,  "cooldown": 8,  "passive": false, "category": "Unique" },
    "Soul Link":       { "manaCost": 100, "cooldown": 20, "passive": false, "category": "Unique" },
    "Regeneration":    { "manaCost": 0,   "cooldown": 3,  "passive": true,  "category": "Passive" },
    "Treasure Hunter": { "manaCost": 0,   "cooldown": 3,  "passive": true,  "category": "Passive" },
    "Scout":           { "manaCost": 0,   "cooldown": 3,  "passive": true,  "category": "Passive" },
    "Diplomat":        { "manaCost": 0,   "cooldown": 3,  "passive": true,  "category": "Passive" },
    "Martial Arts":    { "manaCost": 0,   "cooldown": 3,  "passive": true,  "category": "Passive" },
    "Arcane Mastery":  { "manaCost": 0,   "cooldown": 3,  "passive": true,  "category": "Passive" }
  },
  "personalityEffects": {
    "Brave":      "Never retreats, fights harder when wounded (+25% attack below 1/3 HP)",
    "Cautious":   "Careful and defensive, retreats when hurt (+15% defense)",
    "Loyal":      "Steadfast companion, quickly forms bonds (+50% relationship XP)",
    "Greedy":     "Loves treasure, may pocket some for themselves",
    "Kind":       "Compassionate healer, loved by all",
    "Cruel":      "Merciless in battle, no quarter given",
    "Wise":       "Quick learner, gains experience faster (+25% XP)",
    "Foolish":    "Unpredictable, sometimes brilliantly so",
    "Romantic":   "Seeks deep connections and love",
    "Stoic":      "Emotionally strong, resists mental effects (50% chance to resist Stun/Confusion/Blind)",
    "Vengeful":   "Remembers every slight, seeks payback",
    "Protective": "Will sacrifice themselves for allies",
    "Ambitious":  "Seeks power and recognition",
    "Humble":     "Quiet and supportive, great team player",
    "Mysterious": "Hides secrets and hidden powers",
    "Cheerful":   "Optimistic, keeps morale high (+1 party morale per tick while below 100)"
  },
  "deathRules": {
    "unconsciousTurns": 10,
    "maxFreeRevivals": 3,
    "essentialRevivesAtHpPercent": 25,
    "note": "At 0 HP a companion falls Unconscious for 10 turns if essential or revived fewer than 3 times, otherwise Dead (resurrectable). Unconscious non-essential companions die when the timer runs out."
  }
};

SC.DATA.companions_notes = [
  "Source: src/companion.rs create_all_companions() — all 33 predefined companions ported; names, titles, backstories, personalities, levels, abilities, dialogue, romance dialogue, and personal quests are verbatim.",
  "INVENTED: 'cost' — the Rust source has no gold recruit price (recruitment is a free random offer via get_recruitable_companions). A balanced value of level * 50 gold was assigned to every companion.",
  "DERIVED: 'stats' are not stored literally in the source; they are the exact values Companion::new() computes at each companion's starting level from CharacterClass::base_stats() (src/main.rs) + CompanionSpecies::stat_bonuses() + level scaling. Formulas and both source tables are included verbatim in SC.DATA.companionLeveling.",
  "ADAPTED: stats include 'mana' in addition to the contracted hp/atk/def/spd, because mana is a core companion stat in the source (abilities cost mana).",
  "VERBATIM ODDITY: Granite (Golem, Warrior base speed 10, species speed -15) computes to spd -5 — kept as the formula produces it.",
  "DERIVED: 'howObtained' — get_recruitable_companions() offers 1-3 random un-recruited companions per dungeon level, filtered to companionLevel <= dungeonLevel + 3 AND companionLevel + 5 >= dungeonLevel; each entry's stated level range is that filter solved for dungeon level (floor 1).",
  "OMITTED FIELD: 'evolutions' — no companion evolution system exists in the Rust source, so the field is omitted everywhere.",
  "'kind' is the companion's species name; 'class' (Warrior/Mage/Rogue/Paladin/Ranger/Necromancer) is carried separately since both exist in the source.",
  "'isRomanceable' is true for all 33 (Companion::new default; no companion overrides it). Companions with a 'romanceDialogue' array have bespoke romance lines; the rest are romanceable but have no unique lines.",
  "personalQuest 'target' is the quest completion count from CompanionQuest::target() (e.g. Revenge/artifact/homecoming quests need 1, ProveWorth needs killsNeeded, etc.).",
  "New-recruit defaults from Companion::new(): AI mode Balanced, relationship Neutral, formation position Back Center, no equipment, romance stage 0, not essential.",
  "SC.DATA.companionLeveling carries the XP curve (xpToLevel starts at 100 + level*50, grows x1.3 per level; +8 maxHP, +5 maxMana, +2 ATK, +1 DEF per level with full restore), the relationship/bond tier table (stat multipliers 0.5-1.5 and XP thresholds 100/200/500/1000/2500), romance stages, personality effects, death rules, and the full 36-entry ability catalog (mana costs, cooldowns, passive flags) — placed here rather than as extra globals to honor the file contract.",
  "NOT PORTED (behavior, not companion data): AI mode logic (Aggressive/Defensive/Support/Balanced/Passive), party formations and morale math, formation position modifiers, status-effect ticking, and map symbols/colors — these live in src/companion.rs as code and belong in the web port's logic layer.",
  "Ability names use display names from CompanionAbility::name(); the Rust 'Comp'-prefixed variants (CompFireball, CompBackstab, CompVanish, CompLightning, CompTeleport, CompRegeneration) are rendered by their display names (Fireball, Backstab, Vanish, Lightning, Teleport, Regeneration)."
];

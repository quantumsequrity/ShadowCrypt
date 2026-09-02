# ShadowCrypt

**A Terminal Roguelike Dungeon Crawler — now also a cross-platform action MMORPG**

Descend into the depths of ShadowCrypt, a classic roguelike adventure where death is permanent and every decision matters. Battle through 30 floors of procedurally generated dungeons, face terrifying bosses, and uncover legendary artifacts as you fight your way to defeat the Demon King.

> ## 🌐 NEW: ShadowCrypt Online
> The full game has been converted into a **web MMORPG** playable on **mobile, tablet and PC** — **offline** (installable PWA) and **online** (chat, co-op presence, real-time PvP arena, cloud saves). It adds Haven base-building, real-time farming, companions, crafting, quests, factions and achievements on top of everything below.
>
> **→ See [PLAY_ONLINE.md](PLAY_ONLINE.md)** · Client: `web/` · Server: `server/` (`cd server && npm install && npm start`)

---

## Features

- **6 Unique Character Classes** - Each with distinct abilities and playstyles
- **30 Dungeon Floors** - Spanning 8 thematic environments
- **60+ Enemy Types** - Including 6 bosses and 6 mini-bosses
- **130+ Items** - Weapons, armor, potions, scrolls, rings, amulets, and food
- **13 Status Effects** - Both beneficial buffs and dangerous debuffs
- **Permadeath** - True roguelike experience where every run is unique
- **Hunger System** - Manage your food supply to survive
- **Field of View** - Explore dark dungeons with limited visibility
- **Interactive Environment** - Doors, chests, shrines, traps, and more

---

## Character Classes

Choose your hero wisely. Each class offers a unique approach to conquering the dungeon.

### Warrior
| Stat | Value |
|------|-------|
| HP | 50 |
| Attack | 8 |
| Defense | 5 |
| Mana | 10 |
| Speed | 10 |

**Special Ability: Berserk** - Enter a rage state dealing 2x damage but taking 50% more damage in return.

*Ideal for: Players who prefer direct combat and high survivability.*

---

### Mage
| Stat | Value |
|------|-------|
| HP | 30 |
| Attack | 3 |
| Defense | 2 |
| Mana | 50 |
| Speed | 10 |

**Special Ability: Fireball** - Unleash a devastating area-of-effect fire attack that damages all nearby enemies.

*Ideal for: Players who enjoy spellcasting and dealing with groups of enemies.*

---

### Rogue
| Stat | Value |
|------|-------|
| HP | 35 |
| Attack | 6 |
| Defense | 3 |
| Mana | 20 |
| Speed | 15 |

**Special Ability: Backstab** - Deal 3x damage when attacking an enemy from behind.

*Ideal for: Players who prefer stealth, positioning, and critical strikes.*

---

### Paladin
| Stat | Value |
|------|-------|
| HP | 45 |
| Attack | 6 |
| Defense | 6 |
| Mana | 30 |
| Speed | 8 |

**Special Ability: Holy Light** - Heal yourself while simultaneously dealing bonus damage to undead enemies.

*Ideal for: Balanced gameplay with both offensive and defensive capabilities.*

---

### Ranger
| Stat | Value |
|------|-------|
| HP | 38 |
| Attack | 7 |
| Defense | 3 |
| Mana | 25 |
| Speed | 12 |

**Special Ability: Multi-shot** - Fire arrows at up to 3 enemies simultaneously.

*Ideal for: Players who like ranged combat and crowd control.*

---

### Necromancer
| Stat | Value |
|------|-------|
| HP | 32 |
| Attack | 4 |
| Defense | 2 |
| Mana | 45 |
| Speed | 9 |

**Special Ability: Raise Dead** - Summon a skeleton ally to fight alongside you.

*Ideal for: Players who enjoy summoning minions and dark magic.*

---

## Dungeon Floors and Themes

The dungeon consists of 30 floors divided into 8 themed areas. Each theme features unique enemies, environmental hazards, and atmosphere.

### Floor Layout

| Floors | Theme | Description |
|--------|-------|-------------|
| 1-4 | **Dark Dungeon** | Classic stone corridors filled with vermin and goblins |
| 5-8 | **Twisted Caves** | Natural caverns with trolls, orcs, and elemental creatures |
| 9-12 | **Haunted Crypt** | Undead-infested tombs with ghosts, vampires, and wraiths |
| 13-16 | **Cursed Forest** | Underground forest with wolves, ents, and forest spirits |
| 17-20 | **Frozen Caverns** | Ice-covered tunnels with frost giants and yetis |
| 21-24 | **Volcanic Depths** | Lava-filled chambers with fire elementals and hellhounds |
| 25-28 | **Ancient Ruins** | Forgotten temples guarded by golems and sphinxes |
| 29-30 | **Demon Realm** | The final hellish domain of the Demon King |

### Boss Floors

Bosses await at the end of each major area:

| Floor | Boss | Theme |
|-------|------|-------|
| 5 | **Goblin King** | Dark Dungeon |
| 10 | **Orc Warlord** | Twisted Caves |
| 15 | **Vampire Lord** | Haunted Crypt |
| 20 | **Forest Guardian** | Cursed Forest |
| 25 | **Ice Dragon** | Frozen Caverns |
| 30 | **Demon King** | Demon Realm |

---

## Enemies

### Tier 1: Dark Dungeon (Floors 1-4)
- Rat, Giant Rat
- Bat
- Spider
- Goblin, Kobold
- Skeleton
- Cave Crawler
- **Mini-Boss: Goblin Champion**

### Tier 2: Twisted Caves (Floors 5-8)
- Giant Spider
- Orc, Hobgoblin
- Troll, Cave Ogre
- Slime
- Cave Bear
- Mushroom (hostile)
- Rock Elemental
- **Mini-Boss: Orc Berserker**

### Tier 3: Haunted Crypt (Floors 9-12)
- Zombie, Ghoul
- Ghost, Wraith, Banshee
- Vampire
- Mummy
- Death Knight
- Bone Golem
- **Mini-Boss: Vampire Elite**

### Tier 4: Cursed Forest (Floors 13-16)
- Wolf, Dire Wolf
- Tree Ent
- Forest Troll
- Druid (corrupted)
- Wild Boar
- Giant Wasp
- Venomous Vine
- Forest Spirit
- **Mini-Boss: Ancient Wyrm**

### Tier 5: Frozen Caverns (Floors 17-20)
- Ice Elemental
- Frost Giant
- Yeti Warrior
- Ice Wraith
- Frost Wolf
- Ice Spider
- Frozen Knight
- Wendigo
- **Mini-Boss: Frost Lord**

### Tier 6: Volcanic Depths (Floors 21-24)
- Fire Elemental
- Lava Golem
- Hellhound
- Fire Drake
- Magma Slime
- Salamander
- Cinder Wraith
- Infernal Imp
- **Mini-Boss: Infernal Lord**

### Tier 7: Ancient Ruins (Floors 25-28)
- Golem
- Ancient Guardian
- Sphinx
- Lich
- Gargoyle
- Mummy Lord
- Cursed Statue
- Shadow Assassin

### Tier 8: Demon Realm (Floors 29-30)
- Demon
- Demon Lord
- Succubus
- Balrog
- Pit Fiend
- Shadow Demon
- Abyssal Horror
- Doom Guard

---

## Items

ShadowCrypt features over 130 unique items across multiple categories.

### Weapons (25 types)

**Melee Weapons:**
- Dagger, Short Sword, Long Sword, Greatsword, Katana, Rapier
- Axe, Battle Axe, Thunder Axe
- Mace, War Hammer, Flail, Morningstar
- Spear, Halberd, Trident
- Scythe, Demon Slayer

**Ranged Weapons:**
- Bow, Crossbow

**Magic Weapons:**
- Staff, Wand, Void Staff

**Elemental Weapons:**
- Flame Sword, Frost Blade

### Shields (10 types)
- Buckler, Wooden Shield, Iron Shield, Tower Shield
- Magic Shield, Dragon Shield, Spiked Shield
- Mirror Shield, Phoenix Shield, Abyssal Shield

### Armor (12 types)
- Leather Armor, Chain Mail, Scale Mail, Plate Mail
- Mage Robes, Assassin's Garb
- Dragon Armor, Holy Armor, Demon Armor
- Crystal Armor, Shadow Cloak, Titan Plate

### Helmets (10 types)
- Leather Cap, Iron Helm, Steel Helm
- Crown of Kings, Wizard's Hat, Demon Skull
- Dragon Helm, Crystal Crown, Hood of Shadows, Helm of Valor

### Gloves (8 types)
- Leather Gloves, Iron Gauntlets
- Gloves of Power, Thief's Gloves
- Dragon Gauntlets, Frost Gauntlets, Flame Gauntlets, Gauntlets of Might

### Boots (8 types)
- Leather Boots, Iron Boots
- Boots of Speed, Boots of Leaping
- Winged Boots, Shadow Boots, Lava Walkers, Boots of the Wind

### Rings (15 types)
- Ring of Strength, Ring of Protection, Ring of Speed
- Ring of Regeneration, Ring of Mana, Ring of Luck
- Ring of Fireball, Ring of Invisibility
- Vampire Ring, Ring of Death
- Ring of Frost, Ring of Flame, Ring of Thunder
- Ring of Shadows, Ring of the Ancients

### Amulets (12 types)
- Amulet of Health, Amulet of Mana, Amulet of Protection
- Amulet of Power, Amulet of Wisdom, Amulet of Life
- Amulet of Death, Amulet of the Gods, Amulet of Dragons
- Amulet of Chaos, Amulet of Order, Amulet of Balance

### Potions (20 types)
- Health Potion, Mana Potion, Full Restore Elixir
- Strength Potion, Defense Potion, Speed Potion
- Invisibility Potion, Regeneration Potion, Berserk Potion
- Fire Resist Potion, Ice Resist Potion, Antidote (Poison Resist)
- Giant's Strength, Levitation Potion
- Potion of Experience, Luck Potion, Critical Strike Potion
- Potion of True Sight, Cure All Elixir, Ultimate Power Elixir

### Scrolls (18 types)
- Scroll of Teleport, Scroll of Mapping, Scroll of Identify
- Scroll of Fireball, Scroll of Ice Storm, Scroll of Lightning
- Scroll of Chain Lightning, Scroll of Blizzard, Scroll of Meteor
- Scroll of Earthquake, Scroll of Divine Wrath, Scroll of Darkness
- Scroll of Enchant, Scroll of Summoning, Scroll of Banishment
- Scroll of Time Stop, Scroll of Mass Heal, Scroll of Death

### Food (8 types)
- Apple, Bread, Cheese, Meat, Royal Feast
- Dragon Fruit, Ancient Wine, Golden Apple

### Special Items (10 types)
- Gold, Key, Bomb, Torch, Compass
- Teleport Crystal, Soul Gem, Ancient Relic
- Dragon Scale, Demon Heart

### Item Rarities

Items can spawn with different rarity levels, affecting their stats:

| Rarity | Stat Multiplier | Color |
|--------|-----------------|-------|
| Common | 1.0x | Grey |
| Uncommon (Fine) | 1.25x | Green |
| Rare (Superior) | 1.5x | Blue |
| Epic | 2.0x | Magenta |
| Legendary | 3.0x | Yellow |
| Mythic | 5.0x | Red |

---

## Game Mechanics

### Combat

Combat in ShadowCrypt is turn-based. Move into an enemy to attack them. Your damage is calculated based on your attack stat versus their defense.

- **Attack**: Base weapon damage + character attack stat
- **Defense**: Reduces incoming damage
- **Critical Hits**: Can deal bonus damage (affected by certain items)

### Equipment Slots

You can equip items in the following slots:
- Weapon
- Shield
- Helmet
- Armor
- Gloves
- Boots
- Ring 1
- Ring 2
- Amulet

### Status Effects

**Negative Effects:**
| Effect | Description |
|--------|-------------|
| Poisoned | Take damage over time (green) |
| Burning | Take fire damage over time (red) |
| Frozen | Movement impaired (cyan) |
| Bleeding | Continuous HP loss (dark red) |
| Stunned | Cannot act (yellow) |
| Blind | Reduced vision (dark grey) |
| Weakened | Reduced attack power (dark magenta) |
| Confused | Movement may be random (dark yellow) |

**Positive Effects:**
| Effect | Description |
|--------|-------------|
| Haste | Increased speed (blue) |
| Shielded | Damage reduction (white) |
| Regenerating | Heal over time (magenta) |
| Strengthened | Increased attack (yellow) |
| Invisible | Enemies cannot see you (grey) |

### Hunger System

Your character has a hunger meter that depletes as you take actions. If hunger reaches zero, you start taking starvation damage. Eat food regularly to survive.

| Food | Hunger Restored |
|------|-----------------|
| Apple | 10 |
| Cheese | 20 |
| Bread | 25 |
| Meat | 40 |
| Royal Feast | 100 |

### Field of View

You can only see within a 10-tile radius around your character. Walls and pillars block line of sight. Use torches to temporarily increase visibility.

### Environmental Features

| Tile | Symbol | Effect |
|------|--------|--------|
| Stairs Down | `>` | Descend to next floor |
| Stairs Up | `<` | Return to previous floor |
| Door | `+` | Must be opened to pass |
| Trap | `^` | Triggers damage when stepped on |
| Water | `~` (blue) | Slows movement |
| Lava | `~` (red) | Damages when crossed |
| Chest | `=` | Contains loot |
| Shrine | `&` | Grants blessings (one use) |
| Boss Gate | `8` | Entrance to boss arena |

---

## Controls

| Key | Action |
|-----|--------|
| `W` / `Up` | Move up |
| `S` / `Down` | Move down |
| `A` / `Left` | Move left |
| `D` / `Right` | Move right |
| `Q` | Move up-left (diagonal) |
| `E` | Move up-right (diagonal) |
| `Z` | Move down-left (diagonal) |
| `C` | Move down-right (diagonal) |
| `Space` | Wait one turn |
| `I` | Open inventory |
| `G` | Pick up item |
| `U` | Use item |
| `P` | Drop item |
| `Tab` | View equipment |
| `1-4` | Use skill 1-4 |
| `>` | Descend stairs |
| `<` | Ascend stairs |
| `M` | View message log |
| `?` | Help |
| `Esc` | Menu / Cancel |

---

## Tips for Survival

1. **Always carry food** - Starvation is a common cause of death
2. **Identify items before use** - Some scrolls and potions may be harmful
3. **Explore carefully** - Check for traps before entering new rooms
4. **Manage your resources** - Save powerful scrolls and potions for boss fights
5. **Know when to retreat** - Use stairs to escape dangerous situations
6. **Use the environment** - Lure enemies through lava or into traps
7. **Upgrade equipment regularly** - Higher rarity items make a huge difference
8. **Mind your status effects** - Cure negative effects quickly

---

## Version

- **Version:** 0.1.0
- **Engine:** Rust with crossterm
- **Author:** bloodraven

---

*Good luck, adventurer. The depths await.*

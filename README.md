# ShadowCrypt

**An Expansive Roguelike Dungeon Crawler with Deep RPG Systems**

Descend into the depths of ShadowCrypt, a massive roguelike adventure featuring 100 floors of procedurally-generated dungeons, extensive character progression, and dozens of interconnected gameplay mechanics. The game combines traditional roguelike elements with deep RPG systems inspired by cultivation games and multiplayer MMORPGs.

---

## Table of Contents

- [Installation](#installation)
- [How to Play](#how-to-play)
- [Controls](#controls)
- [Character Creation](#character-creation)
- [Classes & Specializations](#classes--specializations)
- [Species & Races](#species--races)
- [Magic System](#magic-system)
- [Dungeon Floors & Themes](#dungeon-floors--themes)
- [Enemies & Bosses](#enemies--bosses)
- [Items & Equipment](#items--equipment)
- [Progression Systems](#progression-systems)
- [Crafting & Professions](#crafting--professions)
- [Companions & Familiars](#companions--familiars)
- [Advanced Combat Systems](#advanced-combat-systems)
- [Social Systems](#social-systems)
- [Minigames](#minigames)
- [Arena System](#arena-system)
- [Economic System](#economic-system)
- [Story & Adventure](#story--adventure)
- [Game Modes](#game-modes)
- [Additional Systems](#additional-systems)
- [Incomplete & Coming Soon](#incomplete--coming-soon)
- [Game Statistics](#game-statistics)
- [Contributing](#contributing)

---

## Installation

### Prerequisites

- **Rust** (latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** package manager (comes with Rust)

### Quick Install

```bash
# Clone the repository
git clone https://github.com/quantumsequrity/ShadowCrypt.git
cd ShadowCrypt

# Build the game (release mode recommended for performance)
cargo build --release

# Run CLI version
cargo run --release -p shadowcrypt-cli

# Run GUI version
cargo run --release -p shadowcrypt-gui
```

### Build Options

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (slower compilation, optimized runtime)
cargo build --release

# Build only CLI
cargo build --release -p shadowcrypt-cli

# Build only GUI
cargo build --release -p shadowcrypt-gui

# Run tests
cargo test

# Check for issues
cargo check
```

### System Requirements

- **OS:** Windows, macOS, or Linux
- **Terminal:** For CLI version, requires a terminal with Unicode support
- **Display:** For GUI version, requires a graphical display

---

## How to Play

### Starting the Game

1. Run the game using `cargo run --release -p shadowcrypt-cli` (terminal) or `cargo run --release -p shadowcrypt-gui` (graphical)
2. Create a new character or load an existing save
3. Choose your species, class, and customize your character
4. Enter the dungeon and begin your adventure!

### Core Gameplay Loop

1. **Explore** - Navigate procedurally-generated dungeon floors
2. **Fight** - Engage enemies in turn-based combat
3. **Loot** - Collect items, equipment, and resources
4. **Level Up** - Gain experience and improve your character
5. **Descend** - Progress deeper into the dungeon
6. **Survive** - Manage health, mana, hunger, and resources
7. **Defeat Bosses** - Conquer boss floors every 10 levels

### Survival Tips

- **Always carry food** - Starvation is deadly
- **Explore carefully** - Watch for traps and ambushes
- **Save resources** - Keep powerful items for boss fights
- **Know when to retreat** - Use stairs to escape danger
- **Upgrade regularly** - Better gear means better survival
- **Manage status effects** - Cure negative effects quickly
- **Use the environment** - Lure enemies into hazards

---

## Controls

### Movement

| Key | Action |
|-----|--------|
| `W` / `Up Arrow` | Move up |
| `S` / `Down Arrow` | Move down |
| `A` / `Left Arrow` | Move left |
| `D` / `Right Arrow` | Move right |
| `Q` | Move up-left (diagonal) |
| `E` | Move up-right (diagonal) |
| `Z` | Move down-left (diagonal) |
| `C` | Move down-right (diagonal) |
| `Space` | Wait one turn |

### Actions

| Key | Action |
|-----|--------|
| `G` | Pick up item |
| `I` | Open inventory |
| `U` | Use item |
| `P` | Drop item |
| `Tab` | View equipment |
| `>` | Descend stairs |
| `<` | Ascend stairs |

### Combat & Skills

| Key | Action |
|-----|--------|
| `1-4` | Use skill slot 1-4 |
| `5-0` | Use skill slot 5-10 |
| `F` | Toggle formation |
| `T` | Command companions |

### Interface

| Key | Action |
|-----|--------|
| `M` | View message log |
| `J` | Open journal/quests |
| `K` | View skill trees |
| `L` | View map |
| `?` | Help menu |
| `Esc` | Menu / Cancel |

---

## Character Creation

### Character Stats

| Stat | Description |
|------|-------------|
| **HP** | Health Points - your life force |
| **Mana** | Magical energy for spells |
| **Attack** | Physical damage dealt |
| **Defense** | Damage reduction |
| **Speed** | Turn order and evasion |
| **Intelligence** | Magic power and mana pool |
| **Luck** | Critical hits and loot quality |

### Equipment Slots

- Head (Helmet)
- Body (Armor)
- Hands (Gloves)
- Legs (Leggings)
- Feet (Boots)
- Weapon (Main hand)
- Off-hand (Shield/Secondary)
- Ring 1
- Ring 2
- Amulet

---

## Classes & Specializations

### Base Classes (6)

#### Warrior
| Stat | Value | Description |
|------|-------|-------------|
| HP | 50 | High survivability |
| Attack | 8 | Strong melee damage |
| Defense | 5 | Good protection |
| Mana | 10 | Limited magic |
| Speed | 10 | Average |

**Special Ability: Berserk** - Enter rage state, deal 2x damage but take 50% more.

*Playstyle: Direct combat, tanking, frontline fighter*

---

#### Mage
| Stat | Value | Description |
|------|-------|-------------|
| HP | 30 | Fragile |
| Attack | 3 | Weak physical |
| Defense | 2 | Low protection |
| Mana | 50 | Massive mana pool |
| Speed | 10 | Average |

**Special Ability: Fireball** - AoE fire attack damaging all nearby enemies.

*Playstyle: Spellcasting, AoE damage, elemental mastery*

---

#### Rogue
| Stat | Value | Description |
|------|-------|-------------|
| HP | 35 | Moderate |
| Attack | 6 | Good damage |
| Defense | 3 | Light armor |
| Mana | 20 | Some abilities |
| Speed | 15 | Very fast |

**Special Ability: Backstab** - Deal 3x damage when attacking from behind.

*Playstyle: Stealth, positioning, critical strikes*

---

#### Cleric
| Stat | Value | Description |
|------|-------|-------------|
| HP | 40 | Moderate |
| Attack | 5 | Balanced |
| Defense | 4 | Good protection |
| Mana | 40 | High for healing |
| Speed | 9 | Slightly slow |

**Special Ability: Divine Heal** - Restore HP to self and nearby allies.

*Playstyle: Healing, support, holy magic*

---

#### Ranger
| Stat | Value | Description |
|------|-------|-------------|
| HP | 38 | Moderate |
| Attack | 7 | Good ranged |
| Defense | 3 | Light armor |
| Mana | 25 | Nature magic |
| Speed | 12 | Fast |

**Special Ability: Multi-shot** - Fire arrows at up to 3 enemies simultaneously.

*Playstyle: Ranged combat, crowd control, nature magic*

---

#### Monk
| Stat | Value | Description |
|------|-------|-------------|
| HP | 42 | Good |
| Attack | 6 | Unarmed combat |
| Defense | 4 | Agile defense |
| Mana | 30 | Chi/Ki energy |
| Speed | 14 | Very fast |

**Special Ability: Flurry of Blows** - Rapid combo attack hitting multiple times.

*Playstyle: Martial arts, chi abilities, mobility*

---

### Tier 1 Specializations (Level 20)

Each class has 4 specialization options at level 20, totaling 24 specializations:

| Class | Specializations |
|-------|-----------------|
| Warrior | Berserker, Guardian, Champion, Weaponmaster |
| Mage | Elementalist, Archmage, Battle Mage, Enchanter |
| Rogue | Assassin, Shadowdancer, Trickster, Swashbuckler |
| Cleric | High Priest, Paladin, Inquisitor, Oracle |
| Ranger | Beast Master, Sharpshooter, Warden, Scout |
| Monk | Martial Artist, Zen Master, Shadow Monk, Iron Fist |

### Tier 2 Specializations (Level 50)

Advanced specializations unlock at level 50, with 48 total options including:
- Blood Reaver, Titan Guardian, Void Walker
- Arcane Sage, Chrono Mage, Spell Blade
- Plague Lord, Shadow Assassin, Master Thief
- Divine Templar, Death Knight, Seraph
- And many more...

---

## Species & Races

### Major Species (12)

#### Human
- **Subspecies:** Noble, Commoner, Barbarian, Islander
- **Bonuses:** +10% experience gain, versatile stats
- **Abilities:** Adaptability, Leadership

#### Elf
- **Subspecies:** High Elf, Wood Elf, Dark Elf, Sea Elf
- **Bonuses:** +20% mana, +10% speed
- **Abilities:** Keen Senses, Longevity

#### Dwarf
- **Subspecies:** Mountain, Hill, Deep, Frost
- **Bonuses:** +20% defense, poison resistance
- **Abilities:** Stonecunning, Alcohol Tolerance

#### Orc
- **Subspecies:** Green, Grey, Black, Half-Orc
- **Bonuses:** +25% attack, -10% intelligence
- **Abilities:** Bloodrage, Intimidation

#### Undead
- **Subspecies:** Skeleton, Zombie, Ghost, Vampire, Lich
- **Bonuses:** Poison/bleed immunity, no hunger
- **Abilities:** Dark Vision, Undying Will

#### Demon
- **Subspecies:** Imp, Succubus, Pit Fiend, Balor
- **Bonuses:** Fire resistance, +15% attack
- **Abilities:** Hellfire, Demonic Pact

#### Beastkin
- **Subspecies:** Wolf, Cat, Bear, Fox, Rabbit
- **Bonuses:** +15% speed, enhanced senses
- **Abilities:** Beast Form, Pack Tactics

#### Dragonborn
- **Subspecies:** Fire Drake, Ice Drake, Storm Drake, Shadow Drake
- **Bonuses:** Elemental breath, +20% HP
- **Abilities:** Dragon Breath, Scales

#### Fae
- **Subspecies:** Pixie, Sprite, Dryad, Nymph
- **Bonuses:** +30% mana, nature affinity
- **Abilities:** Glamour, Teleport

#### Construct
- **Subspecies:** Golem, Automaton, Warforged, Clockwork
- **Bonuses:** No hunger/poison/bleed, +25% defense
- **Abilities:** Self-Repair, Mechanical Body

#### Elemental
- **Subspecies:** Fire, Water, Earth, Air
- **Bonuses:** Elemental immunity (type), +50% elemental damage
- **Abilities:** Elemental Form, Energy Absorption

#### Celestial
- **Subspecies:** Angel, Seraph, Archon, Nephilim
- **Bonuses:** +20% holy damage, demon slaying
- **Abilities:** Holy Light, Divine Protection

---

## Magic System

### 12 Schools of Magic

| School | Element | Spells | Description |
|--------|---------|--------|-------------|
| **Pyromancy** | Fire | 15 | Destruction, burns, explosions |
| **Cryomancy** | Ice | 15 | Freezing, shatter, slow effects |
| **Electromancy** | Lightning | 12 | Chain damage, paralysis |
| **Geomancy** | Earth | 12 | Defense, stability, earthquakes |
| **Hydromancy** | Water | 12 | Healing, cleansing, flow |
| **Aeromancy** | Wind | 10 | Speed, evasion, flight |
| **Lumimancy** | Light | 14 | Holy damage, healing, revelation |
| **Umbramancy** | Shadow | 14 | Curses, debuffs, stealth |
| **Necromancy** | Death | 18 | Undead command, soul magic |
| **Psychomancy** | Mind | 12 | Control, illusions, telepathy |
| **Chronomancy** | Time | 10 | Haste, slow, time manipulation |
| **Dimensionalism** | Space | 8 | Teleportation, rifts, portals |

### Spell Examples

**Pyromancy:**
- Flame Bolt, Fireball, Inferno
- Fire Wall, Meteor Storm, Phoenix Rebirth

**Necromancy:**
- Raise Skeleton, Drain Life, Death Coil
- Animate Dead, Army of the Dead, Lich Transformation

**Chronomancy:**
- Haste, Slow, Time Stop
- Rewind, Age, Temporal Rift

---

## Dungeon Floors & Themes

### Floor Structure (100 Floors)

| Floors | Theme | Description |
|--------|-------|-------------|
| 1-10 | **Dark Dungeon** | Classic stone corridors, vermin, goblins |
| 11-20 | **Twisted Caves** | Natural caverns, trolls, orcs, elementals |
| 21-30 | **Haunted Crypt** | Undead tombs, ghosts, vampires, liches |
| 31-40 | **Cursed Forest** | Underground forest, wolves, ents, spirits |
| 41-50 | **Frozen Caverns** | Ice tunnels, frost giants, yetis |
| 51-60 | **Volcanic Depths** | Lava chambers, fire elementals, dragons |
| 61-70 | **Ancient Ruins** | Forgotten temples, golems, sphinxes |
| 71-80 | **Crystal Caves** | Gem-encrusted caverns, crystal golems |
| 81-90 | **Void Realm** | Dimensional rifts, aberrations |
| 91-100 | **Demon Realm** | Hellish domain of the Demon King |

### Special Floors

- **Boss Floors:** 10, 20, 30, 40, 50, 60, 70, 80, 90, 100
- **Mini-Boss Floors:** 5, 15, 25, 35, 45, 55, 65, 75, 85, 95
- **Secret Floors:** 10 hidden floors accessible via portals
- **Safe Rooms:** Occasional rest areas with shops

### Floor Features

| Feature | Symbol | Effect |
|---------|--------|--------|
| Stairs Down | `>` | Descend to next floor |
| Stairs Up | `<` | Return to previous floor |
| Door | `+` | Must be opened to pass |
| Trap | `^` | Triggers damage |
| Water | `~` (blue) | Slows movement |
| Lava | `~` (red) | Damages when crossed |
| Chest | `=` | Contains loot |
| Shrine | `&` | Grants blessings (one use) |
| Portal | `O` | Teleportation point |
| Boss Gate | `8` | Entrance to boss arena |

---

## Enemies & Bosses

### Enemy Tiers

**Tier 1-2 (Floors 1-20)**
- Rats, Bats, Spiders, Goblins, Kobolds
- Orcs, Trolls, Slimes, Rock Elementals

**Tier 3-4 (Floors 21-40)**
- Zombies, Ghosts, Vampires, Mummies
- Wolves, Ents, Forest Spirits, Corrupted Druids

**Tier 5-6 (Floors 41-60)**
- Ice Elementals, Frost Giants, Yetis
- Fire Elementals, Lava Golems, Fire Drakes

**Tier 7-8 (Floors 61-80)**
- Ancient Guardians, Golems, Liches
- Crystal Constructs, Void Creatures

**Tier 9-10 (Floors 81-100)**
- Demons, Balrogs, Pit Fiends
- Shadow Lords, Abyssal Horrors, Demon King

### Boss Encounters

| Floor | Boss | Difficulty |
|-------|------|------------|
| 10 | Goblin King | Easy |
| 20 | Orc Warlord | Medium |
| 30 | Vampire Lord | Medium |
| 40 | Forest Guardian | Medium-Hard |
| 50 | Ice Dragon | Hard |
| 60 | Infernal Lord | Hard |
| 70 | Ancient Sphinx | Very Hard |
| 80 | Crystal Titan | Very Hard |
| 90 | Void Overlord | Extreme |
| 100 | Demon King | Ultimate |

---

## Items & Equipment

### Item Rarities

| Rarity | Stat Multiplier | Drop Chance |
|--------|-----------------|-------------|
| Common | 1.0x | 50% |
| Uncommon | 1.25x | 25% |
| Rare | 1.5x | 15% |
| Epic | 2.0x | 7% |
| Legendary | 3.0x | 2.5% |
| Mythic | 5.0x | 0.5% |

### Weapon Types (25+)

**Melee:** Dagger, Sword, Greatsword, Axe, Mace, Spear, Scythe
**Ranged:** Bow, Crossbow, Throwing Weapons
**Magic:** Staff, Wand, Orb, Grimoire
**Elemental:** Flame Sword, Frost Blade, Thunder Axe

### Item Sets (15)

| Set | Pieces | Theme |
|-----|--------|-------|
| Dragon Slayer | 5 | Anti-dragon combat |
| Titan's Might | 4 | Raw strength |
| Arcane Scholar | 5 | Magic power |
| Shadow Dancer | 4 | Stealth and agility |
| Paladin's Valor | 5 | Holy defense |
| Death Knight | 5 | Dark melee |
| Phoenix Rebirth | 4 | Fire and revival |
| Demon Lord | 5 | Demonic power |
| Celestial Guard | 5 | Divine protection |

### Legendary Items

- **Excalibur** - Holy sword with demon slaying
- **Mjolnir** - Lightning hammer with chain damage
- **Masamune** - Perfect katana with critical bonuses
- **Ring of Omniscience** - Reveals all secrets
- **Crown of Eternals** - Massive stat bonuses

---

## Progression Systems

### Experience & Leveling

- Level cap: 100+
- Experience scales with enemy difficulty
- Bonus XP from quests, discoveries, achievements

### Cultivation System (Xianxia)

8 cultivation realms for spiritual advancement:

| Realm | Level | Benefits |
|-------|-------|----------|
| Mortal | 1-10 | Starting realm |
| Qi Condensation | 11-20 | +Qi abilities |
| Foundation | 21-30 | +Stat bonuses |
| Core Formation | 31-40 | +Core abilities |
| Nascent Soul | 41-50 | +Soul powers |
| Spirit Severing | 51-70 | +Advanced powers |
| Dao Seeking | 71-90 | +Dao techniques |
| Immortal | 91+ | Ultimate power |

### Skill Trees

Multiple talent trees with branching paths:
- Element-based trees (Fire, Ice, etc.)
- Combat trees (Offense, Defense, Utility)
- Class-specific trees
- Cultivation trees

---

## Crafting & Professions

### 18 Professions (Max 4 Active)

**Gathering:**
| Profession | Resources |
|------------|-----------|
| Mining | Ores, gems, stones |
| Herbalism | Plants, herbs, flowers |
| Skinning | Leather, hides, scales |
| Fishing | Fish, treasures |
| Woodcutting | Wood, bark, sap |
| Hunting | Meat, bones, materials |

**Crafting:**
| Profession | Products |
|------------|----------|
| Blacksmithing | Weapons, armor |
| Alchemy | Potions, pills |
| Inscription | Scrolls, runes |
| Tailoring | Cloth armor, bags |
| Jewelcrafting | Rings, amulets |
| Cooking | Food, buffs |
| Enchanting | Equipment upgrades |
| Formation Crafting | Tactical formations |

**Service:**
| Profession | Abilities |
|------------|-----------|
| Merchant | Trading bonuses |
| Appraiser | Item identification |
| Cartographer | Map creation |
| Tamer | Beast taming |

### Alchemy Products (50+)

- Health/Mana potions
- Breakthrough pills (cultivation)
- Attribute enhancement pills
- Elemental affinity elixirs
- Body tempering formulas

---

## Companions & Familiars

### Companion System

- Up to 5 active companions
- AI-controlled party members
- Leveling and bonding system
- Morale and loyalty mechanics

### Familiar Types (50+)

| Category | Examples |
|----------|----------|
| Combat | Wolf, Dragon, Imp, Tiger |
| Support | Fairy, Owl, Healing Sprite |
| Utility | Raccoon, Mole, Crow |
| Legendary | Mini Dragon, Unicorn, Phoenix |

Features:
- Evolution paths
- Equipment (collars, armor)
- Breeding with trait inheritance

---

## Advanced Combat Systems

### Summoning System

**50+ summon types** across 6 categories:
- Elemental, Beast, Spirit, Demonic, Celestial, Mythical

**Summoner Ranks:** Apprentice → Journeyman → Summoner → Master → Arch → Planar Lord

### Necromancy System

**30+ undead types** with army management:
- Skeletons, Zombies, Ghosts, Vampires, Liches
- Soul gem storage
- Phylactery mechanics

### Martial Arts System

**30+ martial styles** in 6 categories:
- Northern (kicks), Southern (hands)
- Internal (chi), External (power)
- Weapon styles, Legendary techniques

Features: Chi/Ki energy, combo system (10 hits), tournaments

### Formation System

7 tactical formations: Standard, Defensive, Aggressive, Flanking, Pincer, Phalanx, Skirmish

---

## Social Systems

### Guild System (12 Guilds)

| Guild | Focus | Perks |
|-------|-------|-------|
| Blades Brotherhood | Warriors | +Attack, combat quests |
| Arcanum Circle | Mages | +Mana, spell research |
| Shadow Syndicate | Thieves | +Stealth, heist missions |
| Golden Coin Trading | Merchants | +Gold, trade routes |
| Beast Slayers League | Hunters | +vs Beasts, trophies |

### Kingdom System (8 Kingdoms)

- Valdoria (Human), Sylvaneth (Elf), Ironhold (Dwarf)
- Grommash (Orc), Necropolis (Undead), Infernium (Demon)
- Wildlands (Beastkin), Celestia (Divine)

Diplomatic states: Allied → Friendly → Neutral → Unfriendly → Hostile → At War

### Relationship System

7 levels with NPCs: Hostile → Unfriendly → Neutral → Friendly → Trusted → Beloved → Soulbound

Benefits: Shop discounts, quests, romance options

---

## Minigames

19 unique minigames with difficulty scaling:

| Minigame | Type | Rewards |
|----------|------|---------|
| Lockpicking | Tumbler puzzle | Chest loot |
| Fishing | Timing | Fish, treasures |
| Mining | Rhythm | Ores, gems |
| Alchemy | Precision | Potions |
| Blacksmithing | Temperature | Equipment |
| Card Game | Strategy | Gold, items |
| Racing | Mount racing | Prizes |
| Arena Combat | PvE/PvP | Rankings |

---

## Arena System

### Arena Types (9)

| Arena | Format | Rewards |
|-------|--------|---------|
| Duel | 1v1 | Arena points |
| Team | 3v3, 5v5 | Team rewards |
| Battle Royale | Free-for-all | Champion title |
| Tournament | Elimination | Grand prizes |
| Ranked Ladder | Competitive | Seasonal rewards |

### Rankings

Bronze → Silver → Gold → Platinum → Diamond → Master → Grandmaster → Legend

---

## Economic System

### Currencies (11 Types)

| Currency | Use |
|----------|-----|
| Gold | Standard trading |
| Spirit Stones | Cultivation, high-end items |
| Guild Points | Guild rewards |
| Arena Points | Arena shop |
| Dungeon Tokens | Special vendors |

### Trading Features

- NPC shops and bartering
- Auction house with bidding
- Trade routes with regional pricing
- Traveling merchants
- Black market dealers

---

## Story & Adventure

### Main Story (10 Acts)

| Act | Floors | Title |
|-----|--------|-------|
| 1 | 1-10 | The Awakening |
| 2 | 11-20 | The Gathering Storm |
| 3 | 21-30 | Descent into Darkness |
| 4 | 31-40 | The Burning Path |
| 5 | 41-50 | Frozen Memories |
| 6 | 51-60 | Nature's Wrath |
| 7 | 61-70 | Crystal Dreams |
| 8 | 71-80 | Between Worlds |
| 9 | 81-90 | Divine Conflict |
| 10 | 91-100 | The Final Reckoning |

---

## Game Modes

| Mode | Description |
|------|-------------|
| **Story Mode** | Full 100-floor adventure with narrative |
| **Endless Dungeon** | Infinite scaling floors |
| **Boss Rush** | Back-to-back boss fights |
| **Daily Challenge** | Seeded daily run with leaderboard |
| **Weekly Challenge** | Harder modifiers and rewards |
| **Randomizer** | Everything randomized |
| **Arena Mode** | Pure combat focus |
| **Survival** | Limited resources, permadeath |

---

## Additional Systems

### Weather System
- Dynamic weather affecting combat
- Elemental interactions
- Day/night cycles

### Mount System (8 Types)
Horse, Wolf, Bear, Drake, Griffin, Phoenix, Nightmare, Dragon

### Housing System
- Player homes with customization
- Storage, crafting stations
- NPC quarters

### Achievement System
- 50+ achievements across categories
- Cosmetic rewards and titles

---

## Incomplete & Coming Soon

The following systems are planned or in development:

### Partially Implemented

| System | Status | Description |
|--------|--------|-------------|
| **Multiplayer** | Planned | Co-op dungeon crawling |
| **PvP Arena** | In Progress | Player vs Player battles |
| **Leaderboards** | In Progress | Global rankings |
| **Cloud Saves** | Planned | Cross-device progression |
| **Modding Support** | Planned | Custom content creation |

### Coming Soon

| Feature | Priority | Description |
|---------|----------|-------------|
| **New Species** | High | Additional playable races |
| **Class Rebalancing** | High | Balance adjustments |
| **More Bosses** | Medium | Additional boss encounters |
| **New Themes** | Medium | Additional dungeon themes |
| **Voice Acting** | Low | Narrative voice overs |
| **Achievements** | Medium | Steam/platform achievements |
| **New Game+** | High | Replay with carried progress |
| **Procedural Quests** | Medium | Randomly generated quests |
| **Seasonal Events** | Medium | Holiday-themed content |
| **Mobile Port** | Low | iOS/Android versions |

### Known Issues

- Some skill tree paths not fully balanced
- Certain rare items may not spawn correctly
- AI pathfinding optimization needed for large groups
- Some minigames need difficulty tuning

### Contribution Areas

We welcome contributions in these areas:
- Bug fixes and optimization
- Balance suggestions
- New content (items, enemies, spells)
- Localization/translations
- Documentation improvements

---

## Game Statistics

| Category | Count |
|----------|-------|
| Dungeon Floors | 100 + 10 secret |
| Enemy Types | 60+ |
| Spells | 150+ |
| Martial Styles | 30+ |
| Summon Types | 50+ |
| Species/Subspecies | 12 + 48 |
| Specializations | 72+ |
| Professions | 18 |
| Familiars | 50+ |
| Minigames | 19 |
| Item Sets | 15 |
| Guilds | 12 |
| Kingdoms | 8 |
| Game Modes | 10 |

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## Version

- **Version:** 0.2.0
- **Engine:** Rust with crossterm (CLI) / Custom renderer (GUI)
- **Repository:** [github.com/quantumsequrity/ShadowCrypt](https://github.com/quantumsequrity/ShadowCrypt)

---

*Descend into the depths. Ascend to immortality.*

*Good luck, adventurer. The shadows await.*
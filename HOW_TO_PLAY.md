# How to Play ShadowCrypt

This guide will teach you everything you need to know to survive the depths of ShadowCrypt.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Controls](#controls)
3. [Understanding the Interface](#understanding-the-interface)
4. [Combat](#combat)
5. [Status Effects](#status-effects)
6. [Items and Equipment](#items-and-equipment)
7. [Survival Tips](#survival-tips)
8. [Boss Fight Strategies](#boss-fight-strategies)
9. [Winning the Game](#winning-the-game)

---

## Getting Started

### Launching the Game

After building the game (see HOW_TO_INSTALL.md), run the executable to start your adventure.

### Character Selection

When you start a new game, you will be prompted to choose one of six character classes. Each class has different strengths, weaknesses, and a unique special ability.

#### Class Overview

| Class | Playstyle | Difficulty |
|-------|-----------|------------|
| **Warrior** | Tank and melee damage dealer | Easy |
| **Mage** | High damage spellcaster, fragile | Medium |
| **Rogue** | Fast, high critical damage, low HP | Medium |
| **Paladin** | Balanced fighter with healing | Easy |
| **Ranger** | Ranged combat specialist | Medium |
| **Necromancer** | Summoner with dark magic | Hard |

**Recommended for Beginners:** Warrior or Paladin. Both have high HP and good survivability while you learn the game mechanics.

### Your First Floor

When you spawn on Floor 1, you will see:
- Your character represented by `@`
- Dungeon walls as `#`
- Floor tiles as `.`
- Doors as `+`
- Stairs down as `>`

Your immediate goals:
1. Explore the floor
2. Pick up any items you find
3. Defeat enemies for experience
4. Find the stairs down to proceed

---

## Controls

### Movement

| Key | Direction |
|-----|-----------|
| `W` or `Up Arrow` | Move North |
| `S` or `Down Arrow` | Move South |
| `A` or `Left Arrow` | Move West |
| `D` or `Right Arrow` | Move East |
| `Q` | Move Northwest (diagonal) |
| `E` | Move Northeast (diagonal) |
| `Z` | Move Southwest (diagonal) |
| `C` | Move Southeast (diagonal) |
| `Space` | Wait (skip turn) |

**Tip:** Diagonal movement is essential for escaping enemies and positioning for backstabs (Rogue).

### Interaction

| Key | Action |
|-----|--------|
| `G` | Pick up item on current tile |
| `>` | Descend stairs (when standing on `>`) |
| `<` | Ascend stairs (when standing on `<`) |
| Move into door | Open door |
| Move into enemy | Attack enemy |
| Move into chest | Open chest |

### Inventory and Equipment

| Key | Action |
|-----|--------|
| `I` | Open inventory screen |
| `U` | Use selected item |
| `P` | Drop selected item |
| `Tab` | View/manage equipment |

When in the inventory screen:
- Use arrow keys to navigate items
- Press the corresponding letter to select an item
- Press `Esc` to close inventory

### Skills

| Key | Action |
|-----|--------|
| `1` | Use Skill 1 (Class ability) |
| `2` | Use Skill 2 |
| `3` | Use Skill 3 |
| `4` | Use Skill 4 |

Skills consume mana. Make sure you have enough mana before attempting to use a skill.

### Other

| Key | Action |
|-----|--------|
| `M` | View message log |
| `?` | Display help |
| `Esc` | Open menu / Cancel action |

---

## Understanding the Interface

### The Map

| Symbol | Meaning | Notes |
|--------|---------|-------|
| `@` | Your character | Always at the center of your view |
| `#` | Wall | Cannot be passed |
| `.` | Floor | Safe to walk on |
| `+` | Closed door | Move into it to open |
| `'` | Open door | Can walk through |
| `>` | Stairs down | Press `>` to descend |
| `<` | Stairs up | Press `<` to ascend |
| `^` | Trap | Avoid or triggers damage |
| `~` | Water/Lava | Blue = water, Red = lava (dangerous!) |
| `=` | Chest | Contains items |
| `&` | Shrine | Use for a blessing (one time) |
| `8` | Boss Gate | Leads to boss encounter |
| `O` | Pillar | Blocks movement and sight |

### Enemy Symbols

Enemies are typically represented by letters:
- Lowercase letters = weaker enemies
- Uppercase letters = stronger enemies
- Colored letters indicate enemy type

Examples:
- `r` = Rat
- `g` = Goblin
- `S` = Skeleton
- `D` = Demon

### Item Symbols

| Symbol | Item Type |
|--------|-----------|
| `!` | Potion |
| `?` | Scroll |
| `/` | Sword/Staff |
| `\|` | Dagger/Spear |
| `}` | Bow/Crossbow |
| `)` | Shield |
| `[` | Armor |
| `^` | Helmet |
| `{` | Gloves |
| `o` | Ring |
| `"` | Amulet |
| `%` | Food |
| `$` | Gold |
| `k` | Key |
| `*` | Bomb/Relic |

### Status Bar

The bottom of the screen displays:
- **HP:** Your current health / maximum health
- **MP:** Your current mana / maximum mana
- **Lvl:** Your character level
- **Floor:** Current dungeon floor
- **Hunger:** Your hunger status
- **Effects:** Active status effects

---

## Combat

### Basic Combat

Combat in ShadowCrypt is turn-based:
1. You take your turn (move, attack, use item)
2. All enemies take their turns
3. Repeat

To attack an enemy, simply move into them. Your damage calculation:
```
Damage = (Your Attack + Weapon Attack) - (Enemy Defense)
```

### Attack Positioning

Some classes benefit from positioning:
- **Rogue:** Backstab deals 3x damage. Position yourself behind enemies.
- **Ranger:** Keep distance and use ranged attacks.

### Combat Tips

1. **Corridor Fighting** - Fight in corridors to prevent being surrounded
2. **Kiting** - Attack, then retreat, then attack again
3. **Use Doors** - Close doors to break line of sight
4. **Wait for Enemies** - Let them come to you in favorable terrain
5. **Save Skills for Emergencies** - Don't waste mana on weak enemies

### Enemy Behavior

- Enemies will chase you if they see you
- Some enemies are ranged attackers
- Bosses have special attack patterns
- Undead take extra damage from holy attacks (Paladin)

---

## Status Effects

### Negative Effects (Remove ASAP)

| Effect | Color | What It Does | How to Remove |
|--------|-------|--------------|---------------|
| **Poisoned** | Green | Lose HP each turn | Antidote, Cure All Elixir |
| **Burning** | Red | Fire damage over time | Wait it out, water |
| **Frozen** | Cyan | Cannot move | Wait it out, fire damage |
| **Bleeding** | Dark Red | Continuous HP loss | Healing, bandages |
| **Stunned** | Yellow | Skip your turn | Wait it out |
| **Blind** | Dark Grey | Reduced vision | Wait it out, Potion of True Sight |
| **Weakened** | Dark Magenta | Reduced attack | Wait it out, Strength Potion |
| **Confused** | Dark Yellow | Random movement | Wait it out |

### Positive Effects (Maintain These)

| Effect | Color | What It Does | How to Get |
|--------|-------|--------------|------------|
| **Haste** | Blue | Faster actions | Speed Potion |
| **Shielded** | White | Damage reduction | Defense Potion, scrolls |
| **Regenerating** | Magenta | Heal HP over time | Regeneration Potion, Ring |
| **Strengthened** | Yellow | Increased attack | Strength Potion |
| **Invisible** | Grey | Enemies cannot see you | Invisibility Potion, Ring |

---

## Items and Equipment

### Equipment Slots

You can equip one item in each slot:
- Head (Helmet)
- Body (Armor)
- Hands (Gloves)
- Feet (Boots)
- Main Hand (Weapon)
- Off-Hand (Shield)
- Ring 1
- Ring 2
- Neck (Amulet)

### Item Rarity

Items have different rarities that affect their power:

| Rarity | Prefix | Power Level | How Common |
|--------|--------|-------------|------------|
| Common | (none) | Base stats | Very common |
| Uncommon | Fine | +25% stats | Common |
| Rare | Superior | +50% stats | Uncommon |
| Epic | Epic | +100% stats | Rare |
| Legendary | Legendary | +200% stats | Very rare |
| Mythic | Mythic | +400% stats | Extremely rare |

**Always compare items!** A Rare (+50%) Iron Sword may be better than a Common Steel Sword.

### Consumables

#### Potions (Use with `U`)

| Potion | Effect |
|--------|--------|
| Health Potion | Restore HP |
| Mana Potion | Restore MP |
| Strength Potion | Temporary attack boost |
| Defense Potion | Temporary defense boost |
| Speed Potion | Temporary haste |
| Invisibility Potion | Become invisible briefly |
| Antidote | Cure poison |
| Full Restore Elixir | Fully restore HP and MP |

#### Scrolls (Use with `U`)

| Scroll | Effect |
|--------|--------|
| Scroll of Teleport | Random teleport on floor |
| Scroll of Mapping | Reveal entire floor |
| Scroll of Identify | Identify unknown items |
| Scroll of Fireball | AoE fire damage |
| Scroll of Ice Storm | AoE ice damage + freeze |
| Scroll of Lightning | High single-target damage |
| Scroll of Mass Heal | Heal all allies |

### Food

**Critical for survival!** Food items restore hunger:

| Food | Hunger Restored |
|------|-----------------|
| Apple | 10 |
| Cheese | 20 |
| Bread | 25 |
| Meat | 40 |
| Royal Feast | 100 |

Hunger warnings:
- "You feel hungry" = 50% hunger
- "You are starving!" = 25% hunger
- "You are dying of starvation!" = Taking damage

---

## Survival Tips

### General Tips

1. **Always carry food** - Keep at least 2-3 food items in reserve
2. **Don't fight everything** - Sometimes running is the best option
3. **Use stairs tactically** - Retreat up stairs if overwhelmed
4. **Check every room** - Chests and shrines provide valuable resources
5. **Identify item types** - Once you know a potion color, you know all of that type

### Resource Management

- **Health Potions** - Save for emergencies, use healing at shrines first
- **Mana Potions** - Keep 2-3 for boss fights
- **Scrolls** - Save powerful scrolls (Meteor, Death) for bosses
- **Gold** - May be used at shops (if present) for better equipment

### Avoiding Death

1. **Watch your HP** - Heal before you get critical
2. **Know enemy patterns** - Learn which enemies are dangerous
3. **Mind your surroundings** - Don't get surrounded
4. **Check for traps** - Move carefully in unexplored areas
5. **Use the environment** - Lure enemies through lava, close doors

### Floor-by-Floor Advice

- **Floors 1-4:** Build your basics. Find a weapon and armor.
- **Floors 5-8:** Start collecting potions. First boss at floor 5.
- **Floors 9-12:** Undead are weak to holy damage (Paladin advantage).
- **Floors 13-16:** Forest enemies can poison. Carry antidotes.
- **Floors 17-20:** Ice enemies freeze you. Keep moving.
- **Floors 21-24:** Fire everywhere. Fire resist potions are valuable.
- **Floors 25-28:** Ancient enemies hit hard. Full preparation needed.
- **Floors 29-30:** The Demon Realm. Use everything you have saved.

---

## Boss Fight Strategies

### Goblin King (Floor 5)
- **Difficulty:** Easy
- **Strategy:** Standard melee combat. Watch for his minions.
- **Tips:** Clear the room of lesser goblins first.

### Orc Warlord (Floor 10)
- **Difficulty:** Medium
- **Strategy:** He hits hard. Use defense potions and kiting.
- **Tips:** Corridor fighting limits his mobility.

### Vampire Lord (Floor 15)
- **Difficulty:** Medium-Hard
- **Strategy:** He heals from his attacks. Burst him down quickly.
- **Tips:** Holy Light (Paladin) is very effective. Avoid prolonged fights.

### Forest Guardian (Floor 20)
- **Difficulty:** Hard
- **Strategy:** High HP, can summon forest allies.
- **Tips:** Kill summons quickly or focus the boss with AoE.

### Ice Dragon (Floor 25)
- **Difficulty:** Very Hard
- **Strategy:** Freezing breath attacks. Keep fire resist and cure items.
- **Tips:** Stay mobile to avoid freeze, use fire damage if available.

### Demon King (Floor 30) - Final Boss
- **Difficulty:** Extreme
- **Strategy:** Multiple phases, summons demons, uses all elements.
- **Tips:**
  - Use your best equipment
  - Bring all your saved scrolls and potions
  - Focus on dodging his special attacks
  - Use Invisibility to reset the fight if needed
  - Consider using Time Stop scroll for free damage

---

## Winning the Game

### Victory Conditions

To win ShadowCrypt, you must:
1. Descend through all 30 floors
2. Defeat the Demon King on Floor 30

### After Victory

Upon defeating the Demon King:
- Your final stats are displayed
- Your journey is recorded
- You may start a new game with a different class

### Challenge Runs

After winning, try these challenges:
- **Speed Run:** Complete the game as fast as possible
- **Pacifist:** Avoid killing non-boss enemies
- **Hardcore:** Never use shrines or healing items
- **Class Master:** Beat the game with every class

---

## Quick Reference Card

```
MOVEMENT          ACTIONS           SKILLS
W - Up            G - Pick up       1-4 - Use skill
S - Down          > - Go down
A - Left          < - Go up         MENU
D - Right         I - Inventory     Esc - Menu
Q/E - Diagonals   U - Use item      ? - Help
Z/C - Diagonals   P - Drop item     M - Messages
Space - Wait      Tab - Equipment
```

---

*May your blade stay sharp and your torches never dim.*

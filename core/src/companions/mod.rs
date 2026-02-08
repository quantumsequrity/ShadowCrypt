//! Companion/Pet system: types, AI behavior, leveling, and abilities
//!
//! This module provides a complete companion system for the ShadowCrypt roguelike.
//! Players can find, recruit, and level up various companion creatures that fight
//! alongside them in the dungeon.

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::combat::StatusEffect;
use crate::entities::Enemy;
use crate::world::Map;

/// Maximum number of companions a player can have at once
pub const MAX_COMPANIONS: usize = 3;

/// Experience required for each companion level
pub const XP_PER_LEVEL: [u32; 20] = [
    50, 100, 175, 275, 400, 550, 725, 925, 1150, 1400,
    1700, 2050, 2450, 2900, 3400, 3950, 4550, 5200, 5900, 6650,
];

/// All companion types available in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CompanionKind {
    // Beasts
    Wolf,
    DireWolf,
    Bear,
    Panther,
    GiantEagle,

    // Magical Creatures
    Sprite,
    Wisp,
    FaeDragon,
    Phoenix,
    Golem,

    // Undead (for Necromancers)
    SkeletonWarrior,
    Ghost,
    Wraith,

    // Elemental
    FireSprite,
    IceElemental,
    StormElemental,
    EarthElemental,

    // Humanoid
    GoblinRogue,
    OrcWarrior,
    ImpServant,

    // Rare/Legendary
    DragonWhelp,
    Unicorn,
    DemonHound,
    AngelicGuardian,
}

impl CompanionKind {
    /// Returns the display name of this companion type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Wolf => "Wolf",
            Self::DireWolf => "Dire Wolf",
            Self::Bear => "Bear",
            Self::Panther => "Shadow Panther",
            Self::GiantEagle => "Giant Eagle",
            Self::Sprite => "Forest Sprite",
            Self::Wisp => "Will-o'-Wisp",
            Self::FaeDragon => "Fae Dragon",
            Self::Phoenix => "Phoenix",
            Self::Golem => "Stone Golem",
            Self::SkeletonWarrior => "Skeleton Warrior",
            Self::Ghost => "Spectral Ghost",
            Self::Wraith => "Shadow Wraith",
            Self::FireSprite => "Fire Sprite",
            Self::IceElemental => "Ice Elemental",
            Self::StormElemental => "Storm Elemental",
            Self::EarthElemental => "Earth Elemental",
            Self::GoblinRogue => "Goblin Rogue",
            Self::OrcWarrior => "Orc Warrior",
            Self::ImpServant => "Imp Servant",
            Self::DragonWhelp => "Dragon Whelp",
            Self::Unicorn => "Unicorn",
            Self::DemonHound => "Demon Hound",
            Self::AngelicGuardian => "Angelic Guardian",
        }
    }

    /// Returns the glyph character for this companion
    pub fn glyph(&self) -> char {
        match self {
            Self::Wolf | Self::DireWolf => 'w',
            Self::Bear => 'B',
            Self::Panther => 'p',
            Self::GiantEagle => 'E',
            Self::Sprite | Self::FireSprite => 's',
            Self::Wisp => 'o',
            Self::FaeDragon | Self::DragonWhelp => 'd',
            Self::Phoenix => 'P',
            Self::Golem => 'G',
            Self::SkeletonWarrior => 'S',
            Self::Ghost => 'g',
            Self::Wraith => 'W',
            Self::IceElemental | Self::StormElemental | Self::EarthElemental => 'e',
            Self::GoblinRogue => 'r',
            Self::OrcWarrior => 'O',
            Self::ImpServant => 'i',
            Self::Unicorn => 'U',
            Self::DemonHound => 'H',
            Self::AngelicGuardian => 'A',
        }
    }

    /// Returns a color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Wolf | Self::DireWolf => 1,      // Grey
            Self::Bear => 12,                       // Brown/DarkYellow
            Self::Panther => 0,                     // DarkGrey
            Self::GiantEagle => 11,                 // Yellow
            Self::Sprite => 5,                      // Green
            Self::Wisp => 9,                        // Cyan
            Self::FaeDragon => 13,                  // Magenta
            Self::Phoenix => 3,                     // Red
            Self::Golem => 1,                       // Grey
            Self::SkeletonWarrior => 2,             // White
            Self::Ghost => 9,                       // Cyan
            Self::Wraith => 14,                     // DarkMagenta
            Self::FireSprite => 3,                  // Red
            Self::IceElemental => 9,                // Cyan
            Self::StormElemental => 7,              // Blue
            Self::EarthElemental => 12,             // Brown
            Self::GoblinRogue => 5,                 // Green
            Self::OrcWarrior => 6,                  // DarkGreen
            Self::ImpServant => 3,                  // Red
            Self::DragonWhelp => 11,                // Yellow
            Self::Unicorn => 2,                     // White
            Self::DemonHound => 4,                  // DarkRed
            Self::AngelicGuardian => 11,            // Yellow/Gold
        }
    }

    /// Returns base stats: (hp, attack, defense, speed)
    pub fn base_stats(&self) -> (i32, i32, i32, i32) {
        match self {
            // Beasts - balanced physical stats
            Self::Wolf => (25, 8, 2, 12),
            Self::DireWolf => (45, 14, 4, 10),
            Self::Bear => (70, 12, 8, 6),
            Self::Panther => (30, 15, 2, 14),
            Self::GiantEagle => (35, 12, 3, 15),

            // Magical Creatures - varied specialties
            Self::Sprite => (20, 6, 1, 16),
            Self::Wisp => (15, 4, 0, 18),
            Self::FaeDragon => (40, 14, 5, 12),
            Self::Phoenix => (60, 18, 6, 14),
            Self::Golem => (100, 10, 15, 4),

            // Undead - moderate with special abilities
            Self::SkeletonWarrior => (35, 10, 6, 8),
            Self::Ghost => (25, 12, 0, 12),
            Self::Wraith => (30, 16, 2, 10),

            // Elemental - element-focused
            Self::FireSprite => (25, 14, 2, 14),
            Self::IceElemental => (45, 10, 8, 8),
            Self::StormElemental => (35, 16, 4, 12),
            Self::EarthElemental => (80, 8, 12, 5),

            // Humanoid - tactical
            Self::GoblinRogue => (25, 12, 2, 14),
            Self::OrcWarrior => (55, 14, 6, 7),
            Self::ImpServant => (20, 8, 1, 16),

            // Rare/Legendary - powerful
            Self::DragonWhelp => (50, 20, 8, 10),
            Self::Unicorn => (55, 12, 6, 14),
            Self::DemonHound => (45, 22, 5, 12),
            Self::AngelicGuardian => (70, 15, 12, 10),
        }
    }

    /// Returns the rarity tier of this companion (affects drop rate and power)
    pub fn rarity(&self) -> CompanionRarity {
        match self {
            Self::Wolf | Self::Sprite | Self::SkeletonWarrior | Self::GoblinRogue
            | Self::ImpServant => CompanionRarity::Common,

            Self::DireWolf | Self::Bear | Self::Wisp | Self::Ghost | Self::FireSprite
            | Self::IceElemental | Self::OrcWarrior => CompanionRarity::Uncommon,

            Self::Panther | Self::GiantEagle | Self::FaeDragon | Self::Golem
            | Self::Wraith | Self::StormElemental | Self::EarthElemental => CompanionRarity::Rare,

            Self::Phoenix | Self::DragonWhelp | Self::DemonHound => CompanionRarity::Epic,

            Self::Unicorn | Self::AngelicGuardian => CompanionRarity::Legendary,
        }
    }

    /// Returns the special ability this companion type has
    pub fn ability(&self) -> CompanionAbility {
        match self {
            Self::Wolf | Self::DireWolf => CompanionAbility::PackHowl,
            Self::Bear => CompanionAbility::MaulAttack,
            Self::Panther => CompanionAbility::Ambush,
            Self::GiantEagle => CompanionAbility::DiveBomb,
            Self::Sprite | Self::Unicorn => CompanionAbility::NatureHeal,
            Self::Wisp => CompanionAbility::Illuminate,
            Self::FaeDragon => CompanionAbility::FaeBreath,
            Self::Phoenix => CompanionAbility::Rebirth,
            Self::Golem | Self::EarthElemental => CompanionAbility::Taunt,
            Self::SkeletonWarrior => CompanionAbility::ShieldBash,
            Self::Ghost => CompanionAbility::Terrify,
            Self::Wraith => CompanionAbility::LifeDrain,
            Self::FireSprite => CompanionAbility::FlameAura,
            Self::IceElemental => CompanionAbility::FrostNova,
            Self::StormElemental => CompanionAbility::LightningStrike,
            Self::GoblinRogue => CompanionAbility::PoisonStab,
            Self::OrcWarrior => CompanionAbility::BattleCry,
            Self::ImpServant => CompanionAbility::FireBolt,
            Self::DragonWhelp => CompanionAbility::BreathWeapon,
            Self::DemonHound => CompanionAbility::Hellfire,
            Self::AngelicGuardian => CompanionAbility::DivineProtection,
        }
    }

    /// Returns whether this companion can be found in the given dungeon level range
    pub fn available_at_level(&self, level: u32) -> bool {
        match self.rarity() {
            CompanionRarity::Common => level >= 1,
            CompanionRarity::Uncommon => level >= 5,
            CompanionRarity::Rare => level >= 10,
            CompanionRarity::Epic => level >= 18,
            CompanionRarity::Legendary => level >= 25,
        }
    }

    /// Returns a random companion appropriate for the dungeon level
    pub fn random_for_level(level: u32, rng: &mut impl Rng) -> Self {
        let available: Vec<Self> = [
            Self::Wolf, Self::DireWolf, Self::Bear, Self::Panther, Self::GiantEagle,
            Self::Sprite, Self::Wisp, Self::FaeDragon, Self::Phoenix, Self::Golem,
            Self::SkeletonWarrior, Self::Ghost, Self::Wraith,
            Self::FireSprite, Self::IceElemental, Self::StormElemental, Self::EarthElemental,
            Self::GoblinRogue, Self::OrcWarrior, Self::ImpServant,
            Self::DragonWhelp, Self::Unicorn, Self::DemonHound, Self::AngelicGuardian,
        ].into_iter()
            .filter(|k| k.available_at_level(level))
            .collect();

        available[rng.gen_range(0..available.len())]
    }

    /// Returns all companion kinds as a vec
    pub fn all() -> Vec<Self> {
        vec![
            Self::Wolf, Self::DireWolf, Self::Bear, Self::Panther, Self::GiantEagle,
            Self::Sprite, Self::Wisp, Self::FaeDragon, Self::Phoenix, Self::Golem,
            Self::SkeletonWarrior, Self::Ghost, Self::Wraith,
            Self::FireSprite, Self::IceElemental, Self::StormElemental, Self::EarthElemental,
            Self::GoblinRogue, Self::OrcWarrior, Self::ImpServant,
            Self::DragonWhelp, Self::Unicorn, Self::DemonHound, Self::AngelicGuardian,
        ]
    }
}

/// Rarity tiers for companions
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum CompanionRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl CompanionRarity {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
        }
    }

    /// Returns color index for UI
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Common => 1,      // Grey
            Self::Uncommon => 5,    // Green
            Self::Rare => 7,        // Blue
            Self::Epic => 13,       // Magenta
            Self::Legendary => 11,  // Yellow
        }
    }

    /// Returns the stat multiplier for this rarity
    pub fn stat_bonus(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.15,
            Self::Rare => 1.3,
            Self::Epic => 1.5,
            Self::Legendary => 1.8,
        }
    }

    /// Returns the chance to find a companion of this rarity (per 1000)
    pub fn spawn_chance(&self) -> u32 {
        match self {
            Self::Common => 100,     // 10%
            Self::Uncommon => 50,    // 5%
            Self::Rare => 25,        // 2.5%
            Self::Epic => 10,        // 1%
            Self::Legendary => 3,    // 0.3%
        }
    }
}

/// Special abilities companions can use
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum CompanionAbility {
    // Offensive
    PackHowl,         // Buff allies attack
    MaulAttack,       // Heavy damage + bleed
    Ambush,           // Double damage from stealth
    DiveBomb,         // Ranged attack
    BreathWeapon,     // AoE elemental damage
    LightningStrike,  // Chain damage
    FaeBreath,        // Confusion effect
    Hellfire,         // Burn + damage
    PoisonStab,       // Poison effect
    FireBolt,         // Ranged fire attack

    // Defensive
    Taunt,            // Force enemies to attack companion
    ShieldBash,       // Stun enemy
    BattleCry,        // Buff ally defense
    DivineProtection, // Shield ally
    Rebirth,          // Self-resurrect once

    // Utility
    NatureHeal,       // Heal player
    Illuminate,       // Reveal area
    Terrify,          // Fear enemies (freeze)
    LifeDrain,        // Damage + heal self
    FlameAura,        // Damage nearby enemies
    FrostNova,        // Slow/freeze enemies
}

impl CompanionAbility {
    /// Returns the display name of this ability
    pub fn name(&self) -> &'static str {
        match self {
            Self::PackHowl => "Pack Howl",
            Self::MaulAttack => "Maul",
            Self::Ambush => "Ambush",
            Self::DiveBomb => "Dive Bomb",
            Self::BreathWeapon => "Breath Weapon",
            Self::LightningStrike => "Lightning Strike",
            Self::FaeBreath => "Fae Breath",
            Self::Hellfire => "Hellfire",
            Self::PoisonStab => "Poison Stab",
            Self::FireBolt => "Fire Bolt",
            Self::Taunt => "Taunt",
            Self::ShieldBash => "Shield Bash",
            Self::BattleCry => "Battle Cry",
            Self::DivineProtection => "Divine Protection",
            Self::Rebirth => "Rebirth",
            Self::NatureHeal => "Nature's Blessing",
            Self::Illuminate => "Illuminate",
            Self::Terrify => "Terrify",
            Self::LifeDrain => "Life Drain",
            Self::FlameAura => "Flame Aura",
            Self::FrostNova => "Frost Nova",
        }
    }

    /// Returns the cooldown in turns for this ability
    pub fn cooldown(&self) -> u32 {
        match self {
            Self::PackHowl | Self::BattleCry => 15,
            Self::MaulAttack | Self::Ambush => 5,
            Self::DiveBomb | Self::LightningStrike | Self::FireBolt => 3,
            Self::BreathWeapon | Self::Hellfire => 8,
            Self::FaeBreath | Self::Terrify => 10,
            Self::PoisonStab => 4,
            Self::Taunt => 6,
            Self::ShieldBash | Self::FrostNova => 5,
            Self::DivineProtection => 20,
            Self::Rebirth => 50, // Once per floor essentially
            Self::NatureHeal => 12,
            Self::Illuminate => 25,
            Self::FlameAura => 8,
            Self::LifeDrain => 6,
        }
    }

    /// Returns a description of what the ability does
    pub fn description(&self) -> &'static str {
        match self {
            Self::PackHowl => "Increases attack of all allies for 5 turns",
            Self::MaulAttack => "Deals heavy damage and causes bleeding",
            Self::Ambush => "Deals double damage to unaware enemies",
            Self::DiveBomb => "Ranged attack that deals bonus damage",
            Self::BreathWeapon => "Cone attack dealing elemental damage",
            Self::LightningStrike => "Chain lightning hitting multiple enemies",
            Self::FaeBreath => "Confuses enemies, making them attack randomly",
            Self::Hellfire => "Burns enemies, dealing damage over time",
            Self::PoisonStab => "Poisons the target for extended damage",
            Self::FireBolt => "Launches a bolt of fire at distant enemies",
            Self::Taunt => "Forces nearby enemies to attack the companion",
            Self::ShieldBash => "Stuns an enemy for several turns",
            Self::BattleCry => "Increases defense of all allies",
            Self::DivineProtection => "Grants a protective shield to the player",
            Self::Rebirth => "Resurrects with full health once per floor",
            Self::NatureHeal => "Heals the player significantly",
            Self::Illuminate => "Reveals a large area of the map",
            Self::Terrify => "Freezes nearby enemies with fear",
            Self::LifeDrain => "Drains life from enemy to heal self",
            Self::FlameAura => "Burns all adjacent enemies",
            Self::FrostNova => "Freezes all nearby enemies",
        }
    }
}

/// AI behavior modes for companions
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum CompanionBehavior {
    /// Follow player closely and attack nearby enemies
    #[default]
    Aggressive,
    /// Stay close to player and prioritize defending
    Defensive,
    /// Focus on supporting the player (healing, buffs)
    Support,
    /// Stay at range and use abilities/ranged attacks
    Ranged,
    /// Hold position and only attack enemies that come close
    Guard,
}

impl CompanionBehavior {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Aggressive => "Aggressive",
            Self::Defensive => "Defensive",
            Self::Support => "Support",
            Self::Ranged => "Ranged",
            Self::Guard => "Guard",
        }
    }

    /// Cycle to the next behavior mode
    pub fn next(&self) -> Self {
        match self {
            Self::Aggressive => Self::Defensive,
            Self::Defensive => Self::Support,
            Self::Support => Self::Ranged,
            Self::Ranged => Self::Guard,
            Self::Guard => Self::Aggressive,
        }
    }
}

/// Companion morale/loyalty state
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum CompanionMorale {
    Loyal,      // Will fight to the death
    Content,    // Normal behavior
    Nervous,    // May flee at low HP
    Panicked,   // Will flee
}

impl CompanionMorale {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Loyal => "Loyal",
            Self::Content => "Content",
            Self::Nervous => "Nervous",
            Self::Panicked => "Panicked",
        }
    }
}

/// A companion instance
#[derive(Clone, Serialize, Deserialize)]
pub struct Companion {
    pub kind: CompanionKind,
    pub name: String,
    pub x: usize,
    pub y: usize,

    // Stats
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,

    // Progression
    pub level: u32,
    pub xp: u32,
    pub xp_to_level: u32,
    pub bond_level: u32,       // Affects loyalty and ability strength

    // State
    pub behavior: CompanionBehavior,
    pub morale: CompanionMorale,
    pub status_effects: HashMap<StatusEffect, u32>,
    pub ability_cooldown: u32,
    pub has_rebirth: bool,     // For Phoenix ability

    // Combat tracking
    pub kills: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
}

impl Companion {
    /// Create a new companion
    pub fn new(kind: CompanionKind, x: usize, y: usize) -> Self {
        let (base_hp, base_atk, base_def, speed) = kind.base_stats();
        let rarity_bonus = kind.rarity().stat_bonus();

        let hp = (base_hp as f32 * rarity_bonus) as i32;

        Self {
            kind,
            name: kind.name().to_string(),
            x,
            y,
            hp,
            max_hp: hp,
            attack: (base_atk as f32 * rarity_bonus) as i32,
            defense: (base_def as f32 * rarity_bonus) as i32,
            speed,
            level: 1,
            xp: 0,
            xp_to_level: XP_PER_LEVEL[0],
            bond_level: 0,
            behavior: CompanionBehavior::default(),
            morale: CompanionMorale::Content,
            status_effects: HashMap::new(),
            ability_cooldown: 0,
            has_rebirth: kind.ability() == CompanionAbility::Rebirth,
            kills: 0,
            damage_dealt: 0,
            damage_taken: 0,
        }
    }

    /// Create a companion with a custom name
    pub fn new_named(kind: CompanionKind, name: String, x: usize, y: usize) -> Self {
        let mut companion = Self::new(kind, x, y);
        companion.name = name;
        companion
    }

    /// Returns whether the companion is alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Take damage and return actual damage dealt
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = (amount - self.defense).max(1);
        self.hp -= actual;
        self.damage_taken += actual as u32;

        // Update morale based on HP
        self.update_morale();

        // Check for Phoenix rebirth
        if self.hp <= 0 && self.has_rebirth {
            self.hp = self.max_hp;
            self.has_rebirth = false;
            // Return negative to signal rebirth occurred
            return -1;
        }

        actual
    }

    /// Heal the companion
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
        self.update_morale();
    }

    /// Update morale based on current state
    fn update_morale(&mut self) {
        let hp_percent = (self.hp as f32 / self.max_hp as f32) * 100.0;

        // Bond level affects morale thresholds
        let loyalty_bonus = self.bond_level as f32 * 5.0;

        if self.bond_level >= 10 {
            self.morale = CompanionMorale::Loyal;
        } else if hp_percent > 50.0 - loyalty_bonus {
            self.morale = CompanionMorale::Content;
        } else if hp_percent > 25.0 - loyalty_bonus {
            self.morale = CompanionMorale::Nervous;
        } else {
            self.morale = CompanionMorale::Panicked;
        }
    }

    /// Gain XP and return true if leveled up
    pub fn gain_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        if self.xp >= self.xp_to_level && self.level < 20 {
            self.xp -= self.xp_to_level;
            self.level += 1;

            // Update xp_to_level for next level
            if self.level < 20 {
                self.xp_to_level = XP_PER_LEVEL[self.level as usize - 1];
            }

            // Increase stats on level up
            let hp_gain = 5 + self.level as i32;
            self.max_hp += hp_gain;
            self.hp += hp_gain;
            self.attack += 2;
            self.defense += 1;

            // Bond increases with levels
            if self.level % 3 == 0 {
                self.increase_bond(1);
            }

            return true;
        }
        false
    }

    /// Increase bond level
    pub fn increase_bond(&mut self, amount: u32) {
        self.bond_level = (self.bond_level + amount).min(15);
        self.update_morale();
    }

    /// Returns whether the ability is ready to use
    pub fn can_use_ability(&self) -> bool {
        self.ability_cooldown == 0
    }

    /// Use the companion's ability, returning ability used
    pub fn use_ability(&mut self) -> CompanionAbility {
        let ability = self.kind.ability();
        self.ability_cooldown = ability.cooldown();
        ability
    }

    /// Tick cooldowns and status effects, return any damage taken
    pub fn tick(&mut self) -> i32 {
        // Reduce ability cooldown
        if self.ability_cooldown > 0 {
            self.ability_cooldown -= 1;
        }

        // Tick status effects
        let mut damage = 0;
        let mut to_remove = Vec::new();

        let mut heal_amount = 0;
        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => damage += 2,
                StatusEffect::Burn => damage += 3,
                StatusEffect::Bleed => damage += 1,
                StatusEffect::Regeneration => {
                    heal_amount += 2;
                }
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                to_remove.push(*effect);
            }
        }
        if heal_amount > 0 {
            self.heal(heal_amount);
        }

        for effect in to_remove {
            self.status_effects.remove(&effect);
        }

        if damage > 0 {
            self.hp -= damage;
        }

        damage
    }

    /// Add a status effect
    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    /// Check if companion has a status effect
    pub fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    /// Remove a status effect
    pub fn remove_status(&mut self, effect: StatusEffect) {
        self.status_effects.remove(&effect);
    }

    /// Cycle to the next behavior mode
    pub fn cycle_behavior(&mut self) {
        self.behavior = self.behavior.next();
    }

    /// Returns the display name including level
    pub fn display_name(&self) -> String {
        format!("{} Lv.{}", self.name, self.level)
    }

    /// Returns a summary of stats for display
    pub fn stats_summary(&self) -> String {
        format!(
            "{}: HP {}/{} ATK {} DEF {} [{}]",
            self.display_name(),
            self.hp, self.max_hp,
            self.attack, self.defense,
            self.behavior.name()
        )
    }

    /// Calculate the ability damage/heal based on companion stats and level
    pub fn ability_power(&self) -> i32 {
        let base_power = self.attack + self.level as i32 * 2;
        let bond_bonus = 1.0 + (self.bond_level as f32 * 0.05);
        (base_power as f32 * bond_bonus) as i32
    }
}

/// Actions a companion can take during their turn
#[derive(Clone, Debug)]
pub enum CompanionAction {
    Move(i32, i32),
    Attack(usize),        // Enemy index
    UseAbility(usize),    // Target enemy index (if applicable)
    Heal,                 // Self-heal or heal player
    Follow,               // Move towards player
    Flee,                 // Move away from enemies
    Wait,
}

/// AI decision making for companions
pub struct CompanionAI;

impl CompanionAI {
    /// Decide the best action for a companion based on behavior mode
    pub fn decide(
        companion: &Companion,
        player_pos: (usize, usize),
        enemies: &[Enemy],
        map: &Map,
    ) -> CompanionAction {
        let cx = companion.x as i32;
        let cy = companion.y as i32;
        let (px, py) = (player_pos.0 as i32, player_pos.1 as i32);

        // If panicked, always flee
        if companion.morale == CompanionMorale::Panicked {
            return Self::find_flee_direction(companion, enemies, map);
        }

        // If stunned or frozen, wait
        if companion.has_status(StatusEffect::Stun) || companion.has_status(StatusEffect::Freeze) {
            return CompanionAction::Wait;
        }

        // Find nearest visible enemy
        let nearest_enemy = enemies.iter().enumerate()
            .filter(|(_, e)| e.is_alive() && map.visible[e.y][e.x])
            .min_by_key(|(_, e)| {
                let dx = e.x as i32 - cx;
                let dy = e.y as i32 - cy;
                dx * dx + dy * dy
            });

        // Distance to player
        let player_dist = ((px - cx).pow(2) + (py - cy).pow(2)) as f32;

        match companion.behavior {
            CompanionBehavior::Aggressive => {
                Self::aggressive_behavior(companion, player_pos, nearest_enemy, map)
            }
            CompanionBehavior::Defensive => {
                Self::defensive_behavior(companion, player_pos, nearest_enemy, map, player_dist)
            }
            CompanionBehavior::Support => {
                Self::support_behavior(companion, player_pos, nearest_enemy, map, player_dist)
            }
            CompanionBehavior::Ranged => {
                Self::ranged_behavior(companion, player_pos, nearest_enemy, map)
            }
            CompanionBehavior::Guard => {
                Self::guard_behavior(companion, nearest_enemy, map)
            }
        }
    }

    fn aggressive_behavior(
        companion: &Companion,
        player_pos: (usize, usize),
        nearest_enemy: Option<(usize, &Enemy)>,
        map: &Map,
    ) -> CompanionAction {
        if let Some((idx, enemy)) = nearest_enemy {
            let dx = (enemy.x as i32 - companion.x as i32).abs();
            let dy = (enemy.y as i32 - companion.y as i32).abs();

            // Adjacent - attack
            if dx <= 1 && dy <= 1 {
                // Use ability if available and beneficial
                if companion.can_use_ability() {
                    return CompanionAction::UseAbility(idx);
                }
                return CompanionAction::Attack(idx);
            }

            // Move towards enemy
            let move_x = (enemy.x as i32 - companion.x as i32).signum();
            let move_y = (enemy.y as i32 - companion.y as i32).signum();
            let new_x = (companion.x as i32 + move_x).max(0) as usize;
            let new_y = (companion.y as i32 + move_y).max(0) as usize;

            if map.is_walkable(new_x, new_y) {
                return CompanionAction::Move(move_x, move_y);
            }
        }

        // No enemy - follow player
        Self::move_towards(companion, player_pos, map)
    }

    fn defensive_behavior(
        companion: &Companion,
        player_pos: (usize, usize),
        nearest_enemy: Option<(usize, &Enemy)>,
        map: &Map,
        player_dist: f32,
    ) -> CompanionAction {
        // Stay close to player
        if player_dist > 9.0 {
            return Self::move_towards(companion, player_pos, map);
        }

        // Attack only if enemy is adjacent
        if let Some((idx, enemy)) = nearest_enemy {
            let dx = (enemy.x as i32 - companion.x as i32).abs();
            let dy = (enemy.y as i32 - companion.y as i32).abs();

            if dx <= 1 && dy <= 1 {
                if companion.can_use_ability() {
                    return CompanionAction::UseAbility(idx);
                }
                return CompanionAction::Attack(idx);
            }
        }

        // Stay near player
        if player_dist > 4.0 {
            return Self::move_towards(companion, player_pos, map);
        }

        CompanionAction::Wait
    }

    fn support_behavior(
        companion: &Companion,
        player_pos: (usize, usize),
        nearest_enemy: Option<(usize, &Enemy)>,
        map: &Map,
        player_dist: f32,
    ) -> CompanionAction {
        // Priority: Use healing/support abilities
        if companion.can_use_ability() {
            match companion.kind.ability() {
                CompanionAbility::NatureHeal | CompanionAbility::DivineProtection => {
                    return CompanionAction::Heal;
                }
                CompanionAbility::Illuminate => {
                    return CompanionAction::UseAbility(0);
                }
                _ => {}
            }
        }

        // Stay close to player
        if player_dist > 4.0 {
            return Self::move_towards(companion, player_pos, map);
        }

        // Attack if enemy is adjacent
        if let Some((idx, enemy)) = nearest_enemy {
            let dx = (enemy.x as i32 - companion.x as i32).abs();
            let dy = (enemy.y as i32 - companion.y as i32).abs();

            if dx <= 1 && dy <= 1 {
                return CompanionAction::Attack(idx);
            }
        }

        CompanionAction::Wait
    }

    fn ranged_behavior(
        companion: &Companion,
        player_pos: (usize, usize),
        nearest_enemy: Option<(usize, &Enemy)>,
        map: &Map,
    ) -> CompanionAction {
        if let Some((idx, enemy)) = nearest_enemy {
            let dx = (enemy.x as i32 - companion.x as i32).abs();
            let dy = (enemy.y as i32 - companion.y as i32).abs();
            let dist = ((dx * dx + dy * dy) as f32).sqrt();

            // Use ranged ability if available and at range
            if dist > 1.5 && dist < 8.0 && companion.can_use_ability() {
                match companion.kind.ability() {
                    CompanionAbility::DiveBomb | CompanionAbility::FireBolt |
                    CompanionAbility::LightningStrike | CompanionAbility::BreathWeapon => {
                        return CompanionAction::UseAbility(idx);
                    }
                    _ => {}
                }
            }

            // Too close - back away
            if dist < 3.0 {
                let move_x = -(enemy.x as i32 - companion.x as i32).signum();
                let move_y = -(enemy.y as i32 - companion.y as i32).signum();
                let new_x = (companion.x as i32 + move_x).max(0) as usize;
                let new_y = (companion.y as i32 + move_y).max(0) as usize;

                if map.is_walkable(new_x, new_y) {
                    return CompanionAction::Move(move_x, move_y);
                }
            }

            // Attack if adjacent (no choice)
            if dx <= 1 && dy <= 1 {
                return CompanionAction::Attack(idx);
            }
        }

        // Follow player at moderate distance
        Self::move_towards(companion, player_pos, map)
    }

    fn guard_behavior(
        companion: &Companion,
        nearest_enemy: Option<(usize, &Enemy)>,
        _map: &Map,
    ) -> CompanionAction {
        // Only attack adjacent enemies
        if let Some((idx, enemy)) = nearest_enemy {
            let dx = (enemy.x as i32 - companion.x as i32).abs();
            let dy = (enemy.y as i32 - companion.y as i32).abs();

            if dx <= 1 && dy <= 1 {
                if companion.can_use_ability() {
                    return CompanionAction::UseAbility(idx);
                }
                return CompanionAction::Attack(idx);
            }
        }

        CompanionAction::Wait
    }

    fn move_towards(
        companion: &Companion,
        target: (usize, usize),
        map: &Map,
    ) -> CompanionAction {
        let dx = (target.0 as i32 - companion.x as i32).signum();
        let dy = (target.1 as i32 - companion.y as i32).signum();

        if dx == 0 && dy == 0 {
            return CompanionAction::Wait;
        }

        let new_x = (companion.x as i32 + dx).max(0) as usize;
        let new_y = (companion.y as i32 + dy).max(0) as usize;

        if map.is_walkable(new_x, new_y) {
            return CompanionAction::Move(dx, dy);
        }

        // Try horizontal then vertical
        if dx != 0 {
            let new_x = (companion.x as i32 + dx).max(0) as usize;
            if map.is_walkable(new_x, companion.y) {
                return CompanionAction::Move(dx, 0);
            }
        }
        if dy != 0 {
            let new_y = (companion.y as i32 + dy).max(0) as usize;
            if map.is_walkable(companion.x, new_y) {
                return CompanionAction::Move(0, dy);
            }
        }

        CompanionAction::Follow
    }

    fn find_flee_direction(
        companion: &Companion,
        enemies: &[Enemy],
        map: &Map,
    ) -> CompanionAction {
        // Find direction away from enemies
        let mut total_dx = 0i32;
        let mut total_dy = 0i32;

        for enemy in enemies.iter().filter(|e| e.is_alive()) {
            let dx = companion.x as i32 - enemy.x as i32;
            let dy = companion.y as i32 - enemy.y as i32;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < 100 { // Within 10 tiles
                total_dx += dx.signum();
                total_dy += dy.signum();
            }
        }

        if total_dx == 0 && total_dy == 0 {
            return CompanionAction::Wait;
        }

        let flee_dx = total_dx.signum();
        let flee_dy = total_dy.signum();
        let new_x = (companion.x as i32 + flee_dx).max(0) as usize;
        let new_y = (companion.y as i32 + flee_dy).max(0) as usize;

        if map.is_walkable(new_x, new_y) {
            CompanionAction::Flee
        } else {
            CompanionAction::Wait
        }
    }
}

/// Represents a recruitable companion encounter in the dungeon
#[derive(Clone, Serialize, Deserialize)]
pub struct CompanionEncounter {
    pub x: usize,
    pub y: usize,
    pub kind: CompanionKind,
    pub friendly: bool,      // If false, must defeat first
    pub recruit_chance: u32, // Percentage chance to recruit (0-100)
}

impl CompanionEncounter {
    /// Create a new friendly encounter (can recruit directly)
    pub fn new_friendly(x: usize, y: usize, kind: CompanionKind) -> Self {
        let base_chance = match kind.rarity() {
            CompanionRarity::Common => 90,
            CompanionRarity::Uncommon => 75,
            CompanionRarity::Rare => 60,
            CompanionRarity::Epic => 45,
            CompanionRarity::Legendary => 30,
        };

        Self {
            x,
            y,
            kind,
            friendly: true,
            recruit_chance: base_chance,
        }
    }

    /// Create a hostile encounter (must defeat to potentially recruit)
    pub fn new_hostile(x: usize, y: usize, kind: CompanionKind) -> Self {
        let base_chance = match kind.rarity() {
            CompanionRarity::Common => 40,
            CompanionRarity::Uncommon => 30,
            CompanionRarity::Rare => 20,
            CompanionRarity::Epic => 10,
            CompanionRarity::Legendary => 5,
        };

        Self {
            x,
            y,
            kind,
            friendly: false,
            recruit_chance: base_chance,
        }
    }

    /// Returns the glyph for display (different from companion glyph)
    pub fn glyph(&self) -> char {
        if self.friendly {
            '@' // Friendly marker
        } else {
            self.kind.glyph().to_ascii_uppercase()
        }
    }

    /// Attempt to recruit this companion, returns true if successful
    pub fn try_recruit(&self, rng: &mut impl Rng, player_charisma_bonus: i32) -> bool {
        let final_chance = (self.recruit_chance as i32 + player_charisma_bonus).clamp(5, 95) as u32;
        rng.gen_range(0..100) < final_chance
    }
}

/// Generate random companion encounters for a dungeon level
pub fn generate_encounters(
    rooms: &[crate::world::Room],
    dungeon_level: u32,
    rng: &mut impl Rng,
) -> Vec<CompanionEncounter> {
    let mut encounters = Vec::new();

    // Chance of encounter per room based on level
    let encounter_chance = 5 + dungeon_level.min(15) as i32; // 5-20%

    for (i, room) in rooms.iter().enumerate() {
        // Skip first room (player spawn)
        if i == 0 {
            continue;
        }

        if rng.gen_range(0..100) < encounter_chance {
            let (x, y) = room.random_point(rng);
            let kind = CompanionKind::random_for_level(dungeon_level, rng);

            // Higher levels have more hostile encounters
            let hostile_chance = 30 + dungeon_level.min(40) as i32;
            let friendly = rng.gen_range(0..100) >= hostile_chance;

            let encounter = if friendly {
                CompanionEncounter::new_friendly(x, y, kind)
            } else {
                CompanionEncounter::new_hostile(x, y, kind)
            };

            encounters.push(encounter);
        }
    }

    encounters
}

// ---------------------------------------------------------------------------
// Phase 5: Extended companion features
// ---------------------------------------------------------------------------

/// Relationship level between a companion and the player
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Relationship {
    Hostile,
    Unfriendly,
    Neutral,
    Friendly,
    Trusted,
    Devoted,
    Bonded,
}

impl Relationship {
    /// Returns the trust threshold range (min inclusive) for this relationship level
    pub fn trust_threshold(&self) -> i32 {
        match self {
            Self::Hostile => -100,
            Self::Unfriendly => -50,
            Self::Neutral => 0,
            Self::Friendly => 25,
            Self::Trusted => 50,
            Self::Devoted => 75,
            Self::Bonded => 100,
        }
    }

    /// Determine the relationship from a raw trust value
    pub fn from_trust(trust: i32) -> Self {
        if trust >= 100 {
            Self::Bonded
        } else if trust >= 75 {
            Self::Devoted
        } else if trust >= 50 {
            Self::Trusted
        } else if trust >= 25 {
            Self::Friendly
        } else if trust >= 0 {
            Self::Neutral
        } else if trust >= -50 {
            Self::Unfriendly
        } else {
            Self::Hostile
        }
    }
}

/// Romance state with a companion
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RomanceState {
    None,
    Interested,
    Courting,
    Partner,
    Soulbound,
}

impl RomanceState {
    /// Determine the romance state from a raw trust value
    pub fn from_trust(trust: i32) -> Self {
        if trust >= 120 {
            Self::Soulbound
        } else if trust >= 90 {
            Self::Partner
        } else if trust >= 60 {
            Self::Courting
        } else if trust >= 35 {
            Self::Interested
        } else {
            Self::None
        }
    }
}

/// Party formation strategies
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum PartyFormation {
    Balanced,
    Aggressive,
    Defensive,
    Flanking,
    Ambush,
    Diamond,
}

impl PartyFormation {
    /// Attack power modifier for this formation
    pub fn attack_modifier(&self) -> f32 {
        match self {
            Self::Balanced => 1.0,
            Self::Aggressive => 1.3,
            Self::Defensive => 0.8,
            Self::Flanking => 1.2,
            Self::Ambush => 1.5,
            Self::Diamond => 1.1,
        }
    }

    /// Defense modifier for this formation
    pub fn defense_modifier(&self) -> f32 {
        match self {
            Self::Balanced => 1.0,
            Self::Aggressive => 0.7,
            Self::Defensive => 1.4,
            Self::Flanking => 0.9,
            Self::Ambush => 0.6,
            Self::Diamond => 1.2,
        }
    }

    /// Speed modifier for this formation
    pub fn speed_modifier(&self) -> f32 {
        match self {
            Self::Balanced => 1.0,
            Self::Aggressive => 1.1,
            Self::Defensive => 0.8,
            Self::Flanking => 1.2,
            Self::Ambush => 0.7,
            Self::Diamond => 0.9,
        }
    }
}

/// Positions within a party formation
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum FormationPosition {
    Front,
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
    Rear,
}

impl FormationPosition {
    /// Flat defense bonus granted by this position
    pub fn defense_bonus(&self) -> i32 {
        match self {
            Self::Front => 2,
            Self::FrontLeft => 1,
            Self::FrontRight => 1,
            Self::BackLeft => 3,
            Self::BackRight => 3,
            Self::Rear => 5,
        }
    }

    /// Flat attack bonus granted by this position
    pub fn attack_bonus(&self) -> i32 {
        match self {
            Self::Front => 4,
            Self::FrontLeft => 3,
            Self::FrontRight => 3,
            Self::BackLeft => 1,
            Self::BackRight => 1,
            Self::Rear => 0,
        }
    }
}

/// Personality traits that affect companion interactions and compatibility
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Personality {
    Brave,
    Cowardly,
    Loyal,
    Treacherous,
    Kind,
    Cruel,
    Wise,
    Foolish,
    Patient,
    Impulsive,
    Honest,
    Deceptive,
    Cheerful,
    Melancholy,
    Stoic,
    Passionate,
}

impl Personality {
    /// Returns a compatibility score between two personalities (-1.0 to 1.0).
    /// Positive values mean good compatibility, negative means conflict.
    pub fn compatibility(&self, other: &Personality) -> f32 {
        use Personality::*;
        if self == other {
            return 0.5; // Same personality: decent but not perfect
        }
        match (self, other) {
            // Opposing pairs strongly conflict
            (Brave, Cowardly) | (Cowardly, Brave) => -0.8,
            (Loyal, Treacherous) | (Treacherous, Loyal) => -1.0,
            (Kind, Cruel) | (Cruel, Kind) => -0.9,
            (Wise, Foolish) | (Foolish, Wise) => -0.6,
            (Patient, Impulsive) | (Impulsive, Patient) => -0.5,
            (Honest, Deceptive) | (Deceptive, Honest) => -0.7,
            (Cheerful, Melancholy) | (Melancholy, Cheerful) => -0.3,
            (Stoic, Passionate) | (Passionate, Stoic) => -0.2,

            // Harmonious combinations
            (Brave, Loyal) | (Loyal, Brave) => 0.9,
            (Brave, Passionate) | (Passionate, Brave) => 0.7,
            (Kind, Loyal) | (Loyal, Kind) => 0.8,
            (Kind, Cheerful) | (Cheerful, Kind) => 0.7,
            (Kind, Patient) | (Patient, Kind) => 0.6,
            (Wise, Patient) | (Patient, Wise) => 0.9,
            (Wise, Honest) | (Honest, Wise) => 0.8,
            (Wise, Stoic) | (Stoic, Wise) => 0.7,
            (Honest, Loyal) | (Loyal, Honest) => 0.8,
            (Honest, Kind) | (Kind, Honest) => 0.6,
            (Cheerful, Passionate) | (Passionate, Cheerful) => 0.6,
            (Stoic, Patient) | (Patient, Stoic) => 0.7,
            (Stoic, Brave) | (Brave, Stoic) => 0.6,
            (Stoic, Loyal) | (Loyal, Stoic) => 0.5,

            // Mildly negative or toxic combos
            (Cruel, Treacherous) | (Treacherous, Cruel) => 0.3, // villains get along
            (Deceptive, Treacherous) | (Treacherous, Deceptive) => 0.2,
            (Cowardly, Deceptive) | (Deceptive, Cowardly) => 0.1,
            (Foolish, Impulsive) | (Impulsive, Foolish) => -0.4,
            (Cruel, Impulsive) | (Impulsive, Cruel) => -0.3,

            // Default: mild neutrality
            _ => 0.0,
        }
    }
}

/// Types of personal quests that companions may carry
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CompanionQuestType {
    Redemption,
    Revenge,
    LostFamily,
    ForgottenMemory,
    AncientPower,
    PersonalRival,
    HomeDefense,
    SacredOath,
}

/// Template for a pre-defined, named companion
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompanionTemplate {
    pub name: String,
    pub kind: CompanionKind,
    pub rarity: CompanionRarity,
    pub personality: Personality,
    pub backstory: String,
    pub can_romance: bool,
    pub personal_quest: Option<CompanionQuestType>,
    pub base_hp: i32,
    pub base_attack: i32,
    pub base_defense: i32,
}

/// Returns a collection of 33 pre-defined, lore-rich companions
pub fn predefined_companions() -> Vec<CompanionTemplate> {
    vec![
        // 1
        CompanionTemplate {
            name: "Kael the Shadowblade".into(),
            kind: CompanionKind::GoblinRogue,
            rarity: CompanionRarity::Rare,
            personality: Personality::Brave,
            backstory: "Once a feared assassin of the Nighthollow Guild, Kael turned from the shadows after a contract forced him to betray his only friend.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::Revenge),
            base_hp: 38,
            base_attack: 16,
            base_defense: 4,
        },
        // 2
        CompanionTemplate {
            name: "Lyra Moonwhisper".into(),
            kind: CompanionKind::Sprite,
            rarity: CompanionRarity::Epic,
            personality: Personality::Kind,
            backstory: "A fairy healer exiled from the Moonpetal Court after she dared heal a mortal child, breaking the ancient covenant of non-interference.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::LostFamily),
            base_hp: 28,
            base_attack: 8,
            base_defense: 3,
        },
        // 3
        CompanionTemplate {
            name: "Grimjaw".into(),
            kind: CompanionKind::OrcWarrior,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Stoic,
            backstory: "A scarred orc veteran who speaks little but fights with the fury of a storm. He seeks to duel the warlord who slaughtered his clan.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::PersonalRival),
            base_hp: 65,
            base_attack: 15,
            base_defense: 7,
        },
        // 4
        CompanionTemplate {
            name: "Sister Meridia".into(),
            kind: CompanionKind::AngelicGuardian,
            rarity: CompanionRarity::Legendary,
            personality: Personality::Wise,
            backstory: "A cleric who heard the whisper of a dying god and abandoned her temple to fulfil a sacred oath that may cost her mortality.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::SacredOath),
            base_hp: 72,
            base_attack: 14,
            base_defense: 12,
        },
        // 5
        CompanionTemplate {
            name: "Zix".into(),
            kind: CompanionKind::ImpServant,
            rarity: CompanionRarity::Common,
            personality: Personality::Cheerful,
            backstory: "A mischievous imp who broke his infernal contract and now seeks redemption through acts of reluctant heroism and terrible puns.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::Redemption),
            base_hp: 22,
            base_attack: 10,
            base_defense: 2,
        },
        // 6
        CompanionTemplate {
            name: "Fenris Ironclaw".into(),
            kind: CompanionKind::DireWolf,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Loyal,
            backstory: "The last of a dire wolf pack bonded to an ancient ranger order. He searches tirelessly for the descendants of his fallen master.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::LostFamily),
            base_hp: 50,
            base_attack: 14,
            base_defense: 5,
        },
        // 7
        CompanionTemplate {
            name: "Ashara Emberveil".into(),
            kind: CompanionKind::Phoenix,
            rarity: CompanionRarity::Epic,
            personality: Personality::Passionate,
            backstory: "Born from the final ember of a dying volcano, Ashara carries within her the memory of a civilisation consumed by flame.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::ForgottenMemory),
            base_hp: 62,
            base_attack: 19,
            base_defense: 6,
        },
        // 8
        CompanionTemplate {
            name: "Thorn".into(),
            kind: CompanionKind::EarthElemental,
            rarity: CompanionRarity::Rare,
            personality: Personality::Patient,
            backstory: "An earth elemental awakened by a druid's dying spell, Thorn guards the threshold between the living forest and the blighted wastes.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::HomeDefense),
            base_hp: 90,
            base_attack: 9,
            base_defense: 14,
        },
        // 9
        CompanionTemplate {
            name: "Vesper Nightshade".into(),
            kind: CompanionKind::Wraith,
            rarity: CompanionRarity::Rare,
            personality: Personality::Melancholy,
            backstory: "A noblewoman murdered on her wedding night who now drifts through the dungeon, unable to remember her own name but driven by fragments of sorrow.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::ForgottenMemory),
            base_hp: 32,
            base_attack: 17,
            base_defense: 3,
        },
        // 10
        CompanionTemplate {
            name: "Brok Stoneshield".into(),
            kind: CompanionKind::Golem,
            rarity: CompanionRarity::Rare,
            personality: Personality::Stoic,
            backstory: "Forged by dwarven runesmiths to guard a treasury that no longer exists, Brok now seeks a new purpose worthy of his unyielding frame.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::AncientPower),
            base_hp: 110,
            base_attack: 11,
            base_defense: 16,
        },
        // 11
        CompanionTemplate {
            name: "Pip Thistlewick".into(),
            kind: CompanionKind::Wisp,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Cheerful,
            backstory: "A wisp born from a child's laughter that got lost in the Underhollow. Pip lights the way and hums half-remembered lullabies.".into(),
            can_romance: false,
            personal_quest: None,
            base_hp: 16,
            base_attack: 5,
            base_defense: 1,
        },
        // 12
        CompanionTemplate {
            name: "Raziel Duskmantle".into(),
            kind: CompanionKind::FaeDragon,
            rarity: CompanionRarity::Rare,
            personality: Personality::Deceptive,
            backstory: "A fae dragon exiled from the Feywild for weaving illusions so convincing they nearly toppled a sidhe court.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::Redemption),
            base_hp: 42,
            base_attack: 15,
            base_defense: 5,
        },
        // 13
        CompanionTemplate {
            name: "Helga Bloodfury".into(),
            kind: CompanionKind::Bear,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Brave,
            backstory: "A battle-scarred cave bear who once fought alongside the berserker clans of the Frostpeak Mountains. Her roar can shake the stone.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::PersonalRival),
            base_hp: 78,
            base_attack: 13,
            base_defense: 9,
        },
        // 14
        CompanionTemplate {
            name: "Corvus the Bleak".into(),
            kind: CompanionKind::Ghost,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Honest,
            backstory: "The ghost of a judge who refused to condemn an innocent man and was executed in his place. He now speaks only truths, however painful.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::Revenge),
            base_hp: 26,
            base_attack: 13,
            base_defense: 1,
        },
        // 15
        CompanionTemplate {
            name: "Solara Dawnfire".into(),
            kind: CompanionKind::FireSprite,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Impulsive,
            backstory: "A fire sprite who escaped the Elemental Furnace. She acts first and thinks later, leaving a trail of small fires and apologies.".into(),
            can_romance: true,
            personal_quest: None,
            base_hp: 27,
            base_attack: 15,
            base_defense: 2,
        },
        // 16
        CompanionTemplate {
            name: "Morvaine".into(),
            kind: CompanionKind::DemonHound,
            rarity: CompanionRarity::Epic,
            personality: Personality::Loyal,
            backstory: "A hellhound who broke its chains when its demon master ordered it to devour children. Morvaine now serves those who show it mercy.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::Redemption),
            base_hp: 48,
            base_attack: 23,
            base_defense: 5,
        },
        // 17
        CompanionTemplate {
            name: "Celeste".into(),
            kind: CompanionKind::Unicorn,
            rarity: CompanionRarity::Legendary,
            personality: Personality::Kind,
            backstory: "The last unicorn of the Silverglade, Celeste wanders the dark places of the world searching for a heart pure enough to restore her forest.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::HomeDefense),
            base_hp: 58,
            base_attack: 13,
            base_defense: 7,
        },
        // 18
        CompanionTemplate {
            name: "Skarn".into(),
            kind: CompanionKind::SkeletonWarrior,
            rarity: CompanionRarity::Common,
            personality: Personality::Patient,
            backstory: "A skeleton warrior who remembers nothing of his life but retains an instinctive mastery of sword and shield forged over centuries of undeath.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::ForgottenMemory),
            base_hp: 36,
            base_attack: 11,
            base_defense: 7,
        },
        // 19
        CompanionTemplate {
            name: "Tempest".into(),
            kind: CompanionKind::StormElemental,
            rarity: CompanionRarity::Rare,
            personality: Personality::Passionate,
            backstory: "Torn from a hurricane by a sorcerer's binding, Tempest rages against captivity yet finds strange kinship with those who fight for freedom.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::AncientPower),
            base_hp: 37,
            base_attack: 17,
            base_defense: 4,
        },
        // 20
        CompanionTemplate {
            name: "Mira Frostbloom".into(),
            kind: CompanionKind::IceElemental,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Melancholy,
            backstory: "An ice elemental formed from a frozen tear. Mira remembers the warmth of a love she never had and shields allies with walls of sorrow-ice.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::LostFamily),
            base_hp: 47,
            base_attack: 11,
            base_defense: 9,
        },
        // 21
        CompanionTemplate {
            name: "Aldric the Fallen".into(),
            kind: CompanionKind::SkeletonWarrior,
            rarity: CompanionRarity::Common,
            personality: Personality::Brave,
            backstory: "A paladin raised as undead against his will. Aldric clings to the tenets of his faith even as his bones crumble, seeking final rest through valor.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::SacredOath),
            base_hp: 40,
            base_attack: 12,
            base_defense: 8,
        },
        // 22
        CompanionTemplate {
            name: "Nyx Venomtail".into(),
            kind: CompanionKind::Panther,
            rarity: CompanionRarity::Rare,
            personality: Personality::Deceptive,
            backstory: "A shadow panther from the Abyssal Jungle whose venom can melt steel. She chose her current master after he bested her in a game of wits.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::PersonalRival),
            base_hp: 33,
            base_attack: 16,
            base_defense: 3,
        },
        // 23
        CompanionTemplate {
            name: "Brother Ashwin".into(),
            kind: CompanionKind::Golem,
            rarity: CompanionRarity::Rare,
            personality: Personality::Wise,
            backstory: "A clay golem inscribed with the prayers of a hundred monks. He moves slowly but carries the collected wisdom of a monastery now buried under sand.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::ForgottenMemory),
            base_hp: 105,
            base_attack: 10,
            base_defense: 15,
        },
        // 24
        CompanionTemplate {
            name: "Sera Windrider".into(),
            kind: CompanionKind::GiantEagle,
            rarity: CompanionRarity::Rare,
            personality: Personality::Honest,
            backstory: "A giant eagle bonded to a sky-nomad tribe. Sera descended into the dungeon to find the stolen egg of her sister and will not leave without it.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::LostFamily),
            base_hp: 37,
            base_attack: 13,
            base_defense: 4,
        },
        // 25
        CompanionTemplate {
            name: "Vex".into(),
            kind: CompanionKind::GoblinRogue,
            rarity: CompanionRarity::Common,
            personality: Personality::Treacherous,
            backstory: "A goblin information broker who sells secrets to every side. Vex is useful but should never be trusted with anything sharper than a spoon.".into(),
            can_romance: false,
            personal_quest: None,
            base_hp: 24,
            base_attack: 13,
            base_defense: 2,
        },
        // 26
        CompanionTemplate {
            name: "Ignis Char".into(),
            kind: CompanionKind::DragonWhelp,
            rarity: CompanionRarity::Epic,
            personality: Personality::Impulsive,
            backstory: "A young dragon who hatched prematurely during a siege and imprinted on the first voice it heard. Ignis is powerful, hungry, and easily distracted by shiny objects.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::AncientPower),
            base_hp: 55,
            base_attack: 21,
            base_defense: 9,
        },
        // 27
        CompanionTemplate {
            name: "Thessaly".into(),
            kind: CompanionKind::Wolf,
            rarity: CompanionRarity::Common,
            personality: Personality::Loyal,
            backstory: "A common grey wolf who followed a wounded traveler for three days until they were safe. She has never left their side since.".into(),
            can_romance: false,
            personal_quest: None,
            base_hp: 26,
            base_attack: 9,
            base_defense: 3,
        },
        // 28
        CompanionTemplate {
            name: "Oberon Mistwalker".into(),
            kind: CompanionKind::FaeDragon,
            rarity: CompanionRarity::Rare,
            personality: Personality::Foolish,
            backstory: "A fae dragon who wanders between realms with cheerful incompetence, accidentally opening portals and leaving chaos in his glittering wake.".into(),
            can_romance: false,
            personal_quest: None,
            base_hp: 40,
            base_attack: 13,
            base_defense: 4,
        },
        // 29
        CompanionTemplate {
            name: "Dame Ulra".into(),
            kind: CompanionKind::EarthElemental,
            rarity: CompanionRarity::Rare,
            personality: Personality::Brave,
            backstory: "Once a human knight who merged with the earth after a catastrophic spell. Dame Ulra still honours her vows, shielding the innocent with walls of stone.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::SacredOath),
            base_hp: 85,
            base_attack: 9,
            base_defense: 13,
        },
        // 30
        CompanionTemplate {
            name: "Whisper".into(),
            kind: CompanionKind::Ghost,
            rarity: CompanionRarity::Uncommon,
            personality: Personality::Cowardly,
            backstory: "The timid ghost of a librarian who hides behind bookshelves that no longer exist. She can read any language and is terrified of loud noises.".into(),
            can_romance: false,
            personal_quest: None,
            base_hp: 22,
            base_attack: 10,
            base_defense: 0,
        },
        // 31
        CompanionTemplate {
            name: "Rask Cindermaw".into(),
            kind: CompanionKind::DemonHound,
            rarity: CompanionRarity::Epic,
            personality: Personality::Cruel,
            backstory: "A demonic beast that hunts for sport. Rask respects only strength and will turn on any master it deems weak.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::PersonalRival),
            base_hp: 50,
            base_attack: 24,
            base_defense: 4,
        },
        // 32
        CompanionTemplate {
            name: "Elara Starweave".into(),
            kind: CompanionKind::Sprite,
            rarity: CompanionRarity::Common,
            personality: Personality::Patient,
            backstory: "A sprite who tends wounded creatures in the dungeon depths. She joined the adventurer to ensure fewer things die needlessly in the dark.".into(),
            can_romance: true,
            personal_quest: Some(CompanionQuestType::HomeDefense),
            base_hp: 20,
            base_attack: 6,
            base_defense: 2,
        },
        // 33
        CompanionTemplate {
            name: "Mordecai Ashvane".into(),
            kind: CompanionKind::Wraith,
            rarity: CompanionRarity::Rare,
            personality: Personality::Wise,
            backstory: "An ancient scholar who willingly became a wraith to preserve the knowledge of a dying age. He seeks apprentices worthy of inheriting his forbidden lore.".into(),
            can_romance: false,
            personal_quest: Some(CompanionQuestType::AncientPower),
            base_hp: 34,
            base_attack: 16,
            base_defense: 3,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_companion_creation() {
        let companion = Companion::new(CompanionKind::Wolf, 5, 5);
        assert_eq!(companion.level, 1);
        assert!(companion.hp > 0);
        assert!(companion.is_alive());
    }

    #[test]
    fn test_companion_damage() {
        let mut companion = Companion::new(CompanionKind::Bear, 0, 0);
        let initial_hp = companion.hp;
        let damage = companion.take_damage(10);
        assert!(damage > 0);
        assert!(companion.hp < initial_hp);
    }

    #[test]
    fn test_companion_leveling() {
        let mut companion = Companion::new(CompanionKind::Wolf, 0, 0);
        let initial_stats = (companion.max_hp, companion.attack, companion.defense);

        // Grant enough XP to level up
        companion.xp = companion.xp_to_level - 1;
        let leveled = companion.gain_xp(10);

        assert!(leveled);
        assert_eq!(companion.level, 2);
        assert!(companion.max_hp > initial_stats.0);
        assert!(companion.attack > initial_stats.1);
    }

    #[test]
    fn test_phoenix_rebirth() {
        let mut companion = Companion::new(CompanionKind::Phoenix, 0, 0);
        assert!(companion.has_rebirth);

        // Deal lethal damage
        let result = companion.take_damage(companion.hp + 100);

        // Should have reborn (negative return value)
        assert_eq!(result, -1);
        assert!(companion.is_alive());
        assert_eq!(companion.hp, companion.max_hp);
        assert!(!companion.has_rebirth); // Used up
    }

    #[test]
    fn test_behavior_cycling() {
        let mut companion = Companion::new(CompanionKind::Wolf, 0, 0);
        assert_eq!(companion.behavior, CompanionBehavior::Aggressive);

        companion.cycle_behavior();
        assert_eq!(companion.behavior, CompanionBehavior::Defensive);

        companion.cycle_behavior();
        assert_eq!(companion.behavior, CompanionBehavior::Support);
    }

    #[test]
    fn test_morale_updates() {
        let mut companion = Companion::new(CompanionKind::GoblinRogue, 0, 0);
        assert_eq!(companion.morale, CompanionMorale::Content);

        // Damage to low HP
        companion.hp = companion.max_hp / 5;
        companion.update_morale();
        assert_eq!(companion.morale, CompanionMorale::Panicked);

        // Heal up
        companion.hp = companion.max_hp;
        companion.update_morale();
        assert_eq!(companion.morale, CompanionMorale::Content);

        // High bond makes loyal
        companion.bond_level = 10;
        companion.update_morale();
        assert_eq!(companion.morale, CompanionMorale::Loyal);
    }

    #[test]
    fn test_ability_cooldown() {
        let mut companion = Companion::new(CompanionKind::FireSprite, 0, 0);
        assert!(companion.can_use_ability());

        let _ability = companion.use_ability();
        assert!(!companion.can_use_ability());
        assert!(companion.ability_cooldown > 0);

        // Tick until ready
        while companion.ability_cooldown > 0 {
            companion.tick();
        }
        assert!(companion.can_use_ability());
    }

    #[test]
    fn test_rarity_ordering() {
        assert!(CompanionRarity::Common < CompanionRarity::Uncommon);
        assert!(CompanionRarity::Uncommon < CompanionRarity::Rare);
        assert!(CompanionRarity::Rare < CompanionRarity::Epic);
        assert!(CompanionRarity::Epic < CompanionRarity::Legendary);
    }

    #[test]
    fn test_companion_kinds_have_abilities() {
        for kind in CompanionKind::all() {
            let _ability = kind.ability();
            let _name = kind.ability().name();
            // Should not panic
        }
    }
}

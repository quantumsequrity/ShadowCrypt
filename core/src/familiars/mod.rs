//! Familiar and Pet System
//!
//! Comprehensive system for familiars and pets including:
//! - 50+ familiar types across combat, support, utility, and legendary categories
//! - Pet stats, growth, evolution, and bonding mechanics
//! - Pet equipment system with collars, armor, and accessories
//! - Breeding system with trait inheritance

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::prelude::*;

// ============================================================================
// FAMILIAR CATEGORIES AND TYPES
// ============================================================================

/// Category of familiar
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FamiliarCategory {
    Combat,
    Support,
    Utility,
    Legendary,
}

impl FamiliarCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Combat => "Combat",
            Self::Support => "Support",
            Self::Utility => "Utility",
            Self::Legendary => "Legendary",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Combat => "Familiars that excel in battle, dealing damage and tanking hits.",
            Self::Support => "Familiars that heal, buff, and provide passive bonuses.",
            Self::Utility => "Familiars that help with exploration, gathering, and finding treasure.",
            Self::Legendary => "Rare and powerful familiars with unique abilities.",
        }
    }
}

/// All familiar types (50+ types)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FamiliarType {
    // Combat Pets - Beasts (10)
    Wolf,
    DireWolf,
    Bear,
    GrizzlyBear,
    Tiger,
    SaberTooth,
    Lion,
    WarLion,
    Panther,
    ShadowPanther,

    // Combat Pets - Mystical (8)
    DragonHatchling,
    PhoenixChick,
    Imp,
    DemonPuppy,
    StoneGolem,
    IronGolem,
    FireElemental,
    StormElemental,

    // Support Pets (10)
    Fairy,
    GreaterFairy,
    Pixie,
    DreamPixie,
    Owl,
    WiseOwl,
    Cat,
    LuckyCat,
    Wisp,
    HealingSprite,

    // Utility Pets (12)
    Raccoon,
    ThievingRaccoon,
    Mole,
    TunnelMole,
    Bee,
    QueenBee,
    Crow,
    TreasureCrow,
    Ferret,
    ScoutFerret,
    Squirrel,
    HoarderSquirrel,

    // Rare/Legendary Pets (12)
    MiniDragon,
    PrismaticDragon,
    SpiritBeast,
    AncientSpirit,
    CelestialCreature,
    Seraphim,
    VoidEntity,
    VoidLord,
    PhoenixElder,
    FrostWyrm,
    ThunderBird,
    ChaosBeast,
}

impl FamiliarType {
    pub fn all() -> Vec<Self> {
        vec![
            // Combat - Beasts
            Self::Wolf, Self::DireWolf, Self::Bear, Self::GrizzlyBear,
            Self::Tiger, Self::SaberTooth, Self::Lion, Self::WarLion,
            Self::Panther, Self::ShadowPanther,
            // Combat - Mystical
            Self::DragonHatchling, Self::PhoenixChick, Self::Imp, Self::DemonPuppy,
            Self::StoneGolem, Self::IronGolem, Self::FireElemental, Self::StormElemental,
            // Support
            Self::Fairy, Self::GreaterFairy, Self::Pixie, Self::DreamPixie,
            Self::Owl, Self::WiseOwl, Self::Cat, Self::LuckyCat,
            Self::Wisp, Self::HealingSprite,
            // Utility
            Self::Raccoon, Self::ThievingRaccoon, Self::Mole, Self::TunnelMole,
            Self::Bee, Self::QueenBee, Self::Crow, Self::TreasureCrow,
            Self::Ferret, Self::ScoutFerret, Self::Squirrel, Self::HoarderSquirrel,
            // Legendary
            Self::MiniDragon, Self::PrismaticDragon, Self::SpiritBeast, Self::AncientSpirit,
            Self::CelestialCreature, Self::Seraphim, Self::VoidEntity, Self::VoidLord,
            Self::PhoenixElder, Self::FrostWyrm, Self::ThunderBird, Self::ChaosBeast,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Wolf => "Wolf",
            Self::DireWolf => "Dire Wolf",
            Self::Bear => "Bear",
            Self::GrizzlyBear => "Grizzly Bear",
            Self::Tiger => "Tiger",
            Self::SaberTooth => "Sabertooth Tiger",
            Self::Lion => "Lion",
            Self::WarLion => "War Lion",
            Self::Panther => "Panther",
            Self::ShadowPanther => "Shadow Panther",
            Self::DragonHatchling => "Dragon Hatchling",
            Self::PhoenixChick => "Phoenix Chick",
            Self::Imp => "Imp",
            Self::DemonPuppy => "Demon Puppy",
            Self::StoneGolem => "Stone Golem",
            Self::IronGolem => "Iron Golem",
            Self::FireElemental => "Fire Elemental",
            Self::StormElemental => "Storm Elemental",
            Self::Fairy => "Fairy",
            Self::GreaterFairy => "Greater Fairy",
            Self::Pixie => "Pixie",
            Self::DreamPixie => "Dream Pixie",
            Self::Owl => "Owl",
            Self::WiseOwl => "Wise Owl",
            Self::Cat => "Cat",
            Self::LuckyCat => "Lucky Cat",
            Self::Wisp => "Wisp",
            Self::HealingSprite => "Healing Sprite",
            Self::Raccoon => "Raccoon",
            Self::ThievingRaccoon => "Thieving Raccoon",
            Self::Mole => "Mole",
            Self::TunnelMole => "Tunnel Mole",
            Self::Bee => "Bee",
            Self::QueenBee => "Queen Bee",
            Self::Crow => "Crow",
            Self::TreasureCrow => "Treasure Crow",
            Self::Ferret => "Ferret",
            Self::ScoutFerret => "Scout Ferret",
            Self::Squirrel => "Squirrel",
            Self::HoarderSquirrel => "Hoarder Squirrel",
            Self::MiniDragon => "Mini Dragon",
            Self::PrismaticDragon => "Prismatic Dragon",
            Self::SpiritBeast => "Spirit Beast",
            Self::AncientSpirit => "Ancient Spirit",
            Self::CelestialCreature => "Celestial Creature",
            Self::Seraphim => "Seraphim",
            Self::VoidEntity => "Void Entity",
            Self::VoidLord => "Void Lord",
            Self::PhoenixElder => "Phoenix Elder",
            Self::FrostWyrm => "Frost Wyrm",
            Self::ThunderBird => "Thunderbird",
            Self::ChaosBeast => "Chaos Beast",
        }
    }

    pub fn category(&self) -> FamiliarCategory {
        match self {
            Self::Wolf | Self::DireWolf | Self::Bear | Self::GrizzlyBear |
            Self::Tiger | Self::SaberTooth | Self::Lion | Self::WarLion |
            Self::Panther | Self::ShadowPanther | Self::DragonHatchling |
            Self::PhoenixChick | Self::Imp | Self::DemonPuppy |
            Self::StoneGolem | Self::IronGolem | Self::FireElemental |
            Self::StormElemental => FamiliarCategory::Combat,

            Self::Fairy | Self::GreaterFairy | Self::Pixie | Self::DreamPixie |
            Self::Owl | Self::WiseOwl | Self::Cat | Self::LuckyCat |
            Self::Wisp | Self::HealingSprite => FamiliarCategory::Support,

            Self::Raccoon | Self::ThievingRaccoon | Self::Mole | Self::TunnelMole |
            Self::Bee | Self::QueenBee | Self::Crow | Self::TreasureCrow |
            Self::Ferret | Self::ScoutFerret | Self::Squirrel |
            Self::HoarderSquirrel => FamiliarCategory::Utility,

            Self::MiniDragon | Self::PrismaticDragon | Self::SpiritBeast |
            Self::AncientSpirit | Self::CelestialCreature | Self::Seraphim |
            Self::VoidEntity | Self::VoidLord | Self::PhoenixElder |
            Self::FrostWyrm | Self::ThunderBird | Self::ChaosBeast => FamiliarCategory::Legendary,
        }
    }

    pub fn rarity(&self) -> FamiliarRarity {
        match self {
            Self::Wolf | Self::Bear | Self::Cat | Self::Owl |
            Self::Raccoon | Self::Mole | Self::Bee | Self::Crow |
            Self::Ferret | Self::Squirrel | Self::Imp |
            Self::Wisp => FamiliarRarity::Common,

            Self::DireWolf | Self::Tiger | Self::Lion | Self::Panther |
            Self::Fairy | Self::Pixie | Self::ThievingRaccoon |
            Self::TunnelMole | Self::ScoutFerret |
            Self::DragonHatchling | Self::StoneGolem => FamiliarRarity::Uncommon,

            Self::GrizzlyBear | Self::SaberTooth | Self::WarLion |
            Self::ShadowPanther | Self::GreaterFairy | Self::DreamPixie |
            Self::WiseOwl | Self::LuckyCat | Self::HealingSprite |
            Self::QueenBee | Self::TreasureCrow | Self::HoarderSquirrel |
            Self::PhoenixChick | Self::DemonPuppy | Self::IronGolem |
            Self::FireElemental | Self::StormElemental => FamiliarRarity::Rare,

            Self::MiniDragon | Self::SpiritBeast |
            Self::CelestialCreature | Self::VoidEntity => FamiliarRarity::Epic,

            Self::PrismaticDragon | Self::AncientSpirit | Self::Seraphim |
            Self::VoidLord | Self::PhoenixElder | Self::FrostWyrm |
            Self::ThunderBird | Self::ChaosBeast => FamiliarRarity::Legendary,
        }
    }

    /// Base stats: (hp, attack, defense, speed)
    pub fn base_stats(&self) -> FamiliarBaseStats {
        match self {
            // Combat - Beasts
            Self::Wolf => FamiliarBaseStats::new(30, 12, 5, 14),
            Self::DireWolf => FamiliarBaseStats::new(50, 18, 8, 12),
            Self::Bear => FamiliarBaseStats::new(60, 14, 12, 6),
            Self::GrizzlyBear => FamiliarBaseStats::new(90, 20, 16, 5),
            Self::Tiger => FamiliarBaseStats::new(45, 20, 6, 16),
            Self::SaberTooth => FamiliarBaseStats::new(70, 28, 8, 14),
            Self::Lion => FamiliarBaseStats::new(55, 18, 8, 12),
            Self::WarLion => FamiliarBaseStats::new(80, 24, 12, 11),
            Self::Panther => FamiliarBaseStats::new(40, 16, 5, 18),
            Self::ShadowPanther => FamiliarBaseStats::new(60, 22, 7, 20),

            // Combat - Mystical
            Self::DragonHatchling => FamiliarBaseStats::new(50, 16, 10, 10),
            Self::PhoenixChick => FamiliarBaseStats::new(40, 18, 6, 14),
            Self::Imp => FamiliarBaseStats::new(25, 14, 4, 16),
            Self::DemonPuppy => FamiliarBaseStats::new(45, 20, 8, 12),
            Self::StoneGolem => FamiliarBaseStats::new(100, 12, 20, 4),
            Self::IronGolem => FamiliarBaseStats::new(130, 16, 25, 3),
            Self::FireElemental => FamiliarBaseStats::new(45, 24, 5, 12),
            Self::StormElemental => FamiliarBaseStats::new(50, 22, 6, 14),

            // Support
            Self::Fairy => FamiliarBaseStats::new(20, 6, 3, 14),
            Self::GreaterFairy => FamiliarBaseStats::new(35, 10, 5, 16),
            Self::Pixie => FamiliarBaseStats::new(18, 5, 2, 18),
            Self::DreamPixie => FamiliarBaseStats::new(30, 8, 4, 20),
            Self::Owl => FamiliarBaseStats::new(25, 8, 4, 12),
            Self::WiseOwl => FamiliarBaseStats::new(40, 12, 6, 14),
            Self::Cat => FamiliarBaseStats::new(22, 7, 3, 16),
            Self::LuckyCat => FamiliarBaseStats::new(35, 10, 5, 18),
            Self::Wisp => FamiliarBaseStats::new(15, 4, 2, 20),
            Self::HealingSprite => FamiliarBaseStats::new(28, 6, 4, 16),

            // Utility
            Self::Raccoon => FamiliarBaseStats::new(28, 8, 5, 14),
            Self::ThievingRaccoon => FamiliarBaseStats::new(40, 12, 7, 16),
            Self::Mole => FamiliarBaseStats::new(30, 6, 8, 8),
            Self::TunnelMole => FamiliarBaseStats::new(45, 10, 12, 10),
            Self::Bee => FamiliarBaseStats::new(15, 6, 2, 18),
            Self::QueenBee => FamiliarBaseStats::new(30, 12, 5, 16),
            Self::Crow => FamiliarBaseStats::new(22, 8, 3, 16),
            Self::TreasureCrow => FamiliarBaseStats::new(35, 12, 5, 18),
            Self::Ferret => FamiliarBaseStats::new(20, 7, 3, 18),
            Self::ScoutFerret => FamiliarBaseStats::new(32, 11, 5, 20),
            Self::Squirrel => FamiliarBaseStats::new(18, 5, 3, 20),
            Self::HoarderSquirrel => FamiliarBaseStats::new(30, 8, 5, 22),

            // Legendary
            Self::MiniDragon => FamiliarBaseStats::new(80, 25, 15, 14),
            Self::PrismaticDragon => FamiliarBaseStats::new(120, 35, 22, 16),
            Self::SpiritBeast => FamiliarBaseStats::new(70, 22, 12, 18),
            Self::AncientSpirit => FamiliarBaseStats::new(100, 30, 18, 20),
            Self::CelestialCreature => FamiliarBaseStats::new(90, 28, 20, 16),
            Self::Seraphim => FamiliarBaseStats::new(130, 38, 28, 18),
            Self::VoidEntity => FamiliarBaseStats::new(75, 30, 10, 22),
            Self::VoidLord => FamiliarBaseStats::new(110, 42, 15, 24),
            Self::PhoenixElder => FamiliarBaseStats::new(100, 32, 16, 18),
            Self::FrostWyrm => FamiliarBaseStats::new(140, 30, 25, 12),
            Self::ThunderBird => FamiliarBaseStats::new(85, 35, 14, 24),
            Self::ChaosBeast => FamiliarBaseStats::new(150, 40, 20, 15),
        }
    }

    /// Returns the evolution target, if any
    pub fn evolves_to(&self) -> Option<Self> {
        match self {
            Self::Wolf => Some(Self::DireWolf),
            Self::Bear => Some(Self::GrizzlyBear),
            Self::Tiger => Some(Self::SaberTooth),
            Self::Lion => Some(Self::WarLion),
            Self::Panther => Some(Self::ShadowPanther),
            Self::Fairy => Some(Self::GreaterFairy),
            Self::Pixie => Some(Self::DreamPixie),
            Self::Owl => Some(Self::WiseOwl),
            Self::Cat => Some(Self::LuckyCat),
            Self::Raccoon => Some(Self::ThievingRaccoon),
            Self::Mole => Some(Self::TunnelMole),
            Self::Bee => Some(Self::QueenBee),
            Self::Crow => Some(Self::TreasureCrow),
            Self::Ferret => Some(Self::ScoutFerret),
            Self::Squirrel => Some(Self::HoarderSquirrel),
            Self::DragonHatchling => Some(Self::MiniDragon),
            Self::StoneGolem => Some(Self::IronGolem),
            Self::Imp => Some(Self::DemonPuppy),
            Self::Wisp => Some(Self::HealingSprite),
            Self::MiniDragon => Some(Self::PrismaticDragon),
            Self::SpiritBeast => Some(Self::AncientSpirit),
            Self::CelestialCreature => Some(Self::Seraphim),
            Self::VoidEntity => Some(Self::VoidLord),
            Self::PhoenixChick => Some(Self::PhoenixElder),
            _ => None,
        }
    }

    pub fn evolution_level(&self) -> u32 {
        match self.rarity() {
            FamiliarRarity::Common => 15,
            FamiliarRarity::Uncommon => 25,
            FamiliarRarity::Rare => 35,
            FamiliarRarity::Epic => 45,
            FamiliarRarity::Legendary => 99, // Cannot evolve further
        }
    }

    pub fn innate_ability(&self) -> FamiliarAbility {
        match self {
            // Combat abilities
            Self::Wolf | Self::DireWolf => FamiliarAbility::PackHowl,
            Self::Bear | Self::GrizzlyBear => FamiliarAbility::MaulAttack,
            Self::Tiger | Self::SaberTooth => FamiliarAbility::Pounce,
            Self::Lion | Self::WarLion => FamiliarAbility::Roar,
            Self::Panther | Self::ShadowPanther => FamiliarAbility::ShadowStrike,
            Self::DragonHatchling | Self::MiniDragon | Self::PrismaticDragon => FamiliarAbility::BreathWeapon,
            Self::PhoenixChick | Self::PhoenixElder => FamiliarAbility::Rebirth,
            Self::Imp | Self::DemonPuppy => FamiliarAbility::DarkBolt,
            Self::StoneGolem | Self::IronGolem => FamiliarAbility::Fortify,
            Self::FireElemental => FamiliarAbility::FlameAura,
            Self::StormElemental => FamiliarAbility::LightningStrike,

            // Support abilities
            Self::Fairy | Self::GreaterFairy => FamiliarAbility::Heal,
            Self::Pixie | Self::DreamPixie => FamiliarAbility::ManaRestore,
            Self::Owl | Self::WiseOwl => FamiliarAbility::Scout,
            Self::Cat | Self::LuckyCat => FamiliarAbility::LuckBonus,
            Self::Wisp | Self::HealingSprite => FamiliarAbility::Regeneration,

            // Utility abilities
            Self::Raccoon | Self::ThievingRaccoon => FamiliarAbility::Loot,
            Self::Mole | Self::TunnelMole => FamiliarAbility::Dig,
            Self::Bee | Self::QueenBee => FamiliarAbility::Gather,
            Self::Crow | Self::TreasureCrow => FamiliarAbility::TreasureSense,
            Self::Ferret | Self::ScoutFerret => FamiliarAbility::Detect,
            Self::Squirrel | Self::HoarderSquirrel => FamiliarAbility::Hoard,

            // Legendary abilities
            Self::SpiritBeast | Self::AncientSpirit => FamiliarAbility::SpiritLink,
            Self::CelestialCreature | Self::Seraphim => FamiliarAbility::DivineBlessing,
            Self::VoidEntity | Self::VoidLord => FamiliarAbility::VoidTouch,
            Self::FrostWyrm => FamiliarAbility::FrostBreath,
            Self::ThunderBird => FamiliarAbility::ThunderClap,
            Self::ChaosBeast => FamiliarAbility::ChaosStorm,
        }
    }

    pub fn glyph(&self) -> char {
        match self.category() {
            FamiliarCategory::Combat => match self {
                Self::Wolf | Self::DireWolf => 'w',
                Self::Bear | Self::GrizzlyBear => 'B',
                Self::Tiger | Self::SaberTooth => 't',
                Self::Lion | Self::WarLion => 'L',
                Self::Panther | Self::ShadowPanther => 'p',
                Self::DragonHatchling => 'd',
                Self::PhoenixChick => 'P',
                Self::Imp | Self::DemonPuppy => 'i',
                Self::StoneGolem | Self::IronGolem => 'G',
                Self::FireElemental => 'F',
                Self::StormElemental => 'S',
                _ => '?',
            },
            FamiliarCategory::Support => match self {
                Self::Fairy | Self::GreaterFairy => 'f',
                Self::Pixie | Self::DreamPixie => 'x',
                Self::Owl | Self::WiseOwl => 'O',
                Self::Cat | Self::LuckyCat => 'c',
                Self::Wisp | Self::HealingSprite => 'o',
                _ => '?',
            },
            FamiliarCategory::Utility => match self {
                Self::Raccoon | Self::ThievingRaccoon => 'r',
                Self::Mole | Self::TunnelMole => 'm',
                Self::Bee | Self::QueenBee => 'b',
                Self::Crow | Self::TreasureCrow => 'C',
                Self::Ferret | Self::ScoutFerret => 'e',
                Self::Squirrel | Self::HoarderSquirrel => 's',
                _ => '?',
            },
            FamiliarCategory::Legendary => match self {
                Self::MiniDragon | Self::PrismaticDragon => 'D',
                Self::SpiritBeast | Self::AncientSpirit => 'A',
                Self::CelestialCreature | Self::Seraphim => 'X',
                Self::VoidEntity | Self::VoidLord => 'V',
                Self::PhoenixElder => 'P',
                Self::FrostWyrm => 'W',
                Self::ThunderBird => 'T',
                Self::ChaosBeast => 'K',
                _ => '?',
            },
        }
    }
}

// ============================================================================
// FAMILIAR RARITY
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FamiliarRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl FamiliarRarity {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
        }
    }

    pub fn color_code(&self) -> u8 {
        match self {
            Self::Common => 7,      // White
            Self::Uncommon => 2,    // Green
            Self::Rare => 4,        // Blue
            Self::Epic => 5,        // Purple
            Self::Legendary => 3,   // Gold/Orange
        }
    }

    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.15,
            Self::Rare => 1.30,
            Self::Epic => 1.50,
            Self::Legendary => 1.80,
        }
    }

    pub fn spawn_weight(&self) -> u32 {
        match self {
            Self::Common => 1000,
            Self::Uncommon => 400,
            Self::Rare => 150,
            Self::Epic => 40,
            Self::Legendary => 10,
        }
    }
}

// ============================================================================
// FAMILIAR BASE STATS
// ============================================================================

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FamiliarBaseStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

impl FamiliarBaseStats {
    pub fn new(hp: i32, attack: i32, defense: i32, speed: i32) -> Self {
        Self { hp, attack, defense, speed }
    }
}

// ============================================================================
// FAMILIAR ABILITIES
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FamiliarAbility {
    // Combat Abilities
    PackHowl,
    MaulAttack,
    Pounce,
    Roar,
    ShadowStrike,
    BreathWeapon,
    Rebirth,
    DarkBolt,
    Fortify,
    FlameAura,
    LightningStrike,
    FrostBreath,
    ThunderClap,
    ChaosStorm,

    // Support Abilities
    Heal,
    ManaRestore,
    Scout,
    LuckBonus,
    Regeneration,
    DivineBlessing,
    SpiritLink,

    // Utility Abilities
    Loot,
    Dig,
    Gather,
    TreasureSense,
    Detect,
    Hoard,
    VoidTouch,

    // Bond Abilities (unlocked through bonding)
    SharedVitality,
    MindLink,
    CombinedStrike,
    SoulBond,
    ElementalFusion,
    PerfectHarmony,
}

impl FamiliarAbility {
    pub fn name(&self) -> &'static str {
        match self {
            Self::PackHowl => "Pack Howl",
            Self::MaulAttack => "Maul",
            Self::Pounce => "Pounce",
            Self::Roar => "Intimidating Roar",
            Self::ShadowStrike => "Shadow Strike",
            Self::BreathWeapon => "Breath Weapon",
            Self::Rebirth => "Phoenix Rebirth",
            Self::DarkBolt => "Dark Bolt",
            Self::Fortify => "Fortify",
            Self::FlameAura => "Flame Aura",
            Self::LightningStrike => "Lightning Strike",
            Self::FrostBreath => "Frost Breath",
            Self::ThunderClap => "Thunder Clap",
            Self::ChaosStorm => "Chaos Storm",
            Self::Heal => "Healing Touch",
            Self::ManaRestore => "Mana Restoration",
            Self::Scout => "Scout Ahead",
            Self::LuckBonus => "Lucky Charm",
            Self::Regeneration => "Regeneration Aura",
            Self::DivineBlessing => "Divine Blessing",
            Self::SpiritLink => "Spirit Link",
            Self::Loot => "Expert Looting",
            Self::Dig => "Dig",
            Self::Gather => "Gather Resources",
            Self::TreasureSense => "Treasure Sense",
            Self::Detect => "Detect Hidden",
            Self::Hoard => "Hoard Items",
            Self::VoidTouch => "Void Touch",
            Self::SharedVitality => "Shared Vitality",
            Self::MindLink => "Mind Link",
            Self::CombinedStrike => "Combined Strike",
            Self::SoulBond => "Soul Bond",
            Self::ElementalFusion => "Elemental Fusion",
            Self::PerfectHarmony => "Perfect Harmony",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::PackHowl => "Howl to boost ally attack by 20% for 5 turns.",
            Self::MaulAttack => "Savage attack dealing 150% damage with bleed effect.",
            Self::Pounce => "Leap attack that stuns target for 1 turn.",
            Self::Roar => "Frighten enemies, reducing their attack by 15%.",
            Self::ShadowStrike => "Attack from shadows, guaranteed critical hit.",
            Self::BreathWeapon => "Elemental breath attack hitting all enemies in cone.",
            Self::Rebirth => "Resurrect once per battle with 50% HP.",
            Self::DarkBolt => "Fire a bolt of dark energy for magic damage.",
            Self::Fortify => "Increase defense by 50% for 3 turns.",
            Self::FlameAura => "Burn nearby enemies for fire damage each turn.",
            Self::LightningStrike => "Call lightning dealing massive single-target damage.",
            Self::FrostBreath => "Freeze enemies in cone, slowing them by 50%.",
            Self::ThunderClap => "AoE thunder damage that can stun.",
            Self::ChaosStorm => "Random elemental damage to all enemies.",
            Self::Heal => "Restore 25% of master's max HP.",
            Self::ManaRestore => "Restore 20% of master's max mana.",
            Self::Scout => "Reveal a large area of the map.",
            Self::LuckBonus => "Passively increase drop rates by 15%.",
            Self::Regeneration => "Slowly heal master over time.",
            Self::DivineBlessing => "Remove debuffs and grant damage immunity for 2 turns.",
            Self::SpiritLink => "Share damage taken between familiar and master.",
            Self::Loot => "Increase gold drops by 25%.",
            Self::Dig => "Find buried treasures and reveal hidden passages.",
            Self::Gather => "Passively collect herbs and materials.",
            Self::TreasureSense => "Detect treasure chests on the current floor.",
            Self::Detect => "Reveal traps and hidden enemies.",
            Self::Hoard => "Automatically pick up nearby items.",
            Self::VoidTouch => "Deal true damage ignoring all defenses.",
            Self::SharedVitality => "Familiar and master share HP pool.",
            Self::MindLink => "Familiar can use master's skills.",
            Self::CombinedStrike => "Attack together for massive combo damage.",
            Self::SoulBond => "If one would die, survive with 1 HP instead.",
            Self::ElementalFusion => "Combine elements for unique attacks.",
            Self::PerfectHarmony => "All stats increased by bond level %.",
        }
    }

    pub fn cooldown(&self) -> u32 {
        match self {
            Self::PackHowl | Self::Roar => 8,
            Self::MaulAttack | Self::Pounce | Self::ShadowStrike => 4,
            Self::BreathWeapon | Self::FrostBreath => 6,
            Self::Rebirth => 999, // Once per battle
            Self::DarkBolt | Self::LightningStrike => 3,
            Self::Fortify => 10,
            Self::FlameAura => 12,
            Self::ThunderClap | Self::ChaosStorm => 8,
            Self::Heal => 8,
            Self::ManaRestore => 10,
            Self::Scout => 20,
            Self::LuckBonus => 0, // Passive
            Self::Regeneration => 0, // Passive
            Self::DivineBlessing => 15,
            Self::SpiritLink => 0, // Passive toggle
            Self::Loot => 0, // Passive
            Self::Dig => 5,
            Self::Gather => 0, // Passive
            Self::TreasureSense => 15,
            Self::Detect => 8,
            Self::Hoard => 0, // Passive
            Self::VoidTouch => 6,
            Self::SharedVitality => 0, // Passive
            Self::MindLink => 0, // Passive
            Self::CombinedStrike => 10,
            Self::SoulBond => 0, // Passive
            Self::ElementalFusion => 8,
            Self::PerfectHarmony => 0, // Passive
        }
    }

    pub fn is_passive(&self) -> bool {
        self.cooldown() == 0
    }

    pub fn mana_cost(&self) -> i32 {
        match self {
            Self::PackHowl | Self::Roar => 15,
            Self::MaulAttack | Self::Pounce | Self::ShadowStrike => 10,
            Self::BreathWeapon | Self::FrostBreath => 25,
            Self::Rebirth => 0,
            Self::DarkBolt | Self::LightningStrike => 20,
            Self::Fortify => 15,
            Self::FlameAura => 30,
            Self::ThunderClap | Self::ChaosStorm => 35,
            Self::Heal => 25,
            Self::ManaRestore => 0,
            Self::Scout => 10,
            Self::DivineBlessing => 40,
            Self::VoidTouch => 30,
            Self::CombinedStrike => 25,
            Self::ElementalFusion => 35,
            _ => 0, // Passives have no cost
        }
    }
}

// ============================================================================
// MOOD AND LOYALTY
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamiliarMood {
    Ecstatic,    // +20% all stats
    Happy,       // +10% all stats
    Content,     // Normal
    Unhappy,     // -10% all stats
    Miserable,   // -20% all stats, may refuse commands
}

impl FamiliarMood {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ecstatic => "Ecstatic",
            Self::Happy => "Happy",
            Self::Content => "Content",
            Self::Unhappy => "Unhappy",
            Self::Miserable => "Miserable",
        }
    }

    pub fn stat_modifier(&self) -> f32 {
        match self {
            Self::Ecstatic => 1.20,
            Self::Happy => 1.10,
            Self::Content => 1.0,
            Self::Unhappy => 0.90,
            Self::Miserable => 0.80,
        }
    }

    pub fn from_happiness(happiness: i32) -> Self {
        match happiness {
            90..=100 => Self::Ecstatic,
            70..=89 => Self::Happy,
            40..=69 => Self::Content,
            20..=39 => Self::Unhappy,
            _ => Self::Miserable,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoyaltyLevel {
    Distrustful,  // May flee in combat
    Wary,         // Follows basic commands
    Neutral,      // Standard behavior
    Friendly,     // Better performance
    Devoted,      // Will fight to the death
    Soulbound,    // Maximum loyalty, unlocks soul abilities
}

impl LoyaltyLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Distrustful => "Distrustful",
            Self::Wary => "Wary",
            Self::Neutral => "Neutral",
            Self::Friendly => "Friendly",
            Self::Devoted => "Devoted",
            Self::Soulbound => "Soulbound",
        }
    }

    pub fn from_loyalty(loyalty: i32) -> Self {
        match loyalty {
            95..=100 => Self::Soulbound,
            75..=94 => Self::Devoted,
            50..=74 => Self::Friendly,
            30..=49 => Self::Neutral,
            15..=29 => Self::Wary,
            _ => Self::Distrustful,
        }
    }

    pub fn damage_bonus(&self) -> f32 {
        match self {
            Self::Distrustful => 0.7,
            Self::Wary => 0.85,
            Self::Neutral => 1.0,
            Self::Friendly => 1.1,
            Self::Devoted => 1.25,
            Self::Soulbound => 1.5,
        }
    }
}

// ============================================================================
// PET EQUIPMENT
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Collar,
    Armor,
    Accessory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollarType {
    LeatherCollar,
    StuddedCollar,
    SpikedCollar,
    MysticCollar,
    DragonscaleCollar,
    VoidCollar,
}

impl CollarType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LeatherCollar => "Leather Collar",
            Self::StuddedCollar => "Studded Collar",
            Self::SpikedCollar => "Spiked Collar",
            Self::MysticCollar => "Mystic Collar",
            Self::DragonscaleCollar => "Dragonscale Collar",
            Self::VoidCollar => "Void Collar",
        }
    }

    pub fn stats(&self) -> EquipmentStats {
        match self {
            Self::LeatherCollar => EquipmentStats { hp: 5, attack: 0, defense: 2, speed: 0 },
            Self::StuddedCollar => EquipmentStats { hp: 10, attack: 2, defense: 3, speed: 0 },
            Self::SpikedCollar => EquipmentStats { hp: 5, attack: 8, defense: 2, speed: 0 },
            Self::MysticCollar => EquipmentStats { hp: 15, attack: 5, defense: 5, speed: 2 },
            Self::DragonscaleCollar => EquipmentStats { hp: 30, attack: 10, defense: 12, speed: 0 },
            Self::VoidCollar => EquipmentStats { hp: 20, attack: 15, defense: 8, speed: 5 },
        }
    }

    pub fn rarity(&self) -> FamiliarRarity {
        match self {
            Self::LeatherCollar => FamiliarRarity::Common,
            Self::StuddedCollar => FamiliarRarity::Uncommon,
            Self::SpikedCollar => FamiliarRarity::Rare,
            Self::MysticCollar => FamiliarRarity::Rare,
            Self::DragonscaleCollar => FamiliarRarity::Epic,
            Self::VoidCollar => FamiliarRarity::Legendary,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorType {
    ClothBarding,
    LeatherBarding,
    ChainBarding,
    PlateBarding,
    ElementalBarding,
    CelestialBarding,
}

impl ArmorType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ClothBarding => "Cloth Barding",
            Self::LeatherBarding => "Leather Barding",
            Self::ChainBarding => "Chain Barding",
            Self::PlateBarding => "Plate Barding",
            Self::ElementalBarding => "Elemental Barding",
            Self::CelestialBarding => "Celestial Barding",
        }
    }

    pub fn stats(&self) -> EquipmentStats {
        match self {
            Self::ClothBarding => EquipmentStats { hp: 5, attack: 0, defense: 3, speed: 2 },
            Self::LeatherBarding => EquipmentStats { hp: 10, attack: 0, defense: 6, speed: 1 },
            Self::ChainBarding => EquipmentStats { hp: 15, attack: 0, defense: 10, speed: -1 },
            Self::PlateBarding => EquipmentStats { hp: 25, attack: 0, defense: 18, speed: -3 },
            Self::ElementalBarding => EquipmentStats { hp: 20, attack: 5, defense: 12, speed: 2 },
            Self::CelestialBarding => EquipmentStats { hp: 35, attack: 8, defense: 20, speed: 3 },
        }
    }

    pub fn rarity(&self) -> FamiliarRarity {
        match self {
            Self::ClothBarding => FamiliarRarity::Common,
            Self::LeatherBarding => FamiliarRarity::Common,
            Self::ChainBarding => FamiliarRarity::Uncommon,
            Self::PlateBarding => FamiliarRarity::Rare,
            Self::ElementalBarding => FamiliarRarity::Epic,
            Self::CelestialBarding => FamiliarRarity::Legendary,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessoryType {
    LuckyCharm,
    AttackGem,
    DefenseGem,
    SpeedGem,
    LifeGem,
    BondRing,
    ElementalCrystal,
    SoulGem,
}

impl AccessoryType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LuckyCharm => "Lucky Charm",
            Self::AttackGem => "Attack Gem",
            Self::DefenseGem => "Defense Gem",
            Self::SpeedGem => "Speed Gem",
            Self::LifeGem => "Life Gem",
            Self::BondRing => "Bond Ring",
            Self::ElementalCrystal => "Elemental Crystal",
            Self::SoulGem => "Soul Gem",
        }
    }

    pub fn stats(&self) -> EquipmentStats {
        match self {
            Self::LuckyCharm => EquipmentStats { hp: 0, attack: 0, defense: 0, speed: 0 },
            Self::AttackGem => EquipmentStats { hp: 0, attack: 12, defense: 0, speed: 0 },
            Self::DefenseGem => EquipmentStats { hp: 0, attack: 0, defense: 10, speed: 0 },
            Self::SpeedGem => EquipmentStats { hp: 0, attack: 0, defense: 0, speed: 8 },
            Self::LifeGem => EquipmentStats { hp: 40, attack: 0, defense: 0, speed: 0 },
            Self::BondRing => EquipmentStats { hp: 10, attack: 5, defense: 5, speed: 2 },
            Self::ElementalCrystal => EquipmentStats { hp: 15, attack: 10, defense: 5, speed: 3 },
            Self::SoulGem => EquipmentStats { hp: 25, attack: 8, defense: 8, speed: 5 },
        }
    }

    pub fn rarity(&self) -> FamiliarRarity {
        match self {
            Self::LuckyCharm => FamiliarRarity::Common,
            Self::AttackGem | Self::DefenseGem | Self::SpeedGem => FamiliarRarity::Uncommon,
            Self::LifeGem | Self::BondRing => FamiliarRarity::Rare,
            Self::ElementalCrystal => FamiliarRarity::Epic,
            Self::SoulGem => FamiliarRarity::Legendary,
        }
    }

    pub fn special_effect(&self) -> Option<&'static str> {
        match self {
            Self::LuckyCharm => Some("Increases drop rates by 10%"),
            Self::BondRing => Some("Bond XP gain increased by 25%"),
            Self::ElementalCrystal => Some("Adds elemental damage to attacks"),
            Self::SoulGem => Some("Enables soul abilities"),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct EquipmentStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

impl EquipmentStats {
    pub fn combine(&self, other: &EquipmentStats) -> EquipmentStats {
        EquipmentStats {
            hp: self.hp + other.hp,
            attack: self.attack + other.attack,
            defense: self.defense + other.defense,
            speed: self.speed + other.speed,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PetEquipment {
    pub collar: Option<CollarType>,
    pub armor: Option<ArmorType>,
    pub accessory: Option<AccessoryType>,
}

impl Default for PetEquipment {
    fn default() -> Self {
        Self {
            collar: None,
            armor: None,
            accessory: None,
        }
    }
}

impl PetEquipment {
    pub fn total_stats(&self) -> EquipmentStats {
        let mut total = EquipmentStats::default();
        if let Some(collar) = &self.collar {
            total = total.combine(&collar.stats());
        }
        if let Some(armor) = &self.armor {
            total = total.combine(&armor.stats());
        }
        if let Some(accessory) = &self.accessory {
            total = total.combine(&accessory.stats());
        }
        total
    }
}

// ============================================================================
// FAMILIAR TRAITS (for breeding)
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FamiliarTrait {
    // Positive traits
    Aggressive,     // +15% attack
    Sturdy,         // +15% defense
    Swift,          // +15% speed
    Healthy,        // +15% HP
    Clever,         // Learn abilities faster
    Loyal,          // Bond grows faster
    Lucky,          // Better drop rates
    Brave,          // Never flees
    Elemental,      // Bonus elemental damage
    Ethereal,       // Chance to dodge attacks

    // Negative traits
    Timid,          // -10% attack
    Fragile,        // -10% defense
    Sluggish,       // -10% speed
    Sickly,         // -10% HP
    Stubborn,       // Slower ability learning
    Aloof,          // Bond grows slower

    // Rare traits
    Blessed,        // +10% all stats
    Cursed,         // -5% all stats but +25% damage
    Awakened,       // Unlocks hidden potential
    Primordial,     // Ancient bloodline, unique abilities
}

impl FamiliarTrait {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Aggressive => "Aggressive",
            Self::Sturdy => "Sturdy",
            Self::Swift => "Swift",
            Self::Healthy => "Healthy",
            Self::Clever => "Clever",
            Self::Loyal => "Loyal",
            Self::Lucky => "Lucky",
            Self::Brave => "Brave",
            Self::Elemental => "Elemental",
            Self::Ethereal => "Ethereal",
            Self::Timid => "Timid",
            Self::Fragile => "Fragile",
            Self::Sluggish => "Sluggish",
            Self::Sickly => "Sickly",
            Self::Stubborn => "Stubborn",
            Self::Aloof => "Aloof",
            Self::Blessed => "Blessed",
            Self::Cursed => "Cursed",
            Self::Awakened => "Awakened",
            Self::Primordial => "Primordial",
        }
    }

    pub fn is_positive(&self) -> bool {
        matches!(
            self,
            Self::Aggressive | Self::Sturdy | Self::Swift | Self::Healthy |
            Self::Clever | Self::Loyal | Self::Lucky | Self::Brave |
            Self::Elemental | Self::Ethereal | Self::Blessed | Self::Awakened |
            Self::Primordial
        )
    }

    pub fn inheritance_chance(&self) -> f32 {
        match self {
            Self::Blessed | Self::Awakened | Self::Primordial => 0.15,
            Self::Cursed => 0.25,
            _ => 0.40,
        }
    }
}

// ============================================================================
// FAMILIAR INSTANCE
// ============================================================================

/// Experience required for each level
pub const XP_PER_LEVEL: [u32; 50] = [
    100, 200, 350, 550, 800, 1100, 1450, 1850, 2300, 2800,
    3400, 4100, 4900, 5800, 6800, 7900, 9100, 10400, 11800, 13300,
    15000, 16900, 19000, 21300, 23800, 26500, 29400, 32500, 35800, 39300,
    43100, 47200, 51600, 56300, 61300, 66600, 72200, 78100, 84300, 90800,
    97700, 105000, 112700, 120800, 129300, 138200, 147500, 157200, 167300, 177800,
];

/// Bond XP required for each bond level
pub const BOND_XP_PER_LEVEL: [u32; 20] = [
    50, 150, 300, 500, 750, 1050, 1400, 1800, 2250, 2750,
    3350, 4050, 4850, 5750, 6750, 7850, 9050, 10350, 11750, 13250,
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Familiar {
    pub id: u64,
    pub name: String,
    pub familiar_type: FamiliarType,

    // Position
    pub x: usize,
    pub y: usize,

    // Core stats
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,

    // Progression
    pub level: u32,
    pub xp: u32,
    pub xp_to_level: u32,

    // Bond system
    pub bond_level: u32,
    pub bond_xp: u32,
    pub bond_xp_to_level: u32,

    // Mood and loyalty
    pub happiness: i32,       // 0-100
    pub loyalty: i32,         // 0-100
    pub hunger: i32,          // 0-100 (100 = full)
    pub energy: i32,          // 0-100

    // Abilities
    pub abilities: Vec<FamiliarAbility>,
    pub ability_cooldowns: HashMap<FamiliarAbility, u32>,

    // Equipment
    pub equipment: PetEquipment,

    // Traits
    pub traits: Vec<FamiliarTrait>,

    // Combat tracking
    pub kills: u32,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub battles_fought: u32,

    // Breeding
    pub generation: u32,
    pub parent_ids: Option<(u64, u64)>,
    pub can_breed: bool,
    pub breed_cooldown: u32,

    // State
    pub is_active: bool,
    pub has_used_rebirth: bool,
}

impl Familiar {
    pub fn new(id: u64, familiar_type: FamiliarType, x: usize, y: usize) -> Self {
        let base_stats = familiar_type.base_stats();
        let rarity_mult = familiar_type.rarity().stat_multiplier();

        let max_hp = (base_stats.hp as f32 * rarity_mult) as i32;

        Self {
            id,
            name: familiar_type.name().to_string(),
            familiar_type,
            x,
            y,
            hp: max_hp,
            max_hp,
            attack: (base_stats.attack as f32 * rarity_mult) as i32,
            defense: (base_stats.defense as f32 * rarity_mult) as i32,
            speed: (base_stats.speed as f32 * rarity_mult) as i32,
            level: 1,
            xp: 0,
            xp_to_level: XP_PER_LEVEL[0],
            bond_level: 0,
            bond_xp: 0,
            bond_xp_to_level: BOND_XP_PER_LEVEL[0],
            happiness: 70,
            loyalty: 30,
            hunger: 80,
            energy: 100,
            abilities: vec![familiar_type.innate_ability()],
            ability_cooldowns: HashMap::new(),
            equipment: PetEquipment::default(),
            traits: Vec::new(),
            kills: 0,
            damage_dealt: 0,
            damage_taken: 0,
            battles_fought: 0,
            generation: 1,
            parent_ids: None,
            can_breed: true,
            breed_cooldown: 0,
            is_active: false,
            has_used_rebirth: false,
        }
    }

    pub fn new_with_name(id: u64, familiar_type: FamiliarType, name: String, x: usize, y: usize) -> Self {
        let mut familiar = Self::new(id, familiar_type, x, y);
        familiar.name = name;
        familiar
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn mood(&self) -> FamiliarMood {
        FamiliarMood::from_happiness(self.happiness)
    }

    pub fn loyalty_level(&self) -> LoyaltyLevel {
        LoyaltyLevel::from_loyalty(self.loyalty)
    }

    /// Get effective stats including equipment and mood bonuses
    pub fn effective_stats(&self) -> (i32, i32, i32, i32) {
        let equip_stats = self.equipment.total_stats();
        let mood_mult = self.mood().stat_modifier();
        let loyalty_mult = self.loyalty_level().damage_bonus();

        let trait_hp_mult = self.trait_stat_modifier("hp");
        let trait_atk_mult = self.trait_stat_modifier("attack");
        let trait_def_mult = self.trait_stat_modifier("defense");
        let trait_spd_mult = self.trait_stat_modifier("speed");

        let hp = ((self.max_hp + equip_stats.hp) as f32 * mood_mult * trait_hp_mult) as i32;
        let atk = ((self.attack + equip_stats.attack) as f32 * mood_mult * loyalty_mult * trait_atk_mult) as i32;
        let def = ((self.defense + equip_stats.defense) as f32 * mood_mult * trait_def_mult) as i32;
        let spd = ((self.speed + equip_stats.speed) as f32 * mood_mult * trait_spd_mult) as i32;

        (hp, atk, def, spd)
    }

    fn trait_stat_modifier(&self, stat: &str) -> f32 {
        let mut mult = 1.0;
        for t in &self.traits {
            match (t, stat) {
                (FamiliarTrait::Aggressive, "attack") => mult *= 1.15,
                (FamiliarTrait::Sturdy, "defense") => mult *= 1.15,
                (FamiliarTrait::Swift, "speed") => mult *= 1.15,
                (FamiliarTrait::Healthy, "hp") => mult *= 1.15,
                (FamiliarTrait::Timid, "attack") => mult *= 0.90,
                (FamiliarTrait::Fragile, "defense") => mult *= 0.90,
                (FamiliarTrait::Sluggish, "speed") => mult *= 0.90,
                (FamiliarTrait::Sickly, "hp") => mult *= 0.90,
                (FamiliarTrait::Blessed, _) => mult *= 1.10,
                (FamiliarTrait::Cursed, _) if stat != "attack" => mult *= 0.95,
                (FamiliarTrait::Cursed, "attack") => mult *= 1.25,
                _ => {}
            }
        }
        mult
    }

    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let (_, _, def, _) = self.effective_stats();
        let actual = (amount - def).max(1);
        self.hp -= actual;
        self.damage_taken += actual as u64;

        // Check for rebirth ability
        if self.hp <= 0 && !self.has_used_rebirth {
            if self.abilities.contains(&FamiliarAbility::Rebirth) {
                self.hp = self.max_hp / 2;
                self.has_used_rebirth = true;
                return -1; // Signal rebirth occurred
            }
        }

        actual
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn gain_xp(&mut self, amount: u32) -> bool {
        let clever_bonus = if self.traits.contains(&FamiliarTrait::Clever) { 1.2 } else { 1.0 };
        let adjusted = (amount as f32 * clever_bonus) as u32;

        self.xp += adjusted;

        if self.xp >= self.xp_to_level && self.level < 50 {
            self.xp -= self.xp_to_level;
            self.level += 1;

            if self.level < 50 {
                self.xp_to_level = XP_PER_LEVEL[self.level as usize - 1];
            }

            // Stat growth on level up
            let base = self.familiar_type.base_stats();
            self.max_hp += (base.hp as f32 * 0.1) as i32 + 3;
            self.hp = self.max_hp;
            self.attack += (base.attack as f32 * 0.08) as i32 + 1;
            self.defense += (base.defense as f32 * 0.08) as i32 + 1;
            self.speed += (base.speed as f32 * 0.05) as i32;

            // Learn new abilities at certain levels
            self.check_learn_abilities();

            return true;
        }
        false
    }

    pub fn gain_bond_xp(&mut self, amount: u32) -> bool {
        let loyal_bonus = if self.traits.contains(&FamiliarTrait::Loyal) { 1.25 } else { 1.0 };
        let aloof_penalty = if self.traits.contains(&FamiliarTrait::Aloof) { 0.75 } else { 1.0 };
        let adjusted = (amount as f32 * loyal_bonus * aloof_penalty) as u32;

        self.bond_xp += adjusted;

        if self.bond_xp >= self.bond_xp_to_level && self.bond_level < 20 {
            self.bond_xp -= self.bond_xp_to_level;
            self.bond_level += 1;

            if self.bond_level < 20 {
                self.bond_xp_to_level = BOND_XP_PER_LEVEL[self.bond_level as usize - 1];
            }

            // Increase loyalty with bond level
            self.loyalty = (self.loyalty + 5).min(100);

            // Unlock bond abilities at milestones
            self.check_bond_abilities();

            return true;
        }
        false
    }

    fn check_learn_abilities(&mut self) {
        // Learn abilities based on level and type
        let new_ability = match (self.level, self.familiar_type.category()) {
            (10, FamiliarCategory::Combat) => Some(FamiliarAbility::Fortify),
            (10, FamiliarCategory::Support) => Some(FamiliarAbility::Regeneration),
            (10, FamiliarCategory::Utility) => Some(FamiliarAbility::Detect),
            (10, FamiliarCategory::Legendary) => Some(FamiliarAbility::ElementalFusion),
            (20, FamiliarCategory::Combat) => Some(FamiliarAbility::CombinedStrike),
            (20, FamiliarCategory::Support) => Some(FamiliarAbility::DivineBlessing),
            (20, FamiliarCategory::Utility) => Some(FamiliarAbility::TreasureSense),
            (20, FamiliarCategory::Legendary) => Some(FamiliarAbility::VoidTouch),
            _ => None,
        };

        if let Some(ability) = new_ability {
            if !self.abilities.contains(&ability) {
                self.abilities.push(ability);
            }
        }
    }

    fn check_bond_abilities(&mut self) {
        let new_ability = match self.bond_level {
            5 => Some(FamiliarAbility::SharedVitality),
            10 => Some(FamiliarAbility::MindLink),
            15 => Some(FamiliarAbility::SoulBond),
            20 => Some(FamiliarAbility::PerfectHarmony),
            _ => None,
        };

        if let Some(ability) = new_ability {
            if !self.abilities.contains(&ability) {
                self.abilities.push(ability);
            }
        }
    }

    pub fn can_evolve(&self) -> bool {
        self.familiar_type.evolves_to().is_some() &&
            self.level >= self.familiar_type.evolution_level()
    }

    pub fn evolve(&mut self) -> bool {
        if let Some(evolved_type) = self.familiar_type.evolves_to() {
            if self.level >= self.familiar_type.evolution_level() {
                let old_base = self.familiar_type.base_stats();
                let new_base = evolved_type.base_stats();

                // Calculate stat improvements
                let hp_ratio = self.hp as f32 / self.max_hp as f32;
                self.max_hp += new_base.hp - old_base.hp;
                self.hp = (self.max_hp as f32 * hp_ratio) as i32;
                self.attack += new_base.attack - old_base.attack;
                self.defense += new_base.defense - old_base.defense;
                self.speed += new_base.speed - old_base.speed;

                // Get new innate ability
                let new_ability = evolved_type.innate_ability();
                if !self.abilities.contains(&new_ability) {
                    self.abilities.push(new_ability);
                }

                self.familiar_type = evolved_type;
                return true;
            }
        }
        false
    }

    pub fn feed(&mut self, food_quality: i32) {
        self.hunger = (self.hunger + food_quality).min(100);
        self.happiness = (self.happiness + food_quality / 2).min(100);
        self.gain_bond_xp(food_quality as u32);
    }

    pub fn rest(&mut self) {
        self.energy = 100;
        self.happiness = (self.happiness + 5).min(100);
    }

    pub fn play(&mut self) {
        if self.energy >= 20 {
            self.energy -= 20;
            self.happiness = (self.happiness + 15).min(100);
            self.gain_bond_xp(10);
        }
    }

    pub fn tick(&mut self) {
        // Reduce hunger over time
        self.hunger = (self.hunger - 1).max(0);

        // Happiness affected by hunger
        if self.hunger < 20 {
            self.happiness = (self.happiness - 2).max(0);
        }

        // Reduce ability cooldowns
        for cooldown in self.ability_cooldowns.values_mut() {
            *cooldown = cooldown.saturating_sub(1);
        }

        // Reduce breed cooldown
        self.breed_cooldown = self.breed_cooldown.saturating_sub(1);
    }

    pub fn can_use_ability(&self, ability: &FamiliarAbility) -> bool {
        self.abilities.contains(ability) &&
            self.ability_cooldowns.get(ability).copied().unwrap_or(0) == 0
    }

    pub fn use_ability(&mut self, ability: FamiliarAbility) {
        if self.can_use_ability(&ability) {
            self.ability_cooldowns.insert(ability, ability.cooldown());
        }
    }

    pub fn equip_collar(&mut self, collar: CollarType) -> Option<CollarType> {
        let old = self.equipment.collar.take();
        self.equipment.collar = Some(collar);
        old
    }

    pub fn equip_armor(&mut self, armor: ArmorType) -> Option<ArmorType> {
        let old = self.equipment.armor.take();
        self.equipment.armor = Some(armor);
        old
    }

    pub fn equip_accessory(&mut self, accessory: AccessoryType) -> Option<AccessoryType> {
        let old = self.equipment.accessory.take();
        self.equipment.accessory = Some(accessory);
        old
    }

    pub fn display_name(&self) -> String {
        format!("{} Lv.{}", self.name, self.level)
    }

    pub fn stats_summary(&self) -> String {
        let (hp, atk, def, spd) = self.effective_stats();
        format!(
            "{}: HP {}/{} ATK {} DEF {} SPD {} [{}]",
            self.display_name(),
            self.hp, hp,
            atk, def, spd,
            self.mood().name()
        )
    }
}

// ============================================================================
// BREEDING SYSTEM
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BreedingResult {
    pub offspring_type: FamiliarType,
    pub inherited_traits: Vec<FamiliarTrait>,
    pub new_traits: Vec<FamiliarTrait>,
    pub stat_bonuses: EquipmentStats,
    pub is_rare_offspring: bool,
}

pub struct BreedingSystem;

impl BreedingSystem {
    pub fn can_breed(parent1: &Familiar, parent2: &Familiar) -> bool {
        parent1.can_breed &&
            parent2.can_breed &&
            parent1.breed_cooldown == 0 &&
            parent2.breed_cooldown == 0 &&
            parent1.id != parent2.id &&
            parent1.level >= 10 &&
            parent2.level >= 10
    }

    pub fn breed(parent1: &mut Familiar, parent2: &mut Familiar, rng: &mut impl Rng) -> Option<BreedingResult> {
        if !Self::can_breed(parent1, parent2) {
            return None;
        }

        // Set breeding cooldown
        parent1.breed_cooldown = 100;
        parent2.breed_cooldown = 100;

        // Determine offspring type
        let offspring_type = Self::determine_offspring_type(parent1, parent2, rng);

        // Inherit traits from parents
        let inherited_traits = Self::inherit_traits(parent1, parent2, rng);

        // Chance for new random traits
        let new_traits = Self::generate_new_traits(rng);

        // Calculate stat bonuses from parents
        let stat_bonuses = Self::calculate_stat_bonuses(parent1, parent2);

        // Check for rare offspring
        let is_rare_offspring = rng.gen_ratio(1, 20); // 5% chance

        Some(BreedingResult {
            offspring_type,
            inherited_traits,
            new_traits,
            stat_bonuses,
            is_rare_offspring,
        })
    }

    fn determine_offspring_type(parent1: &Familiar, parent2: &Familiar, rng: &mut impl Rng) -> FamiliarType {
        // Same type = same offspring
        if parent1.familiar_type == parent2.familiar_type {
            return parent1.familiar_type;
        }

        // Same category = random from that category
        if parent1.familiar_type.category() == parent2.familiar_type.category() {
            let types: Vec<_> = FamiliarType::all()
                .into_iter()
                .filter(|t| t.category() == parent1.familiar_type.category())
                .collect();
            return types[rng.gen_range(0..types.len())];
        }

        // Different categories = rare hybrid chance
        if rng.gen_ratio(1, 10) {
            // Legendary offspring possible!
            let legendaries: Vec<_> = FamiliarType::all()
                .into_iter()
                .filter(|t| t.category() == FamiliarCategory::Legendary)
                .collect();
            return legendaries[rng.gen_range(0..legendaries.len())];
        }

        // Otherwise random from either parent's type
        if rng.gen_bool(0.5) {
            parent1.familiar_type
        } else {
            parent2.familiar_type
        }
    }

    fn inherit_traits(parent1: &Familiar, parent2: &Familiar, rng: &mut impl Rng) -> Vec<FamiliarTrait> {
        let mut inherited = Vec::new();

        for t in &parent1.traits {
            if rng.gen_bool(t.inheritance_chance() as f64) {
                inherited.push(*t);
            }
        }

        for t in &parent2.traits {
            if !inherited.contains(t) && rng.gen_bool(t.inheritance_chance() as f64) {
                inherited.push(*t);
            }
        }

        // Limit to 4 inherited traits
        inherited.truncate(4);
        inherited
    }

    fn generate_new_traits(rng: &mut impl Rng) -> Vec<FamiliarTrait> {
        let mut new_traits = Vec::new();

        // 20% chance for a new positive trait
        if rng.gen_ratio(1, 5) {
            let positive_traits = [
                FamiliarTrait::Aggressive, FamiliarTrait::Sturdy,
                FamiliarTrait::Swift, FamiliarTrait::Healthy,
                FamiliarTrait::Clever, FamiliarTrait::Loyal,
                FamiliarTrait::Lucky, FamiliarTrait::Brave,
            ];
            new_traits.push(positive_traits[rng.gen_range(0..positive_traits.len())]);
        }

        // 5% chance for rare trait
        if rng.gen_ratio(1, 20) {
            let rare_traits = [
                FamiliarTrait::Blessed, FamiliarTrait::Awakened,
                FamiliarTrait::Elemental, FamiliarTrait::Ethereal,
            ];
            new_traits.push(rare_traits[rng.gen_range(0..rare_traits.len())]);
        }

        // 1% chance for Primordial
        if rng.gen_ratio(1, 100) {
            new_traits.push(FamiliarTrait::Primordial);
        }

        new_traits
    }

    fn calculate_stat_bonuses(parent1: &Familiar, parent2: &Familiar) -> EquipmentStats {
        // Offspring gets small bonus based on parent stats
        EquipmentStats {
            hp: (parent1.max_hp + parent2.max_hp) / 20,
            attack: (parent1.attack + parent2.attack) / 20,
            defense: (parent1.defense + parent2.defense) / 20,
            speed: (parent1.speed + parent2.speed) / 20,
        }
    }

    pub fn create_offspring(
        result: &BreedingResult,
        id: u64,
        parent1_id: u64,
        parent2_id: u64,
        generation: u32,
    ) -> Familiar {
        let mut offspring = Familiar::new(id, result.offspring_type, 0, 0);

        // Apply inherited traits
        offspring.traits = result.inherited_traits.clone();
        offspring.traits.extend(result.new_traits.clone());

        // Apply stat bonuses
        offspring.max_hp += result.stat_bonuses.hp;
        offspring.hp = offspring.max_hp;
        offspring.attack += result.stat_bonuses.attack;
        offspring.defense += result.stat_bonuses.defense;
        offspring.speed += result.stat_bonuses.speed;

        // Set lineage
        offspring.generation = generation + 1;
        offspring.parent_ids = Some((parent1_id, parent2_id));

        // Rare offspring get bonus
        if result.is_rare_offspring {
            offspring.max_hp = (offspring.max_hp as f32 * 1.1) as i32;
            offspring.hp = offspring.max_hp;
            offspring.attack = (offspring.attack as f32 * 1.1) as i32;
            offspring.defense = (offspring.defense as f32 * 1.1) as i32;
            if !offspring.traits.contains(&FamiliarTrait::Blessed) {
                offspring.traits.push(FamiliarTrait::Blessed);
            }
        }

        offspring
    }
}

// ============================================================================
// FAMILIAR SYSTEM
// ============================================================================

/// Maximum number of active familiars
pub const MAX_ACTIVE_FAMILIARS: usize = 3;
/// Maximum number of familiars in storage
pub const MAX_STORED_FAMILIARS: usize = 50;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FamiliarSystem {
    pub familiars: HashMap<u64, Familiar>,
    pub active_familiar_ids: Vec<u64>,
    pub next_id: u64,

    // Unlocks and progression
    pub total_familiars_caught: u32,
    pub total_familiars_bred: u32,
    pub total_evolutions: u32,
    pub discovered_types: Vec<FamiliarType>,

    // Settings
    pub auto_collect_enabled: bool,
    pub share_xp_enabled: bool,
}

impl Default for FamiliarSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FamiliarSystem {
    pub fn new() -> Self {
        Self {
            familiars: HashMap::new(),
            active_familiar_ids: Vec::new(),
            next_id: 1,
            total_familiars_caught: 0,
            total_familiars_bred: 0,
            total_evolutions: 0,
            discovered_types: Vec::new(),
            auto_collect_enabled: false,
            share_xp_enabled: true,
        }
    }

    pub fn add_familiar(&mut self, mut familiar: Familiar) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        familiar.id = id;

        // Track discovery
        if !self.discovered_types.contains(&familiar.familiar_type) {
            self.discovered_types.push(familiar.familiar_type);
        }

        self.familiars.insert(id, familiar);
        self.total_familiars_caught += 1;
        id
    }

    pub fn remove_familiar(&mut self, id: u64) -> Option<Familiar> {
        self.active_familiar_ids.retain(|&fid| fid != id);
        self.familiars.remove(&id)
    }

    pub fn get_familiar(&self, id: u64) -> Option<&Familiar> {
        self.familiars.get(&id)
    }

    pub fn get_familiar_mut(&mut self, id: u64) -> Option<&mut Familiar> {
        self.familiars.get_mut(&id)
    }

    pub fn set_active(&mut self, id: u64) -> bool {
        if self.familiars.contains_key(&id) {
            if !self.active_familiar_ids.contains(&id) {
                if self.active_familiar_ids.len() < MAX_ACTIVE_FAMILIARS {
                    self.active_familiar_ids.push(id);
                    if let Some(familiar) = self.familiars.get_mut(&id) {
                        familiar.is_active = true;
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn set_inactive(&mut self, id: u64) -> bool {
        if let Some(pos) = self.active_familiar_ids.iter().position(|&fid| fid == id) {
            self.active_familiar_ids.remove(pos);
            if let Some(familiar) = self.familiars.get_mut(&id) {
                familiar.is_active = false;
            }
            return true;
        }
        false
    }

    pub fn get_active_familiars(&self) -> Vec<&Familiar> {
        self.active_familiar_ids
            .iter()
            .filter_map(|&id| self.familiars.get(&id))
            .collect()
    }

    pub fn get_active_familiars_mut(&mut self) -> Vec<&mut Familiar> {
        let ids: Vec<u64> = self.active_familiar_ids.clone();
        self.familiars
            .iter_mut()
            .filter(|(id, _)| ids.contains(id))
            .map(|(_, f)| f)
            .collect()
    }

    pub fn breed_familiars(&mut self, id1: u64, id2: u64, rng: &mut impl Rng) -> Option<u64> {
        // Get mutable references carefully
        let parent1 = self.familiars.get(&id1)?.clone();
        let parent2 = self.familiars.get(&id2)?.clone();

        let mut p1 = parent1;
        let mut p2 = parent2;

        let result = BreedingSystem::breed(&mut p1, &mut p2, rng)?;

        // Update parents
        if let Some(familiar) = self.familiars.get_mut(&id1) {
            familiar.breed_cooldown = p1.breed_cooldown;
        }
        if let Some(familiar) = self.familiars.get_mut(&id2) {
            familiar.breed_cooldown = p2.breed_cooldown;
        }

        let generation = p1.generation.max(p2.generation);
        let offspring = BreedingSystem::create_offspring(&result, 0, id1, id2, generation);

        let offspring_id = self.add_familiar(offspring);
        self.total_familiars_bred += 1;

        Some(offspring_id)
    }

    pub fn evolve_familiar(&mut self, id: u64) -> bool {
        if let Some(familiar) = self.familiars.get_mut(&id) {
            if familiar.evolve() {
                self.total_evolutions += 1;

                // Track new type discovery
                if !self.discovered_types.contains(&familiar.familiar_type) {
                    self.discovered_types.push(familiar.familiar_type);
                }
                return true;
            }
        }
        false
    }

    pub fn distribute_xp(&mut self, xp: u32) {
        if !self.share_xp_enabled || self.active_familiar_ids.is_empty() {
            return;
        }

        let xp_per_familiar = xp / self.active_familiar_ids.len() as u32;
        let ids: Vec<u64> = self.active_familiar_ids.clone();

        for id in ids {
            if let Some(familiar) = self.familiars.get_mut(&id) {
                familiar.gain_xp(xp_per_familiar);
            }
        }
    }

    pub fn distribute_bond_xp(&mut self, xp: u32) {
        let ids: Vec<u64> = self.active_familiar_ids.clone();

        for id in ids {
            if let Some(familiar) = self.familiars.get_mut(&id) {
                familiar.gain_bond_xp(xp);
            }
        }
    }

    pub fn tick_all(&mut self) {
        for familiar in self.familiars.values_mut() {
            familiar.tick();
        }
    }

    pub fn reset_battle_state(&mut self) {
        for familiar in self.familiars.values_mut() {
            familiar.has_used_rebirth = false;
            familiar.ability_cooldowns.clear();
        }
    }

    pub fn get_familiars_by_category(&self, category: FamiliarCategory) -> Vec<&Familiar> {
        self.familiars
            .values()
            .filter(|f| f.familiar_type.category() == category)
            .collect()
    }

    pub fn get_familiars_by_rarity(&self, rarity: FamiliarRarity) -> Vec<&Familiar> {
        self.familiars
            .values()
            .filter(|f| f.familiar_type.rarity() == rarity)
            .collect()
    }

    pub fn storage_count(&self) -> usize {
        self.familiars.len()
    }

    pub fn is_storage_full(&self) -> bool {
        self.familiars.len() >= MAX_STORED_FAMILIARS
    }

    pub fn discovery_progress(&self) -> (usize, usize) {
        (self.discovered_types.len(), FamiliarType::all().len())
    }

    /// Generate a random familiar appropriate for the dungeon level
    pub fn generate_wild_familiar(&self, dungeon_level: u32, rng: &mut impl Rng) -> Familiar {
        // Determine rarity based on level
        let rarity = Self::roll_rarity(dungeon_level, rng);

        // Get all types of that rarity
        let available: Vec<_> = FamiliarType::all()
            .into_iter()
            .filter(|t| t.rarity() == rarity)
            .collect();

        let familiar_type = available[rng.gen_range(0..available.len())];
        let mut familiar = Familiar::new(0, familiar_type, 0, 0);

        // Scale level based on dungeon level
        let target_level = (dungeon_level / 2).max(1).min(30);
        for _ in 1..target_level {
            familiar.gain_xp(familiar.xp_to_level);
        }

        // Random traits
        if rng.gen_ratio(1, 3) {
            let positive_traits = [
                FamiliarTrait::Aggressive, FamiliarTrait::Sturdy,
                FamiliarTrait::Swift, FamiliarTrait::Healthy,
            ];
            familiar.traits.push(positive_traits[rng.gen_range(0..positive_traits.len())]);
        }

        familiar
    }

    fn roll_rarity(dungeon_level: u32, rng: &mut impl Rng) -> FamiliarRarity {
        let legendary_threshold = 10 + dungeon_level / 5;
        let epic_threshold = legendary_threshold + 40 + dungeon_level / 3;
        let rare_threshold = epic_threshold + 150 + dungeon_level;
        let uncommon_threshold = rare_threshold + 400;

        let roll = rng.gen_range(0..1600);

        if roll < legendary_threshold {
            FamiliarRarity::Legendary
        } else if roll < epic_threshold {
            FamiliarRarity::Epic
        } else if roll < rare_threshold {
            FamiliarRarity::Rare
        } else if roll < uncommon_threshold {
            FamiliarRarity::Uncommon
        } else {
            FamiliarRarity::Common
        }
    }

    pub fn summary(&self) -> String {
        let (discovered, total) = self.discovery_progress();
        format!(
            "Familiars: {}/{} stored, {}/{} active, {}/{} discovered",
            self.storage_count(), MAX_STORED_FAMILIARS,
            self.active_familiar_ids.len(), MAX_ACTIVE_FAMILIARS,
            discovered, total
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_familiar_creation() {
        let familiar = Familiar::new(1, FamiliarType::Wolf, 5, 5);
        assert_eq!(familiar.level, 1);
        assert!(familiar.is_alive());
        assert_eq!(familiar.familiar_type.category(), FamiliarCategory::Combat);
    }

    #[test]
    fn test_familiar_leveling() {
        let mut familiar = Familiar::new(1, FamiliarType::Cat, 0, 0);
        let initial_hp = familiar.max_hp;
        let initial_attack = familiar.attack;

        // Level up
        familiar.xp = familiar.xp_to_level;
        let leveled = familiar.gain_xp(1);

        assert!(leveled);
        assert_eq!(familiar.level, 2);
        assert!(familiar.max_hp > initial_hp);
        assert!(familiar.attack > initial_attack);
    }

    #[test]
    fn test_familiar_evolution() {
        let mut familiar = Familiar::new(1, FamiliarType::Wolf, 0, 0);

        // Not ready to evolve
        assert!(!familiar.can_evolve());

        // Level up to evolution threshold
        familiar.level = 15;
        assert!(familiar.can_evolve());

        // Evolve
        assert!(familiar.evolve());
        assert_eq!(familiar.familiar_type, FamiliarType::DireWolf);
    }

    #[test]
    fn test_mood_system() {
        let mut familiar = Familiar::new(1, FamiliarType::Fairy, 0, 0);

        familiar.happiness = 95;
        assert_eq!(familiar.mood(), FamiliarMood::Ecstatic);

        familiar.happiness = 50;
        assert_eq!(familiar.mood(), FamiliarMood::Content);

        familiar.happiness = 10;
        assert_eq!(familiar.mood(), FamiliarMood::Miserable);
    }

    #[test]
    fn test_equipment() {
        let mut familiar = Familiar::new(1, FamiliarType::Bear, 0, 0);
        let (_, base_atk, base_def, _) = familiar.effective_stats();

        familiar.equip_collar(CollarType::SpikedCollar);
        familiar.equip_armor(ArmorType::LeatherBarding);

        let (_, new_atk, new_def, _) = familiar.effective_stats();
        assert!(new_atk > base_atk);
        assert!(new_def > base_def);
    }

    #[test]
    fn test_bond_system() {
        let mut familiar = Familiar::new(1, FamiliarType::Pixie, 0, 0);
        assert_eq!(familiar.bond_level, 0);

        // Gain bond xp
        familiar.bond_xp = familiar.bond_xp_to_level;
        let bonded = familiar.gain_bond_xp(1);

        assert!(bonded);
        assert_eq!(familiar.bond_level, 1);
    }

    #[test]
    fn test_familiar_system() {
        let mut system = FamiliarSystem::new();

        let wolf = Familiar::new(0, FamiliarType::Wolf, 0, 0);
        let id = system.add_familiar(wolf);

        assert!(system.set_active(id));
        assert_eq!(system.get_active_familiars().len(), 1);

        assert!(system.set_inactive(id));
        assert_eq!(system.get_active_familiars().len(), 0);
    }

    #[test]
    fn test_trait_modifiers() {
        let mut familiar = Familiar::new(1, FamiliarType::Tiger, 0, 0);
        let (_, base_atk, _, _) = familiar.effective_stats();

        familiar.traits.push(FamiliarTrait::Aggressive);
        let (_, new_atk, _, _) = familiar.effective_stats();

        assert!(new_atk > base_atk);
    }

    #[test]
    fn test_all_types_have_abilities() {
        for ftype in FamiliarType::all() {
            let ability = ftype.innate_ability();
            assert!(!ability.name().is_empty());
        }
    }

    #[test]
    fn test_rarity_hierarchy() {
        assert!(FamiliarRarity::Common < FamiliarRarity::Uncommon);
        assert!(FamiliarRarity::Uncommon < FamiliarRarity::Rare);
        assert!(FamiliarRarity::Rare < FamiliarRarity::Epic);
        assert!(FamiliarRarity::Epic < FamiliarRarity::Legendary);
    }

    #[test]
    fn test_rebirth_ability() {
        let mut familiar = Familiar::new(1, FamiliarType::PhoenixChick, 0, 0);
        assert!(familiar.abilities.contains(&FamiliarAbility::Rebirth));
        assert!(!familiar.has_used_rebirth);

        // Deal lethal damage
        let result = familiar.take_damage(familiar.hp + 100);

        assert_eq!(result, -1); // Rebirth signal
        assert!(familiar.is_alive());
        assert!(familiar.has_used_rebirth);
    }
}

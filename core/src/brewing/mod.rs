//! Potion Brewing System
//!
//! This module provides a comprehensive brewing system for creating potions
//! from ingredients. It supports:
//! - Various ingredients with different properties and rarities
//! - Defined recipes for crafting known potions
//! - Brewing stations with different capabilities and bonuses
//! - Experimental brewing with random and unpredictable effects
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use shadowcrypt_core::brewing::*;
//!
//! // Create a brewing station
//! let mut station = BrewingStation::new(StationType::Alchemist, 5, 5);
//!
//! // Add ingredients
//! station.add_ingredient(Ingredient::new(IngredientKind::Moonpetal, IngredientQuality::Fine));
//! station.add_ingredient(Ingredient::new(IngredientKind::CrystalDust, IngredientQuality::Common));
//!
//! // Attempt to brew
//! let result = station.brew();
//! ```

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// ============================================================================
// Ingredient System
// ============================================================================

/// Categories of ingredients affecting their brewing properties
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum IngredientCategory {
    /// Herbs and plant materials
    Botanical,
    /// Minerals and crystalline substances
    Mineral,
    /// Parts from creatures
    Creature,
    /// Magical essences and extracts
    Essence,
    /// Rare alchemical reagents
    Reagent,
    /// Forbidden or dangerous materials
    Forbidden,
}

impl IngredientCategory {
    /// Returns the display name for this category
    pub fn name(&self) -> &'static str {
        match self {
            Self::Botanical => "Botanical",
            Self::Mineral => "Mineral",
            Self::Creature => "Creature Part",
            Self::Essence => "Magical Essence",
            Self::Reagent => "Alchemical Reagent",
            Self::Forbidden => "Forbidden Material",
        }
    }

    /// Returns the base instability factor for experimental brewing
    pub fn instability(&self) -> f32 {
        match self {
            Self::Botanical => 0.1,
            Self::Mineral => 0.15,
            Self::Creature => 0.2,
            Self::Essence => 0.25,
            Self::Reagent => 0.3,
            Self::Forbidden => 0.5,
        }
    }
}

/// Quality tiers for ingredients affecting potion potency
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum IngredientQuality {
    Ruined,
    Poor,
    Common,
    Fine,
    Superior,
    Pristine,
    Perfect,
}

impl IngredientQuality {
    /// Returns the potency multiplier for this quality
    pub fn potency_multiplier(&self) -> f32 {
        match self {
            Self::Ruined => 0.25,
            Self::Poor => 0.5,
            Self::Common => 1.0,
            Self::Fine => 1.25,
            Self::Superior => 1.5,
            Self::Pristine => 2.0,
            Self::Perfect => 3.0,
        }
    }

    /// Returns the success rate modifier for brewing
    pub fn success_modifier(&self) -> f32 {
        match self {
            Self::Ruined => -0.3,
            Self::Poor => -0.15,
            Self::Common => 0.0,
            Self::Fine => 0.1,
            Self::Superior => 0.2,
            Self::Pristine => 0.3,
            Self::Perfect => 0.5,
        }
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ruined => "Ruined",
            Self::Poor => "Poor",
            Self::Common => "Common",
            Self::Fine => "Fine",
            Self::Superior => "Superior",
            Self::Pristine => "Pristine",
            Self::Perfect => "Perfect",
        }
    }

    /// Returns a color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Ruined => 0,      // DarkGrey
            Self::Poor => 1,        // Grey
            Self::Common => 2,      // White
            Self::Fine => 5,        // Green
            Self::Superior => 7,    // Blue
            Self::Pristine => 13,   // Magenta
            Self::Perfect => 11,    // Yellow (Gold)
        }
    }
}

/// All ingredient types available in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum IngredientKind {
    // Botanical (15 types)
    Moonpetal,
    Sunbloom,
    Shadowroot,
    Thornvine,
    Mistcap,
    Glowmoss,
    Firebloom,
    Frostfern,
    Venomleaf,
    Healwort,
    Nightshade,
    Dreamweed,
    Bloodthorn,
    Spiritbark,
    Voidmoss,

    // Mineral (12 types)
    CrystalDust,
    IronFilings,
    SulfurPowder,
    SaltPeter,
    MoonStone,
    SunStone,
    ObsidianShard,
    DiamondDust,
    EmeraldPowder,
    RubyEssence,
    SapphireDust,
    AmethystCrystal,

    // Creature (15 types)
    SpiderVenom,
    SnakeFang,
    BatWing,
    TrollBlood,
    PhoenixFeather,
    DragonScale,
    UnicornHair,
    BasiliskEye,
    ManticoreStinger,
    GriffinClaw,
    HydraScale,
    WyrmTooth,
    SalamanderHeart,
    GhostEctoplasm,
    DemonIchor,

    // Essence (10 types)
    PureWater,
    LiquidFire,
    BottledLightning,
    FrozenBreath,
    ShadowMist,
    LiquidLight,
    VoidEssence,
    TimeDroplet,
    LifeEssence,
    DeathEssence,

    // Reagent (8 types)
    PhilosophersSalt,
    AlchemistMercury,
    PrimordialSulfur,
    QuintessenceOil,
    UniversalSolvent,
    TransmutationCatalyst,
    StabilizingAgent,
    PotencyEnhancer,

    // Forbidden (6 types)
    SoulFragment,
    DemonicAsh,
    CursedBlood,
    VoidTear,
    ChaosShard,
    ForbiddenElixir,
}

impl IngredientKind {
    /// Returns the category of this ingredient
    pub fn category(&self) -> IngredientCategory {
        match self {
            // Botanical
            Self::Moonpetal | Self::Sunbloom | Self::Shadowroot | Self::Thornvine
            | Self::Mistcap | Self::Glowmoss | Self::Firebloom | Self::Frostfern
            | Self::Venomleaf | Self::Healwort | Self::Nightshade | Self::Dreamweed
            | Self::Bloodthorn | Self::Spiritbark | Self::Voidmoss => IngredientCategory::Botanical,

            // Mineral
            Self::CrystalDust | Self::IronFilings | Self::SulfurPowder | Self::SaltPeter
            | Self::MoonStone | Self::SunStone | Self::ObsidianShard | Self::DiamondDust
            | Self::EmeraldPowder | Self::RubyEssence | Self::SapphireDust
            | Self::AmethystCrystal => IngredientCategory::Mineral,

            // Creature
            Self::SpiderVenom | Self::SnakeFang | Self::BatWing | Self::TrollBlood
            | Self::PhoenixFeather | Self::DragonScale | Self::UnicornHair
            | Self::BasiliskEye | Self::ManticoreStinger | Self::GriffinClaw
            | Self::HydraScale | Self::WyrmTooth | Self::SalamanderHeart
            | Self::GhostEctoplasm | Self::DemonIchor => IngredientCategory::Creature,

            // Essence
            Self::PureWater | Self::LiquidFire | Self::BottledLightning
            | Self::FrozenBreath | Self::ShadowMist | Self::LiquidLight
            | Self::VoidEssence | Self::TimeDroplet | Self::LifeEssence
            | Self::DeathEssence => IngredientCategory::Essence,

            // Reagent
            Self::PhilosophersSalt | Self::AlchemistMercury | Self::PrimordialSulfur
            | Self::QuintessenceOil | Self::UniversalSolvent | Self::TransmutationCatalyst
            | Self::StabilizingAgent | Self::PotencyEnhancer => IngredientCategory::Reagent,

            // Forbidden
            Self::SoulFragment | Self::DemonicAsh | Self::CursedBlood
            | Self::VoidTear | Self::ChaosShard | Self::ForbiddenElixir => IngredientCategory::Forbidden,
        }
    }

    /// Returns the display name of this ingredient
    pub fn name(&self) -> &'static str {
        match self {
            Self::Moonpetal => "Moonpetal",
            Self::Sunbloom => "Sunbloom",
            Self::Shadowroot => "Shadowroot",
            Self::Thornvine => "Thornvine",
            Self::Mistcap => "Mistcap Mushroom",
            Self::Glowmoss => "Glowmoss",
            Self::Firebloom => "Firebloom",
            Self::Frostfern => "Frost Fern",
            Self::Venomleaf => "Venomleaf",
            Self::Healwort => "Healwort",
            Self::Nightshade => "Nightshade",
            Self::Dreamweed => "Dreamweed",
            Self::Bloodthorn => "Bloodthorn",
            Self::Spiritbark => "Spirit Bark",
            Self::Voidmoss => "Void Moss",

            Self::CrystalDust => "Crystal Dust",
            Self::IronFilings => "Iron Filings",
            Self::SulfurPowder => "Sulfur Powder",
            Self::SaltPeter => "Saltpeter",
            Self::MoonStone => "Moon Stone",
            Self::SunStone => "Sun Stone",
            Self::ObsidianShard => "Obsidian Shard",
            Self::DiamondDust => "Diamond Dust",
            Self::EmeraldPowder => "Emerald Powder",
            Self::RubyEssence => "Ruby Essence",
            Self::SapphireDust => "Sapphire Dust",
            Self::AmethystCrystal => "Amethyst Crystal",

            Self::SpiderVenom => "Spider Venom",
            Self::SnakeFang => "Snake Fang",
            Self::BatWing => "Bat Wing",
            Self::TrollBlood => "Troll Blood",
            Self::PhoenixFeather => "Phoenix Feather",
            Self::DragonScale => "Dragon Scale",
            Self::UnicornHair => "Unicorn Hair",
            Self::BasiliskEye => "Basilisk Eye",
            Self::ManticoreStinger => "Manticore Stinger",
            Self::GriffinClaw => "Griffin Claw",
            Self::HydraScale => "Hydra Scale",
            Self::WyrmTooth => "Wyrm Tooth",
            Self::SalamanderHeart => "Salamander Heart",
            Self::GhostEctoplasm => "Ghost Ectoplasm",
            Self::DemonIchor => "Demon Ichor",

            Self::PureWater => "Pure Water",
            Self::LiquidFire => "Liquid Fire",
            Self::BottledLightning => "Bottled Lightning",
            Self::FrozenBreath => "Frozen Breath",
            Self::ShadowMist => "Shadow Mist",
            Self::LiquidLight => "Liquid Light",
            Self::VoidEssence => "Void Essence",
            Self::TimeDroplet => "Time Droplet",
            Self::LifeEssence => "Life Essence",
            Self::DeathEssence => "Death Essence",

            Self::PhilosophersSalt => "Philosopher's Salt",
            Self::AlchemistMercury => "Alchemist's Mercury",
            Self::PrimordialSulfur => "Primordial Sulfur",
            Self::QuintessenceOil => "Quintessence Oil",
            Self::UniversalSolvent => "Universal Solvent",
            Self::TransmutationCatalyst => "Transmutation Catalyst",
            Self::StabilizingAgent => "Stabilizing Agent",
            Self::PotencyEnhancer => "Potency Enhancer",

            Self::SoulFragment => "Soul Fragment",
            Self::DemonicAsh => "Demonic Ash",
            Self::CursedBlood => "Cursed Blood",
            Self::VoidTear => "Void Tear",
            Self::ChaosShard => "Chaos Shard",
            Self::ForbiddenElixir => "Forbidden Elixir",
        }
    }

    /// Returns the glyph character for this ingredient
    pub fn glyph(&self) -> char {
        match self.category() {
            IngredientCategory::Botanical => '&',
            IngredientCategory::Mineral => '*',
            IngredientCategory::Creature => '~',
            IngredientCategory::Essence => '@',
            IngredientCategory::Reagent => '+',
            IngredientCategory::Forbidden => '#',
        }
    }

    /// Returns the base value of this ingredient in gold
    pub fn base_value(&self) -> i32 {
        match self.category() {
            IngredientCategory::Botanical => 5,
            IngredientCategory::Mineral => 10,
            IngredientCategory::Creature => 20,
            IngredientCategory::Essence => 35,
            IngredientCategory::Reagent => 50,
            IngredientCategory::Forbidden => 100,
        }
    }

    /// Returns the primary effect property of this ingredient
    pub fn primary_property(&self) -> IngredientProperty {
        match self {
            // Healing properties
            Self::Healwort | Self::LifeEssence | Self::UnicornHair
            | Self::PhoenixFeather => IngredientProperty::Healing,

            // Mana properties
            Self::Moonpetal | Self::MoonStone | Self::CrystalDust
            | Self::SapphireDust => IngredientProperty::ManaRestore,

            // Strength properties
            Self::Bloodthorn | Self::TrollBlood | Self::GriffinClaw
            | Self::DragonScale => IngredientProperty::Strength,

            // Defense properties
            Self::IronFilings | Self::HydraScale | Self::ObsidianShard
            | Self::StabilizingAgent => IngredientProperty::Defense,

            // Speed properties
            Self::Sunbloom | Self::SunStone | Self::BottledLightning
            | Self::BatWing => IngredientProperty::Speed,

            // Fire properties
            Self::Firebloom | Self::LiquidFire | Self::SalamanderHeart
            | Self::RubyEssence | Self::PrimordialSulfur => IngredientProperty::Fire,

            // Ice properties
            Self::Frostfern | Self::FrozenBreath | Self::SapphireDust => IngredientProperty::Ice,

            // Poison properties
            Self::Venomleaf | Self::SpiderVenom | Self::SnakeFang
            | Self::ManticoreStinger | Self::Nightshade => IngredientProperty::Poison,

            // Invisibility properties
            Self::Shadowroot | Self::ShadowMist | Self::GhostEctoplasm
            | Self::Mistcap => IngredientProperty::Invisibility,

            // Luck properties
            Self::EmeraldPowder | Self::DiamondDust | Self::AmethystCrystal
            | Self::Glowmoss => IngredientProperty::Luck,

            // Vision properties
            Self::BasiliskEye | Self::LiquidLight | Self::Dreamweed => IngredientProperty::Vision,

            // Regeneration properties
            Self::Spiritbark | Self::Thornvine | Self::WyrmTooth => IngredientProperty::Regeneration,

            // Transmutation properties
            Self::PhilosophersSalt | Self::AlchemistMercury | Self::QuintessenceOil
            | Self::TransmutationCatalyst | Self::UniversalSolvent => IngredientProperty::Transmutation,

            // Chaos/Void properties
            Self::Voidmoss | Self::VoidEssence | Self::VoidTear
            | Self::ChaosShard => IngredientProperty::Chaos,

            // Death/Dark properties
            Self::DeathEssence | Self::DemonicAsh | Self::DemonIchor
            | Self::CursedBlood | Self::SoulFragment => IngredientProperty::Dark,

            // Time properties
            Self::TimeDroplet => IngredientProperty::Time,

            // Potency enhancement
            Self::PotencyEnhancer | Self::ForbiddenElixir => IngredientProperty::Potency,

            // Default
            _ => IngredientProperty::Neutral,
        }
    }

    /// Returns secondary properties of this ingredient
    pub fn secondary_properties(&self) -> Vec<IngredientProperty> {
        match self {
            Self::PhoenixFeather => vec![IngredientProperty::Fire, IngredientProperty::Regeneration],
            Self::DragonScale => vec![IngredientProperty::Fire, IngredientProperty::Defense],
            Self::TrollBlood => vec![IngredientProperty::Regeneration],
            Self::UnicornHair => vec![IngredientProperty::Luck, IngredientProperty::Potency],
            Self::Nightshade => vec![IngredientProperty::Dark, IngredientProperty::Invisibility],
            Self::Dreamweed => vec![IngredientProperty::Invisibility],
            Self::SalamanderHeart => vec![IngredientProperty::Regeneration],
            Self::VoidEssence => vec![IngredientProperty::Dark, IngredientProperty::Time],
            Self::ChaosShard => vec![IngredientProperty::Dark, IngredientProperty::Potency],
            Self::ForbiddenElixir => vec![IngredientProperty::Chaos, IngredientProperty::Dark],
            Self::DemonicAsh => vec![IngredientProperty::Fire],
            Self::TimeDroplet => vec![IngredientProperty::Potency],
            _ => vec![],
        }
    }
}

/// Properties that ingredients can impart to potions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum IngredientProperty {
    Healing,
    ManaRestore,
    Strength,
    Defense,
    Speed,
    Fire,
    Ice,
    Poison,
    Invisibility,
    Luck,
    Vision,
    Regeneration,
    Transmutation,
    Chaos,
    Dark,
    Time,
    Potency,
    Neutral,
}

impl IngredientProperty {
    /// Returns display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Healing => "Healing",
            Self::ManaRestore => "Mana Restoration",
            Self::Strength => "Strength",
            Self::Defense => "Defense",
            Self::Speed => "Speed",
            Self::Fire => "Fire",
            Self::Ice => "Ice",
            Self::Poison => "Poison",
            Self::Invisibility => "Invisibility",
            Self::Luck => "Luck",
            Self::Vision => "Vision",
            Self::Regeneration => "Regeneration",
            Self::Transmutation => "Transmutation",
            Self::Chaos => "Chaos",
            Self::Dark => "Dark",
            Self::Time => "Time",
            Self::Potency => "Potency",
            Self::Neutral => "Neutral",
        }
    }

    /// Returns conflicting properties that reduce brewing success
    pub fn conflicts_with(&self) -> Vec<IngredientProperty> {
        match self {
            Self::Fire => vec![Self::Ice],
            Self::Ice => vec![Self::Fire],
            Self::Healing => vec![Self::Poison, Self::Dark],
            Self::Dark => vec![Self::Healing, Self::Vision],
            Self::Chaos => vec![Self::Defense, Self::Time],
            Self::Time => vec![Self::Chaos],
            _ => vec![],
        }
    }
}

/// An ingredient instance with quality
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub kind: IngredientKind,
    pub quality: IngredientQuality,
}

impl Ingredient {
    /// Create a new ingredient
    pub fn new(kind: IngredientKind, quality: IngredientQuality) -> Self {
        Self { kind, quality }
    }

    /// Returns the display name with quality prefix
    pub fn display_name(&self) -> String {
        format!("{} {}", self.quality.name(), self.kind.name())
    }

    /// Returns the gold value of this ingredient
    pub fn value(&self) -> i32 {
        (self.kind.base_value() as f32 * self.quality.potency_multiplier()) as i32
    }
}

// ============================================================================
// Recipe System
// ============================================================================

/// Result types from brewing a potion
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum BrewedPotionType {
    // Standard potions (matching existing ItemKind potions)
    HealthPotion,
    ManaPotion,
    StrengthPotion,
    DefensePotion,
    SpeedPotion,
    InvisibilityPotion,
    FireResistPotion,
    IceResistPotion,
    PoisonResistPotion,
    RegenerationPotion,
    BerserkPotion,
    GiantPotion,
    LevitationPotion,
    XPPotion,
    FullRestorePotion,
    LuckPotion,
    CriticalPotion,
    VisionPotion,
    CureAllPotion,
    UltimatePowerPotion,

    // Brewing-exclusive potions
    AlchemistFire,
    FrostBomb,
    SmokeBomb,
    LiquidCourage,
    MindClarity,
    WarriorElixir,
    MageElixir,
    ThiefOil,
    PaladinBlessing,
    NecromancerBrew,
    PhoenixTears,
    DragonBreath,
    ShadowVeil,
    TimeWarp,
    VoidTouch,
    ChaosBrew,
    TransmutationFluid,
    PerfectedElixir,
    ForbiddenMixture,
    UltimateAlchemy,

    // Failed/experimental results
    MysteryPotion,
    UnstableMixture,
    VolatileConcoction,
    ToxicSludge,
    InertFluid,
}

impl BrewedPotionType {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::HealthPotion => "Brewed Health Potion",
            Self::ManaPotion => "Brewed Mana Potion",
            Self::StrengthPotion => "Brewed Strength Potion",
            Self::DefensePotion => "Brewed Defense Potion",
            Self::SpeedPotion => "Brewed Speed Potion",
            Self::InvisibilityPotion => "Brewed Invisibility Potion",
            Self::FireResistPotion => "Fire Resistance Elixir",
            Self::IceResistPotion => "Frost Resistance Elixir",
            Self::PoisonResistPotion => "Antivenom",
            Self::RegenerationPotion => "Regeneration Elixir",
            Self::BerserkPotion => "Berserker Brew",
            Self::GiantPotion => "Giant's Draught",
            Self::LevitationPotion => "Levitation Philter",
            Self::XPPotion => "Elixir of Wisdom",
            Self::FullRestorePotion => "Complete Restoration",
            Self::LuckPotion => "Fortune's Favor",
            Self::CriticalPotion => "Critical Strike Elixir",
            Self::VisionPotion => "True Sight Potion",
            Self::CureAllPotion => "Panacea",
            Self::UltimatePowerPotion => "Ultimate Power Elixir",

            Self::AlchemistFire => "Alchemist's Fire",
            Self::FrostBomb => "Frost Bomb",
            Self::SmokeBomb => "Smoke Bomb",
            Self::LiquidCourage => "Liquid Courage",
            Self::MindClarity => "Mind Clarity Potion",
            Self::WarriorElixir => "Warrior's Elixir",
            Self::MageElixir => "Mage's Elixir",
            Self::ThiefOil => "Thief's Shadow Oil",
            Self::PaladinBlessing => "Paladin's Blessing",
            Self::NecromancerBrew => "Necromancer's Brew",
            Self::PhoenixTears => "Phoenix Tears",
            Self::DragonBreath => "Dragon's Breath",
            Self::ShadowVeil => "Shadow Veil Potion",
            Self::TimeWarp => "Time Warp Elixir",
            Self::VoidTouch => "Void Touch Philter",
            Self::ChaosBrew => "Chaos Brew",
            Self::TransmutationFluid => "Transmutation Fluid",
            Self::PerfectedElixir => "Perfected Elixir",
            Self::ForbiddenMixture => "Forbidden Mixture",
            Self::UltimateAlchemy => "Ultimate Alchemy",

            Self::MysteryPotion => "Mystery Potion",
            Self::UnstableMixture => "Unstable Mixture",
            Self::VolatileConcoction => "Volatile Concoction",
            Self::ToxicSludge => "Toxic Sludge",
            Self::InertFluid => "Inert Fluid",
        }
    }

    /// Returns whether this is a dangerous potion that might have negative effects
    pub fn is_dangerous(&self) -> bool {
        matches!(
            self,
            Self::VolatileConcoction | Self::ToxicSludge | Self::ChaosBrew
                | Self::ForbiddenMixture | Self::VoidTouch | Self::UnstableMixture
        )
    }

    /// Returns base potency (1-100 scale)
    pub fn base_potency(&self) -> i32 {
        match self {
            Self::InertFluid => 0,
            Self::ToxicSludge => 10,

            Self::HealthPotion | Self::ManaPotion => 30,
            Self::StrengthPotion | Self::DefensePotion | Self::SpeedPotion => 35,
            Self::FireResistPotion | Self::IceResistPotion | Self::PoisonResistPotion => 40,
            Self::InvisibilityPotion | Self::RegenerationPotion => 45,
            Self::AlchemistFire | Self::FrostBomb | Self::SmokeBomb => 40,
            Self::LiquidCourage | Self::MindClarity => 35,

            Self::BerserkPotion | Self::GiantPotion | Self::LevitationPotion => 50,
            Self::LuckPotion | Self::CriticalPotion | Self::VisionPotion => 45,
            Self::WarriorElixir | Self::MageElixir | Self::ThiefOil => 55,
            Self::PaladinBlessing | Self::NecromancerBrew => 60,

            Self::XPPotion | Self::FullRestorePotion | Self::CureAllPotion => 65,
            Self::PhoenixTears | Self::DragonBreath | Self::ShadowVeil => 70,
            Self::TimeWarp | Self::VoidTouch => 75,

            Self::TransmutationFluid | Self::PerfectedElixir => 80,
            Self::ChaosBrew | Self::ForbiddenMixture => 85,
            Self::UltimatePowerPotion | Self::UltimateAlchemy => 100,

            Self::MysteryPotion => 50,
            Self::UnstableMixture | Self::VolatileConcoction => 60,
        }
    }
}

/// Recipe for brewing a specific potion
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Recipe {
    /// The potion this recipe produces
    pub result: BrewedPotionType,
    /// Required primary ingredient
    pub primary: IngredientKind,
    /// Required secondary ingredient
    pub secondary: IngredientKind,
    /// Optional catalyst ingredient for enhanced effects
    pub catalyst: Option<IngredientKind>,
    /// Base success rate (0.0 - 1.0)
    pub base_success_rate: f32,
    /// Minimum station tier required
    pub required_station_tier: u8,
    /// Whether this recipe has been discovered by the player
    pub discovered: bool,
}

impl Recipe {
    /// Create a new recipe
    pub fn new(
        result: BrewedPotionType,
        primary: IngredientKind,
        secondary: IngredientKind,
        catalyst: Option<IngredientKind>,
        base_success_rate: f32,
        required_station_tier: u8,
    ) -> Self {
        Self {
            result,
            primary,
            secondary,
            catalyst,
            base_success_rate,
            required_station_tier,
            discovered: false,
        }
    }

    /// Check if given ingredients match this recipe
    pub fn matches(&self, ingredients: &[Ingredient]) -> bool {
        let has_primary = ingredients.iter().any(|i| i.kind == self.primary);
        let has_secondary = ingredients.iter().any(|i| i.kind == self.secondary);
        let has_catalyst = self.catalyst.map_or(true, |c| {
            ingredients.iter().any(|i| i.kind == c)
        });

        has_primary && has_secondary && has_catalyst
    }
}

/// Recipe book containing all known recipes
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RecipeBook {
    pub recipes: Vec<Recipe>,
}

impl Default for RecipeBook {
    fn default() -> Self {
        Self::new()
    }
}

impl RecipeBook {
    /// Create a new recipe book with all standard recipes
    pub fn new() -> Self {
        let recipes = vec![
            // Basic healing potions
            Recipe::new(
                BrewedPotionType::HealthPotion,
                IngredientKind::Healwort,
                IngredientKind::PureWater,
                None,
                0.9,
                1,
            ),
            Recipe::new(
                BrewedPotionType::ManaPotion,
                IngredientKind::Moonpetal,
                IngredientKind::CrystalDust,
                None,
                0.85,
                1,
            ),

            // Buff potions
            Recipe::new(
                BrewedPotionType::StrengthPotion,
                IngredientKind::TrollBlood,
                IngredientKind::IronFilings,
                None,
                0.75,
                1,
            ),
            Recipe::new(
                BrewedPotionType::DefensePotion,
                IngredientKind::IronFilings,
                IngredientKind::ObsidianShard,
                Some(IngredientKind::StabilizingAgent),
                0.7,
                2,
            ),
            Recipe::new(
                BrewedPotionType::SpeedPotion,
                IngredientKind::Sunbloom,
                IngredientKind::BottledLightning,
                None,
                0.75,
                1,
            ),

            // Resistance potions
            Recipe::new(
                BrewedPotionType::FireResistPotion,
                IngredientKind::Frostfern,
                IngredientKind::SalamanderHeart,
                None,
                0.7,
                2,
            ),
            Recipe::new(
                BrewedPotionType::IceResistPotion,
                IngredientKind::Firebloom,
                IngredientKind::FrozenBreath,
                None,
                0.7,
                2,
            ),
            Recipe::new(
                BrewedPotionType::PoisonResistPotion,
                IngredientKind::Healwort,
                IngredientKind::SnakeFang,
                None,
                0.8,
                1,
            ),

            // Special potions
            Recipe::new(
                BrewedPotionType::InvisibilityPotion,
                IngredientKind::Shadowroot,
                IngredientKind::GhostEctoplasm,
                Some(IngredientKind::ShadowMist),
                0.6,
                2,
            ),
            Recipe::new(
                BrewedPotionType::RegenerationPotion,
                IngredientKind::TrollBlood,
                IngredientKind::LifeEssence,
                None,
                0.65,
                2,
            ),
            Recipe::new(
                BrewedPotionType::LuckPotion,
                IngredientKind::EmeraldPowder,
                IngredientKind::UnicornHair,
                None,
                0.6,
                2,
            ),
            Recipe::new(
                BrewedPotionType::VisionPotion,
                IngredientKind::BasiliskEye,
                IngredientKind::LiquidLight,
                None,
                0.65,
                2,
            ),

            // Combat potions
            Recipe::new(
                BrewedPotionType::BerserkPotion,
                IngredientKind::Bloodthorn,
                IngredientKind::DemonIchor,
                None,
                0.55,
                2,
            ),
            Recipe::new(
                BrewedPotionType::GiantPotion,
                IngredientKind::TrollBlood,
                IngredientKind::DragonScale,
                Some(IngredientKind::PotencyEnhancer),
                0.5,
                3,
            ),
            Recipe::new(
                BrewedPotionType::CriticalPotion,
                IngredientKind::ManticoreStinger,
                IngredientKind::DiamondDust,
                None,
                0.6,
                2,
            ),

            // Throwable items
            Recipe::new(
                BrewedPotionType::AlchemistFire,
                IngredientKind::LiquidFire,
                IngredientKind::SulfurPowder,
                None,
                0.75,
                1,
            ),
            Recipe::new(
                BrewedPotionType::FrostBomb,
                IngredientKind::FrozenBreath,
                IngredientKind::SaltPeter,
                None,
                0.75,
                1,
            ),
            Recipe::new(
                BrewedPotionType::SmokeBomb,
                IngredientKind::Mistcap,
                IngredientKind::SulfurPowder,
                None,
                0.8,
                1,
            ),

            // Class-specific elixirs
            Recipe::new(
                BrewedPotionType::WarriorElixir,
                IngredientKind::TrollBlood,
                IngredientKind::DragonScale,
                Some(IngredientKind::GriffinClaw),
                0.45,
                3,
            ),
            Recipe::new(
                BrewedPotionType::MageElixir,
                IngredientKind::Moonpetal,
                IngredientKind::VoidEssence,
                Some(IngredientKind::CrystalDust),
                0.45,
                3,
            ),
            Recipe::new(
                BrewedPotionType::ThiefOil,
                IngredientKind::Shadowroot,
                IngredientKind::SpiderVenom,
                Some(IngredientKind::ShadowMist),
                0.5,
                3,
            ),
            Recipe::new(
                BrewedPotionType::PaladinBlessing,
                IngredientKind::UnicornHair,
                IngredientKind::LiquidLight,
                Some(IngredientKind::PhoenixFeather),
                0.4,
                3,
            ),
            Recipe::new(
                BrewedPotionType::NecromancerBrew,
                IngredientKind::DeathEssence,
                IngredientKind::GhostEctoplasm,
                Some(IngredientKind::SoulFragment),
                0.35,
                4,
            ),

            // Legendary potions
            Recipe::new(
                BrewedPotionType::PhoenixTears,
                IngredientKind::PhoenixFeather,
                IngredientKind::LifeEssence,
                Some(IngredientKind::PotencyEnhancer),
                0.3,
                4,
            ),
            Recipe::new(
                BrewedPotionType::DragonBreath,
                IngredientKind::DragonScale,
                IngredientKind::LiquidFire,
                Some(IngredientKind::WyrmTooth),
                0.35,
                4,
            ),
            Recipe::new(
                BrewedPotionType::ShadowVeil,
                IngredientKind::ShadowMist,
                IngredientKind::VoidEssence,
                Some(IngredientKind::Nightshade),
                0.35,
                4,
            ),
            Recipe::new(
                BrewedPotionType::TimeWarp,
                IngredientKind::TimeDroplet,
                IngredientKind::QuintessenceOil,
                Some(IngredientKind::DiamondDust),
                0.25,
                5,
            ),
            Recipe::new(
                BrewedPotionType::VoidTouch,
                IngredientKind::VoidEssence,
                IngredientKind::VoidTear,
                Some(IngredientKind::DeathEssence),
                0.2,
                5,
            ),

            // Master alchemy
            Recipe::new(
                BrewedPotionType::FullRestorePotion,
                IngredientKind::LifeEssence,
                IngredientKind::PhoenixFeather,
                Some(IngredientKind::UnicornHair),
                0.35,
                4,
            ),
            Recipe::new(
                BrewedPotionType::CureAllPotion,
                IngredientKind::Healwort,
                IngredientKind::UnicornHair,
                Some(IngredientKind::PhilosophersSalt),
                0.4,
                3,
            ),
            Recipe::new(
                BrewedPotionType::TransmutationFluid,
                IngredientKind::PhilosophersSalt,
                IngredientKind::AlchemistMercury,
                Some(IngredientKind::TransmutationCatalyst),
                0.25,
                5,
            ),
            Recipe::new(
                BrewedPotionType::PerfectedElixir,
                IngredientKind::QuintessenceOil,
                IngredientKind::UniversalSolvent,
                Some(IngredientKind::DiamondDust),
                0.2,
                5,
            ),

            // Forbidden alchemy
            Recipe::new(
                BrewedPotionType::ChaosBrew,
                IngredientKind::ChaosShard,
                IngredientKind::VoidEssence,
                Some(IngredientKind::DemonicAsh),
                0.3,
                4,
            ),
            Recipe::new(
                BrewedPotionType::ForbiddenMixture,
                IngredientKind::CursedBlood,
                IngredientKind::SoulFragment,
                Some(IngredientKind::ForbiddenElixir),
                0.2,
                5,
            ),
            Recipe::new(
                BrewedPotionType::UltimatePowerPotion,
                IngredientKind::DragonScale,
                IngredientKind::PhoenixFeather,
                Some(IngredientKind::ForbiddenElixir),
                0.15,
                5,
            ),
            Recipe::new(
                BrewedPotionType::UltimateAlchemy,
                IngredientKind::QuintessenceOil,
                IngredientKind::PhilosophersSalt,
                Some(IngredientKind::ForbiddenElixir),
                0.1,
                5,
            ),
        ];

        Self { recipes }
    }

    /// Find a recipe matching the given ingredients
    pub fn find_recipe(&self, ingredients: &[Ingredient]) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.matches(ingredients))
    }

    /// Find a recipe by result type
    pub fn find_by_result(&self, result: BrewedPotionType) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.result == result)
    }

    /// Get all discovered recipes
    pub fn discovered_recipes(&self) -> Vec<&Recipe> {
        self.recipes.iter().filter(|r| r.discovered).collect()
    }

    /// Discover a recipe
    pub fn discover(&mut self, result: BrewedPotionType) {
        if let Some(recipe) = self.recipes.iter_mut().find(|r| r.result == result) {
            recipe.discovered = true;
        }
    }
}

// ============================================================================
// Brewing Station System
// ============================================================================

/// Types of brewing stations with different capabilities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StationType {
    /// Basic cauldron - found in early dungeon
    Cauldron,
    /// Improved alchemist's bench
    Alchemist,
    /// Advanced arcane apparatus
    Arcane,
    /// Master's laboratory equipment
    Laboratory,
    /// Ancient brewing altar
    AncientAltar,
    /// Forbidden dark forge
    DarkForge,
}

impl StationType {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cauldron => "Cauldron",
            Self::Alchemist => "Alchemist's Bench",
            Self::Arcane => "Arcane Apparatus",
            Self::Laboratory => "Master's Laboratory",
            Self::AncientAltar => "Ancient Brewing Altar",
            Self::DarkForge => "Forbidden Dark Forge",
        }
    }

    /// Returns the tier of this station (1-5)
    pub fn tier(&self) -> u8 {
        match self {
            Self::Cauldron => 1,
            Self::Alchemist => 2,
            Self::Arcane => 3,
            Self::Laboratory => 4,
            Self::AncientAltar => 5,
            Self::DarkForge => 5,
        }
    }

    /// Returns the success rate bonus
    pub fn success_bonus(&self) -> f32 {
        match self {
            Self::Cauldron => 0.0,
            Self::Alchemist => 0.1,
            Self::Arcane => 0.15,
            Self::Laboratory => 0.2,
            Self::AncientAltar => 0.25,
            Self::DarkForge => 0.15, // Lower bonus but allows forbidden recipes
        }
    }

    /// Returns the potency bonus multiplier
    pub fn potency_bonus(&self) -> f32 {
        match self {
            Self::Cauldron => 1.0,
            Self::Alchemist => 1.1,
            Self::Arcane => 1.2,
            Self::Laboratory => 1.35,
            Self::AncientAltar => 1.5,
            Self::DarkForge => 1.4,
        }
    }

    /// Returns maximum ingredient capacity
    pub fn capacity(&self) -> usize {
        match self {
            Self::Cauldron => 2,
            Self::Alchemist => 3,
            Self::Arcane => 4,
            Self::Laboratory => 5,
            Self::AncientAltar => 6,
            Self::DarkForge => 5,
        }
    }

    /// Returns whether this station can brew forbidden recipes
    pub fn allows_forbidden(&self) -> bool {
        matches!(self, Self::DarkForge | Self::AncientAltar)
    }

    /// Returns the glyph for map rendering
    pub fn glyph(&self) -> char {
        match self {
            Self::Cauldron => 'U',
            Self::Alchemist => 'A',
            Self::Arcane => 'Y',
            Self::Laboratory => 'L',
            Self::AncientAltar => 'V',
            Self::DarkForge => 'F',
        }
    }
}

/// A brewing station instance
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BrewingStation {
    pub station_type: StationType,
    pub x: usize,
    pub y: usize,
    /// Current ingredients loaded into the station
    pub ingredients: Vec<Ingredient>,
    /// Number of brews completed at this station
    pub brews_completed: u32,
    /// Whether the station has been used recently (cooldown)
    pub on_cooldown: bool,
    /// Turns remaining on cooldown
    pub cooldown_turns: u8,
}

impl BrewingStation {
    /// Create a new brewing station
    pub fn new(station_type: StationType, x: usize, y: usize) -> Self {
        Self {
            station_type,
            x,
            y,
            ingredients: Vec::new(),
            brews_completed: 0,
            on_cooldown: false,
            cooldown_turns: 0,
        }
    }

    /// Add an ingredient to the station
    pub fn add_ingredient(&mut self, ingredient: Ingredient) -> Result<(), BrewingError> {
        if self.ingredients.len() >= self.station_type.capacity() {
            return Err(BrewingError::StationFull);
        }
        if self.on_cooldown {
            return Err(BrewingError::OnCooldown);
        }
        self.ingredients.push(ingredient);
        Ok(())
    }

    /// Remove an ingredient from the station
    pub fn remove_ingredient(&mut self, index: usize) -> Option<Ingredient> {
        if index < self.ingredients.len() {
            Some(self.ingredients.remove(index))
        } else {
            None
        }
    }

    /// Clear all ingredients from the station
    pub fn clear_ingredients(&mut self) -> Vec<Ingredient> {
        std::mem::take(&mut self.ingredients)
    }

    /// Attempt to brew with current ingredients using a recipe book
    pub fn brew(&mut self, recipe_book: &RecipeBook, rng_seed: u64) -> BrewingResult {
        if self.on_cooldown {
            return BrewingResult::Failure(BrewingError::OnCooldown);
        }

        if self.ingredients.len() < 2 {
            return BrewingResult::Failure(BrewingError::NotEnoughIngredients);
        }

        // Check for matching recipe
        if let Some(recipe) = recipe_book.find_recipe(&self.ingredients) {
            // Check station tier requirement
            if self.station_type.tier() < recipe.required_station_tier {
                return BrewingResult::Failure(BrewingError::StationTooWeak);
            }

            // Check forbidden ingredient restrictions
            let has_forbidden = self.ingredients.iter()
                .any(|i| i.kind.category() == IngredientCategory::Forbidden);
            if has_forbidden && !self.station_type.allows_forbidden() {
                return BrewingResult::Failure(BrewingError::ForbiddenNotAllowed);
            }

            // Calculate success chance
            let base_success = recipe.base_success_rate;
            let station_bonus = self.station_type.success_bonus();
            let quality_bonus: f32 = self.ingredients.iter()
                .map(|i| i.quality.success_modifier())
                .sum::<f32>() / self.ingredients.len() as f32;

            let final_success_rate = (base_success + station_bonus + quality_bonus).clamp(0.05, 0.95);

            // Use seed for deterministic RNG
            let roll = simple_rng(rng_seed) as f32 / u64::MAX as f32;

            if roll <= final_success_rate {
                // Success!
                let quality_mult: f32 = self.ingredients.iter()
                    .map(|i| i.quality.potency_multiplier())
                    .product();
                let potency = (recipe.result.base_potency() as f32
                    * self.station_type.potency_bonus()
                    * quality_mult.sqrt()) as i32;

                self.ingredients.clear();
                self.brews_completed += 1;
                self.start_cooldown();

                BrewingResult::Success(BrewedPotion {
                    potion_type: recipe.result,
                    potency: potency.min(200),
                    is_experimental: false,
                    special_effects: Vec::new(),
                })
            } else {
                // Failure - attempt experimental result
                self.experimental_brew(rng_seed.wrapping_add(1))
            }
        } else {
            // No recipe match - experimental brewing
            self.experimental_brew(rng_seed)
        }
    }

    /// Perform experimental brewing with random effects
    fn experimental_brew(&mut self, rng_seed: u64) -> BrewingResult {
        let rng = simple_rng(rng_seed);
        let rng2 = simple_rng(rng_seed.wrapping_add(42));
        let rng3 = simple_rng(rng_seed.wrapping_add(123));

        // Calculate instability based on ingredient properties
        let total_instability: f32 = self.ingredients.iter()
            .map(|i| i.kind.category().instability())
            .sum();

        // Collect all properties
        let mut properties: HashMap<IngredientProperty, u32> = HashMap::new();
        for ingredient in &self.ingredients {
            *properties.entry(ingredient.kind.primary_property()).or_insert(0) += 2;
            for prop in ingredient.kind.secondary_properties() {
                *properties.entry(prop).or_insert(0) += 1;
            }
        }

        // Check for conflicting properties
        let mut conflicts = 0;
        for (prop, _) in &properties {
            for conflict in prop.conflicts_with() {
                if properties.contains_key(&conflict) {
                    conflicts += 1;
                }
            }
        }

        let instability_factor = total_instability + (conflicts as f32 * 0.15);
        let quality_factor: f32 = self.ingredients.iter()
            .map(|i| i.quality.potency_multiplier())
            .sum::<f32>() / self.ingredients.len() as f32;

        // Determine outcome
        let outcome_roll = (rng % 100) as f32 / 100.0;
        let adjusted_roll = outcome_roll - instability_factor + (quality_factor * 0.2);

        let (potion_type, special_effects) = if adjusted_roll > 0.7 {
            // Great success - beneficial mystery potion
            let effects = self.determine_experimental_effects(&properties, rng2, true);
            (BrewedPotionType::MysteryPotion, effects)
        } else if adjusted_roll > 0.4 {
            // Moderate success - unstable but usable
            let effects = self.determine_experimental_effects(&properties, rng2, true);
            (BrewedPotionType::UnstableMixture, effects)
        } else if adjusted_roll > 0.15 {
            // Poor result - volatile with mixed effects
            let effects = self.determine_experimental_effects(&properties, rng2, false);
            (BrewedPotionType::VolatileConcoction, effects)
        } else if adjusted_roll > 0.0 {
            // Bad result - toxic sludge
            (BrewedPotionType::ToxicSludge, vec![ExperimentalEffect::Poison { damage: 5, duration: 5 }])
        } else {
            // Complete failure - inert fluid
            (BrewedPotionType::InertFluid, vec![])
        };

        let potency = ((quality_factor * 30.0) + (rng3 % 40) as f32) as i32;

        self.ingredients.clear();
        self.brews_completed += 1;
        self.start_cooldown();

        BrewingResult::Experimental(BrewedPotion {
            potion_type,
            potency: potency.clamp(10, 100),
            is_experimental: true,
            special_effects,
        })
    }

    /// Determine experimental effects based on ingredient properties
    fn determine_experimental_effects(
        &self,
        properties: &HashMap<IngredientProperty, u32>,
        rng_seed: u64,
        beneficial_bias: bool,
    ) -> Vec<ExperimentalEffect> {
        let mut effects = Vec::new();
        let rng = simple_rng(rng_seed);
        let effect_count = 1 + (rng % 3) as usize;

        let mut props: Vec<_> = properties.iter().collect();
        props.sort_by(|a, b| b.1.cmp(a.1)); // Sort by strength

        for (i, (prop, strength)) in props.iter().take(effect_count).enumerate() {
            let rng_i = simple_rng(rng_seed.wrapping_add(i as u64 * 17));
            let magnitude = (**strength as i32 * 5) + (rng_i % 10) as i32;

            let effect = match prop {
                IngredientProperty::Healing => {
                    if beneficial_bias {
                        ExperimentalEffect::Heal { amount: magnitude * 3 }
                    } else {
                        ExperimentalEffect::Heal { amount: magnitude }
                    }
                }
                IngredientProperty::ManaRestore => {
                    ExperimentalEffect::RestoreMana { amount: magnitude * 2 }
                }
                IngredientProperty::Strength => {
                    ExperimentalEffect::BuffStrength { amount: magnitude / 2, duration: 10 }
                }
                IngredientProperty::Defense => {
                    ExperimentalEffect::BuffDefense { amount: magnitude / 2, duration: 10 }
                }
                IngredientProperty::Speed => {
                    ExperimentalEffect::Haste { duration: magnitude }
                }
                IngredientProperty::Fire => {
                    if beneficial_bias {
                        ExperimentalEffect::FireResist { duration: magnitude }
                    } else {
                        ExperimentalEffect::Burn { damage: magnitude / 3, duration: 5 }
                    }
                }
                IngredientProperty::Ice => {
                    if beneficial_bias {
                        ExperimentalEffect::IceResist { duration: magnitude }
                    } else {
                        ExperimentalEffect::Freeze { duration: magnitude / 5 }
                    }
                }
                IngredientProperty::Poison => {
                    if beneficial_bias {
                        ExperimentalEffect::PoisonResist { duration: magnitude }
                    } else {
                        ExperimentalEffect::Poison { damage: magnitude / 2, duration: 8 }
                    }
                }
                IngredientProperty::Invisibility => {
                    ExperimentalEffect::Invisibility { duration: magnitude / 2 }
                }
                IngredientProperty::Luck => {
                    ExperimentalEffect::Luck { duration: magnitude }
                }
                IngredientProperty::Vision => {
                    ExperimentalEffect::TrueSight { duration: magnitude }
                }
                IngredientProperty::Regeneration => {
                    ExperimentalEffect::Regeneration { rate: magnitude / 4, duration: 15 }
                }
                IngredientProperty::Chaos => {
                    ExperimentalEffect::RandomTeleport
                }
                IngredientProperty::Dark => {
                    if beneficial_bias {
                        ExperimentalEffect::ShadowForm { duration: magnitude / 3 }
                    } else {
                        ExperimentalEffect::Curse { duration: magnitude / 2 }
                    }
                }
                IngredientProperty::Time => {
                    ExperimentalEffect::TimeStop { duration: magnitude / 10 }
                }
                IngredientProperty::Potency => {
                    ExperimentalEffect::EnhanceNext { multiplier: 1.5 }
                }
                IngredientProperty::Transmutation | IngredientProperty::Neutral => {
                    ExperimentalEffect::Heal { amount: magnitude }
                }
            };

            effects.push(effect);
        }

        // Chance for additional random effect
        if (rng % 100) < 20 {
            let random_effect = match (rng.wrapping_add(999)) % 10 {
                0 => ExperimentalEffect::RandomTeleport,
                1 => ExperimentalEffect::Confusion { duration: 5 },
                2 => ExperimentalEffect::Berserk { duration: 8 },
                3 => ExperimentalEffect::GainXP { amount: 50 },
                4 => ExperimentalEffect::SummonAlly,
                5 => ExperimentalEffect::CreateGold { amount: 100 },
                6 => ExperimentalEffect::Polymorph { duration: 10 },
                7 => ExperimentalEffect::Hallucination { duration: 15 },
                8 => ExperimentalEffect::Enlightenment,
                _ => ExperimentalEffect::MysteriousVision,
            };
            effects.push(random_effect);
        }

        effects
    }

    /// Start the cooldown timer
    fn start_cooldown(&mut self) {
        self.on_cooldown = true;
        self.cooldown_turns = match self.station_type {
            StationType::Cauldron => 5,
            StationType::Alchemist => 4,
            StationType::Arcane => 3,
            StationType::Laboratory => 2,
            StationType::AncientAltar => 1,
            StationType::DarkForge => 3,
        };
    }

    /// Process a turn for cooldown
    pub fn tick(&mut self) {
        if self.on_cooldown {
            if self.cooldown_turns > 0 {
                self.cooldown_turns -= 1;
            }
            if self.cooldown_turns == 0 {
                self.on_cooldown = false;
            }
        }
    }
}

// ============================================================================
// Brewing Results and Effects
// ============================================================================

/// Result of a brewing attempt
#[derive(Clone, Debug)]
pub enum BrewingResult {
    /// Successfully created a known potion
    Success(BrewedPotion),
    /// Created an experimental potion with random effects
    Experimental(BrewedPotion),
    /// Brewing failed
    Failure(BrewingError),
}

/// Errors that can occur during brewing
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrewingError {
    NotEnoughIngredients,
    StationFull,
    StationTooWeak,
    OnCooldown,
    ForbiddenNotAllowed,
    MissingCatalyst,
    IncompatibleIngredients,
}

impl BrewingError {
    /// Returns a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Self::NotEnoughIngredients => "Not enough ingredients (need at least 2)",
            Self::StationFull => "Brewing station is full",
            Self::StationTooWeak => "This station isn't powerful enough for this recipe",
            Self::OnCooldown => "Brewing station is cooling down",
            Self::ForbiddenNotAllowed => "This station cannot handle forbidden ingredients",
            Self::MissingCatalyst => "Missing required catalyst ingredient",
            Self::IncompatibleIngredients => "These ingredients are incompatible",
        }
    }
}

/// A brewed potion with its properties
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrewedPotion {
    pub potion_type: BrewedPotionType,
    /// Potency affects the strength of effects (0-200)
    pub potency: i32,
    /// Whether this was created through experimental brewing
    pub is_experimental: bool,
    /// Special effects for experimental potions
    pub special_effects: Vec<ExperimentalEffect>,
}

impl BrewedPotion {
    /// Returns the display name
    pub fn display_name(&self) -> String {
        let potency_prefix = match self.potency {
            0..=20 => "Weak ",
            21..=40 => "Diluted ",
            41..=60 => "",
            61..=80 => "Potent ",
            81..=120 => "Concentrated ",
            121..=160 => "Powerful ",
            _ => "Sublime ",
        };

        if self.is_experimental {
            format!("{}Experimental {}", potency_prefix, self.potion_type.name())
        } else {
            format!("{}{}", potency_prefix, self.potion_type.name())
        }
    }

    /// Returns the value in gold
    pub fn value(&self) -> i32 {
        let base_value = self.potion_type.base_potency();
        let potency_mult = self.potency as f32 / 50.0;
        let experimental_mult = if self.is_experimental { 0.7 } else { 1.0 };

        (base_value as f32 * potency_mult * experimental_mult) as i32
    }
}

/// Special effects that can occur on experimental potions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExperimentalEffect {
    // Beneficial effects
    Heal { amount: i32 },
    RestoreMana { amount: i32 },
    BuffStrength { amount: i32, duration: i32 },
    BuffDefense { amount: i32, duration: i32 },
    Haste { duration: i32 },
    Invisibility { duration: i32 },
    Regeneration { rate: i32, duration: i32 },
    Luck { duration: i32 },
    TrueSight { duration: i32 },
    FireResist { duration: i32 },
    IceResist { duration: i32 },
    PoisonResist { duration: i32 },
    ShadowForm { duration: i32 },
    TimeStop { duration: i32 },
    EnhanceNext { multiplier: f32 },
    GainXP { amount: i32 },
    SummonAlly,
    CreateGold { amount: i32 },
    Enlightenment,
    MysteriousVision,

    // Harmful effects
    Poison { damage: i32, duration: i32 },
    Burn { damage: i32, duration: i32 },
    Freeze { duration: i32 },
    Curse { duration: i32 },
    Confusion { duration: i32 },
    Berserk { duration: i32 },
    RandomTeleport,
    Polymorph { duration: i32 },
    Hallucination { duration: i32 },
}

impl ExperimentalEffect {
    /// Returns a description of the effect
    pub fn description(&self) -> String {
        match self {
            Self::Heal { amount } => format!("Restores {} health", amount),
            Self::RestoreMana { amount } => format!("Restores {} mana", amount),
            Self::BuffStrength { amount, duration } => {
                format!("+{} strength for {} turns", amount, duration)
            }
            Self::BuffDefense { amount, duration } => {
                format!("+{} defense for {} turns", amount, duration)
            }
            Self::Haste { duration } => format!("Grants haste for {} turns", duration),
            Self::Invisibility { duration } => format!("Grants invisibility for {} turns", duration),
            Self::Regeneration { rate, duration } => {
                format!("Regenerate {} HP/turn for {} turns", rate, duration)
            }
            Self::Luck { duration } => format!("Increases luck for {} turns", duration),
            Self::TrueSight { duration } => format!("Grants true sight for {} turns", duration),
            Self::FireResist { duration } => format!("Fire resistance for {} turns", duration),
            Self::IceResist { duration } => format!("Ice resistance for {} turns", duration),
            Self::PoisonResist { duration } => format!("Poison immunity for {} turns", duration),
            Self::ShadowForm { duration } => format!("Shadow form for {} turns", duration),
            Self::TimeStop { duration } => format!("Stops time for {} turns", duration),
            Self::EnhanceNext { multiplier } => {
                format!("Next potion {}x effective", multiplier)
            }
            Self::GainXP { amount } => format!("Gain {} experience", amount),
            Self::SummonAlly => "Summons a temporary ally".to_string(),
            Self::CreateGold { amount } => format!("Creates {} gold", amount),
            Self::Enlightenment => "Reveals all nearby secrets".to_string(),
            Self::MysteriousVision => "Shows a mysterious vision".to_string(),
            Self::Poison { damage, duration } => {
                format!("Poisoned: {} damage/turn for {} turns", damage, duration)
            }
            Self::Burn { damage, duration } => {
                format!("Burning: {} damage/turn for {} turns", damage, duration)
            }
            Self::Freeze { duration } => format!("Frozen for {} turns", duration),
            Self::Curse { duration } => format!("Cursed for {} turns", duration),
            Self::Confusion { duration } => format!("Confused for {} turns", duration),
            Self::Berserk { duration } => format!("Berserk for {} turns", duration),
            Self::RandomTeleport => "Teleports to a random location".to_string(),
            Self::Polymorph { duration } => format!("Polymorphed for {} turns", duration),
            Self::Hallucination { duration } => format!("Hallucinating for {} turns", duration),
        }
    }

    /// Returns whether this effect is harmful
    pub fn is_harmful(&self) -> bool {
        matches!(
            self,
            Self::Poison { .. } | Self::Burn { .. } | Self::Freeze { .. }
                | Self::Curse { .. } | Self::Confusion { .. } | Self::RandomTeleport
                | Self::Polymorph { .. } | Self::Hallucination { .. }
        )
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Simple deterministic RNG for brewing (not cryptographically secure)
fn simple_rng(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

/// Generate a random ingredient kind for loot drops
pub fn random_ingredient(rng_seed: u64, dungeon_level: u8) -> IngredientKind {
    let rng = simple_rng(rng_seed);
    let tier_roll = (rng % 100) as u8;

    // Higher dungeon levels unlock better ingredients
    let max_category: u64 = match dungeon_level {
        1..=3 => 2,   // Botanical and Mineral
        4..=6 => 3,   // + Creature
        7..=9 => 4,   // + Essence
        10..=14 => 5, // + Reagent
        _ => 6,       // + Forbidden
    };

    let category = if tier_roll < 40 {
        0u64 // Common botanical
    } else if tier_roll < 65 {
        rng % 2 // Botanical or Mineral
    } else if tier_roll < 85 {
        (rng % 3).min(max_category - 1)
    } else if tier_roll < 95 {
        (rng % 4).min(max_category - 1)
    } else {
        (rng % max_category).min(max_category - 1)
    };

    let ingredients_by_category: [&[IngredientKind]; 6] = [
        // Botanical
        &[
            IngredientKind::Moonpetal, IngredientKind::Sunbloom, IngredientKind::Shadowroot,
            IngredientKind::Thornvine, IngredientKind::Mistcap, IngredientKind::Glowmoss,
            IngredientKind::Firebloom, IngredientKind::Frostfern, IngredientKind::Venomleaf,
            IngredientKind::Healwort, IngredientKind::Nightshade, IngredientKind::Dreamweed,
            IngredientKind::Bloodthorn, IngredientKind::Spiritbark, IngredientKind::Voidmoss,
        ],
        // Mineral
        &[
            IngredientKind::CrystalDust, IngredientKind::IronFilings, IngredientKind::SulfurPowder,
            IngredientKind::SaltPeter, IngredientKind::MoonStone, IngredientKind::SunStone,
            IngredientKind::ObsidianShard, IngredientKind::DiamondDust, IngredientKind::EmeraldPowder,
            IngredientKind::RubyEssence, IngredientKind::SapphireDust, IngredientKind::AmethystCrystal,
        ],
        // Creature
        &[
            IngredientKind::SpiderVenom, IngredientKind::SnakeFang, IngredientKind::BatWing,
            IngredientKind::TrollBlood, IngredientKind::PhoenixFeather, IngredientKind::DragonScale,
            IngredientKind::UnicornHair, IngredientKind::BasiliskEye, IngredientKind::ManticoreStinger,
            IngredientKind::GriffinClaw, IngredientKind::HydraScale, IngredientKind::WyrmTooth,
            IngredientKind::SalamanderHeart, IngredientKind::GhostEctoplasm, IngredientKind::DemonIchor,
        ],
        // Essence
        &[
            IngredientKind::PureWater, IngredientKind::LiquidFire, IngredientKind::BottledLightning,
            IngredientKind::FrozenBreath, IngredientKind::ShadowMist, IngredientKind::LiquidLight,
            IngredientKind::VoidEssence, IngredientKind::TimeDroplet, IngredientKind::LifeEssence,
            IngredientKind::DeathEssence,
        ],
        // Reagent
        &[
            IngredientKind::PhilosophersSalt, IngredientKind::AlchemistMercury,
            IngredientKind::PrimordialSulfur, IngredientKind::QuintessenceOil,
            IngredientKind::UniversalSolvent, IngredientKind::TransmutationCatalyst,
            IngredientKind::StabilizingAgent, IngredientKind::PotencyEnhancer,
        ],
        // Forbidden
        &[
            IngredientKind::SoulFragment, IngredientKind::DemonicAsh, IngredientKind::CursedBlood,
            IngredientKind::VoidTear, IngredientKind::ChaosShard, IngredientKind::ForbiddenElixir,
        ],
    ];

    let category_ingredients = ingredients_by_category[category as usize];
    let idx = (simple_rng(rng_seed.wrapping_add(17)) as usize) % category_ingredients.len();
    category_ingredients[idx]
}

/// Generate a random ingredient quality based on dungeon level
pub fn random_quality(rng_seed: u64, dungeon_level: u8) -> IngredientQuality {
    let rng = simple_rng(rng_seed);
    let roll = (rng % 100) as i32 + (dungeon_level as i32 * 2);

    match roll {
        0..=5 => IngredientQuality::Ruined,
        6..=20 => IngredientQuality::Poor,
        21..=55 => IngredientQuality::Common,
        56..=75 => IngredientQuality::Fine,
        76..=90 => IngredientQuality::Superior,
        91..=98 => IngredientQuality::Pristine,
        _ => IngredientQuality::Perfect,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingredient_creation() {
        let ingredient = Ingredient::new(IngredientKind::Healwort, IngredientQuality::Fine);
        assert_eq!(ingredient.kind, IngredientKind::Healwort);
        assert_eq!(ingredient.quality, IngredientQuality::Fine);
        assert!(ingredient.display_name().contains("Healwort"));
    }

    #[test]
    fn test_brewing_station_capacity() {
        let mut station = BrewingStation::new(StationType::Cauldron, 5, 5);
        assert_eq!(station.station_type.capacity(), 2);

        let ingredient = Ingredient::new(IngredientKind::Healwort, IngredientQuality::Common);
        assert!(station.add_ingredient(ingredient.clone()).is_ok());
        assert!(station.add_ingredient(ingredient.clone()).is_ok());
        assert!(station.add_ingredient(ingredient).is_err());
    }

    #[test]
    fn test_recipe_matching() {
        let recipe_book = RecipeBook::new();
        let ingredients = vec![
            Ingredient::new(IngredientKind::Healwort, IngredientQuality::Common),
            Ingredient::new(IngredientKind::PureWater, IngredientQuality::Common),
        ];

        let recipe = recipe_book.find_recipe(&ingredients);
        assert!(recipe.is_some());
        assert_eq!(recipe.unwrap().result, BrewedPotionType::HealthPotion);
    }

    #[test]
    fn test_brewing_success() {
        let mut station = BrewingStation::new(StationType::Alchemist, 5, 5);
        let recipe_book = RecipeBook::new();

        station.add_ingredient(Ingredient::new(IngredientKind::Healwort, IngredientQuality::Pristine)).unwrap();
        station.add_ingredient(Ingredient::new(IngredientKind::PureWater, IngredientQuality::Pristine)).unwrap();

        // Use a seed that we know produces a successful result
        let result = station.brew(&recipe_book, 12345);

        match result {
            BrewingResult::Success(potion) | BrewingResult::Experimental(potion) => {
                assert!(potion.potency > 0);
            }
            BrewingResult::Failure(_) => {
                // Experimental can still happen on failure
            }
        }

        assert!(station.ingredients.is_empty());
        assert_eq!(station.brews_completed, 1);
    }

    #[test]
    fn test_quality_ordering() {
        assert!(IngredientQuality::Ruined < IngredientQuality::Poor);
        assert!(IngredientQuality::Poor < IngredientQuality::Common);
        assert!(IngredientQuality::Common < IngredientQuality::Fine);
        assert!(IngredientQuality::Fine < IngredientQuality::Superior);
        assert!(IngredientQuality::Superior < IngredientQuality::Pristine);
        assert!(IngredientQuality::Pristine < IngredientQuality::Perfect);
    }

    #[test]
    fn test_station_cooldown() {
        let mut station = BrewingStation::new(StationType::Cauldron, 5, 5);
        station.start_cooldown();

        assert!(station.on_cooldown);
        assert_eq!(station.cooldown_turns, 5);

        for _ in 0..5 {
            station.tick();
        }

        assert!(!station.on_cooldown);
        assert_eq!(station.cooldown_turns, 0);
    }

    #[test]
    fn test_ingredient_properties() {
        assert_eq!(IngredientKind::Healwort.primary_property(), IngredientProperty::Healing);
        assert_eq!(IngredientKind::LiquidFire.primary_property(), IngredientProperty::Fire);
        assert_eq!(IngredientKind::ChaosShard.primary_property(), IngredientProperty::Chaos);
    }

    #[test]
    fn test_random_ingredient_generation() {
        let ingredient1 = random_ingredient(1000, 1);
        let ingredient2 = random_ingredient(2000, 15);

        // Both should be valid ingredient kinds
        assert!(!ingredient1.name().is_empty());
        assert!(!ingredient2.name().is_empty());
    }

    #[test]
    fn test_experimental_effect_descriptions() {
        let effect = ExperimentalEffect::Heal { amount: 50 };
        assert!(effect.description().contains("50"));
        assert!(!effect.is_harmful());

        let poison = ExperimentalEffect::Poison { damage: 5, duration: 10 };
        assert!(poison.is_harmful());
    }
}

//! Alchemy and Pill Refining System
//!
//! A comprehensive cultivation-style alchemy system featuring:
//! - 50+ pill types across various categories
//! - Pill grades from Common to Celestial with quality levels
//! - Cauldrons, fire sources, and refining techniques
//! - Spirit herbs, monster cores, and heavenly materials
//! - Alchemist progression from Apprentice to Pill God
//! - Side effects and addiction mechanics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

/// Maximum addiction level before severe penalties
pub const MAX_ADDICTION_LEVEL: u32 = 100;
/// Addiction decay per day without pill consumption
pub const ADDICTION_DECAY_RATE: u32 = 2;
/// Base success rate modifier per alchemist rank
pub const RANK_SUCCESS_BONUS: f32 = 0.05;
/// Maximum pills that can be refined in one batch
pub const MAX_BATCH_SIZE: u32 = 9;

// ============================================================================
// Pill Types (50+ types)
// ============================================================================

/// All pill types available in the alchemy system
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum PillType {
    // === Qi Pills (restore cultivation energy) ===
    MinorQiPill,
    LesserQiPill,
    StandardQiPill,
    GreaterQiPill,
    SuperiorQiPill,
    PerfectQiPill,
    HeavenlyQiPill,

    // === Healing Pills (HP restoration) ===
    MinorHealingPill,
    LesserHealingPill,
    StandardHealingPill,
    GreaterHealingPill,
    SuperiorHealingPill,
    RegenerationPill,
    PhoenixBloodPill,

    // === Breakthrough Pills (advance cultivation) ===
    FoundationBreakthroughPill,
    CoreFormationPill,
    NascentSoulPill,
    SoulTransformationPill,
    VoidTribulationPill,
    ImmortalAscensionPill,
    DivineBreakthroughPill,

    // === Attribute Pills (stat boosts) ===
    StrengthEnhancementPill,
    AgilityEnhancementPill,
    ConstitutionEnhancementPill,
    IntelligenceEnhancementPill,
    WisdomEnhancementPill,
    CharismaEnhancementPill,
    AllAttributePill,
    PermanentStrengthPill,
    PermanentAgilityPill,
    PermanentConstitutionPill,

    // === Detox Pills (cure poisons) ===
    MinorAntidotePill,
    GreaterAntidotePill,
    UniversalDetoxPill,
    SpiritCleansingPill,
    BloodPurificationPill,

    // === Longevity Pills (extend lifespan) ===
    LifeExtensionPill,
    CenturyPill,
    MillenniumPill,
    EternalYouthPill,
    ImmortalityPill,

    // === Foundation Pills (strengthen foundation) ===
    RootStabilizingPill,
    MeridianCleansingPill,
    DantianExpansionPill,
    SpiritRootEnhancementPill,
    PerfectFoundationPill,

    // === Core Formation Pills ===
    GoldenCorePill,
    CoreCondensationPill,
    CoreStabilizationPill,
    MultiCoreFormationPill,

    // === Nascent Soul Pills ===
    SoulCondensationPill,
    SoulNourishmentPill,
    SoulProtectionPill,
    DualSoulPill,

    // === Body Tempering Pills ===
    IronBodyPill,
    CopperSkinPill,
    SteelBonePill,
    DiamondBodyPill,
    IndestructibleBodyPill,
    DragonBodyPill,

    // === Mind Clarity Pills ===
    ClearMindPill,
    EnlightenmentPill,
    WisdomPill,
    DivineInsightPill,
    OmnisciencePill,

    // === Elemental Affinity Pills ===
    FireAffinityPill,
    WaterAffinityPill,
    EarthAffinityPill,
    WindAffinityPill,
    LightningAffinityPill,
    IceAffinityPill,
    DarknessAffinityPill,
    LightAffinityPill,
    VoidAffinityPill,
    ChaosAffinityPill,

    // === Special/Rare Pills ===
    ForbiddenPill,
    HeavenDefyingPill,
    CelestialPill,
    PrimordialPill,
    CosmicPill,
}

impl PillType {
    /// Returns the display name of the pill
    pub fn name(&self) -> &'static str {
        match self {
            Self::MinorQiPill => "Minor Qi Restoration Pill",
            Self::LesserQiPill => "Lesser Qi Restoration Pill",
            Self::StandardQiPill => "Qi Restoration Pill",
            Self::GreaterQiPill => "Greater Qi Restoration Pill",
            Self::SuperiorQiPill => "Superior Qi Restoration Pill",
            Self::PerfectQiPill => "Perfect Qi Restoration Pill",
            Self::HeavenlyQiPill => "Heavenly Qi Restoration Pill",
            Self::MinorHealingPill => "Minor Healing Pill",
            Self::LesserHealingPill => "Lesser Healing Pill",
            Self::StandardHealingPill => "Healing Pill",
            Self::GreaterHealingPill => "Greater Healing Pill",
            Self::SuperiorHealingPill => "Superior Healing Pill",
            Self::RegenerationPill => "Regeneration Pill",
            Self::PhoenixBloodPill => "Phoenix Blood Pill",
            Self::FoundationBreakthroughPill => "Foundation Breakthrough Pill",
            Self::CoreFormationPill => "Core Formation Pill",
            Self::NascentSoulPill => "Nascent Soul Pill",
            Self::SoulTransformationPill => "Soul Transformation Pill",
            Self::VoidTribulationPill => "Void Tribulation Pill",
            Self::ImmortalAscensionPill => "Immortal Ascension Pill",
            Self::DivineBreakthroughPill => "Divine Breakthrough Pill",
            Self::StrengthEnhancementPill => "Strength Enhancement Pill",
            Self::AgilityEnhancementPill => "Agility Enhancement Pill",
            Self::ConstitutionEnhancementPill => "Constitution Enhancement Pill",
            Self::IntelligenceEnhancementPill => "Intelligence Enhancement Pill",
            Self::WisdomEnhancementPill => "Wisdom Enhancement Pill",
            Self::CharismaEnhancementPill => "Charisma Enhancement Pill",
            Self::AllAttributePill => "All Attribute Enhancement Pill",
            Self::PermanentStrengthPill => "Permanent Strength Pill",
            Self::PermanentAgilityPill => "Permanent Agility Pill",
            Self::PermanentConstitutionPill => "Permanent Constitution Pill",
            Self::MinorAntidotePill => "Minor Antidote Pill",
            Self::GreaterAntidotePill => "Greater Antidote Pill",
            Self::UniversalDetoxPill => "Universal Detox Pill",
            Self::SpiritCleansingPill => "Spirit Cleansing Pill",
            Self::BloodPurificationPill => "Blood Purification Pill",
            Self::LifeExtensionPill => "Life Extension Pill",
            Self::CenturyPill => "Century Pill",
            Self::MillenniumPill => "Millennium Pill",
            Self::EternalYouthPill => "Eternal Youth Pill",
            Self::ImmortalityPill => "Immortality Pill",
            Self::RootStabilizingPill => "Root Stabilizing Pill",
            Self::MeridianCleansingPill => "Meridian Cleansing Pill",
            Self::DantianExpansionPill => "Dantian Expansion Pill",
            Self::SpiritRootEnhancementPill => "Spirit Root Enhancement Pill",
            Self::PerfectFoundationPill => "Perfect Foundation Pill",
            Self::GoldenCorePill => "Golden Core Pill",
            Self::CoreCondensationPill => "Core Condensation Pill",
            Self::CoreStabilizationPill => "Core Stabilization Pill",
            Self::MultiCoreFormationPill => "Multi-Core Formation Pill",
            Self::SoulCondensationPill => "Soul Condensation Pill",
            Self::SoulNourishmentPill => "Soul Nourishment Pill",
            Self::SoulProtectionPill => "Soul Protection Pill",
            Self::DualSoulPill => "Dual Soul Pill",
            Self::IronBodyPill => "Iron Body Pill",
            Self::CopperSkinPill => "Copper Skin Pill",
            Self::SteelBonePill => "Steel Bone Pill",
            Self::DiamondBodyPill => "Diamond Body Pill",
            Self::IndestructibleBodyPill => "Indestructible Body Pill",
            Self::DragonBodyPill => "Dragon Body Pill",
            Self::ClearMindPill => "Clear Mind Pill",
            Self::EnlightenmentPill => "Enlightenment Pill",
            Self::WisdomPill => "Wisdom Pill",
            Self::DivineInsightPill => "Divine Insight Pill",
            Self::OmnisciencePill => "Omniscience Pill",
            Self::FireAffinityPill => "Fire Affinity Pill",
            Self::WaterAffinityPill => "Water Affinity Pill",
            Self::EarthAffinityPill => "Earth Affinity Pill",
            Self::WindAffinityPill => "Wind Affinity Pill",
            Self::LightningAffinityPill => "Lightning Affinity Pill",
            Self::IceAffinityPill => "Ice Affinity Pill",
            Self::DarknessAffinityPill => "Darkness Affinity Pill",
            Self::LightAffinityPill => "Light Affinity Pill",
            Self::VoidAffinityPill => "Void Affinity Pill",
            Self::ChaosAffinityPill => "Chaos Affinity Pill",
            Self::ForbiddenPill => "Forbidden Pill",
            Self::HeavenDefyingPill => "Heaven Defying Pill",
            Self::CelestialPill => "Celestial Pill",
            Self::PrimordialPill => "Primordial Pill",
            Self::CosmicPill => "Cosmic Pill",
        }
    }

    /// Returns the pill category
    pub fn category(&self) -> PillCategory {
        match self {
            Self::MinorQiPill | Self::LesserQiPill | Self::StandardQiPill
            | Self::GreaterQiPill | Self::SuperiorQiPill | Self::PerfectQiPill
            | Self::HeavenlyQiPill => PillCategory::QiRestoration,

            Self::MinorHealingPill | Self::LesserHealingPill | Self::StandardHealingPill
            | Self::GreaterHealingPill | Self::SuperiorHealingPill | Self::RegenerationPill
            | Self::PhoenixBloodPill => PillCategory::Healing,

            Self::FoundationBreakthroughPill | Self::CoreFormationPill | Self::NascentSoulPill
            | Self::SoulTransformationPill | Self::VoidTribulationPill
            | Self::ImmortalAscensionPill | Self::DivineBreakthroughPill => PillCategory::Breakthrough,

            Self::StrengthEnhancementPill | Self::AgilityEnhancementPill
            | Self::ConstitutionEnhancementPill | Self::IntelligenceEnhancementPill
            | Self::WisdomEnhancementPill | Self::CharismaEnhancementPill
            | Self::AllAttributePill | Self::PermanentStrengthPill
            | Self::PermanentAgilityPill | Self::PermanentConstitutionPill => PillCategory::Attribute,

            Self::MinorAntidotePill | Self::GreaterAntidotePill | Self::UniversalDetoxPill
            | Self::SpiritCleansingPill | Self::BloodPurificationPill => PillCategory::Detox,

            Self::LifeExtensionPill | Self::CenturyPill | Self::MillenniumPill
            | Self::EternalYouthPill | Self::ImmortalityPill => PillCategory::Longevity,

            Self::RootStabilizingPill | Self::MeridianCleansingPill | Self::DantianExpansionPill
            | Self::SpiritRootEnhancementPill | Self::PerfectFoundationPill => PillCategory::Foundation,

            Self::GoldenCorePill | Self::CoreCondensationPill | Self::CoreStabilizationPill
            | Self::MultiCoreFormationPill => PillCategory::CoreFormation,

            Self::SoulCondensationPill | Self::SoulNourishmentPill | Self::SoulProtectionPill
            | Self::DualSoulPill => PillCategory::NascentSoul,

            Self::IronBodyPill | Self::CopperSkinPill | Self::SteelBonePill
            | Self::DiamondBodyPill | Self::IndestructibleBodyPill
            | Self::DragonBodyPill => PillCategory::BodyTempering,

            Self::ClearMindPill | Self::EnlightenmentPill | Self::WisdomPill
            | Self::DivineInsightPill | Self::OmnisciencePill => PillCategory::MindClarity,

            Self::FireAffinityPill | Self::WaterAffinityPill | Self::EarthAffinityPill
            | Self::WindAffinityPill | Self::LightningAffinityPill | Self::IceAffinityPill
            | Self::DarknessAffinityPill | Self::LightAffinityPill | Self::VoidAffinityPill
            | Self::ChaosAffinityPill => PillCategory::ElementalAffinity,

            Self::ForbiddenPill | Self::HeavenDefyingPill | Self::CelestialPill
            | Self::PrimordialPill | Self::CosmicPill => PillCategory::Special,
        }
    }

    /// Returns the minimum grade required to refine this pill
    pub fn minimum_grade(&self) -> PillGrade {
        match self {
            Self::MinorQiPill | Self::MinorHealingPill | Self::MinorAntidotePill
            | Self::IronBodyPill | Self::ClearMindPill => PillGrade::Common,

            Self::LesserQiPill | Self::LesserHealingPill | Self::CopperSkinPill
            | Self::RootStabilizingPill | Self::StrengthEnhancementPill
            | Self::AgilityEnhancementPill => PillGrade::Uncommon,

            Self::StandardQiPill | Self::StandardHealingPill | Self::GreaterAntidotePill
            | Self::MeridianCleansingPill | Self::SteelBonePill
            | Self::EnlightenmentPill | Self::FireAffinityPill | Self::WaterAffinityPill
            | Self::EarthAffinityPill | Self::WindAffinityPill => PillGrade::Rare,

            Self::GreaterQiPill | Self::GreaterHealingPill | Self::FoundationBreakthroughPill
            | Self::DantianExpansionPill | Self::DiamondBodyPill | Self::WisdomPill
            | Self::ConstitutionEnhancementPill | Self::IntelligenceEnhancementPill
            | Self::LifeExtensionPill | Self::UniversalDetoxPill
            | Self::LightningAffinityPill | Self::IceAffinityPill => PillGrade::Epic,

            Self::SuperiorQiPill | Self::SuperiorHealingPill | Self::CoreFormationPill
            | Self::GoldenCorePill | Self::CoreCondensationPill | Self::SpiritRootEnhancementPill
            | Self::IndestructibleBodyPill | Self::DivineInsightPill | Self::CenturyPill
            | Self::WisdomEnhancementPill | Self::CharismaEnhancementPill
            | Self::SpiritCleansingPill | Self::DarknessAffinityPill
            | Self::LightAffinityPill | Self::RegenerationPill => PillGrade::Legendary,

            Self::PerfectQiPill | Self::NascentSoulPill | Self::SoulCondensationPill
            | Self::SoulNourishmentPill | Self::CoreStabilizationPill
            | Self::PerfectFoundationPill | Self::DragonBodyPill
            | Self::AllAttributePill | Self::PermanentStrengthPill | Self::PermanentAgilityPill
            | Self::MillenniumPill | Self::BloodPurificationPill | Self::VoidAffinityPill
            | Self::SoulTransformationPill | Self::MultiCoreFormationPill => PillGrade::Divine,

            Self::HeavenlyQiPill | Self::PhoenixBloodPill | Self::VoidTribulationPill
            | Self::ImmortalAscensionPill | Self::DivineBreakthroughPill
            | Self::SoulProtectionPill | Self::DualSoulPill | Self::OmnisciencePill
            | Self::PermanentConstitutionPill | Self::EternalYouthPill | Self::ImmortalityPill
            | Self::ChaosAffinityPill | Self::ForbiddenPill | Self::HeavenDefyingPill
            | Self::CelestialPill | Self::PrimordialPill | Self::CosmicPill => PillGrade::Celestial,
        }
    }

    /// Returns base effect value (scaled by grade and quality)
    pub fn base_effect(&self) -> i32 {
        match self {
            Self::MinorQiPill => 20,
            Self::LesserQiPill => 40,
            Self::StandardQiPill => 80,
            Self::GreaterQiPill => 150,
            Self::SuperiorQiPill => 300,
            Self::PerfectQiPill => 600,
            Self::HeavenlyQiPill => 1500,
            Self::MinorHealingPill => 25,
            Self::LesserHealingPill => 50,
            Self::StandardHealingPill => 100,
            Self::GreaterHealingPill => 200,
            Self::SuperiorHealingPill => 400,
            Self::RegenerationPill => 50,
            Self::PhoenixBloodPill => 9999,
            Self::StrengthEnhancementPill | Self::AgilityEnhancementPill
            | Self::ConstitutionEnhancementPill | Self::IntelligenceEnhancementPill
            | Self::WisdomEnhancementPill | Self::CharismaEnhancementPill => 5,
            Self::AllAttributePill => 3,
            Self::PermanentStrengthPill | Self::PermanentAgilityPill
            | Self::PermanentConstitutionPill => 1,
            Self::LifeExtensionPill => 10,
            Self::CenturyPill => 100,
            Self::MillenniumPill => 1000,
            Self::EternalYouthPill => 5000,
            Self::ImmortalityPill => 99999,
            _ => 100,
        }
    }

    /// Returns duration in turns (0 for instant effects)
    pub fn duration(&self) -> u32 {
        match self {
            Self::StrengthEnhancementPill | Self::AgilityEnhancementPill
            | Self::ConstitutionEnhancementPill | Self::IntelligenceEnhancementPill
            | Self::WisdomEnhancementPill | Self::CharismaEnhancementPill => 100,
            Self::AllAttributePill => 50,
            Self::RegenerationPill => 30,
            Self::ClearMindPill => 50,
            Self::EnlightenmentPill => 100,
            Self::WisdomPill => 150,
            Self::DivineInsightPill => 200,
            Self::FireAffinityPill | Self::WaterAffinityPill | Self::EarthAffinityPill
            | Self::WindAffinityPill | Self::LightningAffinityPill | Self::IceAffinityPill
            | Self::DarknessAffinityPill | Self::LightAffinityPill => 200,
            Self::VoidAffinityPill | Self::ChaosAffinityPill => 300,
            _ => 0,
        }
    }

    /// Returns the addiction potential (0-100)
    pub fn addiction_potential(&self) -> u32 {
        match self.category() {
            PillCategory::QiRestoration => 5,
            PillCategory::Healing => 3,
            PillCategory::Breakthrough => 15,
            PillCategory::Attribute => 20,
            PillCategory::Detox => 2,
            PillCategory::Longevity => 25,
            PillCategory::Foundation => 10,
            PillCategory::CoreFormation => 12,
            PillCategory::NascentSoul => 18,
            PillCategory::BodyTempering => 8,
            PillCategory::MindClarity => 15,
            PillCategory::ElementalAffinity => 12,
            PillCategory::Special => 35,
        }
    }

    /// Returns whether this pill can trigger a tribulation
    pub fn triggers_tribulation(&self) -> bool {
        matches!(self,
            Self::NascentSoulPill | Self::SoulTransformationPill | Self::VoidTribulationPill
            | Self::ImmortalAscensionPill | Self::DivineBreakthroughPill | Self::ImmortalityPill
            | Self::HeavenDefyingPill | Self::CelestialPill | Self::PrimordialPill
            | Self::CosmicPill
        )
    }
}

/// Categories of pills
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum PillCategory {
    QiRestoration,
    Healing,
    Breakthrough,
    Attribute,
    Detox,
    Longevity,
    Foundation,
    CoreFormation,
    NascentSoul,
    BodyTempering,
    MindClarity,
    ElementalAffinity,
    Special,
}

impl PillCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::QiRestoration => "Qi Restoration",
            Self::Healing => "Healing",
            Self::Breakthrough => "Breakthrough",
            Self::Attribute => "Attribute Enhancement",
            Self::Detox => "Detoxification",
            Self::Longevity => "Longevity",
            Self::Foundation => "Foundation",
            Self::CoreFormation => "Core Formation",
            Self::NascentSoul => "Nascent Soul",
            Self::BodyTempering => "Body Tempering",
            Self::MindClarity => "Mind Clarity",
            Self::ElementalAffinity => "Elemental Affinity",
            Self::Special => "Special",
        }
    }
}

// ============================================================================
// Pill Grades and Quality
// ============================================================================

/// Pill grades from lowest to highest
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum PillGrade {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Divine,
    Celestial,
}

impl PillGrade {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
            Self::Divine => "Divine",
            Self::Celestial => "Celestial",
        }
    }

    pub fn effect_multiplier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.5,
            Self::Rare => 2.5,
            Self::Epic => 4.0,
            Self::Legendary => 7.0,
            Self::Divine => 12.0,
            Self::Celestial => 25.0,
        }
    }

    pub fn base_success_rate(&self) -> f32 {
        match self {
            Self::Common => 0.90,
            Self::Uncommon => 0.75,
            Self::Rare => 0.55,
            Self::Epic => 0.35,
            Self::Legendary => 0.20,
            Self::Divine => 0.10,
            Self::Celestial => 0.05,
        }
    }

    pub fn tribulation_chance(&self) -> f32 {
        match self {
            Self::Common | Self::Uncommon | Self::Rare => 0.0,
            Self::Epic => 0.05,
            Self::Legendary => 0.15,
            Self::Divine => 0.35,
            Self::Celestial => 0.60,
        }
    }

    pub fn required_rank(&self) -> AlchemistRank {
        match self {
            Self::Common => AlchemistRank::Apprentice,
            Self::Uncommon => AlchemistRank::Junior,
            Self::Rare => AlchemistRank::Intermediate,
            Self::Epic => AlchemistRank::Senior,
            Self::Legendary => AlchemistRank::Master,
            Self::Divine => AlchemistRank::PillKing,
            Self::Celestial => AlchemistRank::PillEmperor,
        }
    }

    pub fn base_value(&self) -> u64 {
        match self {
            Self::Common => 10,
            Self::Uncommon => 50,
            Self::Rare => 200,
            Self::Epic => 1000,
            Self::Legendary => 5000,
            Self::Divine => 25000,
            Self::Celestial => 150000,
        }
    }
}

/// Quality levels within each grade
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum PillQuality {
    Low,
    Medium,
    High,
    Perfect,
}

impl PillQuality {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Perfect => "Perfect",
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Low => 0.7,
            Self::Medium => 1.0,
            Self::High => 1.3,
            Self::Perfect => 1.8,
        }
    }

    pub fn achievement_chance(&self) -> f32 {
        match self {
            Self::Low => 0.35,
            Self::Medium => 0.40,
            Self::High => 0.20,
            Self::Perfect => 0.05,
        }
    }
}

// ============================================================================
// Alchemist Ranks
// ============================================================================

/// Alchemist progression ranks
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum AlchemistRank {
    Apprentice,
    Junior,
    Intermediate,
    Senior,
    Master,
    Grandmaster,
    PillKing,
    PillEmperor,
    PillGod,
}

impl AlchemistRank {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Apprentice => "Apprentice Alchemist",
            Self::Junior => "Junior Alchemist",
            Self::Intermediate => "Intermediate Alchemist",
            Self::Senior => "Senior Alchemist",
            Self::Master => "Master Alchemist",
            Self::Grandmaster => "Grandmaster Alchemist",
            Self::PillKing => "Pill King",
            Self::PillEmperor => "Pill Emperor",
            Self::PillGod => "Pill God",
        }
    }

    pub fn success_bonus(&self) -> f32 {
        match self {
            Self::Apprentice => 0.0,
            Self::Junior => 0.05,
            Self::Intermediate => 0.10,
            Self::Senior => 0.15,
            Self::Master => 0.22,
            Self::Grandmaster => 0.30,
            Self::PillKing => 0.40,
            Self::PillEmperor => 0.55,
            Self::PillGod => 0.75,
        }
    }

    pub fn quality_bonus(&self) -> f32 {
        match self {
            Self::Apprentice => 0.0,
            Self::Junior => 0.02,
            Self::Intermediate => 0.05,
            Self::Senior => 0.08,
            Self::Master => 0.12,
            Self::Grandmaster => 0.18,
            Self::PillKing => 0.25,
            Self::PillEmperor => 0.35,
            Self::PillGod => 0.50,
        }
    }

    pub fn exp_for_next(&self) -> u64 {
        match self {
            Self::Apprentice => 100,
            Self::Junior => 500,
            Self::Intermediate => 2000,
            Self::Senior => 8000,
            Self::Master => 30000,
            Self::Grandmaster => 100000,
            Self::PillKing => 500000,
            Self::PillEmperor => 2000000,
            Self::PillGod => u64::MAX,
        }
    }

    pub fn next_rank(&self) -> Option<AlchemistRank> {
        match self {
            Self::Apprentice => Some(Self::Junior),
            Self::Junior => Some(Self::Intermediate),
            Self::Intermediate => Some(Self::Senior),
            Self::Senior => Some(Self::Master),
            Self::Master => Some(Self::Grandmaster),
            Self::Grandmaster => Some(Self::PillKing),
            Self::PillKing => Some(Self::PillEmperor),
            Self::PillEmperor => Some(Self::PillGod),
            Self::PillGod => None,
        }
    }

    pub fn max_batch_size(&self) -> u32 {
        match self {
            Self::Apprentice => 1,
            Self::Junior => 2,
            Self::Intermediate => 3,
            Self::Senior => 4,
            Self::Master => 5,
            Self::Grandmaster => 6,
            Self::PillKing => 7,
            Self::PillEmperor => 8,
            Self::PillGod => 9,
        }
    }
}

// ============================================================================
// Ingredients
// ============================================================================

/// Spirit herb age categories
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum HerbAge {
    Decade,
    Century,
    Millennium,
    TenThousand,
    HundredThousand,
    Million,
    Primordial,
}

impl HerbAge {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Decade => "10-Year",
            Self::Century => "100-Year",
            Self::Millennium => "1000-Year",
            Self::TenThousand => "10000-Year",
            Self::HundredThousand => "100000-Year",
            Self::Million => "Million-Year",
            Self::Primordial => "Primordial",
        }
    }

    pub fn potency_multiplier(&self) -> f32 {
        match self {
            Self::Decade => 1.0,
            Self::Century => 2.0,
            Self::Millennium => 5.0,
            Self::TenThousand => 15.0,
            Self::HundredThousand => 50.0,
            Self::Million => 200.0,
            Self::Primordial => 1000.0,
        }
    }

    pub fn base_value(&self) -> u64 {
        match self {
            Self::Decade => 5,
            Self::Century => 25,
            Self::Millennium => 150,
            Self::TenThousand => 1000,
            Self::HundredThousand => 8000,
            Self::Million => 75000,
            Self::Primordial => 500000,
        }
    }
}

/// Types of spirit herbs
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SpiritHerbType {
    SpiritGrass,
    QiGatheringFlower,
    MoonlightOrchid,
    SunfireBloom,
    JadeLeaf,
    CloudMushroom,
    DragonBloodVine,
    PhoenixTailFern,
    NinePetalLotus,
    ThunderRootGinseng,
    IceSoulFlower,
    FlameHeartOrchid,
    VoidShadowGrass,
    StarfallBlossom,
    HeavenlyDewHerb,
    ImmortalPeach,
    ChaosOriginFlower,
    PrimordialWorldTree,
    CosmicLotus,
    DivineSpiritGrass,
    EternalBloom,
}

impl SpiritHerbType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SpiritGrass => "Spirit Grass",
            Self::QiGatheringFlower => "Qi Gathering Flower",
            Self::MoonlightOrchid => "Moonlight Orchid",
            Self::SunfireBloom => "Sunfire Bloom",
            Self::JadeLeaf => "Jade Leaf",
            Self::CloudMushroom => "Cloud Mushroom",
            Self::DragonBloodVine => "Dragon Blood Vine",
            Self::PhoenixTailFern => "Phoenix Tail Fern",
            Self::NinePetalLotus => "Nine Petal Lotus",
            Self::ThunderRootGinseng => "Thunder Root Ginseng",
            Self::IceSoulFlower => "Ice Soul Flower",
            Self::FlameHeartOrchid => "Flame Heart Orchid",
            Self::VoidShadowGrass => "Void Shadow Grass",
            Self::StarfallBlossom => "Starfall Blossom",
            Self::HeavenlyDewHerb => "Heavenly Dew Herb",
            Self::ImmortalPeach => "Immortal Peach",
            Self::ChaosOriginFlower => "Chaos Origin Flower",
            Self::PrimordialWorldTree => "Primordial World Tree Leaf",
            Self::CosmicLotus => "Cosmic Lotus",
            Self::DivineSpiritGrass => "Divine Spirit Grass",
            Self::EternalBloom => "Eternal Bloom",
        }
    }

    pub fn rarity(&self) -> IngredientRarity {
        match self {
            Self::SpiritGrass | Self::QiGatheringFlower | Self::JadeLeaf
            | Self::CloudMushroom => IngredientRarity::Common,
            Self::MoonlightOrchid | Self::SunfireBloom | Self::DragonBloodVine
            | Self::PhoenixTailFern => IngredientRarity::Uncommon,
            Self::NinePetalLotus | Self::ThunderRootGinseng | Self::IceSoulFlower
            | Self::FlameHeartOrchid => IngredientRarity::Rare,
            Self::VoidShadowGrass | Self::StarfallBlossom | Self::HeavenlyDewHerb
            | Self::ImmortalPeach => IngredientRarity::Epic,
            Self::ChaosOriginFlower | Self::PrimordialWorldTree | Self::CosmicLotus
            | Self::DivineSpiritGrass | Self::EternalBloom => IngredientRarity::Legendary,
        }
    }

    pub fn primary_effect(&self) -> HerbEffect {
        match self {
            Self::SpiritGrass | Self::QiGatheringFlower => HerbEffect::QiRestoration,
            Self::MoonlightOrchid => HerbEffect::MindClarity,
            Self::SunfireBloom | Self::FlameHeartOrchid => HerbEffect::FireElement,
            Self::JadeLeaf => HerbEffect::Healing,
            Self::CloudMushroom => HerbEffect::BodyTempering,
            Self::DragonBloodVine => HerbEffect::StrengthBoost,
            Self::PhoenixTailFern => HerbEffect::Regeneration,
            Self::NinePetalLotus => HerbEffect::BreakthroughAid,
            Self::ThunderRootGinseng => HerbEffect::LightningElement,
            Self::IceSoulFlower => HerbEffect::IceElement,
            Self::VoidShadowGrass => HerbEffect::VoidElement,
            Self::StarfallBlossom => HerbEffect::CosmicEnergy,
            Self::HeavenlyDewHerb => HerbEffect::Purification,
            Self::ImmortalPeach => HerbEffect::Longevity,
            Self::ChaosOriginFlower => HerbEffect::ChaosElement,
            Self::PrimordialWorldTree => HerbEffect::AllElements,
            Self::CosmicLotus => HerbEffect::CosmicEnergy,
            Self::DivineSpiritGrass => HerbEffect::DivineEnergy,
            Self::EternalBloom => HerbEffect::Immortality,
        }
    }
}

/// Effects that herbs can provide
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum HerbEffect {
    QiRestoration,
    Healing,
    MindClarity,
    BodyTempering,
    StrengthBoost,
    Regeneration,
    BreakthroughAid,
    Purification,
    Longevity,
    FireElement,
    WaterElement,
    EarthElement,
    WindElement,
    LightningElement,
    IceElement,
    VoidElement,
    ChaosElement,
    CosmicEnergy,
    DivineEnergy,
    AllElements,
    Immortality,
}

/// Monster core grades
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum MonsterCoreGrade {
    Mortal,
    Spirit,
    Earth,
    Sky,
    King,
    Emperor,
    Saint,
    Divine,
    Primordial,
}

impl MonsterCoreGrade {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mortal => "Mortal Beast Core",
            Self::Spirit => "Spirit Beast Core",
            Self::Earth => "Earth Beast Core",
            Self::Sky => "Sky Beast Core",
            Self::King => "Beast King Core",
            Self::Emperor => "Beast Emperor Core",
            Self::Saint => "Saint Beast Core",
            Self::Divine => "Divine Beast Core",
            Self::Primordial => "Primordial Beast Core",
        }
    }

    pub fn potency(&self) -> f32 {
        match self {
            Self::Mortal => 1.0,
            Self::Spirit => 2.0,
            Self::Earth => 4.0,
            Self::Sky => 8.0,
            Self::King => 16.0,
            Self::Emperor => 32.0,
            Self::Saint => 64.0,
            Self::Divine => 150.0,
            Self::Primordial => 500.0,
        }
    }
}

/// Types of monster cores
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MonsterCoreType {
    WolfCore,
    BearCore,
    TigerCore,
    EagleCore,
    SerpentCore,
    FlameCore,
    FrostCore,
    ThunderCore,
    EarthCore,
    WindCore,
    DragonCore,
    PhoenixCore,
    QilinCore,
    TortoiseCore,
    VermilionBirdCore,
    ChaosCore,
    VoidCore,
    PrimordialCore,
    CosmicCore,
}

impl MonsterCoreType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::WolfCore => "Wolf",
            Self::BearCore => "Bear",
            Self::TigerCore => "Tiger",
            Self::EagleCore => "Eagle",
            Self::SerpentCore => "Serpent",
            Self::FlameCore => "Flame Beast",
            Self::FrostCore => "Frost Beast",
            Self::ThunderCore => "Thunder Beast",
            Self::EarthCore => "Earth Beast",
            Self::WindCore => "Wind Beast",
            Self::DragonCore => "Dragon",
            Self::PhoenixCore => "Phoenix",
            Self::QilinCore => "Qilin",
            Self::TortoiseCore => "Black Tortoise",
            Self::VermilionBirdCore => "Vermilion Bird",
            Self::ChaosCore => "Chaos Beast",
            Self::VoidCore => "Void Beast",
            Self::PrimordialCore => "Primordial Beast",
            Self::CosmicCore => "Cosmic Beast",
        }
    }

    pub fn element(&self) -> Option<ElementType> {
        match self {
            Self::FlameCore | Self::PhoenixCore | Self::VermilionBirdCore => Some(ElementType::Fire),
            Self::FrostCore => Some(ElementType::Ice),
            Self::ThunderCore => Some(ElementType::Lightning),
            Self::EarthCore | Self::TortoiseCore => Some(ElementType::Earth),
            Self::WindCore | Self::EagleCore => Some(ElementType::Wind),
            Self::DragonCore => Some(ElementType::All),
            Self::VoidCore => Some(ElementType::Void),
            Self::ChaosCore => Some(ElementType::Chaos),
            _ => None,
        }
    }
}

/// Rare mineral types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RareMineralType {
    SpiritStone,
    JadeEssence,
    MoonlightCrystal,
    SunStone,
    StarFragment,
    VoidCrystal,
    ChaosCrystal,
    DivineMetal,
    PrimordialOre,
    CosmicDust,
    HeavenlyIron,
    DragonBone,
    PhoenixAsh,
    ImmortalDust,
}

impl RareMineralType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SpiritStone => "Spirit Stone",
            Self::JadeEssence => "Jade Essence",
            Self::MoonlightCrystal => "Moonlight Crystal",
            Self::SunStone => "Sun Stone",
            Self::StarFragment => "Star Fragment",
            Self::VoidCrystal => "Void Crystal",
            Self::ChaosCrystal => "Chaos Crystal",
            Self::DivineMetal => "Divine Metal",
            Self::PrimordialOre => "Primordial Ore",
            Self::CosmicDust => "Cosmic Dust",
            Self::HeavenlyIron => "Heavenly Iron",
            Self::DragonBone => "Dragon Bone",
            Self::PhoenixAsh => "Phoenix Ash",
            Self::ImmortalDust => "Immortal Dust",
        }
    }

    pub fn rarity(&self) -> IngredientRarity {
        match self {
            Self::SpiritStone | Self::JadeEssence => IngredientRarity::Common,
            Self::MoonlightCrystal | Self::SunStone => IngredientRarity::Uncommon,
            Self::StarFragment | Self::HeavenlyIron => IngredientRarity::Rare,
            Self::VoidCrystal | Self::DragonBone | Self::PhoenixAsh => IngredientRarity::Epic,
            Self::ChaosCrystal | Self::DivineMetal | Self::ImmortalDust => IngredientRarity::Legendary,
            Self::PrimordialOre | Self::CosmicDust => IngredientRarity::Mythical,
        }
    }
}

/// Heavenly material types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum HeavenlyMaterialType {
    HeavenlyWater,
    DivineFire,
    SacredEarth,
    PrimordialChaos,
    CosmicEssence,
    DaoFragment,
    WorldOrigin,
    UniversalLaw,
    TimeEssence,
    SpaceEssence,
    LifeEssence,
    DeathEssence,
    KarmaThread,
    DestinyFragment,
    CreationSpark,
}

impl HeavenlyMaterialType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::HeavenlyWater => "Heavenly Water",
            Self::DivineFire => "Divine Fire",
            Self::SacredEarth => "Sacred Earth",
            Self::PrimordialChaos => "Primordial Chaos",
            Self::CosmicEssence => "Cosmic Essence",
            Self::DaoFragment => "Dao Fragment",
            Self::WorldOrigin => "World Origin",
            Self::UniversalLaw => "Universal Law Fragment",
            Self::TimeEssence => "Time Essence",
            Self::SpaceEssence => "Space Essence",
            Self::LifeEssence => "Life Essence",
            Self::DeathEssence => "Death Essence",
            Self::KarmaThread => "Karma Thread",
            Self::DestinyFragment => "Destiny Fragment",
            Self::CreationSpark => "Creation Spark",
        }
    }

    pub fn potency(&self) -> f32 {
        match self {
            Self::HeavenlyWater | Self::DivineFire | Self::SacredEarth => 5.0,
            Self::PrimordialChaos | Self::CosmicEssence => 15.0,
            Self::DaoFragment | Self::WorldOrigin => 50.0,
            Self::UniversalLaw | Self::TimeEssence | Self::SpaceEssence => 100.0,
            Self::LifeEssence | Self::DeathEssence => 150.0,
            Self::KarmaThread | Self::DestinyFragment => 250.0,
            Self::CreationSpark => 500.0,
        }
    }
}

/// Ingredient rarity
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum IngredientRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythical,
}

impl IngredientRarity {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
            Self::Mythical => "Mythical",
        }
    }

    pub fn drop_chance_modifier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 0.5,
            Self::Rare => 0.2,
            Self::Epic => 0.05,
            Self::Legendary => 0.01,
            Self::Mythical => 0.001,
        }
    }
}

/// Element types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ElementType {
    Fire,
    Water,
    Earth,
    Wind,
    Lightning,
    Ice,
    Light,
    Darkness,
    Void,
    Chaos,
    All,
}

/// A spirit herb ingredient
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpiritHerb {
    pub herb_type: SpiritHerbType,
    pub age: HerbAge,
    pub quality: PillQuality,
}

impl SpiritHerb {
    pub fn new(herb_type: SpiritHerbType, age: HerbAge, quality: PillQuality) -> Self {
        Self { herb_type, age, quality }
    }

    pub fn display_name(&self) -> String {
        format!("{} {} ({})", self.age.name(), self.herb_type.name(), self.quality.name())
    }

    pub fn total_potency(&self) -> f32 {
        self.age.potency_multiplier() * self.quality.multiplier()
    }

    pub fn value(&self) -> u64 {
        (self.age.base_value() as f32 * self.quality.multiplier()) as u64
    }
}

/// A monster core ingredient
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MonsterCore {
    pub core_type: MonsterCoreType,
    pub grade: MonsterCoreGrade,
    pub quality: PillQuality,
}

impl MonsterCore {
    pub fn new(core_type: MonsterCoreType, grade: MonsterCoreGrade, quality: PillQuality) -> Self {
        Self { core_type, grade, quality }
    }

    pub fn display_name(&self) -> String {
        format!("{} {} Core ({})", self.grade.name(), self.core_type.name(), self.quality.name())
    }

    pub fn total_potency(&self) -> f32 {
        self.grade.potency() * self.quality.multiplier()
    }
}

/// Combined ingredient type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AlchemyIngredient {
    Herb(SpiritHerb),
    Core(MonsterCore),
    Mineral { mineral_type: RareMineralType, quantity: u32 },
    HeavenlyMaterial { material_type: HeavenlyMaterialType, quantity: u32 },
}

impl AlchemyIngredient {
    pub fn name(&self) -> String {
        match self {
            Self::Herb(h) => h.display_name(),
            Self::Core(c) => c.display_name(),
            Self::Mineral { mineral_type, quantity } => {
                format!("{} x{}", mineral_type.name(), quantity)
            }
            Self::HeavenlyMaterial { material_type, quantity } => {
                format!("{} x{}", material_type.name(), quantity)
            }
        }
    }

    pub fn potency(&self) -> f32 {
        match self {
            Self::Herb(h) => h.total_potency(),
            Self::Core(c) => c.total_potency(),
            Self::Mineral { mineral_type, quantity } => {
                mineral_type.rarity().drop_chance_modifier() * 10.0 * (*quantity as f32)
            }
            Self::HeavenlyMaterial { material_type, quantity } => {
                material_type.potency() * (*quantity as f32)
            }
        }
    }
}

// ============================================================================
// Cauldrons and Fire Sources
// ============================================================================

/// Cauldron quality levels
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum CauldronGrade {
    Mortal,
    Spirit,
    Earth,
    Sky,
    Immortal,
    Divine,
    Celestial,
    Primordial,
}

impl CauldronGrade {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mortal => "Mortal Grade Cauldron",
            Self::Spirit => "Spirit Grade Cauldron",
            Self::Earth => "Earth Grade Cauldron",
            Self::Sky => "Sky Grade Cauldron",
            Self::Immortal => "Immortal Grade Cauldron",
            Self::Divine => "Divine Grade Cauldron",
            Self::Celestial => "Celestial Grade Cauldron",
            Self::Primordial => "Primordial Cauldron",
        }
    }

    pub fn success_bonus(&self) -> f32 {
        match self {
            Self::Mortal => 0.0,
            Self::Spirit => 0.05,
            Self::Earth => 0.10,
            Self::Sky => 0.15,
            Self::Immortal => 0.22,
            Self::Divine => 0.30,
            Self::Celestial => 0.40,
            Self::Primordial => 0.55,
        }
    }

    pub fn quality_bonus(&self) -> f32 {
        match self {
            Self::Mortal => 0.0,
            Self::Spirit => 0.02,
            Self::Earth => 0.05,
            Self::Sky => 0.08,
            Self::Immortal => 0.12,
            Self::Divine => 0.18,
            Self::Celestial => 0.25,
            Self::Primordial => 0.35,
        }
    }

    pub fn max_pill_grade(&self) -> PillGrade {
        match self {
            Self::Mortal => PillGrade::Common,
            Self::Spirit => PillGrade::Uncommon,
            Self::Earth => PillGrade::Rare,
            Self::Sky => PillGrade::Epic,
            Self::Immortal => PillGrade::Legendary,
            Self::Divine => PillGrade::Divine,
            Self::Celestial | Self::Primordial => PillGrade::Celestial,
        }
    }

    pub fn capacity(&self) -> u32 {
        match self {
            Self::Mortal => 3,
            Self::Spirit => 5,
            Self::Earth => 7,
            Self::Sky => 9,
            Self::Immortal => 12,
            Self::Divine => 15,
            Self::Celestial => 18,
            Self::Primordial => 21,
        }
    }
}

/// Fire source types
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum FireSource {
    MortalFire,
    SpiritFire,
    BeastFire,
    EarthFire,
    HeavenlyFire,
    DivineFire,
    ChaosFlame,
    PrimordialFlame,
}

impl FireSource {
    pub fn name(&self) -> &'static str {
        match self {
            Self::MortalFire => "Mortal Fire",
            Self::SpiritFire => "Spirit Fire",
            Self::BeastFire => "Beast Fire",
            Self::EarthFire => "Earth Core Fire",
            Self::HeavenlyFire => "Heavenly Fire",
            Self::DivineFire => "Divine Fire",
            Self::ChaosFlame => "Chaos Flame",
            Self::PrimordialFlame => "Primordial Flame",
        }
    }

    pub fn temperature_control(&self) -> f32 {
        match self {
            Self::MortalFire => 0.5,
            Self::SpiritFire => 0.65,
            Self::BeastFire => 0.75,
            Self::EarthFire => 0.82,
            Self::HeavenlyFire => 0.90,
            Self::DivineFire => 0.95,
            Self::ChaosFlame => 0.85,
            Self::PrimordialFlame => 0.99,
        }
    }

    pub fn success_bonus(&self) -> f32 {
        match self {
            Self::MortalFire => 0.0,
            Self::SpiritFire => 0.05,
            Self::BeastFire => 0.10,
            Self::EarthFire => 0.15,
            Self::HeavenlyFire => 0.22,
            Self::DivineFire => 0.30,
            Self::ChaosFlame => 0.25,
            Self::PrimordialFlame => 0.45,
        }
    }

    pub fn purification_power(&self) -> f32 {
        match self {
            Self::MortalFire => 1.0,
            Self::SpiritFire => 1.5,
            Self::BeastFire => 2.0,
            Self::EarthFire => 3.0,
            Self::HeavenlyFire => 5.0,
            Self::DivineFire => 8.0,
            Self::ChaosFlame => 6.0,
            Self::PrimordialFlame => 15.0,
        }
    }

    pub fn required_rank(&self) -> AlchemistRank {
        match self {
            Self::MortalFire => AlchemistRank::Apprentice,
            Self::SpiritFire => AlchemistRank::Junior,
            Self::BeastFire => AlchemistRank::Intermediate,
            Self::EarthFire => AlchemistRank::Senior,
            Self::HeavenlyFire => AlchemistRank::Master,
            Self::DivineFire => AlchemistRank::PillKing,
            Self::ChaosFlame => AlchemistRank::PillEmperor,
            Self::PrimordialFlame => AlchemistRank::PillGod,
        }
    }
}

/// A cauldron instance
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cauldron {
    pub id: u64,
    pub name: String,
    pub grade: CauldronGrade,
    pub durability: u32,
    pub max_durability: u32,
    pub refinements_completed: u64,
    pub special_effects: Vec<CauldronEffect>,
}

impl Cauldron {
    pub fn new(id: u64, name: String, grade: CauldronGrade) -> Self {
        let max_durability = match grade {
            CauldronGrade::Mortal => 100,
            CauldronGrade::Spirit => 200,
            CauldronGrade::Earth => 400,
            CauldronGrade::Sky => 800,
            CauldronGrade::Immortal => 1500,
            CauldronGrade::Divine => 3000,
            CauldronGrade::Celestial => 6000,
            CauldronGrade::Primordial => u32::MAX,
        };

        Self {
            id,
            name,
            grade,
            durability: max_durability,
            max_durability,
            refinements_completed: 0,
            special_effects: Vec::new(),
        }
    }

    pub fn use_cauldron(&mut self) -> bool {
        if self.durability == 0 {
            return false;
        }
        self.durability = self.durability.saturating_sub(1);
        self.refinements_completed += 1;
        true
    }

    pub fn repair(&mut self, amount: u32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }

    pub fn is_broken(&self) -> bool {
        self.durability == 0
    }
}

/// Special effects a cauldron can have
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CauldronEffect {
    SuccessBonus(f32),
    QualityBonus(f32),
    ReducedFailureDamage(f32),
    BonusPillChance(f32),
    ReducedIngredientCost(f32),
    ElementalAffinity(ElementType),
    TribulationResistance(f32),
}

// ============================================================================
// Refining Techniques
// ============================================================================

/// Pill refining techniques
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RefiningTechnique {
    BasicRefinement,
    StandardRefinement,
    PreciseRefinement,
    NineRotationsMethod,
    ThousandTransformations,
    YinYangBalance,
    FiveElementsHarmony,
    HeavenlyPillMethod,
    DragonTigerFusion,
    CosmicCondensation,
    ChaosRefinement,
    DaoOfAlchemy,
    PrimordialCreation,
    UniversalHarmony,
}

impl RefiningTechnique {
    pub fn name(&self) -> &'static str {
        match self {
            Self::BasicRefinement => "Basic Refinement",
            Self::StandardRefinement => "Standard Refinement",
            Self::PreciseRefinement => "Precise Refinement",
            Self::NineRotationsMethod => "Nine Rotations Method",
            Self::ThousandTransformations => "Thousand Transformations",
            Self::YinYangBalance => "Yin-Yang Balance",
            Self::FiveElementsHarmony => "Five Elements Harmony",
            Self::HeavenlyPillMethod => "Heavenly Pill Method",
            Self::DragonTigerFusion => "Dragon Tiger Fusion",
            Self::CosmicCondensation => "Cosmic Condensation",
            Self::ChaosRefinement => "Chaos Refinement",
            Self::DaoOfAlchemy => "Dao of Alchemy",
            Self::PrimordialCreation => "Primordial Creation",
            Self::UniversalHarmony => "Universal Harmony",
        }
    }

    pub fn required_rank(&self) -> AlchemistRank {
        match self {
            Self::BasicRefinement => AlchemistRank::Apprentice,
            Self::StandardRefinement => AlchemistRank::Junior,
            Self::PreciseRefinement => AlchemistRank::Intermediate,
            Self::NineRotationsMethod | Self::ThousandTransformations => AlchemistRank::Senior,
            Self::YinYangBalance | Self::FiveElementsHarmony => AlchemistRank::Master,
            Self::HeavenlyPillMethod | Self::DragonTigerFusion => AlchemistRank::Grandmaster,
            Self::CosmicCondensation | Self::ChaosRefinement => AlchemistRank::PillKing,
            Self::DaoOfAlchemy => AlchemistRank::PillEmperor,
            Self::PrimordialCreation | Self::UniversalHarmony => AlchemistRank::PillGod,
        }
    }

    pub fn success_modifier(&self) -> f32 {
        match self {
            Self::BasicRefinement => 0.0,
            Self::StandardRefinement => 0.05,
            Self::PreciseRefinement => 0.10,
            Self::NineRotationsMethod => 0.15,
            Self::ThousandTransformations => 0.18,
            Self::YinYangBalance => 0.22,
            Self::FiveElementsHarmony => 0.25,
            Self::HeavenlyPillMethod => 0.30,
            Self::DragonTigerFusion => 0.35,
            Self::CosmicCondensation => 0.40,
            Self::ChaosRefinement => 0.38,
            Self::DaoOfAlchemy => 0.50,
            Self::PrimordialCreation => 0.60,
            Self::UniversalHarmony => 0.75,
        }
    }

    pub fn quality_modifier(&self) -> f32 {
        match self {
            Self::BasicRefinement => 0.0,
            Self::StandardRefinement => 0.02,
            Self::PreciseRefinement => 0.05,
            Self::NineRotationsMethod => 0.08,
            Self::ThousandTransformations => 0.10,
            Self::YinYangBalance => 0.12,
            Self::FiveElementsHarmony => 0.15,
            Self::HeavenlyPillMethod => 0.20,
            Self::DragonTigerFusion => 0.25,
            Self::CosmicCondensation => 0.30,
            Self::ChaosRefinement => 0.28,
            Self::DaoOfAlchemy => 0.40,
            Self::PrimordialCreation => 0.50,
            Self::UniversalHarmony => 0.65,
        }
    }

    pub fn batch_bonus(&self) -> u32 {
        match self {
            Self::BasicRefinement | Self::StandardRefinement | Self::PreciseRefinement => 0,
            Self::NineRotationsMethod | Self::ThousandTransformations => 1,
            Self::YinYangBalance | Self::FiveElementsHarmony => 2,
            Self::HeavenlyPillMethod | Self::DragonTigerFusion => 3,
            Self::CosmicCondensation | Self::ChaosRefinement | Self::DaoOfAlchemy => 4,
            Self::PrimordialCreation | Self::UniversalHarmony => 5,
        }
    }
}

// ============================================================================
// Pills and Refinement Results
// ============================================================================

/// A refined pill instance
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pill {
    pub id: u64,
    pub pill_type: PillType,
    pub grade: PillGrade,
    pub quality: PillQuality,
    pub effect_value: i32,
    pub duration: u32,
    pub side_effects: Vec<SideEffect>,
    pub pill_patterns: u8,
    pub has_pill_tribulation_mark: bool,
    pub refiner_name: Option<String>,
}

impl Pill {
    pub fn new(id: u64, pill_type: PillType, grade: PillGrade, quality: PillQuality) -> Self {
        let base_effect = pill_type.base_effect();
        let effect_value = (base_effect as f32 * grade.effect_multiplier() * quality.multiplier()) as i32;
        let duration = pill_type.duration();
        let pill_patterns = match quality {
            PillQuality::Low => 1,
            PillQuality::Medium => 3,
            PillQuality::High => 6,
            PillQuality::Perfect => 9,
        };

        Self {
            id,
            pill_type,
            grade,
            quality,
            effect_value,
            duration,
            side_effects: Vec::new(),
            pill_patterns,
            has_pill_tribulation_mark: false,
            refiner_name: None,
        }
    }

    pub fn display_name(&self) -> String {
        let tribulation_mark = if self.has_pill_tribulation_mark { " [Tribulation]" } else { "" };
        format!(
            "{} {} {} ({} patterns){}",
            self.quality.name(),
            self.grade.name(),
            self.pill_type.name(),
            self.pill_patterns,
            tribulation_mark
        )
    }

    pub fn value(&self) -> u64 {
        let base = self.grade.base_value();
        let quality_mult = self.quality.multiplier();
        let pattern_mult = 1.0 + (self.pill_patterns as f32 * 0.1);
        let tribulation_mult = if self.has_pill_tribulation_mark { 2.0 } else { 1.0 };
        (base as f32 * quality_mult * pattern_mult * tribulation_mult) as u64
    }
}

// ============================================================================
// Side Effects and Addiction
// ============================================================================

/// Possible side effects from pills
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SideEffect {
    Nausea { duration: u32 },
    Dizziness { duration: u32 },
    Fatigue { duration: u32 },
    MinorQiDeviation { damage: i32 },
    TemporaryStatReduction { stat: StatType, amount: i32, duration: u32 },
    QiBlockage { duration: u32 },
    MeridianDamage { severity: u32 },
    SpiritWeakening { duration: u32 },
    MajorQiDeviation { damage: i32, duration: u32 },
    FoundationDamage { permanent_reduction: i32 },
    SoulDamage { severity: u32 },
    CultivationRegression { levels: u32 },
    WithdrawalSymptoms { severity: u32 },
    IncreasedTolerance { pill_category: PillCategory },
    DependencyFormation { pill_type: PillType },
}

impl SideEffect {
    pub fn description(&self) -> String {
        match self {
            Self::Nausea { duration } => format!("Nausea for {} turns", duration),
            Self::Dizziness { duration } => format!("Dizziness for {} turns", duration),
            Self::Fatigue { duration } => format!("Fatigue for {} turns", duration),
            Self::MinorQiDeviation { damage } => format!("Minor Qi deviation ({} damage)", damage),
            Self::TemporaryStatReduction { stat, amount, duration } => {
                format!("-{} {:?} for {} turns", amount, stat, duration)
            }
            Self::QiBlockage { duration } => format!("Qi blockage for {} turns", duration),
            Self::MeridianDamage { severity } => format!("Meridian damage (severity: {})", severity),
            Self::SpiritWeakening { duration } => format!("Spirit weakened for {} turns", duration),
            Self::MajorQiDeviation { damage, duration } => {
                format!("Major Qi deviation ({} damage, {} turns)", damage, duration)
            }
            Self::FoundationDamage { permanent_reduction } => {
                format!("Foundation damaged (-{} permanent)", permanent_reduction)
            }
            Self::SoulDamage { severity } => format!("Soul damage (severity: {})", severity),
            Self::CultivationRegression { levels } => format!("Cultivation regressed by {} levels", levels),
            Self::WithdrawalSymptoms { severity } => format!("Withdrawal symptoms (severity: {})", severity),
            Self::IncreasedTolerance { pill_category } => {
                format!("Increased tolerance to {:?} pills", pill_category)
            }
            Self::DependencyFormation { pill_type } => format!("Dependency on {} formed", pill_type.name()),
        }
    }

    pub fn severity(&self) -> u32 {
        match self {
            Self::Nausea { .. } | Self::Dizziness { .. } | Self::Fatigue { .. } => 1,
            Self::MinorQiDeviation { .. } => 2,
            Self::TemporaryStatReduction { .. } | Self::QiBlockage { .. } => 3,
            Self::MeridianDamage { severity } => 3 + severity,
            Self::SpiritWeakening { .. } => 4,
            Self::MajorQiDeviation { .. } => 5,
            Self::FoundationDamage { .. } | Self::SoulDamage { severity } => 6 + severity,
            Self::CultivationRegression { levels } => 7 + levels,
            Self::WithdrawalSymptoms { severity } => 2 + severity,
            Self::IncreasedTolerance { .. } => 3,
            Self::DependencyFormation { .. } => 5,
        }
    }
}

/// Stat types for side effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StatType {
    Strength,
    Agility,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    QiCapacity,
    SpiritPower,
}

/// Addiction tracking for a specific pill type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PillAddiction {
    pub pill_type: PillType,
    pub addiction_level: u32,
    pub times_consumed: u64,
    pub last_consumed_turn: u64,
    pub tolerance_level: f32,
    pub withdrawal_active: bool,
}

impl PillAddiction {
    pub fn new(pill_type: PillType) -> Self {
        Self {
            pill_type,
            addiction_level: 0,
            times_consumed: 0,
            last_consumed_turn: 0,
            tolerance_level: 1.0,
            withdrawal_active: false,
        }
    }

    pub fn consume(&mut self, current_turn: u64) {
        self.times_consumed += 1;
        self.last_consumed_turn = current_turn;
        self.withdrawal_active = false;
        let addiction_increase = self.pill_type.addiction_potential() / 10;
        self.addiction_level = (self.addiction_level + addiction_increase).min(MAX_ADDICTION_LEVEL);
        self.tolerance_level += 0.05;
    }

    pub fn update(&mut self, current_turn: u64) {
        let turns_since_last = current_turn.saturating_sub(self.last_consumed_turn);
        if self.addiction_level > 20 && turns_since_last > 100 {
            self.withdrawal_active = true;
        }
        if turns_since_last > 500 && self.addiction_level > 0 {
            self.addiction_level = self.addiction_level.saturating_sub(ADDICTION_DECAY_RATE);
            self.tolerance_level = (self.tolerance_level - 0.01).max(1.0);
        }
    }

    pub fn effectiveness_multiplier(&self) -> f32 {
        1.0 / self.tolerance_level
    }

    pub fn withdrawal_severity(&self) -> u32 {
        if self.withdrawal_active { self.addiction_level / 10 } else { 0 }
    }
}

// ============================================================================
// Pill Tribulation
// ============================================================================

/// Types of pill tribulations
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TribulationType {
    ThunderTribulation,
    FireTribulation,
    HeartDemonTribulation,
    HeavenlyWrath,
    CosmicJudgment,
    ChaosStorm,
}

impl TribulationType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ThunderTribulation => "Thunder Tribulation",
            Self::FireTribulation => "Fire Tribulation",
            Self::HeartDemonTribulation => "Heart Demon Tribulation",
            Self::HeavenlyWrath => "Heavenly Wrath",
            Self::CosmicJudgment => "Cosmic Judgment",
            Self::ChaosStorm => "Chaos Storm",
        }
    }

    pub fn waves(&self) -> u32 {
        match self {
            Self::ThunderTribulation => 3,
            Self::FireTribulation => 3,
            Self::HeartDemonTribulation => 1,
            Self::HeavenlyWrath => 6,
            Self::CosmicJudgment => 9,
            Self::ChaosStorm => 12,
        }
    }

    pub fn base_damage(&self) -> i32 {
        match self {
            Self::ThunderTribulation => 100,
            Self::FireTribulation => 80,
            Self::HeartDemonTribulation => 0,
            Self::HeavenlyWrath => 200,
            Self::CosmicJudgment => 500,
            Self::ChaosStorm => 1000,
        }
    }
}

/// A pill tribulation event
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PillTribulation {
    pub tribulation_type: TribulationType,
    pub pill_grade: PillGrade,
    pub total_waves: u32,
    pub current_wave: u32,
    pub damage_per_wave: i32,
    pub success_bonus_on_survive: f32,
}

impl PillTribulation {
    pub fn new(pill_grade: PillGrade) -> Self {
        let tribulation_type = match pill_grade {
            PillGrade::Epic => TribulationType::ThunderTribulation,
            PillGrade::Legendary => TribulationType::FireTribulation,
            PillGrade::Divine => TribulationType::HeavenlyWrath,
            PillGrade::Celestial => TribulationType::CosmicJudgment,
            _ => TribulationType::ThunderTribulation,
        };

        let damage_multiplier = pill_grade.effect_multiplier();
        let total_waves = tribulation_type.waves();
        let damage_per_wave = (tribulation_type.base_damage() as f32 * damage_multiplier) as i32;

        Self {
            tribulation_type,
            pill_grade,
            total_waves,
            current_wave: 0,
            damage_per_wave,
            success_bonus_on_survive: 0.2 * (pill_grade as u8 as f32),
        }
    }

    pub fn next_wave(&mut self) -> Option<i32> {
        if self.current_wave < self.total_waves {
            self.current_wave += 1;
            Some(self.damage_per_wave)
        } else {
            None
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_wave >= self.total_waves
    }

    pub fn survival_reward(&self) -> (f32, bool) {
        (self.success_bonus_on_survive, true)
    }
}

// ============================================================================
// Pill Recipe
// ============================================================================

/// A recipe for refining a specific pill
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PillRecipe {
    pub pill_type: PillType,
    pub grade: PillGrade,
    pub required_herbs: Vec<(SpiritHerbType, HerbAge, u32)>,
    pub required_cores: Vec<(MonsterCoreType, MonsterCoreGrade, u32)>,
    pub required_minerals: Vec<(RareMineralType, u32)>,
    pub required_heavenly: Vec<(HeavenlyMaterialType, u32)>,
    pub base_success_rate: f32,
    pub required_technique: RefiningTechnique,
    pub required_fire: FireSource,
    pub required_cauldron: CauldronGrade,
    pub refining_time: u32,
    pub discovered: bool,
}

impl PillRecipe {
    pub fn display(&self) -> String {
        format!(
            "{} {} - Success: {:.0}%",
            self.grade.name(),
            self.pill_type.name(),
            self.base_success_rate * 100.0
        )
    }
}

// ============================================================================
// Alchemy System
// ============================================================================

/// Result of a refinement attempt
#[derive(Clone, Debug)]
pub enum RefinementResult {
    Success {
        pills: Vec<Pill>,
        experience_gained: u64,
        triggered_tribulation: bool,
    },
    PartialSuccess {
        pills: Vec<Pill>,
        failed_count: u32,
        experience_gained: u64,
    },
    Failure {
        reason: RefinementFailure,
        ingredients_lost: bool,
        cauldron_damage: u32,
    },
    TribulationTriggered {
        tribulation: PillTribulation,
        pills_at_stake: Vec<Pill>,
    },
}

/// Reasons for refinement failure
#[derive(Clone, Debug)]
pub enum RefinementFailure {
    InsufficientSkill,
    InsufficientIngredients,
    CauldronTooWeak,
    FireTooWeak,
    TechniqueNotLearned,
    CauldronBroken,
    QiExhausted,
    TribulationFailed,
    RandomExplosion,
}

impl RefinementFailure {
    pub fn description(&self) -> &'static str {
        match self {
            Self::InsufficientSkill => "Your alchemy skill is insufficient for this recipe",
            Self::InsufficientIngredients => "Missing required ingredients",
            Self::CauldronTooWeak => "Cauldron grade is too low for this pill",
            Self::FireTooWeak => "Fire source is too weak for this refinement",
            Self::TechniqueNotLearned => "Required refinement technique not learned",
            Self::CauldronBroken => "Cauldron is broken and needs repair",
            Self::QiExhausted => "Insufficient Qi to complete refinement",
            Self::TribulationFailed => "Failed to survive the pill tribulation",
            Self::RandomExplosion => "The cauldron exploded during refinement!",
        }
    }
}

/// Main alchemy system struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AlchemySystem {
    pub alchemist_rank: AlchemistRank,
    pub alchemy_experience: u64,
    pub total_pills_refined: u64,
    pub successful_refinements: u64,
    pub failed_refinements: u64,
    pub cauldrons: Vec<Cauldron>,
    pub active_cauldron_id: Option<u64>,
    pub unlocked_fires: Vec<FireSource>,
    pub active_fire: FireSource,
    pub learned_techniques: Vec<RefiningTechnique>,
    pub discovered_recipes: Vec<PillRecipe>,
    pub recipe_mastery: HashMap<PillType, u32>,
    pub herb_inventory: Vec<SpiritHerb>,
    pub core_inventory: Vec<MonsterCore>,
    pub mineral_inventory: HashMap<RareMineralType, u32>,
    pub heavenly_inventory: HashMap<HeavenlyMaterialType, u32>,
    pub pill_inventory: Vec<Pill>,
    pub addictions: HashMap<PillType, PillAddiction>,
    pub highest_grade_refined: PillGrade,
    pub perfect_pills_created: u64,
    pub tribulations_survived: u64,
    pub tribulations_failed: u64,
    pub active_tribulation: Option<PillTribulation>,
    pub current_turn: u64,
    next_pill_id: u64,
    next_cauldron_id: u64,
}

impl Default for AlchemySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AlchemySystem {
    pub fn new() -> Self {
        let mut system = Self {
            alchemist_rank: AlchemistRank::Apprentice,
            alchemy_experience: 0,
            total_pills_refined: 0,
            successful_refinements: 0,
            failed_refinements: 0,
            cauldrons: Vec::new(),
            active_cauldron_id: None,
            unlocked_fires: vec![FireSource::MortalFire],
            active_fire: FireSource::MortalFire,
            learned_techniques: vec![RefiningTechnique::BasicRefinement],
            discovered_recipes: Vec::new(),
            recipe_mastery: HashMap::new(),
            herb_inventory: Vec::new(),
            core_inventory: Vec::new(),
            mineral_inventory: HashMap::new(),
            heavenly_inventory: HashMap::new(),
            pill_inventory: Vec::new(),
            addictions: HashMap::new(),
            highest_grade_refined: PillGrade::Common,
            perfect_pills_created: 0,
            tribulations_survived: 0,
            tribulations_failed: 0,
            active_tribulation: None,
            current_turn: 0,
            next_pill_id: 1,
            next_cauldron_id: 1,
        };

        system.add_cauldron("Beginner's Cauldron".to_string(), CauldronGrade::Mortal);
        system.active_cauldron_id = Some(1);
        system.initialize_basic_recipes();
        system
    }

    fn initialize_basic_recipes(&mut self) {
        self.discovered_recipes.push(PillRecipe {
            pill_type: PillType::MinorQiPill,
            grade: PillGrade::Common,
            required_herbs: vec![(SpiritHerbType::SpiritGrass, HerbAge::Decade, 2)],
            required_cores: vec![],
            required_minerals: vec![(RareMineralType::SpiritStone, 1)],
            required_heavenly: vec![],
            base_success_rate: 0.85,
            required_technique: RefiningTechnique::BasicRefinement,
            required_fire: FireSource::MortalFire,
            required_cauldron: CauldronGrade::Mortal,
            refining_time: 10,
            discovered: true,
        });

        self.discovered_recipes.push(PillRecipe {
            pill_type: PillType::MinorHealingPill,
            grade: PillGrade::Common,
            required_herbs: vec![(SpiritHerbType::JadeLeaf, HerbAge::Decade, 2)],
            required_cores: vec![],
            required_minerals: vec![],
            required_heavenly: vec![],
            base_success_rate: 0.90,
            required_technique: RefiningTechnique::BasicRefinement,
            required_fire: FireSource::MortalFire,
            required_cauldron: CauldronGrade::Mortal,
            refining_time: 8,
            discovered: true,
        });

        self.discovered_recipes.push(PillRecipe {
            pill_type: PillType::MinorAntidotePill,
            grade: PillGrade::Common,
            required_herbs: vec![(SpiritHerbType::CloudMushroom, HerbAge::Decade, 1)],
            required_cores: vec![],
            required_minerals: vec![(RareMineralType::JadeEssence, 1)],
            required_heavenly: vec![],
            base_success_rate: 0.88,
            required_technique: RefiningTechnique::BasicRefinement,
            required_fire: FireSource::MortalFire,
            required_cauldron: CauldronGrade::Mortal,
            refining_time: 6,
            discovered: true,
        });
    }

    pub fn add_cauldron(&mut self, name: String, grade: CauldronGrade) -> u64 {
        let id = self.next_cauldron_id;
        self.next_cauldron_id += 1;
        self.cauldrons.push(Cauldron::new(id, name, grade));
        id
    }

    pub fn active_cauldron(&self) -> Option<&Cauldron> {
        self.active_cauldron_id.and_then(|id| self.cauldrons.iter().find(|c| c.id == id))
    }

    pub fn active_cauldron_mut(&mut self) -> Option<&mut Cauldron> {
        let id = self.active_cauldron_id?;
        self.cauldrons.iter_mut().find(|c| c.id == id)
    }

    pub fn set_active_cauldron(&mut self, cauldron_id: u64) -> bool {
        if self.cauldrons.iter().any(|c| c.id == cauldron_id) {
            self.active_cauldron_id = Some(cauldron_id);
            true
        } else {
            false
        }
    }

    pub fn unlock_fire(&mut self, fire: FireSource) -> bool {
        if fire.required_rank() <= self.alchemist_rank && !self.unlocked_fires.contains(&fire) {
            self.unlocked_fires.push(fire);
            true
        } else {
            false
        }
    }

    pub fn set_active_fire(&mut self, fire: FireSource) -> bool {
        if self.unlocked_fires.contains(&fire) {
            self.active_fire = fire;
            true
        } else {
            false
        }
    }

    pub fn learn_technique(&mut self, technique: RefiningTechnique) -> bool {
        if technique.required_rank() <= self.alchemist_rank && !self.learned_techniques.contains(&technique) {
            self.learned_techniques.push(technique);
            true
        } else {
            false
        }
    }

    pub fn add_experience(&mut self, exp: u64) -> Option<AlchemistRank> {
        self.alchemy_experience += exp;
        let required = self.alchemist_rank.exp_for_next();
        if self.alchemy_experience >= required {
            if let Some(next_rank) = self.alchemist_rank.next_rank() {
                self.alchemist_rank = next_rank;
                return Some(next_rank);
            }
        }
        None
    }

    pub fn can_refine(&self, recipe: &PillRecipe) -> Result<(), RefinementFailure> {
        if self.alchemist_rank < recipe.grade.required_rank() {
            return Err(RefinementFailure::InsufficientSkill);
        }
        let cauldron = self.active_cauldron().ok_or(RefinementFailure::CauldronBroken)?;
        if cauldron.is_broken() {
            return Err(RefinementFailure::CauldronBroken);
        }
        if cauldron.grade < recipe.required_cauldron {
            return Err(RefinementFailure::CauldronTooWeak);
        }
        if self.active_fire < recipe.required_fire {
            return Err(RefinementFailure::FireTooWeak);
        }
        if !self.learned_techniques.contains(&recipe.required_technique) {
            return Err(RefinementFailure::TechniqueNotLearned);
        }
        for (herb_type, age, count) in &recipe.required_herbs {
            let available = self.herb_inventory.iter()
                .filter(|h| h.herb_type == *herb_type && h.age >= *age)
                .count() as u32;
            if available < *count {
                return Err(RefinementFailure::InsufficientIngredients);
            }
        }
        for (mineral_type, count) in &recipe.required_minerals {
            let available = self.mineral_inventory.get(mineral_type).copied().unwrap_or(0);
            if available < *count {
                return Err(RefinementFailure::InsufficientIngredients);
            }
        }
        Ok(())
    }

    pub fn calculate_success_rate(&self, recipe: &PillRecipe, technique: RefiningTechnique) -> f32 {
        let base_rate = recipe.base_success_rate;
        let rank_bonus = self.alchemist_rank.success_bonus();
        let technique_bonus = technique.success_modifier();
        let cauldron_bonus = self.active_cauldron().map(|c| c.grade.success_bonus()).unwrap_or(0.0);
        let fire_bonus = self.active_fire.success_bonus();
        let mastery_bonus = self.recipe_mastery.get(&recipe.pill_type).map(|m| *m as f32 * 0.002).unwrap_or(0.0);
        (base_rate + rank_bonus + technique_bonus + cauldron_bonus + fire_bonus + mastery_bonus).clamp(0.01, 0.99)
    }

    pub fn refine_pill(&mut self, recipe: &PillRecipe, technique: RefiningTechnique, batch_size: u32, rng_seed: u64) -> RefinementResult {
        if let Err(failure) = self.can_refine(recipe) {
            return RefinementResult::Failure {
                reason: failure,
                ingredients_lost: false,
                cauldron_damage: 0,
            };
        }

        let actual_batch_size = batch_size.min(self.alchemist_rank.max_batch_size());
        let success_rate = self.calculate_success_rate(recipe, technique);
        self.consume_recipe_ingredients(recipe, actual_batch_size);

        if let Some(cauldron) = self.active_cauldron_mut() {
            cauldron.use_cauldron();
        }

        let mut successful_pills = Vec::new();
        let mut failed_count = 0;
        let mut total_exp = 0u64;

        for i in 0..actual_batch_size {
            let roll = simple_rng(rng_seed.wrapping_add(i as u64)) as f32 / u64::MAX as f32;
            if roll <= success_rate {
                let quality = self.determine_quality(rng_seed.wrapping_add(i as u64 + 1000));
                let pill = Pill::new(self.next_pill_id, recipe.pill_type, recipe.grade, quality);
                self.next_pill_id += 1;
                if quality == PillQuality::Perfect {
                    self.perfect_pills_created += 1;
                }
                successful_pills.push(pill);
                total_exp += recipe.grade.base_value() / 10;
            } else {
                failed_count += 1;
            }
        }

        self.total_pills_refined += successful_pills.len() as u64;
        *self.recipe_mastery.entry(recipe.pill_type).or_insert(0) += 1;

        if recipe.grade > self.highest_grade_refined && !successful_pills.is_empty() {
            self.highest_grade_refined = recipe.grade;
        }

        let tribulation_chance = recipe.grade.tribulation_chance();
        let tribulation_roll = simple_rng(rng_seed.wrapping_add(9999)) as f32 / u64::MAX as f32;

        if tribulation_roll < tribulation_chance && recipe.pill_type.triggers_tribulation() {
            let tribulation = PillTribulation::new(recipe.grade);
            self.active_tribulation = Some(tribulation.clone());
            return RefinementResult::TribulationTriggered {
                tribulation,
                pills_at_stake: successful_pills,
            };
        }

        self.add_experience(total_exp);

        if failed_count == 0 {
            self.successful_refinements += 1;
            self.pill_inventory.extend(successful_pills.clone());
            RefinementResult::Success {
                pills: successful_pills,
                experience_gained: total_exp,
                triggered_tribulation: false,
            }
        } else if !successful_pills.is_empty() {
            self.pill_inventory.extend(successful_pills.clone());
            RefinementResult::PartialSuccess {
                pills: successful_pills,
                failed_count,
                experience_gained: total_exp,
            }
        } else {
            self.failed_refinements += 1;
            RefinementResult::Failure {
                reason: RefinementFailure::RandomExplosion,
                ingredients_lost: true,
                cauldron_damage: 5,
            }
        }
    }

    fn consume_recipe_ingredients(&mut self, recipe: &PillRecipe, batch_size: u32) {
        for (herb_type, min_age, count) in &recipe.required_herbs {
            let total_needed = count * batch_size;
            let mut consumed = 0;
            self.herb_inventory.retain(|h| {
                if consumed < total_needed && h.herb_type == *herb_type && h.age >= *min_age {
                    consumed += 1;
                    false
                } else {
                    true
                }
            });
        }
        for (mineral_type, count) in &recipe.required_minerals {
            let total_needed = count * batch_size;
            if let Some(amount) = self.mineral_inventory.get_mut(mineral_type) {
                *amount = amount.saturating_sub(total_needed);
            }
        }
        for (core_type, min_grade, count) in &recipe.required_cores {
            let total_needed = count * batch_size;
            let mut consumed = 0;
            self.core_inventory.retain(|c| {
                if consumed < total_needed && c.core_type == *core_type && c.grade >= *min_grade {
                    consumed += 1;
                    false
                } else {
                    true
                }
            });
        }
        for (material_type, count) in &recipe.required_heavenly {
            let total_needed = count * batch_size;
            if let Some(amount) = self.heavenly_inventory.get_mut(material_type) {
                *amount = amount.saturating_sub(total_needed);
            }
        }
    }

    fn determine_quality(&self, rng_seed: u64) -> PillQuality {
        let roll = simple_rng(rng_seed) as f32 / u64::MAX as f32;
        let quality_bonus = self.alchemist_rank.quality_bonus()
            + self.active_cauldron().map(|c| c.grade.quality_bonus()).unwrap_or(0.0);
        let adjusted_roll = roll + quality_bonus;

        if adjusted_roll > 0.95 {
            PillQuality::Perfect
        } else if adjusted_roll > 0.80 {
            PillQuality::High
        } else if adjusted_roll > 0.50 {
            PillQuality::Medium
        } else {
            PillQuality::Low
        }
    }

    pub fn consume_pill(&mut self, pill_index: usize) -> Option<PillConsumptionResult> {
        if pill_index >= self.pill_inventory.len() {
            return None;
        }
        let pill = self.pill_inventory.remove(pill_index);
        let addiction = self.addictions.entry(pill.pill_type).or_insert_with(|| PillAddiction::new(pill.pill_type));
        addiction.consume(self.current_turn);
        let effectiveness = addiction.effectiveness_multiplier();
        let effective_value = (pill.effect_value as f32 * effectiveness) as i32;
        let side_effects = self.determine_side_effects(&pill, addiction.addiction_level);

        Some(PillConsumptionResult {
            pill_type: pill.pill_type,
            effect_value: effective_value,
            duration: pill.duration,
            side_effects,
            addiction_increased: addiction.addiction_level,
            tolerance_warning: addiction.tolerance_level > 1.5,
        })
    }

    fn determine_side_effects(&self, pill: &Pill, addiction_level: u32) -> Vec<SideEffect> {
        let mut effects = pill.side_effects.clone();
        if addiction_level > 50 {
            effects.push(SideEffect::IncreasedTolerance { pill_category: pill.pill_type.category() });
        }
        if addiction_level > 75 {
            effects.push(SideEffect::DependencyFormation { pill_type: pill.pill_type });
        }
        effects
    }

    pub fn process_tribulation_wave(&mut self, _damage_taken: i32) -> TribulationWaveResult {
        if let Some(ref mut tribulation) = self.active_tribulation {
            let wave_damage = tribulation.next_wave();
            if tribulation.is_complete() {
                self.tribulations_survived += 1;
                let (quality_bonus, gets_mark) = tribulation.survival_reward();
                self.active_tribulation = None;
                TribulationWaveResult::Complete { quality_bonus, pills_get_mark: gets_mark }
            } else {
                TribulationWaveResult::Continue {
                    wave: tribulation.current_wave,
                    total_waves: tribulation.total_waves,
                    damage: wave_damage.unwrap_or(0),
                }
            }
        } else {
            TribulationWaveResult::NoTribulation
        }
    }

    pub fn fail_tribulation(&mut self) {
        if self.active_tribulation.is_some() {
            self.tribulations_failed += 1;
            self.active_tribulation = None;
        }
    }

    pub fn update_addictions(&mut self) {
        for addiction in self.addictions.values_mut() {
            addiction.update(self.current_turn);
        }
    }

    pub fn advance_turn(&mut self) {
        self.current_turn += 1;
        if self.current_turn % 10 == 0 {
            self.update_addictions();
        }
    }

    pub fn get_summary(&self) -> AlchemySummary {
        AlchemySummary {
            rank: self.alchemist_rank,
            experience: self.alchemy_experience,
            exp_to_next: self.alchemist_rank.exp_for_next(),
            total_refined: self.total_pills_refined,
            success_rate: if self.total_pills_refined > 0 {
                self.successful_refinements as f32 / (self.successful_refinements + self.failed_refinements) as f32
            } else {
                0.0
            },
            highest_grade: self.highest_grade_refined,
            perfect_pills: self.perfect_pills_created,
            tribulations_survived: self.tribulations_survived,
            cauldron_count: self.cauldrons.len(),
            techniques_learned: self.learned_techniques.len(),
            recipes_discovered: self.discovered_recipes.len(),
            active_addictions: self.addictions.values().filter(|a| a.addiction_level > 20).count(),
        }
    }

    pub fn add_herb(&mut self, herb: SpiritHerb) {
        self.herb_inventory.push(herb);
    }

    pub fn add_core(&mut self, core: MonsterCore) {
        self.core_inventory.push(core);
    }

    pub fn add_mineral(&mut self, mineral_type: RareMineralType, quantity: u32) {
        *self.mineral_inventory.entry(mineral_type).or_insert(0) += quantity;
    }

    pub fn add_heavenly_material(&mut self, material_type: HeavenlyMaterialType, quantity: u32) {
        *self.heavenly_inventory.entry(material_type).or_insert(0) += quantity;
    }

    pub fn discover_recipe(&mut self, recipe: PillRecipe) {
        if !self.discovered_recipes.iter().any(|r| r.pill_type == recipe.pill_type && r.grade == recipe.grade) {
            self.discovered_recipes.push(recipe);
        }
    }

    pub fn available_recipes(&self) -> Vec<&PillRecipe> {
        self.discovered_recipes.iter().filter(|r| self.can_refine(r).is_ok()).collect()
    }

    pub fn all_recipes(&self) -> &[PillRecipe] {
        &self.discovered_recipes
    }
}

/// Result of consuming a pill
#[derive(Clone, Debug)]
pub struct PillConsumptionResult {
    pub pill_type: PillType,
    pub effect_value: i32,
    pub duration: u32,
    pub side_effects: Vec<SideEffect>,
    pub addiction_increased: u32,
    pub tolerance_warning: bool,
}

/// Result of a tribulation wave
#[derive(Clone, Debug)]
pub enum TribulationWaveResult {
    Continue { wave: u32, total_waves: u32, damage: i32 },
    Complete { quality_bonus: f32, pills_get_mark: bool },
    NoTribulation,
}

/// Summary of alchemist status
#[derive(Clone, Debug)]
pub struct AlchemySummary {
    pub rank: AlchemistRank,
    pub experience: u64,
    pub exp_to_next: u64,
    pub total_refined: u64,
    pub success_rate: f32,
    pub highest_grade: PillGrade,
    pub perfect_pills: u64,
    pub tribulations_survived: u64,
    pub cauldron_count: usize,
    pub techniques_learned: usize,
    pub recipes_discovered: usize,
    pub active_addictions: usize,
}

// ============================================================================
// Utility Functions
// ============================================================================

fn simple_rng(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

pub fn generate_random_herb(rng_seed: u64, area_level: u32) -> SpiritHerb {
    let herb_types = [
        SpiritHerbType::SpiritGrass, SpiritHerbType::QiGatheringFlower,
        SpiritHerbType::MoonlightOrchid, SpiritHerbType::SunfireBloom,
        SpiritHerbType::JadeLeaf, SpiritHerbType::CloudMushroom,
        SpiritHerbType::DragonBloodVine, SpiritHerbType::PhoenixTailFern,
    ];

    let rng = simple_rng(rng_seed);
    let herb_type = herb_types[(rng as usize) % herb_types.len()];

    let age_roll = (simple_rng(rng_seed + 1) % 100) as u32 + area_level * 5;
    let age = if age_roll < 40 { HerbAge::Decade }
        else if age_roll < 70 { HerbAge::Century }
        else if age_roll < 85 { HerbAge::Millennium }
        else if age_roll < 95 { HerbAge::TenThousand }
        else { HerbAge::HundredThousand };

    let quality_roll = simple_rng(rng_seed + 2) % 100;
    let quality = if quality_roll < 35 { PillQuality::Low }
        else if quality_roll < 75 { PillQuality::Medium }
        else if quality_roll < 95 { PillQuality::High }
        else { PillQuality::Perfect };

    SpiritHerb::new(herb_type, age, quality)
}

pub fn generate_random_core(rng_seed: u64, monster_level: u32) -> MonsterCore {
    let core_types = [
        MonsterCoreType::WolfCore, MonsterCoreType::BearCore,
        MonsterCoreType::TigerCore, MonsterCoreType::EagleCore,
        MonsterCoreType::SerpentCore,
    ];

    let rng = simple_rng(rng_seed);
    let core_type = core_types[(rng as usize) % core_types.len()];

    let grade = match monster_level {
        0..=10 => MonsterCoreGrade::Mortal,
        11..=25 => MonsterCoreGrade::Spirit,
        26..=40 => MonsterCoreGrade::Earth,
        41..=60 => MonsterCoreGrade::Sky,
        61..=80 => MonsterCoreGrade::King,
        81..=95 => MonsterCoreGrade::Emperor,
        _ => MonsterCoreGrade::Saint,
    };

    let quality_roll = simple_rng(rng_seed + 1) % 100;
    let quality = if quality_roll < 40 { PillQuality::Low }
        else if quality_roll < 80 { PillQuality::Medium }
        else if quality_roll < 95 { PillQuality::High }
        else { PillQuality::Perfect };

    MonsterCore::new(core_type, grade, quality)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pill_creation() {
        let pill = Pill::new(1, PillType::MinorQiPill, PillGrade::Common, PillQuality::Medium);
        assert_eq!(pill.pill_type, PillType::MinorQiPill);
        assert_eq!(pill.grade, PillGrade::Common);
        assert!(pill.effect_value > 0);
    }

    #[test]
    fn test_alchemy_system_creation() {
        let system = AlchemySystem::new();
        assert_eq!(system.alchemist_rank, AlchemistRank::Apprentice);
        assert!(!system.cauldrons.is_empty());
        assert!(system.active_cauldron_id.is_some());
    }

    #[test]
    fn test_grade_ordering() {
        assert!(PillGrade::Common < PillGrade::Uncommon);
        assert!(PillGrade::Uncommon < PillGrade::Rare);
        assert!(PillGrade::Rare < PillGrade::Epic);
        assert!(PillGrade::Epic < PillGrade::Legendary);
        assert!(PillGrade::Legendary < PillGrade::Divine);
        assert!(PillGrade::Divine < PillGrade::Celestial);
    }

    #[test]
    fn test_rank_progression() {
        let mut system = AlchemySystem::new();
        assert_eq!(system.alchemist_rank, AlchemistRank::Apprentice);
        system.add_experience(100);
        assert_eq!(system.alchemist_rank, AlchemistRank::Junior);
    }

    #[test]
    fn test_cauldron_durability() {
        let mut cauldron = Cauldron::new(1, "Test".to_string(), CauldronGrade::Mortal);
        let initial = cauldron.durability;
        cauldron.use_cauldron();
        assert_eq!(cauldron.durability, initial - 1);
        assert!(!cauldron.is_broken());
    }

    #[test]
    fn test_herb_potency() {
        let herb1 = SpiritHerb::new(SpiritHerbType::SpiritGrass, HerbAge::Decade, PillQuality::Medium);
        let herb2 = SpiritHerb::new(SpiritHerbType::SpiritGrass, HerbAge::Millennium, PillQuality::Medium);
        assert!(herb2.total_potency() > herb1.total_potency());
    }

    #[test]
    fn test_addiction_tracking() {
        let mut addiction = PillAddiction::new(PillType::MinorQiPill);
        assert_eq!(addiction.addiction_level, 0);
        addiction.consume(100);
        assert!(addiction.addiction_level > 0);
        assert_eq!(addiction.times_consumed, 1);
    }

    #[test]
    fn test_tribulation_waves() {
        let mut tribulation = PillTribulation::new(PillGrade::Legendary);
        assert!(!tribulation.is_complete());
        while tribulation.next_wave().is_some() {}
        assert!(tribulation.is_complete());
    }

    #[test]
    fn test_pill_categories() {
        assert_eq!(PillType::MinorQiPill.category(), PillCategory::QiRestoration);
        assert_eq!(PillType::MinorHealingPill.category(), PillCategory::Healing);
        assert_eq!(PillType::FoundationBreakthroughPill.category(), PillCategory::Breakthrough);
    }

    #[test]
    fn test_fire_source_requirements() {
        assert_eq!(FireSource::MortalFire.required_rank(), AlchemistRank::Apprentice);
        assert!(FireSource::PrimordialFlame.required_rank() > FireSource::MortalFire.required_rank());
    }

    #[test]
    fn test_random_generation() {
        let herb = generate_random_herb(12345, 10);
        assert!(!herb.display_name().is_empty());
        let core = generate_random_core(54321, 50);
        assert!(!core.display_name().is_empty());
    }
}

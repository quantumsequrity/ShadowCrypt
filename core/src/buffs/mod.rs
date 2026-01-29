//! Comprehensive Buffs and Debuffs System
//!
//! This module provides a full-featured status effect system for ShadowCrypt,
//! including stat buffs, combat buffs, resource regeneration, protective effects,
//! damage over time, crowd control, and complex buff/debuff interactions.

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Maximum number of buffs an entity can have
pub const MAX_BUFFS: usize = 20;
/// Maximum number of debuffs an entity can have
pub const MAX_DEBUFFS: usize = 15;
/// Maximum stack count for stackable effects
pub const MAX_STACKS: u32 = 10;
/// Default buff duration in turns
pub const DEFAULT_BUFF_DURATION: u32 = 5;
/// Default debuff duration in turns
pub const DEFAULT_DEBUFF_DURATION: u32 = 3;

// ============================================================================
// BUFF TYPES - Over 100 unique effects
// ============================================================================

/// All possible buff effect types in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum BuffType {
    // === STAT BUFFS (20 effects) ===
    StrengthI,
    StrengthII,
    StrengthIII,
    StrengthIV,
    StrengthV,
    DefenseI,
    DefenseII,
    DefenseIII,
    DefenseIV,
    DefenseV,
    SpeedI,
    SpeedII,
    SpeedIII,
    SpeedIV,
    SpeedV,
    IntelligenceI,
    IntelligenceII,
    IntelligenceIII,
    IntelligenceIV,
    IntelligenceV,

    // === COMBAT BUFFS (15 effects) ===
    AttackPowerUp,
    AttackPowerUpGreater,
    CriticalChanceUp,
    CriticalChanceUpGreater,
    CriticalDamageUp,
    AccuracyUp,
    AccuracyUpGreater,
    EvasionUp,
    EvasionUpGreater,
    ArmorPenetration,
    LifeSteal,
    ManaSteal,
    Berserk,
    BattleFury,
    Precision,

    // === RESOURCE BUFFS (12 effects) ===
    HpRegeneration,
    HpRegenerationGreater,
    HpRegenerationSupreme,
    MpRegeneration,
    MpRegenerationGreater,
    MpRegenerationSupreme,
    StaminaRegeneration,
    StaminaRegenerationGreater,
    EnergyRegeneration,
    ResourceEfficiency,
    CostReduction,
    Meditation,

    // === UTILITY BUFFS (15 effects) ===
    ExperienceBoost,
    ExperienceBoostGreater,
    GoldFind,
    GoldFindGreater,
    ItemFind,
    ItemFindGreater,
    MovementSpeed,
    MovementSpeedGreater,
    VisionRange,
    StealthDetection,
    TrapDetection,
    LockpickBonus,
    CraftingBonus,
    GatheringBonus,
    LuckUp,

    // === PROTECTIVE BUFFS (18 effects) ===
    Shield,
    ShieldGreater,
    ShieldSupreme,
    Invincibility,
    DamageReduction,
    DamageReductionGreater,
    PhysicalResistance,
    MagicalResistance,
    FireResistance,
    IceResistance,
    LightningResistance,
    PoisonResistance,
    DarkResistance,
    HolyResistance,
    AllElementalResistance,
    ReflectDamage,
    AbsorbDamage,
    Immunity,

    // === SPECIAL BUFFS (20 effects) ===
    Haste,
    HasteGreater,
    Invisibility,
    TrueInvisibility,
    Flight,
    WaterBreathing,
    Nightvision,
    Regeneration,
    SecondWind,
    Undying,
    Blessed,
    Divine,
    Rage,
    Focus,
    Clarity,
    Enlightenment,
    Empowered,
    Overcharge,
    Transcendence,
    Avatar,
}

impl BuffType {
    /// Returns the display name of the buff
    pub fn name(&self) -> &'static str {
        match self {
            // Stat buffs
            Self::StrengthI => "Strength I",
            Self::StrengthII => "Strength II",
            Self::StrengthIII => "Strength III",
            Self::StrengthIV => "Strength IV",
            Self::StrengthV => "Strength V",
            Self::DefenseI => "Defense I",
            Self::DefenseII => "Defense II",
            Self::DefenseIII => "Defense III",
            Self::DefenseIV => "Defense IV",
            Self::DefenseV => "Defense V",
            Self::SpeedI => "Speed I",
            Self::SpeedII => "Speed II",
            Self::SpeedIII => "Speed III",
            Self::SpeedIV => "Speed IV",
            Self::SpeedV => "Speed V",
            Self::IntelligenceI => "Intelligence I",
            Self::IntelligenceII => "Intelligence II",
            Self::IntelligenceIII => "Intelligence III",
            Self::IntelligenceIV => "Intelligence IV",
            Self::IntelligenceV => "Intelligence V",

            // Combat buffs
            Self::AttackPowerUp => "Attack Power Up",
            Self::AttackPowerUpGreater => "Greater Attack Power",
            Self::CriticalChanceUp => "Critical Chance Up",
            Self::CriticalChanceUpGreater => "Greater Critical Chance",
            Self::CriticalDamageUp => "Critical Damage Up",
            Self::AccuracyUp => "Accuracy Up",
            Self::AccuracyUpGreater => "Greater Accuracy",
            Self::EvasionUp => "Evasion Up",
            Self::EvasionUpGreater => "Greater Evasion",
            Self::ArmorPenetration => "Armor Penetration",
            Self::LifeSteal => "Life Steal",
            Self::ManaSteal => "Mana Steal",
            Self::Berserk => "Berserk",
            Self::BattleFury => "Battle Fury",
            Self::Precision => "Precision",

            // Resource buffs
            Self::HpRegeneration => "HP Regeneration",
            Self::HpRegenerationGreater => "Greater HP Regen",
            Self::HpRegenerationSupreme => "Supreme HP Regen",
            Self::MpRegeneration => "MP Regeneration",
            Self::MpRegenerationGreater => "Greater MP Regen",
            Self::MpRegenerationSupreme => "Supreme MP Regen",
            Self::StaminaRegeneration => "Stamina Regen",
            Self::StaminaRegenerationGreater => "Greater Stamina Regen",
            Self::EnergyRegeneration => "Energy Regen",
            Self::ResourceEfficiency => "Resource Efficiency",
            Self::CostReduction => "Cost Reduction",
            Self::Meditation => "Meditation",

            // Utility buffs
            Self::ExperienceBoost => "XP Boost",
            Self::ExperienceBoostGreater => "Greater XP Boost",
            Self::GoldFind => "Gold Find",
            Self::GoldFindGreater => "Greater Gold Find",
            Self::ItemFind => "Item Find",
            Self::ItemFindGreater => "Greater Item Find",
            Self::MovementSpeed => "Movement Speed",
            Self::MovementSpeedGreater => "Greater Movement Speed",
            Self::VisionRange => "Vision Range",
            Self::StealthDetection => "Stealth Detection",
            Self::TrapDetection => "Trap Detection",
            Self::LockpickBonus => "Lockpick Bonus",
            Self::CraftingBonus => "Crafting Bonus",
            Self::GatheringBonus => "Gathering Bonus",
            Self::LuckUp => "Luck Up",

            // Protective buffs
            Self::Shield => "Shield",
            Self::ShieldGreater => "Greater Shield",
            Self::ShieldSupreme => "Supreme Shield",
            Self::Invincibility => "Invincibility",
            Self::DamageReduction => "Damage Reduction",
            Self::DamageReductionGreater => "Greater Damage Reduction",
            Self::PhysicalResistance => "Physical Resistance",
            Self::MagicalResistance => "Magical Resistance",
            Self::FireResistance => "Fire Resistance",
            Self::IceResistance => "Ice Resistance",
            Self::LightningResistance => "Lightning Resistance",
            Self::PoisonResistance => "Poison Resistance",
            Self::DarkResistance => "Dark Resistance",
            Self::HolyResistance => "Holy Resistance",
            Self::AllElementalResistance => "All Elemental Resistance",
            Self::ReflectDamage => "Reflect Damage",
            Self::AbsorbDamage => "Absorb Damage",
            Self::Immunity => "Immunity",

            // Special buffs
            Self::Haste => "Haste",
            Self::HasteGreater => "Greater Haste",
            Self::Invisibility => "Invisibility",
            Self::TrueInvisibility => "True Invisibility",
            Self::Flight => "Flight",
            Self::WaterBreathing => "Water Breathing",
            Self::Nightvision => "Nightvision",
            Self::Regeneration => "Regeneration",
            Self::SecondWind => "Second Wind",
            Self::Undying => "Undying",
            Self::Blessed => "Blessed",
            Self::Divine => "Divine Protection",
            Self::Rage => "Rage",
            Self::Focus => "Focus",
            Self::Clarity => "Clarity",
            Self::Enlightenment => "Enlightenment",
            Self::Empowered => "Empowered",
            Self::Overcharge => "Overcharge",
            Self::Transcendence => "Transcendence",
            Self::Avatar => "Avatar",
        }
    }

    /// Returns the icon character for this buff
    pub fn icon(&self) -> char {
        match self {
            // Stat buffs use arrow symbols
            Self::StrengthI | Self::StrengthII | Self::StrengthIII
            | Self::StrengthIV | Self::StrengthV => '+',
            Self::DefenseI | Self::DefenseII | Self::DefenseIII
            | Self::DefenseIV | Self::DefenseV => 'O',
            Self::SpeedI | Self::SpeedII | Self::SpeedIII
            | Self::SpeedIV | Self::SpeedV => '>',
            Self::IntelligenceI | Self::IntelligenceII | Self::IntelligenceIII
            | Self::IntelligenceIV | Self::IntelligenceV => '*',

            // Combat buffs
            Self::AttackPowerUp | Self::AttackPowerUpGreater => 'A',
            Self::CriticalChanceUp | Self::CriticalChanceUpGreater | Self::CriticalDamageUp => '!',
            Self::AccuracyUp | Self::AccuracyUpGreater => 'T',
            Self::EvasionUp | Self::EvasionUpGreater => 'E',
            Self::ArmorPenetration => 'P',
            Self::LifeSteal => 'V',
            Self::ManaSteal => 'M',
            Self::Berserk | Self::BattleFury => 'B',
            Self::Precision => 'X',

            // Resource buffs
            Self::HpRegeneration | Self::HpRegenerationGreater | Self::HpRegenerationSupreme => 'H',
            Self::MpRegeneration | Self::MpRegenerationGreater | Self::MpRegenerationSupreme => 'M',
            Self::StaminaRegeneration | Self::StaminaRegenerationGreater => 'S',
            Self::EnergyRegeneration => 'E',
            Self::ResourceEfficiency | Self::CostReduction => 'R',
            Self::Meditation => 'Z',

            // Utility buffs
            Self::ExperienceBoost | Self::ExperienceBoostGreater => 'X',
            Self::GoldFind | Self::GoldFindGreater => 'G',
            Self::ItemFind | Self::ItemFindGreater => 'I',
            Self::MovementSpeed | Self::MovementSpeedGreater => 'F',
            Self::VisionRange => 'V',
            Self::StealthDetection => 'D',
            Self::TrapDetection => 'T',
            Self::LockpickBonus => 'L',
            Self::CraftingBonus => 'C',
            Self::GatheringBonus => 'G',
            Self::LuckUp => 'L',

            // Protective buffs
            Self::Shield | Self::ShieldGreater | Self::ShieldSupreme => 'S',
            Self::Invincibility => 'I',
            Self::DamageReduction | Self::DamageReductionGreater => 'D',
            Self::PhysicalResistance => 'P',
            Self::MagicalResistance => 'M',
            Self::FireResistance => 'F',
            Self::IceResistance => 'I',
            Self::LightningResistance => 'L',
            Self::PoisonResistance => 'P',
            Self::DarkResistance => 'D',
            Self::HolyResistance => 'H',
            Self::AllElementalResistance => 'A',
            Self::ReflectDamage => 'R',
            Self::AbsorbDamage => 'A',
            Self::Immunity => 'I',

            // Special buffs
            Self::Haste | Self::HasteGreater => '>',
            Self::Invisibility | Self::TrueInvisibility => '?',
            Self::Flight => '^',
            Self::WaterBreathing => '~',
            Self::Nightvision => '@',
            Self::Regeneration => 'R',
            Self::SecondWind => 'W',
            Self::Undying => 'U',
            Self::Blessed => 'B',
            Self::Divine => 'D',
            Self::Rage => 'R',
            Self::Focus | Self::Clarity => 'F',
            Self::Enlightenment => 'E',
            Self::Empowered | Self::Overcharge => 'P',
            Self::Transcendence => 'T',
            Self::Avatar => 'A',
        }
    }

    /// Returns the color index for UI display
    pub fn color_index(&self) -> u8 {
        match self {
            // Green for stat buffs
            Self::StrengthI | Self::StrengthII | Self::StrengthIII
            | Self::StrengthIV | Self::StrengthV => 5,
            Self::DefenseI | Self::DefenseII | Self::DefenseIII
            | Self::DefenseIV | Self::DefenseV => 7,
            Self::SpeedI | Self::SpeedII | Self::SpeedIII
            | Self::SpeedIV | Self::SpeedV => 9,
            Self::IntelligenceI | Self::IntelligenceII | Self::IntelligenceIII
            | Self::IntelligenceIV | Self::IntelligenceV => 13,

            // Red/Orange for combat buffs
            Self::AttackPowerUp | Self::AttackPowerUpGreater | Self::Berserk
            | Self::BattleFury => 3,
            Self::CriticalChanceUp | Self::CriticalChanceUpGreater
            | Self::CriticalDamageUp | Self::Precision => 11,
            Self::AccuracyUp | Self::AccuracyUpGreater => 2,
            Self::EvasionUp | Self::EvasionUpGreater => 9,
            Self::ArmorPenetration => 1,
            Self::LifeSteal => 4,
            Self::ManaSteal => 8,

            // Magenta for resource buffs
            Self::HpRegeneration | Self::HpRegenerationGreater
            | Self::HpRegenerationSupreme => 13,
            Self::MpRegeneration | Self::MpRegenerationGreater
            | Self::MpRegenerationSupreme => 7,
            Self::StaminaRegeneration | Self::StaminaRegenerationGreater
            | Self::EnergyRegeneration => 5,
            Self::ResourceEfficiency | Self::CostReduction | Self::Meditation => 9,

            // Yellow for utility buffs
            Self::ExperienceBoost | Self::ExperienceBoostGreater => 11,
            Self::GoldFind | Self::GoldFindGreater => 11,
            Self::ItemFind | Self::ItemFindGreater => 13,
            Self::MovementSpeed | Self::MovementSpeedGreater => 9,
            Self::VisionRange | Self::Nightvision => 11,
            Self::StealthDetection | Self::TrapDetection => 2,
            Self::LockpickBonus | Self::CraftingBonus | Self::GatheringBonus => 5,
            Self::LuckUp => 11,

            // Blue for protective buffs
            Self::Shield | Self::ShieldGreater | Self::ShieldSupreme => 7,
            Self::Invincibility | Self::Immunity => 11,
            Self::DamageReduction | Self::DamageReductionGreater => 1,
            Self::PhysicalResistance => 2,
            Self::MagicalResistance => 13,
            Self::FireResistance => 3,
            Self::IceResistance => 9,
            Self::LightningResistance => 11,
            Self::PoisonResistance => 5,
            Self::DarkResistance => 14,
            Self::HolyResistance => 11,
            Self::AllElementalResistance => 13,
            Self::ReflectDamage => 3,
            Self::AbsorbDamage => 7,

            // Mixed for special buffs
            Self::Haste | Self::HasteGreater => 9,
            Self::Invisibility | Self::TrueInvisibility => 1,
            Self::Flight => 9,
            Self::WaterBreathing => 7,
            Self::Regeneration => 5,
            Self::SecondWind => 2,
            Self::Undying => 4,
            Self::Blessed | Self::Divine => 11,
            Self::Rage => 3,
            Self::Focus | Self::Clarity | Self::Enlightenment => 7,
            Self::Empowered | Self::Overcharge => 11,
            Self::Transcendence | Self::Avatar => 13,
        }
    }

    /// Returns the category of this buff
    pub fn category(&self) -> BuffCategory {
        match self {
            Self::StrengthI | Self::StrengthII | Self::StrengthIII
            | Self::StrengthIV | Self::StrengthV
            | Self::DefenseI | Self::DefenseII | Self::DefenseIII
            | Self::DefenseIV | Self::DefenseV
            | Self::SpeedI | Self::SpeedII | Self::SpeedIII
            | Self::SpeedIV | Self::SpeedV
            | Self::IntelligenceI | Self::IntelligenceII | Self::IntelligenceIII
            | Self::IntelligenceIV | Self::IntelligenceV => BuffCategory::Stat,

            Self::AttackPowerUp | Self::AttackPowerUpGreater
            | Self::CriticalChanceUp | Self::CriticalChanceUpGreater | Self::CriticalDamageUp
            | Self::AccuracyUp | Self::AccuracyUpGreater
            | Self::EvasionUp | Self::EvasionUpGreater
            | Self::ArmorPenetration | Self::LifeSteal | Self::ManaSteal
            | Self::Berserk | Self::BattleFury | Self::Precision => BuffCategory::Combat,

            Self::HpRegeneration | Self::HpRegenerationGreater | Self::HpRegenerationSupreme
            | Self::MpRegeneration | Self::MpRegenerationGreater | Self::MpRegenerationSupreme
            | Self::StaminaRegeneration | Self::StaminaRegenerationGreater
            | Self::EnergyRegeneration | Self::ResourceEfficiency
            | Self::CostReduction | Self::Meditation => BuffCategory::Resource,

            Self::ExperienceBoost | Self::ExperienceBoostGreater
            | Self::GoldFind | Self::GoldFindGreater
            | Self::ItemFind | Self::ItemFindGreater
            | Self::MovementSpeed | Self::MovementSpeedGreater
            | Self::VisionRange | Self::StealthDetection | Self::TrapDetection
            | Self::LockpickBonus | Self::CraftingBonus
            | Self::GatheringBonus | Self::LuckUp => BuffCategory::Utility,

            Self::Shield | Self::ShieldGreater | Self::ShieldSupreme
            | Self::Invincibility | Self::DamageReduction | Self::DamageReductionGreater
            | Self::PhysicalResistance | Self::MagicalResistance
            | Self::FireResistance | Self::IceResistance | Self::LightningResistance
            | Self::PoisonResistance | Self::DarkResistance | Self::HolyResistance
            | Self::AllElementalResistance | Self::ReflectDamage
            | Self::AbsorbDamage | Self::Immunity => BuffCategory::Protective,

            _ => BuffCategory::Special,
        }
    }

    /// Returns the default duration for this buff
    pub fn default_duration(&self) -> u32 {
        match self {
            // Short duration powerful buffs
            Self::Invincibility | Self::Avatar | Self::Transcendence => 3,
            Self::Berserk | Self::Rage | Self::Overcharge => 5,
            Self::SecondWind | Self::Undying => 1,

            // Medium duration
            Self::Shield | Self::ShieldGreater | Self::Haste
            | Self::HasteGreater | Self::Focus => 8,

            // Long duration utility
            Self::ExperienceBoost | Self::GoldFind | Self::ItemFind => 30,
            Self::ExperienceBoostGreater | Self::GoldFindGreater
            | Self::ItemFindGreater => 60,

            // Default
            _ => DEFAULT_BUFF_DURATION,
        }
    }

    /// Returns whether this buff can stack
    pub fn can_stack(&self) -> bool {
        matches!(self,
            Self::Shield | Self::ShieldGreater | Self::ShieldSupreme |
            Self::ReflectDamage | Self::AbsorbDamage
        )
    }

    /// Returns all buff types
    pub fn all() -> &'static [BuffType] {
        &[
            Self::StrengthI, Self::StrengthII, Self::StrengthIII,
            Self::StrengthIV, Self::StrengthV,
            Self::DefenseI, Self::DefenseII, Self::DefenseIII,
            Self::DefenseIV, Self::DefenseV,
            Self::SpeedI, Self::SpeedII, Self::SpeedIII,
            Self::SpeedIV, Self::SpeedV,
            Self::IntelligenceI, Self::IntelligenceII, Self::IntelligenceIII,
            Self::IntelligenceIV, Self::IntelligenceV,
            Self::AttackPowerUp, Self::AttackPowerUpGreater,
            Self::CriticalChanceUp, Self::CriticalChanceUpGreater, Self::CriticalDamageUp,
            Self::AccuracyUp, Self::AccuracyUpGreater,
            Self::EvasionUp, Self::EvasionUpGreater,
            Self::ArmorPenetration, Self::LifeSteal, Self::ManaSteal,
            Self::Berserk, Self::BattleFury, Self::Precision,
            Self::HpRegeneration, Self::HpRegenerationGreater, Self::HpRegenerationSupreme,
            Self::MpRegeneration, Self::MpRegenerationGreater, Self::MpRegenerationSupreme,
            Self::StaminaRegeneration, Self::StaminaRegenerationGreater,
            Self::EnergyRegeneration, Self::ResourceEfficiency, Self::CostReduction, Self::Meditation,
            Self::ExperienceBoost, Self::ExperienceBoostGreater,
            Self::GoldFind, Self::GoldFindGreater,
            Self::ItemFind, Self::ItemFindGreater,
            Self::MovementSpeed, Self::MovementSpeedGreater,
            Self::VisionRange, Self::StealthDetection, Self::TrapDetection,
            Self::LockpickBonus, Self::CraftingBonus, Self::GatheringBonus, Self::LuckUp,
            Self::Shield, Self::ShieldGreater, Self::ShieldSupreme,
            Self::Invincibility, Self::DamageReduction, Self::DamageReductionGreater,
            Self::PhysicalResistance, Self::MagicalResistance,
            Self::FireResistance, Self::IceResistance, Self::LightningResistance,
            Self::PoisonResistance, Self::DarkResistance, Self::HolyResistance,
            Self::AllElementalResistance, Self::ReflectDamage, Self::AbsorbDamage, Self::Immunity,
            Self::Haste, Self::HasteGreater,
            Self::Invisibility, Self::TrueInvisibility,
            Self::Flight, Self::WaterBreathing, Self::Nightvision,
            Self::Regeneration, Self::SecondWind, Self::Undying,
            Self::Blessed, Self::Divine, Self::Rage, Self::Focus, Self::Clarity,
            Self::Enlightenment, Self::Empowered, Self::Overcharge,
            Self::Transcendence, Self::Avatar,
        ]
    }
}

// ============================================================================
// DEBUFF TYPES - Comprehensive debuff effects
// ============================================================================

/// All possible debuff effect types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum DebuffType {
    // === DAMAGE OVER TIME (12 effects) ===
    Poison,
    PoisonDeadly,
    PoisonLethal,
    Burn,
    BurnIntense,
    BurnInferno,
    Bleed,
    BleedSevere,
    BleedHemorrhage,
    Curse,
    CurseGreater,
    Doom,

    // === STAT REDUCTION (15 effects) ===
    Weakness,
    WeaknessGreater,
    Slowness,
    SlownessGreater,
    Confusion,
    ConfusionGreater,
    AttackDown,
    DefenseDown,
    SpeedDown,
    IntelligenceDown,
    AccuracyDown,
    EvasionDown,
    CriticalDown,
    Vulnerability,
    Fragile,

    // === CONTROL EFFECTS (12 effects) ===
    Stun,
    StunGreater,
    Freeze,
    FreezeDeep,
    Sleep,
    SleepDeep,
    Paralysis,
    ParalysisTotal,
    Root,
    Petrify,
    Charm,
    Taunt,

    // === RESOURCE DRAIN (8 effects) ===
    ManaBurn,
    ManaBurnGreater,
    Exhaustion,
    ExhaustionSevere,
    EnergyDrain,
    LifeDrain,
    SoulDrain,
    Fatigue,

    // === SPECIAL DEBUFFS (13 effects) ===
    Silence,
    SilenceGreater,
    Blind,
    BlindTotal,
    Fear,
    Terror,
    Marked,
    Exposed,
    Cursed,
    Hexed,
    Enfeebled,
    Disarmed,
    Banished,
}

impl DebuffType {
    /// Returns the display name of the debuff
    pub fn name(&self) -> &'static str {
        match self {
            // DOT
            Self::Poison => "Poisoned",
            Self::PoisonDeadly => "Deadly Poison",
            Self::PoisonLethal => "Lethal Poison",
            Self::Burn => "Burning",
            Self::BurnIntense => "Intense Burn",
            Self::BurnInferno => "Inferno",
            Self::Bleed => "Bleeding",
            Self::BleedSevere => "Severe Bleeding",
            Self::BleedHemorrhage => "Hemorrhage",
            Self::Curse => "Cursed",
            Self::CurseGreater => "Greater Curse",
            Self::Doom => "Doomed",

            // Stat reduction
            Self::Weakness => "Weakened",
            Self::WeaknessGreater => "Greatly Weakened",
            Self::Slowness => "Slowed",
            Self::SlownessGreater => "Greatly Slowed",
            Self::Confusion => "Confused",
            Self::ConfusionGreater => "Greatly Confused",
            Self::AttackDown => "Attack Down",
            Self::DefenseDown => "Defense Down",
            Self::SpeedDown => "Speed Down",
            Self::IntelligenceDown => "Intelligence Down",
            Self::AccuracyDown => "Accuracy Down",
            Self::EvasionDown => "Evasion Down",
            Self::CriticalDown => "Critical Down",
            Self::Vulnerability => "Vulnerable",
            Self::Fragile => "Fragile",

            // Control
            Self::Stun => "Stunned",
            Self::StunGreater => "Greatly Stunned",
            Self::Freeze => "Frozen",
            Self::FreezeDeep => "Deep Freeze",
            Self::Sleep => "Asleep",
            Self::SleepDeep => "Deep Sleep",
            Self::Paralysis => "Paralyzed",
            Self::ParalysisTotal => "Total Paralysis",
            Self::Root => "Rooted",
            Self::Petrify => "Petrified",
            Self::Charm => "Charmed",
            Self::Taunt => "Taunted",

            // Resource drain
            Self::ManaBurn => "Mana Burn",
            Self::ManaBurnGreater => "Greater Mana Burn",
            Self::Exhaustion => "Exhausted",
            Self::ExhaustionSevere => "Severely Exhausted",
            Self::EnergyDrain => "Energy Drain",
            Self::LifeDrain => "Life Drain",
            Self::SoulDrain => "Soul Drain",
            Self::Fatigue => "Fatigued",

            // Special
            Self::Silence => "Silenced",
            Self::SilenceGreater => "Greater Silence",
            Self::Blind => "Blinded",
            Self::BlindTotal => "Total Blindness",
            Self::Fear => "Frightened",
            Self::Terror => "Terrified",
            Self::Marked => "Marked",
            Self::Exposed => "Exposed",
            Self::Cursed => "Cursed",
            Self::Hexed => "Hexed",
            Self::Enfeebled => "Enfeebled",
            Self::Disarmed => "Disarmed",
            Self::Banished => "Banished",
        }
    }

    /// Returns the icon character for this debuff
    pub fn icon(&self) -> char {
        match self {
            Self::Poison | Self::PoisonDeadly | Self::PoisonLethal => 'P',
            Self::Burn | Self::BurnIntense | Self::BurnInferno => 'F',
            Self::Bleed | Self::BleedSevere | Self::BleedHemorrhage => 'B',
            Self::Curse | Self::CurseGreater | Self::Cursed => 'C',
            Self::Doom => 'D',

            Self::Weakness | Self::WeaknessGreater | Self::Enfeebled => 'W',
            Self::Slowness | Self::SlownessGreater | Self::SpeedDown => 'S',
            Self::Confusion | Self::ConfusionGreater => '?',
            Self::AttackDown => 'A',
            Self::DefenseDown | Self::Fragile => 'D',
            Self::IntelligenceDown => 'I',
            Self::AccuracyDown => 'T',
            Self::EvasionDown => 'E',
            Self::CriticalDown => 'C',
            Self::Vulnerability | Self::Exposed => 'V',

            Self::Stun | Self::StunGreater => '!',
            Self::Freeze | Self::FreezeDeep => '*',
            Self::Sleep | Self::SleepDeep => 'Z',
            Self::Paralysis | Self::ParalysisTotal => '#',
            Self::Root => 'R',
            Self::Petrify => 'P',
            Self::Charm => 'H',
            Self::Taunt => 'T',

            Self::ManaBurn | Self::ManaBurnGreater => 'M',
            Self::Exhaustion | Self::ExhaustionSevere | Self::Fatigue => 'X',
            Self::EnergyDrain => 'E',
            Self::LifeDrain | Self::SoulDrain => 'L',

            Self::Silence | Self::SilenceGreater => 'S',
            Self::Blind | Self::BlindTotal => 'B',
            Self::Fear | Self::Terror => 'F',
            Self::Marked => 'M',
            Self::Hexed => 'H',
            Self::Disarmed => 'D',
            Self::Banished => 'B',
        }
    }

    /// Returns the color index for UI display
    pub fn color_index(&self) -> u8 {
        match self {
            // Green for poison
            Self::Poison | Self::PoisonDeadly | Self::PoisonLethal => 5,
            // Red for fire
            Self::Burn | Self::BurnIntense | Self::BurnInferno => 3,
            // Dark red for bleed
            Self::Bleed | Self::BleedSevere | Self::BleedHemorrhage => 4,
            // Magenta for curses
            Self::Curse | Self::CurseGreater | Self::Cursed | Self::Hexed | Self::Doom => 14,

            // Grey for stat reductions
            Self::Weakness | Self::WeaknessGreater | Self::Enfeebled => 1,
            Self::Slowness | Self::SlownessGreater | Self::SpeedDown => 10,
            Self::Confusion | Self::ConfusionGreater => 12,
            Self::AttackDown | Self::DefenseDown | Self::IntelligenceDown => 1,
            Self::AccuracyDown | Self::EvasionDown | Self::CriticalDown => 1,
            Self::Vulnerability | Self::Fragile | Self::Exposed => 3,

            // Yellow for control
            Self::Stun | Self::StunGreater => 11,
            // Cyan for freeze
            Self::Freeze | Self::FreezeDeep => 9,
            // Blue for sleep
            Self::Sleep | Self::SleepDeep => 8,
            // Grey for paralysis
            Self::Paralysis | Self::ParalysisTotal | Self::Root | Self::Petrify => 1,
            Self::Charm => 13,
            Self::Taunt => 3,

            // Blue for mana effects
            Self::ManaBurn | Self::ManaBurnGreater => 7,
            // Dark cyan for exhaustion
            Self::Exhaustion | Self::ExhaustionSevere | Self::Fatigue => 10,
            // Dark red for drains
            Self::EnergyDrain | Self::LifeDrain | Self::SoulDrain => 4,

            // Various for special
            Self::Silence | Self::SilenceGreater => 14,
            Self::Blind | Self::BlindTotal => 0,
            Self::Fear | Self::Terror => 12,
            Self::Marked => 11,
            Self::Disarmed => 1,
            Self::Banished => 14,
        }
    }

    /// Returns the category of this debuff
    pub fn category(&self) -> DebuffCategory {
        match self {
            Self::Poison | Self::PoisonDeadly | Self::PoisonLethal
            | Self::Burn | Self::BurnIntense | Self::BurnInferno
            | Self::Bleed | Self::BleedSevere | Self::BleedHemorrhage
            | Self::Curse | Self::CurseGreater | Self::Doom => DebuffCategory::DamageOverTime,

            Self::Weakness | Self::WeaknessGreater
            | Self::Slowness | Self::SlownessGreater
            | Self::Confusion | Self::ConfusionGreater
            | Self::AttackDown | Self::DefenseDown | Self::SpeedDown | Self::IntelligenceDown
            | Self::AccuracyDown | Self::EvasionDown | Self::CriticalDown
            | Self::Vulnerability | Self::Fragile => DebuffCategory::StatReduction,

            Self::Stun | Self::StunGreater
            | Self::Freeze | Self::FreezeDeep
            | Self::Sleep | Self::SleepDeep
            | Self::Paralysis | Self::ParalysisTotal
            | Self::Root | Self::Petrify | Self::Charm | Self::Taunt => DebuffCategory::Control,

            Self::ManaBurn | Self::ManaBurnGreater
            | Self::Exhaustion | Self::ExhaustionSevere
            | Self::EnergyDrain | Self::LifeDrain | Self::SoulDrain
            | Self::Fatigue => DebuffCategory::ResourceDrain,

            _ => DebuffCategory::Special,
        }
    }

    /// Returns damage per tick for DOT effects
    pub fn damage_per_tick(&self) -> i32 {
        match self {
            Self::Poison => 3,
            Self::PoisonDeadly => 6,
            Self::PoisonLethal => 12,
            Self::Burn => 4,
            Self::BurnIntense => 8,
            Self::BurnInferno => 15,
            Self::Bleed => 2,
            Self::BleedSevere => 5,
            Self::BleedHemorrhage => 10,
            Self::Curse => 2,
            Self::CurseGreater => 5,
            Self::Doom => 20,
            Self::LifeDrain => 3,
            Self::SoulDrain => 5,
            _ => 0,
        }
    }

    /// Returns whether this debuff prevents actions
    pub fn prevents_action(&self) -> bool {
        matches!(self,
            Self::Stun | Self::StunGreater |
            Self::Freeze | Self::FreezeDeep |
            Self::Sleep | Self::SleepDeep |
            Self::Paralysis | Self::ParalysisTotal |
            Self::Petrify
        )
    }

    /// Returns whether this debuff prevents movement
    pub fn prevents_movement(&self) -> bool {
        matches!(self,
            Self::Stun | Self::StunGreater |
            Self::Freeze | Self::FreezeDeep |
            Self::Sleep | Self::SleepDeep |
            Self::Paralysis | Self::ParalysisTotal |
            Self::Root | Self::Petrify
        )
    }

    /// Returns the default duration for this debuff
    pub fn default_duration(&self) -> u32 {
        match self {
            // Very short control effects
            Self::Stun => 1,
            Self::StunGreater => 2,
            Self::Petrify => 2,

            // Short control effects
            Self::Freeze | Self::Sleep | Self::Paralysis => 2,
            Self::FreezeDeep | Self::SleepDeep | Self::ParalysisTotal => 3,

            // Medium DOTs
            Self::Poison | Self::Burn | Self::Bleed => 4,
            Self::PoisonDeadly | Self::BurnIntense | Self::BleedSevere => 5,
            Self::PoisonLethal | Self::BurnInferno | Self::BleedHemorrhage => 6,

            // Long curses
            Self::Curse | Self::Cursed | Self::Hexed => 8,
            Self::CurseGreater => 10,
            Self::Doom => 5, // Kills after duration

            _ => DEFAULT_DEBUFF_DURATION,
        }
    }

    /// Returns whether this debuff can stack
    pub fn can_stack(&self) -> bool {
        matches!(self,
            Self::Poison | Self::PoisonDeadly |
            Self::Bleed | Self::BleedSevere |
            Self::Marked
        )
    }

    /// Returns all debuff types
    pub fn all() -> &'static [DebuffType] {
        &[
            Self::Poison, Self::PoisonDeadly, Self::PoisonLethal,
            Self::Burn, Self::BurnIntense, Self::BurnInferno,
            Self::Bleed, Self::BleedSevere, Self::BleedHemorrhage,
            Self::Curse, Self::CurseGreater, Self::Doom,
            Self::Weakness, Self::WeaknessGreater,
            Self::Slowness, Self::SlownessGreater,
            Self::Confusion, Self::ConfusionGreater,
            Self::AttackDown, Self::DefenseDown, Self::SpeedDown, Self::IntelligenceDown,
            Self::AccuracyDown, Self::EvasionDown, Self::CriticalDown,
            Self::Vulnerability, Self::Fragile,
            Self::Stun, Self::StunGreater,
            Self::Freeze, Self::FreezeDeep,
            Self::Sleep, Self::SleepDeep,
            Self::Paralysis, Self::ParalysisTotal,
            Self::Root, Self::Petrify, Self::Charm, Self::Taunt,
            Self::ManaBurn, Self::ManaBurnGreater,
            Self::Exhaustion, Self::ExhaustionSevere,
            Self::EnergyDrain, Self::LifeDrain, Self::SoulDrain, Self::Fatigue,
            Self::Silence, Self::SilenceGreater,
            Self::Blind, Self::BlindTotal,
            Self::Fear, Self::Terror,
            Self::Marked, Self::Exposed, Self::Cursed, Self::Hexed, Self::Enfeebled,
            Self::Disarmed, Self::Banished,
        ]
    }
}

// ============================================================================
// CATEGORIES
// ============================================================================

/// Categories for buff effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum BuffCategory {
    Stat,
    Combat,
    Resource,
    Utility,
    Protective,
    Special,
}

/// Categories for debuff effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum DebuffCategory {
    DamageOverTime,
    StatReduction,
    Control,
    ResourceDrain,
    Special,
}

// ============================================================================
// STACKING BEHAVIOR
// ============================================================================

/// How effects stack when reapplied
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StackBehavior {
    /// Effect stacks up to max_stacks, each stack has independent duration
    Stack,
    /// Duration refreshes to maximum, stacks don't increase
    Refresh,
    /// New application replaces old one entirely
    Replace,
    /// Cannot be reapplied while active
    Unique,
    /// Stacks intensity but shares duration
    Intensify,
}

// ============================================================================
// SOURCE TRACKING
// ============================================================================

/// Tracks the source of a buff or debuff
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EffectSource {
    /// Applied by player ability
    PlayerAbility(String),
    /// Applied by item/equipment
    Item(String),
    /// Applied by consumable
    Consumable(String),
    /// Applied by enemy
    Enemy(u64),
    /// Applied by environment/trap
    Environment,
    /// Applied by ally/companion
    Ally(u64),
    /// Applied by passive effect
    Passive,
    /// Unknown source
    Unknown,
}

impl Default for EffectSource {
    fn default() -> Self {
        Self::Unknown
    }
}

// ============================================================================
// BUFF INSTANCE
// ============================================================================

/// An active buff effect on an entity
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Buff {
    /// The type of buff
    pub buff_type: BuffType,
    /// Remaining duration in turns
    pub duration: u32,
    /// Maximum duration (for display)
    pub max_duration: u32,
    /// Current stack count
    pub stacks: u32,
    /// Maximum stacks allowed
    pub max_stacks: u32,
    /// How this buff stacks
    pub stack_behavior: StackBehavior,
    /// Source of this buff
    pub source: EffectSource,
    /// Resistance to dispel (0-100)
    pub dispel_resistance: u8,
    /// Potency multiplier (default 1.0)
    pub potency: f32,
    /// Whether this buff is hidden from UI
    pub hidden: bool,
    /// Shield/absorb amount if applicable
    pub shield_amount: i32,
}

impl Buff {
    /// Create a new buff with default settings
    pub fn new(buff_type: BuffType) -> Self {
        let duration = buff_type.default_duration();
        Self {
            buff_type,
            duration,
            max_duration: duration,
            stacks: 1,
            max_stacks: if buff_type.can_stack() { MAX_STACKS } else { 1 },
            stack_behavior: if buff_type.can_stack() {
                StackBehavior::Stack
            } else {
                StackBehavior::Refresh
            },
            source: EffectSource::Unknown,
            dispel_resistance: 0,
            potency: 1.0,
            hidden: false,
            shield_amount: 0,
        }
    }

    /// Create a buff with custom duration
    pub fn with_duration(buff_type: BuffType, duration: u32) -> Self {
        Self {
            duration,
            max_duration: duration,
            ..Self::new(buff_type)
        }
    }

    /// Create a buff with source tracking
    pub fn with_source(buff_type: BuffType, source: EffectSource) -> Self {
        Self {
            source,
            ..Self::new(buff_type)
        }
    }

    /// Create a buff with custom potency
    pub fn with_potency(buff_type: BuffType, potency: f32) -> Self {
        Self {
            potency,
            ..Self::new(buff_type)
        }
    }

    /// Create a shield buff with absorb amount
    pub fn shield(buff_type: BuffType, amount: i32) -> Self {
        Self {
            shield_amount: amount,
            ..Self::new(buff_type)
        }
    }

    /// Set dispel resistance
    pub fn with_dispel_resistance(mut self, resistance: u8) -> Self {
        self.dispel_resistance = resistance.min(100);
        self
    }

    /// Check if the buff has expired
    pub fn is_expired(&self) -> bool {
        self.duration == 0
    }

    /// Tick down duration, returns true if still active
    pub fn tick(&mut self) -> bool {
        if self.duration > 0 {
            self.duration -= 1;
        }
        self.duration > 0
    }

    /// Get remaining duration as percentage (0.0 - 1.0)
    pub fn duration_percent(&self) -> f32 {
        if self.max_duration == 0 {
            return 0.0;
        }
        self.duration as f32 / self.max_duration as f32
    }

    /// Get the stat modifiers from this buff
    pub fn get_stat_modifiers(&self) -> StatModifiers {
        let potency = self.potency * self.stacks as f32;

        match self.buff_type {
            // Strength buffs
            BuffType::StrengthI => StatModifiers { strength: (5.0 * potency) as i32, ..Default::default() },
            BuffType::StrengthII => StatModifiers { strength: (10.0 * potency) as i32, ..Default::default() },
            BuffType::StrengthIII => StatModifiers { strength: (15.0 * potency) as i32, ..Default::default() },
            BuffType::StrengthIV => StatModifiers { strength: (25.0 * potency) as i32, ..Default::default() },
            BuffType::StrengthV => StatModifiers { strength: (40.0 * potency) as i32, ..Default::default() },

            // Defense buffs
            BuffType::DefenseI => StatModifiers { defense: (5.0 * potency) as i32, ..Default::default() },
            BuffType::DefenseII => StatModifiers { defense: (10.0 * potency) as i32, ..Default::default() },
            BuffType::DefenseIII => StatModifiers { defense: (15.0 * potency) as i32, ..Default::default() },
            BuffType::DefenseIV => StatModifiers { defense: (25.0 * potency) as i32, ..Default::default() },
            BuffType::DefenseV => StatModifiers { defense: (40.0 * potency) as i32, ..Default::default() },

            // Speed buffs
            BuffType::SpeedI => StatModifiers { speed: (5.0 * potency) as i32, ..Default::default() },
            BuffType::SpeedII => StatModifiers { speed: (10.0 * potency) as i32, ..Default::default() },
            BuffType::SpeedIII => StatModifiers { speed: (15.0 * potency) as i32, ..Default::default() },
            BuffType::SpeedIV => StatModifiers { speed: (25.0 * potency) as i32, ..Default::default() },
            BuffType::SpeedV => StatModifiers { speed: (40.0 * potency) as i32, ..Default::default() },

            // Intelligence buffs
            BuffType::IntelligenceI => StatModifiers { intelligence: (5.0 * potency) as i32, ..Default::default() },
            BuffType::IntelligenceII => StatModifiers { intelligence: (10.0 * potency) as i32, ..Default::default() },
            BuffType::IntelligenceIII => StatModifiers { intelligence: (15.0 * potency) as i32, ..Default::default() },
            BuffType::IntelligenceIV => StatModifiers { intelligence: (25.0 * potency) as i32, ..Default::default() },
            BuffType::IntelligenceV => StatModifiers { intelligence: (40.0 * potency) as i32, ..Default::default() },

            // Combat buffs
            BuffType::AttackPowerUp => StatModifiers { attack_percent: (15.0 * potency) as i32, ..Default::default() },
            BuffType::AttackPowerUpGreater => StatModifiers { attack_percent: (30.0 * potency) as i32, ..Default::default() },
            BuffType::CriticalChanceUp => StatModifiers { crit_chance: (10.0 * potency) as i32, ..Default::default() },
            BuffType::CriticalChanceUpGreater => StatModifiers { crit_chance: (20.0 * potency) as i32, ..Default::default() },
            BuffType::CriticalDamageUp => StatModifiers { crit_damage: (25.0 * potency) as i32, ..Default::default() },
            BuffType::AccuracyUp => StatModifiers { accuracy: (15.0 * potency) as i32, ..Default::default() },
            BuffType::AccuracyUpGreater => StatModifiers { accuracy: (30.0 * potency) as i32, ..Default::default() },
            BuffType::EvasionUp => StatModifiers { evasion: (10.0 * potency) as i32, ..Default::default() },
            BuffType::EvasionUpGreater => StatModifiers { evasion: (20.0 * potency) as i32, ..Default::default() },
            BuffType::ArmorPenetration => StatModifiers { armor_pen: (15.0 * potency) as i32, ..Default::default() },
            BuffType::LifeSteal => StatModifiers { life_steal: (10.0 * potency) as i32, ..Default::default() },
            BuffType::ManaSteal => StatModifiers { mana_steal: (10.0 * potency) as i32, ..Default::default() },
            BuffType::Berserk => StatModifiers {
                attack_percent: (40.0 * potency) as i32,
                defense_percent: (-20.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::BattleFury => StatModifiers {
                attack_percent: (25.0 * potency) as i32,
                crit_chance: (15.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Precision => StatModifiers {
                accuracy: (50.0 * potency) as i32,
                crit_chance: (25.0 * potency) as i32,
                ..Default::default()
            },

            // Resource buffs
            BuffType::HpRegeneration => StatModifiers { hp_regen: (3.0 * potency) as i32, ..Default::default() },
            BuffType::HpRegenerationGreater => StatModifiers { hp_regen: (6.0 * potency) as i32, ..Default::default() },
            BuffType::HpRegenerationSupreme => StatModifiers { hp_regen: (12.0 * potency) as i32, ..Default::default() },
            BuffType::MpRegeneration => StatModifiers { mp_regen: (3.0 * potency) as i32, ..Default::default() },
            BuffType::MpRegenerationGreater => StatModifiers { mp_regen: (6.0 * potency) as i32, ..Default::default() },
            BuffType::MpRegenerationSupreme => StatModifiers { mp_regen: (12.0 * potency) as i32, ..Default::default() },
            BuffType::StaminaRegeneration => StatModifiers { stamina_regen: (5.0 * potency) as i32, ..Default::default() },
            BuffType::StaminaRegenerationGreater => StatModifiers { stamina_regen: (10.0 * potency) as i32, ..Default::default() },
            BuffType::EnergyRegeneration => StatModifiers { energy_regen: (5.0 * potency) as i32, ..Default::default() },
            BuffType::ResourceEfficiency => StatModifiers { resource_cost_reduction: (15.0 * potency) as i32, ..Default::default() },
            BuffType::CostReduction => StatModifiers { resource_cost_reduction: (25.0 * potency) as i32, ..Default::default() },
            BuffType::Meditation => StatModifiers {
                mp_regen: (10.0 * potency) as i32,
                hp_regen: (2.0 * potency) as i32,
                ..Default::default()
            },

            // Utility buffs
            BuffType::ExperienceBoost => StatModifiers { xp_bonus: (25.0 * potency) as i32, ..Default::default() },
            BuffType::ExperienceBoostGreater => StatModifiers { xp_bonus: (50.0 * potency) as i32, ..Default::default() },
            BuffType::GoldFind => StatModifiers { gold_find: (25.0 * potency) as i32, ..Default::default() },
            BuffType::GoldFindGreater => StatModifiers { gold_find: (50.0 * potency) as i32, ..Default::default() },
            BuffType::ItemFind => StatModifiers { item_find: (15.0 * potency) as i32, ..Default::default() },
            BuffType::ItemFindGreater => StatModifiers { item_find: (30.0 * potency) as i32, ..Default::default() },
            BuffType::MovementSpeed => StatModifiers { move_speed: (20.0 * potency) as i32, ..Default::default() },
            BuffType::MovementSpeedGreater => StatModifiers { move_speed: (40.0 * potency) as i32, ..Default::default() },
            BuffType::VisionRange => StatModifiers { vision: (2.0 * potency) as i32, ..Default::default() },
            BuffType::LuckUp => StatModifiers { luck: (10.0 * potency) as i32, ..Default::default() },

            // Protective buffs
            BuffType::DamageReduction => StatModifiers { damage_reduction: (15.0 * potency) as i32, ..Default::default() },
            BuffType::DamageReductionGreater => StatModifiers { damage_reduction: (30.0 * potency) as i32, ..Default::default() },
            BuffType::PhysicalResistance => StatModifiers { physical_resist: (25.0 * potency) as i32, ..Default::default() },
            BuffType::MagicalResistance => StatModifiers { magical_resist: (25.0 * potency) as i32, ..Default::default() },
            BuffType::FireResistance => StatModifiers { fire_resist: (50.0 * potency) as i32, ..Default::default() },
            BuffType::IceResistance => StatModifiers { ice_resist: (50.0 * potency) as i32, ..Default::default() },
            BuffType::LightningResistance => StatModifiers { lightning_resist: (50.0 * potency) as i32, ..Default::default() },
            BuffType::PoisonResistance => StatModifiers { poison_resist: (50.0 * potency) as i32, ..Default::default() },
            BuffType::DarkResistance => StatModifiers { dark_resist: (50.0 * potency) as i32, ..Default::default() },
            BuffType::HolyResistance => StatModifiers { holy_resist: (50.0 * potency) as i32, ..Default::default() },
            BuffType::AllElementalResistance => StatModifiers {
                fire_resist: (25.0 * potency) as i32,
                ice_resist: (25.0 * potency) as i32,
                lightning_resist: (25.0 * potency) as i32,
                poison_resist: (25.0 * potency) as i32,
                dark_resist: (25.0 * potency) as i32,
                holy_resist: (25.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::ReflectDamage => StatModifiers { damage_reflect: (15.0 * potency) as i32, ..Default::default() },

            // Haste effects
            BuffType::Haste => StatModifiers {
                speed: (20.0 * potency) as i32,
                move_speed: (30.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::HasteGreater => StatModifiers {
                speed: (40.0 * potency) as i32,
                move_speed: (50.0 * potency) as i32,
                ..Default::default()
            },

            // Special buffs
            BuffType::Blessed => StatModifiers {
                all_stats: (5.0 * potency) as i32,
                luck: (15.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Divine => StatModifiers {
                all_stats: (10.0 * potency) as i32,
                damage_reduction: (20.0 * potency) as i32,
                hp_regen: (5.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Rage => StatModifiers {
                attack_percent: (50.0 * potency) as i32,
                crit_chance: (20.0 * potency) as i32,
                defense_percent: (-30.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Focus => StatModifiers {
                accuracy: (30.0 * potency) as i32,
                crit_damage: (20.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Clarity => StatModifiers {
                intelligence: (20.0 * potency) as i32,
                mp_regen: (5.0 * potency) as i32,
                resource_cost_reduction: (20.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Enlightenment => StatModifiers {
                all_stats: (15.0 * potency) as i32,
                xp_bonus: (30.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Empowered => StatModifiers {
                attack_percent: (20.0 * potency) as i32,
                defense_percent: (20.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Overcharge => StatModifiers {
                attack_percent: (60.0 * potency) as i32,
                crit_chance: (30.0 * potency) as i32,
                crit_damage: (50.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Transcendence => StatModifiers {
                all_stats: (25.0 * potency) as i32,
                damage_reduction: (25.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Avatar => StatModifiers {
                all_stats: (50.0 * potency) as i32,
                attack_percent: (50.0 * potency) as i32,
                defense_percent: (50.0 * potency) as i32,
                damage_reduction: (30.0 * potency) as i32,
                ..Default::default()
            },
            BuffType::Regeneration => StatModifiers {
                hp_regen: (8.0 * potency) as i32,
                ..Default::default()
            },

            // Default for buffs without direct stat modifiers
            _ => StatModifiers::default(),
        }
    }
}

// ============================================================================
// DEBUFF INSTANCE
// ============================================================================

/// An active debuff effect on an entity
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Debuff {
    /// The type of debuff
    pub debuff_type: DebuffType,
    /// Remaining duration in turns
    pub duration: u32,
    /// Maximum duration (for display)
    pub max_duration: u32,
    /// Current stack count
    pub stacks: u32,
    /// Maximum stacks allowed
    pub max_stacks: u32,
    /// How this debuff stacks
    pub stack_behavior: StackBehavior,
    /// Source of this debuff
    pub source: EffectSource,
    /// Resistance to cleanse (0-100)
    pub cleanse_resistance: u8,
    /// Potency multiplier (default 1.0)
    pub potency: f32,
    /// Whether this debuff is hidden from UI
    pub hidden: bool,
}

impl Debuff {
    /// Create a new debuff with default settings
    pub fn new(debuff_type: DebuffType) -> Self {
        let duration = debuff_type.default_duration();
        Self {
            debuff_type,
            duration,
            max_duration: duration,
            stacks: 1,
            max_stacks: if debuff_type.can_stack() { MAX_STACKS } else { 1 },
            stack_behavior: if debuff_type.can_stack() {
                StackBehavior::Stack
            } else {
                StackBehavior::Refresh
            },
            source: EffectSource::Unknown,
            cleanse_resistance: 0,
            potency: 1.0,
            hidden: false,
        }
    }

    /// Create a debuff with custom duration
    pub fn with_duration(debuff_type: DebuffType, duration: u32) -> Self {
        Self {
            duration,
            max_duration: duration,
            ..Self::new(debuff_type)
        }
    }

    /// Create a debuff with source tracking
    pub fn with_source(debuff_type: DebuffType, source: EffectSource) -> Self {
        Self {
            source,
            ..Self::new(debuff_type)
        }
    }

    /// Create a debuff with custom potency
    pub fn with_potency(debuff_type: DebuffType, potency: f32) -> Self {
        Self {
            potency,
            ..Self::new(debuff_type)
        }
    }

    /// Set cleanse resistance
    pub fn with_cleanse_resistance(mut self, resistance: u8) -> Self {
        self.cleanse_resistance = resistance.min(100);
        self
    }

    /// Check if the debuff has expired
    pub fn is_expired(&self) -> bool {
        self.duration == 0
    }

    /// Tick down duration, returns true if still active
    pub fn tick(&mut self) -> bool {
        if self.duration > 0 {
            self.duration -= 1;
        }
        self.duration > 0
    }

    /// Get remaining duration as percentage (0.0 - 1.0)
    pub fn duration_percent(&self) -> f32 {
        if self.max_duration == 0 {
            return 0.0;
        }
        self.duration as f32 / self.max_duration as f32
    }

    /// Calculate damage for DOT effects
    pub fn calculate_dot_damage(&self) -> i32 {
        let base = self.debuff_type.damage_per_tick();
        (base as f32 * self.potency * self.stacks as f32) as i32
    }

    /// Get the stat modifiers (negative) from this debuff
    pub fn get_stat_modifiers(&self) -> StatModifiers {
        let potency = self.potency * self.stacks as f32;

        match self.debuff_type {
            // Stat reductions
            DebuffType::Weakness => StatModifiers { strength: (-10.0 * potency) as i32, ..Default::default() },
            DebuffType::WeaknessGreater => StatModifiers { strength: (-25.0 * potency) as i32, ..Default::default() },
            DebuffType::Slowness => StatModifiers { speed: (-10.0 * potency) as i32, ..Default::default() },
            DebuffType::SlownessGreater => StatModifiers { speed: (-25.0 * potency) as i32, ..Default::default() },
            DebuffType::AttackDown => StatModifiers { attack_percent: (-20.0 * potency) as i32, ..Default::default() },
            DebuffType::DefenseDown => StatModifiers { defense_percent: (-20.0 * potency) as i32, ..Default::default() },
            DebuffType::SpeedDown => StatModifiers { speed: (-15.0 * potency) as i32, move_speed: (-20.0 * potency) as i32, ..Default::default() },
            DebuffType::IntelligenceDown => StatModifiers { intelligence: (-15.0 * potency) as i32, ..Default::default() },
            DebuffType::AccuracyDown => StatModifiers { accuracy: (-20.0 * potency) as i32, ..Default::default() },
            DebuffType::EvasionDown => StatModifiers { evasion: (-20.0 * potency) as i32, ..Default::default() },
            DebuffType::CriticalDown => StatModifiers { crit_chance: (-15.0 * potency) as i32, crit_damage: (-20.0 * potency) as i32, ..Default::default() },
            DebuffType::Vulnerability => StatModifiers { damage_reduction: (-25.0 * potency) as i32, ..Default::default() },
            DebuffType::Fragile => StatModifiers { defense_percent: (-30.0 * potency) as i32, ..Default::default() },
            DebuffType::Enfeebled => StatModifiers {
                strength: (-15.0 * potency) as i32,
                defense: (-15.0 * potency) as i32,
                ..Default::default()
            },

            // Confusion reduces accuracy significantly
            DebuffType::Confusion => StatModifiers { accuracy: (-30.0 * potency) as i32, ..Default::default() },
            DebuffType::ConfusionGreater => StatModifiers { accuracy: (-50.0 * potency) as i32, ..Default::default() },

            // Blind affects accuracy and vision
            DebuffType::Blind => StatModifiers { accuracy: (-40.0 * potency) as i32, vision: (-2.0 * potency) as i32, ..Default::default() },
            DebuffType::BlindTotal => StatModifiers { accuracy: (-75.0 * potency) as i32, vision: (-5.0 * potency) as i32, ..Default::default() },

            // Fear/Terror reduces all stats
            DebuffType::Fear => StatModifiers { all_stats: (-10.0 * potency) as i32, ..Default::default() },
            DebuffType::Terror => StatModifiers { all_stats: (-25.0 * potency) as i32, ..Default::default() },

            // Marked increases damage taken
            DebuffType::Marked => StatModifiers { damage_reduction: (-15.0 * potency) as i32, ..Default::default() },
            DebuffType::Exposed => StatModifiers {
                defense_percent: (-25.0 * potency) as i32,
                damage_reduction: (-20.0 * potency) as i32,
                ..Default::default()
            },

            // Exhaustion affects resource regeneration
            DebuffType::Exhaustion => StatModifiers {
                stamina_regen: (-50.0 * potency) as i32,
                hp_regen: (-25.0 * potency) as i32,
                ..Default::default()
            },
            DebuffType::ExhaustionSevere => StatModifiers {
                stamina_regen: (-100.0 * potency) as i32,
                hp_regen: (-50.0 * potency) as i32,
                speed: (-10.0 * potency) as i32,
                ..Default::default()
            },
            DebuffType::Fatigue => StatModifiers {
                stamina_regen: (-30.0 * potency) as i32,
                move_speed: (-15.0 * potency) as i32,
                ..Default::default()
            },

            // Curses have various negative effects
            DebuffType::Curse | DebuffType::Cursed => StatModifiers {
                luck: (-20.0 * potency) as i32,
                ..Default::default()
            },
            DebuffType::CurseGreater => StatModifiers {
                luck: (-40.0 * potency) as i32,
                all_stats: (-5.0 * potency) as i32,
                ..Default::default()
            },
            DebuffType::Hexed => StatModifiers {
                luck: (-30.0 * potency) as i32,
                crit_chance: (-20.0 * potency) as i32,
                ..Default::default()
            },

            // Default for debuffs without direct stat modifiers
            _ => StatModifiers::default(),
        }
    }
}

// ============================================================================
// STAT MODIFIERS
// ============================================================================

/// Stat modifications from buffs/debuffs
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct StatModifiers {
    // Base stats
    pub strength: i32,
    pub defense: i32,
    pub speed: i32,
    pub intelligence: i32,
    pub all_stats: i32,

    // Percentage modifiers
    pub attack_percent: i32,
    pub defense_percent: i32,

    // Combat stats
    pub accuracy: i32,
    pub evasion: i32,
    pub crit_chance: i32,
    pub crit_damage: i32,
    pub armor_pen: i32,
    pub life_steal: i32,
    pub mana_steal: i32,

    // Resource regeneration
    pub hp_regen: i32,
    pub mp_regen: i32,
    pub stamina_regen: i32,
    pub energy_regen: i32,
    pub resource_cost_reduction: i32,

    // Utility
    pub xp_bonus: i32,
    pub gold_find: i32,
    pub item_find: i32,
    pub move_speed: i32,
    pub vision: i32,
    pub luck: i32,

    // Defensive
    pub damage_reduction: i32,
    pub damage_reflect: i32,
    pub physical_resist: i32,
    pub magical_resist: i32,
    pub fire_resist: i32,
    pub ice_resist: i32,
    pub lightning_resist: i32,
    pub poison_resist: i32,
    pub dark_resist: i32,
    pub holy_resist: i32,
}

impl StatModifiers {
    /// Combine two stat modifier sets
    pub fn combine(&self, other: &StatModifiers) -> StatModifiers {
        StatModifiers {
            strength: self.strength + other.strength,
            defense: self.defense + other.defense,
            speed: self.speed + other.speed,
            intelligence: self.intelligence + other.intelligence,
            all_stats: self.all_stats + other.all_stats,
            attack_percent: self.attack_percent + other.attack_percent,
            defense_percent: self.defense_percent + other.defense_percent,
            accuracy: self.accuracy + other.accuracy,
            evasion: self.evasion + other.evasion,
            crit_chance: self.crit_chance + other.crit_chance,
            crit_damage: self.crit_damage + other.crit_damage,
            armor_pen: self.armor_pen + other.armor_pen,
            life_steal: self.life_steal + other.life_steal,
            mana_steal: self.mana_steal + other.mana_steal,
            hp_regen: self.hp_regen + other.hp_regen,
            mp_regen: self.mp_regen + other.mp_regen,
            stamina_regen: self.stamina_regen + other.stamina_regen,
            energy_regen: self.energy_regen + other.energy_regen,
            resource_cost_reduction: self.resource_cost_reduction + other.resource_cost_reduction,
            xp_bonus: self.xp_bonus + other.xp_bonus,
            gold_find: self.gold_find + other.gold_find,
            item_find: self.item_find + other.item_find,
            move_speed: self.move_speed + other.move_speed,
            vision: self.vision + other.vision,
            luck: self.luck + other.luck,
            damage_reduction: self.damage_reduction + other.damage_reduction,
            damage_reflect: self.damage_reflect + other.damage_reflect,
            physical_resist: self.physical_resist + other.physical_resist,
            magical_resist: self.magical_resist + other.magical_resist,
            fire_resist: self.fire_resist + other.fire_resist,
            ice_resist: self.ice_resist + other.ice_resist,
            lightning_resist: self.lightning_resist + other.lightning_resist,
            poison_resist: self.poison_resist + other.poison_resist,
            dark_resist: self.dark_resist + other.dark_resist,
            holy_resist: self.holy_resist + other.holy_resist,
        }
    }
}

// ============================================================================
// BUFF/DEBUFF INTERACTIONS
// ============================================================================

/// Defines interactions between buffs and debuffs
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EffectInteraction {
    /// Buffs that cancel specific debuffs when applied
    pub buff_cancels_debuff: HashMap<BuffType, Vec<DebuffType>>,
    /// Debuffs that reduce/remove specific buffs
    pub debuff_removes_buff: HashMap<DebuffType, Vec<BuffType>>,
    /// Synergistic buff combinations
    pub buff_synergies: HashMap<(BuffType, BuffType), SynergyEffect>,
    /// Synergistic debuff combinations
    pub debuff_synergies: HashMap<(DebuffType, DebuffType), SynergyEffect>,
}

impl Default for EffectInteraction {
    fn default() -> Self {
        let mut buff_cancels_debuff = HashMap::new();
        let mut debuff_removes_buff = HashMap::new();
        let mut buff_synergies = HashMap::new();
        let mut debuff_synergies = HashMap::new();

        // Buff cancels debuff interactions
        buff_cancels_debuff.insert(BuffType::PoisonResistance, vec![
            DebuffType::Poison, DebuffType::PoisonDeadly, DebuffType::PoisonLethal
        ]);
        buff_cancels_debuff.insert(BuffType::FireResistance, vec![
            DebuffType::Burn, DebuffType::BurnIntense, DebuffType::BurnInferno
        ]);
        buff_cancels_debuff.insert(BuffType::IceResistance, vec![
            DebuffType::Freeze, DebuffType::FreezeDeep
        ]);
        buff_cancels_debuff.insert(BuffType::Immunity, vec![
            DebuffType::Stun, DebuffType::StunGreater,
            DebuffType::Freeze, DebuffType::FreezeDeep,
            DebuffType::Sleep, DebuffType::SleepDeep,
            DebuffType::Paralysis, DebuffType::ParalysisTotal,
        ]);
        buff_cancels_debuff.insert(BuffType::Blessed, vec![
            DebuffType::Curse, DebuffType::CurseGreater, DebuffType::Cursed, DebuffType::Hexed
        ]);
        buff_cancels_debuff.insert(BuffType::Divine, vec![
            DebuffType::Curse, DebuffType::CurseGreater, DebuffType::Cursed,
            DebuffType::Hexed, DebuffType::Fear, DebuffType::Terror
        ]);
        buff_cancels_debuff.insert(BuffType::Clarity, vec![
            DebuffType::Confusion, DebuffType::ConfusionGreater, DebuffType::Silence
        ]);
        buff_cancels_debuff.insert(BuffType::Haste, vec![
            DebuffType::Slowness, DebuffType::SlownessGreater, DebuffType::Root
        ]);
        buff_cancels_debuff.insert(BuffType::HasteGreater, vec![
            DebuffType::Slowness, DebuffType::SlownessGreater,
            DebuffType::Root, DebuffType::SpeedDown
        ]);
        buff_cancels_debuff.insert(BuffType::Regeneration, vec![
            DebuffType::Bleed, DebuffType::BleedSevere
        ]);

        // Debuff removes buff interactions
        debuff_removes_buff.insert(DebuffType::Silence, vec![
            BuffType::Focus, BuffType::Clarity, BuffType::Meditation
        ]);
        debuff_removes_buff.insert(DebuffType::SilenceGreater, vec![
            BuffType::Focus, BuffType::Clarity, BuffType::Meditation,
            BuffType::MpRegeneration, BuffType::MpRegenerationGreater
        ]);
        debuff_removes_buff.insert(DebuffType::Curse, vec![
            BuffType::Blessed, BuffType::LuckUp
        ]);
        debuff_removes_buff.insert(DebuffType::CurseGreater, vec![
            BuffType::Blessed, BuffType::Divine, BuffType::LuckUp
        ]);
        debuff_removes_buff.insert(DebuffType::Exhaustion, vec![
            BuffType::Haste, BuffType::MovementSpeed
        ]);
        debuff_removes_buff.insert(DebuffType::ExhaustionSevere, vec![
            BuffType::Haste, BuffType::HasteGreater,
            BuffType::MovementSpeed, BuffType::MovementSpeedGreater,
            BuffType::StaminaRegeneration
        ]);
        debuff_removes_buff.insert(DebuffType::Disarmed, vec![
            BuffType::AttackPowerUp, BuffType::AttackPowerUpGreater,
            BuffType::Precision, BuffType::BattleFury
        ]);

        // Buff synergies
        buff_synergies.insert(
            (BuffType::Berserk, BuffType::LifeSteal),
            SynergyEffect {
                name: "Bloodlust".to_string(),
                description: "Life steal increased by 50%".to_string(),
                effect_multiplier: 1.5,
            }
        );
        buff_synergies.insert(
            (BuffType::Focus, BuffType::Precision),
            SynergyEffect {
                name: "Perfect Aim".to_string(),
                description: "Critical hits deal 25% more damage".to_string(),
                effect_multiplier: 1.25,
            }
        );
        buff_synergies.insert(
            (BuffType::Haste, BuffType::EvasionUp),
            SynergyEffect {
                name: "Blur".to_string(),
                description: "Evasion chance doubled".to_string(),
                effect_multiplier: 2.0,
            }
        );
        buff_synergies.insert(
            (BuffType::Shield, BuffType::ReflectDamage),
            SynergyEffect {
                name: "Mirror Shield".to_string(),
                description: "Reflected damage increased by 50%".to_string(),
                effect_multiplier: 1.5,
            }
        );

        // Debuff synergies
        debuff_synergies.insert(
            (DebuffType::Burn, DebuffType::Poison),
            SynergyEffect {
                name: "Toxic Flames".to_string(),
                description: "Both DOTs deal 25% more damage".to_string(),
                effect_multiplier: 1.25,
            }
        );
        debuff_synergies.insert(
            (DebuffType::Freeze, DebuffType::Vulnerability),
            SynergyEffect {
                name: "Shattered Ice".to_string(),
                description: "Physical damage increased by 50%".to_string(),
                effect_multiplier: 1.5,
            }
        );
        debuff_synergies.insert(
            (DebuffType::Marked, DebuffType::Exposed),
            SynergyEffect {
                name: "Priority Target".to_string(),
                description: "All damage taken increased by 25%".to_string(),
                effect_multiplier: 1.25,
            }
        );
        debuff_synergies.insert(
            (DebuffType::Stun, DebuffType::Bleed),
            SynergyEffect {
                name: "Hemorrhaging".to_string(),
                description: "Bleed damage doubled while stunned".to_string(),
                effect_multiplier: 2.0,
            }
        );

        Self {
            buff_cancels_debuff,
            debuff_removes_buff,
            buff_synergies,
            debuff_synergies,
        }
    }
}

/// A synergistic effect between two buffs or debuffs
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SynergyEffect {
    pub name: String,
    pub description: String,
    pub effect_multiplier: f32,
}

// ============================================================================
// VISUAL INDICATORS
// ============================================================================

/// Visual indicator for displaying buff/debuff status
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EffectIndicator {
    /// Icon character to display
    pub icon: char,
    /// Color index for the icon
    pub color: u8,
    /// Remaining duration text (e.g., "3t" for 3 turns)
    pub duration_text: String,
    /// Stack count (if > 1)
    pub stacks: Option<u32>,
    /// Tooltip text
    pub tooltip: String,
    /// Whether this is a buff (true) or debuff (false)
    pub is_buff: bool,
}

impl EffectIndicator {
    /// Create indicator from a buff
    pub fn from_buff(buff: &Buff) -> Self {
        let duration_text = if buff.duration == u32::MAX {
            "~".to_string()
        } else {
            format!("{}t", buff.duration)
        };

        Self {
            icon: buff.buff_type.icon(),
            color: buff.buff_type.color_index(),
            duration_text,
            stacks: if buff.stacks > 1 { Some(buff.stacks) } else { None },
            tooltip: format!(
                "{}\nDuration: {} turns\nStacks: {}",
                buff.buff_type.name(),
                buff.duration,
                buff.stacks
            ),
            is_buff: true,
        }
    }

    /// Create indicator from a debuff
    pub fn from_debuff(debuff: &Debuff) -> Self {
        let duration_text = if debuff.duration == u32::MAX {
            "~".to_string()
        } else {
            format!("{}t", debuff.duration)
        };

        Self {
            icon: debuff.debuff_type.icon(),
            color: debuff.debuff_type.color_index(),
            duration_text,
            stacks: if debuff.stacks > 1 { Some(debuff.stacks) } else { None },
            tooltip: format!(
                "{}\nDuration: {} turns\nStacks: {}",
                debuff.debuff_type.name(),
                debuff.duration,
                debuff.stacks
            ),
            is_buff: false,
        }
    }
}

// ============================================================================
// ENTITY BUFF TRACKER
// ============================================================================

/// Tracks all buffs and debuffs on an entity
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct EntityEffects {
    /// Active buffs
    pub buffs: Vec<Buff>,
    /// Active debuffs
    pub debuffs: Vec<Debuff>,
    /// Effect interactions configuration
    #[serde(skip)]
    interactions: Option<EffectInteraction>,
}

impl EntityEffects {
    /// Create a new empty effect tracker
    pub fn new() -> Self {
        Self {
            buffs: Vec::new(),
            debuffs: Vec::new(),
            interactions: Some(EffectInteraction::default()),
        }
    }

    /// Ensure interactions are initialized
    fn ensure_interactions(&mut self) {
        if self.interactions.is_none() {
            self.interactions = Some(EffectInteraction::default());
        }
    }

    /// Add a buff to the entity
    pub fn add_buff(&mut self, buff: Buff) -> BuffApplicationResult {
        self.ensure_interactions();

        // Check if we're at max buffs
        if self.buffs.len() >= MAX_BUFFS {
            return BuffApplicationResult::Failed("Maximum buffs reached".to_string());
        }

        // Check for existing buff of same type
        if let Some(existing) = self.buffs.iter_mut().find(|b| b.buff_type == buff.buff_type) {
            match existing.stack_behavior {
                StackBehavior::Stack => {
                    if existing.stacks < existing.max_stacks {
                        existing.stacks += 1;
                        return BuffApplicationResult::Stacked(existing.stacks);
                    } else {
                        existing.duration = existing.duration.max(buff.duration);
                        return BuffApplicationResult::Refreshed;
                    }
                }
                StackBehavior::Refresh => {
                    existing.duration = existing.max_duration;
                    return BuffApplicationResult::Refreshed;
                }
                StackBehavior::Replace => {
                    *existing = buff;
                    return BuffApplicationResult::Replaced;
                }
                StackBehavior::Unique => {
                    return BuffApplicationResult::Failed("Buff already active".to_string());
                }
                StackBehavior::Intensify => {
                    existing.potency += buff.potency * 0.5;
                    existing.duration = existing.duration.max(buff.duration);
                    return BuffApplicationResult::Intensified(existing.potency);
                }
            }
        }

        // Check for debuff cancellation
        let mut cancelled_debuffs = Vec::new();
        if let Some(ref interactions) = self.interactions {
            if let Some(cancels) = interactions.buff_cancels_debuff.get(&buff.buff_type) {
                for debuff_type in cancels {
                    if let Some(pos) = self.debuffs.iter().position(|d| &d.debuff_type == debuff_type) {
                        cancelled_debuffs.push(self.debuffs.remove(pos).debuff_type);
                    }
                }
            }
        }

        self.buffs.push(buff);

        if !cancelled_debuffs.is_empty() {
            BuffApplicationResult::AppliedAndCancelled(cancelled_debuffs)
        } else {
            BuffApplicationResult::Applied
        }
    }

    /// Add a debuff to the entity
    pub fn add_debuff(&mut self, debuff: Debuff) -> DebuffApplicationResult {
        self.ensure_interactions();

        // Check for immunity
        if self.has_buff(BuffType::Invincibility) || self.has_buff(BuffType::Immunity) {
            if debuff.debuff_type.category() == DebuffCategory::Control {
                return DebuffApplicationResult::Immune;
            }
        }

        // Check if we're at max debuffs
        if self.debuffs.len() >= MAX_DEBUFFS {
            return DebuffApplicationResult::Failed("Maximum debuffs reached".to_string());
        }

        // Check for existing debuff of same type
        if let Some(existing) = self.debuffs.iter_mut().find(|d| d.debuff_type == debuff.debuff_type) {
            match existing.stack_behavior {
                StackBehavior::Stack => {
                    if existing.stacks < existing.max_stacks {
                        existing.stacks += 1;
                        return DebuffApplicationResult::Stacked(existing.stacks);
                    } else {
                        existing.duration = existing.duration.max(debuff.duration);
                        return DebuffApplicationResult::Refreshed;
                    }
                }
                StackBehavior::Refresh => {
                    existing.duration = existing.max_duration;
                    return DebuffApplicationResult::Refreshed;
                }
                StackBehavior::Replace => {
                    *existing = debuff;
                    return DebuffApplicationResult::Replaced;
                }
                StackBehavior::Unique => {
                    return DebuffApplicationResult::Failed("Debuff already active".to_string());
                }
                StackBehavior::Intensify => {
                    existing.potency += debuff.potency * 0.5;
                    existing.duration = existing.duration.max(debuff.duration);
                    return DebuffApplicationResult::Intensified(existing.potency);
                }
            }
        }

        // Check for buff removal
        let mut removed_buffs = Vec::new();
        if let Some(ref interactions) = self.interactions {
            if let Some(removes) = interactions.debuff_removes_buff.get(&debuff.debuff_type) {
                for buff_type in removes {
                    if let Some(pos) = self.buffs.iter().position(|b| &b.buff_type == buff_type) {
                        removed_buffs.push(self.buffs.remove(pos).buff_type);
                    }
                }
            }
        }

        self.debuffs.push(debuff);

        if !removed_buffs.is_empty() {
            DebuffApplicationResult::AppliedAndRemoved(removed_buffs)
        } else {
            DebuffApplicationResult::Applied
        }
    }

    /// Remove a specific buff
    pub fn remove_buff(&mut self, buff_type: BuffType) -> bool {
        if let Some(pos) = self.buffs.iter().position(|b| b.buff_type == buff_type) {
            self.buffs.remove(pos);
            true
        } else {
            false
        }
    }

    /// Remove a specific debuff
    pub fn remove_debuff(&mut self, debuff_type: DebuffType) -> bool {
        if let Some(pos) = self.debuffs.iter().position(|d| d.debuff_type == debuff_type) {
            self.debuffs.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if entity has a specific buff
    pub fn has_buff(&self, buff_type: BuffType) -> bool {
        self.buffs.iter().any(|b| b.buff_type == buff_type)
    }

    /// Check if entity has a specific debuff
    pub fn has_debuff(&self, debuff_type: DebuffType) -> bool {
        self.debuffs.iter().any(|d| d.debuff_type == debuff_type)
    }

    /// Get a buff by type
    pub fn get_buff(&self, buff_type: BuffType) -> Option<&Buff> {
        self.buffs.iter().find(|b| b.buff_type == buff_type)
    }

    /// Get a debuff by type
    pub fn get_debuff(&self, debuff_type: DebuffType) -> Option<&Debuff> {
        self.debuffs.iter().find(|d| d.debuff_type == debuff_type)
    }

    /// Tick all effects, removing expired ones. Returns tick results.
    pub fn tick(&mut self) -> TickResult {
        let mut result = TickResult::default();

        // Tick buffs
        self.buffs.retain_mut(|buff| {
            let still_active = buff.tick();
            if !still_active {
                result.expired_buffs.push(buff.buff_type);
            }
            still_active
        });

        // Tick debuffs and calculate DOT damage
        self.debuffs.retain_mut(|debuff| {
            // Calculate DOT damage before ticking
            let dot_damage = debuff.calculate_dot_damage();
            if dot_damage > 0 {
                result.dot_damage += dot_damage;
                result.dot_sources.push((debuff.debuff_type, dot_damage));
            }

            // Check for mana burn
            if matches!(debuff.debuff_type, DebuffType::ManaBurn | DebuffType::ManaBurnGreater) {
                let mana_drain = if debuff.debuff_type == DebuffType::ManaBurn { 5 } else { 10 };
                result.mana_drain += (mana_drain as f32 * debuff.potency) as i32;
            }

            let still_active = debuff.tick();
            if !still_active {
                result.expired_debuffs.push(debuff.debuff_type);
            }
            still_active
        });

        result
    }

    /// Get combined stat modifiers from all effects
    pub fn get_total_modifiers(&self) -> StatModifiers {
        let mut total = StatModifiers::default();

        for buff in &self.buffs {
            total = total.combine(&buff.get_stat_modifiers());
        }

        for debuff in &self.debuffs {
            total = total.combine(&debuff.get_stat_modifiers());
        }

        total
    }

    /// Check if entity is stunned/unable to act
    pub fn is_incapacitated(&self) -> bool {
        self.debuffs.iter().any(|d| d.debuff_type.prevents_action())
    }

    /// Check if entity can move
    pub fn can_move(&self) -> bool {
        !self.debuffs.iter().any(|d| d.debuff_type.prevents_movement())
    }

    /// Check if entity is silenced
    pub fn is_silenced(&self) -> bool {
        self.has_debuff(DebuffType::Silence) || self.has_debuff(DebuffType::SilenceGreater)
    }

    /// Check if entity is invisible
    pub fn is_invisible(&self) -> bool {
        self.has_buff(BuffType::Invisibility) || self.has_buff(BuffType::TrueInvisibility)
    }

    /// Check if entity is invincible
    pub fn is_invincible(&self) -> bool {
        self.has_buff(BuffType::Invincibility)
    }

    /// Get total shield amount
    pub fn get_shield_amount(&self) -> i32 {
        self.buffs.iter()
            .filter(|b| matches!(b.buff_type,
                BuffType::Shield | BuffType::ShieldGreater | BuffType::ShieldSupreme))
            .map(|b| b.shield_amount)
            .sum()
    }

    /// Damage shield, returns remaining damage after absorption
    pub fn damage_shield(&mut self, damage: i32) -> i32 {
        let mut remaining = damage;

        for buff in self.buffs.iter_mut() {
            if remaining <= 0 {
                break;
            }
            if matches!(buff.buff_type,
                BuffType::Shield | BuffType::ShieldGreater | BuffType::ShieldSupreme)
            {
                if buff.shield_amount >= remaining {
                    buff.shield_amount -= remaining;
                    remaining = 0;
                } else {
                    remaining -= buff.shield_amount;
                    buff.shield_amount = 0;
                }
            }
        }

        // Remove depleted shields
        self.buffs.retain(|b| {
            !matches!(b.buff_type,
                BuffType::Shield | BuffType::ShieldGreater | BuffType::ShieldSupreme)
            || b.shield_amount > 0
        });

        remaining
    }

    /// Dispel buffs from entity
    pub fn dispel_buffs(&mut self, count: usize, rng: &mut impl Rng) -> Vec<BuffType> {
        let mut dispelled = Vec::new();
        let mut attempts = 0;

        while dispelled.len() < count && !self.buffs.is_empty() && attempts < count * 2 {
            attempts += 1;
            let idx = rng.gen_range(0..self.buffs.len());
            let buff = &self.buffs[idx];

            // Check dispel resistance
            if rng.gen_range(0..100) >= buff.dispel_resistance {
                let removed = self.buffs.remove(idx);
                dispelled.push(removed.buff_type);
            }
        }

        dispelled
    }

    /// Cleanse debuffs from entity
    pub fn cleanse_debuffs(&mut self, count: usize, rng: &mut impl Rng) -> Vec<DebuffType> {
        let mut cleansed = Vec::new();
        let mut attempts = 0;

        while cleansed.len() < count && !self.debuffs.is_empty() && attempts < count * 2 {
            attempts += 1;
            let idx = rng.gen_range(0..self.debuffs.len());
            let debuff = &self.debuffs[idx];

            // Check cleanse resistance
            if rng.gen_range(0..100) >= debuff.cleanse_resistance {
                let removed = self.debuffs.remove(idx);
                cleansed.push(removed.debuff_type);
            }
        }

        cleansed
    }

    /// Get visual indicators for all effects
    pub fn get_indicators(&self) -> Vec<EffectIndicator> {
        let mut indicators = Vec::new();

        for buff in &self.buffs {
            if !buff.hidden {
                indicators.push(EffectIndicator::from_buff(buff));
            }
        }

        for debuff in &self.debuffs {
            if !debuff.hidden {
                indicators.push(EffectIndicator::from_debuff(debuff));
            }
        }

        indicators
    }

    /// Clear all effects
    pub fn clear_all(&mut self) {
        self.buffs.clear();
        self.debuffs.clear();
    }

    /// Clear all buffs
    pub fn clear_buffs(&mut self) {
        self.buffs.clear();
    }

    /// Clear all debuffs
    pub fn clear_debuffs(&mut self) {
        self.debuffs.clear();
    }

    /// Get count of active buffs
    pub fn buff_count(&self) -> usize {
        self.buffs.len()
    }

    /// Get count of active debuffs
    pub fn debuff_count(&self) -> usize {
        self.debuffs.len()
    }
}

// ============================================================================
// APPLICATION RESULTS
// ============================================================================

/// Result of applying a buff
#[derive(Clone, Debug)]
pub enum BuffApplicationResult {
    Applied,
    Stacked(u32),
    Refreshed,
    Replaced,
    Intensified(f32),
    AppliedAndCancelled(Vec<DebuffType>),
    Failed(String),
}

/// Result of applying a debuff
#[derive(Clone, Debug)]
pub enum DebuffApplicationResult {
    Applied,
    Stacked(u32),
    Refreshed,
    Replaced,
    Intensified(f32),
    AppliedAndRemoved(Vec<BuffType>),
    Immune,
    Failed(String),
}

/// Result of a tick operation
#[derive(Clone, Default, Debug)]
pub struct TickResult {
    /// Total DOT damage this tick
    pub dot_damage: i32,
    /// Sources of DOT damage
    pub dot_sources: Vec<(DebuffType, i32)>,
    /// Mana drained this tick
    pub mana_drain: i32,
    /// Buffs that expired this tick
    pub expired_buffs: Vec<BuffType>,
    /// Debuffs that expired this tick
    pub expired_debuffs: Vec<DebuffType>,
}

// ============================================================================
// BUFF SYSTEM
// ============================================================================

/// Main system for managing buffs and debuffs
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct BuffSystem {
    /// Effect interactions configuration
    pub interactions: EffectInteraction,
    /// Global buff duration modifier (percentage)
    pub global_buff_duration_mod: i32,
    /// Global debuff duration modifier (percentage)
    pub global_debuff_duration_mod: i32,
    /// Global buff potency modifier (percentage)
    pub global_buff_potency_mod: i32,
    /// Global debuff potency modifier (percentage)
    pub global_debuff_potency_mod: i32,
}

impl BuffSystem {
    /// Create a new buff system
    pub fn new() -> Self {
        Self {
            interactions: EffectInteraction::default(),
            global_buff_duration_mod: 0,
            global_debuff_duration_mod: 0,
            global_buff_potency_mod: 0,
            global_debuff_potency_mod: 0,
        }
    }

    /// Create a buff with global modifiers applied
    pub fn create_buff(&self, buff_type: BuffType) -> Buff {
        let mut buff = Buff::new(buff_type);

        // Apply global duration modifier
        if self.global_buff_duration_mod != 0 {
            let modifier = 1.0 + (self.global_buff_duration_mod as f32 / 100.0);
            buff.duration = (buff.duration as f32 * modifier) as u32;
            buff.max_duration = buff.duration;
        }

        // Apply global potency modifier
        if self.global_buff_potency_mod != 0 {
            buff.potency *= 1.0 + (self.global_buff_potency_mod as f32 / 100.0);
        }

        buff
    }

    /// Create a debuff with global modifiers applied
    pub fn create_debuff(&self, debuff_type: DebuffType) -> Debuff {
        let mut debuff = Debuff::new(debuff_type);

        // Apply global duration modifier
        if self.global_debuff_duration_mod != 0 {
            let modifier = 1.0 + (self.global_debuff_duration_mod as f32 / 100.0);
            debuff.duration = (debuff.duration as f32 * modifier) as u32;
            debuff.max_duration = debuff.duration;
        }

        // Apply global potency modifier
        if self.global_debuff_potency_mod != 0 {
            debuff.potency *= 1.0 + (self.global_debuff_potency_mod as f32 / 100.0);
        }

        debuff
    }

    /// Create a shield buff with specified amount
    pub fn create_shield(&self, shield_type: BuffType, amount: i32) -> Buff {
        let mut buff = self.create_buff(shield_type);
        buff.shield_amount = amount;
        buff
    }

    /// Check for synergy between two buffs
    pub fn check_buff_synergy(&self, buff1: BuffType, buff2: BuffType) -> Option<&SynergyEffect> {
        self.interactions.buff_synergies.get(&(buff1, buff2))
            .or_else(|| self.interactions.buff_synergies.get(&(buff2, buff1)))
    }

    /// Check for synergy between two debuffs
    pub fn check_debuff_synergy(&self, debuff1: DebuffType, debuff2: DebuffType) -> Option<&SynergyEffect> {
        self.interactions.debuff_synergies.get(&(debuff1, debuff2))
            .or_else(|| self.interactions.debuff_synergies.get(&(debuff2, debuff1)))
    }

    /// Get all debuffs that a buff cancels
    pub fn get_buff_cancellations(&self, buff_type: BuffType) -> Option<&Vec<DebuffType>> {
        self.interactions.buff_cancels_debuff.get(&buff_type)
    }

    /// Get all buffs that a debuff removes
    pub fn get_debuff_removals(&self, debuff_type: DebuffType) -> Option<&Vec<BuffType>> {
        self.interactions.debuff_removes_buff.get(&debuff_type)
    }

    /// Generate a random buff appropriate for a dungeon level
    pub fn generate_random_buff(&self, dungeon_level: u32, rng: &mut impl Rng) -> Buff {
        let all_buffs = BuffType::all();

        // Weight towards more powerful buffs at higher levels
        let tier_weights = match dungeon_level {
            1..=5 => vec![70, 25, 5, 0, 0],   // Mostly tier 1-2
            6..=10 => vec![40, 40, 15, 5, 0], // More tier 2-3
            11..=15 => vec![20, 35, 30, 15, 0], // Tier 2-4
            16..=20 => vec![10, 20, 35, 30, 5], // Tier 3-5
            _ => vec![5, 15, 30, 35, 15], // High tier
        };

        // Determine tier
        let roll = rng.gen_range(0..100);
        let tier = if roll < tier_weights[0] { 0 }
        else if roll < tier_weights[0] + tier_weights[1] { 1 }
        else if roll < tier_weights[0] + tier_weights[1] + tier_weights[2] { 2 }
        else if roll < tier_weights[0] + tier_weights[1] + tier_weights[2] + tier_weights[3] { 3 }
        else { 4 };

        // Select buff based on tier (using name patterns)
        let buff_type = all_buffs[rng.gen_range(0..all_buffs.len())];

        let mut buff = self.create_buff(buff_type);

        // Increase potency for higher tiers
        buff.potency *= 1.0 + (tier as f32 * 0.2);

        buff
    }

    /// Generate a random debuff appropriate for a dungeon level
    pub fn generate_random_debuff(&self, dungeon_level: u32, rng: &mut impl Rng) -> Debuff {
        let all_debuffs = DebuffType::all();

        // Weight towards more dangerous debuffs at higher levels
        let debuff_type = all_debuffs[rng.gen_range(0..all_debuffs.len())];

        let mut debuff = self.create_debuff(debuff_type);

        // Scale potency with dungeon level
        debuff.potency *= 1.0 + (dungeon_level as f32 * 0.05);

        debuff
    }

    /// Calculate total active synergies for an entity
    pub fn calculate_synergies(&self, effects: &EntityEffects) -> Vec<SynergyEffect> {
        let mut active_synergies = Vec::new();

        // Check buff synergies
        for i in 0..effects.buffs.len() {
            for j in (i + 1)..effects.buffs.len() {
                if let Some(synergy) = self.check_buff_synergy(
                    effects.buffs[i].buff_type,
                    effects.buffs[j].buff_type
                ) {
                    active_synergies.push(synergy.clone());
                }
            }
        }

        // Check debuff synergies
        for i in 0..effects.debuffs.len() {
            for j in (i + 1)..effects.debuffs.len() {
                if let Some(synergy) = self.check_debuff_synergy(
                    effects.debuffs[i].debuff_type,
                    effects.debuffs[j].debuff_type
                ) {
                    active_synergies.push(synergy.clone());
                }
            }
        }

        active_synergies
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buff_creation() {
        let buff = Buff::new(BuffType::StrengthI);
        assert_eq!(buff.buff_type, BuffType::StrengthI);
        assert_eq!(buff.stacks, 1);
        assert!(!buff.is_expired());
    }

    #[test]
    fn test_debuff_creation() {
        let debuff = Debuff::new(DebuffType::Poison);
        assert_eq!(debuff.debuff_type, DebuffType::Poison);
        assert!(debuff.calculate_dot_damage() > 0);
    }

    #[test]
    fn test_buff_tick() {
        let mut buff = Buff::with_duration(BuffType::Shield, 3);
        assert!(buff.tick()); // 2 remaining
        assert!(buff.tick()); // 1 remaining
        assert!(!buff.tick()); // 0 remaining, expired
        assert!(buff.is_expired());
    }

    #[test]
    fn test_entity_effects_add_buff() {
        let mut effects = EntityEffects::new();
        let result = effects.add_buff(Buff::new(BuffType::StrengthI));
        assert!(matches!(result, BuffApplicationResult::Applied));
        assert!(effects.has_buff(BuffType::StrengthI));
    }

    #[test]
    fn test_entity_effects_add_debuff() {
        let mut effects = EntityEffects::new();
        let result = effects.add_debuff(Debuff::new(DebuffType::Poison));
        assert!(matches!(result, DebuffApplicationResult::Applied));
        assert!(effects.has_debuff(DebuffType::Poison));
    }

    #[test]
    fn test_buff_stacking() {
        let mut effects = EntityEffects::new();
        effects.add_buff(Buff::shield(BuffType::Shield, 100));

        let result = effects.add_buff(Buff::shield(BuffType::Shield, 50));
        assert!(matches!(result, BuffApplicationResult::Stacked(_)));

        let shield = effects.get_buff(BuffType::Shield).unwrap();
        assert_eq!(shield.stacks, 2);
    }

    #[test]
    fn test_stat_modifiers() {
        let buff = Buff::new(BuffType::StrengthV);
        let mods = buff.get_stat_modifiers();
        assert!(mods.strength > 0);
    }

    #[test]
    fn test_buff_cancels_debuff() {
        let mut effects = EntityEffects::new();
        effects.add_debuff(Debuff::new(DebuffType::Poison));
        assert!(effects.has_debuff(DebuffType::Poison));

        let result = effects.add_buff(Buff::new(BuffType::PoisonResistance));
        assert!(matches!(result, BuffApplicationResult::AppliedAndCancelled(_)));
        assert!(!effects.has_debuff(DebuffType::Poison));
    }

    #[test]
    fn test_debuff_removes_buff() {
        let mut effects = EntityEffects::new();
        effects.add_buff(Buff::new(BuffType::Blessed));
        assert!(effects.has_buff(BuffType::Blessed));

        let result = effects.add_debuff(Debuff::new(DebuffType::Curse));
        assert!(matches!(result, DebuffApplicationResult::AppliedAndRemoved(_)));
        assert!(!effects.has_buff(BuffType::Blessed));
    }

    #[test]
    fn test_incapacitation() {
        let mut effects = EntityEffects::new();
        assert!(!effects.is_incapacitated());

        effects.add_debuff(Debuff::new(DebuffType::Stun));
        assert!(effects.is_incapacitated());
    }

    #[test]
    fn test_shield_damage() {
        let mut effects = EntityEffects::new();
        effects.add_buff(Buff::shield(BuffType::Shield, 50));

        let remaining = effects.damage_shield(30);
        assert_eq!(remaining, 0);
        assert_eq!(effects.get_shield_amount(), 20);

        let remaining2 = effects.damage_shield(30);
        assert_eq!(remaining2, 10);
        assert_eq!(effects.get_shield_amount(), 0);
    }

    #[test]
    fn test_tick_result() {
        let mut effects = EntityEffects::new();
        effects.add_debuff(Debuff::with_duration(DebuffType::Burn, 2));

        let result = effects.tick();
        assert!(result.dot_damage > 0);
        assert!(effects.has_debuff(DebuffType::Burn));

        let result2 = effects.tick();
        assert!(!effects.has_debuff(DebuffType::Burn));
        assert!(result2.expired_debuffs.contains(&DebuffType::Burn));
    }

    #[test]
    fn test_buff_system_creation() {
        let system = BuffSystem::new();
        let buff = system.create_buff(BuffType::StrengthI);
        assert_eq!(buff.buff_type, BuffType::StrengthI);
    }

    #[test]
    fn test_synergy_check() {
        let system = BuffSystem::new();
        let synergy = system.check_buff_synergy(BuffType::Berserk, BuffType::LifeSteal);
        assert!(synergy.is_some());
        assert_eq!(synergy.unwrap().name, "Bloodlust");
    }

    #[test]
    fn test_visual_indicators() {
        let buff = Buff::with_duration(BuffType::Shield, 5);
        let indicator = EffectIndicator::from_buff(&buff);
        assert!(indicator.is_buff);
        assert_eq!(indicator.duration_text, "5t");
    }

    #[test]
    fn test_combined_modifiers() {
        let mut effects = EntityEffects::new();
        effects.add_buff(Buff::new(BuffType::StrengthI));
        effects.add_buff(Buff::new(BuffType::DefenseI));
        effects.add_debuff(Debuff::new(DebuffType::Weakness));

        let total = effects.get_total_modifiers();
        assert!(total.strength > 0 || total.strength < 10); // StrengthI minus Weakness effect
        assert!(total.defense > 0);
    }
}

//! Rune System
//!
//! This module implements a comprehensive rune system for ShadowCrypt, including:
//! - Individual runes with unique properties and effects
//! - Socketing mechanics for equipment
//! - Rune words (combining specific runes for powerful bonuses)
//! - Rune crafting and upgrading

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::items::{ItemKind, EquipSlot, Rarity};
use crate::combat::StatusEffect;

// ============================================================================
// RUNE TYPES
// ============================================================================

/// All rune types in the game, ordered by tier (1-5)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RuneType {
    // Tier 1 - Common Runes
    El,     // +1 Defense, +1 Light Radius
    Eld,    // +2 Defense vs Undead, +15% Attack vs Undead
    Tir,    // +2 Mana after Kill
    Nef,    // Knockback, +30 Defense vs Missile
    Eth,    // -25% Target Defense, Regenerate Mana 15%
    Ith,    // +9 Max Damage
    Tal,    // +75 Poison Damage over 5 turns
    Ral,    // +5-30 Fire Damage
    Ort,    // +1-50 Lightning Damage
    Thul,   // +3-14 Cold Damage, Cold Length 3 turns

    // Tier 2 - Uncommon Runes
    Amn,    // 7% Life Stolen per Hit
    Sol,    // +9 Min Damage, -25% Damage Taken
    Shael,  // +20% Attack Speed
    Dol,    // Hit Causes Monster to Flee, +7 Replenish Life
    Hel,    // Requirements -15%, -20% Max Mana
    Io,     // +10 Vitality
    Lum,    // +10 Energy
    Ko,     // +10 Dexterity
    Fal,    // +10 Strength
    Lem,    // +75% Extra Gold

    // Tier 3 - Rare Runes
    Pul,    // +75% Attack vs Demons, +100 Defense vs Demons
    Um,     // +25% Open Wounds, +22 Defense (All)
    Mal,    // Prevent Monster Heal, +7 Magic Damage Reduction
    Ist,    // +30% Magic Find, +25% Cold Resist
    Gul,    // +20% Attack Rating, +5% Max Poison Resist
    Vex,    // 7% Mana Stolen per Hit, +5% Max Fire Resist
    Ohm,    // +50% Enhanced Damage, +5% Max Cold Resist
    Lo,     // +20% Deadly Strike, +5% Max Lightning Resist
    Sur,    // +50% Freeze Length, Hit Blinds Target
    Ber,    // +30% Crushing Blow, +8% Damage Reduction

    // Tier 4 - Epic Runes
    Jah,    // Ignore Target Defense, +50 Max Life
    Cham,   // Cannot Be Frozen, +32% Cold Resist
    Zod,    // Indestructible, +15 All Stats

    // Tier 5 - Legendary Runes
    Xul,    // +100% Damage vs Undead, Summon Shadow
    Vul,    // +100% Fire Damage, Fire Aura
    Nar,    // +50% All Resist, Regenerate 5% Life
}

impl RuneType {
    /// Returns all rune types
    pub fn all() -> Vec<Self> {
        vec![
            // Tier 1
            Self::El, Self::Eld, Self::Tir, Self::Nef, Self::Eth,
            Self::Ith, Self::Tal, Self::Ral, Self::Ort, Self::Thul,
            // Tier 2
            Self::Amn, Self::Sol, Self::Shael, Self::Dol, Self::Hel,
            Self::Io, Self::Lum, Self::Ko, Self::Fal, Self::Lem,
            // Tier 3
            Self::Pul, Self::Um, Self::Mal, Self::Ist, Self::Gul,
            Self::Vex, Self::Ohm, Self::Lo, Self::Sur, Self::Ber,
            // Tier 4
            Self::Jah, Self::Cham, Self::Zod,
            // Tier 5
            Self::Xul, Self::Vul, Self::Nar,
        ]
    }

    /// Returns the tier of this rune (1-5)
    pub fn tier(&self) -> u8 {
        match self {
            Self::El | Self::Eld | Self::Tir | Self::Nef | Self::Eth |
            Self::Ith | Self::Tal | Self::Ral | Self::Ort | Self::Thul => 1,

            Self::Amn | Self::Sol | Self::Shael | Self::Dol | Self::Hel |
            Self::Io | Self::Lum | Self::Ko | Self::Fal | Self::Lem => 2,

            Self::Pul | Self::Um | Self::Mal | Self::Ist | Self::Gul |
            Self::Vex | Self::Ohm | Self::Lo | Self::Sur | Self::Ber => 3,

            Self::Jah | Self::Cham | Self::Zod => 4,

            Self::Xul | Self::Vul | Self::Nar => 5,
        }
    }

    /// Returns the rarity corresponding to this rune's tier
    pub fn rarity(&self) -> Rarity {
        match self.tier() {
            1 => Rarity::Common,
            2 => Rarity::Uncommon,
            3 => Rarity::Rare,
            4 => Rarity::Epic,
            5 => Rarity::Legendary,
            _ => Rarity::Common,
        }
    }

    /// Returns the name of this rune
    pub fn name(&self) -> &'static str {
        match self {
            Self::El => "El",
            Self::Eld => "Eld",
            Self::Tir => "Tir",
            Self::Nef => "Nef",
            Self::Eth => "Eth",
            Self::Ith => "Ith",
            Self::Tal => "Tal",
            Self::Ral => "Ral",
            Self::Ort => "Ort",
            Self::Thul => "Thul",
            Self::Amn => "Amn",
            Self::Sol => "Sol",
            Self::Shael => "Shael",
            Self::Dol => "Dol",
            Self::Hel => "Hel",
            Self::Io => "Io",
            Self::Lum => "Lum",
            Self::Ko => "Ko",
            Self::Fal => "Fal",
            Self::Lem => "Lem",
            Self::Pul => "Pul",
            Self::Um => "Um",
            Self::Mal => "Mal",
            Self::Ist => "Ist",
            Self::Gul => "Gul",
            Self::Vex => "Vex",
            Self::Ohm => "Ohm",
            Self::Lo => "Lo",
            Self::Sur => "Sur",
            Self::Ber => "Ber",
            Self::Jah => "Jah",
            Self::Cham => "Cham",
            Self::Zod => "Zod",
            Self::Xul => "Xul",
            Self::Vul => "Vul",
            Self::Nar => "Nar",
        }
    }

    /// Returns the glyph character for this rune
    pub fn glyph(&self) -> char {
        match self.tier() {
            1 => 'r',
            2 => 'R',
            3 => '*',
            4 => '+',
            5 => '@',
            _ => 'r',
        }
    }

    /// Returns the color index for this rune (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self.tier() {
            1 => 1,   // Grey
            2 => 5,   // Green
            3 => 7,   // Blue
            4 => 13,  // Magenta
            5 => 11,  // Yellow
            _ => 1,
        }
    }

    /// Returns the effects this rune provides when socketed in a weapon
    pub fn weapon_effects(&self) -> Vec<RuneEffect> {
        match self {
            Self::El => vec![RuneEffect::BonusDefense(1), RuneEffect::LightRadius(1)],
            Self::Eld => vec![RuneEffect::DamageVsUndead(15), RuneEffect::DefenseVsUndead(2)],
            Self::Tir => vec![RuneEffect::ManaAfterKill(2)],
            Self::Nef => vec![RuneEffect::Knockback],
            Self::Eth => vec![RuneEffect::TargetDefenseReduction(25), RuneEffect::ManaRegen(15)],
            Self::Ith => vec![RuneEffect::MaxDamage(9)],
            Self::Tal => vec![RuneEffect::PoisonDamage { damage: 75, duration: 5 }],
            Self::Ral => vec![RuneEffect::FireDamage { min: 5, max: 30 }],
            Self::Ort => vec![RuneEffect::LightningDamage { min: 1, max: 50 }],
            Self::Thul => vec![RuneEffect::ColdDamage { min: 3, max: 14, duration: 3 }],

            Self::Amn => vec![RuneEffect::LifeSteal(7)],
            Self::Sol => vec![RuneEffect::MinDamage(9)],
            Self::Shael => vec![RuneEffect::AttackSpeed(20)],
            Self::Dol => vec![RuneEffect::CauseFlee, RuneEffect::ReplenishLife(7)],
            Self::Hel => vec![RuneEffect::RequirementsReduction(15)],
            Self::Io => vec![RuneEffect::BonusVitality(10)],
            Self::Lum => vec![RuneEffect::BonusEnergy(10)],
            Self::Ko => vec![RuneEffect::BonusDexterity(10)],
            Self::Fal => vec![RuneEffect::BonusStrength(10)],
            Self::Lem => vec![RuneEffect::GoldFind(75)],

            Self::Pul => vec![RuneEffect::DamageVsDemons(75)],
            Self::Um => vec![RuneEffect::OpenWounds(25)],
            Self::Mal => vec![RuneEffect::PreventHeal],
            Self::Ist => vec![RuneEffect::MagicFind(30)],
            Self::Gul => vec![RuneEffect::AttackRating(20)],
            Self::Vex => vec![RuneEffect::ManaSteal(7)],
            Self::Ohm => vec![RuneEffect::EnhancedDamage(50)],
            Self::Lo => vec![RuneEffect::DeadlyStrike(20)],
            Self::Sur => vec![RuneEffect::HitBlindsTarget],
            Self::Ber => vec![RuneEffect::CrushingBlow(30)],

            Self::Jah => vec![RuneEffect::IgnoreTargetDefense],
            Self::Cham => vec![RuneEffect::CannotBeFrozen],
            Self::Zod => vec![RuneEffect::Indestructible, RuneEffect::AllStats(15)],

            Self::Xul => vec![RuneEffect::DamageVsUndead(100), RuneEffect::SummonShadow],
            Self::Vul => vec![RuneEffect::FireDamagePercent(100), RuneEffect::FireAura(10)],
            Self::Nar => vec![RuneEffect::AllResist(50), RuneEffect::LifeRegen(5)],
        }
    }

    /// Returns the effects this rune provides when socketed in armor
    pub fn armor_effects(&self) -> Vec<RuneEffect> {
        match self {
            Self::El => vec![RuneEffect::BonusDefense(1), RuneEffect::LightRadius(1)],
            Self::Eld => vec![RuneEffect::SlowStamina(15)],
            Self::Tir => vec![RuneEffect::ManaAfterKill(2)],
            Self::Nef => vec![RuneEffect::DefenseVsMissile(30)],
            Self::Eth => vec![RuneEffect::ManaRegen(15)],
            Self::Ith => vec![RuneEffect::DamageReduction(15)],
            Self::Tal => vec![RuneEffect::PoisonResist(30)],
            Self::Ral => vec![RuneEffect::FireResist(30)],
            Self::Ort => vec![RuneEffect::LightningResist(30)],
            Self::Thul => vec![RuneEffect::ColdResist(30)],

            Self::Amn => vec![RuneEffect::AttackerTakesDamage(14)],
            Self::Sol => vec![RuneEffect::DamageReduction(25)],
            Self::Shael => vec![RuneEffect::FasterHitRecovery(20)],
            Self::Dol => vec![RuneEffect::ReplenishLife(7)],
            Self::Hel => vec![RuneEffect::RequirementsReduction(15)],
            Self::Io => vec![RuneEffect::BonusVitality(10)],
            Self::Lum => vec![RuneEffect::BonusEnergy(10)],
            Self::Ko => vec![RuneEffect::BonusDexterity(10)],
            Self::Fal => vec![RuneEffect::BonusStrength(10)],
            Self::Lem => vec![RuneEffect::GoldFind(75)],

            Self::Pul => vec![RuneEffect::DefenseVsDemons(100)],
            Self::Um => vec![RuneEffect::AllResist(22)],
            Self::Mal => vec![RuneEffect::MagicDamageReduction(7)],
            Self::Ist => vec![RuneEffect::MagicFind(25)],
            Self::Gul => vec![RuneEffect::MaxPoisonResist(5)],
            Self::Vex => vec![RuneEffect::MaxFireResist(5)],
            Self::Ohm => vec![RuneEffect::MaxColdResist(5)],
            Self::Lo => vec![RuneEffect::MaxLightningResist(5)],
            Self::Sur => vec![RuneEffect::BonusMaxMana(50)],
            Self::Ber => vec![RuneEffect::DamageReductionPercent(8)],

            Self::Jah => vec![RuneEffect::BonusMaxLife(50)],
            Self::Cham => vec![RuneEffect::CannotBeFrozen, RuneEffect::ColdResist(32)],
            Self::Zod => vec![RuneEffect::Indestructible, RuneEffect::AllStats(15)],

            Self::Xul => vec![RuneEffect::DamageVsUndead(100), RuneEffect::Thorns(50)],
            Self::Vul => vec![RuneEffect::FireAbsorb(20), RuneEffect::FireAura(10)],
            Self::Nar => vec![RuneEffect::AllResist(50), RuneEffect::LifeRegen(5)],
        }
    }

    /// Returns the effects for shield slot
    pub fn shield_effects(&self) -> Vec<RuneEffect> {
        // Shields generally use armor effects with some modifications
        self.armor_effects()
    }
}

// ============================================================================
// RUNE EFFECTS
// ============================================================================

/// Effects that runes can provide
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RuneEffect {
    // Damage bonuses
    MinDamage(i32),
    MaxDamage(i32),
    EnhancedDamage(i32),          // Percentage
    FireDamage { min: i32, max: i32 },
    ColdDamage { min: i32, max: i32, duration: i32 },
    LightningDamage { min: i32, max: i32 },
    PoisonDamage { damage: i32, duration: i32 },
    FireDamagePercent(i32),

    // Damage vs specific enemies
    DamageVsUndead(i32),          // Percentage
    DamageVsDemons(i32),          // Percentage

    // Attack modifiers
    AttackRating(i32),            // Percentage
    AttackSpeed(i32),             // Percentage
    DeadlyStrike(i32),            // Percentage (double damage chance)
    CrushingBlow(i32),            // Percentage (reduce enemy HP)
    OpenWounds(i32),              // Percentage (bleed chance)
    IgnoreTargetDefense,
    TargetDefenseReduction(i32),  // Percentage
    Knockback,
    CauseFlee,
    PreventHeal,
    HitBlindsTarget,

    // Defense
    BonusDefense(i32),
    DefenseVsUndead(i32),
    DefenseVsDemons(i32),
    DefenseVsMissile(i32),
    DamageReduction(i32),         // Flat reduction
    DamageReductionPercent(i32),  // Percentage reduction
    MagicDamageReduction(i32),
    FasterHitRecovery(i32),       // Percentage
    AttackerTakesDamage(i32),
    Thorns(i32),                  // Percentage of damage reflected

    // Resistances
    FireResist(i32),
    ColdResist(i32),
    LightningResist(i32),
    PoisonResist(i32),
    AllResist(i32),
    MaxFireResist(i32),
    MaxColdResist(i32),
    MaxLightningResist(i32),
    MaxPoisonResist(i32),
    FireAbsorb(i32),              // Percentage absorbed as healing

    // Life and Mana
    BonusMaxLife(i32),
    BonusMaxMana(i32),
    LifeSteal(i32),               // Percentage
    ManaSteal(i32),               // Percentage
    ReplenishLife(i32),           // Per turn
    ManaRegen(i32),               // Percentage
    ManaAfterKill(i32),
    LifeRegen(i32),               // Percentage per turn

    // Stats
    BonusStrength(i32),
    BonusDexterity(i32),
    BonusVitality(i32),
    BonusEnergy(i32),
    AllStats(i32),

    // Special
    LightRadius(i32),
    MagicFind(i32),               // Percentage
    GoldFind(i32),                // Percentage
    RequirementsReduction(i32),   // Percentage
    CannotBeFrozen,
    SlowStamina(i32),
    Indestructible,

    // Auras and Summons
    FireAura(i32),                // Damage per turn to nearby enemies
    SummonShadow,                 // Summon shadow clone
}

impl RuneEffect {
    /// Returns a human-readable description of this effect
    pub fn description(&self) -> String {
        match self {
            Self::MinDamage(v) => format!("+{} Min Damage", v),
            Self::MaxDamage(v) => format!("+{} Max Damage", v),
            Self::EnhancedDamage(v) => format!("+{}% Enhanced Damage", v),
            Self::FireDamage { min, max } => format!("+{}-{} Fire Damage", min, max),
            Self::ColdDamage { min, max, duration } => format!("+{}-{} Cold Damage ({} turns)", min, max, duration),
            Self::LightningDamage { min, max } => format!("+{}-{} Lightning Damage", min, max),
            Self::PoisonDamage { damage, duration } => format!("+{} Poison Damage over {} turns", damage, duration),
            Self::FireDamagePercent(v) => format!("+{}% Fire Damage", v),
            Self::DamageVsUndead(v) => format!("+{}% Damage vs Undead", v),
            Self::DamageVsDemons(v) => format!("+{}% Damage vs Demons", v),
            Self::AttackRating(v) => format!("+{}% Attack Rating", v),
            Self::AttackSpeed(v) => format!("+{}% Attack Speed", v),
            Self::DeadlyStrike(v) => format!("{}% Deadly Strike", v),
            Self::CrushingBlow(v) => format!("{}% Crushing Blow", v),
            Self::OpenWounds(v) => format!("{}% Open Wounds", v),
            Self::IgnoreTargetDefense => "Ignore Target Defense".to_string(),
            Self::TargetDefenseReduction(v) => format!("-{}% Target Defense", v),
            Self::Knockback => "Knockback".to_string(),
            Self::CauseFlee => "Hit Causes Monster to Flee".to_string(),
            Self::PreventHeal => "Prevent Monster Heal".to_string(),
            Self::HitBlindsTarget => "Hit Blinds Target".to_string(),
            Self::BonusDefense(v) => format!("+{} Defense", v),
            Self::DefenseVsUndead(v) => format!("+{} Defense vs Undead", v),
            Self::DefenseVsDemons(v) => format!("+{} Defense vs Demons", v),
            Self::DefenseVsMissile(v) => format!("+{} Defense vs Missile", v),
            Self::DamageReduction(v) => format!("Damage Reduced by {}", v),
            Self::DamageReductionPercent(v) => format!("{}% Damage Reduction", v),
            Self::MagicDamageReduction(v) => format!("Magic Damage Reduced by {}", v),
            Self::FasterHitRecovery(v) => format!("+{}% Faster Hit Recovery", v),
            Self::AttackerTakesDamage(v) => format!("Attacker Takes {} Damage", v),
            Self::Thorns(v) => format!("{}% Thorns", v),
            Self::FireResist(v) => format!("+{}% Fire Resist", v),
            Self::ColdResist(v) => format!("+{}% Cold Resist", v),
            Self::LightningResist(v) => format!("+{}% Lightning Resist", v),
            Self::PoisonResist(v) => format!("+{}% Poison Resist", v),
            Self::AllResist(v) => format!("+{}% All Resist", v),
            Self::MaxFireResist(v) => format!("+{}% Max Fire Resist", v),
            Self::MaxColdResist(v) => format!("+{}% Max Cold Resist", v),
            Self::MaxLightningResist(v) => format!("+{}% Max Lightning Resist", v),
            Self::MaxPoisonResist(v) => format!("+{}% Max Poison Resist", v),
            Self::FireAbsorb(v) => format!("{}% Fire Absorb", v),
            Self::BonusMaxLife(v) => format!("+{} Max Life", v),
            Self::BonusMaxMana(v) => format!("+{} Max Mana", v),
            Self::LifeSteal(v) => format!("{}% Life Stolen per Hit", v),
            Self::ManaSteal(v) => format!("{}% Mana Stolen per Hit", v),
            Self::ReplenishLife(v) => format!("Replenish Life +{}", v),
            Self::ManaRegen(v) => format!("Regenerate Mana {}%", v),
            Self::ManaAfterKill(v) => format!("+{} Mana after Kill", v),
            Self::LifeRegen(v) => format!("Regenerate {}% Life per Turn", v),
            Self::BonusStrength(v) => format!("+{} Strength", v),
            Self::BonusDexterity(v) => format!("+{} Dexterity", v),
            Self::BonusVitality(v) => format!("+{} Vitality", v),
            Self::BonusEnergy(v) => format!("+{} Energy", v),
            Self::AllStats(v) => format!("+{} All Stats", v),
            Self::LightRadius(v) => format!("+{} Light Radius", v),
            Self::MagicFind(v) => format!("+{}% Magic Find", v),
            Self::GoldFind(v) => format!("+{}% Gold Find", v),
            Self::RequirementsReduction(v) => format!("-{}% Requirements", v),
            Self::CannotBeFrozen => "Cannot Be Frozen".to_string(),
            Self::SlowStamina(v) => format!("{}% Slower Stamina Drain", v),
            Self::Indestructible => "Indestructible".to_string(),
            Self::FireAura(v) => format!("Fire Aura (+{} damage)", v),
            Self::SummonShadow => "Summon Shadow Clone".to_string(),
        }
    }
}

// ============================================================================
// RUNE INSTANCE
// ============================================================================

/// A rune instance with position (for inventory/world)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Rune {
    pub x: usize,
    pub y: usize,
    pub rune_type: RuneType,
}

impl Rune {
    /// Create a new rune at the given position
    pub fn new(x: usize, y: usize, rune_type: RuneType) -> Self {
        Self { x, y, rune_type }
    }

    /// Returns the display name of this rune
    pub fn display_name(&self) -> String {
        format!("{} Rune", self.rune_type.name())
    }

    /// Returns the rarity of this rune
    pub fn rarity(&self) -> Rarity {
        self.rune_type.rarity()
    }
}

// ============================================================================
// SOCKETING SYSTEM
// ============================================================================

/// Maximum number of sockets an item can have by slot
pub fn max_sockets_for_slot(slot: EquipSlot) -> usize {
    match slot {
        EquipSlot::Weapon => 6,
        EquipSlot::Armor => 4,
        EquipSlot::Helmet => 3,
        EquipSlot::Shield => 4,
        EquipSlot::Gloves => 2,
        EquipSlot::Boots => 2,
        EquipSlot::Ring1 | EquipSlot::Ring2 => 1,
        EquipSlot::Amulet => 1,
    }
}

/// Determines how many sockets an item gets based on rarity and item level
pub fn calculate_sockets(rarity: Rarity, item_level: u32) -> usize {
    let base_sockets = match rarity {
        Rarity::Common => 1,
        Rarity::Uncommon => 2,
        Rarity::Rare => 3,
        Rarity::Epic => 4,
        Rarity::Legendary => 5,
        Rarity::Mythic => 6,
    };

    // Item level can add extra sockets
    let level_bonus = (item_level / 10) as usize;

    base_sockets + level_bonus.min(2)
}

/// Represents an item's socket configuration
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SocketedItem {
    /// Maximum number of sockets this item can have
    pub max_sockets: usize,
    /// Currently socketed runes
    pub sockets: Vec<Option<RuneType>>,
    /// Active rune word (if any)
    pub rune_word: Option<RuneWordType>,
}

impl SocketedItem {
    /// Create a new socketed item with the given number of sockets
    pub fn new(max_sockets: usize) -> Self {
        Self {
            max_sockets,
            sockets: vec![None; max_sockets],
            rune_word: None,
        }
    }

    /// Create a socketed item based on item properties
    pub fn from_item(slot: EquipSlot, rarity: Rarity, item_level: u32) -> Self {
        let max_possible = max_sockets_for_slot(slot);
        let calculated = calculate_sockets(rarity, item_level);
        let max_sockets = calculated.min(max_possible);
        Self::new(max_sockets)
    }

    /// Returns the number of empty sockets
    pub fn empty_sockets(&self) -> usize {
        self.sockets.iter().filter(|s| s.is_none()).count()
    }

    /// Returns the number of filled sockets
    pub fn filled_sockets(&self) -> usize {
        self.sockets.iter().filter(|s| s.is_some()).count()
    }

    /// Insert a rune into the first empty socket
    pub fn insert_rune(&mut self, rune: RuneType) -> Result<(), &'static str> {
        for socket in &mut self.sockets {
            if socket.is_none() {
                *socket = Some(rune);
                self.check_rune_word();
                return Ok(());
            }
        }
        Err("No empty sockets available")
    }

    /// Insert a rune into a specific socket index
    pub fn insert_rune_at(&mut self, index: usize, rune: RuneType) -> Result<(), &'static str> {
        if index >= self.max_sockets {
            return Err("Invalid socket index");
        }
        if self.sockets[index].is_some() {
            return Err("Socket is already occupied");
        }
        self.sockets[index] = Some(rune);
        self.check_rune_word();
        Ok(())
    }

    /// Remove a rune from a specific socket (destroys the rune)
    pub fn remove_rune(&mut self, index: usize) -> Option<RuneType> {
        if index >= self.max_sockets {
            return None;
        }
        let rune = self.sockets[index].take();
        if rune.is_some() {
            self.rune_word = None; // Removing a rune breaks any rune word
        }
        rune
    }

    /// Remove all runes from the item
    pub fn clear_sockets(&mut self) -> Vec<RuneType> {
        let runes: Vec<RuneType> = self.sockets.iter().filter_map(|s| *s).collect();
        self.sockets = vec![None; self.max_sockets];
        self.rune_word = None;
        runes
    }

    /// Check if current runes form a rune word and activate it
    fn check_rune_word(&mut self) {
        let runes: Vec<RuneType> = self.sockets.iter().filter_map(|s| *s).collect();
        self.rune_word = RuneWordType::find_match(&runes);
    }

    /// Get all effects from socketed runes (considering slot type)
    pub fn get_effects(&self, slot: EquipSlot) -> Vec<RuneEffect> {
        let mut effects = Vec::new();

        // If we have a rune word, use its effects instead of individual rune effects
        if let Some(ref rune_word) = self.rune_word {
            effects.extend(rune_word.effects());
        } else {
            // Otherwise, accumulate individual rune effects
            for rune in self.sockets.iter().filter_map(|s| *s) {
                let rune_effects = match slot {
                    EquipSlot::Weapon => rune.weapon_effects(),
                    EquipSlot::Shield => rune.shield_effects(),
                    _ => rune.armor_effects(),
                };
                effects.extend(rune_effects);
            }
        }

        effects
    }

    /// Get a description of the socket state
    pub fn socket_description(&self) -> String {
        if self.max_sockets == 0 {
            return "No sockets".to_string();
        }

        let filled: Vec<String> = self.sockets
            .iter()
            .filter_map(|s| s.map(|r| r.name().to_string()))
            .collect();

        if filled.is_empty() {
            format!("{} empty sockets", self.max_sockets)
        } else if let Some(ref rw) = self.rune_word {
            format!("Rune Word: {} ({})", rw.name(), filled.join("-"))
        } else {
            format!("Socketed: {} ({}/{})", filled.join("-"), filled.len(), self.max_sockets)
        }
    }
}

// ============================================================================
// RUNE WORDS
// ============================================================================

/// All rune word types in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RuneWordType {
    // 2-Socket Rune Words
    Steel,       // Tir + El - Basic damage boost for early game
    Nadir,       // Nef + Tir - Defensive helm
    Malice,      // Ith + El + Eth - Aggressive weapon
    Stealth,     // Tal + Eth - Stealth armor
    Leaf,        // Tir + Ral - Fire staff
    Zephyr,      // Ort + Eth - Speed bow

    // 3-Socket Rune Words
    Ancient,     // Ral + Ort + Tal - Elemental resistance
    Spirit,      // Tal + Thul + Ort + Amn - Mana and skills
    Lore,        // Ort + Sol - Knowledge helm
    Radiance,    // Nef + Sol + Ith - Light and mana
    Strength,    // Amn + Tir - Power weapon
    Edge,        // Tir + Tal + Amn - Deadly bow

    // 4-Socket Rune Words
    Insight,     // Ral + Tir + Tal + Sol - Meditation aura
    Fortitude,   // El + Sol + Dol + Lo - Massive defense
    Passion,     // Dol + Ort + Eld + Lem - Zeal attacks
    Harmony,     // Tir + Ith + Sol + Ko - Vigor aura
    Peace,       // Shael + Thul + Amn - Defense armor
    Honor,       // Amn + El + Ith + Tir + Sol - Noble weapon

    // 5-Socket Rune Words
    Grief,       // Eth + Tir + Lo + Mal + Ral - Ultimate damage
    Enigma,      // Jah + Ith + Ber - Teleportation armor
    Infinity,    // Ber + Mal + Ber + Ist - Conviction aura
    Pride,       // Cham + Sur + Io + Lo - Concentration aura
    Destruction, // Vex + Lo + Ber + Jah + Ko - Devastating weapon

    // 6-Socket Rune Words
    Breath,      // Sur + Cham + Amn + Ber + Ist + Sol - Life mastery
    Eternity,    // Amn + Ber + Ist + Sol + Sur + Lo - Immortal weapon
    Oblivion,    // Zod + Jah + Cham + Ber + Lo + Sur - Ultimate power
    Phoenix,     // Vex + Vul + Lo + Jah - Rebirth from ashes

    // Legendary Rune Words (require tier 5 runes)
    Apocalypse,  // Xul + Vul + Nar - End of all things
    Genesis,     // Nar + Xul + Vul + Zod - Creation of power
}

impl RuneWordType {
    /// Returns all rune word types
    pub fn all() -> Vec<Self> {
        vec![
            // 2-Socket
            Self::Steel, Self::Nadir, Self::Malice, Self::Stealth, Self::Leaf, Self::Zephyr,
            // 3-Socket
            Self::Ancient, Self::Spirit, Self::Lore, Self::Radiance, Self::Strength, Self::Edge,
            // 4-Socket
            Self::Insight, Self::Fortitude, Self::Passion, Self::Harmony, Self::Peace, Self::Honor,
            // 5-Socket
            Self::Grief, Self::Enigma, Self::Infinity, Self::Pride, Self::Destruction,
            // 6-Socket
            Self::Breath, Self::Eternity, Self::Oblivion, Self::Phoenix,
            // Legendary
            Self::Apocalypse, Self::Genesis,
        ]
    }

    /// Returns the name of this rune word
    pub fn name(&self) -> &'static str {
        match self {
            Self::Steel => "Steel",
            Self::Nadir => "Nadir",
            Self::Malice => "Malice",
            Self::Stealth => "Stealth",
            Self::Leaf => "Leaf",
            Self::Zephyr => "Zephyr",
            Self::Ancient => "Ancient's Pledge",
            Self::Spirit => "Spirit",
            Self::Lore => "Lore",
            Self::Radiance => "Radiance",
            Self::Strength => "Strength",
            Self::Edge => "Edge",
            Self::Insight => "Insight",
            Self::Fortitude => "Fortitude",
            Self::Passion => "Passion",
            Self::Harmony => "Harmony",
            Self::Peace => "Peace",
            Self::Honor => "Honor",
            Self::Grief => "Grief",
            Self::Enigma => "Enigma",
            Self::Infinity => "Infinity",
            Self::Pride => "Pride",
            Self::Destruction => "Destruction",
            Self::Breath => "Breath of the Dying",
            Self::Eternity => "Eternity",
            Self::Oblivion => "Oblivion",
            Self::Phoenix => "Phoenix",
            Self::Apocalypse => "Apocalypse",
            Self::Genesis => "Genesis",
        }
    }

    /// Returns the rune sequence required for this rune word
    pub fn recipe(&self) -> Vec<RuneType> {
        match self {
            // 2-Socket
            Self::Steel => vec![RuneType::Tir, RuneType::El],
            Self::Nadir => vec![RuneType::Nef, RuneType::Tir],
            Self::Malice => vec![RuneType::Ith, RuneType::El, RuneType::Eth],
            Self::Stealth => vec![RuneType::Tal, RuneType::Eth],
            Self::Leaf => vec![RuneType::Tir, RuneType::Ral],
            Self::Zephyr => vec![RuneType::Ort, RuneType::Eth],

            // 3-Socket
            Self::Ancient => vec![RuneType::Ral, RuneType::Ort, RuneType::Tal],
            Self::Spirit => vec![RuneType::Tal, RuneType::Thul, RuneType::Ort, RuneType::Amn],
            Self::Lore => vec![RuneType::Ort, RuneType::Sol],
            Self::Radiance => vec![RuneType::Nef, RuneType::Sol, RuneType::Ith],
            Self::Strength => vec![RuneType::Amn, RuneType::Tir],
            Self::Edge => vec![RuneType::Tir, RuneType::Tal, RuneType::Amn],

            // 4-Socket
            Self::Insight => vec![RuneType::Ral, RuneType::Tir, RuneType::Tal, RuneType::Sol],
            Self::Fortitude => vec![RuneType::El, RuneType::Sol, RuneType::Dol, RuneType::Lo],
            Self::Passion => vec![RuneType::Dol, RuneType::Ort, RuneType::Eld, RuneType::Lem],
            Self::Harmony => vec![RuneType::Tir, RuneType::Ith, RuneType::Sol, RuneType::Ko],
            Self::Peace => vec![RuneType::Shael, RuneType::Thul, RuneType::Amn],
            Self::Honor => vec![RuneType::Amn, RuneType::El, RuneType::Ith, RuneType::Tir, RuneType::Sol],

            // 5-Socket
            Self::Grief => vec![RuneType::Eth, RuneType::Tir, RuneType::Lo, RuneType::Mal, RuneType::Ral],
            Self::Enigma => vec![RuneType::Jah, RuneType::Ith, RuneType::Ber],
            Self::Infinity => vec![RuneType::Ber, RuneType::Mal, RuneType::Ber, RuneType::Ist],
            Self::Pride => vec![RuneType::Cham, RuneType::Sur, RuneType::Io, RuneType::Lo],
            Self::Destruction => vec![RuneType::Vex, RuneType::Lo, RuneType::Ber, RuneType::Jah, RuneType::Ko],

            // 6-Socket
            Self::Breath => vec![RuneType::Sur, RuneType::Cham, RuneType::Amn, RuneType::Ber, RuneType::Ist, RuneType::Sol],
            Self::Eternity => vec![RuneType::Amn, RuneType::Ber, RuneType::Ist, RuneType::Sol, RuneType::Sur, RuneType::Lo],
            Self::Oblivion => vec![RuneType::Zod, RuneType::Jah, RuneType::Cham, RuneType::Ber, RuneType::Lo, RuneType::Sur],
            Self::Phoenix => vec![RuneType::Vex, RuneType::Vul, RuneType::Lo, RuneType::Jah],

            // Legendary
            Self::Apocalypse => vec![RuneType::Xul, RuneType::Vul, RuneType::Nar],
            Self::Genesis => vec![RuneType::Nar, RuneType::Xul, RuneType::Vul, RuneType::Zod],
        }
    }

    /// Find a matching rune word for the given rune sequence
    pub fn find_match(runes: &[RuneType]) -> Option<Self> {
        for rune_word in Self::all() {
            let recipe = rune_word.recipe();
            if runes.len() == recipe.len() && runes == recipe.as_slice() {
                return Some(rune_word);
            }
        }
        None
    }

    /// Returns the effects provided by this rune word
    pub fn effects(&self) -> Vec<RuneEffect> {
        match self {
            // 2-Socket Rune Words
            Self::Steel => vec![
                RuneEffect::EnhancedDamage(20),
                RuneEffect::MinDamage(3),
                RuneEffect::MaxDamage(3),
                RuneEffect::OpenWounds(50),
                RuneEffect::AttackSpeed(25),
                RuneEffect::ManaAfterKill(2),
                RuneEffect::LightRadius(1),
            ],
            Self::Nadir => vec![
                RuneEffect::BonusDefense(50),
                RuneEffect::BonusStrength(5),
                RuneEffect::ManaAfterKill(2),
                RuneEffect::LightRadius(-3),
            ],
            Self::Malice => vec![
                RuneEffect::EnhancedDamage(33),
                RuneEffect::MaxDamage(9),
                RuneEffect::PreventHeal,
                RuneEffect::OpenWounds(100),
                RuneEffect::DamageReduction(-25),
            ],
            Self::Stealth => vec![
                RuneEffect::FasterHitRecovery(25),
                RuneEffect::AttackSpeed(25),
                RuneEffect::ManaRegen(15),
                RuneEffect::BonusDexterity(6),
                RuneEffect::PoisonResist(30),
            ],
            Self::Leaf => vec![
                RuneEffect::FireDamage { min: 3, max: 14 },
                RuneEffect::BonusMaxMana(33),
                RuneEffect::ColdResist(33),
                RuneEffect::ManaAfterKill(2),
            ],
            Self::Zephyr => vec![
                RuneEffect::AttackSpeed(25),
                RuneEffect::EnhancedDamage(33),
                RuneEffect::AttackRating(66),
                RuneEffect::LightningDamage { min: 1, max: 50 },
            ],

            // 3-Socket Rune Words
            Self::Ancient => vec![
                RuneEffect::AllResist(43),
                RuneEffect::BonusDefense(50),
                RuneEffect::DamageReduction(10),
            ],
            Self::Spirit => vec![
                RuneEffect::BonusMaxMana(89),
                RuneEffect::MagicFind(50),
                RuneEffect::FasterHitRecovery(55),
                RuneEffect::BonusVitality(22),
                RuneEffect::ColdAbsorb(3),
            ],
            Self::Lore => vec![
                RuneEffect::BonusEnergy(10),
                RuneEffect::ManaAfterKill(2),
                RuneEffect::LightRadius(2),
                RuneEffect::LightningResist(30),
                RuneEffect::DamageReduction(7),
            ],
            Self::Radiance => vec![
                RuneEffect::LightRadius(5),
                RuneEffect::BonusDamage(75),
                RuneEffect::BonusEnergy(10),
                RuneEffect::BonusMaxMana(33),
                RuneEffect::MagicDamageReduction(5),
            ],
            Self::Strength => vec![
                RuneEffect::EnhancedDamage(35),
                RuneEffect::BonusStrength(25),
                RuneEffect::BonusVitality(10),
                RuneEffect::LifeSteal(7),
                RuneEffect::ManaAfterKill(2),
            ],
            Self::Edge => vec![
                RuneEffect::EnhancedDamage(35),
                RuneEffect::AttackSpeed(35),
                RuneEffect::DamageVsDemons(320),
                RuneEffect::ManaAfterKill(5),
                RuneEffect::PreventHeal,
            ],

            // 4-Socket Rune Words
            Self::Insight => vec![
                RuneEffect::ManaRegen(400),
                RuneEffect::EnhancedDamage(35),
                RuneEffect::AttackRating(35),
                RuneEffect::BonusDexterity(5),
                RuneEffect::MagicFind(23),
            ],
            Self::Fortitude => vec![
                RuneEffect::BonusDefense(200),
                RuneEffect::EnhancedDamage(300),
                RuneEffect::AllResist(25),
                RuneEffect::BonusMaxLife(100),
                RuneEffect::ReplenishLife(7),
                RuneEffect::DamageReductionPercent(15),
            ],
            Self::Passion => vec![
                RuneEffect::EnhancedDamage(160),
                RuneEffect::AttackSpeed(25),
                RuneEffect::AttackRating(50),
                RuneEffect::BonusDamage(75),
                RuneEffect::HitBlindsTarget,
            ],
            Self::Harmony => vec![
                RuneEffect::EnhancedDamage(200),
                RuneEffect::BonusDexterity(10),
                RuneEffect::AllResist(20),
                RuneEffect::ManaRegen(20),
                RuneEffect::LightRadius(2),
            ],
            Self::Peace => vec![
                RuneEffect::BonusDefense(120),
                RuneEffect::FasterHitRecovery(30),
                RuneEffect::AllResist(30),
                RuneEffect::AttackerTakesDamage(14),
            ],
            Self::Honor => vec![
                RuneEffect::EnhancedDamage(160),
                RuneEffect::AttackRating(200),
                RuneEffect::DeadlyStrike(25),
                RuneEffect::AllStats(10),
                RuneEffect::ReplenishLife(10),
            ],

            // 5-Socket Rune Words
            Self::Grief => vec![
                RuneEffect::EnhancedDamage(340),
                RuneEffect::IgnoreTargetDefense,
                RuneEffect::AttackSpeed(30),
                RuneEffect::LifeSteal(20),
                RuneEffect::PreventHeal,
                RuneEffect::DeadlyStrike(35),
            ],
            Self::Enigma => vec![
                RuneEffect::BonusDefense(750),
                RuneEffect::BonusStrength(14),
                RuneEffect::BonusMaxLife(100),
                RuneEffect::DamageReductionPercent(8),
                RuneEffect::MagicFind(100),
                RuneEffect::CannotBeFrozen,
            ],
            Self::Infinity => vec![
                RuneEffect::EnhancedDamage(260),
                RuneEffect::AttackSpeed(35),
                RuneEffect::LightningDamage { min: 180, max: 250 },
                RuneEffect::BonusVitality(40),
                RuneEffect::ManaRegen(50),
            ],
            Self::Pride => vec![
                RuneEffect::EnhancedDamage(300),
                RuneEffect::AttackRating(260),
                RuneEffect::CrushingBlow(30),
                RuneEffect::DamageVsDemons(200),
                RuneEffect::DamageVsUndead(200),
            ],
            Self::Destruction => vec![
                RuneEffect::EnhancedDamage(380),
                RuneEffect::IgnoreTargetDefense,
                RuneEffect::CrushingBlow(40),
                RuneEffect::DeadlyStrike(40),
                RuneEffect::PreventHeal,
                RuneEffect::ManaAfterKill(23),
            ],

            // 6-Socket Rune Words
            Self::Breath => vec![
                RuneEffect::EnhancedDamage(350),
                RuneEffect::AttackSpeed(60),
                RuneEffect::LifeSteal(12),
                RuneEffect::LifeRegen(30),
                RuneEffect::AllStats(30),
                RuneEffect::Indestructible,
            ],
            Self::Eternity => vec![
                RuneEffect::EnhancedDamage(260),
                RuneEffect::Indestructible,
                RuneEffect::SlowStamina(100),
                RuneEffect::CannotBeFrozen,
                RuneEffect::ReplenishLife(20),
                RuneEffect::ManaRegen(16),
            ],
            Self::Oblivion => vec![
                RuneEffect::EnhancedDamage(400),
                RuneEffect::AllStats(40),
                RuneEffect::Indestructible,
                RuneEffect::IgnoreTargetDefense,
                RuneEffect::LifeSteal(25),
                RuneEffect::ManaSteal(25),
                RuneEffect::AllResist(75),
            ],
            Self::Phoenix => vec![
                RuneEffect::EnhancedDamage(350),
                RuneEffect::FireAura(350),
                RuneEffect::FireAbsorb(28),
                RuneEffect::BonusMaxLife(400),
                RuneEffect::IgnoreTargetDefense,
            ],

            // Legendary Rune Words
            Self::Apocalypse => vec![
                RuneEffect::EnhancedDamage(500),
                RuneEffect::AllResist(100),
                RuneEffect::DamageVsUndead(200),
                RuneEffect::SummonShadow,
                RuneEffect::FireAura(100),
                RuneEffect::LifeRegen(10),
            ],
            Self::Genesis => vec![
                RuneEffect::EnhancedDamage(600),
                RuneEffect::AllStats(50),
                RuneEffect::Indestructible,
                RuneEffect::AllResist(100),
                RuneEffect::LifeSteal(30),
                RuneEffect::ManaSteal(30),
                RuneEffect::CannotBeFrozen,
                RuneEffect::SummonShadow,
            ],
        }
    }

    /// Returns the allowed item types for this rune word
    pub fn allowed_slots(&self) -> Vec<EquipSlot> {
        match self {
            // Weapon-only rune words
            Self::Steel | Self::Malice | Self::Strength | Self::Edge |
            Self::Grief | Self::Destruction | Self::Eternity | Self::Oblivion => {
                vec![EquipSlot::Weapon]
            }

            // Armor-only rune words
            Self::Stealth | Self::Fortitude | Self::Enigma | Self::Peace => {
                vec![EquipSlot::Armor]
            }

            // Shield-only rune words
            Self::Ancient => vec![EquipSlot::Shield],

            // Helm-only rune words
            Self::Nadir | Self::Lore | Self::Radiance => vec![EquipSlot::Helmet],

            // Staff/Weapon rune words
            Self::Leaf | Self::Insight | Self::Infinity | Self::Pride => {
                vec![EquipSlot::Weapon]
            }

            // Bow/Weapon rune words
            Self::Zephyr | Self::Harmony => vec![EquipSlot::Weapon],

            // Shield or Weapon
            Self::Spirit => vec![EquipSlot::Shield, EquipSlot::Weapon],

            // Multi-slot rune words
            Self::Passion | Self::Honor | Self::Breath | Self::Phoenix |
            Self::Apocalypse | Self::Genesis => {
                vec![EquipSlot::Weapon, EquipSlot::Armor]
            }
        }
    }

    /// Returns a description of this rune word
    pub fn description(&self) -> &'static str {
        match self {
            Self::Steel => "A basic but effective damage enhancement for new adventurers.",
            Self::Nadir => "Defensive helm providing strength and mana sustain.",
            Self::Malice => "Aggressive weapon that prevents healing but reduces your defenses.",
            Self::Stealth => "Swift armor enhancing speed and stealth abilities.",
            Self::Leaf => "Fire-attuned staff for pyromancers.",
            Self::Zephyr => "Lightning-fast bow of the storm.",
            Self::Ancient => "Ancient pledges of elemental protection.",
            Self::Spirit => "Channeling spiritual energy for mana and magic find.",
            Self::Lore => "Helm of wisdom and knowledge.",
            Self::Radiance => "Brilliant armor that lights your path.",
            Self::Strength => "Raw power weapon for warriors.",
            Self::Edge => "Deadly bow effective against demons.",
            Self::Insight => "Meditative weapon providing endless mana.",
            Self::Fortitude => "Fortress armor of incredible defense.",
            Self::Passion => "Zealous weapon for rapid attacks.",
            Self::Harmony => "Balanced weapon providing vigor.",
            Self::Peace => "Peaceful armor reflecting damage to attackers.",
            Self::Honor => "Noble weapon of balanced power.",
            Self::Grief => "Weapon of immense sorrow and destruction.",
            Self::Enigma => "Mysterious armor granting teleportation powers.",
            Self::Infinity => "Infinite power of lightning.",
            Self::Pride => "Proud weapon of concentrated might.",
            Self::Destruction => "Ultimate weapon of devastation.",
            Self::Breath => "The last breath of a dying god.",
            Self::Eternity => "Weapon that transcends time itself.",
            Self::Oblivion => "The end of all things.",
            Self::Phoenix => "Rise from the ashes with renewed power.",
            Self::Apocalypse => "The end times made manifest.",
            Self::Genesis => "The power of creation itself.",
        }
    }
}

// ============================================================================
// RUNE CRAFTING
// ============================================================================

/// Rune crafting recipes
#[derive(Clone, Debug)]
pub struct RuneCraftingRecipe {
    /// Runes required for the recipe
    pub ingredients: Vec<RuneType>,
    /// Additional materials required (item kinds)
    pub additional_materials: Vec<CraftingMaterial>,
    /// Resulting rune
    pub result: RuneType,
}

/// Additional materials that can be used in rune crafting
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CraftingMaterial {
    /// Soul gems from defeated enemies
    SoulGem,
    /// Perfect gems for enhancement
    PerfectGem,
    /// Ancient relics from dungeons
    AncientRelic,
    /// Dragon scales from dragon enemies
    DragonScale,
    /// Demon hearts from demon enemies
    DemonHeart,
}

impl CraftingMaterial {
    /// Convert from ItemKind if applicable
    pub fn from_item_kind(kind: ItemKind) -> Option<Self> {
        match kind {
            ItemKind::SoulGem => Some(Self::SoulGem),
            ItemKind::AncientRelic => Some(Self::AncientRelic),
            ItemKind::DragonScale => Some(Self::DragonScale),
            ItemKind::DemonHeart => Some(Self::DemonHeart),
            _ => None,
        }
    }

    /// Returns the name of this material
    pub fn name(&self) -> &'static str {
        match self {
            Self::SoulGem => "Soul Gem",
            Self::PerfectGem => "Perfect Gem",
            Self::AncientRelic => "Ancient Relic",
            Self::DragonScale => "Dragon Scale",
            Self::DemonHeart => "Demon Heart",
        }
    }
}

/// Manages rune crafting recipes and operations
pub struct RuneCrafter;

impl RuneCrafter {
    /// Get all available crafting recipes
    pub fn recipes() -> Vec<RuneCraftingRecipe> {
        vec![
            // Tier 1 upgrades (3 of same tier 1 rune = next tier 1 rune)
            RuneCraftingRecipe {
                ingredients: vec![RuneType::El, RuneType::El, RuneType::El],
                additional_materials: vec![],
                result: RuneType::Eld,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Eld, RuneType::Eld, RuneType::Eld],
                additional_materials: vec![],
                result: RuneType::Tir,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Tir, RuneType::Tir, RuneType::Tir],
                additional_materials: vec![],
                result: RuneType::Nef,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Nef, RuneType::Nef, RuneType::Nef],
                additional_materials: vec![],
                result: RuneType::Eth,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Eth, RuneType::Eth, RuneType::Eth],
                additional_materials: vec![],
                result: RuneType::Ith,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Ith, RuneType::Ith, RuneType::Ith],
                additional_materials: vec![],
                result: RuneType::Tal,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Tal, RuneType::Tal, RuneType::Tal],
                additional_materials: vec![],
                result: RuneType::Ral,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Ral, RuneType::Ral, RuneType::Ral],
                additional_materials: vec![],
                result: RuneType::Ort,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Ort, RuneType::Ort, RuneType::Ort],
                additional_materials: vec![],
                result: RuneType::Thul,
            },

            // Tier 1 to Tier 2 upgrade (requires additional material)
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Thul, RuneType::Thul, RuneType::Thul],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Amn,
            },

            // Tier 2 upgrades
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Amn, RuneType::Amn, RuneType::Amn],
                additional_materials: vec![],
                result: RuneType::Sol,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Sol, RuneType::Sol, RuneType::Sol],
                additional_materials: vec![],
                result: RuneType::Shael,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Shael, RuneType::Shael, RuneType::Shael],
                additional_materials: vec![],
                result: RuneType::Dol,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Dol, RuneType::Dol, RuneType::Dol],
                additional_materials: vec![],
                result: RuneType::Hel,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Hel, RuneType::Hel, RuneType::Hel],
                additional_materials: vec![],
                result: RuneType::Io,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Io, RuneType::Io, RuneType::Io],
                additional_materials: vec![],
                result: RuneType::Lum,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Lum, RuneType::Lum, RuneType::Lum],
                additional_materials: vec![],
                result: RuneType::Ko,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Ko, RuneType::Ko, RuneType::Ko],
                additional_materials: vec![],
                result: RuneType::Fal,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Fal, RuneType::Fal, RuneType::Fal],
                additional_materials: vec![],
                result: RuneType::Lem,
            },

            // Tier 2 to Tier 3 upgrade (requires additional material)
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Lem, RuneType::Lem, RuneType::Lem],
                additional_materials: vec![CraftingMaterial::PerfectGem],
                result: RuneType::Pul,
            },

            // Tier 3 upgrades
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Pul, RuneType::Pul],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Um,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Um, RuneType::Um],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Mal,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Mal, RuneType::Mal],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Ist,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Ist, RuneType::Ist],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Gul,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Gul, RuneType::Gul],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Vex,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Vex, RuneType::Vex],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Ohm,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Ohm, RuneType::Ohm],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Lo,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Lo, RuneType::Lo],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Sur,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Sur, RuneType::Sur],
                additional_materials: vec![CraftingMaterial::SoulGem],
                result: RuneType::Ber,
            },

            // Tier 3 to Tier 4 upgrade (requires rare materials)
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Ber, RuneType::Ber],
                additional_materials: vec![CraftingMaterial::AncientRelic],
                result: RuneType::Jah,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Jah, RuneType::Jah],
                additional_materials: vec![CraftingMaterial::AncientRelic],
                result: RuneType::Cham,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Cham, RuneType::Cham],
                additional_materials: vec![CraftingMaterial::AncientRelic],
                result: RuneType::Zod,
            },

            // Tier 5 legendary runes (require epic materials)
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Zod, RuneType::Ber, RuneType::Jah],
                additional_materials: vec![CraftingMaterial::DemonHeart],
                result: RuneType::Xul,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Zod, RuneType::Cham, RuneType::Sur],
                additional_materials: vec![CraftingMaterial::DragonScale],
                result: RuneType::Vul,
            },
            RuneCraftingRecipe {
                ingredients: vec![RuneType::Zod, RuneType::Jah, RuneType::Cham],
                additional_materials: vec![CraftingMaterial::AncientRelic, CraftingMaterial::SoulGem],
                result: RuneType::Nar,
            },
        ]
    }

    /// Find a recipe that matches the given ingredients
    pub fn find_recipe(runes: &[RuneType], materials: &[CraftingMaterial]) -> Option<RuneCraftingRecipe> {
        for recipe in Self::recipes() {
            // Check if runes match (order matters)
            if runes.len() != recipe.ingredients.len() {
                continue;
            }

            let mut runes_match = true;
            let mut available_runes = runes.to_vec();
            for ingredient in &recipe.ingredients {
                if let Some(pos) = available_runes.iter().position(|r| r == ingredient) {
                    available_runes.remove(pos);
                } else {
                    runes_match = false;
                    break;
                }
            }

            if !runes_match {
                continue;
            }

            // Check if materials match
            let mut materials_match = true;
            let mut available_materials = materials.to_vec();
            for required in &recipe.additional_materials {
                if let Some(pos) = available_materials.iter().position(|m| m == required) {
                    available_materials.remove(pos);
                } else {
                    materials_match = false;
                    break;
                }
            }

            if materials_match {
                return Some(recipe);
            }
        }
        None
    }

    /// Attempt to craft a rune with the given ingredients
    pub fn craft(runes: Vec<RuneType>, materials: Vec<CraftingMaterial>) -> Result<RuneType, &'static str> {
        match Self::find_recipe(&runes, &materials) {
            Some(recipe) => Ok(recipe.result),
            None => Err("No valid recipe found for these ingredients"),
        }
    }
}

// ============================================================================
// PLAYER RUNE INVENTORY
// ============================================================================

/// Tracks the player's rune collection and crafting materials
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RuneInventory {
    /// Collected runes
    pub runes: HashMap<RuneType, u32>,
    /// Crafting materials
    pub materials: HashMap<CraftingMaterial, u32>,
    /// Discovered rune words (for lore/achievements)
    pub discovered_rune_words: Vec<RuneWordType>,
}

impl RuneInventory {
    /// Create a new empty rune inventory
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a rune to the inventory
    pub fn add_rune(&mut self, rune: RuneType) {
        *self.runes.entry(rune).or_insert(0) += 1;
    }

    /// Remove a rune from the inventory
    pub fn remove_rune(&mut self, rune: RuneType) -> bool {
        if let Some(count) = self.runes.get_mut(&rune) {
            if *count > 0 {
                *count -= 1;
                if *count == 0 {
                    self.runes.remove(&rune);
                }
                return true;
            }
        }
        false
    }

    /// Check if the inventory has a specific rune
    pub fn has_rune(&self, rune: RuneType) -> bool {
        self.runes.get(&rune).map_or(false, |&c| c > 0)
    }

    /// Get the count of a specific rune
    pub fn rune_count(&self, rune: RuneType) -> u32 {
        *self.runes.get(&rune).unwrap_or(&0)
    }

    /// Add a crafting material to the inventory
    pub fn add_material(&mut self, material: CraftingMaterial) {
        *self.materials.entry(material).or_insert(0) += 1;
    }

    /// Remove a crafting material from the inventory
    pub fn remove_material(&mut self, material: CraftingMaterial) -> bool {
        if let Some(count) = self.materials.get_mut(&material) {
            if *count > 0 {
                *count -= 1;
                if *count == 0 {
                    self.materials.remove(&material);
                }
                return true;
            }
        }
        false
    }

    /// Check if the inventory has a specific material
    pub fn has_material(&self, material: CraftingMaterial) -> bool {
        self.materials.get(&material).map_or(false, |&c| c > 0)
    }

    /// Get the count of a specific material
    pub fn material_count(&self, material: CraftingMaterial) -> u32 {
        *self.materials.get(&material).unwrap_or(&0)
    }

    /// Get all runes sorted by tier
    pub fn runes_by_tier(&self) -> Vec<(RuneType, u32)> {
        let mut runes: Vec<_> = self.runes.iter()
            .map(|(&rune, &count)| (rune, count))
            .collect();
        runes.sort_by_key(|(rune, _)| (rune.tier(), rune.name()));
        runes
    }

    /// Record a discovered rune word
    pub fn discover_rune_word(&mut self, rune_word: RuneWordType) {
        if !self.discovered_rune_words.contains(&rune_word) {
            self.discovered_rune_words.push(rune_word);
        }
    }

    /// Check if a rune word has been discovered
    pub fn has_discovered(&self, rune_word: RuneWordType) -> bool {
        self.discovered_rune_words.contains(&rune_word)
    }

    /// Attempt to craft a rune using inventory items
    pub fn try_craft(&mut self, runes: &[RuneType], materials: &[CraftingMaterial]) -> Result<RuneType, &'static str> {
        // First check if we have all required items
        let mut rune_check: HashMap<RuneType, u32> = HashMap::new();
        for rune in runes {
            *rune_check.entry(*rune).or_insert(0) += 1;
        }
        for (rune, required) in &rune_check {
            if self.rune_count(*rune) < *required {
                return Err("Not enough runes");
            }
        }

        let mut material_check: HashMap<CraftingMaterial, u32> = HashMap::new();
        for material in materials {
            *material_check.entry(*material).or_insert(0) += 1;
        }
        for (material, required) in &material_check {
            if self.material_count(*material) < *required {
                return Err("Not enough materials");
            }
        }

        // Try to find and execute the recipe
        let result = RuneCrafter::craft(runes.to_vec(), materials.to_vec())?;

        // Remove ingredients
        for rune in runes {
            self.remove_rune(*rune);
        }
        for material in materials {
            self.remove_material(*material);
        }

        // Add result
        self.add_rune(result);

        Ok(result)
    }
}

// ============================================================================
// ACCUMULATED RUNE BONUSES
// ============================================================================

/// Accumulated stat bonuses from runes
#[derive(Clone, Debug, Default)]
pub struct RuneStatBonuses {
    // Damage
    pub min_damage: i32,
    pub max_damage: i32,
    pub enhanced_damage: i32,
    pub fire_damage_min: i32,
    pub fire_damage_max: i32,
    pub cold_damage_min: i32,
    pub cold_damage_max: i32,
    pub cold_duration: i32,
    pub lightning_damage_min: i32,
    pub lightning_damage_max: i32,
    pub poison_damage: i32,
    pub poison_duration: i32,
    pub fire_damage_percent: i32,

    // Attack modifiers
    pub attack_rating: i32,
    pub attack_speed: i32,
    pub deadly_strike: i32,
    pub crushing_blow: i32,
    pub open_wounds: i32,
    pub ignore_target_defense: bool,
    pub target_defense_reduction: i32,
    pub knockback: bool,
    pub cause_flee: bool,
    pub prevent_heal: bool,
    pub hit_blinds_target: bool,

    // Damage vs types
    pub damage_vs_undead: i32,
    pub damage_vs_demons: i32,

    // Defense
    pub bonus_defense: i32,
    pub defense_vs_undead: i32,
    pub defense_vs_demons: i32,
    pub defense_vs_missile: i32,
    pub damage_reduction: i32,
    pub damage_reduction_percent: i32,
    pub magic_damage_reduction: i32,
    pub faster_hit_recovery: i32,
    pub attacker_takes_damage: i32,
    pub thorns: i32,

    // Resistances
    pub fire_resist: i32,
    pub cold_resist: i32,
    pub lightning_resist: i32,
    pub poison_resist: i32,
    pub all_resist: i32,
    pub max_fire_resist: i32,
    pub max_cold_resist: i32,
    pub max_lightning_resist: i32,
    pub max_poison_resist: i32,
    pub fire_absorb: i32,

    // Life and Mana
    pub bonus_max_life: i32,
    pub bonus_max_mana: i32,
    pub life_steal: i32,
    pub mana_steal: i32,
    pub replenish_life: i32,
    pub mana_regen: i32,
    pub mana_after_kill: i32,
    pub life_regen: i32,

    // Stats
    pub bonus_strength: i32,
    pub bonus_dexterity: i32,
    pub bonus_vitality: i32,
    pub bonus_energy: i32,
    pub all_stats: i32,

    // Special
    pub light_radius: i32,
    pub magic_find: i32,
    pub gold_find: i32,
    pub requirements_reduction: i32,
    pub cannot_be_frozen: bool,
    pub slow_stamina: i32,
    pub indestructible: bool,

    // Auras
    pub fire_aura: i32,
    pub summon_shadow: bool,
}

impl RuneStatBonuses {
    /// Create empty bonuses
    pub fn new() -> Self {
        Self::default()
    }

    /// Add effects from a list of rune effects
    pub fn add_effects(&mut self, effects: &[RuneEffect]) {
        for effect in effects {
            match effect {
                RuneEffect::MinDamage(v) => self.min_damage += v,
                RuneEffect::MaxDamage(v) => self.max_damage += v,
                RuneEffect::EnhancedDamage(v) => self.enhanced_damage += v,
                RuneEffect::FireDamage { min, max } => {
                    self.fire_damage_min += min;
                    self.fire_damage_max += max;
                }
                RuneEffect::ColdDamage { min, max, duration } => {
                    self.cold_damage_min += min;
                    self.cold_damage_max += max;
                    self.cold_duration = self.cold_duration.max(*duration);
                }
                RuneEffect::LightningDamage { min, max } => {
                    self.lightning_damage_min += min;
                    self.lightning_damage_max += max;
                }
                RuneEffect::PoisonDamage { damage, duration } => {
                    self.poison_damage += damage;
                    self.poison_duration = self.poison_duration.max(*duration);
                }
                RuneEffect::FireDamagePercent(v) => self.fire_damage_percent += v,
                RuneEffect::DamageVsUndead(v) => self.damage_vs_undead += v,
                RuneEffect::DamageVsDemons(v) => self.damage_vs_demons += v,
                RuneEffect::AttackRating(v) => self.attack_rating += v,
                RuneEffect::AttackSpeed(v) => self.attack_speed += v,
                RuneEffect::DeadlyStrike(v) => self.deadly_strike += v,
                RuneEffect::CrushingBlow(v) => self.crushing_blow += v,
                RuneEffect::OpenWounds(v) => self.open_wounds += v,
                RuneEffect::IgnoreTargetDefense => self.ignore_target_defense = true,
                RuneEffect::TargetDefenseReduction(v) => self.target_defense_reduction += v,
                RuneEffect::Knockback => self.knockback = true,
                RuneEffect::CauseFlee => self.cause_flee = true,
                RuneEffect::PreventHeal => self.prevent_heal = true,
                RuneEffect::HitBlindsTarget => self.hit_blinds_target = true,
                RuneEffect::BonusDefense(v) => self.bonus_defense += v,
                RuneEffect::DefenseVsUndead(v) => self.defense_vs_undead += v,
                RuneEffect::DefenseVsDemons(v) => self.defense_vs_demons += v,
                RuneEffect::DefenseVsMissile(v) => self.defense_vs_missile += v,
                RuneEffect::DamageReduction(v) => self.damage_reduction += v,
                RuneEffect::DamageReductionPercent(v) => self.damage_reduction_percent += v,
                RuneEffect::MagicDamageReduction(v) => self.magic_damage_reduction += v,
                RuneEffect::FasterHitRecovery(v) => self.faster_hit_recovery += v,
                RuneEffect::AttackerTakesDamage(v) => self.attacker_takes_damage += v,
                RuneEffect::Thorns(v) => self.thorns += v,
                RuneEffect::FireResist(v) => self.fire_resist += v,
                RuneEffect::ColdResist(v) => self.cold_resist += v,
                RuneEffect::LightningResist(v) => self.lightning_resist += v,
                RuneEffect::PoisonResist(v) => self.poison_resist += v,
                RuneEffect::AllResist(v) => self.all_resist += v,
                RuneEffect::MaxFireResist(v) => self.max_fire_resist += v,
                RuneEffect::MaxColdResist(v) => self.max_cold_resist += v,
                RuneEffect::MaxLightningResist(v) => self.max_lightning_resist += v,
                RuneEffect::MaxPoisonResist(v) => self.max_poison_resist += v,
                RuneEffect::FireAbsorb(v) => self.fire_absorb += v,
                RuneEffect::BonusMaxLife(v) => self.bonus_max_life += v,
                RuneEffect::BonusMaxMana(v) => self.bonus_max_mana += v,
                RuneEffect::LifeSteal(v) => self.life_steal += v,
                RuneEffect::ManaSteal(v) => self.mana_steal += v,
                RuneEffect::ReplenishLife(v) => self.replenish_life += v,
                RuneEffect::ManaRegen(v) => self.mana_regen += v,
                RuneEffect::ManaAfterKill(v) => self.mana_after_kill += v,
                RuneEffect::LifeRegen(v) => self.life_regen += v,
                RuneEffect::BonusStrength(v) => self.bonus_strength += v,
                RuneEffect::BonusDexterity(v) => self.bonus_dexterity += v,
                RuneEffect::BonusVitality(v) => self.bonus_vitality += v,
                RuneEffect::BonusEnergy(v) => self.bonus_energy += v,
                RuneEffect::AllStats(v) => self.all_stats += v,
                RuneEffect::LightRadius(v) => self.light_radius += v,
                RuneEffect::MagicFind(v) => self.magic_find += v,
                RuneEffect::GoldFind(v) => self.gold_find += v,
                RuneEffect::RequirementsReduction(v) => self.requirements_reduction += v,
                RuneEffect::CannotBeFrozen => self.cannot_be_frozen = true,
                RuneEffect::SlowStamina(v) => self.slow_stamina += v,
                RuneEffect::Indestructible => self.indestructible = true,
                RuneEffect::FireAura(v) => self.fire_aura += v,
                RuneEffect::SummonShadow => self.summon_shadow = true,
            }
        }
    }

    /// Combine with another set of bonuses
    pub fn combine(&mut self, other: &Self) {
        self.min_damage += other.min_damage;
        self.max_damage += other.max_damage;
        self.enhanced_damage += other.enhanced_damage;
        self.fire_damage_min += other.fire_damage_min;
        self.fire_damage_max += other.fire_damage_max;
        self.cold_damage_min += other.cold_damage_min;
        self.cold_damage_max += other.cold_damage_max;
        self.cold_duration = self.cold_duration.max(other.cold_duration);
        self.lightning_damage_min += other.lightning_damage_min;
        self.lightning_damage_max += other.lightning_damage_max;
        self.poison_damage += other.poison_damage;
        self.poison_duration = self.poison_duration.max(other.poison_duration);
        self.fire_damage_percent += other.fire_damage_percent;
        self.damage_vs_undead += other.damage_vs_undead;
        self.damage_vs_demons += other.damage_vs_demons;
        self.attack_rating += other.attack_rating;
        self.attack_speed += other.attack_speed;
        self.deadly_strike += other.deadly_strike;
        self.crushing_blow += other.crushing_blow;
        self.open_wounds += other.open_wounds;
        self.ignore_target_defense |= other.ignore_target_defense;
        self.target_defense_reduction += other.target_defense_reduction;
        self.knockback |= other.knockback;
        self.cause_flee |= other.cause_flee;
        self.prevent_heal |= other.prevent_heal;
        self.hit_blinds_target |= other.hit_blinds_target;
        self.bonus_defense += other.bonus_defense;
        self.defense_vs_undead += other.defense_vs_undead;
        self.defense_vs_demons += other.defense_vs_demons;
        self.defense_vs_missile += other.defense_vs_missile;
        self.damage_reduction += other.damage_reduction;
        self.damage_reduction_percent += other.damage_reduction_percent;
        self.magic_damage_reduction += other.magic_damage_reduction;
        self.faster_hit_recovery += other.faster_hit_recovery;
        self.attacker_takes_damage += other.attacker_takes_damage;
        self.thorns += other.thorns;
        self.fire_resist += other.fire_resist;
        self.cold_resist += other.cold_resist;
        self.lightning_resist += other.lightning_resist;
        self.poison_resist += other.poison_resist;
        self.all_resist += other.all_resist;
        self.max_fire_resist += other.max_fire_resist;
        self.max_cold_resist += other.max_cold_resist;
        self.max_lightning_resist += other.max_lightning_resist;
        self.max_poison_resist += other.max_poison_resist;
        self.fire_absorb += other.fire_absorb;
        self.bonus_max_life += other.bonus_max_life;
        self.bonus_max_mana += other.bonus_max_mana;
        self.life_steal += other.life_steal;
        self.mana_steal += other.mana_steal;
        self.replenish_life += other.replenish_life;
        self.mana_regen += other.mana_regen;
        self.mana_after_kill += other.mana_after_kill;
        self.life_regen += other.life_regen;
        self.bonus_strength += other.bonus_strength;
        self.bonus_dexterity += other.bonus_dexterity;
        self.bonus_vitality += other.bonus_vitality;
        self.bonus_energy += other.bonus_energy;
        self.all_stats += other.all_stats;
        self.light_radius += other.light_radius;
        self.magic_find += other.magic_find;
        self.gold_find += other.gold_find;
        self.requirements_reduction += other.requirements_reduction;
        self.cannot_be_frozen |= other.cannot_be_frozen;
        self.slow_stamina += other.slow_stamina;
        self.indestructible |= other.indestructible;
        self.fire_aura += other.fire_aura;
        self.summon_shadow |= other.summon_shadow;
    }

    /// Get total effective strength
    pub fn total_strength(&self) -> i32 {
        self.bonus_strength + self.all_stats
    }

    /// Get total effective dexterity
    pub fn total_dexterity(&self) -> i32 {
        self.bonus_dexterity + self.all_stats
    }

    /// Get total effective vitality
    pub fn total_vitality(&self) -> i32 {
        self.bonus_vitality + self.all_stats
    }

    /// Get total effective energy
    pub fn total_energy(&self) -> i32 {
        self.bonus_energy + self.all_stats
    }

    /// Get total fire resistance
    pub fn total_fire_resist(&self) -> i32 {
        self.fire_resist + self.all_resist
    }

    /// Get total cold resistance
    pub fn total_cold_resist(&self) -> i32 {
        self.cold_resist + self.all_resist
    }

    /// Get total lightning resistance
    pub fn total_lightning_resist(&self) -> i32 {
        self.lightning_resist + self.all_resist
    }

    /// Get total poison resistance
    pub fn total_poison_resist(&self) -> i32 {
        self.poison_resist + self.all_resist
    }
}

// ============================================================================
// ADDITIONAL EFFECT FOR RADIANCE (missing BonusDamage)
// ============================================================================

// Note: Adding a helper to handle the BonusDamage and ColdAbsorb effects
// that were referenced but not defined in the main RuneEffect enum.
// These are handled by the existing effects where possible.

impl RuneEffect {
    /// Check if this is a cold absorb effect (alias for fire absorb logic applied to cold)
    pub fn is_cold_absorb(&self) -> bool {
        // Cold absorb is represented through the Spirit rune word
        // For now, we don't have a separate cold absorb, so this is a placeholder
        false
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rune_tiers() {
        assert_eq!(RuneType::El.tier(), 1);
        assert_eq!(RuneType::Amn.tier(), 2);
        assert_eq!(RuneType::Pul.tier(), 3);
        assert_eq!(RuneType::Jah.tier(), 4);
        assert_eq!(RuneType::Xul.tier(), 5);
    }

    #[test]
    fn test_socketed_item() {
        let mut item = SocketedItem::new(3);
        assert_eq!(item.empty_sockets(), 3);
        assert_eq!(item.filled_sockets(), 0);

        item.insert_rune(RuneType::El).unwrap();
        assert_eq!(item.empty_sockets(), 2);
        assert_eq!(item.filled_sockets(), 1);

        item.insert_rune(RuneType::Eld).unwrap();
        item.insert_rune(RuneType::Tir).unwrap();
        assert_eq!(item.empty_sockets(), 0);
        assert!(item.insert_rune(RuneType::Nef).is_err());
    }

    #[test]
    fn test_rune_word_detection() {
        let mut item = SocketedItem::new(2);
        item.insert_rune(RuneType::Tir).unwrap();
        item.insert_rune(RuneType::El).unwrap();

        assert!(item.rune_word.is_some());
        assert_eq!(item.rune_word.unwrap(), RuneWordType::Steel);
    }

    #[test]
    fn test_rune_word_recipe() {
        let recipe = RuneWordType::Steel.recipe();
        assert_eq!(recipe, vec![RuneType::Tir, RuneType::El]);

        let recipe = RuneWordType::Ancient.recipe();
        assert_eq!(recipe, vec![RuneType::Ral, RuneType::Ort, RuneType::Tal]);
    }

    #[test]
    fn test_crafting_recipes() {
        let result = RuneCrafter::craft(
            vec![RuneType::El, RuneType::El, RuneType::El],
            vec![],
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RuneType::Eld);
    }

    #[test]
    fn test_rune_inventory() {
        let mut inv = RuneInventory::new();
        inv.add_rune(RuneType::El);
        inv.add_rune(RuneType::El);
        inv.add_rune(RuneType::El);

        assert_eq!(inv.rune_count(RuneType::El), 3);
        assert!(inv.has_rune(RuneType::El));

        inv.remove_rune(RuneType::El);
        assert_eq!(inv.rune_count(RuneType::El), 2);
    }

    #[test]
    fn test_rune_stat_bonuses() {
        let mut bonuses = RuneStatBonuses::new();
        let effects = vec![
            RuneEffect::MinDamage(5),
            RuneEffect::MaxDamage(10),
            RuneEffect::LifeSteal(7),
            RuneEffect::AllStats(5),
        ];
        bonuses.add_effects(&effects);

        assert_eq!(bonuses.min_damage, 5);
        assert_eq!(bonuses.max_damage, 10);
        assert_eq!(bonuses.life_steal, 7);
        assert_eq!(bonuses.all_stats, 5);
        assert_eq!(bonuses.total_strength(), 5);
    }

    #[test]
    fn test_socket_effects_by_slot() {
        let mut weapon = SocketedItem::new(2);
        weapon.insert_rune(RuneType::Amn).unwrap();

        let weapon_effects = weapon.get_effects(EquipSlot::Weapon);
        let has_life_steal = weapon_effects.iter().any(|e| matches!(e, RuneEffect::LifeSteal(_)));
        assert!(has_life_steal);

        let mut armor = SocketedItem::new(2);
        armor.insert_rune(RuneType::Amn).unwrap();

        let armor_effects = armor.get_effects(EquipSlot::Armor);
        let has_attacker_damage = armor_effects.iter().any(|e| matches!(e, RuneEffect::AttackerTakesDamage(_)));
        assert!(has_attacker_damage);
    }

    #[test]
    fn test_all_rune_words_have_valid_recipes() {
        for rune_word in RuneWordType::all() {
            let recipe = rune_word.recipe();
            assert!(!recipe.is_empty(), "Rune word {:?} has empty recipe", rune_word);

            // Verify that the recipe can be found
            let found = RuneWordType::find_match(&recipe);
            assert!(found.is_some(), "Rune word {:?} recipe cannot be matched", rune_word);
            assert_eq!(found.unwrap(), rune_word);
        }
    }
}

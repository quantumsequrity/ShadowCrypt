//! Enchantment system: item enchantments, enchanting mechanics, and enchantment scrolls
//!
//! This module provides a comprehensive enchantment system for ShadowCrypt, allowing
//! players to imbue their equipment with magical properties.
//!
//! # Features
//!
//! - Multiple enchantment types (elemental, vampiric, defensive, etc.)
//! - Tiered enchantment power levels
//! - Enchantment scrolls for applying enchantments
//! - Success/failure mechanics with risk
//! - Enchantment stacking and combination rules

use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::combat::StatusEffect;
use crate::items::{EquipSlot, Item, ItemKind, Rarity};

/// The type of enchantment that can be applied to items
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EnchantmentType {
    // Elemental Enchantments
    /// Fire damage: Burns enemies for damage over time
    Fire,
    /// Ice damage: Chance to freeze enemies, slowing them
    Ice,
    /// Lightning damage: Chain lightning on critical hits
    Lightning,
    /// Poison damage: Inflicts poison that deals damage over time
    Poison,

    // Life & Sustain Enchantments
    /// Life Steal: Heal a percentage of damage dealt
    LifeSteal,
    /// Mana Steal: Restore mana on hit
    ManaSteal,
    /// Regeneration: Passive health regeneration while equipped
    Regeneration,
    /// Vitality: Increases maximum HP
    Vitality,

    // Offensive Enchantments
    /// Sharpness: Increases base attack damage
    Sharpness,
    /// Critical: Increases critical hit chance and damage
    Critical,
    /// Armor Pierce: Ignores a percentage of enemy defense
    ArmorPierce,
    /// Bane of Undead: Extra damage against undead enemies
    BaneUndead,
    /// Bane of Demons: Extra damage against demon enemies
    BaneDemons,
    /// Execute: Deal bonus damage to low-health enemies
    Execute,

    // Defensive Enchantments
    /// Protection: Increases defense
    Protection,
    /// Thorns: Reflect damage back to attackers
    Thorns,
    /// Evasion: Chance to completely dodge attacks
    Evasion,
    /// Absorption: Convert a portion of damage to mana
    Absorption,
    /// Fortification: Reduce critical damage taken
    Fortification,

    // Utility Enchantments
    /// Swiftness: Increases movement speed
    Swiftness,
    /// Fortune: Increases gold and item drop rates
    Fortune,
    /// Experience: Bonus XP from kills
    Experience,
    /// Light: Increases vision radius
    Light,
    /// Featherfall: Reduces trap damage
    Featherfall,

    // Special/Rare Enchantments
    /// Vampiric: Powerful life steal that also grants temporary strength
    Vampiric,
    /// Holy: Bonus damage and healing, extra effective against undead
    Holy,
    /// Shadow: Grants invisibility chance on kill
    Shadow,
    /// Berserker: Damage increases as HP decreases
    Berserker,
    /// Soulbound: Cannot be dropped, but grants stat bonuses
    Soulbound,
    /// Chaos: Random elemental damage each hit
    Chaos,
}

impl EnchantmentType {
    /// Returns the display name of the enchantment
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fire => "Fire",
            Self::Ice => "Ice",
            Self::Lightning => "Lightning",
            Self::Poison => "Poison",
            Self::LifeSteal => "Life Steal",
            Self::ManaSteal => "Mana Steal",
            Self::Regeneration => "Regeneration",
            Self::Vitality => "Vitality",
            Self::Sharpness => "Sharpness",
            Self::Critical => "Critical",
            Self::ArmorPierce => "Armor Piercing",
            Self::BaneUndead => "Bane of Undead",
            Self::BaneDemons => "Bane of Demons",
            Self::Execute => "Execute",
            Self::Protection => "Protection",
            Self::Thorns => "Thorns",
            Self::Evasion => "Evasion",
            Self::Absorption => "Absorption",
            Self::Fortification => "Fortification",
            Self::Swiftness => "Swiftness",
            Self::Fortune => "Fortune",
            Self::Experience => "Experience",
            Self::Light => "Light",
            Self::Featherfall => "Featherfall",
            Self::Vampiric => "Vampiric",
            Self::Holy => "Holy",
            Self::Shadow => "Shadow",
            Self::Berserker => "Berserker",
            Self::Soulbound => "Soulbound",
            Self::Chaos => "Chaos",
        }
    }

    /// Returns a color index for the enchantment (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Fire => 3,         // Red
            Self::Ice => 9,          // Cyan
            Self::Lightning => 11,   // Yellow
            Self::Poison => 5,       // Green
            Self::LifeSteal => 4,    // DarkRed
            Self::ManaSteal => 7,    // Blue
            Self::Regeneration => 13, // Magenta
            Self::Vitality => 3,     // Red
            Self::Sharpness => 2,    // White
            Self::Critical => 11,    // Yellow
            Self::ArmorPierce => 1,  // Grey
            Self::BaneUndead => 2,   // White
            Self::BaneDemons => 3,   // Red
            Self::Execute => 4,      // DarkRed
            Self::Protection => 7,   // Blue
            Self::Thorns => 6,       // DarkGreen
            Self::Evasion => 9,      // Cyan
            Self::Absorption => 8,   // DarkBlue
            Self::Fortification => 1, // Grey
            Self::Swiftness => 9,    // Cyan
            Self::Fortune => 11,     // Yellow
            Self::Experience => 5,   // Green
            Self::Light => 11,       // Yellow
            Self::Featherfall => 2,  // White
            Self::Vampiric => 4,     // DarkRed
            Self::Holy => 11,        // Yellow
            Self::Shadow => 14,      // DarkMagenta
            Self::Berserker => 3,    // Red
            Self::Soulbound => 13,   // Magenta
            Self::Chaos => 13,       // Magenta
        }
    }

    /// Returns whether this enchantment is compatible with a given equipment slot
    pub fn compatible_with_slot(&self, slot: EquipSlot) -> bool {
        match self {
            // Weapon-only enchantments
            Self::Fire | Self::Ice | Self::Lightning | Self::Poison
            | Self::LifeSteal | Self::ManaSteal | Self::Sharpness
            | Self::Critical | Self::ArmorPierce | Self::BaneUndead
            | Self::BaneDemons | Self::Execute | Self::Vampiric
            | Self::Chaos => matches!(slot, EquipSlot::Weapon),

            // Armor/defensive enchantments
            Self::Protection | Self::Thorns | Self::Evasion
            | Self::Absorption | Self::Fortification => matches!(
                slot,
                EquipSlot::Armor | EquipSlot::Shield | EquipSlot::Helmet
                | EquipSlot::Gloves | EquipSlot::Boots
            ),

            // Universal enchantments (can go on any equipment)
            Self::Regeneration | Self::Vitality | Self::Swiftness
            | Self::Fortune | Self::Experience | Self::Light
            | Self::Featherfall | Self::Holy | Self::Shadow
            | Self::Berserker | Self::Soulbound => true,
        }
    }

    /// Returns conflicting enchantment types (cannot be on same item)
    pub fn conflicts_with(&self) -> &'static [EnchantmentType] {
        match self {
            // Elemental conflicts
            Self::Fire => &[Self::Ice],
            Self::Ice => &[Self::Fire],

            // Life enchantment conflicts
            Self::LifeSteal => &[Self::Vampiric],
            Self::Vampiric => &[Self::LifeSteal],

            // Alignment conflicts
            Self::Holy => &[Self::Shadow, Self::Vampiric, Self::BaneDemons],
            Self::Shadow => &[Self::Holy, Self::Light],

            // Chaos is incompatible with specific elements
            Self::Chaos => &[Self::Fire, Self::Ice, Self::Lightning, Self::Poison],

            _ => &[],
        }
    }

    /// Returns the base rarity/drop weight of this enchantment
    pub fn rarity_weight(&self) -> u32 {
        match self {
            // Common enchantments
            Self::Sharpness | Self::Protection | Self::Vitality
            | Self::Light | Self::Featherfall => 100,

            // Uncommon enchantments
            Self::Fire | Self::Ice | Self::Poison | Self::Regeneration
            | Self::Critical | Self::Thorns | Self::Swiftness
            | Self::Fortune => 60,

            // Rare enchantments
            Self::Lightning | Self::LifeSteal | Self::ManaSteal
            | Self::ArmorPierce | Self::Evasion | Self::Absorption
            | Self::Experience | Self::BaneUndead => 30,

            // Epic enchantments
            Self::BaneDemons | Self::Execute | Self::Fortification
            | Self::Holy | Self::Shadow | Self::Berserker => 15,

            // Legendary enchantments
            Self::Vampiric | Self::Chaos | Self::Soulbound => 5,
        }
    }

    /// Returns all available enchantment types
    pub fn all() -> &'static [EnchantmentType] {
        &[
            Self::Fire, Self::Ice, Self::Lightning, Self::Poison,
            Self::LifeSteal, Self::ManaSteal, Self::Regeneration, Self::Vitality,
            Self::Sharpness, Self::Critical, Self::ArmorPierce, Self::BaneUndead,
            Self::BaneDemons, Self::Execute, Self::Protection, Self::Thorns,
            Self::Evasion, Self::Absorption, Self::Fortification, Self::Swiftness,
            Self::Fortune, Self::Experience, Self::Light, Self::Featherfall,
            Self::Vampiric, Self::Holy, Self::Shadow, Self::Berserker,
            Self::Soulbound, Self::Chaos,
        ]
    }
}

/// The tier/power level of an enchantment
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum EnchantmentTier {
    /// Minor: Weakest enchantment level
    Minor,
    /// Standard: Normal enchantment power
    Standard,
    /// Greater: Enhanced enchantment effects
    Greater,
    /// Superior: Powerful enchantment effects
    Superior,
    /// Master: Maximum enchantment power
    Master,
}

impl EnchantmentTier {
    /// Returns the display name of the tier
    pub fn name(&self) -> &'static str {
        match self {
            Self::Minor => "Minor",
            Self::Standard => "",
            Self::Greater => "Greater",
            Self::Superior => "Superior",
            Self::Master => "Master",
        }
    }

    /// Returns the prefix for item naming
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Minor => "Minor ",
            Self::Standard => "",
            Self::Greater => "Greater ",
            Self::Superior => "Superior ",
            Self::Master => "Master ",
        }
    }

    /// Returns the effect multiplier for this tier
    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Minor => 0.5,
            Self::Standard => 1.0,
            Self::Greater => 1.5,
            Self::Superior => 2.0,
            Self::Master => 3.0,
        }
    }

    /// Returns the numeric tier level (1-5)
    pub fn level(&self) -> u32 {
        match self {
            Self::Minor => 1,
            Self::Standard => 2,
            Self::Greater => 3,
            Self::Superior => 4,
            Self::Master => 5,
        }
    }

    /// Creates a tier from a numeric level
    pub fn from_level(level: u32) -> Self {
        match level {
            0 | 1 => Self::Minor,
            2 => Self::Standard,
            3 => Self::Greater,
            4 => Self::Superior,
            _ => Self::Master,
        }
    }

    /// Returns the required player level to use scrolls of this tier
    pub fn required_level(&self) -> u32 {
        match self {
            Self::Minor => 1,
            Self::Standard => 5,
            Self::Greater => 10,
            Self::Superior => 18,
            Self::Master => 25,
        }
    }
}

/// An enchantment instance with type and tier
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Enchantment {
    pub enchant_type: EnchantmentType,
    pub tier: EnchantmentTier,
}

impl Enchantment {
    /// Create a new enchantment
    pub fn new(enchant_type: EnchantmentType, tier: EnchantmentTier) -> Self {
        Self { enchant_type, tier }
    }

    /// Returns the full display name of the enchantment
    pub fn display_name(&self) -> String {
        let tier_prefix = self.tier.prefix();
        let type_name = self.enchant_type.name();
        format!("{}{}", tier_prefix, type_name)
    }

    /// Calculate flat attack bonus from this enchantment
    pub fn attack_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Sharpness => 3,
            EnchantmentType::Fire | EnchantmentType::Ice
            | EnchantmentType::Lightning | EnchantmentType::Poison => 2,
            EnchantmentType::BaneUndead | EnchantmentType::BaneDemons => 0, // Conditional
            EnchantmentType::Berserker => 1, // Base, increases with low HP
            EnchantmentType::Chaos => 2,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate flat defense bonus from this enchantment
    pub fn defense_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Protection => 3,
            EnchantmentType::Fortification => 2,
            EnchantmentType::Absorption => 1,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate max HP bonus from this enchantment
    pub fn hp_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Vitality => 15,
            EnchantmentType::Soulbound => 10,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate max mana bonus from this enchantment
    pub fn mana_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Absorption => 10,
            EnchantmentType::ManaSteal => 5,
            EnchantmentType::Soulbound => 5,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate life steal percentage (0-100)
    pub fn life_steal_percent(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::LifeSteal => 8,
            EnchantmentType::Vampiric => 15,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate mana steal amount per hit
    pub fn mana_steal_amount(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::ManaSteal => 3,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate elemental/bonus damage
    pub fn elemental_damage(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Fire => 5,
            EnchantmentType::Ice => 4,
            EnchantmentType::Lightning => 6,
            EnchantmentType::Poison => 3,
            EnchantmentType::Holy => 5,
            EnchantmentType::Chaos => 7,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate critical hit bonus percentage
    pub fn crit_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Critical => 10,
            EnchantmentType::Sharpness => 3,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate armor pierce percentage
    pub fn armor_pierce_percent(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::ArmorPierce => 15,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate thorns damage reflection percentage
    pub fn thorns_percent(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Thorns => 20,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate evasion chance percentage
    pub fn evasion_percent(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Evasion => 5,
            EnchantmentType::Shadow => 3,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate gold/drop bonus percentage
    pub fn fortune_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Fortune => 15,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate XP bonus percentage
    pub fn xp_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Experience => 10,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate vision radius bonus
    pub fn vision_bonus(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Light => 2,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate health regeneration per turn
    pub fn regen_per_turn(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Regeneration => 1,
            EnchantmentType::Holy => 1,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Returns the status effect this enchantment can proc, if any
    pub fn proc_status_effect(&self) -> Option<StatusEffect> {
        match self.enchant_type {
            EnchantmentType::Fire => Some(StatusEffect::Burn),
            EnchantmentType::Ice => Some(StatusEffect::Freeze),
            EnchantmentType::Poison => Some(StatusEffect::Poison),
            EnchantmentType::Lightning => Some(StatusEffect::Stun),
            EnchantmentType::Holy => Some(StatusEffect::Weakness),
            EnchantmentType::Shadow => Some(StatusEffect::Invisibility),
            EnchantmentType::Vampiric => Some(StatusEffect::Strength),
            _ => None,
        }
    }

    /// Returns the proc chance percentage for status effects
    pub fn proc_chance(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Fire | EnchantmentType::Ice | EnchantmentType::Poison => 20,
            EnchantmentType::Lightning => 15,
            EnchantmentType::Holy => 10,
            EnchantmentType::Shadow => 8,
            EnchantmentType::Vampiric => 25,
            EnchantmentType::Chaos => 30,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier().min(1.5)) as i32 // Cap proc chance scaling
    }

    /// Calculate bonus damage against undead
    pub fn undead_bonus_damage(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::BaneUndead => 10,
            EnchantmentType::Holy => 8,
            EnchantmentType::Fire => 3,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate bonus damage against demons
    pub fn demon_bonus_damage(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::BaneDemons => 10,
            EnchantmentType::Holy => 5,
            EnchantmentType::Ice => 2,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier()) as i32
    }

    /// Calculate execute threshold (enemy HP percentage for bonus damage)
    pub fn execute_threshold(&self) -> i32 {
        let base = match self.enchant_type {
            EnchantmentType::Execute => 20,
            _ => 0,
        };
        (base as f32 * self.tier.multiplier().min(2.0)) as i32 // Cap at 40%
    }

    /// Calculate execute bonus damage multiplier
    pub fn execute_multiplier(&self) -> f32 {
        match self.enchant_type {
            EnchantmentType::Execute => 1.0 + (0.5 * self.tier.multiplier()),
            _ => 1.0,
        }
    }
}

/// Types of enchantment scrolls
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EnchantmentScrollType {
    /// Random enchantment scroll - applies a random compatible enchantment
    Random,
    /// Specific enchantment scroll - applies a specific enchantment type
    Specific(EnchantmentType),
    /// Upgrade scroll - increases the tier of an existing enchantment
    Upgrade,
    /// Removal scroll - removes an enchantment from an item
    Removal,
    /// Transfer scroll - moves an enchantment from one item to another
    Transfer,
    /// Blessed scroll - higher success rate, no destruction on failure
    Blessed,
    /// Cursed scroll - high power but may curse the item
    Cursed,
}

impl EnchantmentScrollType {
    /// Returns the display name of the scroll type
    pub fn name(&self) -> String {
        match self {
            Self::Random => "Scroll of Enchantment".to_string(),
            Self::Specific(ench_type) => format!("Scroll of {}", ench_type.name()),
            Self::Upgrade => "Scroll of Enhancement".to_string(),
            Self::Removal => "Scroll of Disenchantment".to_string(),
            Self::Transfer => "Scroll of Transference".to_string(),
            Self::Blessed => "Blessed Scroll of Enchantment".to_string(),
            Self::Cursed => "Cursed Scroll of Power".to_string(),
        }
    }
}

/// An enchantment scroll item
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnchantmentScroll {
    pub scroll_type: EnchantmentScrollType,
    pub tier: EnchantmentTier,
    pub charges: u32,
}

impl EnchantmentScroll {
    /// Create a new enchantment scroll
    pub fn new(scroll_type: EnchantmentScrollType, tier: EnchantmentTier) -> Self {
        Self {
            scroll_type,
            tier,
            charges: 1,
        }
    }

    /// Create a scroll with multiple charges
    pub fn with_charges(scroll_type: EnchantmentScrollType, tier: EnchantmentTier, charges: u32) -> Self {
        Self {
            scroll_type,
            tier,
            charges,
        }
    }

    /// Returns the full display name of the scroll
    pub fn display_name(&self) -> String {
        let tier_prefix = self.tier.prefix();
        let base_name = self.scroll_type.name();
        if self.charges > 1 {
            format!("{}{} ({})", tier_prefix, base_name, self.charges)
        } else {
            format!("{}{}", tier_prefix, base_name)
        }
    }

    /// Returns the base success rate (percentage) for this scroll
    pub fn base_success_rate(&self) -> i32 {
        let base = match self.scroll_type {
            EnchantmentScrollType::Random => 75,
            EnchantmentScrollType::Specific(_) => 85,
            EnchantmentScrollType::Upgrade => 60,
            EnchantmentScrollType::Removal => 95,
            EnchantmentScrollType::Transfer => 50,
            EnchantmentScrollType::Blessed => 95,
            EnchantmentScrollType::Cursed => 100, // Always succeeds, but...
        };

        // Higher tier scrolls are harder to use successfully
        let tier_penalty = match self.tier {
            EnchantmentTier::Minor => 5,
            EnchantmentTier::Standard => 0,
            EnchantmentTier::Greater => -5,
            EnchantmentTier::Superior => -10,
            EnchantmentTier::Master => -20,
        };

        (base + tier_penalty).max(10).min(99)
    }

    /// Returns whether this scroll can destroy the item on failure
    pub fn can_destroy_on_failure(&self) -> bool {
        match self.scroll_type {
            EnchantmentScrollType::Blessed => false,
            EnchantmentScrollType::Removal => false,
            EnchantmentScrollType::Transfer => true,
            EnchantmentScrollType::Cursed => true,
            _ => self.tier >= EnchantmentTier::Greater,
        }
    }

    /// Returns the destruction chance on failure (percentage)
    pub fn destruction_chance(&self) -> i32 {
        if !self.can_destroy_on_failure() {
            return 0;
        }
        match self.tier {
            EnchantmentTier::Minor => 0,
            EnchantmentTier::Standard => 0,
            EnchantmentTier::Greater => 10,
            EnchantmentTier::Superior => 20,
            EnchantmentTier::Master => 35,
        }
    }
}

/// Tracks enchantments on an item
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ItemEnchantments {
    pub enchantments: Vec<Enchantment>,
    pub max_enchantments: u32,
    pub is_cursed: bool,
}

impl ItemEnchantments {
    /// Create a new empty enchantment container
    pub fn new() -> Self {
        Self {
            enchantments: Vec::new(),
            max_enchantments: 3, // Default max
            is_cursed: false,
        }
    }

    /// Create with a custom max enchantment limit
    pub fn with_max(max: u32) -> Self {
        Self {
            enchantments: Vec::new(),
            max_enchantments: max,
            is_cursed: false,
        }
    }

    /// Returns true if the item can accept more enchantments
    pub fn can_add_enchantment(&self) -> bool {
        (self.enchantments.len() as u32) < self.max_enchantments
    }

    /// Returns true if the item has a specific enchantment type
    pub fn has_enchantment(&self, enchant_type: EnchantmentType) -> bool {
        self.enchantments.iter().any(|e| e.enchant_type == enchant_type)
    }

    /// Gets an enchantment by type if present
    pub fn get_enchantment(&self, enchant_type: EnchantmentType) -> Option<&Enchantment> {
        self.enchantments.iter().find(|e| e.enchant_type == enchant_type)
    }

    /// Checks if a new enchantment would conflict with existing ones
    pub fn would_conflict(&self, enchant_type: EnchantmentType) -> bool {
        let conflicts = enchant_type.conflicts_with();
        self.enchantments.iter().any(|e| conflicts.contains(&e.enchant_type))
    }

    /// Add an enchantment, returns true if successful
    pub fn add_enchantment(&mut self, enchantment: Enchantment) -> bool {
        if !self.can_add_enchantment() {
            return false;
        }
        if self.has_enchantment(enchantment.enchant_type) {
            return false;
        }
        if self.would_conflict(enchantment.enchant_type) {
            return false;
        }
        self.enchantments.push(enchantment);
        true
    }

    /// Remove an enchantment by type, returns the removed enchantment if found
    pub fn remove_enchantment(&mut self, enchant_type: EnchantmentType) -> Option<Enchantment> {
        if let Some(pos) = self.enchantments.iter().position(|e| e.enchant_type == enchant_type) {
            Some(self.enchantments.remove(pos))
        } else {
            None
        }
    }

    /// Upgrade an existing enchantment's tier, returns true if successful
    pub fn upgrade_enchantment(&mut self, enchant_type: EnchantmentType) -> bool {
        if let Some(ench) = self.enchantments.iter_mut().find(|e| e.enchant_type == enchant_type) {
            if ench.tier < EnchantmentTier::Master {
                ench.tier = EnchantmentTier::from_level(ench.tier.level() + 1);
                return true;
            }
        }
        false
    }

    /// Calculate total attack bonus from all enchantments
    pub fn total_attack_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.attack_bonus()).sum()
    }

    /// Calculate total defense bonus from all enchantments
    pub fn total_defense_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.defense_bonus()).sum()
    }

    /// Calculate total HP bonus from all enchantments
    pub fn total_hp_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.hp_bonus()).sum()
    }

    /// Calculate total mana bonus from all enchantments
    pub fn total_mana_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.mana_bonus()).sum()
    }

    /// Calculate combined life steal percentage
    pub fn total_life_steal(&self) -> i32 {
        self.enchantments.iter().map(|e| e.life_steal_percent()).sum()
    }

    /// Calculate combined mana steal amount
    pub fn total_mana_steal(&self) -> i32 {
        self.enchantments.iter().map(|e| e.mana_steal_amount()).sum()
    }

    /// Get combined regeneration per turn
    pub fn total_regen(&self) -> i32 {
        self.enchantments.iter().map(|e| e.regen_per_turn()).sum()
    }

    /// Get combined evasion chance
    pub fn total_evasion(&self) -> i32 {
        self.enchantments.iter().map(|e| e.evasion_percent()).sum::<i32>().min(50) // Cap at 50%
    }

    /// Get combined fortune bonus
    pub fn total_fortune(&self) -> i32 {
        self.enchantments.iter().map(|e| e.fortune_bonus()).sum()
    }

    /// Get combined XP bonus
    pub fn total_xp_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.xp_bonus()).sum()
    }

    /// Get combined vision bonus
    pub fn total_vision_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.vision_bonus()).sum()
    }

    /// Returns a formatted string of all enchantment names
    pub fn enchantment_names(&self) -> Vec<String> {
        self.enchantments.iter().map(|e| e.display_name()).collect()
    }
}

/// Result of an enchantment attempt
#[derive(Clone, Debug)]
pub enum EnchantResult {
    /// Enchantment was successfully applied
    Success {
        enchantment: Enchantment,
        message: String,
    },
    /// Enchantment failed but item is intact
    Failure {
        message: String,
    },
    /// Enchantment failed and item was destroyed
    Destroyed {
        message: String,
    },
    /// Item was cursed during the enchantment
    Cursed {
        enchantment: Option<Enchantment>,
        message: String,
    },
    /// Cannot apply this enchantment (incompatible slot, conflicts, etc.)
    Invalid {
        reason: String,
    },
}

/// The main enchantment system
pub struct EnchantmentSystem;

impl EnchantmentSystem {
    /// Attempt to apply a scroll to an item's enchantments
    pub fn apply_scroll(
        scroll: &EnchantmentScroll,
        item_enchantments: &mut ItemEnchantments,
        equipment_slot: EquipSlot,
        player_level: u32,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        // Check player level requirement
        if player_level < scroll.tier.required_level() {
            return EnchantResult::Invalid {
                reason: format!(
                    "Requires level {} to use {} scrolls",
                    scroll.tier.required_level(),
                    scroll.tier.name()
                ),
            };
        }

        match scroll.scroll_type {
            EnchantmentScrollType::Random => {
                Self::apply_random_enchantment(scroll, item_enchantments, equipment_slot, rng)
            }
            EnchantmentScrollType::Specific(enchant_type) => {
                Self::apply_specific_enchantment(scroll, item_enchantments, equipment_slot, enchant_type, rng)
            }
            EnchantmentScrollType::Upgrade => {
                Self::apply_upgrade(scroll, item_enchantments, rng)
            }
            EnchantmentScrollType::Removal => {
                Self::apply_removal(item_enchantments, rng)
            }
            EnchantmentScrollType::Transfer => {
                EnchantResult::Invalid {
                    reason: "Transfer requires a source and target item".to_string(),
                }
            }
            EnchantmentScrollType::Blessed => {
                Self::apply_blessed_enchantment(scroll, item_enchantments, equipment_slot, rng)
            }
            EnchantmentScrollType::Cursed => {
                Self::apply_cursed_enchantment(scroll, item_enchantments, equipment_slot, rng)
            }
        }
    }

    /// Apply a random enchantment to an item
    fn apply_random_enchantment(
        scroll: &EnchantmentScroll,
        item_enchantments: &mut ItemEnchantments,
        equipment_slot: EquipSlot,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        if !item_enchantments.can_add_enchantment() {
            return EnchantResult::Invalid {
                reason: "Item has maximum enchantments".to_string(),
            };
        }

        // Get compatible enchantment types
        let compatible: Vec<EnchantmentType> = EnchantmentType::all()
            .iter()
            .filter(|e| e.compatible_with_slot(equipment_slot))
            .filter(|e| !item_enchantments.has_enchantment(**e))
            .filter(|e| !item_enchantments.would_conflict(**e))
            .copied()
            .collect();

        if compatible.is_empty() {
            return EnchantResult::Invalid {
                reason: "No compatible enchantments available".to_string(),
            };
        }

        // Weight-based random selection
        let total_weight: u32 = compatible.iter().map(|e| e.rarity_weight()).sum();
        let mut roll = rng.gen_range(0..total_weight);
        let mut selected_type = compatible[0];

        for ench_type in &compatible {
            let weight = ench_type.rarity_weight();
            if roll < weight {
                selected_type = *ench_type;
                break;
            }
            roll -= weight;
        }

        // Check success
        let success_roll = rng.gen_range(0..100);
        if success_roll >= scroll.base_success_rate() {
            // Failed
            if scroll.can_destroy_on_failure() {
                let destroy_roll = rng.gen_range(0..100);
                if destroy_roll < scroll.destruction_chance() {
                    return EnchantResult::Destroyed {
                        message: "The enchantment backfired! Your item was destroyed!".to_string(),
                    };
                }
            }
            return EnchantResult::Failure {
                message: "The enchantment fizzles and fails.".to_string(),
            };
        }

        let enchantment = Enchantment::new(selected_type, scroll.tier);
        item_enchantments.add_enchantment(enchantment);

        EnchantResult::Success {
            enchantment,
            message: format!("Your item glows with {} power!", enchantment.display_name()),
        }
    }

    /// Apply a specific enchantment type
    fn apply_specific_enchantment(
        scroll: &EnchantmentScroll,
        item_enchantments: &mut ItemEnchantments,
        equipment_slot: EquipSlot,
        enchant_type: EnchantmentType,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        if !enchant_type.compatible_with_slot(equipment_slot) {
            return EnchantResult::Invalid {
                reason: format!("{} cannot be applied to this item type", enchant_type.name()),
            };
        }

        if item_enchantments.has_enchantment(enchant_type) {
            return EnchantResult::Invalid {
                reason: format!("Item already has {}", enchant_type.name()),
            };
        }

        if item_enchantments.would_conflict(enchant_type) {
            return EnchantResult::Invalid {
                reason: format!("{} conflicts with existing enchantments", enchant_type.name()),
            };
        }

        if !item_enchantments.can_add_enchantment() {
            return EnchantResult::Invalid {
                reason: "Item has maximum enchantments".to_string(),
            };
        }

        // Check success
        let success_roll = rng.gen_range(0..100);
        if success_roll >= scroll.base_success_rate() {
            if scroll.can_destroy_on_failure() {
                let destroy_roll = rng.gen_range(0..100);
                if destroy_roll < scroll.destruction_chance() {
                    return EnchantResult::Destroyed {
                        message: "The enchantment backfired! Your item was destroyed!".to_string(),
                    };
                }
            }
            return EnchantResult::Failure {
                message: format!("The {} enchantment fails to take hold.", enchant_type.name()),
            };
        }

        let enchantment = Enchantment::new(enchant_type, scroll.tier);
        item_enchantments.add_enchantment(enchantment);

        EnchantResult::Success {
            enchantment,
            message: format!("Your item surges with {} energy!", enchantment.display_name()),
        }
    }

    /// Apply an upgrade to an existing enchantment
    fn apply_upgrade(
        scroll: &EnchantmentScroll,
        item_enchantments: &mut ItemEnchantments,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        // Find enchantments that can be upgraded
        let upgradeable: Vec<EnchantmentType> = item_enchantments.enchantments
            .iter()
            .filter(|e| e.tier < EnchantmentTier::Master)
            .map(|e| e.enchant_type)
            .collect();

        if upgradeable.is_empty() {
            return EnchantResult::Invalid {
                reason: "No enchantments can be upgraded".to_string(),
            };
        }

        // Pick random enchantment to upgrade
        let target = upgradeable[rng.gen_range(0..upgradeable.len())];

        // Check success with reduced rate for upgrades
        let success_roll = rng.gen_range(0..100);
        let adjusted_rate = scroll.base_success_rate() - 10; // Upgrades are harder

        if success_roll >= adjusted_rate {
            if scroll.can_destroy_on_failure() {
                let destroy_roll = rng.gen_range(0..100);
                if destroy_roll < scroll.destruction_chance() + 5 { // Extra risk for upgrades
                    return EnchantResult::Destroyed {
                        message: "The enhancement exploded! Your item was destroyed!".to_string(),
                    };
                }
            }
            return EnchantResult::Failure {
                message: "The enhancement fades without effect.".to_string(),
            };
        }

        item_enchantments.upgrade_enchantment(target);
        let upgraded = item_enchantments.get_enchantment(target).unwrap();

        EnchantResult::Success {
            enchantment: *upgraded,
            message: format!("{} has been enhanced to {} tier!", target.name(), upgraded.tier.name()),
        }
    }

    /// Remove a random enchantment
    fn apply_removal(
        item_enchantments: &mut ItemEnchantments,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        if item_enchantments.enchantments.is_empty() {
            return EnchantResult::Invalid {
                reason: "Item has no enchantments to remove".to_string(),
            };
        }

        let idx = rng.gen_range(0..item_enchantments.enchantments.len());
        let removed = item_enchantments.enchantments.remove(idx);
        item_enchantments.is_cursed = false; // Removal also removes curses

        EnchantResult::Success {
            enchantment: removed,
            message: format!("{} has been removed from your item.", removed.display_name()),
        }
    }

    /// Apply a blessed enchantment (higher success, no destruction)
    fn apply_blessed_enchantment(
        scroll: &EnchantmentScroll,
        item_enchantments: &mut ItemEnchantments,
        equipment_slot: EquipSlot,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        // Blessed scrolls work like random but with better rates and no destruction
        if !item_enchantments.can_add_enchantment() {
            return EnchantResult::Invalid {
                reason: "Item has maximum enchantments".to_string(),
            };
        }

        let compatible: Vec<EnchantmentType> = EnchantmentType::all()
            .iter()
            .filter(|e| e.compatible_with_slot(equipment_slot))
            .filter(|e| !item_enchantments.has_enchantment(**e))
            .filter(|e| !item_enchantments.would_conflict(**e))
            .copied()
            .collect();

        if compatible.is_empty() {
            return EnchantResult::Invalid {
                reason: "No compatible enchantments available".to_string(),
            };
        }

        // Always succeeds with blessed scrolls, just pick a good enchantment
        // Blessed scrolls favor rarer enchantments
        let total_weight: u32 = compatible.iter().map(|e| 150 - e.rarity_weight().min(100)).sum();
        let mut roll = rng.gen_range(0..total_weight);
        let mut selected_type = compatible[0];

        for ench_type in &compatible {
            let weight = 150 - ench_type.rarity_weight().min(100);
            if roll < weight {
                selected_type = *ench_type;
                break;
            }
            roll -= weight;
        }

        let enchantment = Enchantment::new(selected_type, scroll.tier);
        item_enchantments.add_enchantment(enchantment);

        EnchantResult::Success {
            enchantment,
            message: format!("Divine light infuses your item with {}!", enchantment.display_name()),
        }
    }

    /// Apply a cursed enchantment (powerful but risky)
    fn apply_cursed_enchantment(
        scroll: &EnchantmentScroll,
        item_enchantments: &mut ItemEnchantments,
        equipment_slot: EquipSlot,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        if !item_enchantments.can_add_enchantment() {
            return EnchantResult::Invalid {
                reason: "Item has maximum enchantments".to_string(),
            };
        }

        // Cursed scrolls favor powerful/dark enchantments
        let dark_enchants = [
            EnchantmentType::Vampiric,
            EnchantmentType::Shadow,
            EnchantmentType::Berserker,
            EnchantmentType::Chaos,
            EnchantmentType::Execute,
            EnchantmentType::LifeSteal,
            EnchantmentType::ArmorPierce,
        ];

        let compatible: Vec<EnchantmentType> = dark_enchants
            .iter()
            .filter(|e| e.compatible_with_slot(equipment_slot))
            .filter(|e| !item_enchantments.has_enchantment(**e))
            .filter(|e| !item_enchantments.would_conflict(**e))
            .copied()
            .collect();

        if compatible.is_empty() {
            return EnchantResult::Invalid {
                reason: "No compatible dark enchantments available".to_string(),
            };
        }

        let selected_type = compatible[rng.gen_range(0..compatible.len())];

        // Higher tier for cursed scrolls
        let boosted_tier = EnchantmentTier::from_level(scroll.tier.level() + 1);
        let enchantment = Enchantment::new(selected_type, boosted_tier);
        item_enchantments.add_enchantment(enchantment);

        // High chance to curse the item
        let curse_roll = rng.gen_range(0..100);
        if curse_roll < 60 {
            item_enchantments.is_cursed = true;
            return EnchantResult::Cursed {
                enchantment: Some(enchantment),
                message: format!(
                    "Dark power surges through your item! {} applied, but the item is now CURSED!",
                    enchantment.display_name()
                ),
            };
        }

        EnchantResult::Success {
            enchantment,
            message: format!("Dark energy infuses your item with {}!", enchantment.display_name()),
        }
    }

    /// Transfer an enchantment from one item to another
    pub fn transfer_enchantment(
        scroll: &EnchantmentScroll,
        source_enchantments: &mut ItemEnchantments,
        target_enchantments: &mut ItemEnchantments,
        target_slot: EquipSlot,
        rng: &mut impl Rng,
    ) -> EnchantResult {
        if source_enchantments.enchantments.is_empty() {
            return EnchantResult::Invalid {
                reason: "Source item has no enchantments".to_string(),
            };
        }

        if !target_enchantments.can_add_enchantment() {
            return EnchantResult::Invalid {
                reason: "Target item has maximum enchantments".to_string(),
            };
        }

        // Find transferable enchantments
        let transferable: Vec<usize> = source_enchantments.enchantments
            .iter()
            .enumerate()
            .filter(|(_, e)| e.enchant_type.compatible_with_slot(target_slot))
            .filter(|(_, e)| !target_enchantments.has_enchantment(e.enchant_type))
            .filter(|(_, e)| !target_enchantments.would_conflict(e.enchant_type))
            .map(|(i, _)| i)
            .collect();

        if transferable.is_empty() {
            return EnchantResult::Invalid {
                reason: "No compatible enchantments can be transferred".to_string(),
            };
        }

        let idx = transferable[rng.gen_range(0..transferable.len())];

        // Transfer has lower success rate
        let success_roll = rng.gen_range(0..100);
        if success_roll >= scroll.base_success_rate() {
            // On failure, source loses the enchantment but target doesn't get it
            let lost = source_enchantments.enchantments.remove(idx);
            return EnchantResult::Failure {
                message: format!("The transfer failed! {} was lost in the process.", lost.display_name()),
            };
        }

        let transferred = source_enchantments.enchantments.remove(idx);
        // Transferred enchantments lose one tier
        let reduced_tier = EnchantmentTier::from_level(transferred.tier.level().saturating_sub(1));
        let new_enchantment = Enchantment::new(transferred.enchant_type, reduced_tier);
        target_enchantments.add_enchantment(new_enchantment);

        EnchantResult::Success {
            enchantment: new_enchantment,
            message: format!("{} has been transferred!", new_enchantment.display_name()),
        }
    }

    /// Generate a random enchantment scroll appropriate for a dungeon level
    pub fn generate_scroll(dungeon_level: u32, rng: &mut impl Rng) -> EnchantmentScroll {
        // Determine tier based on dungeon level
        let tier = match dungeon_level {
            1..=5 => EnchantmentTier::Minor,
            6..=10 => {
                if rng.gen_bool(0.7) { EnchantmentTier::Minor }
                else { EnchantmentTier::Standard }
            }
            11..=15 => {
                if rng.gen_bool(0.5) { EnchantmentTier::Standard }
                else if rng.gen_bool(0.7) { EnchantmentTier::Minor }
                else { EnchantmentTier::Greater }
            }
            16..=20 => {
                if rng.gen_bool(0.4) { EnchantmentTier::Greater }
                else if rng.gen_bool(0.5) { EnchantmentTier::Standard }
                else { EnchantmentTier::Superior }
            }
            21..=25 => {
                if rng.gen_bool(0.5) { EnchantmentTier::Greater }
                else if rng.gen_bool(0.6) { EnchantmentTier::Superior }
                else { EnchantmentTier::Master }
            }
            _ => {
                if rng.gen_bool(0.4) { EnchantmentTier::Superior }
                else { EnchantmentTier::Master }
            }
        };

        // Determine scroll type
        let scroll_type_roll = rng.gen_range(0..100);
        let scroll_type = if scroll_type_roll < 50 {
            EnchantmentScrollType::Random
        } else if scroll_type_roll < 75 {
            // Specific enchantment scroll
            let ench_types = EnchantmentType::all();
            let selected = ench_types[rng.gen_range(0..ench_types.len())];
            EnchantmentScrollType::Specific(selected)
        } else if scroll_type_roll < 85 {
            EnchantmentScrollType::Upgrade
        } else if scroll_type_roll < 92 {
            EnchantmentScrollType::Removal
        } else if scroll_type_roll < 96 {
            EnchantmentScrollType::Blessed
        } else if scroll_type_roll < 99 {
            EnchantmentScrollType::Cursed
        } else {
            EnchantmentScrollType::Transfer
        };

        EnchantmentScroll::new(scroll_type, tier)
    }

    /// Calculate the monetary value of an enchantment
    pub fn enchantment_value(enchantment: &Enchantment) -> u32 {
        let base_value = match enchantment.enchant_type.rarity_weight() {
            w if w >= 80 => 50,   // Common
            w if w >= 50 => 100,  // Uncommon
            w if w >= 25 => 250,  // Rare
            w if w >= 10 => 500,  // Epic
            _ => 1000,            // Legendary
        };

        let tier_mult = match enchantment.tier {
            EnchantmentTier::Minor => 1,
            EnchantmentTier::Standard => 2,
            EnchantmentTier::Greater => 4,
            EnchantmentTier::Superior => 8,
            EnchantmentTier::Master => 16,
        };

        base_value * tier_mult
    }

    /// Calculate the monetary value of a scroll
    pub fn scroll_value(scroll: &EnchantmentScroll) -> u32 {
        let base_value = match &scroll.scroll_type {
            EnchantmentScrollType::Random => 100,
            EnchantmentScrollType::Specific(ench_type) => {
                match ench_type.rarity_weight() {
                    w if w >= 80 => 150,
                    w if w >= 50 => 300,
                    w if w >= 25 => 600,
                    w if w >= 10 => 1200,
                    _ => 2500,
                }
            }
            EnchantmentScrollType::Upgrade => 500,
            EnchantmentScrollType::Removal => 200,
            EnchantmentScrollType::Transfer => 800,
            EnchantmentScrollType::Blessed => 1500,
            EnchantmentScrollType::Cursed => 750,
        };

        let tier_mult = match scroll.tier {
            EnchantmentTier::Minor => 1,
            EnchantmentTier::Standard => 2,
            EnchantmentTier::Greater => 4,
            EnchantmentTier::Superior => 8,
            EnchantmentTier::Master => 16,
        };

        base_value * tier_mult * scroll.charges
    }
}

/// Extended item with enchantment support
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnchantedItem {
    pub base_item: Item,
    pub enchantments: ItemEnchantments,
}

impl EnchantedItem {
    /// Create a new enchanted item wrapper
    pub fn new(item: Item) -> Self {
        Self {
            base_item: item,
            enchantments: ItemEnchantments::new(),
        }
    }

    /// Create with a specific enchantment slot limit based on rarity
    pub fn with_rarity_slots(item: Item) -> Self {
        let max_slots = match item.rarity {
            Rarity::Common => 1,
            Rarity::Uncommon => 2,
            Rarity::Rare => 2,
            Rarity::Epic => 3,
            Rarity::Legendary => 4,
            Rarity::Mythic => 5,
        };
        Self {
            base_item: item,
            enchantments: ItemEnchantments::with_max(max_slots),
        }
    }

    /// Returns the full display name including enchantments
    pub fn display_name(&self) -> String {
        let base_name = self.base_item.display_name();
        if self.enchantments.enchantments.is_empty() {
            return base_name;
        }

        let ench_names: Vec<String> = self.enchantments.enchantments
            .iter()
            .map(|e| e.enchant_type.name().to_string())
            .collect();

        let cursed_prefix = if self.enchantments.is_cursed { "Cursed " } else { "" };
        format!("{}{} of {}", cursed_prefix, base_name, ench_names.join(" & "))
    }

    /// Calculate total stats including enchantments: (attack, defense, hp, mana)
    pub fn total_stats(&self) -> (i32, i32, i32, i32) {
        let (base_atk, base_def, base_hp, base_mana) = self.base_item.stats();
        (
            base_atk + self.enchantments.total_attack_bonus(),
            base_def + self.enchantments.total_defense_bonus(),
            base_hp + self.enchantments.total_hp_bonus(),
            base_mana + self.enchantments.total_mana_bonus(),
        )
    }

    /// Calculate the total value of this item (base + enchantments)
    pub fn total_value(&self) -> u32 {
        let base_value = match self.base_item.rarity {
            Rarity::Common => 10,
            Rarity::Uncommon => 25,
            Rarity::Rare => 75,
            Rarity::Epic => 200,
            Rarity::Legendary => 500,
            Rarity::Mythic => 1500,
        };

        let ench_value: u32 = self.enchantments.enchantments
            .iter()
            .map(|e| EnchantmentSystem::enchantment_value(e))
            .sum();

        base_value + ench_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enchantment_creation() {
        let ench = Enchantment::new(EnchantmentType::Fire, EnchantmentTier::Standard);
        assert_eq!(ench.enchant_type, EnchantmentType::Fire);
        assert_eq!(ench.tier, EnchantmentTier::Standard);
    }

    #[test]
    fn test_enchantment_tier_ordering() {
        assert!(EnchantmentTier::Minor < EnchantmentTier::Standard);
        assert!(EnchantmentTier::Standard < EnchantmentTier::Greater);
        assert!(EnchantmentTier::Greater < EnchantmentTier::Superior);
        assert!(EnchantmentTier::Superior < EnchantmentTier::Master);
    }

    #[test]
    fn test_enchantment_compatibility() {
        assert!(EnchantmentType::Fire.compatible_with_slot(EquipSlot::Weapon));
        assert!(!EnchantmentType::Fire.compatible_with_slot(EquipSlot::Armor));
        assert!(EnchantmentType::Protection.compatible_with_slot(EquipSlot::Armor));
        assert!(EnchantmentType::Vitality.compatible_with_slot(EquipSlot::Weapon));
        assert!(EnchantmentType::Vitality.compatible_with_slot(EquipSlot::Armor));
    }

    #[test]
    fn test_enchantment_conflicts() {
        let conflicts = EnchantmentType::Fire.conflicts_with();
        assert!(conflicts.contains(&EnchantmentType::Ice));
    }

    #[test]
    fn test_item_enchantments_add() {
        let mut item_ench = ItemEnchantments::new();
        let ench = Enchantment::new(EnchantmentType::Fire, EnchantmentTier::Standard);

        assert!(item_ench.add_enchantment(ench));
        assert!(item_ench.has_enchantment(EnchantmentType::Fire));
        assert!(!item_ench.add_enchantment(ench)); // Can't add duplicate
    }

    #[test]
    fn test_enchantment_conflict_prevention() {
        let mut item_ench = ItemEnchantments::new();
        let fire = Enchantment::new(EnchantmentType::Fire, EnchantmentTier::Standard);
        let ice = Enchantment::new(EnchantmentType::Ice, EnchantmentTier::Standard);

        assert!(item_ench.add_enchantment(fire));
        assert!(item_ench.would_conflict(EnchantmentType::Ice));
        assert!(!item_ench.add_enchantment(ice));
    }

    #[test]
    fn test_enchantment_upgrade() {
        let mut item_ench = ItemEnchantments::new();
        let ench = Enchantment::new(EnchantmentType::Fire, EnchantmentTier::Standard);
        item_ench.add_enchantment(ench);

        assert!(item_ench.upgrade_enchantment(EnchantmentType::Fire));
        let upgraded = item_ench.get_enchantment(EnchantmentType::Fire).unwrap();
        assert_eq!(upgraded.tier, EnchantmentTier::Greater);
    }

    #[test]
    fn test_scroll_creation() {
        let scroll = EnchantmentScroll::new(
            EnchantmentScrollType::Specific(EnchantmentType::Fire),
            EnchantmentTier::Greater,
        );
        assert!(scroll.display_name().contains("Fire"));
        assert!(scroll.base_success_rate() > 0);
    }

    #[test]
    fn test_enchanted_item_stats() {
        let item = Item::new(0, 0, ItemKind::LongSword, Rarity::Rare);
        let mut enchanted = EnchantedItem::new(item);

        let (base_atk, _, _, _) = enchanted.total_stats();

        let ench = Enchantment::new(EnchantmentType::Sharpness, EnchantmentTier::Standard);
        enchanted.enchantments.add_enchantment(ench);

        let (new_atk, _, _, _) = enchanted.total_stats();
        assert!(new_atk > base_atk);
    }

    #[test]
    fn test_tier_multipliers() {
        let minor = Enchantment::new(EnchantmentType::Sharpness, EnchantmentTier::Minor);
        let master = Enchantment::new(EnchantmentType::Sharpness, EnchantmentTier::Master);

        assert!(master.attack_bonus() > minor.attack_bonus());
    }
}

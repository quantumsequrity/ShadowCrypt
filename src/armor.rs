//! Comprehensive Armor System
//!
//! This module provides a complete armor system including:
//! - 8 armor slots (Head, Chest, Legs, Boots, Gloves, Shoulders, Belt, Cape)
//! - 9 armor material types (Cloth, Leather, Chain, Scale, Plate, Dragon, Demon, Crystal, Void)
//! - 25 complete armor sets with set bonuses at 2/4/6 pieces
//! - 26 armor enchantments including 4 legendary enchantments
//! - Quality and durability system
//! - Defense calculations (physical, magical, elemental resistances)
//! - Weight system affecting speed and dodge
//! - Class-specific armor bonuses
//! - 20 legendary armor pieces with special abilities

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::prelude::*;
use crossterm::style::Color;
use crate::CharacterClass;

// ============================================================================
// ARMOR SLOTS
// ============================================================================

/// Armor slots - where armor can be equipped
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ArmorSlot {
    Head,
    Chest,
    Legs,
    Boots,
    Gloves,
    Shoulders,
    Belt,
    Cape,
}

impl ArmorSlot {
    pub fn all() -> Vec<ArmorSlot> {
        vec![
            ArmorSlot::Head,
            ArmorSlot::Chest,
            ArmorSlot::Legs,
            ArmorSlot::Boots,
            ArmorSlot::Gloves,
            ArmorSlot::Shoulders,
            ArmorSlot::Belt,
            ArmorSlot::Cape,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ArmorSlot::Head => "Head",
            ArmorSlot::Chest => "Chest",
            ArmorSlot::Legs => "Legs",
            ArmorSlot::Boots => "Boots",
            ArmorSlot::Gloves => "Gloves",
            ArmorSlot::Shoulders => "Shoulders",
            ArmorSlot::Belt => "Belt",
            ArmorSlot::Cape => "Cape",
        }
    }

    /// Base weight multiplier for slot (chest is heaviest, cape is lightest)
    pub fn weight_multiplier(&self) -> f32 {
        match self {
            ArmorSlot::Chest => 1.0,
            ArmorSlot::Legs => 0.7,
            ArmorSlot::Head => 0.4,
            ArmorSlot::Shoulders => 0.35,
            ArmorSlot::Boots => 0.3,
            ArmorSlot::Gloves => 0.25,
            ArmorSlot::Belt => 0.2,
            ArmorSlot::Cape => 0.15,
        }
    }

    /// Defense contribution percentage for this slot
    pub fn defense_contribution(&self) -> f32 {
        match self {
            ArmorSlot::Chest => 0.30,      // 30% of total armor defense
            ArmorSlot::Legs => 0.20,       // 20%
            ArmorSlot::Head => 0.15,       // 15%
            ArmorSlot::Shoulders => 0.10,  // 10%
            ArmorSlot::Boots => 0.08,      // 8%
            ArmorSlot::Gloves => 0.07,     // 7%
            ArmorSlot::Belt => 0.05,       // 5%
            ArmorSlot::Cape => 0.05,       // 5%
        }
    }
}

// ============================================================================
// ARMOR TYPES
// ============================================================================

/// Armor material types affecting stats and weight
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ArmorType {
    Cloth,     // Lightest, lowest physical defense, good for mages
    Leather,   // Light, balanced, good for rogues
    Chain,     // Medium weight, good physical defense
    Scale,     // Medium-heavy, better defense
    Plate,     // Heavy, highest physical defense
    Dragon,    // Legendary material, fire resistance
    Demon,     // Dark material, shadow resistance
    Crystal,   // Magical material, high magic defense
    Void,      // Ultimate material, all resistances
}

impl ArmorType {
    pub fn name(&self) -> &'static str {
        match self {
            ArmorType::Cloth => "Cloth",
            ArmorType::Leather => "Leather",
            ArmorType::Chain => "Chain",
            ArmorType::Scale => "Scale",
            ArmorType::Plate => "Plate",
            ArmorType::Dragon => "Dragon",
            ArmorType::Demon => "Demon",
            ArmorType::Crystal => "Crystal",
            ArmorType::Void => "Void",
        }
    }

    /// Base physical defense modifier
    pub fn physical_defense(&self) -> i32 {
        match self {
            ArmorType::Cloth => 2,
            ArmorType::Leather => 5,
            ArmorType::Chain => 10,
            ArmorType::Scale => 15,
            ArmorType::Plate => 25,
            ArmorType::Dragon => 30,
            ArmorType::Demon => 28,
            ArmorType::Crystal => 20,
            ArmorType::Void => 35,
        }
    }

    /// Base magical defense modifier
    pub fn magical_defense(&self) -> i32 {
        match self {
            ArmorType::Cloth => 15,
            ArmorType::Leather => 8,
            ArmorType::Chain => 5,
            ArmorType::Scale => 8,
            ArmorType::Plate => 3,
            ArmorType::Dragon => 20,
            ArmorType::Demon => 25,
            ArmorType::Crystal => 35,
            ArmorType::Void => 40,
        }
    }

    /// Base weight value (affects speed/dodge)
    pub fn base_weight(&self) -> i32 {
        match self {
            ArmorType::Cloth => 1,
            ArmorType::Leather => 3,
            ArmorType::Chain => 8,
            ArmorType::Scale => 12,
            ArmorType::Plate => 20,
            ArmorType::Dragon => 15,
            ArmorType::Demon => 10,
            ArmorType::Crystal => 8,
            ArmorType::Void => 5,
        }
    }

    /// Speed penalty (negative) or bonus (positive)
    pub fn speed_modifier(&self) -> i32 {
        match self {
            ArmorType::Cloth => 3,
            ArmorType::Leather => 2,
            ArmorType::Chain => 0,
            ArmorType::Scale => -2,
            ArmorType::Plate => -5,
            ArmorType::Dragon => -2,
            ArmorType::Demon => 1,
            ArmorType::Crystal => 0,
            ArmorType::Void => 2,
        }
    }

    /// Dodge chance bonus/penalty (percentage points)
    pub fn dodge_modifier(&self) -> i32 {
        match self {
            ArmorType::Cloth => 15,
            ArmorType::Leather => 10,
            ArmorType::Chain => 0,
            ArmorType::Scale => -5,
            ArmorType::Plate => -15,
            ArmorType::Dragon => -5,
            ArmorType::Demon => 5,
            ArmorType::Crystal => 0,
            ArmorType::Void => 10,
        }
    }

    /// Classes that get bonuses wearing this armor type
    pub fn preferred_classes(&self) -> Vec<CharacterClass> {
        match self {
            ArmorType::Cloth => vec![CharacterClass::Mage, CharacterClass::Necromancer],
            ArmorType::Leather => vec![CharacterClass::Rogue, CharacterClass::Ranger],
            ArmorType::Chain => vec![CharacterClass::Ranger, CharacterClass::Paladin],
            ArmorType::Scale => vec![CharacterClass::Paladin, CharacterClass::Warrior],
            ArmorType::Plate => vec![CharacterClass::Warrior, CharacterClass::Paladin],
            ArmorType::Dragon => vec![CharacterClass::Warrior, CharacterClass::Paladin, CharacterClass::Ranger],
            ArmorType::Demon => vec![CharacterClass::Necromancer, CharacterClass::Rogue],
            ArmorType::Crystal => vec![CharacterClass::Mage, CharacterClass::Paladin],
            ArmorType::Void => vec![], // Void armor has no class preference - universal
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ArmorType::Cloth => Color::White,
            ArmorType::Leather => Color::DarkYellow,
            ArmorType::Chain => Color::Grey,
            ArmorType::Scale => Color::DarkGreen,
            ArmorType::Plate => Color::DarkGrey,
            ArmorType::Dragon => Color::Red,
            ArmorType::Demon => Color::DarkMagenta,
            ArmorType::Crystal => Color::Cyan,
            ArmorType::Void => Color::Magenta,
        }
    }
}

// ============================================================================
// ARMOR ENCHANTMENTS
// ============================================================================

/// Armor enchantments providing special effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ArmorEnchantment {
    // Resistance enchantments
    FireResistance,
    IceResistance,
    LightningResistance,
    PoisonResistance,
    ShadowResistance,
    HolyResistance,
    ArcaneResistance,

    // Defensive enchantments
    Fortification,      // Reduces critical hit chance against wearer
    Thorns,             // Reflects damage back to attackers
    Absorption,         // Converts some damage to mana
    Warding,            // Chance to negate magic damage
    Hardened,           // Increases durability and physical defense

    // Offensive enchantments
    Vengeance,          // Increased damage when below 50% HP
    Spellpower,         // Increases magic damage
    Lifesteal,          // Heal percentage of damage dealt
    Manasteal,          // Restore mana on hit

    // Utility enchantments
    Regeneration,       // Slowly regenerate HP over time
    ManaRegeneration,   // Slowly regenerate mana over time
    Swiftness,          // Increases movement/attack speed
    Stealth,            // Harder for enemies to detect
    Enlightenment,      // Bonus XP gain
    Prosperous,         // Bonus gold find

    // Legendary enchantments
    Invulnerability,    // Periodic immunity (every 20 turns, 2 turn immunity)
    Phoenix,            // Once per floor, revive with 25% HP on death
    TimeWarp,           // Chance to get extra turn
    SoulBound,          // Cannot be lost on death
}

impl ArmorEnchantment {
    pub fn name(&self) -> &'static str {
        match self {
            ArmorEnchantment::FireResistance => "of Fire Resistance",
            ArmorEnchantment::IceResistance => "of Ice Resistance",
            ArmorEnchantment::LightningResistance => "of Lightning Resistance",
            ArmorEnchantment::PoisonResistance => "of Poison Resistance",
            ArmorEnchantment::ShadowResistance => "of Shadow Resistance",
            ArmorEnchantment::HolyResistance => "of Holy Resistance",
            ArmorEnchantment::ArcaneResistance => "of Arcane Resistance",
            ArmorEnchantment::Fortification => "of Fortification",
            ArmorEnchantment::Thorns => "of Thorns",
            ArmorEnchantment::Absorption => "of Absorption",
            ArmorEnchantment::Warding => "of Warding",
            ArmorEnchantment::Hardened => "of Hardening",
            ArmorEnchantment::Vengeance => "of Vengeance",
            ArmorEnchantment::Spellpower => "of Spellpower",
            ArmorEnchantment::Lifesteal => "of the Vampire",
            ArmorEnchantment::Manasteal => "of the Arcanist",
            ArmorEnchantment::Regeneration => "of Regeneration",
            ArmorEnchantment::ManaRegeneration => "of Wisdom",
            ArmorEnchantment::Swiftness => "of Swiftness",
            ArmorEnchantment::Stealth => "of Shadows",
            ArmorEnchantment::Enlightenment => "of Enlightenment",
            ArmorEnchantment::Prosperous => "of Prosperity",
            ArmorEnchantment::Invulnerability => "of Invulnerability",
            ArmorEnchantment::Phoenix => "of the Phoenix",
            ArmorEnchantment::TimeWarp => "of Time",
            ArmorEnchantment::SoulBound => "of Soul Binding",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ArmorEnchantment::FireResistance => "25% fire damage reduction",
            ArmorEnchantment::IceResistance => "25% ice damage reduction",
            ArmorEnchantment::LightningResistance => "25% lightning damage reduction",
            ArmorEnchantment::PoisonResistance => "Immune to poison",
            ArmorEnchantment::ShadowResistance => "25% shadow damage reduction",
            ArmorEnchantment::HolyResistance => "25% holy damage reduction",
            ArmorEnchantment::ArcaneResistance => "25% arcane damage reduction",
            ArmorEnchantment::Fortification => "50% reduced critical hit chance",
            ArmorEnchantment::Thorns => "Reflect 15% melee damage",
            ArmorEnchantment::Absorption => "Convert 10% damage to mana",
            ArmorEnchantment::Warding => "20% chance to negate magic",
            ArmorEnchantment::Hardened => "+5 physical defense, +50% durability",
            ArmorEnchantment::Vengeance => "+30% damage below 50% HP",
            ArmorEnchantment::Spellpower => "+20% magic damage",
            ArmorEnchantment::Lifesteal => "Heal 10% of damage dealt",
            ArmorEnchantment::Manasteal => "Restore 5% mana on hit",
            ArmorEnchantment::Regeneration => "Regenerate 1 HP per turn",
            ArmorEnchantment::ManaRegeneration => "Regenerate 1 mana per turn",
            ArmorEnchantment::Swiftness => "+15% attack speed",
            ArmorEnchantment::Stealth => "Enemies less likely to detect you",
            ArmorEnchantment::Enlightenment => "+25% XP gained",
            ArmorEnchantment::Prosperous => "+50% gold find",
            ArmorEnchantment::Invulnerability => "2 turn immunity every 20 turns",
            ArmorEnchantment::Phoenix => "Revive once per floor",
            ArmorEnchantment::TimeWarp => "10% chance for extra turn",
            ArmorEnchantment::SoulBound => "Keep on death",
        }
    }

    pub fn is_legendary(&self) -> bool {
        matches!(self,
            ArmorEnchantment::Invulnerability |
            ArmorEnchantment::Phoenix |
            ArmorEnchantment::TimeWarp |
            ArmorEnchantment::SoulBound
        )
    }

    pub fn value(&self) -> i32 {
        if self.is_legendary() { 500 } else { 100 }
    }
}

// ============================================================================
// ARMOR QUALITY
// ============================================================================

/// Armor quality affecting all stats
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum ArmorQuality {
    Broken,      // 50% stats
    Damaged,     // 75% stats
    Normal,      // 100% stats
    Fine,        // 115% stats
    Superior,    // 130% stats
    Exceptional, // 150% stats
    Masterwork,  // 175% stats
    Legendary,   // 200% stats
    Divine,      // 250% stats
}

impl ArmorQuality {
    pub fn name(&self) -> &'static str {
        match self {
            ArmorQuality::Broken => "Broken",
            ArmorQuality::Damaged => "Damaged",
            ArmorQuality::Normal => "",
            ArmorQuality::Fine => "Fine",
            ArmorQuality::Superior => "Superior",
            ArmorQuality::Exceptional => "Exceptional",
            ArmorQuality::Masterwork => "Masterwork",
            ArmorQuality::Legendary => "Legendary",
            ArmorQuality::Divine => "Divine",
        }
    }

    pub fn stat_multiplier(&self) -> f32 {
        match self {
            ArmorQuality::Broken => 0.5,
            ArmorQuality::Damaged => 0.75,
            ArmorQuality::Normal => 1.0,
            ArmorQuality::Fine => 1.15,
            ArmorQuality::Superior => 1.30,
            ArmorQuality::Exceptional => 1.50,
            ArmorQuality::Masterwork => 1.75,
            ArmorQuality::Legendary => 2.0,
            ArmorQuality::Divine => 2.5,
        }
    }

    pub fn durability_multiplier(&self) -> f32 {
        match self {
            ArmorQuality::Broken => 0.25,
            ArmorQuality::Damaged => 0.5,
            ArmorQuality::Normal => 1.0,
            ArmorQuality::Fine => 1.2,
            ArmorQuality::Superior => 1.5,
            ArmorQuality::Exceptional => 2.0,
            ArmorQuality::Masterwork => 2.5,
            ArmorQuality::Legendary => 3.0,
            ArmorQuality::Divine => 999.0, // Essentially unbreakable
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ArmorQuality::Broken => Color::DarkGrey,
            ArmorQuality::Damaged => Color::Grey,
            ArmorQuality::Normal => Color::White,
            ArmorQuality::Fine => Color::Green,
            ArmorQuality::Superior => Color::Blue,
            ArmorQuality::Exceptional => Color::Magenta,
            ArmorQuality::Masterwork => Color::Yellow,
            ArmorQuality::Legendary => Color::Red,
            ArmorQuality::Divine => Color::Cyan,
        }
    }

    pub fn random(rng: &mut impl Rng, dungeon_level: u32) -> Self {
        let roll = rng.gen_range(0..100) + dungeon_level as i32;
        match roll {
            0..=20 => ArmorQuality::Damaged,
            21..=60 => ArmorQuality::Normal,
            61..=80 => ArmorQuality::Fine,
            81..=90 => ArmorQuality::Superior,
            91..=97 => ArmorQuality::Exceptional,
            98..=104 => ArmorQuality::Masterwork,
            105..=115 => ArmorQuality::Legendary,
            _ => ArmorQuality::Divine,
        }
    }
}

// ============================================================================
// ARMOR SETS
// ============================================================================

/// Armor sets - complete collections with set bonuses
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ArmorSet {
    // Tier 1: Basic Sets (levels 1-10)
    Apprentice,      // Cloth - +mana
    Scout,           // Leather - +speed
    Militia,         // Chain - balanced
    Guard,           // Scale - +defense
    Knight,          // Plate - +HP

    // Tier 2: Intermediate Sets (levels 11-20)
    Battlemage,      // Cloth - spell damage
    Shadowstrike,    // Leather - crit chance
    Veteran,         // Chain - all stats
    Warden,          // Scale - resistances
    Crusader,        // Plate - holy damage

    // Tier 3: Advanced Sets (levels 21-30)
    Archmage,        // Cloth - ultimate caster
    Assassin,        // Leather - ultimate rogue
    Dragonslayer,    // Dragon - dragon bonuses
    Demonhunter,     // Demon - demon bonuses
    Crystalline,     // Crystal - magic mastery

    // Legendary Sets
    VoidWalker,      // Void - dimensional powers
    PhoenixAscendant,// Fire - rebirth powers
    FrostLord,       // Ice - freeze powers
    Stormcaller,     // Lightning - chain attacks
    BloodKnight,     // Dark - lifesteal mastery
    SolarGuardian,   // Holy - healing aura
    NatureWarden,    // Nature - regeneration
    ShadowLord,      // Shadow - stealth mastery
    TitanForge,      // Earth - unbreakable
    Celestial,       // Divine - godlike power
}

impl ArmorSet {
    pub fn name(&self) -> &'static str {
        match self {
            ArmorSet::Apprentice => "Apprentice",
            ArmorSet::Scout => "Scout",
            ArmorSet::Militia => "Militia",
            ArmorSet::Guard => "Guard",
            ArmorSet::Knight => "Knight",
            ArmorSet::Battlemage => "Battlemage",
            ArmorSet::Shadowstrike => "Shadowstrike",
            ArmorSet::Veteran => "Veteran",
            ArmorSet::Warden => "Warden",
            ArmorSet::Crusader => "Crusader",
            ArmorSet::Archmage => "Archmage",
            ArmorSet::Assassin => "Assassin",
            ArmorSet::Dragonslayer => "Dragonslayer",
            ArmorSet::Demonhunter => "Demonhunter",
            ArmorSet::Crystalline => "Crystalline",
            ArmorSet::VoidWalker => "Void Walker",
            ArmorSet::PhoenixAscendant => "Phoenix Ascendant",
            ArmorSet::FrostLord => "Frost Lord",
            ArmorSet::Stormcaller => "Stormcaller",
            ArmorSet::BloodKnight => "Blood Knight",
            ArmorSet::SolarGuardian => "Solar Guardian",
            ArmorSet::NatureWarden => "Nature's Warden",
            ArmorSet::ShadowLord => "Shadow Lord",
            ArmorSet::TitanForge => "Titan's Forge",
            ArmorSet::Celestial => "Celestial",
        }
    }

    pub fn armor_type(&self) -> ArmorType {
        match self {
            ArmorSet::Apprentice | ArmorSet::Battlemage | ArmorSet::Archmage => ArmorType::Cloth,
            ArmorSet::Scout | ArmorSet::Shadowstrike | ArmorSet::Assassin => ArmorType::Leather,
            ArmorSet::Militia | ArmorSet::Veteran => ArmorType::Chain,
            ArmorSet::Guard | ArmorSet::Warden => ArmorType::Scale,
            ArmorSet::Knight | ArmorSet::Crusader | ArmorSet::TitanForge => ArmorType::Plate,
            ArmorSet::Dragonslayer | ArmorSet::PhoenixAscendant => ArmorType::Dragon,
            ArmorSet::Demonhunter | ArmorSet::BloodKnight | ArmorSet::ShadowLord => ArmorType::Demon,
            ArmorSet::Crystalline | ArmorSet::SolarGuardian | ArmorSet::FrostLord |
            ArmorSet::Stormcaller | ArmorSet::NatureWarden => ArmorType::Crystal,
            ArmorSet::VoidWalker | ArmorSet::Celestial => ArmorType::Void,
        }
    }

    pub fn required_level(&self) -> u32 {
        match self {
            ArmorSet::Apprentice | ArmorSet::Scout | ArmorSet::Militia |
            ArmorSet::Guard | ArmorSet::Knight => 1,

            ArmorSet::Battlemage | ArmorSet::Shadowstrike | ArmorSet::Veteran |
            ArmorSet::Warden | ArmorSet::Crusader => 11,

            ArmorSet::Archmage | ArmorSet::Assassin | ArmorSet::Dragonslayer |
            ArmorSet::Demonhunter | ArmorSet::Crystalline => 21,

            ArmorSet::VoidWalker | ArmorSet::PhoenixAscendant | ArmorSet::FrostLord |
            ArmorSet::Stormcaller | ArmorSet::BloodKnight | ArmorSet::SolarGuardian |
            ArmorSet::NatureWarden | ArmorSet::ShadowLord | ArmorSet::TitanForge |
            ArmorSet::Celestial => 25,
        }
    }

    pub fn is_legendary(&self) -> bool {
        self.required_level() >= 25
    }

    /// Get the bonus for wearing N pieces (2, 4, or 6)
    pub fn set_bonus(&self, pieces: usize) -> ArmorSetBonus {
        match self {
            ArmorSet::Apprentice => match pieces {
                2..=3 => ArmorSetBonus::new(0, 0, 0, 20, 0, 0, 0, None),
                4..=5 => ArmorSetBonus::new(0, 0, 5, 40, 0, 5, 0, None),
                6..=8 => ArmorSetBonus::new(0, 0, 10, 60, 0, 10, 5, Some("Mana Shield: Convert 10% damage to mana cost")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Scout => match pieces {
                2..=3 => ArmorSetBonus::new(0, 0, 0, 0, 3, 5, 0, None),
                4..=5 => ArmorSetBonus::new(0, 5, 0, 0, 6, 10, 0, None),
                6..=8 => ArmorSetBonus::new(0, 10, 5, 10, 10, 15, 10, Some("Swift Strike: 25% chance for free attack")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Militia => match pieces {
                2..=3 => ArmorSetBonus::new(10, 3, 3, 0, 0, 0, 0, None),
                4..=5 => ArmorSetBonus::new(20, 6, 6, 10, 0, 0, 0, None),
                6..=8 => ArmorSetBonus::new(35, 10, 10, 20, 2, 5, 0, Some("Veteran's Resolve: Reduce damage by 5")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Guard => match pieces {
                2..=3 => ArmorSetBonus::new(15, 0, 8, 0, 0, 0, 0, None),
                4..=5 => ArmorSetBonus::new(30, 0, 15, 0, -1, 5, 0, None),
                6..=8 => ArmorSetBonus::new(50, 5, 25, 0, -2, 10, 0, Some("Iron Wall: Block 20% of all damage")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Knight => match pieces {
                2..=3 => ArmorSetBonus::new(25, 5, 5, 0, 0, 0, 0, None),
                4..=5 => ArmorSetBonus::new(50, 10, 12, 10, -1, 0, 0, None),
                6..=8 => ArmorSetBonus::new(80, 15, 20, 20, -2, 5, 5, Some("Knight's Valor: Immune to fear and stun")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Battlemage => match pieces {
                2..=3 => ArmorSetBonus::new(0, 8, 0, 30, 0, 0, 10, None),
                4..=5 => ArmorSetBonus::new(15, 15, 5, 60, 2, 0, 15, None),
                6..=8 => ArmorSetBonus::new(30, 25, 10, 100, 3, 5, 25, Some("Arcane Fury: Spells cost 25% less mana")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Shadowstrike => match pieces {
                2..=3 => ArmorSetBonus::new(0, 10, 0, 0, 4, 10, 5, None),
                4..=5 => ArmorSetBonus::new(10, 20, 5, 15, 6, 15, 10, None),
                6..=8 => ArmorSetBonus::new(25, 35, 10, 30, 8, 20, 20, Some("Shadow Dance: 30% chance to avoid all damage")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Veteran => match pieces {
                2..=3 => ArmorSetBonus::new(20, 8, 8, 10, 1, 3, 5, None),
                4..=5 => ArmorSetBonus::new(40, 15, 15, 25, 2, 6, 10, None),
                6..=8 => ArmorSetBonus::new(65, 25, 25, 40, 4, 10, 15, Some("Battle Hardened: +10% all stats in combat")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Warden => match pieces {
                2..=3 => ArmorSetBonus::new(25, 0, 15, 15, 0, 5, 0, None),
                4..=5 => ArmorSetBonus::new(45, 5, 25, 30, 0, 10, 5, None),
                6..=8 => ArmorSetBonus::new(70, 10, 40, 50, 0, 15, 10, Some("Elemental Ward: 25% resistance to all elements")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Crusader => match pieces {
                2..=3 => ArmorSetBonus::new(30, 12, 10, 20, 0, 0, 0, None),
                4..=5 => ArmorSetBonus::new(55, 22, 18, 40, 0, 5, 10, None),
                6..=8 => ArmorSetBonus::new(90, 35, 30, 60, 0, 10, 15, Some("Holy Crusade: Deal 50% extra damage to undead/demons")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Archmage => match pieces {
                2..=3 => ArmorSetBonus::new(10, 15, 5, 50, 2, 0, 15, None),
                4..=5 => ArmorSetBonus::new(25, 30, 10, 100, 3, 5, 25, None),
                6..=8 => ArmorSetBonus::new(50, 50, 20, 150, 5, 10, 40, Some("Arcane Mastery: Spells can critical hit for 2x damage")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Assassin => match pieces {
                2..=3 => ArmorSetBonus::new(0, 20, 0, 10, 5, 15, 10, None),
                4..=5 => ArmorSetBonus::new(15, 40, 8, 25, 8, 25, 20, None),
                6..=8 => ArmorSetBonus::new(35, 60, 15, 40, 12, 35, 30, Some("Death Mark: Critical hits deal 3x damage")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Dragonslayer => match pieces {
                2..=3 => ArmorSetBonus::new(40, 20, 15, 20, 0, 5, 10, None),
                4..=5 => ArmorSetBonus::new(75, 35, 28, 40, 2, 10, 20, None),
                6..=8 => ArmorSetBonus::new(120, 55, 45, 65, 3, 15, 30, Some("Dragon's Bane: +100% damage to dragons, fire immunity")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Demonhunter => match pieces {
                2..=3 => ArmorSetBonus::new(30, 25, 10, 25, 2, 8, 10, None),
                4..=5 => ArmorSetBonus::new(55, 45, 20, 50, 4, 15, 20, None),
                6..=8 => ArmorSetBonus::new(90, 70, 35, 80, 6, 22, 30, Some("Demon's Bane: +100% damage to demons, shadow immunity")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Crystalline => match pieces {
                2..=3 => ArmorSetBonus::new(20, 20, 20, 40, 0, 5, 15, None),
                4..=5 => ArmorSetBonus::new(40, 40, 35, 80, 2, 10, 25, None),
                6..=8 => ArmorSetBonus::new(70, 60, 55, 130, 3, 15, 40, Some("Crystal Resonance: Magic damage heals you")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::VoidWalker => match pieces {
                2..=3 => ArmorSetBonus::new(35, 30, 25, 50, 5, 15, 20, None),
                4..=5 => ArmorSetBonus::new(70, 55, 45, 100, 8, 25, 35, None),
                6..=8 => ArmorSetBonus::new(120, 85, 70, 160, 12, 35, 50, Some("Void Step: Teleport through walls, phase through attacks")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::PhoenixAscendant => match pieces {
                2..=3 => ArmorSetBonus::new(50, 25, 20, 40, 3, 10, 15, None),
                4..=5 => ArmorSetBonus::new(95, 50, 38, 80, 5, 18, 28, None),
                6..=8 => ArmorSetBonus::new(150, 80, 60, 130, 8, 25, 45, Some("Phoenix Rebirth: Resurrect with full HP once per floor, fire immunity")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::FrostLord => match pieces {
                2..=3 => ArmorSetBonus::new(40, 25, 30, 45, 2, 8, 15, None),
                4..=5 => ArmorSetBonus::new(75, 48, 55, 90, 4, 15, 28, None),
                6..=8 => ArmorSetBonus::new(125, 75, 85, 145, 6, 22, 42, Some("Absolute Zero: Attacks freeze enemies, ice immunity")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Stormcaller => match pieces {
                2..=3 => ArmorSetBonus::new(30, 35, 15, 50, 4, 10, 18, None),
                4..=5 => ArmorSetBonus::new(60, 65, 30, 100, 7, 18, 32, None),
                6..=8 => ArmorSetBonus::new(100, 100, 50, 160, 10, 28, 48, Some("Chain Lightning: Attacks chain to 3 nearby enemies")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::BloodKnight => match pieces {
                2..=3 => ArmorSetBonus::new(60, 30, 15, 30, 2, 5, 12, None),
                4..=5 => ArmorSetBonus::new(110, 58, 30, 60, 4, 12, 25, None),
                6..=8 => ArmorSetBonus::new(180, 90, 50, 100, 6, 20, 40, Some("Blood Pact: Lifesteal 25% of damage dealt")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::SolarGuardian => match pieces {
                2..=3 => ArmorSetBonus::new(45, 20, 25, 35, 1, 8, 15, None),
                4..=5 => ArmorSetBonus::new(85, 40, 48, 70, 2, 15, 28, None),
                6..=8 => ArmorSetBonus::new(140, 65, 75, 115, 4, 22, 45, Some("Solar Flare: Heal allies nearby, burn undead/demons")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::NatureWarden => match pieces {
                2..=3 => ArmorSetBonus::new(40, 15, 20, 40, 2, 10, 12, None),
                4..=5 => ArmorSetBonus::new(80, 32, 40, 80, 4, 18, 25, None),
                6..=8 => ArmorSetBonus::new(130, 55, 65, 130, 6, 28, 40, Some("Nature's Blessing: Regenerate 5 HP per turn, summon treant ally")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::ShadowLord => match pieces {
                2..=3 => ArmorSetBonus::new(25, 40, 10, 35, 6, 20, 15, None),
                4..=5 => ArmorSetBonus::new(50, 75, 22, 70, 10, 32, 28, None),
                6..=8 => ArmorSetBonus::new(85, 115, 40, 115, 15, 45, 45, Some("Shadow Realm: Become invisible, backstab for 4x damage")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::TitanForge => match pieces {
                2..=3 => ArmorSetBonus::new(80, 20, 40, 20, -2, 0, 5, None),
                4..=5 => ArmorSetBonus::new(150, 40, 75, 40, -3, 5, 15, None),
                6..=8 => ArmorSetBonus::new(250, 65, 120, 70, -4, 10, 30, Some("Titan's Might: Immune to knockback/stun, reflect 30% damage")),
                _ => ArmorSetBonus::default(),
            },
            ArmorSet::Celestial => match pieces {
                2..=3 => ArmorSetBonus::new(60, 40, 40, 60, 4, 15, 25, None),
                4..=5 => ArmorSetBonus::new(115, 75, 75, 115, 7, 25, 45, None),
                6..=8 => ArmorSetBonus::new(200, 120, 120, 180, 12, 40, 70, Some("Divine Ascension: +50% all stats, resurrect once, all immunities")),
                _ => ArmorSetBonus::default(),
            },
        }
    }

    pub fn all() -> Vec<ArmorSet> {
        vec![
            ArmorSet::Apprentice, ArmorSet::Scout, ArmorSet::Militia, ArmorSet::Guard, ArmorSet::Knight,
            ArmorSet::Battlemage, ArmorSet::Shadowstrike, ArmorSet::Veteran, ArmorSet::Warden, ArmorSet::Crusader,
            ArmorSet::Archmage, ArmorSet::Assassin, ArmorSet::Dragonslayer, ArmorSet::Demonhunter, ArmorSet::Crystalline,
            ArmorSet::VoidWalker, ArmorSet::PhoenixAscendant, ArmorSet::FrostLord, ArmorSet::Stormcaller,
            ArmorSet::BloodKnight, ArmorSet::SolarGuardian, ArmorSet::NatureWarden, ArmorSet::ShadowLord,
            ArmorSet::TitanForge, ArmorSet::Celestial,
        ]
    }

    pub fn color(&self) -> Color {
        if self.is_legendary() {
            Color::Yellow
        } else {
            self.armor_type().color()
        }
    }
}

// ============================================================================
// ARMOR SET BONUS
// ============================================================================

/// Bonuses granted by armor sets
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArmorSetBonus {
    pub hp_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub mana_bonus: i32,
    pub speed_bonus: i32,
    pub dodge_bonus: i32,
    pub magic_power_bonus: i32,
    pub special_ability: Option<String>,
}

impl ArmorSetBonus {
    pub fn new(
        hp: i32, attack: i32, defense: i32, mana: i32,
        speed: i32, dodge: i32, magic_power: i32,
        special: Option<&'static str>,
    ) -> Self {
        Self {
            hp_bonus: hp,
            attack_bonus: attack,
            defense_bonus: defense,
            mana_bonus: mana,
            speed_bonus: speed,
            dodge_bonus: dodge,
            magic_power_bonus: magic_power,
            special_ability: special.map(|s| s.to_string()),
        }
    }
}

// ============================================================================
// ELEMENTAL RESISTANCES
// ============================================================================

/// Elemental resistances
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ElementalResistances {
    pub fire: i32,
    pub ice: i32,
    pub lightning: i32,
    pub poison: i32,
    pub shadow: i32,
    pub holy: i32,
    pub arcane: i32,
}

impl ElementalResistances {
    pub fn add(&mut self, other: &ElementalResistances) {
        self.fire += other.fire;
        self.ice += other.ice;
        self.lightning += other.lightning;
        self.poison += other.poison;
        self.shadow += other.shadow;
        self.holy += other.holy;
        self.arcane += other.arcane;
    }
}

// ============================================================================
// LEGENDARY ABILITIES
// ============================================================================

/// Legendary armor abilities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum LegendaryAbility {
    // Weapon-like abilities
    Smite,              // Deal holy damage on hit
    SoulRend,           // Ignore armor
    Thunderclap,        // AoE damage on hit
    FrostNova,          // Freeze nearby enemies

    // Defensive abilities
    DivineBubble,       // Periodic immunity shield
    MirrorImage,        // Create decoys
    StoneSkin,          // Massive damage reduction
    VampireCloak,       // Lifesteal on all damage

    // Utility abilities
    BlinkStep,          // Short range teleport
    TimeSkip,           // Skip enemy turns
    SoulCapture,        // Chance to enslave enemy
    ResourceWell,       // Unlimited mana for short time

    // Ultimate abilities
    Apocalypse,         // Massive AoE damage
    Rebirth,            // Auto-resurrect
    Ascension,          // Temporary god mode
    DimensionRift,      // Banish enemies
}

impl LegendaryAbility {
    pub fn name(&self) -> &'static str {
        match self {
            LegendaryAbility::Smite => "Divine Smite",
            LegendaryAbility::SoulRend => "Soul Rend",
            LegendaryAbility::Thunderclap => "Thunderclap",
            LegendaryAbility::FrostNova => "Frost Nova",
            LegendaryAbility::DivineBubble => "Divine Bubble",
            LegendaryAbility::MirrorImage => "Mirror Image",
            LegendaryAbility::StoneSkin => "Stone Skin",
            LegendaryAbility::VampireCloak => "Vampire Cloak",
            LegendaryAbility::BlinkStep => "Blink Step",
            LegendaryAbility::TimeSkip => "Time Skip",
            LegendaryAbility::SoulCapture => "Soul Capture",
            LegendaryAbility::ResourceWell => "Mana Well",
            LegendaryAbility::Apocalypse => "Apocalypse",
            LegendaryAbility::Rebirth => "Phoenix Rebirth",
            LegendaryAbility::Ascension => "Divine Ascension",
            LegendaryAbility::DimensionRift => "Dimension Rift",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            LegendaryAbility::Smite => "Deal 50 bonus holy damage on hit",
            LegendaryAbility::SoulRend => "Attacks ignore 50% of enemy armor",
            LegendaryAbility::Thunderclap => "Hit deals AoE damage to nearby enemies",
            LegendaryAbility::FrostNova => "Chance to freeze all nearby enemies",
            LegendaryAbility::DivineBubble => "Gain immunity shield every 15 turns",
            LegendaryAbility::MirrorImage => "Create 2 decoys that confuse enemies",
            LegendaryAbility::StoneSkin => "Reduce all damage by 25",
            LegendaryAbility::VampireCloak => "Heal 20% of all damage dealt",
            LegendaryAbility::BlinkStep => "Teleport up to 5 tiles on command",
            LegendaryAbility::TimeSkip => "15% chance enemies skip their turn",
            LegendaryAbility::SoulCapture => "5% chance to convert enemy to ally",
            LegendaryAbility::ResourceWell => "Mana costs reduced by 50%",
            LegendaryAbility::Apocalypse => "Once per floor, deal 500 damage to all enemies",
            LegendaryAbility::Rebirth => "Resurrect once per floor with 50% HP",
            LegendaryAbility::Ascension => "Once per floor, become invincible for 10 turns",
            LegendaryAbility::DimensionRift => "Banish one enemy per floor to another dimension",
        }
    }

    pub fn cooldown(&self) -> u32 {
        match self {
            LegendaryAbility::Smite | LegendaryAbility::SoulRend => 0,
            LegendaryAbility::Thunderclap | LegendaryAbility::FrostNova => 5,
            LegendaryAbility::DivineBubble => 15,
            LegendaryAbility::MirrorImage => 20,
            LegendaryAbility::StoneSkin | LegendaryAbility::VampireCloak => 0,
            LegendaryAbility::BlinkStep => 3,
            LegendaryAbility::TimeSkip | LegendaryAbility::SoulCapture => 0,
            LegendaryAbility::ResourceWell => 0,
            LegendaryAbility::Apocalypse | LegendaryAbility::Rebirth |
            LegendaryAbility::Ascension | LegendaryAbility::DimensionRift => 9999, // Once per floor
        }
    }
}

// ============================================================================
// ARMOR PIECE
// ============================================================================

/// A single piece of armor
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmorPiece {
    pub name: String,
    pub slot: ArmorSlot,
    pub armor_type: ArmorType,
    pub quality: ArmorQuality,
    pub set: Option<ArmorSet>,
    pub enchantments: Vec<ArmorEnchantment>,
    pub legendary_ability: Option<LegendaryAbility>,

    // Base stats (before quality/enchantment modifiers)
    pub base_physical_defense: i32,
    pub base_magical_defense: i32,
    pub base_hp: i32,
    pub base_mana: i32,
    pub base_attack: i32,

    // Durability
    pub max_durability: u32,
    pub current_durability: u32,

    // Resistances
    pub resistances: ElementalResistances,

    // Level requirement
    pub required_level: u32,

    // Value in gold
    pub base_value: u32,
}

impl ArmorPiece {
    /// Create a new armor piece
    pub fn new(
        slot: ArmorSlot,
        armor_type: ArmorType,
        quality: ArmorQuality,
        set: Option<ArmorSet>,
        level: u32,
    ) -> Self {
        let slot_mult = slot.defense_contribution();
        let base_phys = (armor_type.physical_defense() as f32 * slot_mult * 10.0) as i32;
        let base_mag = (armor_type.magical_defense() as f32 * slot_mult * 10.0) as i32;

        let base_durability = (100.0 * quality.durability_multiplier()) as u32;

        let name = format!(
            "{} {} {}",
            quality.name(),
            set.map(|s| s.name()).unwrap_or(armor_type.name()),
            slot.name()
        ).trim().to_string();

        Self {
            name,
            slot,
            armor_type,
            quality,
            set,
            enchantments: Vec::new(),
            legendary_ability: None,
            base_physical_defense: base_phys,
            base_magical_defense: base_mag,
            base_hp: 0,
            base_mana: 0,
            base_attack: 0,
            max_durability: base_durability,
            current_durability: base_durability,
            resistances: ElementalResistances::default(),
            required_level: set.map(|s| s.required_level()).unwrap_or(level.max(1)),
            base_value: (50 + level * 10) * quality.stat_multiplier() as u32,
        }
    }

    /// Create a legendary armor piece with special ability
    pub fn new_legendary(
        name: &str,
        slot: ArmorSlot,
        armor_type: ArmorType,
        ability: LegendaryAbility,
        level: u32,
    ) -> Self {
        let mut armor = Self::new(slot, armor_type, ArmorQuality::Legendary, None, level);
        armor.name = name.to_string();
        armor.legendary_ability = Some(ability);
        armor.base_physical_defense = (armor.base_physical_defense as f32 * 1.5) as i32;
        armor.base_magical_defense = (armor.base_magical_defense as f32 * 1.5) as i32;
        armor.base_hp = 30;
        armor.base_mana = 20;
        armor.base_value *= 5;
        armor
    }

    /// Get the effective physical defense
    pub fn physical_defense(&self) -> i32 {
        let quality_mult = self.quality.stat_multiplier();
        let durability_mult = self.current_durability as f32 / self.max_durability.max(1) as f32;
        let enchant_bonus: i32 = self.enchantments.iter()
            .filter(|e| matches!(e, ArmorEnchantment::Hardened))
            .count() as i32 * 5;

        ((self.base_physical_defense as f32 * quality_mult * durability_mult) as i32 + enchant_bonus).max(0)
    }

    /// Get the effective magical defense
    pub fn magical_defense(&self) -> i32 {
        let quality_mult = self.quality.stat_multiplier();
        let durability_mult = self.current_durability as f32 / self.max_durability.max(1) as f32;
        let enchant_bonus: i32 = self.enchantments.iter()
            .filter(|e| matches!(e, ArmorEnchantment::Warding))
            .count() as i32 * 5;

        ((self.base_magical_defense as f32 * quality_mult * durability_mult) as i32 + enchant_bonus).max(0)
    }

    /// Get the weight of this armor piece
    pub fn weight(&self) -> i32 {
        let base = self.armor_type.base_weight() as f32;
        let slot_mult = self.slot.weight_multiplier();
        (base * slot_mult * 10.0) as i32
    }

    /// Get the speed modifier from this armor
    pub fn speed_modifier(&self) -> i32 {
        let base = self.armor_type.speed_modifier();
        let swiftness_bonus: i32 = self.enchantments.iter()
            .filter(|e| matches!(e, ArmorEnchantment::Swiftness))
            .count() as i32 * 2;
        base + swiftness_bonus
    }

    /// Get the dodge modifier from this armor
    pub fn dodge_modifier(&self) -> i32 {
        self.armor_type.dodge_modifier()
    }

    /// Get class-specific bonus (percentage multiplier)
    pub fn class_bonus(&self, class: CharacterClass) -> f32 {
        if self.armor_type.preferred_classes().contains(&class) {
            1.15 // 15% bonus for preferred classes
        } else {
            1.0
        }
    }

    /// Check if the armor has a specific enchantment
    pub fn has_enchantment(&self, enchant: ArmorEnchantment) -> bool {
        self.enchantments.contains(&enchant)
    }

    /// Add an enchantment to this armor
    pub fn add_enchantment(&mut self, enchant: ArmorEnchantment) {
        if !self.enchantments.contains(&enchant) {
            self.enchantments.push(enchant);
            self.base_value += enchant.value() as u32;
        }
    }

    /// Take durability damage
    pub fn take_durability_damage(&mut self, amount: u32) {
        self.current_durability = self.current_durability.saturating_sub(amount);
        // Update quality if durability gets too low
        if self.current_durability == 0 && self.quality != ArmorQuality::Broken {
            self.quality = ArmorQuality::Broken;
        } else if self.current_durability < self.max_durability / 4 && self.quality > ArmorQuality::Damaged {
            self.quality = ArmorQuality::Damaged;
        }
    }

    /// Repair the armor
    pub fn repair(&mut self, amount: u32) {
        self.current_durability = (self.current_durability + amount).min(self.max_durability);
    }

    /// Get the display name
    pub fn display_name(&self) -> String {
        let mut name = self.name.clone();
        if !self.enchantments.is_empty() {
            if let Some(enchant) = self.enchantments.first() {
                name = format!("{} {}", name, enchant.name());
            }
        }
        name
    }

    /// Get the total elemental resistances including enchantments
    pub fn total_resistances(&self) -> ElementalResistances {
        let mut res = self.resistances.clone();

        for enchant in &self.enchantments {
            match enchant {
                ArmorEnchantment::FireResistance => res.fire += 25,
                ArmorEnchantment::IceResistance => res.ice += 25,
                ArmorEnchantment::LightningResistance => res.lightning += 25,
                ArmorEnchantment::PoisonResistance => res.poison += 100, // Immunity
                ArmorEnchantment::ShadowResistance => res.shadow += 25,
                ArmorEnchantment::HolyResistance => res.holy += 25,
                ArmorEnchantment::ArcaneResistance => res.arcane += 25,
                _ => {}
            }
        }

        res
    }

    /// Get regeneration per turn from this armor
    pub fn regeneration_per_turn(&self) -> (i32, i32) {
        let mut hp_regen = 0;
        let mut mana_regen = 0;

        for enchant in &self.enchantments {
            match enchant {
                ArmorEnchantment::Regeneration => hp_regen += 1,
                ArmorEnchantment::ManaRegeneration => mana_regen += 1,
                _ => {}
            }
        }

        (hp_regen, mana_regen)
    }
}

// ============================================================================
// ARMOR EQUIPMENT
// ============================================================================

/// Player's equipped armor collection
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArmorEquipment {
    pub pieces: HashMap<ArmorSlot, ArmorPiece>,
}

impl ArmorEquipment {
    pub fn new() -> Self {
        Self {
            pieces: HashMap::new(),
        }
    }

    /// Equip an armor piece, returning the old piece if any
    pub fn equip(&mut self, piece: ArmorPiece) -> Option<ArmorPiece> {
        let slot = piece.slot;
        self.pieces.insert(slot, piece)
    }

    /// Unequip armor from a slot
    pub fn unequip(&mut self, slot: ArmorSlot) -> Option<ArmorPiece> {
        self.pieces.remove(&slot)
    }

    /// Get the armor piece in a slot
    pub fn get(&self, slot: ArmorSlot) -> Option<&ArmorPiece> {
        self.pieces.get(&slot)
    }

    /// Get mutable reference to armor piece
    pub fn get_mut(&mut self, slot: ArmorSlot) -> Option<&mut ArmorPiece> {
        self.pieces.get_mut(&slot)
    }

    /// Calculate total physical defense
    pub fn total_physical_defense(&self, player_class: CharacterClass) -> i32 {
        self.pieces.values()
            .map(|p| (p.physical_defense() as f32 * p.class_bonus(player_class)) as i32)
            .sum()
    }

    /// Calculate total magical defense
    pub fn total_magical_defense(&self, player_class: CharacterClass) -> i32 {
        self.pieces.values()
            .map(|p| (p.magical_defense() as f32 * p.class_bonus(player_class)) as i32)
            .sum()
    }

    /// Calculate total weight
    pub fn total_weight(&self) -> i32 {
        self.pieces.values().map(|p| p.weight()).sum()
    }

    /// Calculate total speed modifier
    pub fn total_speed_modifier(&self) -> i32 {
        self.pieces.values().map(|p| p.speed_modifier()).sum()
    }

    /// Calculate total dodge modifier
    pub fn total_dodge_modifier(&self) -> i32 {
        self.pieces.values().map(|p| p.dodge_modifier()).sum()
    }

    /// Calculate total elemental resistances
    pub fn total_resistances(&self) -> ElementalResistances {
        let mut res = ElementalResistances::default();
        for piece in self.pieces.values() {
            res.add(&piece.total_resistances());
        }
        res
    }

    /// Get total HP regeneration per turn
    pub fn total_hp_regeneration(&self) -> i32 {
        self.pieces.values().map(|p| p.regeneration_per_turn().0).sum()
    }

    /// Get total mana regeneration per turn
    pub fn total_mana_regeneration(&self) -> i32 {
        self.pieces.values().map(|p| p.regeneration_per_turn().1).sum()
    }

    /// Count pieces from a specific set
    pub fn count_set_pieces(&self, set: ArmorSet) -> usize {
        self.pieces.values()
            .filter(|p| p.set == Some(set))
            .count()
    }

    /// Get all active set bonuses
    pub fn active_set_bonuses(&self) -> Vec<(ArmorSet, ArmorSetBonus)> {
        let mut bonuses = Vec::new();

        for set in ArmorSet::all() {
            let count = self.count_set_pieces(set);
            if count >= 2 {
                bonuses.push((set, set.set_bonus(count)));
            }
        }

        bonuses
    }

    /// Get combined set bonus stats
    pub fn combined_set_bonus(&self) -> ArmorSetBonus {
        let mut combined = ArmorSetBonus::default();

        for (_, bonus) in self.active_set_bonuses() {
            combined.hp_bonus += bonus.hp_bonus;
            combined.attack_bonus += bonus.attack_bonus;
            combined.defense_bonus += bonus.defense_bonus;
            combined.mana_bonus += bonus.mana_bonus;
            combined.speed_bonus += bonus.speed_bonus;
            combined.dodge_bonus += bonus.dodge_bonus;
            combined.magic_power_bonus += bonus.magic_power_bonus;
        }

        combined
    }

    /// Check if player has any legendary abilities
    pub fn legendary_abilities(&self) -> Vec<LegendaryAbility> {
        self.pieces.values()
            .filter_map(|p| p.legendary_ability)
            .collect()
    }

    /// Check for specific enchantment across all armor
    pub fn has_enchantment(&self, enchant: ArmorEnchantment) -> bool {
        self.pieces.values().any(|p| p.has_enchantment(enchant))
    }

    /// Apply durability damage to all equipped armor
    pub fn apply_durability_damage(&mut self, amount: u32) {
        for piece in self.pieces.values_mut() {
            piece.take_durability_damage(amount);
        }
    }

    /// Get total bonus HP from armor
    pub fn total_bonus_hp(&self) -> i32 {
        let base: i32 = self.pieces.values().map(|p| p.base_hp).sum();
        let set_bonus = self.combined_set_bonus().hp_bonus;
        base + set_bonus
    }

    /// Get total bonus mana from armor
    pub fn total_bonus_mana(&self) -> i32 {
        let base: i32 = self.pieces.values().map(|p| p.base_mana).sum();
        let set_bonus = self.combined_set_bonus().mana_bonus;
        base + set_bonus
    }

    /// Get total bonus attack from armor
    pub fn total_bonus_attack(&self) -> i32 {
        let base: i32 = self.pieces.values().map(|p| p.base_attack).sum();
        let set_bonus = self.combined_set_bonus().attack_bonus;
        base + set_bonus
    }
}

// ============================================================================
// DEFENSE CALCULATION
// ============================================================================

/// Defense calculation result
#[derive(Clone, Debug)]
pub struct DefenseCalculation {
    pub physical_reduction: f32,
    pub magical_reduction: f32,
    pub elemental_resistances: ElementalResistances,
    pub dodge_chance: f32,
    pub flat_reduction: i32,
}

impl DefenseCalculation {
    /// Calculate defense from armor equipment
    pub fn from_armor(armor: &ArmorEquipment, player_class: CharacterClass) -> Self {
        let phys_def = armor.total_physical_defense(player_class);
        let mag_def = armor.total_magical_defense(player_class);
        let dodge_mod = armor.total_dodge_modifier();
        let set_bonus = armor.combined_set_bonus();

        // Defense to reduction formula: reduction = defense / (defense + 100)
        // This gives diminishing returns
        let physical_reduction = phys_def as f32 / (phys_def as f32 + 100.0);
        let magical_reduction = mag_def as f32 / (mag_def as f32 + 100.0);

        // Base dodge chance is 5%, modified by armor
        let dodge_chance = (5.0 + dodge_mod as f32 + set_bonus.dodge_bonus as f32).max(0.0).min(75.0) / 100.0;

        Self {
            physical_reduction,
            magical_reduction,
            elemental_resistances: armor.total_resistances(),
            dodge_chance,
            flat_reduction: set_bonus.defense_bonus,
        }
    }

    /// Calculate actual damage taken from physical attack
    pub fn calculate_physical_damage(&self, incoming: i32) -> i32 {
        let reduced = (incoming as f32 * (1.0 - self.physical_reduction)) as i32;
        (reduced - self.flat_reduction).max(1)
    }

    /// Calculate actual damage taken from magical attack
    pub fn calculate_magical_damage(&self, incoming: i32) -> i32 {
        let reduced = (incoming as f32 * (1.0 - self.magical_reduction)) as i32;
        (reduced - self.flat_reduction / 2).max(1)
    }

    /// Calculate elemental damage with resistance
    pub fn calculate_elemental_damage(&self, incoming: i32, element: &str) -> i32 {
        let resistance = match element {
            "fire" => self.elemental_resistances.fire,
            "ice" => self.elemental_resistances.ice,
            "lightning" => self.elemental_resistances.lightning,
            "poison" => self.elemental_resistances.poison,
            "shadow" => self.elemental_resistances.shadow,
            "holy" => self.elemental_resistances.holy,
            "arcane" => self.elemental_resistances.arcane,
            _ => 0,
        };

        let reduction = (resistance as f32 / 100.0).min(0.95); // Cap at 95% reduction
        ((incoming as f32 * (1.0 - reduction)) as i32).max(0)
    }

    /// Check if attack is dodged
    pub fn check_dodge(&self, rng: &mut impl Rng) -> bool {
        rng.gen::<f32>() < self.dodge_chance
    }
}

// ============================================================================
// ARMOR GENERATION
// ============================================================================

/// Generate random armor piece for loot
pub fn generate_random_armor(rng: &mut impl Rng, dungeon_level: u32) -> ArmorPiece {
    let slots = ArmorSlot::all();
    let slot = slots[rng.gen_range(0..slots.len())];

    // Determine armor type based on level
    let armor_type = if dungeon_level >= 25 {
        match rng.gen_range(0..100) {
            0..=20 => ArmorType::Void,
            21..=40 => ArmorType::Crystal,
            41..=55 => ArmorType::Demon,
            56..=70 => ArmorType::Dragon,
            _ => ArmorType::Plate,
        }
    } else if dungeon_level >= 15 {
        match rng.gen_range(0..100) {
            0..=15 => ArmorType::Dragon,
            16..=30 => ArmorType::Demon,
            31..=45 => ArmorType::Crystal,
            46..=65 => ArmorType::Plate,
            _ => ArmorType::Scale,
        }
    } else {
        match rng.gen_range(0..100) {
            0..=25 => ArmorType::Cloth,
            26..=50 => ArmorType::Leather,
            51..=75 => ArmorType::Chain,
            76..=90 => ArmorType::Scale,
            _ => ArmorType::Plate,
        }
    };

    let quality = ArmorQuality::random(rng, dungeon_level);

    // Chance for set piece
    let set = if rng.gen_range(0..100) < 30 {
        let available_sets: Vec<ArmorSet> = ArmorSet::all()
            .into_iter()
            .filter(|s| s.required_level() <= dungeon_level && s.armor_type() == armor_type)
            .collect();

        if !available_sets.is_empty() {
            Some(available_sets[rng.gen_range(0..available_sets.len())])
        } else {
            None
        }
    } else {
        None
    };

    let mut armor = ArmorPiece::new(slot, armor_type, quality, set, dungeon_level);

    // Add enchantments based on quality
    let enchant_count = match quality {
        ArmorQuality::Broken | ArmorQuality::Damaged | ArmorQuality::Normal => 0,
        ArmorQuality::Fine => rng.gen_range(0..=1),
        ArmorQuality::Superior => rng.gen_range(1..=1),
        ArmorQuality::Exceptional => rng.gen_range(1..=2),
        ArmorQuality::Masterwork => rng.gen_range(1..=2),
        ArmorQuality::Legendary => rng.gen_range(2..=3),
        ArmorQuality::Divine => rng.gen_range(2..=4),
    };

    let all_enchants = [
        ArmorEnchantment::FireResistance, ArmorEnchantment::IceResistance,
        ArmorEnchantment::LightningResistance, ArmorEnchantment::PoisonResistance,
        ArmorEnchantment::ShadowResistance, ArmorEnchantment::HolyResistance,
        ArmorEnchantment::ArcaneResistance, ArmorEnchantment::Fortification,
        ArmorEnchantment::Thorns, ArmorEnchantment::Absorption,
        ArmorEnchantment::Warding, ArmorEnchantment::Hardened,
        ArmorEnchantment::Vengeance, ArmorEnchantment::Spellpower,
        ArmorEnchantment::Lifesteal, ArmorEnchantment::Manasteal,
        ArmorEnchantment::Regeneration, ArmorEnchantment::ManaRegeneration,
        ArmorEnchantment::Swiftness, ArmorEnchantment::Stealth,
        ArmorEnchantment::Enlightenment, ArmorEnchantment::Prosperous,
    ];

    for _ in 0..enchant_count {
        let enchant = all_enchants[rng.gen_range(0..all_enchants.len())];
        armor.add_enchantment(enchant);
    }

    // Legendary enchants for Divine quality
    if quality == ArmorQuality::Divine && rng.gen_range(0..100) < 50 {
        let legendary_enchants = [
            ArmorEnchantment::Invulnerability, ArmorEnchantment::Phoenix,
            ArmorEnchantment::TimeWarp, ArmorEnchantment::SoulBound,
        ];
        armor.add_enchantment(legendary_enchants[rng.gen_range(0..legendary_enchants.len())]);
    }

    armor
}

/// Generate a specific legendary armor piece
pub fn generate_legendary_armor(rng: &mut impl Rng, dungeon_level: u32) -> ArmorPiece {
    let legendary_armors = [
        ("Crown of the Void King", ArmorSlot::Head, ArmorType::Void, LegendaryAbility::DimensionRift),
        ("Celestial Halo", ArmorSlot::Head, ArmorType::Crystal, LegendaryAbility::DivineBubble),
        ("Helm of Eternal Frost", ArmorSlot::Head, ArmorType::Crystal, LegendaryAbility::FrostNova),
        ("Dragon Emperor's Crown", ArmorSlot::Head, ArmorType::Dragon, LegendaryAbility::Smite),

        ("Chestplate of the Apocalypse", ArmorSlot::Chest, ArmorType::Demon, LegendaryAbility::Apocalypse),
        ("Phoenix Feather Robe", ArmorSlot::Chest, ArmorType::Cloth, LegendaryAbility::Rebirth),
        ("Titan's Bulwark", ArmorSlot::Chest, ArmorType::Plate, LegendaryAbility::StoneSkin),
        ("Void Touched Vestments", ArmorSlot::Chest, ArmorType::Void, LegendaryAbility::TimeSkip),

        ("Leggings of Infinite Speed", ArmorSlot::Legs, ArmorType::Leather, LegendaryAbility::BlinkStep),
        ("Soul Reaper's Greaves", ArmorSlot::Legs, ArmorType::Demon, LegendaryAbility::SoulCapture),

        ("Boots of the Astral Walker", ArmorSlot::Boots, ArmorType::Void, LegendaryAbility::BlinkStep),
        ("Thundergod's Treads", ArmorSlot::Boots, ArmorType::Crystal, LegendaryAbility::Thunderclap),

        ("Gloves of the Archmage", ArmorSlot::Gloves, ArmorType::Cloth, LegendaryAbility::ResourceWell),
        ("Dragon's Claw Gauntlets", ArmorSlot::Gloves, ArmorType::Dragon, LegendaryAbility::SoulRend),

        ("Shoulders of Divine Judgment", ArmorSlot::Shoulders, ArmorType::Plate, LegendaryAbility::Smite),
        ("Mantle of Shadows", ArmorSlot::Shoulders, ArmorType::Leather, LegendaryAbility::MirrorImage),

        ("Belt of the Blood God", ArmorSlot::Belt, ArmorType::Demon, LegendaryAbility::VampireCloak),
        ("Girdle of Giants", ArmorSlot::Belt, ArmorType::Plate, LegendaryAbility::StoneSkin),

        ("Cape of Divine Ascension", ArmorSlot::Cape, ArmorType::Void, LegendaryAbility::Ascension),
        ("Cloak of the Vampire Lord", ArmorSlot::Cape, ArmorType::Demon, LegendaryAbility::VampireCloak),
    ];

    let (name, slot, armor_type, ability) = legendary_armors[rng.gen_range(0..legendary_armors.len())];
    ArmorPiece::new_legendary(name, slot, armor_type, ability, dungeon_level)
}

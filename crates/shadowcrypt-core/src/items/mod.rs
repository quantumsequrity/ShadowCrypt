//! Items and equipment system for the ShadowCrypt roguelike
//!
//! This module defines all items in the game including weapons, armor,
//! consumables, and special items. Features include:
//! - Item sets with set bonuses
//! - Unique items with special effects
//! - Crafting recipes
//! - Item enchantments

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// ============================================================================
// ENCHANTMENT SYSTEM
// ============================================================================

/// Types of enchantments that can be applied to items
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EnchantmentType {
    // Offensive enchantments
    Sharpness,      // +attack damage
    FireAspect,     // Fire damage on hit
    FrostAspect,    // Slow enemies on hit
    Thunderstrike,  // Chain lightning chance
    Lifesteal,      // Heal on hit
    Venomous,       // Poison damage over time
    Executing,      // Bonus damage to low health enemies
    Crushing,       // Armor penetration

    // Defensive enchantments
    Protection,     // +defense
    Thorns,         // Reflect damage
    Regeneration,   // HP regen over time
    ManaShield,     // Absorb damage with mana
    Resilience,     // Reduce critical damage taken
    Fortification,  // Bonus HP
    Warding,        // Magic resistance
    Evasion,        // Dodge chance

    // Utility enchantments
    Swiftness,      // Movement speed
    Enlightenment,  // Bonus XP
    Fortune,        // Better loot drops
    Soulbound,      // Item cannot be dropped
    Unbreaking,     // Durability preservation
    Reaching,       // Extended attack range
    Illumination,   // Light radius increase
    Featherfall,    // Reduced fall damage
}

impl EnchantmentType {
    /// Returns the display name of the enchantment
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sharpness => "Sharpness",
            Self::FireAspect => "Fire Aspect",
            Self::FrostAspect => "Frost Aspect",
            Self::Thunderstrike => "Thunderstrike",
            Self::Lifesteal => "Lifesteal",
            Self::Venomous => "Venomous",
            Self::Executing => "Executing",
            Self::Crushing => "Crushing",
            Self::Protection => "Protection",
            Self::Thorns => "Thorns",
            Self::Regeneration => "Regeneration",
            Self::ManaShield => "Mana Shield",
            Self::Resilience => "Resilience",
            Self::Fortification => "Fortification",
            Self::Warding => "Warding",
            Self::Evasion => "Evasion",
            Self::Swiftness => "Swiftness",
            Self::Enlightenment => "Enlightenment",
            Self::Fortune => "Fortune",
            Self::Soulbound => "Soulbound",
            Self::Unbreaking => "Unbreaking",
            Self::Reaching => "Reaching",
            Self::Illumination => "Illumination",
            Self::Featherfall => "Featherfall",
        }
    }

    /// Returns the maximum level for this enchantment
    pub fn max_level(&self) -> u8 {
        match self {
            Self::Sharpness | Self::Protection | Self::Fortification => 5,
            Self::FireAspect | Self::FrostAspect | Self::Venomous => 3,
            Self::Thunderstrike | Self::Lifesteal | Self::Thorns => 3,
            Self::Executing | Self::Crushing | Self::Warding => 4,
            Self::Regeneration | Self::ManaShield | Self::Resilience => 3,
            Self::Evasion | Self::Swiftness => 3,
            Self::Enlightenment | Self::Fortune => 3,
            Self::Soulbound | Self::Unbreaking => 1,
            Self::Reaching | Self::Illumination | Self::Featherfall => 2,
        }
    }

    /// Returns whether this enchantment can be applied to the given slot
    pub fn valid_for_slot(&self, slot: EquipSlot) -> bool {
        match self {
            // Weapon-only enchantments
            Self::Sharpness | Self::FireAspect | Self::FrostAspect
            | Self::Thunderstrike | Self::Lifesteal | Self::Venomous
            | Self::Executing | Self::Crushing | Self::Reaching => {
                matches!(slot, EquipSlot::Weapon)
            }
            // Armor enchantments (body armor, helmet, shield)
            Self::Protection | Self::Thorns | Self::Resilience
            | Self::Fortification | Self::Warding => {
                matches!(slot, EquipSlot::Armor | EquipSlot::Helmet | EquipSlot::Shield)
            }
            // Boot enchantments
            Self::Swiftness | Self::Featherfall => {
                matches!(slot, EquipSlot::Boots)
            }
            // Universal enchantments
            Self::Regeneration | Self::ManaShield | Self::Evasion
            | Self::Enlightenment | Self::Fortune | Self::Soulbound
            | Self::Unbreaking | Self::Illumination => true,
        }
    }
}

/// An enchantment instance with type and level
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Enchantment {
    pub enchant_type: EnchantmentType,
    pub level: u8,
}

impl Enchantment {
    /// Create a new enchantment
    pub fn new(enchant_type: EnchantmentType, level: u8) -> Self {
        let clamped_level = level.min(enchant_type.max_level()).max(1);
        Self { enchant_type, level: clamped_level }
    }

    /// Returns the stat modifiers from this enchantment (atk, def, hp, mana)
    pub fn stat_bonus(&self) -> (i32, i32, i32, i32) {
        let lvl = self.level as i32;
        match self.enchant_type {
            EnchantmentType::Sharpness => (3 * lvl, 0, 0, 0),
            EnchantmentType::Protection => (0, 2 * lvl, 0, 0),
            EnchantmentType::Fortification => (0, 0, 10 * lvl, 0),
            EnchantmentType::ManaShield => (0, 0, 0, 15 * lvl),
            EnchantmentType::Lifesteal => (lvl, 0, 0, 0),
            EnchantmentType::Thorns => (0, lvl, 0, 0),
            EnchantmentType::Regeneration => (0, 0, 5 * lvl, 0),
            EnchantmentType::Warding => (0, lvl, 0, 5 * lvl),
            _ => (0, 0, 0, 0), // Effect-based enchantments
        }
    }

    /// Returns display string like "Sharpness III"
    pub fn display_name(&self) -> String {
        let numeral = match self.level {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            _ => "?",
        };
        format!("{} {}", self.enchant_type.name(), numeral)
    }
}

// ============================================================================
// ITEM SETS SYSTEM
// ============================================================================

/// Item sets that provide bonuses when multiple pieces are equipped
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ItemSet {
    // Warrior Sets
    DragonSlayer,      // 4 pieces: Dragon-themed gear
    TitanMight,        // 3 pieces: Heavy armor set
    BerserkerRage,     // 3 pieces: High damage, low defense

    // Mage Sets
    ArcaneScholar,     // 4 pieces: Magic-focused set
    ElementalMaster,   // 5 pieces: Elemental damage set
    VoidWalker,        // 3 pieces: Void/dark magic set

    // Rogue Sets
    ShadowDancer,      // 4 pieces: Stealth and speed
    AssassinsBlade,    // 3 pieces: Critical hit focused
    NightStalker,      // 3 pieces: Night vision and stealth

    // Hybrid Sets
    PaladinValor,      // 4 pieces: Defense and holy damage
    DeathKnight,       // 4 pieces: Lifesteal and dark powers
    PhoenixRebirth,    // 3 pieces: Fire damage and revival

    // Legendary Sets
    AncientKings,      // 5 pieces: Balanced legendary set
    DemonLord,         // 4 pieces: Demonic powers
    CelestialGuard,    // 4 pieces: Divine protection
}

impl ItemSet {
    /// Returns the display name of the set
    pub fn name(&self) -> &'static str {
        match self {
            Self::DragonSlayer => "Dragon Slayer",
            Self::TitanMight => "Titan's Might",
            Self::BerserkerRage => "Berserker's Rage",
            Self::ArcaneScholar => "Arcane Scholar",
            Self::ElementalMaster => "Elemental Master",
            Self::VoidWalker => "Void Walker",
            Self::ShadowDancer => "Shadow Dancer",
            Self::AssassinsBlade => "Assassin's Blade",
            Self::NightStalker => "Night Stalker",
            Self::PaladinValor => "Paladin's Valor",
            Self::DeathKnight => "Death Knight",
            Self::PhoenixRebirth => "Phoenix Rebirth",
            Self::AncientKings => "Ancient Kings",
            Self::DemonLord => "Demon Lord",
            Self::CelestialGuard => "Celestial Guard",
        }
    }

    /// Returns the total number of pieces in this set
    pub fn total_pieces(&self) -> u8 {
        match self {
            Self::DragonSlayer | Self::ShadowDancer | Self::PaladinValor
            | Self::DeathKnight | Self::DemonLord | Self::CelestialGuard
            | Self::ArcaneScholar => 4,
            Self::TitanMight | Self::BerserkerRage | Self::VoidWalker
            | Self::AssassinsBlade | Self::NightStalker | Self::PhoenixRebirth => 3,
            Self::ElementalMaster | Self::AncientKings => 5,
        }
    }

    /// Returns the items that belong to this set
    pub fn pieces(&self) -> Vec<ItemKind> {
        match self {
            Self::DragonSlayer => vec![
                ItemKind::DragonHelm, ItemKind::DragonArmor,
                ItemKind::DragonGauntlets, ItemKind::DragonShield,
            ],
            Self::TitanMight => vec![
                ItemKind::TitanPlate, ItemKind::HelmOfValor, ItemKind::GauntletsOfMight,
            ],
            Self::BerserkerRage => vec![
                ItemKind::BattleAxe, ItemKind::DemonSkull, ItemKind::FlameGauntlets,
            ],
            Self::ArcaneScholar => vec![
                ItemKind::MageRobes, ItemKind::WizardHat, ItemKind::VoidStaff, ItemKind::RingOfMana,
            ],
            Self::ElementalMaster => vec![
                ItemKind::FlameSword, ItemKind::FrostBlade, ItemKind::ThunderAxe,
                ItemKind::RingOfFlame, ItemKind::RingOfFrost,
            ],
            Self::VoidWalker => vec![
                ItemKind::VoidStaff, ItemKind::ShadowCloak, ItemKind::RingOfShadows,
            ],
            Self::ShadowDancer => vec![
                ItemKind::AssassinGarb, ItemKind::HoodOfShadows,
                ItemKind::ShadowBoots, ItemKind::ThievesGloves,
            ],
            Self::AssassinsBlade => vec![
                ItemKind::Dagger, ItemKind::ThievesGloves, ItemKind::RingOfDeath,
            ],
            Self::NightStalker => vec![
                ItemKind::HoodOfShadows, ItemKind::ShadowBoots, ItemKind::RingOfInvisibility,
            ],
            Self::PaladinValor => vec![
                ItemKind::HolyArmor, ItemKind::HelmOfValor,
                ItemKind::PhoenixShield, ItemKind::AmuletOfLife,
            ],
            Self::DeathKnight => vec![
                ItemKind::DemonArmor, ItemKind::DemonSkull,
                ItemKind::DemonSlayer, ItemKind::RingOfTheVampire,
            ],
            Self::PhoenixRebirth => vec![
                ItemKind::PhoenixShield, ItemKind::FlameGauntlets, ItemKind::RingOfFlame,
            ],
            Self::AncientKings => vec![
                ItemKind::CrownOfKings, ItemKind::CrystalArmor, ItemKind::CrystalCrown,
                ItemKind::RingOfTheAncients, ItemKind::AmuletOfTheGods,
            ],
            Self::DemonLord => vec![
                ItemKind::DemonArmor, ItemKind::DemonSkull,
                ItemKind::AbyssalShield, ItemKind::AmuletOfDeath,
            ],
            Self::CelestialGuard => vec![
                ItemKind::HolyArmor, ItemKind::CrystalCrown,
                ItemKind::WingedBoots, ItemKind::AmuletOfTheGods,
            ],
        }
    }

    /// Returns the set bonus for the given number of equipped pieces
    /// Returns (attack_bonus, defense_bonus, hp_bonus, mana_bonus, special_effect)
    pub fn bonus_for_pieces(&self, pieces: u8) -> SetBonus {
        match self {
            Self::DragonSlayer => match pieces {
                2 => SetBonus::new(5, 5, 20, 0, Some(SetEffect::DragonBane)),
                3 => SetBonus::new(10, 10, 40, 0, Some(SetEffect::DragonBane)),
                4 => SetBonus::new(20, 15, 60, 10, Some(SetEffect::DragonSlayerAura)),
                _ => SetBonus::default(),
            },
            Self::TitanMight => match pieces {
                2 => SetBonus::new(0, 10, 30, 0, None),
                3 => SetBonus::new(5, 20, 60, 0, Some(SetEffect::Unstoppable)),
                _ => SetBonus::default(),
            },
            Self::BerserkerRage => match pieces {
                2 => SetBonus::new(15, -5, 0, 0, Some(SetEffect::Frenzy)),
                3 => SetBonus::new(30, -10, 0, 0, Some(SetEffect::Berserk)),
                _ => SetBonus::default(),
            },
            Self::ArcaneScholar => match pieces {
                2 => SetBonus::new(0, 0, 0, 30, Some(SetEffect::ManaRegen)),
                3 => SetBonus::new(5, 0, 0, 50, Some(SetEffect::ManaRegen)),
                4 => SetBonus::new(10, 5, 0, 80, Some(SetEffect::ArcaneAffinity)),
                _ => SetBonus::default(),
            },
            Self::ElementalMaster => match pieces {
                2 => SetBonus::new(5, 0, 0, 20, Some(SetEffect::ElementalResist)),
                3 => SetBonus::new(10, 0, 0, 40, Some(SetEffect::ElementalResist)),
                4 => SetBonus::new(15, 5, 0, 60, Some(SetEffect::ElementalMastery)),
                5 => SetBonus::new(25, 10, 20, 80, Some(SetEffect::ElementalOverload)),
                _ => SetBonus::default(),
            },
            Self::VoidWalker => match pieces {
                2 => SetBonus::new(8, 0, -10, 30, Some(SetEffect::VoidTouch)),
                3 => SetBonus::new(15, 0, -20, 50, Some(SetEffect::VoidEmbrace)),
                _ => SetBonus::default(),
            },
            Self::ShadowDancer => match pieces {
                2 => SetBonus::new(5, 2, 0, 10, Some(SetEffect::ShadowStep)),
                3 => SetBonus::new(10, 5, 0, 20, Some(SetEffect::ShadowStep)),
                4 => SetBonus::new(15, 8, 10, 30, Some(SetEffect::ShadowMeld)),
                _ => SetBonus::default(),
            },
            Self::AssassinsBlade => match pieces {
                2 => SetBonus::new(10, 0, 0, 0, Some(SetEffect::CriticalStrike)),
                3 => SetBonus::new(20, 0, 0, 0, Some(SetEffect::Assassination)),
                _ => SetBonus::default(),
            },
            Self::NightStalker => match pieces {
                2 => SetBonus::new(5, 3, 0, 10, Some(SetEffect::NightVision)),
                3 => SetBonus::new(10, 5, 0, 20, Some(SetEffect::Invisibility)),
                _ => SetBonus::default(),
            },
            Self::PaladinValor => match pieces {
                2 => SetBonus::new(5, 8, 20, 10, Some(SetEffect::HolyAura)),
                3 => SetBonus::new(8, 12, 40, 20, Some(SetEffect::HolyAura)),
                4 => SetBonus::new(12, 18, 60, 30, Some(SetEffect::DivineProtection)),
                _ => SetBonus::default(),
            },
            Self::DeathKnight => match pieces {
                2 => SetBonus::new(10, 5, 0, 0, Some(SetEffect::LifeDrain)),
                3 => SetBonus::new(15, 10, 0, 0, Some(SetEffect::LifeDrain)),
                4 => SetBonus::new(25, 15, 20, 0, Some(SetEffect::DeathGrip)),
                _ => SetBonus::default(),
            },
            Self::PhoenixRebirth => match pieces {
                2 => SetBonus::new(8, 0, 0, 0, Some(SetEffect::FlameAura)),
                3 => SetBonus::new(15, 5, 30, 0, Some(SetEffect::PhoenixRise)),
                _ => SetBonus::default(),
            },
            Self::AncientKings => match pieces {
                2 => SetBonus::new(5, 5, 15, 15, None),
                3 => SetBonus::new(10, 10, 30, 30, Some(SetEffect::RoyalPresence)),
                4 => SetBonus::new(15, 15, 50, 50, Some(SetEffect::RoyalPresence)),
                5 => SetBonus::new(25, 25, 80, 80, Some(SetEffect::KingsMandate)),
                _ => SetBonus::default(),
            },
            Self::DemonLord => match pieces {
                2 => SetBonus::new(12, 5, -10, 20, Some(SetEffect::DemonFire)),
                3 => SetBonus::new(20, 10, -15, 40, Some(SetEffect::DemonFire)),
                4 => SetBonus::new(35, 15, -20, 60, Some(SetEffect::DemonicPact)),
                _ => SetBonus::default(),
            },
            Self::CelestialGuard => match pieces {
                2 => SetBonus::new(5, 10, 20, 20, Some(SetEffect::HolyAura)),
                3 => SetBonus::new(8, 18, 40, 40, Some(SetEffect::CelestialBlessing)),
                4 => SetBonus::new(15, 25, 60, 60, Some(SetEffect::DivineIntervention)),
                _ => SetBonus::default(),
            },
        }
    }
}

/// Special effects granted by set bonuses
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SetEffect {
    // Dragon Slayer
    DragonBane,          // +50% damage to dragons
    DragonSlayerAura,    // Immunity to dragon breath + DragonBane

    // Titan
    Unstoppable,         // Cannot be stunned or knocked back

    // Berserker
    Frenzy,              // Attack speed increases as HP drops
    Berserk,             // +100% damage when below 25% HP

    // Mage
    ManaRegen,           // +5 mana per turn
    ArcaneAffinity,      // -25% spell cost

    // Elemental
    ElementalResist,     // +25% elemental resistance
    ElementalMastery,    // +50% elemental damage
    ElementalOverload,   // Spells have AoE effect

    // Void
    VoidTouch,           // Attacks drain enemy mana
    VoidEmbrace,         // Teleport on taking fatal damage

    // Shadow
    ShadowStep,          // +1 movement speed
    ShadowMeld,          // Invisibility when standing still

    // Assassin
    CriticalStrike,      // +15% crit chance
    Assassination,       // +50% crit damage

    // Night Stalker
    NightVision,         // See in darkness
    Invisibility,        // Periodic invisibility

    // Paladin
    HolyAura,            // Heal allies nearby
    DivineProtection,    // 25% chance to negate damage

    // Death Knight
    LifeDrain,           // Heal 10% of damage dealt
    DeathGrip,           // Pull enemies toward you

    // Phoenix
    FlameAura,           // Burn nearby enemies
    PhoenixRise,         // Revive once per floor with 50% HP

    // Ancient Kings
    RoyalPresence,       // Intimidate weak enemies
    KingsMandate,        // All stats +10%

    // Demon Lord
    DemonFire,           // Fire damage aura
    DemonicPact,         // Sacrifice HP for massive damage

    // Celestial
    CelestialBlessing,   // Regenerate HP and Mana
    DivineIntervention,  // Auto-resurrect once per level
}

impl SetEffect {
    /// Returns the display name of the effect
    pub fn name(&self) -> &'static str {
        match self {
            Self::DragonBane => "Dragon's Bane",
            Self::DragonSlayerAura => "Dragon Slayer Aura",
            Self::Unstoppable => "Unstoppable",
            Self::Frenzy => "Frenzy",
            Self::Berserk => "Berserk",
            Self::ManaRegen => "Mana Regeneration",
            Self::ArcaneAffinity => "Arcane Affinity",
            Self::ElementalResist => "Elemental Resistance",
            Self::ElementalMastery => "Elemental Mastery",
            Self::ElementalOverload => "Elemental Overload",
            Self::VoidTouch => "Void Touch",
            Self::VoidEmbrace => "Void Embrace",
            Self::ShadowStep => "Shadow Step",
            Self::ShadowMeld => "Shadow Meld",
            Self::CriticalStrike => "Critical Strike",
            Self::Assassination => "Assassination",
            Self::NightVision => "Night Vision",
            Self::Invisibility => "Invisibility",
            Self::HolyAura => "Holy Aura",
            Self::DivineProtection => "Divine Protection",
            Self::LifeDrain => "Life Drain",
            Self::DeathGrip => "Death Grip",
            Self::FlameAura => "Flame Aura",
            Self::PhoenixRise => "Phoenix Rise",
            Self::RoyalPresence => "Royal Presence",
            Self::KingsMandate => "King's Mandate",
            Self::DemonFire => "Demon Fire",
            Self::DemonicPact => "Demonic Pact",
            Self::CelestialBlessing => "Celestial Blessing",
            Self::DivineIntervention => "Divine Intervention",
        }
    }

    /// Returns a description of the effect
    pub fn description(&self) -> &'static str {
        match self {
            Self::DragonBane => "Deal 50% bonus damage to dragons",
            Self::DragonSlayerAura => "Immune to dragon breath, +50% damage to dragons",
            Self::Unstoppable => "Cannot be stunned or knocked back",
            Self::Frenzy => "Attack speed increases as HP decreases",
            Self::Berserk => "Deal double damage when below 25% HP",
            Self::ManaRegen => "Regenerate 5 mana per turn",
            Self::ArcaneAffinity => "Spells cost 25% less mana",
            Self::ElementalResist => "Take 25% less elemental damage",
            Self::ElementalMastery => "Deal 50% more elemental damage",
            Self::ElementalOverload => "Spells affect all nearby enemies",
            Self::VoidTouch => "Attacks drain enemy mana",
            Self::VoidEmbrace => "Teleport to safety when taking fatal damage",
            Self::ShadowStep => "Move 1 extra tile per turn",
            Self::ShadowMeld => "Become invisible when standing still",
            Self::CriticalStrike => "15% increased critical hit chance",
            Self::Assassination => "Critical hits deal 50% more damage",
            Self::NightVision => "See clearly in darkness",
            Self::Invisibility => "Periodically become invisible",
            Self::HolyAura => "Heal nearby allies each turn",
            Self::DivineProtection => "25% chance to negate incoming damage",
            Self::LifeDrain => "Heal for 10% of damage dealt",
            Self::DeathGrip => "Pull enemies toward you",
            Self::FlameAura => "Burn enemies that get too close",
            Self::PhoenixRise => "Revive once per floor with 50% HP",
            Self::RoyalPresence => "Weak enemies are intimidated",
            Self::KingsMandate => "All stats increased by 10%",
            Self::DemonFire => "Emanate damaging fire aura",
            Self::DemonicPact => "Sacrifice HP to deal massive damage",
            Self::CelestialBlessing => "Regenerate HP and mana over time",
            Self::DivineIntervention => "Automatically resurrect once per level",
        }
    }
}

/// Bonus stats and effects from a set
#[derive(Clone, Debug, Default)]
pub struct SetBonus {
    pub attack: i32,
    pub defense: i32,
    pub hp: i32,
    pub mana: i32,
    pub effect: Option<SetEffect>,
}

impl SetBonus {
    /// Create a new set bonus
    pub fn new(attack: i32, defense: i32, hp: i32, mana: i32, effect: Option<SetEffect>) -> Self {
        Self { attack, defense, hp, mana, effect }
    }
}

/// Equipment slots for equippable items
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EquipSlot {
    Weapon,
    Shield,
    Helmet,
    Armor,
    Gloves,
    Boots,
    Ring1,
    Ring2,
    Amulet,
}

impl EquipSlot {
    /// Returns the display name of the slot
    pub fn name(&self) -> &'static str {
        match self {
            Self::Weapon => "Weapon",
            Self::Shield => "Shield",
            Self::Helmet => "Helmet",
            Self::Armor => "Armor",
            Self::Gloves => "Gloves",
            Self::Boots => "Boots",
            Self::Ring1 => "Ring (Left)",
            Self::Ring2 => "Ring (Right)",
            Self::Amulet => "Amulet",
        }
    }
}

/// Item rarity levels
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl Rarity {
    /// Returns the name prefix for this rarity
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Common => "",
            Self::Uncommon => "Fine ",
            Self::Rare => "Superior ",
            Self::Epic => "Epic ",
            Self::Legendary => "Legendary ",
            Self::Mythic => "Mythic ",
        }
    }

    /// Returns the stat bonus multiplier for this rarity
    pub fn stat_bonus(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.25,
            Self::Rare => 1.5,
            Self::Epic => 2.0,
            Self::Legendary => 3.0,
            Self::Mythic => 5.0,
        }
    }

    /// Returns the display name of the rarity
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
            Self::Mythic => "Mythic",
        }
    }
}

// ============================================================================
// UNIQUE ITEMS SYSTEM
// ============================================================================

/// Unique items with special effects that cannot be found on regular items
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum UniqueItem {
    // Legendary Weapons
    Excalibur,              // The legendary sword of kings
    Mjolnir,                // Thor's hammer
    Gungnir,                // Odin's spear
    Masamune,               // Legendary katana
    DeathsScythe,           // Reaper's weapon
    StaffOfAges,            // Ancient arcane staff
    SerpentsFang,           // Poisonous dagger
    Stormbringer,           // Lightning bow
    FrostmournesBlade,      // Soul-stealing sword
    InfernosBrand,          // Ever-burning blade

    // Legendary Armor
    AegisOfTheGods,         // Divine shield
    DragonhideMantle,       // Ultimate dragon armor
    ValkyriesWings,         // Flying armor
    TitansGirdle,           // Belt of strength
    EternityRobes,          // Robes of immortality
    PhantomShroud,          // Ghost armor
    CrystallineCarapace,    // Living crystal armor
    BloodlordPlate,         // Vampiric armor

    // Legendary Accessories
    RingOfOmniscience,      // All-seeing ring
    AmuletOfYggdrasil,      // World tree amulet
    CrownOfEternals,        // Immortal crown
    BootsOfHermes,          // Godspeed boots
    GlovesOfMidas,          // Golden touch
    CloakOfShadows,         // True invisibility
    OrbOfDominion,          // Mind control
    HeartOfTheVoid,         // Void crystal

    // Artifact Weapons
    SoulReaver,             // Drains souls
    WorldEnder,             // Destroys all
    TimeSplitter,           // Temporal blade
    RealityBender,          // Warps space
    ChaosBringer,           // Random effects
    OrdersEdge,             // Perfect balance
    PrimordialFlame,        // First fire
    AbyssalMaw,             // Void consumer
}

impl UniqueItem {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Excalibur => "Excalibur",
            Self::Mjolnir => "Mjolnir",
            Self::Gungnir => "Gungnir",
            Self::Masamune => "Masamune",
            Self::DeathsScythe => "Death's Scythe",
            Self::StaffOfAges => "Staff of Ages",
            Self::SerpentsFang => "Serpent's Fang",
            Self::Stormbringer => "Stormbringer",
            Self::FrostmournesBlade => "Frostmourne's Blade",
            Self::InfernosBrand => "Inferno's Brand",
            Self::AegisOfTheGods => "Aegis of the Gods",
            Self::DragonhideMantle => "Dragonhide Mantle",
            Self::ValkyriesWings => "Valkyrie's Wings",
            Self::TitansGirdle => "Titan's Girdle",
            Self::EternityRobes => "Eternity Robes",
            Self::PhantomShroud => "Phantom Shroud",
            Self::CrystallineCarapace => "Crystalline Carapace",
            Self::BloodlordPlate => "Bloodlord Plate",
            Self::RingOfOmniscience => "Ring of Omniscience",
            Self::AmuletOfYggdrasil => "Amulet of Yggdrasil",
            Self::CrownOfEternals => "Crown of Eternals",
            Self::BootsOfHermes => "Boots of Hermes",
            Self::GlovesOfMidas => "Gloves of Midas",
            Self::CloakOfShadows => "Cloak of Shadows",
            Self::OrbOfDominion => "Orb of Dominion",
            Self::HeartOfTheVoid => "Heart of the Void",
            Self::SoulReaver => "Soul Reaver",
            Self::WorldEnder => "World Ender",
            Self::TimeSplitter => "Time Splitter",
            Self::RealityBender => "Reality Bender",
            Self::ChaosBringer => "Chaos Bringer",
            Self::OrdersEdge => "Order's Edge",
            Self::PrimordialFlame => "Primordial Flame",
            Self::AbyssalMaw => "Abyssal Maw",
        }
    }

    /// Returns the lore description
    pub fn lore(&self) -> &'static str {
        match self {
            Self::Excalibur => "The legendary sword pulled from the stone, destined for true kings.",
            Self::Mjolnir => "Thor's mighty hammer, capable of summoning lightning from the heavens.",
            Self::Gungnir => "The spear of Odin, which never misses its mark.",
            Self::Masamune => "A blade of perfect balance, forged by the legendary smith.",
            Self::DeathsScythe => "The weapon of the Grim Reaper himself.",
            Self::StaffOfAges => "Channeling magic from the dawn of time.",
            Self::SerpentsFang => "Dripping with the venom of the World Serpent.",
            Self::Stormbringer => "A bow that fires bolts of pure lightning.",
            Self::FrostmournesBlade => "Hungering for souls, this blade corrupts all who wield it.",
            Self::InfernosBrand => "Forged in the heart of a dying star.",
            Self::AegisOfTheGods => "Divine protection granted by the gods themselves.",
            Self::DragonhideMantle => "Crafted from the scales of an elder dragon.",
            Self::ValkyriesWings => "Worn by the choosers of the slain.",
            Self::TitansGirdle => "Grants the wearer the strength of a titan.",
            Self::EternityRobes => "Woven from threads of time itself.",
            Self::PhantomShroud => "Makes the wearer one with the shadows.",
            Self::CrystallineCarapace => "Living armor that grows and adapts.",
            Self::BloodlordPlate => "Armor of the vampire king.",
            Self::RingOfOmniscience => "See all that was, is, and will be.",
            Self::AmuletOfYggdrasil => "Contains a seed of the World Tree.",
            Self::CrownOfEternals => "Worn by those who have conquered death.",
            Self::BootsOfHermes => "Move faster than the eye can follow.",
            Self::GlovesOfMidas => "Turn all you touch to gold.",
            Self::CloakOfShadows => "Become one with the darkness.",
            Self::OrbOfDominion => "Bend lesser minds to your will.",
            Self::HeartOfTheVoid => "A fragment of pure nothingness.",
            Self::SoulReaver => "Each kill makes it stronger.",
            Self::WorldEnder => "The weapon that ended the last age.",
            Self::TimeSplitter => "Cuts through time itself.",
            Self::RealityBender => "Warps the fabric of existence.",
            Self::ChaosBringer => "Unpredictable and devastating.",
            Self::OrdersEdge => "Perfect balance in all things.",
            Self::PrimordialFlame => "The first fire, still burning.",
            Self::AbyssalMaw => "Consumes all it touches.",
        }
    }

    /// Returns base stats (attack, defense, hp, mana)
    pub fn base_stats(&self) -> (i32, i32, i32, i32) {
        match self {
            // Weapons - high attack
            Self::Excalibur => (50, 10, 30, 20),
            Self::Mjolnir => (45, 5, 0, 40),
            Self::Gungnir => (55, 0, 0, 10),
            Self::Masamune => (60, 5, 0, 0),
            Self::DeathsScythe => (70, 0, -50, 0),
            Self::StaffOfAges => (25, 5, 20, 100),
            Self::SerpentsFang => (35, 0, 0, 20),
            Self::Stormbringer => (40, 0, 0, 50),
            Self::FrostmournesBlade => (55, 0, -30, 30),
            Self::InfernosBrand => (50, 0, 0, 30),

            // Armor - high defense
            Self::AegisOfTheGods => (0, 40, 50, 30),
            Self::DragonhideMantle => (5, 50, 40, 0),
            Self::ValkyriesWings => (10, 30, 30, 30),
            Self::TitansGirdle => (15, 25, 80, 0),
            Self::EternityRobes => (0, 20, 40, 100),
            Self::PhantomShroud => (10, 25, 0, 40),
            Self::CrystallineCarapace => (0, 45, 30, 50),
            Self::BloodlordPlate => (20, 35, 0, 0),

            // Accessories - balanced
            Self::RingOfOmniscience => (10, 10, 30, 60),
            Self::AmuletOfYggdrasil => (0, 15, 100, 50),
            Self::CrownOfEternals => (15, 15, 50, 50),
            Self::BootsOfHermes => (5, 10, 20, 30),
            Self::GlovesOfMidas => (20, 5, 10, 10),
            Self::CloakOfShadows => (15, 20, 0, 40),
            Self::OrbOfDominion => (10, 0, 0, 80),
            Self::HeartOfTheVoid => (25, 0, -40, 100),

            // Artifacts - extreme power
            Self::SoulReaver => (65, 0, 0, 0),
            Self::WorldEnder => (80, 0, -60, 0),
            Self::TimeSplitter => (45, 10, 20, 40),
            Self::RealityBender => (40, 15, 30, 60),
            Self::ChaosBringer => (50, 0, 0, 50),
            Self::OrdersEdge => (40, 20, 40, 40),
            Self::PrimordialFlame => (55, 0, 0, 40),
            Self::AbyssalMaw => (60, 5, -30, 50),
        }
    }

    /// Returns the equipment slot
    pub fn equip_slot(&self) -> EquipSlot {
        match self {
            Self::Excalibur | Self::Mjolnir | Self::Gungnir | Self::Masamune
            | Self::DeathsScythe | Self::StaffOfAges | Self::SerpentsFang
            | Self::Stormbringer | Self::FrostmournesBlade | Self::InfernosBrand
            | Self::SoulReaver | Self::WorldEnder | Self::TimeSplitter
            | Self::RealityBender | Self::ChaosBringer | Self::OrdersEdge
            | Self::PrimordialFlame | Self::AbyssalMaw => EquipSlot::Weapon,

            Self::AegisOfTheGods => EquipSlot::Shield,
            Self::DragonhideMantle | Self::ValkyriesWings | Self::EternityRobes
            | Self::PhantomShroud | Self::CrystallineCarapace | Self::BloodlordPlate => EquipSlot::Armor,
            Self::TitansGirdle | Self::CloakOfShadows => EquipSlot::Armor,

            Self::CrownOfEternals => EquipSlot::Helmet,
            Self::BootsOfHermes => EquipSlot::Boots,
            Self::GlovesOfMidas => EquipSlot::Gloves,
            Self::RingOfOmniscience | Self::HeartOfTheVoid | Self::OrbOfDominion => EquipSlot::Ring1,
            Self::AmuletOfYggdrasil => EquipSlot::Amulet,
        }
    }

    /// Returns the special effect of this unique item
    pub fn special_effect(&self) -> UniqueEffect {
        match self {
            Self::Excalibur => UniqueEffect::HolySmite,
            Self::Mjolnir => UniqueEffect::LightningStorm,
            Self::Gungnir => UniqueEffect::NeverMiss,
            Self::Masamune => UniqueEffect::PerfectCut,
            Self::DeathsScythe => UniqueEffect::InstantDeath,
            Self::StaffOfAges => UniqueEffect::TimeWarp,
            Self::SerpentsFang => UniqueEffect::DeadlyVenom,
            Self::Stormbringer => UniqueEffect::ChainLightning,
            Self::FrostmournesBlade => UniqueEffect::SoulSteal,
            Self::InfernosBrand => UniqueEffect::Immolate,
            Self::AegisOfTheGods => UniqueEffect::DivineShield,
            Self::DragonhideMantle => UniqueEffect::DragonBreath,
            Self::ValkyriesWings => UniqueEffect::Flight,
            Self::TitansGirdle => UniqueEffect::TitanicStrength,
            Self::EternityRobes => UniqueEffect::TimeStop,
            Self::PhantomShroud => UniqueEffect::Phasing,
            Self::CrystallineCarapace => UniqueEffect::Adaptation,
            Self::BloodlordPlate => UniqueEffect::BloodFeast,
            Self::RingOfOmniscience => UniqueEffect::TrueSight,
            Self::AmuletOfYggdrasil => UniqueEffect::WorldTreeBlessing,
            Self::CrownOfEternals => UniqueEffect::Immortality,
            Self::BootsOfHermes => UniqueEffect::Hyperspeed,
            Self::GlovesOfMidas => UniqueEffect::GoldenTouch,
            Self::CloakOfShadows => UniqueEffect::TrueInvisibility,
            Self::OrbOfDominion => UniqueEffect::MindControl,
            Self::HeartOfTheVoid => UniqueEffect::VoidRift,
            Self::SoulReaver => UniqueEffect::SoulAbsorb,
            Self::WorldEnder => UniqueEffect::Apocalypse,
            Self::TimeSplitter => UniqueEffect::TemporalSlash,
            Self::RealityBender => UniqueEffect::DimensionShift,
            Self::ChaosBringer => UniqueEffect::ChaosStrike,
            Self::OrdersEdge => UniqueEffect::PerfectBalance,
            Self::PrimordialFlame => UniqueEffect::EternalFire,
            Self::AbyssalMaw => UniqueEffect::VoidConsume,
        }
    }
}

/// Special effects unique to legendary items
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum UniqueEffect {
    // Weapon Effects
    HolySmite,        // Extra damage to undead/demons, healing aura
    LightningStorm,   // Chance to call lightning on hit
    NeverMiss,        // Attacks always hit
    PerfectCut,       // Ignore armor, high crit
    InstantDeath,     // Chance to instantly kill
    TimeWarp,         // Slow enemies around you
    DeadlyVenom,      // Stacking poison damage
    ChainLightning,   // Attacks chain to nearby enemies
    SoulSteal,        // Gain stats from kills permanently
    Immolate,         // Set enemies on fire, fire immunity

    // Armor Effects
    DivineShield,     // Block one fatal blow per floor
    DragonBreath,     // Breathe fire attack
    Flight,           // Can fly over obstacles
    TitanicStrength,  // Double carry weight, knockback immunity
    TimeStop,         // Freeze time briefly (long cooldown)
    Phasing,          // Walk through walls
    Adaptation,       // Gain resistance to last damage type
    BloodFeast,       // Heal when enemies die nearby

    // Accessory Effects
    TrueSight,        // See invisible, traps, and secret doors
    WorldTreeBlessing,// Regenerate HP and cure ailments
    Immortality,      // Revive on death once per floor
    Hyperspeed,       // +3 movement speed
    GoldenTouch,      // Enemies drop more gold
    TrueInvisibility, // Permanent invisibility until attack
    MindControl,      // Charm enemies to fight for you
    VoidRift,         // Teleport at will

    // Artifact Effects
    SoulAbsorb,       // Permanent +1 attack per 10 kills
    Apocalypse,       // Devastate entire room (once per floor)
    TemporalSlash,    // Attack hits past, present, and future
    DimensionShift,   // Phase between realities
    ChaosStrike,      // Random powerful effect on hit
    PerfectBalance,   // All stats equalized to highest
    EternalFire,      // Burning aura, fire heals you
    VoidConsume,      // Delete enemies from existence
}

impl UniqueEffect {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::HolySmite => "Holy Smite",
            Self::LightningStorm => "Lightning Storm",
            Self::NeverMiss => "Unerring Strike",
            Self::PerfectCut => "Perfect Cut",
            Self::InstantDeath => "Touch of Death",
            Self::TimeWarp => "Time Warp",
            Self::DeadlyVenom => "Deadly Venom",
            Self::ChainLightning => "Chain Lightning",
            Self::SoulSteal => "Soul Steal",
            Self::Immolate => "Immolate",
            Self::DivineShield => "Divine Shield",
            Self::DragonBreath => "Dragon Breath",
            Self::Flight => "Flight",
            Self::TitanicStrength => "Titanic Strength",
            Self::TimeStop => "Time Stop",
            Self::Phasing => "Phasing",
            Self::Adaptation => "Adaptation",
            Self::BloodFeast => "Blood Feast",
            Self::TrueSight => "True Sight",
            Self::WorldTreeBlessing => "World Tree's Blessing",
            Self::Immortality => "Immortality",
            Self::Hyperspeed => "Hyperspeed",
            Self::GoldenTouch => "Golden Touch",
            Self::TrueInvisibility => "True Invisibility",
            Self::MindControl => "Mind Control",
            Self::VoidRift => "Void Rift",
            Self::SoulAbsorb => "Soul Absorb",
            Self::Apocalypse => "Apocalypse",
            Self::TemporalSlash => "Temporal Slash",
            Self::DimensionShift => "Dimension Shift",
            Self::ChaosStrike => "Chaos Strike",
            Self::PerfectBalance => "Perfect Balance",
            Self::EternalFire => "Eternal Fire",
            Self::VoidConsume => "Void Consume",
        }
    }

    /// Returns a description of the effect
    pub fn description(&self) -> &'static str {
        match self {
            Self::HolySmite => "Extra damage to undead and demons, emit healing aura",
            Self::LightningStorm => "Chance to call down lightning on enemies",
            Self::NeverMiss => "Your attacks never miss",
            Self::PerfectCut => "Ignore enemy armor, high critical chance",
            Self::InstantDeath => "Small chance to instantly kill enemies",
            Self::TimeWarp => "Enemies around you move in slow motion",
            Self::DeadlyVenom => "Poison stacks and deals increasing damage",
            Self::ChainLightning => "Attacks chain to nearby enemies",
            Self::SoulSteal => "Permanently gain +1 attack per 10 kills",
            Self::Immolate => "Set enemies ablaze, immune to fire",
            Self::DivineShield => "Block one fatal blow per floor",
            Self::DragonBreath => "Breathe fire as a special attack",
            Self::Flight => "Fly over obstacles and traps",
            Self::TitanicStrength => "Double carrying capacity, immune to knockback",
            Self::TimeStop => "Freeze time briefly (long cooldown)",
            Self::Phasing => "Walk through walls",
            Self::Adaptation => "Gain resistance to the last damage type received",
            Self::BloodFeast => "Heal when enemies die nearby",
            Self::TrueSight => "See invisible enemies, traps, and secrets",
            Self::WorldTreeBlessing => "Slowly regenerate HP and cure ailments",
            Self::Immortality => "Automatically revive once per floor",
            Self::Hyperspeed => "+3 movement speed",
            Self::GoldenTouch => "Enemies drop significantly more gold",
            Self::TrueInvisibility => "Remain invisible until you attack",
            Self::MindControl => "Charm enemies to fight for you",
            Self::VoidRift => "Teleport anywhere at will",
            Self::SoulAbsorb => "Absorb souls to permanently increase power",
            Self::Apocalypse => "Devastate the entire room once per floor",
            Self::TemporalSlash => "Strike enemies across time",
            Self::DimensionShift => "Shift between dimensions to avoid damage",
            Self::ChaosStrike => "Each hit triggers a random powerful effect",
            Self::PerfectBalance => "All your stats become equal to your highest",
            Self::EternalFire => "Burn everything nearby, fire heals you",
            Self::VoidConsume => "Erase enemies from existence",
        }
    }
}

// ============================================================================
// CRAFTING SYSTEM
// ============================================================================

/// Material types used in crafting
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CraftingMaterial {
    // Basic Materials
    IronOre,
    SteelIngot,
    MithrilOre,
    AdamantiteOre,
    Leather,
    DragonLeather,
    Cloth,
    SilkCloth,

    // Gems
    Ruby,
    Sapphire,
    Emerald,
    Diamond,
    Amethyst,
    Topaz,
    BlackOpal,
    StarSapphire,

    // Essences
    FireEssence,
    IceEssence,
    LightningEssence,
    VoidEssence,
    HolyEssence,
    DarkEssence,
    LifeEssence,
    DeathEssence,

    // Rare Components
    DragonHeart,
    PhoenixFeather,
    DemonCore,
    AngelWing,
    TitanBone,
    ElementalCore,
    AncientRune,
    PrimordialShard,
}

impl CraftingMaterial {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::IronOre => "Iron Ore",
            Self::SteelIngot => "Steel Ingot",
            Self::MithrilOre => "Mithril Ore",
            Self::AdamantiteOre => "Adamantite Ore",
            Self::Leather => "Leather",
            Self::DragonLeather => "Dragon Leather",
            Self::Cloth => "Cloth",
            Self::SilkCloth => "Silk Cloth",
            Self::Ruby => "Ruby",
            Self::Sapphire => "Sapphire",
            Self::Emerald => "Emerald",
            Self::Diamond => "Diamond",
            Self::Amethyst => "Amethyst",
            Self::Topaz => "Topaz",
            Self::BlackOpal => "Black Opal",
            Self::StarSapphire => "Star Sapphire",
            Self::FireEssence => "Fire Essence",
            Self::IceEssence => "Ice Essence",
            Self::LightningEssence => "Lightning Essence",
            Self::VoidEssence => "Void Essence",
            Self::HolyEssence => "Holy Essence",
            Self::DarkEssence => "Dark Essence",
            Self::LifeEssence => "Life Essence",
            Self::DeathEssence => "Death Essence",
            Self::DragonHeart => "Dragon Heart",
            Self::PhoenixFeather => "Phoenix Feather",
            Self::DemonCore => "Demon Core",
            Self::AngelWing => "Angel Wing",
            Self::TitanBone => "Titan Bone",
            Self::ElementalCore => "Elemental Core",
            Self::AncientRune => "Ancient Rune",
            Self::PrimordialShard => "Primordial Shard",
        }
    }

    /// Returns the rarity of this material
    pub fn rarity(&self) -> Rarity {
        match self {
            Self::IronOre | Self::Leather | Self::Cloth => Rarity::Common,
            Self::SteelIngot | Self::SilkCloth => Rarity::Uncommon,
            Self::MithrilOre | Self::DragonLeather | Self::Ruby
            | Self::Sapphire | Self::Emerald => Rarity::Rare,
            Self::AdamantiteOre | Self::Diamond | Self::Amethyst
            | Self::Topaz | Self::FireEssence | Self::IceEssence
            | Self::LightningEssence => Rarity::Epic,
            Self::BlackOpal | Self::StarSapphire | Self::VoidEssence
            | Self::HolyEssence | Self::DarkEssence | Self::LifeEssence
            | Self::DeathEssence | Self::DragonHeart | Self::PhoenixFeather
            | Self::DemonCore | Self::AngelWing => Rarity::Legendary,
            Self::TitanBone | Self::ElementalCore | Self::AncientRune
            | Self::PrimordialShard => Rarity::Mythic,
        }
    }

    /// Returns the glyph for this material
    pub fn glyph(&self) -> char {
        match self {
            Self::IronOre | Self::SteelIngot | Self::MithrilOre
            | Self::AdamantiteOre => '#',
            Self::Leather | Self::DragonLeather => '~',
            Self::Cloth | Self::SilkCloth => '=',
            Self::Ruby | Self::Sapphire | Self::Emerald | Self::Diamond
            | Self::Amethyst | Self::Topaz | Self::BlackOpal | Self::StarSapphire => '*',
            Self::FireEssence | Self::IceEssence | Self::LightningEssence
            | Self::VoidEssence | Self::HolyEssence | Self::DarkEssence
            | Self::LifeEssence | Self::DeathEssence => '@',
            Self::DragonHeart | Self::PhoenixFeather | Self::DemonCore
            | Self::AngelWing | Self::TitanBone | Self::ElementalCore
            | Self::AncientRune | Self::PrimordialShard => '&',
        }
    }
}

/// A crafting recipe
#[derive(Clone, Debug)]
pub struct CraftingRecipe {
    pub name: &'static str,
    pub description: &'static str,
    pub materials: Vec<(CraftingMaterial, u32)>,
    pub result: CraftingResult,
    pub required_level: u32,
}

/// What a recipe produces
#[derive(Clone, Debug)]
pub enum CraftingResult {
    Item(ItemKind, Rarity),
    UniqueItem(UniqueItem),
    Enchantment(EnchantmentType, u8),
    Material(CraftingMaterial, u32),
}

impl CraftingRecipe {
    /// Create a new recipe
    pub fn new(
        name: &'static str,
        description: &'static str,
        materials: Vec<(CraftingMaterial, u32)>,
        result: CraftingResult,
        required_level: u32,
    ) -> Self {
        Self { name, description, materials, result, required_level }
    }
}

/// Returns all available crafting recipes
pub fn get_all_recipes() -> Vec<CraftingRecipe> {
    vec![
        // Basic Weapon Recipes
        CraftingRecipe::new(
            "Forge Steel Sword",
            "Craft a reliable steel sword",
            vec![(CraftingMaterial::SteelIngot, 3), (CraftingMaterial::Leather, 1)],
            CraftingResult::Item(ItemKind::LongSword, Rarity::Uncommon),
            1,
        ),
        CraftingRecipe::new(
            "Forge Mithril Blade",
            "Craft a gleaming mithril blade",
            vec![(CraftingMaterial::MithrilOre, 4), (CraftingMaterial::SteelIngot, 2)],
            CraftingResult::Item(ItemKind::LongSword, Rarity::Rare),
            5,
        ),
        CraftingRecipe::new(
            "Forge Adamantite Greatsword",
            "Craft an indestructible greatsword",
            vec![(CraftingMaterial::AdamantiteOre, 6), (CraftingMaterial::MithrilOre, 2), (CraftingMaterial::Diamond, 1)],
            CraftingResult::Item(ItemKind::Greatsword, Rarity::Epic),
            10,
        ),

        // Elemental Weapons
        CraftingRecipe::new(
            "Forge Flame Sword",
            "Imbue a blade with eternal fire",
            vec![(CraftingMaterial::SteelIngot, 3), (CraftingMaterial::FireEssence, 2), (CraftingMaterial::Ruby, 1)],
            CraftingResult::Item(ItemKind::FlameSword, Rarity::Rare),
            8,
        ),
        CraftingRecipe::new(
            "Forge Frost Blade",
            "Create a blade of eternal ice",
            vec![(CraftingMaterial::SteelIngot, 3), (CraftingMaterial::IceEssence, 2), (CraftingMaterial::Sapphire, 1)],
            CraftingResult::Item(ItemKind::FrostBlade, Rarity::Rare),
            8,
        ),
        CraftingRecipe::new(
            "Forge Thunder Axe",
            "Channel lightning into a mighty axe",
            vec![(CraftingMaterial::AdamantiteOre, 4), (CraftingMaterial::LightningEssence, 3), (CraftingMaterial::Topaz, 2)],
            CraftingResult::Item(ItemKind::ThunderAxe, Rarity::Epic),
            12,
        ),

        // Armor Recipes
        CraftingRecipe::new(
            "Craft Dragon Armor",
            "Fashion armor from dragon scales",
            vec![(CraftingMaterial::DragonLeather, 5), (CraftingMaterial::DragonHeart, 1), (CraftingMaterial::AdamantiteOre, 3)],
            CraftingResult::Item(ItemKind::DragonArmor, Rarity::Legendary),
            15,
        ),
        CraftingRecipe::new(
            "Weave Mage Robes",
            "Create robes infused with magic",
            vec![(CraftingMaterial::SilkCloth, 4), (CraftingMaterial::Amethyst, 2), (CraftingMaterial::VoidEssence, 1)],
            CraftingResult::Item(ItemKind::MageRobes, Rarity::Rare),
            7,
        ),
        CraftingRecipe::new(
            "Forge Holy Armor",
            "Blessed armor of divine protection",
            vec![(CraftingMaterial::MithrilOre, 5), (CraftingMaterial::HolyEssence, 3), (CraftingMaterial::AngelWing, 1)],
            CraftingResult::Item(ItemKind::HolyArmor, Rarity::Legendary),
            18,
        ),

        // Accessory Recipes
        CraftingRecipe::new(
            "Craft Ring of Power",
            "A ring pulsing with strength",
            vec![(CraftingMaterial::MithrilOre, 2), (CraftingMaterial::Ruby, 1), (CraftingMaterial::FireEssence, 1)],
            CraftingResult::Item(ItemKind::RingOfStrength, Rarity::Rare),
            6,
        ),
        CraftingRecipe::new(
            "Craft Amulet of the Gods",
            "Divine protection in amulet form",
            vec![(CraftingMaterial::StarSapphire, 1), (CraftingMaterial::HolyEssence, 2), (CraftingMaterial::LifeEssence, 2), (CraftingMaterial::AngelWing, 1)],
            CraftingResult::Item(ItemKind::AmuletOfTheGods, Rarity::Legendary),
            20,
        ),

        // Enchantment Recipes
        CraftingRecipe::new(
            "Essence of Sharpness",
            "Create an enchantment of sharpness",
            vec![(CraftingMaterial::SteelIngot, 2), (CraftingMaterial::Ruby, 1)],
            CraftingResult::Enchantment(EnchantmentType::Sharpness, 1),
            3,
        ),
        CraftingRecipe::new(
            "Greater Essence of Sharpness",
            "Create a powerful sharpness enchantment",
            vec![(CraftingMaterial::AdamantiteOre, 2), (CraftingMaterial::Diamond, 1), (CraftingMaterial::FireEssence, 1)],
            CraftingResult::Enchantment(EnchantmentType::Sharpness, 3),
            10,
        ),
        CraftingRecipe::new(
            "Essence of Fire Aspect",
            "Imbue weapons with flame",
            vec![(CraftingMaterial::FireEssence, 2), (CraftingMaterial::Ruby, 1)],
            CraftingResult::Enchantment(EnchantmentType::FireAspect, 1),
            5,
        ),
        CraftingRecipe::new(
            "Essence of Lifesteal",
            "Create a vampiric enchantment",
            vec![(CraftingMaterial::DarkEssence, 2), (CraftingMaterial::LifeEssence, 1), (CraftingMaterial::BlackOpal, 1)],
            CraftingResult::Enchantment(EnchantmentType::Lifesteal, 2),
            12,
        ),
        CraftingRecipe::new(
            "Essence of Protection",
            "Create a defensive enchantment",
            vec![(CraftingMaterial::MithrilOre, 2), (CraftingMaterial::Sapphire, 1)],
            CraftingResult::Enchantment(EnchantmentType::Protection, 1),
            4,
        ),

        // Material Conversion Recipes
        CraftingRecipe::new(
            "Smelt Steel",
            "Convert iron into steel",
            vec![(CraftingMaterial::IronOre, 3)],
            CraftingResult::Material(CraftingMaterial::SteelIngot, 1),
            1,
        ),
        CraftingRecipe::new(
            "Refine Dragon Leather",
            "Process raw scales into leather",
            vec![(CraftingMaterial::Leather, 2), (CraftingMaterial::FireEssence, 1)],
            CraftingResult::Material(CraftingMaterial::DragonLeather, 1),
            8,
        ),

        // Legendary Item Recipes
        CraftingRecipe::new(
            "Forge Excalibur",
            "The legendary sword of kings",
            vec![
                (CraftingMaterial::AdamantiteOre, 10),
                (CraftingMaterial::HolyEssence, 5),
                (CraftingMaterial::Diamond, 3),
                (CraftingMaterial::AngelWing, 2),
                (CraftingMaterial::PrimordialShard, 1),
            ],
            CraftingResult::UniqueItem(UniqueItem::Excalibur),
            25,
        ),
        CraftingRecipe::new(
            "Forge Mjolnir",
            "The hammer of the thunder god",
            vec![
                (CraftingMaterial::AdamantiteOre, 8),
                (CraftingMaterial::LightningEssence, 6),
                (CraftingMaterial::TitanBone, 2),
                (CraftingMaterial::ElementalCore, 1),
                (CraftingMaterial::PrimordialShard, 1),
            ],
            CraftingResult::UniqueItem(UniqueItem::Mjolnir),
            25,
        ),
        CraftingRecipe::new(
            "Weave Eternity Robes",
            "Robes woven from time itself",
            vec![
                (CraftingMaterial::SilkCloth, 8),
                (CraftingMaterial::VoidEssence, 4),
                (CraftingMaterial::LifeEssence, 3),
                (CraftingMaterial::StarSapphire, 2),
                (CraftingMaterial::AncientRune, 2),
            ],
            CraftingResult::UniqueItem(UniqueItem::EternityRobes),
            22,
        ),
        CraftingRecipe::new(
            "Forge Death's Scythe",
            "The reaper's own weapon",
            vec![
                (CraftingMaterial::AdamantiteOre, 6),
                (CraftingMaterial::DeathEssence, 5),
                (CraftingMaterial::DarkEssence, 4),
                (CraftingMaterial::DemonCore, 2),
                (CraftingMaterial::PrimordialShard, 1),
            ],
            CraftingResult::UniqueItem(UniqueItem::DeathsScythe),
            25,
        ),
    ]
}

/// All item types in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ItemKind {
    // Potions (20)
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

    // Scrolls (18)
    ScrollTeleport,
    ScrollFireball,
    ScrollIceStorm,
    ScrollLightning,
    ScrollMapping,
    ScrollIdentify,
    ScrollEnchant,
    ScrollSummon,
    ScrollBanish,
    ScrollTimeStop,
    ScrollMassHeal,
    ScrollDeath,
    ScrollEarthquake,
    ScrollMeteor,
    ScrollBlizzard,
    ScrollChainLightning,
    ScrollDivineWrath,
    ScrollDarkness,

    // Weapons (25)
    Dagger,
    ShortSword,
    LongSword,
    Greatsword,
    Axe,
    BattleAxe,
    Mace,
    WarHammer,
    Spear,
    Halberd,
    Staff,
    Bow,
    Crossbow,
    Wand,
    Scythe,
    Katana,
    Rapier,
    Flail,
    Morningstar,
    Trident,
    FlameSword,
    FrostBlade,
    ThunderAxe,
    VoidStaff,
    DemonSlayer,

    // Shields (10)
    Buckler,
    WoodenShield,
    IronShield,
    TowerShield,
    MagicShield,
    DragonShield,
    SpikedShield,
    MirrorShield,
    PhoenixShield,
    AbyssalShield,

    // Armor (12)
    LeatherArmor,
    ChainMail,
    ScaleMail,
    PlateMail,
    DragonArmor,
    MageRobes,
    AssassinGarb,
    HolyArmor,
    DemonArmor,
    CrystalArmor,
    ShadowCloak,
    TitanPlate,

    // Helmets (10)
    LeatherCap,
    IronHelm,
    SteelHelm,
    CrownOfKings,
    WizardHat,
    DemonSkull,
    DragonHelm,
    CrystalCrown,
    HoodOfShadows,
    HelmOfValor,

    // Gloves (8)
    LeatherGloves,
    IronGauntlets,
    GlovesOfPower,
    ThievesGloves,
    DragonGauntlets,
    FrostGauntlets,
    FlameGauntlets,
    GauntletsOfMight,

    // Boots (8)
    LeatherBoots,
    IronBoots,
    BootsOfSpeed,
    BootsOfLeaping,
    WingedBoots,
    ShadowBoots,
    LavaWalkers,
    BootsOfTheWind,

    // Rings (15)
    RingOfStrength,
    RingOfProtection,
    RingOfSpeed,
    RingOfRegeneration,
    RingOfFireball,
    RingOfInvisibility,
    RingOfTheVampire,
    RingOfMana,
    RingOfLuck,
    RingOfDeath,
    RingOfFrost,
    RingOfFlame,
    RingOfThunder,
    RingOfShadows,
    RingOfTheAncients,

    // Amulets (12)
    AmuletOfHealth,
    AmuletOfMana,
    AmuletOfProtection,
    AmuletOfPower,
    AmuletOfWisdom,
    AmuletOfLife,
    AmuletOfDeath,
    AmuletOfTheGods,
    AmuletOfDragons,
    AmuletOfChaos,
    AmuletOfOrder,
    AmuletOfBalance,

    // Food (8)
    Bread,
    Meat,
    Apple,
    Cheese,
    Feast,
    DragonFruit,
    AncientWine,
    GoldenApple,

    // Special (10)
    Gold,
    Key,
    Bomb,
    Torch,
    Compass,
    TeleportCrystal,
    SoulGem,
    AncientRelic,
    DragonScale,
    DemonHeart,
}

impl ItemKind {
    /// Returns the display glyph for the item
    pub fn glyph(&self) -> char {
        match self {
            // Potions
            Self::HealthPotion | Self::ManaPotion | Self::StrengthPotion
            | Self::DefensePotion | Self::SpeedPotion | Self::InvisibilityPotion
            | Self::FireResistPotion | Self::IceResistPotion | Self::PoisonResistPotion
            | Self::RegenerationPotion | Self::BerserkPotion | Self::GiantPotion
            | Self::LevitationPotion | Self::XPPotion | Self::FullRestorePotion
            | Self::LuckPotion | Self::CriticalPotion | Self::VisionPotion
            | Self::CureAllPotion | Self::UltimatePowerPotion => '!',

            // Scrolls
            Self::ScrollTeleport | Self::ScrollFireball | Self::ScrollIceStorm
            | Self::ScrollLightning | Self::ScrollMapping | Self::ScrollIdentify
            | Self::ScrollEnchant | Self::ScrollSummon | Self::ScrollBanish
            | Self::ScrollTimeStop | Self::ScrollMassHeal | Self::ScrollDeath
            | Self::ScrollEarthquake | Self::ScrollMeteor | Self::ScrollBlizzard
            | Self::ScrollChainLightning | Self::ScrollDivineWrath | Self::ScrollDarkness => '?',

            // Weapons
            Self::Dagger | Self::Rapier => '|',
            Self::ShortSword | Self::LongSword | Self::Greatsword | Self::Katana => '/',
            Self::Axe | Self::BattleAxe | Self::ThunderAxe => 'P',
            Self::Mace | Self::WarHammer | Self::Flail | Self::Morningstar => 'T',
            Self::Spear | Self::Halberd | Self::Trident => '|',
            Self::Staff | Self::Wand | Self::VoidStaff => '/',
            Self::Bow | Self::Crossbow => '}',
            Self::Scythe | Self::DemonSlayer => '7',
            Self::FlameSword | Self::FrostBlade => '/',

            // Shields
            Self::Buckler | Self::WoodenShield | Self::IronShield | Self::TowerShield
            | Self::MagicShield | Self::DragonShield | Self::SpikedShield
            | Self::MirrorShield | Self::PhoenixShield | Self::AbyssalShield => ')',

            // Armor
            Self::LeatherArmor | Self::ChainMail | Self::ScaleMail | Self::PlateMail
            | Self::DragonArmor | Self::MageRobes | Self::AssassinGarb | Self::HolyArmor
            | Self::DemonArmor | Self::CrystalArmor | Self::ShadowCloak | Self::TitanPlate => '[',

            // Helmets
            Self::LeatherCap | Self::IronHelm | Self::SteelHelm | Self::CrownOfKings
            | Self::WizardHat | Self::DemonSkull | Self::DragonHelm | Self::CrystalCrown
            | Self::HoodOfShadows | Self::HelmOfValor => '^',

            // Gloves
            Self::LeatherGloves | Self::IronGauntlets | Self::GlovesOfPower
            | Self::ThievesGloves | Self::DragonGauntlets | Self::FrostGauntlets
            | Self::FlameGauntlets | Self::GauntletsOfMight => '{',

            // Boots
            Self::LeatherBoots | Self::IronBoots | Self::BootsOfSpeed
            | Self::BootsOfLeaping | Self::WingedBoots | Self::ShadowBoots
            | Self::LavaWalkers | Self::BootsOfTheWind => '}',

            // Rings
            Self::RingOfStrength | Self::RingOfProtection | Self::RingOfSpeed
            | Self::RingOfRegeneration | Self::RingOfFireball | Self::RingOfInvisibility
            | Self::RingOfTheVampire | Self::RingOfMana | Self::RingOfLuck | Self::RingOfDeath
            | Self::RingOfFrost | Self::RingOfFlame | Self::RingOfThunder
            | Self::RingOfShadows | Self::RingOfTheAncients => 'o',

            // Amulets
            Self::AmuletOfHealth | Self::AmuletOfMana | Self::AmuletOfProtection
            | Self::AmuletOfPower | Self::AmuletOfWisdom | Self::AmuletOfLife
            | Self::AmuletOfDeath | Self::AmuletOfTheGods | Self::AmuletOfDragons
            | Self::AmuletOfChaos | Self::AmuletOfOrder | Self::AmuletOfBalance => '"',

            // Food
            Self::Bread | Self::Meat | Self::Apple | Self::Cheese | Self::Feast
            | Self::DragonFruit | Self::AncientWine | Self::GoldenApple => '%',

            // Special
            Self::Gold => '$',
            Self::Key => 'k',
            Self::Bomb => '*',
            Self::Torch => '(',
            Self::Compass => 'c',
            Self::TeleportCrystal => '+',
            Self::SoulGem => 'o',
            Self::AncientRelic => '*',
            Self::DragonScale => 's',
            Self::DemonHeart => 'h',
        }
    }

    /// Returns the display name of the item
    pub fn name(&self) -> &'static str {
        match self {
            Self::HealthPotion => "Health Potion",
            Self::ManaPotion => "Mana Potion",
            Self::StrengthPotion => "Strength Potion",
            Self::DefensePotion => "Defense Potion",
            Self::SpeedPotion => "Speed Potion",
            Self::InvisibilityPotion => "Invisibility Potion",
            Self::FireResistPotion => "Fire Resist Potion",
            Self::IceResistPotion => "Ice Resist Potion",
            Self::PoisonResistPotion => "Antidote",
            Self::RegenerationPotion => "Regeneration Potion",
            Self::BerserkPotion => "Berserk Potion",
            Self::GiantPotion => "Giant's Strength",
            Self::LevitationPotion => "Levitation Potion",
            Self::XPPotion => "Potion of Experience",
            Self::FullRestorePotion => "Full Restore Elixir",
            Self::LuckPotion => "Luck Potion",
            Self::CriticalPotion => "Critical Strike Potion",
            Self::VisionPotion => "Potion of True Sight",
            Self::CureAllPotion => "Cure All Elixir",
            Self::UltimatePowerPotion => "Ultimate Power Elixir",

            Self::ScrollTeleport => "Scroll of Teleport",
            Self::ScrollFireball => "Scroll of Fireball",
            Self::ScrollIceStorm => "Scroll of Ice Storm",
            Self::ScrollLightning => "Scroll of Lightning",
            Self::ScrollMapping => "Scroll of Mapping",
            Self::ScrollIdentify => "Scroll of Identify",
            Self::ScrollEnchant => "Scroll of Enchant",
            Self::ScrollSummon => "Scroll of Summoning",
            Self::ScrollBanish => "Scroll of Banishment",
            Self::ScrollTimeStop => "Scroll of Time Stop",
            Self::ScrollMassHeal => "Scroll of Mass Heal",
            Self::ScrollDeath => "Scroll of Death",
            Self::ScrollEarthquake => "Scroll of Earthquake",
            Self::ScrollMeteor => "Scroll of Meteor",
            Self::ScrollBlizzard => "Scroll of Blizzard",
            Self::ScrollChainLightning => "Scroll of Chain Lightning",
            Self::ScrollDivineWrath => "Scroll of Divine Wrath",
            Self::ScrollDarkness => "Scroll of Darkness",

            Self::Dagger => "Dagger",
            Self::ShortSword => "Short Sword",
            Self::LongSword => "Long Sword",
            Self::Greatsword => "Greatsword",
            Self::Axe => "Battle Axe",
            Self::BattleAxe => "Great Axe",
            Self::Mace => "Mace",
            Self::WarHammer => "War Hammer",
            Self::Spear => "Spear",
            Self::Halberd => "Halberd",
            Self::Staff => "Staff",
            Self::Bow => "Bow",
            Self::Crossbow => "Crossbow",
            Self::Wand => "Wand",
            Self::Scythe => "Scythe",
            Self::Katana => "Katana",
            Self::Rapier => "Rapier",
            Self::Flail => "Flail",
            Self::Morningstar => "Morningstar",
            Self::Trident => "Trident",
            Self::FlameSword => "Flame Sword",
            Self::FrostBlade => "Frost Blade",
            Self::ThunderAxe => "Thunder Axe",
            Self::VoidStaff => "Void Staff",
            Self::DemonSlayer => "Demon Slayer",

            Self::Buckler => "Buckler",
            Self::WoodenShield => "Wooden Shield",
            Self::IronShield => "Iron Shield",
            Self::TowerShield => "Tower Shield",
            Self::MagicShield => "Magic Shield",
            Self::DragonShield => "Dragon Shield",
            Self::SpikedShield => "Spiked Shield",
            Self::MirrorShield => "Mirror Shield",
            Self::PhoenixShield => "Phoenix Shield",
            Self::AbyssalShield => "Abyssal Shield",

            Self::LeatherArmor => "Leather Armor",
            Self::ChainMail => "Chain Mail",
            Self::ScaleMail => "Scale Mail",
            Self::PlateMail => "Plate Mail",
            Self::DragonArmor => "Dragon Armor",
            Self::MageRobes => "Mage Robes",
            Self::AssassinGarb => "Assassin's Garb",
            Self::HolyArmor => "Holy Armor",
            Self::DemonArmor => "Demon Armor",
            Self::CrystalArmor => "Crystal Armor",
            Self::ShadowCloak => "Shadow Cloak",
            Self::TitanPlate => "Titan Plate",

            Self::LeatherCap => "Leather Cap",
            Self::IronHelm => "Iron Helm",
            Self::SteelHelm => "Steel Helm",
            Self::CrownOfKings => "Crown of Kings",
            Self::WizardHat => "Wizard's Hat",
            Self::DemonSkull => "Demon Skull",
            Self::DragonHelm => "Dragon Helm",
            Self::CrystalCrown => "Crystal Crown",
            Self::HoodOfShadows => "Hood of Shadows",
            Self::HelmOfValor => "Helm of Valor",

            Self::LeatherGloves => "Leather Gloves",
            Self::IronGauntlets => "Iron Gauntlets",
            Self::GlovesOfPower => "Gloves of Power",
            Self::ThievesGloves => "Thief's Gloves",
            Self::DragonGauntlets => "Dragon Gauntlets",
            Self::FrostGauntlets => "Frost Gauntlets",
            Self::FlameGauntlets => "Flame Gauntlets",
            Self::GauntletsOfMight => "Gauntlets of Might",

            Self::LeatherBoots => "Leather Boots",
            Self::IronBoots => "Iron Boots",
            Self::BootsOfSpeed => "Boots of Speed",
            Self::BootsOfLeaping => "Boots of Leaping",
            Self::WingedBoots => "Winged Boots",
            Self::ShadowBoots => "Shadow Boots",
            Self::LavaWalkers => "Lava Walkers",
            Self::BootsOfTheWind => "Boots of the Wind",

            Self::RingOfStrength => "Ring of Strength",
            Self::RingOfProtection => "Ring of Protection",
            Self::RingOfSpeed => "Ring of Speed",
            Self::RingOfRegeneration => "Ring of Regeneration",
            Self::RingOfFireball => "Ring of Fireball",
            Self::RingOfInvisibility => "Ring of Invisibility",
            Self::RingOfTheVampire => "Vampire Ring",
            Self::RingOfMana => "Ring of Mana",
            Self::RingOfLuck => "Ring of Luck",
            Self::RingOfDeath => "Ring of Death",
            Self::RingOfFrost => "Ring of Frost",
            Self::RingOfFlame => "Ring of Flame",
            Self::RingOfThunder => "Ring of Thunder",
            Self::RingOfShadows => "Ring of Shadows",
            Self::RingOfTheAncients => "Ring of the Ancients",

            Self::AmuletOfHealth => "Amulet of Health",
            Self::AmuletOfMana => "Amulet of Mana",
            Self::AmuletOfProtection => "Amulet of Protection",
            Self::AmuletOfPower => "Amulet of Power",
            Self::AmuletOfWisdom => "Amulet of Wisdom",
            Self::AmuletOfLife => "Amulet of Life",
            Self::AmuletOfDeath => "Amulet of Death",
            Self::AmuletOfTheGods => "Amulet of the Gods",
            Self::AmuletOfDragons => "Amulet of Dragons",
            Self::AmuletOfChaos => "Amulet of Chaos",
            Self::AmuletOfOrder => "Amulet of Order",
            Self::AmuletOfBalance => "Amulet of Balance",

            Self::Bread => "Bread",
            Self::Meat => "Meat",
            Self::Apple => "Apple",
            Self::Cheese => "Cheese",
            Self::Feast => "Royal Feast",
            Self::DragonFruit => "Dragon Fruit",
            Self::AncientWine => "Ancient Wine",
            Self::GoldenApple => "Golden Apple",

            Self::Gold => "Gold",
            Self::Key => "Key",
            Self::Bomb => "Bomb",
            Self::Torch => "Torch",
            Self::Compass => "Compass",
            Self::TeleportCrystal => "Teleport Crystal",
            Self::SoulGem => "Soul Gem",
            Self::AncientRelic => "Ancient Relic",
            Self::DragonScale => "Dragon Scale",
            Self::DemonHeart => "Demon Heart",
        }
    }

    /// Returns the equipment slot for this item, if equippable
    pub fn equip_slot(&self) -> Option<EquipSlot> {
        match self {
            Self::Dagger | Self::ShortSword | Self::LongSword | Self::Greatsword
            | Self::Axe | Self::BattleAxe | Self::Mace | Self::WarHammer
            | Self::Spear | Self::Halberd | Self::Staff | Self::Bow
            | Self::Crossbow | Self::Wand | Self::Scythe
            | Self::Katana | Self::Rapier | Self::Flail | Self::Morningstar
            | Self::Trident | Self::FlameSword | Self::FrostBlade | Self::ThunderAxe
            | Self::VoidStaff | Self::DemonSlayer => Some(EquipSlot::Weapon),

            Self::Buckler | Self::WoodenShield | Self::IronShield
            | Self::TowerShield | Self::MagicShield | Self::DragonShield
            | Self::SpikedShield | Self::MirrorShield | Self::PhoenixShield
            | Self::AbyssalShield => Some(EquipSlot::Shield),

            Self::LeatherArmor | Self::ChainMail | Self::ScaleMail | Self::PlateMail
            | Self::DragonArmor | Self::MageRobes | Self::AssassinGarb
            | Self::HolyArmor | Self::DemonArmor | Self::CrystalArmor
            | Self::ShadowCloak | Self::TitanPlate => Some(EquipSlot::Armor),

            Self::LeatherCap | Self::IronHelm | Self::SteelHelm
            | Self::CrownOfKings | Self::WizardHat | Self::DemonSkull
            | Self::DragonHelm | Self::CrystalCrown | Self::HoodOfShadows
            | Self::HelmOfValor => Some(EquipSlot::Helmet),

            Self::LeatherGloves | Self::IronGauntlets | Self::GlovesOfPower
            | Self::ThievesGloves | Self::DragonGauntlets
            | Self::FrostGauntlets | Self::FlameGauntlets | Self::GauntletsOfMight => Some(EquipSlot::Gloves),

            Self::LeatherBoots | Self::IronBoots | Self::BootsOfSpeed
            | Self::BootsOfLeaping | Self::WingedBoots
            | Self::ShadowBoots | Self::LavaWalkers | Self::BootsOfTheWind => Some(EquipSlot::Boots),

            Self::RingOfStrength | Self::RingOfProtection | Self::RingOfSpeed
            | Self::RingOfRegeneration | Self::RingOfFireball | Self::RingOfInvisibility
            | Self::RingOfTheVampire | Self::RingOfMana | Self::RingOfLuck
            | Self::RingOfDeath | Self::RingOfFrost | Self::RingOfFlame
            | Self::RingOfThunder | Self::RingOfShadows | Self::RingOfTheAncients => Some(EquipSlot::Ring1),

            Self::AmuletOfHealth | Self::AmuletOfMana | Self::AmuletOfProtection
            | Self::AmuletOfPower | Self::AmuletOfWisdom | Self::AmuletOfLife
            | Self::AmuletOfDeath | Self::AmuletOfTheGods
            | Self::AmuletOfDragons | Self::AmuletOfChaos | Self::AmuletOfOrder
            | Self::AmuletOfBalance => Some(EquipSlot::Amulet),

            _ => None,
        }
    }

    /// Returns the base stats for this item (attack, defense, hp_bonus, mana_bonus)
    pub fn base_stats(&self) -> (i32, i32, i32, i32) {
        match self {
            // Weapons
            Self::Dagger => (3, 0, 0, 0),
            Self::ShortSword => (5, 0, 0, 0),
            Self::LongSword => (8, 0, 0, 0),
            Self::Greatsword => (12, 0, 0, 0),
            Self::Axe => (7, 0, 0, 0),
            Self::BattleAxe => (14, 0, 0, 0),
            Self::Mace => (6, 1, 0, 0),
            Self::WarHammer => (10, 2, 0, 0),
            Self::Spear => (6, 0, 0, 0),
            Self::Halberd => (11, 1, 0, 0),
            Self::Staff => (4, 0, 0, 20),
            Self::Bow => (7, 0, 0, 0),
            Self::Crossbow => (10, 0, 0, 0),
            Self::Wand => (3, 0, 0, 30),
            Self::Scythe => (15, 0, 0, 0),
            Self::Katana => (11, 0, 0, 0),
            Self::Rapier => (9, 1, 0, 0),
            Self::Flail => (10, 0, 0, 0),
            Self::Morningstar => (12, 1, 0, 0),
            Self::Trident => (10, 2, 0, 0),
            Self::FlameSword => (14, 0, 0, 5),
            Self::FrostBlade => (13, 0, 0, 5),
            Self::ThunderAxe => (16, 0, 0, 10),
            Self::VoidStaff => (8, 0, 0, 50),
            Self::DemonSlayer => (20, 0, 0, 0),

            // Shields
            Self::Buckler => (0, 2, 0, 0),
            Self::WoodenShield => (0, 3, 0, 0),
            Self::IronShield => (0, 5, 0, 0),
            Self::TowerShield => (0, 8, 0, 0),
            Self::MagicShield => (0, 6, 0, 10),
            Self::DragonShield => (0, 10, 10, 0),
            Self::SpikedShield => (3, 6, 0, 0),
            Self::MirrorShield => (0, 7, 0, 15),
            Self::PhoenixShield => (0, 9, 15, 5),
            Self::AbyssalShield => (2, 12, 0, 10),

            // Armor
            Self::LeatherArmor => (0, 3, 0, 0),
            Self::ChainMail => (0, 5, 0, 0),
            Self::ScaleMail => (0, 7, 0, 0),
            Self::PlateMail => (0, 10, 0, 0),
            Self::DragonArmor => (0, 15, 20, 0),
            Self::MageRobes => (0, 2, 0, 30),
            Self::AssassinGarb => (3, 4, 0, 0),
            Self::HolyArmor => (0, 12, 10, 10),
            Self::DemonArmor => (5, 14, 0, 0),
            Self::CrystalArmor => (0, 11, 0, 25),
            Self::ShadowCloak => (3, 6, 0, 15),
            Self::TitanPlate => (0, 18, 30, 0),

            // Helmets
            Self::LeatherCap => (0, 1, 0, 0),
            Self::IronHelm => (0, 3, 0, 0),
            Self::SteelHelm => (0, 5, 0, 0),
            Self::CrownOfKings => (2, 3, 20, 20),
            Self::WizardHat => (0, 1, 0, 20),
            Self::DemonSkull => (5, 2, 0, 0),
            Self::DragonHelm => (2, 6, 10, 0),
            Self::CrystalCrown => (0, 4, 10, 25),
            Self::HoodOfShadows => (2, 2, 0, 15),
            Self::HelmOfValor => (3, 5, 15, 0),

            // Gloves
            Self::LeatherGloves => (1, 0, 0, 0),
            Self::IronGauntlets => (2, 1, 0, 0),
            Self::GlovesOfPower => (5, 0, 0, 0),
            Self::ThievesGloves => (3, 0, 0, 0),
            Self::DragonGauntlets => (4, 3, 0, 0),
            Self::FrostGauntlets => (3, 2, 0, 10),
            Self::FlameGauntlets => (5, 1, 0, 5),
            Self::GauntletsOfMight => (7, 2, 5, 0),

            // Boots
            Self::LeatherBoots => (0, 1, 0, 0),
            Self::IronBoots => (0, 2, 0, 0),
            Self::BootsOfSpeed => (0, 1, 0, 0),
            Self::BootsOfLeaping => (0, 1, 0, 0),
            Self::WingedBoots => (0, 2, 0, 10),
            Self::ShadowBoots => (2, 1, 0, 10),
            Self::LavaWalkers => (0, 3, 0, 0),
            Self::BootsOfTheWind => (0, 2, 0, 15),

            // Rings
            Self::RingOfStrength => (5, 0, 0, 0),
            Self::RingOfProtection => (0, 5, 0, 0),
            Self::RingOfSpeed => (0, 0, 0, 0),
            Self::RingOfRegeneration => (0, 0, 10, 0),
            Self::RingOfFireball => (3, 0, 0, 10),
            Self::RingOfInvisibility => (0, 0, 0, 0),
            Self::RingOfTheVampire => (3, 0, 0, 0),
            Self::RingOfMana => (0, 0, 0, 30),
            Self::RingOfLuck => (1, 1, 5, 5),
            Self::RingOfDeath => (10, 0, -20, 0),
            Self::RingOfFrost => (2, 0, 0, 15),
            Self::RingOfFlame => (4, 0, 0, 10),
            Self::RingOfThunder => (5, 0, 0, 20),
            Self::RingOfShadows => (0, 2, 0, 20),
            Self::RingOfTheAncients => (6, 3, 15, 15),

            // Amulets
            Self::AmuletOfHealth => (0, 0, 30, 0),
            Self::AmuletOfMana => (0, 0, 0, 40),
            Self::AmuletOfProtection => (0, 8, 0, 0),
            Self::AmuletOfPower => (8, 0, 0, 0),
            Self::AmuletOfWisdom => (0, 0, 0, 50),
            Self::AmuletOfLife => (0, 0, 50, 0),
            Self::AmuletOfDeath => (15, 0, -30, 0),
            Self::AmuletOfTheGods => (5, 5, 25, 25),
            Self::AmuletOfDragons => (8, 4, 20, 10),
            Self::AmuletOfChaos => (12, 0, -10, 30),
            Self::AmuletOfOrder => (0, 10, 20, 20),
            Self::AmuletOfBalance => (6, 6, 20, 20),

            _ => (0, 0, 0, 0),
        }
    }

    /// Returns whether this item is consumable
    pub fn is_consumable(&self) -> bool {
        matches!(
            self,
            Self::HealthPotion | Self::ManaPotion | Self::StrengthPotion
                | Self::DefensePotion | Self::SpeedPotion | Self::InvisibilityPotion
                | Self::FireResistPotion | Self::IceResistPotion | Self::PoisonResistPotion
                | Self::RegenerationPotion | Self::BerserkPotion | Self::GiantPotion
                | Self::LevitationPotion | Self::XPPotion | Self::FullRestorePotion
                | Self::LuckPotion | Self::CriticalPotion | Self::VisionPotion
                | Self::CureAllPotion | Self::UltimatePowerPotion
                | Self::ScrollTeleport | Self::ScrollFireball | Self::ScrollIceStorm
                | Self::ScrollLightning | Self::ScrollMapping | Self::ScrollIdentify
                | Self::ScrollEnchant | Self::ScrollSummon | Self::ScrollBanish
                | Self::ScrollTimeStop | Self::ScrollMassHeal | Self::ScrollDeath
                | Self::ScrollEarthquake | Self::ScrollMeteor | Self::ScrollBlizzard
                | Self::ScrollChainLightning | Self::ScrollDivineWrath | Self::ScrollDarkness
                | Self::Bread | Self::Meat | Self::Apple | Self::Cheese | Self::Feast
                | Self::DragonFruit | Self::AncientWine | Self::GoldenApple
                | Self::Bomb | Self::Torch
                | Self::TeleportCrystal | Self::SoulGem | Self::AncientRelic
                | Self::DragonScale | Self::DemonHeart
        )
    }

    /// Returns whether this item is food
    pub fn is_food(&self) -> bool {
        matches!(
            self,
            Self::Bread | Self::Meat | Self::Apple | Self::Cheese | Self::Feast
                | Self::DragonFruit | Self::AncientWine | Self::GoldenApple
        )
    }

    /// Returns the hunger value restored by this food item
    pub fn food_value(&self) -> i32 {
        match self {
            Self::Apple => 10,
            Self::Bread => 25,
            Self::Cheese => 20,
            Self::Meat => 40,
            Self::Feast => 100,
            Self::DragonFruit => 30,
            Self::AncientWine => 35,
            Self::GoldenApple => 50,
            _ => 0,
        }
    }
}

/// Represents an item instance in the game
#[derive(Clone, Debug)]
pub struct Item {
    /// X coordinate on the map (when dropped)
    pub x: usize,
    /// Y coordinate on the map (when dropped)
    pub y: usize,
    /// Type of item
    pub kind: ItemKind,
    /// Rarity of the item
    pub rarity: Rarity,
}

impl Item {
    /// Creates a new item of the given type at the specified position
    pub fn new(x: usize, y: usize, kind: ItemKind, rarity: Rarity) -> Self {
        Self { x, y, kind, rarity }
    }

    /// Returns the computed stats for this item based on rarity
    pub fn stats(&self) -> (i32, i32, i32, i32) {
        let (atk, def, hp, mana) = self.kind.base_stats();
        let mult = self.rarity.stat_bonus();
        (
            (atk as f32 * mult) as i32,
            (def as f32 * mult) as i32,
            (hp as f32 * mult) as i32,
            (mana as f32 * mult) as i32,
        )
    }

    /// Returns the display name including rarity prefix
    pub fn display_name(&self) -> String {
        format!("{}{}", self.rarity.prefix(), self.kind.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new(5, 5, ItemKind::LongSword, Rarity::Rare);
        assert_eq!(item.kind, ItemKind::LongSword);
        assert_eq!(item.rarity, Rarity::Rare);
    }

    #[test]
    fn test_item_stats_scale_with_rarity() {
        let common = Item::new(0, 0, ItemKind::LongSword, Rarity::Common);
        let epic = Item::new(0, 0, ItemKind::LongSword, Rarity::Epic);
        let (common_atk, _, _, _) = common.stats();
        let (epic_atk, _, _, _) = epic.stats();
        assert!(epic_atk > common_atk);
    }

    #[test]
    fn test_item_display_name() {
        let item = Item::new(0, 0, ItemKind::IronShield, Rarity::Legendary);
        assert_eq!(item.display_name(), "Legendary Iron Shield");
    }

    #[test]
    fn test_equip_slot() {
        assert_eq!(ItemKind::LongSword.equip_slot(), Some(EquipSlot::Weapon));
        assert_eq!(ItemKind::PlateMail.equip_slot(), Some(EquipSlot::Armor));
        assert_eq!(ItemKind::HealthPotion.equip_slot(), None);
    }
}

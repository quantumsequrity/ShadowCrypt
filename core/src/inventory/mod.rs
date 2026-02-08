//! Comprehensive Inventory Management System
//!
//! This module provides a complete inventory system with:
//! - Multiple storage types (main bag, equipment, quick slots, quest items, etc.)
//! - Item categorization and filtering
//! - Stacking and splitting
//! - Weight/encumbrance system
//! - Auto-loot settings
//! - Personal and shared storage

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::items::{
    CraftingMaterial, Enchantment, EquipSlot as BaseEquipSlot, Item, ItemKind, Rarity,
};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Default main bag capacity
pub const DEFAULT_BAG_CAPACITY: usize = 20;
/// Maximum main bag capacity
pub const MAX_BAG_CAPACITY: usize = 100;
/// Number of quick slots
pub const QUICK_SLOT_COUNT: usize = 10;
/// Default personal stash capacity
pub const DEFAULT_STASH_CAPACITY: usize = 50;
/// Maximum personal stash capacity
pub const MAX_STASH_CAPACITY: usize = 200;
/// Default bank capacity
pub const DEFAULT_BANK_CAPACITY: usize = 100;
/// Maximum bank capacity
pub const MAX_BANK_CAPACITY: usize = 500;
/// Default guild storage capacity
pub const DEFAULT_GUILD_STORAGE_CAPACITY: usize = 200;
/// Maximum stack size for stackable items
pub const MAX_STACK_SIZE: u32 = 999;
/// Base carry capacity (weight units)
pub const BASE_CARRY_CAPACITY: f32 = 100.0;
/// Carry capacity per strength point
pub const CARRY_PER_STRENGTH: f32 = 5.0;

// ============================================================================
// EQUIPMENT SLOTS (Extended)
// ============================================================================

/// Extended equipment slots for the inventory system
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EquipmentSlot {
    /// Head armor (helmets, crowns, hoods)
    Head,
    /// Chest armor (armor, robes, cloaks)
    Chest,
    /// Leg armor (greaves, pants)
    Legs,
    /// Foot armor (boots, shoes)
    Feet,
    /// Hand armor (gloves, gauntlets)
    Hands,
    /// First ring slot
    Ring1,
    /// Second ring slot
    Ring2,
    /// Amulet/necklace slot
    Amulet,
    /// Main weapon slot
    Weapon,
    /// Off-hand slot (shield, secondary weapon, tome)
    Offhand,
    /// Back slot (capes, wings, quivers)
    Back,
    /// Belt slot (utility items)
    Belt,
}

impl EquipmentSlot {
    /// Get all equipment slots
    pub fn all() -> Vec<EquipmentSlot> {
        vec![
            Self::Head,
            Self::Chest,
            Self::Legs,
            Self::Feet,
            Self::Hands,
            Self::Ring1,
            Self::Ring2,
            Self::Amulet,
            Self::Weapon,
            Self::Offhand,
            Self::Back,
            Self::Belt,
        ]
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Head => "Head",
            Self::Chest => "Chest",
            Self::Legs => "Legs",
            Self::Feet => "Feet",
            Self::Hands => "Hands",
            Self::Ring1 => "Ring (Left)",
            Self::Ring2 => "Ring (Right)",
            Self::Amulet => "Amulet",
            Self::Weapon => "Weapon",
            Self::Offhand => "Off-hand",
            Self::Back => "Back",
            Self::Belt => "Belt",
        }
    }

    /// Convert from base EquipSlot
    pub fn from_base(slot: BaseEquipSlot) -> Self {
        match slot {
            BaseEquipSlot::Weapon => Self::Weapon,
            BaseEquipSlot::Shield => Self::Offhand,
            BaseEquipSlot::Helmet => Self::Head,
            BaseEquipSlot::Armor => Self::Chest,
            BaseEquipSlot::Gloves => Self::Hands,
            BaseEquipSlot::Boots => Self::Feet,
            BaseEquipSlot::Ring1 => Self::Ring1,
            BaseEquipSlot::Ring2 => Self::Ring2,
            BaseEquipSlot::Amulet => Self::Amulet,
        }
    }

    /// Convert to base EquipSlot if possible
    pub fn to_base(&self) -> Option<BaseEquipSlot> {
        match self {
            Self::Weapon => Some(BaseEquipSlot::Weapon),
            Self::Offhand => Some(BaseEquipSlot::Shield),
            Self::Head => Some(BaseEquipSlot::Helmet),
            Self::Chest => Some(BaseEquipSlot::Armor),
            Self::Hands => Some(BaseEquipSlot::Gloves),
            Self::Feet => Some(BaseEquipSlot::Boots),
            Self::Ring1 => Some(BaseEquipSlot::Ring1),
            Self::Ring2 => Some(BaseEquipSlot::Ring2),
            Self::Amulet => Some(BaseEquipSlot::Amulet),
            Self::Legs | Self::Back | Self::Belt => None,
        }
    }
}

// ============================================================================
// ITEM CATEGORIES
// ============================================================================

/// Categories for organizing items
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ItemCategory {
    /// Swords, axes, bows, staves, etc.
    Weapons,
    /// Helmets, armor, shields, etc.
    Armor,
    /// Rings, amulets, belts
    Accessories,
    /// Potions, scrolls, food
    Consumables,
    /// Crafting materials
    Materials,
    /// Quest-related items
    QuestItems,
    /// Rare collectibles
    Collectibles,
    /// Low-value items for selling
    Junk,
    /// Currency items
    Currency,
    /// Keys and key items
    KeyItems,
}

impl ItemCategory {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Weapons => "Weapons",
            Self::Armor => "Armor",
            Self::Accessories => "Accessories",
            Self::Consumables => "Consumables",
            Self::Materials => "Materials",
            Self::QuestItems => "Quest Items",
            Self::Collectibles => "Collectibles",
            Self::Junk => "Junk",
            Self::Currency => "Currency",
            Self::KeyItems => "Key Items",
        }
    }

    /// Determine category from ItemKind
    pub fn from_item_kind(kind: ItemKind) -> Self {
        match kind {
            // Weapons
            ItemKind::Dagger
            | ItemKind::ShortSword
            | ItemKind::LongSword
            | ItemKind::Greatsword
            | ItemKind::Axe
            | ItemKind::BattleAxe
            | ItemKind::Mace
            | ItemKind::WarHammer
            | ItemKind::Spear
            | ItemKind::Halberd
            | ItemKind::Staff
            | ItemKind::Bow
            | ItemKind::Crossbow
            | ItemKind::Wand
            | ItemKind::Scythe
            | ItemKind::Katana
            | ItemKind::Rapier
            | ItemKind::Flail
            | ItemKind::Morningstar
            | ItemKind::Trident
            | ItemKind::FlameSword
            | ItemKind::FrostBlade
            | ItemKind::ThunderAxe
            | ItemKind::VoidStaff
            | ItemKind::DemonSlayer => Self::Weapons,

            // Armor (including shields)
            ItemKind::Buckler
            | ItemKind::WoodenShield
            | ItemKind::IronShield
            | ItemKind::TowerShield
            | ItemKind::MagicShield
            | ItemKind::DragonShield
            | ItemKind::SpikedShield
            | ItemKind::MirrorShield
            | ItemKind::PhoenixShield
            | ItemKind::AbyssalShield
            | ItemKind::LeatherArmor
            | ItemKind::ChainMail
            | ItemKind::ScaleMail
            | ItemKind::PlateMail
            | ItemKind::DragonArmor
            | ItemKind::MageRobes
            | ItemKind::AssassinGarb
            | ItemKind::HolyArmor
            | ItemKind::DemonArmor
            | ItemKind::CrystalArmor
            | ItemKind::ShadowCloak
            | ItemKind::TitanPlate
            | ItemKind::LeatherCap
            | ItemKind::IronHelm
            | ItemKind::SteelHelm
            | ItemKind::CrownOfKings
            | ItemKind::WizardHat
            | ItemKind::DemonSkull
            | ItemKind::DragonHelm
            | ItemKind::CrystalCrown
            | ItemKind::HoodOfShadows
            | ItemKind::HelmOfValor
            | ItemKind::LeatherGloves
            | ItemKind::IronGauntlets
            | ItemKind::GlovesOfPower
            | ItemKind::ThievesGloves
            | ItemKind::DragonGauntlets
            | ItemKind::FrostGauntlets
            | ItemKind::FlameGauntlets
            | ItemKind::GauntletsOfMight
            | ItemKind::LeatherBoots
            | ItemKind::IronBoots
            | ItemKind::BootsOfSpeed
            | ItemKind::BootsOfLeaping
            | ItemKind::WingedBoots
            | ItemKind::ShadowBoots
            | ItemKind::LavaWalkers
            | ItemKind::BootsOfTheWind => Self::Armor,

            // Accessories
            ItemKind::RingOfStrength
            | ItemKind::RingOfProtection
            | ItemKind::RingOfSpeed
            | ItemKind::RingOfRegeneration
            | ItemKind::RingOfFireball
            | ItemKind::RingOfInvisibility
            | ItemKind::RingOfTheVampire
            | ItemKind::RingOfMana
            | ItemKind::RingOfLuck
            | ItemKind::RingOfDeath
            | ItemKind::RingOfFrost
            | ItemKind::RingOfFlame
            | ItemKind::RingOfThunder
            | ItemKind::RingOfShadows
            | ItemKind::RingOfTheAncients
            | ItemKind::AmuletOfHealth
            | ItemKind::AmuletOfMana
            | ItemKind::AmuletOfProtection
            | ItemKind::AmuletOfPower
            | ItemKind::AmuletOfWisdom
            | ItemKind::AmuletOfLife
            | ItemKind::AmuletOfDeath
            | ItemKind::AmuletOfTheGods
            | ItemKind::AmuletOfDragons
            | ItemKind::AmuletOfChaos
            | ItemKind::AmuletOfOrder
            | ItemKind::AmuletOfBalance => Self::Accessories,

            // Consumables
            ItemKind::HealthPotion
            | ItemKind::ManaPotion
            | ItemKind::StrengthPotion
            | ItemKind::DefensePotion
            | ItemKind::SpeedPotion
            | ItemKind::InvisibilityPotion
            | ItemKind::FireResistPotion
            | ItemKind::IceResistPotion
            | ItemKind::PoisonResistPotion
            | ItemKind::RegenerationPotion
            | ItemKind::BerserkPotion
            | ItemKind::GiantPotion
            | ItemKind::LevitationPotion
            | ItemKind::XPPotion
            | ItemKind::FullRestorePotion
            | ItemKind::LuckPotion
            | ItemKind::CriticalPotion
            | ItemKind::VisionPotion
            | ItemKind::CureAllPotion
            | ItemKind::UltimatePowerPotion
            | ItemKind::ScrollTeleport
            | ItemKind::ScrollFireball
            | ItemKind::ScrollIceStorm
            | ItemKind::ScrollLightning
            | ItemKind::ScrollMapping
            | ItemKind::ScrollIdentify
            | ItemKind::ScrollEnchant
            | ItemKind::ScrollSummon
            | ItemKind::ScrollBanish
            | ItemKind::ScrollTimeStop
            | ItemKind::ScrollMassHeal
            | ItemKind::ScrollDeath
            | ItemKind::ScrollEarthquake
            | ItemKind::ScrollMeteor
            | ItemKind::ScrollBlizzard
            | ItemKind::ScrollChainLightning
            | ItemKind::ScrollDivineWrath
            | ItemKind::ScrollDarkness
            | ItemKind::Bread
            | ItemKind::Meat
            | ItemKind::Apple
            | ItemKind::Cheese
            | ItemKind::Feast
            | ItemKind::DragonFruit
            | ItemKind::AncientWine
            | ItemKind::GoldenApple
            | ItemKind::RawMeat
            | ItemKind::RawFish
            | ItemKind::RawVegetables
            | ItemKind::RawEgg
            | ItemKind::Mushrooms
            | ItemKind::RawPoultry
            | ItemKind::CookedMeat
            | ItemKind::GrilledFish
            | ItemKind::Stew
            | ItemKind::Omelette
            | ItemKind::RoastChicken
            | ItemKind::MeatPie
            | ItemKind::FruitSalad
            | ItemKind::HeartyStew
            | ItemKind::DragonSteak
            | ItemKind::FeastOfKings
            | ItemKind::Bomb
            | ItemKind::Torch => Self::Consumables,

            // Currency
            ItemKind::Gold => Self::Currency,

            // Key items
            ItemKind::Key | ItemKind::Compass => Self::KeyItems,

            // Collectibles / Materials
            ItemKind::TeleportCrystal
            | ItemKind::SoulGem
            | ItemKind::AncientRelic
            | ItemKind::DragonScale
            | ItemKind::DemonHeart
            | ItemKind::Fish
            | ItemKind::RareFish
            | ItemKind::LegendaryFish
            | ItemKind::OreChunk
            | ItemKind::GemFragment
            | ItemKind::PerfectGem
            | ItemKind::TournamentReward => Self::Materials,
        }
    }
}

// ============================================================================
// INVENTORY ITEM (Extended Item with metadata)
// ============================================================================

/// An inventory item with additional metadata for inventory management
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    /// The base item
    pub item: Item,
    /// Stack count (for stackable items)
    pub quantity: u32,
    /// Whether the item is marked as favorite/locked
    pub is_favorite: bool,
    /// Whether the item is marked as junk
    pub is_junk: bool,
    /// Custom item note/label
    pub note: Option<String>,
    /// Timestamp when item was acquired
    pub acquired_at: u64,
    /// Source of the item (for tracking)
    pub source: ItemSource,
}

/// Source of an item
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum ItemSource {
    /// Looted from enemy
    Loot { enemy_name: String },
    /// Found in chest/container
    Chest { location: String },
    /// Purchased from vendor
    Purchased { vendor_name: String, price: u32 },
    /// Crafted by player
    Crafted,
    /// Quest reward
    QuestReward { quest_name: String },
    /// Traded from another player/NPC
    Trade,
    /// Starting equipment
    Starting,
    /// Unknown/default source
    #[default]
    Unknown,
}

impl InventoryItem {
    /// Create a new inventory item from a base item
    pub fn new(item: Item) -> Self {
        Self {
            item,
            quantity: 1,
            is_favorite: false,
            is_junk: false,
            note: None,
            acquired_at: 0,
            source: ItemSource::Unknown,
        }
    }

    /// Create with a specific quantity
    pub fn with_quantity(item: Item, quantity: u32) -> Self {
        Self {
            item,
            quantity: quantity.min(MAX_STACK_SIZE),
            is_favorite: false,
            is_junk: false,
            note: None,
            acquired_at: 0,
            source: ItemSource::Unknown,
        }
    }

    /// Check if this item can stack with another
    pub fn can_stack_with(&self, other: &InventoryItem) -> bool {
        self.is_stackable()
            && other.is_stackable()
            && self.item.kind == other.item.kind
            && self.item.rarity == other.item.rarity
            && self.item.enchantments == other.item.enchantments
            && self.quantity < MAX_STACK_SIZE
    }

    /// Check if item is stackable
    pub fn is_stackable(&self) -> bool {
        matches!(
            ItemCategory::from_item_kind(self.item.kind),
            ItemCategory::Consumables | ItemCategory::Materials | ItemCategory::Currency
        )
    }

    /// Get category
    pub fn category(&self) -> ItemCategory {
        ItemCategory::from_item_kind(self.item.kind)
    }

    /// Get weight of the item (per unit)
    pub fn unit_weight(&self) -> f32 {
        item_weight(self.item.kind)
    }

    /// Get total weight (quantity * unit weight)
    pub fn total_weight(&self) -> f32 {
        self.unit_weight() * self.quantity as f32
    }

    /// Get value of the item
    pub fn value(&self) -> u32 {
        item_value(self.item.kind, self.item.rarity) * self.quantity
    }

    /// Split the stack, returning a new item with the split amount
    pub fn split(&mut self, amount: u32) -> Option<InventoryItem> {
        if !self.is_stackable() || amount == 0 || amount >= self.quantity {
            return None;
        }

        self.quantity -= amount;
        let mut new_item = self.clone();
        new_item.quantity = amount;
        Some(new_item)
    }

    /// Merge another stack into this one, returns leftover if any
    pub fn merge(&mut self, other: &mut InventoryItem) -> bool {
        if !self.can_stack_with(other) {
            return false;
        }

        let space = MAX_STACK_SIZE - self.quantity;
        let to_add = other.quantity.min(space);

        self.quantity += to_add;
        other.quantity -= to_add;

        true
    }
}

/// Get the weight of an item kind
pub fn item_weight(kind: ItemKind) -> f32 {
    match kind {
        // Weapons
        ItemKind::Dagger => 1.0,
        ItemKind::ShortSword => 2.0,
        ItemKind::LongSword => 3.0,
        ItemKind::Greatsword => 6.0,
        ItemKind::Axe => 3.0,
        ItemKind::BattleAxe => 5.0,
        ItemKind::Mace => 3.0,
        ItemKind::WarHammer => 7.0,
        ItemKind::Spear => 3.0,
        ItemKind::Halberd => 5.0,
        ItemKind::Staff => 2.0,
        ItemKind::Bow => 2.0,
        ItemKind::Crossbow => 4.0,
        ItemKind::Wand => 0.5,
        ItemKind::Scythe => 5.0,
        ItemKind::Katana => 2.5,
        ItemKind::Rapier => 1.5,
        ItemKind::Flail => 4.0,
        ItemKind::Morningstar => 4.0,
        ItemKind::Trident => 4.0,
        ItemKind::FlameSword => 3.0,
        ItemKind::FrostBlade => 3.0,
        ItemKind::ThunderAxe => 5.0,
        ItemKind::VoidStaff => 2.5,
        ItemKind::DemonSlayer => 5.0,

        // Shields
        ItemKind::Buckler => 2.0,
        ItemKind::WoodenShield => 3.0,
        ItemKind::IronShield => 5.0,
        ItemKind::TowerShield => 10.0,
        ItemKind::MagicShield => 3.0,
        ItemKind::DragonShield => 6.0,
        ItemKind::SpikedShield => 6.0,
        ItemKind::MirrorShield => 4.0,
        ItemKind::PhoenixShield => 5.0,
        ItemKind::AbyssalShield => 7.0,

        // Armor
        ItemKind::LeatherArmor => 5.0,
        ItemKind::ChainMail => 15.0,
        ItemKind::ScaleMail => 20.0,
        ItemKind::PlateMail => 30.0,
        ItemKind::DragonArmor => 25.0,
        ItemKind::MageRobes => 2.0,
        ItemKind::AssassinGarb => 3.0,
        ItemKind::HolyArmor => 20.0,
        ItemKind::DemonArmor => 25.0,
        ItemKind::CrystalArmor => 18.0,
        ItemKind::ShadowCloak => 1.0,
        ItemKind::TitanPlate => 40.0,

        // Helmets
        ItemKind::LeatherCap => 1.0,
        ItemKind::IronHelm => 3.0,
        ItemKind::SteelHelm => 4.0,
        ItemKind::CrownOfKings => 2.0,
        ItemKind::WizardHat => 0.5,
        ItemKind::DemonSkull => 3.0,
        ItemKind::DragonHelm => 4.0,
        ItemKind::CrystalCrown => 2.0,
        ItemKind::HoodOfShadows => 0.5,
        ItemKind::HelmOfValor => 4.0,

        // Gloves
        ItemKind::LeatherGloves => 0.5,
        ItemKind::IronGauntlets => 2.0,
        ItemKind::GlovesOfPower => 1.0,
        ItemKind::ThievesGloves => 0.3,
        ItemKind::DragonGauntlets => 2.0,
        ItemKind::FrostGauntlets => 1.5,
        ItemKind::FlameGauntlets => 1.5,
        ItemKind::GauntletsOfMight => 2.5,

        // Boots
        ItemKind::LeatherBoots => 1.0,
        ItemKind::IronBoots => 3.0,
        ItemKind::BootsOfSpeed => 1.0,
        ItemKind::BootsOfLeaping => 1.0,
        ItemKind::WingedBoots => 0.5,
        ItemKind::ShadowBoots => 0.5,
        ItemKind::LavaWalkers => 2.0,
        ItemKind::BootsOfTheWind => 0.5,

        // Accessories (light)
        ItemKind::RingOfStrength
        | ItemKind::RingOfProtection
        | ItemKind::RingOfSpeed
        | ItemKind::RingOfRegeneration
        | ItemKind::RingOfFireball
        | ItemKind::RingOfInvisibility
        | ItemKind::RingOfTheVampire
        | ItemKind::RingOfMana
        | ItemKind::RingOfLuck
        | ItemKind::RingOfDeath
        | ItemKind::RingOfFrost
        | ItemKind::RingOfFlame
        | ItemKind::RingOfThunder
        | ItemKind::RingOfShadows
        | ItemKind::RingOfTheAncients => 0.1,

        ItemKind::AmuletOfHealth
        | ItemKind::AmuletOfMana
        | ItemKind::AmuletOfProtection
        | ItemKind::AmuletOfPower
        | ItemKind::AmuletOfWisdom
        | ItemKind::AmuletOfLife
        | ItemKind::AmuletOfDeath
        | ItemKind::AmuletOfTheGods
        | ItemKind::AmuletOfDragons
        | ItemKind::AmuletOfChaos
        | ItemKind::AmuletOfOrder
        | ItemKind::AmuletOfBalance => 0.2,

        // Consumables (light)
        ItemKind::HealthPotion
        | ItemKind::ManaPotion
        | ItemKind::StrengthPotion
        | ItemKind::DefensePotion
        | ItemKind::SpeedPotion
        | ItemKind::InvisibilityPotion
        | ItemKind::FireResistPotion
        | ItemKind::IceResistPotion
        | ItemKind::PoisonResistPotion
        | ItemKind::RegenerationPotion
        | ItemKind::BerserkPotion
        | ItemKind::GiantPotion
        | ItemKind::LevitationPotion
        | ItemKind::XPPotion
        | ItemKind::FullRestorePotion
        | ItemKind::LuckPotion
        | ItemKind::CriticalPotion
        | ItemKind::VisionPotion
        | ItemKind::CureAllPotion
        | ItemKind::UltimatePowerPotion => 0.3,

        // Scrolls
        ItemKind::ScrollTeleport
        | ItemKind::ScrollFireball
        | ItemKind::ScrollIceStorm
        | ItemKind::ScrollLightning
        | ItemKind::ScrollMapping
        | ItemKind::ScrollIdentify
        | ItemKind::ScrollEnchant
        | ItemKind::ScrollSummon
        | ItemKind::ScrollBanish
        | ItemKind::ScrollTimeStop
        | ItemKind::ScrollMassHeal
        | ItemKind::ScrollDeath
        | ItemKind::ScrollEarthquake
        | ItemKind::ScrollMeteor
        | ItemKind::ScrollBlizzard
        | ItemKind::ScrollChainLightning
        | ItemKind::ScrollDivineWrath
        | ItemKind::ScrollDarkness => 0.1,

        // Food
        ItemKind::Bread
        | ItemKind::Apple
        | ItemKind::Cheese
        | ItemKind::RawEgg
        | ItemKind::Mushrooms => 0.2,

        ItemKind::Meat
        | ItemKind::RawMeat
        | ItemKind::RawFish
        | ItemKind::RawVegetables
        | ItemKind::RawPoultry
        | ItemKind::CookedMeat
        | ItemKind::GrilledFish
        | ItemKind::Omelette
        | ItemKind::RoastChicken
        | ItemKind::FruitSalad => 0.5,

        ItemKind::Stew
        | ItemKind::MeatPie
        | ItemKind::HeartyStew
        | ItemKind::DragonSteak => 1.0,

        ItemKind::Feast | ItemKind::FeastOfKings => 3.0,

        ItemKind::DragonFruit | ItemKind::GoldenApple => 0.3,
        ItemKind::AncientWine => 1.0,

        // Special items
        ItemKind::Gold => 0.01,
        ItemKind::Key => 0.1,
        ItemKind::Bomb => 1.0,
        ItemKind::Torch => 0.5,
        ItemKind::Compass => 0.2,
        ItemKind::TeleportCrystal => 0.3,
        ItemKind::SoulGem => 0.2,
        ItemKind::AncientRelic => 1.0,
        ItemKind::DragonScale => 0.5,
        ItemKind::DemonHeart => 0.5,
        ItemKind::Fish => 0.3,
        ItemKind::RareFish => 0.4,
        ItemKind::LegendaryFish => 0.5,
        ItemKind::OreChunk => 1.0,
        ItemKind::GemFragment => 0.2,
        ItemKind::PerfectGem => 0.3,
        ItemKind::TournamentReward => 0.5,
    }
}

/// Get the base value of an item
pub fn item_value(kind: ItemKind, rarity: Rarity) -> u32 {
    let base = match kind {
        // Weapons
        ItemKind::Dagger => 10,
        ItemKind::ShortSword => 25,
        ItemKind::LongSword => 50,
        ItemKind::Greatsword => 100,
        ItemKind::Axe => 40,
        ItemKind::BattleAxe => 80,
        ItemKind::Mace => 35,
        ItemKind::WarHammer => 90,
        ItemKind::Spear => 30,
        ItemKind::Halberd => 75,
        ItemKind::Staff => 60,
        ItemKind::Bow => 45,
        ItemKind::Crossbow => 70,
        ItemKind::Wand => 80,
        ItemKind::Scythe => 120,
        ItemKind::Katana => 150,
        ItemKind::Rapier => 65,
        ItemKind::Flail => 55,
        ItemKind::Morningstar => 70,
        ItemKind::Trident => 85,
        ItemKind::FlameSword => 300,
        ItemKind::FrostBlade => 300,
        ItemKind::ThunderAxe => 350,
        ItemKind::VoidStaff => 400,
        ItemKind::DemonSlayer => 500,

        // Shields
        ItemKind::Buckler => 15,
        ItemKind::WoodenShield => 20,
        ItemKind::IronShield => 40,
        ItemKind::TowerShield => 80,
        ItemKind::MagicShield => 150,
        ItemKind::DragonShield => 400,
        ItemKind::SpikedShield => 60,
        ItemKind::MirrorShield => 200,
        ItemKind::PhoenixShield => 450,
        ItemKind::AbyssalShield => 500,

        // Armor
        ItemKind::LeatherArmor => 30,
        ItemKind::ChainMail => 80,
        ItemKind::ScaleMail => 120,
        ItemKind::PlateMail => 200,
        ItemKind::DragonArmor => 600,
        ItemKind::MageRobes => 100,
        ItemKind::AssassinGarb => 150,
        ItemKind::HolyArmor => 500,
        ItemKind::DemonArmor => 550,
        ItemKind::CrystalArmor => 400,
        ItemKind::ShadowCloak => 250,
        ItemKind::TitanPlate => 700,

        // Helmets
        ItemKind::LeatherCap => 10,
        ItemKind::IronHelm => 30,
        ItemKind::SteelHelm => 50,
        ItemKind::CrownOfKings => 500,
        ItemKind::WizardHat => 80,
        ItemKind::DemonSkull => 300,
        ItemKind::DragonHelm => 350,
        ItemKind::CrystalCrown => 400,
        ItemKind::HoodOfShadows => 150,
        ItemKind::HelmOfValor => 250,

        // Gloves
        ItemKind::LeatherGloves => 8,
        ItemKind::IronGauntlets => 25,
        ItemKind::GlovesOfPower => 150,
        ItemKind::ThievesGloves => 100,
        ItemKind::DragonGauntlets => 300,
        ItemKind::FrostGauntlets => 200,
        ItemKind::FlameGauntlets => 200,
        ItemKind::GauntletsOfMight => 350,

        // Boots
        ItemKind::LeatherBoots => 12,
        ItemKind::IronBoots => 30,
        ItemKind::BootsOfSpeed => 200,
        ItemKind::BootsOfLeaping => 150,
        ItemKind::WingedBoots => 400,
        ItemKind::ShadowBoots => 250,
        ItemKind::LavaWalkers => 300,
        ItemKind::BootsOfTheWind => 350,

        // Rings
        ItemKind::RingOfStrength => 100,
        ItemKind::RingOfProtection => 100,
        ItemKind::RingOfSpeed => 150,
        ItemKind::RingOfRegeneration => 200,
        ItemKind::RingOfFireball => 180,
        ItemKind::RingOfInvisibility => 300,
        ItemKind::RingOfTheVampire => 400,
        ItemKind::RingOfMana => 120,
        ItemKind::RingOfLuck => 250,
        ItemKind::RingOfDeath => 500,
        ItemKind::RingOfFrost => 150,
        ItemKind::RingOfFlame => 150,
        ItemKind::RingOfThunder => 180,
        ItemKind::RingOfShadows => 220,
        ItemKind::RingOfTheAncients => 600,

        // Amulets
        ItemKind::AmuletOfHealth => 100,
        ItemKind::AmuletOfMana => 100,
        ItemKind::AmuletOfProtection => 120,
        ItemKind::AmuletOfPower => 150,
        ItemKind::AmuletOfWisdom => 180,
        ItemKind::AmuletOfLife => 300,
        ItemKind::AmuletOfDeath => 400,
        ItemKind::AmuletOfTheGods => 800,
        ItemKind::AmuletOfDragons => 600,
        ItemKind::AmuletOfChaos => 500,
        ItemKind::AmuletOfOrder => 500,
        ItemKind::AmuletOfBalance => 550,

        // Consumables
        ItemKind::HealthPotion => 25,
        ItemKind::ManaPotion => 25,
        ItemKind::StrengthPotion => 50,
        ItemKind::DefensePotion => 50,
        ItemKind::SpeedPotion => 40,
        ItemKind::InvisibilityPotion => 100,
        ItemKind::FireResistPotion => 35,
        ItemKind::IceResistPotion => 35,
        ItemKind::PoisonResistPotion => 30,
        ItemKind::RegenerationPotion => 60,
        ItemKind::BerserkPotion => 75,
        ItemKind::GiantPotion => 80,
        ItemKind::LevitationPotion => 70,
        ItemKind::XPPotion => 150,
        ItemKind::FullRestorePotion => 200,
        ItemKind::LuckPotion => 100,
        ItemKind::CriticalPotion => 80,
        ItemKind::VisionPotion => 60,
        ItemKind::CureAllPotion => 150,
        ItemKind::UltimatePowerPotion => 500,

        // Scrolls
        ItemKind::ScrollTeleport => 50,
        ItemKind::ScrollFireball => 40,
        ItemKind::ScrollIceStorm => 45,
        ItemKind::ScrollLightning => 45,
        ItemKind::ScrollMapping => 30,
        ItemKind::ScrollIdentify => 25,
        ItemKind::ScrollEnchant => 200,
        ItemKind::ScrollSummon => 150,
        ItemKind::ScrollBanish => 100,
        ItemKind::ScrollTimeStop => 300,
        ItemKind::ScrollMassHeal => 250,
        ItemKind::ScrollDeath => 400,
        ItemKind::ScrollEarthquake => 200,
        ItemKind::ScrollMeteor => 350,
        ItemKind::ScrollBlizzard => 180,
        ItemKind::ScrollChainLightning => 200,
        ItemKind::ScrollDivineWrath => 500,
        ItemKind::ScrollDarkness => 150,

        // Food
        ItemKind::Bread => 2,
        ItemKind::Meat => 5,
        ItemKind::Apple => 1,
        ItemKind::Cheese => 4,
        ItemKind::Feast => 50,
        ItemKind::DragonFruit => 30,
        ItemKind::AncientWine => 100,
        ItemKind::GoldenApple => 200,
        ItemKind::RawMeat => 3,
        ItemKind::RawFish => 4,
        ItemKind::RawVegetables => 2,
        ItemKind::RawEgg => 2,
        ItemKind::Mushrooms => 3,
        ItemKind::RawPoultry => 4,
        ItemKind::CookedMeat => 8,
        ItemKind::GrilledFish => 10,
        ItemKind::Stew => 15,
        ItemKind::Omelette => 8,
        ItemKind::RoastChicken => 12,
        ItemKind::MeatPie => 20,
        ItemKind::FruitSalad => 10,
        ItemKind::HeartyStew => 25,
        ItemKind::DragonSteak => 100,
        ItemKind::FeastOfKings => 500,

        // Special
        ItemKind::Gold => 1,
        ItemKind::Key => 0,
        ItemKind::Bomb => 20,
        ItemKind::Torch => 5,
        ItemKind::Compass => 50,
        ItemKind::TeleportCrystal => 100,
        ItemKind::SoulGem => 150,
        ItemKind::AncientRelic => 1000,
        ItemKind::DragonScale => 200,
        ItemKind::DemonHeart => 300,
        ItemKind::Fish => 5,
        ItemKind::RareFish => 25,
        ItemKind::LegendaryFish => 100,
        ItemKind::OreChunk => 15,
        ItemKind::GemFragment => 30,
        ItemKind::PerfectGem => 200,
        ItemKind::TournamentReward => 500,
    };

    // Apply rarity multiplier
    let multiplier = match rarity {
        Rarity::Common => 1.0,
        Rarity::Uncommon => 1.5,
        Rarity::Rare => 2.5,
        Rarity::Epic => 5.0,
        Rarity::Legendary => 10.0,
        Rarity::Mythic => 25.0,
    };

    (base as f32 * multiplier) as u32
}

// ============================================================================
// CURRENCY SYSTEM
// ============================================================================

/// Currency types in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CurrencyType {
    /// Standard gold currency
    Gold,
    /// Premium currency
    Gems,
    /// Guild-specific currency
    GuildTokens,
    /// Arena/PvP currency
    ArenaPoints,
    /// Dungeon-specific currency
    DungeonMarks,
    /// Crafting-specific currency
    ArtisanCoins,
    /// Rare currency from bosses
    SoulFragments,
}

impl CurrencyType {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Gold => "Gold",
            Self::Gems => "Gems",
            Self::GuildTokens => "Guild Tokens",
            Self::ArenaPoints => "Arena Points",
            Self::DungeonMarks => "Dungeon Marks",
            Self::ArtisanCoins => "Artisan Coins",
            Self::SoulFragments => "Soul Fragments",
        }
    }

    /// Get currency symbol/glyph
    pub fn symbol(&self) -> char {
        match self {
            Self::Gold => 'G',
            Self::Gems => 'D',
            Self::GuildTokens => 'T',
            Self::ArenaPoints => 'A',
            Self::DungeonMarks => 'M',
            Self::ArtisanCoins => 'C',
            Self::SoulFragments => 'S',
        }
    }
}

/// Currency pouch for storing various currencies
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CurrencyPouch {
    /// Currency amounts
    currencies: HashMap<CurrencyType, u64>,
}

impl CurrencyPouch {
    /// Create a new empty currency pouch
    pub fn new() -> Self {
        Self::default()
    }

    /// Get amount of a currency
    pub fn get(&self, currency: CurrencyType) -> u64 {
        *self.currencies.get(&currency).unwrap_or(&0)
    }

    /// Add currency
    pub fn add(&mut self, currency: CurrencyType, amount: u64) {
        *self.currencies.entry(currency).or_insert(0) += amount;
    }

    /// Remove currency, returns true if successful
    pub fn remove(&mut self, currency: CurrencyType, amount: u64) -> bool {
        let current = self.currencies.entry(currency).or_insert(0);
        if *current >= amount {
            *current -= amount;
            true
        } else {
            false
        }
    }

    /// Check if has enough currency
    pub fn has(&self, currency: CurrencyType, amount: u64) -> bool {
        self.get(currency) >= amount
    }

    /// Get gold specifically
    pub fn gold(&self) -> u64 {
        self.get(CurrencyType::Gold)
    }

    /// Add gold specifically
    pub fn add_gold(&mut self, amount: u64) {
        self.add(CurrencyType::Gold, amount);
    }

    /// Remove gold specifically
    pub fn remove_gold(&mut self, amount: u64) -> bool {
        self.remove(CurrencyType::Gold, amount)
    }
}

// ============================================================================
// MATERIAL STORAGE
// ============================================================================

/// Storage for crafting materials
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct MaterialStorage {
    /// Material amounts
    materials: HashMap<CraftingMaterial, u32>,
    /// Maximum capacity per material type
    capacity_per_type: u32,
}

impl MaterialStorage {
    /// Create new material storage
    pub fn new(capacity: u32) -> Self {
        Self {
            materials: HashMap::new(),
            capacity_per_type: capacity,
        }
    }

    /// Get amount of a material
    pub fn get(&self, material: CraftingMaterial) -> u32 {
        *self.materials.get(&material).unwrap_or(&0)
    }

    /// Add material, returns amount actually added
    pub fn add(&mut self, material: CraftingMaterial, amount: u32) -> u32 {
        let current = self.get(material);
        let space = self.capacity_per_type.saturating_sub(current);
        let to_add = amount.min(space);

        if to_add > 0 {
            *self.materials.entry(material).or_insert(0) += to_add;
        }

        to_add
    }

    /// Remove material, returns true if successful
    pub fn remove(&mut self, material: CraftingMaterial, amount: u32) -> bool {
        let current = self.materials.entry(material).or_insert(0);
        if *current >= amount {
            *current -= amount;
            true
        } else {
            false
        }
    }

    /// Check if has enough material
    pub fn has(&self, material: CraftingMaterial, amount: u32) -> bool {
        self.get(material) >= amount
    }

    /// Get all materials with their amounts
    pub fn all(&self) -> &HashMap<CraftingMaterial, u32> {
        &self.materials
    }

    /// Check if storage is full for a material type
    pub fn is_full(&self, material: CraftingMaterial) -> bool {
        self.get(material) >= self.capacity_per_type
    }
}

// ============================================================================
// SORT AND FILTER OPTIONS
// ============================================================================

/// Sorting options for inventory
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum SortBy {
    /// Sort by item name
    #[default]
    Name,
    /// Sort by item type/category
    Type,
    /// Sort by rarity (highest first)
    Rarity,
    /// Sort by value (highest first)
    Value,
    /// Sort by weight (lightest first)
    Weight,
    /// Sort by when acquired (newest first)
    Recent,
    /// Sort by stack size
    Quantity,
}

/// Sort order
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

/// Filter options for inventory
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct InventoryFilter {
    /// Filter by category
    pub category: Option<ItemCategory>,
    /// Filter by rarity (minimum)
    pub min_rarity: Option<Rarity>,
    /// Filter by rarity (maximum)
    pub max_rarity: Option<Rarity>,
    /// Filter by equipment slot
    pub equip_slot: Option<EquipmentSlot>,
    /// Search text
    pub search: Option<String>,
    /// Show only favorites
    pub favorites_only: bool,
    /// Show only junk
    pub junk_only: bool,
    /// Hide junk items
    pub hide_junk: bool,
    /// Show only equippable items
    pub equippable_only: bool,
    /// Show only consumables
    pub consumables_only: bool,
}

impl InventoryFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by category
    pub fn with_category(mut self, category: ItemCategory) -> Self {
        self.category = Some(category);
        self
    }

    /// Filter by search text
    pub fn with_search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    /// Check if an item matches this filter
    pub fn matches(&self, item: &InventoryItem) -> bool {
        // Category filter
        if let Some(cat) = self.category {
            if item.category() != cat {
                return false;
            }
        }

        // Rarity filter
        if let Some(min) = self.min_rarity {
            if item.item.rarity < min {
                return false;
            }
        }
        if let Some(max) = self.max_rarity {
            if item.item.rarity > max {
                return false;
            }
        }

        // Equipment slot filter
        if let Some(slot) = self.equip_slot {
            if let Some(item_slot) = item.item.kind.equip_slot() {
                if EquipmentSlot::from_base(item_slot) != slot {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Search filter
        if let Some(ref search) = self.search {
            let search_lower = search.to_lowercase();
            let name_lower = item.item.display_name().to_lowercase();
            if !name_lower.contains(&search_lower) {
                return false;
            }
        }

        // Favorite filter
        if self.favorites_only && !item.is_favorite {
            return false;
        }

        // Junk filters
        if self.junk_only && !item.is_junk {
            return false;
        }
        if self.hide_junk && item.is_junk {
            return false;
        }

        // Equippable filter
        if self.equippable_only && item.item.kind.equip_slot().is_none() {
            return false;
        }

        // Consumable filter
        if self.consumables_only && !item.item.kind.is_consumable() {
            return false;
        }

        true
    }
}

// ============================================================================
// AUTO-LOOT SETTINGS
// ============================================================================

/// Auto-loot configuration
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AutoLootSettings {
    /// Enable auto-loot
    pub enabled: bool,
    /// Minimum rarity to auto-loot
    pub min_rarity: Rarity,
    /// Auto-loot gold
    pub auto_loot_gold: bool,
    /// Auto-loot consumables
    pub auto_loot_consumables: bool,
    /// Auto-loot equipment
    pub auto_loot_equipment: bool,
    /// Auto-loot materials
    pub auto_loot_materials: bool,
    /// Auto-loot quest items
    pub auto_loot_quest_items: bool,
    /// Item kinds to ignore
    pub ignore_list: HashSet<ItemKind>,
    /// Auto-equip if upgrade
    pub auto_equip_upgrades: bool,
    /// Auto-mark junk items
    pub auto_mark_junk: bool,
    /// Junk threshold (items below this value are auto-marked)
    pub junk_value_threshold: u32,
}

impl Default for AutoLootSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            min_rarity: Rarity::Common,
            auto_loot_gold: true,
            auto_loot_consumables: true,
            auto_loot_equipment: true,
            auto_loot_materials: true,
            auto_loot_quest_items: true,
            ignore_list: HashSet::new(),
            auto_equip_upgrades: false,
            auto_mark_junk: false,
            junk_value_threshold: 10,
        }
    }
}

impl AutoLootSettings {
    /// Check if an item should be auto-looted
    pub fn should_loot(&self, item: &Item) -> bool {
        if !self.enabled {
            return false;
        }

        // Check ignore list
        if self.ignore_list.contains(&item.kind) {
            return false;
        }

        // Check rarity
        if item.rarity < self.min_rarity {
            return false;
        }

        // Check category settings
        let category = ItemCategory::from_item_kind(item.kind);
        match category {
            ItemCategory::Currency => self.auto_loot_gold,
            ItemCategory::Consumables => self.auto_loot_consumables,
            ItemCategory::Weapons | ItemCategory::Armor | ItemCategory::Accessories => {
                self.auto_loot_equipment
            }
            ItemCategory::Materials => self.auto_loot_materials,
            ItemCategory::QuestItems | ItemCategory::KeyItems => self.auto_loot_quest_items,
            _ => true,
        }
    }

    /// Add item to ignore list
    pub fn ignore_item(&mut self, kind: ItemKind) {
        self.ignore_list.insert(kind);
    }

    /// Remove item from ignore list
    pub fn unignore_item(&mut self, kind: ItemKind) {
        self.ignore_list.remove(&kind);
    }
}

// ============================================================================
// WEIGHT SYSTEM
// ============================================================================

/// Encumbrance levels based on weight
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum EncumbranceLevel {
    /// Under 50% capacity - no penalty
    Light,
    /// 50-75% capacity - minor speed penalty
    Medium,
    /// 75-100% capacity - significant speed penalty
    Heavy,
    /// Over 100% capacity - cannot run, severe penalty
    Overburdened,
}

impl EncumbranceLevel {
    /// Get the speed multiplier for this encumbrance level
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            Self::Light => 1.0,
            Self::Medium => 0.9,
            Self::Heavy => 0.7,
            Self::Overburdened => 0.4,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Medium => "Medium",
            Self::Heavy => "Heavy",
            Self::Overburdened => "Overburdened",
        }
    }
}

/// Weight system configuration
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeightSystem {
    /// Whether weight system is enabled
    pub enabled: bool,
    /// Current carried weight
    pub current_weight: f32,
    /// Maximum carry capacity
    pub max_capacity: f32,
    /// Bonus capacity from items/effects
    pub bonus_capacity: f32,
}

impl Default for WeightSystem {
    fn default() -> Self {
        Self {
            enabled: false,
            current_weight: 0.0,
            max_capacity: BASE_CARRY_CAPACITY,
            bonus_capacity: 0.0,
        }
    }
}

impl WeightSystem {
    /// Create new weight system
    pub fn new(enabled: bool, strength: i32) -> Self {
        Self {
            enabled,
            current_weight: 0.0,
            max_capacity: BASE_CARRY_CAPACITY + (strength as f32 * CARRY_PER_STRENGTH),
            bonus_capacity: 0.0,
        }
    }

    /// Get total capacity
    pub fn total_capacity(&self) -> f32 {
        self.max_capacity + self.bonus_capacity
    }

    /// Get encumbrance level
    pub fn encumbrance(&self) -> EncumbranceLevel {
        if !self.enabled {
            return EncumbranceLevel::Light;
        }

        let ratio = self.current_weight / self.total_capacity();
        if ratio <= 0.5 {
            EncumbranceLevel::Light
        } else if ratio <= 0.75 {
            EncumbranceLevel::Medium
        } else if ratio <= 1.0 {
            EncumbranceLevel::Heavy
        } else {
            EncumbranceLevel::Overburdened
        }
    }

    /// Get remaining capacity
    pub fn remaining_capacity(&self) -> f32 {
        (self.total_capacity() - self.current_weight).max(0.0)
    }

    /// Check if can add weight
    pub fn can_add(&self, weight: f32) -> bool {
        !self.enabled || self.current_weight + weight <= self.total_capacity() * 1.5
    }

    /// Add weight
    pub fn add_weight(&mut self, weight: f32) {
        self.current_weight += weight;
    }

    /// Remove weight
    pub fn remove_weight(&mut self, weight: f32) {
        self.current_weight = (self.current_weight - weight).max(0.0);
    }

    /// Update capacity based on strength
    pub fn update_capacity(&mut self, strength: i32) {
        self.max_capacity = BASE_CARRY_CAPACITY + (strength as f32 * CARRY_PER_STRENGTH);
    }
}

// ============================================================================
// EQUIPMENT MANAGER
// ============================================================================

/// Manages equipped items
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EquipmentManager {
    /// Equipped items by slot
    equipped: HashMap<EquipmentSlot, InventoryItem>,
}

impl EquipmentManager {
    /// Create new equipment manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Equip an item, returns previously equipped item if any
    pub fn equip(&mut self, item: InventoryItem) -> Result<Option<InventoryItem>, EquipError> {
        // Determine slot
        let slot = if let Some(base_slot) = item.item.kind.equip_slot() {
            EquipmentSlot::from_base(base_slot)
        } else {
            return Err(EquipError::NotEquippable);
        };

        // Check if item is broken
        if item.item.is_broken() {
            return Err(EquipError::ItemBroken);
        }

        // Unequip current item
        let old_item = self.equipped.remove(&slot);

        // Equip new item
        self.equipped.insert(slot, item);

        Ok(old_item)
    }

    /// Equip to a specific slot (for rings)
    pub fn equip_to_slot(
        &mut self,
        item: InventoryItem,
        slot: EquipmentSlot,
    ) -> Result<Option<InventoryItem>, EquipError> {
        // Validate slot matches item
        if let Some(base_slot) = item.item.kind.equip_slot() {
            let expected_slot = EquipmentSlot::from_base(base_slot);
            // Allow ring to go to either ring slot
            let valid = match (expected_slot, slot) {
                (EquipmentSlot::Ring1, EquipmentSlot::Ring1 | EquipmentSlot::Ring2) => true,
                (a, b) if a == b => true,
                _ => false,
            };
            if !valid {
                return Err(EquipError::WrongSlot);
            }
        } else {
            return Err(EquipError::NotEquippable);
        }

        if item.item.is_broken() {
            return Err(EquipError::ItemBroken);
        }

        let old_item = self.equipped.remove(&slot);
        self.equipped.insert(slot, item);

        Ok(old_item)
    }

    /// Unequip from a slot
    pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<InventoryItem> {
        self.equipped.remove(&slot)
    }

    /// Get equipped item in a slot
    pub fn get(&self, slot: EquipmentSlot) -> Option<&InventoryItem> {
        self.equipped.get(&slot)
    }

    /// Get mutable equipped item
    pub fn get_mut(&mut self, slot: EquipmentSlot) -> Option<&mut InventoryItem> {
        self.equipped.get_mut(&slot)
    }

    /// Get all equipped items
    pub fn all(&self) -> &HashMap<EquipmentSlot, InventoryItem> {
        &self.equipped
    }

    /// Calculate total stats from equipment
    pub fn total_stats(&self) -> (i32, i32, i32, i32) {
        let mut total = (0, 0, 0, 0);
        for item in self.equipped.values() {
            let (atk, def, hp, mana) = item.item.stats();
            total.0 += atk;
            total.1 += def;
            total.2 += hp;
            total.3 += mana;
        }
        total
    }

    /// Get total weight of equipped items
    pub fn total_weight(&self) -> f32 {
        self.equipped.values().map(|i| i.unit_weight()).sum()
    }

    /// Check if an item would be an upgrade
    pub fn is_upgrade(&self, item: &InventoryItem) -> bool {
        if let Some(base_slot) = item.item.kind.equip_slot() {
            let slot = EquipmentSlot::from_base(base_slot);
            if let Some(current) = self.equipped.get(&slot) {
                let (new_atk, new_def, new_hp, new_mana) = item.item.stats();
                let (cur_atk, cur_def, cur_hp, cur_mana) = current.item.stats();

                // Simple comparison: sum of stats
                let new_total = new_atk + new_def + new_hp + new_mana;
                let cur_total = cur_atk + cur_def + cur_hp + cur_mana;

                return new_total > cur_total;
            }
            // No current item = definitely an upgrade
            return true;
        }
        false
    }
}

/// Equipment errors
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EquipError {
    /// Item cannot be equipped
    NotEquippable,
    /// Item is broken
    ItemBroken,
    /// Wrong slot for item
    WrongSlot,
    /// Level requirement not met
    LevelTooLow,
    /// Class requirement not met
    WrongClass,
}

// ============================================================================
// QUICK SLOTS (Consumables Hotbar)
// ============================================================================

/// Quick slots for consumables
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QuickSlots {
    /// Items in quick slots (index -> item reference)
    slots: [Option<usize>; QUICK_SLOT_COUNT],
}

impl Default for QuickSlots {
    fn default() -> Self {
        Self {
            slots: [None; QUICK_SLOT_COUNT],
        }
    }
}

impl QuickSlots {
    /// Create new quick slots
    pub fn new() -> Self {
        Self::default()
    }

    /// Assign an item to a quick slot
    pub fn assign(&mut self, slot: usize, item_index: usize) -> bool {
        if slot < QUICK_SLOT_COUNT {
            self.slots[slot] = Some(item_index);
            true
        } else {
            false
        }
    }

    /// Clear a quick slot
    pub fn clear(&mut self, slot: usize) {
        if slot < QUICK_SLOT_COUNT {
            self.slots[slot] = None;
        }
    }

    /// Get item index in a slot
    pub fn get(&self, slot: usize) -> Option<usize> {
        if slot < QUICK_SLOT_COUNT {
            self.slots[slot]
        } else {
            None
        }
    }

    /// Update indices after item removal
    pub fn update_indices(&mut self, removed_index: usize) {
        for slot in &mut self.slots {
            if let Some(idx) = slot {
                if *idx == removed_index {
                    *slot = None;
                } else if *idx > removed_index {
                    *idx -= 1;
                }
            }
        }
    }
}

// ============================================================================
// MAIN INVENTORY STRUCTURE
// ============================================================================

/// The main bag storage
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MainBag {
    /// Items in the bag
    items: Vec<InventoryItem>,
    /// Current capacity
    capacity: usize,
}

impl MainBag {
    /// Create new main bag
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity: capacity.min(MAX_BAG_CAPACITY),
        }
    }

    /// Get current item count
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Check if bag is full
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    /// Get remaining space
    pub fn remaining_space(&self) -> usize {
        self.capacity.saturating_sub(self.items.len())
    }

    /// Expand bag capacity
    pub fn expand(&mut self, additional: usize) {
        self.capacity = (self.capacity + additional).min(MAX_BAG_CAPACITY);
    }

    /// Add an item, returns false if full
    pub fn add(&mut self, item: InventoryItem) -> bool {
        // Try to stack first
        if item.is_stackable() {
            for existing in &mut self.items {
                if existing.can_stack_with(&item) {
                    let space = MAX_STACK_SIZE - existing.quantity;
                    let to_add = item.quantity.min(space);
                    existing.quantity += to_add;
                    if to_add == item.quantity {
                        return true;
                    }
                    // Partial stack - need new slot for remainder
                    let mut remainder = item.clone();
                    remainder.quantity = item.quantity - to_add;
                    if !self.is_full() {
                        self.items.push(remainder);
                        return true;
                    }
                    return false;
                }
            }
        }

        // Can't stack, need new slot
        if self.is_full() {
            return false;
        }

        self.items.push(item);
        true
    }

    /// Remove item at index
    pub fn remove(&mut self, index: usize) -> Option<InventoryItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<&InventoryItem> {
        self.items.get(index)
    }

    /// Get mutable item at index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut InventoryItem> {
        self.items.get_mut(index)
    }

    /// Get all items
    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    /// Get mutable items
    pub fn items_mut(&mut self) -> &mut Vec<InventoryItem> {
        &mut self.items
    }

    /// Sort items
    pub fn sort(&mut self, sort_by: SortBy, order: SortOrder) {
        self.items.sort_by(|a, b| {
            let cmp = match sort_by {
                SortBy::Name => a.item.display_name().cmp(&b.item.display_name()),
                SortBy::Type => {
                    let cat_a = a.category() as u8;
                    let cat_b = b.category() as u8;
                    cat_a.cmp(&cat_b)
                }
                SortBy::Rarity => {
                    // Higher rarity first by default
                    let rar_a = a.item.rarity as u8;
                    let rar_b = b.item.rarity as u8;
                    rar_b.cmp(&rar_a)
                }
                SortBy::Value => {
                    // Higher value first by default
                    b.value().cmp(&a.value())
                }
                SortBy::Weight => {
                    // Lighter first by default
                    a.total_weight()
                        .partial_cmp(&b.total_weight())
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                SortBy::Recent => {
                    // Newer first by default
                    b.acquired_at.cmp(&a.acquired_at)
                }
                SortBy::Quantity => {
                    // Higher quantity first by default
                    b.quantity.cmp(&a.quantity)
                }
            };

            match order {
                SortOrder::Ascending => cmp,
                SortOrder::Descending => cmp.reverse(),
            }
        });
    }

    /// Filter items
    pub fn filter(&self, filter: &InventoryFilter) -> Vec<&InventoryItem> {
        self.items.iter().filter(|i| filter.matches(i)).collect()
    }

    /// Find items by kind
    pub fn find_by_kind(&self, kind: ItemKind) -> Vec<(usize, &InventoryItem)> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.item.kind == kind)
            .collect()
    }

    /// Get total weight of items
    pub fn total_weight(&self) -> f32 {
        self.items.iter().map(|i| i.total_weight()).sum()
    }

    /// Get total value of items
    pub fn total_value(&self) -> u32 {
        self.items.iter().map(|i| i.value()).sum()
    }

    /// Get all junk items
    pub fn junk_items(&self) -> Vec<(usize, &InventoryItem)> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.is_junk)
            .collect()
    }

    /// Calculate value of all junk items
    pub fn junk_value(&self) -> u32 {
        self.items
            .iter()
            .filter(|i| i.is_junk)
            .map(|i| i.value())
            .sum()
    }

    /// Remove all junk items, returns their total value
    pub fn sell_all_junk(&mut self) -> u32 {
        let value = self.junk_value();
        self.items.retain(|i| !i.is_junk);
        value
    }
}

// ============================================================================
// STORAGE SYSTEMS
// ============================================================================

/// Personal stash (in town)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PersonalStash {
    /// Items in stash
    items: Vec<InventoryItem>,
    /// Current capacity
    capacity: usize,
    /// Stash tabs (for organization)
    tabs: Vec<StashTab>,
}

/// A stash tab for organization
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StashTab {
    /// Tab name
    pub name: String,
    /// Item indices in this tab
    pub items: Vec<usize>,
    /// Tab color/icon
    pub icon: u8,
}

impl PersonalStash {
    /// Create new personal stash
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity: capacity.min(MAX_STASH_CAPACITY),
            tabs: vec![StashTab {
                name: "General".to_string(),
                items: Vec::new(),
                icon: 0,
            }],
        }
    }

    /// Get item count
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Add item to stash
    pub fn add(&mut self, item: InventoryItem) -> bool {
        if self.items.len() >= self.capacity {
            return false;
        }
        self.items.push(item);
        true
    }

    /// Remove item from stash
    pub fn remove(&mut self, index: usize) -> Option<InventoryItem> {
        if index < self.items.len() {
            // Update tab indices
            for tab in &mut self.tabs {
                tab.items.retain(|&i| i != index);
                for idx in &mut tab.items {
                    if *idx > index {
                        *idx -= 1;
                    }
                }
            }
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<&InventoryItem> {
        self.items.get(index)
    }

    /// Get all items
    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    /// Expand capacity
    pub fn expand(&mut self, additional: usize) {
        self.capacity = (self.capacity + additional).min(MAX_STASH_CAPACITY);
    }

    /// Add a new tab
    pub fn add_tab(&mut self, name: String) {
        self.tabs.push(StashTab {
            name,
            items: Vec::new(),
            icon: self.tabs.len() as u8,
        });
    }

    /// Get tabs
    pub fn tabs(&self) -> &[StashTab] {
        &self.tabs
    }
}

/// Shared bank (between characters)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SharedBank {
    /// Items in bank
    items: Vec<InventoryItem>,
    /// Current capacity
    capacity: usize,
    /// Shared currency
    currency: CurrencyPouch,
}

impl SharedBank {
    /// Create new shared bank
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity: capacity.min(MAX_BANK_CAPACITY),
            currency: CurrencyPouch::new(),
        }
    }

    /// Get item count
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Deposit item
    pub fn deposit(&mut self, item: InventoryItem) -> bool {
        if self.items.len() >= self.capacity {
            return false;
        }
        self.items.push(item);
        true
    }

    /// Withdraw item
    pub fn withdraw(&mut self, index: usize) -> Option<InventoryItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<&InventoryItem> {
        self.items.get(index)
    }

    /// Get all items
    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    /// Deposit currency
    pub fn deposit_currency(&mut self, currency: CurrencyType, amount: u64) {
        self.currency.add(currency, amount);
    }

    /// Withdraw currency
    pub fn withdraw_currency(&mut self, currency: CurrencyType, amount: u64) -> bool {
        self.currency.remove(currency, amount)
    }

    /// Get currency pouch
    pub fn currency(&self) -> &CurrencyPouch {
        &self.currency
    }

    /// Expand capacity
    pub fn expand(&mut self, additional: usize) {
        self.capacity = (self.capacity + additional).min(MAX_BANK_CAPACITY);
    }
}

/// Guild storage
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GuildStorage {
    /// Items in guild storage
    items: Vec<InventoryItem>,
    /// Current capacity
    capacity: usize,
    /// Guild name
    guild_name: String,
    /// Access log
    access_log: Vec<GuildStorageLog>,
}

/// Log entry for guild storage
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GuildStorageLog {
    /// Player who made the action
    pub player_name: String,
    /// Action type
    pub action: GuildStorageAction,
    /// Item involved (display name)
    pub item_name: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Guild storage action types
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum GuildStorageAction {
    Deposit,
    Withdraw,
}

impl GuildStorage {
    /// Create new guild storage
    pub fn new(guild_name: String, capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity: capacity.min(DEFAULT_GUILD_STORAGE_CAPACITY * 2),
            guild_name,
            access_log: Vec::new(),
        }
    }

    /// Get guild name
    pub fn guild_name(&self) -> &str {
        &self.guild_name
    }

    /// Get item count
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Deposit item
    pub fn deposit(&mut self, item: InventoryItem, player_name: &str, timestamp: u64) -> bool {
        if self.items.len() >= self.capacity {
            return false;
        }

        self.access_log.push(GuildStorageLog {
            player_name: player_name.to_string(),
            action: GuildStorageAction::Deposit,
            item_name: item.item.display_name(),
            timestamp,
        });

        self.items.push(item);
        true
    }

    /// Withdraw item
    pub fn withdraw(
        &mut self,
        index: usize,
        player_name: &str,
        timestamp: u64,
    ) -> Option<InventoryItem> {
        if index < self.items.len() {
            let item = self.items.remove(index);

            self.access_log.push(GuildStorageLog {
                player_name: player_name.to_string(),
                action: GuildStorageAction::Withdraw,
                item_name: item.item.display_name(),
                timestamp,
            });

            Some(item)
        } else {
            None
        }
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<&InventoryItem> {
        self.items.get(index)
    }

    /// Get all items
    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    /// Get access log
    pub fn access_log(&self) -> &[GuildStorageLog] {
        &self.access_log
    }

    /// Expand capacity
    pub fn expand(&mut self, additional: usize) {
        self.capacity = (self.capacity + additional).min(DEFAULT_GUILD_STORAGE_CAPACITY * 5);
    }
}

/// Portable storage (bags, rings of holding)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PortableStorage {
    /// Storage name
    pub name: String,
    /// Items
    items: Vec<InventoryItem>,
    /// Capacity
    capacity: usize,
    /// Storage type
    pub storage_type: PortableStorageType,
}

/// Types of portable storage
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum PortableStorageType {
    /// Small pouch (5 slots)
    Pouch,
    /// Medium bag (10 slots)
    Bag,
    /// Large backpack (20 slots)
    Backpack,
    /// Magical bag of holding (30 slots)
    BagOfHolding,
    /// Ring of holding (10 slots, weightless)
    RingOfHolding,
    /// Dimensional pocket (50 slots)
    DimensionalPocket,
}

impl PortableStorageType {
    /// Get default capacity
    pub fn capacity(&self) -> usize {
        match self {
            Self::Pouch => 5,
            Self::Bag => 10,
            Self::Backpack => 20,
            Self::BagOfHolding => 30,
            Self::RingOfHolding => 10,
            Self::DimensionalPocket => 50,
        }
    }

    /// Get name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Pouch => "Pouch",
            Self::Bag => "Bag",
            Self::Backpack => "Backpack",
            Self::BagOfHolding => "Bag of Holding",
            Self::RingOfHolding => "Ring of Holding",
            Self::DimensionalPocket => "Dimensional Pocket",
        }
    }

    /// Whether items inside are weightless
    pub fn is_weightless(&self) -> bool {
        matches!(
            self,
            Self::BagOfHolding | Self::RingOfHolding | Self::DimensionalPocket
        )
    }
}

impl PortableStorage {
    /// Create new portable storage
    pub fn new(storage_type: PortableStorageType) -> Self {
        Self {
            name: storage_type.name().to_string(),
            items: Vec::new(),
            capacity: storage_type.capacity(),
            storage_type,
        }
    }

    /// Get item count
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Is full
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    /// Add item
    pub fn add(&mut self, item: InventoryItem) -> bool {
        if self.is_full() {
            return false;
        }
        self.items.push(item);
        true
    }

    /// Remove item
    pub fn remove(&mut self, index: usize) -> Option<InventoryItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Get item
    pub fn get(&self, index: usize) -> Option<&InventoryItem> {
        self.items.get(index)
    }

    /// Get all items
    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    /// Get weight of contents (0 if weightless storage)
    pub fn contents_weight(&self) -> f32 {
        if self.storage_type.is_weightless() {
            0.0
        } else {
            self.items.iter().map(|i| i.total_weight()).sum()
        }
    }
}

// ============================================================================
// QUEST AND KEY ITEMS
// ============================================================================

/// Quest item storage (unlimited)
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct QuestItemStorage {
    /// Quest items
    items: Vec<InventoryItem>,
}

impl QuestItemStorage {
    /// Create new quest item storage
    pub fn new() -> Self {
        Self::default()
    }

    /// Add quest item
    pub fn add(&mut self, item: InventoryItem) {
        self.items.push(item);
    }

    /// Remove quest item
    pub fn remove(&mut self, index: usize) -> Option<InventoryItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Get quest item
    pub fn get(&self, index: usize) -> Option<&InventoryItem> {
        self.items.get(index)
    }

    /// Get all quest items
    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    /// Check if has a specific quest item
    pub fn has(&self, kind: ItemKind) -> bool {
        self.items.iter().any(|i| i.item.kind == kind)
    }

    /// Count of a specific quest item
    pub fn count_of(&self, kind: ItemKind) -> u32 {
        self.items
            .iter()
            .filter(|i| i.item.kind == kind)
            .map(|i| i.quantity)
            .sum()
    }
}

/// Key item storage (never lost)
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct KeyItemStorage {
    /// Key items (stored by ID for quick lookup)
    items: HashMap<String, InventoryItem>,
}

impl KeyItemStorage {
    /// Create new key item storage
    pub fn new() -> Self {
        Self::default()
    }

    /// Add key item
    pub fn add(&mut self, id: String, item: InventoryItem) {
        self.items.insert(id, item);
    }

    /// Get key item
    pub fn get(&self, id: &str) -> Option<&InventoryItem> {
        self.items.get(id)
    }

    /// Check if has key item
    pub fn has(&self, id: &str) -> bool {
        self.items.contains_key(id)
    }

    /// Get all key items
    pub fn all(&self) -> &HashMap<String, InventoryItem> {
        &self.items
    }

    /// Count of key items
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

// ============================================================================
// MAIN INVENTORY SYSTEM
// ============================================================================

/// The complete inventory system
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InventorySystem {
    /// Main bag storage
    pub main_bag: MainBag,
    /// Equipment manager
    pub equipment: EquipmentManager,
    /// Quick slots
    pub quick_slots: QuickSlots,
    /// Quest items
    pub quest_items: QuestItemStorage,
    /// Key items
    pub key_items: KeyItemStorage,
    /// Currency pouch
    pub currency: CurrencyPouch,
    /// Material storage
    pub materials: MaterialStorage,
    /// Personal stash
    pub stash: PersonalStash,
    /// Portable storage items
    pub portable_storage: Vec<PortableStorage>,
    /// Weight system
    pub weight: WeightSystem,
    /// Auto-loot settings
    pub auto_loot: AutoLootSettings,
    /// Current sort settings
    pub sort_by: SortBy,
    /// Current sort order
    pub sort_order: SortOrder,
    /// Active filter
    pub filter: InventoryFilter,
    /// Timestamp counter for item acquisition
    timestamp: u64,
}

impl Default for InventorySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl InventorySystem {
    /// Create a new inventory system
    pub fn new() -> Self {
        Self {
            main_bag: MainBag::new(DEFAULT_BAG_CAPACITY),
            equipment: EquipmentManager::new(),
            quick_slots: QuickSlots::new(),
            quest_items: QuestItemStorage::new(),
            key_items: KeyItemStorage::new(),
            currency: CurrencyPouch::new(),
            materials: MaterialStorage::new(MAX_STACK_SIZE),
            stash: PersonalStash::new(DEFAULT_STASH_CAPACITY),
            portable_storage: Vec::new(),
            weight: WeightSystem::default(),
            auto_loot: AutoLootSettings::default(),
            sort_by: SortBy::Type,
            sort_order: SortOrder::Ascending,
            filter: InventoryFilter::default(),
            timestamp: 0,
        }
    }

    /// Create with custom settings
    pub fn with_settings(bag_capacity: usize, weight_enabled: bool, strength: i32) -> Self {
        Self {
            main_bag: MainBag::new(bag_capacity),
            equipment: EquipmentManager::new(),
            quick_slots: QuickSlots::new(),
            quest_items: QuestItemStorage::new(),
            key_items: KeyItemStorage::new(),
            currency: CurrencyPouch::new(),
            materials: MaterialStorage::new(MAX_STACK_SIZE),
            stash: PersonalStash::new(DEFAULT_STASH_CAPACITY),
            portable_storage: Vec::new(),
            weight: WeightSystem::new(weight_enabled, strength),
            auto_loot: AutoLootSettings::default(),
            sort_by: SortBy::Type,
            sort_order: SortOrder::Ascending,
            filter: InventoryFilter::default(),
            timestamp: 0,
        }
    }

    /// Get next timestamp
    fn next_timestamp(&mut self) -> u64 {
        self.timestamp += 1;
        self.timestamp
    }

    /// Add item to inventory (auto-routes to appropriate storage)
    pub fn add_item(&mut self, item: Item, source: ItemSource) -> AddItemResult {
        let mut inv_item = InventoryItem::new(item.clone());
        inv_item.acquired_at = self.next_timestamp();
        inv_item.source = source;

        // Check auto-mark junk
        if self.auto_loot.auto_mark_junk && inv_item.value() < self.auto_loot.junk_value_threshold {
            inv_item.is_junk = true;
        }

        let category = inv_item.category();

        // Route to appropriate storage
        match category {
            ItemCategory::Currency => {
                if item.kind == ItemKind::Gold {
                    self.currency.add_gold(inv_item.quantity as u64);
                    return AddItemResult::AddedToCurrency;
                }
            }
            ItemCategory::KeyItems => {
                let id = format!("{}_{}", item.kind.name(), self.key_items.count());
                self.key_items.add(id, inv_item);
                return AddItemResult::AddedToKeyItems;
            }
            ItemCategory::QuestItems => {
                self.quest_items.add(inv_item);
                return AddItemResult::AddedToQuestItems;
            }
            ItemCategory::Materials => {
                // Try to add to material storage first
                if let Some(material) = item_to_crafting_material(item.kind) {
                    let added = self.materials.add(material, inv_item.quantity);
                    if added == inv_item.quantity {
                        return AddItemResult::AddedToMaterials;
                    }
                    inv_item.quantity -= added;
                }
            }
            _ => {}
        }

        // Check weight
        if !self.weight.can_add(inv_item.total_weight()) {
            return AddItemResult::TooHeavy;
        }

        // Check auto-equip
        if self.auto_loot.auto_equip_upgrades
            && inv_item.item.kind.equip_slot().is_some()
            && self.equipment.is_upgrade(&inv_item)
        {
            if let Ok(old_item) = self.equipment.equip(inv_item.clone()) {
                self.weight.add_weight(inv_item.unit_weight());
                if let Some(old) = old_item {
                    // Add old item to bag
                    self.main_bag.add(old);
                }
                return AddItemResult::Equipped;
            }
        }

        // Try to add to main bag
        if self.main_bag.add(inv_item.clone()) {
            self.weight.add_weight(inv_item.total_weight());
            return AddItemResult::AddedToBag;
        }

        // Try portable storage
        for storage in &mut self.portable_storage {
            if storage.add(inv_item.clone()) {
                if !storage.storage_type.is_weightless() {
                    self.weight.add_weight(inv_item.total_weight());
                }
                return AddItemResult::AddedToPortableStorage;
            }
        }

        AddItemResult::InventoryFull
    }

    /// Remove item from main bag
    pub fn remove_from_bag(&mut self, index: usize) -> Option<InventoryItem> {
        if let Some(item) = self.main_bag.remove(index) {
            self.weight.remove_weight(item.total_weight());
            self.quick_slots.update_indices(index);
            Some(item)
        } else {
            None
        }
    }

    /// Equip item from bag
    pub fn equip_from_bag(&mut self, index: usize) -> Result<Option<InventoryItem>, EquipError> {
        let item = self
            .main_bag
            .remove(index)
            .ok_or(EquipError::NotEquippable)?;
        self.quick_slots.update_indices(index);

        match self.equipment.equip(item.clone()) {
            Ok(old_item) => {
                // Update weight
                self.weight.remove_weight(item.total_weight());
                self.weight.add_weight(item.unit_weight());

                // Add old item to bag if any
                if let Some(old) = old_item {
                    self.weight.remove_weight(old.unit_weight());
                    self.weight.add_weight(old.total_weight());
                    self.main_bag.add(old.clone());
                    Ok(Some(old))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                // Put item back
                self.main_bag.add(item);
                Err(e)
            }
        }
    }

    /// Unequip item to bag
    pub fn unequip_to_bag(&mut self, slot: EquipmentSlot) -> Result<(), UnequipError> {
        if self.main_bag.is_full() {
            return Err(UnequipError::BagFull);
        }

        if let Some(item) = self.equipment.unequip(slot) {
            self.weight.remove_weight(item.unit_weight());
            self.weight.add_weight(item.total_weight());
            self.main_bag.add(item);
            Ok(())
        } else {
            Err(UnequipError::SlotEmpty)
        }
    }

    /// Use consumable from bag
    pub fn use_consumable(&mut self, index: usize) -> Option<InventoryItem> {
        let item = self.main_bag.get_mut(index)?;

        if !item.item.kind.is_consumable() {
            return None;
        }

        if item.quantity > 1 {
            item.quantity -= 1;
            let mut used = item.clone();
            used.quantity = 1;
            Some(used)
        } else {
            self.remove_from_bag(index)
        }
    }

    /// Use item from quick slot
    pub fn use_quick_slot(&mut self, slot: usize) -> Option<InventoryItem> {
        let index = self.quick_slots.get(slot)?;
        self.use_consumable(index)
    }

    /// Move item to stash
    pub fn move_to_stash(&mut self, index: usize) -> bool {
        if let Some(item) = self.main_bag.remove(index) {
            self.weight.remove_weight(item.total_weight());
            self.quick_slots.update_indices(index);
            if self.stash.add(item.clone()) {
                true
            } else {
                // Stash full, put back
                self.main_bag.add(item.clone());
                self.weight.add_weight(item.total_weight());
                false
            }
        } else {
            false
        }
    }

    /// Move item from stash to bag
    pub fn move_from_stash(&mut self, index: usize) -> bool {
        if self.main_bag.is_full() {
            return false;
        }

        if let Some(item) = self.stash.remove(index) {
            if self.weight.can_add(item.total_weight()) {
                self.weight.add_weight(item.total_weight());
                self.main_bag.add(item);
                true
            } else {
                // Too heavy, put back
                self.stash.add(item);
                false
            }
        } else {
            false
        }
    }

    /// Split stack
    pub fn split_stack(&mut self, index: usize, amount: u32) -> Option<usize> {
        let item = self.main_bag.get_mut(index)?;
        let new_item = item.split(amount)?;

        if self.main_bag.is_full() {
            // Can't split, would overflow
            if let Some(original) = self.main_bag.get_mut(index) {
                original.quantity += amount;
            }
            return None;
        }

        self.main_bag.add(new_item);
        Some(self.main_bag.count() - 1)
    }

    /// Toggle favorite on item
    pub fn toggle_favorite(&mut self, index: usize) -> bool {
        if let Some(item) = self.main_bag.get_mut(index) {
            item.is_favorite = !item.is_favorite;
            true
        } else {
            false
        }
    }

    /// Toggle junk on item
    pub fn toggle_junk(&mut self, index: usize) -> bool {
        if let Some(item) = self.main_bag.get_mut(index) {
            item.is_junk = !item.is_junk;
            true
        } else {
            false
        }
    }

    /// Sell all junk items
    pub fn sell_all_junk(&mut self) -> u32 {
        let value = self.main_bag.sell_all_junk();
        self.currency.add_gold(value as u64);
        self.recalculate_weight();
        value
    }

    /// Sort inventory
    pub fn sort(&mut self) {
        self.main_bag.sort(self.sort_by, self.sort_order);
    }

    /// Get filtered items
    pub fn filtered_items(&self) -> Vec<&InventoryItem> {
        self.main_bag.filter(&self.filter)
    }

    /// Search items
    pub fn search(&self, query: &str) -> Vec<(usize, &InventoryItem)> {
        let query_lower = query.to_lowercase();
        self.main_bag
            .items()
            .iter()
            .enumerate()
            .filter(|(_, item)| item.item.display_name().to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Recalculate total weight
    pub fn recalculate_weight(&mut self) {
        let mut total = 0.0;

        // Main bag
        total += self.main_bag.total_weight();

        // Equipment
        total += self.equipment.total_weight();

        // Portable storage (non-weightless only)
        for storage in &self.portable_storage {
            total += storage.contents_weight();
        }

        self.weight.current_weight = total;
    }

    /// Get total item count across all storage
    pub fn total_item_count(&self) -> usize {
        let mut count = self.main_bag.count();
        count += self.equipment.all().len();
        count += self.quest_items.items().len();
        count += self.key_items.count();
        for storage in &self.portable_storage {
            count += storage.count();
        }
        count
    }

    /// Get encumbrance level
    pub fn encumbrance(&self) -> EncumbranceLevel {
        self.weight.encumbrance()
    }

    /// Add portable storage
    pub fn add_portable_storage(&mut self, storage: PortableStorage) {
        self.portable_storage.push(storage);
    }

    /// Check if should auto-loot an item
    pub fn should_auto_loot(&self, item: &Item) -> bool {
        self.auto_loot.should_loot(item)
    }

    /// Expand main bag capacity
    pub fn expand_bag(&mut self, additional: usize) {
        self.main_bag.expand(additional);
    }

    /// Get gold amount
    pub fn gold(&self) -> u64 {
        self.currency.gold()
    }

    /// Add gold
    pub fn add_gold(&mut self, amount: u64) {
        self.currency.add_gold(amount);
    }

    /// Remove gold
    pub fn remove_gold(&mut self, amount: u64) -> bool {
        self.currency.remove_gold(amount)
    }

    /// Check if has gold
    pub fn has_gold(&self, amount: u64) -> bool {
        self.currency.has(CurrencyType::Gold, amount)
    }
}

/// Result of adding an item
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AddItemResult {
    /// Added to main bag
    AddedToBag,
    /// Added to currency pouch
    AddedToCurrency,
    /// Added to quest items
    AddedToQuestItems,
    /// Added to key items
    AddedToKeyItems,
    /// Added to material storage
    AddedToMaterials,
    /// Added to portable storage
    AddedToPortableStorage,
    /// Auto-equipped
    Equipped,
    /// Inventory is full
    InventoryFull,
    /// Item is too heavy
    TooHeavy,
}

/// Unequip errors
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnequipError {
    /// Bag is full
    BagFull,
    /// Slot is empty
    SlotEmpty,
}

/// Convert item kind to crafting material (if applicable)
fn item_to_crafting_material(kind: ItemKind) -> Option<CraftingMaterial> {
    match kind {
        ItemKind::DragonScale => Some(CraftingMaterial::DragonLeather),
        ItemKind::DemonHeart => Some(CraftingMaterial::DemonCore),
        ItemKind::SoulGem => Some(CraftingMaterial::LifeEssence),
        _ => None,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_creation() {
        let inv = InventorySystem::new();
        assert_eq!(inv.main_bag.capacity(), DEFAULT_BAG_CAPACITY);
        assert_eq!(inv.main_bag.count(), 0);
    }

    #[test]
    fn test_add_item_to_bag() {
        let mut inv = InventorySystem::new();
        let item = Item::new(0, 0, ItemKind::HealthPotion, Rarity::Common);

        let result = inv.add_item(item, ItemSource::Unknown);
        assert_eq!(result, AddItemResult::AddedToBag);
        assert_eq!(inv.main_bag.count(), 1);
    }

    #[test]
    fn test_gold_adds_to_currency() {
        let mut inv = InventorySystem::new();
        let item = Item::new(0, 0, ItemKind::Gold, Rarity::Common);

        let result = inv.add_item(item, ItemSource::Unknown);
        assert_eq!(result, AddItemResult::AddedToCurrency);
        assert_eq!(inv.gold(), 1);
    }

    #[test]
    fn test_stacking() {
        let mut inv = InventorySystem::new();
        let item1 = Item::new(0, 0, ItemKind::HealthPotion, Rarity::Common);
        let item2 = Item::new(0, 0, ItemKind::HealthPotion, Rarity::Common);

        inv.add_item(item1, ItemSource::Unknown);
        inv.add_item(item2, ItemSource::Unknown);

        // Should stack into one slot
        assert_eq!(inv.main_bag.count(), 1);
        assert_eq!(inv.main_bag.get(0).unwrap().quantity, 2);
    }

    #[test]
    fn test_equip_item() {
        let mut inv = InventorySystem::new();
        let item = Item::new(0, 0, ItemKind::LongSword, Rarity::Common);

        inv.add_item(item, ItemSource::Unknown);
        let result = inv.equip_from_bag(0);

        assert!(result.is_ok());
        assert!(inv.equipment.get(EquipmentSlot::Weapon).is_some());
        assert_eq!(inv.main_bag.count(), 0);
    }

    #[test]
    fn test_unequip_item() {
        let mut inv = InventorySystem::new();
        let item = Item::new(0, 0, ItemKind::LongSword, Rarity::Common);

        inv.add_item(item, ItemSource::Unknown);
        inv.equip_from_bag(0).unwrap();

        let result = inv.unequip_to_bag(EquipmentSlot::Weapon);
        assert!(result.is_ok());
        assert!(inv.equipment.get(EquipmentSlot::Weapon).is_none());
        assert_eq!(inv.main_bag.count(), 1);
    }

    #[test]
    fn test_split_stack() {
        let mut inv = InventorySystem::new();
        let mut item = InventoryItem::new(Item::new(0, 0, ItemKind::HealthPotion, Rarity::Common));
        item.quantity = 10;

        inv.main_bag.add(item);

        let new_index = inv.split_stack(0, 3);
        assert!(new_index.is_some());
        assert_eq!(inv.main_bag.get(0).unwrap().quantity, 7);
        assert_eq!(
            inv.main_bag.get(new_index.unwrap()).unwrap().quantity,
            3
        );
    }

    #[test]
    fn test_favorite_toggle() {
        let mut inv = InventorySystem::new();
        let item = Item::new(0, 0, ItemKind::LongSword, Rarity::Common);

        inv.add_item(item, ItemSource::Unknown);
        assert!(!inv.main_bag.get(0).unwrap().is_favorite);

        inv.toggle_favorite(0);
        assert!(inv.main_bag.get(0).unwrap().is_favorite);

        inv.toggle_favorite(0);
        assert!(!inv.main_bag.get(0).unwrap().is_favorite);
    }

    #[test]
    fn test_junk_selling() {
        let mut inv = InventorySystem::new();

        let mut item1 = InventoryItem::new(Item::new(0, 0, ItemKind::Bread, Rarity::Common));
        item1.is_junk = true;

        let item2 = InventoryItem::new(Item::new(0, 0, ItemKind::LongSword, Rarity::Common));

        inv.main_bag.add(item1);
        inv.main_bag.add(item2);

        let value = inv.sell_all_junk();
        assert!(value > 0);
        assert_eq!(inv.main_bag.count(), 1);
        assert!(inv.gold() > 0);
    }

    #[test]
    fn test_weight_system() {
        let mut inv = InventorySystem::with_settings(DEFAULT_BAG_CAPACITY, true, 10);

        assert_eq!(
            inv.weight.max_capacity,
            BASE_CARRY_CAPACITY + 10.0 * CARRY_PER_STRENGTH
        );

        let item = Item::new(0, 0, ItemKind::PlateMail, Rarity::Common);
        inv.add_item(item, ItemSource::Unknown);

        assert!(inv.weight.current_weight > 0.0);
    }

    #[test]
    fn test_currency_pouch() {
        let mut pouch = CurrencyPouch::new();

        pouch.add_gold(100);
        assert_eq!(pouch.gold(), 100);

        assert!(pouch.remove_gold(50));
        assert_eq!(pouch.gold(), 50);

        assert!(!pouch.remove_gold(100));
        assert_eq!(pouch.gold(), 50);
    }

    #[test]
    fn test_material_storage() {
        let mut storage = MaterialStorage::new(100);

        let added = storage.add(CraftingMaterial::IronOre, 50);
        assert_eq!(added, 50);
        assert_eq!(storage.get(CraftingMaterial::IronOre), 50);

        assert!(storage.remove(CraftingMaterial::IronOre, 30));
        assert_eq!(storage.get(CraftingMaterial::IronOre), 20);
    }

    #[test]
    fn test_quick_slots() {
        let mut slots = QuickSlots::new();

        assert!(slots.assign(0, 5));
        assert_eq!(slots.get(0), Some(5));

        slots.update_indices(3);
        assert_eq!(slots.get(0), Some(4));

        slots.update_indices(4);
        assert_eq!(slots.get(0), None);
    }

    #[test]
    fn test_inventory_filter() {
        let inv = InventorySystem::new();

        let filter = InventoryFilter::new()
            .with_category(ItemCategory::Weapons)
            .with_search("sword".to_string());

        let mut test_item =
            InventoryItem::new(Item::new(0, 0, ItemKind::LongSword, Rarity::Common));
        assert!(filter.matches(&test_item));

        test_item = InventoryItem::new(Item::new(0, 0, ItemKind::HealthPotion, Rarity::Common));
        assert!(!filter.matches(&test_item));
    }

    #[test]
    fn test_portable_storage() {
        let mut storage = PortableStorage::new(PortableStorageType::BagOfHolding);

        assert_eq!(storage.capacity(), 30);
        assert!(storage.storage_type.is_weightless());

        let item = InventoryItem::new(Item::new(0, 0, ItemKind::HealthPotion, Rarity::Common));
        assert!(storage.add(item));
        assert_eq!(storage.count(), 1);
        assert_eq!(storage.contents_weight(), 0.0);
    }

    #[test]
    fn test_auto_loot_settings() {
        let mut settings = AutoLootSettings::default();

        let common_item = Item::new(0, 0, ItemKind::Bread, Rarity::Common);
        let rare_item = Item::new(0, 0, ItemKind::LongSword, Rarity::Rare);

        assert!(settings.should_loot(&common_item));
        assert!(settings.should_loot(&rare_item));

        settings.min_rarity = Rarity::Rare;
        assert!(!settings.should_loot(&common_item));
        assert!(settings.should_loot(&rare_item));
    }

    #[test]
    fn test_encumbrance_levels() {
        let mut weight = WeightSystem::new(true, 10);

        weight.current_weight = 0.0;
        assert_eq!(weight.encumbrance(), EncumbranceLevel::Light);

        weight.current_weight = weight.total_capacity() * 0.6;
        assert_eq!(weight.encumbrance(), EncumbranceLevel::Medium);

        weight.current_weight = weight.total_capacity() * 0.9;
        assert_eq!(weight.encumbrance(), EncumbranceLevel::Heavy);

        weight.current_weight = weight.total_capacity() * 1.2;
        assert_eq!(weight.encumbrance(), EncumbranceLevel::Overburdened);
    }
}

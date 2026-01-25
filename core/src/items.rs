//! Item system: items, equipment, and inventory
//!
//! Enhanced features:
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
    // Offensive
    Sharpness, FireAspect, FrostAspect, Thunderstrike, Lifesteal, Venomous, Executing, Crushing,
    // Defensive
    Protection, Thorns, Regeneration, ManaShield, Resilience, Fortification, Warding, Evasion,
    // Utility
    Swiftness, Enlightenment, Fortune, Soulbound, Unbreaking, Reaching, Illumination, Featherfall,
}

impl EnchantmentType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sharpness => "Sharpness", Self::FireAspect => "Fire Aspect",
            Self::FrostAspect => "Frost Aspect", Self::Thunderstrike => "Thunderstrike",
            Self::Lifesteal => "Lifesteal", Self::Venomous => "Venomous",
            Self::Executing => "Executing", Self::Crushing => "Crushing",
            Self::Protection => "Protection", Self::Thorns => "Thorns",
            Self::Regeneration => "Regeneration", Self::ManaShield => "Mana Shield",
            Self::Resilience => "Resilience", Self::Fortification => "Fortification",
            Self::Warding => "Warding", Self::Evasion => "Evasion",
            Self::Swiftness => "Swiftness", Self::Enlightenment => "Enlightenment",
            Self::Fortune => "Fortune", Self::Soulbound => "Soulbound",
            Self::Unbreaking => "Unbreaking", Self::Reaching => "Reaching",
            Self::Illumination => "Illumination", Self::Featherfall => "Featherfall",
        }
    }

    pub fn max_level(&self) -> u8 {
        match self {
            Self::Sharpness | Self::Protection | Self::Fortification => 5,
            Self::Soulbound | Self::Unbreaking => 1,
            _ => 3,
        }
    }

    pub fn valid_for_slot(&self, slot: EquipSlot) -> bool {
        match self {
            Self::Sharpness | Self::FireAspect | Self::FrostAspect | Self::Thunderstrike
            | Self::Lifesteal | Self::Venomous | Self::Executing | Self::Crushing | Self::Reaching =>
                matches!(slot, EquipSlot::Weapon),
            Self::Protection | Self::Thorns | Self::Resilience | Self::Fortification | Self::Warding =>
                matches!(slot, EquipSlot::Armor | EquipSlot::Helmet | EquipSlot::Shield),
            Self::Swiftness | Self::Featherfall => matches!(slot, EquipSlot::Boots),
            _ => true,
        }
    }
}

/// An enchantment with type and level
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Enchantment {
    pub enchant_type: EnchantmentType,
    pub level: u8,
}

impl Enchantment {
    pub fn new(enchant_type: EnchantmentType, level: u8) -> Self {
        Self { enchant_type, level: level.min(enchant_type.max_level()).max(1) }
    }

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
            _ => (0, 0, 0, 0),
        }
    }

    pub fn display_name(&self) -> String {
        let num = match self.level { 1=>"I", 2=>"II", 3=>"III", 4=>"IV", 5=>"V", _=>"?" };
        format!("{} {}", self.enchant_type.name(), num)
    }
}

// ============================================================================
// ITEM SETS SYSTEM
// ============================================================================

/// Item sets that provide bonuses when multiple pieces are equipped
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ItemSet {
    DragonSlayer, TitanMight, BerserkerRage, ArcaneScholar, ElementalMaster,
    VoidWalker, ShadowDancer, AssassinsBlade, NightStalker, PaladinValor,
    DeathKnight, PhoenixRebirth, AncientKings, DemonLord, CelestialGuard,
}

/// Special effects granted by set bonuses
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SetEffect {
    DragonBane, DragonSlayerAura, Unstoppable, Frenzy, Berserk, ManaRegen, ArcaneAffinity,
    ElementalResist, ElementalMastery, ElementalOverload, VoidTouch, VoidEmbrace,
    ShadowStep, ShadowMeld, CriticalStrike, Assassination, NightVision, Invisibility,
    HolyAura, DivineProtection, LifeDrain, DeathGrip, FlameAura, PhoenixRise,
    RoyalPresence, KingsMandate, DemonFire, DemonicPact, CelestialBlessing, DivineIntervention,
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

impl ItemSet {
    pub fn name(&self) -> &'static str {
        match self {
            Self::DragonSlayer => "Dragon Slayer", Self::TitanMight => "Titan's Might",
            Self::BerserkerRage => "Berserker's Rage", Self::ArcaneScholar => "Arcane Scholar",
            Self::ElementalMaster => "Elemental Master", Self::VoidWalker => "Void Walker",
            Self::ShadowDancer => "Shadow Dancer", Self::AssassinsBlade => "Assassin's Blade",
            Self::NightStalker => "Night Stalker", Self::PaladinValor => "Paladin's Valor",
            Self::DeathKnight => "Death Knight", Self::PhoenixRebirth => "Phoenix Rebirth",
            Self::AncientKings => "Ancient Kings", Self::DemonLord => "Demon Lord",
            Self::CelestialGuard => "Celestial Guard",
        }
    }

    pub fn pieces(&self) -> Vec<ItemKind> {
        match self {
            Self::DragonSlayer => vec![ItemKind::DragonHelm, ItemKind::DragonArmor, ItemKind::DragonGauntlets, ItemKind::DragonShield],
            Self::TitanMight => vec![ItemKind::TitanPlate, ItemKind::HelmOfValor, ItemKind::GauntletsOfMight],
            Self::ShadowDancer => vec![ItemKind::AssassinGarb, ItemKind::HoodOfShadows, ItemKind::ShadowBoots, ItemKind::ThievesGloves],
            Self::ArcaneScholar => vec![ItemKind::MageRobes, ItemKind::WizardHat, ItemKind::VoidStaff, ItemKind::RingOfMana],
            Self::ElementalMaster => vec![ItemKind::FlameSword, ItemKind::FrostBlade, ItemKind::ThunderAxe, ItemKind::RingOfFlame, ItemKind::RingOfFrost],
            Self::DeathKnight => vec![ItemKind::DemonArmor, ItemKind::DemonSkull, ItemKind::DemonSlayer, ItemKind::RingOfTheVampire],
            Self::PaladinValor => vec![ItemKind::HolyArmor, ItemKind::HelmOfValor, ItemKind::PhoenixShield, ItemKind::AmuletOfLife],
            Self::PhoenixRebirth => vec![ItemKind::PhoenixShield, ItemKind::FlameGauntlets, ItemKind::RingOfFlame],
            _ => vec![],
        }
    }

    pub fn bonus_for_pieces(&self, pieces: u8) -> SetBonus {
        match self {
            Self::DragonSlayer => match pieces {
                2 => SetBonus { attack: 5, defense: 5, hp: 20, mana: 0, effect: Some(SetEffect::DragonBane) },
                3 => SetBonus { attack: 10, defense: 10, hp: 40, mana: 0, effect: Some(SetEffect::DragonBane) },
                4 => SetBonus { attack: 20, defense: 15, hp: 60, mana: 10, effect: Some(SetEffect::DragonSlayerAura) },
                _ => SetBonus::default(),
            },
            Self::TitanMight => match pieces {
                2 => SetBonus { attack: 0, defense: 10, hp: 30, mana: 0, effect: None },
                3 => SetBonus { attack: 5, defense: 20, hp: 60, mana: 0, effect: Some(SetEffect::Unstoppable) },
                _ => SetBonus::default(),
            },
            Self::ShadowDancer => match pieces {
                2 => SetBonus { attack: 5, defense: 2, hp: 0, mana: 10, effect: Some(SetEffect::ShadowStep) },
                3 => SetBonus { attack: 10, defense: 5, hp: 0, mana: 20, effect: Some(SetEffect::ShadowStep) },
                4 => SetBonus { attack: 15, defense: 8, hp: 10, mana: 30, effect: Some(SetEffect::ShadowMeld) },
                _ => SetBonus::default(),
            },
            Self::DeathKnight => match pieces {
                2 => SetBonus { attack: 10, defense: 5, hp: 0, mana: 0, effect: Some(SetEffect::LifeDrain) },
                3 => SetBonus { attack: 15, defense: 10, hp: 0, mana: 0, effect: Some(SetEffect::LifeDrain) },
                4 => SetBonus { attack: 25, defense: 15, hp: 20, mana: 0, effect: Some(SetEffect::DeathGrip) },
                _ => SetBonus::default(),
            },
            _ => SetBonus::default(),
        }
    }
}

// ============================================================================
// UNIQUE ITEMS SYSTEM
// ============================================================================

/// Unique items with special effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum UniqueItem {
    Excalibur, Mjolnir, Gungnir, Masamune, DeathsScythe, StaffOfAges, SerpentsFang,
    Stormbringer, FrostmournesBlade, InfernosBrand, AegisOfTheGods, DragonhideMantle,
    ValkyriesWings, EternityRobes, PhantomShroud, RingOfOmniscience, AmuletOfYggdrasil,
    CrownOfEternals, BootsOfHermes, GlovesOfMidas, SoulReaver, WorldEnder, TimeSplitter,
}

/// Special effects unique to legendary items
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum UniqueEffect {
    HolySmite, LightningStorm, NeverMiss, PerfectCut, InstantDeath, TimeWarp, DeadlyVenom,
    ChainLightning, SoulSteal, Immolate, DivineShield, DragonBreath, Flight, TimeStop,
    Phasing, TrueSight, WorldTreeBlessing, Immortality, Hyperspeed, GoldenTouch,
    SoulAbsorb, Apocalypse, TemporalSlash,
}

impl UniqueItem {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Excalibur => "Excalibur", Self::Mjolnir => "Mjolnir",
            Self::Gungnir => "Gungnir", Self::Masamune => "Masamune",
            Self::DeathsScythe => "Death's Scythe", Self::StaffOfAges => "Staff of Ages",
            Self::SerpentsFang => "Serpent's Fang", Self::Stormbringer => "Stormbringer",
            Self::FrostmournesBlade => "Frostmourne's Blade", Self::InfernosBrand => "Inferno's Brand",
            Self::AegisOfTheGods => "Aegis of the Gods", Self::DragonhideMantle => "Dragonhide Mantle",
            Self::ValkyriesWings => "Valkyrie's Wings", Self::EternityRobes => "Eternity Robes",
            Self::PhantomShroud => "Phantom Shroud", Self::RingOfOmniscience => "Ring of Omniscience",
            Self::AmuletOfYggdrasil => "Amulet of Yggdrasil", Self::CrownOfEternals => "Crown of Eternals",
            Self::BootsOfHermes => "Boots of Hermes", Self::GlovesOfMidas => "Gloves of Midas",
            Self::SoulReaver => "Soul Reaver", Self::WorldEnder => "World Ender",
            Self::TimeSplitter => "Time Splitter",
        }
    }

    pub fn lore(&self) -> &'static str {
        match self {
            Self::Excalibur => "The legendary sword pulled from the stone, destined for true kings.",
            Self::Mjolnir => "Thor's mighty hammer, capable of summoning lightning from the heavens.",
            Self::Gungnir => "The spear of Odin, which never misses its mark.",
            Self::Masamune => "A blade of perfect balance, forged by the legendary smith.",
            Self::DeathsScythe => "The weapon of the Grim Reaper himself.",
            Self::StaffOfAges => "Channeling magic from the dawn of time.",
            _ => "A legendary artifact of immense power.",
        }
    }

    pub fn base_stats(&self) -> (i32, i32, i32, i32) {
        match self {
            Self::Excalibur => (50, 10, 30, 20), Self::Mjolnir => (45, 5, 0, 40),
            Self::Gungnir => (55, 0, 0, 10), Self::Masamune => (60, 5, 0, 0),
            Self::DeathsScythe => (70, 0, -50, 0), Self::StaffOfAges => (25, 5, 20, 100),
            Self::SerpentsFang => (35, 0, 0, 20), Self::Stormbringer => (40, 0, 0, 50),
            Self::FrostmournesBlade => (55, 0, -30, 30), Self::InfernosBrand => (50, 0, 0, 30),
            Self::AegisOfTheGods => (0, 40, 50, 30), Self::DragonhideMantle => (5, 50, 40, 0),
            Self::ValkyriesWings => (10, 30, 30, 30), Self::EternityRobes => (0, 20, 40, 100),
            Self::PhantomShroud => (10, 25, 0, 40), Self::RingOfOmniscience => (10, 10, 30, 60),
            Self::AmuletOfYggdrasil => (0, 15, 100, 50), Self::CrownOfEternals => (15, 15, 50, 50),
            Self::BootsOfHermes => (5, 10, 20, 30), Self::GlovesOfMidas => (20, 5, 10, 10),
            Self::SoulReaver => (65, 0, 0, 0), Self::WorldEnder => (80, 0, -60, 0),
            Self::TimeSplitter => (45, 10, 20, 40),
        }
    }

    pub fn equip_slot(&self) -> EquipSlot {
        match self {
            Self::Excalibur | Self::Mjolnir | Self::Gungnir | Self::Masamune | Self::DeathsScythe
            | Self::StaffOfAges | Self::SerpentsFang | Self::Stormbringer | Self::FrostmournesBlade
            | Self::InfernosBrand | Self::SoulReaver | Self::WorldEnder | Self::TimeSplitter => EquipSlot::Weapon,
            Self::AegisOfTheGods => EquipSlot::Shield,
            Self::DragonhideMantle | Self::ValkyriesWings | Self::EternityRobes | Self::PhantomShroud => EquipSlot::Armor,
            Self::CrownOfEternals => EquipSlot::Helmet,
            Self::BootsOfHermes => EquipSlot::Boots,
            Self::GlovesOfMidas => EquipSlot::Gloves,
            Self::RingOfOmniscience => EquipSlot::Ring1,
            Self::AmuletOfYggdrasil => EquipSlot::Amulet,
        }
    }

    pub fn special_effect(&self) -> UniqueEffect {
        match self {
            Self::Excalibur => UniqueEffect::HolySmite, Self::Mjolnir => UniqueEffect::LightningStorm,
            Self::Gungnir => UniqueEffect::NeverMiss, Self::Masamune => UniqueEffect::PerfectCut,
            Self::DeathsScythe => UniqueEffect::InstantDeath, Self::StaffOfAges => UniqueEffect::TimeWarp,
            Self::SerpentsFang => UniqueEffect::DeadlyVenom, Self::Stormbringer => UniqueEffect::ChainLightning,
            Self::FrostmournesBlade => UniqueEffect::SoulSteal, Self::InfernosBrand => UniqueEffect::Immolate,
            Self::AegisOfTheGods => UniqueEffect::DivineShield, Self::DragonhideMantle => UniqueEffect::DragonBreath,
            Self::ValkyriesWings => UniqueEffect::Flight, Self::EternityRobes => UniqueEffect::TimeStop,
            Self::PhantomShroud => UniqueEffect::Phasing, Self::RingOfOmniscience => UniqueEffect::TrueSight,
            Self::AmuletOfYggdrasil => UniqueEffect::WorldTreeBlessing, Self::CrownOfEternals => UniqueEffect::Immortality,
            Self::BootsOfHermes => UniqueEffect::Hyperspeed, Self::GlovesOfMidas => UniqueEffect::GoldenTouch,
            Self::SoulReaver => UniqueEffect::SoulAbsorb, Self::WorldEnder => UniqueEffect::Apocalypse,
            Self::TimeSplitter => UniqueEffect::TemporalSlash,
        }
    }
}

impl UniqueEffect {
    pub fn name(&self) -> &'static str {
        match self {
            Self::HolySmite => "Holy Smite", Self::LightningStorm => "Lightning Storm",
            Self::NeverMiss => "Unerring Strike", Self::PerfectCut => "Perfect Cut",
            Self::InstantDeath => "Touch of Death", Self::TimeWarp => "Time Warp",
            Self::DeadlyVenom => "Deadly Venom", Self::ChainLightning => "Chain Lightning",
            Self::SoulSteal => "Soul Steal", Self::Immolate => "Immolate",
            Self::DivineShield => "Divine Shield", Self::DragonBreath => "Dragon Breath",
            Self::Flight => "Flight", Self::TimeStop => "Time Stop",
            Self::Phasing => "Phasing", Self::TrueSight => "True Sight",
            Self::WorldTreeBlessing => "World Tree's Blessing", Self::Immortality => "Immortality",
            Self::Hyperspeed => "Hyperspeed", Self::GoldenTouch => "Golden Touch",
            Self::SoulAbsorb => "Soul Absorb", Self::Apocalypse => "Apocalypse",
            Self::TemporalSlash => "Temporal Slash",
        }
    }
}

// ============================================================================
// CRAFTING SYSTEM
// ============================================================================

/// Material types used in crafting
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CraftingMaterial {
    IronOre, SteelIngot, MithrilOre, AdamantiteOre, Leather, DragonLeather, Cloth, SilkCloth,
    Ruby, Sapphire, Emerald, Diamond, Amethyst, Topaz, BlackOpal, StarSapphire,
    FireEssence, IceEssence, LightningEssence, VoidEssence, HolyEssence, DarkEssence,
    LifeEssence, DeathEssence, DragonHeart, PhoenixFeather, DemonCore, AngelWing,
    TitanBone, ElementalCore, AncientRune, PrimordialShard,
}

impl CraftingMaterial {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IronOre => "Iron Ore", Self::SteelIngot => "Steel Ingot",
            Self::MithrilOre => "Mithril Ore", Self::AdamantiteOre => "Adamantite Ore",
            Self::Leather => "Leather", Self::DragonLeather => "Dragon Leather",
            Self::Cloth => "Cloth", Self::SilkCloth => "Silk Cloth",
            Self::Ruby => "Ruby", Self::Sapphire => "Sapphire", Self::Emerald => "Emerald",
            Self::Diamond => "Diamond", Self::Amethyst => "Amethyst", Self::Topaz => "Topaz",
            Self::BlackOpal => "Black Opal", Self::StarSapphire => "Star Sapphire",
            Self::FireEssence => "Fire Essence", Self::IceEssence => "Ice Essence",
            Self::LightningEssence => "Lightning Essence", Self::VoidEssence => "Void Essence",
            Self::HolyEssence => "Holy Essence", Self::DarkEssence => "Dark Essence",
            Self::LifeEssence => "Life Essence", Self::DeathEssence => "Death Essence",
            Self::DragonHeart => "Dragon Heart", Self::PhoenixFeather => "Phoenix Feather",
            Self::DemonCore => "Demon Core", Self::AngelWing => "Angel Wing",
            Self::TitanBone => "Titan Bone", Self::ElementalCore => "Elemental Core",
            Self::AncientRune => "Ancient Rune", Self::PrimordialShard => "Primordial Shard",
        }
    }

    pub fn rarity(&self) -> Rarity {
        match self {
            Self::IronOre | Self::Leather | Self::Cloth => Rarity::Common,
            Self::SteelIngot | Self::SilkCloth => Rarity::Uncommon,
            Self::MithrilOre | Self::DragonLeather | Self::Ruby | Self::Sapphire | Self::Emerald => Rarity::Rare,
            Self::AdamantiteOre | Self::Diamond | Self::Amethyst | Self::Topaz
            | Self::FireEssence | Self::IceEssence | Self::LightningEssence => Rarity::Epic,
            Self::BlackOpal | Self::StarSapphire | Self::VoidEssence | Self::HolyEssence
            | Self::DarkEssence | Self::LifeEssence | Self::DeathEssence | Self::DragonHeart
            | Self::PhoenixFeather | Self::DemonCore | Self::AngelWing => Rarity::Legendary,
            Self::TitanBone | Self::ElementalCore | Self::AncientRune | Self::PrimordialShard => Rarity::Mythic,
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            Self::IronOre | Self::SteelIngot | Self::MithrilOre | Self::AdamantiteOre => '#',
            Self::Leather | Self::DragonLeather => '~',
            Self::Cloth | Self::SilkCloth => '=',
            Self::Ruby | Self::Sapphire | Self::Emerald | Self::Diamond | Self::Amethyst
            | Self::Topaz | Self::BlackOpal | Self::StarSapphire => '*',
            Self::FireEssence | Self::IceEssence | Self::LightningEssence | Self::VoidEssence
            | Self::HolyEssence | Self::DarkEssence | Self::LifeEssence | Self::DeathEssence => '@',
            _ => '&',
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

/// Returns core crafting recipes
pub fn get_crafting_recipes() -> Vec<CraftingRecipe> {
    vec![
        CraftingRecipe {
            name: "Forge Steel Sword", description: "Craft a reliable steel sword",
            materials: vec![(CraftingMaterial::SteelIngot, 3), (CraftingMaterial::Leather, 1)],
            result: CraftingResult::Item(ItemKind::LongSword, Rarity::Uncommon), required_level: 1,
        },
        CraftingRecipe {
            name: "Forge Flame Sword", description: "Imbue a blade with eternal fire",
            materials: vec![(CraftingMaterial::SteelIngot, 3), (CraftingMaterial::FireEssence, 2), (CraftingMaterial::Ruby, 1)],
            result: CraftingResult::Item(ItemKind::FlameSword, Rarity::Rare), required_level: 8,
        },
        CraftingRecipe {
            name: "Craft Dragon Armor", description: "Fashion armor from dragon scales",
            materials: vec![(CraftingMaterial::DragonLeather, 5), (CraftingMaterial::DragonHeart, 1), (CraftingMaterial::AdamantiteOre, 3)],
            result: CraftingResult::Item(ItemKind::DragonArmor, Rarity::Legendary), required_level: 15,
        },
        CraftingRecipe {
            name: "Essence of Sharpness", description: "Create a sharpness enchantment",
            materials: vec![(CraftingMaterial::SteelIngot, 2), (CraftingMaterial::Ruby, 1)],
            result: CraftingResult::Enchantment(EnchantmentType::Sharpness, 1), required_level: 3,
        },
        CraftingRecipe {
            name: "Forge Excalibur", description: "The legendary sword of kings",
            materials: vec![
                (CraftingMaterial::AdamantiteOre, 10), (CraftingMaterial::HolyEssence, 5),
                (CraftingMaterial::Diamond, 3), (CraftingMaterial::AngelWing, 2),
                (CraftingMaterial::PrimordialShard, 1),
            ],
            result: CraftingResult::UniqueItem(UniqueItem::Excalibur), required_level: 25,
        },
        CraftingRecipe {
            name: "Smelt Steel", description: "Convert iron into steel",
            materials: vec![(CraftingMaterial::IronOre, 3)],
            result: CraftingResult::Material(CraftingMaterial::SteelIngot, 1), required_level: 1,
        },
    ]
}

/// Equipment slots
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

/// Item rarity tiers
#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

/// Food quality levels - affects hunger restoration and bonuses
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum FoodQuality {
    Rotten,      // Spoiled food - might cause sickness
    Raw,         // Uncooked - less effective
    Stale,       // Old food - reduced effectiveness
    Fresh,       // Normal quality
    Cooked,      // Cooked food - bonus effectiveness
    WellCooked,  // Skillfully prepared
    Gourmet,     // Master chef quality
    Legendary,   // Magical/divine food
}

impl FoodQuality {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            FoodQuality::Rotten => "Rotten",
            FoodQuality::Raw => "Raw",
            FoodQuality::Stale => "Stale",
            FoodQuality::Fresh => "Fresh",
            FoodQuality::Cooked => "Cooked",
            FoodQuality::WellCooked => "Well-Cooked",
            FoodQuality::Gourmet => "Gourmet",
            FoodQuality::Legendary => "Legendary",
        }
    }

    /// Get color index for display
    pub fn color_index(&self) -> u8 {
        match self {
            FoodQuality::Rotten => 4,       // Dark/brown
            FoodQuality::Raw => 3,          // Red
            FoodQuality::Stale => 6,        // Orange
            FoodQuality::Fresh => 1,        // White
            FoodQuality::Cooked => 5,       // Green
            FoodQuality::WellCooked => 13,  // Bright green
            FoodQuality::Gourmet => 11,     // Yellow/gold
            FoodQuality::Legendary => 7,    // Cyan/magic
        }
    }

    /// Get the hunger value multiplier for this quality
    pub fn hunger_multiplier(&self) -> f32 {
        match self {
            FoodQuality::Rotten => 0.3,
            FoodQuality::Raw => 0.6,
            FoodQuality::Stale => 0.8,
            FoodQuality::Fresh => 1.0,
            FoodQuality::Cooked => 1.3,
            FoodQuality::WellCooked => 1.5,
            FoodQuality::Gourmet => 2.0,
            FoodQuality::Legendary => 3.0,
        }
    }

    /// Check if food can be cooked
    pub fn can_cook(&self) -> bool {
        matches!(self, FoodQuality::Raw | FoodQuality::Fresh)
    }

    /// Check if food can spoil/decay
    pub fn can_spoil(&self) -> bool {
        matches!(self, FoodQuality::Fresh | FoodQuality::Cooked | FoodQuality::WellCooked)
    }

    /// Get next quality level when food spoils
    pub fn spoiled(&self) -> FoodQuality {
        match self {
            FoodQuality::Fresh => FoodQuality::Stale,
            FoodQuality::Cooked => FoodQuality::Stale,
            FoodQuality::WellCooked => FoodQuality::Cooked,
            FoodQuality::Gourmet => FoodQuality::WellCooked,
            FoodQuality::Stale => FoodQuality::Rotten,
            _ => *self,
        }
    }
}

impl Rarity {
    /// Returns a color index for the rarity (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Common => 1,      // Grey
            Self::Uncommon => 5,    // Green
            Self::Rare => 7,        // Blue
            Self::Epic => 13,       // Magenta
            Self::Legendary => 11,  // Yellow
            Self::Mythic => 3,      // Red
        }
    }

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

    /// Returns the stat multiplier for this rarity
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
}

/// All item types in the game
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
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

    // Food - Basic (8)
    Bread,
    Meat,
    Apple,
    Cheese,
    Feast,
    DragonFruit,
    AncientWine,
    GoldenApple,

    // Food - Raw ingredients (can be cooked)
    RawMeat,
    RawFish,
    RawVegetables,
    RawEgg,
    Mushrooms,
    RawPoultry,

    // Food - Cooked dishes
    CookedMeat,
    GrilledFish,
    Stew,
    Omelette,
    RoastChicken,
    MeatPie,
    FruitSalad,
    HeartyStew,
    DragonSteak,
    FeastOfKings,

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
    /// Returns the glyph character for this item
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
            | Self::DragonFruit | Self::AncientWine | Self::GoldenApple
            | Self::RawMeat | Self::RawFish | Self::RawVegetables | Self::RawEgg
            | Self::Mushrooms | Self::RawPoultry | Self::CookedMeat | Self::GrilledFish
            | Self::Stew | Self::Omelette | Self::RoastChicken | Self::MeatPie
            | Self::FruitSalad | Self::HeartyStew | Self::DragonSteak | Self::FeastOfKings => '%',

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

            // Raw ingredients
            Self::RawMeat => "Raw Meat",
            Self::RawFish => "Raw Fish",
            Self::RawVegetables => "Raw Vegetables",
            Self::RawEgg => "Raw Egg",
            Self::Mushrooms => "Wild Mushrooms",
            Self::RawPoultry => "Raw Poultry",

            // Cooked dishes
            Self::CookedMeat => "Cooked Meat",
            Self::GrilledFish => "Grilled Fish",
            Self::Stew => "Hearty Stew",
            Self::Omelette => "Omelette",
            Self::RoastChicken => "Roast Chicken",
            Self::MeatPie => "Meat Pie",
            Self::FruitSalad => "Fruit Salad",
            Self::HeartyStew => "Hunter's Stew",
            Self::DragonSteak => "Dragon Steak",
            Self::FeastOfKings => "Feast of Kings",

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

    /// Returns the equipment slot for this item, if any
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

    /// Returns base stats: (attack, defense, hp_bonus, mana_bonus)
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
                | Self::RawMeat | Self::RawFish | Self::RawVegetables | Self::RawEgg
                | Self::Mushrooms | Self::RawPoultry | Self::CookedMeat | Self::GrilledFish
                | Self::Stew | Self::Omelette | Self::RoastChicken | Self::MeatPie
                | Self::FruitSalad | Self::HeartyStew | Self::DragonSteak | Self::FeastOfKings
        )
    }

    /// Returns whether this item can be cooked
    pub fn is_cookable(&self) -> bool {
        matches!(
            self,
            Self::RawMeat | Self::RawFish | Self::RawVegetables | Self::RawEgg
                | Self::Mushrooms | Self::RawPoultry | Self::Meat
        )
    }

    /// Returns the base food/hunger value of this item (before quality adjustment)
    pub fn food_value(&self) -> i32 {
        match self {
            // Basic foods
            Self::Apple => 10,
            Self::Bread => 25,
            Self::Cheese => 20,
            Self::Meat => 40,
            Self::Feast => 100,
            Self::DragonFruit => 30,
            Self::AncientWine => 35,
            Self::GoldenApple => 50,

            // Raw ingredients (lower base value)
            Self::RawMeat => 25,
            Self::RawFish => 20,
            Self::RawVegetables => 15,
            Self::RawEgg => 10,
            Self::Mushrooms => 12,
            Self::RawPoultry => 22,

            // Cooked dishes (higher base value)
            Self::CookedMeat => 45,
            Self::GrilledFish => 40,
            Self::Stew => 55,
            Self::Omelette => 35,
            Self::RoastChicken => 50,
            Self::MeatPie => 60,
            Self::FruitSalad => 30,
            Self::HeartyStew => 70,
            Self::DragonSteak => 90,
            Self::FeastOfKings => 150,

            _ => 0,
        }
    }

    /// Returns the default food quality for this item type
    pub fn default_food_quality(&self) -> FoodQuality {
        match self {
            // Raw ingredients default to Raw quality
            Self::RawMeat | Self::RawFish | Self::RawVegetables
            | Self::RawEgg | Self::RawPoultry => FoodQuality::Raw,

            // Wild mushrooms are risky
            Self::Mushrooms => FoodQuality::Raw,

            // Basic preserved foods are Fresh
            Self::Bread | Self::Cheese | Self::Apple | Self::Meat => FoodQuality::Fresh,

            // Cooked dishes
            Self::CookedMeat | Self::GrilledFish | Self::Omelette => FoodQuality::Cooked,
            Self::Stew | Self::RoastChicken | Self::MeatPie | Self::FruitSalad => FoodQuality::WellCooked,
            Self::HeartyStew => FoodQuality::WellCooked,

            // Special foods
            Self::DragonFruit | Self::AncientWine | Self::GoldenApple => FoodQuality::Gourmet,
            Self::DragonSteak => FoodQuality::Gourmet,
            Self::Feast | Self::FeastOfKings => FoodQuality::Legendary,

            _ => FoodQuality::Fresh,
        }
    }

    /// Returns what this food item turns into when cooked (if cookable)
    pub fn cooked_result(&self) -> Option<ItemKind> {
        match self {
            Self::RawMeat | Self::Meat => Some(Self::CookedMeat),
            Self::RawFish => Some(Self::GrilledFish),
            Self::RawEgg => Some(Self::Omelette),
            Self::RawPoultry => Some(Self::RoastChicken),
            Self::RawVegetables => Some(Self::Stew),
            Self::Mushrooms => Some(Self::Stew),
            _ => None,
        }
    }

    /// Check if combining two ingredients creates a special dish
    pub fn combine_ingredients(a: ItemKind, b: ItemKind) -> Option<ItemKind> {
        let mut ingredients = [a, b];
        ingredients.sort_by_key(|i| *i as i32);

        match ingredients {
            [ItemKind::RawMeat, ItemKind::RawVegetables] => Some(ItemKind::HeartyStew),
            [ItemKind::CookedMeat, ItemKind::Bread] => Some(ItemKind::MeatPie),
            [ItemKind::Apple, ItemKind::DragonFruit] => Some(ItemKind::FruitSalad),
            [ItemKind::RawMeat, ItemKind::DragonScale] => Some(ItemKind::DragonSteak),
            [ItemKind::Feast, ItemKind::GoldenApple] => Some(ItemKind::FeastOfKings),
            _ => None,
        }
    }
}

/// An item instance with position, rarity, enchantments, and more
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub x: usize,
    pub y: usize,
    pub kind: ItemKind,
    pub rarity: Rarity,
    /// Food quality (only relevant for food items)
    pub food_quality: Option<FoodQuality>,
    /// Turns until food spoils (0 = doesn't spoil)
    pub spoil_timer: u32,
    /// Enchantments applied to this item
    #[serde(default)]
    pub enchantments: Vec<Enchantment>,
    /// If this is a unique item
    #[serde(default)]
    pub unique: Option<UniqueItem>,
    /// If identified (for unknown items)
    #[serde(default = "default_true")]
    pub identified: bool,
    /// Durability (100 = full, 0 = broken)
    #[serde(default = "default_durability")]
    pub durability: u8,
    /// Number of times this item has been upgraded
    #[serde(default)]
    pub upgrade_level: u8,
}

fn default_true() -> bool { true }
fn default_durability() -> u8 { 100 }

impl Item {
    /// Create a new item at the given position
    pub fn new(x: usize, y: usize, kind: ItemKind, rarity: Rarity) -> Self {
        let (food_quality, spoil_timer) = if kind.is_food() {
            let quality = kind.default_food_quality();
            let timer = if quality.can_spoil() { 500 } else { 0 };
            (Some(quality), timer)
        } else {
            (None, 0)
        };

        Self {
            x, y, kind, rarity, food_quality, spoil_timer,
            enchantments: Vec::new(), unique: None,
            identified: true, durability: 100, upgrade_level: 0,
        }
    }

    /// Create a new unique item
    pub fn new_unique(x: usize, y: usize, unique: UniqueItem) -> Self {
        Self {
            x, y, kind: ItemKind::AncientRelic, rarity: Rarity::Mythic,
            food_quality: None, spoil_timer: 0,
            enchantments: Vec::new(), unique: Some(unique),
            identified: false, durability: 100, upgrade_level: 0,
        }
    }

    /// Create an item with enchantments
    pub fn new_enchanted(x: usize, y: usize, kind: ItemKind, rarity: Rarity, enchantments: Vec<Enchantment>) -> Self {
        let mut item = Self::new(x, y, kind, rarity);
        item.enchantments = enchantments;
        item
    }

    /// Create a food item with specific quality
    pub fn new_food(x: usize, y: usize, kind: ItemKind, quality: FoodQuality) -> Self {
        let spoil_timer = if quality.can_spoil() { 500 } else { 0 };
        Self {
            x, y, kind, rarity: Rarity::Common,
            food_quality: Some(quality), spoil_timer,
            enchantments: Vec::new(), unique: None,
            identified: true, durability: 100, upgrade_level: 0,
        }
    }

    /// Get the food quality, defaulting to Fresh if not set
    pub fn get_food_quality(&self) -> FoodQuality {
        self.food_quality.unwrap_or(self.kind.default_food_quality())
    }

    /// Tick the spoil timer and potentially degrade quality
    pub fn tick_spoil(&mut self) -> bool {
        if self.spoil_timer > 0 {
            self.spoil_timer -= 1;
            if self.spoil_timer == 0 {
                if let Some(ref mut quality) = self.food_quality {
                    *quality = quality.spoiled();
                    if quality.can_spoil() { self.spoil_timer = 300; }
                    return true;
                }
            }
        }
        false
    }

    /// Returns the stats of this item scaled by rarity and enchantments
    pub fn stats(&self) -> (i32, i32, i32, i32) {
        // Unique items use their own stats
        if let Some(unique) = &self.unique {
            return unique.base_stats();
        }

        let (atk, def, hp, mana) = self.kind.base_stats();
        let mult = self.rarity.stat_bonus();
        let mut final_atk = (atk as f32 * mult) as i32;
        let mut final_def = (def as f32 * mult) as i32;
        let mut final_hp = (hp as f32 * mult) as i32;
        let mut final_mana = (mana as f32 * mult) as i32;

        // Add enchantment bonuses
        for ench in &self.enchantments {
            let (e_atk, e_def, e_hp, e_mana) = ench.stat_bonus();
            final_atk += e_atk; final_def += e_def; final_hp += e_hp; final_mana += e_mana;
        }

        // Apply upgrade bonus (+5% per level)
        let upgrade_mult = 1.0 + (self.upgrade_level as f32 * 0.05);
        final_atk = (final_atk as f32 * upgrade_mult) as i32;
        final_def = (final_def as f32 * upgrade_mult) as i32;

        (final_atk, final_def, final_hp, final_mana)
    }

    /// Returns the display name including rarity prefix and enchantments
    pub fn display_name(&self) -> String {
        if let Some(unique) = &self.unique {
            return if self.identified { unique.name().to_string() } else { "Mysterious Artifact".to_string() };
        }

        let mut name = format!("{}{}", self.rarity.prefix(), self.kind.name());
        if self.upgrade_level > 0 { name = format!("{} +{}", name, self.upgrade_level); }
        if let Some(ench) = self.enchantments.first() {
            name = format!("{} of {}", name, ench.enchant_type.name());
        }
        name
    }

    /// Add an enchantment to this item
    pub fn add_enchantment(&mut self, enchantment: Enchantment) -> bool {
        if let Some(slot) = self.kind.equip_slot() {
            if !enchantment.enchant_type.valid_for_slot(slot) { return false; }
        }
        for existing in &mut self.enchantments {
            if existing.enchant_type == enchantment.enchant_type {
                if existing.level < existing.enchant_type.max_level() {
                    existing.level += 1; return true;
                }
                return false;
            }
        }
        let max = match self.rarity { Rarity::Common => 1, Rarity::Uncommon => 2, Rarity::Rare => 3, Rarity::Epic => 4, Rarity::Legendary => 5, Rarity::Mythic => 6 };
        if self.enchantments.len() >= max { return false; }
        self.enchantments.push(enchantment);
        true
    }

    /// Check if item has a specific enchantment
    pub fn has_enchantment(&self, enchant_type: EnchantmentType) -> bool {
        self.enchantments.iter().any(|e| e.enchant_type == enchant_type)
    }

    /// Get enchantment level for a type (0 if not present)
    pub fn enchantment_level(&self, enchant_type: EnchantmentType) -> u8 {
        self.enchantments.iter().find(|e| e.enchant_type == enchant_type).map(|e| e.level).unwrap_or(0)
    }

    /// Upgrade this item
    pub fn upgrade(&mut self) -> bool {
        let max = match self.rarity { Rarity::Common => 3, Rarity::Uncommon => 5, Rarity::Rare => 7, Rarity::Epic => 10, Rarity::Legendary => 15, Rarity::Mythic => 20 };
        if self.upgrade_level < max { self.upgrade_level += 1; true } else { false }
    }

    /// Repair this item
    pub fn repair(&mut self, amount: u8) { self.durability = (self.durability + amount).min(100); }

    /// Damage this item's durability
    pub fn damage(&mut self, amount: u8) { self.durability = self.durability.saturating_sub(amount); }

    /// Check if item is broken
    pub fn is_broken(&self) -> bool { self.durability == 0 }

    /// Get the set this item belongs to
    pub fn item_set(&self) -> Option<ItemSet> {
        let sets = [ItemSet::DragonSlayer, ItemSet::TitanMight, ItemSet::ShadowDancer, ItemSet::ArcaneScholar,
            ItemSet::ElementalMaster, ItemSet::DeathKnight, ItemSet::PaladinValor, ItemSet::PhoenixRebirth];
        for set in sets { if set.pieces().contains(&self.kind) { return Some(set); } }
        None
    }
}

// ============================================================================
// EQUIPMENT SET TRACKING
// ============================================================================

/// Tracks equipped items and calculates set bonuses
#[derive(Clone, Debug, Default)]
pub struct EquipmentSet {
    equipped: HashMap<EquipSlot, Item>,
}

impl EquipmentSet {
    pub fn new() -> Self { Self { equipped: HashMap::new() } }

    pub fn equip(&mut self, item: Item) -> Option<Item> {
        if let Some(slot) = item.kind.equip_slot() {
            self.equipped.insert(slot, item)
        } else if let Some(unique) = &item.unique {
            self.equipped.insert(unique.equip_slot(), item)
        } else { None }
    }

    pub fn unequip(&mut self, slot: EquipSlot) -> Option<Item> { self.equipped.remove(&slot) }
    pub fn get(&self, slot: EquipSlot) -> Option<&Item> { self.equipped.get(&slot) }

    pub fn total_stats(&self) -> (i32, i32, i32, i32) {
        let mut total = (0, 0, 0, 0);
        for item in self.equipped.values() {
            let (atk, def, hp, mana) = item.stats();
            total.0 += atk; total.1 += def; total.2 += hp; total.3 += mana;
        }
        for bonus in self.get_active_set_bonuses() {
            total.0 += bonus.attack; total.1 += bonus.defense; total.2 += bonus.hp; total.3 += bonus.mana;
        }
        total
    }

    pub fn get_active_set_bonuses(&self) -> Vec<SetBonus> {
        let mut bonuses = Vec::new();
        let mut set_counts: HashMap<ItemSet, u8> = HashMap::new();
        for item in self.equipped.values() {
            if let Some(set) = item.item_set() { *set_counts.entry(set).or_insert(0) += 1; }
        }
        for (set, count) in set_counts {
            if count >= 2 { bonuses.push(set.bonus_for_pieces(count)); }
        }
        bonuses
    }

    pub fn get_active_effects(&self) -> Vec<SetEffect> {
        self.get_active_set_bonuses().into_iter().filter_map(|b| b.effect).collect()
    }

    pub fn has_effect(&self, effect: SetEffect) -> bool { self.get_active_effects().contains(&effect) }

    pub fn get_unique_effects(&self) -> Vec<UniqueEffect> {
        self.equipped.values().filter_map(|item| item.unique.as_ref()).map(|u| u.special_effect()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rarity_ordering() {
        assert!(Rarity::Common < Rarity::Uncommon);
        assert!(Rarity::Uncommon < Rarity::Rare);
        assert!(Rarity::Rare < Rarity::Epic);
        assert!(Rarity::Epic < Rarity::Legendary);
        assert!(Rarity::Legendary < Rarity::Mythic);
    }

    #[test]
    fn test_item_stats_scale_with_rarity() {
        let common = Item::new(0, 0, ItemKind::LongSword, Rarity::Common);
        let legendary = Item::new(0, 0, ItemKind::LongSword, Rarity::Legendary);

        assert!(legendary.stats().0 > common.stats().0);
    }
}

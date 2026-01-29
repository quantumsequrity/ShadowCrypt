//! Professions System
//!
//! A comprehensive profession system including gathering, crafting, and service professions.
//! Players can learn multiple professions, level them up, learn recipes, and gain unique bonuses.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// Constants
// ============================================================================

/// Maximum profession skill level
pub const MAX_PROFESSION_LEVEL: u32 = 150;

/// Number of professions a player can have active at once
pub const MAX_ACTIVE_PROFESSIONS: usize = 4;

/// Base experience required per level
pub const BASE_XP_PER_LEVEL: u32 = 100;

// ============================================================================
// Profession Types and Categories
// ============================================================================

/// Main profession categories
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfessionCategory {
    Gathering,
    Crafting,
    Service,
}

impl ProfessionCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Gathering => "Gathering",
            Self::Crafting => "Crafting",
            Self::Service => "Service",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Gathering => "Professions focused on collecting raw materials from the world.",
            Self::Crafting => "Professions focused on creating items from raw materials.",
            Self::Service => "Professions that provide utility and support abilities.",
        }
    }
}

/// All available professions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Profession {
    // Gathering Professions
    Mining,
    Herbalism,
    Skinning,
    Fishing,
    Woodcutting,
    Hunting,

    // Crafting Professions
    Blacksmithing,
    Alchemy,
    Inscription,
    Tailoring,
    Jewelcrafting,
    Cooking,
    Enchanting,
    FormationCrafting,

    // Service Professions
    Merchant,
    Appraiser,
    Cartographer,
    Tamer,
}

impl Profession {
    pub fn all() -> &'static [Profession] {
        &[
            Self::Mining, Self::Herbalism, Self::Skinning, Self::Fishing,
            Self::Woodcutting, Self::Hunting, Self::Blacksmithing, Self::Alchemy,
            Self::Inscription, Self::Tailoring, Self::Jewelcrafting, Self::Cooking,
            Self::Enchanting, Self::FormationCrafting, Self::Merchant, Self::Appraiser,
            Self::Cartographer, Self::Tamer,
        ]
    }

    pub fn gathering() -> &'static [Profession] {
        &[Self::Mining, Self::Herbalism, Self::Skinning, Self::Fishing, Self::Woodcutting, Self::Hunting]
    }

    pub fn crafting() -> &'static [Profession] {
        &[Self::Blacksmithing, Self::Alchemy, Self::Inscription, Self::Tailoring,
          Self::Jewelcrafting, Self::Cooking, Self::Enchanting, Self::FormationCrafting]
    }

    pub fn service() -> &'static [Profession] {
        &[Self::Merchant, Self::Appraiser, Self::Cartographer, Self::Tamer]
    }

    pub fn category(&self) -> ProfessionCategory {
        match self {
            Self::Mining | Self::Herbalism | Self::Skinning | Self::Fishing |
            Self::Woodcutting | Self::Hunting => ProfessionCategory::Gathering,
            Self::Blacksmithing | Self::Alchemy | Self::Inscription | Self::Tailoring |
            Self::Jewelcrafting | Self::Cooking | Self::Enchanting |
            Self::FormationCrafting => ProfessionCategory::Crafting,
            Self::Merchant | Self::Appraiser | Self::Cartographer | Self::Tamer => ProfessionCategory::Service,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Mining => "Mining",
            Self::Herbalism => "Herbalism",
            Self::Skinning => "Skinning",
            Self::Fishing => "Fishing",
            Self::Woodcutting => "Woodcutting",
            Self::Hunting => "Hunting",
            Self::Blacksmithing => "Blacksmithing",
            Self::Alchemy => "Alchemy",
            Self::Inscription => "Inscription",
            Self::Tailoring => "Tailoring",
            Self::Jewelcrafting => "Jewelcrafting",
            Self::Cooking => "Cooking",
            Self::Enchanting => "Enchanting",
            Self::FormationCrafting => "Formation Crafting",
            Self::Merchant => "Merchant",
            Self::Appraiser => "Appraiser",
            Self::Cartographer => "Cartographer",
            Self::Tamer => "Tamer",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Mining => "Extract ores, gems, and spirit stones from mineral deposits.",
            Self::Herbalism => "Gather herbs, flowers, and magical roots from the wild.",
            Self::Skinning => "Harvest hides, scales, and feathers from defeated creatures.",
            Self::Fishing => "Catch fish, find pearls, and discover underwater treasures.",
            Self::Woodcutting => "Harvest wood, collect sap, and find spirit trees.",
            Self::Hunting => "Track and harvest meat, bones, and monster parts from prey.",
            Self::Blacksmithing => "Forge powerful weapons and sturdy armor from metals.",
            Self::Alchemy => "Brew pills and potions with magical properties.",
            Self::Inscription => "Create scrolls and talismans imbued with power.",
            Self::Tailoring => "Craft magical robes and cloth armor for spellcasters.",
            Self::Jewelcrafting => "Create rings and amulets with mystical properties.",
            Self::Cooking => "Prepare food that provides powerful stat buffs.",
            Self::Enchanting => "Add magical enchantments to weapons and armor.",
            Self::FormationCrafting => "Create formation discs for defensive and offensive arrays.",
            Self::Merchant => "Gain trading bonuses and access to rare goods.",
            Self::Appraiser => "Identify items and reveal their hidden properties.",
            Self::Cartographer => "Gain mapping bonuses and reveal hidden areas.",
            Self::Tamer => "Improved beast taming and companion bonuses.",
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            Self::Mining => '^',
            Self::Herbalism => '"',
            Self::Skinning => '%',
            Self::Fishing => '~',
            Self::Woodcutting => 'T',
            Self::Hunting => 'H',
            Self::Blacksmithing => '&',
            Self::Alchemy => 'A',
            Self::Inscription => 'I',
            Self::Tailoring => '#',
            Self::Jewelcrafting => 'o',
            Self::Cooking => 'C',
            Self::Enchanting => 'E',
            Self::FormationCrafting => 'F',
            Self::Merchant => '$',
            Self::Appraiser => '?',
            Self::Cartographer => 'M',
            Self::Tamer => 'B',
        }
    }

    pub fn synergy_professions(&self) -> Vec<Profession> {
        match self {
            Self::Mining => vec![Self::Blacksmithing, Self::Jewelcrafting],
            Self::Herbalism => vec![Self::Alchemy, Self::Cooking],
            Self::Skinning => vec![Self::Tailoring, Self::Hunting],
            Self::Fishing => vec![Self::Cooking, Self::Alchemy],
            Self::Woodcutting => vec![Self::Blacksmithing, Self::FormationCrafting],
            Self::Hunting => vec![Self::Skinning, Self::Cooking, Self::Tamer],
            Self::Blacksmithing => vec![Self::Mining, Self::Enchanting],
            Self::Alchemy => vec![Self::Herbalism, Self::Cooking],
            Self::Inscription => vec![Self::Enchanting, Self::FormationCrafting],
            Self::Tailoring => vec![Self::Skinning, Self::Enchanting],
            Self::Jewelcrafting => vec![Self::Mining, Self::Enchanting],
            Self::Cooking => vec![Self::Hunting, Self::Fishing, Self::Herbalism],
            Self::Enchanting => vec![Self::Inscription, Self::Jewelcrafting],
            Self::FormationCrafting => vec![Self::Inscription, Self::Enchanting],
            Self::Merchant => vec![Self::Appraiser],
            Self::Appraiser => vec![Self::Merchant, Self::Jewelcrafting],
            Self::Cartographer => vec![Self::Hunting],
            Self::Tamer => vec![Self::Hunting, Self::Cooking],
        }
    }

    pub fn primary_stat(&self) -> &'static str {
        match self {
            Self::Mining | Self::Woodcutting | Self::Blacksmithing => "Strength",
            Self::Herbalism | Self::Alchemy | Self::Inscription |
            Self::Enchanting | Self::FormationCrafting => "Intelligence",
            Self::Skinning | Self::Hunting | Self::Tailoring | Self::Jewelcrafting => "Dexterity",
            Self::Fishing => "Luck",
            Self::Cooking => "Constitution",
            Self::Merchant | Self::Tamer => "Charisma",
            Self::Appraiser => "Perception",
            Self::Cartographer => "Wisdom",
        }
    }
}

// ============================================================================
// Profession Ranks
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ProfessionRank {
    Novice,       // 1-20
    Apprentice,   // 21-40
    Journeyman,   // 41-60
    Expert,       // 61-80
    Master,       // 81-100
    Grandmaster,  // 101-120
    Legend,       // 121-150
}

impl ProfessionRank {
    pub fn all() -> &'static [ProfessionRank] {
        &[Self::Novice, Self::Apprentice, Self::Journeyman, Self::Expert,
          Self::Master, Self::Grandmaster, Self::Legend]
    }

    pub fn from_level(level: u32) -> Self {
        match level {
            1..=20 => Self::Novice,
            21..=40 => Self::Apprentice,
            41..=60 => Self::Journeyman,
            61..=80 => Self::Expert,
            81..=100 => Self::Master,
            101..=120 => Self::Grandmaster,
            _ => Self::Legend,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Novice => "Novice",
            Self::Apprentice => "Apprentice",
            Self::Journeyman => "Journeyman",
            Self::Expert => "Expert",
            Self::Master => "Master",
            Self::Grandmaster => "Grandmaster",
            Self::Legend => "Legend",
        }
    }

    pub fn min_level(&self) -> u32 {
        match self {
            Self::Novice => 1,
            Self::Apprentice => 21,
            Self::Journeyman => 41,
            Self::Expert => 61,
            Self::Master => 81,
            Self::Grandmaster => 101,
            Self::Legend => 121,
        }
    }

    pub fn max_level(&self) -> u32 {
        match self {
            Self::Novice => 20,
            Self::Apprentice => 40,
            Self::Journeyman => 60,
            Self::Expert => 80,
            Self::Master => 100,
            Self::Grandmaster => 120,
            Self::Legend => 150,
        }
    }

    pub fn next_rank(&self) -> Option<Self> {
        match self {
            Self::Novice => Some(Self::Apprentice),
            Self::Apprentice => Some(Self::Journeyman),
            Self::Journeyman => Some(Self::Expert),
            Self::Expert => Some(Self::Master),
            Self::Master => Some(Self::Grandmaster),
            Self::Grandmaster => Some(Self::Legend),
            Self::Legend => None,
        }
    }

    pub fn quality_multiplier(&self) -> f32 {
        match self {
            Self::Novice => 1.0,
            Self::Apprentice => 1.1,
            Self::Journeyman => 1.25,
            Self::Expert => 1.4,
            Self::Master => 1.6,
            Self::Grandmaster => 1.85,
            Self::Legend => 2.0,
        }
    }

    pub fn yield_bonus(&self) -> f32 {
        match self {
            Self::Novice => 0.0,
            Self::Apprentice => 0.1,
            Self::Journeyman => 0.2,
            Self::Expert => 0.35,
            Self::Master => 0.5,
            Self::Grandmaster => 0.75,
            Self::Legend => 1.0,
        }
    }

    pub fn rare_find_bonus(&self) -> f32 {
        match self {
            Self::Novice => 0.0,
            Self::Apprentice => 0.02,
            Self::Journeyman => 0.05,
            Self::Expert => 0.08,
            Self::Master => 0.12,
            Self::Grandmaster => 0.18,
            Self::Legend => 0.25,
        }
    }
}

// ============================================================================
// Gathered Materials
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GatheredMaterial {
    // Mining: ores, gems, spirit stones
    CopperOre, IronOre, SilverOre, GoldOre, MithrilOre, AdamantiteOre, OrihalconOre,
    RoughGem, PureGem, FlawlessGem, SpiritStone, ElementalCrystal, DragoniteOre,

    // Herbalism: herbs, flowers, roots
    CommonHerb, HealingHerb, ManaBloom, Nightshade, FirePetal, FrostLeaf,
    ThunderRoot, MoonFlower, SunBlossom, VoidRoot, LifeFlower, DeathCap, SpiritMoss, DragonBreath,

    // Skinning: hides, scales, feathers
    RawHide, ToughLeather, ExoticLeather, DragonHide, DemonSkin, CelestialHide,
    BasicScales, DragonScales, PhoenixFeather, GriffinFeather, ThunderbirdFeather, ShadowFeather,

    // Fishing: fish, pearls, treasures
    CommonFish, RareFish, GoldenFish, SpiritFish, TreasureFish, Seaweed,
    Pearl, BlackPearl, DragonPearl, SunkenTreasure, AncientRelic, SeaSerpentScale,

    // Woodcutting: wood, sap, spirit trees
    CommonWood, HardWood, ElvenWood, DarkWood, IronWood, SpiritWood, AncientWood,
    WorldTreeBranch, TreeSap, GoldenSap, SpiritSap, Amber,

    // Hunting: meat, bones, monster parts
    RawMeat, QualityMeat, ExoticMeat, DragonMeat, BeastBone, MonsterBone, DragonBone,
    MonsterFang, BeastClaw, MonsterHeart, MonsterEye, VenomSac, ElementalCore, BeastSoul,
}

impl GatheredMaterial {
    pub fn profession(&self) -> Profession {
        match self {
            Self::CopperOre | Self::IronOre | Self::SilverOre | Self::GoldOre |
            Self::MithrilOre | Self::AdamantiteOre | Self::OrihalconOre | Self::RoughGem |
            Self::PureGem | Self::FlawlessGem | Self::SpiritStone | Self::ElementalCrystal |
            Self::DragoniteOre => Profession::Mining,

            Self::CommonHerb | Self::HealingHerb | Self::ManaBloom | Self::Nightshade |
            Self::FirePetal | Self::FrostLeaf | Self::ThunderRoot | Self::MoonFlower |
            Self::SunBlossom | Self::VoidRoot | Self::LifeFlower | Self::DeathCap |
            Self::SpiritMoss | Self::DragonBreath => Profession::Herbalism,

            Self::RawHide | Self::ToughLeather | Self::ExoticLeather | Self::DragonHide |
            Self::DemonSkin | Self::CelestialHide | Self::BasicScales | Self::DragonScales |
            Self::PhoenixFeather | Self::GriffinFeather | Self::ThunderbirdFeather |
            Self::ShadowFeather => Profession::Skinning,

            Self::CommonFish | Self::RareFish | Self::GoldenFish | Self::SpiritFish |
            Self::TreasureFish | Self::Seaweed | Self::Pearl | Self::BlackPearl |
            Self::DragonPearl | Self::SunkenTreasure | Self::AncientRelic |
            Self::SeaSerpentScale => Profession::Fishing,

            Self::CommonWood | Self::HardWood | Self::ElvenWood | Self::DarkWood |
            Self::IronWood | Self::SpiritWood | Self::AncientWood | Self::WorldTreeBranch |
            Self::TreeSap | Self::GoldenSap | Self::SpiritSap | Self::Amber => Profession::Woodcutting,

            Self::RawMeat | Self::QualityMeat | Self::ExoticMeat | Self::DragonMeat |
            Self::BeastBone | Self::MonsterBone | Self::DragonBone | Self::MonsterFang |
            Self::BeastClaw | Self::MonsterHeart | Self::MonsterEye | Self::VenomSac |
            Self::ElementalCore | Self::BeastSoul => Profession::Hunting,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::CopperOre => "Copper Ore", Self::IronOre => "Iron Ore",
            Self::SilverOre => "Silver Ore", Self::GoldOre => "Gold Ore",
            Self::MithrilOre => "Mithril Ore", Self::AdamantiteOre => "Adamantite Ore",
            Self::OrihalconOre => "Orihalcon Ore", Self::RoughGem => "Rough Gem",
            Self::PureGem => "Pure Gem", Self::FlawlessGem => "Flawless Gem",
            Self::SpiritStone => "Spirit Stone", Self::ElementalCrystal => "Elemental Crystal",
            Self::DragoniteOre => "Dragonite Ore", Self::CommonHerb => "Common Herb",
            Self::HealingHerb => "Healing Herb", Self::ManaBloom => "Mana Bloom",
            Self::Nightshade => "Nightshade", Self::FirePetal => "Fire Petal",
            Self::FrostLeaf => "Frost Leaf", Self::ThunderRoot => "Thunder Root",
            Self::MoonFlower => "Moon Flower", Self::SunBlossom => "Sun Blossom",
            Self::VoidRoot => "Void Root", Self::LifeFlower => "Life Flower",
            Self::DeathCap => "Death Cap", Self::SpiritMoss => "Spirit Moss",
            Self::DragonBreath => "Dragon Breath Herb", Self::RawHide => "Raw Hide",
            Self::ToughLeather => "Tough Leather", Self::ExoticLeather => "Exotic Leather",
            Self::DragonHide => "Dragon Hide", Self::DemonSkin => "Demon Skin",
            Self::CelestialHide => "Celestial Hide", Self::BasicScales => "Basic Scales",
            Self::DragonScales => "Dragon Scales", Self::PhoenixFeather => "Phoenix Feather",
            Self::GriffinFeather => "Griffin Feather", Self::ThunderbirdFeather => "Thunderbird Feather",
            Self::ShadowFeather => "Shadow Feather", Self::CommonFish => "Common Fish",
            Self::RareFish => "Rare Fish", Self::GoldenFish => "Golden Fish",
            Self::SpiritFish => "Spirit Fish", Self::TreasureFish => "Treasure Fish",
            Self::Seaweed => "Seaweed", Self::Pearl => "Pearl",
            Self::BlackPearl => "Black Pearl", Self::DragonPearl => "Dragon Pearl",
            Self::SunkenTreasure => "Sunken Treasure", Self::AncientRelic => "Ancient Relic",
            Self::SeaSerpentScale => "Sea Serpent Scale", Self::CommonWood => "Common Wood",
            Self::HardWood => "Hard Wood", Self::ElvenWood => "Elven Wood",
            Self::DarkWood => "Dark Wood", Self::IronWood => "Iron Wood",
            Self::SpiritWood => "Spirit Wood", Self::AncientWood => "Ancient Wood",
            Self::WorldTreeBranch => "World Tree Branch", Self::TreeSap => "Tree Sap",
            Self::GoldenSap => "Golden Sap", Self::SpiritSap => "Spirit Sap",
            Self::Amber => "Amber", Self::RawMeat => "Raw Meat",
            Self::QualityMeat => "Quality Meat", Self::ExoticMeat => "Exotic Meat",
            Self::DragonMeat => "Dragon Meat", Self::BeastBone => "Beast Bone",
            Self::MonsterBone => "Monster Bone", Self::DragonBone => "Dragon Bone",
            Self::MonsterFang => "Monster Fang", Self::BeastClaw => "Beast Claw",
            Self::MonsterHeart => "Monster Heart", Self::MonsterEye => "Monster Eye",
            Self::VenomSac => "Venom Sac", Self::ElementalCore => "Elemental Core",
            Self::BeastSoul => "Beast Soul",
        }
    }

    pub fn rarity(&self) -> MaterialRarity {
        match self {
            Self::CopperOre | Self::CommonHerb | Self::HealingHerb | Self::RawHide |
            Self::CommonFish | Self::Seaweed | Self::CommonWood | Self::TreeSap |
            Self::RawMeat | Self::BeastBone => MaterialRarity::Common,

            Self::IronOre | Self::ManaBloom | Self::ToughLeather | Self::BasicScales |
            Self::RareFish | Self::HardWood | Self::QualityMeat | Self::MonsterBone |
            Self::MonsterFang => MaterialRarity::Uncommon,

            Self::SilverOre | Self::GoldOre | Self::RoughGem | Self::Nightshade |
            Self::FirePetal | Self::FrostLeaf | Self::ExoticLeather | Self::GoldenFish |
            Self::Pearl | Self::ElvenWood | Self::DarkWood | Self::ExoticMeat |
            Self::BeastClaw | Self::VenomSac => MaterialRarity::Rare,

            Self::MithrilOre | Self::PureGem | Self::ThunderRoot | Self::MoonFlower |
            Self::SunBlossom | Self::DragonHide | Self::GriffinFeather | Self::SpiritFish |
            Self::BlackPearl | Self::IronWood | Self::SpiritWood | Self::GoldenSap |
            Self::Amber | Self::MonsterHeart | Self::MonsterEye | Self::ElementalCore => MaterialRarity::Epic,

            Self::AdamantiteOre | Self::FlawlessGem | Self::SpiritStone | Self::VoidRoot |
            Self::LifeFlower | Self::DeathCap | Self::SpiritMoss | Self::DemonSkin |
            Self::DragonScales | Self::PhoenixFeather | Self::ThunderbirdFeather |
            Self::TreasureFish | Self::DragonPearl | Self::SunkenTreasure | Self::AncientWood |
            Self::SpiritSap | Self::DragonMeat | Self::DragonBone | Self::BeastSoul => MaterialRarity::Legendary,

            Self::OrihalconOre | Self::ElementalCrystal | Self::DragoniteOre | Self::DragonBreath |
            Self::CelestialHide | Self::ShadowFeather | Self::AncientRelic |
            Self::SeaSerpentScale | Self::WorldTreeBranch => MaterialRarity::Mythic,
        }
    }

    pub fn required_level(&self) -> u32 {
        match self.rarity() {
            MaterialRarity::Common => 1,
            MaterialRarity::Uncommon => 15,
            MaterialRarity::Rare => 35,
            MaterialRarity::Epic => 55,
            MaterialRarity::Legendary => 85,
            MaterialRarity::Mythic => 115,
        }
    }

    pub fn base_value(&self) -> u32 {
        match self.rarity() {
            MaterialRarity::Common => 5,
            MaterialRarity::Uncommon => 15,
            MaterialRarity::Rare => 50,
            MaterialRarity::Epic => 150,
            MaterialRarity::Legendary => 500,
            MaterialRarity::Mythic => 2000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MaterialRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl MaterialRarity {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common", Self::Uncommon => "Uncommon",
            Self::Rare => "Rare", Self::Epic => "Epic",
            Self::Legendary => "Legendary", Self::Mythic => "Mythic",
        }
    }
}

// ============================================================================
// Recipes and Patterns
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipeSource {
    Trainer,
    Book,
    Experimentation,
    MasterTaught,
    RareDrop,
    RankUnlock,
    QuestReward,
    SecretDiscovery,
}

impl RecipeSource {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Trainer => "Trainer", Self::Book => "Recipe Book",
            Self::Experimentation => "Experimentation", Self::MasterTaught => "Master Taught",
            Self::RareDrop => "Rare Drop", Self::RankUnlock => "Rank Unlock",
            Self::QuestReward => "Quest Reward", Self::SecretDiscovery => "Secret Discovery",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub profession: Profession,
    pub required_level: u32,
    pub ingredients: Vec<(GatheredMaterial, u32)>,
    pub optional_ingredients: Vec<(GatheredMaterial, u32, String)>,
    pub result_item: String,
    pub result_quantity: u32,
    pub experience_gained: u32,
    pub crafting_time: u32,
    pub source: RecipeSource,
    pub rarity: RecipeRarity,
    pub requires_tools: Option<String>,
    pub quality_chance: f32,
}

impl Recipe {
    pub fn new(
        id: impl Into<String>, name: impl Into<String>, description: impl Into<String>,
        profession: Profession, required_level: u32, ingredients: Vec<(GatheredMaterial, u32)>,
        result_item: impl Into<String>, result_quantity: u32, experience_gained: u32,
        source: RecipeSource, rarity: RecipeRarity,
    ) -> Self {
        Self {
            id: id.into(), name: name.into(), description: description.into(),
            profession, required_level, ingredients,
            optional_ingredients: Vec::new(), result_item: result_item.into(),
            result_quantity, experience_gained, crafting_time: 1,
            source, rarity, requires_tools: None, quality_chance: 0.1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RecipeRarity {
    Common, Uncommon, Rare, Epic, Legendary, Secret,
}

impl RecipeRarity {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Common => "Common", Self::Uncommon => "Uncommon",
            Self::Rare => "Rare", Self::Epic => "Epic",
            Self::Legendary => "Legendary", Self::Secret => "Secret",
        }
    }

    pub fn drop_chance(&self) -> f32 {
        match self {
            Self::Common => 0.3, Self::Uncommon => 0.15, Self::Rare => 0.05,
            Self::Epic => 0.02, Self::Legendary => 0.005, Self::Secret => 0.001,
        }
    }
}

// ============================================================================
// Profession Bonuses
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProfessionBonus {
    StatBonus { stat: String, amount: i32 },
    StatPercentBonus { stat: String, percent: f32 },
    GatheringYield { percent: f32 },
    RareFindChance { percent: f32 },
    MaterialSaving { percent: f32 },
    CraftingSpeed { percent: f32 },
    QualityBonus { percent: f32 },
    AbilityUnlock { ability: String, description: String },
    MaterialAccess { material: String },
    TradingBonus { buy_discount: f32, sell_bonus: f32 },
    IdentifyBonus { chance: f32, reveal_hidden: bool },
    MapBonus { reveal_percent: f32, show_resources: bool },
    TamingBonus { success_bonus: f32, loyalty_bonus: f32 },
    ExperienceBonus { percent: f32 },
    RecipeUnlock { recipe_id: String },
}

impl ProfessionBonus {
    pub fn description(&self) -> String {
        match self {
            Self::StatBonus { stat, amount } => format!("+{} {}", amount, stat),
            Self::StatPercentBonus { stat, percent } => format!("+{}% {}", (percent * 100.0) as i32, stat),
            Self::GatheringYield { percent } => format!("+{}% gathering yield", (percent * 100.0) as i32),
            Self::RareFindChance { percent } => format!("+{}% rare find chance", (percent * 100.0) as i32),
            Self::MaterialSaving { percent } => format!("{}% chance to save materials", (percent * 100.0) as i32),
            Self::CraftingSpeed { percent } => format!("+{}% crafting speed", (percent * 100.0) as i32),
            Self::QualityBonus { percent } => format!("+{}% quality chance", (percent * 100.0) as i32),
            Self::AbilityUnlock { ability, .. } => format!("Unlock: {}", ability),
            Self::MaterialAccess { material } => format!("Access to {}", material),
            Self::TradingBonus { buy_discount, sell_bonus } =>
                format!("-{}% buy, +{}% sell", (buy_discount * 100.0) as i32, (sell_bonus * 100.0) as i32),
            Self::IdentifyBonus { chance, .. } => format!("+{}% identify chance", (chance * 100.0) as i32),
            Self::MapBonus { reveal_percent, .. } => format!("+{}% map reveal", (reveal_percent * 100.0) as i32),
            Self::TamingBonus { success_bonus, .. } => format!("+{}% taming success", (success_bonus * 100.0) as i32),
            Self::ExperienceBonus { percent } => format!("+{}% profession XP", (percent * 100.0) as i32),
            Self::RecipeUnlock { recipe_id } => format!("Recipe: {}", recipe_id),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankBonus {
    pub profession: Profession,
    pub rank: ProfessionRank,
    pub bonuses: Vec<ProfessionBonus>,
}

impl RankBonus {
    pub fn new(profession: Profession, rank: ProfessionRank, bonuses: Vec<ProfessionBonus>) -> Self {
        Self { profession, rank, bonuses }
    }
}

// ============================================================================
// Player Profession Data
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfessionProgress {
    pub profession: Profession,
    pub level: u32,
    pub experience: u32,
    pub experience_to_next: u32,
    pub total_experience: u32,
    pub items_crafted: u32,
    pub items_gathered: u32,
    pub rare_finds: u32,
    pub perfect_crafts: u32,
    pub learned_recipes: HashSet<String>,
    pub discovered_recipes: HashSet<String>,
    pub active_bonuses: Vec<ProfessionBonus>,
    pub time_spent: u32,
}

impl ProfessionProgress {
    pub fn new(profession: Profession) -> Self {
        Self {
            profession, level: 1, experience: 0,
            experience_to_next: calculate_exp_to_level(1),
            total_experience: 0, items_crafted: 0, items_gathered: 0,
            rare_finds: 0, perfect_crafts: 0,
            learned_recipes: HashSet::new(), discovered_recipes: HashSet::new(),
            active_bonuses: Vec::new(), time_spent: 0,
        }
    }

    pub fn rank(&self) -> ProfessionRank {
        ProfessionRank::from_level(self.level)
    }

    pub fn add_experience(&mut self, amount: u32) -> bool {
        self.experience += amount;
        self.total_experience += amount;
        let mut leveled = false;
        while self.experience >= self.experience_to_next && self.level < MAX_PROFESSION_LEVEL {
            self.experience -= self.experience_to_next;
            self.level += 1;
            self.experience_to_next = calculate_exp_to_level(self.level);
            leveled = true;
        }
        if self.level >= MAX_PROFESSION_LEVEL {
            self.experience = 0;
            self.experience_to_next = 0;
        }
        leveled
    }

    pub fn learn_recipe(&mut self, recipe_id: &str) -> bool {
        if self.learned_recipes.contains(recipe_id) { return false; }
        self.learned_recipes.insert(recipe_id.to_string());
        true
    }

    pub fn discover_recipe(&mut self, recipe_id: &str) -> bool {
        if self.discovered_recipes.contains(recipe_id) || self.learned_recipes.contains(recipe_id) {
            return false;
        }
        self.discovered_recipes.insert(recipe_id.to_string());
        self.learned_recipes.insert(recipe_id.to_string());
        true
    }

    pub fn known_recipes(&self) -> impl Iterator<Item = &String> {
        self.learned_recipes.iter()
    }

    pub fn effective_level(&self, synergy_bonus: u32) -> u32 {
        (self.level + synergy_bonus).min(MAX_PROFESSION_LEVEL)
    }

    pub fn record_gathering(&mut self, is_rare: bool) {
        self.items_gathered += 1;
        if is_rare { self.rare_finds += 1; }
    }

    pub fn record_crafting(&mut self, is_perfect: bool) {
        self.items_crafted += 1;
        if is_perfect { self.perfect_crafts += 1; }
    }
}

fn calculate_exp_to_level(current_level: u32) -> u32 {
    let rank = ProfessionRank::from_level(current_level);
    let rank_multiplier = match rank {
        ProfessionRank::Novice => 1.0,
        ProfessionRank::Apprentice => 1.5,
        ProfessionRank::Journeyman => 2.0,
        ProfessionRank::Expert => 3.0,
        ProfessionRank::Master => 4.0,
        ProfessionRank::Grandmaster => 6.0,
        ProfessionRank::Legend => 10.0,
    };
    (BASE_XP_PER_LEVEL as f32 * rank_multiplier * (1.0 + (current_level as f32 * 0.05))) as u32
}

// ============================================================================
// Gathering System
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceNode {
    pub x: usize,
    pub y: usize,
    pub material: GatheredMaterial,
    pub quantity: u32,
    pub max_quantity: u32,
    pub respawn_time: u32,
    pub respawn_timer: u32,
    pub depleted: bool,
    pub hidden: bool,
    pub required_level: u32,
}

impl ResourceNode {
    pub fn new(x: usize, y: usize, material: GatheredMaterial, quantity: u32) -> Self {
        Self {
            x, y, material, quantity, max_quantity: quantity,
            respawn_time: 50 + (material.rarity() as u32 * 20),
            respawn_timer: 0, depleted: false,
            hidden: material.rarity() >= MaterialRarity::Epic,
            required_level: material.required_level(),
        }
    }

    pub fn gather(&mut self, skill_level: u32, yield_bonus: f32) -> Option<GatherResult> {
        if self.depleted || skill_level < self.required_level { return None; }
        let base_amount = 1;
        let bonus_amount = (base_amount as f32 * yield_bonus) as u32;
        let total_amount = (base_amount + bonus_amount).min(self.quantity);
        self.quantity -= total_amount;
        if self.quantity == 0 {
            self.depleted = true;
            self.respawn_timer = self.respawn_time;
        }
        let base_exp = 10 + (self.material.rarity() as u32 * 5);
        let level_diff = skill_level.saturating_sub(self.required_level);
        let exp_penalty = (level_diff as f32 * 0.05).min(0.5);
        let final_exp = (base_exp as f32 * (1.0 - exp_penalty)) as u32;
        Some(GatherResult { material: self.material, quantity: total_amount, experience: final_exp, rare_bonus: false })
    }

    pub fn tick(&mut self) {
        if self.depleted && self.respawn_timer > 0 {
            self.respawn_timer -= 1;
            if self.respawn_timer == 0 {
                self.depleted = false;
                self.quantity = self.max_quantity;
            }
        }
    }

    pub fn glyph(&self) -> char {
        if self.depleted { '.' } else { self.material.profession().glyph() }
    }
}

#[derive(Clone, Debug)]
pub struct GatherResult {
    pub material: GatheredMaterial,
    pub quantity: u32,
    pub experience: u32,
    pub rare_bonus: bool,
}

// ============================================================================
// Crafting System
// ============================================================================

#[derive(Clone, Debug)]
pub enum CraftResult {
    Success { item_id: String, quantity: u32, quality: CraftQuality, experience: u32, materials_saved: bool },
    InsufficientMaterials { missing: Vec<(GatheredMaterial, u32)> },
    InsufficientLevel { required: u32, current: u32 },
    RecipeNotKnown,
    MissingTools { tool: String },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CraftQuality {
    Poor, Normal, Good, Excellent, Perfect, Masterwork,
}

impl CraftQuality {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Poor => "Poor", Self::Normal => "Normal", Self::Good => "Good",
            Self::Excellent => "Excellent", Self::Perfect => "Perfect", Self::Masterwork => "Masterwork",
        }
    }

    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Poor => 0.75, Self::Normal => 1.0, Self::Good => 1.15,
            Self::Excellent => 1.3, Self::Perfect => 1.5, Self::Masterwork => 2.0,
        }
    }

    pub fn value_multiplier(&self) -> f32 {
        match self {
            Self::Poor => 0.5, Self::Normal => 1.0, Self::Good => 1.5,
            Self::Excellent => 2.5, Self::Perfect => 4.0, Self::Masterwork => 10.0,
        }
    }
}

// ============================================================================
// Profession System
// ============================================================================

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProfessionSystem {
    pub professions: HashMap<Profession, ProfessionProgress>,
    pub active_professions: Vec<Profession>,
    pub all_recipes: HashMap<String, Recipe>,
    pub rank_bonuses: Vec<RankBonus>,
    pub materials: HashMap<GatheredMaterial, u32>,
    pub total_crafted: u32,
    pub total_gathered: u32,
    pub maxed_professions: u32,
}

impl ProfessionSystem {
    pub fn new() -> Self {
        let mut system = Self {
            professions: HashMap::new(), active_professions: Vec::new(),
            all_recipes: HashMap::new(), rank_bonuses: Vec::new(),
            materials: HashMap::new(), total_crafted: 0, total_gathered: 0, maxed_professions: 0,
        };
        system.initialize_recipes();
        system.initialize_rank_bonuses();
        system
    }

    pub fn learn_profession(&mut self, profession: Profession) -> bool {
        if self.professions.contains_key(&profession) { return false; }
        self.professions.insert(profession, ProfessionProgress::new(profession));
        if self.active_professions.len() < MAX_ACTIVE_PROFESSIONS {
            self.active_professions.push(profession);
        }
        true
    }

    pub fn set_active(&mut self, profession: Profession) -> bool {
        if !self.professions.contains_key(&profession) { return false; }
        if self.active_professions.contains(&profession) { return true; }
        if self.active_professions.len() >= MAX_ACTIVE_PROFESSIONS { return false; }
        self.active_professions.push(profession);
        true
    }

    pub fn deactivate(&mut self, profession: Profession) {
        self.active_professions.retain(|&p| p != profession);
    }

    pub fn get_progress(&self, profession: Profession) -> Option<&ProfessionProgress> {
        self.professions.get(&profession)
    }

    pub fn get_progress_mut(&mut self, profession: Profession) -> Option<&mut ProfessionProgress> {
        self.professions.get_mut(&profession)
    }

    pub fn synergy_bonus(&self, profession: Profession) -> u32 {
        let synergies = profession.synergy_professions();
        let mut bonus = 0u32;
        for synergy in synergies {
            if let Some(progress) = self.professions.get(&synergy) {
                bonus += progress.level / 10;
            }
        }
        bonus.min(20)
    }

    pub fn add_material(&mut self, material: GatheredMaterial, quantity: u32) {
        *self.materials.entry(material).or_insert(0) += quantity;
    }

    pub fn remove_material(&mut self, material: GatheredMaterial, quantity: u32) -> bool {
        if let Some(count) = self.materials.get_mut(&material) {
            if *count >= quantity {
                *count -= quantity;
                if *count == 0 { self.materials.remove(&material); }
                return true;
            }
        }
        false
    }

    pub fn material_count(&self, material: GatheredMaterial) -> u32 {
        self.materials.get(&material).copied().unwrap_or(0)
    }

    pub fn gather(&mut self, node: &mut ResourceNode) -> Option<GatherResult> {
        let profession = node.material.profession();
        let progress = self.professions.get_mut(&profession)?;
        let yield_bonus = progress.rank().yield_bonus();
        let rare_bonus = progress.rank().rare_find_bonus();
        let mut result = node.gather(progress.level, yield_bonus)?;
        if rand::random::<f32>() < rare_bonus {
            result.quantity += 1;
            result.rare_bonus = true;
        }
        progress.record_gathering(result.rare_bonus);
        progress.add_experience(result.experience);
        progress.time_spent += 1;
        self.add_material(result.material, result.quantity);
        self.total_gathered += result.quantity;
        Some(result)
    }

    pub fn craft(&mut self, recipe_id: &str) -> CraftResult {
        let recipe = match self.all_recipes.get(recipe_id) {
            Some(r) => r.clone(),
            None => return CraftResult::RecipeNotKnown,
        };
        let progress = match self.professions.get(&recipe.profession) {
            Some(p) => p,
            None => return CraftResult::InsufficientLevel { required: recipe.required_level, current: 0 },
        };
        if progress.level < recipe.required_level {
            return CraftResult::InsufficientLevel { required: recipe.required_level, current: progress.level };
        }
        if !progress.learned_recipes.contains(&recipe.id) {
            return CraftResult::RecipeNotKnown;
        }
        let mut missing = Vec::new();
        for (material, required) in &recipe.ingredients {
            let have = self.material_count(*material);
            if have < *required { missing.push((*material, *required - have)); }
        }
        if !missing.is_empty() { return CraftResult::InsufficientMaterials { missing }; }

        let rank = progress.rank();
        let quality_mult = rank.quality_multiplier();
        let quality_roll = rand::random::<f32>();
        let adjusted_roll = quality_roll * quality_mult;
        let quality = if adjusted_roll > 0.98 { CraftQuality::Masterwork }
            else if adjusted_roll > 0.90 { CraftQuality::Perfect }
            else if adjusted_roll > 0.75 { CraftQuality::Excellent }
            else if adjusted_roll > 0.50 { CraftQuality::Good }
            else if adjusted_roll > 0.20 { CraftQuality::Normal }
            else { CraftQuality::Poor };

        let save_chance = match rank {
            ProfessionRank::Expert => 0.05, ProfessionRank::Master => 0.10,
            ProfessionRank::Grandmaster => 0.15, ProfessionRank::Legend => 0.25, _ => 0.0,
        };
        let materials_saved = rand::random::<f32>() < save_chance;
        if !materials_saved {
            for (material, required) in &recipe.ingredients {
                self.remove_material(*material, *required);
            }
        }
        let base_exp = recipe.experience_gained;
        let quality_bonus = match quality {
            CraftQuality::Poor => 0.5, CraftQuality::Normal => 1.0, CraftQuality::Good => 1.2,
            CraftQuality::Excellent => 1.5, CraftQuality::Perfect => 2.0, CraftQuality::Masterwork => 3.0,
        };
        let final_exp = (base_exp as f32 * quality_bonus) as u32;
        if let Some(progress) = self.professions.get_mut(&recipe.profession) {
            progress.record_crafting(quality >= CraftQuality::Perfect);
            progress.add_experience(final_exp);
            progress.time_spent += recipe.crafting_time;
        }
        self.total_crafted += recipe.result_quantity;
        CraftResult::Success { item_id: recipe.result_item.clone(), quantity: recipe.result_quantity, quality, experience: final_exp, materials_saved }
    }

    pub fn get_rank_bonuses(&self, profession: Profession) -> Vec<&ProfessionBonus> {
        let progress = match self.professions.get(&profession) { Some(p) => p, None => return Vec::new() };
        let current_rank = progress.rank();
        self.rank_bonuses.iter()
            .filter(|rb| rb.profession == profession && rb.rank <= current_rank)
            .flat_map(|rb| rb.bonuses.iter()).collect()
    }

    pub fn all_active_bonuses(&self) -> Vec<(&Profession, &ProfessionBonus)> {
        let mut bonuses = Vec::new();
        for profession in &self.active_professions {
            for bonus in self.get_rank_bonuses(*profession) {
                bonuses.push((profession, bonus));
            }
        }
        bonuses
    }

    pub fn calculate_stat_bonuses(&self) -> HashMap<String, i32> {
        let mut stats: HashMap<String, i32> = HashMap::new();
        for (_, bonus) in self.all_active_bonuses() {
            if let ProfessionBonus::StatBonus { stat, amount } = bonus {
                *stats.entry(stat.clone()).or_insert(0) += amount;
            }
        }
        stats
    }

    pub fn available_recipes(&self, profession: Profession) -> Vec<&Recipe> {
        let progress = match self.professions.get(&profession) { Some(p) => p, None => return Vec::new() };
        self.all_recipes.values()
            .filter(|r| r.profession == profession && r.required_level <= progress.level && progress.learned_recipes.contains(&r.id))
            .collect()
    }

    pub fn craftable_recipes(&self, profession: Profession) -> Vec<&Recipe> {
        self.available_recipes(profession).into_iter()
            .filter(|r| r.ingredients.iter().all(|(mat, qty)| self.material_count(*mat) >= *qty))
            .collect()
    }

    pub fn learn_recipe(&mut self, profession: Profession, recipe_id: &str) -> bool {
        if let Some(recipe) = self.all_recipes.get(recipe_id) {
            if recipe.profession != profession { return false; }
        } else { return false; }
        if let Some(progress) = self.professions.get_mut(&profession) {
            progress.learn_recipe(recipe_id)
        } else { false }
    }

    pub fn experiment(&mut self, profession: Profession) -> Option<String> {
        let progress = self.professions.get(&profession)?;
        let current_level = progress.level;
        let discoverable: Vec<_> = self.all_recipes.values()
            .filter(|r| r.profession == profession && r.required_level <= current_level
                && r.source == RecipeSource::Experimentation && !progress.learned_recipes.contains(&r.id))
            .collect();
        if discoverable.is_empty() { return None; }
        let discovery_chance = 0.05 + (current_level as f32 * 0.001);
        if rand::random::<f32>() > discovery_chance { return None; }
        let idx = rand::random::<usize>() % discoverable.len();
        let recipe_id = discoverable[idx].id.clone();
        if let Some(progress) = self.professions.get_mut(&profession) {
            progress.discover_recipe(&recipe_id);
        }
        Some(recipe_id)
    }

    pub fn get_summary(&self) -> ProfessionSummary {
        let mut highest_level = 0;
        let mut highest_profession = None;
        for (profession, progress) in &self.professions {
            if progress.level > highest_level {
                highest_level = progress.level;
                highest_profession = Some(*profession);
            }
        }
        let total_recipes: usize = self.professions.values().map(|p| p.learned_recipes.len()).sum();
        ProfessionSummary {
            total_professions: self.professions.len() as u32, active_professions: self.active_professions.len() as u32,
            highest_level, highest_profession, total_crafted: self.total_crafted, total_gathered: self.total_gathered,
            total_recipes: total_recipes as u32, maxed_professions: self.maxed_professions, unique_materials: self.materials.len() as u32,
        }
    }

    fn initialize_recipes(&mut self) {
        // Blacksmithing recipes
        self.add_recipe(Recipe::new("iron_sword", "Iron Sword", "A basic iron sword", Profession::Blacksmithing, 1,
            vec![(GatheredMaterial::IronOre, 3), (GatheredMaterial::CommonWood, 1)], "iron_sword", 1, 20, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("steel_sword", "Steel Sword", "A sturdy steel sword", Profession::Blacksmithing, 25,
            vec![(GatheredMaterial::IronOre, 5), (GatheredMaterial::SilverOre, 2)], "steel_sword", 1, 50, RecipeSource::Trainer, RecipeRarity::Uncommon));
        self.add_recipe(Recipe::new("mithril_blade", "Mithril Blade", "A lightweight blade of mithril", Profession::Blacksmithing, 60,
            vec![(GatheredMaterial::MithrilOre, 5), (GatheredMaterial::SpiritStone, 1)], "mithril_blade", 1, 120, RecipeSource::MasterTaught, RecipeRarity::Rare));
        self.add_recipe(Recipe::new("adamantite_greatsword", "Adamantite Greatsword", "A massive greatsword", Profession::Blacksmithing, 90,
            vec![(GatheredMaterial::AdamantiteOre, 8), (GatheredMaterial::DragonBone, 2), (GatheredMaterial::ElementalCore, 1)],
            "adamantite_greatsword", 1, 250, RecipeSource::RareDrop, RecipeRarity::Epic));
        self.add_recipe(Recipe::new("dragonslayer", "Dragonslayer", "A legendary blade forged to slay dragons", Profession::Blacksmithing, 130,
            vec![(GatheredMaterial::DragoniteOre, 10), (GatheredMaterial::DragonScales, 5), (GatheredMaterial::DragonBone, 3), (GatheredMaterial::ElementalCrystal, 2)],
            "dragonslayer", 1, 500, RecipeSource::SecretDiscovery, RecipeRarity::Legendary));

        // Alchemy recipes
        self.add_recipe(Recipe::new("health_pill", "Health Pill", "Restores health", Profession::Alchemy, 1,
            vec![(GatheredMaterial::HealingHerb, 2)], "health_pill", 3, 15, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("mana_potion", "Mana Potion", "Restores mana", Profession::Alchemy, 10,
            vec![(GatheredMaterial::ManaBloom, 2), (GatheredMaterial::CommonHerb, 1)], "mana_potion", 2, 25, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("strength_elixir", "Strength Elixir", "Temporarily increases strength", Profession::Alchemy, 35,
            vec![(GatheredMaterial::FirePetal, 2), (GatheredMaterial::MonsterFang, 1), (GatheredMaterial::BeastBone, 1)],
            "strength_elixir", 1, 60, RecipeSource::Book, RecipeRarity::Uncommon));
        self.add_recipe(Recipe::new("immortality_pill", "Immortality Pill", "Grants temporary invulnerability", Profession::Alchemy, 100,
            vec![(GatheredMaterial::LifeFlower, 3), (GatheredMaterial::PhoenixFeather, 1), (GatheredMaterial::DragonBreath, 2), (GatheredMaterial::SpiritMoss, 2)],
            "immortality_pill", 1, 400, RecipeSource::SecretDiscovery, RecipeRarity::Legendary));

        // Inscription recipes
        self.add_recipe(Recipe::new("scroll_fireball", "Scroll of Fireball", "A scroll containing fireball", Profession::Inscription, 20,
            vec![(GatheredMaterial::FirePetal, 2), (GatheredMaterial::CommonHerb, 1)], "scroll_fireball", 1, 40, RecipeSource::Trainer, RecipeRarity::Uncommon));
        self.add_recipe(Recipe::new("protection_talisman", "Protection Talisman", "Provides magical protection", Profession::Inscription, 45,
            vec![(GatheredMaterial::SpiritStone, 1), (GatheredMaterial::SilverOre, 2), (GatheredMaterial::MoonFlower, 1)],
            "protection_talisman", 1, 80, RecipeSource::MasterTaught, RecipeRarity::Rare));

        // Tailoring recipes
        self.add_recipe(Recipe::new("cloth_robe", "Cloth Robe", "A simple cloth robe", Profession::Tailoring, 1,
            vec![(GatheredMaterial::RawHide, 3)], "cloth_robe", 1, 15, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("silk_robe", "Silk Robe", "A fine silk robe", Profession::Tailoring, 40,
            vec![(GatheredMaterial::ExoticLeather, 2), (GatheredMaterial::ManaBloom, 2)], "silk_robe", 1, 70, RecipeSource::Book, RecipeRarity::Rare));
        self.add_recipe(Recipe::new("celestial_vestments", "Celestial Vestments", "Robes woven from celestial materials", Profession::Tailoring, 110,
            vec![(GatheredMaterial::CelestialHide, 4), (GatheredMaterial::PhoenixFeather, 2), (GatheredMaterial::SpiritSap, 2)],
            "celestial_vestments", 1, 350, RecipeSource::RareDrop, RecipeRarity::Legendary));

        // Jewelcrafting recipes
        self.add_recipe(Recipe::new("copper_ring", "Copper Ring", "A simple copper ring", Profession::Jewelcrafting, 1,
            vec![(GatheredMaterial::CopperOre, 2)], "copper_ring", 1, 10, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("silver_amulet", "Silver Amulet", "A silver amulet with minor enchantment", Profession::Jewelcrafting, 25,
            vec![(GatheredMaterial::SilverOre, 3), (GatheredMaterial::RoughGem, 1)], "silver_amulet", 1, 45, RecipeSource::Trainer, RecipeRarity::Uncommon));
        self.add_recipe(Recipe::new("dragon_pearl_necklace", "Dragon Pearl Necklace", "A necklace with dragon pearl", Profession::Jewelcrafting, 95,
            vec![(GatheredMaterial::GoldOre, 5), (GatheredMaterial::DragonPearl, 1), (GatheredMaterial::FlawlessGem, 2)],
            "dragon_pearl_necklace", 1, 300, RecipeSource::MasterTaught, RecipeRarity::Epic));

        // Cooking recipes
        self.add_recipe(Recipe::new("cooked_meat", "Cooked Meat", "Simple cooked meat", Profession::Cooking, 1,
            vec![(GatheredMaterial::RawMeat, 1)], "cooked_meat", 1, 10, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("fish_stew", "Fish Stew", "A hearty fish stew", Profession::Cooking, 20,
            vec![(GatheredMaterial::CommonFish, 2), (GatheredMaterial::Seaweed, 1), (GatheredMaterial::CommonHerb, 1)],
            "fish_stew", 1, 30, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("dragon_feast", "Dragon Feast", "A legendary feast of dragon meat", Profession::Cooking, 120,
            vec![(GatheredMaterial::DragonMeat, 2), (GatheredMaterial::DragonBreath, 1), (GatheredMaterial::SpiritFish, 1), (GatheredMaterial::SunBlossom, 2)],
            "dragon_feast", 1, 400, RecipeSource::SecretDiscovery, RecipeRarity::Legendary));

        // Enchanting recipes
        self.add_recipe(Recipe::new("minor_enchantment", "Minor Enchantment", "Adds a minor magical effect", Profession::Enchanting, 10,
            vec![(GatheredMaterial::SpiritStone, 1), (GatheredMaterial::ManaBloom, 2)], "minor_enchantment", 1, 25, RecipeSource::Trainer, RecipeRarity::Common));
        self.add_recipe(Recipe::new("major_enchantment", "Major Enchantment", "Adds a significant magical effect", Profession::Enchanting, 55,
            vec![(GatheredMaterial::ElementalCrystal, 1), (GatheredMaterial::SpiritStone, 2), (GatheredMaterial::VoidRoot, 1)],
            "major_enchantment", 1, 100, RecipeSource::MasterTaught, RecipeRarity::Rare));

        // Formation Crafting recipes
        self.add_recipe(Recipe::new("defensive_formation", "Defensive Formation Disc", "Creates a protective barrier", Profession::FormationCrafting, 30,
            vec![(GatheredMaterial::SpiritStone, 2), (GatheredMaterial::IronOre, 3), (GatheredMaterial::MoonFlower, 1)],
            "defensive_formation", 1, 60, RecipeSource::Trainer, RecipeRarity::Uncommon));
        self.add_recipe(Recipe::new("teleportation_array", "Teleportation Array", "Creates a teleportation point", Profession::FormationCrafting, 80,
            vec![(GatheredMaterial::ElementalCrystal, 2), (GatheredMaterial::SpiritStone, 4), (GatheredMaterial::VoidRoot, 2), (GatheredMaterial::WorldTreeBranch, 1)],
            "teleportation_array", 1, 200, RecipeSource::RareDrop, RecipeRarity::Epic));
    }

    fn add_recipe(&mut self, recipe: Recipe) {
        self.all_recipes.insert(recipe.id.clone(), recipe);
    }

    fn initialize_rank_bonuses(&mut self) {
        // Mining bonuses
        self.rank_bonuses.push(RankBonus::new(Profession::Mining, ProfessionRank::Novice,
            vec![ProfessionBonus::GatheringYield { percent: 0.05 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Mining, ProfessionRank::Apprentice,
            vec![ProfessionBonus::GatheringYield { percent: 0.10 }, ProfessionBonus::RareFindChance { percent: 0.02 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Mining, ProfessionRank::Journeyman,
            vec![ProfessionBonus::StatBonus { stat: "Strength".into(), amount: 2 },
                 ProfessionBonus::AbilityUnlock { ability: "Prospect".into(), description: "Detect nearby ore veins".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Mining, ProfessionRank::Expert,
            vec![ProfessionBonus::MaterialAccess { material: "Mithril".into() }, ProfessionBonus::GatheringYield { percent: 0.20 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Mining, ProfessionRank::Master,
            vec![ProfessionBonus::MaterialAccess { material: "Adamantite".into() }, ProfessionBonus::RareFindChance { percent: 0.10 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Mining, ProfessionRank::Grandmaster,
            vec![ProfessionBonus::MaterialAccess { material: "Spirit Stone".into() }, ProfessionBonus::StatBonus { stat: "Strength".into(), amount: 5 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Mining, ProfessionRank::Legend,
            vec![ProfessionBonus::MaterialAccess { material: "Dragonite".into() },
                 ProfessionBonus::AbilityUnlock { ability: "Earth Sense".into(), description: "Reveal all ore on the current floor".into() }]));

        // Blacksmithing bonuses
        self.rank_bonuses.push(RankBonus::new(Profession::Blacksmithing, ProfessionRank::Novice,
            vec![ProfessionBonus::QualityBonus { percent: 0.05 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Blacksmithing, ProfessionRank::Apprentice,
            vec![ProfessionBonus::CraftingSpeed { percent: 0.10 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Blacksmithing, ProfessionRank::Journeyman,
            vec![ProfessionBonus::StatBonus { stat: "Strength".into(), amount: 3 }, ProfessionBonus::RecipeUnlock { recipe_id: "mithril_blade".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Blacksmithing, ProfessionRank::Expert,
            vec![ProfessionBonus::MaterialSaving { percent: 0.10 }, ProfessionBonus::AbilityUnlock { ability: "Repair".into(), description: "Repair damaged equipment".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Blacksmithing, ProfessionRank::Master,
            vec![ProfessionBonus::QualityBonus { percent: 0.20 }, ProfessionBonus::RecipeUnlock { recipe_id: "adamantite_greatsword".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Blacksmithing, ProfessionRank::Grandmaster,
            vec![ProfessionBonus::AbilityUnlock { ability: "Masterwork".into(), description: "Create masterwork quality items".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Blacksmithing, ProfessionRank::Legend,
            vec![ProfessionBonus::RecipeUnlock { recipe_id: "dragonslayer".into() },
                 ProfessionBonus::AbilityUnlock { ability: "Legendary Forge".into(), description: "Create legendary weapons".into() }]));

        // Merchant bonuses
        self.rank_bonuses.push(RankBonus::new(Profession::Merchant, ProfessionRank::Novice,
            vec![ProfessionBonus::TradingBonus { buy_discount: 0.05, sell_bonus: 0.05 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Merchant, ProfessionRank::Apprentice,
            vec![ProfessionBonus::TradingBonus { buy_discount: 0.10, sell_bonus: 0.10 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Merchant, ProfessionRank::Journeyman,
            vec![ProfessionBonus::AbilityUnlock { ability: "Haggle".into(), description: "Negotiate better prices".into() },
                 ProfessionBonus::StatBonus { stat: "Charisma".into(), amount: 2 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Merchant, ProfessionRank::Expert,
            vec![ProfessionBonus::TradingBonus { buy_discount: 0.20, sell_bonus: 0.20 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Merchant, ProfessionRank::Master,
            vec![ProfessionBonus::AbilityUnlock { ability: "Trade Routes".into(), description: "Access rare merchant inventories".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Merchant, ProfessionRank::Grandmaster,
            vec![ProfessionBonus::TradingBonus { buy_discount: 0.30, sell_bonus: 0.30 }, ProfessionBonus::StatBonus { stat: "Charisma".into(), amount: 5 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Merchant, ProfessionRank::Legend,
            vec![ProfessionBonus::AbilityUnlock { ability: "Master Trader".into(), description: "Buy at half price, sell at double".into() }]));

        // Appraiser bonuses
        self.rank_bonuses.push(RankBonus::new(Profession::Appraiser, ProfessionRank::Novice,
            vec![ProfessionBonus::IdentifyBonus { chance: 0.20, reveal_hidden: false }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Appraiser, ProfessionRank::Journeyman,
            vec![ProfessionBonus::IdentifyBonus { chance: 0.60, reveal_hidden: true }, ProfessionBonus::StatBonus { stat: "Perception".into(), amount: 3 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Appraiser, ProfessionRank::Expert,
            vec![ProfessionBonus::AbilityUnlock { ability: "Detect Curse".into(), description: "Identify cursed items".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Appraiser, ProfessionRank::Master,
            vec![ProfessionBonus::IdentifyBonus { chance: 0.90, reveal_hidden: true }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Appraiser, ProfessionRank::Legend,
            vec![ProfessionBonus::AbilityUnlock { ability: "Legendary Insight".into(), description: "Identify legendary item locations".into() }]));

        // Cartographer bonuses
        self.rank_bonuses.push(RankBonus::new(Profession::Cartographer, ProfessionRank::Novice,
            vec![ProfessionBonus::MapBonus { reveal_percent: 0.05, show_resources: false }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Cartographer, ProfessionRank::Journeyman,
            vec![ProfessionBonus::MapBonus { reveal_percent: 0.15, show_resources: true }, ProfessionBonus::StatBonus { stat: "Wisdom".into(), amount: 2 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Cartographer, ProfessionRank::Expert,
            vec![ProfessionBonus::AbilityUnlock { ability: "Find Path".into(), description: "Show optimal path to stairs".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Cartographer, ProfessionRank::Master,
            vec![ProfessionBonus::MapBonus { reveal_percent: 0.25, show_resources: true }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Cartographer, ProfessionRank::Legend,
            vec![ProfessionBonus::AbilityUnlock { ability: "World Map".into(), description: "Full floor revealed on entry".into() }]));

        // Tamer bonuses
        self.rank_bonuses.push(RankBonus::new(Profession::Tamer, ProfessionRank::Novice,
            vec![ProfessionBonus::TamingBonus { success_bonus: 0.10, loyalty_bonus: 0.05 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Tamer, ProfessionRank::Journeyman,
            vec![ProfessionBonus::StatBonus { stat: "Charisma".into(), amount: 2 },
                 ProfessionBonus::AbilityUnlock { ability: "Beast Speech".into(), description: "Communicate with beasts".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Tamer, ProfessionRank::Expert,
            vec![ProfessionBonus::TamingBonus { success_bonus: 0.35, loyalty_bonus: 0.20 }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Tamer, ProfessionRank::Master,
            vec![ProfessionBonus::AbilityUnlock { ability: "Tame Rare".into(), description: "Tame rare and exotic beasts".into() }]));
        self.rank_bonuses.push(RankBonus::new(Profession::Tamer, ProfessionRank::Legend,
            vec![ProfessionBonus::AbilityUnlock { ability: "Tame Dragon".into(), description: "Tame dragons and mythical beasts".into() }]));
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfessionSummary {
    pub total_professions: u32,
    pub active_professions: u32,
    pub highest_level: u32,
    pub highest_profession: Option<Profession>,
    pub total_crafted: u32,
    pub total_gathered: u32,
    pub total_recipes: u32,
    pub maxed_professions: u32,
    pub unique_materials: u32,
}

// ============================================================================
// Utility Functions
// ============================================================================

pub fn generate_resource_nodes(dungeon_level: u32, floor_width: usize, floor_height: usize) -> Vec<ResourceNode> {
    let mut nodes = Vec::new();
    let base_count = 3 + (dungeon_level / 5) as usize;
    let mining_materials = get_available_materials(Profession::Mining, dungeon_level);
    for _ in 0..base_count {
        if let Some(&material) = mining_materials.get(rand::random::<usize>() % mining_materials.len().max(1)) {
            let x = rand::random::<usize>() % floor_width;
            let y = rand::random::<usize>() % floor_height;
            nodes.push(ResourceNode::new(x, y, material, 2 + (rand::random::<u32>() % 4)));
        }
    }
    let herb_materials = get_available_materials(Profession::Herbalism, dungeon_level);
    for _ in 0..base_count {
        if let Some(&material) = herb_materials.get(rand::random::<usize>() % herb_materials.len().max(1)) {
            let x = rand::random::<usize>() % floor_width;
            let y = rand::random::<usize>() % floor_height;
            nodes.push(ResourceNode::new(x, y, material, 1 + (rand::random::<u32>() % 3)));
        }
    }
    nodes
}

fn get_available_materials(profession: Profession, dungeon_level: u32) -> Vec<GatheredMaterial> {
    let all_materials: Vec<GatheredMaterial> = match profession {
        Profession::Mining => vec![
            GatheredMaterial::CopperOre, GatheredMaterial::IronOre, GatheredMaterial::SilverOre,
            GatheredMaterial::GoldOre, GatheredMaterial::MithrilOre, GatheredMaterial::AdamantiteOre,
            GatheredMaterial::RoughGem, GatheredMaterial::PureGem, GatheredMaterial::SpiritStone,
        ],
        Profession::Herbalism => vec![
            GatheredMaterial::CommonHerb, GatheredMaterial::HealingHerb, GatheredMaterial::ManaBloom,
            GatheredMaterial::Nightshade, GatheredMaterial::FirePetal, GatheredMaterial::FrostLeaf,
            GatheredMaterial::MoonFlower, GatheredMaterial::LifeFlower, GatheredMaterial::SpiritMoss,
        ],
        _ => vec![],
    };
    let level_threshold = dungeon_level * 5;
    all_materials.into_iter().filter(|m| m.required_level() <= level_threshold).collect()
}

pub fn get_skinning_drops(enemy_level: u32, is_boss: bool) -> Vec<(GatheredMaterial, u32, f32)> {
    let mut drops = vec![(GatheredMaterial::RawHide, 1, 0.5)];
    if enemy_level >= 10 { drops.push((GatheredMaterial::ToughLeather, 1, 0.3)); }
    if enemy_level >= 25 { drops.push((GatheredMaterial::ExoticLeather, 1, 0.2)); drops.push((GatheredMaterial::BasicScales, 1, 0.15)); }
    if is_boss {
        if enemy_level >= 30 { drops.push((GatheredMaterial::DragonHide, 1, 0.4)); drops.push((GatheredMaterial::DragonScales, 1, 0.3)); }
        if enemy_level >= 50 { drops.push((GatheredMaterial::DemonSkin, 1, 0.3)); drops.push((GatheredMaterial::PhoenixFeather, 1, 0.2)); }
    }
    drops
}

pub fn get_hunting_drops(enemy_level: u32, is_boss: bool) -> Vec<(GatheredMaterial, u32, f32)> {
    let mut drops = vec![(GatheredMaterial::RawMeat, 1, 0.6), (GatheredMaterial::BeastBone, 1, 0.4)];
    if enemy_level >= 15 { drops.push((GatheredMaterial::QualityMeat, 1, 0.3)); drops.push((GatheredMaterial::MonsterFang, 1, 0.15)); }
    if enemy_level >= 30 { drops.push((GatheredMaterial::ExoticMeat, 1, 0.2)); drops.push((GatheredMaterial::VenomSac, 1, 0.1)); }
    if is_boss {
        if enemy_level >= 40 { drops.push((GatheredMaterial::ElementalCore, 1, 0.3)); }
        if enemy_level >= 60 { drops.push((GatheredMaterial::DragonMeat, 1, 0.4)); drops.push((GatheredMaterial::DragonBone, 1, 0.3)); }
    }
    drops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profession_ranks() {
        assert_eq!(ProfessionRank::from_level(1), ProfessionRank::Novice);
        assert_eq!(ProfessionRank::from_level(20), ProfessionRank::Novice);
        assert_eq!(ProfessionRank::from_level(21), ProfessionRank::Apprentice);
        assert_eq!(ProfessionRank::from_level(100), ProfessionRank::Master);
        assert_eq!(ProfessionRank::from_level(150), ProfessionRank::Legend);
    }

    #[test]
    fn test_profession_progress() {
        let mut progress = ProfessionProgress::new(Profession::Mining);
        assert_eq!(progress.level, 1);
        assert_eq!(progress.rank(), ProfessionRank::Novice);
        let leveled = progress.add_experience(150);
        assert!(leveled);
        assert!(progress.level > 1);
    }

    #[test]
    fn test_profession_system() {
        let mut system = ProfessionSystem::new();
        assert!(system.learn_profession(Profession::Mining));
        assert!(!system.learn_profession(Profession::Mining));
        assert!(system.active_professions.contains(&Profession::Mining));
    }

    #[test]
    fn test_material_gathering() {
        let mut system = ProfessionSystem::new();
        system.learn_profession(Profession::Mining);
        let mut node = ResourceNode::new(5, 5, GatheredMaterial::IronOre, 3);
        let result = system.gather(&mut node);
        assert!(result.is_some());
        assert!(system.material_count(GatheredMaterial::IronOre) > 0);
    }

    #[test]
    fn test_resource_node() {
        let mut node = ResourceNode::new(0, 0, GatheredMaterial::IronOre, 3);
        assert!(!node.depleted);
        while !node.depleted { node.gather(10, 0.0); }
        assert!(node.depleted);
        assert!(node.gather(10, 0.0).is_none());
        while node.depleted { node.tick(); }
        assert!(!node.depleted);
    }
}

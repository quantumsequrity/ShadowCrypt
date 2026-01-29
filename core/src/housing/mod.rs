//! Housing and Base Building System
//!
//! A comprehensive system for player housing including:
//! - 7 housing types from starter cottages to floating islands
//! - 10+ room types with unique bonuses
//! - Furniture and decorations system
//! - NPC hiring and management
//! - Base upgrades and defenses
//! - Housing benefits (rested bonus, fast travel, storage)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of rooms per housing type
pub const MAX_ROOMS_COTTAGE: usize = 3;
pub const MAX_ROOMS_HOUSE: usize = 6;
pub const MAX_ROOMS_MANSION: usize = 12;
pub const MAX_ROOMS_CASTLE: usize = 25;
pub const MAX_ROOMS_POCKET_DIMENSION: usize = 50;
pub const MAX_ROOMS_FLOATING_ISLAND: usize = 30;
pub const MAX_ROOMS_UNDERGROUND_LAIR: usize = 40;

/// Rested bonus duration in turns
pub const RESTED_BONUS_DURATION: u32 = 100;

/// Maximum NPCs per housing type
pub const MAX_NPCS_COTTAGE: usize = 1;
pub const MAX_NPCS_HOUSE: usize = 3;
pub const MAX_NPCS_MANSION: usize = 8;
pub const MAX_NPCS_CASTLE: usize = 20;
pub const MAX_NPCS_POCKET_DIMENSION: usize = 15;
pub const MAX_NPCS_FLOATING_ISLAND: usize = 12;
pub const MAX_NPCS_UNDERGROUND_LAIR: usize = 25;

// ============================================================================
// Housing Types
// ============================================================================

/// The different types of housing available
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HousingType {
    /// Small cottage - starter home
    SmallCottage,
    /// Medium house - standard home
    MediumHouse,
    /// Large mansion - luxury estate
    LargeMansion,
    /// Castle - fortified stronghold
    Castle,
    /// Pocket dimension home - extradimensional space
    PocketDimension,
    /// Floating island - aerial sanctuary
    FloatingIsland,
    /// Underground lair - subterranean hideout
    UndergroundLair,
}

impl HousingType {
    pub fn all() -> &'static [HousingType] {
        &[
            Self::SmallCottage,
            Self::MediumHouse,
            Self::LargeMansion,
            Self::Castle,
            Self::PocketDimension,
            Self::FloatingIsland,
            Self::UndergroundLair,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::SmallCottage => "Small Cottage",
            Self::MediumHouse => "Medium House",
            Self::LargeMansion => "Large Mansion",
            Self::Castle => "Castle",
            Self::PocketDimension => "Pocket Dimension",
            Self::FloatingIsland => "Floating Island",
            Self::UndergroundLair => "Underground Lair",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::SmallCottage => "A cozy starter home with basic amenities. Perfect for adventurers just starting out.",
            Self::MediumHouse => "A comfortable dwelling with room for expansion. Suitable for established adventurers.",
            Self::LargeMansion => "A luxurious estate befitting nobility. Multiple wings and grand halls await.",
            Self::Castle => "A fortified stronghold with defensive walls and towers. Command your domain.",
            Self::PocketDimension => "An extradimensional space defying physical laws. Infinite possibilities within.",
            Self::FloatingIsland => "A mystical island suspended in the clouds. Peaceful and secure from ground threats.",
            Self::UndergroundLair => "A vast subterranean complex hidden from prying eyes. Perfect for those seeking secrecy.",
        }
    }

    pub fn max_rooms(&self) -> usize {
        match self {
            Self::SmallCottage => MAX_ROOMS_COTTAGE,
            Self::MediumHouse => MAX_ROOMS_HOUSE,
            Self::LargeMansion => MAX_ROOMS_MANSION,
            Self::Castle => MAX_ROOMS_CASTLE,
            Self::PocketDimension => MAX_ROOMS_POCKET_DIMENSION,
            Self::FloatingIsland => MAX_ROOMS_FLOATING_ISLAND,
            Self::UndergroundLair => MAX_ROOMS_UNDERGROUND_LAIR,
        }
    }

    pub fn max_npcs(&self) -> usize {
        match self {
            Self::SmallCottage => MAX_NPCS_COTTAGE,
            Self::MediumHouse => MAX_NPCS_HOUSE,
            Self::LargeMansion => MAX_NPCS_MANSION,
            Self::Castle => MAX_NPCS_CASTLE,
            Self::PocketDimension => MAX_NPCS_POCKET_DIMENSION,
            Self::FloatingIsland => MAX_NPCS_FLOATING_ISLAND,
            Self::UndergroundLair => MAX_NPCS_UNDERGROUND_LAIR,
        }
    }

    pub fn base_cost(&self) -> u64 {
        match self {
            Self::SmallCottage => 500,
            Self::MediumHouse => 2_500,
            Self::LargeMansion => 15_000,
            Self::Castle => 100_000,
            Self::PocketDimension => 250_000,
            Self::FloatingIsland => 500_000,
            Self::UndergroundLair => 75_000,
        }
    }

    pub fn level_requirement(&self) -> u32 {
        match self {
            Self::SmallCottage => 1,
            Self::MediumHouse => 10,
            Self::LargeMansion => 25,
            Self::Castle => 40,
            Self::PocketDimension => 60,
            Self::FloatingIsland => 70,
            Self::UndergroundLair => 35,
        }
    }

    pub fn base_storage(&self) -> u32 {
        match self {
            Self::SmallCottage => 50,
            Self::MediumHouse => 150,
            Self::LargeMansion => 400,
            Self::Castle => 1000,
            Self::PocketDimension => 2000,
            Self::FloatingIsland => 800,
            Self::UndergroundLair => 1500,
        }
    }

    pub fn defense_rating(&self) -> u32 {
        match self {
            Self::SmallCottage => 10,
            Self::MediumHouse => 25,
            Self::LargeMansion => 50,
            Self::Castle => 200,
            Self::PocketDimension => 150,
            Self::FloatingIsland => 175,
            Self::UndergroundLair => 125,
        }
    }

    pub fn upgrade_to(&self) -> Option<HousingType> {
        match self {
            Self::SmallCottage => Some(Self::MediumHouse),
            Self::MediumHouse => Some(Self::LargeMansion),
            Self::LargeMansion => Some(Self::Castle),
            _ => None,
        }
    }

    pub fn unique_feature(&self) -> &'static str {
        match self {
            Self::SmallCottage => "Cozy Hearth - Extra rested bonus duration",
            Self::MediumHouse => "Garden Plot - Can grow basic herbs",
            Self::LargeMansion => "Grand Hall - Host gatherings for reputation",
            Self::Castle => "Throne Room - Command armies and vassals",
            Self::PocketDimension => "Time Dilation - Crafting takes less time",
            Self::FloatingIsland => "Cloud Cover - Immune to ground-based raids",
            Self::UndergroundLair => "Hidden Entrance - Cannot be found by enemies",
        }
    }
}

// ============================================================================
// Room Types
// ============================================================================

/// Types of rooms that can be built in housing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomType {
    /// Bedroom - provides rest bonus
    Bedroom,
    /// Storage room - extra item storage
    StorageRoom,
    /// Workshop - crafting bonus
    Workshop,
    /// Alchemy lab - potion bonus
    AlchemyLab,
    /// Training room - XP bonus
    TrainingRoom,
    /// Garden - grow herbs
    Garden,
    /// Stable - mount storage
    Stable,
    /// Trophy room - display achievements
    TrophyRoom,
    /// Library - lore storage
    Library,
    /// Vault - secure storage
    Vault,
    /// Kitchen - food preparation
    Kitchen,
    /// Armory - weapon storage and maintenance
    Armory,
    /// Throne room - leadership bonuses
    ThroneRoom,
    /// Observatory - magical research
    Observatory,
    /// Dungeon - prisoner holding
    Dungeon,
    /// Summoning chamber - summon creatures
    SummoningChamber,
    /// Portal room - fast travel hub
    PortalRoom,
    /// Treasury - gold storage
    Treasury,
    /// Meditation chamber - mana regeneration
    MeditationChamber,
    /// Guest quarters - companion housing
    GuestQuarters,
}

impl RoomType {
    pub fn all() -> &'static [RoomType] {
        &[
            Self::Bedroom,
            Self::StorageRoom,
            Self::Workshop,
            Self::AlchemyLab,
            Self::TrainingRoom,
            Self::Garden,
            Self::Stable,
            Self::TrophyRoom,
            Self::Library,
            Self::Vault,
            Self::Kitchen,
            Self::Armory,
            Self::ThroneRoom,
            Self::Observatory,
            Self::Dungeon,
            Self::SummoningChamber,
            Self::PortalRoom,
            Self::Treasury,
            Self::MeditationChamber,
            Self::GuestQuarters,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Bedroom => "Bedroom",
            Self::StorageRoom => "Storage Room",
            Self::Workshop => "Workshop",
            Self::AlchemyLab => "Alchemy Lab",
            Self::TrainingRoom => "Training Room",
            Self::Garden => "Garden",
            Self::Stable => "Stable",
            Self::TrophyRoom => "Trophy Room",
            Self::Library => "Library",
            Self::Vault => "Vault",
            Self::Kitchen => "Kitchen",
            Self::Armory => "Armory",
            Self::ThroneRoom => "Throne Room",
            Self::Observatory => "Observatory",
            Self::Dungeon => "Dungeon",
            Self::SummoningChamber => "Summoning Chamber",
            Self::PortalRoom => "Portal Room",
            Self::Treasury => "Treasury",
            Self::MeditationChamber => "Meditation Chamber",
            Self::GuestQuarters => "Guest Quarters",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Bedroom => "A comfortable place to rest and recover. Provides the rested bonus.",
            Self::StorageRoom => "Additional storage space for items and equipment.",
            Self::Workshop => "A crafting station for creating and improving equipment.",
            Self::AlchemyLab => "A laboratory for brewing potions and elixirs.",
            Self::TrainingRoom => "A training area for combat practice and skill improvement.",
            Self::Garden => "A plot for growing herbs, plants, and magical ingredients.",
            Self::Stable => "Housing for mounts and beasts of burden.",
            Self::TrophyRoom => "Display your achievements and conquered foes.",
            Self::Library => "A repository of knowledge, lore, and spell tomes.",
            Self::Vault => "Secure storage protected by magical wards.",
            Self::Kitchen => "Prepare food for stat-boosting meals.",
            Self::Armory => "Store and maintain weapons and armor.",
            Self::ThroneRoom => "Command your domain and receive visitors.",
            Self::Observatory => "Study the stars for magical insights.",
            Self::Dungeon => "Hold prisoners and interrogate enemies.",
            Self::SummoningChamber => "Summon and bind otherworldly creatures.",
            Self::PortalRoom => "Create portals for fast travel.",
            Self::Treasury => "Store vast amounts of gold and gems.",
            Self::MeditationChamber => "Restore mana and gain inner peace.",
            Self::GuestQuarters => "Housing for companions and visitors.",
        }
    }

    pub fn build_cost(&self) -> u64 {
        match self {
            Self::Bedroom => 100,
            Self::StorageRoom => 200,
            Self::Workshop => 500,
            Self::AlchemyLab => 800,
            Self::TrainingRoom => 600,
            Self::Garden => 300,
            Self::Stable => 400,
            Self::TrophyRoom => 350,
            Self::Library => 750,
            Self::Vault => 2000,
            Self::Kitchen => 250,
            Self::Armory => 600,
            Self::ThroneRoom => 5000,
            Self::Observatory => 3000,
            Self::Dungeon => 1500,
            Self::SummoningChamber => 4000,
            Self::PortalRoom => 10000,
            Self::Treasury => 3500,
            Self::MeditationChamber => 1200,
            Self::GuestQuarters => 450,
        }
    }

    pub fn upgrade_cost_multiplier(&self) -> f32 {
        match self {
            Self::Vault | Self::Treasury | Self::ThroneRoom => 2.5,
            Self::PortalRoom | Self::SummoningChamber | Self::Observatory => 2.0,
            _ => 1.5,
        }
    }

    pub fn max_level(&self) -> u32 {
        match self {
            Self::Bedroom | Self::StorageRoom | Self::Kitchen => 5,
            Self::Workshop | Self::AlchemyLab | Self::Garden => 10,
            Self::TrainingRoom | Self::Library | Self::MeditationChamber => 10,
            Self::Vault | Self::Treasury => 7,
            Self::ThroneRoom | Self::PortalRoom => 3,
            _ => 5,
        }
    }

    pub fn bonus(&self) -> RoomBonus {
        match self {
            Self::Bedroom => RoomBonus::RestedBonus { duration_multiplier: 1.5 },
            Self::StorageRoom => RoomBonus::StorageIncrease { slots: 25 },
            Self::Workshop => RoomBonus::CraftingBonus { percent: 15 },
            Self::AlchemyLab => RoomBonus::PotionBonus { percent: 20 },
            Self::TrainingRoom => RoomBonus::XPBonus { percent: 10 },
            Self::Garden => RoomBonus::HerbGrowth { slots: 5, growth_rate: 1.0 },
            Self::Stable => RoomBonus::MountSlots { count: 3 },
            Self::TrophyRoom => RoomBonus::AchievementDisplay { slots: 10 },
            Self::Library => RoomBonus::LoreStorage { capacity: 50, research_bonus: 10 },
            Self::Vault => RoomBonus::SecureStorage { slots: 20, theft_protection: 100 },
            Self::Kitchen => RoomBonus::FoodPreparation { meal_slots: 5, buff_duration: 50 },
            Self::Armory => RoomBonus::WeaponMaintenance { durability_bonus: 20, storage: 15 },
            Self::ThroneRoom => RoomBonus::LeadershipBonus { reputation_gain: 25, npc_capacity: 5 },
            Self::Observatory => RoomBonus::MagicalResearch { spell_discovery: 10, mana_bonus: 15 },
            Self::Dungeon => RoomBonus::PrisonerCapacity { cells: 5, interrogation_bonus: 20 },
            Self::SummoningChamber => RoomBonus::SummonCapacity { creatures: 3, binding_strength: 25 },
            Self::PortalRoom => RoomBonus::FastTravel { destinations: 3, cooldown_reduction: 20 },
            Self::Treasury => RoomBonus::GoldStorage { capacity: 100000, interest_rate: 1 },
            Self::MeditationChamber => RoomBonus::ManaRegen { percent: 25, clarity_bonus: 10 },
            Self::GuestQuarters => RoomBonus::CompanionHousing { beds: 4, morale_bonus: 15 },
        }
    }

    pub fn required_housing(&self) -> Vec<HousingType> {
        match self {
            Self::ThroneRoom => vec![HousingType::Castle],
            Self::Observatory => vec![HousingType::FloatingIsland, HousingType::Castle, HousingType::PocketDimension],
            Self::Dungeon => vec![HousingType::Castle, HousingType::UndergroundLair],
            Self::SummoningChamber => vec![HousingType::Castle, HousingType::UndergroundLair, HousingType::PocketDimension],
            Self::PortalRoom => vec![HousingType::Castle, HousingType::PocketDimension, HousingType::FloatingIsland],
            _ => vec![],
        }
    }
}

/// Bonuses provided by rooms
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoomBonus {
    RestedBonus { duration_multiplier: f32 },
    StorageIncrease { slots: u32 },
    CraftingBonus { percent: u32 },
    PotionBonus { percent: u32 },
    XPBonus { percent: u32 },
    HerbGrowth { slots: u32, growth_rate: f32 },
    MountSlots { count: u32 },
    AchievementDisplay { slots: u32 },
    LoreStorage { capacity: u32, research_bonus: u32 },
    SecureStorage { slots: u32, theft_protection: u32 },
    FoodPreparation { meal_slots: u32, buff_duration: u32 },
    WeaponMaintenance { durability_bonus: u32, storage: u32 },
    LeadershipBonus { reputation_gain: u32, npc_capacity: u32 },
    MagicalResearch { spell_discovery: u32, mana_bonus: u32 },
    PrisonerCapacity { cells: u32, interrogation_bonus: u32 },
    SummonCapacity { creatures: u32, binding_strength: u32 },
    FastTravel { destinations: u32, cooldown_reduction: u32 },
    GoldStorage { capacity: u64, interest_rate: u32 },
    ManaRegen { percent: u32, clarity_bonus: u32 },
    CompanionHousing { beds: u32, morale_bonus: u32 },
}

/// A built room instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: u64,
    pub room_type: RoomType,
    pub level: u32,
    pub name: String,
    pub furniture: Vec<Furniture>,
    pub decorations: Vec<Decoration>,
    pub condition: u32, // 0-100
    pub active: bool,
}

impl Room {
    pub fn new(id: u64, room_type: RoomType) -> Self {
        Self {
            id,
            room_type,
            level: 1,
            name: room_type.name().to_string(),
            furniture: Vec::new(),
            decorations: Vec::new(),
            condition: 100,
            active: true,
        }
    }

    pub fn upgrade_cost(&self) -> u64 {
        let base = self.room_type.build_cost();
        let multiplier = self.room_type.upgrade_cost_multiplier();
        (base as f32 * multiplier * self.level as f32) as u64
    }

    pub fn can_upgrade(&self) -> bool {
        self.level < self.room_type.max_level()
    }

    pub fn upgrade(&mut self) -> bool {
        if self.can_upgrade() {
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn scaled_bonus(&self) -> RoomBonus {
        let base = self.room_type.bonus();
        let scale = 1.0 + (self.level - 1) as f32 * 0.2;

        match base {
            RoomBonus::StorageIncrease { slots } => {
                RoomBonus::StorageIncrease { slots: (slots as f32 * scale) as u32 }
            }
            RoomBonus::CraftingBonus { percent } => {
                RoomBonus::CraftingBonus { percent: (percent as f32 * scale) as u32 }
            }
            RoomBonus::PotionBonus { percent } => {
                RoomBonus::PotionBonus { percent: (percent as f32 * scale) as u32 }
            }
            RoomBonus::XPBonus { percent } => {
                RoomBonus::XPBonus { percent: (percent as f32 * scale) as u32 }
            }
            RoomBonus::ManaRegen { percent, clarity_bonus } => {
                RoomBonus::ManaRegen {
                    percent: (percent as f32 * scale) as u32,
                    clarity_bonus: (clarity_bonus as f32 * scale) as u32,
                }
            }
            other => other,
        }
    }

    pub fn repair_cost(&self) -> u64 {
        let damage = 100 - self.condition;
        (self.room_type.build_cost() * damage as u64) / 100
    }

    pub fn repair(&mut self) {
        self.condition = 100;
    }

    pub fn degrade(&mut self, amount: u32) {
        self.condition = self.condition.saturating_sub(amount);
        if self.condition == 0 {
            self.active = false;
        }
    }
}

// ============================================================================
// Furniture and Decorations
// ============================================================================

/// Categories of furniture
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FurnitureCategory {
    Functional,
    Storage,
    Crafting,
    Comfort,
    Lighting,
    Display,
}

/// Types of furniture
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FurnitureType {
    // Beds
    SimpleBed,
    ComfortableBed,
    LuxuryBed,
    RoyalBed,
    EnchantedBed,
    // Storage
    WoodenChest,
    IronChest,
    MagicChest,
    DimensionalChest,
    // Crafting
    BasicWorkbench,
    AdvancedWorkbench,
    MasterWorkbench,
    AlchemyTable,
    EnchantingTable,
    RuneForge,
    // Seating
    WoodenChair,
    CushionedChair,
    Throne,
    // Tables
    DiningTable,
    WorkTable,
    MapTable,
    // Other
    Bookshelf,
    WeaponRack,
    ArmorStand,
    MagicCrystal,
    Fireplace,
    Chandelier,
    Mirror,
    Clock,
}

impl FurnitureType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SimpleBed => "Simple Bed",
            Self::ComfortableBed => "Comfortable Bed",
            Self::LuxuryBed => "Luxury Bed",
            Self::RoyalBed => "Royal Bed",
            Self::EnchantedBed => "Enchanted Bed",
            Self::WoodenChest => "Wooden Chest",
            Self::IronChest => "Iron Chest",
            Self::MagicChest => "Magic Chest",
            Self::DimensionalChest => "Dimensional Chest",
            Self::BasicWorkbench => "Basic Workbench",
            Self::AdvancedWorkbench => "Advanced Workbench",
            Self::MasterWorkbench => "Master Workbench",
            Self::AlchemyTable => "Alchemy Table",
            Self::EnchantingTable => "Enchanting Table",
            Self::RuneForge => "Rune Forge",
            Self::WoodenChair => "Wooden Chair",
            Self::CushionedChair => "Cushioned Chair",
            Self::Throne => "Throne",
            Self::DiningTable => "Dining Table",
            Self::WorkTable => "Work Table",
            Self::MapTable => "Map Table",
            Self::Bookshelf => "Bookshelf",
            Self::WeaponRack => "Weapon Rack",
            Self::ArmorStand => "Armor Stand",
            Self::MagicCrystal => "Magic Crystal",
            Self::Fireplace => "Fireplace",
            Self::Chandelier => "Chandelier",
            Self::Mirror => "Mirror",
            Self::Clock => "Clock",
        }
    }

    pub fn category(&self) -> FurnitureCategory {
        match self {
            Self::SimpleBed | Self::ComfortableBed | Self::LuxuryBed |
            Self::RoyalBed | Self::EnchantedBed => FurnitureCategory::Comfort,
            Self::WoodenChest | Self::IronChest | Self::MagicChest |
            Self::DimensionalChest | Self::Bookshelf | Self::WeaponRack |
            Self::ArmorStand => FurnitureCategory::Storage,
            Self::BasicWorkbench | Self::AdvancedWorkbench | Self::MasterWorkbench |
            Self::AlchemyTable | Self::EnchantingTable | Self::RuneForge => FurnitureCategory::Crafting,
            Self::WoodenChair | Self::CushionedChair | Self::Throne => FurnitureCategory::Comfort,
            Self::DiningTable | Self::WorkTable | Self::MapTable => FurnitureCategory::Functional,
            Self::Fireplace | Self::Chandelier => FurnitureCategory::Lighting,
            Self::MagicCrystal | Self::Mirror | Self::Clock => FurnitureCategory::Display,
        }
    }

    pub fn cost(&self) -> u64 {
        match self {
            Self::SimpleBed => 50,
            Self::ComfortableBed => 150,
            Self::LuxuryBed => 500,
            Self::RoyalBed => 2000,
            Self::EnchantedBed => 5000,
            Self::WoodenChest => 30,
            Self::IronChest => 100,
            Self::MagicChest => 500,
            Self::DimensionalChest => 5000,
            Self::BasicWorkbench => 100,
            Self::AdvancedWorkbench => 500,
            Self::MasterWorkbench => 2000,
            Self::AlchemyTable => 750,
            Self::EnchantingTable => 1500,
            Self::RuneForge => 3000,
            Self::WoodenChair => 20,
            Self::CushionedChair => 75,
            Self::Throne => 5000,
            Self::DiningTable => 100,
            Self::WorkTable => 150,
            Self::MapTable => 300,
            Self::Bookshelf => 80,
            Self::WeaponRack => 120,
            Self::ArmorStand => 150,
            Self::MagicCrystal => 1000,
            Self::Fireplace => 200,
            Self::Chandelier => 500,
            Self::Mirror => 250,
            Self::Clock => 400,
        }
    }

    pub fn bonus(&self) -> Option<FurnitureBonus> {
        match self {
            Self::SimpleBed => Some(FurnitureBonus::RestBonus(10)),
            Self::ComfortableBed => Some(FurnitureBonus::RestBonus(25)),
            Self::LuxuryBed => Some(FurnitureBonus::RestBonus(50)),
            Self::RoyalBed => Some(FurnitureBonus::RestBonus(75)),
            Self::EnchantedBed => Some(FurnitureBonus::RestBonus(100)),
            Self::WoodenChest => Some(FurnitureBonus::Storage(10)),
            Self::IronChest => Some(FurnitureBonus::Storage(25)),
            Self::MagicChest => Some(FurnitureBonus::Storage(50)),
            Self::DimensionalChest => Some(FurnitureBonus::Storage(200)),
            Self::BasicWorkbench => Some(FurnitureBonus::CraftingSpeed(10)),
            Self::AdvancedWorkbench => Some(FurnitureBonus::CraftingSpeed(25)),
            Self::MasterWorkbench => Some(FurnitureBonus::CraftingSpeed(50)),
            Self::AlchemyTable => Some(FurnitureBonus::AlchemyBonus(20)),
            Self::EnchantingTable => Some(FurnitureBonus::EnchantBonus(25)),
            Self::RuneForge => Some(FurnitureBonus::RuneBonus(30)),
            Self::Throne => Some(FurnitureBonus::ReputationBonus(20)),
            Self::MapTable => Some(FurnitureBonus::ExplorationBonus(15)),
            Self::Bookshelf => Some(FurnitureBonus::LoreStorage(20)),
            Self::WeaponRack => Some(FurnitureBonus::WeaponStorage(8)),
            Self::ArmorStand => Some(FurnitureBonus::ArmorStorage(4)),
            Self::MagicCrystal => Some(FurnitureBonus::ManaRegen(10)),
            Self::Fireplace => Some(FurnitureBonus::Comfort(15)),
            Self::Chandelier => Some(FurnitureBonus::Aesthetics(20)),
            _ => None,
        }
    }
}

/// Bonuses from furniture
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FurnitureBonus {
    RestBonus(u32),
    Storage(u32),
    CraftingSpeed(u32),
    AlchemyBonus(u32),
    EnchantBonus(u32),
    RuneBonus(u32),
    ReputationBonus(u32),
    ExplorationBonus(u32),
    LoreStorage(u32),
    WeaponStorage(u32),
    ArmorStorage(u32),
    ManaRegen(u32),
    Comfort(u32),
    Aesthetics(u32),
}

/// A placed furniture instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Furniture {
    pub id: u64,
    pub furniture_type: FurnitureType,
    pub quality: ItemQuality,
    pub condition: u32,
    pub enchantments: Vec<FurnitureEnchantment>,
    pub position: (u32, u32),
}

impl Furniture {
    pub fn new(id: u64, furniture_type: FurnitureType, quality: ItemQuality) -> Self {
        Self {
            id,
            furniture_type,
            quality,
            condition: 100,
            enchantments: Vec::new(),
            position: (0, 0),
        }
    }

    pub fn effective_bonus(&self) -> Option<FurnitureBonus> {
        let base = self.furniture_type.bonus()?;
        let quality_mult = self.quality.multiplier();

        Some(match base {
            FurnitureBonus::RestBonus(v) => FurnitureBonus::RestBonus((v as f32 * quality_mult) as u32),
            FurnitureBonus::Storage(v) => FurnitureBonus::Storage((v as f32 * quality_mult) as u32),
            FurnitureBonus::CraftingSpeed(v) => FurnitureBonus::CraftingSpeed((v as f32 * quality_mult) as u32),
            other => other,
        })
    }
}

/// Item quality levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemQuality {
    Poor,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl ItemQuality {
    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Poor => 0.5,
            Self::Common => 1.0,
            Self::Uncommon => 1.25,
            Self::Rare => 1.5,
            Self::Epic => 2.0,
            Self::Legendary => 3.0,
            Self::Mythic => 5.0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Poor => "Poor",
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
            Self::Mythic => "Mythic",
        }
    }
}

/// Furniture enchantments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FurnitureEnchantment {
    Durability { bonus: u32 },
    Efficiency { percent: u32 },
    Capacity { bonus: u32 },
    Protection { level: u32 },
    Comfort { bonus: u32 },
    Fortune { luck_bonus: u32 },
}

// ============================================================================
// Decorations
// ============================================================================

/// Types of decorations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecorationType {
    // Paintings
    SmallPainting,
    LargePainting,
    MasterPainting,
    PortraitPainting,
    LandscapePainting,
    // Statues
    SmallStatue,
    MediumStatue,
    GrandStatue,
    HeroStatue,
    GodStatue,
    // Rugs
    SimpleRug,
    WovenRug,
    FineRug,
    RoyalCarpet,
    MagicCarpet,
    // Plants
    PottedPlant,
    FlowerArrangement,
    BonsaiTree,
    MagicPlant,
    AncientTree,
    // Trophies
    MonsterHead,
    BossSkull,
    DragonSkull,
    AncientRelic,
    // Wall decorations
    Tapestry,
    Banner,
    Shield,
    MountedWeapon,
    // Lighting
    Candelabra,
    MagicOrb,
    FairyLights,
    // Other
    Fountain,
    Aquarium,
    Trophy,
    Certificate,
}

impl DecorationType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SmallPainting => "Small Painting",
            Self::LargePainting => "Large Painting",
            Self::MasterPainting => "Master Painting",
            Self::PortraitPainting => "Portrait",
            Self::LandscapePainting => "Landscape Painting",
            Self::SmallStatue => "Small Statue",
            Self::MediumStatue => "Medium Statue",
            Self::GrandStatue => "Grand Statue",
            Self::HeroStatue => "Hero Statue",
            Self::GodStatue => "God Statue",
            Self::SimpleRug => "Simple Rug",
            Self::WovenRug => "Woven Rug",
            Self::FineRug => "Fine Rug",
            Self::RoyalCarpet => "Royal Carpet",
            Self::MagicCarpet => "Magic Carpet",
            Self::PottedPlant => "Potted Plant",
            Self::FlowerArrangement => "Flower Arrangement",
            Self::BonsaiTree => "Bonsai Tree",
            Self::MagicPlant => "Magic Plant",
            Self::AncientTree => "Ancient Tree",
            Self::MonsterHead => "Monster Head",
            Self::BossSkull => "Boss Skull",
            Self::DragonSkull => "Dragon Skull",
            Self::AncientRelic => "Ancient Relic",
            Self::Tapestry => "Tapestry",
            Self::Banner => "Banner",
            Self::Shield => "Decorative Shield",
            Self::MountedWeapon => "Mounted Weapon",
            Self::Candelabra => "Candelabra",
            Self::MagicOrb => "Magic Orb",
            Self::FairyLights => "Fairy Lights",
            Self::Fountain => "Fountain",
            Self::Aquarium => "Aquarium",
            Self::Trophy => "Trophy",
            Self::Certificate => "Certificate",
        }
    }

    pub fn cost(&self) -> u64 {
        match self {
            Self::SmallPainting => 100,
            Self::LargePainting => 300,
            Self::MasterPainting => 2000,
            Self::PortraitPainting => 500,
            Self::LandscapePainting => 400,
            Self::SmallStatue => 200,
            Self::MediumStatue => 600,
            Self::GrandStatue => 2500,
            Self::HeroStatue => 5000,
            Self::GodStatue => 25000,
            Self::SimpleRug => 50,
            Self::WovenRug => 150,
            Self::FineRug => 500,
            Self::RoyalCarpet => 2000,
            Self::MagicCarpet => 10000,
            Self::PottedPlant => 25,
            Self::FlowerArrangement => 75,
            Self::BonsaiTree => 300,
            Self::MagicPlant => 1500,
            Self::AncientTree => 10000,
            Self::MonsterHead => 0, // Trophy drops
            Self::BossSkull => 0,
            Self::DragonSkull => 0,
            Self::AncientRelic => 0,
            Self::Tapestry => 400,
            Self::Banner => 150,
            Self::Shield => 200,
            Self::MountedWeapon => 300,
            Self::Candelabra => 250,
            Self::MagicOrb => 1000,
            Self::FairyLights => 500,
            Self::Fountain => 3000,
            Self::Aquarium => 2000,
            Self::Trophy => 0, // Achievement reward
            Self::Certificate => 0,
        }
    }

    pub fn aesthetics_value(&self) -> u32 {
        match self {
            Self::SmallPainting => 5,
            Self::LargePainting => 10,
            Self::MasterPainting => 30,
            Self::PortraitPainting => 15,
            Self::LandscapePainting => 12,
            Self::SmallStatue => 8,
            Self::MediumStatue => 15,
            Self::GrandStatue => 35,
            Self::HeroStatue => 50,
            Self::GodStatue => 100,
            Self::SimpleRug => 3,
            Self::WovenRug => 8,
            Self::FineRug => 15,
            Self::RoyalCarpet => 30,
            Self::MagicCarpet => 50,
            Self::PottedPlant => 2,
            Self::FlowerArrangement => 5,
            Self::BonsaiTree => 12,
            Self::MagicPlant => 25,
            Self::AncientTree => 60,
            Self::MonsterHead => 20,
            Self::BossSkull => 40,
            Self::DragonSkull => 100,
            Self::AncientRelic => 75,
            Self::Tapestry => 15,
            Self::Banner => 8,
            Self::Shield => 10,
            Self::MountedWeapon => 12,
            Self::Candelabra => 10,
            Self::MagicOrb => 20,
            Self::FairyLights => 15,
            Self::Fountain => 40,
            Self::Aquarium => 25,
            Self::Trophy => 30,
            Self::Certificate => 10,
        }
    }

    pub fn is_trophy(&self) -> bool {
        matches!(
            self,
            Self::MonsterHead | Self::BossSkull | Self::DragonSkull |
            Self::AncientRelic | Self::Trophy | Self::Certificate
        )
    }
}

/// A placed decoration instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Decoration {
    pub id: u64,
    pub decoration_type: DecorationType,
    pub name: String,
    pub description: Option<String>,
    pub quality: ItemQuality,
    pub position: (u32, u32),
    /// For trophies, the source (boss name, achievement, etc.)
    pub source: Option<String>,
}

impl Decoration {
    pub fn new(id: u64, decoration_type: DecorationType, quality: ItemQuality) -> Self {
        Self {
            id,
            decoration_type,
            name: decoration_type.name().to_string(),
            description: None,
            quality,
            position: (0, 0),
            source: None,
        }
    }

    pub fn trophy(id: u64, decoration_type: DecorationType, source: String, description: String) -> Self {
        Self {
            id,
            decoration_type,
            name: format!("{} - {}", decoration_type.name(), source),
            description: Some(description),
            quality: ItemQuality::Legendary,
            position: (0, 0),
            source: Some(source),
        }
    }

    pub fn aesthetics_value(&self) -> u32 {
        let base = self.decoration_type.aesthetics_value();
        (base as f32 * self.quality.multiplier()) as u32
    }
}

// ============================================================================
// Housing NPCs
// ============================================================================

/// Types of NPCs that can live in housing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HousingNpcType {
    // Servants
    Butler,
    Maid,
    Cook,
    Gardener,
    Stablehand,
    // Guards
    Guard,
    EliteGuard,
    MageGuard,
    Captain,
    // Specialists
    Blacksmith,
    Alchemist,
    Enchanter,
    Librarian,
    Healer,
    Trainer,
    // Companions
    Companion,
    Pet,
    Familiar,
    // Merchants
    Merchant,
    TravelingTrader,
    Collector,
}

impl HousingNpcType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Butler => "Butler",
            Self::Maid => "Maid",
            Self::Cook => "Cook",
            Self::Gardener => "Gardener",
            Self::Stablehand => "Stablehand",
            Self::Guard => "Guard",
            Self::EliteGuard => "Elite Guard",
            Self::MageGuard => "Mage Guard",
            Self::Captain => "Guard Captain",
            Self::Blacksmith => "Blacksmith",
            Self::Alchemist => "Alchemist",
            Self::Enchanter => "Enchanter",
            Self::Librarian => "Librarian",
            Self::Healer => "Healer",
            Self::Trainer => "Trainer",
            Self::Companion => "Companion",
            Self::Pet => "Pet",
            Self::Familiar => "Familiar",
            Self::Merchant => "Merchant",
            Self::TravelingTrader => "Traveling Trader",
            Self::Collector => "Collector",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Butler => "Manages the household and provides daily bonuses.",
            Self::Maid => "Keeps the home clean, improving room efficiency.",
            Self::Cook => "Prepares meals that provide stat buffs.",
            Self::Gardener => "Tends the garden, improving herb growth.",
            Self::Stablehand => "Cares for mounts, improving their stats.",
            Self::Guard => "Protects the home from intruders.",
            Self::EliteGuard => "A skilled warrior providing strong defense.",
            Self::MageGuard => "A mage who defends with magic.",
            Self::Captain => "Commands other guards, boosting their effectiveness.",
            Self::Blacksmith => "Repairs and upgrades equipment.",
            Self::Alchemist => "Brews potions while you're away.",
            Self::Enchanter => "Can enchant furniture and items.",
            Self::Librarian => "Organizes lore and provides research bonuses.",
            Self::Healer => "Provides healing services.",
            Self::Trainer => "Offers training for XP bonuses.",
            Self::Companion => "A loyal companion who lives with you.",
            Self::Pet => "A friendly pet that boosts morale.",
            Self::Familiar => "A magical familiar that aids in magic.",
            Self::Merchant => "Buys and sells items from your home.",
            Self::TravelingTrader => "Visits periodically with rare goods.",
            Self::Collector => "Buys specific rare items at premium prices.",
        }
    }

    pub fn hire_cost(&self) -> u64 {
        match self {
            Self::Butler => 500,
            Self::Maid => 200,
            Self::Cook => 300,
            Self::Gardener => 250,
            Self::Stablehand => 200,
            Self::Guard => 400,
            Self::EliteGuard => 1500,
            Self::MageGuard => 2000,
            Self::Captain => 3000,
            Self::Blacksmith => 1000,
            Self::Alchemist => 1200,
            Self::Enchanter => 2000,
            Self::Librarian => 600,
            Self::Healer => 1500,
            Self::Trainer => 1000,
            Self::Companion => 0, // Must be recruited through gameplay
            Self::Pet => 100,
            Self::Familiar => 0, // Must be summoned
            Self::Merchant => 800,
            Self::TravelingTrader => 0, // Appears randomly
            Self::Collector => 0, // Appears based on reputation
        }
    }

    pub fn daily_wage(&self) -> u64 {
        match self {
            Self::Butler => 20,
            Self::Maid => 8,
            Self::Cook => 12,
            Self::Gardener => 10,
            Self::Stablehand => 8,
            Self::Guard => 15,
            Self::EliteGuard => 50,
            Self::MageGuard => 75,
            Self::Captain => 100,
            Self::Blacksmith => 30,
            Self::Alchemist => 40,
            Self::Enchanter => 60,
            Self::Librarian => 20,
            Self::Healer => 50,
            Self::Trainer => 35,
            Self::Companion | Self::Pet | Self::Familiar => 0,
            Self::Merchant => 25,
            Self::TravelingTrader | Self::Collector => 0,
        }
    }

    pub fn category(&self) -> NpcCategory {
        match self {
            Self::Butler | Self::Maid | Self::Cook | Self::Gardener | Self::Stablehand => NpcCategory::Servant,
            Self::Guard | Self::EliteGuard | Self::MageGuard | Self::Captain => NpcCategory::Guard,
            Self::Blacksmith | Self::Alchemist | Self::Enchanter | Self::Librarian |
            Self::Healer | Self::Trainer => NpcCategory::Specialist,
            Self::Companion | Self::Pet | Self::Familiar => NpcCategory::Companion,
            Self::Merchant | Self::TravelingTrader | Self::Collector => NpcCategory::Merchant,
        }
    }
}

/// Categories of housing NPCs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NpcCategory {
    Servant,
    Guard,
    Specialist,
    Companion,
    Merchant,
}

/// A housed NPC instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HousingNpc {
    pub id: u64,
    pub npc_type: HousingNpcType,
    pub name: String,
    pub level: u32,
    pub happiness: u32, // 0-100
    pub loyalty: u32,   // 0-100
    pub hired_turn: u64,
    pub assigned_room: Option<u64>,
    pub skills: Vec<NpcSkill>,
    pub schedule: NpcSchedule,
    pub inventory: Vec<String>, // For merchants
}

impl HousingNpc {
    pub fn new(id: u64, npc_type: HousingNpcType, name: String) -> Self {
        Self {
            id,
            npc_type,
            name,
            level: 1,
            happiness: 75,
            loyalty: 50,
            hired_turn: 0,
            assigned_room: None,
            skills: Self::default_skills(npc_type),
            schedule: NpcSchedule::default(),
            inventory: Vec::new(),
        }
    }

    fn default_skills(npc_type: HousingNpcType) -> Vec<NpcSkill> {
        match npc_type {
            HousingNpcType::Blacksmith => vec![
                NpcSkill::Repair { efficiency: 20 },
                NpcSkill::Craft { specialty: "Weapons".to_string() },
            ],
            HousingNpcType::Alchemist => vec![
                NpcSkill::Brew { potion_types: vec!["Health".to_string(), "Mana".to_string()] },
            ],
            HousingNpcType::Guard | HousingNpcType::EliteGuard | HousingNpcType::MageGuard => vec![
                NpcSkill::Defend { power: 20 },
            ],
            HousingNpcType::Cook => vec![
                NpcSkill::Cook { recipes: vec!["Basic Meal".to_string()] },
            ],
            HousingNpcType::Gardener => vec![
                NpcSkill::Garden { growth_bonus: 15 },
            ],
            HousingNpcType::Trainer => vec![
                NpcSkill::Train { xp_bonus: 10 },
            ],
            HousingNpcType::Healer => vec![
                NpcSkill::Heal { power: 30 },
            ],
            _ => vec![],
        }
    }

    pub fn effective_daily_wage(&self) -> u64 {
        let base = self.npc_type.daily_wage();
        // Unhappy NPCs demand more
        let happiness_mod = if self.happiness < 25 {
            1.5
        } else if self.happiness < 50 {
            1.25
        } else {
            1.0
        };
        (base as f64 * happiness_mod) as u64
    }

    pub fn modify_happiness(&mut self, amount: i32) {
        let new_val = (self.happiness as i32 + amount).clamp(0, 100);
        self.happiness = new_val as u32;
    }

    pub fn modify_loyalty(&mut self, amount: i32) {
        let new_val = (self.loyalty as i32 + amount).clamp(0, 100);
        self.loyalty = new_val as u32;
    }

    pub fn might_leave(&self) -> bool {
        self.happiness < 20 && self.loyalty < 30
    }

    pub fn effectiveness(&self) -> f32 {
        let happiness_factor = self.happiness as f32 / 100.0;
        let loyalty_factor = self.loyalty as f32 / 100.0;
        let level_factor = 1.0 + (self.level - 1) as f32 * 0.1;

        happiness_factor * 0.5 + loyalty_factor * 0.3 + level_factor * 0.2
    }
}

/// Skills that NPCs can have
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NpcSkill {
    Repair { efficiency: u32 },
    Craft { specialty: String },
    Brew { potion_types: Vec<String> },
    Cook { recipes: Vec<String> },
    Defend { power: u32 },
    Garden { growth_bonus: u32 },
    Train { xp_bonus: u32 },
    Heal { power: u32 },
    Research { topics: Vec<String> },
    Enchant { types: Vec<String> },
    Trade { discount: u32 },
    Collect { items: Vec<String> },
}

/// NPC schedule
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NpcSchedule {
    pub work_start: u32, // Hour of day (0-23)
    pub work_end: u32,
    pub break_time: u32,
    pub days_off: Vec<u32>, // Days of the week (0-6)
}

// ============================================================================
// Base Upgrades and Defenses
// ============================================================================

/// Types of housing upgrades
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeType {
    // Structural
    ExpandRoom,
    AddFloor,
    ReinforcedWalls,
    MagicWards,
    // Defensive
    Walls,
    Moat,
    Towers,
    Traps,
    Golems,
    MagicBarrier,
    // Utility
    PlumbingSystem,
    HeatingSystem,
    LightingSystem,
    TeleportPad,
    // Aesthetic
    GardenExpansion,
    Courtyard,
    Balcony,
    Rooftop,
    Basement,
    SecretPassage,
}

impl UpgradeType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ExpandRoom => "Room Expansion",
            Self::AddFloor => "Additional Floor",
            Self::ReinforcedWalls => "Reinforced Walls",
            Self::MagicWards => "Magic Wards",
            Self::Walls => "Defensive Walls",
            Self::Moat => "Moat",
            Self::Towers => "Guard Towers",
            Self::Traps => "Defensive Traps",
            Self::Golems => "Guardian Golems",
            Self::MagicBarrier => "Magic Barrier",
            Self::PlumbingSystem => "Plumbing System",
            Self::HeatingSystem => "Heating System",
            Self::LightingSystem => "Lighting System",
            Self::TeleportPad => "Teleport Pad",
            Self::GardenExpansion => "Garden Expansion",
            Self::Courtyard => "Courtyard",
            Self::Balcony => "Balcony",
            Self::Rooftop => "Rooftop Access",
            Self::Basement => "Basement",
            Self::SecretPassage => "Secret Passage",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ExpandRoom => "Increase the size of a room for more furniture.",
            Self::AddFloor => "Add another floor to your home.",
            Self::ReinforcedWalls => "Strengthen walls against attacks.",
            Self::MagicWards => "Add magical protection against intrusion.",
            Self::Walls => "Build defensive walls around your property.",
            Self::Moat => "Dig a moat to deter ground attacks.",
            Self::Towers => "Construct guard towers for archers.",
            Self::Traps => "Install traps to catch intruders.",
            Self::Golems => "Create golems to patrol and defend.",
            Self::MagicBarrier => "Erect a magical barrier around the property.",
            Self::PlumbingSystem => "Install running water throughout.",
            Self::HeatingSystem => "Add heating for comfort bonus.",
            Self::LightingSystem => "Install magical lighting.",
            Self::TeleportPad => "Create a teleportation platform.",
            Self::GardenExpansion => "Expand garden space for more plants.",
            Self::Courtyard => "Add an outdoor courtyard area.",
            Self::Balcony => "Build balconies with scenic views.",
            Self::Rooftop => "Create rooftop access and terrace.",
            Self::Basement => "Dig a basement for storage or secrets.",
            Self::SecretPassage => "Build hidden passages for escape.",
        }
    }

    pub fn cost(&self) -> u64 {
        match self {
            Self::ExpandRoom => 500,
            Self::AddFloor => 5000,
            Self::ReinforcedWalls => 2000,
            Self::MagicWards => 3000,
            Self::Walls => 4000,
            Self::Moat => 6000,
            Self::Towers => 3500,
            Self::Traps => 1500,
            Self::Golems => 10000,
            Self::MagicBarrier => 15000,
            Self::PlumbingSystem => 1000,
            Self::HeatingSystem => 1500,
            Self::LightingSystem => 800,
            Self::TeleportPad => 20000,
            Self::GardenExpansion => 1200,
            Self::Courtyard => 2500,
            Self::Balcony => 800,
            Self::Rooftop => 1500,
            Self::Basement => 3000,
            Self::SecretPassage => 5000,
        }
    }

    pub fn max_level(&self) -> u32 {
        match self {
            Self::Walls | Self::Towers | Self::MagicWards | Self::MagicBarrier => 5,
            Self::Traps | Self::Golems => 3,
            Self::AddFloor | Self::Basement => 3,
            _ => 1,
        }
    }

    pub fn category(&self) -> UpgradeCategory {
        match self {
            Self::ExpandRoom | Self::AddFloor | Self::ReinforcedWalls | Self::MagicWards => UpgradeCategory::Structural,
            Self::Walls | Self::Moat | Self::Towers | Self::Traps | Self::Golems | Self::MagicBarrier => UpgradeCategory::Defensive,
            Self::PlumbingSystem | Self::HeatingSystem | Self::LightingSystem | Self::TeleportPad => UpgradeCategory::Utility,
            Self::GardenExpansion | Self::Courtyard | Self::Balcony | Self::Rooftop | Self::Basement | Self::SecretPassage => UpgradeCategory::Aesthetic,
        }
    }

    pub fn defense_bonus(&self) -> u32 {
        match self {
            Self::ReinforcedWalls => 25,
            Self::MagicWards => 30,
            Self::Walls => 50,
            Self::Moat => 40,
            Self::Towers => 35,
            Self::Traps => 20,
            Self::Golems => 60,
            Self::MagicBarrier => 75,
            _ => 0,
        }
    }
}

/// Categories of upgrades
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeCategory {
    Structural,
    Defensive,
    Utility,
    Aesthetic,
}

/// An installed upgrade
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HousingUpgrade {
    pub upgrade_type: UpgradeType,
    pub level: u32,
    pub installed_turn: u64,
    pub condition: u32,
}

impl HousingUpgrade {
    pub fn new(upgrade_type: UpgradeType, turn: u64) -> Self {
        Self {
            upgrade_type,
            level: 1,
            installed_turn: turn,
            condition: 100,
        }
    }

    pub fn upgrade_cost(&self) -> u64 {
        let base = self.upgrade_type.cost();
        base * (self.level as u64 + 1)
    }

    pub fn can_upgrade(&self) -> bool {
        self.level < self.upgrade_type.max_level()
    }

    pub fn upgrade(&mut self) -> bool {
        if self.can_upgrade() {
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn scaled_defense(&self) -> u32 {
        let base = self.upgrade_type.defense_bonus();
        base * self.level
    }
}

// ============================================================================
// Housing Benefits
// ============================================================================

/// Rested bonus from sleeping at home
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RestedBonus {
    pub active: bool,
    pub duration_remaining: u32,
    pub xp_bonus: u32,
    pub health_regen: u32,
    pub mana_regen: u32,
    pub stamina_regen: u32,
}

impl Default for RestedBonus {
    fn default() -> Self {
        Self {
            active: false,
            duration_remaining: 0,
            xp_bonus: 0,
            health_regen: 0,
            mana_regen: 0,
            stamina_regen: 0,
        }
    }
}

impl RestedBonus {
    pub fn apply(duration: u32, quality: u32) -> Self {
        Self {
            active: true,
            duration_remaining: duration,
            xp_bonus: 10 + quality / 5,
            health_regen: 5 + quality / 10,
            mana_regen: 5 + quality / 10,
            stamina_regen: 10 + quality / 5,
        }
    }

    pub fn tick(&mut self) {
        if self.active && self.duration_remaining > 0 {
            self.duration_remaining -= 1;
            if self.duration_remaining == 0 {
                self.active = false;
            }
        }
    }
}

/// Fast travel destination
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FastTravelDestination {
    pub name: String,
    pub location_id: u64,
    pub unlocked: bool,
    pub cost: u64,
    pub cooldown: u32,
    pub last_used: u64,
}

/// Housing storage
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HousingStorage {
    pub general_items: Vec<StoredItem>,
    pub vault_items: Vec<StoredItem>,
    pub weapon_storage: Vec<StoredItem>,
    pub armor_storage: Vec<StoredItem>,
    pub material_storage: Vec<StoredItem>,
    pub capacity: StorageCapacity,
}

/// Stored item reference
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredItem {
    pub item_id: String,
    pub name: String,
    pub quantity: u32,
    pub stored_turn: u64,
}

/// Storage capacity limits
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageCapacity {
    pub general: u32,
    pub vault: u32,
    pub weapons: u32,
    pub armor: u32,
    pub materials: u32,
}

impl Default for StorageCapacity {
    fn default() -> Self {
        Self {
            general: 50,
            vault: 10,
            weapons: 10,
            armor: 10,
            materials: 25,
        }
    }
}

// ============================================================================
// Garden System
// ============================================================================

/// A garden plot for growing herbs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GardenPlot {
    pub id: u64,
    pub name: String,
    pub size: u32,
    pub soil_quality: u32,
    pub plants: Vec<PlantedCrop>,
    pub features: Vec<GardenFeature>,
}

impl GardenPlot {
    pub fn new(id: u64, size: u32) -> Self {
        Self {
            id,
            name: format!("Garden Plot {}", id),
            size,
            soil_quality: 50,
            plants: Vec::new(),
            features: Vec::new(),
        }
    }

    pub fn available_slots(&self) -> u32 {
        self.size.saturating_sub(self.plants.len() as u32)
    }

    pub fn plant(&mut self, crop: PlantedCrop) -> bool {
        if self.available_slots() > 0 {
            self.plants.push(crop);
            true
        } else {
            false
        }
    }

    pub fn harvest_ready(&self) -> Vec<&PlantedCrop> {
        self.plants.iter().filter(|p| p.is_ready()).collect()
    }

    pub fn tick(&mut self, growth_bonus: f32) {
        for plant in &mut self.plants {
            plant.grow(self.soil_quality, growth_bonus);
        }
    }
}

/// A planted crop
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlantedCrop {
    pub plant_type: PlantType,
    pub growth_stage: u32,
    pub growth_required: u32,
    pub health: u32,
    pub yield_amount: u32,
    pub planted_turn: u64,
}

impl PlantedCrop {
    pub fn new(plant_type: PlantType, turn: u64) -> Self {
        Self {
            plant_type,
            growth_stage: 0,
            growth_required: plant_type.growth_time(),
            health: 100,
            yield_amount: plant_type.base_yield(),
            planted_turn: turn,
        }
    }

    pub fn grow(&mut self, soil_quality: u32, bonus: f32) {
        if self.health > 0 && self.growth_stage < self.growth_required {
            let growth = (1.0 + (soil_quality as f32 / 100.0) + bonus) as u32;
            self.growth_stage = (self.growth_stage + growth).min(self.growth_required);
        }
    }

    pub fn is_ready(&self) -> bool {
        self.growth_stage >= self.growth_required
    }

    pub fn progress_percent(&self) -> f32 {
        self.growth_stage as f32 / self.growth_required as f32 * 100.0
    }
}

/// Types of plants that can be grown
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlantType {
    // Herbs
    Herb,
    MedicinalHerb,
    MagicHerb,
    RareHerb,
    // Flowers
    Flower,
    MagicFlower,
    // Vegetables
    Vegetable,
    Root,
    // Special
    MandrakeRoot,
    NightshadeFlower,
    GlowMushroom,
    ManaBloom,
    LifeLeaf,
    DeathBlossom,
}

impl PlantType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Herb => "Common Herb",
            Self::MedicinalHerb => "Medicinal Herb",
            Self::MagicHerb => "Magic Herb",
            Self::RareHerb => "Rare Herb",
            Self::Flower => "Flower",
            Self::MagicFlower => "Magic Flower",
            Self::Vegetable => "Vegetable",
            Self::Root => "Root",
            Self::MandrakeRoot => "Mandrake Root",
            Self::NightshadeFlower => "Nightshade Flower",
            Self::GlowMushroom => "Glow Mushroom",
            Self::ManaBloom => "Mana Bloom",
            Self::LifeLeaf => "Life Leaf",
            Self::DeathBlossom => "Death Blossom",
        }
    }

    pub fn growth_time(&self) -> u32 {
        match self {
            Self::Herb | Self::Flower | Self::Vegetable => 10,
            Self::MedicinalHerb | Self::Root => 15,
            Self::MagicHerb | Self::MagicFlower => 25,
            Self::RareHerb | Self::GlowMushroom => 40,
            Self::MandrakeRoot | Self::NightshadeFlower => 50,
            Self::ManaBloom | Self::LifeLeaf => 60,
            Self::DeathBlossom => 80,
        }
    }

    pub fn base_yield(&self) -> u32 {
        match self {
            Self::Herb | Self::Flower | Self::Vegetable => 3,
            Self::MedicinalHerb | Self::Root => 2,
            Self::MagicHerb | Self::MagicFlower | Self::GlowMushroom => 2,
            Self::RareHerb => 1,
            Self::MandrakeRoot | Self::NightshadeFlower => 1,
            Self::ManaBloom | Self::LifeLeaf | Self::DeathBlossom => 1,
        }
    }

    pub fn seed_cost(&self) -> u64 {
        match self {
            Self::Herb | Self::Flower | Self::Vegetable => 10,
            Self::MedicinalHerb | Self::Root => 25,
            Self::MagicHerb | Self::MagicFlower => 75,
            Self::RareHerb | Self::GlowMushroom => 200,
            Self::MandrakeRoot | Self::NightshadeFlower => 500,
            Self::ManaBloom | Self::LifeLeaf => 1000,
            Self::DeathBlossom => 2500,
        }
    }
}

/// Garden features
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GardenFeature {
    Irrigation { efficiency: u32 },
    Greenhouse { growth_bonus: u32 },
    Scarecrow { pest_protection: u32 },
    Compost { soil_bonus: u32 },
    MagicStone { mana_infusion: u32 },
}

// ============================================================================
// Main Housing Instance
// ============================================================================

/// A player's housing instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Housing {
    pub id: u64,
    pub housing_type: HousingType,
    pub name: String,
    pub location: HousingLocation,
    pub rooms: Vec<Room>,
    pub npcs: Vec<HousingNpc>,
    pub upgrades: Vec<HousingUpgrade>,
    pub storage: HousingStorage,
    pub gardens: Vec<GardenPlot>,
    pub fast_travel_destinations: Vec<FastTravelDestination>,
    pub aesthetics_score: u32,
    pub defense_rating: u32,
    pub comfort_level: u32,
    pub purchase_turn: u64,
    pub total_gold_invested: u64,
    pub last_visited: u64,
}

/// Housing location in the world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HousingLocation {
    pub world_x: i32,
    pub world_y: i32,
    pub region: String,
    pub biome: String,
}

impl Housing {
    pub fn new(id: u64, housing_type: HousingType, name: String, location: HousingLocation, turn: u64) -> Self {
        let mut housing = Self {
            id,
            housing_type,
            name,
            location,
            rooms: Vec::new(),
            npcs: Vec::new(),
            upgrades: Vec::new(),
            storage: HousingStorage {
                capacity: StorageCapacity {
                    general: housing_type.base_storage(),
                    ..Default::default()
                },
                ..Default::default()
            },
            gardens: Vec::new(),
            fast_travel_destinations: vec![FastTravelDestination {
                name: "Home".to_string(),
                location_id: id,
                unlocked: true,
                cost: 0,
                cooldown: 0,
                last_used: 0,
            }],
            aesthetics_score: 0,
            defense_rating: housing_type.defense_rating(),
            comfort_level: 50,
            purchase_turn: turn,
            total_gold_invested: housing_type.base_cost(),
            last_visited: turn,
        };

        // Add default bedroom
        housing.rooms.push(Room::new(1, RoomType::Bedroom));

        housing
    }

    pub fn can_add_room(&self) -> bool {
        self.rooms.len() < self.housing_type.max_rooms()
    }

    pub fn add_room(&mut self, room_type: RoomType) -> Result<u64, HousingError> {
        if !self.can_add_room() {
            return Err(HousingError::MaxRoomsReached);
        }

        let required = room_type.required_housing();
        if !required.is_empty() && !required.contains(&self.housing_type) {
            return Err(HousingError::RoomNotAllowed);
        }

        let id = self.rooms.len() as u64 + 1;
        self.rooms.push(Room::new(id, room_type));
        self.total_gold_invested += room_type.build_cost();
        self.recalculate_stats();

        Ok(id)
    }

    pub fn can_hire_npc(&self) -> bool {
        self.npcs.len() < self.housing_type.max_npcs()
    }

    pub fn hire_npc(&mut self, npc: HousingNpc) -> Result<u64, HousingError> {
        if !self.can_hire_npc() {
            return Err(HousingError::MaxNpcsReached);
        }

        let id = npc.id;
        self.npcs.push(npc);

        Ok(id)
    }

    pub fn fire_npc(&mut self, npc_id: u64) -> Option<HousingNpc> {
        if let Some(pos) = self.npcs.iter().position(|n| n.id == npc_id) {
            Some(self.npcs.remove(pos))
        } else {
            None
        }
    }

    pub fn add_upgrade(&mut self, upgrade_type: UpgradeType, turn: u64) -> Result<(), HousingError> {
        // Check if already at max level
        if let Some(existing) = self.upgrades.iter_mut().find(|u| u.upgrade_type == upgrade_type) {
            if existing.can_upgrade() {
                existing.upgrade();
                self.total_gold_invested += existing.upgrade_cost();
                self.recalculate_stats();
                return Ok(());
            } else {
                return Err(HousingError::MaxUpgradeLevel);
            }
        }

        self.upgrades.push(HousingUpgrade::new(upgrade_type, turn));
        self.total_gold_invested += upgrade_type.cost();
        self.recalculate_stats();

        Ok(())
    }

    pub fn add_garden(&mut self, size: u32) -> u64 {
        let id = self.gardens.len() as u64 + 1;
        self.gardens.push(GardenPlot::new(id, size));
        id
    }

    pub fn recalculate_stats(&mut self) {
        // Calculate aesthetics
        self.aesthetics_score = self.rooms.iter()
            .flat_map(|r| r.decorations.iter())
            .map(|d| d.aesthetics_value())
            .sum();

        // Calculate defense
        self.defense_rating = self.housing_type.defense_rating() +
            self.upgrades.iter()
                .map(|u| u.scaled_defense())
                .sum::<u32>();

        // Calculate comfort
        let furniture_comfort: u32 = self.rooms.iter()
            .flat_map(|r| r.furniture.iter())
            .filter_map(|f| match f.furniture_type.bonus() {
                Some(FurnitureBonus::Comfort(c)) => Some(c),
                Some(FurnitureBonus::RestBonus(r)) => Some(r / 2),
                _ => None,
            })
            .sum();

        self.comfort_level = 50 + furniture_comfort.min(50);
    }

    pub fn calculate_rest_bonus(&self) -> RestedBonus {
        let base_duration = RESTED_BONUS_DURATION;

        // Find best bed
        let bed_bonus: u32 = self.rooms.iter()
            .filter(|r| r.room_type == RoomType::Bedroom)
            .flat_map(|r| r.furniture.iter())
            .filter_map(|f| match f.effective_bonus() {
                Some(FurnitureBonus::RestBonus(b)) => Some(b),
                _ => None,
            })
            .max()
            .unwrap_or(0);

        let duration = base_duration + (bed_bonus * 2);

        RestedBonus::apply(duration, self.comfort_level + bed_bonus)
    }

    pub fn total_storage(&self) -> u32 {
        let base = self.storage.capacity.general;
        let from_rooms: u32 = self.rooms.iter()
            .filter(|r| r.room_type == RoomType::StorageRoom)
            .map(|r| match r.scaled_bonus() {
                RoomBonus::StorageIncrease { slots } => slots,
                _ => 0,
            })
            .sum();
        let from_furniture: u32 = self.rooms.iter()
            .flat_map(|r| r.furniture.iter())
            .filter_map(|f| match f.effective_bonus() {
                Some(FurnitureBonus::Storage(s)) => Some(s),
                _ => None,
            })
            .sum();

        base + from_rooms + from_furniture
    }

    pub fn daily_maintenance_cost(&self) -> u64 {
        let npc_wages: u64 = self.npcs.iter()
            .map(|n| n.effective_daily_wage())
            .sum();

        let base_maintenance = match self.housing_type {
            HousingType::SmallCottage => 5,
            HousingType::MediumHouse => 15,
            HousingType::LargeMansion => 50,
            HousingType::Castle => 200,
            HousingType::PocketDimension => 150,
            HousingType::FloatingIsland => 175,
            HousingType::UndergroundLair => 100,
        };

        npc_wages + base_maintenance
    }

    pub fn tick(&mut self, current_turn: u64) {
        // Tick gardens
        let gardener_bonus: f32 = self.npcs.iter()
            .filter(|n| n.npc_type == HousingNpcType::Gardener)
            .map(|n| n.effectiveness() * 0.2)
            .sum();

        for garden in &mut self.gardens {
            garden.tick(gardener_bonus);
        }

        // Degrade rooms slightly
        for room in &mut self.rooms {
            if current_turn % 100 == 0 {
                room.degrade(1);
            }
        }

        // Update NPC happiness based on conditions
        let comfort = self.comfort_level;
        for npc in &mut self.npcs {
            if comfort >= 75 {
                npc.modify_happiness(1);
            } else if comfort < 40 {
                npc.modify_happiness(-1);
            }
        }
    }

    pub fn get_bonuses(&self) -> HousingBonuses {
        let mut bonuses = HousingBonuses::default();

        // Gather all room bonuses
        for room in &self.rooms {
            if !room.active {
                continue;
            }

            match room.scaled_bonus() {
                RoomBonus::XPBonus { percent } => bonuses.xp_bonus += percent,
                RoomBonus::CraftingBonus { percent } => bonuses.crafting_bonus += percent,
                RoomBonus::PotionBonus { percent } => bonuses.potion_bonus += percent,
                RoomBonus::ManaRegen { percent, .. } => bonuses.mana_regen += percent,
                _ => {}
            }
        }

        // Add NPC bonuses
        for npc in &self.npcs {
            let eff = npc.effectiveness();
            for skill in &npc.skills {
                match skill {
                    NpcSkill::Train { xp_bonus } => {
                        bonuses.xp_bonus += (*xp_bonus as f32 * eff) as u32;
                    }
                    NpcSkill::Garden { growth_bonus } => {
                        bonuses.garden_growth += (*growth_bonus as f32 * eff) as u32;
                    }
                    _ => {}
                }
            }
        }

        bonuses
    }
}

/// Aggregated bonuses from housing
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HousingBonuses {
    pub xp_bonus: u32,
    pub crafting_bonus: u32,
    pub potion_bonus: u32,
    pub mana_regen: u32,
    pub health_regen: u32,
    pub garden_growth: u32,
    pub defense_bonus: u32,
    pub storage_bonus: u32,
}

/// Housing errors
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HousingError {
    MaxRoomsReached,
    MaxNpcsReached,
    MaxUpgradeLevel,
    RoomNotAllowed,
    InsufficientFunds,
    LevelTooLow,
    NoAvailableSlots,
    ItemNotFound,
    NpcNotFound,
    RoomNotFound,
}

impl std::fmt::Display for HousingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaxRoomsReached => write!(f, "Maximum number of rooms reached"),
            Self::MaxNpcsReached => write!(f, "Maximum number of NPCs reached"),
            Self::MaxUpgradeLevel => write!(f, "Upgrade already at maximum level"),
            Self::RoomNotAllowed => write!(f, "This room type is not allowed for this housing type"),
            Self::InsufficientFunds => write!(f, "Insufficient funds"),
            Self::LevelTooLow => write!(f, "Player level too low"),
            Self::NoAvailableSlots => write!(f, "No available slots"),
            Self::ItemNotFound => write!(f, "Item not found"),
            Self::NpcNotFound => write!(f, "NPC not found"),
            Self::RoomNotFound => write!(f, "Room not found"),
        }
    }
}

impl std::error::Error for HousingError {}

// ============================================================================
// Housing System
// ============================================================================

/// Events that can occur in housing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HousingEvent {
    NpcHired { npc_id: u64, npc_name: String },
    NpcLeft { npc_id: u64, npc_name: String, reason: String },
    RoomBuilt { room_id: u64, room_type: RoomType },
    RoomUpgraded { room_id: u64, new_level: u32 },
    UpgradeInstalled { upgrade_type: UpgradeType },
    HarvestReady { garden_id: u64, plant_type: PlantType },
    MerchantVisit { merchant_name: String },
    IntruderDetected { defense_successful: bool },
    FurniturePlaced { furniture_type: FurnitureType, room_id: u64 },
    MaintenanceDue { cost: u64 },
    RestedBonusGained { duration: u32 },
}

/// The main housing system manager
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HousingSystem {
    pub owned_housing: Vec<Housing>,
    pub primary_home: Option<u64>,
    pub total_gold_spent: u64,
    pub events: Vec<HousingEvent>,
    pub rested_bonus: RestedBonus,
    pub last_tick: u64,
    next_housing_id: u64,
    next_npc_id: u64,
}

impl HousingSystem {
    pub fn new() -> Self {
        Self {
            owned_housing: Vec::new(),
            primary_home: None,
            total_gold_spent: 0,
            events: Vec::new(),
            rested_bonus: RestedBonus::default(),
            last_tick: 0,
            next_housing_id: 1,
            next_npc_id: 1,
        }
    }

    pub fn purchase_housing(
        &mut self,
        housing_type: HousingType,
        name: String,
        location: HousingLocation,
        turn: u64,
    ) -> Result<u64, HousingError> {
        let id = self.next_housing_id;
        self.next_housing_id += 1;

        let housing = Housing::new(id, housing_type, name, location, turn);
        self.total_gold_spent += housing_type.base_cost();
        self.owned_housing.push(housing);

        if self.primary_home.is_none() {
            self.primary_home = Some(id);
        }

        Ok(id)
    }

    pub fn sell_housing(&mut self, housing_id: u64) -> Option<u64> {
        if let Some(pos) = self.owned_housing.iter().position(|h| h.id == housing_id) {
            let housing = self.owned_housing.remove(pos);
            let sell_price = housing.total_gold_invested / 2;

            if self.primary_home == Some(housing_id) {
                self.primary_home = self.owned_housing.first().map(|h| h.id);
            }

            Some(sell_price)
        } else {
            None
        }
    }

    pub fn get_housing(&self, id: u64) -> Option<&Housing> {
        self.owned_housing.iter().find(|h| h.id == id)
    }

    pub fn get_housing_mut(&mut self, id: u64) -> Option<&mut Housing> {
        self.owned_housing.iter_mut().find(|h| h.id == id)
    }

    pub fn get_primary_home(&self) -> Option<&Housing> {
        self.primary_home.and_then(|id| self.get_housing(id))
    }

    pub fn get_primary_home_mut(&mut self) -> Option<&mut Housing> {
        if let Some(id) = self.primary_home {
            self.get_housing_mut(id)
        } else {
            None
        }
    }

    pub fn set_primary_home(&mut self, housing_id: u64) -> bool {
        if self.owned_housing.iter().any(|h| h.id == housing_id) {
            self.primary_home = Some(housing_id);
            true
        } else {
            false
        }
    }

    pub fn hire_npc(
        &mut self,
        housing_id: u64,
        npc_type: HousingNpcType,
        name: String,
        turn: u64,
    ) -> Result<u64, HousingError> {
        let housing = self.get_housing_mut(housing_id)
            .ok_or(HousingError::ItemNotFound)?;

        if !housing.can_hire_npc() {
            return Err(HousingError::MaxNpcsReached);
        }

        let npc_id = self.next_npc_id;
        self.next_npc_id += 1;

        let mut npc = HousingNpc::new(npc_id, npc_type, name.clone());
        npc.hired_turn = turn;

        housing.hire_npc(npc)?;

        self.events.push(HousingEvent::NpcHired {
            npc_id,
            npc_name: name,
        });

        Ok(npc_id)
    }

    pub fn fire_npc(&mut self, housing_id: u64, npc_id: u64) -> Result<(), HousingError> {
        let housing = self.get_housing_mut(housing_id)
            .ok_or(HousingError::ItemNotFound)?;

        if let Some(npc) = housing.fire_npc(npc_id) {
            self.events.push(HousingEvent::NpcLeft {
                npc_id,
                npc_name: npc.name,
                reason: "Fired".to_string(),
            });
            Ok(())
        } else {
            Err(HousingError::NpcNotFound)
        }
    }

    pub fn build_room(
        &mut self,
        housing_id: u64,
        room_type: RoomType,
    ) -> Result<u64, HousingError> {
        let housing = self.get_housing_mut(housing_id)
            .ok_or(HousingError::ItemNotFound)?;

        let room_id = housing.add_room(room_type)?;

        self.events.push(HousingEvent::RoomBuilt {
            room_id,
            room_type,
        });

        Ok(room_id)
    }

    pub fn upgrade_room(
        &mut self,
        housing_id: u64,
        room_id: u64,
    ) -> Result<u32, HousingError> {
        let housing = self.get_housing_mut(housing_id)
            .ok_or(HousingError::ItemNotFound)?;

        let room = housing.rooms.iter_mut()
            .find(|r| r.id == room_id)
            .ok_or(HousingError::RoomNotFound)?;

        if !room.can_upgrade() {
            return Err(HousingError::MaxUpgradeLevel);
        }

        let cost = room.upgrade_cost();
        room.upgrade();
        housing.total_gold_invested += cost;

        let new_level = room.level;

        self.events.push(HousingEvent::RoomUpgraded {
            room_id,
            new_level,
        });

        Ok(new_level)
    }

    pub fn install_upgrade(
        &mut self,
        housing_id: u64,
        upgrade_type: UpgradeType,
        turn: u64,
    ) -> Result<(), HousingError> {
        let housing = self.get_housing_mut(housing_id)
            .ok_or(HousingError::ItemNotFound)?;

        housing.add_upgrade(upgrade_type, turn)?;

        self.events.push(HousingEvent::UpgradeInstalled { upgrade_type });

        Ok(())
    }

    pub fn sleep_at_home(&mut self, housing_id: u64) -> Option<RestedBonus> {
        let housing = self.get_housing(housing_id)?;
        let bonus = housing.calculate_rest_bonus();
        self.rested_bonus = bonus.clone();

        self.events.push(HousingEvent::RestedBonusGained {
            duration: bonus.duration_remaining,
        });

        Some(bonus)
    }

    pub fn fast_travel_home(&self) -> Option<&HousingLocation> {
        self.get_primary_home().map(|h| &h.location)
    }

    pub fn total_daily_costs(&self) -> u64 {
        self.owned_housing.iter()
            .map(|h| h.daily_maintenance_cost())
            .sum()
    }

    pub fn total_storage_capacity(&self) -> u32 {
        self.owned_housing.iter()
            .map(|h| h.total_storage())
            .sum()
    }

    pub fn tick(&mut self, current_turn: u64) {
        // Update rested bonus
        self.rested_bonus.tick();

        // Tick all housing
        for housing in &mut self.owned_housing {
            housing.tick(current_turn);

            // Check for NPCs that might leave
            let leaving: Vec<(u64, String)> = housing.npcs.iter()
                .filter(|n| n.might_leave())
                .map(|n| (n.id, n.name.clone()))
                .collect();

            for (id, name) in leaving {
                housing.fire_npc(id);
                self.events.push(HousingEvent::NpcLeft {
                    npc_id: id,
                    npc_name: name,
                    reason: "Low happiness and loyalty".to_string(),
                });
            }

            // Check gardens for ready harvests
            for garden in &housing.gardens {
                for plant in garden.harvest_ready() {
                    self.events.push(HousingEvent::HarvestReady {
                        garden_id: garden.id,
                        plant_type: plant.plant_type,
                    });
                }
            }
        }

        self.last_tick = current_turn;
    }

    pub fn get_combined_bonuses(&self) -> HousingBonuses {
        let mut combined = HousingBonuses::default();

        for housing in &self.owned_housing {
            let bonuses = housing.get_bonuses();
            combined.xp_bonus += bonuses.xp_bonus;
            combined.crafting_bonus += bonuses.crafting_bonus;
            combined.potion_bonus += bonuses.potion_bonus;
            combined.mana_regen += bonuses.mana_regen;
            combined.health_regen += bonuses.health_regen;
            combined.garden_growth += bonuses.garden_growth;
            combined.defense_bonus += bonuses.defense_bonus;
            combined.storage_bonus += bonuses.storage_bonus;
        }

        // Add rested bonus if active
        if self.rested_bonus.active {
            combined.xp_bonus += self.rested_bonus.xp_bonus;
            combined.health_regen += self.rested_bonus.health_regen;
            combined.mana_regen += self.rested_bonus.mana_regen;
        }

        combined
    }

    pub fn drain_events(&mut self) -> Vec<HousingEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn housing_count(&self) -> usize {
        self.owned_housing.len()
    }

    pub fn total_rooms(&self) -> usize {
        self.owned_housing.iter()
            .map(|h| h.rooms.len())
            .sum()
    }

    pub fn total_npcs(&self) -> usize {
        self.owned_housing.iter()
            .map(|h| h.npcs.len())
            .sum()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate a random NPC name based on type
pub fn generate_npc_name(npc_type: HousingNpcType) -> String {
    let first_names = match npc_type.category() {
        NpcCategory::Servant => vec!["James", "Alfred", "Geoffrey", "Martha", "Eliza", "Beatrice"],
        NpcCategory::Guard => vec!["Roland", "Marcus", "Viktor", "Elena", "Astrid", "Kira"],
        NpcCategory::Specialist => vec!["Theron", "Magnus", "Aldric", "Seraphina", "Morgana", "Isolde"],
        NpcCategory::Companion => vec!["Shadow", "Storm", "Ember", "Frost", "Luna", "Sol"],
        NpcCategory::Merchant => vec!["Gareth", "Hugo", "Felix", "Lydia", "Portia", "Claudia"],
    };

    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() % first_names.len() as u128) as usize;

    first_names[idx].to_string()
}

/// Calculate total housing value
pub fn calculate_housing_value(housing: &Housing) -> u64 {
    let base = housing.housing_type.base_cost();
    let rooms: u64 = housing.rooms.iter()
        .map(|r| r.room_type.build_cost() * r.level as u64)
        .sum();
    let furniture: u64 = housing.rooms.iter()
        .flat_map(|r| r.furniture.iter())
        .map(|f| (f.furniture_type.cost() as f32 * f.quality.multiplier()) as u64)
        .sum();
    let decorations: u64 = housing.rooms.iter()
        .flat_map(|r| r.decorations.iter())
        .map(|d| (d.decoration_type.cost() as f32 * d.quality.multiplier()) as u64)
        .sum();
    let upgrades: u64 = housing.upgrades.iter()
        .map(|u| u.upgrade_type.cost() * u.level as u64)
        .sum();

    base + rooms + furniture + decorations + upgrades
}

/// Check if player can afford housing
pub fn can_afford_housing(player_gold: u64, housing_type: HousingType) -> bool {
    player_gold >= housing_type.base_cost()
}

/// Check if player meets level requirement for housing
pub fn meets_housing_requirements(player_level: u32, housing_type: HousingType) -> bool {
    player_level >= housing_type.level_requirement()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_housing_creation() {
        let location = HousingLocation {
            world_x: 100,
            world_y: 200,
            region: "Test Region".to_string(),
            biome: "Forest".to_string(),
        };

        let housing = Housing::new(1, HousingType::SmallCottage, "Test Home".to_string(), location, 0);

        assert_eq!(housing.housing_type, HousingType::SmallCottage);
        assert_eq!(housing.rooms.len(), 1); // Default bedroom
        assert!(housing.can_add_room());
    }

    #[test]
    fn test_room_upgrade() {
        let mut room = Room::new(1, RoomType::Workshop);
        assert!(room.can_upgrade());
        assert!(room.upgrade());
        assert_eq!(room.level, 2);
    }

    #[test]
    fn test_npc_happiness() {
        let mut npc = HousingNpc::new(1, HousingNpcType::Butler, "Test".to_string());
        npc.modify_happiness(-60);
        npc.modify_loyalty(-40);
        assert!(npc.might_leave());
    }

    #[test]
    fn test_housing_system() {
        let mut system = HousingSystem::new();
        let location = HousingLocation {
            world_x: 0,
            world_y: 0,
            region: "Test".to_string(),
            biome: "Test".to_string(),
        };

        let id = system.purchase_housing(
            HousingType::SmallCottage,
            "Home".to_string(),
            location,
            0,
        ).unwrap();

        assert_eq!(system.housing_count(), 1);
        assert_eq!(system.primary_home, Some(id));
    }

    #[test]
    fn test_rested_bonus() {
        let bonus = RestedBonus::apply(100, 75);
        assert!(bonus.active);
        assert!(bonus.xp_bonus > 0);
        assert!(bonus.duration_remaining > 0);
    }

    #[test]
    fn test_garden_planting() {
        let mut garden = GardenPlot::new(1, 5);
        let crop = PlantedCrop::new(PlantType::Herb, 0);

        assert!(garden.plant(crop));
        assert_eq!(garden.available_slots(), 4);
    }

    #[test]
    fn test_furniture_bonus() {
        let furniture = Furniture::new(1, FurnitureType::ComfortableBed, ItemQuality::Rare);
        let bonus = furniture.effective_bonus();
        assert!(bonus.is_some());
    }

    #[test]
    fn test_decoration_aesthetics() {
        let decoration = Decoration::new(1, DecorationType::GrandStatue, ItemQuality::Epic);
        let value = decoration.aesthetics_value();
        assert!(value > DecorationType::GrandStatue.aesthetics_value());
    }

    #[test]
    fn test_housing_type_properties() {
        for housing_type in HousingType::all() {
            assert!(!housing_type.name().is_empty());
            assert!(housing_type.max_rooms() > 0);
            assert!(housing_type.base_cost() > 0);
        }
    }

    #[test]
    fn test_room_type_properties() {
        for room_type in RoomType::all() {
            assert!(!room_type.name().is_empty());
            assert!(room_type.build_cost() > 0);
            assert!(room_type.max_level() > 0);
        }
    }
}

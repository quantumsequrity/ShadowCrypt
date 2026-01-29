//! Advanced Dungeon Generation System
//!
//! This module provides a comprehensive procedural dungeon generation system with:
//! - 35+ unique room types with specific gameplay mechanics
//! - Multiple room shapes (rectangle, circle, L-shape, T-shape, cross, irregular, cave-like)
//! - Special features (hidden rooms, secret passages, teleporters, elevators)
//! - Environmental hazards (lava, spikes, poison gas, ice, darkness)
//! - Theme-specific generation with appropriate monsters and loot
//! - Procedural events and dynamic encounters

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::entities::EnemyKind;
use crate::items::Item;
use crate::world::{
    DungeonTheme, Map, Room, Tile, BOSS_LEVELS, MAP_HEIGHT, MAP_WIDTH,
    MAX_ROOMS, MAX_ROOM_SIZE, MIN_ROOM_SIZE,
};

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of special rooms per dungeon level
pub const MAX_SPECIAL_ROOMS: usize = 5;
/// Maximum number of hidden rooms per level
pub const MAX_HIDDEN_ROOMS: usize = 2;
/// Minimum distance between special features
pub const MIN_FEATURE_DISTANCE: usize = 5;
/// Maximum corridor length before adding a door
pub const MAX_CORRIDOR_LENGTH: usize = 12;

// ============================================================================
// Room Types (35+ types)
// ============================================================================

/// Comprehensive room types for dungeon generation
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RoomType {
    // Combat Rooms
    StandardCombat,
    AmbushRoom,
    ArenaRoom,
    HordeRoom,
    DuelChamber,
    GuardPost,

    // Treasure Rooms
    TreasureVault,
    GoldCache,
    GemChamber,
    ArtifactRoom,
    DragonHoard,

    // Puzzle Rooms
    PuzzleChamber,
    RiddleRoom,
    SequenceRoom,
    MirrorMaze,
    TimedChallenge,

    // Trap Rooms
    TrapGauntlet,
    SpikePit,
    ArrowGallery,
    CrushingWalls,
    PoisonChamber,

    // Boss Chambers
    BossLair,
    MiniBossRoom,
    ChampionArena,
    ThroneRoom,

    // Rest Areas
    SafeHaven,
    Campsite,
    HotSpring,
    MeditationRoom,

    // Shop Rooms
    MerchantCamp,
    BlackMarket,
    Armory,
    Apothecary,
    EnchantmentShop,

    // Shrine Rooms
    BlessingShrine,
    CurseShrine,
    SacrificeAltar,
    HealingFountain,
    ManaWell,
    StatShrine,

    // Library Rooms
    AncientLibrary,
    ScrollArchive,
    TomeVault,
    SecretStudy,

    // Forge Rooms
    SmithyForge,
    RuneForge,
    ElementalForge,
    LegendaryAnvil,

    // Garden Rooms
    MushroomGarden,
    CrystalGarden,
    PoisonGarden,
    HerbGarden,

    // Prison Cells
    DungeonCell,
    TortureChamber,
    ExecutionRoom,
    EscapedPrisoner,

    // Throne Rooms
    AbandonedThrone,
    CultistThrone,
    GoblinKingCourt,
    NecromancerDen,

    // Altar Rooms
    DarkAltar,
    BloodAltar,
    ElementalAltar,
    AncientAltar,

    // Portal Rooms
    TeleportHub,
    DimensionalRift,
    StairsNexus,
    WaypointShrine,

    // Miscellaneous
    Storage,
    Barracks,
    MessHall,
    Laboratory,
    Observatory,
    Crypt,
    Ossuary,
    Treasury,
    WeaponCache,
    Workshop,
}

impl RoomType {
    /// Returns the display name of this room type
    pub fn name(&self) -> &'static str {
        match self {
            // Combat
            Self::StandardCombat => "Combat Chamber",
            Self::AmbushRoom => "Ambush Point",
            Self::ArenaRoom => "Battle Arena",
            Self::HordeRoom => "Monster Horde",
            Self::DuelChamber => "Duel Chamber",
            Self::GuardPost => "Guard Post",

            // Treasure
            Self::TreasureVault => "Treasure Vault",
            Self::GoldCache => "Gold Cache",
            Self::GemChamber => "Gem Chamber",
            Self::ArtifactRoom => "Artifact Repository",
            Self::DragonHoard => "Dragon Hoard",

            // Puzzle
            Self::PuzzleChamber => "Puzzle Chamber",
            Self::RiddleRoom => "Riddle Room",
            Self::SequenceRoom => "Sequence Room",
            Self::MirrorMaze => "Mirror Maze",
            Self::TimedChallenge => "Timed Challenge",

            // Trap
            Self::TrapGauntlet => "Trap Gauntlet",
            Self::SpikePit => "Spike Pit",
            Self::ArrowGallery => "Arrow Gallery",
            Self::CrushingWalls => "Crushing Walls",
            Self::PoisonChamber => "Poison Chamber",

            // Boss
            Self::BossLair => "Boss Lair",
            Self::MiniBossRoom => "Mini-Boss Room",
            Self::ChampionArena => "Champion Arena",
            Self::ThroneRoom => "Throne Room",

            // Rest
            Self::SafeHaven => "Safe Haven",
            Self::Campsite => "Campsite",
            Self::HotSpring => "Hot Spring",
            Self::MeditationRoom => "Meditation Room",

            // Shop
            Self::MerchantCamp => "Merchant Camp",
            Self::BlackMarket => "Black Market",
            Self::Armory => "Armory",
            Self::Apothecary => "Apothecary",
            Self::EnchantmentShop => "Enchantment Shop",

            // Shrine
            Self::BlessingShrine => "Blessing Shrine",
            Self::CurseShrine => "Curse Shrine",
            Self::SacrificeAltar => "Sacrifice Altar",
            Self::HealingFountain => "Healing Fountain",
            Self::ManaWell => "Mana Well",
            Self::StatShrine => "Empowerment Shrine",

            // Library
            Self::AncientLibrary => "Ancient Library",
            Self::ScrollArchive => "Scroll Archive",
            Self::TomeVault => "Tome Vault",
            Self::SecretStudy => "Secret Study",

            // Forge
            Self::SmithyForge => "Smithy Forge",
            Self::RuneForge => "Rune Forge",
            Self::ElementalForge => "Elemental Forge",
            Self::LegendaryAnvil => "Legendary Anvil",

            // Garden
            Self::MushroomGarden => "Mushroom Garden",
            Self::CrystalGarden => "Crystal Garden",
            Self::PoisonGarden => "Poison Garden",
            Self::HerbGarden => "Herb Garden",

            // Prison
            Self::DungeonCell => "Dungeon Cell",
            Self::TortureChamber => "Torture Chamber",
            Self::ExecutionRoom => "Execution Room",
            Self::EscapedPrisoner => "Escaped Prisoner",

            // Throne
            Self::AbandonedThrone => "Abandoned Throne",
            Self::CultistThrone => "Cultist Throne",
            Self::GoblinKingCourt => "Goblin Court",
            Self::NecromancerDen => "Necromancer Den",

            // Altar
            Self::DarkAltar => "Dark Altar",
            Self::BloodAltar => "Blood Altar",
            Self::ElementalAltar => "Elemental Altar",
            Self::AncientAltar => "Ancient Altar",

            // Portal
            Self::TeleportHub => "Teleport Hub",
            Self::DimensionalRift => "Dimensional Rift",
            Self::StairsNexus => "Stairs Nexus",
            Self::WaypointShrine => "Waypoint Shrine",

            // Misc
            Self::Storage => "Storage Room",
            Self::Barracks => "Barracks",
            Self::MessHall => "Mess Hall",
            Self::Laboratory => "Laboratory",
            Self::Observatory => "Observatory",
            Self::Crypt => "Crypt",
            Self::Ossuary => "Ossuary",
            Self::Treasury => "Treasury",
            Self::WeaponCache => "Weapon Cache",
            Self::Workshop => "Workshop",
        }
    }

    /// Returns whether this room type is safe (no enemies spawn)
    pub fn is_safe(&self) -> bool {
        matches!(
            self,
            Self::SafeHaven
                | Self::Campsite
                | Self::HotSpring
                | Self::MeditationRoom
                | Self::MerchantCamp
                | Self::BlackMarket
                | Self::Armory
                | Self::Apothecary
                | Self::EnchantmentShop
                | Self::HealingFountain
                | Self::TeleportHub
                | Self::WaypointShrine
        )
    }

    /// Returns the minimum level this room type can appear at
    pub fn min_level(&self) -> u32 {
        match self {
            // Early game
            Self::StandardCombat | Self::GuardPost | Self::Storage | Self::DungeonCell => 1,
            Self::TreasureVault | Self::PuzzleChamber | Self::SafeHaven | Self::Campsite => 1,
            Self::BlessingShrine | Self::HealingFountain | Self::MerchantCamp => 1,

            // Mid-early game
            Self::AmbushRoom | Self::TrapGauntlet | Self::GoldCache | Self::Barracks => 3,
            Self::AncientLibrary | Self::SmithyForge | Self::MushroomGarden => 3,
            Self::ManaWell | Self::StatShrine => 3,

            // Mid game
            Self::ArenaRoom | Self::GemChamber | Self::RiddleRoom | Self::SpikePit => 5,
            Self::MiniBossRoom | Self::HotSpring | Self::BlackMarket | Self::ScrollArchive => 5,
            Self::HerbGarden | Self::TortureChamber | Self::TeleportHub => 5,

            // Mid-late game
            Self::HordeRoom | Self::ArtifactRoom | Self::SequenceRoom | Self::ArrowGallery => 8,
            Self::ChampionArena | Self::MeditationRoom | Self::Apothecary | Self::TomeVault => 8,
            Self::RuneForge | Self::CrystalGarden | Self::ExecutionRoom => 8,

            // Late game
            Self::DuelChamber | Self::MirrorMaze | Self::CrushingWalls | Self::PoisonChamber => 12,
            Self::EnchantmentShop | Self::CurseShrine | Self::SacrificeAltar | Self::SecretStudy => 12,
            Self::ElementalForge | Self::PoisonGarden | Self::AbandonedThrone => 12,

            // End game
            Self::DragonHoard | Self::TimedChallenge | Self::BossLair | Self::ThroneRoom => 15,
            Self::BloodAltar | Self::DarkAltar | Self::LegendaryAnvil => 15,
            Self::CultistThrone | Self::GoblinKingCourt | Self::NecromancerDen => 15,

            // Very late game
            Self::WeaponCache | Self::ElementalAltar | Self::AncientAltar => 18,
            Self::DimensionalRift | Self::EscapedPrisoner => 18,

            // Special
            Self::Laboratory | Self::Observatory | Self::Crypt | Self::Ossuary => 10,
            Self::Treasury | Self::MessHall | Self::Workshop | Self::StairsNexus => 5,
            Self::WaypointShrine => 1,
        }
    }

    /// Returns the spawn weight for this room type at a given level
    pub fn spawn_weight(&self, level: u32, theme: DungeonTheme) -> f64 {
        if level < self.min_level() {
            return 0.0;
        }

        let base_weight = match self {
            // Common rooms
            Self::StandardCombat => 0.25,
            Self::GuardPost | Self::Storage => 0.15,
            Self::Barracks | Self::MessHall => 0.12,

            // Uncommon rooms
            Self::TreasureVault | Self::GoldCache => 0.08,
            Self::PuzzleChamber | Self::TrapGauntlet => 0.07,
            Self::AncientLibrary | Self::SmithyForge => 0.06,

            // Rare rooms
            Self::ArenaRoom | Self::MiniBossRoom => 0.04,
            Self::DragonHoard | Self::ArtifactRoom => 0.03,
            Self::BlackMarket | Self::EnchantmentShop => 0.04,

            // Very rare rooms
            Self::BossLair | Self::ThroneRoom => 0.02,
            Self::DimensionalRift | Self::LegendaryAnvil => 0.01,

            // Safe rooms (always have decent chance)
            Self::SafeHaven | Self::Campsite | Self::MerchantCamp => 0.06,
            Self::HealingFountain | Self::ManaWell => 0.05,

            _ => 0.05, // Default for unspecified
        };

        // Apply theme modifiers
        let theme_modifier = match (self, theme) {
            // Dungeon theme bonuses
            (Self::DungeonCell | Self::TortureChamber | Self::Storage, DungeonTheme::Dungeon) => 1.5,
            (Self::GuardPost | Self::Barracks, DungeonTheme::Dungeon) => 1.3,

            // Cave theme bonuses
            (Self::MushroomGarden | Self::CrystalGarden, DungeonTheme::Cave) => 2.0,
            (Self::HordeRoom, DungeonTheme::Cave) => 1.5,

            // Crypt theme bonuses
            (Self::Crypt | Self::Ossuary, DungeonTheme::Crypt) => 2.0,
            (Self::NecromancerDen | Self::DarkAltar, DungeonTheme::Crypt) => 1.8,
            (Self::SacrificeAltar | Self::BloodAltar, DungeonTheme::Crypt) => 1.5,

            // Forest theme bonuses
            (Self::HerbGarden | Self::MushroomGarden, DungeonTheme::Forest) => 2.0,
            (Self::MeditationRoom | Self::SafeHaven, DungeonTheme::Forest) => 1.3,

            // Ice theme bonuses
            (Self::CrystalGarden, DungeonTheme::IceCavern) => 2.0,
            (Self::HotSpring, DungeonTheme::IceCavern) => 1.5,

            // Volcanic theme bonuses
            (Self::ElementalForge | Self::SmithyForge, DungeonTheme::VolcanicLair) => 2.0,
            (Self::ElementalAltar, DungeonTheme::VolcanicLair) => 1.5,

            // Ancient Ruins bonuses
            (Self::AncientLibrary | Self::ScrollArchive | Self::TomeVault, DungeonTheme::AncientRuins) => 2.0,
            (Self::AncientAltar | Self::AbandonedThrone, DungeonTheme::AncientRuins) => 1.8,

            // Demon Realm bonuses
            (Self::DarkAltar | Self::BloodAltar | Self::SacrificeAltar, DungeonTheme::DemonRealm) => 2.0,
            (Self::BossLair | Self::ThroneRoom, DungeonTheme::DemonRealm) => 1.5,

            _ => 1.0,
        };

        base_weight * theme_modifier
    }

    /// Returns enemies appropriate for this room type
    pub fn appropriate_enemies(&self, level: u32) -> Vec<EnemyKind> {
        let base_enemies: Vec<EnemyKind> = match self {
            Self::StandardCombat | Self::AmbushRoom | Self::HordeRoom => {
                vec![] // Use level-based enemies
            }
            Self::GuardPost | Self::Barracks => match level {
                1..=8 => vec![EnemyKind::Goblin, EnemyKind::Hobgoblin, EnemyKind::Orc],
                9..=16 => vec![EnemyKind::DeathKnight, EnemyKind::FrozenKnight],
                _ => vec![EnemyKind::DoomGuard, EnemyKind::AncientGuardian],
            },
            Self::NecromancerDen | Self::Crypt | Self::Ossuary => {
                vec![
                    EnemyKind::Skeleton,
                    EnemyKind::Zombie,
                    EnemyKind::Ghost,
                    EnemyKind::Wraith,
                    EnemyKind::Lich,
                ]
            }
            Self::MushroomGarden => vec![EnemyKind::Mushroom, EnemyKind::Slime],
            Self::PoisonGarden => vec![EnemyKind::Spider, EnemyKind::GiantSpider, EnemyKind::VenomousVine],
            _ => vec![], // Safe rooms or use default
        };
        base_enemies
    }
}

// ============================================================================
// Room Shapes
// ============================================================================

/// Different shapes a room can take
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum RoomShape {
    /// Standard rectangular room
    Rectangle,
    /// Circular room
    Circle,
    /// L-shaped room (corner)
    LShape,
    /// T-shaped room (three branches)
    TShape,
    /// Cross-shaped room (four branches)
    Cross,
    /// Irregular/organic shape
    Irregular,
    /// Cave-like with rounded edges
    CaveLike,
    /// Octagonal room
    Octagon,
    /// Diamond-shaped room
    Diamond,
    /// Donut-shaped (hollow center)
    Donut,
}

impl RoomShape {
    /// Returns a random shape with appropriate weights
    pub fn random(rng: &mut impl Rng, theme: DungeonTheme) -> Self {
        let weights: Vec<(Self, f64)> = match theme {
            DungeonTheme::Cave => vec![
                (Self::Rectangle, 0.15),
                (Self::Circle, 0.20),
                (Self::Irregular, 0.30),
                (Self::CaveLike, 0.35),
            ],
            DungeonTheme::Dungeon | DungeonTheme::Crypt => vec![
                (Self::Rectangle, 0.50),
                (Self::LShape, 0.15),
                (Self::TShape, 0.10),
                (Self::Cross, 0.08),
                (Self::Octagon, 0.12),
                (Self::Donut, 0.05),
            ],
            DungeonTheme::AncientRuins => vec![
                (Self::Rectangle, 0.25),
                (Self::Circle, 0.15),
                (Self::Octagon, 0.20),
                (Self::Diamond, 0.15),
                (Self::Cross, 0.15),
                (Self::Irregular, 0.10),
            ],
            _ => vec![
                (Self::Rectangle, 0.40),
                (Self::Circle, 0.15),
                (Self::LShape, 0.12),
                (Self::TShape, 0.08),
                (Self::Cross, 0.08),
                (Self::Irregular, 0.10),
                (Self::CaveLike, 0.07),
            ],
        };

        let total: f64 = weights.iter().map(|(_, w)| w).sum();
        let mut roll = rng.gen::<f64>() * total;

        for (shape, weight) in weights {
            roll -= weight;
            if roll <= 0.0 {
                return shape;
            }
        }

        Self::Rectangle
    }

    /// Returns minimum size multiplier for this shape
    pub fn size_multiplier(&self) -> f64 {
        match self {
            Self::Rectangle => 1.0,
            Self::Circle => 1.2,
            Self::LShape | Self::TShape => 1.3,
            Self::Cross => 1.5,
            Self::Irregular | Self::CaveLike => 1.1,
            Self::Octagon => 1.15,
            Self::Diamond => 1.2,
            Self::Donut => 1.8,
        }
    }
}

// ============================================================================
// Special Features
// ============================================================================

/// Special dungeon features that can appear in or between rooms
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SpecialFeature {
    /// A room that doesn't appear on the map until discovered
    HiddenRoom {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        revealed: bool,
    },
    /// A secret passage connecting two points
    SecretPassage {
        start: (usize, usize),
        end: (usize, usize),
        revealed: bool,
    },
    /// A wall that can be destroyed
    DestructibleWall {
        positions: Vec<(usize, usize)>,
        hp: i32,
        destroyed: bool,
    },
    /// A pressure plate that triggers something
    PressurePlate {
        position: (usize, usize),
        triggered: bool,
        effect: PressurePlateEffect,
    },
    /// A locked door requiring a specific key
    LockedDoor {
        position: (usize, usize),
        key_type: KeyType,
        locked: bool,
    },
    /// A teleporter pad
    Teleporter {
        position: (usize, usize),
        destination: (usize, usize),
        bidirectional: bool,
        active: bool,
    },
    /// An elevator between floors
    Elevator {
        position: (usize, usize),
        floors: Vec<u32>,
        current_floor: u32,
    },
    /// A lever that affects the dungeon
    Lever {
        position: (usize, usize),
        pulled: bool,
        effect: LeverEffect,
    },
    /// A treasure chest with traps
    TrappedChest {
        position: (usize, usize),
        trap_type: ChestTrap,
        opened: bool,
        disarmed: bool,
    },
    /// A magical barrier
    MagicBarrier {
        positions: Vec<(usize, usize)>,
        active: bool,
        trigger: BarrierTrigger,
    },
}

/// Effects triggered by pressure plates
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum PressurePlateEffect {
    OpenDoor,
    CloseDoor,
    ActivateTrap,
    SummonEnemies,
    RevealHiddenRoom,
    ToggleLights,
    ReleasePoisonGas,
    SpikeTrap,
    ArrowTrap,
    AlarmBell,
}

/// Effects triggered by levers
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum LeverEffect {
    OpenGate,
    DrainWater,
    ExtendBridge,
    RotateRoom,
    DisableTraps,
    LowerSpikes,
    OpenSecretDoor,
    ActivateElevator,
    SealRoom,
    ReleaseMonster,
}

/// Types of keys for locked doors
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum KeyType {
    Bronze,
    Silver,
    Gold,
    Ruby,
    Sapphire,
    Emerald,
    Diamond,
    Obsidian,
    Crystal,
    Skeleton,
    Master,
    Boss,
}

impl KeyType {
    /// Returns the display name of this key type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bronze => "Bronze Key",
            Self::Silver => "Silver Key",
            Self::Gold => "Gold Key",
            Self::Ruby => "Ruby Key",
            Self::Sapphire => "Sapphire Key",
            Self::Emerald => "Emerald Key",
            Self::Diamond => "Diamond Key",
            Self::Obsidian => "Obsidian Key",
            Self::Crystal => "Crystal Key",
            Self::Skeleton => "Skeleton Key",
            Self::Master => "Master Key",
            Self::Boss => "Boss Key",
        }
    }

    /// Returns the minimum level this key type appears
    pub fn min_level(&self) -> u32 {
        match self {
            Self::Bronze => 1,
            Self::Silver => 3,
            Self::Gold => 5,
            Self::Ruby | Self::Sapphire | Self::Emerald => 10,
            Self::Diamond | Self::Obsidian => 15,
            Self::Crystal => 20,
            Self::Skeleton => 8,
            Self::Master => 25,
            Self::Boss => 5, // Appears on boss levels
        }
    }

    /// Returns appropriate key type for level
    pub fn for_level(level: u32, rng: &mut impl Rng) -> Self {
        let keys: Vec<Self> = match level {
            1..=2 => vec![Self::Bronze],
            3..=4 => vec![Self::Bronze, Self::Silver],
            5..=9 => vec![Self::Silver, Self::Gold, Self::Skeleton],
            10..=14 => vec![Self::Gold, Self::Ruby, Self::Sapphire, Self::Emerald],
            15..=19 => vec![Self::Ruby, Self::Sapphire, Self::Diamond, Self::Obsidian],
            20..=24 => vec![Self::Diamond, Self::Obsidian, Self::Crystal],
            _ => vec![Self::Crystal, Self::Master],
        };
        keys[rng.gen_range(0..keys.len())]
    }
}

/// Types of traps on chests
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ChestTrap {
    None,
    Poison,
    Explosion,
    Dart,
    Gas,
    Curse,
    Mimic,
    Alarm,
    Lightning,
    Petrification,
}

impl ChestTrap {
    /// Returns a random trap appropriate for level
    pub fn for_level(level: u32, rng: &mut impl Rng) -> Self {
        if rng.gen_bool(0.3) {
            return Self::None;
        }

        let traps: Vec<Self> = match level {
            1..=4 => vec![Self::Dart, Self::Poison],
            5..=9 => vec![Self::Dart, Self::Poison, Self::Gas, Self::Alarm],
            10..=14 => vec![Self::Poison, Self::Explosion, Self::Mimic, Self::Curse],
            15..=19 => vec![Self::Explosion, Self::Mimic, Self::Curse, Self::Lightning],
            _ => vec![Self::Mimic, Self::Curse, Self::Lightning, Self::Petrification],
        };
        traps[rng.gen_range(0..traps.len())]
    }
}

/// Triggers for magical barriers
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum BarrierTrigger {
    KillAllEnemies,
    SolvePuzzle,
    ActivateLever,
    UseKey(KeyType),
    PayGold(u32),
    SacrificeHealth(i32),
    TimeBased(u32),
}

// ============================================================================
// Environmental Hazards
// ============================================================================

/// Environmental hazards that can be placed in dungeons
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EnvironmentalHazard {
    /// A pool of lava that deals fire damage
    LavaPit {
        positions: Vec<(usize, usize)>,
        damage_per_turn: i32,
        spreading: bool,
    },
    /// Spike trap that can be triggered
    SpikeTrap {
        position: (usize, usize),
        damage: i32,
        triggered: bool,
        resets: bool,
    },
    /// Area filled with poison gas
    PoisonGas {
        positions: Vec<(usize, usize)>,
        damage_per_turn: i32,
        duration: u32,
        spreading: bool,
    },
    /// Floor that crumbles when stepped on
    CrumblingFloor {
        positions: Vec<(usize, usize)>,
        steps_until_collapse: HashMap<(usize, usize), u8>,
    },
    /// Water that requires swimming
    WaterArea {
        positions: Vec<(usize, usize)>,
        depth: WaterDepth,
        has_current: bool,
        current_direction: Option<(i32, i32)>,
    },
    /// Slippery ice that affects movement
    IceArea {
        positions: Vec<(usize, usize)>,
        slide_distance: usize,
    },
    /// Area of magical darkness
    DarknessZone {
        positions: Vec<(usize, usize)>,
        radius: usize,
        blocks_all_light: bool,
    },
    /// Fire that spreads and damages
    FireHazard {
        positions: Vec<(usize, usize)>,
        damage_per_turn: i32,
        spread_chance: f64,
    },
    /// Acid pool that destroys items
    AcidPool {
        positions: Vec<(usize, usize)>,
        damage_per_turn: i32,
        destroys_items: bool,
    },
    /// Electric field that stuns
    ElectricField {
        positions: Vec<(usize, usize)>,
        damage: i32,
        stun_duration: u32,
    },
    /// Wind that pushes entities
    WindCurrent {
        positions: Vec<(usize, usize)>,
        direction: (i32, i32),
        strength: u32,
    },
    /// Quicksand that slows and traps
    Quicksand {
        positions: Vec<(usize, usize)>,
        escape_difficulty: u32,
    },
    /// Spore clouds from fungi
    SporeCloud {
        position: (usize, usize),
        radius: usize,
        effect: SporeEffect,
    },
}

/// Depth of water areas
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum WaterDepth {
    Shallow,   // Slows movement
    Medium,    // Requires swimming
    Deep,      // Can drown
}

/// Effects from spore clouds
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SporeEffect {
    Poison,
    Confusion,
    Sleep,
    Hallucination,
    Healing,
    Rage,
}

// ============================================================================
// Theme-Specific Tile Sets
// ============================================================================

/// Theme-specific tile palette for dungeon generation
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThemeTileSet {
    pub floor_tiles: Vec<(Tile, f64)>,
    pub wall_decoration: Vec<(Tile, f64)>,
    pub hazards: Vec<(Tile, f64)>,
    pub features: Vec<(Tile, f64)>,
    pub ambient_light: f32,
    pub fog_density: f32,
}

impl ThemeTileSet {
    /// Get tileset for a specific theme
    pub fn for_theme(theme: DungeonTheme) -> Self {
        match theme {
            DungeonTheme::Dungeon => Self {
                floor_tiles: vec![
                    (Tile::Floor, 0.85),
                    (Tile::Bones, 0.05),
                    (Tile::Cobweb, 0.05),
                    (Tile::Rubble, 0.05),
                ],
                wall_decoration: vec![
                    (Tile::Cobweb, 0.3),
                    (Tile::Brazier, 0.2),
                    (Tile::UnlitBrazier, 0.1),
                    (Tile::ArmorStand, 0.2),
                    (Tile::WeaponRack, 0.2),
                ],
                hazards: vec![(Tile::Trap, 0.6), (Tile::SpikeTrap, 0.3), (Tile::ArrowTrap, 0.1)],
                features: vec![
                    (Tile::Chest, 0.3),
                    (Tile::Pillar, 0.3),
                    (Tile::Door, 0.2),
                    (Tile::SecretDoor, 0.1),
                    (Tile::LockedChest, 0.1),
                ],
                ambient_light: 0.4,
                fog_density: 0.2,
            },
            DungeonTheme::Cave => Self {
                floor_tiles: vec![
                    (Tile::Floor, 0.70),
                    (Tile::Rubble, 0.10),
                    (Tile::Mushrooms, 0.10),
                    (Tile::GlowingMushrooms, 0.05),
                    (Tile::Water, 0.05),
                ],
                wall_decoration: vec![
                    (Tile::Vines, 0.3),
                    (Tile::Mushrooms, 0.25),
                    (Tile::GlowingMushrooms, 0.15),
                    (Tile::CrystalFormation, 0.15),
                    (Tile::Rubble, 0.15),
                ],
                hazards: vec![
                    (Tile::Water, 0.4),
                    (Tile::CrumblingFloor, 0.3),
                    (Tile::SporeColony, 0.2),
                    (Tile::PoisonGas, 0.1),
                ],
                features: vec![
                    (Tile::CrystalFormation, 0.3),
                    (Tile::MineCart, 0.1),
                    (Tile::Rails, 0.1),
                    (Tile::Chest, 0.3),
                    (Tile::GlowingMushrooms, 0.2),
                ],
                ambient_light: 0.2,
                fog_density: 0.3,
            },
            DungeonTheme::Crypt => Self {
                floor_tiles: vec![
                    (Tile::Floor, 0.70),
                    (Tile::Bones, 0.15),
                    (Tile::Cobweb, 0.10),
                    (Tile::BloodStain, 0.05),
                ],
                wall_decoration: vec![
                    (Tile::Cobweb, 0.25),
                    (Tile::Bones, 0.20),
                    (Tile::Sarcophagus, 0.20),
                    (Tile::Statue, 0.15),
                    (Tile::CrackedStatue, 0.10),
                    (Tile::UnlitBrazier, 0.10),
                ],
                hazards: vec![
                    (Tile::PoisonGas, 0.3),
                    (Tile::Trap, 0.3),
                    (Tile::CrumblingFloor, 0.2),
                    (Tile::AcidPool, 0.2),
                ],
                features: vec![
                    (Tile::Sarcophagus, 0.25),
                    (Tile::Altar, 0.20),
                    (Tile::Chest, 0.20),
                    (Tile::MagicCircle, 0.15),
                    (Tile::LockedChest, 0.20),
                ],
                ambient_light: 0.15,
                fog_density: 0.4,
            },
            DungeonTheme::Forest => Self {
                floor_tiles: vec![
                    (Tile::Grass, 0.70),
                    (Tile::Floor, 0.10),
                    (Tile::Vines, 0.10),
                    (Tile::Mushrooms, 0.05),
                    (Tile::Water, 0.05),
                ],
                wall_decoration: vec![
                    (Tile::Vines, 0.40),
                    (Tile::Mushrooms, 0.20),
                    (Tile::GlowingMushrooms, 0.10),
                    (Tile::Statue, 0.15),
                    (Tile::Bookshelf, 0.15),
                ],
                hazards: vec![
                    (Tile::Water, 0.3),
                    (Tile::PoisonTrap, 0.2),
                    (Tile::SporeColony, 0.3),
                    (Tile::Trap, 0.2),
                ],
                features: vec![
                    (Tile::Fountain, 0.25),
                    (Tile::Shrine, 0.20),
                    (Tile::Chest, 0.25),
                    (Tile::MagicCircle, 0.15),
                    (Tile::HealingShrine, 0.15),
                ],
                ambient_light: 0.6,
                fog_density: 0.15,
            },
            DungeonTheme::IceCavern => Self {
                floor_tiles: vec![
                    (Tile::Ice, 0.60),
                    (Tile::FrozenGround, 0.20),
                    (Tile::Floor, 0.15),
                    (Tile::Water, 0.05),
                ],
                wall_decoration: vec![
                    (Tile::CrystalFormation, 0.40),
                    (Tile::Statue, 0.15),
                    (Tile::CrackedStatue, 0.15),
                    (Tile::Pillar, 0.30),
                ],
                hazards: vec![
                    (Tile::Ice, 0.4),
                    (Tile::Water, 0.2),
                    (Tile::SpikeTrap, 0.2),
                    (Tile::CrumblingFloor, 0.2),
                ],
                features: vec![
                    (Tile::CrystalFormation, 0.35),
                    (Tile::Chest, 0.25),
                    (Tile::LockedChest, 0.15),
                    (Tile::Fountain, 0.15),
                    (Tile::BrokenFountain, 0.10),
                ],
                ambient_light: 0.5,
                fog_density: 0.25,
            },
            DungeonTheme::VolcanicLair => Self {
                floor_tiles: vec![
                    (Tile::ScorchedEarth, 0.50),
                    (Tile::Floor, 0.30),
                    (Tile::Lava, 0.10),
                    (Tile::Rubble, 0.10),
                ],
                wall_decoration: vec![
                    (Tile::Brazier, 0.35),
                    (Tile::UnlitBrazier, 0.10),
                    (Tile::Rubble, 0.25),
                    (Tile::CrystalFormation, 0.15),
                    (Tile::Bones, 0.15),
                ],
                hazards: vec![
                    (Tile::Lava, 0.5),
                    (Tile::FireTrap, 0.25),
                    (Tile::ScorchedEarth, 0.15),
                    (Tile::CrumblingFloor, 0.1),
                ],
                features: vec![
                    (Tile::Chest, 0.25),
                    (Tile::GemDeposit, 0.20),
                    (Tile::GoldPile, 0.15),
                    (Tile::Altar, 0.20),
                    (Tile::MagicCircle, 0.20),
                ],
                ambient_light: 0.7,
                fog_density: 0.35,
            },
            DungeonTheme::AncientRuins => Self {
                floor_tiles: vec![
                    (Tile::Sand, 0.50),
                    (Tile::Floor, 0.30),
                    (Tile::Rubble, 0.15),
                    (Tile::Grass, 0.05),
                ],
                wall_decoration: vec![
                    (Tile::Statue, 0.25),
                    (Tile::CrackedStatue, 0.20),
                    (Tile::Pillar, 0.20),
                    (Tile::Bookshelf, 0.15),
                    (Tile::Rubble, 0.20),
                ],
                hazards: vec![
                    (Tile::Trap, 0.3),
                    (Tile::SpikeTrap, 0.2),
                    (Tile::CrumblingFloor, 0.3),
                    (Tile::PoisonGas, 0.2),
                ],
                features: vec![
                    (Tile::Chest, 0.20),
                    (Tile::LockedChest, 0.15),
                    (Tile::Altar, 0.20),
                    (Tile::MagicCircle, 0.20),
                    (Tile::Sarcophagus, 0.15),
                    (Tile::Bookshelf, 0.10),
                ],
                ambient_light: 0.45,
                fog_density: 0.2,
            },
            DungeonTheme::DemonRealm => Self {
                floor_tiles: vec![
                    (Tile::ScorchedEarth, 0.40),
                    (Tile::Floor, 0.25),
                    (Tile::BloodStain, 0.15),
                    (Tile::Lava, 0.10),
                    (Tile::Bones, 0.10),
                ],
                wall_decoration: vec![
                    (Tile::Brazier, 0.30),
                    (Tile::Bones, 0.25),
                    (Tile::Statue, 0.15),
                    (Tile::CrackedStatue, 0.15),
                    (Tile::Altar, 0.15),
                ],
                hazards: vec![
                    (Tile::Lava, 0.35),
                    (Tile::FireTrap, 0.20),
                    (Tile::PoisonGas, 0.20),
                    (Tile::AcidPool, 0.15),
                    (Tile::PoisonTrap, 0.10),
                ],
                features: vec![
                    (Tile::Altar, 0.25),
                    (Tile::MagicCircle, 0.25),
                    (Tile::Chest, 0.15),
                    (Tile::LockedChest, 0.15),
                    (Tile::GemDeposit, 0.10),
                    (Tile::GoldPile, 0.10),
                ],
                ambient_light: 0.3,
                fog_density: 0.5,
            },
        }
    }

    /// Pick a random floor tile from the set
    pub fn random_floor(&self, rng: &mut impl Rng) -> Tile {
        self.weighted_choice(&self.floor_tiles, rng)
    }

    /// Pick a random wall decoration
    pub fn random_wall_decoration(&self, rng: &mut impl Rng) -> Tile {
        self.weighted_choice(&self.wall_decoration, rng)
    }

    /// Pick a random hazard
    pub fn random_hazard(&self, rng: &mut impl Rng) -> Tile {
        self.weighted_choice(&self.hazards, rng)
    }

    /// Pick a random feature
    pub fn random_feature(&self, rng: &mut impl Rng) -> Tile {
        self.weighted_choice(&self.features, rng)
    }

    fn weighted_choice(&self, choices: &[(Tile, f64)], rng: &mut impl Rng) -> Tile {
        let total: f64 = choices.iter().map(|(_, w)| w).sum();
        let mut roll = rng.gen::<f64>() * total;

        for (tile, weight) in choices {
            roll -= weight;
            if roll <= 0.0 {
                return *tile;
            }
        }

        choices[0].0
    }
}

// ============================================================================
// Procedural Events
// ============================================================================

/// Random events that can occur during dungeon exploration
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ProceduralEvent {
    /// Random enemy encounter
    RandomEncounter {
        enemies: Vec<EnemyKind>,
        ambush: bool,
    },
    /// Wandering monster appears
    WanderingMonster {
        enemy: EnemyKind,
        patrol_path: Vec<(usize, usize)>,
    },
    /// Dynamic spawn point activates
    DynamicSpawn {
        position: (usize, usize),
        enemy_type: EnemyKind,
        spawn_rate: u32,
        max_spawns: u32,
        spawned: u32,
    },
    /// Treasure appears
    TreasureDiscovery {
        position: (usize, usize),
        items: Vec<Item>,
        guarded: bool,
    },
    /// Environmental change
    EnvironmentChange {
        change_type: EnvironmentChangeType,
        affected_tiles: Vec<(usize, usize)>,
    },
    /// NPC encounter
    NPCEncounter {
        npc_type: NPCEventType,
        position: (usize, usize),
        friendly: bool,
    },
    /// Trap activation
    TrapTriggered {
        position: (usize, usize),
        trap_type: TrapEventType,
        damage: i32,
    },
    /// Blessing or curse
    DivineIntervention {
        beneficial: bool,
        effect: DivineEffect,
    },
    /// Merchant caravan
    MerchantCaravan {
        position: (usize, usize),
        items_for_sale: Vec<Item>,
        gold_discount: f32,
    },
    /// Portal opens
    PortalOpens {
        position: (usize, usize),
        destination: PortalDestination,
        duration: u32,
    },
    /// Boss awakens early
    BossAwakens {
        boss_type: EnemyKind,
        warning_message: String,
    },
    /// Earthquake
    Earthquake {
        intensity: u32,
        affected_rooms: Vec<usize>,
        new_passages: Vec<((usize, usize), (usize, usize))>,
    },
}

/// Types of environment changes
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum EnvironmentChangeType {
    FloodWithWater,
    LavaRising,
    CaveIn,
    PoisonGasRelease,
    LightsOut,
    FireSpreading,
    IceSpreading,
    VegetationGrowth,
}

/// Types of NPCs in events
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum NPCEventType {
    WanderingMerchant,
    LostAdventurer,
    TrapDisarmer,
    Healer,
    MysteriousStranger,
    GhostlySpirit,
    RivalAdventurer,
    BountyHunter,
}

/// Types of traps in events
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum TrapEventType {
    PoisonDart,
    FallingRocks,
    PitTrap,
    FireBlast,
    IceBlast,
    LightningBolt,
    Teleport,
    Confusion,
}

/// Divine effects from interventions
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum DivineEffect {
    FullHeal,
    StatBoost,
    TemporaryInvincibility,
    Teleport,
    RevealMap,
    SummonAlly,
    CurseWeapon,
    Petrification,
    Blindness,
    LevelDrain,
}

/// Portal destinations
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum PortalDestination {
    RandomRoom,
    BossRoom,
    TreasureRoom,
    SafeRoom,
    NextFloor,
    PreviousFloor,
    SecretArea,
}

// ============================================================================
// Enhanced Room Structure
// ============================================================================

/// Extended room with advanced features
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnhancedRoom {
    /// Base room data
    pub base: Room,
    /// Room type
    pub room_type: RoomType,
    /// Room shape
    pub shape: RoomShape,
    /// Special features in this room
    pub features: Vec<SpecialFeature>,
    /// Hazards in this room
    pub hazards: Vec<EnvironmentalHazard>,
    /// Whether this room is hidden
    pub hidden: bool,
    /// Light level (0.0 - 1.0)
    pub light_level: f32,
    /// Connected rooms (indices)
    pub connections: Vec<usize>,
    /// Enemies in this room
    pub enemy_slots: Vec<(usize, usize)>,
    /// Item spawn locations
    pub item_slots: Vec<(usize, usize)>,
    /// Whether the room has been visited
    pub visited: bool,
    /// Whether the room has been cleared
    pub cleared: bool,
    /// Room difficulty rating (1-10)
    pub difficulty: u8,
    /// Loot quality multiplier
    pub loot_multiplier: f32,
}

impl EnhancedRoom {
    /// Create a new enhanced room
    pub fn new(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        room_type: RoomType,
        shape: RoomShape,
    ) -> Self {
        let base = Room::new(x, y, width, height);

        Self {
            base,
            room_type,
            shape,
            features: Vec::new(),
            hazards: Vec::new(),
            hidden: false,
            light_level: 1.0,
            connections: Vec::new(),
            enemy_slots: Vec::new(),
            item_slots: Vec::new(),
            visited: false,
            cleared: false,
            difficulty: 1,
            loot_multiplier: 1.0,
        }
    }

    /// Check if position is inside the room
    pub fn contains(&self, x: usize, y: usize) -> bool {
        self.base.contains(x, y)
    }

    /// Get room center
    pub fn center(&self) -> (usize, usize) {
        self.base.center()
    }

    /// Calculate spawn positions based on shape
    pub fn calculate_spawn_positions(&mut self, rng: &mut impl Rng) {
        let positions = self.base.floor_positions();
        let center = self.center();

        // Filter positions based on shape constraints
        let valid_positions: Vec<_> = positions
            .into_iter()
            .filter(|&(x, y)| {
                // Avoid center for enemy spawns
                let dx = (x as i32 - center.0 as i32).abs();
                let dy = (y as i32 - center.1 as i32).abs();
                dx > 2 || dy > 2
            })
            .collect();

        // Select enemy spawn positions
        let enemy_count = match self.room_type {
            RoomType::HordeRoom => 8,
            RoomType::ArenaRoom | RoomType::BossLair => 1,
            RoomType::AmbushRoom => 6,
            RoomType::GuardPost | RoomType::Barracks => 4,
            _ if self.room_type.is_safe() => 0,
            _ => 3,
        };

        self.enemy_slots = valid_positions
            .choose_multiple(rng, enemy_count.min(valid_positions.len()))
            .copied()
            .collect();

        // Select item spawn positions
        let item_count = match self.room_type {
            RoomType::TreasureVault | RoomType::DragonHoard => 6,
            RoomType::GoldCache | RoomType::GemChamber => 4,
            RoomType::ArtifactRoom | RoomType::AncientLibrary => 3,
            _ => 1,
        };

        let remaining: Vec<_> = valid_positions
            .into_iter()
            .filter(|p| !self.enemy_slots.contains(p))
            .collect();

        self.item_slots = remaining
            .choose_multiple(rng, item_count.min(remaining.len()))
            .copied()
            .collect();
    }
}

// ============================================================================
// Main Dungeon Generator
// ============================================================================

/// Configuration for dungeon generation
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DungeonConfig {
    /// Target number of rooms
    pub target_rooms: usize,
    /// Minimum room size
    pub min_room_size: usize,
    /// Maximum room size
    pub max_room_size: usize,
    /// Chance of special room (0.0 - 1.0)
    pub special_room_chance: f64,
    /// Chance of hidden room (0.0 - 1.0)
    pub hidden_room_chance: f64,
    /// Chance of secret passage (0.0 - 1.0)
    pub secret_passage_chance: f64,
    /// Trap density (0.0 - 1.0)
    pub trap_density: f64,
    /// Hazard density (0.0 - 1.0)
    pub hazard_density: f64,
    /// Event frequency (turns between events)
    pub event_frequency: u32,
    /// Maximum corridor length
    pub max_corridor_length: usize,
    /// Door frequency (0.0 - 1.0)
    pub door_frequency: f64,
    /// Locked door frequency (0.0 - 1.0)
    pub locked_door_frequency: f64,
}

impl Default for DungeonConfig {
    fn default() -> Self {
        Self {
            target_rooms: MAX_ROOMS,
            min_room_size: MIN_ROOM_SIZE,
            max_room_size: MAX_ROOM_SIZE,
            special_room_chance: 0.25,
            hidden_room_chance: 0.10,
            secret_passage_chance: 0.15,
            trap_density: 0.08,
            hazard_density: 0.05,
            event_frequency: 50,
            max_corridor_length: MAX_CORRIDOR_LENGTH,
            door_frequency: 0.4,
            locked_door_frequency: 0.15,
        }
    }
}

impl DungeonConfig {
    /// Create config for specific difficulty level
    pub fn for_level(level: u32) -> Self {
        let mut config = Self::default();

        // Scale difficulty with level
        config.trap_density = 0.05 + (level as f64 * 0.01).min(0.15);
        config.hazard_density = 0.03 + (level as f64 * 0.008).min(0.12);
        config.special_room_chance = 0.20 + (level as f64 * 0.01).min(0.15);
        config.hidden_room_chance = 0.08 + (level as f64 * 0.005).min(0.08);
        config.locked_door_frequency = 0.10 + (level as f64 * 0.008).min(0.15);

        // Boss levels have more rooms
        if BOSS_LEVELS.contains(&level) {
            config.target_rooms += 3;
            config.special_room_chance += 0.1;
        }

        config
    }
}

/// Main dungeon generator struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DungeonGenerator {
    /// Current dungeon level
    pub level: u32,
    /// Dungeon theme
    pub theme: DungeonTheme,
    /// Generation configuration
    pub config: DungeonConfig,
    /// Generated enhanced rooms
    pub rooms: Vec<EnhancedRoom>,
    /// Special features
    pub features: Vec<SpecialFeature>,
    /// Environmental hazards
    pub hazards: Vec<EnvironmentalHazard>,
    /// Pending procedural events
    pub pending_events: Vec<ProceduralEvent>,
    /// Theme-specific tileset
    pub tileset: ThemeTileSet,
    /// Keys required for this level
    pub required_keys: HashSet<KeyType>,
    /// Whether boss room exists
    pub has_boss_room: bool,
    /// Spawn points for wandering monsters
    pub wandering_monster_spawns: Vec<(usize, usize)>,
    /// Current event timer
    pub event_timer: u32,
    /// Random seed used for generation
    pub seed: u64,
}

impl DungeonGenerator {
    /// Create a new dungeon generator for the given level
    pub fn new(level: u32) -> Self {
        let theme = DungeonTheme::from_level(level);
        let config = DungeonConfig::for_level(level);
        let tileset = ThemeTileSet::for_theme(theme);

        Self {
            level,
            theme,
            config,
            rooms: Vec::new(),
            features: Vec::new(),
            hazards: Vec::new(),
            pending_events: Vec::new(),
            tileset,
            required_keys: HashSet::new(),
            has_boss_room: BOSS_LEVELS.contains(&level),
            wandering_monster_spawns: Vec::new(),
            event_timer: 0,
            seed: 0,
        }
    }

    /// Generate a complete dungeon level
    pub fn generate(&mut self, rng: &mut impl Rng) -> Map {
        self.seed = rng.gen();
        let mut map = Map::new();
        map.theme = self.theme;

        // Phase 1: Generate rooms
        self.generate_rooms(rng);

        // Phase 2: Carve rooms into map
        self.carve_rooms(&mut map, rng);

        // Phase 3: Connect rooms with corridors
        self.connect_rooms(&mut map, rng);

        // Phase 4: Add special features
        self.add_special_features(&mut map, rng);

        // Phase 5: Add environmental hazards
        self.add_hazards(&mut map, rng);

        // Phase 6: Place stairs and important features
        self.place_stairs(&mut map, rng);

        // Phase 7: Generate procedural events
        self.generate_events(rng);

        // Phase 8: Add theme-specific decorations
        self.add_decorations(&mut map, rng);

        // Transfer room data to map
        for room in &self.rooms {
            map.rooms.push(room.base.clone());
        }

        map
    }

    /// Generate room layout
    fn generate_rooms(&mut self, rng: &mut impl Rng) {
        let mut attempts = 0;
        let max_attempts = self.config.target_rooms * 10;

        while self.rooms.len() < self.config.target_rooms && attempts < max_attempts {
            attempts += 1;

            // Determine room type
            let room_type = self.choose_room_type(rng);
            let shape = RoomShape::random(rng, self.theme);

            // Calculate size based on type and shape
            let (min_size, max_size) = self.size_for_room_type(room_type);
            let size_mult = shape.size_multiplier();

            let width = rng.gen_range(min_size..=max_size);
            let height = rng.gen_range(min_size..=max_size);
            let width = ((width as f64 * size_mult) as usize).min(MAX_ROOM_SIZE);
            let height = ((height as f64 * size_mult) as usize).min(MAX_ROOM_SIZE);

            // Find valid position
            let x = rng.gen_range(2..MAP_WIDTH.saturating_sub(width + 2));
            let y = rng.gen_range(2..MAP_HEIGHT.saturating_sub(height + 2));

            let new_room = EnhancedRoom::new(x, y, width, height, room_type, shape);

            // Check for overlap
            let overlaps = self.rooms.iter().any(|r| new_room.base.intersects(&r.base));

            if !overlaps {
                self.rooms.push(new_room);
            }
        }

        // Ensure boss room on boss levels
        if self.has_boss_room && !self.rooms.iter().any(|r| r.room_type == RoomType::BossLair) {
            if let Some(last_room) = self.rooms.last_mut() {
                last_room.room_type = RoomType::BossLair;
                last_room.difficulty = 10;
                last_room.loot_multiplier = 3.0;
            }
        }

        // Calculate spawn positions for all rooms
        for room in &mut self.rooms {
            room.calculate_spawn_positions(rng);
        }
    }

    /// Choose room type based on level and weights
    fn choose_room_type(&self, rng: &mut impl Rng) -> RoomType {
        let candidates: Vec<RoomType> = vec![
            RoomType::StandardCombat,
            RoomType::AmbushRoom,
            RoomType::ArenaRoom,
            RoomType::HordeRoom,
            RoomType::GuardPost,
            RoomType::TreasureVault,
            RoomType::GoldCache,
            RoomType::PuzzleChamber,
            RoomType::TrapGauntlet,
            RoomType::SafeHaven,
            RoomType::MerchantCamp,
            RoomType::BlessingShrine,
            RoomType::HealingFountain,
            RoomType::AncientLibrary,
            RoomType::SmithyForge,
            RoomType::Storage,
            RoomType::Barracks,
            RoomType::Crypt,
            RoomType::Laboratory,
        ];

        // Calculate weights
        let weights: Vec<f64> = candidates
            .iter()
            .map(|t| t.spawn_weight(self.level, self.theme))
            .collect();

        let total: f64 = weights.iter().sum();
        let mut roll = rng.gen::<f64>() * total;

        for (room_type, weight) in candidates.iter().zip(weights.iter()) {
            roll -= weight;
            if roll <= 0.0 {
                return *room_type;
            }
        }

        RoomType::StandardCombat
    }

    /// Get size constraints for room type
    fn size_for_room_type(&self, room_type: RoomType) -> (usize, usize) {
        match room_type {
            RoomType::BossLair | RoomType::ArenaRoom | RoomType::DragonHoard => (12, 18),
            RoomType::HordeRoom | RoomType::TreasureVault => (10, 15),
            RoomType::MerchantCamp | RoomType::SafeHaven => (8, 12),
            RoomType::DungeonCell | RoomType::Storage => (4, 7),
            _ => (self.config.min_room_size, self.config.max_room_size),
        }
    }

    /// Carve rooms into the map based on their shapes
    fn carve_rooms(&self, map: &mut Map, rng: &mut impl Rng) {
        for room in &self.rooms {
            match room.shape {
                RoomShape::Rectangle => self.carve_rectangle(map, room, rng),
                RoomShape::Circle => self.carve_circle(map, room, rng),
                RoomShape::LShape => self.carve_l_shape(map, room, rng),
                RoomShape::TShape => self.carve_t_shape(map, room, rng),
                RoomShape::Cross => self.carve_cross(map, room, rng),
                RoomShape::Irregular => self.carve_irregular(map, room, rng),
                RoomShape::CaveLike => self.carve_cave_like(map, room, rng),
                RoomShape::Octagon => self.carve_octagon(map, room, rng),
                RoomShape::Diamond => self.carve_diamond(map, room, rng),
                RoomShape::Donut => self.carve_donut(map, room, rng),
            }
        }
    }

    /// Carve a rectangular room
    fn carve_rectangle(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);

        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x..room.base.x + room.base.width {
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve a circular room
    fn carve_circle(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let (cx, cy) = room.center();
        let radius = room.base.width.min(room.base.height) / 2;
        let floor_tile = self.tileset.random_floor(rng);

        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x..room.base.x + room.base.width {
                let dx = x as i32 - cx as i32;
                let dy = y as i32 - cy as i32;
                let dist = ((dx * dx + dy * dy) as f64).sqrt();

                if dist <= radius as f64 && x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve an L-shaped room
    fn carve_l_shape(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);
        let half_w = room.base.width / 2;
        let half_h = room.base.height / 2;

        // Horizontal part
        for y in room.base.y..room.base.y + half_h {
            for x in room.base.x..room.base.x + room.base.width {
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }

        // Vertical part
        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x..room.base.x + half_w {
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve a T-shaped room
    fn carve_t_shape(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);
        let third_h = room.base.height / 3;
        let third_w = room.base.width / 3;

        // Horizontal bar (top)
        for y in room.base.y..room.base.y + third_h {
            for x in room.base.x..room.base.x + room.base.width {
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }

        // Vertical stem
        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x + third_w..room.base.x + third_w * 2 {
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve a cross-shaped room
    fn carve_cross(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);
        let third_h = room.base.height / 3;
        let third_w = room.base.width / 3;

        // Horizontal bar
        for y in room.base.y + third_h..room.base.y + third_h * 2 {
            for x in room.base.x..room.base.x + room.base.width {
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }

        // Vertical bar
        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x + third_w..room.base.x + third_w * 2 {
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve an irregular/organic room
    fn carve_irregular(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        // Start with rectangle then add/remove cells randomly
        self.carve_rectangle(map, room, rng);

        let (cx, cy) = room.center();

        // Add some random cells around edges
        for _ in 0..20 {
            let x = rng.gen_range(room.base.x.saturating_sub(1)..room.base.x + room.base.width + 1);
            let y = rng.gen_range(room.base.y.saturating_sub(1)..room.base.y + room.base.height + 1);

            if x > 0 && x < MAP_WIDTH - 1 && y > 0 && y < MAP_HEIGHT - 1 {
                // Check if adjacent to floor
                let has_floor_neighbor = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .any(|&(dx, dy)| {
                        let nx = (x as i32 + dx) as usize;
                        let ny = (y as i32 + dy) as usize;
                        map.tiles[ny][nx].walkable()
                    });

                if has_floor_neighbor && rng.gen_bool(0.6) {
                    map.tiles[y][x] = self.tileset.random_floor(rng);
                }
            }
        }

        // Remove some edge cells
        for _ in 0..10 {
            let x = rng.gen_range(room.base.x..room.base.x + room.base.width);
            let y = rng.gen_range(room.base.y..room.base.y + room.base.height);

            let dx = (x as i32 - cx as i32).abs();
            let dy = (y as i32 - cy as i32).abs();

            // Only remove cells far from center
            if (dx > 2 || dy > 2) && rng.gen_bool(0.3) {
                map.tiles[y][x] = Tile::Wall;
            }
        }
    }

    /// Carve a cave-like room using cellular automata
    fn carve_cave_like(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);

        // Initialize with random fill
        let mut cells: Vec<Vec<bool>> = vec![vec![false; room.base.width]; room.base.height];

        for row in cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell = rng.gen_bool(0.55);
            }
        }

        // Run cellular automata iterations
        for _ in 0..4 {
            let mut new_cells = cells.clone();

            for y in 1..room.base.height - 1 {
                for x in 1..room.base.width - 1 {
                    let mut neighbors = 0;
                    for dy in -1i32..=1 {
                        for dx in -1i32..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let ny = (y as i32 + dy) as usize;
                            let nx = (x as i32 + dx) as usize;
                            if cells[ny][nx] {
                                neighbors += 1;
                            }
                        }
                    }

                    new_cells[y][x] = neighbors >= 4;
                }
            }

            cells = new_cells;
        }

        // Carve the cave
        for (y, row) in cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let map_x = room.base.x + x;
                let map_y = room.base.y + y;

                if cell && map_x < MAP_WIDTH && map_y < MAP_HEIGHT {
                    map.tiles[map_y][map_x] = floor_tile;
                }
            }
        }

        // Ensure center is always carved
        let (cx, cy) = room.center();
        for dy in -2i32..=2 {
            for dx in -2i32..=2 {
                let x = (cx as i32 + dx) as usize;
                let y = (cy as i32 + dy) as usize;
                if x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve an octagonal room
    fn carve_octagon(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);
        let cut = room.base.width.min(room.base.height) / 4;

        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x..room.base.x + room.base.width {
                let local_x = x - room.base.x;
                let local_y = y - room.base.y;

                // Cut corners
                let top_left = local_x + local_y >= cut;
                let top_right = (room.base.width - 1 - local_x) + local_y >= cut;
                let bottom_left = local_x + (room.base.height - 1 - local_y) >= cut;
                let bottom_right =
                    (room.base.width - 1 - local_x) + (room.base.height - 1 - local_y) >= cut;

                if top_left && top_right && bottom_left && bottom_right {
                    if x < MAP_WIDTH && y < MAP_HEIGHT {
                        map.tiles[y][x] = floor_tile;
                    }
                }
            }
        }
    }

    /// Carve a diamond-shaped room
    fn carve_diamond(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);
        let (cx, cy) = room.center();
        let half_w = room.base.width / 2;
        let half_h = room.base.height / 2;

        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x..room.base.x + room.base.width {
                let dx = (x as i32 - cx as i32).abs() as f64 / half_w as f64;
                let dy = (y as i32 - cy as i32).abs() as f64 / half_h as f64;

                if dx + dy <= 1.0 && x < MAP_WIDTH && y < MAP_HEIGHT {
                    map.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve a donut-shaped room (hollow center)
    fn carve_donut(&self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let floor_tile = self.tileset.random_floor(rng);
        let (cx, cy) = room.center();
        let outer_radius = room.base.width.min(room.base.height) / 2;
        let inner_radius = outer_radius / 3;

        for y in room.base.y..room.base.y + room.base.height {
            for x in room.base.x..room.base.x + room.base.width {
                let dx = x as i32 - cx as i32;
                let dy = y as i32 - cy as i32;
                let dist = ((dx * dx + dy * dy) as f64).sqrt();

                if dist <= outer_radius as f64 && dist >= inner_radius as f64 {
                    if x < MAP_WIDTH && y < MAP_HEIGHT {
                        map.tiles[y][x] = floor_tile;
                    }
                }
            }
        }
    }

    /// Connect rooms with corridors
    fn connect_rooms(&mut self, map: &mut Map, rng: &mut impl Rng) {
        if self.rooms.len() < 2 {
            return;
        }

        let floor_tile = self.theme.floor_tile();

        // Connect each room to the next
        for i in 1..self.rooms.len() {
            let (c1_x, c1_y) = self.rooms[i - 1].center();
            let (c2_x, c2_y) = self.rooms[i].center();

            // Mark connection
            self.rooms[i].connections.push(i - 1);
            self.rooms[i - 1].connections.push(i);

            // Decide corridor style
            if rng.gen_bool(0.5) {
                self.carve_h_tunnel(map, c1_x, c2_x, c1_y, floor_tile);
                self.carve_v_tunnel(map, c1_y, c2_y, c2_x, floor_tile);
            } else {
                self.carve_v_tunnel(map, c1_y, c2_y, c1_x, floor_tile);
                self.carve_h_tunnel(map, c1_x, c2_x, c2_y, floor_tile);
            }

            // Add door at connection point
            if rng.gen_bool(self.config.door_frequency) {
                let door_x = if rng.gen_bool(0.5) { c1_x } else { c2_x };
                let door_y = if rng.gen_bool(0.5) { c1_y } else { c2_y };

                if map.tiles[door_y][door_x].walkable() {
                    if rng.gen_bool(self.config.locked_door_frequency) {
                        let key_type = KeyType::for_level(self.level, rng);
                        self.features.push(SpecialFeature::LockedDoor {
                            position: (door_x, door_y),
                            key_type,
                            locked: true,
                        });
                        self.required_keys.insert(key_type);
                        map.tiles[door_y][door_x] = Tile::VaultDoor;
                    } else {
                        map.tiles[door_y][door_x] = Tile::Door;
                    }
                }
            }
        }

        // Add some extra connections for loops
        let extra_connections = (self.rooms.len() / 5).max(1);
        for _ in 0..extra_connections {
            let i = rng.gen_range(0..self.rooms.len());
            let j = rng.gen_range(0..self.rooms.len());

            if i != j && !self.rooms[i].connections.contains(&j) {
                let (c1_x, c1_y) = self.rooms[i].center();
                let (c2_x, c2_y) = self.rooms[j].center();

                self.carve_h_tunnel(map, c1_x, c2_x, c1_y, floor_tile);
                self.carve_v_tunnel(map, c1_y, c2_y, c2_x, floor_tile);

                self.rooms[i].connections.push(j);
                self.rooms[j].connections.push(i);
            }
        }
    }

    /// Carve horizontal tunnel
    fn carve_h_tunnel(&self, map: &mut Map, x1: usize, x2: usize, y: usize, tile: Tile) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            if x < MAP_WIDTH && y < MAP_HEIGHT && map.tiles[y][x] == Tile::Wall {
                map.tiles[y][x] = tile;
            }
        }
    }

    /// Carve vertical tunnel
    fn carve_v_tunnel(&self, map: &mut Map, y1: usize, y2: usize, x: usize, tile: Tile) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            if x < MAP_WIDTH && y < MAP_HEIGHT && map.tiles[y][x] == Tile::Wall {
                map.tiles[y][x] = tile;
            }
        }
    }

    /// Add special features to dungeon
    fn add_special_features(&mut self, map: &mut Map, rng: &mut impl Rng) {
        // Add secret passages
        if rng.gen_bool(self.config.secret_passage_chance) && self.rooms.len() >= 2 {
            let i = rng.gen_range(0..self.rooms.len());
            let j = rng.gen_range(0..self.rooms.len());

            if i != j {
                let start = self.rooms[i].center();
                let end = self.rooms[j].center();

                self.features.push(SpecialFeature::SecretPassage {
                    start,
                    end,
                    revealed: false,
                });

                // Mark secret door
                if start.0 < MAP_WIDTH && start.1 < MAP_HEIGHT {
                    map.tiles[start.1][start.0] = Tile::SecretDoor;
                }
            }
        }

        // Add teleporters on later levels
        if self.level >= 5 && rng.gen_bool(0.2) && self.rooms.len() >= 3 {
            let i = rng.gen_range(0..self.rooms.len());
            let j = rng.gen_range(0..self.rooms.len());

            if i != j {
                let pos = self.rooms[i].center();
                let dest = self.rooms[j].center();

                self.features.push(SpecialFeature::Teleporter {
                    position: pos,
                    destination: dest,
                    bidirectional: rng.gen_bool(0.5),
                    active: true,
                });

                if pos.0 < MAP_WIDTH && pos.1 < MAP_HEIGHT {
                    map.tiles[pos.1][pos.0] = Tile::MagicCircle;
                }
                if dest.0 < MAP_WIDTH && dest.1 < MAP_HEIGHT {
                    map.tiles[dest.1][dest.0] = Tile::MagicCircle;
                }
            }
        }

        // Add pressure plates
        for room in &self.rooms {
            if rng.gen_bool(0.15) {
                let positions = room.base.floor_positions();
                if let Some(&pos) = positions.choose(rng) {
                    let effect = *[
                        PressurePlateEffect::OpenDoor,
                        PressurePlateEffect::ActivateTrap,
                        PressurePlateEffect::SpikeTrap,
                        PressurePlateEffect::AlarmBell,
                    ]
                    .choose(rng)
                    .unwrap();

                    self.features.push(SpecialFeature::PressurePlate {
                        position: pos,
                        triggered: false,
                        effect,
                    });

                    if pos.0 < MAP_WIDTH && pos.1 < MAP_HEIGHT {
                        map.tiles[pos.1][pos.0] = Tile::PuzzleTrigger;
                    }
                }
            }
        }

        // Add trapped chests
        for room in &self.rooms {
            if matches!(
                room.room_type,
                RoomType::TreasureVault | RoomType::GoldCache | RoomType::DragonHoard
            ) {
                for &pos in &room.item_slots {
                    if rng.gen_bool(0.4) {
                        let trap = ChestTrap::for_level(self.level, rng);
                        self.features.push(SpecialFeature::TrappedChest {
                            position: pos,
                            trap_type: trap,
                            opened: false,
                            disarmed: false,
                        });

                        if pos.0 < MAP_WIDTH && pos.1 < MAP_HEIGHT {
                            if matches!(trap, ChestTrap::Mimic) {
                                map.tiles[pos.1][pos.0] = Tile::MimicChest;
                            } else {
                                map.tiles[pos.1][pos.0] = Tile::LockedChest;
                            }
                        }
                    }
                }
            }
        }

        // Add levers
        for room in &self.rooms {
            if rng.gen_bool(0.1) {
                let perimeter = room.base.perimeter_positions();
                if let Some(&pos) = perimeter.choose(rng) {
                    let effect = *[
                        LeverEffect::OpenGate,
                        LeverEffect::DisableTraps,
                        LeverEffect::OpenSecretDoor,
                    ]
                    .choose(rng)
                    .unwrap();

                    self.features.push(SpecialFeature::Lever {
                        position: pos,
                        pulled: false,
                        effect,
                    });
                }
            }
        }
    }

    /// Add environmental hazards
    fn add_hazards(&mut self, map: &mut Map, rng: &mut impl Rng) {
        for room in &self.rooms {
            // Skip safe rooms
            if room.room_type.is_safe() {
                continue;
            }

            // Add hazards based on theme
            match self.theme {
                DungeonTheme::VolcanicLair | DungeonTheme::DemonRealm => {
                    if rng.gen_bool(self.config.hazard_density * 2.0) {
                        self.add_lava_hazard(map, room, rng);
                    }
                }
                DungeonTheme::IceCavern => {
                    if rng.gen_bool(self.config.hazard_density * 2.0) {
                        self.add_ice_hazard(map, room, rng);
                    }
                }
                DungeonTheme::Cave => {
                    if rng.gen_bool(self.config.hazard_density) {
                        self.add_water_hazard(map, room, rng);
                    }
                }
                DungeonTheme::Crypt => {
                    if rng.gen_bool(self.config.hazard_density) {
                        self.add_poison_hazard(map, room, rng);
                    }
                }
                _ => {}
            }

            // Add spike traps
            if matches!(room.room_type, RoomType::TrapGauntlet | RoomType::SpikePit) {
                self.add_spike_hazard(map, room, rng);
            }

            // Random hazards
            if rng.gen_bool(self.config.hazard_density) {
                let positions = room.base.floor_positions();
                if let Some(&pos) = positions.choose(rng) {
                    let hazard_tile = self.tileset.random_hazard(rng);
                    if pos.0 < MAP_WIDTH && pos.1 < MAP_HEIGHT {
                        map.tiles[pos.1][pos.0] = hazard_tile;
                    }
                }
            }
        }
    }

    fn add_lava_hazard(&mut self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let (cx, cy) = room.center();
        let size = rng.gen_range(2..=4);
        let mut positions = Vec::new();

        for dy in -(size as i32)..=(size as i32) {
            for dx in -(size as i32)..=(size as i32) {
                let x = (cx as i32 + dx) as usize;
                let y = (cy as i32 + dy) as usize;

                if x > 0 && x < MAP_WIDTH - 1 && y > 0 && y < MAP_HEIGHT - 1 {
                    if room.contains(x, y) && rng.gen_bool(0.7) {
                        map.tiles[y][x] = Tile::Lava;
                        positions.push((x, y));
                    }
                }
            }
        }

        self.hazards.push(EnvironmentalHazard::LavaPit {
            positions,
            damage_per_turn: 10 + self.level as i32,
            spreading: self.level >= 15,
        });
    }

    fn add_ice_hazard(&mut self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let positions: Vec<_> = room
            .base
            .floor_positions()
            .into_iter()
            .filter(|_| rng.gen_bool(0.4))
            .collect();

        for &(x, y) in &positions {
            if x < MAP_WIDTH && y < MAP_HEIGHT {
                map.tiles[y][x] = Tile::Ice;
            }
        }

        self.hazards.push(EnvironmentalHazard::IceArea {
            positions,
            slide_distance: 3,
        });
    }

    fn add_water_hazard(&mut self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let (cx, cy) = room.center();
        let size = rng.gen_range(2..=5);
        let mut positions = Vec::new();

        for dy in -(size as i32)..=(size as i32) {
            for dx in -(size as i32)..=(size as i32) {
                let x = (cx as i32 + dx) as usize;
                let y = (cy as i32 + dy) as usize;

                if x > 0 && x < MAP_WIDTH - 1 && y > 0 && y < MAP_HEIGHT - 1 {
                    if room.contains(x, y) && rng.gen_bool(0.6) {
                        map.tiles[y][x] = Tile::Water;
                        positions.push((x, y));
                    }
                }
            }
        }

        self.hazards.push(EnvironmentalHazard::WaterArea {
            positions,
            depth: WaterDepth::Shallow,
            has_current: rng.gen_bool(0.3),
            current_direction: if rng.gen_bool(0.3) {
                Some((rng.gen_range(-1..=1), rng.gen_range(-1..=1)))
            } else {
                None
            },
        });
    }

    fn add_poison_hazard(&mut self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let positions: Vec<_> = room
            .base
            .floor_positions()
            .into_iter()
            .filter(|_| rng.gen_bool(0.2))
            .take(6)
            .collect();

        for &(x, y) in &positions {
            if x < MAP_WIDTH && y < MAP_HEIGHT {
                map.tiles[y][x] = Tile::PoisonGas;
            }
        }

        self.hazards.push(EnvironmentalHazard::PoisonGas {
            positions,
            damage_per_turn: 3 + (self.level / 3) as i32,
            duration: 10,
            spreading: false,
        });
    }

    fn add_spike_hazard(&mut self, map: &mut Map, room: &EnhancedRoom, rng: &mut impl Rng) {
        let positions = room.base.floor_positions();
        let spike_count = (positions.len() / 4).max(3);

        for pos in positions.choose_multiple(rng, spike_count) {
            if pos.0 < MAP_WIDTH && pos.1 < MAP_HEIGHT {
                map.tiles[pos.1][pos.0] = Tile::SpikeTrap;

                self.hazards.push(EnvironmentalHazard::SpikeTrap {
                    position: *pos,
                    damage: 5 + (self.level / 2) as i32,
                    triggered: false,
                    resets: true,
                });
            }
        }
    }

    /// Place stairs and important navigation features
    fn place_stairs(&mut self, map: &mut Map, _rng: &mut impl Rng) {
        if self.rooms.is_empty() {
            return;
        }

        // Place stairs up in first room (if not level 1)
        if self.level > 1 {
            let first_room = &self.rooms[0];
            let (x, y) = first_room.center();
            if x < MAP_WIDTH && y < MAP_HEIGHT {
                map.tiles[y][x] = Tile::StairsUp;
            }
        }

        // Place stairs down (or boss gate) in last room
        if let Some(last_room) = self.rooms.last() {
            let (x, y) = last_room.center();
            if x < MAP_WIDTH && y < MAP_HEIGHT {
                if self.has_boss_room {
                    map.tiles[y][x] = Tile::BossGate;
                } else {
                    map.tiles[y][x] = Tile::StairsDown;
                }
            }
        }

        // Add waypoint shrines
        if self.level % 5 == 0 && self.rooms.len() >= 3 {
            let mid_room = &self.rooms[self.rooms.len() / 2];
            let (x, y) = mid_room.center();
            if x < MAP_WIDTH && y < MAP_HEIGHT && map.tiles[y][x].walkable() {
                map.tiles[y][x] = Tile::TeleportShrine;
            }
        }
    }

    /// Generate procedural events
    fn generate_events(&mut self, rng: &mut impl Rng) {
        // Wandering monsters
        let wanderer_count = 1 + (self.level / 5) as usize;
        for _ in 0..wanderer_count {
            if let Some(room) = self.rooms.choose(rng) {
                if !room.room_type.is_safe() {
                    let enemy = EnemyKind::for_level(self.level, rng);
                    let patrol: Vec<_> = self
                        .rooms
                        .iter()
                        .filter(|r| !r.room_type.is_safe())
                        .take(4)
                        .map(|r| r.center())
                        .collect();

                    self.pending_events.push(ProceduralEvent::WanderingMonster {
                        enemy,
                        patrol_path: patrol,
                    });

                    self.wandering_monster_spawns.push(room.center());
                }
            }
        }

        // Dynamic spawn points in horde rooms
        for room in &self.rooms {
            if room.room_type == RoomType::HordeRoom {
                let enemy = EnemyKind::for_level(self.level, rng);
                self.pending_events.push(ProceduralEvent::DynamicSpawn {
                    position: room.center(),
                    enemy_type: enemy,
                    spawn_rate: 10,
                    max_spawns: 5,
                    spawned: 0,
                });
            }
        }

        // Random encounter chance
        if rng.gen_bool(0.3) {
            let enemies: Vec<_> = (0..rng.gen_range(2..5))
                .map(|_| EnemyKind::for_level(self.level, rng))
                .collect();

            self.pending_events.push(ProceduralEvent::RandomEncounter {
                enemies,
                ambush: rng.gen_bool(0.3),
            });
        }

        // Merchant caravan
        if rng.gen_bool(0.15) && self.rooms.len() >= 3 {
            if let Some(room) = self.rooms.iter().find(|r| r.room_type.is_safe()) {
                self.pending_events.push(ProceduralEvent::MerchantCaravan {
                    position: room.center(),
                    items_for_sale: Vec::new(), // Populated later
                    gold_discount: rng.gen_range(0.0..0.2),
                });
            }
        }
    }

    /// Add theme-specific decorations
    fn add_decorations(&self, map: &mut Map, rng: &mut impl Rng) {
        for room in &self.rooms {
            // Add wall decorations on perimeter
            let perimeter = room.base.perimeter_positions();
            for &(x, y) in perimeter.iter().take(5) {
                if rng.gen_bool(0.15) && x < MAP_WIDTH && y < MAP_HEIGHT {
                    if map.tiles[y][x].walkable() {
                        let decoration = self.tileset.random_wall_decoration(rng);
                        map.tiles[y][x] = decoration;
                    }
                }
            }

            // Add features in room
            let floor_positions = room.base.floor_positions();
            for &(x, y) in floor_positions.iter().take(3) {
                if rng.gen_bool(0.08) && x < MAP_WIDTH && y < MAP_HEIGHT {
                    if map.tiles[y][x].walkable() && !map.tiles[y][x].is_shrine() {
                        let feature = self.tileset.random_feature(rng);
                        map.tiles[y][x] = feature;
                    }
                }
            }

            // Add pillars in larger rooms
            if room.base.width >= 10 && room.base.height >= 10 && rng.gen_bool(0.5) {
                let (cx, cy) = room.center();
                for &(dx, dy) in &[(-3i32, -3i32), (3, -3), (-3, 3), (3, 3)] {
                    let px = (cx as i32 + dx) as usize;
                    let py = (cy as i32 + dy) as usize;
                    if px < MAP_WIDTH && py < MAP_HEIGHT && map.tiles[py][px].walkable() {
                        map.tiles[py][px] = Tile::Pillar;
                    }
                }
            }
        }
    }

    /// Process a game tick for events
    pub fn tick(&mut self, rng: &mut impl Rng) -> Vec<ProceduralEvent> {
        self.event_timer += 1;
        let mut triggered_events = Vec::new();

        // Check for timed events
        if self.event_timer >= self.config.event_frequency {
            self.event_timer = 0;

            // Potentially trigger a random event
            if rng.gen_bool(0.3) && !self.pending_events.is_empty() {
                if let Some(event) = self.pending_events.pop() {
                    triggered_events.push(event);
                }
            }
        }

        triggered_events
    }

    /// Get spawn data for a specific room
    pub fn get_room_spawns(&self, room_index: usize) -> Option<&Vec<(usize, usize)>> {
        self.rooms.get(room_index).map(|r| &r.enemy_slots)
    }

    /// Mark a room as visited
    pub fn visit_room(&mut self, room_index: usize) {
        if let Some(room) = self.rooms.get_mut(room_index) {
            room.visited = true;
        }
    }

    /// Mark a room as cleared
    pub fn clear_room(&mut self, room_index: usize) {
        if let Some(room) = self.rooms.get_mut(room_index) {
            room.cleared = true;
        }
    }

    /// Find which room contains a position
    pub fn room_at(&self, x: usize, y: usize) -> Option<usize> {
        self.rooms.iter().position(|r| r.contains(x, y))
    }

    /// Get statistics about generated dungeon
    pub fn stats(&self) -> DungeonStats {
        DungeonStats {
            total_rooms: self.rooms.len(),
            special_rooms: self
                .rooms
                .iter()
                .filter(|r| !matches!(r.room_type, RoomType::StandardCombat))
                .count(),
            hidden_rooms: self.rooms.iter().filter(|r| r.hidden).count(),
            total_features: self.features.len(),
            total_hazards: self.hazards.len(),
            pending_events: self.pending_events.len(),
            required_keys: self.required_keys.len(),
            wandering_monsters: self.wandering_monster_spawns.len(),
        }
    }
}

/// Statistics about a generated dungeon
#[derive(Clone, Debug)]
pub struct DungeonStats {
    pub total_rooms: usize,
    pub special_rooms: usize,
    pub hidden_rooms: usize,
    pub total_features: usize,
    pub total_hazards: usize,
    pub pending_events: usize,
    pub required_keys: usize,
    pub wandering_monsters: usize,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dungeon_generation() {
        let mut rng = rand::thread_rng();
        let mut generator = DungeonGenerator::new(1);
        let map = generator.generate(&mut rng);

        assert!(!map.rooms.is_empty());
        assert!(!generator.rooms.is_empty());
    }

    #[test]
    fn test_room_shapes() {
        let mut rng = rand::thread_rng();

        for theme in &[
            DungeonTheme::Dungeon,
            DungeonTheme::Cave,
            DungeonTheme::Crypt,
        ] {
            let shape = RoomShape::random(&mut rng, *theme);
            assert!(shape.size_multiplier() >= 1.0);
        }
    }

    #[test]
    fn test_key_types() {
        let mut rng = rand::thread_rng();

        for level in [1, 5, 10, 15, 20, 25, 30] {
            let key = KeyType::for_level(level, &mut rng);
            assert!(key.min_level() <= level);
        }
    }

    #[test]
    fn test_theme_tilesets() {
        let mut rng = rand::thread_rng();

        for theme in &[
            DungeonTheme::Dungeon,
            DungeonTheme::Cave,
            DungeonTheme::Crypt,
            DungeonTheme::Forest,
            DungeonTheme::IceCavern,
            DungeonTheme::VolcanicLair,
            DungeonTheme::AncientRuins,
            DungeonTheme::DemonRealm,
        ] {
            let tileset = ThemeTileSet::for_theme(*theme);
            assert!(!tileset.floor_tiles.is_empty());
            assert!(!tileset.hazards.is_empty());

            // Test that tiles can be picked
            let _ = tileset.random_floor(&mut rng);
            let _ = tileset.random_hazard(&mut rng);
            let _ = tileset.random_feature(&mut rng);
        }
    }

    #[test]
    fn test_boss_levels() {
        for level in BOSS_LEVELS {
            let generator = DungeonGenerator::new(level);
            assert!(generator.has_boss_room);
        }
    }

    #[test]
    fn test_room_type_weights() {
        let combat_weight = RoomType::StandardCombat.spawn_weight(5, DungeonTheme::Dungeon);
        assert!(combat_weight > 0.0);

        // Boss lair should have lower weight than combat
        let boss_weight = RoomType::BossLair.spawn_weight(5, DungeonTheme::Dungeon);
        assert!(boss_weight < combat_weight);
    }
}

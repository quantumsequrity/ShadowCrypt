//! World system: map, tiles, rooms, and dungeon generation
//!
//! This module contains the dungeon generation system including:
//! - Standard room generation
//! - Special room types (treasure vaults, boss arenas, puzzle rooms, trap gauntlets, merchant camps)
//! - Corridor and tunnel generation
//! - Theme-based environmental features

use rand::prelude::*;
use serde::{Serialize, Deserialize};

/// Map dimensions
pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 45;
pub const VIEW_RADIUS: i32 = 10;
pub const MAX_ROOMS: usize = 20;
pub const MIN_ROOM_SIZE: usize = 5;
pub const MAX_ROOM_SIZE: usize = 15;
pub const MAX_DUNGEON_LEVEL: u32 = 30;
pub const BOSS_LEVELS: [u32; 6] = [5, 10, 15, 20, 25, 30];

/// Special room size constraints
pub const SPECIAL_ROOM_MIN_SIZE: usize = 8;
pub const SPECIAL_ROOM_MAX_SIZE: usize = 14;
pub const BOSS_ARENA_MIN_SIZE: usize = 12;
pub const BOSS_ARENA_MAX_SIZE: usize = 18;

/// Tile types in the dungeon
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Tile {
    Wall,
    Floor,
    StairsDown,
    StairsUp,
    Door,
    OpenDoor,
    Trap,
    DisarmedTrap,
    Water,
    Lava,
    Chest,
    OpenChest,
    Shrine,
    UsedShrine,
    Pillar,
    Grass,
    Ice,
    Sand,
    BossGate,
    // New shrine types
    BlessingShrine,
    UsedBlessingShrine,
    CurseShrine,
    UsedCurseShrine,
    StatShrine,
    UsedStatShrine,
    TeleportShrine,
    UsedTeleportShrine,
    ChaosShrine,
    UsedChaosShrine,
    WarShrine,
    UsedWarShrine,
    WisdomShrine,
    UsedWisdomShrine,
    SacrificeShrine,
    UsedSacrificeShrine,
    HealingShrine,
    UsedHealingShrine,
    LuckShrine,
    UsedLuckShrine,
    // Special room tiles
    VaultDoor,
    VaultFloor,
    GoldPile,
    GemDeposit,
    ArenaPillar,
    ArenaGate,
    ArenaFloor,
    BloodStain,
    PuzzleFloor,
    PuzzleTrigger,
    PuzzleActivated,
    PuzzleBarrier,
    PuzzleBarrierOpen,
    TrapFloor,
    SpikeTrap,
    FireTrap,
    PoisonTrap,
    ArrowTrap,
    MerchantRug,
    MerchantStall,
    Campfire,
    SupplyCrate,
    WeaponRack,
    PotionShelf,
}

impl Tile {
    /// Returns the glyph character for this tile
    pub fn glyph(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Floor => '.',
            Self::StairsDown => '>',
            Self::StairsUp => '<',
            Self::Door => '+',
            Self::OpenDoor => '\'',
            Self::Trap => '^',
            Self::DisarmedTrap => '_',
            Self::Water => '~',
            Self::Lava => '~',
            Self::Chest => '=',
            Self::OpenChest => '-',
            Self::Shrine => '&',
            Self::UsedShrine => '.',
            Self::Pillar => 'O',
            Self::Grass => '"',
            Self::Ice => '.',
            Self::Sand => '.',
            Self::BossGate => '8',
            // New shrine glyphs - each has a unique character
            Self::BlessingShrine => '*',
            Self::UsedBlessingShrine => '.',
            Self::CurseShrine => '!',
            Self::UsedCurseShrine => '.',
            Self::StatShrine => '+',
            Self::UsedStatShrine => '.',
            Self::TeleportShrine => '@',
            Self::UsedTeleportShrine => '.',
            Self::ChaosShrine => '?',
            Self::UsedChaosShrine => '.',
            Self::WarShrine => '%',
            Self::UsedWarShrine => '.',
            Self::WisdomShrine => '$',
            Self::UsedWisdomShrine => '.',
            Self::SacrificeShrine => '/',
            Self::UsedSacrificeShrine => '.',
            Self::HealingShrine => '+',
            Self::UsedHealingShrine => '.',
            Self::LuckShrine => '7',
            Self::UsedLuckShrine => '.',
            // Special room glyphs
            Self::VaultDoor => '+',
            Self::VaultFloor => '.',
            Self::GoldPile => '$',
            Self::GemDeposit => '*',
            Self::ArenaPillar => 'I',
            Self::ArenaGate => '#',
            Self::ArenaFloor => '.',
            Self::BloodStain => '~',
            Self::PuzzleFloor => ':',
            Self::PuzzleTrigger => '!',
            Self::PuzzleActivated => '_',
            Self::PuzzleBarrier => '|',
            Self::PuzzleBarrierOpen => '.',
            Self::TrapFloor => '.',
            Self::SpikeTrap => '^',
            Self::FireTrap => 'f',
            Self::PoisonTrap => 'p',
            Self::ArrowTrap => '>',
            Self::MerchantRug => ',',
            Self::MerchantStall => '#',
            Self::Campfire => '*',
            Self::SupplyCrate => '=',
            Self::WeaponRack => '/',
            Self::PotionShelf => '!',
        }
    }

    /// Returns a color index for the tile (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Wall => 0,        // DarkGrey
            Self::Floor => 1,       // Grey
            Self::StairsDown => 9,  // Cyan
            Self::StairsUp => 9,    // Cyan
            Self::Door => 11,       // Yellow
            Self::OpenDoor => 12,   // DarkYellow
            Self::Trap => 3,        // Red
            Self::DisarmedTrap => 0, // DarkGrey
            Self::Water => 7,       // Blue
            Self::Lava => 3,        // Red
            Self::Chest => 11,      // Yellow
            Self::OpenChest => 12,  // DarkYellow
            Self::Shrine => 13,     // Magenta
            Self::UsedShrine => 14, // DarkMagenta
            Self::Pillar => 2,      // White
            Self::Grass => 5,       // Green
            Self::Ice => 9,         // Cyan
            Self::Sand => 11,       // Yellow
            Self::BossGate => 3,    // Red
            // New shrine colors
            Self::BlessingShrine => 11,      // Yellow (divine)
            Self::UsedBlessingShrine => 14,  // DarkMagenta
            Self::CurseShrine => 4,          // DarkRed (ominous)
            Self::UsedCurseShrine => 14,     // DarkMagenta
            Self::StatShrine => 5,           // Green (growth)
            Self::UsedStatShrine => 14,      // DarkMagenta
            Self::TeleportShrine => 7,       // Blue (arcane)
            Self::UsedTeleportShrine => 14,  // DarkMagenta
            Self::ChaosShrine => 13,         // Magenta (chaotic)
            Self::UsedChaosShrine => 14,     // DarkMagenta
            Self::WarShrine => 3,            // Red (battle)
            Self::UsedWarShrine => 14,       // DarkMagenta
            Self::WisdomShrine => 9,         // Cyan (knowledge)
            Self::UsedWisdomShrine => 14,    // DarkMagenta
            Self::SacrificeShrine => 4,      // DarkRed (blood)
            Self::UsedSacrificeShrine => 14, // DarkMagenta
            Self::HealingShrine => 2,        // White (pure)
            Self::UsedHealingShrine => 14,   // DarkMagenta
            Self::LuckShrine => 11,          // Yellow (fortune)
            Self::UsedLuckShrine => 14,      // DarkMagenta
            // Special room colors
            Self::VaultDoor => 11,           // Yellow (gold trim)
            Self::VaultFloor => 12,          // DarkYellow (rich)
            Self::GoldPile => 11,            // Yellow (gold)
            Self::GemDeposit => 13,          // Magenta (precious)
            Self::ArenaPillar => 3,          // Red (blood-stained)
            Self::ArenaGate => 4,            // DarkRed (iron)
            Self::ArenaFloor => 4,           // DarkRed (bloodied)
            Self::BloodStain => 3,           // Red (blood)
            Self::PuzzleFloor => 9,          // Cyan (arcane)
            Self::PuzzleTrigger => 7,        // Blue (glowing)
            Self::PuzzleActivated => 5,      // Green (solved)
            Self::PuzzleBarrier => 7,        // Blue (magical)
            Self::PuzzleBarrierOpen => 1,    // Grey (disabled)
            Self::TrapFloor => 0,            // DarkGrey (hidden)
            Self::SpikeTrap => 4,            // DarkRed
            Self::FireTrap => 3,             // Red (fire)
            Self::PoisonTrap => 5,           // Green (toxic)
            Self::ArrowTrap => 12,           // DarkYellow (mechanism)
            Self::MerchantRug => 4,          // DarkRed (fine rug)
            Self::MerchantStall => 12,       // DarkYellow (wood)
            Self::Campfire => 3,             // Red (flames)
            Self::SupplyCrate => 12,         // DarkYellow (wood)
            Self::WeaponRack => 2,           // White (metal)
            Self::PotionShelf => 13,         // Magenta (magical)
        }
    }

    /// Returns whether this tile can be walked on
    pub fn walkable(&self) -> bool {
        matches!(
            self,
            Self::Floor
                | Self::StairsDown
                | Self::StairsUp
                | Self::OpenDoor
                | Self::Trap
                | Self::DisarmedTrap
                | Self::Water
                | Self::Grass
                | Self::Ice
                | Self::Sand
                | Self::UsedShrine
                | Self::OpenChest
                | Self::BossGate
                | Self::UsedBlessingShrine
                | Self::UsedCurseShrine
                | Self::UsedStatShrine
                | Self::UsedTeleportShrine
                | Self::UsedChaosShrine
                | Self::UsedWarShrine
                | Self::UsedWisdomShrine
                | Self::UsedSacrificeShrine
                | Self::UsedHealingShrine
                | Self::UsedLuckShrine
                // Special room walkable tiles
                | Self::VaultFloor
                | Self::ArenaFloor
                | Self::BloodStain
                | Self::PuzzleFloor
                | Self::PuzzleTrigger
                | Self::PuzzleActivated
                | Self::PuzzleBarrierOpen
                | Self::TrapFloor
                | Self::SpikeTrap
                | Self::FireTrap
                | Self::PoisonTrap
                | Self::ArrowTrap
                | Self::MerchantRug
                | Self::Campfire
        )
    }

    /// Returns whether this tile is a special room trap
    pub fn is_special_trap(&self) -> bool {
        matches!(
            self,
            Self::SpikeTrap | Self::FireTrap | Self::PoisonTrap | Self::ArrowTrap
        )
    }

    /// Returns whether this tile is interactive in a special room
    pub fn is_interactive(&self) -> bool {
        matches!(
            self,
            Self::GoldPile
                | Self::GemDeposit
                | Self::PuzzleTrigger
                | Self::Campfire
                | Self::SupplyCrate
                | Self::WeaponRack
                | Self::PotionShelf
                | Self::Chest
        )
    }

    /// Returns whether this tile is an active (usable) shrine
    pub fn is_shrine(&self) -> bool {
        matches!(
            self,
            Self::Shrine
                | Self::BlessingShrine
                | Self::CurseShrine
                | Self::StatShrine
                | Self::TeleportShrine
                | Self::ChaosShrine
                | Self::WarShrine
                | Self::WisdomShrine
                | Self::SacrificeShrine
                | Self::HealingShrine
                | Self::LuckShrine
        )
    }

    /// Returns the used version of this shrine tile
    pub fn used_shrine(&self) -> Self {
        match self {
            Self::Shrine => Self::UsedShrine,
            Self::BlessingShrine => Self::UsedBlessingShrine,
            Self::CurseShrine => Self::UsedCurseShrine,
            Self::StatShrine => Self::UsedStatShrine,
            Self::TeleportShrine => Self::UsedTeleportShrine,
            Self::ChaosShrine => Self::UsedChaosShrine,
            Self::WarShrine => Self::UsedWarShrine,
            Self::WisdomShrine => Self::UsedWisdomShrine,
            Self::SacrificeShrine => Self::UsedSacrificeShrine,
            Self::HealingShrine => Self::UsedHealingShrine,
            Self::LuckShrine => Self::UsedLuckShrine,
            _ => *self, // Return self if not a shrine
        }
    }

    /// Returns the display name of the shrine
    pub fn shrine_name(&self) -> Option<&'static str> {
        match self {
            Self::Shrine => Some("Ancient Shrine"),
            Self::BlessingShrine => Some("Shrine of Blessings"),
            Self::CurseShrine => Some("Cursed Shrine"),
            Self::StatShrine => Some("Shrine of Empowerment"),
            Self::TeleportShrine => Some("Shrine of Translocation"),
            Self::ChaosShrine => Some("Shrine of Chaos"),
            Self::WarShrine => Some("War Shrine"),
            Self::WisdomShrine => Some("Shrine of Wisdom"),
            Self::SacrificeShrine => Some("Sacrificial Shrine"),
            Self::HealingShrine => Some("Shrine of Healing"),
            Self::LuckShrine => Some("Shrine of Fortune"),
            _ => None,
        }
    }

    /// Returns whether this tile blocks line of sight
    pub fn blocks_sight(&self) -> bool {
        matches!(
            self,
            Self::Wall
                | Self::Door
                | Self::Pillar
                | Self::VaultDoor
                | Self::ArenaGate
                | Self::ArenaPillar
                | Self::PuzzleBarrier
                | Self::MerchantStall
        )
    }

    /// Returns the trap damage type if this tile is a trap
    pub fn trap_damage_type(&self) -> Option<&'static str> {
        match self {
            Self::Trap => Some("physical"),
            Self::SpikeTrap => Some("piercing"),
            Self::FireTrap => Some("fire"),
            Self::PoisonTrap => Some("poison"),
            Self::ArrowTrap => Some("piercing"),
            _ => None,
        }
    }
}

/// Dungeon themes based on level
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum DungeonTheme {
    Dungeon,
    Cave,
    Crypt,
    Forest,
    IceCavern,
    VolcanicLair,
    AncientRuins,
    DemonRealm,
}

impl DungeonTheme {
    /// Returns the theme for a given dungeon level
    pub fn from_level(level: u32) -> Self {
        match level {
            1..=4 => Self::Dungeon,
            5..=8 => Self::Cave,
            9..=12 => Self::Crypt,
            13..=16 => Self::Forest,
            17..=20 => Self::IceCavern,
            21..=24 => Self::VolcanicLair,
            25..=28 => Self::AncientRuins,
            _ => Self::DemonRealm,
        }
    }

    /// Returns the default floor tile for this theme
    pub fn floor_tile(&self) -> Tile {
        match self {
            Self::Dungeon | Self::Cave | Self::Crypt => Tile::Floor,
            Self::Forest => Tile::Grass,
            Self::IceCavern => Tile::Ice,
            Self::AncientRuins => Tile::Sand,
            Self::VolcanicLair | Self::DemonRealm => Tile::Floor,
        }
    }

    /// Returns the display name of this theme
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dungeon => "Dark Dungeon",
            Self::Cave => "Twisted Caves",
            Self::Crypt => "Haunted Crypt",
            Self::Forest => "Cursed Forest",
            Self::IceCavern => "Frozen Caverns",
            Self::VolcanicLair => "Volcanic Depths",
            Self::AncientRuins => "Ancient Ruins",
            Self::DemonRealm => "Demon Realm",
        }
    }
}

/// Special room types with unique generation and gameplay mechanics
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum SpecialRoomType {
    /// Standard dungeon room
    Normal,
    /// Treasure vault with gold, gems, and rare items - heavily guarded
    TreasureVault,
    /// Boss arena with pillars, gates, and space for epic battles
    BossArena,
    /// Puzzle room with triggers, barriers, and rewards
    PuzzleRoom,
    /// Trap gauntlet with dangerous obstacles and traps
    TrapGauntlet,
    /// Merchant camp with shops, campfire, and supplies
    MerchantCamp,
}

impl SpecialRoomType {
    /// Returns the display name of this room type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Room",
            Self::TreasureVault => "Treasure Vault",
            Self::BossArena => "Boss Arena",
            Self::PuzzleRoom => "Puzzle Chamber",
            Self::TrapGauntlet => "Trap Gauntlet",
            Self::MerchantCamp => "Merchant Camp",
        }
    }

    /// Returns whether enemies can spawn in this room type
    pub fn allows_enemies(&self) -> bool {
        match self {
            Self::Normal => true,
            Self::TreasureVault => true,  // Guarded
            Self::BossArena => true,      // Boss spawns here
            Self::PuzzleRoom => false,    // Safe until solved
            Self::TrapGauntlet => false,  // Traps are the danger
            Self::MerchantCamp => false,  // Safe zone
        }
    }

    /// Returns the minimum size for this room type
    pub fn min_size(&self) -> usize {
        match self {
            Self::Normal => MIN_ROOM_SIZE,
            Self::TreasureVault => SPECIAL_ROOM_MIN_SIZE,
            Self::BossArena => BOSS_ARENA_MIN_SIZE,
            Self::PuzzleRoom => SPECIAL_ROOM_MIN_SIZE,
            Self::TrapGauntlet => SPECIAL_ROOM_MIN_SIZE + 2,
            Self::MerchantCamp => SPECIAL_ROOM_MIN_SIZE,
        }
    }

    /// Returns the maximum size for this room type
    pub fn max_size(&self) -> usize {
        match self {
            Self::Normal => MAX_ROOM_SIZE,
            Self::TreasureVault => SPECIAL_ROOM_MAX_SIZE,
            Self::BossArena => BOSS_ARENA_MAX_SIZE,
            Self::PuzzleRoom => SPECIAL_ROOM_MAX_SIZE,
            Self::TrapGauntlet => SPECIAL_ROOM_MAX_SIZE + 2,
            Self::MerchantCamp => SPECIAL_ROOM_MAX_SIZE,
        }
    }

    /// Returns the spawn weight for this room type at a given level
    pub fn spawn_weight(&self, level: u32) -> f64 {
        match self {
            Self::Normal => 1.0,  // Always possible
            Self::TreasureVault => {
                if level >= 3 { 0.08 + (level as f64 * 0.005) } else { 0.0 }
            }
            Self::BossArena => {
                if BOSS_LEVELS.contains(&level) { 1.0 } else { 0.0 }
            }
            Self::PuzzleRoom => {
                if level >= 2 { 0.10 + (level as f64 * 0.003) } else { 0.0 }
            }
            Self::TrapGauntlet => {
                if level >= 4 { 0.08 + (level as f64 * 0.004) } else { 0.0 }
            }
            Self::MerchantCamp => {
                // More likely every 3-4 levels, rare otherwise
                if level % 3 == 0 || level % 4 == 0 {
                    0.25
                } else {
                    0.05
                }
            }
        }
    }
}

/// A room in the dungeon
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub is_boss_room: bool,
    /// The special type of this room
    pub room_type: SpecialRoomType,
    /// Whether this room has been cleared/completed
    pub cleared: bool,
    /// Difficulty rating for this room (1-10)
    pub difficulty: u8,
}

impl Room {
    /// Create a new room
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height, is_boss_room: false }
    }

    /// Returns the center point of the room
    pub fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Returns whether this room intersects with another
    pub fn intersects(&self, other: &Room) -> bool {
        self.x <= other.x + other.width + 1
            && self.x + self.width + 1 >= other.x
            && self.y <= other.y + other.height + 1
            && self.y + self.height + 1 >= other.y
    }

    /// Returns a random point within the room
    pub fn random_point(&self, rng: &mut impl Rng) -> (usize, usize) {
        (
            rng.gen_range(self.x + 1..self.x + self.width - 1),
            rng.gen_range(self.y + 1..self.y + self.height - 1),
        )
    }
}

/// The dungeon map
#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub visible: Vec<Vec<bool>>,
    pub explored: Vec<Vec<bool>>,
    pub rooms: Vec<Room>,
    pub theme: DungeonTheme,
}

impl Map {
    /// Create a new empty map
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Wall; MAP_WIDTH]; MAP_HEIGHT],
            visible: vec![vec![false; MAP_WIDTH]; MAP_HEIGHT],
            explored: vec![vec![false; MAP_WIDTH]; MAP_HEIGHT],
            rooms: Vec::new(),
            theme: DungeonTheme::Dungeon,
        }
    }

    /// Generate a new dungeon level
    pub fn generate(&mut self, rng: &mut impl Rng, level: u32) {
        self.theme = DungeonTheme::from_level(level);
        let floor_tile = self.theme.floor_tile();

        // Reset
        self.tiles = vec![vec![Tile::Wall; MAP_WIDTH]; MAP_HEIGHT];
        self.visible = vec![vec![false; MAP_WIDTH]; MAP_HEIGHT];
        self.rooms.clear();

        let is_boss_level = BOSS_LEVELS.contains(&level);

        // Generate rooms
        let target_rooms = if is_boss_level { MAX_ROOMS + 1 } else { MAX_ROOMS };

        for _ in 0..target_rooms * 4 {
            if self.rooms.len() >= target_rooms {
                break;
            }

            let width = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
            let height = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
            let x = rng.gen_range(1..MAP_WIDTH - width - 1);
            let y = rng.gen_range(1..MAP_HEIGHT - height - 1);

            let new_room = Room::new(x, y, width, height);

            let overlaps = self.rooms.iter().any(|r| new_room.intersects(r));
            if !overlaps {
                self.carve_room(&new_room, floor_tile);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms.last().unwrap().center();

                    if rng.gen_bool(0.5) {
                        self.carve_h_tunnel(prev_x, new_x, prev_y, floor_tile);
                        self.carve_v_tunnel(prev_y, new_y, new_x, floor_tile);
                    } else {
                        self.carve_v_tunnel(prev_y, new_y, prev_x, floor_tile);
                        self.carve_h_tunnel(prev_x, new_x, new_y, floor_tile);
                    }

                    // Add doors at tunnel intersections
                    if rng.gen_bool(0.3) {
                        self.tiles[new_y][prev_x] = Tile::Door;
                    }
                }

                self.rooms.push(new_room);
            }
        }

        // Add special features
        self.add_features(rng, level);

        // Place stairs
        if self.rooms.len() >= 2 {
            let last_room = self.rooms.last().unwrap();
            let (sx, sy) = last_room.center();

            if is_boss_level {
                self.tiles[sy][sx] = Tile::BossGate;
            } else {
                self.tiles[sy][sx] = Tile::StairsDown;
            }

            if level > 1 {
                let first_room = &self.rooms[0];
                let (ux, uy) = first_room.center();
                self.tiles[uy][ux] = Tile::StairsUp;
            }
        }
    }

    /// Carve out a room
    fn carve_room(&mut self, room: &Room, floor_tile: Tile) {
        for y in room.y..room.y + room.height {
            for x in room.x..room.x + room.width {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    /// Carve a horizontal tunnel
    fn carve_h_tunnel(&mut self, x1: usize, x2: usize, y: usize, floor_tile: Tile) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            if self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    /// Carve a vertical tunnel
    fn carve_v_tunnel(&mut self, y1: usize, y2: usize, x: usize, floor_tile: Tile) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            if self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    /// Add special features to the dungeon
    fn add_features(&mut self, rng: &mut impl Rng, level: u32) {
        // Add traps
        for room in &self.rooms[1..] {
            if rng.gen_bool(0.2) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() && self.tiles[y][x] != Tile::StairsDown {
                    self.tiles[y][x] = Tile::Trap;
                }
            }
        }

        // Add chests
        for room in &self.rooms[1..] {
            if rng.gen_bool(0.15) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::Chest;
                }
            }
        }

        // Add shrines - multiple types with level-based spawning
        self.spawn_shrines(rng, level);

        // Add water/lava pools based on theme
        match self.theme {
            DungeonTheme::Cave | DungeonTheme::Dungeon => {
                for room in &self.rooms {
                    if rng.gen_bool(0.1) {
                        let (x, y) = room.random_point(rng);
                        for dy in 0..3 {
                            for dx in 0..3 {
                                let nx = x + dx;
                                let ny = y + dy;
                                if nx < MAP_WIDTH && ny < MAP_HEIGHT && self.tiles[ny][nx].walkable() {
                                    self.tiles[ny][nx] = Tile::Water;
                                }
                            }
                        }
                    }
                }
            }
            DungeonTheme::VolcanicLair | DungeonTheme::DemonRealm => {
                for room in &self.rooms {
                    if rng.gen_bool(0.15) {
                        let (x, y) = room.random_point(rng);
                        for dy in 0..2 {
                            for dx in 0..2 {
                                let nx = x + dx;
                                let ny = y + dy;
                                if nx < MAP_WIDTH && ny < MAP_HEIGHT && self.tiles[ny][nx].walkable() {
                                    self.tiles[ny][nx] = Tile::Lava;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // Add pillars
        for room in &self.rooms {
            if room.width > 8 && room.height > 8 && rng.gen_bool(0.3) {
                let cx = room.x + room.width / 2;
                let cy = room.y + room.height / 2;
                for &(dx, dy) in &[(-2i32, -2i32), (2, -2), (-2, 2), (2, 2)] {
                    let px = (cx as i32 + dx) as usize;
                    let py = (cy as i32 + dy) as usize;
                    if self.tiles[py][px].walkable() {
                        self.tiles[py][px] = Tile::Pillar;
                    }
                }
            }
        }
    }

    /// Spawns various shrine types throughout the dungeon
    fn spawn_shrines(&mut self, rng: &mut impl Rng, level: u32) {
        if self.rooms.len() < 2 {
            return;
        }

        // Calculate number of shrines based on level
        let base_shrine_chance = 0.15 + level as f64 * 0.02;
        let max_shrines = 1 + (level / 5) as usize;
        let mut shrines_placed = 0;

        // Define shrine types with their spawn weights based on dungeon theme and level
        let shrine_types: Vec<(Tile, f64)> = match self.theme {
            DungeonTheme::Dungeon => vec![
                (Tile::Shrine, 0.20),
                (Tile::BlessingShrine, 0.15),
                (Tile::HealingShrine, 0.20),
                (Tile::StatShrine, 0.10),
                (Tile::TeleportShrine, 0.10),
                (Tile::ChaosShrine, 0.08),
                (Tile::WarShrine, 0.10),
                (Tile::WisdomShrine, 0.07),
            ],
            DungeonTheme::Cave => vec![
                (Tile::Shrine, 0.15),
                (Tile::StatShrine, 0.15),
                (Tile::TeleportShrine, 0.20),
                (Tile::ChaosShrine, 0.15),
                (Tile::HealingShrine, 0.15),
                (Tile::LuckShrine, 0.10),
                (Tile::WarShrine, 0.10),
            ],
            DungeonTheme::Crypt => vec![
                (Tile::CurseShrine, 0.20),
                (Tile::SacrificeShrine, 0.20),
                (Tile::Shrine, 0.10),
                (Tile::WisdomShrine, 0.15),
                (Tile::ChaosShrine, 0.15),
                (Tile::HealingShrine, 0.10),
                (Tile::BlessingShrine, 0.10),
            ],
            DungeonTheme::Forest => vec![
                (Tile::HealingShrine, 0.25),
                (Tile::BlessingShrine, 0.20),
                (Tile::WisdomShrine, 0.15),
                (Tile::Shrine, 0.15),
                (Tile::StatShrine, 0.10),
                (Tile::LuckShrine, 0.15),
            ],
            DungeonTheme::IceCavern => vec![
                (Tile::WisdomShrine, 0.20),
                (Tile::TeleportShrine, 0.15),
                (Tile::StatShrine, 0.15),
                (Tile::HealingShrine, 0.15),
                (Tile::Shrine, 0.15),
                (Tile::ChaosShrine, 0.10),
                (Tile::LuckShrine, 0.10),
            ],
            DungeonTheme::VolcanicLair => vec![
                (Tile::WarShrine, 0.25),
                (Tile::SacrificeShrine, 0.20),
                (Tile::ChaosShrine, 0.15),
                (Tile::StatShrine, 0.15),
                (Tile::Shrine, 0.10),
                (Tile::CurseShrine, 0.15),
            ],
            DungeonTheme::AncientRuins => vec![
                (Tile::WisdomShrine, 0.25),
                (Tile::BlessingShrine, 0.15),
                (Tile::StatShrine, 0.15),
                (Tile::TeleportShrine, 0.15),
                (Tile::Shrine, 0.15),
                (Tile::LuckShrine, 0.15),
            ],
            DungeonTheme::DemonRealm => vec![
                (Tile::CurseShrine, 0.20),
                (Tile::SacrificeShrine, 0.20),
                (Tile::ChaosShrine, 0.20),
                (Tile::WarShrine, 0.15),
                (Tile::Shrine, 0.10),
                (Tile::StatShrine, 0.15),
            ],
        };

        // Try to place shrines in rooms
        for room_idx in 1..self.rooms.len() {
            if shrines_placed >= max_shrines {
                break;
            }

            if rng.gen_bool(base_shrine_chance) {
                let room = &self.rooms[room_idx];
                let (x, y) = room.center();

                if self.tiles[y][x].walkable() && !self.tiles[y][x].is_shrine() {
                    // Choose shrine type based on weights
                    let shrine_type = self.choose_weighted_shrine(rng, &shrine_types);
                    self.tiles[y][x] = shrine_type;
                    shrines_placed += 1;
                }
            }
        }

        // Guaranteed shrine on deeper levels (every 5 levels)
        if level % 5 == 0 && shrines_placed == 0 && self.rooms.len() > 2 {
            let room_idx = rng.gen_range(1..self.rooms.len() - 1);
            let room = &self.rooms[room_idx];
            let (x, y) = room.center();
            if self.tiles[y][x].walkable() {
                let shrine_type = self.choose_weighted_shrine(rng, &shrine_types);
                self.tiles[y][x] = shrine_type;
            }
        }
    }

    /// Choose a shrine type based on weighted probabilities
    fn choose_weighted_shrine(&self, rng: &mut impl Rng, weights: &[(Tile, f64)]) -> Tile {
        let total_weight: f64 = weights.iter().map(|(_, w)| w).sum();
        let mut roll = rng.gen::<f64>() * total_weight;

        for (tile, weight) in weights {
            roll -= weight;
            if roll <= 0.0 {
                return *tile;
            }
        }

        // Fallback to generic shrine
        Tile::Shrine
    }

    /// Compute field of view from a position
    pub fn compute_fov(&mut self, px: usize, py: usize) {
        for row in &mut self.visible {
            for cell in row {
                *cell = false;
            }
        }

        for angle in 0..360 {
            let rad = (angle as f32) * std::f32::consts::PI / 180.0;
            let dx = rad.cos();
            let dy = rad.sin();

            let mut x = px as f32 + 0.5;
            let mut y = py as f32 + 0.5;

            for _ in 0..VIEW_RADIUS {
                let ix = x as usize;
                let iy = y as usize;

                if ix >= MAP_WIDTH || iy >= MAP_HEIGHT {
                    break;
                }

                self.visible[iy][ix] = true;
                self.explored[iy][ix] = true;

                if self.tiles[iy][ix].blocks_sight() {
                    break;
                }

                x += dx;
                y += dy;
            }
        }
    }

    /// Check if a position is walkable
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= MAP_WIDTH || y >= MAP_HEIGHT {
            return false;
        }
        self.tiles[y][x].walkable()
    }

    /// Reveal the entire map
    pub fn reveal_all(&mut self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                self.explored[y][x] = true;
            }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_center() {
        let room = Room::new(10, 10, 8, 6);
        assert_eq!(room.center(), (14, 13));
    }

    #[test]
    fn test_room_intersection() {
        let room1 = Room::new(0, 0, 5, 5);
        let room2 = Room::new(4, 4, 5, 5);
        let room3 = Room::new(10, 10, 5, 5);

        assert!(room1.intersects(&room2));
        assert!(!room1.intersects(&room3));
    }

    #[test]
    fn test_map_generation() {
        let mut map = Map::new();
        let mut rng = rand::thread_rng();
        map.generate(&mut rng, 1);

        assert!(!map.rooms.is_empty());
    }
}

//! World system: map, tiles, rooms, and dungeon generation
//!
//! This module contains the dungeon generation system including:
//! - 100 floors of dungeon with unique themes
//! - Boss floors every 10 levels
//! - Mini-boss floors every 5 levels
//! - Secret floors accessible through special portals
//! - Floor modifiers affecting gameplay
//! - Theme-based environmental features

use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Map dimensions
pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 45;
pub const VIEW_RADIUS: i32 = 10;
pub const MAX_ROOMS: usize = 20;
pub const MIN_ROOM_SIZE: usize = 5;
pub const MAX_ROOM_SIZE: usize = 15;

/// Maximum dungeon level - expanded to 100 floors
pub const MAX_DUNGEON_LEVEL: u32 = 100;

/// Boss levels - every 10th floor
pub const BOSS_LEVELS: [u32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

/// Mini-boss levels - every 5th floor (excluding boss floors)
pub const MINI_BOSS_LEVELS: [u32; 10] = [5, 15, 25, 35, 45, 55, 65, 75, 85, 95];

/// Secret floor indices (accessible via special portals)
pub const SECRET_FLOOR_COUNT: u32 = 10;

/// Special room size constraints
pub const SPECIAL_ROOM_MIN_SIZE: usize = 8;
pub const SPECIAL_ROOM_MAX_SIZE: usize = 14;
pub const BOSS_ARENA_MIN_SIZE: usize = 12;
pub const BOSS_ARENA_MAX_SIZE: usize = 18;

/// Tile types in the dungeon
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
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
    // Theme-specific tiles
    Bones,
    Cobweb,
    Sarcophagus,
    SewerGrate,
    ToxicPool,
    MineCart,
    Rails,
    MagmaVent,
    ObsidianFloor,
    FrozenPool,
    Snowdrift,
    JungleVines,
    AncientStatue,
    CrystalFormation,
    MagicCircle,
    VoidRift,
    Tentacle,
    DivineLight,
    HolyAltar,
    ChaosPortal,
    AbyssalPit,
    // Secret passage tiles
    SecretDoor,
    RevealedSecretDoor,
    SecretPortal,
    // Environmental hazard tiles
    PoisonGas,
    FireGeyser,
    IceSpikes,
    VoidTendril,
    ChaosField,
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
    // Shrine types
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
            // Theme-specific glyphs
            Self::Bones => ',',
            Self::Cobweb => ';',
            Self::Sarcophagus => 'S',
            Self::SewerGrate => '#',
            Self::ToxicPool => '~',
            Self::MineCart => 'c',
            Self::Rails => '=',
            Self::MagmaVent => '*',
            Self::ObsidianFloor => '.',
            Self::FrozenPool => '~',
            Self::Snowdrift => '.',
            Self::JungleVines => '%',
            Self::AncientStatue => '&',
            Self::CrystalFormation => '*',
            Self::MagicCircle => '@',
            Self::VoidRift => 'O',
            Self::Tentacle => '~',
            Self::DivineLight => '*',
            Self::HolyAltar => 'A',
            Self::ChaosPortal => '0',
            Self::AbyssalPit => ' ',
            // Secret passages
            Self::SecretDoor => '#',
            Self::RevealedSecretDoor => '+',
            Self::SecretPortal => 'O',
            // Environmental hazards
            Self::PoisonGas => '*',
            Self::FireGeyser => '^',
            Self::IceSpikes => '^',
            Self::VoidTendril => '~',
            Self::ChaosField => '?',
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
            // Shrine glyphs
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
            // Theme-specific colors
            Self::Bones | Self::Cobweb => 2,           // White
            Self::Sarcophagus => 1,                     // Grey
            Self::SewerGrate => 0,                      // DarkGrey
            Self::ToxicPool => 5,                       // Green
            Self::MineCart => 12,                       // DarkYellow
            Self::Rails => 1,                           // Grey
            Self::MagmaVent => 3,                       // Red
            Self::ObsidianFloor => 0,                   // DarkGrey
            Self::FrozenPool => 9,                      // Cyan
            Self::Snowdrift => 2,                       // White
            Self::JungleVines => 5,                     // Green
            Self::AncientStatue => 11,                  // Yellow
            Self::CrystalFormation => 13,               // Magenta
            Self::MagicCircle => 7,                     // Blue
            Self::VoidRift => 14,                       // DarkMagenta
            Self::Tentacle => 4,                        // DarkRed
            Self::DivineLight => 11,                    // Yellow
            Self::HolyAltar => 2,                       // White
            Self::ChaosPortal => 3,                     // Red
            Self::AbyssalPit => 0,                      // DarkGrey
            // Secret passages
            Self::SecretDoor => 0,                      // DarkGrey (hidden)
            Self::RevealedSecretDoor => 11,             // Yellow
            Self::SecretPortal => 13,                   // Magenta
            // Environmental hazards
            Self::PoisonGas => 5,                       // Green
            Self::FireGeyser => 3,                      // Red
            Self::IceSpikes => 9,                       // Cyan
            Self::VoidTendril => 14,                    // DarkMagenta
            Self::ChaosField => 13,                     // Magenta
            // Special room colors
            Self::VaultDoor | Self::GoldPile => 11,    // Yellow
            Self::VaultFloor => 12,                     // DarkYellow
            Self::GemDeposit => 13,                     // Magenta
            Self::ArenaPillar | Self::BloodStain => 3,  // Red
            Self::ArenaGate | Self::ArenaFloor => 4,    // DarkRed
            Self::PuzzleFloor | Self::PuzzleTrigger => 9, // Cyan
            Self::PuzzleActivated => 5,                 // Green
            Self::PuzzleBarrier => 7,                   // Blue
            Self::PuzzleBarrierOpen => 1,               // Grey
            Self::TrapFloor => 0,                       // DarkGrey
            Self::SpikeTrap => 4,                       // DarkRed
            Self::FireTrap => 3,                        // Red
            Self::PoisonTrap => 5,                      // Green
            Self::ArrowTrap => 12,                      // DarkYellow
            Self::MerchantRug => 4,                     // DarkRed
            Self::MerchantStall => 12,                  // DarkYellow
            Self::Campfire => 3,                        // Red
            Self::SupplyCrate => 12,                    // DarkYellow
            Self::WeaponRack => 2,                      // White
            Self::PotionShelf => 13,                    // Magenta
            // Shrine colors
            Self::BlessingShrine => 11,
            Self::UsedBlessingShrine => 14,
            Self::CurseShrine => 4,
            Self::UsedCurseShrine => 14,
            Self::StatShrine => 5,
            Self::UsedStatShrine => 14,
            Self::TeleportShrine => 7,
            Self::UsedTeleportShrine => 14,
            Self::ChaosShrine => 13,
            Self::UsedChaosShrine => 14,
            Self::WarShrine => 3,
            Self::UsedWarShrine => 14,
            Self::WisdomShrine => 9,
            Self::UsedWisdomShrine => 14,
            Self::SacrificeShrine => 4,
            Self::UsedSacrificeShrine => 14,
            Self::HealingShrine => 2,
            Self::UsedHealingShrine => 14,
            Self::LuckShrine => 11,
            Self::UsedLuckShrine => 14,
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
                | Self::Bones
                | Self::ObsidianFloor
                | Self::Snowdrift
                | Self::DivineLight
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
                | Self::RevealedSecretDoor
                | Self::PoisonGas
                | Self::ChaosField
        )
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
                | Self::SecretDoor
                | Self::JungleVines
                | Self::CrystalFormation
        )
    }

    /// Returns whether this tile is a shrine
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
            _ => *self,
        }
    }

    /// Returns the display name of this tile
    pub fn name(&self) -> &'static str {
        match self {
            Self::Wall => "Wall",
            Self::Floor => "Floor",
            Self::StairsDown => "Stairs Down",
            Self::StairsUp => "Stairs Up",
            Self::Door => "Door",
            Self::OpenDoor => "Open Door",
            Self::Trap => "Trap",
            Self::DisarmedTrap => "Disarmed Trap",
            Self::Water => "Water",
            Self::Lava => "Lava",
            Self::Chest => "Chest",
            Self::OpenChest => "Open Chest",
            Self::Shrine => "Shrine",
            Self::UsedShrine => "Used Shrine",
            Self::Pillar => "Pillar",
            Self::Grass => "Grass",
            Self::Ice => "Ice",
            Self::Sand => "Sand",
            Self::BossGate => "Boss Gate",
            Self::Bones => "Bones",
            Self::Cobweb => "Cobweb",
            Self::Sarcophagus => "Sarcophagus",
            Self::SewerGrate => "Sewer Grate",
            Self::ToxicPool => "Toxic Pool",
            Self::MineCart => "Mine Cart",
            Self::Rails => "Rails",
            Self::MagmaVent => "Magma Vent",
            Self::ObsidianFloor => "Obsidian Floor",
            Self::FrozenPool => "Frozen Pool",
            Self::Snowdrift => "Snowdrift",
            Self::JungleVines => "Jungle Vines",
            Self::AncientStatue => "Ancient Statue",
            Self::CrystalFormation => "Crystal Formation",
            Self::MagicCircle => "Magic Circle",
            Self::VoidRift => "Void Rift",
            Self::Tentacle => "Tentacle",
            Self::DivineLight => "Divine Light",
            Self::HolyAltar => "Holy Altar",
            Self::ChaosPortal => "Chaos Portal",
            Self::AbyssalPit => "Abyssal Pit",
            Self::SecretDoor => "Wall",  // Hidden
            Self::RevealedSecretDoor => "Secret Door",
            Self::SecretPortal => "Secret Portal",
            Self::PoisonGas => "Poison Gas",
            Self::FireGeyser => "Fire Geyser",
            Self::IceSpikes => "Ice Spikes",
            Self::VoidTendril => "Void Tendril",
            Self::ChaosField => "Chaos Field",
            Self::VaultDoor => "Vault Door",
            Self::VaultFloor => "Vault Floor",
            Self::GoldPile => "Gold Pile",
            Self::GemDeposit => "Gem Deposit",
            Self::ArenaPillar => "Arena Pillar",
            Self::ArenaGate => "Arena Gate",
            Self::ArenaFloor => "Arena Floor",
            Self::BloodStain => "Blood Stain",
            Self::PuzzleFloor => "Puzzle Floor",
            Self::PuzzleTrigger => "Puzzle Trigger",
            Self::PuzzleActivated => "Activated Trigger",
            Self::PuzzleBarrier => "Puzzle Barrier",
            Self::PuzzleBarrierOpen => "Open Passage",
            Self::TrapFloor => "Floor",
            Self::SpikeTrap => "Spike Trap",
            Self::FireTrap => "Fire Trap",
            Self::PoisonTrap => "Poison Trap",
            Self::ArrowTrap => "Arrow Trap",
            Self::MerchantRug => "Merchant Rug",
            Self::MerchantStall => "Merchant Stall",
            Self::Campfire => "Campfire",
            Self::SupplyCrate => "Supply Crate",
            Self::WeaponRack => "Weapon Rack",
            Self::PotionShelf => "Potion Shelf",
            Self::BlessingShrine => "Shrine of Blessings",
            Self::UsedBlessingShrine => "Used Shrine",
            Self::CurseShrine => "Cursed Shrine",
            Self::UsedCurseShrine => "Used Shrine",
            Self::StatShrine => "Shrine of Empowerment",
            Self::UsedStatShrine => "Used Shrine",
            Self::TeleportShrine => "Shrine of Translocation",
            Self::UsedTeleportShrine => "Used Shrine",
            Self::ChaosShrine => "Shrine of Chaos",
            Self::UsedChaosShrine => "Used Shrine",
            Self::WarShrine => "War Shrine",
            Self::UsedWarShrine => "Used Shrine",
            Self::WisdomShrine => "Shrine of Wisdom",
            Self::UsedWisdomShrine => "Used Shrine",
            Self::SacrificeShrine => "Sacrificial Shrine",
            Self::UsedSacrificeShrine => "Used Shrine",
            Self::HealingShrine => "Shrine of Healing",
            Self::UsedHealingShrine => "Used Shrine",
            Self::LuckShrine => "Shrine of Fortune",
            Self::UsedLuckShrine => "Used Shrine",
        }
    }
}

/// Dungeon themes - 10 unique themes for each 10-floor section
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum DungeonTheme {
    /// Floors 1-10: Dark underground tombs filled with undead
    Catacombs,
    /// Floors 11-20: Flooded tunnels with vermin and water hazards
    Sewers,
    /// Floors 21-30: Deep earth tunnels with constructs and minerals
    Mines,
    /// Floors 31-40: Volcanic depths with fire and demons
    VolcanicDepths,
    /// Floors 41-50: Frozen underground with ice and spirits
    FrozenCaverns,
    /// Floors 51-60: Overgrown ruins with nature and beasts
    JungleRuins,
    /// Floors 61-70: Magical caves with crystals and elementals
    CrystalCaves,
    /// Floors 71-80: Reality-warped tunnels with void creatures
    VoidTunnels,
    /// Floors 81-90: Celestial realm with divine beings
    DivineRealm,
    /// Floors 91-100: The final chaos realm with ultimate challenges
    Abyss,
    /// Secret floors - special hidden areas
    SecretRealm,
}

impl DungeonTheme {
    /// Returns the theme for a given dungeon level
    pub fn from_level(level: u32) -> Self {
        match level {
            1..=10 => Self::Catacombs,
            11..=20 => Self::Sewers,
            21..=30 => Self::Mines,
            31..=40 => Self::VolcanicDepths,
            41..=50 => Self::FrozenCaverns,
            51..=60 => Self::JungleRuins,
            61..=70 => Self::CrystalCaves,
            71..=80 => Self::VoidTunnels,
            81..=90 => Self::DivineRealm,
            91..=100 => Self::Abyss,
            _ => Self::Abyss,
        }
    }

    /// Returns the default floor tile for this theme
    pub fn floor_tile(&self) -> Tile {
        match self {
            Self::Catacombs => Tile::Floor,
            Self::Sewers => Tile::Floor,
            Self::Mines => Tile::Floor,
            Self::VolcanicDepths => Tile::ObsidianFloor,
            Self::FrozenCaverns => Tile::Ice,
            Self::JungleRuins => Tile::Grass,
            Self::CrystalCaves => Tile::Floor,
            Self::VoidTunnels => Tile::Floor,
            Self::DivineRealm => Tile::Floor,
            Self::Abyss => Tile::Floor,
            Self::SecretRealm => Tile::Floor,
        }
    }

    /// Returns the display name of this theme
    pub fn name(&self) -> &'static str {
        match self {
            Self::Catacombs => "The Catacombs",
            Self::Sewers => "The Sewers",
            Self::Mines => "The Mines",
            Self::VolcanicDepths => "The Volcanic Depths",
            Self::FrozenCaverns => "The Frozen Caverns",
            Self::JungleRuins => "The Jungle Ruins",
            Self::CrystalCaves => "The Crystal Caves",
            Self::VoidTunnels => "The Void Tunnels",
            Self::DivineRealm => "The Divine Realm",
            Self::Abyss => "The Abyss",
            Self::SecretRealm => "Secret Realm",
        }
    }

    /// Returns the description of this theme
    pub fn description(&self) -> &'static str {
        match self {
            Self::Catacombs => "Dark underground tombs filled with the restless dead",
            Self::Sewers => "Flooded tunnels teeming with vermin and toxic waters",
            Self::Mines => "Deep earth passages with ancient constructs guarding precious ores",
            Self::VolcanicDepths => "Scorching caverns where demons lurk in rivers of magma",
            Self::FrozenCaverns => "Ice-bound passages haunted by vengeful spirits",
            Self::JungleRuins => "Overgrown temples reclaimed by savage beasts and nature",
            Self::CrystalCaves => "Magical caverns where elementals guard arcane treasures",
            Self::VoidTunnels => "Reality-warped tunnels where aberrations dwell",
            Self::DivineRealm => "Sacred halls where celestials test the worthy",
            Self::Abyss => "The realm of pure chaos where only the strongest survive",
            Self::SecretRealm => "A hidden dimension between worlds",
        }
    }

    /// Returns the primary enemy types for this theme
    pub fn enemy_types(&self) -> &'static [&'static str] {
        match self {
            Self::Catacombs => &["skeleton", "zombie", "ghost", "wraith", "vampire"],
            Self::Sewers => &["rat", "slime", "giant_spider", "crocodile", "plague_bearer"],
            Self::Mines => &["golem", "earth_elemental", "kobold", "dwarf_ghost", "crystal_guardian"],
            Self::VolcanicDepths => &["fire_elemental", "imp", "hellhound", "demon", "magma_golem"],
            Self::FrozenCaverns => &["ice_elemental", "frost_spirit", "yeti", "ice_wraith", "frozen_zombie"],
            Self::JungleRuins => &["giant_snake", "panther", "treant", "jungle_troll", "ancient_guardian"],
            Self::CrystalCaves => &["crystal_golem", "arcane_elemental", "mana_wyrm", "spell_weaver", "gem_dragon"],
            Self::VoidTunnels => &["void_spawn", "tentacle_horror", "mind_flayer", "beholder", "dimensional_shambler"],
            Self::DivineRealm => &["angel", "valkyrie", "celestial_guardian", "seraph", "divine_construct"],
            Self::Abyss => &["chaos_demon", "elder_horror", "void_lord", "entropy_beast", "primordial"],
            Self::SecretRealm => &["guardian_spirit", "treasure_mimic", "ancient_construct", "secret_keeper"],
        }
    }

    /// Returns the hazard tile for this theme
    pub fn hazard_tile(&self) -> Option<Tile> {
        match self {
            Self::Catacombs => Some(Tile::PoisonGas),
            Self::Sewers => Some(Tile::ToxicPool),
            Self::Mines => None,
            Self::VolcanicDepths => Some(Tile::Lava),
            Self::FrozenCaverns => Some(Tile::IceSpikes),
            Self::JungleRuins => Some(Tile::PoisonGas),
            Self::CrystalCaves => None,
            Self::VoidTunnels => Some(Tile::VoidTendril),
            Self::DivineRealm => None,
            Self::Abyss => Some(Tile::ChaosField),
            Self::SecretRealm => None,
        }
    }

    /// Returns decorative tiles for this theme
    pub fn decoration_tiles(&self) -> &'static [Tile] {
        match self {
            Self::Catacombs => &[Tile::Bones, Tile::Cobweb, Tile::Sarcophagus],
            Self::Sewers => &[Tile::SewerGrate, Tile::ToxicPool, Tile::Water],
            Self::Mines => &[Tile::MineCart, Tile::Rails, Tile::GemDeposit],
            Self::VolcanicDepths => &[Tile::MagmaVent, Tile::Lava, Tile::ObsidianFloor],
            Self::FrozenCaverns => &[Tile::FrozenPool, Tile::Snowdrift, Tile::IceSpikes],
            Self::JungleRuins => &[Tile::JungleVines, Tile::AncientStatue, Tile::Grass],
            Self::CrystalCaves => &[Tile::CrystalFormation, Tile::MagicCircle, Tile::GemDeposit],
            Self::VoidTunnels => &[Tile::VoidRift, Tile::Tentacle, Tile::VoidTendril],
            Self::DivineRealm => &[Tile::DivineLight, Tile::HolyAltar, Tile::Pillar],
            Self::Abyss => &[Tile::ChaosPortal, Tile::AbyssalPit, Tile::ChaosField],
            Self::SecretRealm => &[Tile::MagicCircle, Tile::SecretPortal, Tile::GoldPile],
        }
    }
}

/// Floor type classification
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum FloorType {
    /// Regular floor with standard enemies and loot
    Normal,
    /// Every 5th floor (5, 15, 25, etc.) - has a mini-boss
    MiniBoss,
    /// Every 10th floor - major boss encounter
    Boss,
    /// Hidden floors accessible via portals
    Secret,
    /// Transitional floor between themes
    Transition,
}

impl FloorType {
    /// Determines the floor type from a level number
    pub fn from_level(level: u32, is_secret: bool) -> Self {
        if is_secret {
            Self::Secret
        } else if BOSS_LEVELS.contains(&level) {
            Self::Boss
        } else if MINI_BOSS_LEVELS.contains(&level) {
            Self::MiniBoss
        } else if level % 10 == 1 && level > 1 {
            Self::Transition
        } else {
            Self::Normal
        }
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Dungeon Floor",
            Self::MiniBoss => "Mini-Boss Floor",
            Self::Boss => "Boss Floor",
            Self::Secret => "Secret Floor",
            Self::Transition => "Transitional Floor",
        }
    }
}

/// Floor modifiers that affect gameplay on specific floors
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FloorModifiers {
    /// Enemy difficulty multiplier (1.0 = normal)
    pub enemy_difficulty: f32,
    /// Loot quality multiplier (1.0 = normal)
    pub loot_quality: f32,
    /// Environmental hazard frequency (0.0-1.0)
    pub hazard_frequency: f32,
    /// Visibility range modifier (1.0 = normal VIEW_RADIUS)
    pub visibility_modifier: f32,
    /// Special mechanics active on this floor
    pub special_mechanics: Vec<SpecialMechanic>,
    /// Enemy spawn rate multiplier
    pub spawn_rate: f32,
    /// Experience multiplier
    pub exp_multiplier: f32,
    /// Gold drop multiplier
    pub gold_multiplier: f32,
}

impl FloorModifiers {
    /// Create modifiers for a specific floor
    pub fn for_level(level: u32, floor_type: FloorType) -> Self {
        let base_difficulty = 1.0 + (level as f32 - 1.0) * 0.05;
        let theme = DungeonTheme::from_level(level);

        let mut modifiers = Self {
            enemy_difficulty: base_difficulty,
            loot_quality: 1.0 + (level as f32 - 1.0) * 0.03,
            hazard_frequency: Self::calculate_hazard_frequency(level, &theme),
            visibility_modifier: Self::calculate_visibility(level, &theme),
            special_mechanics: Self::get_mechanics_for_theme(&theme, level),
            spawn_rate: 1.0,
            exp_multiplier: 1.0 + (level as f32 - 1.0) * 0.02,
            gold_multiplier: 1.0 + (level as f32 - 1.0) * 0.04,
        };

        // Adjust based on floor type
        match floor_type {
            FloorType::MiniBoss => {
                modifiers.enemy_difficulty *= 1.3;
                modifiers.loot_quality *= 1.5;
                modifiers.spawn_rate *= 0.7;
                modifiers.exp_multiplier *= 1.5;
            }
            FloorType::Boss => {
                modifiers.enemy_difficulty *= 1.5;
                modifiers.loot_quality *= 2.0;
                modifiers.spawn_rate *= 0.5;
                modifiers.exp_multiplier *= 2.0;
                modifiers.gold_multiplier *= 2.5;
            }
            FloorType::Secret => {
                modifiers.loot_quality *= 3.0;
                modifiers.exp_multiplier *= 2.5;
                modifiers.gold_multiplier *= 5.0;
                modifiers.spawn_rate *= 0.5;
            }
            FloorType::Transition => {
                modifiers.enemy_difficulty *= 0.8;
                modifiers.hazard_frequency *= 0.5;
            }
            FloorType::Normal => {}
        }

        modifiers
    }

    fn calculate_hazard_frequency(level: u32, theme: &DungeonTheme) -> f32 {
        let base = match theme {
            DungeonTheme::Catacombs => 0.1,
            DungeonTheme::Sewers => 0.25,
            DungeonTheme::Mines => 0.15,
            DungeonTheme::VolcanicDepths => 0.3,
            DungeonTheme::FrozenCaverns => 0.2,
            DungeonTheme::JungleRuins => 0.2,
            DungeonTheme::CrystalCaves => 0.1,
            DungeonTheme::VoidTunnels => 0.35,
            DungeonTheme::DivineRealm => 0.05,
            DungeonTheme::Abyss => 0.4,
            DungeonTheme::SecretRealm => 0.1,
        };
        (base + (level as f32 * 0.005)).min(0.5)
    }

    fn calculate_visibility(level: u32, theme: &DungeonTheme) -> f32 {
        match theme {
            DungeonTheme::Catacombs => 0.9,
            DungeonTheme::Sewers => 0.7,
            DungeonTheme::Mines => 0.8,
            DungeonTheme::VolcanicDepths => 0.85,
            DungeonTheme::FrozenCaverns => 0.75,
            DungeonTheme::JungleRuins => 0.65,
            DungeonTheme::CrystalCaves => 1.2,  // Crystals provide light
            DungeonTheme::VoidTunnels => 0.5,
            DungeonTheme::DivineRealm => 1.5,   // Divine light
            DungeonTheme::Abyss => 0.4,
            DungeonTheme::SecretRealm => 1.0,
        }
    }

    fn get_mechanics_for_theme(theme: &DungeonTheme, level: u32) -> Vec<SpecialMechanic> {
        let mut mechanics = Vec::new();

        match theme {
            DungeonTheme::Catacombs => {
                mechanics.push(SpecialMechanic::UndeadRising);
                if level >= 5 {
                    mechanics.push(SpecialMechanic::Darkness);
                }
            }
            DungeonTheme::Sewers => {
                mechanics.push(SpecialMechanic::FloodingWaters);
                mechanics.push(SpecialMechanic::PoisonCloud);
            }
            DungeonTheme::Mines => {
                mechanics.push(SpecialMechanic::CaveIn);
                if level >= 25 {
                    mechanics.push(SpecialMechanic::MineralRich);
                }
            }
            DungeonTheme::VolcanicDepths => {
                mechanics.push(SpecialMechanic::LavaFlow);
                mechanics.push(SpecialMechanic::HeatWave);
            }
            DungeonTheme::FrozenCaverns => {
                mechanics.push(SpecialMechanic::Freezing);
                mechanics.push(SpecialMechanic::SlipperyIce);
            }
            DungeonTheme::JungleRuins => {
                mechanics.push(SpecialMechanic::Overgrowth);
                mechanics.push(SpecialMechanic::AmbushPredators);
            }
            DungeonTheme::CrystalCaves => {
                mechanics.push(SpecialMechanic::ManaResonance);
                mechanics.push(SpecialMechanic::CrystalReflection);
            }
            DungeonTheme::VoidTunnels => {
                mechanics.push(SpecialMechanic::RealityWarp);
                mechanics.push(SpecialMechanic::VoidCorruption);
            }
            DungeonTheme::DivineRealm => {
                mechanics.push(SpecialMechanic::DivineJudgment);
                mechanics.push(SpecialMechanic::Purification);
            }
            DungeonTheme::Abyss => {
                mechanics.push(SpecialMechanic::ChaosStorm);
                mechanics.push(SpecialMechanic::RealityCollapse);
                mechanics.push(SpecialMechanic::FinalBoss);
            }
            DungeonTheme::SecretRealm => {
                mechanics.push(SpecialMechanic::TreasureHoard);
                mechanics.push(SpecialMechanic::AncientGuardians);
            }
        }

        mechanics
    }
}

impl Default for FloorModifiers {
    fn default() -> Self {
        Self {
            enemy_difficulty: 1.0,
            loot_quality: 1.0,
            hazard_frequency: 0.1,
            visibility_modifier: 1.0,
            special_mechanics: Vec::new(),
            spawn_rate: 1.0,
            exp_multiplier: 1.0,
            gold_multiplier: 1.0,
        }
    }
}

/// Special mechanics that can be active on a floor
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SpecialMechanic {
    // Catacombs mechanics
    UndeadRising,      // Dead enemies may rise again
    Darkness,          // Reduced visibility

    // Sewers mechanics
    FloodingWaters,    // Water level rises periodically
    PoisonCloud,       // Poison gas clouds appear

    // Mines mechanics
    CaveIn,            // Random cave-ins block paths
    MineralRich,       // Extra ore and gem spawns

    // Volcanic mechanics
    LavaFlow,          // Lava spreads over time
    HeatWave,          // Periodic fire damage

    // Frozen mechanics
    Freezing,          // Can become frozen, slowing movement
    SlipperyIce,       // Sliding on ice

    // Jungle mechanics
    Overgrowth,        // Vines grow and block paths
    AmbushPredators,   // Enemies can ambush from hiding

    // Crystal mechanics
    ManaResonance,     // Spells are amplified
    CrystalReflection, // Projectiles can reflect

    // Void mechanics
    RealityWarp,       // Random teleportation
    VoidCorruption,    // Sanity damage over time

    // Divine mechanics
    DivineJudgment,    // Periodic judgment checks
    Purification,      // Evil items are destroyed

    // Abyss mechanics
    ChaosStorm,        // Random effects occur
    RealityCollapse,   // Floor layout shifts
    FinalBoss,         // Final boss mechanics

    // Secret mechanics
    TreasureHoard,     // Extra treasure
    AncientGuardians,  // Powerful but rewarding guardians
}

impl SpecialMechanic {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::UndeadRising => "Undead Rising",
            Self::Darkness => "Eternal Darkness",
            Self::FloodingWaters => "Flooding Waters",
            Self::PoisonCloud => "Poison Clouds",
            Self::CaveIn => "Unstable Tunnels",
            Self::MineralRich => "Mineral Rich",
            Self::LavaFlow => "Lava Flow",
            Self::HeatWave => "Heat Wave",
            Self::Freezing => "Bone-Chilling Cold",
            Self::SlipperyIce => "Slippery Ice",
            Self::Overgrowth => "Rapid Overgrowth",
            Self::AmbushPredators => "Lurking Predators",
            Self::ManaResonance => "Mana Resonance",
            Self::CrystalReflection => "Crystal Reflection",
            Self::RealityWarp => "Reality Warp",
            Self::VoidCorruption => "Void Corruption",
            Self::DivineJudgment => "Divine Judgment",
            Self::Purification => "Purifying Light",
            Self::ChaosStorm => "Chaos Storm",
            Self::RealityCollapse => "Reality Collapse",
            Self::FinalBoss => "Final Confrontation",
            Self::TreasureHoard => "Treasure Hoard",
            Self::AncientGuardians => "Ancient Guardians",
        }
    }

    /// Returns the description
    pub fn description(&self) -> &'static str {
        match self {
            Self::UndeadRising => "Slain enemies may rise again as undead",
            Self::Darkness => "Vision is severely limited in the eternal dark",
            Self::FloodingWaters => "Water levels rise and fall periodically",
            Self::PoisonCloud => "Toxic clouds drift through the corridors",
            Self::CaveIn => "Tunnels may collapse, blocking paths",
            Self::MineralRich => "Rich mineral deposits yield extra resources",
            Self::LavaFlow => "Rivers of lava slowly spread across the floor",
            Self::HeatWave => "Waves of intense heat cause periodic damage",
            Self::Freezing => "The bitter cold can freeze you in place",
            Self::SlipperyIce => "Ice causes uncontrolled sliding",
            Self::Overgrowth => "Vines rapidly grow, blocking passages",
            Self::AmbushPredators => "Enemies may ambush from concealment",
            Self::ManaResonance => "Magical energy is amplified here",
            Self::CrystalReflection => "Crystals reflect projectiles unpredictably",
            Self::RealityWarp => "Reality shifts, causing random teleportation",
            Self::VoidCorruption => "Void energy corrupts mind and body",
            Self::DivineJudgment => "Divine beings judge your worthiness",
            Self::Purification => "Holy light destroys cursed items",
            Self::ChaosStorm => "Reality itself is unstable here",
            Self::RealityCollapse => "The floor layout shifts unpredictably",
            Self::FinalBoss => "The ultimate challenge awaits",
            Self::TreasureHoard => "Ancient treasures fill this hidden realm",
            Self::AncientGuardians => "Powerful guardians protect ancient secrets",
        }
    }
}

/// Special room types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SpecialRoomType {
    Normal,
    TreasureVault,
    BossArena,
    MiniBossArena,
    PuzzleRoom,
    TrapGauntlet,
    MerchantCamp,
    Shrine,
    SecretPortalRoom,
}

impl SpecialRoomType {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Room",
            Self::TreasureVault => "Treasure Vault",
            Self::BossArena => "Boss Arena",
            Self::MiniBossArena => "Mini-Boss Arena",
            Self::PuzzleRoom => "Puzzle Chamber",
            Self::TrapGauntlet => "Trap Gauntlet",
            Self::MerchantCamp => "Merchant Camp",
            Self::Shrine => "Shrine Room",
            Self::SecretPortalRoom => "Secret Portal Room",
        }
    }

    /// Returns whether enemies can spawn in this room type
    pub fn allows_enemies(&self) -> bool {
        match self {
            Self::Normal => true,
            Self::TreasureVault => true,
            Self::BossArena => true,
            Self::MiniBossArena => true,
            Self::PuzzleRoom => false,
            Self::TrapGauntlet => false,
            Self::MerchantCamp => false,
            Self::Shrine => false,
            Self::SecretPortalRoom => false,
        }
    }

    /// Returns minimum size for this room type
    pub fn min_size(&self) -> usize {
        match self {
            Self::Normal => MIN_ROOM_SIZE,
            Self::TreasureVault => SPECIAL_ROOM_MIN_SIZE,
            Self::BossArena => BOSS_ARENA_MIN_SIZE,
            Self::MiniBossArena => SPECIAL_ROOM_MIN_SIZE + 2,
            Self::PuzzleRoom => SPECIAL_ROOM_MIN_SIZE,
            Self::TrapGauntlet => SPECIAL_ROOM_MIN_SIZE + 2,
            Self::MerchantCamp => SPECIAL_ROOM_MIN_SIZE,
            Self::Shrine => MIN_ROOM_SIZE + 2,
            Self::SecretPortalRoom => MIN_ROOM_SIZE,
        }
    }

    /// Returns maximum size for this room type
    pub fn max_size(&self) -> usize {
        match self {
            Self::Normal => MAX_ROOM_SIZE,
            Self::TreasureVault => SPECIAL_ROOM_MAX_SIZE,
            Self::BossArena => BOSS_ARENA_MAX_SIZE,
            Self::MiniBossArena => SPECIAL_ROOM_MAX_SIZE + 2,
            Self::PuzzleRoom => SPECIAL_ROOM_MAX_SIZE,
            Self::TrapGauntlet => SPECIAL_ROOM_MAX_SIZE + 2,
            Self::MerchantCamp => SPECIAL_ROOM_MAX_SIZE,
            Self::Shrine => MIN_ROOM_SIZE + 4,
            Self::SecretPortalRoom => MIN_ROOM_SIZE + 2,
        }
    }
}

/// Secret floor information
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SecretFloor {
    /// The secret floor index (1-10)
    pub index: u32,
    /// The base level this secret floor is associated with
    pub base_level: u32,
    /// Whether this secret floor has been discovered
    pub discovered: bool,
    /// Whether this secret floor has been completed
    pub completed: bool,
    /// Special rewards available on this floor
    pub rewards: Vec<String>,
}

impl SecretFloor {
    /// Create a new secret floor
    pub fn new(index: u32, base_level: u32) -> Self {
        Self {
            index,
            base_level,
            discovered: false,
            completed: false,
            rewards: Self::generate_rewards(index),
        }
    }

    fn generate_rewards(index: u32) -> Vec<String> {
        match index {
            1 => vec!["Rare Weapon".to_string(), "Gold Hoard".to_string()],
            2 => vec!["Legendary Armor".to_string(), "Skill Tome".to_string()],
            3 => vec!["Unique Artifact".to_string(), "Rare Materials".to_string()],
            4 => vec!["Epic Accessory".to_string(), "Ancient Scroll".to_string()],
            5 => vec!["Mythic Weapon".to_string(), "Boss Key".to_string()],
            6 => vec!["Divine Blessing".to_string(), "Rare Pet Egg".to_string()],
            7 => vec!["Void Essence".to_string(), "Reality Fragment".to_string()],
            8 => vec!["Celestial Gear".to_string(), "Divine Favor".to_string()],
            9 => vec!["Chaos Crystal".to_string(), "Ultimate Power".to_string()],
            10 => vec!["Final Secret".to_string(), "True Ending Key".to_string()],
            _ => vec!["Mystery Reward".to_string()],
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
    pub room_type: SpecialRoomType,
    pub cleared: bool,
    pub difficulty: u8,
}

impl Room {
    /// Create a new room
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            is_boss_room: false,
            room_type: SpecialRoomType::Normal,
            cleared: false,
            difficulty: 1,
        }
    }

    /// Create a new room with a specific type
    pub fn new_special(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        room_type: SpecialRoomType,
        difficulty: u8,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            is_boss_room: matches!(room_type, SpecialRoomType::BossArena),
            room_type,
            cleared: false,
            difficulty,
        }
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

    /// Returns all floor positions in this room
    pub fn floor_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in (self.y + 1)..(self.y + self.height - 1) {
            for x in (self.x + 1)..(self.x + self.width - 1) {
                positions.push((x, y));
            }
        }
        positions
    }

    /// Returns the corner positions of the room (inside)
    pub fn corner_positions(&self) -> [(usize, usize); 4] {
        [
            (self.x + 1, self.y + 1),
            (self.x + self.width - 2, self.y + 1),
            (self.x + 1, self.y + self.height - 2),
            (self.x + self.width - 2, self.y + self.height - 2),
        ]
    }

    /// Returns whether a position is inside this room
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x > self.x && x < self.x + self.width - 1 && y > self.y && y < self.y + self.height - 1
    }

    /// Marks this room as cleared
    pub fn mark_cleared(&mut self) {
        self.cleared = true;
    }
}

/// The dungeon map
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub visible: Vec<Vec<bool>>,
    pub explored: Vec<Vec<bool>>,
    pub rooms: Vec<Room>,
    pub theme: DungeonTheme,
    pub floor_type: FloorType,
    pub modifiers: FloorModifiers,
    pub special_room_indices: Vec<usize>,
    pub has_merchant: bool,
    pub secret_portal_location: Option<(usize, usize)>,
    pub level: u32,
}

impl Map {
    /// Create a new empty map
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Wall; MAP_WIDTH]; MAP_HEIGHT],
            visible: vec![vec![false; MAP_WIDTH]; MAP_HEIGHT],
            explored: vec![vec![false; MAP_WIDTH]; MAP_HEIGHT],
            rooms: Vec::new(),
            theme: DungeonTheme::Catacombs,
            floor_type: FloorType::Normal,
            modifiers: FloorModifiers::default(),
            special_room_indices: Vec::new(),
            has_merchant: false,
            secret_portal_location: None,
            level: 1,
        }
    }

    /// Generate a new dungeon level
    pub fn generate(&mut self, rng: &mut impl Rng, level: u32) {
        self.level = level;
        self.theme = DungeonTheme::from_level(level);
        self.floor_type = FloorType::from_level(level, false);
        self.modifiers = FloorModifiers::for_level(level, self.floor_type);
        let floor_tile = self.theme.floor_tile();

        // Reset
        self.tiles = vec![vec![Tile::Wall; MAP_WIDTH]; MAP_HEIGHT];
        self.visible = vec![vec![false; MAP_WIDTH]; MAP_HEIGHT];
        self.rooms.clear();
        self.special_room_indices.clear();
        self.has_merchant = false;
        self.secret_portal_location = None;

        let is_boss_level = BOSS_LEVELS.contains(&level);
        let is_mini_boss_level = MINI_BOSS_LEVELS.contains(&level);

        // Generate rooms
        let target_rooms = if is_boss_level {
            MAX_ROOMS + 2
        } else if is_mini_boss_level {
            MAX_ROOMS + 1
        } else {
            MAX_ROOMS
        };

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

                    if rng.gen_bool(0.3) {
                        self.tiles[new_y][prev_x] = Tile::Door;
                    }
                }

                self.rooms.push(new_room);
            }
        }

        // Add theme-specific features
        self.add_theme_features(rng, level);

        // Add special features
        self.add_features(rng, level);

        // Generate special rooms based on level
        self.generate_special_rooms(rng, level);

        // Add secret portal chance
        self.maybe_add_secret_portal(rng, level);

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

    /// Generate a secret floor
    pub fn generate_secret(&mut self, rng: &mut impl Rng, secret_index: u32) {
        self.level = 1000 + secret_index; // Use high level for secret floors
        self.theme = DungeonTheme::SecretRealm;
        self.floor_type = FloorType::Secret;
        self.modifiers = FloorModifiers::for_level(secret_index * 10, FloorType::Secret);
        let floor_tile = self.theme.floor_tile();

        // Reset
        self.tiles = vec![vec![Tile::Wall; MAP_WIDTH]; MAP_HEIGHT];
        self.visible = vec![vec![false; MAP_WIDTH]; MAP_HEIGHT];
        self.rooms.clear();
        self.special_room_indices.clear();
        self.has_merchant = false;
        self.secret_portal_location = None;

        // Secret floors have fewer but larger rooms with more treasure
        let target_rooms = MAX_ROOMS / 2;

        for _ in 0..target_rooms * 4 {
            if self.rooms.len() >= target_rooms {
                break;
            }

            let width = rng.gen_range(MAX_ROOM_SIZE - 2..=MAX_ROOM_SIZE + 4);
            let height = rng.gen_range(MAX_ROOM_SIZE - 2..=MAX_ROOM_SIZE + 4);
            let x = rng.gen_range(1..MAP_WIDTH.saturating_sub(width + 1));
            let y = rng.gen_range(1..MAP_HEIGHT.saturating_sub(height + 1));

            if x + width >= MAP_WIDTH || y + height >= MAP_HEIGHT {
                continue;
            }

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
                }

                self.rooms.push(new_room);
            }
        }

        // Add lots of treasure to secret floors
        self.add_secret_floor_treasure(rng);

        // Add return portal in the last room
        if let Some(last_room) = self.rooms.last() {
            let (px, py) = last_room.center();
            self.tiles[py][px] = Tile::SecretPortal;
        }
    }

    fn add_secret_floor_treasure(&mut self, rng: &mut impl Rng) {
        for room in &self.rooms {
            // Add multiple chests per room
            for _ in 0..rng.gen_range(2..5) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::Chest;
                }
            }

            // Add gold piles
            for _ in 0..rng.gen_range(3..7) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::GoldPile;
                }
            }

            // Add gem deposits
            for _ in 0..rng.gen_range(1..4) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::GemDeposit;
                }
            }
        }
    }

    fn maybe_add_secret_portal(&mut self, rng: &mut impl Rng, level: u32) {
        // Secret portals appear roughly every 10 levels with 15% chance
        if level % 10 == 0 || rng.gen_bool(0.05) {
            if self.rooms.len() > 3 {
                let room_idx = rng.gen_range(1..self.rooms.len() - 1);
                let room = &self.rooms[room_idx];
                let (x, y) = room.random_point(rng);

                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::SecretDoor;
                    self.secret_portal_location = Some((x, y));
                }
            }
        }
    }

    fn add_theme_features(&mut self, rng: &mut impl Rng, _level: u32) {
        let decorations = self.theme.decoration_tiles();

        for room in self.rooms.clone() {
            // Add theme-specific decorations
            let decoration_count = rng.gen_range(1..4);
            for _ in 0..decoration_count {
                if !decorations.is_empty() {
                    let (x, y) = room.random_point(rng);
                    if self.tiles[y][x].walkable() {
                        let decoration = decorations[rng.gen_range(0..decorations.len())];
                        // Don't place blocking decorations
                        if !decoration.blocks_sight() {
                            self.tiles[y][x] = decoration;
                        }
                    }
                }
            }

            // Add hazards based on theme
            if let Some(hazard) = self.theme.hazard_tile() {
                if rng.gen_bool(self.modifiers.hazard_frequency as f64) {
                    let (x, y) = room.random_point(rng);
                    if self.tiles[y][x].walkable() {
                        self.tiles[y][x] = hazard;
                    }
                }
            }
        }
    }

    fn generate_special_rooms(&mut self, rng: &mut impl Rng, level: u32) {
        let is_boss_level = BOSS_LEVELS.contains(&level);
        let is_mini_boss_level = MINI_BOSS_LEVELS.contains(&level);

        // Always add boss/mini-boss arena on appropriate floors
        if is_boss_level && self.rooms.len() > 1 {
            let last_idx = self.rooms.len() - 1;
            self.rooms[last_idx].room_type = SpecialRoomType::BossArena;
            self.rooms[last_idx].is_boss_room = true;
            self.rooms[last_idx].difficulty = 10;
            self.special_room_indices.push(last_idx);
        } else if is_mini_boss_level && self.rooms.len() > 1 {
            let last_idx = self.rooms.len() - 1;
            self.rooms[last_idx].room_type = SpecialRoomType::MiniBossArena;
            self.rooms[last_idx].difficulty = 7;
            self.special_room_indices.push(last_idx);
        }

        // Add other special rooms based on level
        for room_idx in 1..self.rooms.len().saturating_sub(1) {
            let room_type = self.determine_special_room_type(rng, level);
            if room_type != SpecialRoomType::Normal {
                self.rooms[room_idx].room_type = room_type;
                self.rooms[room_idx].difficulty = (level / 10 + 1).min(10) as u8;
                self.special_room_indices.push(room_idx);
            }
        }
    }

    fn determine_special_room_type(&self, rng: &mut impl Rng, level: u32) -> SpecialRoomType {
        let roll = rng.gen::<f64>();

        // Merchant camps are more common on certain levels
        if level % 5 == 0 && !self.has_merchant && roll < 0.3 {
            return SpecialRoomType::MerchantCamp;
        }

        // Other special rooms based on level progression
        if level >= 5 && roll < 0.08 {
            return SpecialRoomType::TreasureVault;
        }
        if level >= 3 && roll < 0.12 {
            return SpecialRoomType::PuzzleRoom;
        }
        if level >= 7 && roll < 0.10 {
            return SpecialRoomType::TrapGauntlet;
        }
        if roll < 0.15 {
            return SpecialRoomType::Shrine;
        }

        SpecialRoomType::Normal
    }

    /// Carve out a room
    fn carve_room(&mut self, room: &Room, floor_tile: Tile) {
        for y in room.y..room.y + room.height {
            for x in room.x..room.x + room.width {
                if y < MAP_HEIGHT && x < MAP_WIDTH {
                    self.tiles[y][x] = floor_tile;
                }
            }
        }
    }

    /// Carve a horizontal tunnel
    fn carve_h_tunnel(&mut self, x1: usize, x2: usize, y: usize, floor_tile: Tile) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            if y < MAP_HEIGHT && x < MAP_WIDTH && self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    /// Carve a vertical tunnel
    fn carve_v_tunnel(&mut self, y1: usize, y2: usize, x: usize, floor_tile: Tile) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            if y < MAP_HEIGHT && x < MAP_WIDTH && self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    /// Add special features to the dungeon
    fn add_features(&mut self, rng: &mut impl Rng, level: u32) {
        // Add traps (more on deeper levels)
        let trap_chance = 0.15 + (level as f64 * 0.005);
        for room in &self.rooms[1..] {
            if rng.gen_bool(trap_chance.min(0.35)) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() && self.tiles[y][x] != Tile::StairsDown {
                    self.tiles[y][x] = Tile::Trap;
                }
            }
        }

        // Add chests
        let chest_chance = 0.12 + (level as f64 * 0.003);
        for room in &self.rooms[1..] {
            if rng.gen_bool(chest_chance.min(0.25)) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::Chest;
                }
            }
        }

        // Add shrines
        self.spawn_shrines(rng, level);

        // Add water/lava pools based on theme
        match self.theme {
            DungeonTheme::Sewers => {
                for room in &self.rooms.clone() {
                    if rng.gen_bool(0.2) {
                        let (x, y) = room.random_point(rng);
                        for dy in 0..4 {
                            for dx in 0..4 {
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
            DungeonTheme::VolcanicDepths | DungeonTheme::Abyss => {
                for room in &self.rooms.clone() {
                    if rng.gen_bool(0.15) {
                        let (x, y) = room.random_point(rng);
                        for dy in 0..3 {
                            for dx in 0..3 {
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
            DungeonTheme::FrozenCaverns => {
                for room in &self.rooms.clone() {
                    if rng.gen_bool(0.15) {
                        let (x, y) = room.random_point(rng);
                        for dy in 0..3 {
                            for dx in 0..3 {
                                let nx = x + dx;
                                let ny = y + dy;
                                if nx < MAP_WIDTH && ny < MAP_HEIGHT && self.tiles[ny][nx].walkable() {
                                    self.tiles[ny][nx] = Tile::FrozenPool;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // Add pillars in larger rooms
        for room in &self.rooms.clone() {
            if room.width > 8 && room.height > 8 && rng.gen_bool(0.3) {
                let cx = room.x + room.width / 2;
                let cy = room.y + room.height / 2;
                for &(dx, dy) in &[(-2i32, -2i32), (2, -2), (-2, 2), (2, 2)] {
                    let px = (cx as i32 + dx) as usize;
                    let py = (cy as i32 + dy) as usize;
                    if px < MAP_WIDTH && py < MAP_HEIGHT && self.tiles[py][px].walkable() {
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

        let base_shrine_chance = 0.12 + level as f64 * 0.015;
        let max_shrines = 1 + (level / 8) as usize;
        let mut shrines_placed = 0;

        let shrine_types = self.get_shrine_types_for_theme();

        for room_idx in 1..self.rooms.len() {
            if shrines_placed >= max_shrines {
                break;
            }

            if rng.gen_bool(base_shrine_chance) {
                let room = &self.rooms[room_idx];
                let (x, y) = room.center();

                if self.tiles[y][x].walkable() && !self.tiles[y][x].is_shrine() {
                    let shrine_type = self.choose_weighted_shrine(rng, &shrine_types);
                    self.tiles[y][x] = shrine_type;
                    shrines_placed += 1;
                }
            }
        }

        // Guaranteed shrine on milestone levels
        if level % 10 == 0 && shrines_placed == 0 && self.rooms.len() > 2 {
            let room_idx = rng.gen_range(1..self.rooms.len() - 1);
            let room = &self.rooms[room_idx];
            let (x, y) = room.center();
            if self.tiles[y][x].walkable() {
                let shrine_type = self.choose_weighted_shrine(rng, &shrine_types);
                self.tiles[y][x] = shrine_type;
            }
        }
    }

    fn get_shrine_types_for_theme(&self) -> Vec<(Tile, f64)> {
        match self.theme {
            DungeonTheme::Catacombs => vec![
                (Tile::CurseShrine, 0.20),
                (Tile::SacrificeShrine, 0.20),
                (Tile::Shrine, 0.15),
                (Tile::WisdomShrine, 0.15),
                (Tile::HealingShrine, 0.15),
                (Tile::BlessingShrine, 0.15),
            ],
            DungeonTheme::Sewers => vec![
                (Tile::Shrine, 0.20),
                (Tile::StatShrine, 0.20),
                (Tile::HealingShrine, 0.20),
                (Tile::ChaosShrine, 0.15),
                (Tile::LuckShrine, 0.15),
                (Tile::TeleportShrine, 0.10),
            ],
            DungeonTheme::Mines => vec![
                (Tile::StatShrine, 0.25),
                (Tile::LuckShrine, 0.20),
                (Tile::WarShrine, 0.20),
                (Tile::Shrine, 0.15),
                (Tile::HealingShrine, 0.20),
            ],
            DungeonTheme::VolcanicDepths => vec![
                (Tile::WarShrine, 0.25),
                (Tile::SacrificeShrine, 0.20),
                (Tile::ChaosShrine, 0.20),
                (Tile::StatShrine, 0.15),
                (Tile::CurseShrine, 0.20),
            ],
            DungeonTheme::FrozenCaverns => vec![
                (Tile::WisdomShrine, 0.25),
                (Tile::HealingShrine, 0.20),
                (Tile::StatShrine, 0.15),
                (Tile::TeleportShrine, 0.15),
                (Tile::Shrine, 0.15),
                (Tile::LuckShrine, 0.10),
            ],
            DungeonTheme::JungleRuins => vec![
                (Tile::HealingShrine, 0.25),
                (Tile::BlessingShrine, 0.20),
                (Tile::StatShrine, 0.15),
                (Tile::WisdomShrine, 0.15),
                (Tile::LuckShrine, 0.15),
                (Tile::Shrine, 0.10),
            ],
            DungeonTheme::CrystalCaves => vec![
                (Tile::WisdomShrine, 0.25),
                (Tile::TeleportShrine, 0.20),
                (Tile::StatShrine, 0.20),
                (Tile::BlessingShrine, 0.15),
                (Tile::LuckShrine, 0.20),
            ],
            DungeonTheme::VoidTunnels => vec![
                (Tile::ChaosShrine, 0.25),
                (Tile::TeleportShrine, 0.20),
                (Tile::CurseShrine, 0.20),
                (Tile::SacrificeShrine, 0.15),
                (Tile::Shrine, 0.20),
            ],
            DungeonTheme::DivineRealm => vec![
                (Tile::BlessingShrine, 0.30),
                (Tile::HealingShrine, 0.25),
                (Tile::WisdomShrine, 0.20),
                (Tile::StatShrine, 0.15),
                (Tile::Shrine, 0.10),
            ],
            DungeonTheme::Abyss => vec![
                (Tile::ChaosShrine, 0.25),
                (Tile::SacrificeShrine, 0.20),
                (Tile::CurseShrine, 0.20),
                (Tile::WarShrine, 0.20),
                (Tile::StatShrine, 0.15),
            ],
            DungeonTheme::SecretRealm => vec![
                (Tile::BlessingShrine, 0.25),
                (Tile::LuckShrine, 0.25),
                (Tile::StatShrine, 0.20),
                (Tile::WisdomShrine, 0.15),
                (Tile::HealingShrine, 0.15),
            ],
        }
    }

    fn choose_weighted_shrine(&self, rng: &mut impl Rng, weights: &[(Tile, f64)]) -> Tile {
        let total_weight: f64 = weights.iter().map(|(_, w)| w).sum();
        let mut roll = rng.gen::<f64>() * total_weight;

        for (tile, weight) in weights {
            roll -= weight;
            if roll <= 0.0 {
                return *tile;
            }
        }

        Tile::Shrine
    }

    /// Compute field of view from a position
    pub fn compute_fov(&mut self, px: usize, py: usize) {
        for row in &mut self.visible {
            for cell in row {
                *cell = false;
            }
        }

        let effective_radius = (VIEW_RADIUS as f32 * self.modifiers.visibility_modifier) as i32;

        for angle in 0..360 {
            let rad = (angle as f32) * std::f32::consts::PI / 180.0;
            let dx = rad.cos();
            let dy = rad.sin();

            let mut x = px as f32 + 0.5;
            let mut y = py as f32 + 0.5;

            for _ in 0..effective_radius {
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

    /// Returns the tile at a position
    pub fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < MAP_WIDTH && y < MAP_HEIGHT {
            Some(self.tiles[y][x])
        } else {
            None
        }
    }

    /// Sets the tile at a position
    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < MAP_WIDTH && y < MAP_HEIGHT {
            self.tiles[y][x] = tile;
        }
    }

    /// Check if a level is a boss level
    pub fn is_boss_level(level: u32) -> bool {
        BOSS_LEVELS.contains(&level)
    }

    /// Check if a level is a mini-boss level
    pub fn is_mini_boss_level(level: u32) -> bool {
        MINI_BOSS_LEVELS.contains(&level)
    }

    /// Get the boss name for a level
    pub fn get_boss_name(level: u32) -> Option<&'static str> {
        match level {
            10 => Some("The Crypt Lord"),
            20 => Some("The Sewer King"),
            30 => Some("The Mine Colossus"),
            40 => Some("The Infernal Duke"),
            50 => Some("The Frost Wyrm"),
            60 => Some("The Jungle Titan"),
            70 => Some("The Crystal Archon"),
            80 => Some("The Void Emperor"),
            90 => Some("The Divine Judge"),
            100 => Some("The Chaos Primordial"),
            _ => None,
        }
    }

    /// Get the mini-boss name for a level
    pub fn get_mini_boss_name(level: u32) -> Option<&'static str> {
        match level {
            5 => Some("Skeletal Champion"),
            15 => Some("Giant Rat King"),
            25 => Some("Iron Golem"),
            35 => Some("Flame Demon"),
            45 => Some("Ice Wraith"),
            55 => Some("Beast Lord"),
            65 => Some("Arcane Guardian"),
            75 => Some("Void Stalker"),
            85 => Some("Fallen Angel"),
            95 => Some("Chaos Knight"),
            _ => None,
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for floor progression
pub fn get_floor_info(level: u32) -> (DungeonTheme, FloorType, &'static str) {
    let theme = DungeonTheme::from_level(level);
    let floor_type = FloorType::from_level(level, false);
    let description = theme.description();
    (theme, floor_type, description)
}

/// Check if a secret floor is available at a given level
pub fn secret_floor_available(level: u32) -> bool {
    level % 10 == 0 && level <= MAX_DUNGEON_LEVEL
}

/// Get the secret floor index for a given level
pub fn get_secret_floor_index(level: u32) -> Option<u32> {
    if secret_floor_available(level) {
        Some(level / 10)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_dungeon_level() {
        assert_eq!(MAX_DUNGEON_LEVEL, 100);
    }

    #[test]
    fn test_boss_levels() {
        assert!(BOSS_LEVELS.contains(&10));
        assert!(BOSS_LEVELS.contains(&50));
        assert!(BOSS_LEVELS.contains(&100));
        assert!(!BOSS_LEVELS.contains(&5));
    }

    #[test]
    fn test_mini_boss_levels() {
        assert!(MINI_BOSS_LEVELS.contains(&5));
        assert!(MINI_BOSS_LEVELS.contains(&55));
        assert!(!MINI_BOSS_LEVELS.contains(&10));
    }

    #[test]
    fn test_theme_from_level() {
        assert_eq!(DungeonTheme::from_level(1), DungeonTheme::Catacombs);
        assert_eq!(DungeonTheme::from_level(10), DungeonTheme::Catacombs);
        assert_eq!(DungeonTheme::from_level(11), DungeonTheme::Sewers);
        assert_eq!(DungeonTheme::from_level(31), DungeonTheme::VolcanicDepths);
        assert_eq!(DungeonTheme::from_level(91), DungeonTheme::Abyss);
        assert_eq!(DungeonTheme::from_level(100), DungeonTheme::Abyss);
    }

    #[test]
    fn test_floor_type_from_level() {
        assert_eq!(FloorType::from_level(1, false), FloorType::Normal);
        assert_eq!(FloorType::from_level(5, false), FloorType::MiniBoss);
        assert_eq!(FloorType::from_level(10, false), FloorType::Boss);
        assert_eq!(FloorType::from_level(11, false), FloorType::Transition);
        assert_eq!(FloorType::from_level(50, true), FloorType::Secret);
    }

    #[test]
    fn test_floor_modifiers() {
        let normal_mods = FloorModifiers::for_level(1, FloorType::Normal);
        let boss_mods = FloorModifiers::for_level(10, FloorType::Boss);

        assert!(boss_mods.enemy_difficulty > normal_mods.enemy_difficulty);
        assert!(boss_mods.loot_quality > normal_mods.loot_quality);
        assert!(boss_mods.exp_multiplier > normal_mods.exp_multiplier);
    }

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
        assert_eq!(map.theme, DungeonTheme::Catacombs);
        assert_eq!(map.floor_type, FloorType::Normal);
    }

    #[test]
    fn test_boss_floor_generation() {
        let mut map = Map::new();
        let mut rng = rand::thread_rng();
        map.generate(&mut rng, 10);

        assert_eq!(map.floor_type, FloorType::Boss);
        assert!(map.modifiers.enemy_difficulty > 1.0);
    }

    #[test]
    fn test_secret_floor_generation() {
        let mut map = Map::new();
        let mut rng = rand::thread_rng();
        map.generate_secret(&mut rng, 1);

        assert_eq!(map.theme, DungeonTheme::SecretRealm);
        assert_eq!(map.floor_type, FloorType::Secret);
    }

    #[test]
    fn test_get_boss_name() {
        assert_eq!(Map::get_boss_name(10), Some("The Crypt Lord"));
        assert_eq!(Map::get_boss_name(100), Some("The Chaos Primordial"));
        assert_eq!(Map::get_boss_name(5), None);
    }

    #[test]
    fn test_tile_walkable() {
        assert!(Tile::Floor.walkable());
        assert!(!Tile::Wall.walkable());
        assert!(Tile::StairsDown.walkable());
        assert!(!Tile::Lava.walkable());
    }

    #[test]
    fn test_secret_floor_available() {
        assert!(secret_floor_available(10));
        assert!(secret_floor_available(100));
        assert!(!secret_floor_available(5));
        assert!(!secret_floor_available(15));
    }
}

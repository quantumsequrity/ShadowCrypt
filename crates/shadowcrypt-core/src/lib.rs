//! ShadowCrypt Core Library
//!
//! This library contains the core game logic for the ShadowCrypt roguelike game.
//! It provides all the necessary types, systems, and utilities for building
//! both CLI and GUI versions of the game.

pub mod species;
pub mod classes;
pub mod magic;
pub mod combat;
pub mod items;
pub mod world;
pub mod save;
pub mod ai;
pub mod ui;
pub mod game;

// Re-export commonly used types at the crate root for convenience
pub use classes::CharacterClass;
pub use combat::{StatusEffect, Enemy, EnemyKind};
pub use items::{Item, ItemKind, Rarity, EquipSlot};
pub use world::{Map, Tile, DungeonTheme, Room};
pub use game::{GameState, Player};
pub use magic::Skill;

/// Game constants
pub mod constants {
    pub const MAP_WIDTH: usize = 100;
    pub const MAP_HEIGHT: usize = 45;
    pub const VIEW_RADIUS: i32 = 10;
    pub const MAX_ROOMS: usize = 20;
    pub const MIN_ROOM_SIZE: usize = 5;
    pub const MAX_ROOM_SIZE: usize = 15;
    pub const MAX_DUNGEON_LEVEL: u32 = 30;
    pub const BOSS_LEVELS: [u32; 6] = [5, 10, 15, 20, 25, 30];
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::classes::CharacterClass;
    pub use crate::combat::{StatusEffect, Enemy, EnemyKind};
    pub use crate::items::{Item, ItemKind, Rarity, EquipSlot};
    pub use crate::world::{Map, Tile, DungeonTheme, Room};
    pub use crate::game::{GameState, Player};
    pub use crate::magic::Skill;
    pub use crate::constants::*;
}

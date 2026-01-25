//! ShadowCrypt Core Library
//!
//! This library contains all the core game logic for the ShadowCrypt roguelike,
//! designed to be shared between CLI and GUI frontends.
//!
//! # Modules
//!
//! - `classes` - Character classes and their properties
//! - `magic` - Skills and spells
//! - `combat` - Status effects and combat calculations
//! - `items` - Items, equipment, and inventory
//! - `entities` - Player and enemy entities
//! - `world` - Map, tiles, and dungeon generation
//! - `ai` - AI decision making for auto-play
//! - `game` - Main game state and logic
//! - `save` - Save/load functionality
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use shadowcrypt_core::prelude::*;
//!
//! // Create a new game with a Warrior character
//! let mut game = GameState::new(CharacterClass::Warrior);
//!
//! // Move the player
//! game.move_player(1, 0);
//!
//! // Use a skill
//! game.use_skill();
//!
//! // Check game state
//! if game.victory {
//!     println!("You won!");
//! }
//! ```

pub mod classes;
pub mod magic;
pub mod combat;
pub mod items;
pub mod entities;
pub mod world;
pub mod ai;
pub mod game;
pub mod save;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::classes::CharacterClass;
    pub use crate::magic::Skill;
    pub use crate::combat::StatusEffect;
    pub use crate::items::{Item, ItemKind, Rarity, EquipSlot};
    pub use crate::entities::{Player, Enemy, EnemyKind};
    pub use crate::world::{Map, Tile, DungeonTheme, Room};
    pub use crate::world::{MAP_WIDTH, MAP_HEIGHT, VIEW_RADIUS, MAX_DUNGEON_LEVEL, BOSS_LEVELS};
    pub use crate::ai::{AIAction, AIDecider};
    pub use crate::game::{GameState, GameMessage};
    pub use crate::save::{save_game, load_game, save_exists, delete_save};
}

// Re-export commonly used types at crate root
pub use game::GameState;
pub use classes::CharacterClass;
pub use magic::Skill;
pub use combat::StatusEffect;
pub use items::{Item, ItemKind, Rarity, EquipSlot};
pub use entities::{Player, Enemy, EnemyKind};
pub use world::{Map, Tile, DungeonTheme, Room};
pub use ai::{AIAction, AIDecider};

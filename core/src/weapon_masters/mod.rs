//! Weapon mastery system
//!
//! Allows players to gain proficiency with different weapon types.

use serde::{Deserialize, Serialize};

/// Types of weapons that can be mastered
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Dagger,
    Spear,
    Bow,
    Staff,
    Fist,
}

/// Mastery level for a weapon type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MasteryLevel {
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
    Grandmaster,
}

impl Default for MasteryLevel {
    fn default() -> Self {
        Self::Novice
    }
}

/// Player's weapon mastery progress
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WeaponMastery {
    pub experience: u32,
    pub level: MasteryLevel,
}

impl WeaponMastery {
    pub fn new() -> Self {
        Self::default()
    }
}

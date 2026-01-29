//! Treasure system
//!
//! Special treasures, artifacts, and collectibles.

use serde::{Deserialize, Serialize};

/// Rarity of treasure items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreasureRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl Default for TreasureRarity {
    fn default() -> Self {
        Self::Common
    }
}

/// Types of treasures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreasureType {
    Gold,
    Gem,
    Artifact,
    Relic,
    Scroll,
    Map,
}

/// A treasure item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treasure {
    pub kind: TreasureType,
    pub rarity: TreasureRarity,
    pub name: String,
    pub value: u32,
    pub discovered: bool,
}

impl Treasure {
    pub fn new(kind: TreasureType, rarity: TreasureRarity, name: &str) -> Self {
        let value = match rarity {
            TreasureRarity::Common => 10,
            TreasureRarity::Uncommon => 50,
            TreasureRarity::Rare => 200,
            TreasureRarity::Epic => 1000,
            TreasureRarity::Legendary => 5000,
            TreasureRarity::Mythic => 25000,
        };
        Self {
            kind,
            rarity,
            name: name.to_string(),
            value,
            discovered: false,
        }
    }
}

/// Player's treasure collection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasureCollection {
    pub items: Vec<Treasure>,
    pub total_value: u32,
}

impl TreasureCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, treasure: Treasure) {
        self.total_value += treasure.value;
        self.items.push(treasure);
    }
}

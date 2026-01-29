//! Mount system
//!
//! Rideable creatures for travel and combat.

use serde::{Deserialize, Serialize};

/// Types of mounts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MountType {
    Horse,
    Wolf,
    Bear,
    Drake,
    Griffin,
    Phoenix,
    NightMare,
    Dragon,
}

/// Mount rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MountRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Default for MountRarity {
    fn default() -> Self {
        Self::Common
    }
}

/// A mount creature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mount {
    pub kind: MountType,
    pub rarity: MountRarity,
    pub name: String,
    pub speed_bonus: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub level: u32,
    pub experience: u32,
}

impl Mount {
    pub fn new(kind: MountType, rarity: MountRarity, name: &str) -> Self {
        let (speed_bonus, stamina) = match kind {
            MountType::Horse => (3, 100),
            MountType::Wolf => (4, 80),
            MountType::Bear => (2, 150),
            MountType::Drake => (5, 120),
            MountType::Griffin => (6, 100),
            MountType::Phoenix => (7, 80),
            MountType::NightMare => (5, 110),
            MountType::Dragon => (8, 150),
        };
        let rarity_multiplier = match rarity {
            MountRarity::Common => 1.0,
            MountRarity::Uncommon => 1.2,
            MountRarity::Rare => 1.5,
            MountRarity::Epic => 2.0,
            MountRarity::Legendary => 3.0,
        };
        Self {
            kind,
            rarity,
            name: name.to_string(),
            speed_bonus: (speed_bonus as f32 * rarity_multiplier) as i32,
            stamina: (stamina as f32 * rarity_multiplier) as i32,
            max_stamina: (stamina as f32 * rarity_multiplier) as i32,
            level: 1,
            experience: 0,
        }
    }
}

/// Player's mount stable
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MountStable {
    pub mounts: Vec<Mount>,
    pub active_mount: Option<usize>,
}

impl MountStable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mount(&mut self, mount: Mount) {
        self.mounts.push(mount);
    }

    pub fn get_active(&self) -> Option<&Mount> {
        self.active_mount.and_then(|i| self.mounts.get(i))
    }
}

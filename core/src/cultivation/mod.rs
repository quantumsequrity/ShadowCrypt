//! Cultivation system for spiritual advancement
//!
//! A cultivation system inspired by xianxia novels for character growth.

use serde::{Deserialize, Serialize};

/// Cultivation realm representing spiritual advancement level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CultivationRealm {
    Mortal,
    QiCondensation,
    FoundationEstablishment,
    CoreFormation,
    NascentSoul,
    SpiritSevering,
    DaoSeeking,
    Immortal,
}

impl Default for CultivationRealm {
    fn default() -> Self {
        Self::Mortal
    }
}

/// Player's cultivation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultivationState {
    pub realm: CultivationRealm,
    pub qi: i32,
    pub max_qi: i32,
    pub enlightenment: u32,
}

impl Default for CultivationState {
    fn default() -> Self {
        Self {
            realm: CultivationRealm::Mortal,
            qi: 0,
            max_qi: 100,
            enlightenment: 0,
        }
    }
}

impl CultivationState {
    pub fn new() -> Self {
        Self::default()
    }
}

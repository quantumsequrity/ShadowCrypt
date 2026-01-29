//! Combat formations system
//!
//! Tactical formations for party-based combat.

use serde::{Deserialize, Serialize};

/// Types of combat formations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FormationType {
    Standard,
    Defensive,
    Aggressive,
    Flanking,
    Pincer,
    Phalanx,
    Skirmish,
}

impl Default for FormationType {
    fn default() -> Self {
        Self::Standard
    }
}

/// Bonuses provided by a formation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormationBonus {
    pub attack_modifier: i32,
    pub defense_modifier: i32,
    pub speed_modifier: i32,
    pub coordination_bonus: i32,
}

impl FormationType {
    pub fn get_bonus(&self) -> FormationBonus {
        match self {
            Self::Standard => FormationBonus::default(),
            Self::Defensive => FormationBonus {
                attack_modifier: -2,
                defense_modifier: 5,
                speed_modifier: -1,
                coordination_bonus: 0,
            },
            Self::Aggressive => FormationBonus {
                attack_modifier: 5,
                defense_modifier: -3,
                speed_modifier: 1,
                coordination_bonus: 0,
            },
            Self::Flanking => FormationBonus {
                attack_modifier: 3,
                defense_modifier: -1,
                speed_modifier: 2,
                coordination_bonus: 2,
            },
            Self::Pincer => FormationBonus {
                attack_modifier: 4,
                defense_modifier: -2,
                speed_modifier: 0,
                coordination_bonus: 3,
            },
            Self::Phalanx => FormationBonus {
                attack_modifier: 0,
                defense_modifier: 8,
                speed_modifier: -3,
                coordination_bonus: 4,
            },
            Self::Skirmish => FormationBonus {
                attack_modifier: 2,
                defense_modifier: 0,
                speed_modifier: 4,
                coordination_bonus: 1,
            },
        }
    }
}

/// Party's formation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationState {
    pub current: FormationType,
    pub unlocked: Vec<FormationType>,
}

impl Default for FormationState {
    fn default() -> Self {
        Self {
            current: FormationType::Standard,
            unlocked: vec![FormationType::Standard],
        }
    }
}

impl FormationState {
    pub fn new() -> Self {
        Self::default()
    }
}

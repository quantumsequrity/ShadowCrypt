//! Boss system for ShadowCrypt
//!
//! This module handles boss encounters, phases, and special mechanics.

use crate::entities::EnemyKind;

/// Boss encounter phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossPhase {
    Phase1,
    Phase2,
    Phase3,
    Enraged,
}

/// Boss encounter data
#[derive(Debug, Clone)]
pub struct BossEncounter {
    pub kind: EnemyKind,
    pub phase: BossPhase,
    pub health_thresholds: Vec<f32>,
}

impl BossEncounter {
    pub fn new(kind: EnemyKind) -> Self {
        Self {
            kind,
            phase: BossPhase::Phase1,
            health_thresholds: vec![0.75, 0.50, 0.25],
        }
    }
}

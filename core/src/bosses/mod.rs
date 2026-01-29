//! Boss system
//!
//! Special boss enemies with unique mechanics and rewards.

use serde::{Deserialize, Serialize};

/// Boss difficulty tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BossTier {
    Minor,
    Major,
    Elite,
    Legendary,
    Mythic,
    WorldBoss,
}

impl Default for BossTier {
    fn default() -> Self {
        Self::Minor
    }
}

/// Boss types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BossType {
    Dragon,
    Demon,
    Undead,
    Elemental,
    Giant,
    Aberration,
    Construct,
    Divine,
}

/// A boss enemy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boss {
    pub id: String,
    pub name: String,
    pub boss_type: BossType,
    pub tier: BossTier,
    pub hp: i32,
    pub max_hp: i32,
    pub damage: i32,
    pub defense: i32,
    pub level: u32,
    pub phases: Vec<BossPhase>,
    pub current_phase: usize,
    pub enraged: bool,
    pub defeated: bool,
}

impl Boss {
    pub fn new(id: &str, name: &str, boss_type: BossType, tier: BossTier, level: u32) -> Self {
        let base_hp = match tier {
            BossTier::Minor => 500,
            BossTier::Major => 1500,
            BossTier::Elite => 5000,
            BossTier::Legendary => 15000,
            BossTier::Mythic => 50000,
            BossTier::WorldBoss => 200000,
        };
        let hp = base_hp + (level as i32 * 100);
        Self {
            id: id.to_string(),
            name: name.to_string(),
            boss_type,
            tier,
            hp,
            max_hp: hp,
            damage: 20 + level as i32 * 5,
            defense: 10 + level as i32 * 2,
            level,
            phases: Vec::new(),
            current_phase: 0,
            enraged: false,
            defeated: false,
        }
    }

    pub fn hp_percentage(&self) -> f32 {
        self.hp as f32 / self.max_hp as f32 * 100.0
    }

    pub fn check_phase_transition(&mut self) -> bool {
        if self.current_phase < self.phases.len() {
            let phase = &self.phases[self.current_phase];
            if self.hp_percentage() <= phase.hp_threshold {
                self.current_phase += 1;
                return true;
            }
        }
        false
    }
}

/// A phase in a boss fight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossPhase {
    pub name: String,
    pub hp_threshold: f32,
    pub damage_multiplier: f32,
    pub speed_multiplier: f32,
    pub special_ability: Option<String>,
}

/// Boss encounter state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BossEncounterState {
    pub active_boss: Option<Boss>,
    pub bosses_defeated: Vec<String>,
    pub total_boss_kills: u32,
}

impl BossEncounterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_defeat(&mut self, boss_id: &str) {
        self.bosses_defeated.push(boss_id.to_string());
        self.total_boss_kills += 1;
    }
}

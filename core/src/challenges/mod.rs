//! Challenge and Modifier System for ShadowCrypt
//!
//! This module provides various challenge modes and gameplay modifiers that alter
//! the core game rules to create unique and varied gameplay experiences.
//!
//! # Challenge Modes
//!
//! - **Hardcore**: Permadeath with increased difficulty
//! - **Speedrun**: Race against time with turn limits
//! - **Pacifist**: Complete the game without direct combat
//! - **Cursed**: Random afflictions with greater rewards
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use shadowcrypt_core::challenges::{ChallengeMode, ChallengeConfig, GameModifiers};
//!
//! // Create a hardcore speedrun challenge
//! let config = ChallengeConfig::new()
//!     .with_mode(ChallengeMode::Hardcore)
//!     .with_mode(ChallengeMode::Speedrun)
//!     .with_turn_limit(5000);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The primary challenge modes available in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChallengeMode {
    /// Permadeath mode - no saves, death is permanent, increased enemy stats
    Hardcore,
    /// Race against time - limited turns, speed bonuses, time pressure
    Speedrun,
    /// Non-violent playthrough - cannot directly attack enemies
    Pacifist,
    /// Random curses and afflictions, but better loot drops
    Cursed,
}

impl ChallengeMode {
    /// Returns the display name of the challenge mode
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hardcore => "Hardcore",
            Self::Speedrun => "Speedrun",
            Self::Pacifist => "Pacifist",
            Self::Cursed => "Cursed",
        }
    }

    /// Returns a description of the challenge mode
    pub fn description(&self) -> &'static str {
        match self {
            Self::Hardcore => "Permadeath enabled. No saving. Enemies deal +25% damage. Death is permanent.",
            Self::Speedrun => "Complete the game within the turn limit. Bonus score for speed. No pausing.",
            Self::Pacifist => "Cannot directly attack enemies. Use traps, scrolls, and the environment.",
            Self::Cursed => "Random curses afflict you. Enemies are stronger. Loot quality improved.",
        }
    }

    /// Returns a short summary for UI display
    pub fn short_description(&self) -> &'static str {
        match self {
            Self::Hardcore => "Permadeath, +25% enemy damage",
            Self::Speedrun => "Turn limit, speed bonus",
            Self::Pacifist => "No direct attacks",
            Self::Cursed => "Random curses, better loot",
        }
    }

    /// Returns the difficulty multiplier for this mode
    pub fn difficulty_multiplier(&self) -> f32 {
        match self {
            Self::Hardcore => 1.5,
            Self::Speedrun => 1.2,
            Self::Pacifist => 2.0,
            Self::Cursed => 1.8,
        }
    }

    /// Returns the score multiplier for completing the game with this mode
    pub fn score_multiplier(&self) -> f32 {
        match self {
            Self::Hardcore => 2.0,
            Self::Speedrun => 1.5,
            Self::Pacifist => 3.0,
            Self::Cursed => 1.75,
        }
    }

    /// Returns all available challenge modes
    pub fn all() -> Vec<Self> {
        vec![Self::Hardcore, Self::Speedrun, Self::Pacifist, Self::Cursed]
    }

    /// Returns the icon/glyph for this mode (for terminal display)
    pub fn glyph(&self) -> char {
        match self {
            Self::Hardcore => '!',
            Self::Speedrun => '%',
            Self::Pacifist => '&',
            Self::Cursed => '*',
        }
    }

    /// Returns a color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Hardcore => 3,  // Red
            Self::Speedrun => 11, // Yellow
            Self::Pacifist => 9,  // Cyan
            Self::Cursed => 13,   // Magenta
        }
    }
}

/// Types of curses that can afflict the player in Cursed mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurseType {
    /// Health slowly drains over time
    Withering,
    /// Mana regeneration is halved
    ManaDrain,
    /// Random items in inventory become unusable
    ItemDecay,
    /// Vision range is reduced
    Blindness,
    /// Movement is slowed (enemies get extra actions)
    Lethargy,
    /// All damage taken is increased
    Vulnerability,
    /// Gold pickups are reduced
    Poverty,
    /// XP gains are reduced
    Stagnation,
    /// Hunger depletes faster
    Ravenous,
    /// Skills cost more mana
    Exhaustion,
    /// Enemies are alerted to your presence from farther away
    Marked,
    /// Traps are harder to detect
    Oblivious,
}

impl CurseType {
    /// Returns the display name of the curse
    pub fn name(&self) -> &'static str {
        match self {
            Self::Withering => "Withering",
            Self::ManaDrain => "Mana Drain",
            Self::ItemDecay => "Item Decay",
            Self::Blindness => "Blindness",
            Self::Lethargy => "Lethargy",
            Self::Vulnerability => "Vulnerability",
            Self::Poverty => "Poverty",
            Self::Stagnation => "Stagnation",
            Self::Ravenous => "Ravenous",
            Self::Exhaustion => "Exhaustion",
            Self::Marked => "Marked",
            Self::Oblivious => "Oblivious",
        }
    }

    /// Returns a description of the curse's effect
    pub fn description(&self) -> &'static str {
        match self {
            Self::Withering => "Lose 1 HP every 10 turns",
            Self::ManaDrain => "Mana regeneration halved",
            Self::ItemDecay => "Random items may become unusable",
            Self::Blindness => "Vision range reduced by 2",
            Self::Lethargy => "Enemies occasionally get extra actions",
            Self::Vulnerability => "Take 20% more damage",
            Self::Poverty => "Gold pickups reduced by 50%",
            Self::Stagnation => "XP gains reduced by 25%",
            Self::Ravenous => "Hunger depletes twice as fast",
            Self::Exhaustion => "Skills cost 50% more mana",
            Self::Marked => "Enemies detect you from farther away",
            Self::Oblivious => "Traps are invisible until triggered",
        }
    }

    /// Returns all curse types
    pub fn all() -> Vec<Self> {
        vec![
            Self::Withering,
            Self::ManaDrain,
            Self::ItemDecay,
            Self::Blindness,
            Self::Lethargy,
            Self::Vulnerability,
            Self::Poverty,
            Self::Stagnation,
            Self::Ravenous,
            Self::Exhaustion,
            Self::Marked,
            Self::Oblivious,
        ]
    }

    /// Returns the severity level (1-3) affecting how impactful the curse is
    pub fn severity(&self) -> u8 {
        match self {
            Self::Poverty | Self::Stagnation | Self::Oblivious => 1,
            Self::ManaDrain | Self::Blindness | Self::Ravenous | Self::Exhaustion | Self::Marked => 2,
            Self::Withering | Self::ItemDecay | Self::Lethargy | Self::Vulnerability => 3,
        }
    }
}

/// Active curse instance with duration tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveCurse {
    pub curse_type: CurseType,
    pub remaining_turns: Option<u32>, // None = permanent until removed
    pub intensity: f32,               // Multiplier for curse effect (1.0 = normal)
}

impl ActiveCurse {
    /// Create a new active curse
    pub fn new(curse_type: CurseType, duration: Option<u32>) -> Self {
        Self {
            curse_type,
            remaining_turns: duration,
            intensity: 1.0,
        }
    }

    /// Create a permanent curse
    pub fn permanent(curse_type: CurseType) -> Self {
        Self {
            curse_type,
            remaining_turns: None,
            intensity: 1.0,
        }
    }

    /// Create an intensified curse
    pub fn intensified(curse_type: CurseType, duration: Option<u32>, intensity: f32) -> Self {
        Self {
            curse_type,
            remaining_turns: duration,
            intensity,
        }
    }

    /// Tick the curse and return true if it expired
    pub fn tick(&mut self) -> bool {
        if let Some(ref mut turns) = self.remaining_turns {
            *turns = turns.saturating_sub(1);
            *turns == 0
        } else {
            false
        }
    }

    /// Check if the curse is still active
    pub fn is_active(&self) -> bool {
        self.remaining_turns.map_or(true, |t| t > 0)
    }
}

/// Speedrun milestones for tracking progress
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeedrunMilestone {
    pub name: String,
    pub turn_reached: u32,
    pub level_reached: u32,
}

/// Speedrun tracking data
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SpeedrunData {
    /// Turn limit for the run
    pub turn_limit: u32,
    /// Turns remaining
    pub turns_remaining: u32,
    /// Best split times for each dungeon level
    pub level_splits: Vec<u32>,
    /// Milestones achieved
    pub milestones: Vec<SpeedrunMilestone>,
    /// Whether any time bonuses are active
    pub time_bonus_active: bool,
    /// Bonus turns earned from speed bonuses
    pub bonus_turns_earned: u32,
}

impl SpeedrunData {
    /// Create new speedrun data with a turn limit
    pub fn new(turn_limit: u32) -> Self {
        Self {
            turn_limit,
            turns_remaining: turn_limit,
            level_splits: Vec::new(),
            milestones: Vec::new(),
            time_bonus_active: false,
            bonus_turns_earned: 0,
        }
    }

    /// Consume a turn and return true if time has run out
    pub fn tick(&mut self) -> bool {
        self.turns_remaining = self.turns_remaining.saturating_sub(1);
        self.turns_remaining == 0
    }

    /// Record reaching a new dungeon level
    pub fn record_level(&mut self, level: u32, current_turn: u32) {
        self.level_splits.push(current_turn);
        self.milestones.push(SpeedrunMilestone {
            name: format!("Reached Level {}", level),
            turn_reached: current_turn,
            level_reached: level,
        });
    }

    /// Add bonus turns (from speed pickups, etc.)
    pub fn add_bonus_turns(&mut self, turns: u32) {
        self.turns_remaining += turns;
        self.bonus_turns_earned += turns;
    }

    /// Calculate time bonus score multiplier based on remaining turns
    pub fn time_bonus_multiplier(&self) -> f32 {
        let remaining_ratio = self.turns_remaining as f32 / self.turn_limit as f32;
        1.0 + (remaining_ratio * 0.5) // Up to 50% bonus for fast completion
    }

    /// Get percentage of time remaining
    pub fn time_remaining_percent(&self) -> f32 {
        (self.turns_remaining as f32 / self.turn_limit as f32) * 100.0
    }
}

/// Pacifist mode tracking
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PacifistData {
    /// Number of enemies killed indirectly (traps, environment, etc.)
    pub indirect_kills: u32,
    /// Number of enemies evaded
    pub enemies_evaded: u32,
    /// Number of scrolls used
    pub scrolls_used: u32,
    /// Whether the run is still valid (no direct attacks)
    pub is_valid: bool,
    /// Turn of last violation (if any)
    pub violation_turn: Option<u32>,
}

impl PacifistData {
    /// Create new pacifist tracking data
    pub fn new() -> Self {
        Self {
            indirect_kills: 0,
            enemies_evaded: 0,
            scrolls_used: 0,
            is_valid: true,
            violation_turn: None,
        }
    }

    /// Record a direct attack violation
    pub fn record_violation(&mut self, turn: u32) {
        if self.is_valid {
            self.is_valid = false;
            self.violation_turn = Some(turn);
        }
    }

    /// Record an indirect kill
    pub fn record_indirect_kill(&mut self) {
        self.indirect_kills += 1;
    }

    /// Record evading an enemy
    pub fn record_evasion(&mut self) {
        self.enemies_evaded += 1;
    }

    /// Record using a scroll
    pub fn record_scroll_use(&mut self) {
        self.scrolls_used += 1;
    }

    /// Calculate pacifist score bonus
    pub fn score_bonus(&self) -> f32 {
        if !self.is_valid {
            return 0.0;
        }
        // Bonus based on evasions and indirect methods
        let evasion_bonus = (self.enemies_evaded as f32 * 0.01).min(0.5);
        let indirect_bonus = (self.indirect_kills as f32 * 0.02).min(0.5);
        1.0 + evasion_bonus + indirect_bonus
    }
}

/// Hardcore mode tracking
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HardcoreData {
    /// Whether this is the character's first life (for achievements)
    pub first_life: bool,
    /// Number of close calls (HP dropped below 10%)
    pub close_calls: u32,
    /// Highest damage taken in a single hit
    pub max_damage_taken: i32,
    /// Number of bosses defeated
    pub bosses_defeated: u32,
    /// Deepest level reached
    pub deepest_level: u32,
}

impl HardcoreData {
    /// Create new hardcore tracking data
    pub fn new() -> Self {
        Self {
            first_life: true,
            close_calls: 0,
            max_damage_taken: 0,
            bosses_defeated: 0,
            deepest_level: 1,
        }
    }

    /// Record taking damage
    pub fn record_damage(&mut self, damage: i32, hp_after: i32, max_hp: i32) {
        if damage > self.max_damage_taken {
            self.max_damage_taken = damage;
        }
        if hp_after > 0 && hp_after < max_hp / 10 {
            self.close_calls += 1;
        }
    }

    /// Record defeating a boss
    pub fn record_boss_defeat(&mut self) {
        self.bosses_defeated += 1;
    }

    /// Record reaching a new level
    pub fn record_level(&mut self, level: u32) {
        if level > self.deepest_level {
            self.deepest_level = level;
        }
    }

    /// Calculate hardcore score bonus
    pub fn score_bonus(&self) -> f32 {
        let boss_bonus = self.bosses_defeated as f32 * 0.1;
        let level_bonus = self.deepest_level as f32 * 0.02;
        let survival_bonus = if self.first_life { 0.5 } else { 0.0 };
        1.0 + boss_bonus + level_bonus + survival_bonus
    }
}

/// Game modifiers that affect gameplay mechanics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameModifiers {
    // Damage modifiers
    /// Multiplier for damage dealt by player
    pub player_damage_mult: f32,
    /// Multiplier for damage dealt to player
    pub enemy_damage_mult: f32,

    // Loot modifiers
    /// Multiplier for gold drops
    pub gold_mult: f32,
    /// Multiplier for XP gains
    pub xp_mult: f32,
    /// Bonus to item rarity rolls
    pub rarity_bonus: i32,
    /// Multiplier for item drop rate
    pub drop_rate_mult: f32,

    // Survival modifiers
    /// Multiplier for hunger depletion rate
    pub hunger_rate_mult: f32,
    /// Multiplier for mana costs
    pub mana_cost_mult: f32,
    /// Multiplier for health regeneration
    pub regen_mult: f32,

    // Vision modifiers
    /// Bonus/penalty to view radius
    pub vision_modifier: i32,
    /// Whether traps are visible
    pub traps_visible: bool,

    // Combat modifiers
    /// Whether direct attacks are allowed
    pub attacks_allowed: bool,
    /// Multiplier for skill damage
    pub skill_damage_mult: f32,
    /// Whether enemies get extra actions occasionally
    pub enemy_extra_actions: bool,
    /// Chance for enemies to get an extra action (0.0-1.0)
    pub extra_action_chance: f32,

    // Enemy modifiers
    /// Multiplier for enemy HP
    pub enemy_hp_mult: f32,
    /// Bonus to enemy detection range
    pub enemy_detection_bonus: i32,

    // Misc modifiers
    /// Whether saving is allowed
    pub saving_allowed: bool,
    /// Whether the game tracks a turn limit
    pub has_turn_limit: bool,
    /// Score multiplier for final calculation
    pub score_mult: f32,
}

impl Default for GameModifiers {
    fn default() -> Self {
        Self {
            player_damage_mult: 1.0,
            enemy_damage_mult: 1.0,
            gold_mult: 1.0,
            xp_mult: 1.0,
            rarity_bonus: 0,
            drop_rate_mult: 1.0,
            hunger_rate_mult: 1.0,
            mana_cost_mult: 1.0,
            regen_mult: 1.0,
            vision_modifier: 0,
            traps_visible: true,
            attacks_allowed: true,
            skill_damage_mult: 1.0,
            enemy_extra_actions: false,
            extra_action_chance: 0.0,
            enemy_hp_mult: 1.0,
            enemy_detection_bonus: 0,
            saving_allowed: true,
            has_turn_limit: false,
            score_mult: 1.0,
        }
    }
}

impl GameModifiers {
    /// Create modifiers for a specific challenge mode
    pub fn for_mode(mode: ChallengeMode) -> Self {
        let mut modifiers = Self::default();

        match mode {
            ChallengeMode::Hardcore => {
                modifiers.enemy_damage_mult = 1.25;
                modifiers.enemy_hp_mult = 1.15;
                modifiers.saving_allowed = false;
                modifiers.score_mult = 2.0;
            }
            ChallengeMode::Speedrun => {
                modifiers.has_turn_limit = true;
                modifiers.hunger_rate_mult = 0.5; // Reduced hunger for speed
                modifiers.score_mult = 1.5;
            }
            ChallengeMode::Pacifist => {
                modifiers.attacks_allowed = false;
                modifiers.skill_damage_mult = 0.0; // Damage skills don't work directly
                modifiers.xp_mult = 1.5; // Bonus XP to compensate
                modifiers.drop_rate_mult = 1.5; // More items to use
                modifiers.score_mult = 3.0;
            }
            ChallengeMode::Cursed => {
                modifiers.enemy_hp_mult = 1.2;
                modifiers.enemy_damage_mult = 1.15;
                modifiers.rarity_bonus = 15; // Better loot
                modifiers.drop_rate_mult = 1.3;
                modifiers.gold_mult = 1.25;
                modifiers.score_mult = 1.75;
            }
        }

        modifiers
    }

    /// Combine modifiers from multiple challenge modes
    pub fn combine(modes: &[ChallengeMode]) -> Self {
        let mut combined = Self::default();

        for mode in modes {
            let mode_mods = Self::for_mode(*mode);

            // Multiplicative combinations
            combined.player_damage_mult *= mode_mods.player_damage_mult;
            combined.enemy_damage_mult *= mode_mods.enemy_damage_mult;
            combined.gold_mult *= mode_mods.gold_mult;
            combined.xp_mult *= mode_mods.xp_mult;
            combined.drop_rate_mult *= mode_mods.drop_rate_mult;
            combined.hunger_rate_mult *= mode_mods.hunger_rate_mult;
            combined.mana_cost_mult *= mode_mods.mana_cost_mult;
            combined.regen_mult *= mode_mods.regen_mult;
            combined.skill_damage_mult *= mode_mods.skill_damage_mult;
            combined.enemy_hp_mult *= mode_mods.enemy_hp_mult;
            combined.score_mult *= mode_mods.score_mult;

            // Additive combinations
            combined.rarity_bonus += mode_mods.rarity_bonus;
            combined.vision_modifier += mode_mods.vision_modifier;
            combined.enemy_detection_bonus += mode_mods.enemy_detection_bonus;

            // Boolean combinations (most restrictive wins)
            combined.traps_visible = combined.traps_visible && mode_mods.traps_visible;
            combined.attacks_allowed = combined.attacks_allowed && mode_mods.attacks_allowed;
            combined.saving_allowed = combined.saving_allowed && mode_mods.saving_allowed;

            // Boolean combinations (any enables)
            combined.enemy_extra_actions =
                combined.enemy_extra_actions || mode_mods.enemy_extra_actions;
            combined.has_turn_limit = combined.has_turn_limit || mode_mods.has_turn_limit;

            // Maximum for chance-based
            combined.extra_action_chance =
                combined.extra_action_chance.max(mode_mods.extra_action_chance);
        }

        combined
    }

    /// Apply curse effects to modifiers
    pub fn apply_curse(&mut self, curse: &ActiveCurse) {
        let intensity = curse.intensity;
        match curse.curse_type {
            CurseType::Withering => {} // Handled separately in tick
            CurseType::ManaDrain => {
                self.mana_cost_mult *= 1.0 + (0.5 * intensity);
            }
            CurseType::ItemDecay => {} // Handled separately
            CurseType::Blindness => {
                self.vision_modifier -= (2.0 * intensity) as i32;
            }
            CurseType::Lethargy => {
                self.enemy_extra_actions = true;
                self.extra_action_chance = 0.2 * intensity;
            }
            CurseType::Vulnerability => {
                self.enemy_damage_mult *= 1.0 + (0.2 * intensity);
            }
            CurseType::Poverty => {
                self.gold_mult *= 1.0 - (0.5 * intensity).min(0.9);
            }
            CurseType::Stagnation => {
                self.xp_mult *= 1.0 - (0.25 * intensity).min(0.9);
            }
            CurseType::Ravenous => {
                self.hunger_rate_mult *= 1.0 + (1.0 * intensity);
            }
            CurseType::Exhaustion => {
                self.mana_cost_mult *= 1.0 + (0.5 * intensity);
            }
            CurseType::Marked => {
                self.enemy_detection_bonus += (3.0 * intensity) as i32;
            }
            CurseType::Oblivious => {
                self.traps_visible = false;
            }
        }
    }
}

/// Configuration for a challenge run
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeConfig {
    /// Active challenge modes
    pub modes: HashSet<ChallengeMode>,
    /// Combined game modifiers
    pub modifiers: GameModifiers,
    /// Turn limit for speedrun mode
    pub turn_limit: u32,
    /// Seed for reproducible runs (optional)
    pub seed: Option<u64>,
    /// Custom name for this challenge configuration
    pub name: Option<String>,
}

impl Default for ChallengeConfig {
    fn default() -> Self {
        Self {
            modes: HashSet::new(),
            modifiers: GameModifiers::default(),
            turn_limit: 10000, // Default turn limit for speedrun
            seed: None,
            name: None,
        }
    }
}

impl ChallengeConfig {
    /// Create a new empty challenge configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a challenge mode
    pub fn with_mode(mut self, mode: ChallengeMode) -> Self {
        self.modes.insert(mode);
        self.recalculate_modifiers();
        self
    }

    /// Remove a challenge mode
    pub fn without_mode(mut self, mode: ChallengeMode) -> Self {
        self.modes.remove(&mode);
        self.recalculate_modifiers();
        self
    }

    /// Set the turn limit for speedrun mode
    pub fn with_turn_limit(mut self, limit: u32) -> Self {
        self.turn_limit = limit;
        self
    }

    /// Set a seed for reproducible runs
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set a custom name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Recalculate modifiers based on active modes
    fn recalculate_modifiers(&mut self) {
        let modes: Vec<ChallengeMode> = self.modes.iter().copied().collect();
        self.modifiers = GameModifiers::combine(&modes);
    }

    /// Check if a specific mode is active
    pub fn has_mode(&self, mode: ChallengeMode) -> bool {
        self.modes.contains(&mode)
    }

    /// Get all active modes
    pub fn active_modes(&self) -> Vec<ChallengeMode> {
        self.modes.iter().copied().collect()
    }

    /// Calculate total difficulty multiplier
    pub fn difficulty_multiplier(&self) -> f32 {
        self.modes
            .iter()
            .map(|m| m.difficulty_multiplier())
            .product()
    }

    /// Calculate total score multiplier
    pub fn score_multiplier(&self) -> f32 {
        self.modifiers.score_mult
    }

    /// Get display name for this configuration
    pub fn display_name(&self) -> String {
        if let Some(ref name) = self.name {
            return name.clone();
        }

        if self.modes.is_empty() {
            return "Standard".to_string();
        }

        let mode_names: Vec<&str> = self.modes.iter().map(|m| m.name()).collect();
        mode_names.join(" + ")
    }

    /// Generate a summary of active modifiers
    pub fn modifier_summary(&self) -> Vec<String> {
        let mut summary = Vec::new();
        let m = &self.modifiers;

        if m.enemy_damage_mult != 1.0 {
            summary.push(format!(
                "Enemy damage: {:.0}%",
                m.enemy_damage_mult * 100.0
            ));
        }
        if m.enemy_hp_mult != 1.0 {
            summary.push(format!("Enemy HP: {:.0}%", m.enemy_hp_mult * 100.0));
        }
        if m.gold_mult != 1.0 {
            summary.push(format!("Gold drops: {:.0}%", m.gold_mult * 100.0));
        }
        if m.xp_mult != 1.0 {
            summary.push(format!("XP gains: {:.0}%", m.xp_mult * 100.0));
        }
        if m.rarity_bonus != 0 {
            summary.push(format!("Item rarity: {:+}", m.rarity_bonus));
        }
        if !m.saving_allowed {
            summary.push("Saving disabled".to_string());
        }
        if !m.attacks_allowed {
            summary.push("Direct attacks disabled".to_string());
        }
        if m.has_turn_limit {
            summary.push(format!("Turn limit: {}", self.turn_limit));
        }

        summary
    }
}

/// Complete challenge state for an active run
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeState {
    /// The challenge configuration
    pub config: ChallengeConfig,
    /// Hardcore mode data (if active)
    pub hardcore_data: Option<HardcoreData>,
    /// Speedrun mode data (if active)
    pub speedrun_data: Option<SpeedrunData>,
    /// Pacifist mode data (if active)
    pub pacifist_data: Option<PacifistData>,
    /// Active curses (if in cursed mode)
    pub active_curses: Vec<ActiveCurse>,
    /// Current turn count
    pub turn_count: u32,
    /// Whether the challenge is still valid
    pub is_valid: bool,
    /// Reason for invalidation (if any)
    pub invalidation_reason: Option<String>,
}

impl ChallengeState {
    /// Create a new challenge state from a configuration
    pub fn new(config: ChallengeConfig) -> Self {
        let hardcore_data = if config.has_mode(ChallengeMode::Hardcore) {
            Some(HardcoreData::new())
        } else {
            None
        };

        let speedrun_data = if config.has_mode(ChallengeMode::Speedrun) {
            Some(SpeedrunData::new(config.turn_limit))
        } else {
            None
        };

        let pacifist_data = if config.has_mode(ChallengeMode::Pacifist) {
            Some(PacifistData::new())
        } else {
            None
        };

        Self {
            config,
            hardcore_data,
            speedrun_data,
            pacifist_data,
            active_curses: Vec::new(),
            turn_count: 0,
            is_valid: true,
            invalidation_reason: None,
        }
    }

    /// Process a turn and return any events that occurred
    pub fn tick(&mut self) -> Vec<ChallengeEvent> {
        let mut events = Vec::new();
        self.turn_count += 1;

        // Tick speedrun timer
        if let Some(ref mut data) = self.speedrun_data {
            if data.tick() {
                events.push(ChallengeEvent::TimeExpired);
                self.invalidate("Time limit exceeded");
            } else if data.turns_remaining <= 100 && data.turns_remaining % 25 == 0 {
                events.push(ChallengeEvent::TimeWarning(data.turns_remaining));
            }
        }

        // Tick curses
        let mut expired_curses = Vec::new();
        for (i, curse) in self.active_curses.iter_mut().enumerate() {
            if curse.tick() {
                expired_curses.push(i);
                events.push(ChallengeEvent::CurseExpired(curse.curse_type));
            }

            // Apply withering damage
            if curse.curse_type == CurseType::Withering && self.turn_count % 10 == 0 {
                events.push(ChallengeEvent::CurseDamage(
                    CurseType::Withering,
                    (1.0 * curse.intensity) as i32,
                ));
            }
        }

        // Remove expired curses (in reverse order to maintain indices)
        for i in expired_curses.into_iter().rev() {
            self.active_curses.remove(i);
        }

        events
    }

    /// Add a curse
    pub fn add_curse(&mut self, curse: ActiveCurse) {
        // Check if we already have this curse type
        if let Some(existing) = self
            .active_curses
            .iter_mut()
            .find(|c| c.curse_type == curse.curse_type)
        {
            // Intensify existing curse instead of adding duplicate
            existing.intensity = (existing.intensity + 0.25).min(2.0);
            if let (Some(ref mut remaining), Some(new_duration)) =
                (&mut existing.remaining_turns, curse.remaining_turns)
            {
                *remaining = (*remaining).max(new_duration);
            }
        } else {
            self.active_curses.push(curse);
        }

        // Recalculate modifiers with curse effects
        self.recalculate_modifiers();
    }

    /// Remove a curse by type
    pub fn remove_curse(&mut self, curse_type: CurseType) -> bool {
        let initial_len = self.active_curses.len();
        self.active_curses.retain(|c| c.curse_type != curse_type);
        let removed = self.active_curses.len() < initial_len;
        if removed {
            self.recalculate_modifiers();
        }
        removed
    }

    /// Recalculate modifiers including curse effects
    fn recalculate_modifiers(&mut self) {
        // Start with base modifiers from modes
        let modes: Vec<ChallengeMode> = self.config.modes.iter().copied().collect();
        self.config.modifiers = GameModifiers::combine(&modes);

        // Apply curse effects
        for curse in &self.active_curses {
            self.config.modifiers.apply_curse(curse);
        }
    }

    /// Record a direct attack (for pacifist mode validation)
    pub fn record_attack(&mut self) {
        if let Some(ref mut data) = self.pacifist_data {
            data.record_violation(self.turn_count);
            self.invalidate("Direct attack performed");
        }
    }

    /// Record an indirect kill
    pub fn record_indirect_kill(&mut self) {
        if let Some(ref mut data) = self.pacifist_data {
            data.record_indirect_kill();
        }
    }

    /// Record damage taken (for hardcore tracking)
    pub fn record_damage_taken(&mut self, damage: i32, hp_after: i32, max_hp: i32) {
        if let Some(ref mut data) = self.hardcore_data {
            data.record_damage(damage, hp_after, max_hp);
        }
    }

    /// Record reaching a new dungeon level
    pub fn record_level_reached(&mut self, level: u32) {
        if let Some(ref mut data) = self.hardcore_data {
            data.record_level(level);
        }
        if let Some(ref mut data) = self.speedrun_data {
            data.record_level(level, self.turn_count);
        }
    }

    /// Record defeating a boss
    pub fn record_boss_defeated(&mut self) {
        if let Some(ref mut data) = self.hardcore_data {
            data.record_boss_defeat();
        }
    }

    /// Invalidate the challenge run
    pub fn invalidate(&mut self, reason: &str) {
        if self.is_valid {
            self.is_valid = false;
            self.invalidation_reason = Some(reason.to_string());
        }
    }

    /// Check if the run allows saving
    pub fn can_save(&self) -> bool {
        self.config.modifiers.saving_allowed
    }

    /// Check if direct attacks are allowed
    pub fn can_attack(&self) -> bool {
        self.config.modifiers.attacks_allowed
    }

    /// Get current modifiers
    pub fn modifiers(&self) -> &GameModifiers {
        &self.config.modifiers
    }

    /// Calculate final score with all multipliers
    pub fn calculate_final_score(&self, base_score: u32) -> u32 {
        let mut multiplier = self.config.score_multiplier();

        // Add mode-specific bonuses
        if let Some(ref data) = self.hardcore_data {
            multiplier *= data.score_bonus();
        }
        if let Some(ref data) = self.speedrun_data {
            multiplier *= data.time_bonus_multiplier();
        }
        if let Some(ref data) = self.pacifist_data {
            multiplier *= data.score_bonus();
        }

        // Penalty for invalidation
        if !self.is_valid {
            multiplier *= 0.5;
        }

        (base_score as f32 * multiplier) as u32
    }

    /// Generate a summary of the challenge run
    pub fn generate_summary(&self) -> ChallengeSummary {
        ChallengeSummary {
            config_name: self.config.display_name(),
            modes: self.config.active_modes(),
            turn_count: self.turn_count,
            is_valid: self.is_valid,
            invalidation_reason: self.invalidation_reason.clone(),
            curses_suffered: self.active_curses.len() as u32,
            hardcore_data: self.hardcore_data.clone(),
            speedrun_data: self.speedrun_data.clone(),
            pacifist_data: self.pacifist_data.clone(),
            final_score_multiplier: self.config.score_multiplier(),
        }
    }
}

/// Events that can occur during challenge processing
#[derive(Clone, Debug)]
pub enum ChallengeEvent {
    /// Speedrun time has expired
    TimeExpired,
    /// Warning that time is running low
    TimeWarning(u32),
    /// A curse has expired
    CurseExpired(CurseType),
    /// Damage dealt by a curse
    CurseDamage(CurseType, i32),
    /// A new curse has been applied
    CurseApplied(CurseType),
    /// Challenge has been invalidated
    Invalidated(String),
}

/// Summary of a challenge run for display
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeSummary {
    pub config_name: String,
    pub modes: Vec<ChallengeMode>,
    pub turn_count: u32,
    pub is_valid: bool,
    pub invalidation_reason: Option<String>,
    pub curses_suffered: u32,
    pub hardcore_data: Option<HardcoreData>,
    pub speedrun_data: Option<SpeedrunData>,
    pub pacifist_data: Option<PacifistData>,
    pub final_score_multiplier: f32,
}

/// Predefined challenge configurations
pub mod presets {
    use super::*;

    /// Standard hardcore mode
    pub fn hardcore() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Hardcore)
            .with_name("Hardcore")
    }

    /// Standard speedrun mode with default turn limit
    pub fn speedrun() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Speedrun)
            .with_turn_limit(8000)
            .with_name("Speedrun")
    }

    /// Quick speedrun with tighter turn limit
    pub fn speedrun_fast() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Speedrun)
            .with_turn_limit(5000)
            .with_name("Speed Demon")
    }

    /// Standard pacifist mode
    pub fn pacifist() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Pacifist)
            .with_name("Pacifist")
    }

    /// Standard cursed mode
    pub fn cursed() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Cursed)
            .with_name("Cursed")
    }

    /// Ultimate challenge: all modes combined
    pub fn ultimate() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Hardcore)
            .with_mode(ChallengeMode::Speedrun)
            .with_mode(ChallengeMode::Cursed)
            .with_turn_limit(6000)
            .with_name("Ultimate Challenge")
    }

    /// Ironman: Hardcore + Cursed
    pub fn ironman() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Hardcore)
            .with_mode(ChallengeMode::Cursed)
            .with_name("Ironman")
    }

    /// Enlightened: Pacifist + Speedrun
    pub fn enlightened() -> ChallengeConfig {
        ChallengeConfig::new()
            .with_mode(ChallengeMode::Pacifist)
            .with_mode(ChallengeMode::Speedrun)
            .with_turn_limit(12000) // Extra time for pacifist approach
            .with_name("Enlightened")
    }

    /// Get all preset configurations
    pub fn all() -> Vec<ChallengeConfig> {
        vec![
            hardcore(),
            speedrun(),
            speedrun_fast(),
            pacifist(),
            cursed(),
            ultimate(),
            ironman(),
            enlightened(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_mode_properties() {
        assert_eq!(ChallengeMode::Hardcore.name(), "Hardcore");
        assert!(ChallengeMode::Hardcore.difficulty_multiplier() > 1.0);
        assert!(ChallengeMode::Hardcore.score_multiplier() > 1.0);
    }

    #[test]
    fn test_game_modifiers_hardcore() {
        let mods = GameModifiers::for_mode(ChallengeMode::Hardcore);
        assert_eq!(mods.enemy_damage_mult, 1.25);
        assert!(!mods.saving_allowed);
    }

    #[test]
    fn test_game_modifiers_pacifist() {
        let mods = GameModifiers::for_mode(ChallengeMode::Pacifist);
        assert!(!mods.attacks_allowed);
        assert_eq!(mods.xp_mult, 1.5);
    }

    #[test]
    fn test_modifiers_combine() {
        let combined = GameModifiers::combine(&[ChallengeMode::Hardcore, ChallengeMode::Cursed]);
        // Both modes increase enemy damage
        assert!(combined.enemy_damage_mult > 1.25);
        assert!(!combined.saving_allowed);
    }

    #[test]
    fn test_challenge_config_builder() {
        let config = ChallengeConfig::new()
            .with_mode(ChallengeMode::Hardcore)
            .with_mode(ChallengeMode::Speedrun)
            .with_turn_limit(5000)
            .with_name("Test Challenge");

        assert!(config.has_mode(ChallengeMode::Hardcore));
        assert!(config.has_mode(ChallengeMode::Speedrun));
        assert_eq!(config.turn_limit, 5000);
        assert_eq!(config.display_name(), "Test Challenge");
    }

    #[test]
    fn test_challenge_state_speedrun() {
        let config = ChallengeConfig::new()
            .with_mode(ChallengeMode::Speedrun)
            .with_turn_limit(100);

        let mut state = ChallengeState::new(config);

        // Tick through turns
        for _ in 0..99 {
            let events = state.tick();
            assert!(!events.iter().any(|e| matches!(e, ChallengeEvent::TimeExpired)));
        }

        // Final tick should expire
        let events = state.tick();
        assert!(events.iter().any(|e| matches!(e, ChallengeEvent::TimeExpired)));
    }

    #[test]
    fn test_curse_application() {
        let config = ChallengeConfig::new().with_mode(ChallengeMode::Cursed);

        let mut state = ChallengeState::new(config);
        let curse = ActiveCurse::new(CurseType::Vulnerability, Some(10));

        state.add_curse(curse);
        assert_eq!(state.active_curses.len(), 1);
        assert!(state.config.modifiers.enemy_damage_mult > 1.15);
    }

    #[test]
    fn test_pacifist_validation() {
        let config = ChallengeConfig::new().with_mode(ChallengeMode::Pacifist);

        let mut state = ChallengeState::new(config);
        assert!(state.is_valid);
        assert!(!state.can_attack());

        state.record_attack();
        assert!(!state.is_valid);
        assert!(state.pacifist_data.as_ref().unwrap().violation_turn.is_some());
    }

    #[test]
    fn test_presets() {
        let all = presets::all();
        assert!(!all.is_empty());

        let ultimate = presets::ultimate();
        assert!(ultimate.has_mode(ChallengeMode::Hardcore));
        assert!(ultimate.has_mode(ChallengeMode::Speedrun));
        assert!(ultimate.has_mode(ChallengeMode::Cursed));
    }

    #[test]
    fn test_score_calculation() {
        let config = ChallengeConfig::new()
            .with_mode(ChallengeMode::Hardcore)
            .with_mode(ChallengeMode::Cursed);

        let state = ChallengeState::new(config);
        let base_score = 1000;
        let final_score = state.calculate_final_score(base_score);

        // Should be significantly higher due to multipliers
        assert!(final_score > base_score);
    }
}

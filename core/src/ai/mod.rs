//! AI system: Comprehensive auto-play mode for autonomous game completion
//!
//! This module provides a sophisticated AI system capable of completing the game autonomously.
//! It includes:
//! - Configurable play styles (Aggressive, Defensive, Balanced, Speedrun, Completionist)
//! - Intelligent decision making for combat, exploration, and resource management
//! - Class-specific AI profiles (Warrior, Mage, Rogue, Paladin)
//! - Learning and adaptation based on combat effectiveness
//! - Performance statistics and tracking

use std::collections::{HashMap, HashSet, VecDeque};
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::items::{ItemKind, Rarity, EquipSlot};
use crate::world::{Map, Tile, MAP_WIDTH, MAP_HEIGHT, BOSS_LEVELS};
use crate::entities::{Player, Enemy, EnemyKind};
use crate::classes::CharacterClass;
use crate::magic::Skill;
use crate::combat::StatusEffect;

// ============================================================================
// CORE AI ACTIONS
// ============================================================================

/// Actions the AI can take
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AIAction {
    Move(i32, i32),
    UseSkill,
    UseItem(usize),
    Descend,
    Ascend,
    Wait,
    Rest,
    Flee(i32, i32),
    EquipItem(usize),
    DropItem(usize),
    CycleSkill,
}

impl AIAction {
    /// Returns true if this is a movement action
    pub fn is_movement(&self) -> bool {
        matches!(self, AIAction::Move(_, _) | AIAction::Flee(_, _))
    }

    /// Returns true if this is a combat action
    pub fn is_combat(&self) -> bool {
        matches!(self, AIAction::Move(_, _) | AIAction::UseSkill | AIAction::UseItem(_))
    }
}

// ============================================================================
// CONFIGURATION ENUMS
// ============================================================================

/// Play style determines overall AI behavior priorities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoPlayStyle {
    /// Maximum aggression - seeks combat, prioritizes damage
    Aggressive,
    /// Maximum caution - avoids unnecessary risks, prioritizes survival
    Defensive,
    /// Balanced approach - adapts to situation
    Balanced,
    /// Minimizes turns - rushes to stairs, avoids unnecessary exploration
    Speedrun,
    /// Full exploration - clears every room, collects everything
    Completionist,
}

impl Default for AutoPlayStyle {
    fn default() -> Self {
        Self::Balanced
    }
}

impl AutoPlayStyle {
    /// Returns all available play styles
    pub fn all() -> &'static [Self] {
        &[Self::Aggressive, Self::Defensive, Self::Balanced, Self::Speedrun, Self::Completionist]
    }

    /// Returns display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Aggressive => "Aggressive",
            Self::Defensive => "Defensive",
            Self::Balanced => "Balanced",
            Self::Speedrun => "Speedrun",
            Self::Completionist => "Completionist",
        }
    }

    /// Returns description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Aggressive => "Seeks combat and maximizes damage output",
            Self::Defensive => "Prioritizes survival and resource conservation",
            Self::Balanced => "Adapts strategy based on current situation",
            Self::Speedrun => "Rushes to complete levels as fast as possible",
            Self::Completionist => "Explores everything and collects all items",
        }
    }
}

/// Target priority for combat decisions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPriority {
    /// Target enemies with lowest HP first
    Weakest,
    /// Target enemies with highest HP/threat first
    Strongest,
    /// Target nearest enemy first
    Closest,
    /// Target highest XP value enemies first
    MostValuable,
    /// Target enemies that can inflict status effects
    MostDangerous,
}

impl Default for TargetPriority {
    fn default() -> Self {
        Self::Closest
    }
}

/// Loot filter settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LootFilter {
    /// Minimum rarity to pick up equipment
    pub min_equipment_rarity: Rarity,
    /// Always pick up consumables (potions, scrolls)
    pub pickup_consumables: bool,
    /// Always pick up food
    pub pickup_food: bool,
    /// Always pick up gold
    pub pickup_gold: bool,
    /// Pick up items that are upgrades only
    pub upgrades_only: bool,
    /// Item names to always pick up (using string comparison since ItemKind doesn't implement Hash)
    #[serde(default)]
    pub always_pickup_names: Vec<String>,
    /// Item names to never pick up
    #[serde(default)]
    pub never_pickup_names: Vec<String>,
}

impl Default for LootFilter {
    fn default() -> Self {
        Self {
            min_equipment_rarity: Rarity::Common,
            pickup_consumables: true,
            pickup_food: true,
            pickup_gold: true,
            upgrades_only: false,
            always_pickup_names: Vec::new(),
            never_pickup_names: Vec::new(),
        }
    }
}

impl LootFilter {
    /// Create a speedrun-focused loot filter
    pub fn speedrun() -> Self {
        Self {
            min_equipment_rarity: Rarity::Rare,
            pickup_consumables: true,
            pickup_food: true,
            pickup_gold: false,
            upgrades_only: true,
            always_pickup_names: Vec::new(),
            never_pickup_names: Vec::new(),
        }
    }

    /// Create a completionist loot filter
    pub fn completionist() -> Self {
        Self {
            min_equipment_rarity: Rarity::Common,
            pickup_consumables: true,
            pickup_food: true,
            pickup_gold: true,
            upgrades_only: false,
            always_pickup_names: Vec::new(),
            never_pickup_names: Vec::new(),
        }
    }

    /// Add an item to always pickup list
    pub fn always_pickup(&mut self, kind: ItemKind) {
        self.always_pickup_names.push(kind.name().to_string());
    }

    /// Add an item to never pickup list
    pub fn never_pickup(&mut self, kind: ItemKind) {
        self.never_pickup_names.push(kind.name().to_string());
    }

    /// Check if an item should be picked up
    pub fn should_pickup(&self, kind: ItemKind, rarity: Rarity, _player: &Player) -> bool {
        let item_name = kind.name();

        // Check never pickup list
        if self.never_pickup_names.iter().any(|n| n == item_name) {
            return false;
        }

        // Check always pickup list
        if self.always_pickup_names.iter().any(|n| n == item_name) {
            return true;
        }

        // Gold
        if matches!(kind, ItemKind::Gold) {
            return self.pickup_gold;
        }

        // Food
        if kind.is_food() {
            return self.pickup_food;
        }

        // Consumables
        if kind.is_consumable() {
            return self.pickup_consumables;
        }

        // Equipment - check rarity
        if kind.equip_slot().is_some() {
            return rarity >= self.min_equipment_rarity;
        }

        true
    }
}

/// Exploration mode determines how the AI explores the dungeon
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplorationMode {
    /// Clear every room and corridor
    FullClear,
    /// Go directly to stairs once found
    RushStairs,
    /// Prioritize finding treasure/chests
    TreasureHunting,
    /// Balance exploration with progression
    Balanced,
}

impl Default for ExplorationMode {
    fn default() -> Self {
        Self::Balanced
    }
}

// ============================================================================
// AUTO-PLAY CONFIGURATION
// ============================================================================

/// Comprehensive configuration for the auto-play AI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutoPlayConfig {
    /// Overall play style
    pub play_style: AutoPlayStyle,
    /// Target priority in combat
    pub target_priority: TargetPriority,
    /// Loot filter settings
    pub loot_filter: LootFilter,
    /// Exploration behavior
    pub exploration_mode: ExplorationMode,
    /// HP percentage to trigger resting (0-100)
    pub rest_hp_threshold: u8,
    /// MP percentage to trigger resting (0-100)
    pub rest_mp_threshold: u8,
    /// HP percentage to trigger fleeing from combat (0-100)
    pub flee_threshold: u8,
    /// Whether to use skills in combat
    pub use_skills: bool,
    /// Whether to use consumable items
    pub use_items: bool,
    /// Whether to automatically manage inventory
    pub manage_inventory: bool,
    /// Whether to auto-equip better items
    pub auto_equip: bool,
    /// Maximum turns to wait before progressing
    pub max_idle_turns: u32,
    /// Hunger threshold to eat food (0-100)
    pub eat_threshold: u8,
    /// Whether to prioritize quests
    pub prioritize_quests: bool,
    /// Whether to use shrines
    pub use_shrines: bool,
}

impl Default for AutoPlayConfig {
    fn default() -> Self {
        Self {
            play_style: AutoPlayStyle::Balanced,
            target_priority: TargetPriority::Closest,
            loot_filter: LootFilter::default(),
            exploration_mode: ExplorationMode::Balanced,
            rest_hp_threshold: 50,
            rest_mp_threshold: 30,
            flee_threshold: 20,
            use_skills: true,
            use_items: true,
            manage_inventory: true,
            auto_equip: true,
            max_idle_turns: 100,
            eat_threshold: 30,
            prioritize_quests: true,
            use_shrines: true,
        }
    }
}

impl AutoPlayConfig {
    /// Create an aggressive configuration
    pub fn aggressive() -> Self {
        Self {
            play_style: AutoPlayStyle::Aggressive,
            target_priority: TargetPriority::Strongest,
            loot_filter: LootFilter::default(),
            exploration_mode: ExplorationMode::FullClear,
            rest_hp_threshold: 30,
            rest_mp_threshold: 20,
            flee_threshold: 10,
            use_skills: true,
            use_items: true,
            manage_inventory: true,
            auto_equip: true,
            max_idle_turns: 50,
            eat_threshold: 20,
            prioritize_quests: false,
            use_shrines: true,
        }
    }

    /// Create a defensive configuration
    pub fn defensive() -> Self {
        Self {
            play_style: AutoPlayStyle::Defensive,
            target_priority: TargetPriority::Weakest,
            loot_filter: LootFilter::default(),
            exploration_mode: ExplorationMode::Balanced,
            rest_hp_threshold: 70,
            rest_mp_threshold: 50,
            flee_threshold: 40,
            use_skills: true,
            use_items: true,
            manage_inventory: true,
            auto_equip: true,
            max_idle_turns: 150,
            eat_threshold: 50,
            prioritize_quests: true,
            use_shrines: true,
        }
    }

    /// Create a speedrun configuration
    pub fn speedrun() -> Self {
        Self {
            play_style: AutoPlayStyle::Speedrun,
            target_priority: TargetPriority::Closest,
            loot_filter: LootFilter::speedrun(),
            exploration_mode: ExplorationMode::RushStairs,
            rest_hp_threshold: 40,
            rest_mp_threshold: 20,
            flee_threshold: 25,
            use_skills: true,
            use_items: true,
            manage_inventory: false,
            auto_equip: true,
            max_idle_turns: 20,
            eat_threshold: 15,
            prioritize_quests: false,
            use_shrines: false,
        }
    }

    /// Create a completionist configuration
    pub fn completionist() -> Self {
        Self {
            play_style: AutoPlayStyle::Completionist,
            target_priority: TargetPriority::MostValuable,
            loot_filter: LootFilter::completionist(),
            exploration_mode: ExplorationMode::FullClear,
            rest_hp_threshold: 60,
            rest_mp_threshold: 40,
            flee_threshold: 25,
            use_skills: true,
            use_items: true,
            manage_inventory: true,
            auto_equip: true,
            max_idle_turns: 200,
            eat_threshold: 40,
            prioritize_quests: true,
            use_shrines: true,
        }
    }
}

// ============================================================================
// THREAT ASSESSMENT
// ============================================================================

/// Threat level for danger assessment
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    None,
    Low,
    Moderate,
    High,
    Critical,
    Lethal,
}

/// Information about a threat
#[derive(Clone, Debug)]
pub struct ThreatInfo {
    /// Overall threat level
    pub level: ThreatLevel,
    /// Total enemy damage potential per turn
    pub damage_potential: i32,
    /// Number of enemies
    pub enemy_count: usize,
    /// Estimated turns to death if no action taken
    pub turns_to_death: i32,
    /// Whether boss is present
    pub boss_present: bool,
    /// Enemies with dangerous abilities
    pub dangerous_enemies: Vec<usize>,
    /// Recommended action
    pub recommendation: ThreatRecommendation,
}

/// Recommended action based on threat
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreatRecommendation {
    Engage,
    Caution,
    Retreat,
    Flee,
    Rest,
}

// ============================================================================
// COMBAT MEMORY - Using Vec since EnemyKind doesn't impl Hash
// ============================================================================

/// Memory of combat effectiveness against enemy types
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CombatMemory {
    /// Total damage dealt to this enemy type
    pub total_damage_dealt: i64,
    /// Total damage received from this enemy type
    pub total_damage_received: i64,
    /// Number of encounters
    pub encounters: u32,
    /// Number of kills
    pub kills: u32,
    /// Number of deaths caused by this enemy
    pub deaths: u32,
    /// Average turns to kill
    pub avg_turns_to_kill: f32,
    /// Most effective skill used (by name since Skill doesn't impl Hash)
    pub best_skill_name: Option<String>,
    /// Most effective skill damage
    pub best_skill_damage: i32,
}

/// Entry for combat memory storage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatMemoryEntry {
    pub enemy_name: String,
    pub memory: CombatMemory,
}

/// Collection of combat memories
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CombatMemoryStore {
    entries: Vec<CombatMemoryEntry>,
}

impl CombatMemoryStore {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn get(&self, enemy_kind: EnemyKind) -> Option<&CombatMemory> {
        let name = enemy_kind.name();
        self.entries.iter().find(|e| e.enemy_name == name).map(|e| &e.memory)
    }

    pub fn get_mut(&mut self, enemy_kind: EnemyKind) -> Option<&mut CombatMemory> {
        let name = enemy_kind.name();
        self.entries.iter_mut().find(|e| e.enemy_name == name).map(|e| &mut e.memory)
    }

    pub fn get_or_insert(&mut self, enemy_kind: EnemyKind) -> &mut CombatMemory {
        let name = enemy_kind.name().to_string();
        if !self.entries.iter().any(|e| e.enemy_name == name) {
            self.entries.push(CombatMemoryEntry {
                enemy_name: name.clone(),
                memory: CombatMemory::default(),
            });
        }
        self.entries.iter_mut().find(|e| e.enemy_name == name).map(|e| &mut e.memory).unwrap()
    }
}

// ============================================================================
// AI DECISION ENGINE
// ============================================================================

/// Current AI objective
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AIObjective {
    /// No specific objective
    None,
    /// Kill a specific enemy
    KillEnemy(usize, usize),
    /// Move to a position
    MoveTo(usize, usize),
    /// Find and descend stairs
    FindStairs,
    /// Explore unexplored areas
    Explore,
    /// Rest to recover HP/MP
    Rest,
    /// Flee from danger
    Flee,
    /// Find and use shrine
    UseShrine,
    /// Collect nearby items
    CollectLoot,
    /// Kill the boss
    DefeatBoss,
}

/// The core AI decision-making engine
#[derive(Clone, Debug)]
pub struct AIDecisionEngine {
    config: AutoPlayConfig,
    /// Learned enemy effectiveness data
    combat_memory: CombatMemoryStore,
    /// Path cache for navigation
    path_cache: Option<Vec<(usize, usize)>>,
    /// Current objective
    current_objective: AIObjective,
    /// Explored positions this level
    explored_positions: HashSet<(usize, usize)>,
    /// Turns spent on current objective
    objective_turns: u32,
}

impl Default for AIDecisionEngine {
    fn default() -> Self {
        Self::new(AutoPlayConfig::default())
    }
}

impl AIDecisionEngine {
    /// Create a new decision engine with configuration
    pub fn new(config: AutoPlayConfig) -> Self {
        Self {
            config,
            combat_memory: CombatMemoryStore::new(),
            path_cache: None,
            current_objective: AIObjective::None,
            explored_positions: HashSet::new(),
            objective_turns: 0,
        }
    }

    /// Update configuration
    pub fn set_config(&mut self, config: AutoPlayConfig) {
        self.config = config;
    }

    /// Reset for a new level
    pub fn reset_level(&mut self) {
        self.path_cache = None;
        self.explored_positions.clear();
        self.current_objective = AIObjective::None;
        self.objective_turns = 0;
    }

    // ========================================================================
    // THREAT EVALUATION
    // ========================================================================

    /// Evaluate the current threat level
    pub fn evaluate_threats(&self, player: &Player, enemies: &[Enemy], map: &Map) -> ThreatInfo {
        let visible_enemies: Vec<(usize, &Enemy)> = enemies.iter().enumerate()
            .filter(|(_, e)| e.is_alive() && map.visible[e.y][e.x])
            .collect();

        if visible_enemies.is_empty() {
            return ThreatInfo {
                level: ThreatLevel::None,
                damage_potential: 0,
                enemy_count: 0,
                turns_to_death: i32::MAX,
                boss_present: false,
                dangerous_enemies: Vec::new(),
                recommendation: ThreatRecommendation::Engage,
            };
        }

        let mut total_damage = 0;
        let mut dangerous = Vec::new();
        let mut boss_present = false;

        for (idx, enemy) in &visible_enemies {
            let potential_damage = (enemy.attack - player.total_defense()).max(1);
            total_damage += potential_damage;

            if enemy.kind.is_boss() {
                boss_present = true;
                dangerous.push(*idx);
            } else if enemy.kind.can_poison() || enemy.kind.can_freeze() || enemy.kind.can_burn() {
                dangerous.push(*idx);
            }
        }

        let turns_to_death = if total_damage > 0 {
            player.hp / total_damage
        } else {
            i32::MAX
        };

        let hp_percent = (player.hp * 100) / player.total_max_hp();

        let level = if boss_present && hp_percent < 50 {
            ThreatLevel::Lethal
        } else if turns_to_death <= 2 || hp_percent < 15 {
            ThreatLevel::Critical
        } else if turns_to_death <= 4 || hp_percent < 30 {
            ThreatLevel::High
        } else if visible_enemies.len() >= 3 || hp_percent < 50 {
            ThreatLevel::Moderate
        } else if !visible_enemies.is_empty() {
            ThreatLevel::Low
        } else {
            ThreatLevel::None
        };

        let recommendation = match level {
            ThreatLevel::Lethal => ThreatRecommendation::Flee,
            ThreatLevel::Critical => {
                if hp_percent < self.config.flee_threshold as i32 {
                    ThreatRecommendation::Flee
                } else {
                    ThreatRecommendation::Retreat
                }
            }
            ThreatLevel::High => ThreatRecommendation::Caution,
            ThreatLevel::Moderate | ThreatLevel::Low => ThreatRecommendation::Engage,
            ThreatLevel::None => {
                if hp_percent < self.config.rest_hp_threshold as i32 {
                    ThreatRecommendation::Rest
                } else {
                    ThreatRecommendation::Engage
                }
            }
        };

        ThreatInfo {
            level,
            damage_potential: total_damage,
            enemy_count: visible_enemies.len(),
            turns_to_death,
            boss_present,
            dangerous_enemies: dangerous,
            recommendation,
        }
    }

    // ========================================================================
    // COMBAT EVALUATION
    // ========================================================================

    /// Evaluate the best combat action
    pub fn evaluate_combat_action(
        &self,
        player: &Player,
        enemies: &[Enemy],
        map: &Map,
    ) -> Option<AIAction> {
        let visible_enemies: Vec<(usize, &Enemy)> = enemies.iter().enumerate()
            .filter(|(_, e)| e.is_alive() && map.visible[e.y][e.x])
            .collect();

        if visible_enemies.is_empty() {
            return None;
        }

        let px = player.x as i32;
        let py = player.y as i32;

        // Find target based on priority
        let target = self.select_target(player, &visible_enemies);

        if target.is_none() {
            return None;
        }

        let (target_idx, target_enemy) = target.unwrap();
        let tx = target_enemy.x as i32;
        let ty = target_enemy.y as i32;
        let dist = ((tx - px).abs().max((ty - py).abs())) as i32;

        // Adjacent enemy - attack or use melee skill
        if dist <= 1 {
            // Check if we should use a skill
            if self.config.use_skills && player.can_use_skill() {
                let skill = player.skills.get(player.active_skill);
                if let Some(s) = skill {
                    if self.should_use_skill_in_combat(player, *s, &visible_enemies) {
                        return Some(AIAction::UseSkill);
                    }
                }
            }

            // Basic attack (move into enemy)
            let dx = (tx - px).signum();
            let dy = (ty - py).signum();
            return Some(AIAction::Move(dx, dy));
        }

        // Enemy in skill range - use ranged skill if available
        if self.config.use_skills && player.can_use_skill() && dist <= 5 {
            let skill = player.skills.get(player.active_skill);
            if let Some(s) = skill {
                if self.is_ranged_skill(*s) {
                    return Some(AIAction::UseSkill);
                }
            }
        }

        // Move toward target
        let dx = (tx - px).signum();
        let dy = (ty - py).signum();
        let nx = (px + dx) as usize;
        let ny = (py + dy) as usize;

        if map.is_walkable(nx, ny) {
            return Some(AIAction::Move(dx, dy));
        }

        // Try alternative movement
        if dx != 0 && map.is_walkable((px + dx) as usize, py as usize) {
            return Some(AIAction::Move(dx, 0));
        }
        if dy != 0 && map.is_walkable(px as usize, (py + dy) as usize) {
            return Some(AIAction::Move(0, dy));
        }

        None
    }

    /// Select the best target based on priority
    fn select_target<'a>(
        &self,
        player: &Player,
        enemies: &[(usize, &'a Enemy)],
    ) -> Option<(usize, &'a Enemy)> {
        if enemies.is_empty() {
            return None;
        }

        let px = player.x as i32;
        let py = player.y as i32;

        let best = match self.config.target_priority {
            TargetPriority::Weakest => {
                enemies.iter().min_by_key(|(_, e)| e.hp)
            }
            TargetPriority::Strongest => {
                enemies.iter().max_by_key(|(_, e)| e.hp)
            }
            TargetPriority::Closest => {
                enemies.iter().min_by_key(|(_, e)| {
                    let dx = e.x as i32 - px;
                    let dy = e.y as i32 - py;
                    dx * dx + dy * dy
                })
            }
            TargetPriority::MostValuable => {
                enemies.iter().max_by_key(|(_, e)| e.xp_value)
            }
            TargetPriority::MostDangerous => {
                enemies.iter().max_by_key(|(_, e)| {
                    let mut danger = e.attack;
                    if e.kind.can_poison() { danger += 10; }
                    if e.kind.can_freeze() { danger += 15; }
                    if e.kind.can_burn() { danger += 10; }
                    if e.kind.is_boss() { danger += 100; }
                    danger
                })
            }
        };

        best.map(|(idx, e)| (*idx, *e))
    }

    /// Check if skill should be used in combat
    fn should_use_skill_in_combat(&self, player: &Player, skill: Skill, enemies: &[(usize, &Enemy)]) -> bool {
        let mana_percent = (player.mana * 100) / player.total_max_mana().max(1);

        // Don't use skill if low on mana (save for emergencies)
        if mana_percent < 20 && !matches!(skill, Skill::HolyLight | Skill::LifeDrain) {
            return false;
        }

        match skill {
            // AoE skills - use when multiple enemies
            Skill::Cleave | Skill::Whirlwind | Skill::Fireball | Skill::Lightning |
            Skill::MultiShot | Skill::Curse => {
                enemies.len() >= 2
            }
            // Defensive/healing - use when needed
            Skill::HolyLight => {
                let hp_percent = (player.hp * 100) / player.total_max_hp();
                hp_percent < 50 || enemies.iter().any(|(_, e)| e.kind.is_undead())
            }
            Skill::DivineShield => {
                let hp_percent = (player.hp * 100) / player.total_max_hp();
                hp_percent < 40 && !player.has_status(StatusEffect::Shield)
            }
            Skill::Vanish => {
                let hp_percent = (player.hp * 100) / player.total_max_hp();
                hp_percent < 30 && enemies.len() >= 2
            }
            Skill::LifeDrain => {
                let hp_percent = (player.hp * 100) / player.total_max_hp();
                hp_percent < 60
            }
            // Single target - always useful
            Skill::Backstab | Skill::Smite | Skill::PoisonBlade | Skill::PoisonArrow => true,
            // Buff skills
            Skill::Berserk => {
                enemies.iter().any(|(_, e)| e.kind.is_boss()) || enemies.len() >= 3
            }
            // Utility
            Skill::Teleport => false, // Don't use in combat unless fleeing
            Skill::EagleEye => false, // Exploration skill
            Skill::TrapSet | Skill::Consecrate => false, // Setup skills
            // Summon
            Skill::RaiseDead => player.minions.len() < 3,
            _ => true,
        }
    }

    /// Check if a skill is ranged
    fn is_ranged_skill(&self, skill: Skill) -> bool {
        matches!(skill,
            Skill::Fireball | Skill::IceSpear | Skill::Lightning |
            Skill::MultiShot | Skill::PoisonArrow | Skill::Smite |
            Skill::LifeDrain | Skill::Curse
        )
    }

    // ========================================================================
    // MOVEMENT EVALUATION
    // ========================================================================

    /// Evaluate movement direction for exploration or objectives
    pub fn evaluate_movement(
        &mut self,
        player: &Player,
        enemies: &[Enemy],
        map: &Map,
        dungeon_level: u32,
        boss_defeated: bool,
    ) -> Option<AIAction> {
        let px = player.x;
        let py = player.y;

        // Update explored positions
        self.explored_positions.insert((px, py));

        // Check current tile for special actions
        let current_tile = map.tiles[py][px];

        // Use shrines if configured
        if self.config.use_shrines && current_tile.is_shrine() {
            // Shrine interaction happens via movement in game
        }

        // Check for stairs
        if current_tile == Tile::StairsDown {
            if boss_defeated || !BOSS_LEVELS.contains(&dungeon_level) {
                // Check if we should descend
                match self.config.exploration_mode {
                    ExplorationMode::RushStairs => return Some(AIAction::Descend),
                    ExplorationMode::FullClear => {
                        if self.is_level_cleared(enemies, map) {
                            return Some(AIAction::Descend);
                        }
                    }
                    _ => {
                        // Balanced - descend if no visible enemies
                        let no_enemies = !enemies.iter().any(|e| e.is_alive() && map.visible[e.y][e.x]);
                        if no_enemies {
                            return Some(AIAction::Descend);
                        }
                    }
                }
            }
        }

        // Find movement target based on objective
        let target = match &self.current_objective {
            AIObjective::MoveTo(x, y) => Some((*x, *y)),
            AIObjective::FindStairs => self.find_stairs(map),
            AIObjective::Explore => self.find_unexplored_area(px, py, map),
            AIObjective::UseShrine => self.find_shrine(px, py, map),
            AIObjective::CollectLoot => None, // Items are auto-collected on movement
            _ => None,
        };

        if let Some((tx, ty)) = target {
            return self.move_toward(px, py, tx, ty, map, enemies);
        }

        // Default exploration
        self.find_unexplored_area(px, py, map)
            .and_then(|(tx, ty)| self.move_toward(px, py, tx, ty, map, enemies))
    }

    /// Move toward a target position
    fn move_toward(
        &self,
        px: usize,
        py: usize,
        tx: usize,
        ty: usize,
        map: &Map,
        enemies: &[Enemy],
    ) -> Option<AIAction> {
        if px == tx && py == ty {
            return None;
        }

        // Simple pathfinding - try direct movement first
        let dx = (tx as i32 - px as i32).signum();
        let dy = (ty as i32 - py as i32).signum();

        let directions = [
            (dx, dy),
            (dx, 0),
            (0, dy),
            (dy, dx), // Diagonal alternatives
            (-dy, dx),
        ];

        for (dx, dy) in directions {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = (px as i32 + dx) as usize;
            let ny = (py as i32 + dy) as usize;

            if nx < MAP_WIDTH && ny < MAP_HEIGHT && map.is_walkable(nx, ny) {
                // Check for enemies at position
                let enemy_at_pos = enemies.iter().any(|e| e.x == nx && e.y == ny && e.is_alive());
                if !enemy_at_pos {
                    return Some(AIAction::Move(dx, dy));
                }
            }
        }

        // Try pathfinding for blocked paths
        if let Some(path) = self.find_path(px, py, tx, ty, map, enemies) {
            if let Some(&(nx, ny)) = path.first() {
                let dx = nx as i32 - px as i32;
                let dy = ny as i32 - py as i32;
                return Some(AIAction::Move(dx, dy));
            }
        }

        None
    }

    /// A* pathfinding
    fn find_path(
        &self,
        start_x: usize,
        start_y: usize,
        goal_x: usize,
        goal_y: usize,
        map: &Map,
        enemies: &[Enemy],
    ) -> Option<Vec<(usize, usize)>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut open_set: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut g_score: HashMap<(usize, usize), i32> = HashMap::new();

        let heuristic = |x: usize, y: usize| -> i32 {
            ((x as i32 - goal_x as i32).abs() + (y as i32 - goal_y as i32).abs()) as i32
        };

        g_score.insert((start_x, start_y), 0);
        open_set.push(Reverse((heuristic(start_x, start_y), start_x, start_y)));

        let enemy_positions: HashSet<(usize, usize)> = enemies.iter()
            .filter(|e| e.is_alive())
            .map(|e| (e.x, e.y))
            .collect();

        while let Some(Reverse((_, x, y))) = open_set.pop() {
            if x == goal_x && y == goal_y {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current = (x, y);
                while let Some(&prev) = came_from.get(&current) {
                    path.push(current);
                    current = prev;
                }
                path.reverse();
                return Some(path);
            }

            let current_g = *g_score.get(&(x, y)).unwrap_or(&i32::MAX);

            for (dx, dy) in &[(-1i32, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;

                if nx >= MAP_WIDTH || ny >= MAP_HEIGHT {
                    continue;
                }

                if !map.is_walkable(nx, ny) && !(nx == goal_x && ny == goal_y) {
                    continue;
                }

                if enemy_positions.contains(&(nx, ny)) && !(nx == goal_x && ny == goal_y) {
                    continue;
                }

                let tentative_g = current_g + 1;
                let prev_g = *g_score.get(&(nx, ny)).unwrap_or(&i32::MAX);

                if tentative_g < prev_g {
                    came_from.insert((nx, ny), (x, y));
                    g_score.insert((nx, ny), tentative_g);
                    let f = tentative_g + heuristic(nx, ny);
                    open_set.push(Reverse((f, nx, ny)));
                }
            }
        }

        None
    }

    /// Find stairs on the map
    fn find_stairs(&self, map: &Map) -> Option<(usize, usize)> {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if map.explored[y][x] && map.tiles[y][x] == Tile::StairsDown {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Find unexplored area to explore
    fn find_unexplored_area(&self, px: usize, py: usize, map: &Map) -> Option<(usize, usize)> {
        let mut best: Option<(usize, usize, i32)> = None;

        // Look for visible but unexplored tiles
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                // Check for tiles at the edge of explored area
                if map.explored[y][x] && map.tiles[y][x].walkable() {
                    // Check if this tile borders unexplored area
                    let borders_unexplored = [(-1i32, 0), (1, 0), (0, -1), (0, 1)].iter()
                        .any(|(dx, dy)| {
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            nx < MAP_WIDTH && ny < MAP_HEIGHT && !map.explored[ny][nx]
                        });

                    if borders_unexplored && !self.explored_positions.contains(&(x, y)) {
                        let dist = (x as i32 - px as i32).abs() + (y as i32 - py as i32).abs();
                        if best.is_none() || dist < best.unwrap().2 {
                            best = Some((x, y, dist));
                        }
                    }
                }
            }
        }

        best.map(|(x, y, _)| (x, y))
    }

    /// Find a shrine
    fn find_shrine(&self, px: usize, py: usize, map: &Map) -> Option<(usize, usize)> {
        let mut best: Option<(usize, usize, i32)> = None;

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if map.explored[y][x] && map.tiles[y][x].is_shrine() {
                    let dist = (x as i32 - px as i32).abs() + (y as i32 - py as i32).abs();
                    if best.is_none() || dist < best.unwrap().2 {
                        best = Some((x, y, dist));
                    }
                }
            }
        }

        best.map(|(x, y, _)| (x, y))
    }

    /// Check if level is cleared
    fn is_level_cleared(&self, enemies: &[Enemy], map: &Map) -> bool {
        // Check if all enemies are dead
        let enemies_alive = enemies.iter().any(|e| e.is_alive());
        if enemies_alive {
            return false;
        }

        // Check if most of map is explored
        let total_walkable = map.tiles.iter().flatten()
            .filter(|t| t.walkable())
            .count();
        let explored_walkable = (0..MAP_HEIGHT).flat_map(|y| (0..MAP_WIDTH).map(move |x| (x, y)))
            .filter(|&(x, y)| map.explored[y][x] && map.tiles[y][x].walkable())
            .count();

        explored_walkable >= total_walkable * 80 / 100
    }

    // ========================================================================
    // ITEM EVALUATION
    // ========================================================================

    /// Evaluate whether to pick up an item
    pub fn evaluate_loot(&self, player: &Player, kind: ItemKind, rarity: Rarity) -> bool {
        self.config.loot_filter.should_pickup(kind, rarity, player)
    }

    /// Evaluate item usage
    pub fn evaluate_item_use(&self, player: &Player, enemies: &[Enemy], map: &Map) -> Option<AIAction> {
        if !self.config.use_items {
            return None;
        }

        let hp_percent = (player.hp * 100) / player.total_max_hp();
        let mp_percent = (player.mana * 100) / player.total_max_mana().max(1);
        let hunger_percent = (player.hunger * 100) / player.max_hunger;

        // Check for enemies
        let in_combat = enemies.iter().any(|e| e.is_alive() && map.visible[e.y][e.x]);

        // Critical HP - use health potion
        if hp_percent < 30 {
            for (i, item) in player.inventory.iter().enumerate() {
                if matches!(item.kind, ItemKind::HealthPotion | ItemKind::FullRestorePotion) {
                    return Some(AIAction::UseItem(i));
                }
            }
        }

        // Starving - eat food
        if hunger_percent < self.config.eat_threshold as i32 {
            for (i, item) in player.inventory.iter().enumerate() {
                if item.kind.is_food() {
                    return Some(AIAction::UseItem(i));
                }
            }
        }

        // Use mana potion if low and need skills
        if mp_percent < 20 && in_combat && self.config.use_skills {
            for (i, item) in player.inventory.iter().enumerate() {
                if matches!(item.kind, ItemKind::ManaPotion) {
                    return Some(AIAction::UseItem(i));
                }
            }
        }

        // Use buff potions in tough fights
        if in_combat {
            let visible_enemies: Vec<&Enemy> = enemies.iter()
                .filter(|e| e.is_alive() && map.visible[e.y][e.x])
                .collect();

            let tough_fight = visible_enemies.iter().any(|e| e.kind.is_boss()) ||
                visible_enemies.len() >= 4;

            if tough_fight {
                for (i, item) in player.inventory.iter().enumerate() {
                    match item.kind {
                        ItemKind::StrengthPotion if !player.has_status(StatusEffect::Strength) => {
                            return Some(AIAction::UseItem(i));
                        }
                        ItemKind::DefensePotion if !player.has_status(StatusEffect::Shield) => {
                            return Some(AIAction::UseItem(i));
                        }
                        ItemKind::InvisibilityPotion if hp_percent < 40 => {
                            return Some(AIAction::UseItem(i));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Cure status effects
        if player.has_status(StatusEffect::Poison) {
            for (i, item) in player.inventory.iter().enumerate() {
                if matches!(item.kind, ItemKind::PoisonResistPotion | ItemKind::CureAllPotion) {
                    return Some(AIAction::UseItem(i));
                }
            }
        }

        None
    }

    /// Evaluate what objectives to prioritize
    pub fn evaluate_objectives(
        &mut self,
        player: &Player,
        enemies: &[Enemy],
        map: &Map,
        dungeon_level: u32,
        boss_defeated: bool,
    ) -> AIObjective {
        let hp_percent = (player.hp * 100) / player.total_max_hp();
        let threat = self.evaluate_threats(player, enemies, map);

        // Increment objective turns
        self.objective_turns += 1;

        // Reset objective if stuck
        if self.objective_turns > self.config.max_idle_turns {
            self.current_objective = AIObjective::None;
            self.objective_turns = 0;
        }

        // Emergency situations
        if threat.recommendation == ThreatRecommendation::Flee {
            return AIObjective::Flee;
        }

        if threat.recommendation == ThreatRecommendation::Rest && !threat.boss_present {
            return AIObjective::Rest;
        }

        // Boss present - prioritize killing it
        if threat.boss_present && !boss_defeated {
            return AIObjective::DefeatBoss;
        }

        // Combat priority
        if threat.level >= ThreatLevel::Low && threat.recommendation == ThreatRecommendation::Engage {
            if let Some((_, enemy)) = enemies.iter().enumerate()
                .filter(|(_, e)| e.is_alive() && map.visible[e.y][e.x])
                .next()
            {
                return AIObjective::KillEnemy(enemy.x, enemy.y);
            }
        }

        // Based on play style
        match self.config.play_style {
            AutoPlayStyle::Speedrun => {
                // Rush to stairs
                if self.find_stairs(map).is_some() {
                    if boss_defeated || !BOSS_LEVELS.contains(&dungeon_level) {
                        return AIObjective::FindStairs;
                    }
                }
                AIObjective::Explore
            }
            AutoPlayStyle::Completionist => {
                // Full exploration
                if self.find_unexplored_area(player.x, player.y, map).is_some() {
                    return AIObjective::Explore;
                }
                // Then find shrines
                if self.config.use_shrines && self.find_shrine(player.x, player.y, map).is_some() {
                    return AIObjective::UseShrine;
                }
                AIObjective::FindStairs
            }
            AutoPlayStyle::Aggressive => {
                // Seek combat
                if enemies.iter().any(|e| e.is_alive()) {
                    if let Some(enemy) = enemies.iter().find(|e| e.is_alive() && map.visible[e.y][e.x]) {
                        return AIObjective::KillEnemy(enemy.x, enemy.y);
                    }
                }
                AIObjective::Explore
            }
            AutoPlayStyle::Defensive | AutoPlayStyle::Balanced => {
                // Balanced approach
                if hp_percent < 70 && !enemies.iter().any(|e| e.is_alive() && map.visible[e.y][e.x]) {
                    return AIObjective::Rest;
                }
                if self.find_unexplored_area(player.x, player.y, map).is_some() {
                    return AIObjective::Explore;
                }
                AIObjective::FindStairs
            }
        }
    }

    // ========================================================================
    // LEARNING AND ADAPTATION
    // ========================================================================

    /// Record combat result for learning
    pub fn record_combat(&mut self, enemy_kind: EnemyKind, damage_dealt: i32, damage_received: i32, killed: bool, skill_used: Option<Skill>, skill_damage: i32) {
        let memory = self.combat_memory.get_or_insert(enemy_kind);
        memory.total_damage_dealt += damage_dealt as i64;
        memory.total_damage_received += damage_received as i64;
        memory.encounters += 1;

        if killed {
            memory.kills += 1;
        }

        if let Some(skill) = skill_used {
            if skill_damage > memory.best_skill_damage {
                memory.best_skill_name = Some(skill.name().to_string());
                memory.best_skill_damage = skill_damage;
            }
        }
    }

    /// Record a death caused by enemy
    pub fn record_death(&mut self, enemy_kind: EnemyKind) {
        let memory = self.combat_memory.get_or_insert(enemy_kind);
        memory.deaths += 1;
    }

    /// Get effectiveness rating against enemy type
    pub fn get_enemy_effectiveness(&self, enemy_kind: EnemyKind) -> f32 {
        if let Some(memory) = self.combat_memory.get(enemy_kind) {
            if memory.encounters == 0 {
                return 0.5;
            }
            let kill_rate = memory.kills as f32 / memory.encounters as f32;
            let damage_ratio = if memory.total_damage_received > 0 {
                memory.total_damage_dealt as f32 / memory.total_damage_received as f32
            } else {
                2.0
            };
            (kill_rate + damage_ratio.min(2.0) / 2.0) / 2.0
        } else {
            0.5 // Unknown enemy
        }
    }

    /// Get the best skill name to use against an enemy type
    pub fn get_best_skill_for_enemy(&self, enemy_kind: EnemyKind) -> Option<String> {
        self.combat_memory.get(enemy_kind).and_then(|m| m.best_skill_name.clone())
    }
}

// ============================================================================
// AUTO PLAYER
// ============================================================================

/// Performance statistics for the auto player
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AutoPlayStats {
    /// Total turns played
    pub turns_played: u64,
    /// Enemies killed
    pub enemies_killed: u64,
    /// Damage dealt
    pub damage_dealt: u64,
    /// Damage received
    pub damage_received: u64,
    /// Items collected
    pub items_collected: u64,
    /// Gold collected
    pub gold_collected: u64,
    /// Skills used
    pub skills_used: u64,
    /// Items used
    pub items_used: u64,
    /// Deaths
    pub deaths: u64,
    /// Floors cleared
    pub floors_cleared: u64,
    /// Bosses defeated
    pub bosses_defeated: u64,
    /// Wins
    pub victories: u64,
    /// Average turns per floor
    pub avg_turns_per_floor: f32,
    /// Best floor reached
    pub best_floor: u32,
    /// Fastest victory (turns)
    pub fastest_victory: Option<u64>,
}

/// The main auto-player controller
#[derive(Clone)]
pub struct AutoPlayer {
    /// Configuration
    pub config: AutoPlayConfig,
    /// Decision engine
    pub engine: AIDecisionEngine,
    /// Performance statistics
    pub stats: AutoPlayStats,
    /// Whether auto-play is running
    pub running: bool,
    /// Step mode (advance one action at a time)
    pub step_mode: bool,
    /// Actions per second limit
    pub speed_limit: Option<u32>,
    /// Current turn on this floor
    floor_turns: u32,
}

impl Default for AutoPlayer {
    fn default() -> Self {
        Self::new(AutoPlayConfig::default())
    }
}

impl AutoPlayer {
    /// Create a new auto player with configuration
    pub fn new(config: AutoPlayConfig) -> Self {
        Self {
            engine: AIDecisionEngine::new(config.clone()),
            config,
            stats: AutoPlayStats::default(),
            running: false,
            step_mode: false,
            speed_limit: Some(10),
            floor_turns: 0,
        }
    }

    /// Create with a specific play style
    pub fn with_style(style: AutoPlayStyle) -> Self {
        let config = match style {
            AutoPlayStyle::Aggressive => AutoPlayConfig::aggressive(),
            AutoPlayStyle::Defensive => AutoPlayConfig::defensive(),
            AutoPlayStyle::Speedrun => AutoPlayConfig::speedrun(),
            AutoPlayStyle::Completionist => AutoPlayConfig::completionist(),
            AutoPlayStyle::Balanced => AutoPlayConfig::default(),
        };
        Self::new(config)
    }

    /// Start auto-play
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop auto-play
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Toggle step mode
    pub fn toggle_step_mode(&mut self) {
        self.step_mode = !self.step_mode;
    }

    /// Reset for a new level
    pub fn on_new_level(&mut self) {
        self.stats.avg_turns_per_floor = if self.stats.floors_cleared > 0 {
            self.stats.turns_played as f32 / self.stats.floors_cleared as f32
        } else {
            0.0
        };
        self.floor_turns = 0;
        self.engine.reset_level();
    }

    /// Called when floor is cleared
    pub fn on_floor_cleared(&mut self, floor: u32) {
        self.stats.floors_cleared += 1;
        if floor > self.stats.best_floor {
            self.stats.best_floor = floor;
        }
    }

    /// Called on victory
    pub fn on_victory(&mut self, total_turns: u64) {
        self.stats.victories += 1;
        if self.stats.fastest_victory.is_none() || total_turns < self.stats.fastest_victory.unwrap() {
            self.stats.fastest_victory = Some(total_turns);
        }
    }

    /// Called on death
    pub fn on_death(&mut self, enemy_kind: Option<EnemyKind>) {
        self.stats.deaths += 1;
        if let Some(kind) = enemy_kind {
            self.engine.record_death(kind);
        }
    }

    /// Determine the optimal action for the current game state
    pub fn decide(
        &mut self,
        player: &Player,
        enemies: &[Enemy],
        items: &[crate::items::Item],
        map: &Map,
        dungeon_level: u32,
        boss_defeated: bool,
    ) -> AIAction {
        self.floor_turns += 1;
        self.stats.turns_played += 1;

        // 1. Check for emergency item use
        if let Some(action) = self.engine.evaluate_item_use(player, enemies, map) {
            self.stats.items_used += 1;
            return action;
        }

        // 2. Evaluate current threats
        let threat = self.engine.evaluate_threats(player, enemies, map);

        // 3. Handle flee situations
        if threat.recommendation == ThreatRecommendation::Flee {
            if let Some(action) = self.get_flee_action(player, enemies, map) {
                return action;
            }
        }

        // 4. Combat actions if enemies visible
        if threat.level >= ThreatLevel::Low {
            if let Some(action) = self.engine.evaluate_combat_action(player, enemies, map) {
                return action;
            }
        }

        // 5. Update objectives
        let objective = self.engine.evaluate_objectives(player, enemies, map, dungeon_level, boss_defeated);
        self.engine.current_objective = objective;

        // 6. Movement/exploration
        if let Some(action) = self.engine.evaluate_movement(player, enemies, map, dungeon_level, boss_defeated) {
            return action;
        }

        // 7. Random exploration if stuck
        let mut rng = thread_rng();
        let directions = [
            (0, -1), (0, 1), (-1, 0), (1, 0),
            (-1, -1), (1, -1), (-1, 1), (1, 1),
        ];
        let mut shuffled = directions.to_vec();
        shuffled.shuffle(&mut rng);

        for (dx, dy) in shuffled {
            let nx = (player.x as i32 + dx) as usize;
            let ny = (player.y as i32 + dy) as usize;
            if nx < MAP_WIDTH && ny < MAP_HEIGHT && map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
        }

        AIAction::Wait
    }

    /// Get flee action
    fn get_flee_action(&self, player: &Player, enemies: &[Enemy], map: &Map) -> Option<AIAction> {
        let px = player.x as i32;
        let py = player.y as i32;

        // Calculate direction away from enemies
        let mut flee_dx = 0i32;
        let mut flee_dy = 0i32;

        for enemy in enemies.iter().filter(|e| e.is_alive() && map.visible[e.y][e.x]) {
            flee_dx -= (enemy.x as i32 - px).signum();
            flee_dy -= (enemy.y as i32 - py).signum();
        }

        if flee_dx == 0 && flee_dy == 0 {
            return None;
        }

        flee_dx = flee_dx.signum();
        flee_dy = flee_dy.signum();

        // Try to move in flee direction
        let directions = [
            (flee_dx, flee_dy),
            (flee_dx, 0),
            (0, flee_dy),
            (-flee_dy, flee_dx),
        ];

        for (dx, dy) in directions {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if nx < MAP_WIDTH && ny < MAP_HEIGHT && map.is_walkable(nx, ny) {
                let enemy_at_pos = enemies.iter().any(|e| e.x == nx && e.y == ny && e.is_alive());
                if !enemy_at_pos {
                    return Some(AIAction::Flee(dx, dy));
                }
            }
        }

        None
    }
}

// ============================================================================
// AI PROFILES (Class-specific configurations)
// ============================================================================

/// Pre-configured AI profiles for different character classes
#[derive(Clone, Debug)]
pub struct AIProfile {
    pub name: &'static str,
    pub description: &'static str,
    pub config: AutoPlayConfig,
    pub preferred_skills: Vec<Skill>,
    pub avoid_skills: Vec<Skill>,
}

impl AIProfile {
    /// Get profile for a character class
    pub fn for_class(class: CharacterClass) -> Self {
        match class {
            CharacterClass::Warrior => Self::warrior(),
            CharacterClass::Mage => Self::mage(),
            CharacterClass::Rogue => Self::rogue(),
            CharacterClass::Paladin => Self::paladin(),
            CharacterClass::Ranger => Self::ranger(),
            CharacterClass::Necromancer => Self::necromancer(),
        }
    }

    /// Warrior AI - Tank, melee focused
    pub fn warrior() -> Self {
        let mut config = AutoPlayConfig::aggressive();
        config.rest_hp_threshold = 40;
        config.flee_threshold = 15;
        config.target_priority = TargetPriority::Strongest;

        Self {
            name: "Warrior",
            description: "Aggressive melee fighter, prioritizes strongest enemies",
            config,
            preferred_skills: vec![Skill::Berserk, Skill::Cleave, Skill::Whirlwind],
            avoid_skills: vec![],
        }
    }

    /// Mage AI - Ranged, spell focused
    pub fn mage() -> Self {
        let mut config = AutoPlayConfig::defensive();
        config.rest_mp_threshold = 40;
        config.target_priority = TargetPriority::Closest;
        config.use_skills = true;

        Self {
            name: "Mage",
            description: "Ranged spellcaster, maintains distance and uses AoE",
            config,
            preferred_skills: vec![Skill::Fireball, Skill::Lightning, Skill::IceSpear],
            avoid_skills: vec![],
        }
    }

    /// Rogue AI - Stealth, evasion focused
    pub fn rogue() -> Self {
        let mut config = AutoPlayConfig::default();
        config.play_style = AutoPlayStyle::Aggressive;
        config.flee_threshold = 25;
        config.target_priority = TargetPriority::Weakest;

        Self {
            name: "Rogue",
            description: "Hit-and-run tactics, picks off weak enemies",
            config,
            preferred_skills: vec![Skill::Backstab, Skill::Vanish, Skill::PoisonBlade],
            avoid_skills: vec![],
        }
    }

    /// Paladin AI - Balanced, healing focused
    pub fn paladin() -> Self {
        let mut config = AutoPlayConfig::default();
        config.rest_hp_threshold = 50;
        config.target_priority = TargetPriority::MostDangerous;

        Self {
            name: "Paladin",
            description: "Balanced fighter with self-healing, targets undead",
            config,
            preferred_skills: vec![Skill::HolyLight, Skill::DivineShield, Skill::Smite],
            avoid_skills: vec![],
        }
    }

    /// Ranger AI - Ranged, traps
    pub fn ranger() -> Self {
        let mut config = AutoPlayConfig::default();
        config.play_style = AutoPlayStyle::Balanced;
        config.target_priority = TargetPriority::Closest;

        Self {
            name: "Ranger",
            description: "Ranged attacker with multi-target capabilities",
            config,
            preferred_skills: vec![Skill::MultiShot, Skill::EagleEye, Skill::PoisonArrow],
            avoid_skills: vec![],
        }
    }

    /// Necromancer AI - Summons, life drain
    pub fn necromancer() -> Self {
        let mut config = AutoPlayConfig::defensive();
        config.target_priority = TargetPriority::Weakest;

        Self {
            name: "Necromancer",
            description: "Summons minions and drains life from enemies",
            config,
            preferred_skills: vec![Skill::RaiseDead, Skill::LifeDrain, Skill::Curse],
            avoid_skills: vec![Skill::DarkPact], // Too risky for AI
        }
    }
}

// ============================================================================
// LEGACY SUPPORT - Original AIDecider
// ============================================================================

/// AI decision making for auto-play mode (Legacy compatibility)
pub struct AIDecider;

impl AIDecider {
    /// Decide the best action for the player (Legacy interface)
    pub fn decide(
        player: &Player,
        enemies: &[Enemy],
        _items: &[crate::items::Item],
        map: &Map,
        dungeon_level: u32,
        boss_defeated: bool,
    ) -> AIAction {
        let config = AutoPlayConfig::default();
        let mut auto_player = AutoPlayer::new(config);
        auto_player.decide(player, enemies, &[], map, dungeon_level, boss_defeated)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::CharacterClass;

    #[test]
    fn test_auto_play_config_default() {
        let config = AutoPlayConfig::default();
        assert_eq!(config.play_style, AutoPlayStyle::Balanced);
        assert!(config.use_skills);
        assert!(config.use_items);
    }

    #[test]
    fn test_auto_play_styles() {
        for style in AutoPlayStyle::all() {
            assert!(!style.name().is_empty());
            assert!(!style.description().is_empty());
        }
    }

    #[test]
    fn test_ai_profile_for_each_class() {
        for class in CharacterClass::all() {
            let profile = AIProfile::for_class(class);
            assert!(!profile.name.is_empty());
            assert!(!profile.preferred_skills.is_empty());
        }
    }

    #[test]
    fn test_loot_filter() {
        let filter = LootFilter::default();
        let player = Player::new(0, 0, CharacterClass::Warrior);

        assert!(filter.should_pickup(ItemKind::HealthPotion, Rarity::Common, &player));
        assert!(filter.should_pickup(ItemKind::Gold, Rarity::Common, &player));
        assert!(filter.should_pickup(ItemKind::Bread, Rarity::Common, &player));
        assert!(filter.should_pickup(ItemKind::LongSword, Rarity::Common, &player));
    }

    #[test]
    fn test_threat_levels_ordering() {
        assert!(ThreatLevel::None < ThreatLevel::Low);
        assert!(ThreatLevel::Low < ThreatLevel::Moderate);
        assert!(ThreatLevel::Moderate < ThreatLevel::High);
        assert!(ThreatLevel::High < ThreatLevel::Critical);
        assert!(ThreatLevel::Critical < ThreatLevel::Lethal);
    }

    #[test]
    fn test_ai_decision_engine_creation() {
        let config = AutoPlayConfig::aggressive();
        let engine = AIDecisionEngine::new(config);
        assert!(engine.combat_memory.entries.is_empty());
    }

    #[test]
    fn test_auto_player_creation() {
        let player = AutoPlayer::with_style(AutoPlayStyle::Speedrun);
        assert!(!player.running);
        assert_eq!(player.config.play_style, AutoPlayStyle::Speedrun);
    }

    #[test]
    fn test_combat_memory_store() {
        let mut store = CombatMemoryStore::new();
        let memory = store.get_or_insert(EnemyKind::Rat);
        memory.encounters = 5;
        memory.kills = 4;

        let retrieved = store.get(EnemyKind::Rat);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().encounters, 5);
    }

    #[test]
    fn test_stats_tracking() {
        let mut player = AutoPlayer::new(AutoPlayConfig::default());
        player.stats.enemies_killed = 10;
        player.stats.damage_dealt = 500;
        player.on_floor_cleared(5);

        assert_eq!(player.stats.floors_cleared, 1);
        assert_eq!(player.stats.best_floor, 5);
    }
}

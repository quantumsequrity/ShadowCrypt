//! Entity system: player, enemies, and their properties
//!
//! This module provides a complete autonomous entity system where all entities
//! (enemies, NPCs, companions) move and act independently, creating a living world.

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::classes::CharacterClass;
use crate::combat::{StatusEffect, ElementType, ElementalResistances};
use crate::companions::Companion;
use crate::items::{EquipSlot, Item, ItemKind, FoodQuality};
use crate::magic::Skill;

// ============================================================================
// Autonomous Entity Behavior System
// ============================================================================

/// Behavioral states for autonomous entities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum EntityBehavior {
    /// Standing still, waiting for stimuli
    #[default]
    Idle,
    /// Moving along a set patrol route
    Patrol,
    /// Actively hunting a target (player or other enemy)
    Hunt,
    /// Running away from danger
    Flee,
    /// Guarding a specific location
    Guard,
    /// Wandering randomly
    Wander,
    /// Engaged in trading activities (for merchants)
    Trade,
    /// Sleeping/resting (reduced awareness)
    Sleep,
    /// Fighting another entity
    Combat,
    /// Socializing with another entity
    Socialize,
    /// Following another entity
    Follow,
    /// Investigating a sound or event
    Investigate,
    /// Territorial defense - attacking intruders
    Territorial,
    /// Foraging/hunting for food (wild creatures)
    Forage,
}

impl EntityBehavior {
    /// Returns the display name of this behavior
    pub fn name(&self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Patrol => "Patrolling",
            Self::Hunt => "Hunting",
            Self::Flee => "Fleeing",
            Self::Guard => "Guarding",
            Self::Wander => "Wandering",
            Self::Trade => "Trading",
            Self::Sleep => "Sleeping",
            Self::Combat => "Fighting",
            Self::Socialize => "Socializing",
            Self::Follow => "Following",
            Self::Investigate => "Investigating",
            Self::Territorial => "Defending Territory",
            Self::Forage => "Foraging",
        }
    }

    /// Returns the awareness level (0.0 - 1.0) for this behavior
    pub fn awareness(&self) -> f32 {
        match self {
            Self::Sleep => 0.2,
            Self::Idle => 0.5,
            Self::Wander | Self::Forage => 0.6,
            Self::Trade | Self::Socialize => 0.4,
            Self::Patrol | Self::Guard => 0.8,
            Self::Hunt | Self::Combat | Self::Territorial => 1.0,
            Self::Flee | Self::Investigate => 0.9,
            Self::Follow => 0.7,
        }
    }
}

/// Movement patterns for entities
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum MovementPattern {
    /// Entity stays in place
    #[default]
    Stationary,
    /// Moves randomly within an area
    Random { radius: usize },
    /// Patrols between waypoints
    Patrol { waypoints: Vec<(usize, usize)>, current_idx: usize, reverse: bool },
    /// Follows another entity (by position tracking)
    Follow { target_x: usize, target_y: usize, min_dist: usize, max_dist: usize },
    /// Flees from a position
    Flee { from_x: usize, from_y: usize },
    /// Circles around a point
    Circle { center_x: usize, center_y: usize, radius: usize, angle: f32 },
    /// Moves towards a specific destination
    Destination { target_x: usize, target_y: usize },
    /// Territorial - stays within a defined area
    Territory { center_x: usize, center_y: usize, radius: usize },
}

impl MovementPattern {
    /// Create a patrol pattern between points
    pub fn new_patrol(points: Vec<(usize, usize)>) -> Self {
        Self::Patrol { waypoints: points, current_idx: 0, reverse: false }
    }

    /// Create a territory pattern
    pub fn new_territory(x: usize, y: usize, radius: usize) -> Self {
        Self::Territory { center_x: x, center_y: y, radius }
    }

    /// Create a follow pattern
    pub fn new_follow(x: usize, y: usize) -> Self {
        Self::Follow { target_x: x, target_y: y, min_dist: 2, max_dist: 5 }
    }
}

/// Disposition towards other entities
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum EntityDisposition {
    /// Will attack on sight
    Hostile,
    /// Wary but won't attack unprovoked
    #[default]
    Neutral,
    /// Friendly, will help
    Friendly,
    /// Allied, will fight together
    Allied,
    /// Fearful, will flee
    Fearful,
}

/// Entity faction for determining relationships
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum EntityFaction {
    /// Player and allies
    Player,
    /// Undead creatures
    Undead,
    /// Goblinoid creatures
    Goblinoid,
    /// Beast/animal creatures
    Beast,
    /// Elemental creatures
    Elemental,
    /// Demon creatures
    Demon,
    /// Neutral creatures
    #[default]
    Neutral,
    /// Friendly NPCs
    Friendly,
    /// Wild/untamed creatures
    Wild,
}

impl EntityFaction {
    /// Get disposition towards another faction
    pub fn disposition_towards(&self, other: &EntityFaction) -> EntityDisposition {
        if self == other {
            return EntityDisposition::Allied;
        }

        match (self, other) {
            // Player relations
            (EntityFaction::Player, EntityFaction::Friendly) => EntityDisposition::Friendly,
            (EntityFaction::Friendly, EntityFaction::Player) => EntityDisposition::Friendly,

            // Hostile factions
            (EntityFaction::Undead, EntityFaction::Beast) => EntityDisposition::Hostile,
            (EntityFaction::Beast, EntityFaction::Undead) => EntityDisposition::Hostile,
            (EntityFaction::Goblinoid, EntityFaction::Beast) => EntityDisposition::Hostile,
            (EntityFaction::Demon, _) => EntityDisposition::Hostile,
            (_, EntityFaction::Demon) => EntityDisposition::Hostile,

            // Territorial conflicts
            (EntityFaction::Beast, EntityFaction::Wild) => EntityDisposition::Neutral,
            (EntityFaction::Wild, _) => EntityDisposition::Hostile,

            // All hostile to player
            (_, EntityFaction::Player) => EntityDisposition::Hostile,
            (EntityFaction::Player, _) => EntityDisposition::Hostile,

            _ => EntityDisposition::Neutral,
        }
    }
}

/// AI decision-making for autonomous entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EntityAI {
    /// Current behavior state
    pub behavior: EntityBehavior,
    /// Movement pattern
    pub movement: MovementPattern,
    /// Entity faction
    pub faction: EntityFaction,
    /// Action points (speed-based action scheduling)
    pub action_points: i32,
    /// Action cost for different actions
    pub base_speed: i32,
    /// Current target position (if any)
    pub target_pos: Option<(usize, usize)>,
    /// Current target entity ID (if tracking someone)
    pub target_entity: Option<u64>,
    /// Home position (for territorial/guard behavior)
    pub home_pos: Option<(usize, usize)>,
    /// Aggro range - how far entity can detect threats
    pub aggro_range: usize,
    /// Vision range
    pub vision_range: usize,
    /// Memory of last seen player position
    pub last_seen_player: Option<(usize, usize)>,
    /// Turns since last saw player
    pub turns_since_player: u32,
    /// Interest points for investigation
    pub interest_points: Vec<(usize, usize, u32)>, // x, y, priority
    /// Time spent in current behavior
    pub behavior_timer: u32,
    /// Whether entity is currently active (for sleep cycles)
    pub is_active: bool,
    /// Preferred time of activity (for day/night cycles)
    pub active_time: ActivityTime,
    /// Social cooldown (for NPC interactions)
    pub social_cooldown: u32,
    /// Restock timer (for merchants)
    pub restock_timer: u32,
}

/// Time preference for entity activity
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum ActivityTime {
    /// Active during day
    Diurnal,
    /// Active during night
    Nocturnal,
    /// Always active
    #[default]
    Always,
    /// Active at dawn/dusk
    Crepuscular,
}

impl Default for EntityAI {
    fn default() -> Self {
        Self {
            behavior: EntityBehavior::Idle,
            movement: MovementPattern::Stationary,
            faction: EntityFaction::Neutral,
            action_points: 0,
            base_speed: 10,
            target_pos: None,
            target_entity: None,
            home_pos: None,
            aggro_range: 8,
            vision_range: 10,
            last_seen_player: None,
            turns_since_player: 0,
            interest_points: Vec::new(),
            behavior_timer: 0,
            is_active: true,
            active_time: ActivityTime::Always,
            social_cooldown: 0,
            restock_timer: 0,
        }
    }
}

impl EntityAI {
    /// Create AI for an enemy type
    pub fn for_enemy(kind: EnemyKind) -> Self {
        let mut ai = Self::default();

        // Set faction based on enemy type
        ai.faction = kind.faction();

        // Set behavior and movement based on enemy type
        match kind {
            // Undead tend to patrol or guard
            EnemyKind::Skeleton | EnemyKind::Zombie | EnemyKind::Mummy => {
                ai.behavior = EntityBehavior::Patrol;
                ai.active_time = ActivityTime::Nocturnal;
            }
            // Ghosts wander
            EnemyKind::Ghost | EnemyKind::Wraith | EnemyKind::Banshee => {
                ai.behavior = EntityBehavior::Wander;
                ai.movement = MovementPattern::Random { radius: 15 };
                ai.active_time = ActivityTime::Nocturnal;
            }
            // Beasts are territorial
            EnemyKind::Wolf | EnemyKind::DireWolf | EnemyKind::CaveBear => {
                ai.behavior = EntityBehavior::Territorial;
                ai.aggro_range = 10;
            }
            // Goblins patrol and investigate
            EnemyKind::Goblin | EnemyKind::Hobgoblin | EnemyKind::Kobold => {
                ai.behavior = EntityBehavior::Patrol;
                ai.aggro_range = 6;
            }
            // Spiders ambush
            EnemyKind::Spider | EnemyKind::GiantSpider | EnemyKind::IceSpider => {
                ai.behavior = EntityBehavior::Guard;
                ai.aggro_range = 4;
            }
            // Bosses guard their room
            _ if kind.is_boss() => {
                ai.behavior = EntityBehavior::Guard;
                ai.aggro_range = 15;
                ai.vision_range = 20;
            }
            // Default behavior
            _ => {
                ai.behavior = EntityBehavior::Wander;
                ai.movement = MovementPattern::Random { radius: 8 };
            }
        }

        ai
    }

    /// Update action points based on speed
    pub fn tick_action_points(&mut self) {
        self.action_points += self.base_speed;
    }

    /// Check if entity has enough action points to act
    pub fn can_act(&self) -> bool {
        self.action_points >= 10
    }

    /// Consume action points for an action
    pub fn consume_action(&mut self, cost: i32) {
        self.action_points -= cost;
    }

    /// Add an interest point for investigation
    pub fn add_interest(&mut self, x: usize, y: usize, priority: u32) {
        // Remove old interest at same position
        self.interest_points.retain(|(ix, iy, _)| *ix != x || *iy != y);
        self.interest_points.push((x, y, priority));
        // Sort by priority (highest first)
        self.interest_points.sort_by(|a, b| b.2.cmp(&a.2));
        // Keep only top 5 interests
        self.interest_points.truncate(5);
    }

    /// Get the highest priority interest point
    pub fn get_top_interest(&self) -> Option<(usize, usize)> {
        self.interest_points.first().map(|(x, y, _)| (*x, *y))
    }

    /// Decay interest points over time
    pub fn decay_interests(&mut self) {
        for (_, _, priority) in &mut self.interest_points {
            *priority = priority.saturating_sub(1);
        }
        self.interest_points.retain(|(_, _, p)| *p > 0);
    }

    /// Update behavior based on current state
    pub fn update_behavior(&mut self, can_see_player: bool, player_dist: f32, hp_percent: f32) {
        self.behavior_timer += 1;

        // Flee if low health
        if hp_percent < 0.2 && self.behavior != EntityBehavior::Flee {
            self.behavior = EntityBehavior::Flee;
            self.behavior_timer = 0;
            return;
        }

        // Hunt if player visible and aggressive
        if can_see_player && player_dist < self.aggro_range as f32 {
            if self.behavior != EntityBehavior::Hunt && self.behavior != EntityBehavior::Flee {
                self.behavior = EntityBehavior::Hunt;
                self.behavior_timer = 0;
            }
            self.turns_since_player = 0;
            return;
        }

        // Update turns since player
        self.turns_since_player += 1;

        // Return to default behavior after losing player
        if self.turns_since_player > 10 && self.behavior == EntityBehavior::Hunt {
            self.behavior = EntityBehavior::Patrol;
            self.behavior_timer = 0;
        }

        // Investigate sounds/events
        if !self.interest_points.is_empty() && self.behavior != EntityBehavior::Hunt {
            self.behavior = EntityBehavior::Investigate;
        }
    }
}

/// Actions an entity can take autonomously
#[derive(Clone, Debug)]
pub enum EntityAction {
    /// Move in a direction
    Move(i32, i32),
    /// Attack a target at position
    Attack(usize, usize),
    /// Use a special ability
    UseAbility(usize), // ability index
    /// Wait/do nothing
    Wait,
    /// Flee from combat
    Flee,
    /// Interact with another entity (NPC conversation, etc.)
    Interact(u64), // target entity ID
    /// Patrol to next waypoint
    PatrolNext,
    /// Guard current position
    Guard,
    /// Investigate a point of interest
    Investigate(usize, usize),
    /// Rest/sleep
    Rest,
    /// Restock inventory (merchants)
    Restock,
    /// Make noise (alerts nearby entities)
    MakeNoise(u32), // noise radius
}

/// Result of entity interaction
#[derive(Clone, Debug)]
pub struct EntityInteractionResult {
    pub messages: Vec<String>,
    pub damage_dealt: Option<(u64, i32)>, // target_id, damage
    pub status_applied: Option<(u64, StatusEffect, u32)>, // target_id, effect, duration
    pub sound_made: Option<(usize, usize, u32)>, // x, y, radius
    pub xp_gained: Option<u32>,
    pub entity_died: Option<u64>,
}

impl Default for EntityInteractionResult {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            damage_dealt: None,
            status_applied: None,
            sound_made: None,
            xp_gained: None,
            entity_died: None,
        }
    }
}

// ============================================================================
// Entity Scheduling System
// ============================================================================

/// Scheduler for processing entity actions in order of speed
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EntityScheduler {
    /// Queue of entity IDs sorted by action points
    action_queue: Vec<(u64, i32)>, // entity_id, action_points
    /// Current game tick
    pub current_tick: u64,
    /// Base time units per turn
    pub time_per_turn: u32,
}

impl EntityScheduler {
    pub fn new() -> Self {
        Self {
            action_queue: Vec::new(),
            current_tick: 0,
            time_per_turn: 100,
        }
    }

    /// Add an entity to the scheduler
    pub fn add_entity(&mut self, entity_id: u64, speed: i32) {
        self.action_queue.push((entity_id, speed));
        self.sort_queue();
    }

    /// Remove an entity from the scheduler
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.action_queue.retain(|(id, _)| *id != entity_id);
    }

    /// Update an entity's action points
    pub fn update_entity(&mut self, entity_id: u64, action_points: i32) {
        if let Some((_, ap)) = self.action_queue.iter_mut().find(|(id, _)| *id == entity_id) {
            *ap = action_points;
        }
        self.sort_queue();
    }

    /// Get the next entity to act
    pub fn next_actor(&mut self) -> Option<u64> {
        if let Some((id, ap)) = self.action_queue.first() {
            if *ap >= 10 {
                return Some(*id);
            }
        }
        None
    }

    /// Advance time and grant action points to all entities
    pub fn tick(&mut self, entities: &mut [(u64, i32)]) {
        self.current_tick += 1;
        for (id, speed) in entities {
            if let Some((_, ap)) = self.action_queue.iter_mut().find(|(eid, _)| *eid == *id) {
                *ap += *speed;
            }
        }
        self.sort_queue();
    }

    /// Sort the queue by action points (highest first)
    fn sort_queue(&mut self) {
        self.action_queue.sort_by(|a, b| b.1.cmp(&a.1));
    }

    /// Get all entities ready to act this turn
    pub fn get_ready_entities(&self) -> Vec<u64> {
        self.action_queue.iter()
            .filter(|(_, ap)| *ap >= 10)
            .map(|(id, _)| *id)
            .collect()
    }
}

/// Hunger stages with increasing severity
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum HungerStage {
    Stuffed,      // 90-100+ hunger - slight speed penalty, bonus regen
    Satisfied,    // 70-89 hunger - optimal state, small bonuses
    Peckish,      // 50-69 hunger - neutral state
    Hungry,       // 30-49 hunger - minor penalties start
    VeryHungry,   // 10-29 hunger - moderate penalties
    Starving,     // 1-9 hunger - severe penalties
    Famished,     // 0 or below - taking damage, near death
}

impl HungerStage {
    /// Get hunger stage from current hunger value
    pub fn from_hunger(hunger: i32, max_hunger: i32) -> Self {
        let percent = (hunger * 100) / max_hunger.max(1);
        match percent {
            p if p >= 90 => HungerStage::Stuffed,
            p if p >= 70 => HungerStage::Satisfied,
            p if p >= 50 => HungerStage::Peckish,
            p if p >= 30 => HungerStage::Hungry,
            p if p >= 10 => HungerStage::VeryHungry,
            p if p >= 1 => HungerStage::Starving,
            _ => HungerStage::Famished,
        }
    }

    /// Get the name of this hunger stage
    pub fn name(&self) -> &'static str {
        match self {
            HungerStage::Stuffed => "Stuffed",
            HungerStage::Satisfied => "Satisfied",
            HungerStage::Peckish => "Peckish",
            HungerStage::Hungry => "Hungry",
            HungerStage::VeryHungry => "Very Hungry",
            HungerStage::Starving => "Starving",
            HungerStage::Famished => "Famished",
        }
    }

    /// Get color index for UI display
    pub fn color_index(&self) -> u8 {
        match self {
            HungerStage::Stuffed => 5,      // Green
            HungerStage::Satisfied => 13,   // Bright green
            HungerStage::Peckish => 1,      // White/grey
            HungerStage::Hungry => 11,      // Yellow
            HungerStage::VeryHungry => 6,   // Orange
            HungerStage::Starving => 3,     // Red
            HungerStage::Famished => 4,     // Dark red
        }
    }

    /// Get attack modifier for this hunger stage (percentage)
    pub fn attack_modifier(&self) -> i32 {
        match self {
            HungerStage::Stuffed => -5,     // Slightly sluggish
            HungerStage::Satisfied => 10,   // Well-nourished bonus
            HungerStage::Peckish => 0,
            HungerStage::Hungry => -5,
            HungerStage::VeryHungry => -15,
            HungerStage::Starving => -30,
            HungerStage::Famished => -50,
        }
    }

    /// Get defense modifier for this hunger stage (percentage)
    pub fn defense_modifier(&self) -> i32 {
        match self {
            HungerStage::Stuffed => 5,      // Full belly provides padding
            HungerStage::Satisfied => 5,
            HungerStage::Peckish => 0,
            HungerStage::Hungry => -5,
            HungerStage::VeryHungry => -10,
            HungerStage::Starving => -20,
            HungerStage::Famished => -40,
        }
    }

    /// Get HP regeneration modifier (flat bonus/penalty per regen tick)
    pub fn regen_modifier(&self) -> i32 {
        match self {
            HungerStage::Stuffed => 2,      // Extra regen when stuffed
            HungerStage::Satisfied => 1,
            HungerStage::Peckish => 0,
            HungerStage::Hungry => 0,
            HungerStage::VeryHungry => -1,  // Regen is slower
            HungerStage::Starving => -2,
            HungerStage::Famished => -3,
        }
    }

    /// Get hunger decay rate (how fast hunger depletes per tick)
    pub fn decay_rate(&self) -> i32 {
        match self {
            HungerStage::Stuffed => 2,      // Faster decay when overfull
            HungerStage::Satisfied => 1,
            HungerStage::Peckish => 1,
            HungerStage::Hungry => 1,
            HungerStage::VeryHungry => 1,
            HungerStage::Starving => 1,
            HungerStage::Famished => 0,     // Can't go lower
        }
    }

    /// Get starvation damage per tick (only for Famished)
    pub fn starvation_damage(&self) -> i32 {
        match self {
            HungerStage::Famished => 3,
            HungerStage::Starving => 1,
            _ => 0,
        }
    }
}

/// Satiation bonus from eating high-quality food
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct SatiationBonus {
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub max_hp_bonus: i32,
    pub regen_bonus: i32,
    pub turns_remaining: u32,
}

impl SatiationBonus {
    pub fn new(quality: FoodQuality, base_food_value: i32) -> Self {
        let multiplier = match quality {
            FoodQuality::Rotten => 0.0,
            FoodQuality::Raw => 0.5,
            FoodQuality::Stale => 0.75,
            FoodQuality::Fresh => 1.0,
            FoodQuality::Cooked => 1.5,
            FoodQuality::WellCooked => 2.0,
            FoodQuality::Gourmet => 3.0,
            FoodQuality::Legendary => 5.0,
        };

        let duration = match quality {
            FoodQuality::Rotten => 0,
            FoodQuality::Raw => 10,
            FoodQuality::Stale => 15,
            FoodQuality::Fresh => 25,
            FoodQuality::Cooked => 40,
            FoodQuality::WellCooked => 60,
            FoodQuality::Gourmet => 100,
            FoodQuality::Legendary => 200,
        };

        let base = (base_food_value as f32 * multiplier * 0.1) as i32;

        Self {
            attack_bonus: base.max(0),
            defense_bonus: (base / 2).max(0),
            max_hp_bonus: (base * 2).max(0),
            regen_bonus: if quality >= FoodQuality::Cooked { 1 } else { 0 },
            turns_remaining: duration,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.turns_remaining > 0 {
            self.turns_remaining -= 1;
            true
        } else {
            false
        }
    }

    pub fn is_active(&self) -> bool {
        self.turns_remaining > 0
    }
}

/// All enemy types in the game
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum EnemyKind {
    // Tier 1: Dungeon (levels 1-4)
    Rat,
    Bat,
    Spider,
    Goblin,
    Skeleton,
    Kobold,
    GiantRat,
    CaveCrawler,

    // Tier 2: Cave (levels 5-8)
    GiantSpider,
    Orc,
    Troll,
    CaveOgre,
    Slime,
    Hobgoblin,
    CaveBear,
    Mushroom,
    RockElemental,

    // Tier 3: Crypt (levels 9-12)
    Zombie,
    Ghost,
    Wraith,
    Vampire,
    Mummy,
    Ghoul,
    Banshee,
    DeathKnight,
    BoneGolem,

    // Tier 4: Forest (levels 13-16)
    Wolf,
    DireWolf,
    TreeEnt,
    ForestTroll,
    Druid,
    WildBoar,
    GiantWasp,
    VenomousVine,
    ForestSpirit,

    // Tier 5: Ice Cavern (levels 17-20)
    IceElemental,
    FrostGiant,
    YetiWarrior,
    IceWraith,
    FrostWolf,
    IceSpider,
    FrozenKnight,
    Wendigo,

    // Tier 6: Volcanic (levels 21-24)
    FireElemental,
    LavaGolem,
    Hellhound,
    FireDrake,
    MagmaSlime,
    Salamander,
    CinderWraith,
    InfernalImp,

    // Tier 7: Ancient Ruins (levels 25-28)
    Golem,
    AncientGuardian,
    Sphinx,
    Lich,
    Gargoyle,
    MummyLord,
    CursedStatue,
    ShadowAssassin,

    // Tier 8: Demon Realm (levels 29-30)
    Demon,
    DemonLord,
    Succubus,
    Balrog,
    PitFiend,
    ShadowDemon,
    AbyssalHorror,
    DoomGuard,

    // Bosses (one per area)
    BossGoblinKing,      // Level 5
    BossOrcWarlord,      // Level 10
    BossVampireLord,     // Level 15
    BossForestGuardian,  // Level 20
    BossIceDragon,       // Level 25
    BossDemonKing,       // Level 30

    // Mini-Bosses
    GoblinChampion,
    OrcBerserker,
    VampireElite,
    AncientWyrm,
    FrostLord,
    InfernalLord,
}

impl EnemyKind {
    /// Returns the glyph character for this enemy
    pub fn glyph(&self) -> char {
        match self {
            Self::Rat | Self::GiantRat => 'r',
            Self::Bat => 'b',
            Self::Spider => 's',
            Self::Goblin => 'g',
            Self::Skeleton => 'k',
            Self::Kobold => 'k',
            Self::CaveCrawler => 'c',
            Self::GiantSpider => 'S',
            Self::Orc => 'o',
            Self::Troll => 't',
            Self::CaveOgre => 'O',
            Self::Slime | Self::MagmaSlime => 'j',
            Self::Hobgoblin => 'h',
            Self::CaveBear => 'B',
            Self::Mushroom => 'm',
            Self::RockElemental => 'R',
            Self::Zombie => 'z',
            Self::Ghost => 'G',
            Self::Wraith | Self::CinderWraith => 'W',
            Self::Vampire | Self::VampireElite => 'V',
            Self::Mummy => 'M',
            Self::Ghoul => 'g',
            Self::Banshee => 'B',
            Self::DeathKnight => 'K',
            Self::BoneGolem => 'G',
            Self::Wolf | Self::FrostWolf => 'w',
            Self::DireWolf => 'W',
            Self::TreeEnt => 'T',
            Self::ForestTroll => 't',
            Self::Druid => 'd',
            Self::WildBoar => 'b',
            Self::GiantWasp => 'w',
            Self::VenomousVine => 'v',
            Self::ForestSpirit => 'S',
            Self::IceElemental => 'E',
            Self::FrostGiant => 'F',
            Self::YetiWarrior => 'Y',
            Self::IceWraith => 'w',
            Self::IceSpider => 'S',
            Self::FrozenKnight => 'K',
            Self::Wendigo => 'W',
            Self::FireElemental => 'E',
            Self::LavaGolem => 'L',
            Self::Hellhound => 'H',
            Self::FireDrake => 'D',
            Self::Salamander => 's',
            Self::InfernalImp => 'i',
            Self::Golem => 'G',
            Self::AncientGuardian => 'A',
            Self::Sphinx => 'X',
            Self::Lich => 'L',
            Self::Gargoyle => 'G',
            Self::MummyLord => 'M',
            Self::CursedStatue => 'S',
            Self::ShadowAssassin => 'a',
            Self::Demon => 'D',
            Self::DemonLord => '&',
            Self::Succubus => 's',
            Self::Balrog => 'B',
            Self::PitFiend => 'P',
            Self::ShadowDemon => 'S',
            Self::AbyssalHorror => 'H',
            Self::DoomGuard => 'D',
            Self::BossGoblinKing | Self::GoblinChampion => 'K',
            Self::BossOrcWarlord | Self::OrcBerserker => 'W',
            Self::BossVampireLord => 'V',
            Self::BossForestGuardian | Self::AncientWyrm => 'G',
            Self::BossIceDragon | Self::FrostLord => 'D',
            Self::BossDemonKing | Self::InfernalLord => '&',
        }
    }

    /// Returns a color index for this enemy (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Rat | Self::Bat | Self::GiantRat | Self::CaveCrawler => 0,
            Self::Spider | Self::GiantSpider | Self::IceSpider => 12,
            Self::Goblin | Self::BossGoblinKing | Self::GoblinChampion | Self::Kobold => 5,
            Self::Skeleton | Self::Mummy | Self::BoneGolem => 2,
            Self::Orc | Self::BossOrcWarlord | Self::OrcBerserker | Self::Hobgoblin => 6,
            Self::Troll | Self::ForestTroll => 10,
            Self::CaveOgre | Self::CaveBear => 12,
            Self::Slime | Self::Mushroom => 5,
            Self::RockElemental => 1,
            Self::Zombie | Self::Ghoul => 6,
            Self::Ghost | Self::Wraith | Self::IceWraith | Self::Banshee => 1,
            Self::Vampire | Self::BossVampireLord | Self::VampireElite => 4,
            Self::DeathKnight => 14,
            Self::Wolf | Self::DireWolf | Self::FrostWolf => 1,
            Self::TreeEnt | Self::BossForestGuardian | Self::VenomousVine => 5,
            Self::Druid | Self::ForestSpirit => 6,
            Self::WildBoar => 12,
            Self::GiantWasp => 11,
            Self::IceElemental | Self::FrostGiant | Self::YetiWarrior | Self::BossIceDragon
            | Self::FrozenKnight | Self::Wendigo | Self::FrostLord => 9,
            Self::FireElemental | Self::LavaGolem | Self::Hellhound | Self::FireDrake
            | Self::MagmaSlime | Self::Salamander | Self::CinderWraith | Self::InfernalImp
            | Self::InfernalLord => 3,
            Self::Golem | Self::AncientGuardian | Self::CursedStatue => 11,
            Self::Sphinx | Self::Gargoyle => 11,
            Self::Lich | Self::MummyLord => 13,
            Self::ShadowAssassin => 0,
            Self::Demon | Self::DemonLord | Self::Balrog | Self::BossDemonKing
            | Self::PitFiend | Self::DoomGuard => 3,
            Self::Succubus | Self::ShadowDemon => 13,
            Self::AbyssalHorror => 4,
            Self::AncientWyrm => 5,
        }
    }

    /// Returns the display name of this enemy
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rat => "Rat",
            Self::Bat => "Bat",
            Self::Spider => "Spider",
            Self::Goblin => "Goblin",
            Self::Skeleton => "Skeleton",
            Self::Kobold => "Kobold",
            Self::GiantRat => "Giant Rat",
            Self::CaveCrawler => "Cave Crawler",
            Self::GiantSpider => "Giant Spider",
            Self::Orc => "Orc",
            Self::Troll => "Troll",
            Self::CaveOgre => "Cave Ogre",
            Self::Slime => "Slime",
            Self::Hobgoblin => "Hobgoblin",
            Self::CaveBear => "Cave Bear",
            Self::Mushroom => "Toxic Mushroom",
            Self::RockElemental => "Rock Elemental",
            Self::Zombie => "Zombie",
            Self::Ghost => "Ghost",
            Self::Wraith => "Wraith",
            Self::Vampire => "Vampire",
            Self::Mummy => "Mummy",
            Self::Ghoul => "Ghoul",
            Self::Banshee => "Banshee",
            Self::DeathKnight => "Death Knight",
            Self::BoneGolem => "Bone Golem",
            Self::Wolf => "Wolf",
            Self::DireWolf => "Dire Wolf",
            Self::TreeEnt => "Tree Ent",
            Self::ForestTroll => "Forest Troll",
            Self::Druid => "Corrupted Druid",
            Self::WildBoar => "Wild Boar",
            Self::GiantWasp => "Giant Wasp",
            Self::VenomousVine => "Venomous Vine",
            Self::ForestSpirit => "Forest Spirit",
            Self::IceElemental => "Ice Elemental",
            Self::FrostGiant => "Frost Giant",
            Self::YetiWarrior => "Yeti Warrior",
            Self::IceWraith => "Ice Wraith",
            Self::FrostWolf => "Frost Wolf",
            Self::IceSpider => "Ice Spider",
            Self::FrozenKnight => "Frozen Knight",
            Self::Wendigo => "Wendigo",
            Self::FireElemental => "Fire Elemental",
            Self::LavaGolem => "Lava Golem",
            Self::Hellhound => "Hellhound",
            Self::FireDrake => "Fire Drake",
            Self::MagmaSlime => "Magma Slime",
            Self::Salamander => "Salamander",
            Self::CinderWraith => "Cinder Wraith",
            Self::InfernalImp => "Infernal Imp",
            Self::Golem => "Stone Golem",
            Self::AncientGuardian => "Ancient Guardian",
            Self::Sphinx => "Sphinx",
            Self::Lich => "Lich",
            Self::Gargoyle => "Gargoyle",
            Self::MummyLord => "Mummy Lord",
            Self::CursedStatue => "Cursed Statue",
            Self::ShadowAssassin => "Shadow Assassin",
            Self::Demon => "Demon",
            Self::DemonLord => "Demon Lord",
            Self::Succubus => "Succubus",
            Self::Balrog => "Balrog",
            Self::PitFiend => "Pit Fiend",
            Self::ShadowDemon => "Shadow Demon",
            Self::AbyssalHorror => "Abyssal Horror",
            Self::DoomGuard => "Doom Guard",
            Self::BossGoblinKing => "GOBLIN KING",
            Self::BossOrcWarlord => "ORC WARLORD",
            Self::BossVampireLord => "VAMPIRE LORD",
            Self::BossForestGuardian => "FOREST GUARDIAN",
            Self::BossIceDragon => "ICE DRAGON",
            Self::BossDemonKing => "DEMON KING",
            Self::GoblinChampion => "Goblin Champion",
            Self::OrcBerserker => "Orc Berserker",
            Self::VampireElite => "Vampire Elite",
            Self::AncientWyrm => "Ancient Wyrm",
            Self::FrostLord => "Frost Lord",
            Self::InfernalLord => "Infernal Lord",
        }
    }

    /// Returns base stats: (hp, attack, defense, xp_value)
    pub fn base_stats(&self) -> (i32, i32, i32, u32) {
        match self {
            // Tier 1: Dungeon
            Self::Rat => (8, 3, 0, 5),
            Self::Bat => (6, 2, 0, 4),
            Self::Spider => (10, 4, 1, 8),
            Self::Goblin => (15, 5, 2, 12),
            Self::Skeleton => (12, 6, 1, 10),
            Self::Kobold => (10, 4, 1, 7),
            Self::GiantRat => (14, 5, 1, 10),
            Self::CaveCrawler => (18, 6, 2, 15),

            // Tier 2: Cave
            Self::GiantSpider => (25, 8, 3, 25),
            Self::Orc => (35, 10, 4, 30),
            Self::Troll => (50, 8, 6, 40),
            Self::CaveOgre => (60, 12, 5, 50),
            Self::Slime => (40, 6, 8, 35),
            Self::Hobgoblin => (30, 9, 4, 28),
            Self::CaveBear => (55, 14, 6, 45),
            Self::Mushroom => (20, 5, 2, 20),
            Self::RockElemental => (70, 10, 12, 55),

            // Tier 3: Crypt
            Self::Zombie => (45, 10, 4, 40),
            Self::Ghost => (30, 12, 2, 45),
            Self::Wraith => (35, 15, 3, 55),
            Self::Vampire => (55, 14, 6, 70),
            Self::Mummy => (50, 11, 8, 60),
            Self::Ghoul => (40, 12, 3, 50),
            Self::Banshee => (35, 16, 2, 65),
            Self::DeathKnight => (75, 18, 10, 85),
            Self::BoneGolem => (90, 14, 12, 80),

            // Tier 4: Forest
            Self::Wolf => (40, 12, 3, 50),
            Self::DireWolf => (60, 16, 5, 75),
            Self::TreeEnt => (100, 14, 12, 100),
            Self::ForestTroll => (80, 15, 8, 90),
            Self::Druid => (50, 18, 4, 80),
            Self::WildBoar => (55, 14, 5, 60),
            Self::GiantWasp => (35, 16, 2, 55),
            Self::VenomousVine => (45, 12, 6, 65),
            Self::ForestSpirit => (40, 20, 3, 75),

            // Tier 5: Ice Cavern
            Self::IceElemental => (70, 18, 8, 110),
            Self::FrostGiant => (120, 22, 12, 150),
            Self::YetiWarrior => (90, 20, 10, 130),
            Self::IceWraith => (60, 24, 6, 120),
            Self::FrostWolf => (55, 18, 6, 100),
            Self::IceSpider => (50, 16, 5, 95),
            Self::FrozenKnight => (100, 22, 14, 140),
            Self::Wendigo => (85, 26, 8, 160),

            // Tier 6: Volcanic
            Self::FireElemental => (80, 22, 8, 140),
            Self::LavaGolem => (150, 20, 18, 180),
            Self::Hellhound => (70, 25, 8, 150),
            Self::FireDrake => (100, 28, 12, 200),
            Self::MagmaSlime => (65, 18, 10, 130),
            Self::Salamander => (75, 24, 7, 160),
            Self::CinderWraith => (55, 28, 4, 170),
            Self::InfernalImp => (45, 22, 5, 120),

            // Tier 7: Ancient Ruins
            Self::Golem => (180, 22, 20, 220),
            Self::AncientGuardian => (200, 25, 22, 250),
            Self::Sphinx => (150, 30, 15, 280),
            Self::Lich => (120, 35, 12, 300),
            Self::Gargoyle => (130, 24, 18, 200),
            Self::MummyLord => (160, 28, 16, 260),
            Self::CursedStatue => (220, 20, 25, 240),
            Self::ShadowAssassin => (80, 40, 8, 280),

            // Tier 8: Demon Realm
            Self::Demon => (140, 30, 15, 280),
            Self::DemonLord => (200, 35, 20, 350),
            Self::Succubus => (100, 28, 10, 250),
            Self::Balrog => (250, 40, 25, 400),
            Self::PitFiend => (180, 38, 18, 380),
            Self::ShadowDemon => (120, 35, 12, 320),
            Self::AbyssalHorror => (300, 45, 22, 450),
            Self::DoomGuard => (220, 42, 20, 420),

            // Bosses
            Self::BossGoblinKing => (200, 20, 10, 500),
            Self::BossOrcWarlord => (400, 30, 15, 1000),
            Self::BossVampireLord => (600, 40, 20, 2000),
            Self::BossForestGuardian => (800, 45, 25, 3000),
            Self::BossIceDragon => (1200, 55, 30, 5000),
            Self::BossDemonKing => (2000, 70, 40, 10000),

            // Mini-Bosses
            Self::GoblinChampion => (150, 18, 8, 300),
            Self::OrcBerserker => (250, 28, 12, 600),
            Self::VampireElite => (350, 35, 18, 1200),
            Self::AncientWyrm => (500, 40, 22, 1800),
            Self::FrostLord => (700, 48, 28, 3500),
            Self::InfernalLord => (900, 55, 32, 5500),
        }
    }

    /// Returns whether this enemy is a boss
    pub fn is_boss(&self) -> bool {
        matches!(
            self,
            Self::BossGoblinKing
                | Self::BossOrcWarlord
                | Self::BossVampireLord
                | Self::BossForestGuardian
                | Self::BossIceDragon
                | Self::BossDemonKing
                | Self::GoblinChampion
                | Self::OrcBerserker
                | Self::VampireElite
                | Self::AncientWyrm
                | Self::FrostLord
                | Self::InfernalLord
        )
    }

    /// Returns whether this enemy is undead
    pub fn is_undead(&self) -> bool {
        matches!(
            self,
            Self::Skeleton | Self::Zombie | Self::Ghost | Self::Wraith
            | Self::Vampire | Self::Mummy | Self::Lich | Self::BossVampireLord
            | Self::Ghoul | Self::Banshee | Self::DeathKnight | Self::BoneGolem
            | Self::VampireElite | Self::MummyLord | Self::CinderWraith
        )
    }

    /// Returns whether this enemy can poison
    pub fn can_poison(&self) -> bool {
        matches!(self, Self::Spider | Self::GiantSpider | Self::Slime
            | Self::Mushroom | Self::VenomousVine | Self::GiantWasp | Self::IceSpider)
    }

    /// Returns whether this enemy can burn
    pub fn can_burn(&self) -> bool {
        matches!(
            self,
            Self::FireElemental | Self::LavaGolem | Self::Hellhound | Self::FireDrake
            | Self::Demon | Self::DemonLord | Self::Balrog | Self::BossDemonKing
            | Self::MagmaSlime | Self::Salamander | Self::CinderWraith | Self::InfernalImp
            | Self::PitFiend | Self::InfernalLord
        )
    }

    /// Returns whether this enemy can freeze
    pub fn can_freeze(&self) -> bool {
        matches!(
            self,
            Self::IceElemental | Self::FrostGiant | Self::YetiWarrior
            | Self::IceWraith | Self::BossIceDragon | Self::FrostWolf
            | Self::IceSpider | Self::FrozenKnight | Self::Wendigo | Self::FrostLord
        )
    }

    /// Returns whether this enemy can cause bleeding
    pub fn can_bleed(&self) -> bool {
        matches!(
            self,
            Self::Wolf | Self::DireWolf | Self::Skeleton | Self::DeathKnight
            | Self::ShadowAssassin | Self::Vampire | Self::Ghoul
            | Self::BossVampireLord | Self::BossOrcWarlord
        )
    }

    /// Returns the faction this enemy belongs to
    pub fn faction(&self) -> EntityFaction {
        match self {
            // Undead faction
            Self::Skeleton | Self::Zombie | Self::Ghost | Self::Wraith
            | Self::Vampire | Self::Mummy | Self::Ghoul | Self::Banshee
            | Self::DeathKnight | Self::BoneGolem | Self::Lich | Self::MummyLord
            | Self::IceWraith | Self::CinderWraith | Self::BossVampireLord
            | Self::VampireElite => EntityFaction::Undead,

            // Goblinoid faction
            Self::Goblin | Self::Hobgoblin | Self::Kobold | Self::Orc
            | Self::CaveOgre | Self::Troll | Self::ForestTroll
            | Self::BossGoblinKing | Self::GoblinChampion
            | Self::BossOrcWarlord | Self::OrcBerserker => EntityFaction::Goblinoid,

            // Beast faction
            Self::Rat | Self::GiantRat | Self::Bat | Self::Wolf | Self::DireWolf
            | Self::CaveBear | Self::WildBoar | Self::FrostWolf => EntityFaction::Beast,

            // Elemental faction
            Self::RockElemental | Self::IceElemental | Self::FireElemental
            | Self::LavaGolem | Self::MagmaSlime | Self::Golem
            | Self::FrostGiant | Self::CursedStatue | Self::Salamander => EntityFaction::Elemental,

            // Demon faction
            Self::Demon | Self::DemonLord | Self::Succubus | Self::Balrog
            | Self::PitFiend | Self::ShadowDemon | Self::AbyssalHorror | Self::DoomGuard
            | Self::InfernalImp | Self::Hellhound | Self::BossDemonKing | Self::InfernalLord => EntityFaction::Demon,

            // Wild faction (aggressive animals/creatures)
            Self::Spider | Self::GiantSpider | Self::IceSpider | Self::Slime
            | Self::Mushroom | Self::VenomousVine | Self::GiantWasp
            | Self::YetiWarrior | Self::Wendigo | Self::CaveCrawler => EntityFaction::Wild,

            // Default neutral
            _ => EntityFaction::Neutral,
        }
    }

    /// Returns whether this enemy is territorial (will fight others for space)
    pub fn is_territorial(&self) -> bool {
        matches!(
            self,
            Self::Wolf | Self::DireWolf | Self::CaveBear | Self::Spider
            | Self::GiantSpider | Self::Orc | Self::Troll | Self::TreeEnt
            | Self::YetiWarrior | Self::FireDrake | Self::FrostGiant
        )
    }

    /// Returns whether this enemy hunts other creatures
    pub fn is_predator(&self) -> bool {
        matches!(
            self,
            Self::Wolf | Self::DireWolf | Self::Spider | Self::GiantSpider
            | Self::Vampire | Self::Wraith | Self::Hellhound | Self::Wendigo
            | Self::FrostWolf | Self::Ghoul
        )
    }

    /// Returns prey types this enemy will hunt
    pub fn prey_factions(&self) -> Vec<EntityFaction> {
        if self.is_predator() {
            match self.faction() {
                EntityFaction::Beast => vec![EntityFaction::Wild, EntityFaction::Goblinoid],
                EntityFaction::Undead => vec![EntityFaction::Beast, EntityFaction::Goblinoid],
                EntityFaction::Demon => vec![EntityFaction::Beast, EntityFaction::Undead, EntityFaction::Goblinoid],
                _ => vec![],
            }
        } else {
            vec![]
        }
    }

    /// Returns a random enemy for the given dungeon level
    pub fn for_level(level: u32, rng: &mut impl Rng) -> Self {
        let enemies: Vec<Self> = match level {
            1..=4 => vec![Self::Rat, Self::Bat, Self::Spider, Self::Goblin, Self::Skeleton,
                         Self::Kobold, Self::GiantRat, Self::CaveCrawler],
            5..=8 => vec![Self::GiantSpider, Self::Orc, Self::Troll, Self::CaveOgre, Self::Slime,
                         Self::Hobgoblin, Self::CaveBear, Self::Mushroom, Self::RockElemental,
                         Self::GoblinChampion],
            9..=12 => vec![Self::Zombie, Self::Ghost, Self::Wraith, Self::Vampire, Self::Mummy,
                          Self::Ghoul, Self::Banshee, Self::DeathKnight, Self::BoneGolem,
                          Self::OrcBerserker],
            13..=16 => vec![Self::Wolf, Self::DireWolf, Self::TreeEnt, Self::ForestTroll, Self::Druid,
                           Self::WildBoar, Self::GiantWasp, Self::VenomousVine, Self::ForestSpirit,
                           Self::VampireElite],
            17..=20 => vec![Self::IceElemental, Self::FrostGiant, Self::YetiWarrior, Self::IceWraith,
                           Self::FrostWolf, Self::IceSpider, Self::FrozenKnight, Self::Wendigo,
                           Self::AncientWyrm],
            21..=24 => vec![Self::FireElemental, Self::LavaGolem, Self::Hellhound, Self::FireDrake,
                           Self::MagmaSlime, Self::Salamander, Self::CinderWraith, Self::InfernalImp,
                           Self::FrostLord],
            25..=28 => vec![Self::Golem, Self::AncientGuardian, Self::Sphinx, Self::Lich,
                           Self::Gargoyle, Self::MummyLord, Self::CursedStatue, Self::ShadowAssassin,
                           Self::InfernalLord],
            _ => vec![Self::Demon, Self::DemonLord, Self::Succubus, Self::Balrog,
                     Self::PitFiend, Self::ShadowDemon, Self::AbyssalHorror, Self::DoomGuard],
        };
        enemies[rng.gen_range(0..enemies.len())]
    }

    /// Returns the boss for a given level, if any
    pub fn boss_for_level(level: u32) -> Option<Self> {
        match level {
            5 => Some(Self::BossGoblinKing),
            10 => Some(Self::BossOrcWarlord),
            15 => Some(Self::BossVampireLord),
            20 => Some(Self::BossForestGuardian),
            25 => Some(Self::BossIceDragon),
            30 => Some(Self::BossDemonKing),
            _ => None,
        }
    }
}

/// An enemy instance with autonomous AI behavior
#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    /// Unique identifier for this enemy instance
    pub id: u64,
    /// Position X
    pub x: usize,
    /// Position Y
    pub y: usize,
    /// Enemy type
    pub kind: EnemyKind,
    /// Current health
    pub hp: i32,
    /// Maximum health
    pub max_hp: i32,
    /// Attack power
    pub attack: i32,
    /// Defense value
    pub defense: i32,
    /// Experience value when killed
    pub xp_value: u32,
    /// Speed (affects action order)
    pub speed: i32,
    /// Active status effects with duration
    pub status_effects: HashMap<StatusEffect, u32>,
    /// Last known player position
    pub last_seen_player: Option<(usize, usize)>,
    /// Autonomous AI state
    pub ai: EntityAI,
    /// Current target (another enemy ID for inter-enemy combat)
    pub current_target: Option<u64>,
    /// Damage dealt to current target
    pub combat_damage: i32,
    /// Whether this enemy has acted this turn
    pub acted_this_turn: bool,
    /// Number of turns alive
    pub turns_alive: u32,
    /// Spawn room index (for territorial behavior)
    pub spawn_room: Option<usize>,
}

/// Global counter for generating unique enemy IDs
static ENEMY_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// Generate a unique enemy ID
pub fn generate_enemy_id() -> u64 {
    ENEMY_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

impl Enemy {
    /// Create a new enemy with autonomous AI
    pub fn new(x: usize, y: usize, kind: EnemyKind, level: u32) -> Self {
        let (base_hp, base_atk, base_def, base_xp) = kind.base_stats();
        let scale = 1.0 + (level as f32 * 0.1);
        let hp = (base_hp as f32 * scale) as i32;

        // Calculate speed based on enemy type
        let base_speed = match kind {
            // Fast enemies
            EnemyKind::Bat | EnemyKind::Wolf | EnemyKind::DireWolf
            | EnemyKind::GiantWasp | EnemyKind::FrostWolf => 14,
            // Normal speed
            EnemyKind::Goblin | EnemyKind::Skeleton | EnemyKind::Spider
            | EnemyKind::Orc | EnemyKind::Vampire => 10,
            // Slow enemies
            EnemyKind::Troll | EnemyKind::Golem | EnemyKind::LavaGolem
            | EnemyKind::CaveOgre | EnemyKind::BoneGolem => 6,
            // Very slow
            EnemyKind::Slime | EnemyKind::RockElemental | EnemyKind::CursedStatue => 4,
            // Default
            _ => 8,
        };

        let mut ai = EntityAI::for_enemy(kind);
        ai.base_speed = base_speed;
        ai.home_pos = Some((x, y));

        Self {
            id: generate_enemy_id(),
            x,
            y,
            kind,
            hp,
            max_hp: hp,
            attack: (base_atk as f32 * scale) as i32,
            defense: (base_def as f32 * scale) as i32,
            xp_value: (base_xp as f32 * scale) as u32,
            speed: base_speed,
            status_effects: HashMap::new(),
            last_seen_player: None,
            ai,
            current_target: None,
            combat_damage: 0,
            acted_this_turn: false,
            turns_alive: 0,
            spawn_room: None,
        }
    }

    /// Create an enemy with a specific room assignment
    pub fn new_in_room(x: usize, y: usize, kind: EnemyKind, level: u32, room_idx: usize) -> Self {
        let mut enemy = Self::new(x, y, kind, level);
        enemy.spawn_room = Some(room_idx);

        // Set territory for territorial creatures
        if kind.is_territorial() {
            enemy.ai.movement = MovementPattern::new_territory(x, y, 8);
            enemy.ai.behavior = EntityBehavior::Territorial;
        }

        enemy
    }

    /// Update the AI state based on game conditions
    pub fn update_ai(&mut self, can_see_player: bool, player_pos: Option<(usize, usize)>) {
        let player_dist = if let Some((px, py)) = player_pos {
            let dx = self.x as f32 - px as f32;
            let dy = self.y as f32 - py as f32;
            (dx * dx + dy * dy).sqrt()
        } else {
            f32::MAX
        };

        let hp_percent = self.hp as f32 / self.max_hp as f32;

        // Track player position
        if can_see_player {
            if let Some(pos) = player_pos {
                self.last_seen_player = Some(pos);
                self.ai.last_seen_player = Some(pos);
            }
        }

        self.ai.update_behavior(can_see_player, player_dist, hp_percent);
        self.ai.tick_action_points();
        self.ai.decay_interests();
        self.turns_alive += 1;
    }

    /// Decide what action to take autonomously
    pub fn decide_action(&self, enemies: &[Enemy], player_pos: (usize, usize), map_visible: &[[bool; 100]; 45]) -> EntityAction {
        // Check if stunned or frozen
        if self.has_status(StatusEffect::Stun) || self.has_status(StatusEffect::Freeze) {
            return EntityAction::Wait;
        }

        match self.ai.behavior {
            EntityBehavior::Hunt => self.hunt_action(player_pos),
            EntityBehavior::Flee => self.flee_action(player_pos),
            EntityBehavior::Patrol => self.patrol_action(),
            EntityBehavior::Wander => self.wander_action(),
            EntityBehavior::Guard => self.guard_action(player_pos),
            EntityBehavior::Territorial => self.territorial_action(enemies, player_pos),
            EntityBehavior::Combat => self.combat_action(enemies),
            EntityBehavior::Investigate => self.investigate_action(),
            EntityBehavior::Sleep => {
                // Wake up if player nearby
                let dx = (self.x as i32 - player_pos.0 as i32).abs();
                let dy = (self.y as i32 - player_pos.1 as i32).abs();
                if dx <= 3 && dy <= 3 {
                    EntityAction::Wait // Wake up next turn
                } else {
                    EntityAction::Rest
                }
            }
            _ => EntityAction::Wait,
        }
    }

    fn hunt_action(&self, player_pos: (usize, usize)) -> EntityAction {
        let dx = player_pos.0 as i32 - self.x as i32;
        let dy = player_pos.1 as i32 - self.y as i32;

        // If adjacent, attack
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return EntityAction::Attack(player_pos.0, player_pos.1);
        }

        // Move towards player
        EntityAction::Move(dx.signum(), dy.signum())
    }

    fn flee_action(&self, player_pos: (usize, usize)) -> EntityAction {
        let dx = self.x as i32 - player_pos.0 as i32;
        let dy = self.y as i32 - player_pos.1 as i32;

        // Move away from player
        EntityAction::Move(dx.signum(), dy.signum())
    }

    fn patrol_action(&self) -> EntityAction {
        match &self.ai.movement {
            MovementPattern::Patrol { waypoints, current_idx, .. } => {
                if waypoints.is_empty() {
                    return EntityAction::Wait;
                }
                let (tx, ty) = waypoints[*current_idx];
                let dx = tx as i32 - self.x as i32;
                let dy = ty as i32 - self.y as i32;

                if dx == 0 && dy == 0 {
                    EntityAction::PatrolNext
                } else {
                    EntityAction::Move(dx.signum(), dy.signum())
                }
            }
            _ => self.wander_action(),
        }
    }

    fn wander_action(&self) -> EntityAction {
        // Random movement
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..=1);
        let dy = rng.gen_range(-1..=1);
        if dx == 0 && dy == 0 {
            EntityAction::Wait
        } else {
            EntityAction::Move(dx, dy)
        }
    }

    fn guard_action(&self, player_pos: (usize, usize)) -> EntityAction {
        let dx = player_pos.0 as i32 - self.x as i32;
        let dy = player_pos.1 as i32 - self.y as i32;
        let dist = ((dx * dx + dy * dy) as f32).sqrt();

        // Attack if player in range
        if dist <= self.ai.aggro_range as f32 {
            if dx.abs() <= 1 && dy.abs() <= 1 {
                return EntityAction::Attack(player_pos.0, player_pos.1);
            }
            return EntityAction::Move(dx.signum(), dy.signum());
        }

        // Return to guard position
        if let Some((hx, hy)) = self.ai.home_pos {
            if self.x != hx || self.y != hy {
                let dx = hx as i32 - self.x as i32;
                let dy = hy as i32 - self.y as i32;
                return EntityAction::Move(dx.signum(), dy.signum());
            }
        }

        EntityAction::Guard
    }

    fn territorial_action(&self, enemies: &[Enemy], player_pos: (usize, usize)) -> EntityAction {
        // Check for intruding enemies of different faction
        for other in enemies {
            if other.id == self.id || !other.is_alive() {
                continue;
            }

            let other_faction = other.kind.faction();
            let disposition = self.kind.faction().disposition_towards(&other_faction);

            if disposition == EntityDisposition::Hostile {
                let dx = other.x as i32 - self.x as i32;
                let dy = other.y as i32 - self.y as i32;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();

                // Attack intruder if in territory
                if let MovementPattern::Territory { center_x, center_y, radius } = &self.ai.movement {
                    let other_in_territory = {
                        let odx = other.x as i32 - *center_x as i32;
                        let ody = other.y as i32 - *center_y as i32;
                        ((odx * odx + ody * ody) as f32).sqrt() <= *radius as f32
                    };

                    if other_in_territory && dist < 10.0 {
                        if dx.abs() <= 1 && dy.abs() <= 1 {
                            return EntityAction::Attack(other.x, other.y);
                        }
                        return EntityAction::Move(dx.signum(), dy.signum());
                    }
                }
            }
        }

        // Default to guarding
        self.guard_action(player_pos)
    }

    fn combat_action(&self, enemies: &[Enemy]) -> EntityAction {
        if let Some(target_id) = self.current_target {
            if let Some(target) = enemies.iter().find(|e| e.id == target_id && e.is_alive()) {
                let dx = target.x as i32 - self.x as i32;
                let dy = target.y as i32 - self.y as i32;

                if dx.abs() <= 1 && dy.abs() <= 1 {
                    return EntityAction::Attack(target.x, target.y);
                }
                return EntityAction::Move(dx.signum(), dy.signum());
            }
        }
        EntityAction::Wait
    }

    fn investigate_action(&self) -> EntityAction {
        if let Some((ix, iy)) = self.ai.get_top_interest() {
            let dx = ix as i32 - self.x as i32;
            let dy = iy as i32 - self.y as i32;

            if dx == 0 && dy == 0 {
                // Reached the point, remove interest
                return EntityAction::Wait;
            }

            return EntityAction::Move(dx.signum(), dy.signum());
        }
        EntityAction::Wait
    }

    /// Process the enemy attacking another enemy
    pub fn attack_enemy(&mut self, target: &mut Enemy, rng: &mut impl Rng) -> EntityInteractionResult {
        let mut result = EntityInteractionResult::default();

        let damage = (self.attack - target.defense).max(1);
        target.hp -= damage;

        result.damage_dealt = Some((target.id, damage));
        result.messages.push(format!("{} attacks {} for {} damage!",
            self.kind.name(), target.kind.name(), damage));

        // Apply status effects based on attacker type
        if self.kind.can_poison() && rng.gen_bool(0.2) {
            target.add_status(StatusEffect::Poison, 3);
            result.status_applied = Some((target.id, StatusEffect::Poison, 3));
        }
        if self.kind.can_burn() && rng.gen_bool(0.2) {
            target.add_status(StatusEffect::Burn, 3);
            result.status_applied = Some((target.id, StatusEffect::Burn, 3));
        }
        if self.kind.can_freeze() && rng.gen_bool(0.15) {
            target.add_status(StatusEffect::Freeze, 2);
            result.status_applied = Some((target.id, StatusEffect::Freeze, 2));
        }

        // Make combat noise
        result.sound_made = Some((self.x, self.y, 8));

        // Check if target died
        if !target.is_alive() {
            result.entity_died = Some(target.id);
            result.messages.push(format!("{} has been slain!", target.kind.name()));
        }

        result
    }

    /// Advance patrol to next waypoint
    pub fn advance_patrol(&mut self) {
        if let MovementPattern::Patrol { waypoints, current_idx, reverse } = &mut self.ai.movement {
            if waypoints.is_empty() {
                return;
            }

            if *reverse {
                if *current_idx == 0 {
                    *reverse = false;
                    *current_idx = 1.min(waypoints.len() - 1);
                } else {
                    *current_idx -= 1;
                }
            } else {
                if *current_idx >= waypoints.len() - 1 {
                    *reverse = true;
                    *current_idx = waypoints.len().saturating_sub(2);
                } else {
                    *current_idx += 1;
                }
            }
        }
    }

    /// Returns whether the enemy is alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Take damage and return actual damage dealt
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = (amount - self.defense).max(1);
        self.hp -= actual;
        actual
    }

    /// Add a status effect
    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    /// Check if enemy has a status effect
    pub fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    /// Tick all status effects and return damage events
    pub fn tick_status_effects(&mut self) -> Vec<(StatusEffect, i32)> {
        let mut damage_events = Vec::new();
        let mut to_remove = Vec::new();

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => damage_events.push((*effect, 3)),
                StatusEffect::Burn => damage_events.push((*effect, 5)),
                StatusEffect::Bleed => damage_events.push((*effect, 2)),
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                to_remove.push(*effect);
            }
        }

        for effect in to_remove {
            self.status_effects.remove(&effect);
        }

        damage_events
    }
}

/// The player character
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub class: CharacterClass,
    pub hp: i32,
    pub max_hp: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub speed: i32,
    pub gold: u32,
    pub level: u32,
    pub xp: u32,
    pub xp_to_level: u32,
    pub hunger: i32,
    pub max_hunger: i32,
    pub keys: u32,
    pub kills: u32,
    pub status_effects: HashMap<StatusEffect, u32>,
    pub equipment: HashMap<EquipSlot, Item>,
    pub inventory: Vec<Item>,
    pub skills: Vec<Skill>,
    pub active_skill: usize,
    pub minions: Vec<Enemy>,
    /// Recruited companions that fight alongside the player
    pub companions: Vec<Companion>,
    /// Active satiation bonus from eating quality food
    pub satiation_bonus: Option<SatiationBonus>,
    /// Cooking skill level (0-100) - affects cooking quality
    pub cooking_skill: u32,
    /// Meals cooked (for skill progression)
    pub meals_cooked: u32,
    /// Base elemental resistances (from class/level)
    pub base_elemental_resistances: ElementalResistances,
    /// Temporary elemental resistance bonuses (from potions, etc.)
    pub temp_elemental_resistances: HashMap<ElementType, (f32, u32)>,
}

impl Player {
    /// Create a new player
    pub fn new(x: usize, y: usize, class: CharacterClass) -> Self {
        let (hp, attack, defense, mana, speed) = class.base_stats();
        Self {
            x,
            y,
            class,
            hp,
            max_hp: hp,
            mana,
            max_mana: mana,
            base_attack: attack,
            base_defense: defense,
            speed,
            gold: 0,
            level: 1,
            xp: 0,
            xp_to_level: 100,
            hunger: 100,
            max_hunger: 100,
            keys: 0,
            kills: 0,
            status_effects: HashMap::new(),
            equipment: HashMap::new(),
            inventory: Vec::new(),
            skills: Skill::for_class(class),
            active_skill: 0,
            minions: Vec::new(),
            companions: Vec::new(),
            satiation_bonus: None,
            cooking_skill: 0,
            meals_cooked: 0,
            base_elemental_resistances: class.elemental_resistances(),
            temp_elemental_resistances: HashMap::new(),
        }
    }

    /// Get current hunger stage
    pub fn hunger_stage(&self) -> HungerStage {
        HungerStage::from_hunger(self.hunger, self.max_hunger)
    }

    /// Calculate total attack including hunger and satiation effects
    pub fn total_attack(&self) -> i32 {
        let mut total = self.base_attack;
        for item in self.equipment.values() {
            let (atk, _, _, _) = item.stats();
            total += atk;
        }
        if self.has_status(StatusEffect::Strength) {
            total = (total as f32 * 1.5) as i32;
        }
        if self.has_status(StatusEffect::Weakness) {
            total = (total as f32 * 0.5) as i32;
        }

        // Apply hunger stage modifier
        let hunger_stage = self.hunger_stage();
        let hunger_mod = hunger_stage.attack_modifier();
        total = ((total as f32) * (1.0 + hunger_mod as f32 / 100.0)) as i32;

        // Apply satiation bonus
        if let Some(ref bonus) = self.satiation_bonus {
            if bonus.is_active() {
                total += bonus.attack_bonus;
            }
        }

        total.max(1)
    }

    /// Calculate total defense including hunger and satiation effects
    pub fn total_defense(&self) -> i32 {
        let mut total = self.base_defense;
        for item in self.equipment.values() {
            let (_, def, _, _) = item.stats();
            total += def;
        }
        if self.has_status(StatusEffect::Shield) {
            total += 10;
        }

        // Apply hunger stage modifier
        let hunger_stage = self.hunger_stage();
        let hunger_mod = hunger_stage.defense_modifier();
        total = ((total as f32) * (1.0 + hunger_mod as f32 / 100.0)) as i32;

        // Apply satiation bonus
        if let Some(ref bonus) = self.satiation_bonus {
            if bonus.is_active() {
                total += bonus.defense_bonus;
            }
        }

        total.max(0)
    }

    /// Calculate total max HP including satiation bonus
    pub fn total_max_hp(&self) -> i32 {
        let mut total = self.max_hp;
        for item in self.equipment.values() {
            let (_, _, hp, _) = item.stats();
            total += hp;
        }

        // Apply satiation bonus
        if let Some(ref bonus) = self.satiation_bonus {
            if bonus.is_active() {
                total += bonus.max_hp_bonus;
            }
        }

        total
    }

    /// Calculate total max mana
    pub fn total_max_mana(&self) -> i32 {
        let mut total = self.max_mana;
        for item in self.equipment.values() {
            let (_, _, _, mana) = item.stats();
            total += mana;
        }
        total
    }

    /// Gain XP and return true if leveled up
    pub fn gain_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        if self.xp >= self.xp_to_level {
            self.xp -= self.xp_to_level;
            self.level += 1;
            self.xp_to_level = (self.xp_to_level as f32 * 1.4) as u32;
            self.max_hp += 8 + (self.level as i32 / 3);
            self.hp = self.total_max_hp();
            self.max_mana += 5;
            self.mana = self.total_max_mana();
            self.base_attack += 2;
            self.base_defense += 1;
            return true;
        }
        false
    }

    /// Heal the player
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.total_max_hp());
    }

    /// Restore mana
    pub fn restore_mana(&mut self, amount: i32) {
        self.mana = (self.mana + amount).min(self.total_max_mana());
    }

    /// Eat food with quality - returns messages about the meal
    pub fn eat(&mut self, food_value: i32) {
        self.hunger = (self.hunger + food_value).min(self.max_hunger + 20); // Can overfill slightly
    }

    /// Eat food with quality and apply satiation bonus
    pub fn eat_quality_food(&mut self, food_value: i32, quality: FoodQuality) -> Vec<String> {
        let mut messages = Vec::new();
        let old_stage = self.hunger_stage();

        // Calculate actual food value based on quality
        let quality_mult = match quality {
            FoodQuality::Rotten => 0.3,      // Barely fills, might make sick
            FoodQuality::Raw => 0.6,
            FoodQuality::Stale => 0.8,
            FoodQuality::Fresh => 1.0,
            FoodQuality::Cooked => 1.3,
            FoodQuality::WellCooked => 1.5,
            FoodQuality::Gourmet => 2.0,
            FoodQuality::Legendary => 3.0,
        };

        let actual_value = (food_value as f32 * quality_mult) as i32;
        self.hunger = (self.hunger + actual_value).min(self.max_hunger + 20);

        // Apply satiation bonus for good quality food
        if quality >= FoodQuality::Fresh {
            let bonus = SatiationBonus::new(quality, food_value);
            if bonus.turns_remaining > 0 {
                // Stack or replace satiation bonus
                if let Some(ref mut existing) = self.satiation_bonus {
                    // Combine bonuses if new is better
                    if bonus.attack_bonus > existing.attack_bonus {
                        existing.attack_bonus = bonus.attack_bonus;
                    }
                    if bonus.defense_bonus > existing.defense_bonus {
                        existing.defense_bonus = bonus.defense_bonus;
                    }
                    if bonus.max_hp_bonus > existing.max_hp_bonus {
                        existing.max_hp_bonus = bonus.max_hp_bonus;
                    }
                    existing.turns_remaining = existing.turns_remaining.max(bonus.turns_remaining);
                } else {
                    self.satiation_bonus = Some(bonus);
                }

                messages.push(format!("The {} meal gives you strength! (+{} Atk, +{} Def for {} turns)",
                    quality.name(),
                    bonus.attack_bonus,
                    bonus.defense_bonus,
                    bonus.turns_remaining));
            }
        }

        // Negative effects for rotten food
        if quality == FoodQuality::Rotten {
            messages.push("Ugh! The food was rotten!".to_string());
            // 50% chance of food poisoning
            messages.push("You feel sick...".to_string());
        }

        let new_stage = self.hunger_stage();
        if new_stage != old_stage {
            messages.push(format!("You feel {}.", new_stage.name().to_lowercase()));
        }

        messages
    }

    /// Improve cooking skill from cooking
    pub fn improve_cooking(&mut self) -> Option<String> {
        self.meals_cooked += 1;
        let old_skill = self.cooking_skill;

        // Skill improves with practice, diminishing returns
        let improvement = (100 - self.cooking_skill) / 20 + 1;
        self.cooking_skill = (self.cooking_skill + improvement).min(100);

        if self.cooking_skill > old_skill && self.cooking_skill % 10 == 0 {
            Some(format!("Your cooking skill improved to {}!", self.cooking_skill))
        } else {
            None
        }
    }

    /// Get cooking quality based on skill
    pub fn get_cooking_result(&self, base_quality: FoodQuality) -> FoodQuality {
        // Cooking can improve quality up to a limit
        let skill_bonus = self.cooking_skill / 25; // 0-4 quality levels

        let base_level = match base_quality {
            FoodQuality::Rotten => 0,
            FoodQuality::Raw => 1,
            FoodQuality::Stale => 2,
            FoodQuality::Fresh => 3,
            FoodQuality::Cooked => 4,
            FoodQuality::WellCooked => 5,
            FoodQuality::Gourmet => 6,
            FoodQuality::Legendary => 7,
        };

        // Cooking raw food improves it by 2-4 levels based on skill
        let improvement = 2 + skill_bonus as i32;
        let new_level = (base_level + improvement).min(6); // Can't reach Legendary by cooking

        match new_level {
            0 => FoodQuality::Rotten,
            1 => FoodQuality::Raw,
            2 => FoodQuality::Stale,
            3 => FoodQuality::Fresh,
            4 => FoodQuality::Cooked,
            5 => FoodQuality::WellCooked,
            6 => FoodQuality::Gourmet,
            _ => FoodQuality::Legendary,
        }
    }

    /// Add a status effect
    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    /// Check if player has a status effect
    pub fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    /// Remove a status effect
    pub fn remove_status(&mut self, effect: StatusEffect) {
        self.status_effects.remove(&effect);
    }

    /// Tick all status effects and return messages
    pub fn tick_status_effects(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        let mut to_remove = Vec::new();
        let mut damage = 0;
        let mut heal = 0;

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => {
                    damage += 2;
                    messages.push("You take poison damage!".to_string());
                }
                StatusEffect::Burn => {
                    damage += 3;
                    messages.push("You are burning!".to_string());
                }
                StatusEffect::Bleed => {
                    damage += 1;
                    messages.push("You are bleeding!".to_string());
                }
                StatusEffect::Regeneration => {
                    heal += 3;
                }
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                to_remove.push(*effect);
                messages.push(format!("{} wore off.", effect.name()));
            }
        }

        self.hp -= damage;
        self.heal(heal);

        for effect in to_remove {
            self.status_effects.remove(&effect);
        }

        messages
    }

    /// Tick hunger and return messages about hunger state
    pub fn tick_hunger(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        let old_stage = self.hunger_stage();

        // Decay hunger based on current stage
        let decay = old_stage.decay_rate();
        self.hunger = (self.hunger - decay).max(-10); // Can go slightly negative

        let new_stage = self.hunger_stage();

        // Apply starvation damage
        let damage = new_stage.starvation_damage();
        if damage > 0 {
            self.hp -= damage;
            messages.push(format!("You are {}! (-{} HP)", new_stage.name().to_lowercase(), damage));
        }

        // Notify on stage changes
        if new_stage != old_stage {
            match new_stage {
                HungerStage::Stuffed => messages.push("You feel overly full and sluggish.".to_string()),
                HungerStage::Satisfied => messages.push("You feel well-nourished.".to_string()),
                HungerStage::Peckish => messages.push("You could eat something.".to_string()),
                HungerStage::Hungry => messages.push("Your stomach growls. You are hungry.".to_string()),
                HungerStage::VeryHungry => messages.push("You are very hungry! Find food soon!".to_string()),
                HungerStage::Starving => messages.push("You are starving! Your body is weakening!".to_string()),
                HungerStage::Famished => messages.push("You are famished! You will die without food!".to_string()),
            }
        }

        // Tick satiation bonus
        if let Some(ref mut bonus) = self.satiation_bonus {
            if !bonus.tick() {
                messages.push("Your satiation bonus has worn off.".to_string());
            }
        }
        if self.satiation_bonus.as_ref().map_or(false, |b| !b.is_active()) {
            self.satiation_bonus = None;
        }

        messages
    }

    /// Get regeneration bonus from hunger and satiation
    pub fn hunger_regen_bonus(&self) -> i32 {
        let mut bonus = self.hunger_stage().regen_modifier();
        if let Some(ref satiation) = self.satiation_bonus {
            if satiation.is_active() {
                bonus += satiation.regen_bonus;
            }
        }
        bonus
    }

    /// Equip an item and return the previously equipped item
    pub fn equip(&mut self, item: Item) -> Option<Item> {
        if let Some(slot) = item.kind.equip_slot() {
            // Handle rings specially - can wear two
            let actual_slot = if slot == EquipSlot::Ring1 {
                if self.equipment.contains_key(&EquipSlot::Ring1) && !self.equipment.contains_key(&EquipSlot::Ring2) {
                    EquipSlot::Ring2
                } else {
                    EquipSlot::Ring1
                }
            } else {
                slot
            };
            let old = self.equipment.remove(&actual_slot);
            self.equipment.insert(actual_slot, item);
            old
        } else {
            None
        }
    }

    /// Check if player can use their current skill
    pub fn can_use_skill(&self) -> bool {
        if self.skills.is_empty() {
            return false;
        }
        let skill = self.skills[self.active_skill];
        self.mana >= skill.mana_cost()
    }

    /// Get the current active skill
    pub fn current_skill(&self) -> Option<Skill> {
        if self.skills.is_empty() {
            None
        } else {
            Some(self.skills[self.active_skill])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(5, 5, CharacterClass::Warrior);
        assert_eq!(player.x, 5);
        assert_eq!(player.y, 5);
        assert!(player.hp > 0);
    }

    #[test]
    fn test_enemy_damage() {
        let mut enemy = Enemy::new(0, 0, EnemyKind::Rat, 1);
        let initial_hp = enemy.hp;
        let damage = enemy.take_damage(5);
        assert!(damage > 0);
        assert!(enemy.hp < initial_hp);
    }

    #[test]
    fn test_player_xp_gain() {
        let mut player = Player::new(0, 0, CharacterClass::Warrior);
        let initial_level = player.level;
        player.xp = player.xp_to_level - 1;
        let leveled = player.gain_xp(10);
        assert!(leveled);
        assert_eq!(player.level, initial_level + 1);
    }
}

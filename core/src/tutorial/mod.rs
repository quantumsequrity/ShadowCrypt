//! Tutorial System for ShadowCrypt
//!
//! This module provides a comprehensive tutorial and help system including:
//! - Tutorial phases for different game mechanics (movement, combat, inventory, etc.)
//! - Multiple tutorial types (pop-up hints, guided tutorials, contextual tips, practice rooms)
//! - Configurable tutorial settings (skip all, show only new features, reset, difficulty)
//! - Achievement integration for tutorial completion rewards
//! - In-game help system with searchable topics and hotkey reference
//! - New player experience with reduced difficulty and gradual feature unlocking

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of hints to show per session for contextual tips
pub const MAX_CONTEXTUAL_HINTS_PER_SESSION: u32 = 50;

/// Delay in turns before showing the next contextual hint
pub const CONTEXTUAL_HINT_COOLDOWN: u32 = 5;

/// Starting area difficulty multiplier (lower = easier)
pub const STARTING_AREA_DIFFICULTY: f32 = 0.5;

/// Number of floors considered as "starting area" for new players
pub const STARTING_AREA_FLOORS: u32 = 3;

/// Experience bonus multiplier in starting area
pub const STARTING_AREA_XP_BONUS: f32 = 1.25;

// ============================================================================
// Tutorial Phases
// ============================================================================

/// Represents different phases/topics of the tutorial system
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TutorialPhase {
    /// Basic movement using WASD/arrow keys
    BasicMovement,
    /// Combat basics: attacking enemies, using skills
    CombatBasics,
    /// Inventory management: picking up, dropping, organizing items
    InventoryManagement,
    /// Equipment system: equipping gear, understanding stats
    EquipmentAndStats,
    /// Skills and abilities: using class skills, cooldowns, mana
    SkillsAndAbilities,
    /// Exploration: reading the map, using stairs, finding secrets
    Exploration,
    /// NPCs and shops: interacting with merchants, buying/selling
    NPCsAndShops,
    /// Quest system: accepting, tracking, completing quests
    Quests,
    /// Crafting basics: recipes, materials, crafting stations
    CraftingBasics,
    /// Advanced systems: runes, enchantments, companions, etc.
    AdvancedSystems,
}

impl TutorialPhase {
    /// Returns all tutorial phases in recommended order
    pub fn all() -> Vec<TutorialPhase> {
        vec![
            Self::BasicMovement,
            Self::CombatBasics,
            Self::InventoryManagement,
            Self::EquipmentAndStats,
            Self::SkillsAndAbilities,
            Self::Exploration,
            Self::NPCsAndShops,
            Self::Quests,
            Self::CraftingBasics,
            Self::AdvancedSystems,
        ]
    }

    /// Returns the display name of this phase
    pub fn name(&self) -> &'static str {
        match self {
            Self::BasicMovement => "Basic Movement",
            Self::CombatBasics => "Combat Basics",
            Self::InventoryManagement => "Inventory Management",
            Self::EquipmentAndStats => "Equipment & Stats",
            Self::SkillsAndAbilities => "Skills & Abilities",
            Self::Exploration => "Exploration",
            Self::NPCsAndShops => "NPCs & Shops",
            Self::Quests => "Quests",
            Self::CraftingBasics => "Crafting Basics",
            Self::AdvancedSystems => "Advanced Systems",
        }
    }

    /// Returns a brief description of what this phase teaches
    pub fn description(&self) -> &'static str {
        match self {
            Self::BasicMovement => "Learn how to move your character using WASD or arrow keys",
            Self::CombatBasics => "Master the art of combat: attacking enemies and using skills",
            Self::InventoryManagement => "Manage your items: picking up, dropping, and organizing",
            Self::EquipmentAndStats => "Understand equipment slots and how stats affect your character",
            Self::SkillsAndAbilities => "Learn to use class skills, manage cooldowns and mana",
            Self::Exploration => "Navigate the dungeon: reading maps, using stairs, finding secrets",
            Self::NPCsAndShops => "Interact with friendly NPCs, buy and sell at shops",
            Self::Quests => "Accept and complete quests for rewards and progression",
            Self::CraftingBasics => "Craft items using materials and recipes",
            Self::AdvancedSystems => "Master advanced mechanics: runes, enchantments, companions",
        }
    }

    /// Returns the estimated time to complete this tutorial phase (in minutes)
    pub fn estimated_time(&self) -> u32 {
        match self {
            Self::BasicMovement => 2,
            Self::CombatBasics => 5,
            Self::InventoryManagement => 3,
            Self::EquipmentAndStats => 4,
            Self::SkillsAndAbilities => 5,
            Self::Exploration => 4,
            Self::NPCsAndShops => 3,
            Self::Quests => 4,
            Self::CraftingBasics => 5,
            Self::AdvancedSystems => 10,
        }
    }

    /// Returns prerequisites for this phase (phases that should be completed first)
    pub fn prerequisites(&self) -> Vec<TutorialPhase> {
        match self {
            Self::BasicMovement => vec![],
            Self::CombatBasics => vec![Self::BasicMovement],
            Self::InventoryManagement => vec![Self::BasicMovement],
            Self::EquipmentAndStats => vec![Self::InventoryManagement],
            Self::SkillsAndAbilities => vec![Self::CombatBasics],
            Self::Exploration => vec![Self::BasicMovement],
            Self::NPCsAndShops => vec![Self::Exploration, Self::InventoryManagement],
            Self::Quests => vec![Self::NPCsAndShops],
            Self::CraftingBasics => vec![Self::InventoryManagement],
            Self::AdvancedSystems => vec![
                Self::EquipmentAndStats,
                Self::SkillsAndAbilities,
                Self::CraftingBasics,
            ],
        }
    }

    /// Returns the number of steps in this tutorial phase
    pub fn step_count(&self) -> u32 {
        match self {
            Self::BasicMovement => 5,
            Self::CombatBasics => 8,
            Self::InventoryManagement => 6,
            Self::EquipmentAndStats => 7,
            Self::SkillsAndAbilities => 8,
            Self::Exploration => 6,
            Self::NPCsAndShops => 5,
            Self::Quests => 6,
            Self::CraftingBasics => 7,
            Self::AdvancedSystems => 12,
        }
    }
}

// ============================================================================
// Tutorial Types
// ============================================================================

/// Different types of tutorials available
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TutorialType {
    /// Quick pop-up hints that can be dismissed
    PopupHint,
    /// Step-by-step guided tutorials
    GuidedTutorial,
    /// Tips that appear when relevant actions occur
    ContextualTip,
    /// Safe practice areas for learning mechanics
    PracticeRoom,
}

impl TutorialType {
    /// Returns all tutorial types
    pub fn all() -> Vec<TutorialType> {
        vec![
            Self::PopupHint,
            Self::GuidedTutorial,
            Self::ContextualTip,
            Self::PracticeRoom,
        ]
    }

    /// Returns the display name of this tutorial type
    pub fn name(&self) -> &'static str {
        match self {
            Self::PopupHint => "Pop-up Hint",
            Self::GuidedTutorial => "Guided Tutorial",
            Self::ContextualTip => "Contextual Tip",
            Self::PracticeRoom => "Practice Room",
        }
    }

    /// Returns a description of this tutorial type
    pub fn description(&self) -> &'static str {
        match self {
            Self::PopupHint => "Quick, dismissable hints that appear once",
            Self::GuidedTutorial => "Step-by-step instructions with objectives",
            Self::ContextualTip => "Tips that appear when you perform related actions",
            Self::PracticeRoom => "Safe areas where you can practice without danger",
        }
    }
}

// ============================================================================
// Tutorial Difficulty
// ============================================================================

/// Tutorial difficulty levels affecting hint frequency and detail
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub enum TutorialDifficulty {
    /// Minimal hints, only essential information
    Minimal,
    /// Standard hints with moderate frequency
    #[default]
    Standard,
    /// Detailed hints with high frequency for beginners
    Detailed,
    /// Maximum hand-holding with constant guidance
    Comprehensive,
}

impl TutorialDifficulty {
    /// Returns all difficulty levels
    pub fn all() -> Vec<TutorialDifficulty> {
        vec![
            Self::Minimal,
            Self::Standard,
            Self::Detailed,
            Self::Comprehensive,
        ]
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Minimal => "Minimal",
            Self::Standard => "Standard",
            Self::Detailed => "Detailed",
            Self::Comprehensive => "Comprehensive",
        }
    }

    /// Returns the hint frequency multiplier (higher = more frequent)
    pub fn hint_frequency(&self) -> f32 {
        match self {
            Self::Minimal => 0.25,
            Self::Standard => 1.0,
            Self::Detailed => 2.0,
            Self::Comprehensive => 4.0,
        }
    }

    /// Returns whether to show advanced tips at this difficulty
    pub fn show_advanced_tips(&self) -> bool {
        matches!(self, Self::Detailed | Self::Comprehensive)
    }
}

// ============================================================================
// Tutorial Settings
// ============================================================================

/// Configuration options for the tutorial system
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TutorialSettings {
    /// Skip all tutorials entirely
    pub skip_all_tutorials: bool,
    /// Only show tutorials for new/unfamiliar features
    pub show_only_new_features: bool,
    /// Enable pop-up hints
    pub enable_popup_hints: bool,
    /// Enable guided tutorials
    pub enable_guided_tutorials: bool,
    /// Enable contextual tips
    pub enable_contextual_tips: bool,
    /// Enable practice rooms
    pub enable_practice_rooms: bool,
    /// Tutorial difficulty level
    pub difficulty: TutorialDifficulty,
    /// Maximum contextual hints per session
    pub max_contextual_hints: u32,
    /// Cooldown between contextual hints (in turns)
    pub contextual_hint_cooldown: u32,
    /// Show hotkey reminders
    pub show_hotkey_reminders: bool,
    /// Auto-pause during important tutorials
    pub auto_pause_tutorials: bool,
    /// Highlight tutorial-related UI elements
    pub highlight_ui_elements: bool,
    /// Play sound for tutorial notifications
    pub tutorial_sounds: bool,
}

impl Default for TutorialSettings {
    fn default() -> Self {
        Self {
            skip_all_tutorials: false,
            show_only_new_features: false,
            enable_popup_hints: true,
            enable_guided_tutorials: true,
            enable_contextual_tips: true,
            enable_practice_rooms: true,
            difficulty: TutorialDifficulty::Standard,
            max_contextual_hints: MAX_CONTEXTUAL_HINTS_PER_SESSION,
            contextual_hint_cooldown: CONTEXTUAL_HINT_COOLDOWN,
            show_hotkey_reminders: true,
            auto_pause_tutorials: true,
            highlight_ui_elements: true,
            tutorial_sounds: true,
        }
    }
}

impl TutorialSettings {
    /// Creates settings for experienced players (minimal tutorials)
    pub fn experienced() -> Self {
        Self {
            skip_all_tutorials: false,
            show_only_new_features: true,
            enable_popup_hints: false,
            enable_guided_tutorials: false,
            enable_contextual_tips: true,
            enable_practice_rooms: false,
            difficulty: TutorialDifficulty::Minimal,
            max_contextual_hints: 10,
            contextual_hint_cooldown: 20,
            show_hotkey_reminders: false,
            auto_pause_tutorials: false,
            highlight_ui_elements: false,
            tutorial_sounds: false,
        }
    }

    /// Creates settings for complete beginners (maximum guidance)
    pub fn beginner() -> Self {
        Self {
            skip_all_tutorials: false,
            show_only_new_features: false,
            enable_popup_hints: true,
            enable_guided_tutorials: true,
            enable_contextual_tips: true,
            enable_practice_rooms: true,
            difficulty: TutorialDifficulty::Comprehensive,
            max_contextual_hints: 100,
            contextual_hint_cooldown: 3,
            show_hotkey_reminders: true,
            auto_pause_tutorials: true,
            highlight_ui_elements: true,
            tutorial_sounds: true,
        }
    }

    /// Checks if any tutorial type is enabled
    pub fn any_tutorials_enabled(&self) -> bool {
        !self.skip_all_tutorials
            && (self.enable_popup_hints
                || self.enable_guided_tutorials
                || self.enable_contextual_tips
                || self.enable_practice_rooms)
    }

    /// Checks if a specific tutorial type is enabled
    pub fn is_type_enabled(&self, tutorial_type: TutorialType) -> bool {
        if self.skip_all_tutorials {
            return false;
        }
        match tutorial_type {
            TutorialType::PopupHint => self.enable_popup_hints,
            TutorialType::GuidedTutorial => self.enable_guided_tutorials,
            TutorialType::ContextualTip => self.enable_contextual_tips,
            TutorialType::PracticeRoom => self.enable_practice_rooms,
        }
    }
}

// ============================================================================
// Tutorial Step
// ============================================================================

/// Represents a single step in a guided tutorial
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TutorialStep {
    /// Unique identifier for this step
    pub id: u32,
    /// The phase this step belongs to
    pub phase: TutorialPhase,
    /// Title of this step
    pub title: String,
    /// Detailed instruction text
    pub instruction: String,
    /// Optional hint text
    pub hint: Option<String>,
    /// Required action to complete this step
    pub required_action: TutorialAction,
    /// Whether this step has been completed
    pub completed: bool,
    /// Optional UI element to highlight
    pub highlight_element: Option<String>,
    /// Optional position for the tutorial popup
    pub popup_position: PopupPosition,
}

impl TutorialStep {
    /// Creates a new tutorial step
    pub fn new(
        id: u32,
        phase: TutorialPhase,
        title: &str,
        instruction: &str,
        required_action: TutorialAction,
    ) -> Self {
        Self {
            id,
            phase,
            title: title.to_string(),
            instruction: instruction.to_string(),
            hint: None,
            required_action,
            completed: false,
            highlight_element: None,
            popup_position: PopupPosition::Center,
        }
    }

    /// Adds a hint to this step
    pub fn with_hint(mut self, hint: &str) -> Self {
        self.hint = Some(hint.to_string());
        self
    }

    /// Sets the UI element to highlight
    pub fn with_highlight(mut self, element: &str) -> Self {
        self.highlight_element = Some(element.to_string());
        self
    }

    /// Sets the popup position
    pub fn with_position(mut self, position: PopupPosition) -> Self {
        self.popup_position = position;
        self
    }
}

/// Position for tutorial popups
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum PopupPosition {
    TopLeft,
    TopCenter,
    TopRight,
    #[default]
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    NearTarget,
}

/// Actions that can complete tutorial steps
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TutorialAction {
    /// Move in any direction
    Move,
    /// Move in a specific direction
    MoveDirection(Direction),
    /// Attack any enemy
    Attack,
    /// Use any skill
    UseSkill,
    /// Use a specific skill by name
    UseSpecificSkill(String),
    /// Pick up any item
    PickupItem,
    /// Open inventory
    OpenInventory,
    /// Equip any item
    EquipItem,
    /// Unequip any item
    UnequipItem,
    /// Use a consumable item
    UseConsumable,
    /// Open the map
    OpenMap,
    /// Use stairs (up or down)
    UseStairs,
    /// Talk to any NPC
    TalkToNPC,
    /// Buy from a shop
    BuyItem,
    /// Sell to a shop
    SellItem,
    /// Accept a quest
    AcceptQuest,
    /// Complete a quest
    CompleteQuest,
    /// Craft any item
    CraftItem,
    /// Open help menu
    OpenHelp,
    /// Press any key to continue
    AnyKey,
    /// Wait/rest for a turn
    Wait,
    /// Dismiss the popup
    Dismiss,
    /// Custom action with identifier
    Custom(String),
}

/// Cardinal directions for movement tutorials
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    /// Returns the key associated with this direction
    pub fn key_hint(&self) -> &'static str {
        match self {
            Self::North => "W or Up Arrow",
            Self::South => "S or Down Arrow",
            Self::East => "D or Right Arrow",
            Self::West => "A or Left Arrow",
            Self::NorthEast => "E or Numpad 9",
            Self::NorthWest => "Q or Numpad 7",
            Self::SouthEast => "C or Numpad 3",
            Self::SouthWest => "Z or Numpad 1",
        }
    }
}

// ============================================================================
// Tutorial Hint
// ============================================================================

/// A pop-up hint that appears once
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TutorialHint {
    /// Unique identifier
    pub id: String,
    /// The phase this hint relates to
    pub phase: TutorialPhase,
    /// Type of tutorial
    pub tutorial_type: TutorialType,
    /// Title of the hint
    pub title: String,
    /// Main hint text
    pub text: String,
    /// Optional additional tips
    pub tips: Vec<String>,
    /// Trigger condition for contextual hints
    pub trigger: HintTrigger,
    /// Priority (higher = more important)
    pub priority: u32,
    /// Whether this hint has been shown
    pub shown: bool,
    /// Whether the hint was dismissed
    pub dismissed: bool,
}

impl TutorialHint {
    /// Creates a new tutorial hint
    pub fn new(
        id: &str,
        phase: TutorialPhase,
        tutorial_type: TutorialType,
        title: &str,
        text: &str,
        trigger: HintTrigger,
    ) -> Self {
        Self {
            id: id.to_string(),
            phase,
            tutorial_type,
            title: title.to_string(),
            text: text.to_string(),
            tips: Vec::new(),
            trigger,
            priority: 50,
            shown: false,
            dismissed: false,
        }
    }

    /// Adds tips to this hint
    pub fn with_tips(mut self, tips: Vec<&str>) -> Self {
        self.tips = tips.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Sets the priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}

/// Conditions that trigger contextual hints
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum HintTrigger {
    /// Show immediately on game start
    GameStart,
    /// Show when entering a new floor
    NewFloor,
    /// Show when first encountering an enemy
    FirstEnemy,
    /// Show when health drops below percentage
    LowHealth(u32),
    /// Show when mana drops below percentage
    LowMana(u32),
    /// Show when inventory is nearly full
    InventoryFull,
    /// Show when finding first item of type
    FirstItemType(String),
    /// Show when first entering a shop
    FirstShop,
    /// Show when first meeting an NPC
    FirstNPC,
    /// Show when first quest becomes available
    FirstQuest,
    /// Show when first boss is encountered
    FirstBoss,
    /// Show when player dies
    OnDeath,
    /// Show after certain number of turns
    AfterTurns(u32),
    /// Show when specific action is performed
    OnAction(TutorialAction),
    /// Show when leveling up
    OnLevelUp,
    /// Manual trigger only
    Manual,
    /// Custom trigger with identifier
    Custom(String),
}

// ============================================================================
// Practice Room
// ============================================================================

/// Represents a safe practice area
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PracticeRoom {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Description of what can be practiced
    pub description: String,
    /// Which phase this practices
    pub phase: TutorialPhase,
    /// Whether enemies can damage the player
    pub safe_mode: bool,
    /// Whether items are consumed in practice
    pub consume_items: bool,
    /// Whether skills use mana/cooldowns
    pub use_resources: bool,
    /// Special enemies spawned for practice
    pub practice_enemies: Vec<PracticeEnemy>,
    /// Items provided for practice
    pub provided_items: Vec<String>,
    /// Maximum practice time in turns
    pub max_turns: Option<u32>,
    /// Objectives to complete in this room
    pub objectives: Vec<PracticeObjective>,
}

impl PracticeRoom {
    /// Creates a new practice room
    pub fn new(id: &str, name: &str, description: &str, phase: TutorialPhase) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            phase,
            safe_mode: true,
            consume_items: false,
            use_resources: false,
            practice_enemies: Vec::new(),
            provided_items: Vec::new(),
            max_turns: None,
            objectives: Vec::new(),
        }
    }

    /// Sets safe mode
    pub fn with_safe_mode(mut self, safe: bool) -> Self {
        self.safe_mode = safe;
        self
    }

    /// Adds practice enemies
    pub fn with_enemies(mut self, enemies: Vec<PracticeEnemy>) -> Self {
        self.practice_enemies = enemies;
        self
    }

    /// Adds provided items
    pub fn with_items(mut self, items: Vec<&str>) -> Self {
        self.provided_items = items.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Adds objectives
    pub fn with_objectives(mut self, objectives: Vec<PracticeObjective>) -> Self {
        self.objectives = objectives;
        self
    }
}

/// A practice enemy that spawns in practice rooms
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PracticeEnemy {
    /// Enemy type name
    pub enemy_type: String,
    /// Number to spawn
    pub count: u32,
    /// HP multiplier (lower for easier practice)
    pub hp_multiplier: f32,
    /// Damage multiplier (lower for safer practice)
    pub damage_multiplier: f32,
    /// Whether this enemy attacks the player
    pub aggressive: bool,
}

impl PracticeEnemy {
    /// Creates a passive training dummy
    pub fn dummy(enemy_type: &str, count: u32) -> Self {
        Self {
            enemy_type: enemy_type.to_string(),
            count,
            hp_multiplier: 1.0,
            damage_multiplier: 0.0,
            aggressive: false,
        }
    }

    /// Creates a weak practice enemy
    pub fn weak(enemy_type: &str, count: u32) -> Self {
        Self {
            enemy_type: enemy_type.to_string(),
            count,
            hp_multiplier: 0.5,
            damage_multiplier: 0.25,
            aggressive: true,
        }
    }
}

/// An objective to complete in a practice room
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PracticeObjective {
    /// Description of the objective
    pub description: String,
    /// Required action to complete
    pub action: TutorialAction,
    /// Number of times to perform the action
    pub required_count: u32,
    /// Current progress
    pub current_count: u32,
    /// Whether completed
    pub completed: bool,
}

impl PracticeObjective {
    /// Creates a new practice objective
    pub fn new(description: &str, action: TutorialAction, required_count: u32) -> Self {
        Self {
            description: description.to_string(),
            action,
            required_count,
            current_count: 0,
            completed: false,
        }
    }

    /// Updates progress and returns true if newly completed
    pub fn update(&mut self) -> bool {
        if self.completed {
            return false;
        }
        self.current_count += 1;
        if self.current_count >= self.required_count {
            self.completed = true;
            return true;
        }
        false
    }
}

// ============================================================================
// Achievement Tutorials
// ============================================================================

/// Rewards for completing tutorial phases
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TutorialReward {
    /// Type of reward
    pub reward_type: TutorialRewardType,
    /// Quantity if applicable
    pub quantity: u32,
    /// Item name if applicable
    pub item_name: Option<String>,
    /// Description of the reward
    pub description: String,
}

impl TutorialReward {
    /// Creates a gold reward
    pub fn gold(amount: u32) -> Self {
        Self {
            reward_type: TutorialRewardType::Gold,
            quantity: amount,
            item_name: None,
            description: format!("{} gold", amount),
        }
    }

    /// Creates an experience reward
    pub fn experience(amount: u32) -> Self {
        Self {
            reward_type: TutorialRewardType::Experience,
            quantity: amount,
            item_name: None,
            description: format!("{} experience", amount),
        }
    }

    /// Creates an item reward
    pub fn item(name: &str, quantity: u32) -> Self {
        Self {
            reward_type: TutorialRewardType::Item,
            quantity,
            item_name: Some(name.to_string()),
            description: if quantity > 1 {
                format!("{} x{}", name, quantity)
            } else {
                name.to_string()
            },
        }
    }

    /// Creates an achievement unlock
    pub fn achievement(name: &str) -> Self {
        Self {
            reward_type: TutorialRewardType::Achievement,
            quantity: 1,
            item_name: Some(name.to_string()),
            description: format!("Achievement: {}", name),
        }
    }

    /// Creates a stat bonus
    pub fn stat_bonus(stat: &str, amount: u32) -> Self {
        Self {
            reward_type: TutorialRewardType::StatBonus,
            quantity: amount,
            item_name: Some(stat.to_string()),
            description: format!("+{} {}", amount, stat),
        }
    }
}

/// Types of rewards from tutorials
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TutorialRewardType {
    /// Gold currency
    Gold,
    /// Experience points
    Experience,
    /// An item
    Item,
    /// An achievement unlock
    Achievement,
    /// Permanent stat bonus
    StatBonus,
    /// Unlock a feature
    FeatureUnlock,
    /// Cosmetic reward
    Cosmetic,
}

/// Tutorial-related achievements
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TutorialAchievementId {
    /// Completed basic movement tutorial
    FirstSteps,
    /// Completed combat basics tutorial
    CombatReady,
    /// Completed inventory tutorial
    Organizer,
    /// Completed equipment tutorial
    WellEquipped,
    /// Completed skills tutorial
    SkillfulLearner,
    /// Completed exploration tutorial
    Navigator,
    /// Completed NPC tutorial
    SocialButterfly,
    /// Completed quest tutorial
    Questor,
    /// Completed crafting tutorial
    Apprentice,
    /// Completed advanced systems tutorial
    Scholar,
    /// Completed all tutorials
    GraduatedHero,
    /// Completed all tutorials without skipping
    DedicatedStudent,
    /// Completed tutorials in under 30 minutes
    QuickLearner,
}

impl TutorialAchievementId {
    /// Returns all tutorial achievement IDs
    pub fn all() -> Vec<TutorialAchievementId> {
        vec![
            Self::FirstSteps,
            Self::CombatReady,
            Self::Organizer,
            Self::WellEquipped,
            Self::SkillfulLearner,
            Self::Navigator,
            Self::SocialButterfly,
            Self::Questor,
            Self::Apprentice,
            Self::Scholar,
            Self::GraduatedHero,
            Self::DedicatedStudent,
            Self::QuickLearner,
        ]
    }

    /// Returns the name of this achievement
    pub fn name(&self) -> &'static str {
        match self {
            Self::FirstSteps => "First Steps",
            Self::CombatReady => "Combat Ready",
            Self::Organizer => "Organizer",
            Self::WellEquipped => "Well Equipped",
            Self::SkillfulLearner => "Skillful Learner",
            Self::Navigator => "Navigator",
            Self::SocialButterfly => "Social Butterfly",
            Self::Questor => "Questor",
            Self::Apprentice => "Apprentice Crafter",
            Self::Scholar => "Scholar",
            Self::GraduatedHero => "Graduated Hero",
            Self::DedicatedStudent => "Dedicated Student",
            Self::QuickLearner => "Quick Learner",
        }
    }

    /// Returns the description of this achievement
    pub fn description(&self) -> &'static str {
        match self {
            Self::FirstSteps => "Complete the basic movement tutorial",
            Self::CombatReady => "Complete the combat basics tutorial",
            Self::Organizer => "Complete the inventory management tutorial",
            Self::WellEquipped => "Complete the equipment tutorial",
            Self::SkillfulLearner => "Complete the skills and abilities tutorial",
            Self::Navigator => "Complete the exploration tutorial",
            Self::SocialButterfly => "Complete the NPCs and shops tutorial",
            Self::Questor => "Complete the quest system tutorial",
            Self::Apprentice => "Complete the crafting basics tutorial",
            Self::Scholar => "Complete the advanced systems tutorial",
            Self::GraduatedHero => "Complete all tutorial phases",
            Self::DedicatedStudent => "Complete all tutorials without skipping any",
            Self::QuickLearner => "Complete all tutorials in under 30 minutes",
        }
    }

    /// Returns the phase this achievement is tied to (if any)
    pub fn related_phase(&self) -> Option<TutorialPhase> {
        match self {
            Self::FirstSteps => Some(TutorialPhase::BasicMovement),
            Self::CombatReady => Some(TutorialPhase::CombatBasics),
            Self::Organizer => Some(TutorialPhase::InventoryManagement),
            Self::WellEquipped => Some(TutorialPhase::EquipmentAndStats),
            Self::SkillfulLearner => Some(TutorialPhase::SkillsAndAbilities),
            Self::Navigator => Some(TutorialPhase::Exploration),
            Self::SocialButterfly => Some(TutorialPhase::NPCsAndShops),
            Self::Questor => Some(TutorialPhase::Quests),
            Self::Apprentice => Some(TutorialPhase::CraftingBasics),
            Self::Scholar => Some(TutorialPhase::AdvancedSystems),
            _ => None,
        }
    }
}

// ============================================================================
// Help System
// ============================================================================

/// Categories for help topics
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum HelpCategory {
    /// Getting started and basics
    GettingStarted,
    /// Controls and input
    Controls,
    /// Combat mechanics
    Combat,
    /// Items and inventory
    Items,
    /// Character stats and progression
    Character,
    /// World and exploration
    Exploration,
    /// NPCs and dialogue
    NPCs,
    /// Quests and objectives
    Quests,
    /// Crafting and recipes
    Crafting,
    /// Advanced game systems
    Advanced,
    /// Frequently asked questions
    FAQ,
}

impl HelpCategory {
    /// Returns all help categories
    pub fn all() -> Vec<HelpCategory> {
        vec![
            Self::GettingStarted,
            Self::Controls,
            Self::Combat,
            Self::Items,
            Self::Character,
            Self::Exploration,
            Self::NPCs,
            Self::Quests,
            Self::Crafting,
            Self::Advanced,
            Self::FAQ,
        ]
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::GettingStarted => "Getting Started",
            Self::Controls => "Controls",
            Self::Combat => "Combat",
            Self::Items => "Items & Inventory",
            Self::Character => "Character & Stats",
            Self::Exploration => "Exploration",
            Self::NPCs => "NPCs & Dialogue",
            Self::Quests => "Quests",
            Self::Crafting => "Crafting",
            Self::Advanced => "Advanced Systems",
            Self::FAQ => "FAQ",
        }
    }
}

/// A single help topic
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HelpTopic {
    /// Unique identifier
    pub id: String,
    /// Display title
    pub title: String,
    /// Category this topic belongs to
    pub category: HelpCategory,
    /// Main content text
    pub content: String,
    /// Related topic IDs
    pub related_topics: Vec<String>,
    /// Keywords for searching
    pub keywords: Vec<String>,
    /// Whether this is a frequently accessed topic
    pub frequently_accessed: bool,
}

impl HelpTopic {
    /// Creates a new help topic
    pub fn new(id: &str, title: &str, category: HelpCategory, content: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            category,
            content: content.to_string(),
            related_topics: Vec::new(),
            keywords: Vec::new(),
            frequently_accessed: false,
        }
    }

    /// Adds related topics
    pub fn with_related(mut self, topics: Vec<&str>) -> Self {
        self.related_topics = topics.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Adds keywords
    pub fn with_keywords(mut self, keywords: Vec<&str>) -> Self {
        self.keywords = keywords.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Marks as frequently accessed
    pub fn as_frequent(mut self) -> Self {
        self.frequently_accessed = true;
        self
    }

    /// Checks if this topic matches a search query
    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();

        if self.title.to_lowercase().contains(&query_lower) {
            return true;
        }

        if self.content.to_lowercase().contains(&query_lower) {
            return true;
        }

        for keyword in &self.keywords {
            if keyword.to_lowercase().contains(&query_lower) {
                return true;
            }
        }

        false
    }
}

/// Hotkey definition for the reference
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hotkey {
    /// The key or key combination
    pub keys: String,
    /// Description of what it does
    pub description: String,
    /// Category for organization
    pub category: HotkeyCategory,
    /// Whether this is rebindable
    pub rebindable: bool,
}

impl Hotkey {
    /// Creates a new hotkey definition
    pub fn new(keys: &str, description: &str, category: HotkeyCategory) -> Self {
        Self {
            keys: keys.to_string(),
            description: description.to_string(),
            category,
            rebindable: true,
        }
    }

    /// Marks this hotkey as non-rebindable
    pub fn fixed(mut self) -> Self {
        self.rebindable = false;
        self
    }
}

/// Categories for hotkeys
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum HotkeyCategory {
    Movement,
    Combat,
    Inventory,
    Interface,
    Interaction,
    System,
}

impl HotkeyCategory {
    /// Returns all hotkey categories
    pub fn all() -> Vec<HotkeyCategory> {
        vec![
            Self::Movement,
            Self::Combat,
            Self::Inventory,
            Self::Interface,
            Self::Interaction,
            Self::System,
        ]
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Movement => "Movement",
            Self::Combat => "Combat",
            Self::Inventory => "Inventory",
            Self::Interface => "Interface",
            Self::Interaction => "Interaction",
            Self::System => "System",
        }
    }
}

/// The complete help system
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HelpSystem {
    /// All help topics
    pub topics: HashMap<String, HelpTopic>,
    /// All hotkey definitions
    pub hotkeys: Vec<Hotkey>,
    /// Recently viewed topic IDs
    pub recent_topics: Vec<String>,
    /// Maximum recent topics to remember
    pub max_recent: usize,
    /// Current search query
    pub current_search: Option<String>,
    /// Current selected category filter
    pub category_filter: Option<HelpCategory>,
}

impl Default for HelpSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpSystem {
    /// Creates a new help system with default topics and hotkeys
    pub fn new() -> Self {
        let mut system = Self {
            topics: HashMap::new(),
            hotkeys: Vec::new(),
            recent_topics: Vec::new(),
            max_recent: 10,
            current_search: None,
            category_filter: None,
        };

        system.initialize_default_topics();
        system.initialize_default_hotkeys();
        system
    }

    /// Initializes default help topics
    fn initialize_default_topics(&mut self) {
        // Getting Started topics
        self.add_topic(HelpTopic::new(
            "welcome",
            "Welcome to ShadowCrypt",
            HelpCategory::GettingStarted,
            "Welcome, brave adventurer! ShadowCrypt is a challenging roguelike dungeon \
            crawler where you must descend through dangerous floors filled with monsters, \
            traps, and treasures. Your goal is to reach the bottom of the dungeon and \
            defeat the final boss.\n\n\
            Key Features:\n\
            - Permadeath: Death is permanent, but each run teaches you something new\n\
            - Procedural Generation: Every dungeon is unique\n\
            - Character Classes: Choose from multiple classes with unique abilities\n\
            - Deep Combat: Strategic combat with skills, items, and positioning\n\n\
            Good luck, and may fortune favor the bold!"
        ).with_keywords(vec!["start", "begin", "new", "intro"]).as_frequent());

        self.add_topic(HelpTopic::new(
            "first_run",
            "Your First Run",
            HelpCategory::GettingStarted,
            "Starting your first adventure:\n\n\
            1. Choose a character class that matches your playstyle\n\
            2. Use WASD or arrow keys to move around\n\
            3. Bump into enemies to attack them\n\
            4. Press 'i' to open your inventory\n\
            5. Press '?' for help at any time\n\n\
            Tips for new players:\n\
            - Don't be afraid to die - it's part of learning\n\
            - Explore carefully and check for traps\n\
            - Manage your health and resources\n\
            - Use stairs (>) to descend deeper"
        ).with_keywords(vec!["first", "beginner", "new player"]).as_frequent());

        // Controls topics
        self.add_topic(HelpTopic::new(
            "movement",
            "Movement Controls",
            HelpCategory::Controls,
            "Movement:\n\
            W / Up Arrow    - Move north\n\
            S / Down Arrow  - Move south\n\
            A / Left Arrow  - Move west\n\
            D / Right Arrow - Move east\n\n\
            Diagonal Movement (Numpad or):\n\
            Q / Numpad 7 - Northwest\n\
            E / Numpad 9 - Northeast\n\
            Z / Numpad 1 - Southwest\n\
            C / Numpad 3 - Southeast\n\n\
            Other:\n\
            . / Numpad 5 - Wait one turn\n\
            > - Descend stairs\n\
            < - Ascend stairs"
        ).with_keywords(vec!["move", "walk", "wasd", "arrow", "direction"]).as_frequent());

        self.add_topic(HelpTopic::new(
            "combat_keys",
            "Combat Controls",
            HelpCategory::Controls,
            "Combat Controls:\n\n\
            Basic Combat:\n\
            - Move into an enemy to attack\n\
            - Space - Use primary skill\n\
            - 1-9 - Use numbered skill slot\n\n\
            Targeting:\n\
            - Tab - Cycle through visible enemies\n\
            - T - Toggle targeting mode\n\
            - Enter - Confirm target\n\
            - Escape - Cancel targeting"
        ).with_keywords(vec!["fight", "attack", "skill", "target"]));

        // Combat topics
        self.add_topic(HelpTopic::new(
            "combat_basics",
            "Combat Basics",
            HelpCategory::Combat,
            "Combat in ShadowCrypt is turn-based. Each action you take consumes one turn.\n\n\
            Basic Attack:\n\
            Walk into an enemy to perform a basic attack. Damage is calculated based on \
            your Attack stat minus the enemy's Defense.\n\n\
            Skills:\n\
            Each class has unique skills that consume mana. Skills can deal damage, \
            heal, buff, or debuff.\n\n\
            Status Effects:\n\
            Both you and enemies can be affected by status effects like Poison, \
            Burn, Freeze, and more.\n\n\
            Tips:\n\
            - Position yourself to avoid being surrounded\n\
            - Use terrain to funnel enemies\n\
            - Know when to fight and when to flee"
        ).with_keywords(vec!["fight", "battle", "damage", "defense"]).as_frequent());

        // Items topics
        self.add_topic(HelpTopic::new(
            "inventory",
            "Inventory Management",
            HelpCategory::Items,
            "Your inventory holds items you collect:\n\n\
            Opening Inventory: Press 'i'\n\n\
            Actions:\n\
            - Use: Press the item's letter to use/equip it\n\
            - Drop: Press 'd' then the item's letter\n\
            - Examine: Press 'x' then the item's letter\n\n\
            Item Types:\n\
            - Weapons: Increase attack damage\n\
            - Armor: Increase defense\n\
            - Consumables: Potions, scrolls, food\n\
            - Materials: Used for crafting\n\n\
            Capacity:\n\
            Your inventory has limited space. Drop unwanted items or sell them at shops."
        ).with_keywords(vec!["items", "bag", "carry", "hold"]).as_frequent());

        // Add more default topics as needed...
    }

    /// Initializes default hotkey definitions
    fn initialize_default_hotkeys(&mut self) {
        // Movement hotkeys
        self.hotkeys.push(Hotkey::new("W / Up", "Move north", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new("S / Down", "Move south", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new("A / Left", "Move west", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new("D / Right", "Move east", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new("Q", "Move northwest", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new("E", "Move northeast", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new("Z", "Move southwest", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new("C", "Move southeast", HotkeyCategory::Movement));
        self.hotkeys.push(Hotkey::new(". / 5", "Wait one turn", HotkeyCategory::Movement));

        // Combat hotkeys
        self.hotkeys.push(Hotkey::new("Space", "Use primary skill", HotkeyCategory::Combat));
        self.hotkeys.push(Hotkey::new("1-9", "Use skill in slot", HotkeyCategory::Combat));
        self.hotkeys.push(Hotkey::new("Tab", "Cycle targets", HotkeyCategory::Combat));
        self.hotkeys.push(Hotkey::new("T", "Toggle targeting", HotkeyCategory::Combat));

        // Inventory hotkeys
        self.hotkeys.push(Hotkey::new("I", "Open inventory", HotkeyCategory::Inventory));
        self.hotkeys.push(Hotkey::new("G / ,", "Pick up item", HotkeyCategory::Inventory));
        self.hotkeys.push(Hotkey::new("D", "Drop item", HotkeyCategory::Inventory));
        self.hotkeys.push(Hotkey::new("E", "Equip/Unequip", HotkeyCategory::Inventory));
        self.hotkeys.push(Hotkey::new("U", "Use item", HotkeyCategory::Inventory));

        // Interface hotkeys
        self.hotkeys.push(Hotkey::new("M", "Open map", HotkeyCategory::Interface));
        self.hotkeys.push(Hotkey::new("J", "Open quest log", HotkeyCategory::Interface));
        self.hotkeys.push(Hotkey::new("K", "Open skills", HotkeyCategory::Interface));
        self.hotkeys.push(Hotkey::new("L", "Open character sheet", HotkeyCategory::Interface));
        self.hotkeys.push(Hotkey::new("?", "Open help", HotkeyCategory::Interface).fixed());

        // Interaction hotkeys
        self.hotkeys.push(Hotkey::new(">", "Descend stairs", HotkeyCategory::Interaction));
        self.hotkeys.push(Hotkey::new("<", "Ascend stairs", HotkeyCategory::Interaction));
        self.hotkeys.push(Hotkey::new("Enter", "Interact/Confirm", HotkeyCategory::Interaction));
        self.hotkeys.push(Hotkey::new("O", "Open door", HotkeyCategory::Interaction));

        // System hotkeys
        self.hotkeys.push(Hotkey::new("Escape", "Cancel/Menu", HotkeyCategory::System).fixed());
        self.hotkeys.push(Hotkey::new("F5", "Quick save", HotkeyCategory::System));
        self.hotkeys.push(Hotkey::new("F9", "Quick load", HotkeyCategory::System));
        self.hotkeys.push(Hotkey::new("F1", "Toggle tutorials", HotkeyCategory::System));
    }

    /// Adds a help topic
    pub fn add_topic(&mut self, topic: HelpTopic) {
        self.topics.insert(topic.id.clone(), topic);
    }

    /// Gets a topic by ID
    pub fn get_topic(&self, id: &str) -> Option<&HelpTopic> {
        self.topics.get(id)
    }

    /// Gets a topic by ID and adds it to recent
    pub fn view_topic(&mut self, id: &str) -> Option<&HelpTopic> {
        if self.topics.contains_key(id) {
            // Add to recent, removing if already present
            self.recent_topics.retain(|t| t != id);
            self.recent_topics.insert(0, id.to_string());
            if self.recent_topics.len() > self.max_recent {
                self.recent_topics.pop();
            }
        }
        self.topics.get(id)
    }

    /// Searches topics by query
    pub fn search(&self, query: &str) -> Vec<&HelpTopic> {
        self.topics
            .values()
            .filter(|topic| topic.matches_search(query))
            .collect()
    }

    /// Gets topics by category
    pub fn get_by_category(&self, category: HelpCategory) -> Vec<&HelpTopic> {
        self.topics
            .values()
            .filter(|topic| topic.category == category)
            .collect()
    }

    /// Gets frequently accessed topics
    pub fn get_frequent_topics(&self) -> Vec<&HelpTopic> {
        self.topics
            .values()
            .filter(|topic| topic.frequently_accessed)
            .collect()
    }

    /// Gets recent topics
    pub fn get_recent_topics(&self) -> Vec<&HelpTopic> {
        self.recent_topics
            .iter()
            .filter_map(|id| self.topics.get(id))
            .collect()
    }

    /// Gets hotkeys by category
    pub fn get_hotkeys_by_category(&self, category: HotkeyCategory) -> Vec<&Hotkey> {
        self.hotkeys
            .iter()
            .filter(|h| h.category == category)
            .collect()
    }
}

// ============================================================================
// New Player Experience
// ============================================================================

/// Configuration for the new player experience
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPlayerExperience {
    /// Whether new player experience is enabled
    pub enabled: bool,
    /// Current state of the new player journey
    pub state: NewPlayerState,
    /// Features that have been unlocked/introduced
    pub unlocked_features: HashSet<GameFeature>,
    /// The guided first quest ID (if active)
    pub first_quest_id: Option<u32>,
    /// Progress in the first quest
    pub first_quest_step: u32,
    /// Turns played in new player mode
    pub turns_played: u32,
    /// Floors completed in new player mode
    pub floors_completed: u32,
    /// Whether the player has opted out of NPE
    pub opted_out: bool,
    /// Difficulty modifiers active in starting area
    pub difficulty_modifiers: StartingAreaModifiers,
}

impl Default for NewPlayerExperience {
    fn default() -> Self {
        Self {
            enabled: true,
            state: NewPlayerState::Introduction,
            unlocked_features: HashSet::new(),
            first_quest_id: None,
            first_quest_step: 0,
            turns_played: 0,
            floors_completed: 0,
            opted_out: false,
            difficulty_modifiers: StartingAreaModifiers::default(),
        }
    }
}

impl NewPlayerExperience {
    /// Creates a new player experience for an experienced player (disabled)
    pub fn experienced() -> Self {
        let mut npe = Self::default();
        npe.enabled = false;
        npe.opted_out = true;
        npe.state = NewPlayerState::Completed;
        // Unlock all features
        for feature in GameFeature::all() {
            npe.unlocked_features.insert(feature);
        }
        npe
    }

    /// Checks if a feature is unlocked
    pub fn is_feature_unlocked(&self, feature: GameFeature) -> bool {
        !self.enabled || self.unlocked_features.contains(&feature)
    }

    /// Unlocks a feature
    pub fn unlock_feature(&mut self, feature: GameFeature) -> bool {
        if !self.unlocked_features.contains(&feature) {
            self.unlocked_features.insert(feature);
            return true;
        }
        false
    }

    /// Advances the new player state
    pub fn advance_state(&mut self) {
        self.state = match self.state {
            NewPlayerState::Introduction => NewPlayerState::BasicControls,
            NewPlayerState::BasicControls => NewPlayerState::FirstCombat,
            NewPlayerState::FirstCombat => NewPlayerState::FirstItems,
            NewPlayerState::FirstItems => NewPlayerState::FirstFloorComplete,
            NewPlayerState::FirstFloorComplete => NewPlayerState::ExpandingWorld,
            NewPlayerState::ExpandingWorld => NewPlayerState::Completed,
            NewPlayerState::Completed => NewPlayerState::Completed,
        };
    }

    /// Checks if currently in starting area
    pub fn in_starting_area(&self) -> bool {
        self.enabled && self.floors_completed < STARTING_AREA_FLOORS as u32
    }

    /// Gets the current difficulty multiplier
    pub fn get_difficulty_multiplier(&self) -> f32 {
        if self.in_starting_area() {
            STARTING_AREA_DIFFICULTY
        } else {
            1.0
        }
    }

    /// Gets the current XP bonus multiplier
    pub fn get_xp_bonus(&self) -> f32 {
        if self.in_starting_area() {
            STARTING_AREA_XP_BONUS
        } else {
            1.0
        }
    }

    /// Updates turn counter
    pub fn record_turn(&mut self) {
        self.turns_played += 1;
    }

    /// Records floor completion
    pub fn record_floor_complete(&mut self) {
        self.floors_completed += 1;

        // Unlock features based on floors completed
        match self.floors_completed {
            1 => {
                self.unlock_feature(GameFeature::BasicCombat);
                self.unlock_feature(GameFeature::ItemPickup);
            }
            2 => {
                self.unlock_feature(GameFeature::Equipment);
                self.unlock_feature(GameFeature::Skills);
            }
            3 => {
                self.unlock_feature(GameFeature::NPCs);
                self.unlock_feature(GameFeature::Shops);
            }
            4 => {
                self.unlock_feature(GameFeature::Quests);
            }
            5 => {
                self.unlock_feature(GameFeature::Crafting);
            }
            _ => {
                // Unlock remaining features
                for feature in GameFeature::all() {
                    self.unlock_feature(feature);
                }
            }
        }
    }

    /// Opts out of new player experience
    pub fn opt_out(&mut self) {
        self.opted_out = true;
        self.enabled = false;
        for feature in GameFeature::all() {
            self.unlock_feature(feature);
        }
    }
}

/// States in the new player journey
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum NewPlayerState {
    /// Initial introduction sequence
    #[default]
    Introduction,
    /// Learning basic movement and controls
    BasicControls,
    /// First combat encounter
    FirstCombat,
    /// Picking up first items
    FirstItems,
    /// Completing the first floor
    FirstFloorComplete,
    /// Expanding into more game systems
    ExpandingWorld,
    /// New player experience completed
    Completed,
}

impl NewPlayerState {
    /// Returns a description of the current state
    pub fn description(&self) -> &'static str {
        match self {
            Self::Introduction => "Welcome to ShadowCrypt! Let's start your adventure.",
            Self::BasicControls => "Learn how to move and explore the dungeon.",
            Self::FirstCombat => "Defeat your first enemy!",
            Self::FirstItems => "Pick up items to help you survive.",
            Self::FirstFloorComplete => "Find the stairs and descend deeper.",
            Self::ExpandingWorld => "Discover new features and mechanics.",
            Self::Completed => "You've mastered the basics. Good luck!",
        }
    }
}

/// Game features that can be locked/unlocked for new players
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum GameFeature {
    BasicMovement,
    BasicCombat,
    ItemPickup,
    Inventory,
    Equipment,
    Skills,
    NPCs,
    Shops,
    Quests,
    Crafting,
    Runes,
    Enchantments,
    Companions,
    Guilds,
    Arena,
    Trading,
}

impl GameFeature {
    /// Returns all game features
    pub fn all() -> Vec<GameFeature> {
        vec![
            Self::BasicMovement,
            Self::BasicCombat,
            Self::ItemPickup,
            Self::Inventory,
            Self::Equipment,
            Self::Skills,
            Self::NPCs,
            Self::Shops,
            Self::Quests,
            Self::Crafting,
            Self::Runes,
            Self::Enchantments,
            Self::Companions,
            Self::Guilds,
            Self::Arena,
            Self::Trading,
        ]
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::BasicMovement => "Basic Movement",
            Self::BasicCombat => "Combat",
            Self::ItemPickup => "Item Pickup",
            Self::Inventory => "Inventory",
            Self::Equipment => "Equipment",
            Self::Skills => "Skills",
            Self::NPCs => "NPCs",
            Self::Shops => "Shops",
            Self::Quests => "Quests",
            Self::Crafting => "Crafting",
            Self::Runes => "Runes",
            Self::Enchantments => "Enchantments",
            Self::Companions => "Companions",
            Self::Guilds => "Guilds",
            Self::Arena => "Arena",
            Self::Trading => "Trading",
        }
    }
}

/// Difficulty modifiers for the starting area
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StartingAreaModifiers {
    /// Enemy damage multiplier
    pub enemy_damage_mult: f32,
    /// Enemy health multiplier
    pub enemy_health_mult: f32,
    /// Enemy spawn rate multiplier
    pub spawn_rate_mult: f32,
    /// Trap damage multiplier
    pub trap_damage_mult: f32,
    /// Item drop rate multiplier
    pub item_drop_mult: f32,
    /// Gold drop rate multiplier
    pub gold_drop_mult: f32,
    /// Experience gained multiplier
    pub xp_mult: f32,
    /// Healing effectiveness multiplier
    pub healing_mult: f32,
}

impl Default for StartingAreaModifiers {
    fn default() -> Self {
        Self {
            enemy_damage_mult: 0.5,
            enemy_health_mult: 0.75,
            spawn_rate_mult: 0.75,
            trap_damage_mult: 0.5,
            item_drop_mult: 1.25,
            gold_drop_mult: 1.25,
            xp_mult: 1.25,
            healing_mult: 1.5,
        }
    }
}

// ============================================================================
// Tutorial Progress
// ============================================================================

/// Tracks progress for a single tutorial phase
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TutorialProgress {
    /// The phase this tracks
    pub phase: TutorialPhase,
    /// Whether this phase is completed
    pub completed: bool,
    /// Whether this phase was skipped
    pub skipped: bool,
    /// Current step in the tutorial (for guided tutorials)
    pub current_step: u32,
    /// Time spent on this tutorial (in seconds)
    pub time_spent: u32,
    /// Timestamp when started
    pub started_at: Option<u64>,
    /// Timestamp when completed
    pub completed_at: Option<u64>,
    /// Rewards claimed for this phase
    pub rewards_claimed: bool,
}

impl TutorialProgress {
    /// Creates new progress for a phase
    pub fn new(phase: TutorialPhase) -> Self {
        Self {
            phase,
            completed: false,
            skipped: false,
            current_step: 0,
            time_spent: 0,
            started_at: None,
            completed_at: None,
            rewards_claimed: false,
        }
    }

    /// Checks if this phase is in progress
    pub fn is_in_progress(&self) -> bool {
        self.started_at.is_some() && !self.completed && !self.skipped
    }

    /// Gets completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.completed {
            return 100.0;
        }
        let total_steps = self.phase.step_count();
        if total_steps == 0 {
            return 0.0;
        }
        (self.current_step as f32 / total_steps as f32) * 100.0
    }
}

// ============================================================================
// Main Tutorial System
// ============================================================================

/// The main tutorial system managing all tutorial functionality
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TutorialSystem {
    /// Tutorial settings
    pub settings: TutorialSettings,
    /// Progress for each tutorial phase
    pub progress: HashMap<TutorialPhase, TutorialProgress>,
    /// All tutorial hints
    pub hints: HashMap<String, TutorialHint>,
    /// Currently active guided tutorial steps
    pub active_steps: Vec<TutorialStep>,
    /// Current step index in active tutorial
    pub current_step_index: usize,
    /// Practice rooms available
    pub practice_rooms: HashMap<String, PracticeRoom>,
    /// Currently active practice room ID
    pub active_practice_room: Option<String>,
    /// The help system
    pub help: HelpSystem,
    /// New player experience
    pub new_player: NewPlayerExperience,
    /// Pending notifications/popups to show
    pub pending_popups: Vec<TutorialPopup>,
    /// Contextual hints shown this session
    pub contextual_hints_shown: u32,
    /// Turns since last contextual hint
    pub turns_since_hint: u32,
    /// Total time spent in tutorials (seconds)
    pub total_tutorial_time: u32,
    /// Achievements unlocked from tutorials
    pub unlocked_achievements: HashSet<TutorialAchievementId>,
    /// Rewards earned from tutorials
    pub earned_rewards: Vec<TutorialReward>,
    /// Whether the tutorial system is paused
    pub paused: bool,
}

impl Default for TutorialSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TutorialSystem {
    /// Creates a new tutorial system
    pub fn new() -> Self {
        let mut system = Self {
            settings: TutorialSettings::default(),
            progress: HashMap::new(),
            hints: HashMap::new(),
            active_steps: Vec::new(),
            current_step_index: 0,
            practice_rooms: HashMap::new(),
            active_practice_room: None,
            help: HelpSystem::new(),
            new_player: NewPlayerExperience::default(),
            pending_popups: Vec::new(),
            contextual_hints_shown: 0,
            turns_since_hint: 0,
            total_tutorial_time: 0,
            unlocked_achievements: HashSet::new(),
            earned_rewards: Vec::new(),
            paused: false,
        };

        // Initialize progress for all phases
        for phase in TutorialPhase::all() {
            system.progress.insert(phase, TutorialProgress::new(phase));
        }

        // Initialize default hints
        system.initialize_default_hints();

        // Initialize default practice rooms
        system.initialize_default_practice_rooms();

        system
    }

    /// Creates a tutorial system for experienced players
    pub fn for_experienced_player() -> Self {
        let mut system = Self::new();
        system.settings = TutorialSettings::experienced();
        system.new_player = NewPlayerExperience::experienced();

        // Mark all tutorials as completed
        for progress in system.progress.values_mut() {
            progress.completed = true;
            progress.skipped = true;
        }

        system
    }

    /// Initializes default tutorial hints
    fn initialize_default_hints(&mut self) {
        // Movement hints
        self.add_hint(TutorialHint::new(
            "hint_movement_start",
            TutorialPhase::BasicMovement,
            TutorialType::PopupHint,
            "Welcome!",
            "Use WASD or Arrow keys to move around the dungeon.",
            HintTrigger::GameStart,
        ).with_priority(100));

        self.add_hint(TutorialHint::new(
            "hint_first_enemy",
            TutorialPhase::CombatBasics,
            TutorialType::ContextualTip,
            "Enemy Spotted!",
            "Walk into enemies to attack them. Be careful - they hit back!",
            HintTrigger::FirstEnemy,
        ).with_priority(90));

        self.add_hint(TutorialHint::new(
            "hint_low_health",
            TutorialPhase::CombatBasics,
            TutorialType::ContextualTip,
            "Low Health Warning",
            "Your health is low! Use a health potion (press 'i' for inventory) or retreat.",
            HintTrigger::LowHealth(25),
        ).with_priority(95));

        self.add_hint(TutorialHint::new(
            "hint_inventory_full",
            TutorialPhase::InventoryManagement,
            TutorialType::ContextualTip,
            "Inventory Full",
            "Your inventory is full. Press 'd' to drop items or sell them at a shop.",
            HintTrigger::InventoryFull,
        ).with_priority(70));

        self.add_hint(TutorialHint::new(
            "hint_first_shop",
            TutorialPhase::NPCsAndShops,
            TutorialType::PopupHint,
            "Shop Found!",
            "You've found a shop! Press Enter to interact and browse their wares.",
            HintTrigger::FirstShop,
        ).with_priority(80));

        self.add_hint(TutorialHint::new(
            "hint_level_up",
            TutorialPhase::SkillsAndAbilities,
            TutorialType::PopupHint,
            "Level Up!",
            "Congratulations! You've leveled up. Check your skills (K) for new abilities.",
            HintTrigger::OnLevelUp,
        ).with_priority(85));

        self.add_hint(TutorialHint::new(
            "hint_death",
            TutorialPhase::CombatBasics,
            TutorialType::PopupHint,
            "You Died",
            "Death is a learning experience. Each run teaches you something new. Try again!",
            HintTrigger::OnDeath,
        ).with_tips(vec![
            "Watch your health and retreat when needed",
            "Use potions and skills strategically",
            "Sometimes the best fight is the one you avoid",
        ]).with_priority(100));
    }

    /// Initializes default practice rooms
    fn initialize_default_practice_rooms(&mut self) {
        // Combat practice room
        self.add_practice_room(
            PracticeRoom::new(
                "combat_basics",
                "Combat Training Arena",
                "Practice basic combat against training dummies",
                TutorialPhase::CombatBasics,
            )
            .with_enemies(vec![
                PracticeEnemy::dummy("Training Dummy", 3),
                PracticeEnemy::weak("Goblin", 2),
            ])
            .with_items(vec!["Health Potion", "Mana Potion"])
            .with_objectives(vec![
                PracticeObjective::new("Attack an enemy", TutorialAction::Attack, 5),
                PracticeObjective::new("Use a skill", TutorialAction::UseSkill, 3),
            ])
        );

        // Inventory practice room
        self.add_practice_room(
            PracticeRoom::new(
                "inventory_basics",
                "Inventory Training",
                "Practice managing your inventory",
                TutorialPhase::InventoryManagement,
            )
            .with_items(vec![
                "Health Potion",
                "Health Potion",
                "Iron Sword",
                "Leather Armor",
                "Gold Coin",
                "Apple",
            ])
            .with_objectives(vec![
                PracticeObjective::new("Pick up an item", TutorialAction::PickupItem, 3),
                PracticeObjective::new("Open inventory", TutorialAction::OpenInventory, 1),
                PracticeObjective::new("Equip an item", TutorialAction::EquipItem, 1),
            ])
        );
    }

    /// Adds a hint to the system
    pub fn add_hint(&mut self, hint: TutorialHint) {
        self.hints.insert(hint.id.clone(), hint);
    }

    /// Adds a practice room to the system
    pub fn add_practice_room(&mut self, room: PracticeRoom) {
        self.practice_rooms.insert(room.id.clone(), room);
    }

    /// Resets all tutorial progress
    pub fn reset_all(&mut self) {
        for progress in self.progress.values_mut() {
            *progress = TutorialProgress::new(progress.phase);
        }

        for hint in self.hints.values_mut() {
            hint.shown = false;
            hint.dismissed = false;
        }

        self.active_steps.clear();
        self.current_step_index = 0;
        self.active_practice_room = None;
        self.pending_popups.clear();
        self.contextual_hints_shown = 0;
        self.turns_since_hint = 0;
        self.total_tutorial_time = 0;
        self.unlocked_achievements.clear();
        self.earned_rewards.clear();
        self.new_player = NewPlayerExperience::default();
    }

    /// Resets a specific tutorial phase
    pub fn reset_phase(&mut self, phase: TutorialPhase) {
        if let Some(progress) = self.progress.get_mut(&phase) {
            *progress = TutorialProgress::new(phase);
        }

        // Reset related hints
        for hint in self.hints.values_mut() {
            if hint.phase == phase {
                hint.shown = false;
                hint.dismissed = false;
            }
        }
    }

    /// Starts a guided tutorial for a phase
    pub fn start_guided_tutorial(&mut self, phase: TutorialPhase) {
        if !self.settings.enable_guided_tutorials {
            return;
        }

        // Generate steps for this phase
        self.active_steps = self.generate_steps_for_phase(phase);
        self.current_step_index = 0;

        // Update progress
        if let Some(progress) = self.progress.get_mut(&phase) {
            progress.started_at = Some(self.current_timestamp());
            progress.current_step = 0;
        }
    }

    /// Generates tutorial steps for a phase
    fn generate_steps_for_phase(&self, phase: TutorialPhase) -> Vec<TutorialStep> {
        match phase {
            TutorialPhase::BasicMovement => vec![
                TutorialStep::new(
                    1, phase,
                    "Moving Around",
                    "Use W, A, S, D or the Arrow keys to move your character.",
                    TutorialAction::Move,
                ).with_hint("Try moving in different directions"),
                TutorialStep::new(
                    2, phase,
                    "Diagonal Movement",
                    "Use Q, E, Z, C or numpad for diagonal movement.",
                    TutorialAction::Move,
                ),
                TutorialStep::new(
                    3, phase,
                    "Waiting",
                    "Press '.' or Numpad 5 to wait in place for one turn.",
                    TutorialAction::Wait,
                ),
                TutorialStep::new(
                    4, phase,
                    "Exploring",
                    "Move around to explore and reveal the map.",
                    TutorialAction::Move,
                ),
                TutorialStep::new(
                    5, phase,
                    "Movement Complete!",
                    "Great job! You've mastered basic movement.",
                    TutorialAction::Dismiss,
                ),
            ],
            TutorialPhase::CombatBasics => vec![
                TutorialStep::new(
                    1, phase,
                    "Finding Enemies",
                    "Look for enemies marked in red on the map.",
                    TutorialAction::Move,
                ),
                TutorialStep::new(
                    2, phase,
                    "Basic Attack",
                    "Walk into an enemy to perform a basic melee attack.",
                    TutorialAction::Attack,
                ).with_hint("Your damage is based on your Attack stat"),
                TutorialStep::new(
                    3, phase,
                    "Combat Continued",
                    "Continue attacking until the enemy is defeated.",
                    TutorialAction::Attack,
                ),
                TutorialStep::new(
                    4, phase,
                    "Using Skills",
                    "Press Space to use your primary skill (if you have mana).",
                    TutorialAction::UseSkill,
                ).with_hint("Skills are more powerful but cost mana"),
                TutorialStep::new(
                    5, phase,
                    "Skill Slots",
                    "Use number keys 1-9 to activate skills in those slots.",
                    TutorialAction::AnyKey,
                ),
                TutorialStep::new(
                    6, phase,
                    "Targeting",
                    "Press Tab to cycle through visible enemies for targeting.",
                    TutorialAction::AnyKey,
                ),
                TutorialStep::new(
                    7, phase,
                    "Retreat When Needed",
                    "If your health is low, retreat and heal!",
                    TutorialAction::AnyKey,
                ),
                TutorialStep::new(
                    8, phase,
                    "Combat Complete!",
                    "Excellent! You now know the basics of combat.",
                    TutorialAction::Dismiss,
                ),
            ],
            // Add more phases as needed...
            _ => vec![
                TutorialStep::new(
                    1, phase,
                    phase.name(),
                    phase.description(),
                    TutorialAction::AnyKey,
                ),
                TutorialStep::new(
                    2, phase,
                    "Tutorial Complete",
                    "You've completed this tutorial section.",
                    TutorialAction::Dismiss,
                ),
            ],
        }
    }

    /// Advances to the next step in the active tutorial
    pub fn advance_step(&mut self) -> bool {
        if self.active_steps.is_empty() {
            return false;
        }

        // Mark current step as completed
        if let Some(step) = self.active_steps.get_mut(self.current_step_index) {
            step.completed = true;
        }

        self.current_step_index += 1;

        // Update progress
        if let Some(step) = self.active_steps.get(self.current_step_index.saturating_sub(1)) {
            if let Some(progress) = self.progress.get_mut(&step.phase) {
                progress.current_step = self.current_step_index as u32;
            }
        }

        // Check if tutorial is complete
        if self.current_step_index >= self.active_steps.len() {
            self.complete_active_tutorial();
            return true;
        }

        false
    }

    /// Completes the currently active tutorial
    fn complete_active_tutorial(&mut self) {
        if let Some(step) = self.active_steps.first() {
            let phase = step.phase;

            if let Some(progress) = self.progress.get_mut(&phase) {
                progress.completed = true;
                progress.completed_at = Some(self.current_timestamp());
            }

            // Award achievement
            if let Some(achievement) = self.get_phase_achievement(phase) {
                if !self.unlocked_achievements.contains(&achievement) {
                    self.unlocked_achievements.insert(achievement);
                    self.pending_popups.push(TutorialPopup::achievement(
                        achievement.name(),
                        achievement.description(),
                    ));
                }
            }

            // Award rewards
            self.award_phase_rewards(phase);

            // Check for all-complete achievement
            self.check_all_complete();
        }

        self.active_steps.clear();
        self.current_step_index = 0;
    }

    /// Gets the achievement for completing a phase
    fn get_phase_achievement(&self, phase: TutorialPhase) -> Option<TutorialAchievementId> {
        match phase {
            TutorialPhase::BasicMovement => Some(TutorialAchievementId::FirstSteps),
            TutorialPhase::CombatBasics => Some(TutorialAchievementId::CombatReady),
            TutorialPhase::InventoryManagement => Some(TutorialAchievementId::Organizer),
            TutorialPhase::EquipmentAndStats => Some(TutorialAchievementId::WellEquipped),
            TutorialPhase::SkillsAndAbilities => Some(TutorialAchievementId::SkillfulLearner),
            TutorialPhase::Exploration => Some(TutorialAchievementId::Navigator),
            TutorialPhase::NPCsAndShops => Some(TutorialAchievementId::SocialButterfly),
            TutorialPhase::Quests => Some(TutorialAchievementId::Questor),
            TutorialPhase::CraftingBasics => Some(TutorialAchievementId::Apprentice),
            TutorialPhase::AdvancedSystems => Some(TutorialAchievementId::Scholar),
        }
    }

    /// Awards rewards for completing a phase
    fn award_phase_rewards(&mut self, phase: TutorialPhase) {
        let rewards = match phase {
            TutorialPhase::BasicMovement => vec![
                TutorialReward::gold(50),
            ],
            TutorialPhase::CombatBasics => vec![
                TutorialReward::experience(100),
                TutorialReward::item("Minor Health Potion", 3),
            ],
            TutorialPhase::InventoryManagement => vec![
                TutorialReward::item("Bag of Holding", 1),
            ],
            TutorialPhase::EquipmentAndStats => vec![
                TutorialReward::item("Adventurer's Ring", 1),
            ],
            TutorialPhase::SkillsAndAbilities => vec![
                TutorialReward::experience(150),
                TutorialReward::item("Mana Potion", 3),
            ],
            TutorialPhase::Exploration => vec![
                TutorialReward::item("Map Fragment", 1),
                TutorialReward::gold(100),
            ],
            TutorialPhase::NPCsAndShops => vec![
                TutorialReward::gold(200),
            ],
            TutorialPhase::Quests => vec![
                TutorialReward::experience(200),
            ],
            TutorialPhase::CraftingBasics => vec![
                TutorialReward::item("Crafting Kit", 1),
                TutorialReward::item("Iron Ore", 5),
            ],
            TutorialPhase::AdvancedSystems => vec![
                TutorialReward::experience(500),
                TutorialReward::stat_bonus("All Stats", 1),
            ],
        };

        for reward in rewards {
            self.earned_rewards.push(reward);
        }

        // Mark rewards as pending
        if let Some(progress) = self.progress.get_mut(&phase) {
            progress.rewards_claimed = false;
        }
    }

    /// Checks if all tutorials are complete
    fn check_all_complete(&mut self) {
        let all_complete = self.progress.values().all(|p| p.completed);
        let none_skipped = self.progress.values().all(|p| !p.skipped);

        if all_complete && !self.unlocked_achievements.contains(&TutorialAchievementId::GraduatedHero) {
            self.unlocked_achievements.insert(TutorialAchievementId::GraduatedHero);
            self.pending_popups.push(TutorialPopup::achievement(
                "Graduated Hero",
                "You've completed all tutorials! You're ready for the real adventure.",
            ));

            if none_skipped {
                self.unlocked_achievements.insert(TutorialAchievementId::DedicatedStudent);
            }
        }
    }

    /// Skips the current tutorial phase
    pub fn skip_current_tutorial(&mut self) {
        if let Some(step) = self.active_steps.first() {
            let phase = step.phase;
            if let Some(progress) = self.progress.get_mut(&phase) {
                progress.skipped = true;
                progress.completed = true;
            }
        }
        self.active_steps.clear();
        self.current_step_index = 0;
    }

    /// Gets the current tutorial step
    pub fn current_step(&self) -> Option<&TutorialStep> {
        self.active_steps.get(self.current_step_index)
    }

    /// Checks if a tutorial action matches the current step requirement
    pub fn check_action(&mut self, action: &TutorialAction) -> bool {
        if let Some(step) = self.active_steps.get(self.current_step_index) {
            if &step.required_action == action
                || step.required_action == TutorialAction::AnyKey
                || step.required_action == TutorialAction::Dismiss
            {
                return true;
            }

            // Handle Move matching MoveDirection
            if matches!(&step.required_action, TutorialAction::Move)
                && matches!(action, TutorialAction::MoveDirection(_))
            {
                return true;
            }
        }
        false
    }

    /// Records a game action for tutorial tracking
    pub fn record_action(&mut self, action: TutorialAction) {
        // Check active tutorial step
        if self.check_action(&action) {
            self.advance_step();
        }

        // Check practice room objectives
        if let Some(room_id) = &self.active_practice_room.clone() {
            if let Some(room) = self.practice_rooms.get_mut(room_id) {
                for objective in &mut room.objectives {
                    if objective.action == action {
                        objective.update();
                    }
                }
            }
        }

        // Update new player experience
        match action {
            TutorialAction::Move | TutorialAction::MoveDirection(_) => {
                if self.new_player.state == NewPlayerState::Introduction {
                    self.new_player.advance_state();
                }
            }
            TutorialAction::Attack => {
                self.new_player.unlock_feature(GameFeature::BasicCombat);
            }
            TutorialAction::PickupItem => {
                self.new_player.unlock_feature(GameFeature::ItemPickup);
            }
            TutorialAction::OpenInventory => {
                self.new_player.unlock_feature(GameFeature::Inventory);
            }
            _ => {}
        }
    }

    /// Processes a turn for tutorial tracking
    pub fn process_turn(&mut self) {
        self.turns_since_hint += 1;
        self.new_player.record_turn();
    }

    /// Triggers a contextual hint by trigger type
    pub fn trigger_hint(&mut self, trigger: &HintTrigger) -> Option<TutorialPopup> {
        if !self.settings.enable_contextual_tips {
            return None;
        }

        if self.contextual_hints_shown >= self.settings.max_contextual_hints {
            return None;
        }

        if self.turns_since_hint < self.settings.contextual_hint_cooldown {
            return None;
        }

        // Find matching hint
        let hint_id = self.hints.iter()
            .filter(|(_, h)| !h.shown && h.trigger == *trigger)
            .max_by_key(|(_, h)| h.priority)
            .map(|(id, _)| id.clone());

        if let Some(id) = hint_id {
            if let Some(hint) = self.hints.get_mut(&id) {
                hint.shown = true;
                self.contextual_hints_shown += 1;
                self.turns_since_hint = 0;

                return Some(TutorialPopup::hint(
                    &hint.title,
                    &hint.text,
                    hint.tips.clone(),
                ));
            }
        }

        None
    }

    /// Enters a practice room
    pub fn enter_practice_room(&mut self, room_id: &str) -> bool {
        if !self.settings.enable_practice_rooms {
            return false;
        }

        if self.practice_rooms.contains_key(room_id) {
            self.active_practice_room = Some(room_id.to_string());
            return true;
        }
        false
    }

    /// Exits the current practice room
    pub fn exit_practice_room(&mut self) {
        self.active_practice_room = None;
    }

    /// Gets the current practice room
    pub fn current_practice_room(&self) -> Option<&PracticeRoom> {
        self.active_practice_room.as_ref()
            .and_then(|id| self.practice_rooms.get(id))
    }

    /// Gets the next pending popup
    pub fn pop_popup(&mut self) -> Option<TutorialPopup> {
        if self.pending_popups.is_empty() {
            None
        } else {
            Some(self.pending_popups.remove(0))
        }
    }

    /// Checks if a phase is completed
    pub fn is_phase_completed(&self, phase: TutorialPhase) -> bool {
        self.progress.get(&phase).map_or(false, |p| p.completed)
    }

    /// Gets overall tutorial completion percentage
    pub fn overall_completion(&self) -> f32 {
        let completed = self.progress.values().filter(|p| p.completed).count() as f32;
        let total = self.progress.len() as f32;
        if total == 0.0 {
            return 0.0;
        }
        (completed / total) * 100.0
    }

    /// Gets phases available to start (prerequisites met)
    pub fn available_phases(&self) -> Vec<TutorialPhase> {
        TutorialPhase::all()
            .into_iter()
            .filter(|phase| {
                !self.is_phase_completed(*phase)
                    && phase.prerequisites().iter().all(|p| self.is_phase_completed(*p))
            })
            .collect()
    }

    /// Gets a timestamp (placeholder - would use actual time in real implementation)
    fn current_timestamp(&self) -> u64 {
        // In real implementation, this would return actual system time
        0
    }

    /// Gets unclaimed rewards
    pub fn get_unclaimed_rewards(&self) -> Vec<&TutorialReward> {
        self.earned_rewards.iter().collect()
    }

    /// Claims all pending rewards
    pub fn claim_rewards(&mut self) -> Vec<TutorialReward> {
        let rewards = std::mem::take(&mut self.earned_rewards);

        for progress in self.progress.values_mut() {
            progress.rewards_claimed = true;
        }

        rewards
    }
}

// ============================================================================
// Tutorial Popup
// ============================================================================

/// A tutorial popup to display to the player
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TutorialPopup {
    /// Type of popup
    pub popup_type: TutorialPopupType,
    /// Title text
    pub title: String,
    /// Main content text
    pub content: String,
    /// Additional tips
    pub tips: Vec<String>,
    /// Whether this popup can be dismissed
    pub dismissable: bool,
    /// Position on screen
    pub position: PopupPosition,
    /// Duration in seconds (None = until dismissed)
    pub duration: Option<f32>,
}

impl TutorialPopup {
    /// Creates a hint popup
    pub fn hint(title: &str, content: &str, tips: Vec<String>) -> Self {
        Self {
            popup_type: TutorialPopupType::Hint,
            title: title.to_string(),
            content: content.to_string(),
            tips,
            dismissable: true,
            position: PopupPosition::Center,
            duration: None,
        }
    }

    /// Creates an achievement popup
    pub fn achievement(title: &str, description: &str) -> Self {
        Self {
            popup_type: TutorialPopupType::Achievement,
            title: format!("Achievement Unlocked: {}", title),
            content: description.to_string(),
            tips: Vec::new(),
            dismissable: true,
            position: PopupPosition::TopCenter,
            duration: Some(5.0),
        }
    }

    /// Creates a tutorial step popup
    pub fn step(step: &TutorialStep) -> Self {
        let mut tips = Vec::new();
        if let Some(hint) = &step.hint {
            tips.push(hint.clone());
        }

        Self {
            popup_type: TutorialPopupType::TutorialStep,
            title: step.title.clone(),
            content: step.instruction.clone(),
            tips,
            dismissable: false,
            position: step.popup_position,
            duration: None,
        }
    }

    /// Creates a reward notification popup
    pub fn reward(rewards: &[TutorialReward]) -> Self {
        let content = rewards.iter()
            .map(|r| format!("- {}", r.description))
            .collect::<Vec<_>>()
            .join("\n");

        Self {
            popup_type: TutorialPopupType::Reward,
            title: "Tutorial Rewards!".to_string(),
            content,
            tips: Vec::new(),
            dismissable: true,
            position: PopupPosition::Center,
            duration: None,
        }
    }
}

/// Types of tutorial popups
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TutorialPopupType {
    Hint,
    TutorialStep,
    Achievement,
    Reward,
    Warning,
    Notification,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_system_creation() {
        let system = TutorialSystem::new();
        assert_eq!(system.progress.len(), TutorialPhase::all().len());
        assert!(!system.hints.is_empty());
    }

    #[test]
    fn test_tutorial_settings_defaults() {
        let settings = TutorialSettings::default();
        assert!(!settings.skip_all_tutorials);
        assert!(settings.enable_popup_hints);
        assert!(settings.any_tutorials_enabled());
    }

    #[test]
    fn test_experienced_settings() {
        let settings = TutorialSettings::experienced();
        assert!(!settings.enable_popup_hints);
        assert!(settings.show_only_new_features);
    }

    #[test]
    fn test_tutorial_phase_prerequisites() {
        let prereqs = TutorialPhase::CombatBasics.prerequisites();
        assert!(prereqs.contains(&TutorialPhase::BasicMovement));
    }

    #[test]
    fn test_start_guided_tutorial() {
        let mut system = TutorialSystem::new();
        system.start_guided_tutorial(TutorialPhase::BasicMovement);
        assert!(!system.active_steps.is_empty());
        assert_eq!(system.current_step_index, 0);
    }

    #[test]
    fn test_advance_tutorial_step() {
        let mut system = TutorialSystem::new();
        system.start_guided_tutorial(TutorialPhase::BasicMovement);
        let initial_index = system.current_step_index;
        system.advance_step();
        assert_eq!(system.current_step_index, initial_index + 1);
    }

    #[test]
    fn test_skip_tutorial() {
        let mut system = TutorialSystem::new();
        system.start_guided_tutorial(TutorialPhase::BasicMovement);
        system.skip_current_tutorial();
        assert!(system.active_steps.is_empty());
        assert!(system.is_phase_completed(TutorialPhase::BasicMovement));
    }

    #[test]
    fn test_reset_tutorials() {
        let mut system = TutorialSystem::new();
        system.start_guided_tutorial(TutorialPhase::BasicMovement);
        system.skip_current_tutorial();
        assert!(system.is_phase_completed(TutorialPhase::BasicMovement));

        system.reset_all();
        assert!(!system.is_phase_completed(TutorialPhase::BasicMovement));
    }

    #[test]
    fn test_help_system_search() {
        let help = HelpSystem::new();
        let results = help.search("movement");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_help_topic_categories() {
        let help = HelpSystem::new();
        let controls = help.get_by_category(HelpCategory::Controls);
        assert!(!controls.is_empty());
    }

    #[test]
    fn test_new_player_experience() {
        let mut npe = NewPlayerExperience::default();
        assert!(npe.enabled);
        assert!(npe.in_starting_area());

        npe.record_floor_complete();
        assert!(npe.is_feature_unlocked(GameFeature::BasicCombat));
    }

    #[test]
    fn test_experienced_player_npe() {
        let npe = NewPlayerExperience::experienced();
        assert!(!npe.enabled);
        assert!(!npe.in_starting_area());
        assert!(npe.is_feature_unlocked(GameFeature::Crafting));
    }

    #[test]
    fn test_practice_room() {
        let room = PracticeRoom::new(
            "test",
            "Test Room",
            "A test practice room",
            TutorialPhase::CombatBasics,
        ).with_safe_mode(true);

        assert!(room.safe_mode);
    }

    #[test]
    fn test_tutorial_reward_creation() {
        let gold = TutorialReward::gold(100);
        assert_eq!(gold.quantity, 100);
        assert_eq!(gold.reward_type, TutorialRewardType::Gold);

        let item = TutorialReward::item("Sword", 1);
        assert_eq!(item.item_name, Some("Sword".to_string()));
    }

    #[test]
    fn test_hotkey_categories() {
        let help = HelpSystem::new();
        let movement = help.get_hotkeys_by_category(HotkeyCategory::Movement);
        assert!(!movement.is_empty());
    }

    #[test]
    fn test_tutorial_completion_percentage() {
        let mut system = TutorialSystem::new();
        assert_eq!(system.overall_completion(), 0.0);

        system.start_guided_tutorial(TutorialPhase::BasicMovement);
        system.skip_current_tutorial();

        let completion = system.overall_completion();
        assert!(completion > 0.0);
    }

    #[test]
    fn test_available_phases() {
        let system = TutorialSystem::new();
        let available = system.available_phases();
        assert!(available.contains(&TutorialPhase::BasicMovement));
        // Combat requires movement, so shouldn't be available initially if we check properly
    }

    #[test]
    fn test_record_action() {
        let mut system = TutorialSystem::new();
        system.start_guided_tutorial(TutorialPhase::BasicMovement);
        let initial_step = system.current_step_index;

        // Record a move action (should advance step)
        system.record_action(TutorialAction::MoveDirection(Direction::North));
        assert!(system.current_step_index > initial_step || system.active_steps.is_empty());
    }

    #[test]
    fn test_popup_creation() {
        let popup = TutorialPopup::hint("Test", "Test content", vec!["Tip 1".to_string()]);
        assert_eq!(popup.title, "Test");
        assert!(popup.dismissable);

        let achievement = TutorialPopup::achievement("Test Achievement", "Description");
        assert!(achievement.title.contains("Achievement Unlocked"));
    }
}

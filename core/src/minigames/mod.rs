//! Minigames System: Interactive skill-based activities with rewards
//!
//! This module provides a comprehensive minigame system for ShadowCrypt including:
//! - Various minigame types (lockpicking, fishing, mining, cooking, etc.)
//! - Difficulty scaling based on player skill and context
//! - Reward systems with items, gold, experience, and rare drops
//! - Tournament system for competitive play
//! - Accessibility settings for skipping or simplifying minigames
//! - Achievement tracking for mastery

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::items::{Item, ItemKind, Rarity};

// ============================================================================
// Constants
// ============================================================================

/// Maximum difficulty level for minigames
pub const MAX_DIFFICULTY: u32 = 10;

/// Base time limit in milliseconds for timed minigames
pub const BASE_TIME_LIMIT_MS: u64 = 30000;

/// Experience multiplier for perfect completion
pub const PERFECT_XP_MULTIPLIER: f32 = 2.0;

/// Gold multiplier for perfect completion
pub const PERFECT_GOLD_MULTIPLIER: f32 = 1.5;

/// Penalty multiplier when auto-skipping minigames
pub const AUTO_SKIP_PENALTY: f32 = 0.5;

/// Number of tournament rounds
pub const TOURNAMENT_ROUNDS: u32 = 5;

/// Maximum practice attempts before cooldown
pub const MAX_PRACTICE_ATTEMPTS: u32 = 10;

// ============================================================================
// Minigame Types
// ============================================================================

/// All available minigame types in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MinigameType {
    /// Pick locks on chests and doors using tumbler manipulation
    Lockpicking,
    /// Catch fish using timing-based mechanics
    Fishing,
    /// Extract ores and gems with rhythm-based mining
    Mining,
    /// Gather plants with pattern recognition
    Herbalism,
    /// Prepare food with ingredient timing
    Cooking,
    /// Brew potions with precise measurements
    Alchemy,
    /// Forge items with temperature control
    Blacksmithing,
    /// Play card games against NPCs
    CardGame,
    /// Roll dice in gambling games
    DiceGame,
    /// Hit notes in rhythm-based challenges
    RhythmGame,
    /// Solve puzzles to unlock secrets
    PuzzleSolving,
    /// Race mounts against opponents
    Racing,
    /// Hit targets with precision timing
    TargetPractice,
    /// Arm wrestling with button mashing
    ArmWrestling,
    /// Drinking contest with QTE sequences
    DrinkingContest,
    /// Haggling with NPCs using strategy
    Haggling,
    /// Pickpocketing with stealth timing
    Pickpocketing,
    /// Disarming traps with careful precision
    TrapDisarming,
    /// Taming wild creatures
    CreatureTaming,
}

impl MinigameType {
    /// Returns the display name of this minigame type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Lockpicking => "Lockpicking",
            Self::Fishing => "Fishing",
            Self::Mining => "Mining",
            Self::Herbalism => "Herbalism",
            Self::Cooking => "Cooking",
            Self::Alchemy => "Alchemy",
            Self::Blacksmithing => "Blacksmithing",
            Self::CardGame => "Card Game",
            Self::DiceGame => "Dice Game",
            Self::RhythmGame => "Rhythm Game",
            Self::PuzzleSolving => "Puzzle Solving",
            Self::Racing => "Racing",
            Self::TargetPractice => "Target Practice",
            Self::ArmWrestling => "Arm Wrestling",
            Self::DrinkingContest => "Drinking Contest",
            Self::Haggling => "Haggling",
            Self::Pickpocketing => "Pickpocketing",
            Self::TrapDisarming => "Trap Disarming",
            Self::CreatureTaming => "Creature Taming",
        }
    }

    /// Returns a description of the minigame
    pub fn description(&self) -> &'static str {
        match self {
            Self::Lockpicking => "Manipulate tumblers to open locked chests and doors. Listen for clicks and feel for resistance.",
            Self::Fishing => "Cast your line and wait for a bite. Time your reeling perfectly to catch fish.",
            Self::Mining => "Strike ore veins with proper timing to extract valuable materials without breaking tools.",
            Self::Herbalism => "Carefully harvest plants by identifying the correct patterns. Preserve their magical properties.",
            Self::Cooking => "Combine ingredients and time your cooking to prepare food with powerful buffs.",
            Self::Alchemy => "Mix reagents with precise measurements. Control temperature for perfect potions.",
            Self::Blacksmithing => "Hammer heated metal in rhythm to forge powerful equipment. Control the heat.",
            Self::CardGame => "Outplay your opponent in strategic card battles. Know when to hold and fold.",
            Self::DiceGame => "Roll the bones and test your luck. Choose wisely when to push your fortune.",
            Self::RhythmGame => "Hit the notes in time with the music. Perfect your performance for bonuses.",
            Self::PuzzleSolving => "Solve intricate puzzles to unlock secrets and ancient mechanisms.",
            Self::Racing => "Race your mount against opponents. Manage stamina and find shortcuts.",
            Self::TargetPractice => "Test your aim with precision archery. Account for wind and distance.",
            Self::ArmWrestling => "Overpower your opponent with rapid inputs. Watch for their fatigue.",
            Self::DrinkingContest => "Outlast your opponent in a drinking contest. Manage your tolerance.",
            Self::Haggling => "Negotiate prices with merchants. Read their tells and make counter-offers.",
            Self::Pickpocketing => "Time your moves to steal without detection. Watch the target's attention.",
            Self::TrapDisarming => "Carefully disarm traps by following the correct sequence. One mistake is deadly.",
            Self::CreatureTaming => "Calm and befriend wild creatures through patience and timing.",
        }
    }

    /// Returns the glyph for this minigame type
    pub fn glyph(&self) -> char {
        match self {
            Self::Lockpicking => 'L',
            Self::Fishing => 'F',
            Self::Mining => 'M',
            Self::Herbalism => 'H',
            Self::Cooking => 'C',
            Self::Alchemy => 'A',
            Self::Blacksmithing => 'B',
            Self::CardGame => 'c',
            Self::DiceGame => 'd',
            Self::RhythmGame => 'R',
            Self::PuzzleSolving => 'P',
            Self::Racing => 'r',
            Self::TargetPractice => 'T',
            Self::ArmWrestling => 'W',
            Self::DrinkingContest => 'D',
            Self::Haggling => '$',
            Self::Pickpocketing => 'p',
            Self::TrapDisarming => 't',
            Self::CreatureTaming => 'Z',
        }
    }

    /// Returns the input mechanic type for this minigame
    pub fn input_mechanic(&self) -> InputMechanic {
        match self {
            Self::Lockpicking => InputMechanic::PrecisionTiming,
            Self::Fishing => InputMechanic::HoldAndRelease,
            Self::Mining => InputMechanic::RhythmSequence,
            Self::Herbalism => InputMechanic::PatternMatching,
            Self::Cooking => InputMechanic::TimingSequence,
            Self::Alchemy => InputMechanic::ResourceManagement,
            Self::Blacksmithing => InputMechanic::RhythmSequence,
            Self::CardGame => InputMechanic::StrategicChoice,
            Self::DiceGame => InputMechanic::RiskAssessment,
            Self::RhythmGame => InputMechanic::RhythmSequence,
            Self::PuzzleSolving => InputMechanic::PatternMatching,
            Self::Racing => InputMechanic::ReactionTime,
            Self::TargetPractice => InputMechanic::PrecisionTiming,
            Self::ArmWrestling => InputMechanic::ButtonMashing,
            Self::DrinkingContest => InputMechanic::QuickTimeEvent,
            Self::Haggling => InputMechanic::StrategicChoice,
            Self::Pickpocketing => InputMechanic::StealthTiming,
            Self::TrapDisarming => InputMechanic::PrecisionTiming,
            Self::CreatureTaming => InputMechanic::PatternMatching,
        }
    }

    /// Returns the base difficulty for this minigame type
    pub fn base_difficulty(&self) -> u32 {
        match self {
            Self::Fishing | Self::Mining | Self::DiceGame => 2,
            Self::Herbalism | Self::Cooking | Self::CardGame => 3,
            Self::Lockpicking | Self::TargetPractice | Self::Racing => 4,
            Self::Alchemy | Self::RhythmGame | Self::ArmWrestling => 5,
            Self::Blacksmithing | Self::DrinkingContest | Self::Haggling => 6,
            Self::PuzzleSolving | Self::Pickpocketing | Self::CreatureTaming => 7,
            Self::TrapDisarming => 8,
        }
    }

    /// Returns the base time limit in milliseconds
    pub fn base_time_limit(&self) -> u64 {
        match self {
            Self::DiceGame | Self::CardGame => 60000,  // Strategy games get more time
            Self::Fishing => 45000,
            Self::PuzzleSolving => 90000,
            Self::Racing => 120000,
            Self::Haggling => 60000,
            Self::DrinkingContest => 30000,
            Self::ArmWrestling => 15000,
            _ => BASE_TIME_LIMIT_MS,
        }
    }

    /// Returns the skill associated with this minigame
    pub fn associated_skill(&self) -> MinigameSkill {
        match self {
            Self::Lockpicking | Self::TrapDisarming | Self::Pickpocketing => MinigameSkill::Dexterity,
            Self::Fishing | Self::Herbalism | Self::CreatureTaming => MinigameSkill::Patience,
            Self::Mining | Self::Blacksmithing | Self::ArmWrestling => MinigameSkill::Strength,
            Self::Cooking | Self::Alchemy => MinigameSkill::Precision,
            Self::CardGame | Self::Haggling | Self::PuzzleSolving => MinigameSkill::Intelligence,
            Self::DiceGame | Self::DrinkingContest => MinigameSkill::Luck,
            Self::RhythmGame | Self::Racing => MinigameSkill::Rhythm,
            Self::TargetPractice => MinigameSkill::Accuracy,
        }
    }

    /// Returns whether this minigame supports practice mode
    pub fn supports_practice(&self) -> bool {
        !matches!(self, Self::DiceGame | Self::CardGame | Self::DrinkingContest)
    }

    /// Returns whether this minigame can be part of tournaments
    pub fn supports_tournament(&self) -> bool {
        matches!(self,
            Self::Fishing | Self::Mining | Self::Cooking | Self::CardGame |
            Self::DiceGame | Self::RhythmGame | Self::Racing | Self::TargetPractice |
            Self::ArmWrestling | Self::DrinkingContest
        )
    }

    /// Returns all minigame types
    pub fn all() -> Vec<Self> {
        vec![
            Self::Lockpicking, Self::Fishing, Self::Mining, Self::Herbalism,
            Self::Cooking, Self::Alchemy, Self::Blacksmithing, Self::CardGame,
            Self::DiceGame, Self::RhythmGame, Self::PuzzleSolving, Self::Racing,
            Self::TargetPractice, Self::ArmWrestling, Self::DrinkingContest,
            Self::Haggling, Self::Pickpocketing, Self::TrapDisarming, Self::CreatureTaming,
        ]
    }
}

// ============================================================================
// Input Mechanics
// ============================================================================

/// Types of input mechanics used by minigames
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum InputMechanic {
    /// Precise timing windows (lockpicking, target practice)
    PrecisionTiming,
    /// Hold and release at the right moment (fishing)
    HoldAndRelease,
    /// Hit inputs in rhythm (mining, blacksmithing, rhythm game)
    RhythmSequence,
    /// Match patterns or sequences (herbalism, puzzles)
    PatternMatching,
    /// Time multiple inputs in sequence (cooking)
    TimingSequence,
    /// Manage limited resources (alchemy)
    ResourceManagement,
    /// Make strategic decisions (cards, haggling)
    StrategicChoice,
    /// Assess risk vs reward (dice)
    RiskAssessment,
    /// Quick reaction tests (racing)
    ReactionTime,
    /// Rapid repeated inputs (arm wrestling)
    ButtonMashing,
    /// Quick time events (drinking contest)
    QuickTimeEvent,
    /// Time actions during safe windows (pickpocketing)
    StealthTiming,
}

impl InputMechanic {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::PrecisionTiming => "Precision Timing",
            Self::HoldAndRelease => "Hold and Release",
            Self::RhythmSequence => "Rhythm Sequence",
            Self::PatternMatching => "Pattern Matching",
            Self::TimingSequence => "Timing Sequence",
            Self::ResourceManagement => "Resource Management",
            Self::StrategicChoice => "Strategic Choice",
            Self::RiskAssessment => "Risk Assessment",
            Self::ReactionTime => "Reaction Time",
            Self::ButtonMashing => "Button Mashing",
            Self::QuickTimeEvent => "Quick Time Event",
            Self::StealthTiming => "Stealth Timing",
        }
    }

    /// Returns a description of how this mechanic works
    pub fn description(&self) -> &'static str {
        match self {
            Self::PrecisionTiming => "Press the action key at exactly the right moment in a timing window.",
            Self::HoldAndRelease => "Hold the action key and release at the optimal moment.",
            Self::RhythmSequence => "Press keys in time with a rhythm or beat pattern.",
            Self::PatternMatching => "Identify and replicate patterns or sequences.",
            Self::TimingSequence => "Complete multiple timed actions in the correct order.",
            Self::ResourceManagement => "Balance and allocate limited resources effectively.",
            Self::StrategicChoice => "Make decisions that affect the outcome strategically.",
            Self::RiskAssessment => "Decide when to push your luck versus playing safe.",
            Self::ReactionTime => "React quickly to visual or audio cues.",
            Self::ButtonMashing => "Press buttons rapidly to build up power or speed.",
            Self::QuickTimeEvent => "Press the correct keys when prompted within time limits.",
            Self::StealthTiming => "Act during safe windows while avoiding detection.",
        }
    }

    /// Returns whether this mechanic can be simplified for accessibility
    pub fn can_simplify(&self) -> bool {
        !matches!(self, Self::StrategicChoice | Self::RiskAssessment)
    }

    /// Returns the QTE alternative for this mechanic
    pub fn qte_alternative(&self) -> Option<&'static str> {
        match self {
            Self::PrecisionTiming => Some("Single button press with wider timing window"),
            Self::HoldAndRelease => Some("Two button presses: start and stop"),
            Self::RhythmSequence => Some("Simplified beat pattern with visual cues"),
            Self::PatternMatching => Some("Multiple choice selection"),
            Self::TimingSequence => Some("Turn-based sequential inputs"),
            Self::ResourceManagement => Some("Automatic resource allocation with confirmation"),
            Self::ReactionTime => Some("Slower cues with longer windows"),
            Self::ButtonMashing => Some("Hold button instead of mashing"),
            Self::QuickTimeEvent => Some("Slower prompts with audio cues"),
            Self::StealthTiming => Some("Clearer safe window indicators"),
            _ => None,
        }
    }
}

// ============================================================================
// Minigame Skills
// ============================================================================

/// Skills that affect minigame performance
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MinigameSkill {
    /// Manual dexterity for precise actions
    Dexterity,
    /// Patience for timing-based activities
    Patience,
    /// Physical strength for power-based activities
    Strength,
    /// Precision for exact measurements
    Precision,
    /// Intelligence for strategic thinking
    Intelligence,
    /// Luck for chance-based outcomes
    Luck,
    /// Rhythm for music and timing
    Rhythm,
    /// Accuracy for aiming
    Accuracy,
}

impl MinigameSkill {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dexterity => "Dexterity",
            Self::Patience => "Patience",
            Self::Strength => "Strength",
            Self::Precision => "Precision",
            Self::Intelligence => "Intelligence",
            Self::Luck => "Luck",
            Self::Rhythm => "Rhythm",
            Self::Accuracy => "Accuracy",
        }
    }

    /// Returns the bonus provided per skill level
    pub fn bonus_per_level(&self) -> f32 {
        match self {
            Self::Dexterity => 0.05,     // 5% timing window increase
            Self::Patience => 0.08,      // 8% time limit increase
            Self::Strength => 0.10,      // 10% power increase
            Self::Precision => 0.05,     // 5% accuracy increase
            Self::Intelligence => 0.07,  // 7% puzzle hint chance
            Self::Luck => 0.03,          // 3% bonus outcome chance
            Self::Rhythm => 0.06,        // 6% timing tolerance
            Self::Accuracy => 0.08,      // 8% aim assist
        }
    }

    /// Returns the experience needed to level up this skill
    pub fn xp_to_level(&self, current_level: u32) -> u32 {
        100 * current_level + (current_level * current_level * 10)
    }
}

// ============================================================================
// Difficulty System
// ============================================================================

/// Difficulty level for minigames
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum DifficultyLevel {
    Tutorial,
    Easy,
    Normal,
    Hard,
    Expert,
    Master,
    Legendary,
}

impl DifficultyLevel {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Tutorial => "Tutorial",
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::Expert => "Expert",
            Self::Master => "Master",
            Self::Legendary => "Legendary",
        }
    }

    /// Returns the numeric difficulty value (1-10)
    pub fn value(&self) -> u32 {
        match self {
            Self::Tutorial => 1,
            Self::Easy => 2,
            Self::Normal => 4,
            Self::Hard => 6,
            Self::Expert => 7,
            Self::Master => 9,
            Self::Legendary => 10,
        }
    }

    /// Returns the time modifier (multiplier for time limit)
    pub fn time_modifier(&self) -> f32 {
        match self {
            Self::Tutorial => 2.0,
            Self::Easy => 1.5,
            Self::Normal => 1.0,
            Self::Hard => 0.85,
            Self::Expert => 0.7,
            Self::Master => 0.55,
            Self::Legendary => 0.4,
        }
    }

    /// Returns the reward modifier
    pub fn reward_modifier(&self) -> f32 {
        match self {
            Self::Tutorial => 0.25,
            Self::Easy => 0.5,
            Self::Normal => 1.0,
            Self::Hard => 1.5,
            Self::Expert => 2.0,
            Self::Master => 3.0,
            Self::Legendary => 5.0,
        }
    }

    /// Returns the tolerance modifier for timing windows
    pub fn tolerance_modifier(&self) -> f32 {
        match self {
            Self::Tutorial => 2.0,
            Self::Easy => 1.5,
            Self::Normal => 1.0,
            Self::Hard => 0.8,
            Self::Expert => 0.6,
            Self::Master => 0.4,
            Self::Legendary => 0.25,
        }
    }

    /// Convert from numeric value
    pub fn from_value(value: u32) -> Self {
        match value {
            0..=1 => Self::Tutorial,
            2..=3 => Self::Easy,
            4..=5 => Self::Normal,
            6 => Self::Hard,
            7..=8 => Self::Expert,
            9 => Self::Master,
            _ => Self::Legendary,
        }
    }

    /// Returns the next difficulty level
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Tutorial => Some(Self::Easy),
            Self::Easy => Some(Self::Normal),
            Self::Normal => Some(Self::Hard),
            Self::Hard => Some(Self::Expert),
            Self::Expert => Some(Self::Master),
            Self::Master => Some(Self::Legendary),
            Self::Legendary => None,
        }
    }
}

// ============================================================================
// Minigame State
// ============================================================================

/// Current state of a minigame session
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MinigameState {
    /// Type of minigame
    pub minigame_type: MinigameType,
    /// Current difficulty
    pub difficulty: DifficultyLevel,
    /// Whether this is practice mode
    pub is_practice: bool,
    /// Time remaining in milliseconds
    pub time_remaining_ms: u64,
    /// Current score or progress (0-100)
    pub progress: u32,
    /// Number of mistakes made
    pub mistakes: u32,
    /// Maximum allowed mistakes
    pub max_mistakes: u32,
    /// Current phase or stage of the minigame
    pub current_phase: u32,
    /// Total phases to complete
    pub total_phases: u32,
    /// Current input sequence (for pattern games)
    pub input_sequence: Vec<MinigameInput>,
    /// Expected sequence (for validation)
    pub expected_sequence: Vec<MinigameInput>,
    /// Combo counter for consecutive successes
    pub combo: u32,
    /// Best combo achieved this session
    pub best_combo: u32,
    /// Whether the game is currently active
    pub is_active: bool,
    /// Whether the game is paused
    pub is_paused: bool,
    /// Random seed for reproducibility
    pub seed: u64,
    /// Context-specific data
    pub context_data: HashMap<String, i32>,
}

impl MinigameState {
    /// Create a new minigame state
    pub fn new(minigame_type: MinigameType, difficulty: DifficultyLevel, is_practice: bool) -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();

        let base_time = minigame_type.base_time_limit();
        let time_remaining = (base_time as f32 * difficulty.time_modifier()) as u64;

        let max_mistakes = match difficulty {
            DifficultyLevel::Tutorial => 10,
            DifficultyLevel::Easy => 5,
            DifficultyLevel::Normal => 3,
            DifficultyLevel::Hard => 2,
            DifficultyLevel::Expert => 1,
            DifficultyLevel::Master => 1,
            DifficultyLevel::Legendary => 0,
        };

        let total_phases = match minigame_type {
            MinigameType::Lockpicking => 3 + difficulty.value() / 2,
            MinigameType::Cooking => 4 + difficulty.value() / 3,
            MinigameType::Alchemy => 5 + difficulty.value() / 2,
            MinigameType::Blacksmithing => 4 + difficulty.value() / 2,
            MinigameType::PuzzleSolving => 3 + difficulty.value(),
            _ => 1,
        };

        Self {
            minigame_type,
            difficulty,
            is_practice,
            time_remaining_ms: time_remaining,
            progress: 0,
            mistakes: 0,
            max_mistakes,
            current_phase: 0,
            total_phases,
            input_sequence: Vec::new(),
            expected_sequence: Vec::new(),
            combo: 0,
            best_combo: 0,
            is_active: true,
            is_paused: false,
            seed,
            context_data: HashMap::new(),
        }
    }

    /// Generate the expected sequence for pattern-based games
    pub fn generate_sequence(&mut self, length: usize) {
        let mut rng = rand::thread_rng();
        self.expected_sequence.clear();

        for _ in 0..length {
            let input = match rng.gen_range(0..4) {
                0 => MinigameInput::Up,
                1 => MinigameInput::Down,
                2 => MinigameInput::Left,
                _ => MinigameInput::Right,
            };
            self.expected_sequence.push(input);
        }
    }

    /// Process a player input
    pub fn process_input(&mut self, input: MinigameInput) -> InputResult {
        if !self.is_active || self.is_paused {
            return InputResult::Ignored;
        }

        self.input_sequence.push(input);

        // Check against expected sequence for pattern games
        if !self.expected_sequence.is_empty() {
            let idx = self.input_sequence.len() - 1;
            if idx < self.expected_sequence.len() {
                if self.input_sequence[idx] == self.expected_sequence[idx] {
                    self.combo += 1;
                    if self.combo > self.best_combo {
                        self.best_combo = self.combo;
                    }

                    if self.input_sequence.len() == self.expected_sequence.len() {
                        self.current_phase += 1;
                        self.progress = (self.current_phase * 100 / self.total_phases).min(100);
                        self.input_sequence.clear();

                        if self.current_phase >= self.total_phases {
                            return InputResult::PhaseComplete;
                        }
                        return InputResult::Correct;
                    }
                    return InputResult::Correct;
                } else {
                    self.mistakes += 1;
                    self.combo = 0;
                    self.input_sequence.clear();

                    if self.mistakes >= self.max_mistakes && !self.is_practice {
                        self.is_active = false;
                        return InputResult::Failed;
                    }
                    return InputResult::Incorrect;
                }
            }
        }

        InputResult::Processed
    }

    /// Update time remaining
    pub fn tick(&mut self, delta_ms: u64) {
        if !self.is_active || self.is_paused {
            return;
        }

        if self.time_remaining_ms > delta_ms {
            self.time_remaining_ms -= delta_ms;
        } else {
            self.time_remaining_ms = 0;
            if !self.is_practice {
                self.is_active = false;
            }
        }
    }

    /// Check if the minigame is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 100 || self.current_phase >= self.total_phases
    }

    /// Check if the minigame failed
    pub fn is_failed(&self) -> bool {
        !self.is_active && !self.is_complete()
    }

    /// Calculate the performance rating (0-100)
    pub fn performance_rating(&self) -> u32 {
        if self.is_failed() {
            return 0;
        }

        let base_score = self.progress;
        let time_bonus = if self.time_remaining_ms > 0 {
            (self.time_remaining_ms as f32 / self.minigame_type.base_time_limit() as f32 * 20.0) as u32
        } else {
            0
        };
        let combo_bonus = (self.best_combo * 2).min(20);
        let mistake_penalty = self.mistakes * 10;

        (base_score + time_bonus + combo_bonus).saturating_sub(mistake_penalty).min(100)
    }

    /// Check if this was a perfect completion
    pub fn is_perfect(&self) -> bool {
        self.is_complete() && self.mistakes == 0 && self.performance_rating() >= 95
    }
}

/// Possible inputs for minigames
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MinigameInput {
    Up,
    Down,
    Left,
    Right,
    Action,
    Cancel,
    Special,
    Number(u8),
}

/// Result of processing an input
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputResult {
    Correct,
    Incorrect,
    Processed,
    PhaseComplete,
    Failed,
    Ignored,
}

// ============================================================================
// Minigame Results and Rewards
// ============================================================================

/// Result of completing a minigame
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MinigameResult {
    /// Type of minigame completed
    pub minigame_type: MinigameType,
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    /// Whether the minigame was successful
    pub success: bool,
    /// Performance rating (0-100)
    pub performance: u32,
    /// Whether it was a perfect completion
    pub is_perfect: bool,
    /// Time taken in milliseconds
    pub time_taken_ms: u64,
    /// Mistakes made
    pub mistakes: u32,
    /// Best combo achieved
    pub best_combo: u32,
    /// Whether this was practice mode
    pub was_practice: bool,
    /// Whether auto-skip was used
    pub was_auto_skipped: bool,
}

impl MinigameResult {
    /// Create a result from a minigame state
    pub fn from_state(state: &MinigameState, base_time: u64) -> Self {
        Self {
            minigame_type: state.minigame_type,
            difficulty: state.difficulty,
            success: state.is_complete(),
            performance: state.performance_rating(),
            is_perfect: state.is_perfect(),
            time_taken_ms: base_time.saturating_sub(state.time_remaining_ms),
            mistakes: state.mistakes,
            best_combo: state.best_combo,
            was_practice: state.is_practice,
            was_auto_skipped: false,
        }
    }

    /// Create a result for auto-skipped minigame
    pub fn auto_skipped(minigame_type: MinigameType, difficulty: DifficultyLevel) -> Self {
        Self {
            minigame_type,
            difficulty,
            success: true,
            performance: 50,
            is_perfect: false,
            time_taken_ms: 0,
            mistakes: 0,
            best_combo: 0,
            was_practice: false,
            was_auto_skipped: true,
        }
    }
}

/// Rewards earned from completing a minigame
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MinigameReward {
    /// Gold earned
    pub gold: u32,
    /// Experience earned
    pub experience: u32,
    /// Skill experience for the associated skill
    pub skill_experience: u32,
    /// Items earned
    pub items: Vec<RewardItem>,
    /// Special bonuses (buff name, duration)
    pub buffs: Vec<(String, u32)>,
    /// Whether a rare drop occurred
    pub rare_drop: bool,
    /// Achievement unlocked (if any)
    pub achievement: Option<MinigameAchievementId>,
}

impl MinigameReward {
    /// Create an empty reward
    pub fn empty() -> Self {
        Self {
            gold: 0,
            experience: 0,
            skill_experience: 0,
            items: Vec::new(),
            buffs: Vec::new(),
            rare_drop: false,
            achievement: None,
        }
    }

    /// Calculate rewards based on minigame result
    pub fn calculate(result: &MinigameResult, context: &MinigameContext, rng: &mut impl Rng) -> Self {
        if result.was_practice || !result.success {
            return Self::empty();
        }

        let difficulty_mult = result.difficulty.reward_modifier();
        let performance_mult = result.performance as f32 / 100.0;
        let skip_mult = if result.was_auto_skipped { AUTO_SKIP_PENALTY } else { 1.0 };
        let perfect_mult = if result.is_perfect { PERFECT_XP_MULTIPLIER } else { 1.0 };

        let base_gold = context.base_gold_reward;
        let base_xp = context.base_xp_reward;

        let gold = (base_gold as f32 * difficulty_mult * performance_mult * skip_mult) as u32;
        let experience = (base_xp as f32 * difficulty_mult * performance_mult * perfect_mult) as u32;
        let skill_experience = (50.0 * difficulty_mult * performance_mult) as u32;

        // Generate items based on context
        let mut items = Vec::new();
        for (item_kind, rarity, chance) in &context.possible_items {
            let adjusted_chance = (*chance as f32 * performance_mult * difficulty_mult) as u32;
            if rng.gen_range(0..100) < adjusted_chance {
                items.push(RewardItem {
                    item_kind: *item_kind,
                    rarity: *rarity,
                    quantity: 1,
                });
            }
        }

        // Rare drop check
        let rare_drop_chance = if result.is_perfect { 15 } else { 5 };
        let rare_drop = rng.gen_range(0..100) < rare_drop_chance;
        if rare_drop {
            if let Some((item_kind, rarity)) = context.rare_drop {
                items.push(RewardItem {
                    item_kind,
                    rarity,
                    quantity: 1,
                });
            }
        }

        // Buffs for high performance
        let mut buffs = Vec::new();
        if result.performance >= 90 {
            let buff_name = match result.minigame_type {
                MinigameType::Cooking => "Well Fed",
                MinigameType::Alchemy => "Alchemical Insight",
                MinigameType::Fishing => "Patient Angler",
                MinigameType::Mining => "Miner's Fortune",
                _ => "Skilled Hands",
            };
            let duration = 50 + (result.performance as u32 / 2);
            buffs.push((buff_name.to_string(), duration));
        }

        Self {
            gold,
            experience,
            skill_experience,
            items,
            buffs,
            rare_drop,
            achievement: None,
        }
    }
}

/// A reward item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RewardItem {
    pub item_kind: ItemKind,
    pub rarity: Rarity,
    pub quantity: u32,
}

// ============================================================================
// Minigame Context
// ============================================================================

/// Context for a minigame session (what triggered it, rewards available, etc.)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MinigameContext {
    /// What triggered this minigame
    pub trigger: MinigameTrigger,
    /// Base gold reward
    pub base_gold_reward: u32,
    /// Base experience reward
    pub base_xp_reward: u32,
    /// Possible item rewards (item kind, rarity, chance %)
    pub possible_items: Vec<(ItemKind, Rarity, u32)>,
    /// Rare drop (item kind, rarity) - only if rare drop triggers
    pub rare_drop: Option<(ItemKind, Rarity)>,
    /// Minimum difficulty override
    pub min_difficulty: Option<DifficultyLevel>,
    /// Maximum difficulty override
    pub max_difficulty: Option<DifficultyLevel>,
    /// Whether failure has consequences
    pub failure_consequence: Option<FailureConsequence>,
    /// Associated location (x, y)
    pub location: Option<(usize, usize)>,
    /// NPC opponent (for competitive games)
    pub opponent: Option<String>,
}

impl MinigameContext {
    /// Create a default context for a minigame type
    pub fn default_for(minigame_type: MinigameType) -> Self {
        let (base_gold, base_xp) = match minigame_type {
            MinigameType::Lockpicking => (50, 30),
            MinigameType::Fishing => (20, 25),
            MinigameType::Mining => (30, 20),
            MinigameType::Herbalism => (25, 25),
            MinigameType::Cooking => (15, 30),
            MinigameType::Alchemy => (40, 40),
            MinigameType::Blacksmithing => (50, 35),
            MinigameType::CardGame => (100, 20),
            MinigameType::DiceGame => (75, 15),
            MinigameType::RhythmGame => (30, 35),
            MinigameType::PuzzleSolving => (60, 50),
            MinigameType::Racing => (80, 30),
            MinigameType::TargetPractice => (35, 25),
            MinigameType::ArmWrestling => (50, 20),
            MinigameType::DrinkingContest => (40, 15),
            MinigameType::Haggling => (0, 25), // Gold comes from the deal
            MinigameType::Pickpocketing => (75, 30),
            MinigameType::TrapDisarming => (0, 45),
            MinigameType::CreatureTaming => (0, 60),
        };

        Self {
            trigger: MinigameTrigger::Manual,
            base_gold_reward: base_gold,
            base_xp_reward: base_xp,
            possible_items: Vec::new(),
            rare_drop: None,
            min_difficulty: None,
            max_difficulty: None,
            failure_consequence: None,
            location: None,
            opponent: None,
        }
    }

    /// Create context for a locked chest
    pub fn locked_chest(chest_rarity: Rarity, x: usize, y: usize) -> Self {
        let difficulty = match chest_rarity {
            Rarity::Common => DifficultyLevel::Easy,
            Rarity::Uncommon => DifficultyLevel::Normal,
            Rarity::Rare => DifficultyLevel::Hard,
            Rarity::Epic => DifficultyLevel::Expert,
            Rarity::Legendary => DifficultyLevel::Master,
            Rarity::Mythic => DifficultyLevel::Legendary,
        };

        Self {
            trigger: MinigameTrigger::LockedChest,
            base_gold_reward: 50 * (chest_rarity as u32 + 1),
            base_xp_reward: 30,
            possible_items: Vec::new(), // Chest contents handled separately
            rare_drop: None,
            min_difficulty: Some(difficulty),
            max_difficulty: Some(difficulty),
            failure_consequence: Some(FailureConsequence::LockJammed),
            location: Some((x, y)),
            opponent: None,
        }
    }

    /// Create context for fishing at a fishing spot
    pub fn fishing_spot(water_type: WaterType, x: usize, y: usize) -> Self {
        let (base_gold, difficulty) = match water_type {
            WaterType::Pond => (15, DifficultyLevel::Easy),
            WaterType::River => (25, DifficultyLevel::Normal),
            WaterType::Lake => (35, DifficultyLevel::Hard),
            WaterType::Ocean => (50, DifficultyLevel::Expert),
            WaterType::Magical => (100, DifficultyLevel::Master),
        };

        Self {
            trigger: MinigameTrigger::FishingSpot,
            base_gold_reward: base_gold,
            base_xp_reward: 25,
            possible_items: vec![
                (ItemKind::Fish, Rarity::Common, 80),
                (ItemKind::RareFish, Rarity::Rare, 15),
            ],
            rare_drop: Some((ItemKind::LegendaryFish, Rarity::Legendary)),
            min_difficulty: Some(difficulty),
            max_difficulty: None,
            failure_consequence: Some(FailureConsequence::LineBroken),
            location: Some((x, y)),
            opponent: None,
        }
    }

    /// Create context for mining
    pub fn mining_node(ore_type: OreType, x: usize, y: usize) -> Self {
        let (base_gold, difficulty, rarity) = match ore_type {
            OreType::Iron => (20, DifficultyLevel::Easy, Rarity::Common),
            OreType::Silver => (40, DifficultyLevel::Normal, Rarity::Uncommon),
            OreType::Gold => (60, DifficultyLevel::Hard, Rarity::Rare),
            OreType::Mithril => (100, DifficultyLevel::Expert, Rarity::Epic),
            OreType::Adamantite => (150, DifficultyLevel::Master, Rarity::Legendary),
        };

        Self {
            trigger: MinigameTrigger::MiningNode,
            base_gold_reward: base_gold,
            base_xp_reward: 20,
            possible_items: vec![
                (ItemKind::OreChunk, rarity, 100),
                (ItemKind::GemFragment, Rarity::Rare, 10),
            ],
            rare_drop: Some((ItemKind::PerfectGem, Rarity::Epic)),
            min_difficulty: Some(difficulty),
            max_difficulty: None,
            failure_consequence: Some(FailureConsequence::ToolBroken),
            location: Some((x, y)),
            opponent: None,
        }
    }
}

/// What triggered the minigame
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MinigameTrigger {
    Manual,
    LockedChest,
    LockedDoor,
    FishingSpot,
    MiningNode,
    HerbNode,
    CraftingStation,
    NPCChallenge,
    Tournament,
    Quest,
    Trap,
    TameableCreature,
    Merchant,
}

/// Consequence of failing a minigame
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum FailureConsequence {
    /// Lock becomes jammed, needs a key or higher skill
    LockJammed,
    /// Fishing line breaks, lose bait
    LineBroken,
    /// Mining tool breaks
    ToolBroken,
    /// Plant destroyed, no harvest
    PlantDestroyed,
    /// Potion explodes, take damage
    PotionExploded,
    /// Item destroyed in crafting
    ItemDestroyed,
    /// Lose gold to opponent
    LoseGold(u32),
    /// Take damage
    TakeDamage(u32),
    /// Trigger alarm (stealth games)
    AlarmTriggered,
    /// Trap activates
    TrapActivated,
    /// Creature flees
    CreatureFled,
    /// Reputation loss
    ReputationLoss(u32),
}

// ============================================================================
// Resource Types
// ============================================================================

/// Types of water for fishing
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WaterType {
    Pond,
    River,
    Lake,
    Ocean,
    Magical,
}

impl WaterType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Pond => "Pond",
            Self::River => "River",
            Self::Lake => "Lake",
            Self::Ocean => "Ocean",
            Self::Magical => "Magical Waters",
        }
    }
}

/// Types of ore for mining
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum OreType {
    Iron,
    Silver,
    Gold,
    Mithril,
    Adamantite,
}

impl OreType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Iron => "Iron",
            Self::Silver => "Silver",
            Self::Gold => "Gold",
            Self::Mithril => "Mithril",
            Self::Adamantite => "Adamantite",
        }
    }
}

// ============================================================================
// Minigame Settings (Accessibility)
// ============================================================================

/// Player settings for minigames
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MinigameSettings {
    /// Allow auto-skipping minigames (with penalty)
    pub allow_auto_skip: bool,
    /// Minigames that are always auto-skipped
    pub auto_skip_types: Vec<MinigameType>,
    /// Use simplified QTE alternatives
    pub use_simplified_controls: bool,
    /// Extend time limits by this percentage
    pub time_extension_percent: u32,
    /// Increase timing tolerance by this percentage
    pub tolerance_increase_percent: u32,
    /// Show visual hints and guides
    pub show_hints: bool,
    /// Use audio cues
    pub audio_cues_enabled: bool,
    /// Colorblind mode
    pub colorblind_mode: bool,
    /// Maximum difficulty (cap all minigames)
    pub max_difficulty_cap: Option<DifficultyLevel>,
    /// Allow unlimited practice attempts
    pub unlimited_practice: bool,
    /// Skip tutorials after first completion
    pub skip_tutorials: bool,
}

impl Default for MinigameSettings {
    fn default() -> Self {
        Self {
            allow_auto_skip: true,
            auto_skip_types: Vec::new(),
            use_simplified_controls: false,
            time_extension_percent: 0,
            tolerance_increase_percent: 0,
            show_hints: true,
            audio_cues_enabled: true,
            colorblind_mode: false,
            max_difficulty_cap: None,
            unlimited_practice: false,
            skip_tutorials: false,
        }
    }
}

impl MinigameSettings {
    /// Check if a minigame should be auto-skipped
    pub fn should_auto_skip(&self, minigame_type: MinigameType) -> bool {
        self.auto_skip_types.contains(&minigame_type)
    }

    /// Get the effective time limit with settings applied
    pub fn effective_time_limit(&self, base_time: u64) -> u64 {
        let extension = base_time * self.time_extension_percent as u64 / 100;
        base_time + extension
    }

    /// Get the effective tolerance with settings applied
    pub fn effective_tolerance(&self, base_tolerance: f32) -> f32 {
        let increase = base_tolerance * self.tolerance_increase_percent as f32 / 100.0;
        base_tolerance + increase
    }

    /// Get the effective difficulty with cap applied
    pub fn effective_difficulty(&self, requested: DifficultyLevel) -> DifficultyLevel {
        if let Some(cap) = self.max_difficulty_cap {
            if requested > cap {
                return cap;
            }
        }
        requested
    }
}

// ============================================================================
// Tournament System
// ============================================================================

/// A minigame tournament
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tournament {
    /// Unique identifier
    pub id: u64,
    /// Tournament name
    pub name: String,
    /// Type of minigame
    pub minigame_type: MinigameType,
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    /// Current round (0-indexed)
    pub current_round: u32,
    /// Total rounds
    pub total_rounds: u32,
    /// Participants (name, score)
    pub participants: Vec<TournamentParticipant>,
    /// Player's current position (1-indexed)
    pub player_position: u32,
    /// Player's total score
    pub player_score: u32,
    /// Entry fee paid
    pub entry_fee: u32,
    /// Prize pool
    pub prize_pool: u32,
    /// Whether the tournament is active
    pub is_active: bool,
    /// Whether the tournament is complete
    pub is_complete: bool,
}

impl Tournament {
    /// Create a new tournament
    pub fn new(
        id: u64,
        name: String,
        minigame_type: MinigameType,
        difficulty: DifficultyLevel,
        entry_fee: u32,
        num_participants: usize,
        rng: &mut impl Rng,
    ) -> Self {
        let mut participants = Vec::new();

        // Generate NPC participants
        let npc_names = [
            "Swift Fingers McGee", "Lucky Lou", "The Champion", "Novice Nick",
            "Veteran Val", "Steady Eddie", "Quick Draw Quinn", "Precise Pat",
            "Rhythm Randy", "Master Marcus", "Expert Emily", "Pro Pete",
        ];

        for i in 0..num_participants.min(npc_names.len()) {
            let skill_level = rng.gen_range(40..95);
            participants.push(TournamentParticipant {
                name: npc_names[i].to_string(),
                is_player: false,
                score: 0,
                rounds_completed: 0,
                eliminated: false,
                skill_level,
            });
        }

        // Add player
        participants.push(TournamentParticipant {
            name: "You".to_string(),
            is_player: true,
            score: 0,
            rounds_completed: 0,
            eliminated: false,
            skill_level: 0, // Player skill is determined by actual performance
        });

        let num_participants = participants.len();
        let prize_pool = entry_fee * num_participants as u32;

        Self {
            id,
            name,
            minigame_type,
            difficulty,
            current_round: 0,
            total_rounds: TOURNAMENT_ROUNDS,
            participants,
            player_position: num_participants as u32,
            player_score: 0,
            entry_fee,
            prize_pool,
            is_active: true,
            is_complete: false,
        }
    }

    /// Process player's round result
    pub fn process_player_round(&mut self, result: &MinigameResult, rng: &mut impl Rng) {
        // Update player score
        let player_idx = self.participants.iter().position(|p| p.is_player).unwrap();
        let round_score = result.performance;
        self.participants[player_idx].score += round_score;
        self.participants[player_idx].rounds_completed += 1;
        self.player_score = self.participants[player_idx].score;

        // Simulate NPC rounds
        for participant in &mut self.participants {
            if !participant.is_player && !participant.eliminated {
                let base_performance = participant.skill_level as u32;
                let variance = rng.gen_range(0..20) as i32 - 10;
                let npc_score = (base_performance as i32 + variance).clamp(0, 100) as u32;
                participant.score += npc_score;
                participant.rounds_completed += 1;
            }
        }

        // Sort by score and update positions
        self.participants.sort_by(|a, b| b.score.cmp(&a.score));

        // Update player position
        self.player_position = self.participants.iter()
            .position(|p| p.is_player)
            .map(|p| p as u32 + 1)
            .unwrap_or(1);

        self.current_round += 1;

        // Eliminate bottom performers each round (except last)
        if self.current_round < self.total_rounds && self.participants.len() > 3 {
            let eliminate_count = 1.max(self.participants.len() / 4);
            let threshold = self.participants.len() - eliminate_count;
            for (i, participant) in self.participants.iter_mut().enumerate() {
                if i >= threshold {
                    participant.eliminated = true;
                }
            }
            self.participants.retain(|p| !p.eliminated || p.is_player);
        }

        // Check if tournament is complete
        if self.current_round >= self.total_rounds {
            self.is_complete = true;
            self.is_active = false;
        }

        // Check if player is eliminated
        let player = self.participants.iter().find(|p| p.is_player).unwrap();
        if player.eliminated {
            self.is_active = false;
        }
    }

    /// Get the tournament prizes based on final position
    pub fn get_prizes(&self) -> TournamentPrize {
        if !self.is_complete {
            return TournamentPrize::empty();
        }

        match self.player_position {
            1 => TournamentPrize {
                gold: self.prize_pool * 60 / 100,
                trophy: Some(TrophyType::Gold),
                special_item: Some((ItemKind::TournamentReward, Rarity::Legendary)),
                title: Some(format!("{} Champion", self.minigame_type.name())),
            },
            2 => TournamentPrize {
                gold: self.prize_pool * 25 / 100,
                trophy: Some(TrophyType::Silver),
                special_item: Some((ItemKind::TournamentReward, Rarity::Epic)),
                title: None,
            },
            3 => TournamentPrize {
                gold: self.prize_pool * 15 / 100,
                trophy: Some(TrophyType::Bronze),
                special_item: Some((ItemKind::TournamentReward, Rarity::Rare)),
                title: None,
            },
            _ => TournamentPrize {
                gold: 0,
                trophy: None,
                special_item: None,
                title: None,
            },
        }
    }

    /// Get the current leaderboard
    pub fn leaderboard(&self) -> Vec<(&TournamentParticipant, u32)> {
        self.participants.iter()
            .enumerate()
            .map(|(i, p)| (p, i as u32 + 1))
            .collect()
    }
}

/// A tournament participant
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TournamentParticipant {
    pub name: String,
    pub is_player: bool,
    pub score: u32,
    pub rounds_completed: u32,
    pub eliminated: bool,
    pub skill_level: u32,
}

/// Tournament prize
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TournamentPrize {
    pub gold: u32,
    pub trophy: Option<TrophyType>,
    pub special_item: Option<(ItemKind, Rarity)>,
    pub title: Option<String>,
}

impl TournamentPrize {
    pub fn empty() -> Self {
        Self {
            gold: 0,
            trophy: None,
            special_item: None,
            title: None,
        }
    }
}

/// Trophy types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TrophyType {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

impl TrophyType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bronze => "Bronze Trophy",
            Self::Silver => "Silver Trophy",
            Self::Gold => "Gold Trophy",
            Self::Platinum => "Platinum Trophy",
            Self::Diamond => "Diamond Trophy",
        }
    }
}

// ============================================================================
// Achievements
// ============================================================================

/// Minigame achievement identifiers
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MinigameAchievementId {
    // General
    FirstMinigame,
    MinigameMaster,
    PerfectScore,
    SpeedDemon,
    NoMistakes,
    ComboKing,

    // Lockpicking
    MasterThief,
    PickedHundredLocks,
    LegendaryLocksmith,

    // Fishing
    FirstCatch,
    BigFish,
    LegendaryAngler,
    CaughtHundredFish,

    // Mining
    StrikingGold,
    GemHunter,
    MiningMaster,

    // Cooking
    MasterChef,
    PerfectMeal,
    FeastForKings,

    // Alchemy
    MasterAlchemist,
    PotionPerfection,
    ExplosiveMistake,

    // Blacksmithing
    MasterSmith,
    LegendaryForge,
    PerfectBlade,

    // Card Games
    CardSharp,
    HighRoller,
    Undefeated,

    // Tournaments
    TournamentWinner,
    GrandChampion,
    UndefeatedChampion,

    // Mastery
    JackOfAllTrades,
    MasterOfAll,
    LegendaryPlayer,
}

impl MinigameAchievementId {
    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::FirstMinigame => "First Steps",
            Self::MinigameMaster => "Minigame Master",
            Self::PerfectScore => "Perfect!",
            Self::SpeedDemon => "Speed Demon",
            Self::NoMistakes => "Flawless",
            Self::ComboKing => "Combo King",
            Self::MasterThief => "Master Thief",
            Self::PickedHundredLocks => "Locksmith",
            Self::LegendaryLocksmith => "Legendary Locksmith",
            Self::FirstCatch => "First Catch",
            Self::BigFish => "Big Fish",
            Self::LegendaryAngler => "Legendary Angler",
            Self::CaughtHundredFish => "Patient Fisher",
            Self::StrikingGold => "Striking Gold",
            Self::GemHunter => "Gem Hunter",
            Self::MiningMaster => "Mining Master",
            Self::MasterChef => "Master Chef",
            Self::PerfectMeal => "Perfect Meal",
            Self::FeastForKings => "Feast for Kings",
            Self::MasterAlchemist => "Master Alchemist",
            Self::PotionPerfection => "Potion Perfection",
            Self::ExplosiveMistake => "Explosive Mistake",
            Self::MasterSmith => "Master Smith",
            Self::LegendaryForge => "Legendary Forge",
            Self::PerfectBlade => "Perfect Blade",
            Self::CardSharp => "Card Sharp",
            Self::HighRoller => "High Roller",
            Self::Undefeated => "Undefeated",
            Self::TournamentWinner => "Tournament Winner",
            Self::GrandChampion => "Grand Champion",
            Self::UndefeatedChampion => "Undefeated Champion",
            Self::JackOfAllTrades => "Jack of All Trades",
            Self::MasterOfAll => "Master of All",
            Self::LegendaryPlayer => "Legendary Player",
        }
    }

    /// Returns the description
    pub fn description(&self) -> &'static str {
        match self {
            Self::FirstMinigame => "Complete your first minigame",
            Self::MinigameMaster => "Complete 100 minigames",
            Self::PerfectScore => "Achieve a perfect score in any minigame",
            Self::SpeedDemon => "Complete a minigame with more than 50% time remaining",
            Self::NoMistakes => "Complete a Hard difficulty minigame with no mistakes",
            Self::ComboKing => "Achieve a 20+ combo in any minigame",
            Self::MasterThief => "Pick 50 locks",
            Self::PickedHundredLocks => "Pick 100 locks",
            Self::LegendaryLocksmith => "Pick a Legendary difficulty lock",
            Self::FirstCatch => "Catch your first fish",
            Self::BigFish => "Catch a rare fish",
            Self::LegendaryAngler => "Catch a legendary fish",
            Self::CaughtHundredFish => "Catch 100 fish",
            Self::StrikingGold => "Mine gold ore",
            Self::GemHunter => "Find a perfect gem while mining",
            Self::MiningMaster => "Mine 100 ore nodes",
            Self::MasterChef => "Prepare 50 meals",
            Self::PerfectMeal => "Prepare a perfect quality meal",
            Self::FeastForKings => "Prepare a Legendary meal",
            Self::MasterAlchemist => "Brew 50 potions",
            Self::PotionPerfection => "Brew a perfect potion",
            Self::ExplosiveMistake => "Have a potion explode",
            Self::MasterSmith => "Forge 50 items",
            Self::LegendaryForge => "Forge a Legendary item",
            Self::PerfectBlade => "Forge a perfect quality weapon",
            Self::CardSharp => "Win 25 card games",
            Self::HighRoller => "Win 1000 gold in gambling minigames",
            Self::Undefeated => "Win 10 card games in a row",
            Self::TournamentWinner => "Win a tournament",
            Self::GrandChampion => "Win 10 tournaments",
            Self::UndefeatedChampion => "Win a tournament without losing a round",
            Self::JackOfAllTrades => "Complete every type of minigame",
            Self::MasterOfAll => "Achieve mastery in every minigame type",
            Self::LegendaryPlayer => "Complete all minigame achievements",
        }
    }

    /// Returns the reward for this achievement
    pub fn reward(&self) -> AchievementReward {
        match self {
            Self::FirstMinigame => AchievementReward { gold: 50, experience: 100, title: None },
            Self::MinigameMaster => AchievementReward { gold: 500, experience: 1000, title: Some("Minigame Master".to_string()) },
            Self::PerfectScore => AchievementReward { gold: 100, experience: 200, title: None },
            Self::SpeedDemon => AchievementReward { gold: 75, experience: 150, title: None },
            Self::NoMistakes => AchievementReward { gold: 150, experience: 300, title: None },
            Self::ComboKing => AchievementReward { gold: 200, experience: 400, title: None },
            Self::MasterThief => AchievementReward { gold: 250, experience: 500, title: Some("Master Thief".to_string()) },
            Self::PickedHundredLocks => AchievementReward { gold: 300, experience: 600, title: None },
            Self::LegendaryLocksmith => AchievementReward { gold: 500, experience: 1000, title: Some("Legendary Locksmith".to_string()) },
            Self::FirstCatch => AchievementReward { gold: 25, experience: 50, title: None },
            Self::BigFish => AchievementReward { gold: 100, experience: 200, title: None },
            Self::LegendaryAngler => AchievementReward { gold: 500, experience: 1000, title: Some("Legendary Angler".to_string()) },
            Self::CaughtHundredFish => AchievementReward { gold: 300, experience: 600, title: None },
            Self::StrikingGold => AchievementReward { gold: 100, experience: 200, title: None },
            Self::GemHunter => AchievementReward { gold: 250, experience: 500, title: None },
            Self::MiningMaster => AchievementReward { gold: 400, experience: 800, title: Some("Mining Master".to_string()) },
            Self::MasterChef => AchievementReward { gold: 300, experience: 600, title: Some("Master Chef".to_string()) },
            Self::PerfectMeal => AchievementReward { gold: 150, experience: 300, title: None },
            Self::FeastForKings => AchievementReward { gold: 500, experience: 1000, title: None },
            Self::MasterAlchemist => AchievementReward { gold: 400, experience: 800, title: Some("Master Alchemist".to_string()) },
            Self::PotionPerfection => AchievementReward { gold: 200, experience: 400, title: None },
            Self::ExplosiveMistake => AchievementReward { gold: 50, experience: 100, title: None },
            Self::MasterSmith => AchievementReward { gold: 400, experience: 800, title: Some("Master Smith".to_string()) },
            Self::LegendaryForge => AchievementReward { gold: 750, experience: 1500, title: None },
            Self::PerfectBlade => AchievementReward { gold: 300, experience: 600, title: None },
            Self::CardSharp => AchievementReward { gold: 250, experience: 500, title: Some("Card Sharp".to_string()) },
            Self::HighRoller => AchievementReward { gold: 500, experience: 1000, title: None },
            Self::Undefeated => AchievementReward { gold: 400, experience: 800, title: None },
            Self::TournamentWinner => AchievementReward { gold: 500, experience: 1000, title: None },
            Self::GrandChampion => AchievementReward { gold: 2000, experience: 4000, title: Some("Grand Champion".to_string()) },
            Self::UndefeatedChampion => AchievementReward { gold: 1000, experience: 2000, title: None },
            Self::JackOfAllTrades => AchievementReward { gold: 500, experience: 1000, title: Some("Jack of All Trades".to_string()) },
            Self::MasterOfAll => AchievementReward { gold: 2500, experience: 5000, title: Some("Master of All".to_string()) },
            Self::LegendaryPlayer => AchievementReward { gold: 5000, experience: 10000, title: Some("Legendary Player".to_string()) },
        }
    }
}

/// Reward for completing an achievement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementReward {
    pub gold: u32,
    pub experience: u32,
    pub title: Option<String>,
}

// ============================================================================
// Player Statistics
// ============================================================================

/// Player's minigame statistics and progress
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct MinigameStats {
    /// Total minigames played
    pub total_played: u32,
    /// Total minigames won
    pub total_won: u32,
    /// Total perfect completions
    pub perfect_completions: u32,
    /// Statistics per minigame type
    pub type_stats: HashMap<MinigameType, TypeStats>,
    /// Skill levels
    pub skill_levels: HashMap<MinigameSkill, u32>,
    /// Skill experience
    pub skill_xp: HashMap<MinigameSkill, u32>,
    /// Achievements unlocked
    pub achievements: Vec<MinigameAchievementId>,
    /// Tournament wins
    pub tournament_wins: u32,
    /// Tournaments participated
    pub tournaments_participated: u32,
    /// Best combo ever
    pub best_combo_ever: u32,
    /// Fastest completion time by type (in ms)
    pub fastest_times: HashMap<MinigameType, u64>,
    /// Total gold earned from minigames
    pub total_gold_earned: u64,
    /// Practice attempts remaining
    pub practice_attempts: u32,
    /// Last practice reset time
    pub last_practice_reset: u64,
    /// Titles earned
    pub titles: Vec<String>,
    /// Trophies earned
    pub trophies: Vec<TrophyType>,
}

impl MinigameStats {
    /// Create new stats
    pub fn new() -> Self {
        let mut stats = Self::default();
        stats.practice_attempts = MAX_PRACTICE_ATTEMPTS;

        // Initialize skill levels
        for skill in [
            MinigameSkill::Dexterity, MinigameSkill::Patience, MinigameSkill::Strength,
            MinigameSkill::Precision, MinigameSkill::Intelligence, MinigameSkill::Luck,
            MinigameSkill::Rhythm, MinigameSkill::Accuracy,
        ] {
            stats.skill_levels.insert(skill, 1);
            stats.skill_xp.insert(skill, 0);
        }

        stats
    }

    /// Record a minigame result
    pub fn record_result(&mut self, result: &MinigameResult, reward: &MinigameReward) {
        self.total_played += 1;

        if result.success {
            self.total_won += 1;
        }

        if result.is_perfect {
            self.perfect_completions += 1;
        }

        if result.best_combo > self.best_combo_ever {
            self.best_combo_ever = result.best_combo;
        }

        // Update type stats
        let type_stats = self.type_stats
            .entry(result.minigame_type)
            .or_insert_with(TypeStats::default);
        type_stats.record(result);

        // Update fastest time
        if result.success {
            let entry = self.fastest_times.entry(result.minigame_type).or_insert(u64::MAX);
            if result.time_taken_ms < *entry {
                *entry = result.time_taken_ms;
            }
        }

        // Add skill XP
        let skill = result.minigame_type.associated_skill();
        self.add_skill_xp(skill, reward.skill_experience);

        // Track gold
        self.total_gold_earned += reward.gold as u64;

        // Check for achievements
        self.check_achievements(result);
    }

    /// Add experience to a skill
    pub fn add_skill_xp(&mut self, skill: MinigameSkill, xp: u32) -> bool {
        let current_xp = self.skill_xp.entry(skill).or_insert(0);
        *current_xp += xp;

        let level = self.skill_levels.entry(skill).or_insert(1);
        let xp_needed = skill.xp_to_level(*level);

        if *current_xp >= xp_needed {
            *current_xp -= xp_needed;
            *level += 1;
            return true; // Leveled up
        }

        false
    }

    /// Get skill level
    pub fn skill_level(&self, skill: MinigameSkill) -> u32 {
        self.skill_levels.get(&skill).copied().unwrap_or(1)
    }

    /// Get skill bonus
    pub fn skill_bonus(&self, skill: MinigameSkill) -> f32 {
        let level = self.skill_level(skill);
        level as f32 * skill.bonus_per_level()
    }

    /// Check if an achievement is unlocked
    pub fn has_achievement(&self, id: MinigameAchievementId) -> bool {
        self.achievements.contains(&id)
    }

    /// Unlock an achievement
    pub fn unlock_achievement(&mut self, id: MinigameAchievementId) -> Option<AchievementReward> {
        if self.has_achievement(id) {
            return None;
        }

        self.achievements.push(id);
        let reward = id.reward();

        if let Some(title) = &reward.title {
            if !self.titles.contains(title) {
                self.titles.push(title.clone());
            }
        }

        Some(reward)
    }

    /// Check and unlock achievements based on result
    fn check_achievements(&mut self, result: &MinigameResult) {
        // First minigame
        if self.total_played == 1 {
            self.unlock_achievement(MinigameAchievementId::FirstMinigame);
        }

        // Minigame master
        if self.total_played >= 100 {
            self.unlock_achievement(MinigameAchievementId::MinigameMaster);
        }

        // Perfect score
        if result.is_perfect {
            self.unlock_achievement(MinigameAchievementId::PerfectScore);
        }

        // Speed demon (more than 50% time remaining)
        let base_time = result.minigame_type.base_time_limit();
        if result.success && result.time_taken_ms < base_time / 2 {
            self.unlock_achievement(MinigameAchievementId::SpeedDemon);
        }

        // No mistakes on hard
        if result.success && result.mistakes == 0 && result.difficulty >= DifficultyLevel::Hard {
            self.unlock_achievement(MinigameAchievementId::NoMistakes);
        }

        // Combo king
        if result.best_combo >= 20 {
            self.unlock_achievement(MinigameAchievementId::ComboKing);
        }

        // Type-specific achievements
        // Clone the type_stats to avoid borrowing self immutably while calling unlock_achievement
        if let Some(type_stats) = self.type_stats.get(&result.minigame_type).cloned() {
            match result.minigame_type {
                MinigameType::Lockpicking => {
                    if type_stats.total_completed >= 50 {
                        self.unlock_achievement(MinigameAchievementId::MasterThief);
                    }
                    if type_stats.total_completed >= 100 {
                        self.unlock_achievement(MinigameAchievementId::PickedHundredLocks);
                    }
                    if result.success && result.difficulty == DifficultyLevel::Legendary {
                        self.unlock_achievement(MinigameAchievementId::LegendaryLocksmith);
                    }
                }
                MinigameType::Fishing => {
                    if type_stats.total_completed == 1 {
                        self.unlock_achievement(MinigameAchievementId::FirstCatch);
                    }
                    if type_stats.total_completed >= 100 {
                        self.unlock_achievement(MinigameAchievementId::CaughtHundredFish);
                    }
                }
                MinigameType::Mining => {
                    if type_stats.total_completed >= 100 {
                        self.unlock_achievement(MinigameAchievementId::MiningMaster);
                    }
                }
                MinigameType::Cooking => {
                    if type_stats.total_completed >= 50 {
                        self.unlock_achievement(MinigameAchievementId::MasterChef);
                    }
                    if result.is_perfect {
                        self.unlock_achievement(MinigameAchievementId::PerfectMeal);
                    }
                }
                MinigameType::Alchemy => {
                    if type_stats.total_completed >= 50 {
                        self.unlock_achievement(MinigameAchievementId::MasterAlchemist);
                    }
                    if result.is_perfect {
                        self.unlock_achievement(MinigameAchievementId::PotionPerfection);
                    }
                    if !result.success {
                        self.unlock_achievement(MinigameAchievementId::ExplosiveMistake);
                    }
                }
                MinigameType::Blacksmithing => {
                    if type_stats.total_completed >= 50 {
                        self.unlock_achievement(MinigameAchievementId::MasterSmith);
                    }
                    if result.is_perfect {
                        self.unlock_achievement(MinigameAchievementId::PerfectBlade);
                    }
                }
                MinigameType::CardGame => {
                    if type_stats.total_won >= 25 {
                        self.unlock_achievement(MinigameAchievementId::CardSharp);
                    }
                    if type_stats.win_streak >= 10 {
                        self.unlock_achievement(MinigameAchievementId::Undefeated);
                    }
                }
                _ => {}
            }
        }

        // Jack of all trades - completed every type
        let all_types = MinigameType::all();
        let completed_types: Vec<_> = self.type_stats.iter()
            .filter(|(_, stats)| stats.total_completed > 0)
            .map(|(t, _)| *t)
            .collect();
        if completed_types.len() >= all_types.len() {
            self.unlock_achievement(MinigameAchievementId::JackOfAllTrades);
        }
    }

    /// Get mastery level for a minigame type (0-5)
    pub fn mastery_level(&self, minigame_type: MinigameType) -> u32 {
        let stats = match self.type_stats.get(&minigame_type) {
            Some(s) => s,
            None => return 0,
        };

        let completed = stats.total_completed;
        let perfects = stats.perfect_count;
        let win_rate = if stats.total_played > 0 {
            stats.total_won as f32 / stats.total_played as f32
        } else {
            0.0
        };

        // Calculate mastery based on multiple factors
        let completion_score = (completed / 20).min(2);
        let perfect_score = (perfects / 5).min(2);
        let rate_score = if win_rate >= 0.9 { 1 } else { 0 };

        (completion_score + perfect_score + rate_score).min(5)
    }

    /// Get win rate
    pub fn win_rate(&self) -> f32 {
        if self.total_played == 0 {
            return 0.0;
        }
        self.total_won as f32 / self.total_played as f32 * 100.0
    }

    /// Use a practice attempt
    pub fn use_practice_attempt(&mut self) -> bool {
        if self.practice_attempts > 0 {
            self.practice_attempts -= 1;
            true
        } else {
            false
        }
    }

    /// Reset practice attempts (called daily)
    pub fn reset_practice_attempts(&mut self) {
        self.practice_attempts = MAX_PRACTICE_ATTEMPTS;
    }
}

/// Statistics for a specific minigame type
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct TypeStats {
    pub total_played: u32,
    pub total_won: u32,
    pub total_completed: u32,
    pub perfect_count: u32,
    pub best_performance: u32,
    pub best_combo: u32,
    pub total_mistakes: u32,
    pub win_streak: u32,
    pub best_win_streak: u32,
    pub highest_difficulty_completed: Option<DifficultyLevel>,
}

impl TypeStats {
    /// Record a result
    pub fn record(&mut self, result: &MinigameResult) {
        self.total_played += 1;

        if result.success {
            self.total_won += 1;
            self.total_completed += 1;
            self.win_streak += 1;

            if self.win_streak > self.best_win_streak {
                self.best_win_streak = self.win_streak;
            }

            if result.performance > self.best_performance {
                self.best_performance = result.performance;
            }

            match &self.highest_difficulty_completed {
                None => self.highest_difficulty_completed = Some(result.difficulty),
                Some(d) if result.difficulty > *d => {
                    self.highest_difficulty_completed = Some(result.difficulty);
                }
                _ => {}
            }
        } else {
            self.win_streak = 0;
        }

        if result.is_perfect {
            self.perfect_count += 1;
        }

        if result.best_combo > self.best_combo {
            self.best_combo = result.best_combo;
        }

        self.total_mistakes += result.mistakes;
    }
}

// ============================================================================
// Minigame System
// ============================================================================

/// The main minigame system
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MinigameSystem {
    /// Current active minigame (if any)
    pub current_game: Option<MinigameState>,
    /// Current context
    pub current_context: Option<MinigameContext>,
    /// Player settings
    pub settings: MinigameSettings,
    /// Player statistics
    pub stats: MinigameStats,
    /// Active tournament (if any)
    pub active_tournament: Option<Tournament>,
    /// Available tournaments
    pub available_tournaments: Vec<Tournament>,
    /// Last tournament ID
    last_tournament_id: u64,
}

impl Default for MinigameSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MinigameSystem {
    /// Create a new minigame system
    pub fn new() -> Self {
        Self {
            current_game: None,
            current_context: None,
            settings: MinigameSettings::default(),
            stats: MinigameStats::new(),
            active_tournament: None,
            available_tournaments: Vec::new(),
            last_tournament_id: 0,
        }
    }

    /// Start a minigame
    pub fn start_minigame(
        &mut self,
        minigame_type: MinigameType,
        context: MinigameContext,
        is_practice: bool,
    ) -> Result<(), String> {
        if self.current_game.is_some() {
            return Err("A minigame is already in progress".to_string());
        }

        // Check if should auto-skip
        if self.settings.should_auto_skip(minigame_type) && !is_practice {
            // Handle auto-skip
            let result = MinigameResult::auto_skipped(minigame_type, DifficultyLevel::Normal);
            let mut rng = rand::thread_rng();
            let reward = MinigameReward::calculate(&result, &context, &mut rng);
            self.stats.record_result(&result, &reward);
            return Ok(());
        }

        // Determine difficulty
        let base_difficulty = minigame_type.base_difficulty();
        let context_difficulty = context.min_difficulty
            .map(|d| d.value())
            .unwrap_or(base_difficulty);
        let effective_difficulty = self.settings.effective_difficulty(
            DifficultyLevel::from_value(context_difficulty)
        );

        // Check practice attempts
        if is_practice && !self.settings.unlimited_practice {
            if self.stats.practice_attempts == 0 {
                return Err("No practice attempts remaining".to_string());
            }
            self.stats.use_practice_attempt();
        }

        let state = MinigameState::new(minigame_type, effective_difficulty, is_practice);
        self.current_game = Some(state);
        self.current_context = Some(context);

        Ok(())
    }

    /// Process input for the current minigame
    pub fn process_input(&mut self, input: MinigameInput) -> Option<InputResult> {
        let game = self.current_game.as_mut()?;
        Some(game.process_input(input))
    }

    /// Update the current minigame (call each frame/tick)
    pub fn update(&mut self, delta_ms: u64) {
        if let Some(game) = &mut self.current_game {
            game.tick(delta_ms);
        }
    }

    /// Complete the current minigame and get results
    pub fn complete_minigame(&mut self) -> Option<(MinigameResult, MinigameReward)> {
        let game = self.current_game.take()?;
        let context = self.current_context.take()?;

        let base_time = game.minigame_type.base_time_limit();
        let result = MinigameResult::from_state(&game, base_time);

        let mut rng = rand::thread_rng();
        let reward = MinigameReward::calculate(&result, &context, &mut rng);

        self.stats.record_result(&result, &reward);

        // Handle tournament round if active
        if let Some(tournament) = &mut self.active_tournament {
            if tournament.is_active {
                tournament.process_player_round(&result, &mut rng);
            }
        }

        Some((result, reward))
    }

    /// Cancel the current minigame
    pub fn cancel_minigame(&mut self) {
        self.current_game = None;
        self.current_context = None;
    }

    /// Check if a minigame is in progress
    pub fn is_minigame_active(&self) -> bool {
        self.current_game.is_some()
    }

    /// Get the current minigame state
    pub fn current_state(&self) -> Option<&MinigameState> {
        self.current_game.as_ref()
    }

    /// Create a new tournament
    pub fn create_tournament(
        &mut self,
        name: String,
        minigame_type: MinigameType,
        difficulty: DifficultyLevel,
        entry_fee: u32,
        num_participants: usize,
    ) -> u64 {
        self.last_tournament_id += 1;
        let mut rng = rand::thread_rng();

        let tournament = Tournament::new(
            self.last_tournament_id,
            name,
            minigame_type,
            difficulty,
            entry_fee,
            num_participants,
            &mut rng,
        );

        self.available_tournaments.push(tournament);
        self.last_tournament_id
    }

    /// Join a tournament
    pub fn join_tournament(&mut self, tournament_id: u64) -> Result<(), String> {
        if self.active_tournament.is_some() {
            return Err("Already in a tournament".to_string());
        }

        let idx = self.available_tournaments.iter()
            .position(|t| t.id == tournament_id)
            .ok_or("Tournament not found")?;

        let tournament = self.available_tournaments.remove(idx);
        self.stats.tournaments_participated += 1;
        self.active_tournament = Some(tournament);

        Ok(())
    }

    /// Start the next tournament round
    pub fn start_tournament_round(&mut self) -> Result<MinigameType, String> {
        let tournament = self.active_tournament.as_ref()
            .ok_or("Not in a tournament")?;

        if !tournament.is_active {
            return Err("Tournament is not active".to_string());
        }

        if tournament.is_complete {
            return Err("Tournament is complete".to_string());
        }

        Ok(tournament.minigame_type)
    }

    /// Complete the active tournament and get prizes
    pub fn complete_tournament(&mut self) -> Option<TournamentPrize> {
        let tournament = self.active_tournament.take()?;

        if !tournament.is_complete {
            return None;
        }

        let prize = tournament.get_prizes();

        if tournament.player_position == 1 {
            self.stats.tournament_wins += 1;
            if let Some(trophy) = prize.trophy {
                self.stats.trophies.push(trophy);
            }

            // Check tournament achievements
            if self.stats.tournament_wins >= 10 {
                self.stats.unlock_achievement(MinigameAchievementId::GrandChampion);
            }
        }

        self.stats.unlock_achievement(MinigameAchievementId::TournamentWinner);

        Some(prize)
    }

    /// Generate random tournaments for the day
    pub fn generate_daily_tournaments(&mut self, dungeon_level: u32, rng: &mut impl Rng) {
        self.available_tournaments.clear();

        let tournament_types: Vec<MinigameType> = MinigameType::all()
            .into_iter()
            .filter(|t| t.supports_tournament())
            .collect();

        let num_tournaments = 2 + (dungeon_level / 10).min(3);

        for _ in 0..num_tournaments {
            let minigame_type = tournament_types[rng.gen_range(0..tournament_types.len())];
            let difficulty = DifficultyLevel::from_value(
                2 + rng.gen_range(0..(dungeon_level / 5).min(8))
            );
            let entry_fee = 50 * (difficulty.value() + 1);
            let num_participants = 4 + rng.gen_range(0..8);

            let names = [
                "Daily Challenge", "Grand Tournament", "Champion's Cup",
                "Master's Trial", "Weekly Showdown", "Elite Competition",
            ];
            let name = format!("{} - {}", names[rng.gen_range(0..names.len())], minigame_type.name());

            self.create_tournament(name, minigame_type, difficulty, entry_fee, num_participants);
        }
    }

    /// Get available tournaments
    pub fn available_tournaments(&self) -> &[Tournament] {
        &self.available_tournaments
    }

    /// Get the active tournament
    pub fn active_tournament(&self) -> Option<&Tournament> {
        self.active_tournament.as_ref()
    }

    /// Get player statistics
    pub fn stats(&self) -> &MinigameStats {
        &self.stats
    }

    /// Get mutable player statistics
    pub fn stats_mut(&mut self) -> &mut MinigameStats {
        &mut self.stats
    }

    /// Get settings
    pub fn settings(&self) -> &MinigameSettings {
        &self.settings
    }

    /// Get mutable settings
    pub fn settings_mut(&mut self) -> &mut MinigameSettings {
        &mut self.settings
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate a minigame context for a locked object
pub fn create_lock_context(lock_quality: Rarity, x: usize, y: usize) -> MinigameContext {
    MinigameContext::locked_chest(lock_quality, x, y)
}

/// Generate a minigame context for fishing
pub fn create_fishing_context(water_type: WaterType, x: usize, y: usize) -> MinigameContext {
    MinigameContext::fishing_spot(water_type, x, y)
}

/// Generate a minigame context for mining
pub fn create_mining_context(ore_type: OreType, x: usize, y: usize) -> MinigameContext {
    MinigameContext::mining_node(ore_type, x, y)
}

/// Get recommended difficulty based on player stats and game context
pub fn recommend_difficulty(
    minigame_type: MinigameType,
    stats: &MinigameStats,
    dungeon_level: u32,
) -> DifficultyLevel {
    let mastery = stats.mastery_level(minigame_type);
    let skill = stats.skill_level(minigame_type.associated_skill());

    // Base on player progression
    let player_score = mastery * 2 + skill;

    // Adjust for dungeon level
    let dungeon_score = dungeon_level / 5;

    let combined = (player_score + dungeon_score) / 2;

    DifficultyLevel::from_value(combined.min(MAX_DIFFICULTY))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minigame_type_properties() {
        for minigame_type in MinigameType::all() {
            assert!(!minigame_type.name().is_empty());
            assert!(!minigame_type.description().is_empty());
            assert!(minigame_type.base_difficulty() <= MAX_DIFFICULTY);
            assert!(minigame_type.base_time_limit() > 0);
        }
    }

    #[test]
    fn test_difficulty_levels() {
        let difficulties = [
            DifficultyLevel::Tutorial,
            DifficultyLevel::Easy,
            DifficultyLevel::Normal,
            DifficultyLevel::Hard,
            DifficultyLevel::Expert,
            DifficultyLevel::Master,
            DifficultyLevel::Legendary,
        ];

        for (i, d) in difficulties.iter().enumerate() {
            if i > 0 {
                assert!(d.value() > difficulties[i - 1].value());
                assert!(d.reward_modifier() > difficulties[i - 1].reward_modifier());
                assert!(d.time_modifier() < difficulties[i - 1].time_modifier());
            }
        }
    }

    #[test]
    fn test_minigame_state_creation() {
        let state = MinigameState::new(
            MinigameType::Lockpicking,
            DifficultyLevel::Normal,
            false,
        );

        assert!(state.is_active);
        assert!(!state.is_paused);
        assert_eq!(state.progress, 0);
        assert_eq!(state.mistakes, 0);
        assert!(state.time_remaining_ms > 0);
    }

    #[test]
    fn test_minigame_state_sequence() {
        let mut state = MinigameState::new(
            MinigameType::Herbalism,
            DifficultyLevel::Easy,
            false,
        );

        state.generate_sequence(4);
        assert_eq!(state.expected_sequence.len(), 4);

        // Test correct inputs
        for input in state.expected_sequence.clone() {
            let result = state.process_input(input);
            assert!(matches!(result, InputResult::Correct | InputResult::PhaseComplete));
        }
    }

    #[test]
    fn test_minigame_settings() {
        let mut settings = MinigameSettings::default();

        assert!(settings.allow_auto_skip);
        assert!(!settings.should_auto_skip(MinigameType::Fishing));

        settings.auto_skip_types.push(MinigameType::Fishing);
        assert!(settings.should_auto_skip(MinigameType::Fishing));

        settings.time_extension_percent = 50;
        assert_eq!(settings.effective_time_limit(1000), 1500);
    }

    #[test]
    fn test_minigame_stats() {
        let mut stats = MinigameStats::new();

        assert_eq!(stats.total_played, 0);
        assert_eq!(stats.skill_level(MinigameSkill::Dexterity), 1);

        let result = MinigameResult {
            minigame_type: MinigameType::Lockpicking,
            difficulty: DifficultyLevel::Normal,
            success: true,
            performance: 85,
            is_perfect: false,
            time_taken_ms: 15000,
            mistakes: 1,
            best_combo: 5,
            was_practice: false,
            was_auto_skipped: false,
        };

        let reward = MinigameReward::empty();
        stats.record_result(&result, &reward);

        assert_eq!(stats.total_played, 1);
        assert_eq!(stats.total_won, 1);
    }

    #[test]
    fn test_tournament_creation() {
        let mut rng = rand::thread_rng();
        let tournament = Tournament::new(
            1,
            "Test Tournament".to_string(),
            MinigameType::Fishing,
            DifficultyLevel::Normal,
            100,
            8,
            &mut rng,
        );

        assert_eq!(tournament.id, 1);
        assert!(tournament.is_active);
        assert!(!tournament.is_complete);
        assert!(tournament.participants.len() > 1);
        assert!(tournament.participants.iter().any(|p| p.is_player));
    }

    #[test]
    fn test_minigame_system() {
        let mut system = MinigameSystem::new();

        assert!(!system.is_minigame_active());

        let context = MinigameContext::default_for(MinigameType::Mining);
        system.start_minigame(MinigameType::Mining, context, false).unwrap();

        assert!(system.is_minigame_active());

        // Complete with success
        if let Some(game) = &mut system.current_game {
            game.progress = 100;
            game.current_phase = game.total_phases;
        }

        let (result, _reward) = system.complete_minigame().unwrap();
        assert!(result.success);
        assert!(!system.is_minigame_active());
    }

    #[test]
    fn test_achievement_unlocking() {
        let mut stats = MinigameStats::new();

        assert!(!stats.has_achievement(MinigameAchievementId::FirstMinigame));

        let reward = stats.unlock_achievement(MinigameAchievementId::FirstMinigame);
        assert!(reward.is_some());
        assert!(stats.has_achievement(MinigameAchievementId::FirstMinigame));

        // Should not unlock twice
        let reward2 = stats.unlock_achievement(MinigameAchievementId::FirstMinigame);
        assert!(reward2.is_none());
    }

    #[test]
    fn test_skill_leveling() {
        let mut stats = MinigameStats::new();

        let initial_level = stats.skill_level(MinigameSkill::Dexterity);
        assert_eq!(initial_level, 1);

        // Add enough XP to level up
        let xp_needed = MinigameSkill::Dexterity.xp_to_level(1);
        let leveled = stats.add_skill_xp(MinigameSkill::Dexterity, xp_needed);

        assert!(leveled);
        assert_eq!(stats.skill_level(MinigameSkill::Dexterity), 2);
    }

    #[test]
    fn test_reward_calculation() {
        let result = MinigameResult {
            minigame_type: MinigameType::Fishing,
            difficulty: DifficultyLevel::Hard,
            success: true,
            performance: 90,
            is_perfect: true,
            time_taken_ms: 20000,
            mistakes: 0,
            best_combo: 10,
            was_practice: false,
            was_auto_skipped: false,
        };

        let context = MinigameContext::default_for(MinigameType::Fishing);
        let mut rng = rand::thread_rng();
        let reward = MinigameReward::calculate(&result, &context, &mut rng);

        assert!(reward.gold > 0);
        assert!(reward.experience > 0);
        assert!(reward.skill_experience > 0);
    }

    #[test]
    fn test_type_stats() {
        let mut type_stats = TypeStats::default();

        let result = MinigameResult {
            minigame_type: MinigameType::Lockpicking,
            difficulty: DifficultyLevel::Normal,
            success: true,
            performance: 100,
            is_perfect: true,
            time_taken_ms: 10000,
            mistakes: 0,
            best_combo: 15,
            was_practice: false,
            was_auto_skipped: false,
        };

        type_stats.record(&result);

        assert_eq!(type_stats.total_played, 1);
        assert_eq!(type_stats.total_won, 1);
        assert_eq!(type_stats.perfect_count, 1);
        assert_eq!(type_stats.best_performance, 100);
        assert_eq!(type_stats.best_combo, 15);
    }

    #[test]
    fn test_input_mechanics() {
        for minigame_type in MinigameType::all() {
            let mechanic = minigame_type.input_mechanic();
            assert!(!mechanic.name().is_empty());
            assert!(!mechanic.description().is_empty());
        }
    }

    #[test]
    fn test_context_creation() {
        let chest_context = MinigameContext::locked_chest(Rarity::Rare, 10, 10);
        assert!(chest_context.min_difficulty.is_some());
        assert!(chest_context.failure_consequence.is_some());

        let fishing_context = MinigameContext::fishing_spot(WaterType::Lake, 5, 5);
        assert!(fishing_context.base_gold_reward > 0);
        assert!(!fishing_context.possible_items.is_empty());

        let mining_context = MinigameContext::mining_node(OreType::Gold, 3, 3);
        assert!(mining_context.base_gold_reward > 0);
    }

    #[test]
    fn test_difficulty_from_value() {
        assert_eq!(DifficultyLevel::from_value(0), DifficultyLevel::Tutorial);
        assert_eq!(DifficultyLevel::from_value(1), DifficultyLevel::Tutorial);
        assert_eq!(DifficultyLevel::from_value(4), DifficultyLevel::Normal);
        assert_eq!(DifficultyLevel::from_value(10), DifficultyLevel::Legendary);
        assert_eq!(DifficultyLevel::from_value(100), DifficultyLevel::Legendary);
    }

    #[test]
    fn test_mastery_level() {
        let mut stats = MinigameStats::new();

        // Initially no mastery
        assert_eq!(stats.mastery_level(MinigameType::Fishing), 0);

        // Add some completions
        let type_stats = stats.type_stats.entry(MinigameType::Fishing).or_default();
        type_stats.total_completed = 50;
        type_stats.perfect_count = 10;
        type_stats.total_played = 55;
        type_stats.total_won = 50;

        assert!(stats.mastery_level(MinigameType::Fishing) > 0);
    }
}

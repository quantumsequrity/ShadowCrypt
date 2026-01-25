//! Corruption and Sanity System
//!
//! This module implements a corruption/sanity mechanic that affects gameplay,
//! visual rendering, and introduces madness effects as the player's sanity deteriorates.
//!
//! # Overview
//!
//! Players accumulate corruption through various means:
//! - Encountering eldritch horrors
//! - Using dark magic or cursed items
//! - Exploring corrupted areas
//! - Taking damage from certain enemies
//!
//! As corruption increases, the player experiences:
//! - Visual distortions (hallucinations, color shifts)
//! - Gameplay modifiers (stat changes, random effects)
//! - Madness effects (involuntary actions, perception changes)

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Maximum corruption value before complete madness
pub const MAX_CORRUPTION: i32 = 100;

/// Corruption threshold percentages for each stage
pub const THRESHOLD_UNEASY: i32 = 15;
pub const THRESHOLD_DISTURBED: i32 = 30;
pub const THRESHOLD_UNSTABLE: i32 = 50;
pub const THRESHOLD_FRACTURED: i32 = 70;
pub const THRESHOLD_SHATTERED: i32 = 85;

/// Corruption levels representing stages of mental deterioration
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug, Hash)]
pub enum CorruptionLevel {
    /// 0-14%: Normal mental state
    Lucid,
    /// 15-29%: Slight unease, minor visual artifacts
    Uneasy,
    /// 30-49%: Growing paranoia, occasional hallucinations
    Disturbed,
    /// 50-69%: Severe distortions, frequent madness effects
    Unstable,
    /// 70-84%: Near-complete breakdown, constant effects
    Fractured,
    /// 85-100%: Total madness, extreme effects
    Shattered,
}

impl CorruptionLevel {
    /// Determine corruption level from raw corruption value
    pub fn from_corruption(corruption: i32) -> Self {
        let percent = (corruption * 100) / MAX_CORRUPTION.max(1);
        match percent {
            p if p >= THRESHOLD_SHATTERED => CorruptionLevel::Shattered,
            p if p >= THRESHOLD_FRACTURED => CorruptionLevel::Fractured,
            p if p >= THRESHOLD_UNSTABLE => CorruptionLevel::Unstable,
            p if p >= THRESHOLD_DISTURBED => CorruptionLevel::Disturbed,
            p if p >= THRESHOLD_UNEASY => CorruptionLevel::Uneasy,
            _ => CorruptionLevel::Lucid,
        }
    }

    /// Get display name for this corruption level
    pub fn name(&self) -> &'static str {
        match self {
            CorruptionLevel::Lucid => "Lucid",
            CorruptionLevel::Uneasy => "Uneasy",
            CorruptionLevel::Disturbed => "Disturbed",
            CorruptionLevel::Unstable => "Unstable",
            CorruptionLevel::Fractured => "Fractured",
            CorruptionLevel::Shattered => "Shattered",
        }
    }

    /// Get a descriptive flavor text for this state
    pub fn description(&self) -> &'static str {
        match self {
            CorruptionLevel::Lucid => "Your mind is clear and focused.",
            CorruptionLevel::Uneasy => "Something feels wrong. Shadows seem to move at the edge of your vision.",
            CorruptionLevel::Disturbed => "Whispers echo in your mind. Reality wavers.",
            CorruptionLevel::Unstable => "The walls breathe. Faces appear in the darkness.",
            CorruptionLevel::Fractured => "You can no longer tell what is real. The void stares back.",
            CorruptionLevel::Shattered => "THEY ARE EVERYWHERE. THE TRUTH DEVOURS.",
        }
    }

    /// Get color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            CorruptionLevel::Lucid => 2,      // White
            CorruptionLevel::Uneasy => 11,    // Yellow
            CorruptionLevel::Disturbed => 6,  // Dark Yellow/Orange
            CorruptionLevel::Unstable => 13,  // Magenta
            CorruptionLevel::Fractured => 14, // Dark Magenta
            CorruptionLevel::Shattered => 4,  // Dark Red
        }
    }

    /// Get attack modifier percentage for this corruption level
    pub fn attack_modifier(&self) -> i32 {
        match self {
            CorruptionLevel::Lucid => 0,
            CorruptionLevel::Uneasy => 0,
            CorruptionLevel::Disturbed => 5,    // Desperation adds slight power
            CorruptionLevel::Unstable => 10,    // Madness fuels aggression
            CorruptionLevel::Fractured => 15,   // Unhinged violence
            CorruptionLevel::Shattered => 25,   // Berserk strength
        }
    }

    /// Get defense modifier percentage for this corruption level
    pub fn defense_modifier(&self) -> i32 {
        match self {
            CorruptionLevel::Lucid => 0,
            CorruptionLevel::Uneasy => 0,
            CorruptionLevel::Disturbed => -5,   // Distracted
            CorruptionLevel::Unstable => -10,   // Erratic
            CorruptionLevel::Fractured => -15,  // Vulnerable
            CorruptionLevel::Shattered => -20,  // Wide open
        }
    }

    /// Get accuracy modifier (chance to miss) for this corruption level
    pub fn miss_chance(&self) -> f32 {
        match self {
            CorruptionLevel::Lucid => 0.0,
            CorruptionLevel::Uneasy => 0.02,
            CorruptionLevel::Disturbed => 0.05,
            CorruptionLevel::Unstable => 0.10,
            CorruptionLevel::Fractured => 0.15,
            CorruptionLevel::Shattered => 0.20,
        }
    }

    /// Get mana cost modifier percentage for this corruption level
    pub fn mana_cost_modifier(&self) -> i32 {
        match self {
            CorruptionLevel::Lucid => 0,
            CorruptionLevel::Uneasy => 0,
            CorruptionLevel::Disturbed => 10,   // Focus wavers
            CorruptionLevel::Unstable => 20,    // Concentration broken
            CorruptionLevel::Fractured => 30,   // Mind fragmenting
            CorruptionLevel::Shattered => 50,   // Chaos reigns
        }
    }

    /// Get hallucination chance per turn
    pub fn hallucination_chance(&self) -> f32 {
        match self {
            CorruptionLevel::Lucid => 0.0,
            CorruptionLevel::Uneasy => 0.02,
            CorruptionLevel::Disturbed => 0.08,
            CorruptionLevel::Unstable => 0.15,
            CorruptionLevel::Fractured => 0.25,
            CorruptionLevel::Shattered => 0.40,
        }
    }

    /// Get chance for random action override per turn
    pub fn madness_action_chance(&self) -> f32 {
        match self {
            CorruptionLevel::Lucid => 0.0,
            CorruptionLevel::Uneasy => 0.0,
            CorruptionLevel::Disturbed => 0.03,
            CorruptionLevel::Unstable => 0.08,
            CorruptionLevel::Fractured => 0.15,
            CorruptionLevel::Shattered => 0.25,
        }
    }

    /// Get natural corruption decay rate per turn
    pub fn natural_decay_rate(&self) -> i32 {
        match self {
            CorruptionLevel::Lucid => 1,
            CorruptionLevel::Uneasy => 1,
            CorruptionLevel::Disturbed => 0,
            CorruptionLevel::Unstable => 0,
            CorruptionLevel::Fractured => 0,
            CorruptionLevel::Shattered => 0,  // No natural decay at high levels
        }
    }

    /// Returns true if this level causes visual distortions
    pub fn has_visual_distortion(&self) -> bool {
        matches!(
            self,
            CorruptionLevel::Disturbed
                | CorruptionLevel::Unstable
                | CorruptionLevel::Fractured
                | CorruptionLevel::Shattered
        )
    }

    /// Returns true if this level can cause spontaneous damage
    pub fn can_cause_self_harm(&self) -> bool {
        matches!(
            self,
            CorruptionLevel::Fractured | CorruptionLevel::Shattered
        )
    }
}

/// Visual distortion effects that alter how the game world appears
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Hash)]
pub enum VisualDistortion {
    /// No distortion
    None,
    /// Slight color shifts and flickering
    ColorShift,
    /// Phantom shadows appear to move
    ShadowFlicker,
    /// Walls appear to pulse or breathe
    WallsPulse,
    /// Enemies appear as different creatures
    EnemyMorphing,
    /// Items appear as other items
    ItemMirage,
    /// The map layout appears to shift
    SpatialDistortion,
    /// Everything becomes inverted/negative
    InvertedVision,
    /// Blood seeps from walls
    BleedingWalls,
    /// Faces appear in surfaces
    FacesInWalls,
    /// Text becomes scrambled/eldritch
    TextCorruption,
    /// The player sees their own corpse
    DeathVision,
}

impl VisualDistortion {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            VisualDistortion::None => "None",
            VisualDistortion::ColorShift => "Color Shift",
            VisualDistortion::ShadowFlicker => "Flickering Shadows",
            VisualDistortion::WallsPulse => "Breathing Walls",
            VisualDistortion::EnemyMorphing => "Shapeshifting Horrors",
            VisualDistortion::ItemMirage => "Illusory Items",
            VisualDistortion::SpatialDistortion => "Spatial Anomaly",
            VisualDistortion::InvertedVision => "Inverted Reality",
            VisualDistortion::BleedingWalls => "Bleeding Walls",
            VisualDistortion::FacesInWalls => "Watching Faces",
            VisualDistortion::TextCorruption => "Eldritch Script",
            VisualDistortion::DeathVision => "Death Premonition",
        }
    }

    /// Get intensity (0.0 - 1.0) for rendering
    pub fn intensity(&self) -> f32 {
        match self {
            VisualDistortion::None => 0.0,
            VisualDistortion::ColorShift => 0.2,
            VisualDistortion::ShadowFlicker => 0.3,
            VisualDistortion::WallsPulse => 0.4,
            VisualDistortion::EnemyMorphing => 0.5,
            VisualDistortion::ItemMirage => 0.4,
            VisualDistortion::SpatialDistortion => 0.6,
            VisualDistortion::InvertedVision => 0.7,
            VisualDistortion::BleedingWalls => 0.6,
            VisualDistortion::FacesInWalls => 0.8,
            VisualDistortion::TextCorruption => 0.5,
            VisualDistortion::DeathVision => 1.0,
        }
    }

    /// Generate a random distortion appropriate for the corruption level
    pub fn random_for_level(level: CorruptionLevel, rng: &mut impl Rng) -> Self {
        let distortions: Vec<Self> = match level {
            CorruptionLevel::Lucid => vec![VisualDistortion::None],
            CorruptionLevel::Uneasy => vec![
                VisualDistortion::None,
                VisualDistortion::ColorShift,
                VisualDistortion::ShadowFlicker,
            ],
            CorruptionLevel::Disturbed => vec![
                VisualDistortion::ColorShift,
                VisualDistortion::ShadowFlicker,
                VisualDistortion::WallsPulse,
                VisualDistortion::ItemMirage,
            ],
            CorruptionLevel::Unstable => vec![
                VisualDistortion::WallsPulse,
                VisualDistortion::EnemyMorphing,
                VisualDistortion::ItemMirage,
                VisualDistortion::SpatialDistortion,
                VisualDistortion::BleedingWalls,
            ],
            CorruptionLevel::Fractured => vec![
                VisualDistortion::EnemyMorphing,
                VisualDistortion::SpatialDistortion,
                VisualDistortion::InvertedVision,
                VisualDistortion::BleedingWalls,
                VisualDistortion::FacesInWalls,
                VisualDistortion::TextCorruption,
            ],
            CorruptionLevel::Shattered => vec![
                VisualDistortion::SpatialDistortion,
                VisualDistortion::InvertedVision,
                VisualDistortion::FacesInWalls,
                VisualDistortion::TextCorruption,
                VisualDistortion::DeathVision,
            ],
        };
        distortions[rng.gen_range(0..distortions.len())]
    }
}

/// Madness effects that can trigger during gameplay
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Hash)]
pub enum MadnessEffect {
    /// Player moves in a random direction
    RandomMovement,
    /// Player attacks in a random direction (including empty space)
    WildSwing,
    /// Player becomes briefly paralyzed with fear
    Paralysis,
    /// Player screams, alerting nearby enemies
    Scream,
    /// Player drops a random item
    DropItem,
    /// Player uses a random skill
    WildMagic,
    /// Player hears whispers (generates fake messages)
    Whispers,
    /// Player sees phantom enemies
    PhantomEnemies,
    /// Player takes psychic damage
    PsychicDamage,
    /// Player heals slightly (the void sustains its vessels)
    VoidSustenance,
    /// Player temporarily gains dark power
    DarkEmpowerment,
    /// Player becomes confused, inverting controls
    InvertedControls,
    /// Player forgets their surroundings (fog of war resets)
    MemoryLapse,
    /// Player's equipment temporarily unequips
    EquipmentMalfunction,
    /// Player laughs maniacally (skip turn)
    ManiacalLaughter,
}

impl MadnessEffect {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            MadnessEffect::RandomMovement => "Compulsive Movement",
            MadnessEffect::WildSwing => "Wild Swing",
            MadnessEffect::Paralysis => "Paralyzed with Fear",
            MadnessEffect::Scream => "Maddened Scream",
            MadnessEffect::DropItem => "Butterfingers",
            MadnessEffect::WildMagic => "Wild Magic Surge",
            MadnessEffect::Whispers => "The Whispers",
            MadnessEffect::PhantomEnemies => "Phantom Terrors",
            MadnessEffect::PsychicDamage => "Mind Fracture",
            MadnessEffect::VoidSustenance => "Void Sustenance",
            MadnessEffect::DarkEmpowerment => "Dark Empowerment",
            MadnessEffect::InvertedControls => "Disorientation",
            MadnessEffect::MemoryLapse => "Memory Lapse",
            MadnessEffect::EquipmentMalfunction => "Equipment Malfunction",
            MadnessEffect::ManiacalLaughter => "Maniacal Laughter",
        }
    }

    /// Get a message to display when this effect triggers
    pub fn trigger_message(&self) -> &'static str {
        match self {
            MadnessEffect::RandomMovement => "Your legs move on their own!",
            MadnessEffect::WildSwing => "You lash out at shadows!",
            MadnessEffect::Paralysis => "Terror grips you! You cannot move!",
            MadnessEffect::Scream => "A scream tears from your throat!",
            MadnessEffect::DropItem => "Your hands tremble uncontrollably!",
            MadnessEffect::WildMagic => "Magic erupts from within you!",
            MadnessEffect::Whispers => "Do you hear them too? The whispers...",
            MadnessEffect::PhantomEnemies => "THEY'RE EVERYWHERE! Wait... nothing's there.",
            MadnessEffect::PsychicDamage => "Your mind tears at itself!",
            MadnessEffect::VoidSustenance => "The darkness... it nourishes you.",
            MadnessEffect::DarkEmpowerment => "Eldritch power courses through you!",
            MadnessEffect::InvertedControls => "Which way is up? Left? RIGHT?!",
            MadnessEffect::MemoryLapse => "Where... where am I?",
            MadnessEffect::EquipmentMalfunction => "Your gear feels wrong, alien...",
            MadnessEffect::ManiacalLaughter => "Ha... haha... HAHAHAHA!",
        }
    }

    /// Is this effect beneficial?
    pub fn is_beneficial(&self) -> bool {
        matches!(
            self,
            MadnessEffect::VoidSustenance | MadnessEffect::DarkEmpowerment
        )
    }

    /// Is this effect harmful?
    pub fn is_harmful(&self) -> bool {
        matches!(
            self,
            MadnessEffect::RandomMovement
                | MadnessEffect::WildSwing
                | MadnessEffect::Paralysis
                | MadnessEffect::Scream
                | MadnessEffect::DropItem
                | MadnessEffect::PsychicDamage
                | MadnessEffect::InvertedControls
                | MadnessEffect::MemoryLapse
                | MadnessEffect::EquipmentMalfunction
                | MadnessEffect::ManiacalLaughter
        )
    }

    /// Generate a random madness effect appropriate for the corruption level
    pub fn random_for_level(level: CorruptionLevel, rng: &mut impl Rng) -> Self {
        let effects: Vec<Self> = match level {
            CorruptionLevel::Lucid | CorruptionLevel::Uneasy => vec![],
            CorruptionLevel::Disturbed => vec![
                MadnessEffect::Whispers,
                MadnessEffect::RandomMovement,
                MadnessEffect::Paralysis,
            ],
            CorruptionLevel::Unstable => vec![
                MadnessEffect::Whispers,
                MadnessEffect::RandomMovement,
                MadnessEffect::WildSwing,
                MadnessEffect::Paralysis,
                MadnessEffect::PhantomEnemies,
                MadnessEffect::PsychicDamage,
                MadnessEffect::VoidSustenance,
            ],
            CorruptionLevel::Fractured => vec![
                MadnessEffect::RandomMovement,
                MadnessEffect::WildSwing,
                MadnessEffect::Scream,
                MadnessEffect::DropItem,
                MadnessEffect::WildMagic,
                MadnessEffect::PhantomEnemies,
                MadnessEffect::PsychicDamage,
                MadnessEffect::VoidSustenance,
                MadnessEffect::DarkEmpowerment,
                MadnessEffect::InvertedControls,
                MadnessEffect::MemoryLapse,
            ],
            CorruptionLevel::Shattered => vec![
                MadnessEffect::RandomMovement,
                MadnessEffect::WildSwing,
                MadnessEffect::Scream,
                MadnessEffect::DropItem,
                MadnessEffect::WildMagic,
                MadnessEffect::PsychicDamage,
                MadnessEffect::VoidSustenance,
                MadnessEffect::DarkEmpowerment,
                MadnessEffect::InvertedControls,
                MadnessEffect::MemoryLapse,
                MadnessEffect::EquipmentMalfunction,
                MadnessEffect::ManiacalLaughter,
            ],
        };

        if effects.is_empty() {
            MadnessEffect::Whispers // Fallback
        } else {
            effects[rng.gen_range(0..effects.len())]
        }
    }
}

/// Sources of corruption gain
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Hash)]
pub enum CorruptionSource {
    /// Damage from eldritch enemies
    EldritchDamage,
    /// Using dark magic or cursed items
    DarkMagicUse,
    /// Entering corrupted areas
    CorruptedArea,
    /// Reading forbidden texts
    ForbiddenKnowledge,
    /// Killing innocent creatures
    InnocentBlood,
    /// Making dark pacts
    DarkPact,
    /// Being in darkness too long
    ProlongedDarkness,
    /// Consuming corrupted food/items
    CorruptedConsumption,
    /// Witnessing horrific events
    Trauma,
    /// Failed sanity check
    FailedSanityCheck,
    /// Artifact curse
    CursedArtifact,
    /// Boss encounter
    BossPresence,
}

impl CorruptionSource {
    /// Get the base corruption amount for this source
    pub fn base_corruption(&self) -> i32 {
        match self {
            CorruptionSource::EldritchDamage => 2,
            CorruptionSource::DarkMagicUse => 5,
            CorruptionSource::CorruptedArea => 1,
            CorruptionSource::ForbiddenKnowledge => 8,
            CorruptionSource::InnocentBlood => 10,
            CorruptionSource::DarkPact => 15,
            CorruptionSource::ProlongedDarkness => 1,
            CorruptionSource::CorruptedConsumption => 6,
            CorruptionSource::Trauma => 4,
            CorruptionSource::FailedSanityCheck => 3,
            CorruptionSource::CursedArtifact => 12,
            CorruptionSource::BossPresence => 5,
        }
    }

    /// Get display message for this corruption source
    pub fn message(&self) -> &'static str {
        match self {
            CorruptionSource::EldritchDamage => "Eldritch energies seep into your mind!",
            CorruptionSource::DarkMagicUse => "Dark power exacts its toll.",
            CorruptionSource::CorruptedArea => "This place is wrong. It taints you.",
            CorruptionSource::ForbiddenKnowledge => "The knowledge burns into your psyche!",
            CorruptionSource::InnocentBlood => "Their blood stains your soul.",
            CorruptionSource::DarkPact => "The pact is sealed. Part of you is lost.",
            CorruptionSource::ProlongedDarkness => "The darkness whispers...",
            CorruptionSource::CorruptedConsumption => "Something foul spreads within you.",
            CorruptionSource::Trauma => "The horror etches itself into your memory.",
            CorruptionSource::FailedSanityCheck => "Your grip on reality weakens.",
            CorruptionSource::CursedArtifact => "The artifact's curse seeps into you!",
            CorruptionSource::BossPresence => "Its mere presence corrupts your mind!",
        }
    }
}

/// Ways to reduce corruption
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Hash)]
pub enum PurificationMethod {
    /// Rest at a shrine
    ShrineBlessing,
    /// Consume purifying potion
    PurifyingPotion,
    /// Divine intervention
    DivineIntervention,
    /// Meditation (skip turns)
    Meditation,
    /// Exposure to holy light
    HolyLight,
    /// Confession/repentance (drop cursed items)
    Repentance,
    /// Natural recovery over time
    NaturalRecovery,
    /// Defeating a boss
    BossVictory,
}

impl PurificationMethod {
    /// Get the base purification amount
    pub fn base_purification(&self) -> i32 {
        match self {
            PurificationMethod::ShrineBlessing => 15,
            PurificationMethod::PurifyingPotion => 25,
            PurificationMethod::DivineIntervention => 50,
            PurificationMethod::Meditation => 5,
            PurificationMethod::HolyLight => 10,
            PurificationMethod::Repentance => 20,
            PurificationMethod::NaturalRecovery => 1,
            PurificationMethod::BossVictory => 30,
        }
    }

    /// Get display message for this purification
    pub fn message(&self) -> &'static str {
        match self {
            PurificationMethod::ShrineBlessing => "The shrine's blessing cleanses your mind.",
            PurificationMethod::PurifyingPotion => "The potion burns away the corruption!",
            PurificationMethod::DivineIntervention => "Divine light floods your being!",
            PurificationMethod::Meditation => "Inner peace slowly returns.",
            PurificationMethod::HolyLight => "The light pushes back the darkness.",
            PurificationMethod::Repentance => "You feel lighter, cleansed.",
            PurificationMethod::NaturalRecovery => "Your mind slowly recovers.",
            PurificationMethod::BossVictory => "Defeating the evil lifts a weight from your soul.",
        }
    }
}

/// Whisper messages that play during corruption events
pub const WHISPER_MESSAGES: &[&str] = &[
    "Join us...",
    "You cannot escape.",
    "We see you.",
    "The void awaits.",
    "Your soul is ours.",
    "Everything ends.",
    "There is no hope.",
    "Embrace the darkness.",
    "Reality is a lie.",
    "We are eternal.",
    "You are already dead.",
    "The truth will destroy you.",
    "They are watching.",
    "Your friends are not real.",
    "This world is a prison.",
    "Let go.",
    "The abyss hungers.",
    "You invited us in.",
    "There is no escape.",
    "Madness is freedom.",
    "Pain is temporary. We are forever.",
    "Do you remember who you were?",
    "The walls have eyes.",
    "Behind you...",
    "IT COMES.",
];

/// The main corruption state tracker for a player
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CorruptionState {
    /// Current corruption value (0 - MAX_CORRUPTION)
    pub corruption: i32,
    /// Active visual distortions
    pub active_distortions: Vec<VisualDistortion>,
    /// Currently active madness effects
    pub active_madness: HashSet<MadnessEffect>,
    /// Turns since last corruption gain
    pub turns_since_corruption: u32,
    /// Total corruption gained this session
    pub total_corruption_gained: i32,
    /// Total corruption purified this session
    pub total_corruption_purified: i32,
    /// Number of madness episodes experienced
    pub madness_episodes: u32,
    /// Whether controls are currently inverted
    pub controls_inverted: bool,
    /// Turns remaining with inverted controls
    pub inversion_turns: u32,
    /// Phantom enemies currently visible (positions)
    pub phantom_positions: Vec<(usize, usize)>,
    /// Current empowerment bonus (from dark empowerment)
    pub dark_power_bonus: i32,
    /// Turns remaining for dark empowerment
    pub dark_power_turns: u32,
    /// Accumulated meditation turns
    pub meditation_progress: u32,
    /// Corruption resistance (reduces gain)
    pub corruption_resistance: i32,
    /// Corruption affinity (increases benefits of high corruption)
    pub corruption_affinity: i32,
}

impl Default for CorruptionState {
    fn default() -> Self {
        Self::new()
    }
}

impl CorruptionState {
    /// Create a new corruption state
    pub fn new() -> Self {
        Self {
            corruption: 0,
            active_distortions: Vec::new(),
            active_madness: HashSet::new(),
            turns_since_corruption: 0,
            total_corruption_gained: 0,
            total_corruption_purified: 0,
            madness_episodes: 0,
            controls_inverted: false,
            inversion_turns: 0,
            phantom_positions: Vec::new(),
            dark_power_bonus: 0,
            dark_power_turns: 0,
            meditation_progress: 0,
            corruption_resistance: 0,
            corruption_affinity: 0,
        }
    }

    /// Get the current corruption level
    pub fn level(&self) -> CorruptionLevel {
        CorruptionLevel::from_corruption(self.corruption)
    }

    /// Get corruption as a percentage (0-100)
    pub fn percentage(&self) -> i32 {
        (self.corruption * 100) / MAX_CORRUPTION.max(1)
    }

    /// Add corruption from a specific source
    pub fn add_corruption(&mut self, source: CorruptionSource) -> i32 {
        let base = source.base_corruption();
        let modified = (base - self.corruption_resistance).max(1);
        self.corruption = (self.corruption + modified).min(MAX_CORRUPTION);
        self.total_corruption_gained += modified;
        self.turns_since_corruption = 0;
        modified
    }

    /// Add raw corruption amount
    pub fn add_raw_corruption(&mut self, amount: i32) -> i32 {
        let modified = (amount - self.corruption_resistance).max(0);
        self.corruption = (self.corruption + modified).min(MAX_CORRUPTION);
        self.total_corruption_gained += modified;
        self.turns_since_corruption = 0;
        modified
    }

    /// Reduce corruption through purification
    pub fn purify(&mut self, method: PurificationMethod) -> i32 {
        let amount = method.base_purification();
        self.corruption = (self.corruption - amount).max(0);
        self.total_corruption_purified += amount;
        amount
    }

    /// Reduce corruption by a raw amount
    pub fn reduce_raw_corruption(&mut self, amount: i32) -> i32 {
        let actual = amount.min(self.corruption);
        self.corruption -= actual;
        self.total_corruption_purified += actual;
        actual
    }

    /// Process a turn of corruption effects
    /// Returns messages to display and the triggered madness effect (if any)
    pub fn tick(&mut self, rng: &mut impl Rng) -> (Vec<String>, Option<MadnessEffect>) {
        let mut messages = Vec::new();
        let level = self.level();

        self.turns_since_corruption += 1;

        // Natural decay for low corruption levels
        if self.turns_since_corruption >= 10 {
            let decay = level.natural_decay_rate();
            if decay > 0 && self.corruption > 0 {
                self.corruption = (self.corruption - decay).max(0);
            }
        }

        // Update visual distortions
        self.active_distortions.clear();
        if level.has_visual_distortion() && rng.gen_bool(level.hallucination_chance() as f64) {
            let distortion = VisualDistortion::random_for_level(level, rng);
            if distortion != VisualDistortion::None {
                self.active_distortions.push(distortion);
                messages.push(format!("Visual disturbance: {}", distortion.name()));
            }
        }

        // Tick inverted controls
        if self.inversion_turns > 0 {
            self.inversion_turns -= 1;
            if self.inversion_turns == 0 {
                self.controls_inverted = false;
                messages.push("Your sense of direction returns.".to_string());
            }
        }

        // Tick dark empowerment
        if self.dark_power_turns > 0 {
            self.dark_power_turns -= 1;
            if self.dark_power_turns == 0 {
                self.dark_power_bonus = 0;
                messages.push("The dark power fades.".to_string());
            }
        }

        // Clear old phantom positions periodically
        if rng.gen_bool(0.3) && !self.phantom_positions.is_empty() {
            self.phantom_positions.clear();
        }

        // Check for madness effect trigger
        let madness_effect = if rng.gen_bool(level.madness_action_chance() as f64) {
            let effect = MadnessEffect::random_for_level(level, rng);
            self.active_madness.insert(effect);
            self.madness_episodes += 1;
            messages.push(effect.trigger_message().to_string());
            Some(effect)
        } else {
            None
        };

        // Random whispers at high corruption
        if level >= CorruptionLevel::Disturbed && rng.gen_bool(0.1) {
            let whisper = WHISPER_MESSAGES[rng.gen_range(0..WHISPER_MESSAGES.len())];
            messages.push(format!("You hear a whisper: \"{}\"", whisper));
        }

        (messages, madness_effect)
    }

    /// Apply a triggered madness effect
    /// Returns (hp_change, should_skip_turn, random_direction)
    pub fn apply_madness_effect(
        &mut self,
        effect: MadnessEffect,
        rng: &mut impl Rng,
    ) -> (i32, bool, Option<(i32, i32)>) {
        let mut hp_change = 0;
        let mut skip_turn = false;
        let mut random_direction = None;

        match effect {
            MadnessEffect::RandomMovement => {
                let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)];
                random_direction = Some(dirs[rng.gen_range(0..8)]);
            }
            MadnessEffect::WildSwing => {
                let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                random_direction = Some(dirs[rng.gen_range(0..4)]);
            }
            MadnessEffect::Paralysis => {
                skip_turn = true;
            }
            MadnessEffect::Scream => {
                // Alert enemies is handled by caller
            }
            MadnessEffect::DropItem => {
                // Item dropping is handled by caller
            }
            MadnessEffect::WildMagic => {
                // Skill use is handled by caller
            }
            MadnessEffect::Whispers => {
                // Just flavor, already handled in tick
            }
            MadnessEffect::PhantomEnemies => {
                // Generate phantom positions
                self.phantom_positions.clear();
                let count = rng.gen_range(1..=4);
                for _ in 0..count {
                    let x = rng.gen_range(0..80);
                    let y = rng.gen_range(0..50);
                    self.phantom_positions.push((x, y));
                }
            }
            MadnessEffect::PsychicDamage => {
                hp_change = -(rng.gen_range(3..=10));
            }
            MadnessEffect::VoidSustenance => {
                hp_change = rng.gen_range(2..=6);
            }
            MadnessEffect::DarkEmpowerment => {
                self.dark_power_bonus = rng.gen_range(5..=15);
                self.dark_power_turns = rng.gen_range(5..=15);
            }
            MadnessEffect::InvertedControls => {
                self.controls_inverted = true;
                self.inversion_turns = rng.gen_range(5..=15);
            }
            MadnessEffect::MemoryLapse => {
                // FOV reset is handled by caller
            }
            MadnessEffect::EquipmentMalfunction => {
                // Equipment unequip is handled by caller
            }
            MadnessEffect::ManiacalLaughter => {
                skip_turn = true;
            }
        }

        self.active_madness.remove(&effect);
        (hp_change, skip_turn, random_direction)
    }

    /// Get total attack modifier from corruption (including dark empowerment)
    pub fn attack_modifier(&self) -> i32 {
        let level_mod = self.level().attack_modifier();
        let affinity_bonus = if self.corruption_affinity > 0 {
            (level_mod * self.corruption_affinity) / 100
        } else {
            0
        };
        level_mod + affinity_bonus + self.dark_power_bonus
    }

    /// Get total defense modifier from corruption
    pub fn defense_modifier(&self) -> i32 {
        self.level().defense_modifier()
    }

    /// Check if an attack should miss due to corruption
    pub fn should_miss(&self, rng: &mut impl Rng) -> bool {
        rng.gen_bool(self.level().miss_chance() as f64)
    }

    /// Get mana cost multiplier (1.0 = normal)
    pub fn mana_cost_multiplier(&self) -> f32 {
        1.0 + (self.level().mana_cost_modifier() as f32 / 100.0)
    }

    /// Apply inverted controls if active
    pub fn apply_inversion(&self, dx: i32, dy: i32) -> (i32, i32) {
        if self.controls_inverted {
            (-dx, -dy)
        } else {
            (dx, dy)
        }
    }

    /// Check if there are phantom enemies at a position
    pub fn has_phantom_at(&self, x: usize, y: usize) -> bool {
        self.phantom_positions.contains(&(x, y))
    }

    /// Get a corrupted version of a character for display
    pub fn corrupt_glyph(&self, original: char, rng: &mut impl Rng) -> char {
        let level = self.level();
        if level < CorruptionLevel::Disturbed {
            return original;
        }

        let corruption_chance = match level {
            CorruptionLevel::Disturbed => 0.05,
            CorruptionLevel::Unstable => 0.10,
            CorruptionLevel::Fractured => 0.20,
            CorruptionLevel::Shattered => 0.35,
            _ => 0.0,
        };

        if rng.gen_bool(corruption_chance) {
            let corrupted_glyphs = ['?', '!', '%', '&', '$', '@', '#', '*', '^', '~'];
            corrupted_glyphs[rng.gen_range(0..corrupted_glyphs.len())]
        } else {
            original
        }
    }

    /// Get a corrupted version of text for display
    pub fn corrupt_text(&self, text: &str, rng: &mut impl Rng) -> String {
        let level = self.level();
        if level < CorruptionLevel::Fractured {
            return text.to_string();
        }

        let corruption_chance = match level {
            CorruptionLevel::Fractured => 0.1,
            CorruptionLevel::Shattered => 0.25,
            _ => 0.0,
        };

        text.chars()
            .map(|c| {
                if c.is_alphabetic() && rng.gen_bool(corruption_chance) {
                    let zalgo = ['̷', '̸', '̵', '̶', '̴', '̡', '̢', '̛', '̜', '̝'];
                    let z = zalgo[rng.gen_range(0..zalgo.len())];
                    format!("{}{}", c, z)
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    /// Process meditation (call when player skips turn intentionally)
    pub fn meditate(&mut self) -> Option<String> {
        self.meditation_progress += 1;
        if self.meditation_progress >= 5 {
            self.meditation_progress = 0;
            let reduced = self.purify(PurificationMethod::Meditation);
            if reduced > 0 {
                return Some(format!("Meditation cleanses {} corruption.", reduced));
            }
        }
        None
    }

    /// Reset meditation progress (called when player takes other actions)
    pub fn break_meditation(&mut self) {
        self.meditation_progress = 0;
    }

    /// Check if player is at maximum corruption
    pub fn is_max_corruption(&self) -> bool {
        self.corruption >= MAX_CORRUPTION
    }

    /// Check if player has any active distortions
    pub fn has_active_distortions(&self) -> bool {
        !self.active_distortions.is_empty()
    }

    /// Get summary statistics
    pub fn stats_summary(&self) -> CorruptionStats {
        CorruptionStats {
            current: self.corruption,
            maximum: MAX_CORRUPTION,
            level: self.level(),
            total_gained: self.total_corruption_gained,
            total_purified: self.total_corruption_purified,
            madness_episodes: self.madness_episodes,
            resistance: self.corruption_resistance,
            affinity: self.corruption_affinity,
        }
    }
}

/// Statistics about corruption for UI display
#[derive(Clone, Debug)]
pub struct CorruptionStats {
    pub current: i32,
    pub maximum: i32,
    pub level: CorruptionLevel,
    pub total_gained: i32,
    pub total_purified: i32,
    pub madness_episodes: u32,
    pub resistance: i32,
    pub affinity: i32,
}

impl CorruptionStats {
    /// Format for display
    pub fn display(&self) -> String {
        format!(
            "{}/{} ({}) - {} episodes",
            self.current,
            self.maximum,
            self.level.name(),
            self.madness_episodes
        )
    }
}

/// Corruption-related item effects
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum CorruptionItemEffect {
    /// Reduces corruption when used
    Purifying(i32),
    /// Increases corruption when used
    Corrupting(i32),
    /// Grants corruption resistance
    Resistance(i32),
    /// Grants corruption affinity
    Affinity(i32),
    /// Protects from corruption gain for X turns
    Protection(u32),
    /// Causes sanity check
    SanityCheck,
}

/// Enemy types that can cause corruption
pub fn is_eldritch_enemy(enemy_name: &str) -> bool {
    let eldritch_names = [
        "Wraith", "Ghost", "Banshee", "Lich", "Shadow", "Void",
        "Abyssal", "Eldritch", "Cosmic", "Nightmare", "Dream",
        "Mind", "Psychic", "Phantom", "Specter", "Horror",
    ];
    eldritch_names.iter().any(|&name| enemy_name.contains(name))
}

/// Calculate corruption damage bonus
pub fn corruption_damage_bonus(attacker_corruption: i32) -> i32 {
    let level = CorruptionLevel::from_corruption(attacker_corruption);
    level.attack_modifier()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corruption_levels() {
        assert_eq!(CorruptionLevel::from_corruption(0), CorruptionLevel::Lucid);
        assert_eq!(CorruptionLevel::from_corruption(15), CorruptionLevel::Uneasy);
        assert_eq!(CorruptionLevel::from_corruption(30), CorruptionLevel::Disturbed);
        assert_eq!(CorruptionLevel::from_corruption(50), CorruptionLevel::Unstable);
        assert_eq!(CorruptionLevel::from_corruption(70), CorruptionLevel::Fractured);
        assert_eq!(CorruptionLevel::from_corruption(85), CorruptionLevel::Shattered);
        assert_eq!(CorruptionLevel::from_corruption(100), CorruptionLevel::Shattered);
    }

    #[test]
    fn test_corruption_state() {
        let mut state = CorruptionState::new();
        assert_eq!(state.corruption, 0);
        assert_eq!(state.level(), CorruptionLevel::Lucid);

        state.add_corruption(CorruptionSource::EldritchDamage);
        assert!(state.corruption > 0);
        assert!(state.total_corruption_gained > 0);
    }

    #[test]
    fn test_purification() {
        let mut state = CorruptionState::new();
        state.corruption = 50;

        let reduced = state.purify(PurificationMethod::ShrineBlessing);
        assert!(reduced > 0);
        assert!(state.corruption < 50);
    }

    #[test]
    fn test_control_inversion() {
        let state = CorruptionState {
            controls_inverted: true,
            ..Default::default()
        };

        let (dx, dy) = state.apply_inversion(1, 0);
        assert_eq!((dx, dy), (-1, 0));
    }

    #[test]
    fn test_eldritch_detection() {
        assert!(is_eldritch_enemy("Abyssal Horror"));
        assert!(is_eldritch_enemy("Shadow Wraith"));
        assert!(!is_eldritch_enemy("Goblin"));
        assert!(!is_eldritch_enemy("Orc"));
    }
}

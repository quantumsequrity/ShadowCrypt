//! Comprehensive Dialogue System
//!
//! A full-featured dialogue system for ShadowCrypt supporting:
//! - Branching dialogue trees with conditions and consequences
//! - Multiple dialogue types (NPC chat, quest, shop, romance, combat, lore, tutorial)
//! - Dialogue effects (reputation, quests, items, area unlocks, events)
//! - Skill checks (persuasion, intimidation, knowledge)
//! - Race/class-specific dialogue options
//! - Past choice callbacks and dialogue history
//! - NPC personality system affecting dialogue tone

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::classes::CharacterClass;
use crate::items::{ItemKind, Rarity};
use crate::species::Species;

// ============================================================================
// Core Identifiers
// ============================================================================

/// Unique identifier for dialogue nodes
pub type DialogueNodeId = String;

/// Unique identifier for NPCs
pub type NpcId = String;

/// Unique identifier for dialogue trees
pub type DialogueTreeId = String;

/// Unique identifier for quests
pub type QuestId = String;

/// Unique identifier for factions
pub type FactionId = String;

/// Unique identifier for knowledge topics
pub type TopicId = String;

/// Unique identifier for areas/locations
pub type AreaId = String;

/// Unique identifier for events
pub type EventId = String;

// ============================================================================
// Dialogue Types
// ============================================================================

/// The type/category of dialogue interaction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialogueType {
    /// Standard NPC conversation
    StandardChat,
    /// Quest-related dialogue (accept, progress, complete)
    QuestDialogue,
    /// Shop/trading interaction
    ShopInteraction,
    /// Romance/relationship dialogue
    RomanceDialogue,
    /// Combat taunts and battle cries
    CombatTaunt,
    /// Lore and world-building exposition
    LoreExposition,
    /// Tutorial and hint dialogue
    TutorialHint,
    /// Greeting dialogue when first meeting
    Greeting,
    /// Farewell dialogue when ending conversation
    Farewell,
    /// Ambient/background chatter
    AmbientChatter,
    /// Critical story dialogue
    StoryDialogue,
    /// Negotiation dialogue (bartering, diplomacy)
    Negotiation,
    /// Interrogation dialogue
    Interrogation,
}

impl DialogueType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::StandardChat => "Chat",
            Self::QuestDialogue => "Quest",
            Self::ShopInteraction => "Shop",
            Self::RomanceDialogue => "Romance",
            Self::CombatTaunt => "Combat",
            Self::LoreExposition => "Lore",
            Self::TutorialHint => "Tutorial",
            Self::Greeting => "Greeting",
            Self::Farewell => "Farewell",
            Self::AmbientChatter => "Ambient",
            Self::StoryDialogue => "Story",
            Self::Negotiation => "Negotiation",
            Self::Interrogation => "Interrogation",
        }
    }

    /// Returns whether this dialogue type should be logged in history
    pub fn should_log(&self) -> bool {
        matches!(
            self,
            Self::QuestDialogue | Self::RomanceDialogue | Self::LoreExposition | Self::StoryDialogue
        )
    }

    /// Returns whether this dialogue type can be bookmarked
    pub fn can_bookmark(&self) -> bool {
        matches!(
            self,
            Self::QuestDialogue | Self::LoreExposition | Self::StoryDialogue | Self::TutorialHint
        )
    }
}

// ============================================================================
// NPC Personality System
// ============================================================================

/// Base personality type affecting dialogue tone and options
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PersonalityType {
    /// Warm, welcoming, helpful
    Friendly,
    /// Antagonistic, rude, threatening
    Hostile,
    /// Enigmatic, cryptic, secretive
    Mysterious,
    /// Humorous, witty, playful
    Comedic,
    /// Sorrowful, melancholic, regretful
    Tragic,
    /// Calm, balanced, reasonable
    #[default]
    Neutral,
    /// Arrogant, proud, condescending
    Arrogant,
    /// Timid, anxious, fearful
    Fearful,
    /// Scholarly, precise, analytical
    Scholarly,
    /// Zealous, devoted, fanatical
    Zealous,
    /// Greedy, opportunistic, mercantile
    Mercenary,
    /// Noble, honorable, righteous
    Noble,
}

impl PersonalityType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Friendly => "Friendly",
            Self::Hostile => "Hostile",
            Self::Mysterious => "Mysterious",
            Self::Comedic => "Comedic",
            Self::Tragic => "Tragic",
            Self::Neutral => "Neutral",
            Self::Arrogant => "Arrogant",
            Self::Fearful => "Fearful",
            Self::Scholarly => "Scholarly",
            Self::Zealous => "Zealous",
            Self::Mercenary => "Mercenary",
            Self::Noble => "Noble",
        }
    }

    /// Returns greeting style text prefix based on personality
    pub fn greeting_style(&self) -> &'static str {
        match self {
            Self::Friendly => "warmly greets you",
            Self::Hostile => "glares at you",
            Self::Mysterious => "studies you silently",
            Self::Comedic => "grins mischievously",
            Self::Tragic => "looks up with weary eyes",
            Self::Neutral => "acknowledges you",
            Self::Arrogant => "barely glances your way",
            Self::Fearful => "nervously looks around",
            Self::Scholarly => "peers at you curiously",
            Self::Zealous => "regards you with fervent eyes",
            Self::Mercenary => "eyes your coin pouch",
            Self::Noble => "nods respectfully",
        }
    }

    /// Returns price modifier for shop interactions (1.0 = normal)
    pub fn price_modifier(&self) -> f32 {
        match self {
            Self::Friendly => 0.9,
            Self::Hostile => 1.3,
            Self::Mercenary => 1.2,
            Self::Fearful => 0.85,
            Self::Noble => 1.0,
            _ => 1.0,
        }
    }

    /// Returns how likely this personality is to reveal information
    pub fn information_willingness(&self) -> f32 {
        match self {
            Self::Friendly => 0.9,
            Self::Scholarly => 0.95,
            Self::Comedic => 0.7,
            Self::Mysterious => 0.3,
            Self::Hostile => 0.2,
            Self::Fearful => 0.5,
            Self::Noble => 0.8,
            _ => 0.6,
        }
    }
}

/// Detailed personality traits that modify dialogue behavior
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PersonalityTraits {
    /// Base personality type
    pub personality_type: PersonalityType,
    /// Tendency to give discounts (0.0 - 1.0)
    pub generosity: f32,
    /// Likelihood to share information (0.0 - 1.0)
    pub openness: f32,
    /// Patience with the player (affects repeat questions)
    pub patience: f32,
    /// How much reputation affects their behavior
    pub reputation_sensitivity: f32,
    /// Preferred topics of conversation
    pub preferred_topics: Vec<TopicId>,
    /// Disliked topics (may refuse to discuss)
    pub disliked_topics: Vec<TopicId>,
    /// Special speech patterns or quirks
    pub speech_quirks: Vec<SpeechQuirk>,
}

impl PersonalityTraits {
    pub fn new(personality_type: PersonalityType) -> Self {
        let (generosity, openness, patience, reputation_sensitivity) = match personality_type {
            PersonalityType::Friendly => (0.7, 0.8, 0.9, 0.5),
            PersonalityType::Hostile => (0.1, 0.2, 0.2, 0.8),
            PersonalityType::Mysterious => (0.3, 0.2, 0.6, 0.3),
            PersonalityType::Comedic => (0.5, 0.7, 0.8, 0.4),
            PersonalityType::Tragic => (0.4, 0.5, 0.7, 0.6),
            PersonalityType::Neutral => (0.5, 0.5, 0.5, 0.5),
            PersonalityType::Arrogant => (0.2, 0.3, 0.3, 0.9),
            PersonalityType::Fearful => (0.6, 0.4, 0.4, 0.7),
            PersonalityType::Scholarly => (0.4, 0.9, 0.6, 0.3),
            PersonalityType::Zealous => (0.3, 0.6, 0.4, 0.2),
            PersonalityType::Mercenary => (0.1, 0.4, 0.5, 0.6),
            PersonalityType::Noble => (0.6, 0.7, 0.7, 0.8),
        };

        Self {
            personality_type,
            generosity,
            openness,
            patience,
            reputation_sensitivity,
            preferred_topics: Vec::new(),
            disliked_topics: Vec::new(),
            speech_quirks: Vec::new(),
        }
    }

    pub fn with_quirk(mut self, quirk: SpeechQuirk) -> Self {
        self.speech_quirks.push(quirk);
        self
    }

    pub fn with_preferred_topic(mut self, topic: TopicId) -> Self {
        self.preferred_topics.push(topic);
        self
    }

    pub fn with_disliked_topic(mut self, topic: TopicId) -> Self {
        self.disliked_topics.push(topic);
        self
    }
}

/// Speech patterns and quirks that make NPCs distinctive
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeechQuirk {
    /// Speaks in third person
    ThirdPerson,
    /// Uses archaic language
    ArchaicSpeech,
    /// Frequently uses a catchphrase
    Catchphrase(String),
    /// Stutters or hesitates
    Stutter,
    /// Speaks formally
    Formal,
    /// Uses slang and casual speech
    Casual,
    /// Frequently sighs or pauses
    Melancholic,
    /// Speaks in rhymes
    Rhyming,
    /// Uses excessive flattery
    Flattering,
    /// Makes constant jokes
    Punster,
    /// Speaks in riddles
    Riddling,
    /// Uses military terminology
    Military,
    /// Speaks with an accent (described)
    Accent(String),
}

// ============================================================================
// Dialogue Conditions
// ============================================================================

/// Conditions that must be met for dialogue options to appear
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DialogueCondition {
    /// Player must have minimum stats
    HasStats {
        strength: Option<i32>,
        intelligence: Option<i32>,
        charisma: Option<i32>,
        dexterity: Option<i32>,
    },
    /// Player must have specific item(s)
    HasItem {
        item_kind: ItemKind,
        rarity: Option<Rarity>,
        quantity: u32,
    },
    /// Player must have minimum gold
    HasGold(u32),
    /// Player must have minimum reputation with faction
    HasReputation {
        faction: FactionId,
        min_value: i32,
    },
    /// Player must be specific class
    IsClass(CharacterClass),
    /// Player must be specific species
    IsSpecies(Species),
    /// Player level requirement
    MinLevel(u32),
    /// Quest must be in specific state
    QuestState {
        quest_id: QuestId,
        state: QuestStateRequirement,
    },
    /// Player must have learned specific knowledge
    HasKnowledge(TopicId),
    /// Time of day requirement
    TimeOfDay(TimeRequirement),
    /// Previous choice must have been made
    PreviousChoice {
        dialogue_tree_id: DialogueTreeId,
        choice_id: String,
    },
    /// NPC disposition must be at certain level
    NpcDisposition {
        npc_id: NpcId,
        min_value: i32,
    },
    /// Area must be unlocked/visited
    AreaUnlocked(AreaId),
    /// Event must have occurred
    EventOccurred(EventId),
    /// Player must not have item (inverse of HasItem)
    NotHasItem(ItemKind),
    /// Combination of conditions (all must be true)
    All(Vec<DialogueCondition>),
    /// Any of the conditions (at least one must be true)
    Any(Vec<DialogueCondition>),
    /// Negation of a condition
    Not(Box<DialogueCondition>),
    /// Random chance (0.0 - 1.0)
    RandomChance(f32),
    /// Number of times player has talked to this NPC
    ConversationCount {
        npc_id: NpcId,
        min_count: u32,
    },
    /// Skill check requirement (displays as skill check in UI)
    SkillCheck {
        skill: SkillCheckType,
        difficulty: i32,
    },
}

/// Types of skill checks that can appear in dialogue
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillCheckType {
    /// Convincing through charm and logic
    Persuasion,
    /// Threatening or coercing
    Intimidation,
    /// Lying convincingly
    Deception,
    /// Noticing hidden details
    Perception,
    /// Recalling learned information
    Knowledge,
    /// Understanding magical concepts
    Arcana,
    /// Understanding nature and beasts
    Nature,
    /// Understanding religion and divine
    Religion,
    /// Understanding history and lore
    History,
    /// Physical feats of strength
    Strength,
    /// Physical feats of agility
    Dexterity,
    /// Mental fortitude
    Willpower,
    /// Picking locks, sleight of hand
    Thievery,
    /// Medical knowledge
    Medicine,
    /// Survival and tracking
    Survival,
}

impl SkillCheckType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Persuasion => "Persuasion",
            Self::Intimidation => "Intimidation",
            Self::Deception => "Deception",
            Self::Perception => "Perception",
            Self::Knowledge => "Knowledge",
            Self::Arcana => "Arcana",
            Self::Nature => "Nature",
            Self::Religion => "Religion",
            Self::History => "History",
            Self::Strength => "Strength",
            Self::Dexterity => "Dexterity",
            Self::Willpower => "Willpower",
            Self::Thievery => "Thievery",
            Self::Medicine => "Medicine",
            Self::Survival => "Survival",
        }
    }

    /// Returns the display color for this skill type
    pub fn color(&self) -> &'static str {
        match self {
            Self::Persuasion | Self::Deception => "yellow",
            Self::Intimidation | Self::Strength => "red",
            Self::Perception | Self::Survival => "green",
            Self::Knowledge | Self::History => "blue",
            Self::Arcana | Self::Willpower => "purple",
            Self::Nature | Self::Medicine => "cyan",
            Self::Religion => "white",
            Self::Dexterity | Self::Thievery => "orange",
        }
    }
}

/// Quest state requirements for conditions
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStateRequirement {
    /// Quest not yet started
    NotStarted,
    /// Quest is active
    Active,
    /// Quest is complete but not turned in
    Completed,
    /// Quest has been turned in
    TurnedIn,
    /// Quest has been failed
    Failed,
    /// Quest is at specific stage
    AtStage(u32),
}

/// Time of day requirements
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeRequirement {
    Day,
    Night,
    Dawn,
    Dusk,
    Midnight,
    Noon,
    /// Specific hour range (0-23)
    HourRange { start: u8, end: u8 },
}

// ============================================================================
// Dialogue Effects
// ============================================================================

/// Effects/consequences that occur when selecting a dialogue option
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DialogueEffect {
    /// Modify reputation with a faction
    ModifyReputation {
        faction: FactionId,
        amount: i32,
    },
    /// Start a quest
    StartQuest {
        quest_id: QuestId,
        stage: Option<u32>,
    },
    /// Advance quest to next stage
    AdvanceQuest {
        quest_id: QuestId,
        to_stage: u32,
    },
    /// Complete a quest
    CompleteQuest(QuestId),
    /// Fail a quest
    FailQuest(QuestId),
    /// Give item to player
    GiveItem {
        item_kind: ItemKind,
        rarity: Rarity,
        quantity: u32,
    },
    /// Take item from player
    TakeItem {
        item_kind: ItemKind,
        quantity: u32,
    },
    /// Give gold to player
    GiveGold(u32),
    /// Take gold from player
    TakeGold(u32),
    /// Unlock an area
    UnlockArea(AreaId),
    /// Trigger a game event
    TriggerEvent(EventId),
    /// Change NPC behavior/state
    ChangeNpcBehavior {
        npc_id: NpcId,
        new_behavior: NpcBehaviorChange,
    },
    /// Modify NPC disposition towards player
    ModifyDisposition {
        npc_id: NpcId,
        amount: i32,
    },
    /// Learn knowledge/topic
    LearnKnowledge(TopicId),
    /// Heal player
    HealPlayer {
        hp: Option<i32>,
        mp: Option<i32>,
        full_restore: bool,
    },
    /// Damage player
    DamagePlayer {
        hp: i32,
        damage_type: Option<String>,
    },
    /// Grant experience
    GiveExperience(u32),
    /// Apply status effect
    ApplyStatus {
        effect_name: String,
        duration: u32,
    },
    /// Remove status effect
    RemoveStatus(String),
    /// Open shop interface
    OpenShop {
        shop_id: String,
        discount_percent: Option<u32>,
    },
    /// Teleport player
    TeleportPlayer {
        area_id: AreaId,
        x: Option<usize>,
        y: Option<usize>,
    },
    /// Start combat
    StartCombat {
        enemy_ids: Vec<String>,
    },
    /// End dialogue immediately
    EndDialogue,
    /// Play sound effect
    PlaySound(String),
    /// Show visual effect
    ShowVisual(String),
    /// Record this choice for future reference
    RecordChoice {
        choice_id: String,
    },
    /// Multiple effects in sequence
    Multiple(Vec<DialogueEffect>),
    /// Conditional effect
    Conditional {
        condition: Box<DialogueCondition>,
        if_true: Box<DialogueEffect>,
        if_false: Option<Box<DialogueEffect>>,
    },
    /// Advance romance relationship
    AdvanceRomance {
        npc_id: NpcId,
        stage: RomanceStage,
    },
    /// Teach skill or ability
    TeachSkill(String),
    /// Reveal map area
    RevealMap {
        area_id: Option<AreaId>,
        radius: Option<u32>,
    },
}

/// Types of NPC behavior changes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcBehaviorChange {
    /// NPC becomes hostile
    BecomeHostile,
    /// NPC becomes friendly
    BecomeFriendly,
    /// NPC becomes neutral
    BecomeNeutral,
    /// NPC moves to new location
    MoveTo { area_id: AreaId, x: usize, y: usize },
    /// NPC disappears
    Despawn,
    /// NPC follows player
    FollowPlayer,
    /// NPC stops following player
    StopFollowing,
    /// NPC becomes a merchant
    BecomeMerchant(String),
    /// NPC becomes a quest giver
    BecomeQuestGiver(Vec<QuestId>),
    /// Custom behavior state
    Custom(String),
}

/// Romance relationship stages
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RomanceStage {
    /// Not interested / strangers
    None,
    /// Initial interest
    Acquaintance,
    /// Friendly, getting to know each other
    Friend,
    /// Strong friendship, hints of more
    CloseFriend,
    /// Romantic interest confirmed
    Interested,
    /// Dating / courting
    Courting,
    /// Committed relationship
    Partner,
    /// Deep bond / married
    Soulmate,
}

impl RomanceStage {
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "Stranger",
            Self::Acquaintance => "Acquaintance",
            Self::Friend => "Friend",
            Self::CloseFriend => "Close Friend",
            Self::Interested => "Interested",
            Self::Courting => "Courting",
            Self::Partner => "Partner",
            Self::Soulmate => "Soulmate",
        }
    }

    /// Returns the next stage in progression
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::None => Some(Self::Acquaintance),
            Self::Acquaintance => Some(Self::Friend),
            Self::Friend => Some(Self::CloseFriend),
            Self::CloseFriend => Some(Self::Interested),
            Self::Interested => Some(Self::Courting),
            Self::Courting => Some(Self::Partner),
            Self::Partner => Some(Self::Soulmate),
            Self::Soulmate => None,
        }
    }
}

// ============================================================================
// Dialogue Nodes and Choices
// ============================================================================

/// A single response option in a dialogue node
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueChoice {
    /// Unique identifier for this choice
    pub id: String,
    /// Display text for the choice
    pub text: String,
    /// Short text for display (if text is too long)
    pub short_text: Option<String>,
    /// Tooltip/hover text
    pub tooltip: Option<String>,
    /// Node to go to when selected
    pub next_node: Option<DialogueNodeId>,
    /// Conditions required to show this choice
    pub conditions: Vec<DialogueCondition>,
    /// Effects when this choice is selected
    pub effects: Vec<DialogueEffect>,
    /// Whether this is a skill check choice
    pub skill_check: Option<SkillCheckInfo>,
    /// Priority for ordering (higher = shown first)
    pub priority: i32,
    /// Whether this choice is only for specific classes
    pub class_restriction: Option<Vec<CharacterClass>>,
    /// Whether this choice is only for specific species
    pub species_restriction: Option<Vec<Species>>,
    /// Alternative text if conditions aren't met (shown grayed out)
    pub locked_text: Option<String>,
    /// Whether to show this choice even if locked
    pub show_when_locked: bool,
    /// Tags for categorization
    pub tags: HashSet<String>,
}

impl DialogueChoice {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            short_text: None,
            tooltip: None,
            next_node: None,
            conditions: Vec::new(),
            effects: Vec::new(),
            skill_check: None,
            priority: 0,
            class_restriction: None,
            species_restriction: None,
            locked_text: None,
            show_when_locked: false,
            tags: HashSet::new(),
        }
    }

    /// Set the next node to transition to
    pub fn with_next_node(mut self, node_id: impl Into<DialogueNodeId>) -> Self {
        self.next_node = Some(node_id.into());
        self
    }

    /// Add a condition
    pub fn with_condition(mut self, condition: DialogueCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Add an effect
    pub fn with_effect(mut self, effect: DialogueEffect) -> Self {
        self.effects.push(effect);
        self
    }

    /// Add a skill check
    pub fn with_skill_check(mut self, skill: SkillCheckType, difficulty: i32) -> Self {
        self.skill_check = Some(SkillCheckInfo {
            skill,
            difficulty,
            success_node: None,
            failure_node: None,
            critical_success_node: None,
            critical_failure_node: None,
            success_effects: Vec::new(),
            failure_effects: Vec::new(),
        });
        self
    }

    /// Set class restriction
    pub fn for_class(mut self, class: CharacterClass) -> Self {
        self.class_restriction
            .get_or_insert_with(Vec::new)
            .push(class);
        self
    }

    /// Set species restriction
    pub fn for_species(mut self, species: Species) -> Self {
        self.species_restriction
            .get_or_insert_with(Vec::new)
            .push(species);
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set locked text
    pub fn with_locked_text(mut self, text: impl Into<String>) -> Self {
        self.locked_text = Some(text.into());
        self.show_when_locked = true;
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.insert(tag.into());
        self
    }

    /// Set tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

/// Information about a skill check on a dialogue choice
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillCheckInfo {
    /// Type of skill being checked
    pub skill: SkillCheckType,
    /// Difficulty threshold
    pub difficulty: i32,
    /// Node to go to on success (overrides choice's next_node)
    pub success_node: Option<DialogueNodeId>,
    /// Node to go to on failure
    pub failure_node: Option<DialogueNodeId>,
    /// Node for critical success (rolling much higher)
    pub critical_success_node: Option<DialogueNodeId>,
    /// Node for critical failure (rolling much lower)
    pub critical_failure_node: Option<DialogueNodeId>,
    /// Additional effects on success
    pub success_effects: Vec<DialogueEffect>,
    /// Additional effects on failure
    pub failure_effects: Vec<DialogueEffect>,
}

impl SkillCheckInfo {
    pub fn new(skill: SkillCheckType, difficulty: i32) -> Self {
        Self {
            skill,
            difficulty,
            success_node: None,
            failure_node: None,
            critical_success_node: None,
            critical_failure_node: None,
            success_effects: Vec::new(),
            failure_effects: Vec::new(),
        }
    }

    pub fn with_success_node(mut self, node: impl Into<DialogueNodeId>) -> Self {
        self.success_node = Some(node.into());
        self
    }

    pub fn with_failure_node(mut self, node: impl Into<DialogueNodeId>) -> Self {
        self.failure_node = Some(node.into());
        self
    }
}

/// A single node in the dialogue tree
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueNode {
    /// Unique identifier for this node
    pub id: DialogueNodeId,
    /// The speaking character's name
    pub speaker: String,
    /// Speaker's portrait/image identifier
    pub speaker_portrait: Option<String>,
    /// The dialogue text
    pub text: String,
    /// Alternative text based on personality
    pub personality_variants: HashMap<PersonalityType, String>,
    /// Alternative text based on player class
    pub class_variants: HashMap<CharacterClass, String>,
    /// Alternative text based on player species
    pub species_variants: HashMap<Species, String>,
    /// Available response choices
    pub choices: Vec<DialogueChoice>,
    /// Type of dialogue
    pub dialogue_type: DialogueType,
    /// Effects that trigger when entering this node
    pub on_enter_effects: Vec<DialogueEffect>,
    /// Effects that trigger when leaving this node
    pub on_exit_effects: Vec<DialogueEffect>,
    /// Conditions required to enter this node
    pub entry_conditions: Vec<DialogueCondition>,
    /// If conditions not met, go to this node instead
    pub fallback_node: Option<DialogueNodeId>,
    /// Tags for searching and categorization
    pub tags: HashSet<String>,
    /// Whether this node ends the dialogue
    pub is_terminal: bool,
    /// Voice line identifier
    pub voice_line: Option<String>,
    /// Animation to play
    pub animation: Option<String>,
    /// Background music change
    pub music: Option<String>,
    /// Ambient sound
    pub ambient_sound: Option<String>,
}

impl DialogueNode {
    pub fn new(
        id: impl Into<DialogueNodeId>,
        speaker: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            speaker: speaker.into(),
            speaker_portrait: None,
            text: text.into(),
            personality_variants: HashMap::new(),
            class_variants: HashMap::new(),
            species_variants: HashMap::new(),
            choices: Vec::new(),
            dialogue_type: DialogueType::StandardChat,
            on_enter_effects: Vec::new(),
            on_exit_effects: Vec::new(),
            entry_conditions: Vec::new(),
            fallback_node: None,
            tags: HashSet::new(),
            is_terminal: false,
            voice_line: None,
            animation: None,
            music: None,
            ambient_sound: None,
        }
    }

    /// Add a response choice
    pub fn with_choice(mut self, choice: DialogueChoice) -> Self {
        self.choices.push(choice);
        self
    }

    /// Set dialogue type
    pub fn with_type(mut self, dialogue_type: DialogueType) -> Self {
        self.dialogue_type = dialogue_type;
        self
    }

    /// Add personality variant text
    pub fn with_personality_variant(
        mut self,
        personality: PersonalityType,
        text: impl Into<String>,
    ) -> Self {
        self.personality_variants.insert(personality, text.into());
        self
    }

    /// Add class variant text
    pub fn with_class_variant(
        mut self,
        class: CharacterClass,
        text: impl Into<String>,
    ) -> Self {
        self.class_variants.insert(class, text.into());
        self
    }

    /// Add species variant text
    pub fn with_species_variant(mut self, species: Species, text: impl Into<String>) -> Self {
        self.species_variants.insert(species, text.into());
        self
    }

    /// Add on-enter effect
    pub fn with_enter_effect(mut self, effect: DialogueEffect) -> Self {
        self.on_enter_effects.push(effect);
        self
    }

    /// Add on-exit effect
    pub fn with_exit_effect(mut self, effect: DialogueEffect) -> Self {
        self.on_exit_effects.push(effect);
        self
    }

    /// Add entry condition
    pub fn with_entry_condition(mut self, condition: DialogueCondition) -> Self {
        self.entry_conditions.push(condition);
        self
    }

    /// Set fallback node
    pub fn with_fallback(mut self, node_id: impl Into<DialogueNodeId>) -> Self {
        self.fallback_node = Some(node_id.into());
        self
    }

    /// Mark as terminal (ends dialogue)
    pub fn terminal(mut self) -> Self {
        self.is_terminal = true;
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.insert(tag.into());
        self
    }

    /// Set speaker portrait
    pub fn with_portrait(mut self, portrait: impl Into<String>) -> Self {
        self.speaker_portrait = Some(portrait.into());
        self
    }

    /// Get text with personality/class/species variants applied
    pub fn get_text(
        &self,
        personality: Option<PersonalityType>,
        class: Option<CharacterClass>,
        species: Option<Species>,
    ) -> &str {
        // Priority: Species > Class > Personality > Default
        if let Some(s) = species {
            if let Some(text) = self.species_variants.get(&s) {
                return text;
            }
        }
        if let Some(c) = class {
            if let Some(text) = self.class_variants.get(&c) {
                return text;
            }
        }
        if let Some(p) = personality {
            if let Some(text) = self.personality_variants.get(&p) {
                return text;
            }
        }
        &self.text
    }
}

// ============================================================================
// Dialogue Tree
// ============================================================================

/// A complete dialogue tree containing all nodes for a conversation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueTree {
    /// Unique identifier
    pub id: DialogueTreeId,
    /// Human-readable name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// All nodes in the tree
    pub nodes: HashMap<DialogueNodeId, DialogueNode>,
    /// Starting node ID
    pub root_node: DialogueNodeId,
    /// Alternative starting nodes based on conditions
    pub conditional_roots: Vec<ConditionalRoot>,
    /// Default personality for this dialogue
    pub default_personality: PersonalityType,
    /// Whether this tree can be repeated
    pub repeatable: bool,
    /// Cooldown in game turns before can repeat
    pub cooldown_turns: Option<u32>,
    /// Tags for categorization
    pub tags: HashSet<String>,
    /// Required conditions to start this dialogue at all
    pub start_conditions: Vec<DialogueCondition>,
}

/// Conditional starting node
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalRoot {
    /// Conditions for this root
    pub conditions: Vec<DialogueCondition>,
    /// Node to start at if conditions met
    pub node_id: DialogueNodeId,
    /// Priority (higher = checked first)
    pub priority: i32,
}

impl DialogueTree {
    pub fn new(
        id: impl Into<DialogueTreeId>,
        name: impl Into<String>,
        root_node: DialogueNode,
    ) -> Self {
        let root_id = root_node.id.clone();
        let mut nodes = HashMap::new();
        nodes.insert(root_node.id.clone(), root_node);

        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            nodes,
            root_node: root_id,
            conditional_roots: Vec::new(),
            default_personality: PersonalityType::Neutral,
            repeatable: true,
            cooldown_turns: None,
            tags: HashSet::new(),
            start_conditions: Vec::new(),
        }
    }

    /// Add a node to the tree
    pub fn with_node(mut self, node: DialogueNode) -> Self {
        self.nodes.insert(node.id.clone(), node);
        self
    }

    /// Add multiple nodes
    pub fn with_nodes(mut self, nodes: Vec<DialogueNode>) -> Self {
        for node in nodes {
            self.nodes.insert(node.id.clone(), node);
        }
        self
    }

    /// Set default personality
    pub fn with_personality(mut self, personality: PersonalityType) -> Self {
        self.default_personality = personality;
        self
    }

    /// Add conditional root
    pub fn with_conditional_root(
        mut self,
        conditions: Vec<DialogueCondition>,
        node_id: impl Into<DialogueNodeId>,
        priority: i32,
    ) -> Self {
        self.conditional_roots.push(ConditionalRoot {
            conditions,
            node_id: node_id.into(),
            priority,
        });
        // Sort by priority descending
        self.conditional_roots.sort_by(|a, b| b.priority.cmp(&a.priority));
        self
    }

    /// Set as non-repeatable
    pub fn non_repeatable(mut self) -> Self {
        self.repeatable = false;
        self
    }

    /// Set cooldown
    pub fn with_cooldown(mut self, turns: u32) -> Self {
        self.cooldown_turns = Some(turns);
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.insert(tag.into());
        self
    }

    /// Get node by ID
    pub fn get_node(&self, id: &DialogueNodeId) -> Option<&DialogueNode> {
        self.nodes.get(id)
    }

    /// Get mutable node by ID
    pub fn get_node_mut(&mut self, id: &DialogueNodeId) -> Option<&mut DialogueNode> {
        self.nodes.get_mut(id)
    }
}

// ============================================================================
// Dialogue History
// ============================================================================

/// A logged entry in dialogue history
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueHistoryEntry {
    /// Unique entry ID
    pub id: u64,
    /// When this dialogue occurred (game turn)
    pub game_turn: u64,
    /// Real-world timestamp
    pub timestamp: u64,
    /// NPC involved
    pub npc_id: NpcId,
    /// NPC name (cached for display)
    pub npc_name: String,
    /// Dialogue tree used
    pub tree_id: DialogueTreeId,
    /// Node visited
    pub node_id: DialogueNodeId,
    /// Speaker text
    pub speaker_text: String,
    /// Player's chosen response (if any)
    pub player_response: Option<String>,
    /// Choice ID selected
    pub choice_id: Option<String>,
    /// Type of dialogue
    pub dialogue_type: DialogueType,
    /// Whether this entry is bookmarked
    pub bookmarked: bool,
    /// User notes on this entry
    pub notes: Option<String>,
    /// Tags
    pub tags: HashSet<String>,
    /// Related quest (if any)
    pub related_quest: Option<QuestId>,
    /// Location/area where dialogue occurred
    pub location: Option<AreaId>,
}

impl DialogueHistoryEntry {
    pub fn new(
        id: u64,
        game_turn: u64,
        npc_id: NpcId,
        npc_name: String,
        tree_id: DialogueTreeId,
        node_id: DialogueNodeId,
        speaker_text: String,
        dialogue_type: DialogueType,
    ) -> Self {
        Self {
            id,
            game_turn,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            npc_id,
            npc_name,
            tree_id,
            node_id,
            speaker_text,
            player_response: None,
            choice_id: None,
            dialogue_type,
            bookmarked: false,
            notes: None,
            tags: HashSet::new(),
            related_quest: None,
            location: None,
        }
    }

    /// Set player response
    pub fn with_response(mut self, response: String, choice_id: String) -> Self {
        self.player_response = Some(response);
        self.choice_id = Some(choice_id);
        self
    }

    /// Set bookmark
    pub fn bookmarked(mut self) -> Self {
        self.bookmarked = true;
        self
    }

    /// Add note
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes = Some(note.into());
        self
    }

    /// Set related quest
    pub fn with_quest(mut self, quest_id: QuestId) -> Self {
        self.related_quest = Some(quest_id);
        self
    }

    /// Set location
    pub fn with_location(mut self, location: AreaId) -> Self {
        self.location = Some(location);
        self
    }
}

/// Complete dialogue history for the game
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DialogueHistory {
    /// All history entries
    entries: VecDeque<DialogueHistoryEntry>,
    /// Maximum entries to keep
    max_entries: usize,
    /// Next entry ID
    next_id: u64,
    /// Index of bookmarked entries for quick access
    bookmarked_ids: HashSet<u64>,
    /// Index by NPC
    by_npc: HashMap<NpcId, Vec<u64>>,
    /// Index by quest
    by_quest: HashMap<QuestId, Vec<u64>>,
    /// Index by dialogue type
    by_type: HashMap<DialogueType, Vec<u64>>,
    /// Choices made (for callback lookups)
    choices_made: HashMap<(DialogueTreeId, String), bool>,
}

impl DialogueHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
            next_id: 1,
            bookmarked_ids: HashSet::new(),
            by_npc: HashMap::new(),
            by_quest: HashMap::new(),
            by_type: HashMap::new(),
            choices_made: HashMap::new(),
        }
    }

    /// Add a new entry
    pub fn add_entry(&mut self, mut entry: DialogueHistoryEntry) {
        entry.id = self.next_id;
        self.next_id += 1;

        // Update indices
        self.by_npc
            .entry(entry.npc_id.clone())
            .or_default()
            .push(entry.id);
        if let Some(ref quest) = entry.related_quest {
            self.by_quest
                .entry(quest.clone())
                .or_default()
                .push(entry.id);
        }
        self.by_type
            .entry(entry.dialogue_type)
            .or_default()
            .push(entry.id);

        // Record choice if made
        if let Some(ref choice_id) = entry.choice_id {
            self.choices_made
                .insert((entry.tree_id.clone(), choice_id.clone()), true);
        }

        // Track bookmark
        if entry.bookmarked {
            self.bookmarked_ids.insert(entry.id);
        }

        // Add entry, removing old if necessary
        self.entries.push_back(entry);
        while self.entries.len() > self.max_entries {
            if let Some(removed) = self.entries.pop_front() {
                // Clean up indices
                self.bookmarked_ids.remove(&removed.id);
            }
        }
    }

    /// Get entry by ID
    pub fn get_entry(&self, id: u64) -> Option<&DialogueHistoryEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Get mutable entry by ID
    pub fn get_entry_mut(&mut self, id: u64) -> Option<&mut DialogueHistoryEntry> {
        self.entries.iter_mut().find(|e| e.id == id)
    }

    /// Toggle bookmark on entry
    pub fn toggle_bookmark(&mut self, id: u64) {
        if let Some(entry) = self.get_entry_mut(id) {
            entry.bookmarked = !entry.bookmarked;
            if entry.bookmarked {
                self.bookmarked_ids.insert(id);
            } else {
                self.bookmarked_ids.remove(&id);
            }
        }
    }

    /// Get all bookmarked entries
    pub fn get_bookmarked(&self) -> Vec<&DialogueHistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.bookmarked)
            .collect()
    }

    /// Get entries for a specific NPC
    pub fn get_by_npc(&self, npc_id: &NpcId) -> Vec<&DialogueHistoryEntry> {
        self.by_npc
            .get(npc_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.get_entry(*id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get entries for a specific quest
    pub fn get_by_quest(&self, quest_id: &QuestId) -> Vec<&DialogueHistoryEntry> {
        self.by_quest
            .get(quest_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.get_entry(*id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get entries of a specific type
    pub fn get_by_type(&self, dialogue_type: DialogueType) -> Vec<&DialogueHistoryEntry> {
        self.by_type
            .get(&dialogue_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.get_entry(*id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if a choice was previously made
    pub fn was_choice_made(&self, tree_id: &DialogueTreeId, choice_id: &str) -> bool {
        self.choices_made
            .get(&(tree_id.clone(), choice_id.to_string()))
            .copied()
            .unwrap_or(false)
    }

    /// Search entries by text
    pub fn search(&self, query: &str) -> Vec<&DialogueHistoryEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.speaker_text.to_lowercase().contains(&query_lower)
                    || e.player_response
                        .as_ref()
                        .map(|r| r.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || e.npc_name.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get recent entries
    pub fn get_recent(&self, count: usize) -> Vec<&DialogueHistoryEntry> {
        self.entries.iter().rev().take(count).collect()
    }

    /// Get conversation count with an NPC
    pub fn get_conversation_count(&self, npc_id: &NpcId) -> usize {
        self.by_npc.get(npc_id).map(|v| v.len()).unwrap_or(0)
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.entries.clear();
        self.bookmarked_ids.clear();
        self.by_npc.clear();
        self.by_quest.clear();
        self.by_type.clear();
        // Keep choices_made for game continuity
    }

    /// Get total entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ============================================================================
// Dialogue State
// ============================================================================

/// Current state of an active dialogue
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueState {
    /// The dialogue tree being used
    pub tree_id: DialogueTreeId,
    /// Current node
    pub current_node_id: DialogueNodeId,
    /// NPC being spoken to
    pub npc_id: NpcId,
    /// NPC's personality for this conversation
    pub personality: PersonalityType,
    /// Nodes visited in this conversation
    pub visited_nodes: Vec<DialogueNodeId>,
    /// Choices made in this conversation
    pub choices_made: Vec<(DialogueNodeId, String)>,
    /// Pending effects to execute
    pub pending_effects: Vec<DialogueEffect>,
    /// Whether dialogue is complete
    pub is_complete: bool,
    /// Turn when dialogue started
    pub started_turn: u64,
}

impl DialogueState {
    pub fn new(
        tree_id: DialogueTreeId,
        root_node_id: DialogueNodeId,
        npc_id: NpcId,
        personality: PersonalityType,
        current_turn: u64,
    ) -> Self {
        Self {
            tree_id,
            current_node_id: root_node_id.clone(),
            npc_id,
            personality,
            visited_nodes: vec![root_node_id],
            choices_made: Vec::new(),
            pending_effects: Vec::new(),
            is_complete: false,
            started_turn: current_turn,
        }
    }

    /// Record a choice made
    pub fn record_choice(&mut self, node_id: DialogueNodeId, choice_id: String) {
        self.choices_made.push((node_id, choice_id));
    }

    /// Move to a new node
    pub fn move_to_node(&mut self, node_id: DialogueNodeId) {
        self.current_node_id = node_id.clone();
        self.visited_nodes.push(node_id);
    }

    /// Check if a node has been visited
    pub fn has_visited(&self, node_id: &DialogueNodeId) -> bool {
        self.visited_nodes.contains(node_id)
    }

    /// Add pending effect
    pub fn add_effect(&mut self, effect: DialogueEffect) {
        self.pending_effects.push(effect);
    }

    /// Take pending effects (consuming them)
    pub fn take_effects(&mut self) -> Vec<DialogueEffect> {
        std::mem::take(&mut self.pending_effects)
    }

    /// Complete the dialogue
    pub fn complete(&mut self) {
        self.is_complete = true;
    }
}

// ============================================================================
// Player Dialogue Context
// ============================================================================

/// Player-side context for dialogue condition evaluation
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerDialogueContext {
    /// Player class
    pub class: Option<CharacterClass>,
    /// Player species
    pub species: Option<Species>,
    /// Player level
    pub level: u32,
    /// Player gold
    pub gold: u32,
    /// Player stats
    pub stats: PlayerStats,
    /// Known topics/knowledge
    pub known_topics: HashSet<TopicId>,
    /// Reputation with factions
    pub reputations: HashMap<FactionId, i32>,
    /// Items in inventory (simplified, using string keys for compatibility)
    pub inventory_items: HashMap<String, u32>,
    /// Active quests and their states
    pub quest_states: HashMap<QuestId, QuestStateRequirement>,
    /// Unlocked areas
    pub unlocked_areas: HashSet<AreaId>,
    /// Events that have occurred
    pub occurred_events: HashSet<EventId>,
    /// NPC dispositions
    pub npc_dispositions: HashMap<NpcId, i32>,
    /// Romance stages with NPCs
    pub romance_stages: HashMap<NpcId, RomanceStage>,
    /// Current time of day
    pub time_of_day: TimeRequirement,
    /// Current game turn
    pub current_turn: u64,
    /// Current area
    pub current_area: Option<AreaId>,
}

/// Simplified player stats for dialogue checks
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    pub strength: i32,
    pub intelligence: i32,
    pub charisma: i32,
    pub dexterity: i32,
    pub wisdom: i32,
    pub constitution: i32,
    /// Derived skill values
    pub skills: HashMap<SkillCheckType, i32>,
}

/// Inventory item reference (used instead of ItemKind for HashMap compatibility)
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemRef {
    pub kind: String,
    pub rarity: Option<String>,
}

impl PlayerStats {
    /// Get effective skill value
    pub fn get_skill(&self, skill: SkillCheckType) -> i32 {
        self.skills.get(&skill).copied().unwrap_or_else(|| {
            // Default calculation based on stats
            match skill {
                SkillCheckType::Persuasion => (self.charisma + self.intelligence) / 2,
                SkillCheckType::Intimidation => (self.strength + self.charisma) / 2,
                SkillCheckType::Deception => (self.charisma + self.dexterity) / 2,
                SkillCheckType::Perception => (self.wisdom + self.intelligence) / 2,
                SkillCheckType::Knowledge
                | SkillCheckType::History
                | SkillCheckType::Arcana => self.intelligence,
                SkillCheckType::Nature | SkillCheckType::Survival => self.wisdom,
                SkillCheckType::Religion => (self.wisdom + self.intelligence) / 2,
                SkillCheckType::Strength => self.strength,
                SkillCheckType::Dexterity | SkillCheckType::Thievery => self.dexterity,
                SkillCheckType::Willpower => (self.wisdom + self.constitution) / 2,
                SkillCheckType::Medicine => (self.intelligence + self.wisdom) / 2,
            }
        })
    }
}

impl PlayerDialogueContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set class
    pub fn with_class(mut self, class: CharacterClass) -> Self {
        self.class = Some(class);
        self
    }

    /// Set species
    pub fn with_species(mut self, species: Species) -> Self {
        self.species = Some(species);
        self
    }

    /// Set level
    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    /// Add known topic
    pub fn with_knowledge(mut self, topic: TopicId) -> Self {
        self.known_topics.insert(topic);
        self
    }

    /// Set reputation
    pub fn with_reputation(mut self, faction: FactionId, value: i32) -> Self {
        self.reputations.insert(faction, value);
        self
    }

    /// Add item to inventory (using item name as key)
    pub fn with_item_by_name(mut self, item_name: impl Into<String>, quantity: u32) -> Self {
        let name = item_name.into();
        *self.inventory_items.entry(name).or_insert(0) += quantity;
        self
    }

    /// Add item to inventory using ItemKind
    pub fn with_item(mut self, item: ItemKind, quantity: u32) -> Self {
        let name = format!("{:?}", item);
        *self.inventory_items.entry(name).or_insert(0) += quantity;
        self
    }

    /// Check if player has item by name
    pub fn has_item_by_name(&self, item_name: &str, quantity: u32) -> bool {
        self.inventory_items
            .get(item_name)
            .map(|&q| q >= quantity)
            .unwrap_or(false)
    }

    /// Check if player has item using ItemKind
    pub fn has_item(&self, item: &ItemKind, quantity: u32) -> bool {
        let name = format!("{:?}", item);
        self.inventory_items
            .get(&name)
            .map(|&q| q >= quantity)
            .unwrap_or(false)
    }

    /// Get reputation with faction
    pub fn get_reputation(&self, faction: &FactionId) -> i32 {
        self.reputations.get(faction).copied().unwrap_or(0)
    }

    /// Get NPC disposition
    pub fn get_disposition(&self, npc_id: &NpcId) -> i32 {
        self.npc_dispositions.get(npc_id).copied().unwrap_or(50)
    }

    /// Get romance stage
    pub fn get_romance_stage(&self, npc_id: &NpcId) -> RomanceStage {
        self.romance_stages
            .get(npc_id)
            .copied()
            .unwrap_or(RomanceStage::None)
    }
}

// ============================================================================
// Dialogue System
// ============================================================================

/// Result of a skill check
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillCheckResult {
    /// Type of skill checked
    pub skill: SkillCheckType,
    /// Difficulty target
    pub difficulty: i32,
    /// Player's roll/value
    pub player_value: i32,
    /// Random modifier (if any)
    pub roll_modifier: i32,
    /// Final result
    pub total: i32,
    /// Whether check succeeded
    pub success: bool,
    /// Whether it was a critical result
    pub critical: Option<CriticalResult>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriticalResult {
    CriticalSuccess,
    CriticalFailure,
}

/// Main dialogue system managing all dialogue interactions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueSystem {
    /// All available dialogue trees
    pub trees: HashMap<DialogueTreeId, DialogueTree>,
    /// NPC to dialogue tree mapping
    pub npc_dialogues: HashMap<NpcId, Vec<DialogueTreeId>>,
    /// NPC personalities
    pub npc_personalities: HashMap<NpcId, PersonalityTraits>,
    /// Current active dialogue state (if any)
    pub active_dialogue: Option<DialogueState>,
    /// Dialogue history
    pub history: DialogueHistory,
    /// Last dialogue time per NPC (for cooldowns)
    pub last_dialogue_turn: HashMap<NpcId, u64>,
    /// Global dialogue variables (for complex state tracking)
    pub variables: HashMap<String, DialogueVariable>,
    /// Combat taunts by enemy type
    pub combat_taunts: HashMap<String, Vec<String>>,
    /// Tutorial hints by topic
    pub tutorial_hints: HashMap<String, Vec<DialogueTree>>,
    /// Lore entries by topic
    pub lore_entries: HashMap<TopicId, Vec<DialogueTree>>,
}

/// Variable types for complex dialogue state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DialogueVariable {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<DialogueVariable>),
}

impl Default for DialogueSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogueSystem {
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
            npc_dialogues: HashMap::new(),
            npc_personalities: HashMap::new(),
            active_dialogue: None,
            history: DialogueHistory::new(1000),
            last_dialogue_turn: HashMap::new(),
            variables: HashMap::new(),
            combat_taunts: HashMap::new(),
            tutorial_hints: HashMap::new(),
            lore_entries: HashMap::new(),
        }
    }

    /// Register a dialogue tree
    pub fn register_tree(&mut self, tree: DialogueTree) {
        self.trees.insert(tree.id.clone(), tree);
    }

    /// Assign dialogue tree to NPC
    pub fn assign_dialogue_to_npc(&mut self, npc_id: NpcId, tree_id: DialogueTreeId) {
        self.npc_dialogues
            .entry(npc_id)
            .or_default()
            .push(tree_id);
    }

    /// Set NPC personality
    pub fn set_npc_personality(&mut self, npc_id: NpcId, traits: PersonalityTraits) {
        self.npc_personalities.insert(npc_id, traits);
    }

    /// Get NPC personality
    pub fn get_npc_personality(&self, npc_id: &NpcId) -> Option<&PersonalityTraits> {
        self.npc_personalities.get(npc_id)
    }

    /// Start dialogue with an NPC
    pub fn start_dialogue(
        &mut self,
        npc_id: &NpcId,
        context: &PlayerDialogueContext,
    ) -> Result<&DialogueNode, DialogueError> {
        // Check if already in dialogue
        if self.active_dialogue.is_some() {
            return Err(DialogueError::AlreadyInDialogue);
        }

        // Get NPC's dialogue trees
        let tree_ids = self
            .npc_dialogues
            .get(npc_id)
            .ok_or(DialogueError::NpcNotFound(npc_id.clone()))?;

        // Find appropriate dialogue tree
        let tree_id = self.find_best_tree(tree_ids, context)?;
        let tree = self
            .trees
            .get(&tree_id)
            .ok_or(DialogueError::TreeNotFound(tree_id.clone()))?;

        // Check cooldown
        if let Some(cooldown) = tree.cooldown_turns {
            if let Some(&last_turn) = self.last_dialogue_turn.get(npc_id) {
                if context.current_turn < last_turn + cooldown as u64 {
                    return Err(DialogueError::OnCooldown {
                        remaining: (last_turn + cooldown as u64) - context.current_turn,
                    });
                }
            }
        }

        // Check start conditions
        for condition in &tree.start_conditions {
            if !self.evaluate_condition(condition, context) {
                return Err(DialogueError::ConditionsNotMet);
            }
        }

        // Find starting node (check conditional roots first)
        let start_node_id = self.find_start_node(tree, context);

        // Get personality
        let personality = self
            .npc_personalities
            .get(npc_id)
            .map(|t| t.personality_type)
            .unwrap_or(tree.default_personality);

        // Create dialogue state
        let state = DialogueState::new(
            tree_id.clone(),
            start_node_id.clone(),
            npc_id.clone(),
            personality,
            context.current_turn,
        );

        self.active_dialogue = Some(state);
        self.last_dialogue_turn
            .insert(npc_id.clone(), context.current_turn);

        // Get the starting node
        let tree = self.trees.get(&tree_id).unwrap();
        tree.get_node(&start_node_id)
            .ok_or(DialogueError::NodeNotFound(start_node_id))
    }

    /// Find the best dialogue tree from a list
    fn find_best_tree(
        &self,
        tree_ids: &[DialogueTreeId],
        context: &PlayerDialogueContext,
    ) -> Result<DialogueTreeId, DialogueError> {
        for tree_id in tree_ids {
            if let Some(tree) = self.trees.get(tree_id) {
                // Check if tree's start conditions are met
                let conditions_met = tree
                    .start_conditions
                    .iter()
                    .all(|c| self.evaluate_condition(c, context));

                if conditions_met && (tree.repeatable || !self.has_completed_tree(tree_id)) {
                    return Ok(tree_id.clone());
                }
            }
        }
        Err(DialogueError::NoValidTree)
    }

    /// Find the starting node for a tree
    fn find_start_node(&self, tree: &DialogueTree, context: &PlayerDialogueContext) -> DialogueNodeId {
        // Check conditional roots in priority order
        for conditional in &tree.conditional_roots {
            let all_met = conditional
                .conditions
                .iter()
                .all(|c| self.evaluate_condition(c, context));
            if all_met {
                return conditional.node_id.clone();
            }
        }
        // Fall back to default root
        tree.root_node.clone()
    }

    /// Check if a tree has been completed
    fn has_completed_tree(&self, _tree_id: &DialogueTreeId) -> bool {
        // Would track completed non-repeatable trees
        false
    }

    /// Get current dialogue node
    pub fn get_current_node(&self) -> Option<&DialogueNode> {
        let state = self.active_dialogue.as_ref()?;
        let tree = self.trees.get(&state.tree_id)?;
        tree.get_node(&state.current_node_id)
    }

    /// Get available choices for current node
    pub fn get_available_choices(
        &self,
        context: &PlayerDialogueContext,
    ) -> Vec<AvailableChoice> {
        let state = match &self.active_dialogue {
            Some(s) => s,
            None => return Vec::new(),
        };

        let tree = match self.trees.get(&state.tree_id) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let node = match tree.get_node(&state.current_node_id) {
            Some(n) => n,
            None => return Vec::new(),
        };

        let mut choices: Vec<AvailableChoice> = node
            .choices
            .iter()
            .filter_map(|choice| {
                // Check class restriction
                if let Some(ref classes) = choice.class_restriction {
                    if let Some(player_class) = context.class {
                        if !classes.contains(&player_class) {
                            return None;
                        }
                    }
                }

                // Check species restriction
                if let Some(ref species) = choice.species_restriction {
                    if let Some(player_species) = context.species {
                        if !species.contains(&player_species) {
                            return None;
                        }
                    }
                }

                // Check conditions
                let conditions_met = choice
                    .conditions
                    .iter()
                    .all(|c| self.evaluate_condition(c, context));

                if conditions_met {
                    Some(AvailableChoice {
                        choice: choice.clone(),
                        is_available: true,
                        locked_reason: None,
                        skill_check_preview: choice.skill_check.as_ref().map(|sc| {
                            let player_skill = context.stats.get_skill(sc.skill);
                            SkillCheckPreview {
                                skill: sc.skill,
                                difficulty: sc.difficulty,
                                player_value: player_skill,
                                success_chance: calculate_success_chance(player_skill, sc.difficulty),
                            }
                        }),
                    })
                } else if choice.show_when_locked {
                    Some(AvailableChoice {
                        choice: choice.clone(),
                        is_available: false,
                        locked_reason: choice.locked_text.clone(),
                        skill_check_preview: None,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by priority
        choices.sort_by(|a, b| b.choice.priority.cmp(&a.choice.priority));
        choices
    }

    /// Select a choice and advance dialogue
    pub fn select_choice(
        &mut self,
        choice_index: usize,
        context: &PlayerDialogueContext,
        rng: &mut impl rand::Rng,
    ) -> Result<DialogueSelectResult, DialogueError> {
        let state = self
            .active_dialogue
            .as_mut()
            .ok_or(DialogueError::NotInDialogue)?;

        let tree = self
            .trees
            .get(&state.tree_id)
            .ok_or(DialogueError::TreeNotFound(state.tree_id.clone()))?;

        let node = tree
            .get_node(&state.current_node_id)
            .ok_or(DialogueError::NodeNotFound(state.current_node_id.clone()))?;

        // Get available choices (same filtering as get_available_choices)
        let available = self.get_available_choices(context);
        let selected = available
            .get(choice_index)
            .ok_or(DialogueError::InvalidChoice)?;

        if !selected.is_available {
            return Err(DialogueError::ChoiceLocked);
        }

        let choice = &selected.choice;

        // Handle skill check if present
        let skill_check_result = if let Some(ref skill_check) = choice.skill_check {
            let result = self.perform_skill_check(skill_check, context, rng);
            Some(result)
        } else {
            None
        };

        // Record the choice
        let state = self.active_dialogue.as_mut().unwrap();
        state.record_choice(state.current_node_id.clone(), choice.id.clone());

        // Execute on_exit effects from current node
        for effect in &node.on_exit_effects {
            state.add_effect(effect.clone());
        }

        // Execute choice effects
        for effect in &choice.effects {
            state.add_effect(effect.clone());
        }

        // Handle skill check result effects
        if let Some(ref result) = skill_check_result {
            if let Some(ref skill_check) = choice.skill_check {
                if result.success {
                    for effect in &skill_check.success_effects {
                        state.add_effect(effect.clone());
                    }
                } else {
                    for effect in &skill_check.failure_effects {
                        state.add_effect(effect.clone());
                    }
                }
            }
        }

        // Determine next node
        let next_node_id = if let Some(ref result) = skill_check_result {
            if let Some(ref skill_check) = choice.skill_check {
                match result.critical {
                    Some(CriticalResult::CriticalSuccess) => {
                        skill_check.critical_success_node.clone()
                            .or_else(|| skill_check.success_node.clone())
                            .or_else(|| choice.next_node.clone())
                    }
                    Some(CriticalResult::CriticalFailure) => {
                        skill_check.critical_failure_node.clone()
                            .or_else(|| skill_check.failure_node.clone())
                    }
                    None if result.success => {
                        skill_check.success_node.clone()
                            .or_else(|| choice.next_node.clone())
                    }
                    None => skill_check.failure_node.clone(),
                }
            } else {
                choice.next_node.clone()
            }
        } else {
            choice.next_node.clone()
        };

        // Move to next node or end dialogue
        let state = self.active_dialogue.as_mut().unwrap();
        let is_terminal = if let Some(ref next_id) = next_node_id {
            state.move_to_node(next_id.clone());
            // Execute on_enter effects for new node
            if let Some(next_node) = tree.get_node(next_id) {
                for effect in &next_node.on_enter_effects {
                    state.add_effect(effect.clone());
                }
                next_node.is_terminal
            } else {
                true
            }
        } else {
            true
        };

        if is_terminal {
            state.complete();
        }

        // Log to history if appropriate
        if node.dialogue_type.should_log() {
            let npc_name = state.npc_id.clone(); // Would get actual name
            let entry = DialogueHistoryEntry::new(
                0, // Will be assigned
                context.current_turn,
                state.npc_id.clone(),
                npc_name,
                state.tree_id.clone(),
                state.current_node_id.clone(),
                node.text.clone(),
                node.dialogue_type,
            )
            .with_response(choice.text.clone(), choice.id.clone());

            self.history.add_entry(entry);
        }

        // Take pending effects
        let effects = state.take_effects();

        Ok(DialogueSelectResult {
            next_node_id,
            effects,
            skill_check_result,
            dialogue_complete: state.is_complete,
        })
    }

    /// Perform a skill check
    fn perform_skill_check(
        &self,
        skill_check: &SkillCheckInfo,
        context: &PlayerDialogueContext,
        rng: &mut impl rand::Rng,
    ) -> SkillCheckResult {
        let player_skill = context.stats.get_skill(skill_check.skill);
        let roll_modifier: i32 = rng.gen_range(-5..=5);
        let total = player_skill + roll_modifier;
        let success = total >= skill_check.difficulty;

        // Check for critical results
        let critical = if total >= skill_check.difficulty + 10 {
            Some(CriticalResult::CriticalSuccess)
        } else if total <= skill_check.difficulty - 10 {
            Some(CriticalResult::CriticalFailure)
        } else {
            None
        };

        SkillCheckResult {
            skill: skill_check.skill,
            difficulty: skill_check.difficulty,
            player_value: player_skill,
            roll_modifier,
            total,
            success,
            critical,
        }
    }

    /// End current dialogue
    pub fn end_dialogue(&mut self) -> Option<DialogueState> {
        self.active_dialogue.take()
    }

    /// Check if currently in dialogue
    pub fn is_in_dialogue(&self) -> bool {
        self.active_dialogue.is_some()
    }

    /// Evaluate a dialogue condition
    pub fn evaluate_condition(
        &self,
        condition: &DialogueCondition,
        context: &PlayerDialogueContext,
    ) -> bool {
        match condition {
            DialogueCondition::HasStats { strength, intelligence, charisma, dexterity } => {
                strength.map_or(true, |min| context.stats.strength >= min)
                    && intelligence.map_or(true, |min| context.stats.intelligence >= min)
                    && charisma.map_or(true, |min| context.stats.charisma >= min)
                    && dexterity.map_or(true, |min| context.stats.dexterity >= min)
            }
            DialogueCondition::HasItem { item_kind, rarity: _, quantity } => {
                context.has_item(item_kind, *quantity)
            }
            DialogueCondition::HasGold(amount) => context.gold >= *amount,
            DialogueCondition::HasReputation { faction, min_value } => {
                context.get_reputation(faction) >= *min_value
            }
            DialogueCondition::IsClass(class) => context.class == Some(*class),
            DialogueCondition::IsSpecies(species) => context.species == Some(*species),
            DialogueCondition::MinLevel(level) => context.level >= *level,
            DialogueCondition::QuestState { quest_id, state } => {
                context
                    .quest_states
                    .get(quest_id)
                    .map_or(false, |s| s == state)
            }
            DialogueCondition::HasKnowledge(topic) => context.known_topics.contains(topic),
            DialogueCondition::TimeOfDay(required) => context.time_of_day == *required,
            DialogueCondition::PreviousChoice { dialogue_tree_id, choice_id } => {
                self.history.was_choice_made(dialogue_tree_id, choice_id)
            }
            DialogueCondition::NpcDisposition { npc_id, min_value } => {
                context.get_disposition(npc_id) >= *min_value
            }
            DialogueCondition::AreaUnlocked(area) => context.unlocked_areas.contains(area),
            DialogueCondition::EventOccurred(event) => context.occurred_events.contains(event),
            DialogueCondition::NotHasItem(item) => !context.has_item(item, 1),
            DialogueCondition::All(conditions) => {
                conditions.iter().all(|c| self.evaluate_condition(c, context))
            }
            DialogueCondition::Any(conditions) => {
                conditions.iter().any(|c| self.evaluate_condition(c, context))
            }
            DialogueCondition::Not(condition) => !self.evaluate_condition(condition, context),
            DialogueCondition::RandomChance(chance) => rand::random::<f32>() < *chance,
            DialogueCondition::ConversationCount { npc_id, min_count } => {
                self.history.get_conversation_count(npc_id) >= *min_count as usize
            }
            DialogueCondition::SkillCheck { skill, difficulty } => {
                context.stats.get_skill(*skill) >= *difficulty
            }
        }
    }

    /// Set a dialogue variable
    pub fn set_variable(&mut self, name: impl Into<String>, value: DialogueVariable) {
        self.variables.insert(name.into(), value);
    }

    /// Get a dialogue variable
    pub fn get_variable(&self, name: &str) -> Option<&DialogueVariable> {
        self.variables.get(name)
    }

    /// Register combat taunts for an enemy type
    pub fn register_combat_taunts(&mut self, enemy_type: impl Into<String>, taunts: Vec<String>) {
        self.combat_taunts.insert(enemy_type.into(), taunts);
    }

    /// Get a random combat taunt
    pub fn get_combat_taunt(&self, enemy_type: &str, rng: &mut impl rand::Rng) -> Option<&str> {
        self.combat_taunts.get(enemy_type).and_then(|taunts| {
            if taunts.is_empty() {
                None
            } else {
                Some(taunts[rng.gen_range(0..taunts.len())].as_str())
            }
        })
    }

    /// Register a tutorial hint
    pub fn register_tutorial(&mut self, topic: impl Into<String>, tree: DialogueTree) {
        self.tutorial_hints
            .entry(topic.into())
            .or_default()
            .push(tree);
    }

    /// Register a lore entry
    pub fn register_lore(&mut self, topic: TopicId, tree: DialogueTree) {
        self.lore_entries.entry(topic).or_default().push(tree);
    }

    /// Get lore entries for a topic
    pub fn get_lore(&self, topic: &TopicId) -> Option<&Vec<DialogueTree>> {
        self.lore_entries.get(topic)
    }

    /// Parse a dialogue tree from JSON
    pub fn parse_tree_from_json(json: &str) -> Result<DialogueTree, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize a dialogue tree to JSON
    pub fn serialize_tree_to_json(tree: &DialogueTree) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(tree)
    }

    /// Import multiple trees from JSON array
    pub fn import_trees_from_json(&mut self, json: &str) -> Result<usize, serde_json::Error> {
        let trees: Vec<DialogueTree> = serde_json::from_str(json)?;
        let count = trees.len();
        for tree in trees {
            self.register_tree(tree);
        }
        Ok(count)
    }

    /// Export all trees to JSON
    pub fn export_trees_to_json(&self) -> Result<String, serde_json::Error> {
        let trees: Vec<&DialogueTree> = self.trees.values().collect();
        serde_json::to_string_pretty(&trees)
    }
}

// ============================================================================
// Helper Types and Functions
// ============================================================================

/// A choice available to the player
#[derive(Clone, Debug)]
pub struct AvailableChoice {
    /// The choice
    pub choice: DialogueChoice,
    /// Whether it can be selected
    pub is_available: bool,
    /// Why it's locked (if applicable)
    pub locked_reason: Option<String>,
    /// Skill check preview (if applicable)
    pub skill_check_preview: Option<SkillCheckPreview>,
}

/// Preview of a skill check
#[derive(Clone, Debug)]
pub struct SkillCheckPreview {
    /// Skill type
    pub skill: SkillCheckType,
    /// Difficulty
    pub difficulty: i32,
    /// Player's current value
    pub player_value: i32,
    /// Estimated success chance
    pub success_chance: f32,
}

/// Result of selecting a choice
#[derive(Clone, Debug)]
pub struct DialogueSelectResult {
    /// Next node to display (if any)
    pub next_node_id: Option<DialogueNodeId>,
    /// Effects to execute
    pub effects: Vec<DialogueEffect>,
    /// Skill check result (if applicable)
    pub skill_check_result: Option<SkillCheckResult>,
    /// Whether dialogue is complete
    pub dialogue_complete: bool,
}

/// Errors that can occur in the dialogue system
#[derive(Clone, Debug)]
pub enum DialogueError {
    /// Already in a dialogue
    AlreadyInDialogue,
    /// Not currently in dialogue
    NotInDialogue,
    /// NPC not found
    NpcNotFound(NpcId),
    /// Dialogue tree not found
    TreeNotFound(DialogueTreeId),
    /// Node not found
    NodeNotFound(DialogueNodeId),
    /// No valid dialogue tree available
    NoValidTree,
    /// Conditions not met to start dialogue
    ConditionsNotMet,
    /// Dialogue on cooldown
    OnCooldown { remaining: u64 },
    /// Invalid choice index
    InvalidChoice,
    /// Choice is locked
    ChoiceLocked,
    /// Parse error
    ParseError(String),
}

impl std::fmt::Display for DialogueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyInDialogue => write!(f, "Already in dialogue"),
            Self::NotInDialogue => write!(f, "Not currently in dialogue"),
            Self::NpcNotFound(id) => write!(f, "NPC not found: {}", id),
            Self::TreeNotFound(id) => write!(f, "Dialogue tree not found: {}", id),
            Self::NodeNotFound(id) => write!(f, "Dialogue node not found: {}", id),
            Self::NoValidTree => write!(f, "No valid dialogue tree available"),
            Self::ConditionsNotMet => write!(f, "Conditions not met to start dialogue"),
            Self::OnCooldown { remaining } => write!(f, "Dialogue on cooldown for {} turns", remaining),
            Self::InvalidChoice => write!(f, "Invalid choice index"),
            Self::ChoiceLocked => write!(f, "Choice is locked"),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for DialogueError {}

/// Calculate success chance for a skill check
fn calculate_success_chance(player_value: i32, difficulty: i32) -> f32 {
    // Simple linear calculation with roll modifier range of -5 to +5
    let diff = player_value - difficulty;
    let base_chance = 0.5 + (diff as f32 * 0.1);
    base_chance.clamp(0.05, 0.95)
}

// ============================================================================
// Pre-built Dialogue Templates
// ============================================================================

/// Create a standard greeting dialogue
pub fn create_greeting_dialogue(
    npc_id: &str,
    npc_name: &str,
    personality: PersonalityType,
) -> DialogueTree {
    let greeting_text = match personality {
        PersonalityType::Friendly => format!("Welcome, traveler! It's wonderful to see a friendly face. I'm {}. How may I help you today?", npc_name),
        PersonalityType::Hostile => format!("What do you want? Make it quick. I'm {}, and I don't have time for idle chatter.", npc_name),
        PersonalityType::Mysterious => format!("Ah... you've found me. I am {}. Perhaps fate has drawn us together for a reason...", npc_name),
        PersonalityType::Comedic => format!("Well well well! Look what the dungeon dragged in! I'm {}, the funniest person you'll meet down here. And probably the only one, let's be honest.", npc_name),
        PersonalityType::Tragic => format!("*sighs* Another soul wandering these cursed halls. I am {}... or what remains of them.", npc_name),
        PersonalityType::Neutral => format!("Greetings. I am {}. What brings you here?", npc_name),
        PersonalityType::Arrogant => format!("You dare approach me? I am {}! You should feel honored to be in my presence.", npc_name),
        PersonalityType::Fearful => format!("O-oh! You startled me! I-I'm {}. Please, you're not going to hurt me, are you?", npc_name),
        PersonalityType::Scholarly => format!("Interesting! A new subject for observation! I am {}, researcher of all things arcane and mundane.", npc_name),
        PersonalityType::Zealous => format!("The light has guided you to me! I am {}, servant of the divine purpose! Are you ready to hear the truth?", npc_name),
        PersonalityType::Mercenary => format!("A customer! Excellent. I'm {}. Everything has a price, friend. What can I sell you today?", npc_name),
        PersonalityType::Noble => format!("Greetings, adventurer. I am {}, and I offer my assistance to those who prove worthy.", npc_name),
    };

    let root = DialogueNode::new(
        "greeting",
        npc_name,
        greeting_text,
    )
    .with_type(DialogueType::Greeting)
    .with_choice(
        DialogueChoice::new("greeting_friendly", "It's nice to meet you.")
            .with_next_node("main_menu")
            .with_effect(DialogueEffect::ModifyDisposition {
                npc_id: npc_id.to_string(),
                amount: 5,
            }),
    )
    .with_choice(
        DialogueChoice::new("greeting_business", "Let's get down to business.")
            .with_next_node("main_menu"),
    )
    .with_choice(
        DialogueChoice::new("greeting_leave", "Actually, I should go.")
            .with_effect(DialogueEffect::EndDialogue),
    );

    let main_menu = DialogueNode::new(
        "main_menu",
        npc_name,
        "What would you like to discuss?",
    )
    .with_choice(
        DialogueChoice::new("ask_info", "Tell me about yourself.")
            .with_next_node("about_self"),
    )
    .with_choice(
        DialogueChoice::new("ask_area", "What can you tell me about this place?")
            .with_next_node("about_area"),
    )
    .with_choice(
        DialogueChoice::new("farewell", "Farewell.")
            .with_effect(DialogueEffect::EndDialogue),
    );

    let about_self = DialogueNode::new(
        "about_self",
        npc_name,
        "There's not much to tell, really. I've been here longer than I care to remember.",
    )
    .with_choice(
        DialogueChoice::new("back", "I see. What else can we discuss?")
            .with_next_node("main_menu"),
    );

    let about_area = DialogueNode::new(
        "about_area",
        npc_name,
        "These halls hold many secrets. Tread carefully, adventurer.",
    )
    .with_type(DialogueType::LoreExposition)
    .with_choice(
        DialogueChoice::new("back", "Thanks for the warning. Anything else?")
            .with_next_node("main_menu"),
    );

    DialogueTree::new(format!("{}_greeting", npc_id), format!("{} Greeting", npc_name), root)
        .with_node(main_menu)
        .with_node(about_self)
        .with_node(about_area)
        .with_personality(personality)
}

/// Create a shop dialogue
pub fn create_shop_dialogue(npc_id: &str, npc_name: &str, shop_id: &str) -> DialogueTree {
    let root = DialogueNode::new(
        "shop_greeting",
        npc_name,
        "Looking to buy or sell? I've got the finest wares in the dungeon!",
    )
    .with_type(DialogueType::ShopInteraction)
    .with_choice(
        DialogueChoice::new("browse", "Show me what you have.")
            .with_effect(DialogueEffect::OpenShop {
                shop_id: shop_id.to_string(),
                discount_percent: None,
            }),
    )
    .with_choice(
        DialogueChoice::new("haggle", "[Persuasion] Perhaps we could negotiate better prices?")
            .with_skill_check(SkillCheckType::Persuasion, 15)
            .with_next_node("haggle_success")
            .with_locked_text("[Persuasion 15] I could negotiate better prices..."),
    )
    .with_choice(
        DialogueChoice::new("leave", "Not today, thanks.")
            .with_effect(DialogueEffect::EndDialogue),
    );

    let haggle_success = DialogueNode::new(
        "haggle_success",
        npc_name,
        "Ha! You drive a hard bargain. Fine, I'll give you a small discount.",
    )
    .with_choice(
        DialogueChoice::new("shop_discount", "Now let's see those wares.")
            .with_effect(DialogueEffect::OpenShop {
                shop_id: shop_id.to_string(),
                discount_percent: Some(10),
            }),
    );

    DialogueTree::new(format!("{}_shop", npc_id), format!("{} Shop", npc_name), root)
        .with_node(haggle_success)
}

/// Create a quest dialogue
pub fn create_quest_dialogue(
    npc_id: &str,
    npc_name: &str,
    quest_id: &str,
    quest_name: &str,
    quest_description: &str,
) -> DialogueTree {
    let root = DialogueNode::new(
        "quest_offer",
        npc_name,
        format!("I have a task that needs doing. {}. Will you help?", quest_description),
    )
    .with_type(DialogueType::QuestDialogue)
    .with_choice(
        DialogueChoice::new("accept", "I'll do it.")
            .with_effect(DialogueEffect::StartQuest {
                quest_id: quest_id.to_string(),
                stage: None,
            })
            .with_effect(DialogueEffect::ModifyDisposition {
                npc_id: npc_id.to_string(),
                amount: 10,
            })
            .with_next_node("quest_accepted"),
    )
    .with_choice(
        DialogueChoice::new("ask_more", "Tell me more about this task.")
            .with_next_node("quest_details"),
    )
    .with_choice(
        DialogueChoice::new("decline", "I can't help you right now.")
            .with_effect(DialogueEffect::ModifyDisposition {
                npc_id: npc_id.to_string(),
                amount: -5,
            })
            .with_effect(DialogueEffect::EndDialogue),
    );

    let quest_details = DialogueNode::new(
        "quest_details",
        npc_name,
        format!("The quest is called '{}'. You'll be handsomely rewarded upon completion.", quest_name),
    )
    .with_type(DialogueType::QuestDialogue)
    .with_choice(
        DialogueChoice::new("accept_after_details", "Alright, I'll take the job.")
            .with_effect(DialogueEffect::StartQuest {
                quest_id: quest_id.to_string(),
                stage: None,
            })
            .with_next_node("quest_accepted"),
    )
    .with_choice(
        DialogueChoice::new("decline_after_details", "I need to think about it.")
            .with_effect(DialogueEffect::EndDialogue),
    );

    let quest_accepted = DialogueNode::new(
        "quest_accepted",
        npc_name,
        "Excellent! Return to me when the task is complete.",
    )
    .terminal();

    DialogueTree::new(
        format!("{}_quest_{}", npc_id, quest_id),
        format!("{} - {}", npc_name, quest_name),
        root,
    )
    .with_node(quest_details)
    .with_node(quest_accepted)
    .with_tag("quest")
}

/// Create a romance dialogue stage
pub fn create_romance_dialogue(
    npc_id: &str,
    npc_name: &str,
    current_stage: RomanceStage,
) -> Option<DialogueTree> {
    let (text, next_stage) = match current_stage {
        RomanceStage::None => (
            "Hmm? Oh, we haven't met before, have we?",
            RomanceStage::Acquaintance,
        ),
        RomanceStage::Acquaintance => (
            "Oh, it's you again. Nice to see a familiar face.",
            RomanceStage::Friend,
        ),
        RomanceStage::Friend => (
            "I've really enjoyed our conversations. You're... different from the others.",
            RomanceStage::CloseFriend,
        ),
        RomanceStage::CloseFriend => (
            "I... I've been thinking about you a lot lately. Is that strange?",
            RomanceStage::Interested,
        ),
        RomanceStage::Interested => (
            "Every time you leave, I count the moments until you return. I think... I think I'm falling for you.",
            RomanceStage::Courting,
        ),
        RomanceStage::Courting => (
            "Being with you feels right. I want this to last forever.",
            RomanceStage::Partner,
        ),
        RomanceStage::Partner => (
            "You complete me in ways I never knew I needed. I love you.",
            RomanceStage::Soulmate,
        ),
        RomanceStage::Soulmate => return None, // Max stage reached
    };

    let root = DialogueNode::new("romance_main", npc_name, text)
        .with_type(DialogueType::RomanceDialogue)
        .with_choice(
            DialogueChoice::new("romance_positive", "I feel the same way.")
                .with_effect(DialogueEffect::AdvanceRomance {
                    npc_id: npc_id.to_string(),
                    stage: next_stage,
                })
                .with_effect(DialogueEffect::ModifyDisposition {
                    npc_id: npc_id.to_string(),
                    amount: 20,
                })
                .with_next_node("romance_positive_response"),
        )
        .with_choice(
            DialogueChoice::new("romance_neutral", "Let's take things slow.")
                .with_next_node("romance_neutral_response"),
        )
        .with_choice(
            DialogueChoice::new("romance_negative", "I'm not sure about this...")
                .with_effect(DialogueEffect::ModifyDisposition {
                    npc_id: npc_id.to_string(),
                    amount: -10,
                })
                .with_next_node("romance_negative_response"),
        );

    let positive = DialogueNode::new(
        "romance_positive_response",
        npc_name,
        "*smiles warmly* That makes me so happy to hear.",
    )
    .terminal();

    let neutral = DialogueNode::new(
        "romance_neutral_response",
        npc_name,
        "Of course. I understand. I'll be here when you're ready.",
    )
    .terminal();

    let negative = DialogueNode::new(
        "romance_negative_response",
        npc_name,
        "*looks away* I... I see. Perhaps I misread the situation.",
    )
    .terminal();

    Some(
        DialogueTree::new(
            format!("{}_romance_{:?}", npc_id, current_stage),
            format!("{} Romance", npc_name),
            root,
        )
        .with_node(positive)
        .with_node(neutral)
        .with_node(negative)
        .with_tag("romance")
        .non_repeatable(),
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialogue_tree_creation() {
        let root = DialogueNode::new("root", "Test NPC", "Hello there!")
            .with_choice(DialogueChoice::new("greeting", "Hi!").with_next_node("response"))
            .with_choice(DialogueChoice::new("bye", "Goodbye."));

        let response = DialogueNode::new("response", "Test NPC", "Nice to meet you!").terminal();

        let tree = DialogueTree::new("test_tree", "Test Dialogue", root).with_node(response);

        assert_eq!(tree.id, "test_tree");
        assert_eq!(tree.nodes.len(), 2);
        assert!(tree.get_node(&"root".to_string()).is_some());
        assert!(tree.get_node(&"response".to_string()).is_some());
    }

    #[test]
    fn test_dialogue_system_basic() {
        let mut system = DialogueSystem::new();

        let root = DialogueNode::new("root", "Test NPC", "Hello!")
            .with_choice(DialogueChoice::new("bye", "Goodbye."));

        let tree = DialogueTree::new("test", "Test", root);
        system.register_tree(tree);
        system.assign_dialogue_to_npc("npc1".to_string(), "test".to_string());

        let context = PlayerDialogueContext::new();
        let result = system.start_dialogue(&"npc1".to_string(), &context);
        assert!(result.is_ok());
        assert!(system.is_in_dialogue());
    }

    #[test]
    fn test_condition_evaluation() {
        let system = DialogueSystem::new();
        let context = PlayerDialogueContext::new()
            .with_level(10)
            .with_class(CharacterClass::Warrior);

        assert!(system.evaluate_condition(&DialogueCondition::MinLevel(5), &context));
        assert!(!system.evaluate_condition(&DialogueCondition::MinLevel(15), &context));
        assert!(system.evaluate_condition(
            &DialogueCondition::IsClass(CharacterClass::Warrior),
            &context
        ));
        assert!(!system.evaluate_condition(
            &DialogueCondition::IsClass(CharacterClass::Mage),
            &context
        ));
    }

    #[test]
    fn test_dialogue_history() {
        let mut history = DialogueHistory::new(100);

        let entry = DialogueHistoryEntry::new(
            0,
            100,
            "npc1".to_string(),
            "Test NPC".to_string(),
            "tree1".to_string(),
            "node1".to_string(),
            "Hello!".to_string(),
            DialogueType::StandardChat,
        )
        .with_response("Hi!".to_string(), "greeting".to_string());

        history.add_entry(entry);

        assert_eq!(history.len(), 1);
        assert!(history.was_choice_made(&"tree1".to_string(), "greeting"));
        assert!(!history.was_choice_made(&"tree1".to_string(), "farewell"));
    }

    #[test]
    fn test_personality_traits() {
        let traits = PersonalityTraits::new(PersonalityType::Friendly)
            .with_quirk(SpeechQuirk::Formal)
            .with_preferred_topic("magic".to_string());

        assert_eq!(traits.personality_type, PersonalityType::Friendly);
        assert!(traits.generosity > 0.5);
        assert!(traits.preferred_topics.contains(&"magic".to_string()));
    }

    #[test]
    fn test_skill_check_chance() {
        // Player skill equals difficulty = 50% chance
        assert!((calculate_success_chance(15, 15) - 0.5).abs() < 0.01);

        // Player skill much higher = high chance
        assert!(calculate_success_chance(25, 15) > 0.9);

        // Player skill much lower = low chance
        assert!(calculate_success_chance(5, 15) < 0.15);
    }

    #[test]
    fn test_template_dialogues() {
        let greeting = create_greeting_dialogue("merchant1", "Bob the Merchant", PersonalityType::Friendly);
        assert!(!greeting.nodes.is_empty());
        assert_eq!(greeting.default_personality, PersonalityType::Friendly);

        let shop = create_shop_dialogue("merchant1", "Bob", "shop1");
        assert!(shop.nodes.contains_key(&"shop_greeting".to_string()));

        let quest = create_quest_dialogue("elder", "Elder Sage", "quest1", "The Lost Artifact", "Find the artifact");
        assert!(quest.tags.contains("quest"));

        let romance = create_romance_dialogue("npc1", "Test", RomanceStage::Friend);
        assert!(romance.is_some());
    }

    #[test]
    fn test_serialization() {
        let root = DialogueNode::new("root", "NPC", "Test")
            .with_choice(DialogueChoice::new("c1", "Choice 1"));

        let tree = DialogueTree::new("test", "Test Tree", root);

        let json = DialogueSystem::serialize_tree_to_json(&tree);
        assert!(json.is_ok());

        let parsed = DialogueSystem::parse_tree_from_json(&json.unwrap());
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().id, "test");
    }
}

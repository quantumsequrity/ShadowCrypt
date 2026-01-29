//! NPC and Dialogue System
//!
//! This module provides friendly NPCs and a dialogue tree system for the ShadowCrypt roguelike.
//! NPCs include merchants, quest givers, healers, and other friendly characters that
//! the player can interact with throughout the dungeon.

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::items::{Item, ItemKind, Rarity};

// ============================================================================
// Dialogue System
// ============================================================================

/// Unique identifier for dialogue nodes
pub type DialogueNodeId = u32;

/// A response option in a dialogue
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DialogueChoice {
    /// The text displayed for this choice
    pub text: String,
    /// The node to transition to when this choice is selected (None = end dialogue)
    pub next_node: Option<DialogueNodeId>,
    /// Optional action to trigger when this choice is selected
    pub action: Option<DialogueAction>,
    /// Condition that must be met for this choice to be available
    pub condition: Option<DialogueCondition>,
}

impl DialogueChoice {
    /// Create a new dialogue choice
    pub fn new(text: impl Into<String>, next_node: Option<DialogueNodeId>) -> Self {
        Self {
            text: text.into(),
            next_node,
            action: None,
            condition: None,
        }
    }

    /// Create a choice with an action
    pub fn with_action(mut self, action: DialogueAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Create a choice with a condition
    pub fn with_condition(mut self, condition: DialogueCondition) -> Self {
        self.condition = Some(condition);
        self
    }
}

/// Actions that can be triggered by dialogue choices
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum DialogueAction {
    /// Heal the player for a certain amount
    Heal(i32),
    /// Fully restore HP and mana
    FullRestore,
    /// Give gold to the player
    GiveGold(u32),
    /// Take gold from the player (for purchases)
    TakeGold(u32),
    /// Give an item to the player
    GiveItem(ItemKind, Rarity),
    /// Start a quest
    StartQuest(QuestId),
    /// Complete a quest
    CompleteQuest(QuestId),
    /// Open the merchant's shop interface
    OpenShop,
    /// Teach a skill or provide a buff
    GrantBuff(BuffType),
    /// Reveal the map
    RevealMap,
    /// Teleport the player to stairs
    TeleportToStairs,
    /// Remove negative status effects
    CureStatus,
    /// Provide information about the current floor
    RevealFloorInfo,
    /// Upgrade a piece of equipment
    UpgradeEquipment,
    /// Identify all items in inventory
    IdentifyItems,
    /// Rest and restore some HP/mana
    Rest,
}

/// Conditions for dialogue choices to be available
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum DialogueCondition {
    /// Player must have at least this much gold
    HasGold(u32),
    /// Player must have a specific item
    HasItem(ItemKind),
    /// Player must have completed a quest
    QuestCompleted(QuestId),
    /// Player must have an active quest
    QuestActive(QuestId),
    /// Player must be at or below this HP percentage
    LowHealth(u8),
    /// Player must have this many kills
    HasKills(u32),
    /// Player level must be at least this
    MinLevel(u32),
    /// Dungeon level must be at least this
    MinDungeonLevel(u32),
}

/// Types of temporary buffs NPCs can grant
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum BuffType {
    /// Increased attack damage
    Strength,
    /// Increased defense
    Protection,
    /// Faster movement/action
    Haste,
    /// Health regeneration
    Regeneration,
    /// See invisible enemies
    TrueSight,
    /// Resistance to fire damage
    FireResistance,
    /// Resistance to ice damage
    IceResistance,
    /// Resistance to poison
    PoisonResistance,
    /// Increased critical hit chance
    CriticalStrike,
    /// Increased luck for item drops
    Fortune,
}

impl BuffType {
    /// Returns the display name of this buff
    pub fn name(&self) -> &'static str {
        match self {
            Self::Strength => "Strength",
            Self::Protection => "Protection",
            Self::Haste => "Haste",
            Self::Regeneration => "Regeneration",
            Self::TrueSight => "True Sight",
            Self::FireResistance => "Fire Resistance",
            Self::IceResistance => "Ice Resistance",
            Self::PoisonResistance => "Poison Resistance",
            Self::CriticalStrike => "Critical Strike",
            Self::Fortune => "Fortune",
        }
    }

    /// Returns the duration in turns
    pub fn duration(&self) -> u32 {
        match self {
            Self::Strength => 50,
            Self::Protection => 50,
            Self::Haste => 30,
            Self::Regeneration => 40,
            Self::TrueSight => 100,
            Self::FireResistance => 60,
            Self::IceResistance => 60,
            Self::PoisonResistance => 60,
            Self::CriticalStrike => 40,
            Self::Fortune => 100,
        }
    }
}

/// A single node in a dialogue tree
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DialogueNode {
    /// Unique identifier for this node
    pub id: DialogueNodeId,
    /// The NPC's dialogue text
    pub text: String,
    /// Speaker name (usually the NPC's name)
    pub speaker: String,
    /// Available response choices
    pub choices: Vec<DialogueChoice>,
}

impl DialogueNode {
    /// Create a new dialogue node
    pub fn new(id: DialogueNodeId, speaker: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id,
            speaker: speaker.into(),
            text: text.into(),
            choices: Vec::new(),
        }
    }

    /// Add a choice to this node
    pub fn add_choice(mut self, choice: DialogueChoice) -> Self {
        self.choices.push(choice);
        self
    }
}

/// A complete dialogue tree containing multiple nodes
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DialogueTree {
    /// All nodes in the dialogue tree
    pub nodes: HashMap<DialogueNodeId, DialogueNode>,
    /// The starting node
    pub root_node: DialogueNodeId,
    /// Current active node (for tracking dialogue state)
    pub current_node: Option<DialogueNodeId>,
}

impl DialogueTree {
    /// Create a new dialogue tree with a root node
    pub fn new(root: DialogueNode) -> Self {
        let root_id = root.id;
        let mut nodes = HashMap::new();
        nodes.insert(root_id, root);
        Self {
            nodes,
            root_node: root_id,
            current_node: None,
        }
    }

    /// Add a node to the tree
    pub fn add_node(mut self, node: DialogueNode) -> Self {
        self.nodes.insert(node.id, node);
        self
    }

    /// Start the dialogue from the root
    pub fn start(&mut self) -> Option<&DialogueNode> {
        self.current_node = Some(self.root_node);
        self.nodes.get(&self.root_node)
    }

    /// Get the current dialogue node
    pub fn current(&self) -> Option<&DialogueNode> {
        self.current_node.and_then(|id| self.nodes.get(&id))
    }

    /// Select a choice and advance the dialogue
    /// Returns the action to perform (if any) and whether dialogue continues
    pub fn select_choice(&mut self, choice_idx: usize) -> (Option<DialogueAction>, bool) {
        let current = match self.current_node.and_then(|id| self.nodes.get(&id)) {
            Some(node) => node.clone(),
            None => return (None, false),
        };

        if choice_idx >= current.choices.len() {
            return (None, true);
        }

        let choice = &current.choices[choice_idx];
        let action = choice.action.clone();

        match choice.next_node {
            Some(next_id) => {
                self.current_node = Some(next_id);
                (action, true)
            }
            None => {
                self.current_node = None;
                (action, false)
            }
        }
    }

    /// Check if dialogue is active
    pub fn is_active(&self) -> bool {
        self.current_node.is_some()
    }

    /// End the dialogue
    pub fn end(&mut self) {
        self.current_node = None;
    }
}

// ============================================================================
// Quest System
// ============================================================================

/// Unique identifier for quests
pub type QuestId = u32;

/// Quest objective types
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum QuestObjective {
    /// Kill a certain number of enemies
    KillEnemies { target: u32, current: u32 },
    /// Kill a specific boss
    KillBoss { boss_level: u32, completed: bool },
    /// Collect a certain amount of gold
    CollectGold { target: u32, current: u32 },
    /// Reach a certain dungeon level
    ReachLevel { target: u32, completed: bool },
    /// Find a specific item
    FindItem { item: ItemKind, found: bool },
    /// Deliver an item to an NPC
    DeliverItem { item: ItemKind, npc_id: NPCId, delivered: bool },
    /// Explore a number of rooms
    ExploreRooms { target: u32, current: u32 },
    /// Survive for a number of turns
    Survive { target: u32, current: u32 },
}

impl QuestObjective {
    /// Check if the objective is complete
    pub fn is_complete(&self) -> bool {
        match self {
            Self::KillEnemies { target, current } => current >= target,
            Self::KillBoss { completed, .. } => *completed,
            Self::CollectGold { target, current } => current >= target,
            Self::ReachLevel { completed, .. } => *completed,
            Self::FindItem { found, .. } => *found,
            Self::DeliverItem { delivered, .. } => *delivered,
            Self::ExploreRooms { target, current } => current >= target,
            Self::Survive { target, current } => current >= target,
        }
    }

    /// Get a description of the objective
    pub fn description(&self) -> String {
        match self {
            Self::KillEnemies { target, current } => {
                format!("Kill {} enemies ({}/{})", target, current, target)
            }
            Self::KillBoss { boss_level, completed } => {
                let status = if *completed { "Done" } else { "In Progress" };
                format!("Defeat the boss on level {} ({})", boss_level, status)
            }
            Self::CollectGold { target, current } => {
                format!("Collect {} gold ({}/{})", target, current, target)
            }
            Self::ReachLevel { target, completed } => {
                let status = if *completed { "Done" } else { "In Progress" };
                format!("Reach dungeon level {} ({})", target, status)
            }
            Self::FindItem { item, found } => {
                let status = if *found { "Found" } else { "Not Found" };
                format!("Find {} ({})", item.name(), status)
            }
            Self::DeliverItem { item, delivered, .. } => {
                let status = if *delivered { "Delivered" } else { "Not Delivered" };
                format!("Deliver {} ({})", item.name(), status)
            }
            Self::ExploreRooms { target, current } => {
                format!("Explore {} rooms ({}/{})", target, current, target)
            }
            Self::Survive { target, current } => {
                format!("Survive {} turns ({}/{})", target, current, target)
            }
        }
    }
}

/// A quest that can be given by NPCs
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Quest {
    /// Unique identifier
    pub id: QuestId,
    /// Quest name
    pub name: String,
    /// Quest description
    pub description: String,
    /// The NPC who gave this quest
    pub giver_id: NPCId,
    /// Quest objectives
    pub objectives: Vec<QuestObjective>,
    /// Reward: gold amount
    pub reward_gold: u32,
    /// Reward: experience points
    pub reward_xp: u32,
    /// Reward: optional item
    pub reward_item: Option<(ItemKind, Rarity)>,
    /// Whether the quest has been completed
    pub completed: bool,
    /// Whether rewards have been claimed
    pub rewards_claimed: bool,
}

impl Quest {
    /// Create a new quest
    pub fn new(
        id: QuestId,
        name: impl Into<String>,
        description: impl Into<String>,
        giver_id: NPCId,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            giver_id,
            objectives: Vec::new(),
            reward_gold: 0,
            reward_xp: 0,
            reward_item: None,
            completed: false,
            rewards_claimed: false,
        }
    }

    /// Add an objective to the quest
    pub fn add_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    /// Set gold reward
    pub fn with_gold_reward(mut self, gold: u32) -> Self {
        self.reward_gold = gold;
        self
    }

    /// Set XP reward
    pub fn with_xp_reward(mut self, xp: u32) -> Self {
        self.reward_xp = xp;
        self
    }

    /// Set item reward
    pub fn with_item_reward(mut self, item: ItemKind, rarity: Rarity) -> Self {
        self.reward_item = Some((item, rarity));
        self
    }

    /// Check if all objectives are complete
    pub fn is_complete(&self) -> bool {
        !self.objectives.is_empty() && self.objectives.iter().all(|o| o.is_complete())
    }

    /// Update quest completion status
    pub fn update_completion(&mut self) {
        self.completed = self.is_complete();
    }
}

// ============================================================================
// NPC Types and Definitions
// ============================================================================

/// Unique identifier for NPCs
pub type NPCId = u32;

/// Different types of friendly NPCs
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Hash)]
pub enum NPCKind {
    // Merchants
    /// General goods merchant - sells potions and basic supplies
    Merchant,
    /// Weapons and armor specialist
    Blacksmith,
    /// Sells magical items and scrolls
    Enchanter,
    /// Rare and exotic items at high prices
    ExoticTrader,

    // Quest Givers
    /// Village elder with wisdom and quests
    Elder,
    /// Adventurer seeking assistance
    Adventurer,
    /// Mysterious figure with dark quests
    MysteriousStranger,
    /// Scholar researching the dungeon
    Scholar,

    // Healers
    /// Temple priest who heals and blesses
    Priest,
    /// Nature healer with herbal remedies
    Herbalist,
    /// Combat medic found in dungeons
    FieldMedic,

    // Utility NPCs
    /// Provides information and maps
    Cartographer,
    /// Identifies items and provides lore
    Sage,
    /// Upgrades equipment
    Artificer,
    /// Offers rest and food
    Innkeeper,

    // Special NPCs
    /// Ghostly spirit with cryptic messages
    Ghost,
    /// Trapped soul needing rescue
    PrisonerSpirit,
    /// Guardian of a shrine
    ShrineKeeper,
    /// Wandering bard with tales and buffs
    Bard,
}

impl NPCKind {
    /// Returns the glyph character for this NPC type
    pub fn glyph(&self) -> char {
        match self {
            Self::Merchant | Self::ExoticTrader => '$',
            Self::Blacksmith => 'B',
            Self::Enchanter => 'E',
            Self::Elder => 'e',
            Self::Adventurer => 'A',
            Self::MysteriousStranger => '?',
            Self::Scholar => 'S',
            Self::Priest => 'P',
            Self::Herbalist => 'H',
            Self::FieldMedic => '+',
            Self::Cartographer => 'C',
            Self::Sage => 's',
            Self::Artificer => 'a',
            Self::Innkeeper => 'I',
            Self::Ghost => 'G',
            Self::PrisonerSpirit => 'p',
            Self::ShrineKeeper => 'K',
            Self::Bard => 'b',
        }
    }

    /// Returns a color index for this NPC type
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Merchant | Self::ExoticTrader => 11,  // Yellow (gold)
            Self::Blacksmith => 1,                       // Grey (metal)
            Self::Enchanter => 13,                       // Magenta (magic)
            Self::Elder => 2,                            // White (wisdom)
            Self::Adventurer => 5,                       // Green
            Self::MysteriousStranger => 14,              // Purple
            Self::Scholar => 7,                          // Blue
            Self::Priest => 11,                          // Yellow (holy)
            Self::Herbalist => 5,                        // Green (nature)
            Self::FieldMedic => 3,                       // Red (health)
            Self::Cartographer => 7,                     // Blue
            Self::Sage => 9,                             // Cyan
            Self::Artificer => 6,                        // Brown/Orange
            Self::Innkeeper => 12,                       // Light grey
            Self::Ghost => 1,                            // Grey (spectral)
            Self::PrisonerSpirit => 9,                   // Cyan
            Self::ShrineKeeper => 11,                    // Yellow
            Self::Bard => 13,                            // Magenta
        }
    }

    /// Returns the display name of this NPC type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Merchant => "Merchant",
            Self::Blacksmith => "Blacksmith",
            Self::Enchanter => "Enchanter",
            Self::ExoticTrader => "Exotic Trader",
            Self::Elder => "Village Elder",
            Self::Adventurer => "Adventurer",
            Self::MysteriousStranger => "Mysterious Stranger",
            Self::Scholar => "Scholar",
            Self::Priest => "Priest",
            Self::Herbalist => "Herbalist",
            Self::FieldMedic => "Field Medic",
            Self::Cartographer => "Cartographer",
            Self::Sage => "Sage",
            Self::Artificer => "Artificer",
            Self::Innkeeper => "Innkeeper",
            Self::Ghost => "Wandering Spirit",
            Self::PrisonerSpirit => "Imprisoned Soul",
            Self::ShrineKeeper => "Shrine Keeper",
            Self::Bard => "Wandering Bard",
        }
    }

    /// Returns a description of this NPC type
    pub fn description(&self) -> &'static str {
        match self {
            Self::Merchant => "A traveling merchant selling various goods.",
            Self::Blacksmith => "A skilled smith who can repair and sell equipment.",
            Self::Enchanter => "A mystical vendor of scrolls and magical items.",
            Self::ExoticTrader => "A rare dealer in legendary artifacts.",
            Self::Elder => "A wise elder with knowledge of ancient quests.",
            Self::Adventurer => "A fellow dungeon explorer seeking allies.",
            Self::MysteriousStranger => "A cloaked figure with secrets to share.",
            Self::Scholar => "A researcher studying the dungeon's mysteries.",
            Self::Priest => "A holy servant offering healing and blessings.",
            Self::Herbalist => "A nature healer with potent remedies.",
            Self::FieldMedic => "A combat medic who tends to wounded adventurers.",
            Self::Cartographer => "A mapmaker who can reveal hidden areas.",
            Self::Sage => "An ancient sage with vast knowledge.",
            Self::Artificer => "A craftsman who can enhance equipment.",
            Self::Innkeeper => "A friendly host offering rest and supplies.",
            Self::Ghost => "A restless spirit bound to this place.",
            Self::PrisonerSpirit => "A soul trapped by dark magic.",
            Self::ShrineKeeper => "A guardian of sacred grounds.",
            Self::Bard => "A traveling musician with inspiring tales.",
        }
    }

    /// Returns whether this NPC type can appear at the given dungeon level
    pub fn can_spawn_at_level(&self, level: u32) -> bool {
        match self {
            Self::Merchant | Self::FieldMedic => true,
            Self::Blacksmith => level >= 3,
            Self::Enchanter => level >= 5,
            Self::ExoticTrader => level >= 15,
            Self::Elder => level <= 10,
            Self::Adventurer => level >= 2 && level <= 20,
            Self::MysteriousStranger => level >= 10,
            Self::Scholar => level >= 5 && level <= 25,
            Self::Priest => level >= 3,
            Self::Herbalist => level <= 15,
            Self::Cartographer => level >= 5,
            Self::Sage => level >= 10,
            Self::Artificer => level >= 8,
            Self::Innkeeper => level <= 20,
            Self::Ghost => level >= 8,
            Self::PrisonerSpirit => level >= 12,
            Self::ShrineKeeper => level >= 5,
            Self::Bard => true,
        }
    }

    /// Returns whether this NPC is a merchant type
    pub fn is_merchant(&self) -> bool {
        matches!(self, Self::Merchant | Self::Blacksmith | Self::Enchanter | Self::ExoticTrader)
    }

    /// Returns whether this NPC is a healer type
    pub fn is_healer(&self) -> bool {
        matches!(self, Self::Priest | Self::Herbalist | Self::FieldMedic)
    }

    /// Returns whether this NPC can give quests
    pub fn is_quest_giver(&self) -> bool {
        matches!(
            self,
            Self::Elder | Self::Adventurer | Self::MysteriousStranger |
            Self::Scholar | Self::PrisonerSpirit | Self::ShrineKeeper
        )
    }
}

/// Shop inventory item with price
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShopItem {
    pub item: ItemKind,
    pub rarity: Rarity,
    pub price: u32,
    pub stock: u32,
}

impl ShopItem {
    pub fn new(item: ItemKind, rarity: Rarity, price: u32, stock: u32) -> Self {
        Self { item, rarity, price, stock }
    }
}

/// Actions NPCs can take autonomously
#[derive(Clone, Debug)]
pub enum NPCAction {
    /// Move in a direction
    Move(i32, i32),
    /// Wait/idle
    Wait,
    /// Talk (ambient dialogue)
    Talk,
    /// Flee from danger
    Flee,
    /// Trade with another entity
    Trade,
    /// Rest
    Rest,
    /// Restock inventory
    Restock,
}

/// NPC behavior for autonomous actions
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub enum NPCBehavior {
    /// Standing still at current location
    #[default]
    Stationary,
    /// Wandering around the area
    Wandering,
    /// Walking to a destination
    Walking,
    /// Resting/sleeping
    Resting,
    /// Currently talking to another NPC
    Socializing,
    /// Trading with another entity
    Trading,
    /// Fleeing from danger
    Fleeing,
}

/// Activity schedule for NPCs
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NPCSchedule {
    /// Home position (where NPC returns to rest)
    pub home_pos: (usize, usize),
    /// Work position (where NPC conducts business)
    pub work_pos: (usize, usize),
    /// Wander radius from current position
    pub wander_radius: usize,
    /// Current activity time slot
    pub current_slot: u8, // 0-23 for hour of day
}

impl Default for NPCSchedule {
    fn default() -> Self {
        Self {
            home_pos: (0, 0),
            work_pos: (0, 0),
            wander_radius: 5,
            current_slot: 12, // Noon
        }
    }
}

/// An NPC instance in the game world
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NPC {
    /// Unique identifier
    pub id: NPCId,
    /// Position X
    pub x: usize,
    /// Position Y
    pub y: usize,
    /// NPC type
    pub kind: NPCKind,
    /// Custom name (if any)
    pub name: Option<String>,
    /// Dialogue tree for this NPC
    pub dialogue: DialogueTree,
    /// Shop inventory (for merchants)
    pub shop_inventory: Vec<ShopItem>,
    /// Quests offered by this NPC
    pub offered_quests: Vec<Quest>,
    /// Whether the player has interacted with this NPC before
    pub has_met: bool,
    /// Disposition towards player (affects prices, dialogue, etc.)
    pub disposition: i32,
    /// Floor level where this NPC was spawned
    pub spawn_level: u32,
    /// Current behavior state
    pub behavior: NPCBehavior,
    /// Schedule for daily activities
    pub schedule: NPCSchedule,
    /// Target position when walking
    pub target_pos: Option<(usize, usize)>,
    /// Current conversation partner (NPC ID)
    pub conversation_partner: Option<NPCId>,
    /// Turns until next restock (for merchants)
    pub restock_timer: u32,
    /// Action cooldown
    pub action_cooldown: u32,
    /// Turns since last player interaction
    pub turns_since_player: u32,
}

impl NPC {
    /// Create a new NPC with autonomous behavior
    pub fn new(id: NPCId, x: usize, y: usize, kind: NPCKind, dungeon_level: u32) -> Self {
        let dialogue = create_default_dialogue(kind);
        let shop_inventory = if kind.is_merchant() {
            create_shop_inventory(kind, dungeon_level)
        } else {
            Vec::new()
        };

        // Determine initial behavior based on NPC type
        let behavior = match kind {
            NPCKind::Bard | NPCKind::Adventurer => NPCBehavior::Wandering,
            NPCKind::Ghost | NPCKind::PrisonerSpirit => NPCBehavior::Wandering,
            _ => NPCBehavior::Stationary,
        };

        // Set up schedule
        let schedule = NPCSchedule {
            home_pos: (x, y),
            work_pos: (x, y),
            wander_radius: match kind {
                NPCKind::Bard => 15,
                NPCKind::Adventurer => 12,
                NPCKind::Ghost => 20,
                NPCKind::MysteriousStranger => 10,
                _ => 5,
            },
            current_slot: 12,
        };

        // Restock timer for merchants (100-200 turns)
        let restock_timer = if kind.is_merchant() { 150 } else { 0 };

        Self {
            id,
            x,
            y,
            kind,
            name: None,
            dialogue,
            shop_inventory,
            offered_quests: Vec::new(),
            has_met: false,
            disposition: 50, // Neutral
            spawn_level: dungeon_level,
            behavior,
            schedule,
            target_pos: None,
            conversation_partner: None,
            restock_timer,
            action_cooldown: 0,
            turns_since_player: 0,
        }
    }

    /// Update NPC autonomous behavior
    pub fn update(&mut self, rng: &mut impl Rng, dungeon_level: u32) {
        // Decrease cooldowns
        if self.action_cooldown > 0 {
            self.action_cooldown -= 1;
        }
        self.turns_since_player += 1;

        // Handle restock timer for merchants
        if self.kind.is_merchant() && self.restock_timer > 0 {
            self.restock_timer -= 1;
            if self.restock_timer == 0 {
                self.restock(dungeon_level);
                self.restock_timer = rng.gen_range(100..200);
            }
        }

        // End conversations after some time
        if self.behavior == NPCBehavior::Socializing {
            if rng.gen_bool(0.1) {
                self.behavior = NPCBehavior::Stationary;
                self.conversation_partner = None;
            }
        }
    }

    /// Decide next action based on behavior
    pub fn decide_action(&self, rng: &mut impl Rng) -> NPCAction {
        match self.behavior {
            NPCBehavior::Wandering => {
                if rng.gen_bool(0.3) {
                    // Random movement within wander radius
                    let dx = rng.gen_range(-1..=1);
                    let dy = rng.gen_range(-1..=1);
                    NPCAction::Move(dx, dy)
                } else {
                    NPCAction::Wait
                }
            }
            NPCBehavior::Walking => {
                if let Some((tx, ty)) = self.target_pos {
                    let dx = (tx as i32 - self.x as i32).signum();
                    let dy = (ty as i32 - self.y as i32).signum();
                    if dx == 0 && dy == 0 {
                        NPCAction::Wait
                    } else {
                        NPCAction::Move(dx, dy)
                    }
                } else {
                    NPCAction::Wait
                }
            }
            NPCBehavior::Fleeing => {
                // Flee towards home position
                let dx = (self.schedule.home_pos.0 as i32 - self.x as i32).signum();
                let dy = (self.schedule.home_pos.1 as i32 - self.y as i32).signum();
                NPCAction::Move(dx, dy)
            }
            NPCBehavior::Socializing => {
                // Maybe say something
                if rng.gen_bool(0.1) {
                    NPCAction::Talk
                } else {
                    NPCAction::Wait
                }
            }
            _ => NPCAction::Wait,
        }
    }

    /// Check if NPC is in danger and should flee
    pub fn check_danger(&mut self, enemies: &[(usize, usize)]) -> bool {
        for &(ex, ey) in enemies {
            let dx = (self.x as i32 - ex as i32).abs();
            let dy = (self.y as i32 - ey as i32).abs();
            if dx <= 3 && dy <= 3 {
                self.behavior = NPCBehavior::Fleeing;
                return true;
            }
        }
        false
    }

    /// Try to start a conversation with another NPC
    pub fn try_socialize(&mut self, other_id: NPCId) -> bool {
        if self.action_cooldown > 0 || self.behavior == NPCBehavior::Socializing {
            return false;
        }

        self.behavior = NPCBehavior::Socializing;
        self.conversation_partner = Some(other_id);
        self.action_cooldown = 30; // Cooldown before next social interaction
        true
    }

    /// Generate a random ambient conversation line
    pub fn get_ambient_line(&self, rng: &mut impl Rng) -> Option<String> {
        let lines: &[&str] = match self.kind {
            NPCKind::Merchant | NPCKind::Blacksmith | NPCKind::Enchanter => &[
                "Business has been slow lately...",
                "I need to restock soon.",
                "These dungeons attract all sorts.",
                "Quality goods, fair prices!",
            ],
            NPCKind::Adventurer => &[
                "The deeper levels are treacherous.",
                "I've seen things down here...",
                "Watch out for traps!",
                "Have you found any good loot?",
            ],
            NPCKind::Bard => &[
                "*hums a tune*",
                "I should write a song about this place.",
                "Every hero needs a bard!",
                "*strums lute quietly*",
            ],
            NPCKind::Ghost | NPCKind::PrisonerSpirit => &[
                "...so cold...",
                "*fades slightly*",
                "Remember us...",
                "The darkness... it calls...",
            ],
            NPCKind::Priest | NPCKind::ShrineKeeper => &[
                "May the light guide you.",
                "Stay vigilant against evil.",
                "Blessings upon you.",
                "*prays quietly*",
            ],
            _ => &[
                "...",
                "*looks around*",
                "Hmm...",
            ],
        };

        if rng.gen_bool(0.05) {
            Some(lines[rng.gen_range(0..lines.len())].to_string())
        } else {
            None
        }
    }

    /// Create an NPC with a custom name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Get the display name (custom name or type name)
    pub fn display_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.kind.name().to_string())
    }

    /// Start a conversation with this NPC
    pub fn start_dialogue(&mut self) -> Option<&DialogueNode> {
        self.has_met = true;
        self.dialogue.start()
    }

    /// Get the current dialogue node
    pub fn current_dialogue(&self) -> Option<&DialogueNode> {
        self.dialogue.current()
    }

    /// Select a dialogue choice
    pub fn select_dialogue_choice(&mut self, choice_idx: usize) -> (Option<DialogueAction>, bool) {
        self.dialogue.select_choice(choice_idx)
    }

    /// End the dialogue
    pub fn end_dialogue(&mut self) {
        self.dialogue.end();
    }

    /// Check if dialogue is active
    pub fn is_in_dialogue(&self) -> bool {
        self.dialogue.is_active()
    }

    /// Add a quest to this NPC's offered quests
    pub fn add_quest(&mut self, quest: Quest) {
        self.offered_quests.push(quest);
    }

    /// Modify disposition
    pub fn modify_disposition(&mut self, amount: i32) {
        self.disposition = (self.disposition + amount).clamp(0, 100);
    }

    /// Get price modifier based on disposition
    pub fn price_modifier(&self) -> f32 {
        // Higher disposition = lower prices
        // 0 disposition = 1.5x prices, 100 disposition = 0.75x prices
        1.5 - (self.disposition as f32 * 0.0075)
    }

    /// Restock shop inventory
    pub fn restock(&mut self, dungeon_level: u32) {
        if self.kind.is_merchant() {
            self.shop_inventory = create_shop_inventory(self.kind, dungeon_level);
        }
    }
}

// ============================================================================
// Dialogue Creation Helpers
// ============================================================================

/// Create a default dialogue tree for an NPC type
fn create_default_dialogue(kind: NPCKind) -> DialogueTree {
    match kind {
        NPCKind::Merchant => create_merchant_dialogue(),
        NPCKind::Blacksmith => create_blacksmith_dialogue(),
        NPCKind::Enchanter => create_enchanter_dialogue(),
        NPCKind::ExoticTrader => create_exotic_trader_dialogue(),
        NPCKind::Priest => create_priest_dialogue(),
        NPCKind::Herbalist => create_herbalist_dialogue(),
        NPCKind::FieldMedic => create_field_medic_dialogue(),
        NPCKind::Elder => create_elder_dialogue(),
        NPCKind::Adventurer => create_adventurer_dialogue(),
        NPCKind::MysteriousStranger => create_mysterious_stranger_dialogue(),
        NPCKind::Scholar => create_scholar_dialogue(),
        NPCKind::Cartographer => create_cartographer_dialogue(),
        NPCKind::Sage => create_sage_dialogue(),
        NPCKind::Artificer => create_artificer_dialogue(),
        NPCKind::Innkeeper => create_innkeeper_dialogue(),
        NPCKind::Ghost => create_ghost_dialogue(),
        NPCKind::PrisonerSpirit => create_prisoner_spirit_dialogue(),
        NPCKind::ShrineKeeper => create_shrine_keeper_dialogue(),
        NPCKind::Bard => create_bard_dialogue(),
    }
}

fn create_merchant_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Merchant", "Welcome, traveler! I have many fine goods for sale. What can I interest you in today?")
        .add_choice(DialogueChoice::new("Show me your wares.", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("What news from the surface?", Some(2)))
        .add_choice(DialogueChoice::new("Farewell.", None));

    let shop_node = DialogueNode::new(1, "Merchant", "Take your time browsing. Quality goods at fair prices!")
        .add_choice(DialogueChoice::new("Thank you.", None));

    let news_node = DialogueNode::new(2, "Merchant", "Dark times, friend. The monsters grow bolder each day. But where there's danger, there's opportunity for profit... and heroism!")
        .add_choice(DialogueChoice::new("Indeed. Show me your wares.", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("Stay safe, merchant.", None));

    DialogueTree::new(root)
        .add_node(shop_node)
        .add_node(news_node)
}

fn create_blacksmith_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Blacksmith", "Hail, warrior! These halls echo with the clash of steel. Need something forged or repaired?")
        .add_choice(DialogueChoice::new("I need weapons and armor.", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("Can you upgrade my equipment?", Some(2)))
        .add_choice(DialogueChoice::new("How did you end up down here?", Some(3)))
        .add_choice(DialogueChoice::new("Farewell.", None));

    let shop_node = DialogueNode::new(1, "Blacksmith", "Strong steel for a strong arm. Take your pick!")
        .add_choice(DialogueChoice::new("Thanks.", None));

    let upgrade_node = DialogueNode::new(2, "Blacksmith", "Aye, for 500 gold I can improve your gear. The metal speaks to me, and I can make it sing!")
        .add_choice(
            DialogueChoice::new("Upgrade my equipment. [500 gold]", None)
                .with_action(DialogueAction::UpgradeEquipment)
                .with_condition(DialogueCondition::HasGold(500))
        )
        .add_choice(DialogueChoice::new("Maybe later.", None));

    let story_node = DialogueNode::new(3, "Blacksmith", "The forge calls to the worthy, no matter where it stands. I follow the ore, and it led me here. Besides, adventurers pay well!")
        .add_choice(DialogueChoice::new("Wise philosophy. Show me your wares.", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("May your forge burn bright.", None));

    DialogueTree::new(root)
        .add_node(shop_node)
        .add_node(upgrade_node)
        .add_node(story_node)
}

fn create_enchanter_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Enchanter", "Ah, I sense magical potential in you... The arcane arts offer power beyond steel. Care to browse my collection?")
        .add_choice(DialogueChoice::new("Show me your magical wares.", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("Can you identify my items?", Some(2)))
        .add_choice(DialogueChoice::new("Tell me about magic in this dungeon.", Some(3)))
        .add_choice(DialogueChoice::new("Farewell, mage.", None));

    let shop_node = DialogueNode::new(1, "Enchanter", "Scrolls, wands, and artifacts of power. Handle with care!")
        .add_choice(DialogueChoice::new("Thank you.", None));

    let identify_node = DialogueNode::new(2, "Enchanter", "For 200 gold, I shall reveal the true nature of all items in your possession.")
        .add_choice(
            DialogueChoice::new("Identify all my items. [200 gold]", None)
                .with_action(DialogueAction::IdentifyItems)
                .with_condition(DialogueCondition::HasGold(200))
        )
        .add_choice(DialogueChoice::new("Perhaps another time.", None));

    let lore_node = DialogueNode::new(3, "Enchanter", "This place pulses with ancient magic. The deeper you go, the stronger it grows. The Demon King's power warps reality itself. Be wary of curses!")
        .add_choice(DialogueChoice::new("I'll be careful. Your wares?", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("Thank you for the warning.", None));

    DialogueTree::new(root)
        .add_node(shop_node)
        .add_node(identify_node)
        .add_node(lore_node)
}

fn create_exotic_trader_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Exotic Trader", "Psst... you look like someone who appreciates the finer things. I deal in items most merchants have never even seen.")
        .add_choice(DialogueChoice::new("Show me your exotic goods.", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("Where do you get these items?", Some(2)))
        .add_choice(DialogueChoice::new("This seems suspicious...", Some(3)))
        .add_choice(DialogueChoice::new("Not interested.", None));

    let shop_node = DialogueNode::new(1, "Exotic Trader", "Legendary artifacts, mythic weapons... for those who can afford them!")
        .add_choice(DialogueChoice::new("Impressive.", None));

    let source_node = DialogueNode::new(2, "Exotic Trader", "Let's just say I have... connections in places most dare not tread. The deeper realms hold treasures beyond imagination.")
        .add_choice(DialogueChoice::new("Intriguing. Show me.", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("I'll take my chances.", None));

    let suspicious_node = DialogueNode::new(3, "Exotic Trader", "Suspicious? Friend, I'm the most honest trader in these depths. Where else will you find Mythic-grade equipment? The alternative is... dying without them.")
        .add_choice(DialogueChoice::new("Fair point. What do you have?", Some(1)).with_action(DialogueAction::OpenShop))
        .add_choice(DialogueChoice::new("I'll manage.", None));

    DialogueTree::new(root)
        .add_node(shop_node)
        .add_node(source_node)
        .add_node(suspicious_node)
}

fn create_priest_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Priest", "Blessings upon you, child. The light shines even in these dark depths. How may I serve?")
        .add_choice(DialogueChoice::new("I need healing.", Some(1)))
        .add_choice(DialogueChoice::new("Grant me your blessing.", Some(2)))
        .add_choice(DialogueChoice::new("I seek to cure my ailments.", Some(3)))
        .add_choice(DialogueChoice::new("May the light guide you.", None));

    let heal_node = DialogueNode::new(1, "Priest", "The light flows through me to heal your wounds. For 100 gold, I can fully restore your health and mana.")
        .add_choice(
            DialogueChoice::new("Heal me. [100 gold]", Some(4))
                .with_action(DialogueAction::FullRestore)
                .with_condition(DialogueCondition::HasGold(100))
        )
        .add_choice(DialogueChoice::new("A small healing will do. [Free]", None).with_action(DialogueAction::Heal(30)))
        .add_choice(DialogueChoice::new("Not now.", None));

    let blessing_node = DialogueNode::new(2, "Priest", "I can bestow upon you the blessing of protection, strengthening your defenses against the darkness.")
        .add_choice(
            DialogueChoice::new("Grant me protection. [50 gold]", None)
                .with_action(DialogueAction::GrantBuff(BuffType::Protection))
                .with_condition(DialogueCondition::HasGold(50))
        )
        .add_choice(DialogueChoice::new("Perhaps later.", None));

    let cure_node = DialogueNode::new(3, "Priest", "I sense corruption upon you. Let me purify your body and soul.")
        .add_choice(
            DialogueChoice::new("Cure my afflictions. [75 gold]", None)
                .with_action(DialogueAction::CureStatus)
                .with_condition(DialogueCondition::HasGold(75))
        )
        .add_choice(DialogueChoice::new("I can endure.", None));

    let healed_node = DialogueNode::new(4, "Priest", "The light has restored you. Go forth with renewed vigor!")
        .add_choice(DialogueChoice::new("Thank you, holy one.", None));

    DialogueTree::new(root)
        .add_node(heal_node)
        .add_node(blessing_node)
        .add_node(cure_node)
        .add_node(healed_node)
}

fn create_herbalist_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Herbalist", "Greetings, seeker. Nature provides all we need to heal and thrive. What remedy do you seek?")
        .add_choice(DialogueChoice::new("I need healing.", Some(1)))
        .add_choice(DialogueChoice::new("Do you have any protective remedies?", Some(2)))
        .add_choice(DialogueChoice::new("Cure my poison.", Some(3)))
        .add_choice(DialogueChoice::new("Farewell.", None));

    let heal_node = DialogueNode::new(1, "Herbalist", "My healing salve can mend your wounds. A generous helping costs 50 gold.")
        .add_choice(
            DialogueChoice::new("Heal me. [50 gold]", None)
                .with_action(DialogueAction::Heal(50))
                .with_condition(DialogueCondition::HasGold(50))
        )
        .add_choice(DialogueChoice::new("A small treatment. [Free]", None).with_action(DialogueAction::Heal(20)))
        .add_choice(DialogueChoice::new("Maybe later.", None));

    let remedy_node = DialogueNode::new(2, "Herbalist", "I can brew a potion of regeneration that will heal you over time. 75 gold for this potent remedy.")
        .add_choice(
            DialogueChoice::new("Give me regeneration. [75 gold]", None)
                .with_action(DialogueAction::GrantBuff(BuffType::Regeneration))
                .with_condition(DialogueCondition::HasGold(75))
        )
        .add_choice(DialogueChoice::new("Not now.", None));

    let cure_node = DialogueNode::new(3, "Herbalist", "Ah, I can see the venom coursing through you. My antidote will cleanse you.")
        .add_choice(
            DialogueChoice::new("Cure my poison. [40 gold]", None)
                .with_action(DialogueAction::CureStatus)
                .with_condition(DialogueCondition::HasGold(40))
        )
        .add_choice(DialogueChoice::new("I'll manage.", None));

    DialogueTree::new(root)
        .add_node(heal_node)
        .add_node(remedy_node)
        .add_node(cure_node)
}

fn create_field_medic_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Field Medic", "Easy there, soldier. Let me take a look at those wounds. Combat medicine is my specialty.")
        .add_choice(DialogueChoice::new("Patch me up.", Some(1)))
        .add_choice(DialogueChoice::new("Got anything for the road?", Some(2)))
        .add_choice(DialogueChoice::new("How do you survive down here?", Some(3)))
        .add_choice(DialogueChoice::new("Stay sharp, medic.", None));

    let heal_node = DialogueNode::new(1, "Field Medic", "Hold still. This might sting, but you'll be battle-ready in no time. 60 gold for full treatment.")
        .add_choice(
            DialogueChoice::new("Full treatment. [60 gold]", None)
                .with_action(DialogueAction::FullRestore)
                .with_condition(DialogueCondition::HasGold(60))
        )
        .add_choice(DialogueChoice::new("Just bandage me up. [Free]", None).with_action(DialogueAction::Heal(25)))
        .add_choice(DialogueChoice::new("I can walk it off.", None));

    let supplies_node = DialogueNode::new(2, "Field Medic", "Here, take this healing potion. Consider it professional courtesy. Just come back alive.")
        .add_choice(DialogueChoice::new("Thanks, doc.", None).with_action(DialogueAction::GiveItem(ItemKind::HealthPotion, Rarity::Common)));

    let survival_node = DialogueNode::new(3, "Field Medic", "Rule one: never fight what you can avoid. Rule two: always have an escape route. Rule three: a live coward gathers more gold than a dead hero.")
        .add_choice(DialogueChoice::new("Words to live by.", None));

    DialogueTree::new(root)
        .add_node(heal_node)
        .add_node(supplies_node)
        .add_node(survival_node)
}

fn create_elder_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Village Elder", "Ah, another brave soul ventures into the depths. I have seen many like you come... few return. Perhaps you will be different.")
        .add_choice(DialogueChoice::new("Do you have a quest for me?", Some(1)))
        .add_choice(DialogueChoice::new("Tell me about this dungeon.", Some(2)))
        .add_choice(DialogueChoice::new("Any advice for survival?", Some(3)))
        .add_choice(DialogueChoice::new("Farewell, elder.", None));

    let quest_node = DialogueNode::new(1, "Village Elder", "Indeed. The deeper levels are plagued by a great evil. Defeat 20 monsters and I shall reward you handsomely.")
        .add_choice(
            DialogueChoice::new("I accept this quest.", None)
                .with_action(DialogueAction::StartQuest(1))
        )
        .add_choice(DialogueChoice::new("Perhaps later.", None));

    let lore_node = DialogueNode::new(2, "Village Elder", "Long ago, this was a great kingdom. The Demon King corrupted it from within. Now only monsters remain... and the brave few who oppose them.")
        .add_choice(DialogueChoice::new("How do I defeat the Demon King?", Some(4)))
        .add_choice(DialogueChoice::new("Thank you for the history lesson.", None));

    let advice_node = DialogueNode::new(3, "Village Elder", "Trust in your skills, but know when to retreat. The dungeon has many secrets - shrines that heal, hidden treasures, and allies in unexpected places.")
        .add_choice(DialogueChoice::new("I'll remember that.", None));

    let demon_king_node = DialogueNode::new(4, "Village Elder", "He dwells on the 30th level, guarded by his most powerful servants. You must grow strong, find legendary equipment, and steel your resolve. Only then will you stand a chance.")
        .add_choice(DialogueChoice::new("I will not fail.", None));

    DialogueTree::new(root)
        .add_node(quest_node)
        .add_node(lore_node)
        .add_node(advice_node)
        .add_node(demon_king_node)
}

fn create_adventurer_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Adventurer", "Hey there, fellow explorer! Good to see a friendly face in these depths. Name's Garrett. You?")
        .add_choice(DialogueChoice::new("Looking for party members?", Some(1)))
        .add_choice(DialogueChoice::new("Find anything good down here?", Some(2)))
        .add_choice(DialogueChoice::new("Watch your back out there.", None));

    let party_node = DialogueNode::new(1, "Adventurer", "Ha! I work better alone, but I respect the offer. Tell you what - clear out some of these beasts and I'll share my stash with you.")
        .add_choice(
            DialogueChoice::new("Deal. How many?", None)
                .with_action(DialogueAction::StartQuest(2))
        )
        .add_choice(DialogueChoice::new("Maybe another time.", None));

    let treasure_node = DialogueNode::new(2, "Adventurer", "A few trinkets here and there. The real treasure is deeper, but so are the dangers. Here, take this - you look like you could use it.")
        .add_choice(DialogueChoice::new("Thanks!", None).with_action(DialogueAction::GiveGold(50)));

    DialogueTree::new(root)
        .add_node(party_node)
        .add_node(treasure_node)
}

fn create_mysterious_stranger_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "???", "...You see me. Interesting. Most cannot perceive what lurks between shadows.")
        .add_choice(DialogueChoice::new("Who are you?", Some(1)))
        .add_choice(DialogueChoice::new("What do you want?", Some(2)))
        .add_choice(DialogueChoice::new("*step back slowly*", None));

    let who_node = DialogueNode::new(1, "Mysterious Stranger", "Names have power. Mine is not freely given. I am... a watcher. A guide for those worthy of dark truths.")
        .add_choice(DialogueChoice::new("What truths?", Some(3)))
        .add_choice(DialogueChoice::new("You're clearly mad.", None));

    let want_node = DialogueNode::new(2, "Mysterious Stranger", "I seek those who would challenge the natural order. The Demon King's throne... it could be yours.")
        .add_choice(DialogueChoice::new("I'm listening.", Some(3)))
        .add_choice(DialogueChoice::new("I'm no usurper. I'm here to destroy him.", Some(4)));

    let dark_truth_node = DialogueNode::new(3, "Mysterious Stranger", "The path to power demands sacrifice. Bring me 1000 gold, and I shall reveal secrets hidden from mortal eyes.")
        .add_choice(
            DialogueChoice::new("*hand over 1000 gold*", None)
                .with_action(DialogueAction::RevealMap)
                .with_condition(DialogueCondition::HasGold(1000))
        )
        .add_choice(DialogueChoice::new("I'll find my own way.", None));

    let hero_node = DialogueNode::new(4, "Mysterious Stranger", "Ah, a hero. How... quaint. Very well. Perhaps our goals align after all. Destroy him if you can. I shall be watching.")
        .add_choice(DialogueChoice::new("*nod and leave*", None));

    DialogueTree::new(root)
        .add_node(who_node)
        .add_node(want_node)
        .add_node(dark_truth_node)
        .add_node(hero_node)
}

fn create_scholar_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Scholar", "Fascinating! A living specimen of adventurer-class humanoid! I must document your journey for my research!")
        .add_choice(DialogueChoice::new("Research? What are you studying?", Some(1)))
        .add_choice(DialogueChoice::new("Do you have any useful information?", Some(2)))
        .add_choice(DialogueChoice::new("Um, good luck with that.", None));

    let research_node = DialogueNode::new(1, "Scholar", "The dungeon ecosystem, of course! The monsters, the magic, the architecture! Did you know the walls rearrange themselves? Remarkable!")
        .add_choice(DialogueChoice::new("That explains why I keep getting lost.", Some(2)))
        .add_choice(DialogueChoice::new("Riveting. Goodbye.", None));

    let info_node = DialogueNode::new(2, "Scholar", "Indeed! My research has revealed much about this floor. For a modest contribution to science, I can share my findings.")
        .add_choice(
            DialogueChoice::new("Tell me about this floor. [150 gold]", None)
                .with_action(DialogueAction::RevealFloorInfo)
                .with_condition(DialogueCondition::HasGold(150))
        )
        .add_choice(DialogueChoice::new("I'll explore myself.", None));

    DialogueTree::new(root)
        .add_node(research_node)
        .add_node(info_node)
}

fn create_cartographer_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Cartographer", "Greetings, traveler. I've mapped much of these depths. Perhaps my charts can be of use to you?")
        .add_choice(DialogueChoice::new("Can you show me a map?", Some(1)))
        .add_choice(DialogueChoice::new("Where are the stairs?", Some(2)))
        .add_choice(DialogueChoice::new("No thanks.", None));

    let map_node = DialogueNode::new(1, "Cartographer", "For 200 gold, I can provide you with a complete map of this floor. Every room, every corridor revealed.")
        .add_choice(
            DialogueChoice::new("I'll take the map. [200 gold]", None)
                .with_action(DialogueAction::RevealMap)
                .with_condition(DialogueCondition::HasGold(200))
        )
        .add_choice(DialogueChoice::new("Too expensive.", None));

    let stairs_node = DialogueNode::new(2, "Cartographer", "For 100 gold, I can mark the location of the stairs leading deeper into the dungeon.")
        .add_choice(
            DialogueChoice::new("Show me the stairs. [100 gold]", None)
                .with_action(DialogueAction::TeleportToStairs)
                .with_condition(DialogueCondition::HasGold(100))
        )
        .add_choice(DialogueChoice::new("I'll find them myself.", None));

    DialogueTree::new(root)
        .add_node(map_node)
        .add_node(stairs_node)
}

fn create_sage_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Sage", "Seek you wisdom, young one? I have studied the arcane for centuries. Perhaps I can illuminate your path.")
        .add_choice(DialogueChoice::new("What can you tell me about this place?", Some(1)))
        .add_choice(DialogueChoice::new("Can you identify my items?", Some(2)))
        .add_choice(DialogueChoice::new("Grant me your wisdom.", Some(3)))
        .add_choice(DialogueChoice::new("I seek my own wisdom.", None));

    let lore_node = DialogueNode::new(1, "Sage", "This dungeon was once a grand castle, before darkness claimed it. Each level descends into older, more dangerous depths. The magic here grows stronger - and more corrupt - the deeper you go.")
        .add_choice(DialogueChoice::new("How deep does it go?", Some(4)))
        .add_choice(DialogueChoice::new("Useful information. Thanks.", None));

    let identify_node = DialogueNode::new(2, "Sage", "My mystical sight can reveal the true nature of items. For 150 gold, all shall be known.")
        .add_choice(
            DialogueChoice::new("Identify everything. [150 gold]", None)
                .with_action(DialogueAction::IdentifyItems)
                .with_condition(DialogueCondition::HasGold(150))
        )
        .add_choice(DialogueChoice::new("Perhaps later.", None));

    let buff_node = DialogueNode::new(3, "Sage", "I can grant you true sight, allowing you to see that which is hidden. 100 gold for this boon.")
        .add_choice(
            DialogueChoice::new("Grant me true sight. [100 gold]", None)
                .with_action(DialogueAction::GrantBuff(BuffType::TrueSight))
                .with_condition(DialogueCondition::HasGold(100))
        )
        .add_choice(DialogueChoice::new("Not now.", None));

    let depth_node = DialogueNode::new(4, "Sage", "Thirty levels separate you from the Demon King. Each fifth level holds a powerful boss. Prepare well, young one, for the challenges ahead grow ever more dire.")
        .add_choice(DialogueChoice::new("I will be ready.", None));

    DialogueTree::new(root)
        .add_node(lore_node)
        .add_node(identify_node)
        .add_node(buff_node)
        .add_node(depth_node)
}

fn create_artificer_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Artificer", "Ah, another customer! I specialize in enhancing equipment to their maximum potential. Interested?")
        .add_choice(DialogueChoice::new("Can you upgrade my gear?", Some(1)))
        .add_choice(DialogueChoice::new("What's your process?", Some(2)))
        .add_choice(DialogueChoice::new("Not right now.", None));

    let upgrade_node = DialogueNode::new(1, "Artificer", "For 750 gold, I can infuse your equipment with magical energy, permanently increasing its power. It's a complex process, but the results speak for themselves.")
        .add_choice(
            DialogueChoice::new("Upgrade my equipment. [750 gold]", None)
                .with_action(DialogueAction::UpgradeEquipment)
                .with_condition(DialogueCondition::HasGold(750))
        )
        .add_choice(DialogueChoice::new("Maybe when I have more gold.", None));

    let process_node = DialogueNode::new(2, "Artificer", "I extract magical essence from the dungeon itself and bind it to your equipment. The deeper the floor, the more potent the essence. That's why I set up shop here!")
        .add_choice(DialogueChoice::new("Fascinating. Upgrade my gear?", Some(1)))
        .add_choice(DialogueChoice::new("Good to know.", None));

    DialogueTree::new(root)
        .add_node(upgrade_node)
        .add_node(process_node)
}

fn create_innkeeper_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Innkeeper", "Welcome to the Dungeon Rest! We've got beds, food, and drink. What'll it be?")
        .add_choice(DialogueChoice::new("I need rest.", Some(1)))
        .add_choice(DialogueChoice::new("Got any food?", Some(2)))
        .add_choice(DialogueChoice::new("How's business down here?", Some(3)))
        .add_choice(DialogueChoice::new("Just passing through.", None));

    let rest_node = DialogueNode::new(1, "Innkeeper", "A room for the night costs 80 gold. You'll wake up refreshed and ready for battle!")
        .add_choice(
            DialogueChoice::new("I'll take a room. [80 gold]", None)
                .with_action(DialogueAction::Rest)
                .with_condition(DialogueCondition::HasGold(80))
        )
        .add_choice(DialogueChoice::new("Too expensive.", None));

    let food_node = DialogueNode::new(2, "Innkeeper", "Fresh bread and hearty stew! 30 gold for a meal that'll keep you going.")
        .add_choice(
            DialogueChoice::new("I'll have a meal. [30 gold]", None)
                .with_action(DialogueAction::GiveItem(ItemKind::Feast, Rarity::Common))
                .with_condition(DialogueCondition::HasGold(30))
        )
        .add_choice(DialogueChoice::new("I'm not hungry.", None));

    let business_node = DialogueNode::new(3, "Innkeeper", "Surprisingly good! Adventurers always need a safe place to rest, and I'm the only inn for thirty floors. Ha!")
        .add_choice(DialogueChoice::new("Clever location.", None));

    DialogueTree::new(root)
        .add_node(rest_node)
        .add_node(food_node)
        .add_node(business_node)
}

fn create_ghost_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Wandering Spirit", "...mortal... you can see me? How... unusual. I have wandered here for... so long...")
        .add_choice(DialogueChoice::new("Who were you?", Some(1)))
        .add_choice(DialogueChoice::new("Can you help me?", Some(2)))
        .add_choice(DialogueChoice::new("Find peace, spirit.", None));

    let past_node = DialogueNode::new(1, "Wandering Spirit", "I was... an adventurer, like you. I came seeking glory... found only death. The Demon King's magic binds me here.")
        .add_choice(DialogueChoice::new("I'm sorry.", Some(3)))
        .add_choice(DialogueChoice::new("I'll avenge you.", Some(4)));

    let help_node = DialogueNode::new(2, "Wandering Spirit", "I cannot interact with the physical world... but I can see what you cannot. I know where danger lurks on this floor.")
        .add_choice(DialogueChoice::new("Show me.", None).with_action(DialogueAction::RevealFloorInfo))
        .add_choice(DialogueChoice::new("Thank you, spirit.", None));

    let sorry_node = DialogueNode::new(3, "Wandering Spirit", "Sorrow changes nothing. Go... succeed where I failed. End the Demon King's reign, and perhaps... we shall all find peace.")
        .add_choice(DialogueChoice::new("I will.", None));

    let avenge_node = DialogueNode::new(4, "Wandering Spirit", "Bold words... I hope you have the strength to back them. Take this knowledge - it may aid your quest.")
        .add_choice(DialogueChoice::new("Thank you.", None).with_action(DialogueAction::RevealMap));

    DialogueTree::new(root)
        .add_node(past_node)
        .add_node(help_node)
        .add_node(sorry_node)
        .add_node(avenge_node)
}

fn create_prisoner_spirit_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Imprisoned Soul", "Please... help me... I am bound by dark chains of magic. Free me, and I shall reward you!")
        .add_choice(DialogueChoice::new("How can I free you?", Some(1)))
        .add_choice(DialogueChoice::new("This could be a trap...", Some(2)))
        .add_choice(DialogueChoice::new("I cannot help you.", None));

    let how_node = DialogueNode::new(1, "Imprisoned Soul", "Defeat 10 undead creatures on this floor. Their destruction will weaken the magic binding me!")
        .add_choice(
            DialogueChoice::new("I accept this task.", None)
                .with_action(DialogueAction::StartQuest(3))
        )
        .add_choice(DialogueChoice::new("That sounds dangerous.", None));

    let trap_node = DialogueNode::new(2, "Imprisoned Soul", "I understand your caution. But I swear upon my soul, I am no trick of the Demon King. Please... I have been trapped for so long...")
        .add_choice(DialogueChoice::new("Very well, how can I help?", Some(1)))
        .add_choice(DialogueChoice::new("I'm sorry, I cannot risk it.", None));

    DialogueTree::new(root)
        .add_node(how_node)
        .add_node(trap_node)
}

fn create_shrine_keeper_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Shrine Keeper", "Welcome, pilgrim. This sacred place offers solace from the darkness. How may I serve you?")
        .add_choice(DialogueChoice::new("I seek healing.", Some(1)))
        .add_choice(DialogueChoice::new("What is this shrine?", Some(2)))
        .add_choice(DialogueChoice::new("Bless my journey.", Some(3)))
        .add_choice(DialogueChoice::new("May this shrine endure.", None));

    let heal_node = DialogueNode::new(1, "Shrine Keeper", "The shrine's power can restore you fully, but it requires an offering of 150 gold to maintain its magic.")
        .add_choice(
            DialogueChoice::new("Make an offering. [150 gold]", None)
                .with_action(DialogueAction::FullRestore)
                .with_condition(DialogueCondition::HasGold(150))
        )
        .add_choice(DialogueChoice::new("I cannot afford that.", Some(4)));

    let shrine_node = DialogueNode::new(2, "Shrine Keeper", "This shrine predates the dungeon itself. It is a beacon of hope in the darkness, powered by faith and sacrifice. As long as adventurers believe, it will endure.")
        .add_choice(DialogueChoice::new("A comforting thought.", None));

    let bless_node = DialogueNode::new(3, "Shrine Keeper", "I can invoke the shrine's blessing upon you. For 100 gold, you shall receive divine protection.")
        .add_choice(
            DialogueChoice::new("Bless me. [100 gold]", None)
                .with_action(DialogueAction::GrantBuff(BuffType::Protection))
                .with_condition(DialogueCondition::HasGold(100))
        )
        .add_choice(DialogueChoice::new("Not now.", None));

    let poor_node = DialogueNode::new(4, "Shrine Keeper", "The shrine recognizes your need. Accept this small blessing freely.")
        .add_choice(DialogueChoice::new("Thank you.", None).with_action(DialogueAction::Heal(30)));

    DialogueTree::new(root)
        .add_node(heal_node)
        .add_node(shrine_node)
        .add_node(bless_node)
        .add_node(poor_node)
}

fn create_bard_dialogue() -> DialogueTree {
    let root = DialogueNode::new(0, "Bard", "Ah, a new audience! Allow me to regale you with tales of heroism and woe! *strums lute dramatically*")
        .add_choice(DialogueChoice::new("Play me a song!", Some(1)))
        .add_choice(DialogueChoice::new("Do you know any useful information?", Some(2)))
        .add_choice(DialogueChoice::new("Maybe later, bard.", None));

    let song_node = DialogueNode::new(1, "Bard", "What manner of song would you like? A battle hymn to strengthen your arm? Or a gentle melody to soothe your wounds?")
        .add_choice(
            DialogueChoice::new("A battle hymn! [50 gold]", Some(3))
                .with_action(DialogueAction::GrantBuff(BuffType::Strength))
                .with_condition(DialogueCondition::HasGold(50))
        )
        .add_choice(
            DialogueChoice::new("A healing melody. [50 gold]", Some(3))
                .with_action(DialogueAction::GrantBuff(BuffType::Regeneration))
                .with_condition(DialogueCondition::HasGold(50))
        )
        .add_choice(
            DialogueChoice::new("A song of fortune! [50 gold]", Some(3))
                .with_action(DialogueAction::GrantBuff(BuffType::Fortune))
                .with_condition(DialogueCondition::HasGold(50))
        )
        .add_choice(DialogueChoice::new("On second thought...", None));

    let info_node = DialogueNode::new(2, "Bard", "I travel far and wide, collecting stories. The monsters here fear fire, the bosses guard great treasures, and the Demon King... well, that's the ending I'm still writing!")
        .add_choice(DialogueChoice::new("Helpful. Play me a song?", Some(1)))
        .add_choice(DialogueChoice::new("Thanks for the tips.", None));

    let song_done_node = DialogueNode::new(3, "Bard", "*plays an inspiring tune* May this melody carry you through the darkness! Now go forth and make a tale worth singing!")
        .add_choice(DialogueChoice::new("Thank you, bard!", None));

    DialogueTree::new(root)
        .add_node(song_node)
        .add_node(info_node)
        .add_node(song_done_node)
}

// ============================================================================
// Shop Inventory Generation
// ============================================================================

/// Generate shop inventory for a merchant NPC
fn create_shop_inventory(kind: NPCKind, dungeon_level: u32) -> Vec<ShopItem> {
    let mut inventory = Vec::new();

    match kind {
        NPCKind::Merchant => {
            // Basic supplies
            inventory.push(ShopItem::new(ItemKind::HealthPotion, Rarity::Common, 50, 5));
            inventory.push(ShopItem::new(ItemKind::ManaPotion, Rarity::Common, 40, 5));
            inventory.push(ShopItem::new(ItemKind::Bread, Rarity::Common, 15, 10));
            inventory.push(ShopItem::new(ItemKind::Torch, Rarity::Common, 25, 3));

            if dungeon_level >= 5 {
                inventory.push(ShopItem::new(ItemKind::HealthPotion, Rarity::Uncommon, 100, 3));
                inventory.push(ShopItem::new(ItemKind::PoisonResistPotion, Rarity::Common, 60, 2));
            }

            if dungeon_level >= 10 {
                inventory.push(ShopItem::new(ItemKind::ScrollTeleport, Rarity::Common, 150, 2));
                inventory.push(ShopItem::new(ItemKind::StrengthPotion, Rarity::Common, 80, 2));
            }
        }

        NPCKind::Blacksmith => {
            // Weapons and armor based on level
            if dungeon_level < 10 {
                inventory.push(ShopItem::new(ItemKind::ShortSword, Rarity::Common, 100, 1));
                inventory.push(ShopItem::new(ItemKind::LeatherArmor, Rarity::Common, 80, 1));
                inventory.push(ShopItem::new(ItemKind::WoodenShield, Rarity::Common, 60, 1));
            } else if dungeon_level < 20 {
                inventory.push(ShopItem::new(ItemKind::LongSword, Rarity::Uncommon, 250, 1));
                inventory.push(ShopItem::new(ItemKind::ChainMail, Rarity::Uncommon, 200, 1));
                inventory.push(ShopItem::new(ItemKind::IronShield, Rarity::Uncommon, 180, 1));
            } else {
                inventory.push(ShopItem::new(ItemKind::Greatsword, Rarity::Rare, 500, 1));
                inventory.push(ShopItem::new(ItemKind::PlateMail, Rarity::Rare, 450, 1));
                inventory.push(ShopItem::new(ItemKind::TowerShield, Rarity::Rare, 400, 1));
            }

            // Always stock some basic gear
            inventory.push(ShopItem::new(ItemKind::Dagger, Rarity::Common, 30, 2));
            inventory.push(ShopItem::new(ItemKind::LeatherCap, Rarity::Common, 40, 1));
            inventory.push(ShopItem::new(ItemKind::LeatherBoots, Rarity::Common, 35, 1));
        }

        NPCKind::Enchanter => {
            // Magical items and scrolls
            inventory.push(ShopItem::new(ItemKind::ScrollFireball, Rarity::Common, 200, 2));
            inventory.push(ShopItem::new(ItemKind::ScrollIceStorm, Rarity::Common, 200, 2));
            inventory.push(ShopItem::new(ItemKind::Wand, Rarity::Uncommon, 300, 1));
            inventory.push(ShopItem::new(ItemKind::Staff, Rarity::Uncommon, 250, 1));

            if dungeon_level >= 10 {
                inventory.push(ShopItem::new(ItemKind::ScrollLightning, Rarity::Uncommon, 350, 2));
                inventory.push(ShopItem::new(ItemKind::MageRobes, Rarity::Uncommon, 280, 1));
            }

            if dungeon_level >= 15 {
                inventory.push(ShopItem::new(ItemKind::RingOfMana, Rarity::Rare, 500, 1));
                inventory.push(ShopItem::new(ItemKind::WizardHat, Rarity::Rare, 400, 1));
            }
        }

        NPCKind::ExoticTrader => {
            // Rare and legendary items
            inventory.push(ShopItem::new(ItemKind::FlameSword, Rarity::Epic, 2000, 1));
            inventory.push(ShopItem::new(ItemKind::FrostBlade, Rarity::Epic, 2000, 1));
            inventory.push(ShopItem::new(ItemKind::DragonArmor, Rarity::Legendary, 5000, 1));
            inventory.push(ShopItem::new(ItemKind::AmuletOfTheGods, Rarity::Legendary, 4000, 1));
            inventory.push(ShopItem::new(ItemKind::RingOfTheAncients, Rarity::Legendary, 3500, 1));

            if dungeon_level >= 25 {
                inventory.push(ShopItem::new(ItemKind::DemonSlayer, Rarity::Mythic, 10000, 1));
                inventory.push(ShopItem::new(ItemKind::TitanPlate, Rarity::Mythic, 8000, 1));
            }
        }

        _ => {} // Non-merchant NPCs don't have shop inventory
    }

    inventory
}

// ============================================================================
// NPC Spawning and Management
// ============================================================================

/// Manager for all NPCs in the game
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct NPCManager {
    /// All NPCs currently in the game
    pub npcs: Vec<NPC>,
    /// Next available NPC ID
    next_id: NPCId,
    /// Active quests
    pub active_quests: Vec<Quest>,
    /// Completed quest IDs
    pub completed_quests: Vec<QuestId>,
}

impl NPCManager {
    /// Create a new NPC manager
    pub fn new() -> Self {
        Self {
            npcs: Vec::new(),
            next_id: 1,
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
        }
    }

    /// Spawn NPCs for a dungeon level
    pub fn spawn_npcs_for_level(
        &mut self,
        level: u32,
        rooms: &[(usize, usize)],  // Room centers
        rng: &mut impl Rng,
    ) {
        // Clear existing NPCs (they don't persist between floors)
        self.npcs.clear();

        // Determine how many NPCs to spawn (more on lower floors, fewer deep down)
        let base_count = match level {
            1..=5 => 3,
            6..=15 => 2,
            16..=25 => 2,
            _ => 1,
        };

        let npc_count = rng.gen_range(1..=base_count);

        // Collect valid NPC types for this level
        let valid_kinds: Vec<NPCKind> = [
            NPCKind::Merchant,
            NPCKind::Blacksmith,
            NPCKind::Enchanter,
            NPCKind::ExoticTrader,
            NPCKind::Elder,
            NPCKind::Adventurer,
            NPCKind::MysteriousStranger,
            NPCKind::Scholar,
            NPCKind::Priest,
            NPCKind::Herbalist,
            NPCKind::FieldMedic,
            NPCKind::Cartographer,
            NPCKind::Sage,
            NPCKind::Artificer,
            NPCKind::Innkeeper,
            NPCKind::Ghost,
            NPCKind::PrisonerSpirit,
            NPCKind::ShrineKeeper,
            NPCKind::Bard,
        ]
        .into_iter()
        .filter(|k| k.can_spawn_at_level(level))
        .collect();

        if valid_kinds.is_empty() || rooms.len() < 2 {
            return;
        }

        // Spawn NPCs in different rooms (skip room 0, that's the start)
        let mut available_rooms: Vec<usize> = (1..rooms.len()).collect();
        available_rooms.shuffle(rng);

        for i in 0..npc_count.min(available_rooms.len()) {
            let room_idx = available_rooms[i];
            let (x, y) = rooms[room_idx];
            let kind = valid_kinds[rng.gen_range(0..valid_kinds.len())];

            let npc = NPC::new(self.next_id, x, y, kind, level);
            self.next_id += 1;
            self.npcs.push(npc);
        }
    }

    /// Get NPC at position (if any)
    pub fn get_npc_at(&self, x: usize, y: usize) -> Option<&NPC> {
        self.npcs.iter().find(|npc| npc.x == x && npc.y == y)
    }

    /// Get mutable NPC at position (if any)
    pub fn get_npc_at_mut(&mut self, x: usize, y: usize) -> Option<&mut NPC> {
        self.npcs.iter_mut().find(|npc| npc.x == x && npc.y == y)
    }

    /// Get NPC by ID
    pub fn get_npc(&self, id: NPCId) -> Option<&NPC> {
        self.npcs.iter().find(|npc| npc.id == id)
    }

    /// Get mutable NPC by ID
    pub fn get_npc_mut(&mut self, id: NPCId) -> Option<&mut NPC> {
        self.npcs.iter_mut().find(|npc| npc.id == id)
    }

    /// Add a quest to active quests
    pub fn start_quest(&mut self, quest: Quest) {
        if !self.active_quests.iter().any(|q| q.id == quest.id) {
            self.active_quests.push(quest);
        }
    }

    /// Complete a quest and return rewards
    pub fn complete_quest(&mut self, quest_id: QuestId) -> Option<(u32, u32, Option<(ItemKind, Rarity)>)> {
        if let Some(idx) = self.active_quests.iter().position(|q| q.id == quest_id && q.is_complete()) {
            let quest = self.active_quests.remove(idx);
            self.completed_quests.push(quest_id);
            Some((quest.reward_gold, quest.reward_xp, quest.reward_item))
        } else {
            None
        }
    }

    /// Update quest progress for kill objectives
    pub fn update_kill_progress(&mut self, kill_count: u32) {
        for quest in &mut self.active_quests {
            for objective in &mut quest.objectives {
                if let QuestObjective::KillEnemies { target: _, current } = objective {
                    *current = kill_count;
                }
            }
            quest.update_completion();
        }
    }

    /// Update quest progress for reaching a level
    pub fn update_level_progress(&mut self, dungeon_level: u32) {
        for quest in &mut self.active_quests {
            for objective in &mut quest.objectives {
                if let QuestObjective::ReachLevel { target, completed } = objective {
                    if dungeon_level >= *target {
                        *completed = true;
                    }
                }
            }
            quest.update_completion();
        }
    }

    /// Check if a quest is completed
    pub fn is_quest_completed(&self, quest_id: QuestId) -> bool {
        self.completed_quests.contains(&quest_id)
    }

    /// Check if a quest is active
    pub fn is_quest_active(&self, quest_id: QuestId) -> bool {
        self.active_quests.iter().any(|q| q.id == quest_id)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialogue_tree() {
        let root = DialogueNode::new(0, "Test NPC", "Hello!")
            .add_choice(DialogueChoice::new("Hi!", Some(1)))
            .add_choice(DialogueChoice::new("Bye!", None));

        let node1 = DialogueNode::new(1, "Test NPC", "Nice to meet you!")
            .add_choice(DialogueChoice::new("Same!", None));

        let mut tree = DialogueTree::new(root).add_node(node1);

        // Start dialogue
        let start = tree.start();
        assert!(start.is_some());
        assert_eq!(start.unwrap().text, "Hello!");

        // Select first choice
        let (action, continues) = tree.select_choice(0);
        assert!(action.is_none());
        assert!(continues);

        // Check current node
        let current = tree.current();
        assert!(current.is_some());
        assert_eq!(current.unwrap().text, "Nice to meet you!");
    }

    #[test]
    fn test_npc_creation() {
        let npc = NPC::new(1, 5, 5, NPCKind::Merchant, 1);
        assert_eq!(npc.id, 1);
        assert_eq!(npc.x, 5);
        assert_eq!(npc.y, 5);
        assert!(!npc.shop_inventory.is_empty()); // Merchants have inventory
    }

    #[test]
    fn test_quest_completion() {
        let quest = Quest::new(1, "Test Quest", "Kill some enemies", 1)
            .add_objective(QuestObjective::KillEnemies { target: 10, current: 10 })
            .with_gold_reward(100);

        assert!(quest.is_complete());
    }

    #[test]
    fn test_npc_manager() {
        let mut manager = NPCManager::new();
        let rooms = vec![(5, 5), (15, 15), (25, 25)];
        let mut rng = rand::thread_rng();

        manager.spawn_npcs_for_level(1, &rooms, &mut rng);
        assert!(!manager.npcs.is_empty());
    }

    #[test]
    fn test_shop_inventory() {
        let inventory = create_shop_inventory(NPCKind::Merchant, 1);
        assert!(!inventory.is_empty());
        assert!(inventory.iter().any(|i| i.item == ItemKind::HealthPotion));
    }
}

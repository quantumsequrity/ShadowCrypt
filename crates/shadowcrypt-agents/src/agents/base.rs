//! Base agent implementation

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{AgentId, AgentKind, AgentState, AgentStats};
use crate::personality::Personality;
use crate::memory::Memory;
use crate::behavior::{Goal, BehaviorTree};
use crate::factions::FactionId;

/// The core agent structure that all agents share
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier
    pub id: AgentId,
    /// Agent kind/type
    pub kind: AgentKind,
    /// Display name
    pub name: String,
    /// Current position
    pub x: usize,
    pub y: usize,
    /// Current stats
    pub hp: i32,
    pub stats: AgentStats,
    /// Current state
    pub state: AgentState,
    /// Personality traits
    pub personality: Personality,
    /// Memory system
    pub memory: Memory,
    /// Current goals
    pub goals: Vec<Goal>,
    /// Behavior tree for decision making
    pub behavior: BehaviorTree,
    /// Faction membership
    pub faction: Option<FactionId>,
    /// Gold/currency
    pub gold: u32,
    /// Inventory items (item IDs)
    pub inventory: Vec<String>,
    /// Currently speaking dialogue
    pub current_dialogue: Option<String>,
    /// Last action taken
    pub last_action: Option<AgentAction>,
    /// Turn counter
    pub turn: u32,
    /// Is this agent visible
    pub visible: bool,
    /// Is this agent active
    pub active: bool,
    /// Custom data storage
    pub data: HashMap<String, AgentData>,
}

impl Agent {
    /// Creates a new agent
    pub fn new(kind: AgentKind, name: String, x: usize, y: usize) -> Self {
        let stats = kind.base_stats();
        Self {
            id: AgentId::new(),
            kind,
            name,
            x,
            y,
            hp: stats.max_hp,
            stats,
            state: AgentState::Idle,
            personality: Personality::random_for_kind(kind),
            memory: Memory::new(),
            goals: Vec::new(),
            behavior: BehaviorTree::default_for_kind(kind),
            faction: None,
            gold: 0,
            inventory: Vec::new(),
            current_dialogue: None,
            last_action: None,
            turn: 0,
            visible: kind.category() != super::AgentCategory::Environmental,
            active: true,
            data: HashMap::new(),
        }
    }

    /// Creates an agent with a specific ID
    pub fn with_id(mut self, id: AgentId) -> Self {
        self.id = id;
        self
    }

    /// Sets the agent's faction
    pub fn with_faction(mut self, faction: FactionId) -> Self {
        self.faction = Some(faction);
        self
    }

    /// Returns true if the agent is alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0 && self.state != AgentState::Dead
    }

    /// Returns true if the agent can be interacted with
    pub fn can_interact(&self) -> bool {
        self.is_alive() && self.state.is_passive()
    }

    /// Takes damage and returns actual damage taken
    pub fn take_damage(&mut self, damage: i32) -> i32 {
        let actual = (damage - self.stats.defense).max(1);
        self.hp -= actual;
        if self.hp <= 0 {
            self.hp = 0;
            self.state = AgentState::Dead;
        }
        actual
    }

    /// Heals the agent
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.stats.max_hp);
    }

    /// Moves the agent to a new position
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.state = AgentState::Moving;
    }

    /// Sets the agent's current dialogue
    pub fn say(&mut self, dialogue: &str) {
        self.current_dialogue = Some(dialogue.to_string());
        self.state = AgentState::Talking;
    }

    /// Clears the current dialogue
    pub fn stop_talking(&mut self) {
        self.current_dialogue = None;
        if self.state == AgentState::Talking {
            self.state = AgentState::Idle;
        }
    }

    /// Gets custom data
    pub fn get_data(&self, key: &str) -> Option<&AgentData> {
        self.data.get(key)
    }

    /// Sets custom data
    pub fn set_data(&mut self, key: &str, value: AgentData) {
        self.data.insert(key.to_string(), value);
    }

    /// Returns the distance to another position
    pub fn distance_to(&self, x: usize, y: usize) -> f64 {
        let dx = self.x as f64 - x as f64;
        let dy = self.y as f64 - y as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the manhattan distance to another position
    pub fn manhattan_distance_to(&self, x: usize, y: usize) -> usize {
        let dx = (self.x as i32 - x as i32).unsigned_abs() as usize;
        let dy = (self.y as i32 - y as i32).unsigned_abs() as usize;
        dx + dy
    }

    /// Adds a goal to the agent
    pub fn add_goal(&mut self, goal: Goal) {
        self.goals.push(goal);
        self.goals.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Removes completed goals
    pub fn cleanup_goals(&mut self) {
        self.goals.retain(|g| !g.completed);
    }

    /// Gets the current top goal
    pub fn current_goal(&self) -> Option<&Goal> {
        self.goals.first()
    }

    /// Returns the agent's display glyph
    pub fn glyph(&self) -> char {
        self.kind.glyph()
    }

    /// Returns the agent's display color as RGB
    pub fn color(&self) -> (u8, u8, u8) {
        match self.kind.category() {
            super::AgentCategory::Npc => (100, 200, 255), // Light blue
            super::AgentCategory::Enemy => (255, 100, 100), // Light red
            super::AgentCategory::Companion => (100, 255, 100), // Light green
            super::AgentCategory::Environmental => (200, 200, 100), // Yellow
            super::AgentCategory::System => (200, 100, 200), // Purple
        }
    }
}

/// Actions an agent can take
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentAction {
    /// Move in a direction
    Move { dx: i32, dy: i32 },
    /// Attack a target
    Attack { target_id: AgentId },
    /// Use a skill/ability
    UseSkill { skill_name: String, target: Option<AgentId> },
    /// Speak dialogue
    Speak { message: String, target: Option<AgentId> },
    /// Trade with another agent
    Trade { partner_id: AgentId, offer: Vec<String>, request: Vec<String> },
    /// Pick up an item
    PickUp { item_id: String },
    /// Drop an item
    Drop { item_id: String },
    /// Use an item
    UseItem { item_id: String },
    /// Rest to recover
    Rest,
    /// Wait/do nothing
    Wait,
    /// Flee from danger
    Flee { from_x: usize, from_y: usize },
    /// Follow a target
    Follow { target_id: AgentId },
    /// Patrol to a point
    Patrol { waypoint_x: usize, waypoint_y: usize },
    /// Interact with environment
    Interact { target_x: usize, target_y: usize },
    /// Cast a spell
    Cast { spell_name: String, target_x: usize, target_y: usize },
    /// Hide from view
    Hide,
    /// Search the area
    Search { area_x: usize, area_y: usize, radius: usize },
    /// Craft an item
    Craft { recipe_name: String },
    /// Custom action
    Custom { action_type: String, params: HashMap<String, String> },
}

/// Custom data that can be stored on an agent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentData {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Vec(Vec<AgentData>),
    Map(HashMap<String, AgentData>),
}

impl AgentData {
    pub fn as_int(&self) -> Option<i64> {
        if let Self::Int(v) = self { Some(*v) } else { None }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Self::Float(v) = self { Some(*v) } else { None }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Self::String(v) = self { Some(v) } else { None }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self { Some(*v) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new(AgentKind::Blacksmith, "Bjorn".to_string(), 10, 10);
        assert_eq!(agent.name, "Bjorn");
        assert_eq!(agent.kind, AgentKind::Blacksmith);
        assert!(agent.is_alive());
    }

    #[test]
    fn test_agent_damage() {
        let mut agent = Agent::new(AgentKind::Guard, "Guard".to_string(), 5, 5);
        let initial_hp = agent.hp;
        agent.take_damage(10);
        assert!(agent.hp < initial_hp);
    }

    #[test]
    fn test_agent_death() {
        let mut agent = Agent::new(AgentKind::GoblinScout, "Goblin".to_string(), 0, 0);
        agent.take_damage(1000);
        assert!(!agent.is_alive());
        assert_eq!(agent.state, AgentState::Dead);
    }
}

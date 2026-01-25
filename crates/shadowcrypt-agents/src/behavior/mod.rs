//! Behavior system for agents
//!
//! Implements behavior trees, goals, and decision-making systems.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::prelude::*;

use crate::agents::{Agent, AgentId, AgentKind, AgentCategory, AgentState, AgentAction, AgentManager};

/// A goal that an agent is trying to achieve
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Goal {
    /// Type of goal
    pub kind: GoalKind,
    /// Priority level
    pub priority: GoalPriority,
    /// Whether the goal is completed
    pub completed: bool,
    /// Optional target agent
    pub target: Option<AgentId>,
    /// Optional target position
    pub target_pos: Option<(usize, usize)>,
    /// Progress (0.0 to 1.0)
    pub progress: f32,
    /// Timeout in turns
    pub timeout: Option<u32>,
    /// Custom data
    pub data: HashMap<String, String>,
}

impl Goal {
    /// Creates a new goal
    pub fn new(kind: GoalKind, priority: GoalPriority) -> Self {
        Self {
            kind,
            priority,
            completed: false,
            target: None,
            target_pos: None,
            progress: 0.0,
            timeout: None,
            data: HashMap::new(),
        }
    }

    /// Sets the target agent
    pub fn with_target(mut self, target: AgentId) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the target position
    pub fn with_position(mut self, x: usize, y: usize) -> Self {
        self.target_pos = Some((x, y));
        self
    }

    /// Sets a timeout
    pub fn with_timeout(mut self, turns: u32) -> Self {
        self.timeout = Some(turns);
        self
    }
}

/// Types of goals agents can have
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GoalKind {
    // Basic goals
    Idle,
    Move,
    Rest,
    Survive,

    // Combat goals
    AttackEnemy,
    DefendSelf,
    Flee,
    Charge,
    Pursue,
    Ambush,
    Rampage,
    DrainLife,
    RaiseUndead,
    DefendTerritory,

    // Social goals
    Socialize,
    Trade,
    Beg,
    Steal,
    Command,
    Rally,
    AlertAllies,
    SupportAllies,
    CastSpell,

    // Work goals
    Work,
    Craft,
    Practice,
    Study,
    Teach,
    Patrol,
    ProtectArea,
    Hunt,
    Explore,

    // Support goals
    Heal,
    Bless,
    SupportMaster,
    ProtectMaster,
    Follow,
    Scout,
    AlertMaster,
    PickUpItems,

    // Entertainment
    Entertain,
    Play,

    // System goals
    ManageQuests,
    TrackProgress,
    TellStory,
    CreateAtmosphere,
    Guide,
    RewardPlayer,
    BalanceGame,
    AdjustDifficulty,

    // Environmental goals
    ManageWeather,
    ManageTime,
    ManageSeason,
    ManageEnvironment,
    SpawnCreatures,
    ManagePopulation,
    TriggerEvents,
    CreateHazard,
    DamageArea,
    CreateChallenges,

    // Custom
    Custom,
}

/// Priority levels for goals
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GoalPriority {
    /// Optional goals
    Low = 1,
    /// Normal priority
    Medium = 2,
    /// Important goals
    High = 3,
    /// Life or death
    Critical = 4,
}

/// Behavior tree for decision making
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BehaviorTree {
    /// Root node
    pub root: BehaviorNode,
    /// Custom behavior functions (stored as names)
    pub custom_behaviors: Vec<String>,
}

impl BehaviorTree {
    /// Creates a default behavior tree for an agent kind
    pub fn default_for_kind(kind: AgentKind) -> Self {
        let root = match kind.category() {
            AgentCategory::Npc => Self::npc_behavior(),
            AgentCategory::Enemy => Self::enemy_behavior(),
            AgentCategory::Companion => Self::companion_behavior(),
            AgentCategory::Environmental => Self::environmental_behavior(),
            AgentCategory::System => Self::system_behavior(),
        };

        Self {
            root,
            custom_behaviors: Vec::new(),
        }
    }

    /// Creates NPC behavior tree
    fn npc_behavior() -> BehaviorNode {
        BehaviorNode::Selector(vec![
            // Priority 1: Flee if in danger
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("is_in_danger".to_string()),
                BehaviorNode::Action(BehaviorAction::Flee),
            ]),
            // Priority 2: Respond to player
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("player_nearby".to_string()),
                BehaviorNode::Action(BehaviorAction::Greet),
            ]),
            // Priority 3: Do work
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("is_work_time".to_string()),
                BehaviorNode::Action(BehaviorAction::Work),
            ]),
            // Priority 4: Socialize
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("has_nearby_npc".to_string()),
                BehaviorNode::RandomChance(0.3),
                BehaviorNode::Action(BehaviorAction::Socialize),
            ]),
            // Default: Wander
            BehaviorNode::Action(BehaviorAction::Wander),
        ])
    }

    /// Creates enemy behavior tree
    fn enemy_behavior() -> BehaviorNode {
        BehaviorNode::Selector(vec![
            // Priority 1: Flee if very low HP
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("hp_critical".to_string()),
                BehaviorNode::Action(BehaviorAction::Flee),
            ]),
            // Priority 2: Attack visible enemy
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("enemy_visible".to_string()),
                BehaviorNode::Action(BehaviorAction::Attack),
            ]),
            // Priority 3: Pursue last known position
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("has_last_seen".to_string()),
                BehaviorNode::Action(BehaviorAction::Pursue),
            ]),
            // Priority 4: Patrol
            BehaviorNode::Action(BehaviorAction::Patrol),
        ])
    }

    /// Creates companion behavior tree
    fn companion_behavior() -> BehaviorNode {
        BehaviorNode::Selector(vec![
            // Priority 1: Protect master if low HP
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("master_in_danger".to_string()),
                BehaviorNode::Action(BehaviorAction::ProtectMaster),
            ]),
            // Priority 2: Attack threats
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("enemy_near_master".to_string()),
                BehaviorNode::Action(BehaviorAction::Attack),
            ]),
            // Priority 3: Follow master
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition("too_far_from_master".to_string()),
                BehaviorNode::Action(BehaviorAction::FollowMaster),
            ]),
            // Default: Stay near master
            BehaviorNode::Action(BehaviorAction::Stay),
        ])
    }

    /// Creates environmental behavior tree
    fn environmental_behavior() -> BehaviorNode {
        BehaviorNode::Sequence(vec![
            BehaviorNode::Action(BehaviorAction::ManageEnvironment),
        ])
    }

    /// Creates system behavior tree
    fn system_behavior() -> BehaviorNode {
        BehaviorNode::Sequence(vec![
            BehaviorNode::Action(BehaviorAction::ManageSystem),
        ])
    }

    /// Decides an action for an agent
    pub fn decide(&self, agent: &Agent, manager: &AgentManager) -> Option<AgentAction> {
        self.root.execute(agent, manager)
    }
}

/// Node in a behavior tree
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BehaviorNode {
    /// Execute children until one succeeds
    Selector(Vec<BehaviorNode>),
    /// Execute children until one fails
    Sequence(Vec<BehaviorNode>),
    /// Check a condition
    Condition(String),
    /// Execute an action
    Action(BehaviorAction),
    /// Random chance (0.0 to 1.0)
    RandomChance(f32),
    /// Invert the result
    Inverter(Box<BehaviorNode>),
    /// Always succeed
    Succeeder(Box<BehaviorNode>),
    /// Repeat N times
    Repeater(Box<BehaviorNode>, u32),
}

impl Default for BehaviorNode {
    fn default() -> Self {
        Self::Action(BehaviorAction::Idle)
    }
}

impl BehaviorNode {
    /// Executes the behavior node
    pub fn execute(&self, agent: &Agent, manager: &AgentManager) -> Option<AgentAction> {
        match self {
            Self::Selector(children) => {
                for child in children {
                    if let Some(action) = child.execute(agent, manager) {
                        return Some(action);
                    }
                }
                None
            }
            Self::Sequence(children) => {
                let mut last_action = None;
                for child in children {
                    match child {
                        Self::Condition(name) => {
                            if !check_condition(name, agent, manager) {
                                return None;
                            }
                        }
                        Self::RandomChance(chance) => {
                            if rand::random::<f32>() > *chance {
                                return None;
                            }
                        }
                        _ => {
                            last_action = child.execute(agent, manager);
                        }
                    }
                }
                last_action
            }
            Self::Condition(name) => {
                if check_condition(name, agent, manager) {
                    Some(AgentAction::Wait)
                } else {
                    None
                }
            }
            Self::Action(action) => action.to_agent_action(agent, manager),
            Self::RandomChance(chance) => {
                if rand::random::<f32>() <= *chance {
                    Some(AgentAction::Wait)
                } else {
                    None
                }
            }
            Self::Inverter(child) => {
                if child.execute(agent, manager).is_some() {
                    None
                } else {
                    Some(AgentAction::Wait)
                }
            }
            Self::Succeeder(child) => {
                child.execute(agent, manager);
                Some(AgentAction::Wait)
            }
            Self::Repeater(child, count) => {
                for _ in 0..*count {
                    child.execute(agent, manager);
                }
                Some(AgentAction::Wait)
            }
        }
    }
}

/// Check a named condition
fn check_condition(name: &str, agent: &Agent, manager: &AgentManager) -> bool {
    match name {
        "is_alive" => agent.is_alive(),
        "is_idle" => agent.state == AgentState::Idle,
        "is_in_danger" => agent.hp < agent.stats.max_hp / 4,
        "hp_critical" => agent.hp < agent.stats.max_hp / 5,
        "hp_low" => agent.hp < agent.stats.max_hp / 2,
        "player_nearby" => {
            // Check if any agent within 5 tiles is player-like
            manager.in_radius(agent.x, agent.y, 5).len() > 1
        }
        "enemy_visible" => {
            // Check for visible enemies
            manager.in_radius(agent.x, agent.y, 10)
                .iter()
                .any(|a| a.kind.category() != agent.kind.category() && a.is_alive())
        }
        "has_nearby_npc" => {
            manager.in_radius(agent.x, agent.y, 3)
                .iter()
                .any(|a| a.id != agent.id && a.kind.category() == AgentCategory::Npc)
        }
        "has_last_seen" => agent.memory.last_enemy_position.is_some(),
        "too_far_from_master" => {
            // Companions should stay within 5 tiles of master
            if let Some(_master_id) = agent.data.get("master") {
                // Simplified check
                false
            } else {
                false
            }
        }
        "master_in_danger" => false, // Would need master reference
        "enemy_near_master" => false, // Would need master reference
        "is_work_time" => true, // Could check time of day
        _ => false,
    }
}

/// Actions that can be performed by behavior nodes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BehaviorAction {
    Idle,
    Wait,
    Wander,
    Flee,
    Attack,
    Pursue,
    Patrol,
    Work,
    Greet,
    Socialize,
    Trade,
    FollowMaster,
    ProtectMaster,
    Stay,
    ManageEnvironment,
    ManageSystem,
    Custom(String),
}

impl BehaviorAction {
    /// Converts to an AgentAction
    pub fn to_agent_action(&self, agent: &Agent, manager: &AgentManager) -> Option<AgentAction> {
        let mut rng = rand::thread_rng();

        match self {
            Self::Idle | Self::Wait => Some(AgentAction::Wait),
            Self::Wander => {
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                Some(AgentAction::Move { dx, dy })
            }
            Self::Flee => {
                // Find nearest threat and flee from it
                let threats: Vec<_> = manager.in_radius(agent.x, agent.y, 10)
                    .into_iter()
                    .filter(|a| a.kind.category() == AgentCategory::Enemy && a.is_alive())
                    .collect();

                if let Some(threat) = threats.first() {
                    Some(AgentAction::Flee { from_x: threat.x, from_y: threat.y })
                } else {
                    Some(AgentAction::Wait)
                }
            }
            Self::Attack => {
                // Find nearest target
                let targets: Vec<_> = manager.in_radius(agent.x, agent.y, 1)
                    .into_iter()
                    .filter(|a| a.id != agent.id && a.is_alive())
                    .collect();

                if let Some(target) = targets.first() {
                    Some(AgentAction::Attack { target_id: target.id })
                } else {
                    // Move toward visible enemies
                    let enemies: Vec<_> = manager.in_radius(agent.x, agent.y, 10)
                        .into_iter()
                        .filter(|a| a.kind.category() != agent.kind.category() && a.is_alive())
                        .collect();

                    if let Some(enemy) = enemies.first() {
                        let dx = (enemy.x as i32 - agent.x as i32).signum();
                        let dy = (enemy.y as i32 - agent.y as i32).signum();
                        Some(AgentAction::Move { dx, dy })
                    } else {
                        Some(AgentAction::Wait)
                    }
                }
            }
            Self::Pursue => {
                if let Some((x, y)) = agent.memory.last_enemy_position {
                    let dx = (x as i32 - agent.x as i32).signum();
                    let dy = (y as i32 - agent.y as i32).signum();
                    Some(AgentAction::Move { dx, dy })
                } else {
                    Some(AgentAction::Wait)
                }
            }
            Self::Patrol => {
                // Random patrol movement
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                Some(AgentAction::Move { dx, dy })
            }
            Self::Work => {
                // NPCs working
                Some(AgentAction::Wait)
            }
            Self::Greet => {
                // Greet nearby agents
                let dialogues = crate::agents::NpcBehaviors::dialogues_for_kind(agent.kind);
                if !dialogues.is_empty() {
                    let msg = dialogues[rng.gen_range(0..dialogues.len())].to_string();
                    Some(AgentAction::Speak { message: msg, target: None })
                } else {
                    Some(AgentAction::Wait)
                }
            }
            Self::Socialize => {
                // Talk to nearby NPCs
                Some(AgentAction::Wait)
            }
            Self::Trade => {
                Some(AgentAction::Wait)
            }
            Self::FollowMaster | Self::ProtectMaster | Self::Stay => {
                Some(AgentAction::Wait)
            }
            Self::ManageEnvironment | Self::ManageSystem => {
                Some(AgentAction::Wait)
            }
            Self::Custom(_name) => {
                Some(AgentAction::Wait)
            }
        }
    }
}

/// Utility AI for more complex decision making
#[derive(Clone, Debug)]
pub struct UtilityAI {
    /// Considerations for each action
    pub considerations: HashMap<String, Vec<Consideration>>,
}

/// A consideration that affects action utility
#[derive(Clone, Debug)]
pub struct Consideration {
    pub name: String,
    pub curve: ResponseCurve,
    pub weight: f32,
}

/// Response curves for utility calculations
#[derive(Clone, Copy, Debug)]
pub enum ResponseCurve {
    Linear,
    Quadratic,
    Logistic,
    InverseLinear,
    Constant(f32),
}

impl ResponseCurve {
    /// Evaluates the curve at a given input (0.0 to 1.0)
    pub fn evaluate(&self, x: f32) -> f32 {
        let x = x.clamp(0.0, 1.0);
        match self {
            Self::Linear => x,
            Self::Quadratic => x * x,
            Self::Logistic => 1.0 / (1.0 + (-12.0 * (x - 0.5)).exp()),
            Self::InverseLinear => 1.0 - x,
            Self::Constant(v) => *v,
        }
    }
}

impl UtilityAI {
    /// Creates a new utility AI
    pub fn new() -> Self {
        Self {
            considerations: HashMap::new(),
        }
    }

    /// Adds a consideration for an action
    pub fn add_consideration(&mut self, action: &str, consideration: Consideration) {
        self.considerations
            .entry(action.to_string())
            .or_insert_with(Vec::new)
            .push(consideration);
    }

    /// Calculates utility for an action
    pub fn calculate_utility(&self, action: &str, inputs: &HashMap<String, f32>) -> f32 {
        let Some(considerations) = self.considerations.get(action) else {
            return 0.0;
        };

        let mut utility = 1.0;
        for consideration in considerations {
            let input = inputs.get(&consideration.name).copied().unwrap_or(0.5);
            let value = consideration.curve.evaluate(input);
            utility *= value * consideration.weight;
        }

        utility
    }

    /// Chooses the best action
    pub fn choose_action(&self, inputs: &HashMap<String, f32>) -> Option<String> {
        self.considerations
            .keys()
            .max_by(|a, b| {
                let ua = self.calculate_utility(a, inputs);
                let ub = self.calculate_utility(b, inputs);
                ua.partial_cmp(&ub).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }
}

impl Default for UtilityAI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_creation() {
        let goal = Goal::new(GoalKind::AttackEnemy, GoalPriority::High);
        assert_eq!(goal.kind, GoalKind::AttackEnemy);
        assert_eq!(goal.priority, GoalPriority::High);
        assert!(!goal.completed);
    }

    #[test]
    fn test_response_curves() {
        assert_eq!(ResponseCurve::Linear.evaluate(0.5), 0.5);
        assert_eq!(ResponseCurve::Quadratic.evaluate(0.5), 0.25);
        assert_eq!(ResponseCurve::InverseLinear.evaluate(0.5), 0.5);
    }

    #[test]
    fn test_behavior_tree() {
        let tree = BehaviorTree::default_for_kind(AgentKind::Guard);
        assert!(!matches!(tree.root, BehaviorNode::Action(BehaviorAction::Idle)));
    }
}

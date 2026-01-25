//! System agent implementations
//!
//! These agents manage game systems: quests, narration, tutorials,
//! achievements, and difficulty balancing.

use super::{Agent, AgentId, AgentKind, AgentState};
use crate::behavior::{Goal, GoalKind, GoalPriority};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// System agent behaviors
pub struct SystemBehaviors;

impl SystemBehaviors {
    /// Creates default goals for system agents
    pub fn default_goals(kind: AgentKind) -> Vec<Goal> {
        match kind {
            AgentKind::QuestGiver => vec![
                Goal::new(GoalKind::ManageQuests, GoalPriority::High),
                Goal::new(GoalKind::TrackProgress, GoalPriority::Medium),
            ],
            AgentKind::Narrator => vec![
                Goal::new(GoalKind::TellStory, GoalPriority::High),
                Goal::new(GoalKind::CreateAtmosphere, GoalPriority::Medium),
            ],
            AgentKind::TutorialGuide => vec![
                Goal::new(GoalKind::Teach, GoalPriority::High),
                Goal::new(GoalKind::Guide, GoalPriority::High),
            ],
            AgentKind::AchievementTracker => vec![
                Goal::new(GoalKind::TrackProgress, GoalPriority::High),
                Goal::new(GoalKind::RewardPlayer, GoalPriority::Medium),
            ],
            AgentKind::DifficultyAdjuster => vec![
                Goal::new(GoalKind::BalanceGame, GoalPriority::High),
                Goal::new(GoalKind::AdjustDifficulty, GoalPriority::Medium),
            ],
            _ => vec![],
        }
    }
}

/// Quest system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub objectives: Vec<QuestObjective>,
    pub rewards: Vec<QuestReward>,
    pub state: QuestState,
    pub giver: Option<AgentId>,
    pub level_requirement: u32,
    pub time_limit: Option<u32>,
    pub chain: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestObjective {
    pub description: String,
    pub kind: ObjectiveKind,
    pub current: u32,
    pub target: u32,
    pub completed: bool,
    pub optional: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectiveKind {
    Kill { enemy_kind: String },
    Collect { item_kind: String },
    Deliver { item: String, to: String },
    Explore { location: String },
    Escort { npc: String, to: String },
    Talk { npc: String },
    Craft { item: String },
    Survive { turns: u32 },
    Reach { x: usize, y: usize },
    Protect { npc: String, duration: u32 },
    Solve { puzzle: String },
    Custom { event: String },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestState {
    Available,
    Active,
    Completed,
    Failed,
    TurnedIn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QuestReward {
    Gold(u32),
    Experience(u32),
    Item(String),
    Reputation { faction: String, amount: i32 },
    Skill(String),
    Unlock(String),
    Title(String),
}

impl Quest {
    pub fn new(id: u32, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            objectives: Vec::new(),
            rewards: Vec::new(),
            state: QuestState::Available,
            giver: None,
            level_requirement: 1,
            time_limit: None,
            chain: None,
        }
    }

    pub fn add_objective(&mut self, objective: QuestObjective) {
        self.objectives.push(objective);
    }

    pub fn add_reward(&mut self, reward: QuestReward) {
        self.rewards.push(reward);
    }

    pub fn is_complete(&self) -> bool {
        self.objectives.iter()
            .filter(|o| !o.optional)
            .all(|o| o.completed)
    }

    pub fn update_objective(&mut self, kind: &ObjectiveKind, amount: u32) {
        for obj in &mut self.objectives {
            if std::mem::discriminant(&obj.kind) == std::mem::discriminant(kind) {
                obj.current = (obj.current + amount).min(obj.target);
                if obj.current >= obj.target {
                    obj.completed = true;
                }
            }
        }

        if self.is_complete() {
            self.state = QuestState::Completed;
        }
    }
}

/// Quest manager
#[derive(Clone, Debug, Default)]
pub struct QuestManager {
    pub quests: HashMap<u32, Quest>,
    pub active_quests: Vec<u32>,
    pub completed_quests: Vec<u32>,
    pub next_id: u32,
}

impl QuestManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_quest(&mut self, mut quest: Quest) -> u32 {
        let id = self.next_id;
        quest.id = id;
        self.quests.insert(id, quest);
        self.next_id += 1;
        id
    }

    pub fn accept_quest(&mut self, id: u32) -> bool {
        if let Some(quest) = self.quests.get_mut(&id) {
            if quest.state == QuestState::Available {
                quest.state = QuestState::Active;
                self.active_quests.push(id);
                return true;
            }
        }
        false
    }

    pub fn complete_quest(&mut self, id: u32) -> Option<Vec<QuestReward>> {
        if let Some(quest) = self.quests.get_mut(&id) {
            if quest.state == QuestState::Completed {
                quest.state = QuestState::TurnedIn;
                self.active_quests.retain(|&q| q != id);
                self.completed_quests.push(id);
                return Some(quest.rewards.clone());
            }
        }
        None
    }

    /// Creates sample quests
    pub fn create_sample_quests(&mut self) {
        let mut quest = Quest::new(0, "First Blood", "Defeat your first enemy in the dungeon.");
        quest.add_objective(QuestObjective {
            description: "Kill any enemy".to_string(),
            kind: ObjectiveKind::Kill { enemy_kind: "any".to_string() },
            current: 0,
            target: 1,
            completed: false,
            optional: false,
        });
        quest.add_reward(QuestReward::Gold(50));
        quest.add_reward(QuestReward::Experience(100));
        self.add_quest(quest);

        let mut quest = Quest::new(0, "Goblin Slayer", "Clear the dungeon of goblin scouts.");
        quest.add_objective(QuestObjective {
            description: "Kill 10 goblin scouts".to_string(),
            kind: ObjectiveKind::Kill { enemy_kind: "goblin_scout".to_string() },
            current: 0,
            target: 10,
            completed: false,
            optional: false,
        });
        quest.add_reward(QuestReward::Gold(200));
        quest.add_reward(QuestReward::Experience(300));
        quest.add_reward(QuestReward::Item("Steel Sword".to_string()));
        quest.level_requirement = 2;
        self.add_quest(quest);

        let mut quest = Quest::new(0, "Descent", "Reach dungeon level 5.");
        quest.add_objective(QuestObjective {
            description: "Descend to level 5".to_string(),
            kind: ObjectiveKind::Reach { x: 0, y: 0 },
            current: 0,
            target: 5,
            completed: false,
            optional: false,
        });
        quest.add_reward(QuestReward::Gold(500));
        quest.add_reward(QuestReward::Experience(500));
        self.add_quest(quest);
    }
}

/// Narrator messages and story events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NarratorMessage {
    pub id: u32,
    pub text: String,
    pub trigger: NarratorTrigger,
    pub priority: u32,
    pub shown: bool,
    pub repeatable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NarratorTrigger {
    GameStart,
    LevelEntry(u32),
    BossEncounter(String),
    FirstKill,
    LowHealth,
    Discovery(String),
    Achievement(String),
    Death,
    Victory,
    TimeOfDay(String),
    Custom(String),
}

/// Narrator system
#[derive(Clone, Debug, Default)]
pub struct Narrator {
    pub messages: Vec<NarratorMessage>,
    pub message_queue: Vec<String>,
    pub current_message: Option<String>,
}

impl Narrator {
    pub fn new() -> Self {
        let mut narrator = Self::default();
        narrator.load_default_messages();
        narrator
    }

    fn load_default_messages(&mut self) {
        self.messages.push(NarratorMessage {
            id: 1,
            text: "You stand at the entrance of ShadowCrypt. The darkness beckons...".to_string(),
            trigger: NarratorTrigger::GameStart,
            priority: 100,
            shown: false,
            repeatable: false,
        });

        self.messages.push(NarratorMessage {
            id: 2,
            text: "The air grows colder as you descend deeper into the crypt...".to_string(),
            trigger: NarratorTrigger::LevelEntry(3),
            priority: 50,
            shown: false,
            repeatable: false,
        });

        self.messages.push(NarratorMessage {
            id: 3,
            text: "Your first blood. Many more will fall before you.".to_string(),
            trigger: NarratorTrigger::FirstKill,
            priority: 80,
            shown: false,
            repeatable: false,
        });

        self.messages.push(NarratorMessage {
            id: 4,
            text: "Your life force wanes. Tread carefully, adventurer.".to_string(),
            trigger: NarratorTrigger::LowHealth,
            priority: 90,
            shown: false,
            repeatable: true,
        });

        self.messages.push(NarratorMessage {
            id: 5,
            text: "And so, another soul joins the crypt's collection...".to_string(),
            trigger: NarratorTrigger::Death,
            priority: 100,
            shown: false,
            repeatable: true,
        });

        self.messages.push(NarratorMessage {
            id: 6,
            text: "Against all odds, you have conquered the darkness. You are the champion of ShadowCrypt!".to_string(),
            trigger: NarratorTrigger::Victory,
            priority: 100,
            shown: false,
            repeatable: false,
        });
    }

    pub fn check_triggers(&mut self, trigger: &NarratorTrigger) -> Option<String> {
        for msg in &mut self.messages {
            if !msg.shown || msg.repeatable {
                if std::mem::discriminant(&msg.trigger) == std::mem::discriminant(trigger) {
                    msg.shown = true;
                    return Some(msg.text.clone());
                }
            }
        }
        None
    }

    pub fn queue_message(&mut self, text: String) {
        self.message_queue.push(text);
    }

    pub fn next_message(&mut self) -> Option<String> {
        if self.current_message.is_some() {
            return self.current_message.take();
        }
        if !self.message_queue.is_empty() {
            return Some(self.message_queue.remove(0));
        }
        None
    }
}

/// Tutorial system
#[derive(Clone, Debug)]
pub struct TutorialSystem {
    pub tutorials: Vec<Tutorial>,
    pub completed: Vec<u32>,
    pub current: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Tutorial {
    pub id: u32,
    pub name: String,
    pub steps: Vec<TutorialStep>,
    pub current_step: usize,
    pub completed: bool,
}

#[derive(Clone, Debug)]
pub struct TutorialStep {
    pub text: String,
    pub highlight: Option<(usize, usize)>,
    pub required_action: Option<String>,
    pub completed: bool,
}

impl TutorialSystem {
    pub fn new() -> Self {
        let mut system = Self {
            tutorials: Vec::new(),
            completed: Vec::new(),
            current: None,
        };
        system.load_tutorials();
        system
    }

    fn load_tutorials(&mut self) {
        self.tutorials.push(Tutorial {
            id: 1,
            name: "Movement".to_string(),
            steps: vec![
                TutorialStep {
                    text: "Use WASD or arrow keys to move.".to_string(),
                    highlight: None,
                    required_action: Some("move".to_string()),
                    completed: false,
                },
                TutorialStep {
                    text: "Use Q, E, Z, C to move diagonally.".to_string(),
                    highlight: None,
                    required_action: Some("move_diagonal".to_string()),
                    completed: false,
                },
            ],
            current_step: 0,
            completed: false,
        });

        self.tutorials.push(Tutorial {
            id: 2,
            name: "Combat".to_string(),
            steps: vec![
                TutorialStep {
                    text: "Walk into an enemy to attack.".to_string(),
                    highlight: None,
                    required_action: Some("attack".to_string()),
                    completed: false,
                },
                TutorialStep {
                    text: "Press 1-4 to select skills, then use them.".to_string(),
                    highlight: None,
                    required_action: Some("use_skill".to_string()),
                    completed: false,
                },
            ],
            current_step: 0,
            completed: false,
        });

        self.tutorials.push(Tutorial {
            id: 3,
            name: "Items".to_string(),
            steps: vec![
                TutorialStep {
                    text: "Walk over items to pick them up.".to_string(),
                    highlight: None,
                    required_action: Some("pickup".to_string()),
                    completed: false,
                },
                TutorialStep {
                    text: "Press I to open inventory, then U to use items.".to_string(),
                    highlight: None,
                    required_action: Some("use_item".to_string()),
                    completed: false,
                },
            ],
            current_step: 0,
            completed: false,
        });
    }

    pub fn start_tutorial(&mut self, id: u32) -> Option<&str> {
        if let Some(tutorial) = self.tutorials.iter().find(|t| t.id == id) {
            self.current = Some(id);
            if let Some(step) = tutorial.steps.first() {
                return Some(&step.text);
            }
        }
        None
    }

    pub fn advance(&mut self, action: &str) -> Option<&str> {
        if let Some(current_id) = self.current {
            if let Some(tutorial) = self.tutorials.iter_mut().find(|t| t.id == current_id) {
                if let Some(step) = tutorial.steps.get_mut(tutorial.current_step) {
                    if let Some(ref required) = step.required_action {
                        if required == action {
                            step.completed = true;
                            tutorial.current_step += 1;

                            if tutorial.current_step >= tutorial.steps.len() {
                                tutorial.completed = true;
                                self.completed.push(current_id);
                                self.current = None;
                                return None;
                            }

                            return Some(&tutorial.steps[tutorial.current_step].text);
                        }
                    }
                }
            }
        }
        None
    }
}

/// Achievement system
#[derive(Clone, Debug)]
pub struct AchievementSystem {
    pub achievements: Vec<Achievement>,
    pub unlocked: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Achievement {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub condition: AchievementCondition,
    pub reward: Option<QuestReward>,
    pub hidden: bool,
    pub unlocked: bool,
}

#[derive(Clone, Debug)]
pub enum AchievementCondition {
    KillCount(u32),
    ReachLevel(u32),
    ReachDungeonLevel(u32),
    CollectGold(u32),
    FindItem(String),
    DefeatBoss(String),
    SurviveTurns(u32),
    DieCount(u32),
    CompleteQuests(u32),
    Custom(String),
}

impl AchievementSystem {
    pub fn new() -> Self {
        let mut system = Self {
            achievements: Vec::new(),
            unlocked: Vec::new(),
        };
        system.load_achievements();
        system
    }

    fn load_achievements(&mut self) {
        self.achievements.push(Achievement {
            id: 1,
            name: "First Steps".to_string(),
            description: "Enter the dungeon for the first time.".to_string(),
            condition: AchievementCondition::Custom("game_start".to_string()),
            reward: None,
            hidden: false,
            unlocked: false,
        });

        self.achievements.push(Achievement {
            id: 2,
            name: "Warrior".to_string(),
            description: "Kill 100 enemies.".to_string(),
            condition: AchievementCondition::KillCount(100),
            reward: Some(QuestReward::Title("Warrior".to_string())),
            hidden: false,
            unlocked: false,
        });

        self.achievements.push(Achievement {
            id: 3,
            name: "Deep Diver".to_string(),
            description: "Reach dungeon level 15.".to_string(),
            condition: AchievementCondition::ReachDungeonLevel(15),
            reward: Some(QuestReward::Gold(1000)),
            hidden: false,
            unlocked: false,
        });

        self.achievements.push(Achievement {
            id: 4,
            name: "Champion".to_string(),
            description: "Defeat the Demon King.".to_string(),
            condition: AchievementCondition::DefeatBoss("Demon King".to_string()),
            reward: Some(QuestReward::Title("Champion of ShadowCrypt".to_string())),
            hidden: false,
            unlocked: false,
        });

        self.achievements.push(Achievement {
            id: 5,
            name: "Persistent".to_string(),
            description: "Die 10 times.".to_string(),
            condition: AchievementCondition::DieCount(10),
            reward: None,
            hidden: true,
            unlocked: false,
        });

        self.achievements.push(Achievement {
            id: 6,
            name: "Rich".to_string(),
            description: "Collect 10000 gold.".to_string(),
            condition: AchievementCondition::CollectGold(10000),
            reward: Some(QuestReward::Item("Golden Crown".to_string())),
            hidden: false,
            unlocked: false,
        });
    }

    pub fn check_unlock(&mut self, condition: &AchievementCondition) -> Vec<&Achievement> {
        let mut unlocked = Vec::new();
        for achievement in &mut self.achievements {
            if !achievement.unlocked {
                if std::mem::discriminant(&achievement.condition) == std::mem::discriminant(condition) {
                    achievement.unlocked = true;
                    self.unlocked.push(achievement.id);
                    unlocked.push(&*achievement);
                }
            }
        }
        unlocked
    }
}

/// Dynamic difficulty adjustment
#[derive(Clone, Debug)]
pub struct DifficultyAdjuster {
    /// Current difficulty multiplier (1.0 = normal)
    pub multiplier: f32,
    /// Player performance metrics
    pub metrics: PerformanceMetrics,
    /// Target difficulty (how hard the game should feel)
    pub target_difficulty: f32,
    /// How quickly difficulty adjusts
    pub adjustment_rate: f32,
}

#[derive(Clone, Debug, Default)]
pub struct PerformanceMetrics {
    pub deaths: u32,
    pub kills: u32,
    pub damage_taken: i32,
    pub damage_dealt: i32,
    pub potions_used: u32,
    pub turns_survived: u32,
    pub gold_collected: u32,
    pub levels_completed: u32,
}

impl DifficultyAdjuster {
    pub fn new(target: f32) -> Self {
        Self {
            multiplier: 1.0,
            metrics: PerformanceMetrics::default(),
            target_difficulty: target,
            adjustment_rate: 0.05,
        }
    }

    /// Calculates player performance score (0.0 = struggling, 1.0 = dominating)
    pub fn performance_score(&self) -> f32 {
        let m = &self.metrics;
        let kill_ratio = if m.deaths > 0 { m.kills as f32 / m.deaths as f32 } else { m.kills as f32 };
        let damage_ratio = if m.damage_taken > 0 { m.damage_dealt as f32 / m.damage_taken as f32 } else { 10.0 };
        let survival = (m.turns_survived as f32 / 100.0).min(10.0);

        ((kill_ratio * 0.3) + (damage_ratio * 0.3) + (survival * 0.4)).min(10.0) / 10.0
    }

    /// Updates difficulty based on performance
    pub fn update(&mut self) {
        let performance = self.performance_score();
        let diff = performance - self.target_difficulty;

        if diff > 0.1 {
            // Player doing well, increase difficulty
            self.multiplier += self.adjustment_rate;
        } else if diff < -0.1 {
            // Player struggling, decrease difficulty
            self.multiplier -= self.adjustment_rate;
        }

        // Clamp multiplier
        self.multiplier = self.multiplier.clamp(0.5, 2.0);
    }

    /// Gets enemy stat multiplier
    pub fn enemy_multiplier(&self) -> f32 {
        self.multiplier
    }

    /// Gets loot multiplier (inverse of difficulty)
    pub fn loot_multiplier(&self) -> f32 {
        2.0 - self.multiplier
    }
}

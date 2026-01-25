//! Agent memory and learning systems
//!
//! Agents can remember events, learn from experience, and share knowledge.

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};

use crate::agents::AgentId;

/// Memory system for agents
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Memory {
    /// Short-term memories (recent events)
    pub short_term: VecDeque<MemoryEntry>,
    /// Long-term memories (important events)
    pub long_term: Vec<MemoryEntry>,
    /// Knowledge base
    pub knowledge: HashMap<String, Knowledge>,
    /// Known agents
    pub known_agents: HashMap<AgentId, AgentMemory>,
    /// Known locations
    pub known_locations: HashMap<String, LocationMemory>,
    /// Last known position of enemy
    pub last_enemy_position: Option<(usize, usize)>,
    /// Path memory for navigation
    pub visited_positions: VecDeque<(usize, usize)>,
    /// Maximum short-term memory size
    pub max_short_term: usize,
    /// Maximum visited positions to remember
    pub max_visited: usize,
}

impl Memory {
    /// Creates a new memory system
    pub fn new() -> Self {
        Self {
            short_term: VecDeque::new(),
            long_term: Vec::new(),
            knowledge: HashMap::new(),
            known_agents: HashMap::new(),
            known_locations: HashMap::new(),
            last_enemy_position: None,
            visited_positions: VecDeque::new(),
            max_short_term: 50,
            max_visited: 100,
        }
    }

    /// Adds a memory
    pub fn remember(&mut self, memory: MemoryEntry) {
        // Important memories go to long-term
        if memory.importance >= MemoryImportance::Important {
            self.long_term.push(memory.clone());
        }

        // All memories go to short-term first
        self.short_term.push_back(memory);
        while self.short_term.len() > self.max_short_term {
            self.short_term.pop_front();
        }
    }

    /// Remembers a position
    pub fn remember_position(&mut self, x: usize, y: usize) {
        self.visited_positions.push_back((x, y));
        while self.visited_positions.len() > self.max_visited {
            self.visited_positions.pop_front();
        }
    }

    /// Remembers seeing an enemy
    pub fn remember_enemy(&mut self, x: usize, y: usize) {
        self.last_enemy_position = Some((x, y));
    }

    /// Forgets the last enemy position
    pub fn forget_enemy(&mut self) {
        self.last_enemy_position = None;
    }

    /// Adds knowledge
    pub fn learn(&mut self, key: &str, knowledge: Knowledge) {
        self.knowledge.insert(key.to_string(), knowledge);
    }

    /// Gets knowledge
    pub fn recall(&self, key: &str) -> Option<&Knowledge> {
        self.knowledge.get(key)
    }

    /// Remembers an agent
    pub fn remember_agent(&mut self, id: AgentId, memory: AgentMemory) {
        self.known_agents.insert(id, memory);
    }

    /// Gets memory of an agent
    pub fn recall_agent(&self, id: AgentId) -> Option<&AgentMemory> {
        self.known_agents.get(&id)
    }

    /// Remembers a location
    pub fn remember_location(&mut self, name: &str, memory: LocationMemory) {
        self.known_locations.insert(name.to_string(), memory);
    }

    /// Gets memory of a location
    pub fn recall_location(&self, name: &str) -> Option<&LocationMemory> {
        self.known_locations.get(name)
    }

    /// Searches memories for a topic
    pub fn search(&self, topic: &str) -> Vec<&MemoryEntry> {
        let topic_lower = topic.to_lowercase();
        self.short_term
            .iter()
            .chain(self.long_term.iter())
            .filter(|m| m.description.to_lowercase().contains(&topic_lower))
            .collect()
    }

    /// Gets the most recent memory
    pub fn most_recent(&self) -> Option<&MemoryEntry> {
        self.short_term.back()
    }

    /// Gets important memories
    pub fn important_memories(&self) -> Vec<&MemoryEntry> {
        self.long_term
            .iter()
            .filter(|m| m.importance >= MemoryImportance::Important)
            .collect()
    }

    /// Consolidates short-term into long-term (sleep/rest)
    pub fn consolidate(&mut self) {
        // Move moderately important memories to long-term
        let to_transfer: Vec<_> = self.short_term
            .iter()
            .filter(|m| m.importance >= MemoryImportance::Moderate)
            .cloned()
            .collect();

        for memory in to_transfer {
            if !self.long_term.iter().any(|m| m.turn == memory.turn && m.kind == memory.kind) {
                self.long_term.push(memory);
            }
        }
    }

    /// Fades old memories (called periodically)
    pub fn fade(&mut self, current_turn: u32) {
        // Remove very old short-term memories
        let cutoff = current_turn.saturating_sub(100);
        self.short_term.retain(|m| m.turn >= cutoff);

        // Fade importance of long-term memories over time
        for memory in &mut self.long_term {
            let age = current_turn.saturating_sub(memory.turn);
            if age > 1000 && memory.importance == MemoryImportance::Moderate {
                memory.importance = MemoryImportance::Trivial;
            }
        }

        // Remove trivial long-term memories
        self.long_term.retain(|m| m.importance >= MemoryImportance::Moderate);
    }
}

/// A single memory entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Type of memory
    pub kind: MemoryKind,
    /// Description of what happened
    pub description: String,
    /// When it happened
    pub turn: u32,
    /// Where it happened
    pub location: Option<(usize, usize)>,
    /// Involved agents
    pub agents: Vec<AgentId>,
    /// Importance level
    pub importance: MemoryImportance,
    /// Emotional context
    pub emotion: MemoryEmotion,
}

impl MemoryEntry {
    /// Creates a new memory
    pub fn new(kind: MemoryKind, description: &str, turn: u32) -> Self {
        Self {
            kind,
            description: description.to_string(),
            turn,
            location: None,
            agents: Vec::new(),
            importance: MemoryImportance::Trivial,
            emotion: MemoryEmotion::Neutral,
        }
    }

    /// Sets the location
    pub fn at(mut self, x: usize, y: usize) -> Self {
        self.location = Some((x, y));
        self
    }

    /// Adds involved agents
    pub fn involving(mut self, agents: Vec<AgentId>) -> Self {
        self.agents = agents;
        self
    }

    /// Sets importance
    pub fn with_importance(mut self, importance: MemoryImportance) -> Self {
        self.importance = importance;
        self
    }

    /// Sets emotion
    pub fn with_emotion(mut self, emotion: MemoryEmotion) -> Self {
        self.emotion = emotion;
        self
    }
}

/// Types of memories
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryKind {
    /// Saw something
    Observation,
    /// Heard something
    Auditory,
    /// Participated in combat
    Combat,
    /// Had a conversation
    Conversation,
    /// Traded with someone
    Trade,
    /// Discovered a location
    Discovery,
    /// Learned information
    Learning,
    /// Experienced danger
    Danger,
    /// Received help
    Help,
    /// Was betrayed
    Betrayal,
    /// Achieved a goal
    Achievement,
    /// Failed at something
    Failure,
    /// Custom memory
    Custom,
}

/// Importance levels for memories
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MemoryImportance {
    /// Easily forgotten
    Trivial = 0,
    /// Might remember
    Minor = 1,
    /// Will remember
    Moderate = 2,
    /// Strongly remembered
    Important = 3,
    /// Never forgotten
    Critical = 4,
}

/// Emotional context of memories
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryEmotion {
    Neutral,
    Happy,
    Sad,
    Angry,
    Fearful,
    Surprised,
    Disgusted,
    Curious,
    Grateful,
    Resentful,
}

/// Knowledge that agents can have
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Knowledge {
    /// Topic of knowledge
    pub topic: String,
    /// Content/details
    pub content: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,
    /// Source of knowledge
    pub source: KnowledgeSource,
    /// When learned
    pub learned_turn: u32,
    /// Times this knowledge was used
    pub use_count: u32,
}

impl Knowledge {
    /// Creates new knowledge
    pub fn new(topic: &str, content: &str, source: KnowledgeSource) -> Self {
        Self {
            topic: topic.to_string(),
            content: content.to_string(),
            confidence: 1.0,
            source,
            learned_turn: 0,
            use_count: 0,
        }
    }

    /// Increases use count
    pub fn use_knowledge(&mut self) {
        self.use_count += 1;
    }

    /// Decays confidence over time
    pub fn decay(&mut self, amount: f32) {
        self.confidence = (self.confidence - amount).max(0.0);
    }
}

/// Source of knowledge
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeSource {
    /// Learned by observing
    Observation,
    /// Told by another agent
    Hearsay,
    /// Read from a book/scroll
    Written,
    /// Figured out
    Deduction,
    /// Innate knowledge
    Innate,
    /// Player told them
    PlayerTold,
}

/// Memory of an agent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentMemory {
    /// Agent's kind (if known)
    pub kind: Option<String>,
    /// Agent's name (if known)
    pub name: Option<String>,
    /// Last known position
    pub last_position: Option<(usize, usize)>,
    /// Last time seen
    pub last_seen: u32,
    /// Relationship sentiment (-100 to 100)
    pub sentiment: i32,
    /// Number of interactions
    pub interaction_count: u32,
    /// Notable events with this agent
    pub events: Vec<String>,
    /// Trust level (0.0 to 1.0)
    pub trust: f32,
}

impl AgentMemory {
    /// Creates a new agent memory
    pub fn new() -> Self {
        Self {
            kind: None,
            name: None,
            last_position: None,
            last_seen: 0,
            sentiment: 0,
            interaction_count: 0,
            events: Vec::new(),
            trust: 0.5,
        }
    }

    /// Updates from seeing the agent
    pub fn saw_at(&mut self, x: usize, y: usize, turn: u32) {
        self.last_position = Some((x, y));
        self.last_seen = turn;
    }

    /// Records an interaction
    pub fn interacted(&mut self, event: &str, sentiment_change: i32) {
        self.interaction_count += 1;
        self.sentiment = (self.sentiment + sentiment_change).clamp(-100, 100);
        self.events.push(event.to_string());
        if self.events.len() > 10 {
            self.events.remove(0);
        }
    }
}

impl Default for AgentMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory of a location
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationMemory {
    /// Position
    pub position: (usize, usize),
    /// Description
    pub description: String,
    /// Times visited
    pub visit_count: u32,
    /// Last visited turn
    pub last_visited: u32,
    /// Is it dangerous?
    pub dangerous: bool,
    /// Items/resources found here
    pub resources: Vec<String>,
    /// Agents seen here
    pub seen_agents: Vec<AgentId>,
}

impl LocationMemory {
    /// Creates a new location memory
    pub fn new(x: usize, y: usize, description: &str) -> Self {
        Self {
            position: (x, y),
            description: description.to_string(),
            visit_count: 1,
            last_visited: 0,
            dangerous: false,
            resources: Vec::new(),
            seen_agents: Vec::new(),
        }
    }

    /// Records a visit
    pub fn visited(&mut self, turn: u32) {
        self.visit_count += 1;
        self.last_visited = turn;
    }

    /// Marks as dangerous
    pub fn mark_dangerous(&mut self) {
        self.dangerous = true;
    }
}

/// Learning system for agents
#[derive(Clone, Debug, Default)]
pub struct LearningSystem {
    /// Experience points in different skills
    pub skills: HashMap<String, f32>,
    /// Learned behaviors
    pub behaviors: HashMap<String, f32>,
    /// Learning rate
    pub learning_rate: f32,
}

impl LearningSystem {
    /// Creates a new learning system
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            behaviors: HashMap::new(),
            learning_rate: 0.1,
        }
    }

    /// Learns from experience
    pub fn learn_skill(&mut self, skill: &str, amount: f32) {
        let current = self.skills.entry(skill.to_string()).or_insert(0.0);
        *current = (*current + amount * self.learning_rate).min(100.0);
    }

    /// Learns a behavior pattern
    pub fn learn_behavior(&mut self, behavior: &str, success: bool) {
        let current = self.behaviors.entry(behavior.to_string()).or_insert(0.5);
        if success {
            *current = (*current + self.learning_rate).min(1.0);
        } else {
            *current = (*current - self.learning_rate).max(0.0);
        }
    }

    /// Gets skill level
    pub fn skill_level(&self, skill: &str) -> f32 {
        self.skills.get(skill).copied().unwrap_or(0.0)
    }

    /// Gets behavior preference
    pub fn behavior_preference(&self, behavior: &str) -> f32 {
        self.behaviors.get(behavior).copied().unwrap_or(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let mut memory = Memory::new();
        memory.remember(MemoryEntry::new(
            MemoryKind::Observation,
            "Saw a goblin",
            1,
        ));
        assert_eq!(memory.short_term.len(), 1);
    }

    #[test]
    fn test_knowledge() {
        let mut memory = Memory::new();
        memory.learn("secret_door", Knowledge::new(
            "secret_door",
            "There's a secret door in the library",
            KnowledgeSource::Observation,
        ));
        assert!(memory.recall("secret_door").is_some());
    }

    #[test]
    fn test_learning() {
        let mut learning = LearningSystem::new();
        learning.learn_skill("combat", 10.0);
        assert!(learning.skill_level("combat") > 0.0);
    }
}

//! ShadowCrypt Multi-Agent System
//!
//! A comprehensive multi-agent framework featuring 75+ autonomous agents
//! that run in parallel and communicate with each other.
//!
//! # Architecture
//!
//! The system is organized into several key modules:
//!
//! - `agents`: Core agent definitions and 75+ agent types
//! - `communication`: Inter-agent messaging and event systems
//! - `behavior`: Behavior trees, goals, and decision making
//! - `memory`: Agent memory, learning, and knowledge systems
//! - `factions`: Faction systems, alliances, and politics
//! - `personality`: Personality traits, emotions, and moods
//! - `relationships`: Social networks and relationship tracking
//! - `parallel`: Parallel execution and synchronization

pub mod agents;
pub mod communication;
pub mod behavior;
pub mod memory;
pub mod factions;
pub mod personality;
pub mod relationships;
pub mod parallel;

// Re-export commonly used types
pub use agents::{Agent, AgentId, AgentKind, AgentState, AgentManager};
pub use communication::{Message, MessageBus, Event, EventKind};
pub use behavior::{Behavior, Goal, GoalPriority, BehaviorTree};
pub use memory::{Memory, MemoryKind, Knowledge, LearningSystem};
pub use factions::{Faction, FactionId, FactionRelation, PoliticalSystem};
pub use personality::{Personality, Trait, Emotion, Mood};
pub use relationships::{Relationship, RelationshipKind, SocialNetwork};
pub use parallel::{AgentScheduler, ParallelExecutor, SyncPrimitive};

/// Version of the multi-agent system
pub const VERSION: &str = "1.0.0";

/// Maximum number of agents supported
pub const MAX_AGENTS: usize = 500;

/// Default number of agents to spawn
pub const DEFAULT_AGENT_COUNT: usize = 75;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::agents::*;
    pub use crate::communication::*;
    pub use crate::behavior::*;
    pub use crate::memory::*;
    pub use crate::factions::*;
    pub use crate::personality::*;
    pub use crate::relationships::*;
    pub use crate::parallel::*;
}

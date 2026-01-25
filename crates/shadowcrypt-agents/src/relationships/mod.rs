//! Relationship system for agents
//!
//! Tracks relationships, social networks, and interactions between agents.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::agents::AgentId;

/// A relationship between two agents
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Relationship {
    /// Source agent
    pub from: AgentId,
    /// Target agent
    pub to: AgentId,
    /// Type of relationship
    pub kind: RelationshipKind,
    /// Strength of relationship (-100 to 100)
    pub strength: i32,
    /// Trust level (0.0 to 1.0)
    pub trust: f32,
    /// Familiarity level (0.0 to 1.0)
    pub familiarity: f32,
    /// Number of interactions
    pub interactions: u32,
    /// Last interaction turn
    pub last_interaction: u32,
    /// Relationship history
    pub history: Vec<RelationshipEvent>,
}

impl Relationship {
    /// Creates a new relationship
    pub fn new(from: AgentId, to: AgentId, kind: RelationshipKind) -> Self {
        Self {
            from,
            to,
            kind,
            strength: 0,
            trust: 0.5,
            familiarity: 0.0,
            interactions: 0,
            last_interaction: 0,
            history: Vec::new(),
        }
    }

    /// Modifies the relationship strength
    pub fn modify(&mut self, amount: i32, turn: u32) {
        self.strength = (self.strength + amount).clamp(-100, 100);
        self.interactions += 1;
        self.last_interaction = turn;
        self.familiarity = (self.familiarity + 0.05).min(1.0);
    }

    /// Records an event in the relationship history
    pub fn record_event(&mut self, event: RelationshipEvent) {
        self.history.push(event);
        if self.history.len() > 20 {
            self.history.remove(0);
        }
    }

    /// Updates trust based on an action
    pub fn update_trust(&mut self, positive: bool) {
        if positive {
            self.trust = (self.trust + 0.1).min(1.0);
        } else {
            self.trust = (self.trust - 0.15).max(0.0);
        }
    }

    /// Gets the relationship level
    pub fn level(&self) -> RelationshipLevel {
        match self.strength {
            i32::MIN..=-75 => RelationshipLevel::Nemesis,
            -74..=-50 => RelationshipLevel::Enemy,
            -49..=-25 => RelationshipLevel::Rival,
            -24..=-1 => RelationshipLevel::Unfriendly,
            0..=24 => RelationshipLevel::Acquaintance,
            25..=49 => RelationshipLevel::Friendly,
            50..=74 => RelationshipLevel::Friend,
            75..=89 => RelationshipLevel::CloseFriend,
            90..=i32::MAX => RelationshipLevel::BestFriend,
        }
    }

    /// Returns if this is a hostile relationship
    pub fn is_hostile(&self) -> bool {
        self.strength < -25
    }

    /// Returns if this is a friendly relationship
    pub fn is_friendly(&self) -> bool {
        self.strength > 25
    }
}

/// Types of relationships
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipKind {
    // Family relationships
    Parent,
    Child,
    Sibling,
    Spouse,
    Relative,

    // Social relationships
    Friend,
    Acquaintance,
    Rival,
    Enemy,
    Nemesis,

    // Professional relationships
    Employer,
    Employee,
    Colleague,
    Mentor,
    Student,
    Customer,
    Supplier,

    // Romantic relationships
    Crush,
    Partner,
    ExPartner,

    // Special relationships
    Master,
    Servant,
    Companion,
    Ally,
    Follower,
}

/// Relationship levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationshipLevel {
    Nemesis,
    Enemy,
    Rival,
    Unfriendly,
    Acquaintance,
    Friendly,
    Friend,
    CloseFriend,
    BestFriend,
}

/// Events in a relationship
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipEvent {
    pub turn: u32,
    pub event_type: RelationshipEventType,
    pub description: String,
    pub impact: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipEventType {
    Met,
    Talked,
    Helped,
    Betrayed,
    Fought,
    Traded,
    GaveGift,
    ReceivedGift,
    Insulted,
    Complimented,
    SavedLife,
    TookLife,
    SharesSecret,
    BrokePromise,
    KeptPromise,
}

/// Social network managing all relationships
#[derive(Clone, Debug, Default)]
pub struct SocialNetwork {
    /// All relationships indexed by (from, to)
    relationships: HashMap<(AgentId, AgentId), Relationship>,
    /// Graph for complex queries
    graph: DiGraph<AgentId, RelationshipKind>,
    /// Node indices for agents
    node_indices: HashMap<AgentId, NodeIndex>,
    /// Groups/cliques
    groups: Vec<SocialGroup>,
}

impl SocialNetwork {
    /// Creates a new social network
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets or creates a node index for an agent
    fn get_or_create_node(&mut self, agent: AgentId) -> NodeIndex {
        if let Some(&idx) = self.node_indices.get(&agent) {
            idx
        } else {
            let idx = self.graph.add_node(agent);
            self.node_indices.insert(agent, idx);
            idx
        }
    }

    /// Creates a relationship between two agents
    pub fn create_relationship(&mut self, from: AgentId, to: AgentId, kind: RelationshipKind) {
        let relationship = Relationship::new(from, to, kind);
        self.relationships.insert((from, to), relationship);

        // Add to graph
        let from_idx = self.get_or_create_node(from);
        let to_idx = self.get_or_create_node(to);
        self.graph.add_edge(from_idx, to_idx, kind);
    }

    /// Gets a relationship
    pub fn get(&self, from: AgentId, to: AgentId) -> Option<&Relationship> {
        self.relationships.get(&(from, to))
    }

    /// Gets a mutable relationship
    pub fn get_mut(&mut self, from: AgentId, to: AgentId) -> Option<&mut Relationship> {
        self.relationships.get_mut(&(from, to))
    }

    /// Gets or creates a relationship
    pub fn get_or_create(&mut self, from: AgentId, to: AgentId) -> &mut Relationship {
        if !self.relationships.contains_key(&(from, to)) {
            self.create_relationship(from, to, RelationshipKind::Acquaintance);
        }
        self.relationships.get_mut(&(from, to)).unwrap()
    }

    /// Gets all relationships for an agent
    pub fn relationships_of(&self, agent: AgentId) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|((from, to), _)| *from == agent || *to == agent)
            .map(|(_, r)| r)
            .collect()
    }

    /// Gets all friends of an agent
    pub fn friends_of(&self, agent: AgentId) -> Vec<AgentId> {
        self.relationships
            .iter()
            .filter(|((from, _), r)| *from == agent && r.is_friendly())
            .map(|((_, to), _)| *to)
            .collect()
    }

    /// Gets all enemies of an agent
    pub fn enemies_of(&self, agent: AgentId) -> Vec<AgentId> {
        self.relationships
            .iter()
            .filter(|((from, _), r)| *from == agent && r.is_hostile())
            .map(|((_, to), _)| *to)
            .collect()
    }

    /// Modifies a relationship
    pub fn modify_relationship(&mut self, from: AgentId, to: AgentId, amount: i32, turn: u32) {
        let relationship = self.get_or_create(from, to);
        relationship.modify(amount, turn);
    }

    /// Records an event between two agents
    pub fn record_event(
        &mut self,
        from: AgentId,
        to: AgentId,
        event_type: RelationshipEventType,
        description: &str,
        turn: u32,
    ) {
        let impact = match event_type {
            RelationshipEventType::Met => 5,
            RelationshipEventType::Talked => 3,
            RelationshipEventType::Helped => 15,
            RelationshipEventType::Betrayed => -30,
            RelationshipEventType::Fought => -20,
            RelationshipEventType::Traded => 5,
            RelationshipEventType::GaveGift => 20,
            RelationshipEventType::ReceivedGift => 10,
            RelationshipEventType::Insulted => -15,
            RelationshipEventType::Complimented => 10,
            RelationshipEventType::SavedLife => 50,
            RelationshipEventType::TookLife => -100,
            RelationshipEventType::SharesSecret => 25,
            RelationshipEventType::BrokePromise => -25,
            RelationshipEventType::KeptPromise => 15,
        };

        let event = RelationshipEvent {
            turn,
            event_type,
            description: description.to_string(),
            impact,
        };

        let relationship = self.get_or_create(from, to);
        relationship.modify(impact, turn);
        relationship.record_event(event.clone());

        // Update trust
        let positive = impact > 0;
        relationship.update_trust(positive);
    }

    /// Creates a social group
    pub fn create_group(&mut self, name: &str, members: Vec<AgentId>) -> usize {
        let group = SocialGroup {
            name: name.to_string(),
            members,
            leader: None,
            cohesion: 0.5,
        };
        self.groups.push(group);
        self.groups.len() - 1
    }

    /// Gets groups an agent belongs to
    pub fn groups_of(&self, agent: AgentId) -> Vec<&SocialGroup> {
        self.groups
            .iter()
            .filter(|g| g.members.contains(&agent))
            .collect()
    }

    /// Finds mutual friends between two agents
    pub fn mutual_friends(&self, a: AgentId, b: AgentId) -> Vec<AgentId> {
        let friends_a: std::collections::HashSet<_> = self.friends_of(a).into_iter().collect();
        let friends_b: std::collections::HashSet<_> = self.friends_of(b).into_iter().collect();
        friends_a.intersection(&friends_b).copied().collect()
    }

    /// Calculates relationship strength between two agents including indirect connections
    pub fn influence(&self, from: AgentId, to: AgentId) -> f32 {
        // Direct relationship
        if let Some(rel) = self.get(from, to) {
            return rel.strength as f32 / 100.0;
        }

        // Check for indirect connections through mutual friends
        let mutual = self.mutual_friends(from, to);
        if mutual.is_empty() {
            return 0.0;
        }

        // Average influence through mutual friends (weakened)
        let mut total = 0.0;
        for &friend in &mutual {
            let a_strength = self.get(from, friend).map(|r| r.strength).unwrap_or(0);
            let b_strength = self.get(friend, to).map(|r| r.strength).unwrap_or(0);
            total += (a_strength + b_strength) as f32 / 400.0;
        }
        total / mutual.len() as f32
    }

    /// Gets network statistics
    pub fn stats(&self) -> NetworkStats {
        let mut stats = NetworkStats::default();
        stats.total_relationships = self.relationships.len();

        for rel in self.relationships.values() {
            if rel.is_friendly() {
                stats.friendly += 1;
            } else if rel.is_hostile() {
                stats.hostile += 1;
            } else {
                stats.neutral += 1;
            }
        }

        stats.total_agents = self.node_indices.len();
        stats.total_groups = self.groups.len();

        stats
    }
}

/// A social group/clique
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SocialGroup {
    /// Group name
    pub name: String,
    /// Member agents
    pub members: Vec<AgentId>,
    /// Group leader
    pub leader: Option<AgentId>,
    /// How unified the group is (0.0 to 1.0)
    pub cohesion: f32,
}

impl SocialGroup {
    /// Adds a member
    pub fn add_member(&mut self, agent: AgentId) {
        if !self.members.contains(&agent) {
            self.members.push(agent);
        }
    }

    /// Removes a member
    pub fn remove_member(&mut self, agent: AgentId) {
        self.members.retain(|&id| id != agent);
        if self.leader == Some(agent) {
            self.leader = self.members.first().copied();
        }
    }

    /// Sets the leader
    pub fn set_leader(&mut self, agent: AgentId) {
        if self.members.contains(&agent) {
            self.leader = Some(agent);
        }
    }
}

/// Network statistics
#[derive(Clone, Debug, Default)]
pub struct NetworkStats {
    pub total_relationships: usize,
    pub friendly: usize,
    pub hostile: usize,
    pub neutral: usize,
    pub total_agents: usize,
    pub total_groups: usize,
}

/// Reputation with a specific entity
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Reputation {
    /// Raw reputation value (-100 to 100)
    pub value: i32,
    /// Fame/infamy (0 to 100)
    pub fame: u32,
    /// Titles earned
    pub titles: Vec<String>,
    /// Actions that affected reputation
    pub history: Vec<ReputationEvent>,
}

impl Reputation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modify(&mut self, amount: i32, reason: &str, turn: u32) {
        self.value = (self.value + amount).clamp(-100, 100);
        self.fame = (self.fame + amount.unsigned_abs()).min(100);
        self.history.push(ReputationEvent {
            turn,
            amount,
            reason: reason.to_string(),
        });
    }

    pub fn level(&self) -> ReputationLevel {
        ReputationLevel::from_value(self.value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub turn: u32,
    pub amount: i32,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReputationLevel {
    Infamous,
    Disliked,
    Unknown,
    Known,
    Respected,
    Famous,
    Legendary,
}

impl ReputationLevel {
    pub fn from_value(value: i32) -> Self {
        match value {
            i32::MIN..=-50 => Self::Infamous,
            -49..=-20 => Self::Disliked,
            -19..=19 => Self::Unknown,
            20..=39 => Self::Known,
            40..=59 => Self::Respected,
            60..=79 => Self::Famous,
            80..=i32::MAX => Self::Legendary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_creation() {
        let a = AgentId::new();
        let b = AgentId::new();
        let rel = Relationship::new(a, b, RelationshipKind::Friend);
        assert_eq!(rel.kind, RelationshipKind::Friend);
        assert_eq!(rel.strength, 0);
    }

    #[test]
    fn test_social_network() {
        let mut network = SocialNetwork::new();
        let a = AgentId::new();
        let b = AgentId::new();

        network.create_relationship(a, b, RelationshipKind::Friend);
        network.modify_relationship(a, b, 50, 1);

        let rel = network.get(a, b).unwrap();
        assert_eq!(rel.strength, 50);
        assert!(rel.is_friendly());
    }

    #[test]
    fn test_relationship_levels() {
        let a = AgentId::new();
        let b = AgentId::new();
        let mut rel = Relationship::new(a, b, RelationshipKind::Friend);

        rel.strength = 90;
        assert_eq!(rel.level(), RelationshipLevel::BestFriend);

        rel.strength = -80;
        assert_eq!(rel.level(), RelationshipLevel::Nemesis);
    }
}

//! Faction system for agents
//!
//! Manages faction membership, relations, reputation, and politics.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::fmt;

use crate::agents::{AgentId, AgentKind};

/// Unique identifier for a faction
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactionId(pub Uuid);

impl FactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_u128(value: u128) -> Self {
        Self(Uuid::from_u128(value))
    }
}

impl Default for FactionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for FactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Faction({})", &self.0.to_string()[..8])
    }
}

/// A faction in the game world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Faction {
    /// Unique identifier
    pub id: FactionId,
    /// Faction name
    pub name: String,
    /// Description
    pub description: String,
    /// Faction type
    pub faction_type: FactionType,
    /// Leader agent ID
    pub leader: Option<AgentId>,
    /// Member agent IDs
    pub members: Vec<AgentId>,
    /// Faction resources
    pub resources: FactionResources,
    /// Faction reputation with player
    pub player_reputation: i32,
    /// Faction goals
    pub goals: Vec<FactionGoal>,
    /// Faction traits
    pub traits: Vec<FactionTrait>,
    /// Territory positions
    pub territory: Vec<(usize, usize)>,
    /// Faction color (RGB)
    pub color: (u8, u8, u8),
}

impl Faction {
    /// Creates a new faction
    pub fn new(name: &str, faction_type: FactionType) -> Self {
        Self {
            id: FactionId::new(),
            name: name.to_string(),
            description: String::new(),
            faction_type,
            leader: None,
            members: Vec::new(),
            resources: FactionResources::default(),
            player_reputation: 0,
            goals: Vec::new(),
            traits: Vec::new(),
            territory: Vec::new(),
            color: (128, 128, 128),
        }
    }

    /// Sets the faction leader
    pub fn with_leader(mut self, leader: AgentId) -> Self {
        self.leader = Some(leader);
        if !self.members.contains(&leader) {
            self.members.push(leader);
        }
        self
    }

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

    /// Gets member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Checks if an agent is a member
    pub fn is_member(&self, agent: AgentId) -> bool {
        self.members.contains(&agent)
    }

    /// Modifies player reputation
    pub fn modify_reputation(&mut self, amount: i32) {
        self.player_reputation = (self.player_reputation + amount).clamp(-100, 100);
    }

    /// Gets reputation level
    pub fn reputation_level(&self) -> ReputationLevel {
        ReputationLevel::from_value(self.player_reputation)
    }

    /// Adds a trait
    pub fn add_trait(&mut self, trait_: FactionTrait) {
        if !self.traits.contains(&trait_) {
            self.traits.push(trait_);
        }
    }
}

/// Types of factions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionType {
    /// City or settlement
    Settlement,
    /// Merchant guild
    Guild,
    /// Military order
    Military,
    /// Religious order
    Religious,
    /// Criminal organization
    Criminal,
    /// Monster horde
    Monster,
    /// Ancient order
    Ancient,
    /// Neutral organization
    Neutral,
}

/// Faction resources
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FactionResources {
    /// Gold/money
    pub gold: u32,
    /// Food supplies
    pub food: u32,
    /// Weapons/armor
    pub weapons: u32,
    /// Magical resources
    pub magic: u32,
    /// Influence/political power
    pub influence: u32,
}

/// Faction goals
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionGoal {
    pub name: String,
    pub description: String,
    pub priority: u32,
    pub progress: f32,
    pub target: f32,
}

/// Faction traits that affect behavior
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionTrait {
    Militaristic,
    Peaceful,
    Mercantile,
    Religious,
    Secretive,
    Expansionist,
    Isolationist,
    Honorable,
    Treacherous,
    Xenophobic,
    Welcoming,
}

/// Reputation levels with factions
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReputationLevel {
    Hated,
    Hostile,
    Unfriendly,
    Neutral,
    Friendly,
    Honored,
    Exalted,
}

impl ReputationLevel {
    pub fn from_value(value: i32) -> Self {
        match value {
            i32::MIN..=-75 => Self::Hated,
            -74..=-50 => Self::Hostile,
            -49..=-25 => Self::Unfriendly,
            -24..=24 => Self::Neutral,
            25..=49 => Self::Friendly,
            50..=74 => Self::Honored,
            75..=i32::MAX => Self::Exalted,
        }
    }

    pub fn can_trade(&self) -> bool {
        *self >= Self::Unfriendly
    }

    pub fn can_quest(&self) -> bool {
        *self >= Self::Neutral
    }

    pub fn is_hostile(&self) -> bool {
        *self <= Self::Hostile
    }
}

/// Relationship between two factions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionRelation {
    /// Allied factions
    Allied,
    /// Friendly relations
    Friendly,
    /// No special relations
    Neutral,
    /// Unfriendly but not at war
    Unfriendly,
    /// At war
    AtWar,
    /// Vassal/subject
    Vassal,
    /// Overlord
    Overlord,
}

impl FactionRelation {
    /// Returns if this relation is hostile
    pub fn is_hostile(&self) -> bool {
        matches!(self, Self::Unfriendly | Self::AtWar)
    }

    /// Returns if this relation is friendly
    pub fn is_friendly(&self) -> bool {
        matches!(self, Self::Allied | Self::Friendly)
    }
}

/// Manages all factions
#[derive(Clone, Debug, Default)]
pub struct FactionManager {
    /// All factions
    factions: HashMap<FactionId, Faction>,
    /// Relations between factions
    relations: HashMap<(FactionId, FactionId), FactionRelation>,
    /// Agent to faction mapping
    agent_factions: HashMap<AgentId, FactionId>,
}

impl FactionManager {
    /// Creates a new faction manager
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.create_default_factions();
        manager
    }

    /// Creates default game factions
    fn create_default_factions(&mut self) {
        // Village faction
        let village = Faction::new("Village of Haven", FactionType::Settlement);
        let village_id = self.add_faction(village);

        // Merchant guild
        let merchants = Faction::new("Merchant's Guild", FactionType::Guild);
        let merchant_id = self.add_faction(merchants);

        // Temple
        let temple = Faction::new("Temple of Light", FactionType::Religious);
        let temple_id = self.add_faction(temple);

        // Guard corps
        let guards = Faction::new("Town Guard", FactionType::Military);
        let guard_id = self.add_faction(guards);

        // Thieves guild
        let thieves = Faction::new("Shadow Brotherhood", FactionType::Criminal);
        let thief_id = self.add_faction(thieves);

        // Goblin horde
        let goblins = Faction::new("Goblin Horde", FactionType::Monster);
        let goblin_id = self.add_faction(goblins);

        // Orc clan
        let orcs = Faction::new("Bloodfang Clan", FactionType::Monster);
        let orc_id = self.add_faction(orcs);

        // Undead legion
        let undead = Faction::new("Legion of the Dead", FactionType::Monster);
        let undead_id = self.add_faction(undead);

        // Demon cult
        let demons = Faction::new("Cult of Shadows", FactionType::Ancient);
        let demon_id = self.add_faction(demons);

        // Set up relations
        self.set_relation(village_id, merchant_id, FactionRelation::Friendly);
        self.set_relation(village_id, temple_id, FactionRelation::Allied);
        self.set_relation(village_id, guard_id, FactionRelation::Allied);
        self.set_relation(village_id, thief_id, FactionRelation::Unfriendly);
        self.set_relation(village_id, goblin_id, FactionRelation::AtWar);
        self.set_relation(village_id, orc_id, FactionRelation::AtWar);
        self.set_relation(village_id, undead_id, FactionRelation::AtWar);
        self.set_relation(village_id, demon_id, FactionRelation::AtWar);

        self.set_relation(temple_id, undead_id, FactionRelation::AtWar);
        self.set_relation(temple_id, demon_id, FactionRelation::AtWar);

        self.set_relation(goblin_id, orc_id, FactionRelation::Unfriendly);
        self.set_relation(undead_id, demon_id, FactionRelation::Friendly);
    }

    /// Adds a faction
    pub fn add_faction(&mut self, faction: Faction) -> FactionId {
        let id = faction.id;
        self.factions.insert(id, faction);
        id
    }

    /// Gets a faction
    pub fn get(&self, id: FactionId) -> Option<&Faction> {
        self.factions.get(&id)
    }

    /// Gets a mutable faction
    pub fn get_mut(&mut self, id: FactionId) -> Option<&mut Faction> {
        self.factions.get_mut(&id)
    }

    /// Gets all factions
    pub fn all(&self) -> impl Iterator<Item = &Faction> {
        self.factions.values()
    }

    /// Finds faction by name
    pub fn find_by_name(&self, name: &str) -> Option<&Faction> {
        self.factions.values().find(|f| f.name == name)
    }

    /// Gets the faction for an agent
    pub fn agent_faction(&self, agent: AgentId) -> Option<FactionId> {
        self.agent_factions.get(&agent).copied()
    }

    /// Sets the faction for an agent
    pub fn set_agent_faction(&mut self, agent: AgentId, faction: FactionId) {
        // Remove from old faction
        if let Some(old_faction) = self.agent_factions.get(&agent).copied() {
            if let Some(f) = self.factions.get_mut(&old_faction) {
                f.remove_member(agent);
            }
        }

        // Add to new faction
        self.agent_factions.insert(agent, faction);
        if let Some(f) = self.factions.get_mut(&faction) {
            f.add_member(agent);
        }
    }

    /// Sets the relation between two factions
    pub fn set_relation(&mut self, a: FactionId, b: FactionId, relation: FactionRelation) {
        // Store both directions
        self.relations.insert((a, b), relation);
        self.relations.insert((b, a), relation);
    }

    /// Gets the relation between two factions
    pub fn get_relation(&self, a: FactionId, b: FactionId) -> FactionRelation {
        self.relations.get(&(a, b)).copied().unwrap_or(FactionRelation::Neutral)
    }

    /// Checks if two agents are hostile to each other
    pub fn are_hostile(&self, agent_a: AgentId, agent_b: AgentId) -> bool {
        let faction_a = self.agent_factions.get(&agent_a);
        let faction_b = self.agent_factions.get(&agent_b);

        match (faction_a, faction_b) {
            (Some(&a), Some(&b)) => self.get_relation(a, b).is_hostile(),
            _ => false,
        }
    }

    /// Checks if two agents are friendly
    pub fn are_friendly(&self, agent_a: AgentId, agent_b: AgentId) -> bool {
        let faction_a = self.agent_factions.get(&agent_a);
        let faction_b = self.agent_factions.get(&agent_b);

        match (faction_a, faction_b) {
            (Some(&a), Some(&b)) => {
                a == b || self.get_relation(a, b).is_friendly()
            }
            _ => false,
        }
    }

    /// Gets the default faction for an agent kind
    pub fn default_faction_for(&self, kind: AgentKind) -> Option<FactionId> {
        let faction_name = match kind {
            // NPCs go to village
            AgentKind::VillageElder | AgentKind::Farmer | AgentKind::Baker |
            AgentKind::Fisher | AgentKind::Child => "Village of Haven",
            // Merchants go to guild
            AgentKind::Merchant | AgentKind::Traveler => "Merchant's Guild",
            // Religious go to temple
            AgentKind::Priest | AgentKind::Healer => "Temple of Light",
            // Guards go to guard corps
            AgentKind::Guard | AgentKind::Captain => "Town Guard",
            // Thieves go to shadow brotherhood
            AgentKind::Thief | AgentKind::Beggar => "Shadow Brotherhood",
            // Goblins
            AgentKind::GoblinScout | AgentKind::GoblinShaman => "Goblin Horde",
            // Orcs
            AgentKind::OrcWarrior | AgentKind::OrcChieftain => "Bloodfang Clan",
            // Undead
            AgentKind::SkeletonSoldier | AgentKind::SkeletonMage |
            AgentKind::ZombieHorde | AgentKind::VampireLord |
            AgentKind::EnemyNecromancer | AgentKind::LichKing => "Legion of the Dead",
            // Demons
            AgentKind::DemonSoldier | AgentKind::DemonLord |
            AgentKind::DarkKnight | AgentKind::ChaosBeast => "Cult of Shadows",
            _ => return None,
        };

        self.find_by_name(faction_name).map(|f| f.id)
    }
}

/// Political system for faction interactions
#[derive(Clone, Debug, Default)]
pub struct PoliticalSystem {
    /// Active treaties
    pub treaties: Vec<Treaty>,
    /// Recent political events
    pub events: Vec<PoliticalEvent>,
    /// War declarations
    pub wars: Vec<War>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Treaty {
    pub id: u32,
    pub name: String,
    pub parties: Vec<FactionId>,
    pub treaty_type: TreatyType,
    pub start_turn: u32,
    pub duration: Option<u32>,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatyType {
    Peace,
    Trade,
    Alliance,
    NonAggression,
    MutualDefense,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoliticalEvent {
    pub turn: u32,
    pub description: String,
    pub factions: Vec<FactionId>,
    pub impact: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct War {
    pub id: u32,
    pub name: String,
    pub attackers: Vec<FactionId>,
    pub defenders: Vec<FactionId>,
    pub start_turn: u32,
    pub battles: Vec<Battle>,
    pub active: bool,
    pub winner: Option<FactionId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Battle {
    pub turn: u32,
    pub location: (usize, usize),
    pub attacker_casualties: u32,
    pub defender_casualties: u32,
    pub winner: Option<FactionId>,
}

impl PoliticalSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_treaty(&mut self, treaty: Treaty) {
        self.treaties.push(treaty);
    }

    pub fn declare_war(&mut self, attackers: Vec<FactionId>, defenders: Vec<FactionId>, turn: u32) -> u32 {
        let id = self.wars.len() as u32;
        self.wars.push(War {
            id,
            name: format!("War #{}", id),
            attackers,
            defenders,
            start_turn: turn,
            battles: Vec::new(),
            active: true,
            winner: None,
        });
        id
    }

    pub fn end_war(&mut self, war_id: u32, winner: Option<FactionId>) {
        if let Some(war) = self.wars.iter_mut().find(|w| w.id == war_id) {
            war.active = false;
            war.winner = winner;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faction_creation() {
        let faction = Faction::new("Test Faction", FactionType::Settlement);
        assert_eq!(faction.name, "Test Faction");
        assert_eq!(faction.member_count(), 0);
    }

    #[test]
    fn test_faction_manager() {
        let manager = FactionManager::new();
        assert!(manager.factions.len() >= 5);
    }

    #[test]
    fn test_faction_relations() {
        let manager = FactionManager::new();
        let village = manager.find_by_name("Village of Haven").unwrap();
        let goblins = manager.find_by_name("Goblin Horde").unwrap();
        assert!(manager.get_relation(village.id, goblins.id).is_hostile());
    }
}

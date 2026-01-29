//! Relationships system
//!
//! NPC relationships, faction standing, and social interactions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Relationship level with an NPC
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RelationshipLevel {
    Hostile,
    Unfriendly,
    Neutral,
    Friendly,
    Trusted,
    Beloved,
    Soulbound,
}

impl Default for RelationshipLevel {
    fn default() -> Self {
        Self::Neutral
    }
}

impl RelationshipLevel {
    pub fn from_points(points: i32) -> Self {
        match points {
            p if p < -100 => Self::Hostile,
            p if p < -25 => Self::Unfriendly,
            p if p < 25 => Self::Neutral,
            p if p < 75 => Self::Friendly,
            p if p < 150 => Self::Trusted,
            p if p < 300 => Self::Beloved,
            _ => Self::Soulbound,
        }
    }

    pub fn shop_discount(&self) -> f32 {
        match self {
            Self::Hostile => 1.5,
            Self::Unfriendly => 1.2,
            Self::Neutral => 1.0,
            Self::Friendly => 0.95,
            Self::Trusted => 0.9,
            Self::Beloved => 0.8,
            Self::Soulbound => 0.7,
        }
    }
}

/// A relationship with an NPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcRelationship {
    pub npc_id: String,
    pub npc_name: String,
    pub points: i32,
    pub level: RelationshipLevel,
    pub interactions: u32,
    pub gifts_given: u32,
    pub quests_completed: u32,
}

impl NpcRelationship {
    pub fn new(npc_id: &str, npc_name: &str) -> Self {
        Self {
            npc_id: npc_id.to_string(),
            npc_name: npc_name.to_string(),
            points: 0,
            level: RelationshipLevel::Neutral,
            interactions: 0,
            gifts_given: 0,
            quests_completed: 0,
        }
    }

    pub fn add_points(&mut self, amount: i32) {
        self.points += amount;
        self.level = RelationshipLevel::from_points(self.points);
    }

    pub fn interact(&mut self) {
        self.interactions += 1;
        self.add_points(1);
    }

    pub fn give_gift(&mut self, value: i32) {
        self.gifts_given += 1;
        self.add_points(value / 10);
    }

    pub fn complete_quest(&mut self, difficulty: u32) {
        self.quests_completed += 1;
        self.add_points(difficulty as i32 * 5);
    }
}

/// Faction standing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FactionStanding {
    Exiled,
    Hated,
    Disliked,
    Neutral,
    Liked,
    Honored,
    Revered,
    Exalted,
}

impl Default for FactionStanding {
    fn default() -> Self {
        Self::Neutral
    }
}

impl FactionStanding {
    pub fn from_reputation(rep: i32) -> Self {
        match rep {
            r if r < -1000 => Self::Exiled,
            r if r < -500 => Self::Hated,
            r if r < -100 => Self::Disliked,
            r if r < 100 => Self::Neutral,
            r if r < 500 => Self::Liked,
            r if r < 1000 => Self::Honored,
            r if r < 2000 => Self::Revered,
            _ => Self::Exalted,
        }
    }
}

/// Relationship with a faction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionRelationship {
    pub faction_id: String,
    pub faction_name: String,
    pub reputation: i32,
    pub standing: FactionStanding,
    pub quests_completed: u32,
    pub enemies_killed: u32,
}

impl FactionRelationship {
    pub fn new(faction_id: &str, faction_name: &str) -> Self {
        Self {
            faction_id: faction_id.to_string(),
            faction_name: faction_name.to_string(),
            reputation: 0,
            standing: FactionStanding::Neutral,
            quests_completed: 0,
            enemies_killed: 0,
        }
    }

    pub fn add_reputation(&mut self, amount: i32) {
        self.reputation += amount;
        self.standing = FactionStanding::from_reputation(self.reputation);
    }
}

/// Player's relationships manager
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipsManager {
    pub npc_relationships: HashMap<String, NpcRelationship>,
    pub faction_relationships: HashMap<String, FactionRelationship>,
}

impl RelationshipsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_npc_relationship(&mut self, npc_id: &str, npc_name: &str) -> &mut NpcRelationship {
        self.npc_relationships
            .entry(npc_id.to_string())
            .or_insert_with(|| NpcRelationship::new(npc_id, npc_name))
    }

    pub fn get_faction_relationship(&mut self, faction_id: &str, faction_name: &str) -> &mut FactionRelationship {
        self.faction_relationships
            .entry(faction_id.to_string())
            .or_insert_with(|| FactionRelationship::new(faction_id, faction_name))
    }

    pub fn get_npc_level(&self, npc_id: &str) -> RelationshipLevel {
        self.npc_relationships
            .get(npc_id)
            .map(|r| r.level)
            .unwrap_or(RelationshipLevel::Neutral)
    }

    pub fn get_faction_standing(&self, faction_id: &str) -> FactionStanding {
        self.faction_relationships
            .get(faction_id)
            .map(|r| r.standing)
            .unwrap_or(FactionStanding::Neutral)
    }
}

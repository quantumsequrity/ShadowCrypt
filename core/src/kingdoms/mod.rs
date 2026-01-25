//! Kingdoms and Politics System
//!
//! 8 kingdoms with territories, rulers, armies, wars, and diplomacy.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The 8 major kingdoms
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kingdom {
    Valdoria,      // Human empire
    Sylvaneth,     // Elven realm
    Ironhold,      // Dwarven kingdom
    Grommash,      // Orc horde
    Necropolis,    // Undead dominion
    Infernium,     // Demon realm
    Wildlands,     // Beastkin tribes
    Celestia,      // Divine kingdom
}

impl Kingdom {
    pub fn all() -> &'static [Kingdom] {
        &[Self::Valdoria, Self::Sylvaneth, Self::Ironhold, Self::Grommash,
          Self::Necropolis, Self::Infernium, Self::Wildlands, Self::Celestia]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Valdoria => "Kingdom of Valdoria",
            Self::Sylvaneth => "Sylvaneth Dominion",
            Self::Ironhold => "Ironhold Citadel",
            Self::Grommash => "Grommash Horde",
            Self::Necropolis => "The Necropolis",
            Self::Infernium => "Infernium Depths",
            Self::Wildlands => "The Wildlands",
            Self::Celestia => "Celestial Realm",
        }
    }

    pub fn ruler_title(&self) -> &'static str {
        match self {
            Self::Valdoria => "King",
            Self::Sylvaneth => "High Queen",
            Self::Ironhold => "High Thane",
            Self::Grommash => "Warchief",
            Self::Necropolis => "Lich King",
            Self::Infernium => "Demon Lord",
            Self::Wildlands => "Alpha",
            Self::Celestia => "Archon",
        }
    }

    pub fn primary_species(&self) -> &'static str {
        match self {
            Self::Valdoria => "Human",
            Self::Sylvaneth => "Elf",
            Self::Ironhold => "Dwarf",
            Self::Grommash => "Orc",
            Self::Necropolis => "Undead",
            Self::Infernium => "Demon",
            Self::Wildlands => "Beastkin",
            Self::Celestia => "Celestial",
        }
    }

    pub fn default_relations(&self) -> HashMap<Kingdom, DiplomaticRelation> {
        let mut relations = HashMap::new();
        for other in Kingdom::all() {
            if *other != *self {
                let relation = match (self, other) {
                    // Allied pairs
                    (Self::Valdoria, Self::Ironhold) | (Self::Ironhold, Self::Valdoria) => DiplomaticRelation::Allied,
                    (Self::Sylvaneth, Self::Celestia) | (Self::Celestia, Self::Sylvaneth) => DiplomaticRelation::Allied,
                    // At war pairs
                    (Self::Valdoria, Self::Grommash) | (Self::Grommash, Self::Valdoria) => DiplomaticRelation::AtWar,
                    (Self::Sylvaneth, Self::Necropolis) | (Self::Necropolis, Self::Sylvaneth) => DiplomaticRelation::AtWar,
                    (Self::Celestia, Self::Infernium) | (Self::Infernium, Self::Celestia) => DiplomaticRelation::AtWar,
                    (Self::Celestia, Self::Necropolis) | (Self::Necropolis, Self::Celestia) => DiplomaticRelation::AtWar,
                    // Hostile
                    (Self::Infernium, _) | (_, Self::Infernium) => DiplomaticRelation::Hostile,
                    (Self::Necropolis, _) | (_, Self::Necropolis) => DiplomaticRelation::Hostile,
                    // Default neutral
                    _ => DiplomaticRelation::Neutral,
                };
                relations.insert(*other, relation);
            }
        }
        relations
    }
}

/// Diplomatic relations between kingdoms
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiplomaticRelation {
    Allied,
    Friendly,
    Neutral,
    Unfriendly,
    Hostile,
    AtWar,
}

impl DiplomaticRelation {
    pub fn reputation_modifier(&self) -> i32 {
        match self {
            Self::Allied => 50,
            Self::Friendly => 25,
            Self::Neutral => 0,
            Self::Unfriendly => -25,
            Self::Hostile => -50,
            Self::AtWar => -100,
        }
    }
}

/// A kingdom's state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KingdomState {
    pub kingdom: Kingdom,
    pub ruler_name: String,
    pub treasury: u64,
    pub army_size: u32,
    pub territory_count: u32,
    pub population: u64,
    pub relations: HashMap<Kingdom, DiplomaticRelation>,
    pub player_reputation: i32,
    pub active_wars: Vec<Kingdom>,
    pub treaties: Vec<Treaty>,
}

impl KingdomState {
    pub fn new(kingdom: Kingdom) -> Self {
        Self {
            kingdom,
            ruler_name: Self::generate_ruler_name(kingdom),
            treasury: 100000,
            army_size: 5000,
            territory_count: 10,
            population: 100000,
            relations: kingdom.default_relations(),
            player_reputation: 0,
            active_wars: Vec::new(),
            treaties: Vec::new(),
        }
    }

    fn generate_ruler_name(kingdom: Kingdom) -> String {
        match kingdom {
            Kingdom::Valdoria => "King Aldric III".to_string(),
            Kingdom::Sylvaneth => "High Queen Aelindra".to_string(),
            Kingdom::Ironhold => "High Thane Borin Steelbeard".to_string(),
            Kingdom::Grommash => "Warchief Grom'thar".to_string(),
            Kingdom::Necropolis => "The Lich King Malachar".to_string(),
            Kingdom::Infernium => "Demon Lord Azgoroth".to_string(),
            Kingdom::Wildlands => "Alpha Fenris".to_string(),
            Kingdom::Celestia => "Archon Seraphiel".to_string(),
        }
    }

    pub fn reputation_level(&self) -> ReputationLevel {
        match self.player_reputation {
            r if r >= 100 => ReputationLevel::Exalted,
            r if r >= 50 => ReputationLevel::Revered,
            r if r >= 25 => ReputationLevel::Honored,
            r if r >= 0 => ReputationLevel::Neutral,
            r if r >= -25 => ReputationLevel::Unfriendly,
            r if r >= -50 => ReputationLevel::Hostile,
            _ => ReputationLevel::Hated,
        }
    }

    pub fn modify_reputation(&mut self, amount: i32) {
        self.player_reputation = (self.player_reputation + amount).clamp(-100, 100);
    }

    pub fn declare_war(&mut self, target: Kingdom) {
        if !self.active_wars.contains(&target) {
            self.active_wars.push(target);
            self.relations.insert(target, DiplomaticRelation::AtWar);
        }
    }

    pub fn make_peace(&mut self, target: Kingdom) {
        self.active_wars.retain(|k| *k != target);
        self.relations.insert(target, DiplomaticRelation::Unfriendly);
    }
}

/// Player reputation levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReputationLevel {
    Exalted,
    Revered,
    Honored,
    Neutral,
    Unfriendly,
    Hostile,
    Hated,
}

impl ReputationLevel {
    pub fn shop_discount(&self) -> f32 {
        match self {
            Self::Exalted => 0.20,
            Self::Revered => 0.15,
            Self::Honored => 0.10,
            Self::Neutral => 0.0,
            Self::Unfriendly => -0.10,
            Self::Hostile => -0.25,
            Self::Hated => -0.50,
        }
    }
}

/// Treaties between kingdoms
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Treaty {
    pub kind: TreatyKind,
    pub parties: [Kingdom; 2],
    pub duration: u32,
    pub terms: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatyKind {
    Peace,
    Trade,
    Alliance,
    NonAggression,
    MutualDefense,
}

/// World politics manager
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PoliticsSystem {
    pub kingdoms: HashMap<Kingdom, KingdomState>,
    pub world_events: Vec<WorldEvent>,
    pub turn: u32,
}

impl PoliticsSystem {
    pub fn new() -> Self {
        let mut kingdoms = HashMap::new();
        for k in Kingdom::all() {
            kingdoms.insert(*k, KingdomState::new(*k));
        }
        Self {
            kingdoms,
            world_events: Vec::new(),
            turn: 0,
        }
    }

    pub fn tick(&mut self) {
        self.turn += 1;
        // Process kingdom actions, wars, etc.
    }

    pub fn get_kingdom(&self, kingdom: Kingdom) -> Option<&KingdomState> {
        self.kingdoms.get(&kingdom)
    }

    pub fn get_kingdom_mut(&mut self, kingdom: Kingdom) -> Option<&mut KingdomState> {
        self.kingdoms.get_mut(&kingdom)
    }

    pub fn player_reputation(&self, kingdom: Kingdom) -> i32 {
        self.kingdoms.get(&kingdom).map(|k| k.player_reputation).unwrap_or(0)
    }

    pub fn modify_player_reputation(&mut self, kingdom: Kingdom, amount: i32) {
        if let Some(k) = self.kingdoms.get_mut(&kingdom) {
            k.modify_reputation(amount);
        }
    }
}

/// World events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEvent {
    pub kind: WorldEventKind,
    pub description: String,
    pub turn_occurred: u32,
    pub affected_kingdoms: Vec<Kingdom>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldEventKind {
    WarDeclared,
    PeaceTreaty,
    RulerDied,
    Invasion,
    Plague,
    Famine,
    Discovery,
    Alliance,
    Betrayal,
}

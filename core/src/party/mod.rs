//! Party System
//!
//! Form parties with NPCs and companions, manage formations,
//! share loot, and coordinate tactics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum party size
pub const MAX_PARTY_SIZE: usize = 6;

/// Party member types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PartyMember {
    Player,
    Companion { id: u32, name: String, class: String },
    Hireling { id: u32, name: String, class: String, cost_per_turn: u32 },
    Summon { id: u32, name: String, duration: u32 },
    Npc { id: u32, name: String, temporary: bool },
}

impl PartyMember {
    pub fn name(&self) -> &str {
        match self {
            Self::Player => "You",
            Self::Companion { name, .. } => name,
            Self::Hireling { name, .. } => name,
            Self::Summon { name, .. } => name,
            Self::Npc { name, .. } => name,
        }
    }

    pub fn is_temporary(&self) -> bool {
        match self {
            Self::Player => false,
            Self::Companion { .. } => false,
            Self::Hireling { .. } => true,
            Self::Summon { .. } => true,
            Self::Npc { temporary, .. } => *temporary,
        }
    }
}

/// Party formation types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Formation {
    Line,           // Single file
    Wedge,          // V-shape, leader at front
    Circle,         // Defensive circle
    Scattered,      // Spread out
    Column,         // Two-by-two march
    Defensive,      // Tank front, others back
    Offensive,      // DPS front, support back
}

impl Formation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Line => "Single Line",
            Self::Wedge => "Wedge Formation",
            Self::Circle => "Defensive Circle",
            Self::Scattered => "Scattered",
            Self::Column => "Column March",
            Self::Defensive => "Defensive Stance",
            Self::Offensive => "Offensive Push",
        }
    }

    pub fn bonuses(&self) -> FormationBonuses {
        match self {
            Self::Line => FormationBonuses { attack: 0, defense: 0, speed: 5, stealth: 10 },
            Self::Wedge => FormationBonuses { attack: 10, defense: -5, speed: 0, stealth: -5 },
            Self::Circle => FormationBonuses { attack: -5, defense: 15, speed: -10, stealth: -10 },
            Self::Scattered => FormationBonuses { attack: 0, defense: -5, speed: 10, stealth: 15 },
            Self::Column => FormationBonuses { attack: 0, defense: 5, speed: 5, stealth: 0 },
            Self::Defensive => FormationBonuses { attack: -10, defense: 20, speed: -5, stealth: 0 },
            Self::Offensive => FormationBonuses { attack: 15, defense: -10, speed: 5, stealth: -10 },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct FormationBonuses {
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub stealth: i32,
}

/// Party role
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartyRole {
    Leader,
    Tank,
    DPS,
    Healer,
    Support,
    Scout,
}

/// Loot distribution modes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LootMode {
    FreeForAll,
    RoundRobin,
    NeedBeforeGreed,
    MasterLooter,
    GroupLoot,
}

/// Party tactics
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartyTactic {
    Aggressive,     // Attack on sight
    Defensive,      // Only attack when attacked
    Cautious,       // Avoid combat if possible
    Stealthy,       // Prioritize stealth
    Balanced,       // Mix of offense and defense
}

/// A party
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Party {
    pub name: String,
    pub members: Vec<PartyMember>,
    pub roles: HashMap<u32, PartyRole>,
    pub formation: Formation,
    pub tactic: PartyTactic,
    pub loot_mode: LootMode,
    pub shared_gold: u32,
    pub morale: i32,
    pub experience_shared: bool,
    pub active: bool,
}

impl Party {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            members: vec![PartyMember::Player],
            roles: HashMap::new(),
            formation: Formation::Column,
            tactic: PartyTactic::Balanced,
            loot_mode: LootMode::FreeForAll,
            shared_gold: 0,
            morale: 100,
            experience_shared: true,
            active: true,
        }
    }

    pub fn add_member(&mut self, member: PartyMember) -> bool {
        if self.members.len() >= MAX_PARTY_SIZE {
            return false;
        }
        self.members.push(member);
        true
    }

    pub fn remove_member(&mut self, index: usize) -> Option<PartyMember> {
        if index > 0 && index < self.members.len() {
            Some(self.members.remove(index))
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.members.len()
    }

    pub fn is_full(&self) -> bool {
        self.members.len() >= MAX_PARTY_SIZE
    }

    pub fn set_role(&mut self, member_id: u32, role: PartyRole) {
        self.roles.insert(member_id, role);
    }

    pub fn formation_bonuses(&self) -> FormationBonuses {
        self.formation.bonuses()
    }

    pub fn tick(&mut self) {
        // Process summon durations
        self.members.retain(|m| {
            if let PartyMember::Summon { duration, .. } = m {
                *duration > 0
            } else {
                true
            }
        });

        // Decrease summon durations
        for member in &mut self.members {
            if let PartyMember::Summon { duration, .. } = member {
                *duration = duration.saturating_sub(1);
            }
        }
    }

    pub fn xp_share(&self, total_xp: u32) -> u32 {
        if self.experience_shared {
            let member_count = self.members.len() as u32;
            // Bonus for party play
            let bonus = match member_count {
                1 => 100,
                2 => 95,
                3 => 90,
                4 => 85,
                5 => 80,
                _ => 75,
            };
            (total_xp * bonus) / 100
        } else {
            total_xp
        }
    }

    pub fn gold_share(&self, total_gold: u32) -> u32 {
        total_gold / self.members.len() as u32
    }

    pub fn modify_morale(&mut self, amount: i32) {
        self.morale = (self.morale + amount).clamp(0, 100);
    }

    pub fn morale_modifier(&self) -> f32 {
        match self.morale {
            m if m >= 80 => 1.1,
            m if m >= 50 => 1.0,
            m if m >= 25 => 0.9,
            _ => 0.75,
        }
    }
}

/// Party system manager
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PartySystem {
    pub current_party: Option<Party>,
    pub saved_parties: Vec<Party>,
    pub hireling_roster: Vec<HirelingInfo>,
}

impl PartySystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_party(&mut self, name: &str) {
        self.current_party = Some(Party::new(name));
    }

    pub fn disband(&mut self) {
        if let Some(party) = self.current_party.take() {
            self.saved_parties.push(party);
        }
    }

    pub fn party(&self) -> Option<&Party> {
        self.current_party.as_ref()
    }

    pub fn party_mut(&mut self) -> Option<&mut Party> {
        self.current_party.as_mut()
    }

    pub fn tick(&mut self) {
        if let Some(party) = &mut self.current_party {
            party.tick();
        }
    }
}

/// Info about hirelings for hire
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HirelingInfo {
    pub id: u32,
    pub name: String,
    pub class: String,
    pub level: u32,
    pub cost_per_turn: u32,
    pub hire_cost: u32,
    pub skills: Vec<String>,
}

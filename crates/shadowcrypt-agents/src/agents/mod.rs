//! Core agent definitions and types
//!
//! This module contains 75+ different agent types organized by category:
//! - NPCs (Villagers, Merchants, Craftsmen)
//! - Enemies (Monsters, Bosses, Minions)
//! - Companions (Pets, Summons, Allies)
//! - Environmental (Weather, Time, Nature)
//! - System (Quest givers, Narrators, Guides)

mod base;
mod npc;
mod enemy;
mod companion;
mod environmental;
mod system;
mod manager;

pub use base::*;
pub use npc::*;
pub use enemy::*;
pub use companion::*;
pub use environmental::*;
pub use system::*;
pub use manager::*;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt;

/// Unique identifier for an agent
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);

impl AgentId {
    /// Creates a new random agent ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an agent ID from a u128
    pub fn from_u128(value: u128) -> Self {
        Self(Uuid::from_u128(value))
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Agent({})", &self.0.to_string()[..8])
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

/// The 75+ agent kinds organized by category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentKind {
    // === NPC Agents (25 types) ===
    /// Village elder - wise leader
    VillageElder,
    /// Blacksmith - crafts weapons
    Blacksmith,
    /// Alchemist - creates potions
    Alchemist,
    /// Merchant - trades goods
    Merchant,
    /// Innkeeper - provides rest
    Innkeeper,
    /// Farmer - grows food
    Farmer,
    /// Guard - protects areas
    Guard,
    /// Captain - leads guards
    Captain,
    /// Priest - heals and blesses
    Priest,
    /// Scholar - provides knowledge
    Scholar,
    /// Librarian - guards ancient tomes
    Librarian,
    /// Bard - tells stories and sings
    Bard,
    /// Thief - pickpockets and steals
    Thief,
    /// Beggar - asks for alms
    Beggar,
    /// Noble - wealthy aristocrat
    Noble,
    /// Child - innocent youth
    Child,
    /// Hermit - reclusive sage
    Hermit,
    /// Miner - extracts ores
    Miner,
    /// Hunter - tracks beasts
    Hunter,
    /// Fisher - catches fish
    Fisher,
    /// Baker - makes bread
    Baker,
    /// Healer - treats wounds
    Healer,
    /// Wizard - practices magic
    Wizard,
    /// Apprentice - learning trades
    Apprentice,
    /// Traveler - wanders the land
    Traveler,

    // === Enemy Agents (20 types) ===
    /// Goblin scout - weak but numerous
    GoblinScout,
    /// Goblin shaman - casts curses
    GoblinShaman,
    /// Orc warrior - brutal fighter
    OrcWarrior,
    /// Orc chieftain - leads war bands
    OrcChieftain,
    /// Skeleton soldier - undead fighter
    SkeletonSoldier,
    /// Skeleton mage - undead caster
    SkeletonMage,
    /// Zombie horde - shambling mass
    ZombieHorde,
    /// Vampire lord - drains life
    VampireLord,
    /// Werewolf - savage beast
    Werewolf,
    /// Bandit leader - criminal boss
    BanditLeader,
    /// Assassin - deadly killer
    Assassin,
    /// Dark knight - fallen paladin
    DarkKnight,
    /// Necromancer - raises dead
    EnemyNecromancer,
    /// Demon soldier - hellish warrior
    DemonSoldier,
    /// Demon lord - powerful fiend
    DemonLord,
    /// Dragon whelp - young dragon
    DragonWhelp,
    /// Elder dragon - ancient wyrm
    ElderDragon,
    /// Lich king - undead sorcerer
    LichKing,
    /// Chaos beast - corrupted monster
    ChaosBeast,
    /// Shadow assassin - dark killer
    ShadowAssassin,

    // === Companion Agents (15 types) ===
    /// Wolf companion - loyal pet
    WolfCompanion,
    /// Bear companion - strong protector
    BearCompanion,
    /// Hawk companion - aerial scout
    HawkCompanion,
    /// Skeleton minion - summoned undead
    SkeletonMinion,
    /// Fire elemental - summoned flame
    FireElemental,
    /// Ice elemental - summoned frost
    IceElemental,
    /// Earth elemental - summoned stone
    EarthElemental,
    /// Spirit guide - ethereal helper
    SpiritGuide,
    /// Mercenary - hired sword
    Mercenary,
    /// Squire - loyal assistant
    Squire,
    /// Familiar - magical pet
    Familiar,
    /// Golem - constructed servant
    Golem,
    /// Fairy companion - magical helper
    FairyCompanion,
    /// Shadow clone - dark duplicate
    ShadowClone,
    /// Guardian angel - divine protector
    GuardianAngel,

    // === Environmental Agents (10 types) ===
    /// Weather controller - manages climate
    WeatherController,
    /// Day/night cycle - manages time
    DayNightCycle,
    /// Season manager - manages seasons
    SeasonManager,
    /// Wildlife spawner - creates animals
    WildlifeSpawner,
    /// Vegetation manager - grows plants
    VegetationManager,
    /// Event spawner - triggers events
    EventSpawner,
    /// Ambient sound - manages audio
    AmbientSound,
    /// Fog controller - manages visibility
    FogController,
    /// Earthquake - ground shaker
    Earthquake,
    /// Storm - weather event
    Storm,

    // === System Agents (5 types) ===
    /// Quest giver - assigns quests
    QuestGiver,
    /// Narrator - tells the story
    Narrator,
    /// Tutorial guide - teaches players
    TutorialGuide,
    /// Achievement tracker - tracks progress
    AchievementTracker,
    /// Difficulty adjuster - balances game
    DifficultyAdjuster,
}

impl AgentKind {
    /// Returns the display name of the agent kind
    pub fn name(&self) -> &'static str {
        match self {
            // NPCs
            Self::VillageElder => "Village Elder",
            Self::Blacksmith => "Blacksmith",
            Self::Alchemist => "Alchemist",
            Self::Merchant => "Merchant",
            Self::Innkeeper => "Innkeeper",
            Self::Farmer => "Farmer",
            Self::Guard => "Guard",
            Self::Captain => "Guard Captain",
            Self::Priest => "Priest",
            Self::Scholar => "Scholar",
            Self::Librarian => "Librarian",
            Self::Bard => "Bard",
            Self::Thief => "Thief",
            Self::Beggar => "Beggar",
            Self::Noble => "Noble",
            Self::Child => "Child",
            Self::Hermit => "Hermit",
            Self::Miner => "Miner",
            Self::Hunter => "Hunter",
            Self::Fisher => "Fisher",
            Self::Baker => "Baker",
            Self::Healer => "Healer",
            Self::Wizard => "Wizard",
            Self::Apprentice => "Apprentice",
            Self::Traveler => "Traveler",
            // Enemies
            Self::GoblinScout => "Goblin Scout",
            Self::GoblinShaman => "Goblin Shaman",
            Self::OrcWarrior => "Orc Warrior",
            Self::OrcChieftain => "Orc Chieftain",
            Self::SkeletonSoldier => "Skeleton Soldier",
            Self::SkeletonMage => "Skeleton Mage",
            Self::ZombieHorde => "Zombie Horde",
            Self::VampireLord => "Vampire Lord",
            Self::Werewolf => "Werewolf",
            Self::BanditLeader => "Bandit Leader",
            Self::Assassin => "Assassin",
            Self::DarkKnight => "Dark Knight",
            Self::EnemyNecromancer => "Necromancer",
            Self::DemonSoldier => "Demon Soldier",
            Self::DemonLord => "Demon Lord",
            Self::DragonWhelp => "Dragon Whelp",
            Self::ElderDragon => "Elder Dragon",
            Self::LichKing => "Lich King",
            Self::ChaosBeast => "Chaos Beast",
            Self::ShadowAssassin => "Shadow Assassin",
            // Companions
            Self::WolfCompanion => "Wolf",
            Self::BearCompanion => "Bear",
            Self::HawkCompanion => "Hawk",
            Self::SkeletonMinion => "Skeleton Minion",
            Self::FireElemental => "Fire Elemental",
            Self::IceElemental => "Ice Elemental",
            Self::EarthElemental => "Earth Elemental",
            Self::SpiritGuide => "Spirit Guide",
            Self::Mercenary => "Mercenary",
            Self::Squire => "Squire",
            Self::Familiar => "Familiar",
            Self::Golem => "Golem",
            Self::FairyCompanion => "Fairy",
            Self::ShadowClone => "Shadow Clone",
            Self::GuardianAngel => "Guardian Angel",
            // Environmental
            Self::WeatherController => "Weather Spirit",
            Self::DayNightCycle => "Time Keeper",
            Self::SeasonManager => "Season Spirit",
            Self::WildlifeSpawner => "Nature Spirit",
            Self::VegetationManager => "Plant Spirit",
            Self::EventSpawner => "Fate Weaver",
            Self::AmbientSound => "Echo Spirit",
            Self::FogController => "Mist Spirit",
            Self::Earthquake => "Earth Tremor",
            Self::Storm => "Storm Spirit",
            // System
            Self::QuestGiver => "Quest Giver",
            Self::Narrator => "Narrator",
            Self::TutorialGuide => "Guide",
            Self::AchievementTracker => "Achievement Spirit",
            Self::DifficultyAdjuster => "Balance Spirit",
        }
    }

    /// Returns the glyph used to display this agent
    pub fn glyph(&self) -> char {
        match self {
            // NPCs get human glyphs
            Self::VillageElder | Self::Priest | Self::Hermit => '☥',
            Self::Blacksmith | Self::Miner => '⚒',
            Self::Alchemist | Self::Wizard | Self::Healer => '⚗',
            Self::Merchant | Self::Innkeeper | Self::Baker => '⚖',
            Self::Guard | Self::Captain => '⚔',
            Self::Farmer | Self::Fisher | Self::Hunter => '⚜',
            Self::Scholar | Self::Librarian => '📖',
            Self::Bard => '♪',
            Self::Thief | Self::Beggar => '👤',
            Self::Noble => '♔',
            Self::Child | Self::Apprentice => '☺',
            Self::Traveler => '⚶',
            // Enemies
            Self::GoblinScout | Self::GoblinShaman => 'g',
            Self::OrcWarrior | Self::OrcChieftain => 'o',
            Self::SkeletonSoldier | Self::SkeletonMage => 's',
            Self::ZombieHorde => 'z',
            Self::VampireLord => 'V',
            Self::Werewolf => 'W',
            Self::BanditLeader => 'B',
            Self::Assassin | Self::ShadowAssassin => 'A',
            Self::DarkKnight => 'K',
            Self::EnemyNecromancer | Self::LichKing => 'N',
            Self::DemonSoldier => 'd',
            Self::DemonLord => 'D',
            Self::DragonWhelp => 'w',
            Self::ElderDragon => '🐉',
            Self::ChaosBeast => 'X',
            // Companions
            Self::WolfCompanion => '🐺',
            Self::BearCompanion => '🐻',
            Self::HawkCompanion => '🦅',
            Self::SkeletonMinion => '☠',
            Self::FireElemental => '🔥',
            Self::IceElemental => '❄',
            Self::EarthElemental => '�ite',
            Self::SpiritGuide => '👻',
            Self::Mercenary => '⚔',
            Self::Squire => '🛡',
            Self::Familiar => '🐱',
            Self::Golem => '🗿',
            Self::FairyCompanion => '✨',
            Self::ShadowClone => '◐',
            Self::GuardianAngel => '👼',
            // Environmental (invisible but have representation)
            Self::WeatherController => '☁',
            Self::DayNightCycle => '☀',
            Self::SeasonManager => '🍂',
            Self::WildlifeSpawner => '🌿',
            Self::VegetationManager => '🌱',
            Self::EventSpawner => '⚡',
            Self::AmbientSound => '♫',
            Self::FogController => '🌫',
            Self::Earthquake => '⚡',
            Self::Storm => '⛈',
            // System (usually invisible)
            Self::QuestGiver => '❗',
            Self::Narrator => '📜',
            Self::TutorialGuide => '❓',
            Self::AchievementTracker => '🏆',
            Self::DifficultyAdjuster => '⚙',
        }
    }

    /// Returns the category of this agent
    pub fn category(&self) -> AgentCategory {
        match self {
            Self::VillageElder | Self::Blacksmith | Self::Alchemist |
            Self::Merchant | Self::Innkeeper | Self::Farmer |
            Self::Guard | Self::Captain | Self::Priest |
            Self::Scholar | Self::Librarian | Self::Bard |
            Self::Thief | Self::Beggar | Self::Noble |
            Self::Child | Self::Hermit | Self::Miner |
            Self::Hunter | Self::Fisher | Self::Baker |
            Self::Healer | Self::Wizard | Self::Apprentice |
            Self::Traveler => AgentCategory::Npc,

            Self::GoblinScout | Self::GoblinShaman | Self::OrcWarrior |
            Self::OrcChieftain | Self::SkeletonSoldier | Self::SkeletonMage |
            Self::ZombieHorde | Self::VampireLord | Self::Werewolf |
            Self::BanditLeader | Self::Assassin | Self::DarkKnight |
            Self::EnemyNecromancer | Self::DemonSoldier | Self::DemonLord |
            Self::DragonWhelp | Self::ElderDragon | Self::LichKing |
            Self::ChaosBeast | Self::ShadowAssassin => AgentCategory::Enemy,

            Self::WolfCompanion | Self::BearCompanion | Self::HawkCompanion |
            Self::SkeletonMinion | Self::FireElemental | Self::IceElemental |
            Self::EarthElemental | Self::SpiritGuide | Self::Mercenary |
            Self::Squire | Self::Familiar | Self::Golem |
            Self::FairyCompanion | Self::ShadowClone | Self::GuardianAngel => AgentCategory::Companion,

            Self::WeatherController | Self::DayNightCycle | Self::SeasonManager |
            Self::WildlifeSpawner | Self::VegetationManager | Self::EventSpawner |
            Self::AmbientSound | Self::FogController | Self::Earthquake |
            Self::Storm => AgentCategory::Environmental,

            Self::QuestGiver | Self::Narrator | Self::TutorialGuide |
            Self::AchievementTracker | Self::DifficultyAdjuster => AgentCategory::System,
        }
    }

    /// Returns all agent kinds
    pub fn all() -> Vec<AgentKind> {
        vec![
            // NPCs
            Self::VillageElder, Self::Blacksmith, Self::Alchemist,
            Self::Merchant, Self::Innkeeper, Self::Farmer,
            Self::Guard, Self::Captain, Self::Priest,
            Self::Scholar, Self::Librarian, Self::Bard,
            Self::Thief, Self::Beggar, Self::Noble,
            Self::Child, Self::Hermit, Self::Miner,
            Self::Hunter, Self::Fisher, Self::Baker,
            Self::Healer, Self::Wizard, Self::Apprentice,
            Self::Traveler,
            // Enemies
            Self::GoblinScout, Self::GoblinShaman, Self::OrcWarrior,
            Self::OrcChieftain, Self::SkeletonSoldier, Self::SkeletonMage,
            Self::ZombieHorde, Self::VampireLord, Self::Werewolf,
            Self::BanditLeader, Self::Assassin, Self::DarkKnight,
            Self::EnemyNecromancer, Self::DemonSoldier, Self::DemonLord,
            Self::DragonWhelp, Self::ElderDragon, Self::LichKing,
            Self::ChaosBeast, Self::ShadowAssassin,
            // Companions
            Self::WolfCompanion, Self::BearCompanion, Self::HawkCompanion,
            Self::SkeletonMinion, Self::FireElemental, Self::IceElemental,
            Self::EarthElemental, Self::SpiritGuide, Self::Mercenary,
            Self::Squire, Self::Familiar, Self::Golem,
            Self::FairyCompanion, Self::ShadowClone, Self::GuardianAngel,
            // Environmental
            Self::WeatherController, Self::DayNightCycle, Self::SeasonManager,
            Self::WildlifeSpawner, Self::VegetationManager, Self::EventSpawner,
            Self::AmbientSound, Self::FogController, Self::Earthquake,
            Self::Storm,
            // System
            Self::QuestGiver, Self::Narrator, Self::TutorialGuide,
            Self::AchievementTracker, Self::DifficultyAdjuster,
        ]
    }

    /// Returns the base stats for this agent kind
    pub fn base_stats(&self) -> AgentStats {
        match self.category() {
            AgentCategory::Npc => AgentStats {
                max_hp: 50,
                attack: 5,
                defense: 3,
                speed: 10,
                intelligence: 50,
                perception: 40,
            },
            AgentCategory::Enemy => {
                let tier = match self {
                    Self::GoblinScout | Self::SkeletonSoldier => 1,
                    Self::GoblinShaman | Self::OrcWarrior | Self::ZombieHorde => 2,
                    Self::OrcChieftain | Self::SkeletonMage | Self::Werewolf => 3,
                    Self::BanditLeader | Self::Assassin | Self::VampireLord => 4,
                    Self::DarkKnight | Self::EnemyNecromancer | Self::DragonWhelp => 5,
                    Self::DemonSoldier | Self::ChaosBeast | Self::ShadowAssassin => 6,
                    Self::DemonLord | Self::ElderDragon | Self::LichKing => 7,
                    _ => 3,
                };
                AgentStats {
                    max_hp: 30 + tier * 20,
                    attack: 5 + tier * 3,
                    defense: 2 + tier * 2,
                    speed: 8 + tier,
                    intelligence: 20 + tier * 10,
                    perception: 30 + tier * 5,
                }
            }
            AgentCategory::Companion => AgentStats {
                max_hp: 40,
                attack: 8,
                defense: 5,
                speed: 12,
                intelligence: 30,
                perception: 60,
            },
            AgentCategory::Environmental => AgentStats {
                max_hp: 999,
                attack: 0,
                defense: 999,
                speed: 100,
                intelligence: 100,
                perception: 100,
            },
            AgentCategory::System => AgentStats {
                max_hp: 999,
                attack: 0,
                defense: 999,
                speed: 100,
                intelligence: 100,
                perception: 100,
            },
        }
    }
}

/// Category of agent
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentCategory {
    /// Non-player characters
    Npc,
    /// Enemy agents
    Enemy,
    /// Companion agents
    Companion,
    /// Environmental agents
    Environmental,
    /// System agents
    System,
}

/// Base stats for an agent
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct AgentStats {
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub intelligence: i32,
    pub perception: i32,
}

/// Current state of an agent
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentState {
    /// Agent is idle
    Idle,
    /// Agent is moving
    Moving,
    /// Agent is attacking
    Attacking,
    /// Agent is defending
    Defending,
    /// Agent is fleeing
    Fleeing,
    /// Agent is patrolling
    Patrolling,
    /// Agent is talking
    Talking,
    /// Agent is trading
    Trading,
    /// Agent is crafting
    Crafting,
    /// Agent is resting
    Resting,
    /// Agent is dead
    Dead,
    /// Agent is following
    Following,
    /// Agent is searching
    Searching,
    /// Agent is casting
    Casting,
    /// Agent is stunned
    Stunned,
    /// Agent is hiding
    Hiding,
}

impl AgentState {
    /// Returns true if the agent can act
    pub fn can_act(&self) -> bool {
        matches!(
            self,
            Self::Idle | Self::Moving | Self::Patrolling |
            Self::Searching | Self::Following
        )
    }

    /// Returns true if the agent is hostile
    pub fn is_hostile(&self) -> bool {
        matches!(self, Self::Attacking | Self::Casting)
    }

    /// Returns true if the agent is passive
    pub fn is_passive(&self) -> bool {
        matches!(
            self,
            Self::Idle | Self::Talking | Self::Trading |
            Self::Crafting | Self::Resting
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id_creation() {
        let id1 = AgentId::new();
        let id2 = AgentId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_agent_kinds_count() {
        assert!(AgentKind::all().len() >= 75);
    }

    #[test]
    fn test_agent_categories() {
        assert_eq!(AgentKind::Blacksmith.category(), AgentCategory::Npc);
        assert_eq!(AgentKind::GoblinScout.category(), AgentCategory::Enemy);
        assert_eq!(AgentKind::WolfCompanion.category(), AgentCategory::Companion);
    }
}

//! Achievement system for ShadowCrypt
//!
//! This module provides a comprehensive achievement tracking system with support for:
//! - Combat achievements (kills, combos, damage dealt)
//! - Exploration achievements (rooms visited, floors reached, secrets found)
//! - Collection achievements (items found, gold collected, equipment obtained)
//! - Boss achievements (defeating specific bosses, boss rush modes)
//! - Class mastery achievements (winning with each class)
//! - Challenge achievements (speedruns, no-death runs, pacifist runs)

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::classes::CharacterClass;
use crate::items::Rarity;
use crate::entities::EnemyKind;

/// Achievement category for organization and filtering
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum AchievementCategory {
    /// Combat-related achievements (kills, damage, etc.)
    Combat,
    /// Exploration achievements (floors, rooms, secrets)
    Exploration,
    /// Item and collection achievements
    Collection,
    /// Boss-related achievements
    Boss,
    /// Class mastery achievements
    ClassMastery,
    /// Special challenge achievements
    Challenge,
    /// Progression and milestone achievements
    Progression,
    /// Secret/hidden achievements
    Secret,
}

impl AchievementCategory {
    /// Returns the display name of the category
    pub fn name(&self) -> &'static str {
        match self {
            Self::Combat => "Combat",
            Self::Exploration => "Exploration",
            Self::Collection => "Collection",
            Self::Boss => "Boss Slayer",
            Self::ClassMastery => "Class Mastery",
            Self::Challenge => "Challenge",
            Self::Progression => "Progression",
            Self::Secret => "Secret",
        }
    }

    /// Returns the color index for the category (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Combat => 3,      // Red
            Self::Exploration => 7, // Blue
            Self::Collection => 11, // Yellow
            Self::Boss => 13,       // Magenta
            Self::ClassMastery => 9,// Cyan
            Self::Challenge => 5,   // Green
            Self::Progression => 1, // White
            Self::Secret => 14,     // Dark gray
        }
    }
}

/// Represents a reward granted for unlocking an achievement
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AchievementReward {
    /// A title the player can display
    Title(String),
    /// A permanent stat bonus for new characters
    StatBonus { stat: StatType, amount: i32 },
    /// Unlocks a special starting item
    StartingItem(String),
    /// Unlocks a cosmetic option
    Cosmetic(String),
    /// Grants bonus gold for new characters
    StartingGold(u32),
    /// Unlocks a special ability or perk
    Perk(String),
}

/// Types of stats that can be modified by achievement rewards
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum StatType {
    MaxHp,
    MaxMana,
    Attack,
    Defense,
    Speed,
}

impl StatType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::MaxHp => "Max HP",
            Self::MaxMana => "Max Mana",
            Self::Attack => "Attack",
            Self::Defense => "Defense",
            Self::Speed => "Speed",
        }
    }
}

/// Defines a single achievement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievement {
    /// Unique identifier for the achievement
    pub id: AchievementId,
    /// Display name
    pub name: String,
    /// Description of how to unlock
    pub description: String,
    /// Category for organization
    pub category: AchievementCategory,
    /// Whether unlocked
    pub unlocked: bool,
    /// When it was unlocked (turn count when unlocked, 0 if not)
    pub unlocked_at_turn: u32,
    /// Optional reward for unlocking
    pub reward: Option<AchievementReward>,
    /// Whether this achievement is hidden until unlocked
    pub hidden: bool,
    /// Progress toward unlocking (for multi-stage achievements)
    pub progress: u32,
    /// Target progress value (for multi-stage achievements)
    pub target: u32,
    /// Point value for achievement score
    pub points: u32,
}

/// Unique identifier for achievements
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum AchievementId {
    // Combat achievements (1-50)
    FirstBlood,
    Apprentice,        // 10 kills
    Hunter,            // 50 kills
    Warrior,           // 100 kills
    Slayer,            // 500 kills
    Destroyer,         // 1000 kills
    Genocide,          // 5000 kills
    Exterminator,      // 10000 kills
    ComboKill3,        // Kill 3 enemies in one turn
    ComboKill5,        // Kill 5 enemies in one turn
    OneHitKill,        // Kill enemy with one attack
    Untouchable,       // Clear a floor without taking damage
    CriticalMaster,    // Land 100 critical hits
    ElementalSlayer,   // Kill one of each elemental type
    UndeadBane,        // Kill 100 undead enemies
    DemonHunter,       // Kill 100 demons
    DragonSlayer,      // Kill 10 dragon-type enemies

    // Exploration achievements (51-80)
    Explorer,          // Visit 50 rooms
    Adventurer,        // Visit 200 rooms
    Cartographer,      // Visit 1000 rooms
    FloorCleared,      // Clear all enemies on a floor
    Delver,            // Reach floor 10
    DeepDiver,         // Reach floor 20
    AbyssWalker,       // Reach floor 30
    SecretFinder,      // Find 10 secret rooms
    TrapDisarmer,      // Disarm 25 traps
    DoorOpener,        // Open 100 doors
    ShrinePilgrim,     // Use 20 shrines
    ChestHunter,       // Open 50 chests
    MapRevealer,       // Reveal entire floor 10 times

    // Collection achievements (81-120)
    Collector,         // Find 50 unique items
    Hoarder,           // Find 100 unique items
    Completionist,     // Find all unique items
    WealthyAdventurer, // Collect 10000 gold total
    GoldHoarder,       // Collect 100000 gold total
    DragonWealth,      // Collect 1000000 gold total
    PotionMaster,      // Use 100 potions
    ScrollScholar,     // Use 50 scrolls
    GourmetChef,       // Eat one of every food type
    CommonCollector,   // Find 50 common items
    RareCollector,     // Find 25 rare items
    EpicCollector,     // Find 15 epic items
    LegendaryCollector,// Find 5 legendary items
    MythicCollector,   // Find 1 mythic item
    FullyEquipped,     // Equip all slots at once
    LegendaryGear,     // Equip a full set of legendary gear
    RingBearer,        // Equip both ring slots
    WeaponMaster,      // Equip 20 different weapons

    // Boss achievements (121-150)
    GoblinKingSlayer,  // Defeat Goblin King
    OrcWarlordSlayer,  // Defeat Orc Warlord
    VampireLordSlayer, // Defeat Vampire Lord
    ForestGuardianSlayer, // Defeat Forest Guardian
    IceDragonSlayer,   // Defeat Ice Dragon
    DemonKingSlayer,   // Defeat Demon King (final boss)
    BossRush,          // Defeat all bosses in one run
    SpeedBoss,         // Defeat any boss in under 10 turns
    FlawlessBoss,      // Defeat any boss without taking damage
    BossHunter,        // Defeat 10 bosses total
    BossSlayer,        // Defeat 50 bosses total

    // Class mastery achievements (151-170)
    WarriorMaster,     // Beat game as Warrior
    MageMaster,        // Beat game as Mage
    RogueMaster,       // Beat game as Rogue
    PaladinMaster,     // Beat game as Paladin
    RangerMaster,      // Beat game as Ranger
    NecromancerMaster, // Beat game as Necromancer
    JackOfAllTrades,   // Beat game with all classes
    TrueHero,          // Beat game 10 times with any class
    LegendaryHero,     // Beat game 50 times with any class

    // Challenge achievements (171-200)
    Speedrunner,       // Beat game in under 1000 turns
    SpeedDemon,        // Beat game in under 500 turns
    LightningFast,     // Beat game in under 300 turns
    Immortal,          // Beat game without dying (in a session)
    Pacifist,          // Complete floor 1 without killing
    TruePacifist,      // Complete floors 1-5 with minimal kills
    NakedRun,          // Beat a boss with no equipment
    LowLevel,          // Beat the game at level 10 or lower
    SoloDemon,         // Defeat Demon King solo (no minions)
    NoHeal,            // Beat a boss without healing
    PerfectFloor,      // Complete any floor without taking damage

    // Progression achievements (201-220)
    LevelUp,           // Reach level 2
    Veteran,           // Reach level 10
    Champion,          // Reach level 20
    Legendary,         // Reach level 30
    MaxLevel,          // Reach maximum level
    SkillUser,         // Use a skill 10 times
    SkillMaster,       // Use skills 100 times
    FirstDeath,        // Die for the first time
    Persistent,        // Die 10 times
    Determined,        // Die 50 times
    NeverGiveUp,       // Die 100 times

    // Secret achievements (221-240)
    Survivor,          // Survive with 1 HP
    LastStand,         // Win a fight with 1 HP
    GoldRush,          // Find 1000 gold on one floor
    LuckyFind,         // Find a mythic item in first 5 floors
    DoubleKill,        // Kill 2 enemies at exact same time
    OverkillMaster,    // Deal 10x enemy HP in damage
    StatusMaster,      // Apply all status effects in one run
    DoorLord,          // Have 10+ keys at once
    FullBelly,         // Max out hunger stat
    SpellSlinger,      // Cast 50 spells in one run
}

impl AchievementId {
    /// Returns all achievement IDs
    pub fn all() -> Vec<AchievementId> {
        vec![
            // Combat
            Self::FirstBlood, Self::Apprentice, Self::Hunter, Self::Warrior,
            Self::Slayer, Self::Destroyer, Self::Genocide, Self::Exterminator,
            Self::ComboKill3, Self::ComboKill5, Self::OneHitKill, Self::Untouchable,
            Self::CriticalMaster, Self::ElementalSlayer, Self::UndeadBane,
            Self::DemonHunter, Self::DragonSlayer,
            // Exploration
            Self::Explorer, Self::Adventurer, Self::Cartographer, Self::FloorCleared,
            Self::Delver, Self::DeepDiver, Self::AbyssWalker, Self::SecretFinder,
            Self::TrapDisarmer, Self::DoorOpener, Self::ShrinePilgrim, Self::ChestHunter,
            Self::MapRevealer,
            // Collection
            Self::Collector, Self::Hoarder, Self::Completionist, Self::WealthyAdventurer,
            Self::GoldHoarder, Self::DragonWealth, Self::PotionMaster, Self::ScrollScholar,
            Self::GourmetChef, Self::CommonCollector, Self::RareCollector, Self::EpicCollector,
            Self::LegendaryCollector, Self::MythicCollector, Self::FullyEquipped,
            Self::LegendaryGear, Self::RingBearer, Self::WeaponMaster,
            // Boss
            Self::GoblinKingSlayer, Self::OrcWarlordSlayer, Self::VampireLordSlayer,
            Self::ForestGuardianSlayer, Self::IceDragonSlayer, Self::DemonKingSlayer,
            Self::BossRush, Self::SpeedBoss, Self::FlawlessBoss, Self::BossHunter,
            Self::BossSlayer,
            // Class Mastery
            Self::WarriorMaster, Self::MageMaster, Self::RogueMaster, Self::PaladinMaster,
            Self::RangerMaster, Self::NecromancerMaster, Self::JackOfAllTrades,
            Self::TrueHero, Self::LegendaryHero,
            // Challenge
            Self::Speedrunner, Self::SpeedDemon, Self::LightningFast, Self::Immortal,
            Self::Pacifist, Self::TruePacifist, Self::NakedRun, Self::LowLevel,
            Self::SoloDemon, Self::NoHeal, Self::PerfectFloor,
            // Progression
            Self::LevelUp, Self::Veteran, Self::Champion, Self::Legendary, Self::MaxLevel,
            Self::SkillUser, Self::SkillMaster, Self::FirstDeath, Self::Persistent,
            Self::Determined, Self::NeverGiveUp,
            // Secret
            Self::Survivor, Self::LastStand, Self::GoldRush, Self::LuckyFind,
            Self::DoubleKill, Self::OverkillMaster, Self::StatusMaster, Self::DoorLord,
            Self::FullBelly, Self::SpellSlinger,
        ]
    }
}

impl Achievement {
    /// Creates a new achievement definition
    pub fn new(
        id: AchievementId,
        name: &str,
        description: &str,
        category: AchievementCategory,
        target: u32,
        points: u32,
        reward: Option<AchievementReward>,
        hidden: bool,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            category,
            unlocked: false,
            unlocked_at_turn: 0,
            reward,
            hidden,
            progress: 0,
            target,
            points,
        }
    }

    /// Returns all achievement definitions
    pub fn all_definitions() -> Vec<Achievement> {
        vec![
            // Combat achievements
            Achievement::new(
                AchievementId::FirstBlood,
                "First Blood",
                "Kill your first enemy",
                AchievementCategory::Combat,
                1, 5, None, false
            ),
            Achievement::new(
                AchievementId::Apprentice,
                "Apprentice",
                "Kill 10 enemies",
                AchievementCategory::Combat,
                10, 10, None, false
            ),
            Achievement::new(
                AchievementId::Hunter,
                "Hunter",
                "Kill 50 enemies",
                AchievementCategory::Combat,
                50, 15,
                Some(AchievementReward::Title("Hunter".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::Warrior,
                "Warrior",
                "Kill 100 enemies",
                AchievementCategory::Combat,
                100, 25,
                Some(AchievementReward::Title("Warrior".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::Slayer,
                "Slayer",
                "Kill 500 enemies",
                AchievementCategory::Combat,
                500, 50,
                Some(AchievementReward::Title("Slayer".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::Destroyer,
                "Destroyer",
                "Kill 1000 enemies",
                AchievementCategory::Combat,
                1000, 75,
                Some(AchievementReward::StatBonus { stat: StatType::Attack, amount: 2 }),
                false
            ),
            Achievement::new(
                AchievementId::Genocide,
                "Genocide",
                "Kill 5000 enemies",
                AchievementCategory::Combat,
                5000, 100,
                Some(AchievementReward::Title("Genocider".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::Exterminator,
                "Exterminator",
                "Kill 10000 enemies",
                AchievementCategory::Combat,
                10000, 200,
                Some(AchievementReward::StatBonus { stat: StatType::Attack, amount: 5 }),
                true
            ),
            Achievement::new(
                AchievementId::ComboKill3,
                "Triple Kill",
                "Kill 3 enemies in a single turn",
                AchievementCategory::Combat,
                1, 20, None, false
            ),
            Achievement::new(
                AchievementId::ComboKill5,
                "Pentakill",
                "Kill 5 enemies in a single turn",
                AchievementCategory::Combat,
                1, 50,
                Some(AchievementReward::Title("Annihilator".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::OneHitKill,
                "One Punch",
                "Kill an enemy with a single attack",
                AchievementCategory::Combat,
                1, 10, None, false
            ),
            Achievement::new(
                AchievementId::Untouchable,
                "Untouchable",
                "Clear a floor without taking any damage",
                AchievementCategory::Combat,
                1, 75,
                Some(AchievementReward::Title("Ghost".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::CriticalMaster,
                "Critical Master",
                "Land 100 critical hits",
                AchievementCategory::Combat,
                100, 40, None, false
            ),
            Achievement::new(
                AchievementId::ElementalSlayer,
                "Elemental Slayer",
                "Kill one of each elemental enemy type",
                AchievementCategory::Combat,
                4, 35, None, false
            ),
            Achievement::new(
                AchievementId::UndeadBane,
                "Undead Bane",
                "Kill 100 undead enemies",
                AchievementCategory::Combat,
                100, 30,
                Some(AchievementReward::Title("Exorcist".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::DemonHunter,
                "Demon Hunter",
                "Kill 100 demon enemies",
                AchievementCategory::Combat,
                100, 40,
                Some(AchievementReward::Title("Demon Hunter".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::DragonSlayer,
                "Dragon Slayer",
                "Kill 10 dragon-type enemies",
                AchievementCategory::Combat,
                10, 50,
                Some(AchievementReward::Title("Dragon Slayer".to_string())),
                false
            ),

            // Exploration achievements
            Achievement::new(
                AchievementId::Explorer,
                "Explorer",
                "Visit 50 rooms",
                AchievementCategory::Exploration,
                50, 10, None, false
            ),
            Achievement::new(
                AchievementId::Adventurer,
                "Adventurer",
                "Visit 200 rooms",
                AchievementCategory::Exploration,
                200, 25, None, false
            ),
            Achievement::new(
                AchievementId::Cartographer,
                "Cartographer",
                "Visit 1000 rooms",
                AchievementCategory::Exploration,
                1000, 50,
                Some(AchievementReward::Title("Cartographer".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::FloorCleared,
                "Floor Cleared",
                "Clear all enemies on a single floor",
                AchievementCategory::Exploration,
                1, 15, None, false
            ),
            Achievement::new(
                AchievementId::Delver,
                "Delver",
                "Reach dungeon floor 10",
                AchievementCategory::Exploration,
                1, 20, None, false
            ),
            Achievement::new(
                AchievementId::DeepDiver,
                "Deep Diver",
                "Reach dungeon floor 20",
                AchievementCategory::Exploration,
                1, 35, None, false
            ),
            Achievement::new(
                AchievementId::AbyssWalker,
                "Abyss Walker",
                "Reach dungeon floor 30",
                AchievementCategory::Exploration,
                1, 50,
                Some(AchievementReward::Title("Abyss Walker".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::SecretFinder,
                "Secret Finder",
                "Discover 10 secret rooms",
                AchievementCategory::Exploration,
                10, 30, None, false
            ),
            Achievement::new(
                AchievementId::TrapDisarmer,
                "Trap Disarmer",
                "Trigger (and survive) 25 traps",
                AchievementCategory::Exploration,
                25, 20, None, false
            ),
            Achievement::new(
                AchievementId::DoorOpener,
                "Door Opener",
                "Open 100 doors",
                AchievementCategory::Exploration,
                100, 15, None, false
            ),
            Achievement::new(
                AchievementId::ShrinePilgrim,
                "Shrine Pilgrim",
                "Use 20 shrines",
                AchievementCategory::Exploration,
                20, 25, None, false
            ),
            Achievement::new(
                AchievementId::ChestHunter,
                "Chest Hunter",
                "Open 50 chests",
                AchievementCategory::Exploration,
                50, 20, None, false
            ),
            Achievement::new(
                AchievementId::MapRevealer,
                "Map Revealer",
                "Fully reveal 10 floors",
                AchievementCategory::Exploration,
                10, 25, None, false
            ),

            // Collection achievements
            Achievement::new(
                AchievementId::Collector,
                "Collector",
                "Find 50 unique items",
                AchievementCategory::Collection,
                50, 25, None, false
            ),
            Achievement::new(
                AchievementId::Hoarder,
                "Hoarder",
                "Find 100 unique items",
                AchievementCategory::Collection,
                100, 50,
                Some(AchievementReward::Title("Hoarder".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::Completionist,
                "Completionist",
                "Find every unique item in the game",
                AchievementCategory::Collection,
                150, 200,
                Some(AchievementReward::Title("Completionist".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::WealthyAdventurer,
                "Wealthy Adventurer",
                "Collect 10,000 gold total",
                AchievementCategory::Collection,
                10000, 20, None, false
            ),
            Achievement::new(
                AchievementId::GoldHoarder,
                "Gold Hoarder",
                "Collect 100,000 gold total",
                AchievementCategory::Collection,
                100000, 50,
                Some(AchievementReward::StartingGold(100)),
                false
            ),
            Achievement::new(
                AchievementId::DragonWealth,
                "Dragon's Wealth",
                "Collect 1,000,000 gold total",
                AchievementCategory::Collection,
                1000000, 100,
                Some(AchievementReward::StartingGold(500)),
                true
            ),
            Achievement::new(
                AchievementId::PotionMaster,
                "Potion Master",
                "Use 100 potions",
                AchievementCategory::Collection,
                100, 25, None, false
            ),
            Achievement::new(
                AchievementId::ScrollScholar,
                "Scroll Scholar",
                "Use 50 scrolls",
                AchievementCategory::Collection,
                50, 25, None, false
            ),
            Achievement::new(
                AchievementId::GourmetChef,
                "Gourmet",
                "Eat every type of food",
                AchievementCategory::Collection,
                8, 20, None, false
            ),
            Achievement::new(
                AchievementId::CommonCollector,
                "Common Collector",
                "Find 50 common items",
                AchievementCategory::Collection,
                50, 10, None, false
            ),
            Achievement::new(
                AchievementId::RareCollector,
                "Rare Collector",
                "Find 25 rare items",
                AchievementCategory::Collection,
                25, 20, None, false
            ),
            Achievement::new(
                AchievementId::EpicCollector,
                "Epic Collector",
                "Find 15 epic items",
                AchievementCategory::Collection,
                15, 35, None, false
            ),
            Achievement::new(
                AchievementId::LegendaryCollector,
                "Legendary Collector",
                "Find 5 legendary items",
                AchievementCategory::Collection,
                5, 50, None, false
            ),
            Achievement::new(
                AchievementId::MythicCollector,
                "Mythic Discovery",
                "Find a mythic item",
                AchievementCategory::Collection,
                1, 100,
                Some(AchievementReward::Title("Mythic Hunter".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::FullyEquipped,
                "Fully Equipped",
                "Equip items in all equipment slots",
                AchievementCategory::Collection,
                1, 25, None, false
            ),
            Achievement::new(
                AchievementId::LegendaryGear,
                "Legendary Gear",
                "Equip legendary items in 5+ slots",
                AchievementCategory::Collection,
                1, 75,
                Some(AchievementReward::Title("Legend".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::RingBearer,
                "Ring Bearer",
                "Equip rings in both ring slots",
                AchievementCategory::Collection,
                1, 15, None, false
            ),
            Achievement::new(
                AchievementId::WeaponMaster,
                "Weapon Master",
                "Equip 20 different weapons throughout your adventures",
                AchievementCategory::Collection,
                20, 30, None, false
            ),

            // Boss achievements
            Achievement::new(
                AchievementId::GoblinKingSlayer,
                "Kingslayer",
                "Defeat the Goblin King",
                AchievementCategory::Boss,
                1, 25, None, false
            ),
            Achievement::new(
                AchievementId::OrcWarlordSlayer,
                "Warlord's Bane",
                "Defeat the Orc Warlord",
                AchievementCategory::Boss,
                1, 35, None, false
            ),
            Achievement::new(
                AchievementId::VampireLordSlayer,
                "Vampire Hunter",
                "Defeat the Vampire Lord",
                AchievementCategory::Boss,
                1, 45, None, false
            ),
            Achievement::new(
                AchievementId::ForestGuardianSlayer,
                "Nature's Nemesis",
                "Defeat the Forest Guardian",
                AchievementCategory::Boss,
                1, 55, None, false
            ),
            Achievement::new(
                AchievementId::IceDragonSlayer,
                "Ice Breaker",
                "Defeat the Ice Dragon",
                AchievementCategory::Boss,
                1, 75,
                Some(AchievementReward::Title("Dragonslayer".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::DemonKingSlayer,
                "Demon Vanquisher",
                "Defeat the Demon King",
                AchievementCategory::Boss,
                1, 150,
                Some(AchievementReward::Title("Demon Vanquisher".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::BossRush,
                "Boss Rush",
                "Defeat all bosses in a single run",
                AchievementCategory::Boss,
                1, 100,
                Some(AchievementReward::Title("Boss Slayer".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::SpeedBoss,
                "Speed Boss",
                "Defeat any boss in under 10 turns of combat",
                AchievementCategory::Boss,
                1, 50, None, false
            ),
            Achievement::new(
                AchievementId::FlawlessBoss,
                "Flawless Victory",
                "Defeat any boss without taking damage",
                AchievementCategory::Boss,
                1, 75,
                Some(AchievementReward::Title("Flawless".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::BossHunter,
                "Boss Hunter",
                "Defeat 10 bosses total",
                AchievementCategory::Boss,
                10, 40, None, false
            ),
            Achievement::new(
                AchievementId::BossSlayer,
                "Boss Slayer",
                "Defeat 50 bosses total",
                AchievementCategory::Boss,
                50, 100,
                Some(AchievementReward::StatBonus { stat: StatType::Attack, amount: 3 }),
                false
            ),

            // Class mastery achievements
            Achievement::new(
                AchievementId::WarriorMaster,
                "Warrior Master",
                "Complete the game as a Warrior",
                AchievementCategory::ClassMastery,
                1, 50, None, false
            ),
            Achievement::new(
                AchievementId::MageMaster,
                "Mage Master",
                "Complete the game as a Mage",
                AchievementCategory::ClassMastery,
                1, 50, None, false
            ),
            Achievement::new(
                AchievementId::RogueMaster,
                "Rogue Master",
                "Complete the game as a Rogue",
                AchievementCategory::ClassMastery,
                1, 50, None, false
            ),
            Achievement::new(
                AchievementId::PaladinMaster,
                "Paladin Master",
                "Complete the game as a Paladin",
                AchievementCategory::ClassMastery,
                1, 50, None, false
            ),
            Achievement::new(
                AchievementId::RangerMaster,
                "Ranger Master",
                "Complete the game as a Ranger",
                AchievementCategory::ClassMastery,
                1, 50, None, false
            ),
            Achievement::new(
                AchievementId::NecromancerMaster,
                "Necromancer Master",
                "Complete the game as a Necromancer",
                AchievementCategory::ClassMastery,
                1, 50, None, false
            ),
            Achievement::new(
                AchievementId::JackOfAllTrades,
                "Jack of All Trades",
                "Complete the game with all 6 classes",
                AchievementCategory::ClassMastery,
                6, 200,
                Some(AchievementReward::Perk("Bonus starting stats for all classes".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::TrueHero,
                "True Hero",
                "Complete the game 10 times",
                AchievementCategory::ClassMastery,
                10, 100,
                Some(AchievementReward::Title("True Hero".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::LegendaryHero,
                "Legendary Hero",
                "Complete the game 50 times",
                AchievementCategory::ClassMastery,
                50, 300,
                Some(AchievementReward::Title("Legendary Hero".to_string())),
                true
            ),

            // Challenge achievements
            Achievement::new(
                AchievementId::Speedrunner,
                "Speedrunner",
                "Complete the game in under 1000 turns",
                AchievementCategory::Challenge,
                1, 75,
                Some(AchievementReward::Title("Speedrunner".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::SpeedDemon,
                "Speed Demon",
                "Complete the game in under 500 turns",
                AchievementCategory::Challenge,
                1, 150,
                Some(AchievementReward::Title("Speed Demon".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::LightningFast,
                "Lightning Fast",
                "Complete the game in under 300 turns",
                AchievementCategory::Challenge,
                1, 300,
                Some(AchievementReward::Title("Lightning".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::Immortal,
                "Immortal",
                "Complete the game without dying",
                AchievementCategory::Challenge,
                1, 200,
                Some(AchievementReward::Title("Immortal".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::Pacifist,
                "Pacifist",
                "Complete floor 1 without killing any enemies",
                AchievementCategory::Challenge,
                1, 50,
                Some(AchievementReward::Title("Pacifist".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::TruePacifist,
                "True Pacifist",
                "Complete floors 1-5 with minimal kills",
                AchievementCategory::Challenge,
                1, 100,
                Some(AchievementReward::Title("True Pacifist".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::NakedRun,
                "Naked Run",
                "Defeat any boss without any equipment",
                AchievementCategory::Challenge,
                1, 100, None, true
            ),
            Achievement::new(
                AchievementId::LowLevel,
                "Low Level",
                "Complete the game at level 10 or lower",
                AchievementCategory::Challenge,
                1, 150,
                Some(AchievementReward::Title("Underdog".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::SoloDemon,
                "Solo Demon",
                "Defeat the Demon King without any minions",
                AchievementCategory::Challenge,
                1, 75, None, false
            ),
            Achievement::new(
                AchievementId::NoHeal,
                "No Heal",
                "Defeat any boss without using healing",
                AchievementCategory::Challenge,
                1, 60, None, false
            ),
            Achievement::new(
                AchievementId::PerfectFloor,
                "Perfect Floor",
                "Complete any floor without taking damage",
                AchievementCategory::Challenge,
                1, 50, None, false
            ),

            // Progression achievements
            Achievement::new(
                AchievementId::LevelUp,
                "Level Up!",
                "Reach level 2",
                AchievementCategory::Progression,
                1, 5, None, false
            ),
            Achievement::new(
                AchievementId::Veteran,
                "Veteran",
                "Reach level 10",
                AchievementCategory::Progression,
                1, 20, None, false
            ),
            Achievement::new(
                AchievementId::Champion,
                "Champion",
                "Reach level 20",
                AchievementCategory::Progression,
                1, 40, None, false
            ),
            Achievement::new(
                AchievementId::Legendary,
                "Legendary",
                "Reach level 30",
                AchievementCategory::Progression,
                1, 75,
                Some(AchievementReward::Title("Legendary".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::MaxLevel,
                "Maximum Power",
                "Reach maximum level",
                AchievementCategory::Progression,
                1, 100,
                Some(AchievementReward::Title("Maxed".to_string())),
                false
            ),
            Achievement::new(
                AchievementId::SkillUser,
                "Skill User",
                "Use skills 10 times",
                AchievementCategory::Progression,
                10, 10, None, false
            ),
            Achievement::new(
                AchievementId::SkillMaster,
                "Skill Master",
                "Use skills 100 times",
                AchievementCategory::Progression,
                100, 30, None, false
            ),
            Achievement::new(
                AchievementId::FirstDeath,
                "First Death",
                "Die for the first time",
                AchievementCategory::Progression,
                1, 5, None, false
            ),
            Achievement::new(
                AchievementId::Persistent,
                "Persistent",
                "Die 10 times",
                AchievementCategory::Progression,
                10, 15, None, false
            ),
            Achievement::new(
                AchievementId::Determined,
                "Determined",
                "Die 50 times",
                AchievementCategory::Progression,
                50, 30, None, false
            ),
            Achievement::new(
                AchievementId::NeverGiveUp,
                "Never Give Up",
                "Die 100 times",
                AchievementCategory::Progression,
                100, 50,
                Some(AchievementReward::Title("Undying".to_string())),
                true
            ),

            // Secret achievements
            Achievement::new(
                AchievementId::Survivor,
                "Survivor",
                "Survive with exactly 1 HP",
                AchievementCategory::Secret,
                1, 25, None, true
            ),
            Achievement::new(
                AchievementId::LastStand,
                "Last Stand",
                "Win a fight while at 1 HP",
                AchievementCategory::Secret,
                1, 50,
                Some(AchievementReward::Title("Last Stand".to_string())),
                true
            ),
            Achievement::new(
                AchievementId::GoldRush,
                "Gold Rush",
                "Find 1000+ gold on a single floor",
                AchievementCategory::Secret,
                1, 30, None, true
            ),
            Achievement::new(
                AchievementId::LuckyFind,
                "Lucky Find",
                "Find a mythic item in the first 5 floors",
                AchievementCategory::Secret,
                1, 100, None, true
            ),
            Achievement::new(
                AchievementId::DoubleKill,
                "Double Kill",
                "Kill 2 enemies at the exact same time",
                AchievementCategory::Secret,
                1, 25, None, true
            ),
            Achievement::new(
                AchievementId::OverkillMaster,
                "Overkill",
                "Deal 10x an enemy's max HP in a single hit",
                AchievementCategory::Secret,
                1, 40, None, true
            ),
            Achievement::new(
                AchievementId::StatusMaster,
                "Status Master",
                "Apply all status effects in a single run",
                AchievementCategory::Secret,
                1, 50, None, true
            ),
            Achievement::new(
                AchievementId::DoorLord,
                "Door Lord",
                "Have 10 or more keys at once",
                AchievementCategory::Secret,
                1, 30, None, true
            ),
            Achievement::new(
                AchievementId::FullBelly,
                "Full Belly",
                "Reach maximum hunger level",
                AchievementCategory::Secret,
                1, 15, None, true
            ),
            Achievement::new(
                AchievementId::SpellSlinger,
                "Spell Slinger",
                "Cast 50 spells in a single run",
                AchievementCategory::Secret,
                50, 35, None, true
            ),
        ]
    }
}

/// Persistent statistics tracked across all games
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AchievementStats {
    // Combat stats
    pub total_kills: u32,
    pub kills_by_type: HashMap<String, u32>,
    pub undead_kills: u32,
    pub demon_kills: u32,
    pub dragon_kills: u32,
    pub elemental_types_killed: HashSet<String>,
    pub max_combo_kills: u32,
    pub critical_hits: u32,
    pub one_hit_kills: u32,
    pub total_damage_dealt: u64,
    pub total_damage_taken: u64,

    // Exploration stats
    pub rooms_visited: u32,
    pub floors_fully_revealed: u32,
    pub max_floor_reached: u32,
    pub secret_rooms_found: u32,
    pub traps_triggered: u32,
    pub doors_opened: u32,
    pub shrines_used: u32,
    pub chests_opened: u32,
    pub floors_cleared: u32,

    // Collection stats
    pub unique_items_found: HashSet<String>,
    pub total_gold_collected: u64,
    pub potions_used: u32,
    pub scrolls_used: u32,
    pub food_types_eaten: HashSet<String>,
    pub weapons_equipped: HashSet<String>,
    pub items_by_rarity: HashMap<String, u32>,

    // Boss stats
    pub bosses_defeated: HashSet<String>,
    pub total_boss_kills: u32,
    pub fastest_boss_kill_turns: u32,
    pub flawless_boss_kills: u32,
    pub boss_rush_completed: bool,

    // Class stats
    pub games_won_by_class: HashMap<String, u32>,
    pub total_games_won: u32,
    pub total_games_played: u32,

    // Death and progression stats
    pub total_deaths: u32,
    pub max_player_level: u32,
    pub total_skills_used: u32,
    pub fastest_win_turns: u32,
    pub deathless_wins: u32,
}

/// Current run statistics (reset each game)
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RunStats {
    pub kills_this_run: u32,
    pub kills_this_floor: u32,
    pub kills_this_turn: u32,
    pub damage_taken_this_floor: u32,
    pub damage_taken_this_turn: u32,
    pub gold_this_floor: u32,
    pub skills_used_this_run: u32,
    pub heals_since_boss_fight: u32,
    pub turns_in_boss_fight: u32,
    pub boss_fight_damage_taken: u32,
    pub died_this_session: bool,
    pub bosses_killed_this_run: HashSet<String>,
    pub status_effects_applied: HashSet<String>,
    pub floor1_kills: u32,
    pub floors_5_kills: u32,
}

/// The main achievement tracker
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTracker {
    /// All achievement definitions and their unlock status
    pub achievements: HashMap<AchievementId, Achievement>,
    /// Persistent statistics across all games
    pub stats: AchievementStats,
    /// Current run statistics
    pub run_stats: RunStats,
    /// Notifications for recently unlocked achievements
    pub pending_notifications: Vec<String>,
    /// Total achievement score
    pub total_score: u32,
}

impl Default for AchievementTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl AchievementTracker {
    /// Creates a new achievement tracker with all achievements
    pub fn new() -> Self {
        let mut achievements = HashMap::new();
        for achievement in Achievement::all_definitions() {
            achievements.insert(achievement.id, achievement);
        }

        Self {
            achievements,
            stats: AchievementStats::default(),
            run_stats: RunStats::default(),
            pending_notifications: Vec::new(),
            total_score: 0,
        }
    }

    /// Resets run-specific stats for a new game
    pub fn reset_run(&mut self) {
        self.run_stats = RunStats::default();
    }

    /// Resets stats for a new floor
    pub fn reset_floor(&mut self) {
        self.run_stats.kills_this_floor = 0;
        self.run_stats.damage_taken_this_floor = 0;
        self.run_stats.gold_this_floor = 0;
    }

    /// Resets stats for a new turn
    pub fn reset_turn(&mut self) {
        self.run_stats.kills_this_turn = 0;
        self.run_stats.damage_taken_this_turn = 0;
    }

    /// Unlocks an achievement by ID
    pub fn unlock(&mut self, id: AchievementId, turn: u32) -> bool {
        if let Some(achievement) = self.achievements.get_mut(&id) {
            if !achievement.unlocked {
                achievement.unlocked = true;
                achievement.unlocked_at_turn = turn;
                self.total_score += achievement.points;

                let notification = if let Some(ref reward) = achievement.reward {
                    match reward {
                        AchievementReward::Title(title) => {
                            format!("Achievement Unlocked: {}! Earned title: {}", achievement.name, title)
                        }
                        AchievementReward::StatBonus { stat, amount } => {
                            format!("Achievement Unlocked: {}! Earned +{} {}", achievement.name, amount, stat.name())
                        }
                        AchievementReward::StartingGold(amount) => {
                            format!("Achievement Unlocked: {}! Earned +{} starting gold", achievement.name, amount)
                        }
                        _ => format!("Achievement Unlocked: {}!", achievement.name),
                    }
                } else {
                    format!("Achievement Unlocked: {}!", achievement.name)
                };

                self.pending_notifications.push(notification);
                return true;
            }
        }
        false
    }

    /// Updates achievement progress without unlocking
    pub fn update_progress(&mut self, id: AchievementId, progress: u32, turn: u32) {
        if let Some(achievement) = self.achievements.get_mut(&id) {
            if !achievement.unlocked {
                achievement.progress = progress;
                if achievement.progress >= achievement.target {
                    self.unlock(id, turn);
                }
            }
        }
    }

    /// Checks if an achievement is unlocked
    pub fn is_unlocked(&self, id: AchievementId) -> bool {
        self.achievements.get(&id).map_or(false, |a| a.unlocked)
    }

    /// Gets a pending notification, if any
    pub fn pop_notification(&mut self) -> Option<String> {
        if self.pending_notifications.is_empty() {
            None
        } else {
            Some(self.pending_notifications.remove(0))
        }
    }

    /// Records a kill and checks related achievements
    pub fn record_kill(&mut self, enemy_kind: EnemyKind, one_hit: bool, turn: u32) {
        self.stats.total_kills += 1;
        self.run_stats.kills_this_run += 1;
        self.run_stats.kills_this_floor += 1;
        self.run_stats.kills_this_turn += 1;

        // Track kills by type
        let name = enemy_kind.name().to_string();
        *self.stats.kills_by_type.entry(name.clone()).or_insert(0) += 1;

        // Track undead kills
        if enemy_kind.is_undead() {
            self.stats.undead_kills += 1;
            self.update_progress(AchievementId::UndeadBane, self.stats.undead_kills, turn);
        }

        // Track demon kills
        if matches!(enemy_kind,
            EnemyKind::Demon | EnemyKind::DemonLord | EnemyKind::Succubus |
            EnemyKind::Balrog | EnemyKind::PitFiend | EnemyKind::ShadowDemon |
            EnemyKind::AbyssalHorror | EnemyKind::DoomGuard | EnemyKind::BossDemonKing |
            EnemyKind::InfernalImp | EnemyKind::InfernalLord
        ) {
            self.stats.demon_kills += 1;
            self.update_progress(AchievementId::DemonHunter, self.stats.demon_kills, turn);
        }

        // Track dragon kills
        if matches!(enemy_kind,
            EnemyKind::FireDrake | EnemyKind::BossIceDragon | EnemyKind::AncientWyrm
        ) {
            self.stats.dragon_kills += 1;
            self.update_progress(AchievementId::DragonSlayer, self.stats.dragon_kills, turn);
        }

        // Track elemental kills
        if matches!(enemy_kind,
            EnemyKind::FireElemental | EnemyKind::IceElemental |
            EnemyKind::RockElemental | EnemyKind::LavaGolem
        ) {
            self.stats.elemental_types_killed.insert(name);
            let count = self.stats.elemental_types_killed.len() as u32;
            self.update_progress(AchievementId::ElementalSlayer, count, turn);
        }

        // One-hit kill tracking
        if one_hit {
            self.stats.one_hit_kills += 1;
            if !self.is_unlocked(AchievementId::OneHitKill) {
                self.unlock(AchievementId::OneHitKill, turn);
            }
        }

        // Floor 1 kill tracking for pacifist
        if self.stats.max_floor_reached <= 1 {
            self.run_stats.floor1_kills += 1;
        }
        if self.stats.max_floor_reached <= 5 {
            self.run_stats.floors_5_kills += 1;
        }

        // Check kill count achievements
        self.update_progress(AchievementId::FirstBlood, self.stats.total_kills, turn);
        self.update_progress(AchievementId::Apprentice, self.stats.total_kills, turn);
        self.update_progress(AchievementId::Hunter, self.stats.total_kills, turn);
        self.update_progress(AchievementId::Warrior, self.stats.total_kills, turn);
        self.update_progress(AchievementId::Slayer, self.stats.total_kills, turn);
        self.update_progress(AchievementId::Destroyer, self.stats.total_kills, turn);
        self.update_progress(AchievementId::Genocide, self.stats.total_kills, turn);
        self.update_progress(AchievementId::Exterminator, self.stats.total_kills, turn);
    }

    /// Checks combo kill achievements at end of turn
    pub fn check_combo_kills(&mut self, turn: u32) {
        let combo = self.run_stats.kills_this_turn;
        if combo > self.stats.max_combo_kills {
            self.stats.max_combo_kills = combo;
        }

        if combo >= 2 && !self.is_unlocked(AchievementId::DoubleKill) {
            self.unlock(AchievementId::DoubleKill, turn);
        }
        if combo >= 3 {
            self.update_progress(AchievementId::ComboKill3, 1, turn);
        }
        if combo >= 5 {
            self.update_progress(AchievementId::ComboKill5, 1, turn);
        }
    }

    /// Records boss defeat
    pub fn record_boss_defeat(&mut self, boss_kind: EnemyKind, turns_taken: u32, damage_taken: u32, turn: u32) {
        let boss_name = boss_kind.name().to_string();

        self.stats.bosses_defeated.insert(boss_name.clone());
        self.stats.total_boss_kills += 1;
        self.run_stats.bosses_killed_this_run.insert(boss_name.clone());

        // Track fastest boss kill
        if self.stats.fastest_boss_kill_turns == 0 || turns_taken < self.stats.fastest_boss_kill_turns {
            self.stats.fastest_boss_kill_turns = turns_taken;
        }

        // Flawless boss kill
        if damage_taken == 0 {
            self.stats.flawless_boss_kills += 1;
            self.unlock(AchievementId::FlawlessBoss, turn);
        }

        // Speed boss kill
        if turns_taken <= 10 {
            self.unlock(AchievementId::SpeedBoss, turn);
        }

        // Specific boss achievements
        match boss_kind {
            EnemyKind::BossGoblinKing => self.unlock(AchievementId::GoblinKingSlayer, turn),
            EnemyKind::BossOrcWarlord => self.unlock(AchievementId::OrcWarlordSlayer, turn),
            EnemyKind::BossVampireLord => self.unlock(AchievementId::VampireLordSlayer, turn),
            EnemyKind::BossForestGuardian => self.unlock(AchievementId::ForestGuardianSlayer, turn),
            EnemyKind::BossIceDragon => self.unlock(AchievementId::IceDragonSlayer, turn),
            EnemyKind::BossDemonKing => self.unlock(AchievementId::DemonKingSlayer, turn),
            _ => false,
        };

        // Boss hunter achievements
        self.update_progress(AchievementId::BossHunter, self.stats.total_boss_kills, turn);
        self.update_progress(AchievementId::BossSlayer, self.stats.total_boss_kills, turn);

        // Check boss rush (all bosses in one run)
        if self.run_stats.bosses_killed_this_run.len() >= 6 {
            self.unlock(AchievementId::BossRush, turn);
        }
    }

    /// Records floor reached
    pub fn record_floor_reached(&mut self, floor: u32, turn: u32) {
        if floor > self.stats.max_floor_reached {
            self.stats.max_floor_reached = floor;
        }

        // Check floor achievements
        if floor >= 10 {
            self.unlock(AchievementId::Delver, turn);
        }
        if floor >= 20 {
            self.unlock(AchievementId::DeepDiver, turn);
        }
        if floor >= 30 {
            self.unlock(AchievementId::AbyssWalker, turn);
        }

        // Reset floor stats
        self.reset_floor();

        // Check pacifist on floor 2+
        if floor == 2 && self.run_stats.floor1_kills == 0 {
            self.unlock(AchievementId::Pacifist, turn);
        }
        if floor == 6 && self.run_stats.floors_5_kills <= 5 {
            self.unlock(AchievementId::TruePacifist, turn);
        }
    }

    /// Records floor cleared (all enemies dead)
    pub fn record_floor_cleared(&mut self, turn: u32) {
        self.stats.floors_cleared += 1;
        self.unlock(AchievementId::FloorCleared, turn);

        // Check untouchable (no damage taken on floor)
        if self.run_stats.damage_taken_this_floor == 0 {
            self.unlock(AchievementId::Untouchable, turn);
            self.unlock(AchievementId::PerfectFloor, turn);
        }
    }

    /// Records room visit
    pub fn record_room_visit(&mut self, turn: u32) {
        self.stats.rooms_visited += 1;
        self.update_progress(AchievementId::Explorer, self.stats.rooms_visited, turn);
        self.update_progress(AchievementId::Adventurer, self.stats.rooms_visited, turn);
        self.update_progress(AchievementId::Cartographer, self.stats.rooms_visited, turn);
    }

    /// Records damage taken
    pub fn record_damage_taken(&mut self, amount: u32) {
        self.stats.total_damage_taken += amount as u64;
        self.run_stats.damage_taken_this_floor += amount;
        self.run_stats.damage_taken_this_turn += amount;
        self.run_stats.boss_fight_damage_taken += amount;
    }

    /// Records damage dealt
    pub fn record_damage_dealt(&mut self, amount: u32, enemy_max_hp: u32, turn: u32) {
        self.stats.total_damage_dealt += amount as u64;

        // Check overkill
        if amount >= enemy_max_hp * 10 {
            self.unlock(AchievementId::OverkillMaster, turn);
        }
    }

    /// Records item found
    pub fn record_item_found(&mut self, item_name: &str, rarity: Rarity, floor: u32, turn: u32) {
        self.stats.unique_items_found.insert(item_name.to_string());

        let rarity_name = format!("{:?}", rarity);
        *self.stats.items_by_rarity.entry(rarity_name.clone()).or_insert(0) += 1;

        // Collection achievements
        let unique_count = self.stats.unique_items_found.len() as u32;
        self.update_progress(AchievementId::Collector, unique_count, turn);
        self.update_progress(AchievementId::Hoarder, unique_count, turn);
        self.update_progress(AchievementId::Completionist, unique_count, turn);

        // Rarity-specific achievements
        match rarity {
            Rarity::Common => {
                let count = *self.stats.items_by_rarity.get("Common").unwrap_or(&0);
                self.update_progress(AchievementId::CommonCollector, count, turn);
            }
            Rarity::Rare => {
                let count = *self.stats.items_by_rarity.get("Rare").unwrap_or(&0);
                self.update_progress(AchievementId::RareCollector, count, turn);
            }
            Rarity::Epic => {
                let count = *self.stats.items_by_rarity.get("Epic").unwrap_or(&0);
                self.update_progress(AchievementId::EpicCollector, count, turn);
            }
            Rarity::Legendary => {
                let count = *self.stats.items_by_rarity.get("Legendary").unwrap_or(&0);
                self.update_progress(AchievementId::LegendaryCollector, count, turn);
            }
            Rarity::Mythic => {
                self.unlock(AchievementId::MythicCollector, turn);
                if floor <= 5 {
                    self.unlock(AchievementId::LuckyFind, turn);
                }
            }
            _ => {}
        }
    }

    /// Records gold collected
    pub fn record_gold_collected(&mut self, amount: u32, turn: u32) {
        self.stats.total_gold_collected += amount as u64;
        self.run_stats.gold_this_floor += amount;

        self.update_progress(AchievementId::WealthyAdventurer, self.stats.total_gold_collected as u32, turn);

        if self.stats.total_gold_collected >= 100000 {
            self.unlock(AchievementId::GoldHoarder, turn);
        }
        if self.stats.total_gold_collected >= 1000000 {
            self.unlock(AchievementId::DragonWealth, turn);
        }

        // Gold rush (1000 gold on one floor)
        if self.run_stats.gold_this_floor >= 1000 {
            self.unlock(AchievementId::GoldRush, turn);
        }
    }

    /// Records potion use
    pub fn record_potion_used(&mut self, turn: u32) {
        self.stats.potions_used += 1;
        self.update_progress(AchievementId::PotionMaster, self.stats.potions_used, turn);
    }

    /// Records scroll use
    pub fn record_scroll_used(&mut self, turn: u32) {
        self.stats.scrolls_used += 1;
        self.update_progress(AchievementId::ScrollScholar, self.stats.scrolls_used, turn);
    }

    /// Records food eaten
    pub fn record_food_eaten(&mut self, food_name: &str, turn: u32) {
        self.stats.food_types_eaten.insert(food_name.to_string());
        let count = self.stats.food_types_eaten.len() as u32;
        self.update_progress(AchievementId::GourmetChef, count, turn);
    }

    /// Records weapon equipped
    pub fn record_weapon_equipped(&mut self, weapon_name: &str, turn: u32) {
        self.stats.weapons_equipped.insert(weapon_name.to_string());
        let count = self.stats.weapons_equipped.len() as u32;
        self.update_progress(AchievementId::WeaponMaster, count, turn);
    }

    /// Records door opened
    pub fn record_door_opened(&mut self, turn: u32) {
        self.stats.doors_opened += 1;
        self.update_progress(AchievementId::DoorOpener, self.stats.doors_opened, turn);
    }

    /// Records chest opened
    pub fn record_chest_opened(&mut self, turn: u32) {
        self.stats.chests_opened += 1;
        self.update_progress(AchievementId::ChestHunter, self.stats.chests_opened, turn);
    }

    /// Records shrine used
    pub fn record_shrine_used(&mut self, turn: u32) {
        self.stats.shrines_used += 1;
        self.update_progress(AchievementId::ShrinePilgrim, self.stats.shrines_used, turn);
    }

    /// Records trap triggered
    pub fn record_trap_triggered(&mut self, turn: u32) {
        self.stats.traps_triggered += 1;
        self.update_progress(AchievementId::TrapDisarmer, self.stats.traps_triggered, turn);
    }

    /// Records skill use
    pub fn record_skill_used(&mut self, turn: u32) {
        self.stats.total_skills_used += 1;
        self.run_stats.skills_used_this_run += 1;

        self.update_progress(AchievementId::SkillUser, self.stats.total_skills_used, turn);
        self.update_progress(AchievementId::SkillMaster, self.stats.total_skills_used, turn);
        self.update_progress(AchievementId::SpellSlinger, self.run_stats.skills_used_this_run, turn);
    }

    /// Records player level up
    pub fn record_level_up(&mut self, new_level: u32, turn: u32) {
        if new_level > self.stats.max_player_level {
            self.stats.max_player_level = new_level;
        }

        if new_level >= 2 {
            self.unlock(AchievementId::LevelUp, turn);
        }
        if new_level >= 10 {
            self.unlock(AchievementId::Veteran, turn);
        }
        if new_level >= 20 {
            self.unlock(AchievementId::Champion, turn);
        }
        if new_level >= 30 {
            self.unlock(AchievementId::Legendary, turn);
        }
        if new_level >= 50 {
            self.unlock(AchievementId::MaxLevel, turn);
        }
    }

    /// Records player death
    pub fn record_death(&mut self, turn: u32) {
        self.stats.total_deaths += 1;
        self.run_stats.died_this_session = true;

        self.update_progress(AchievementId::FirstDeath, self.stats.total_deaths, turn);
        self.update_progress(AchievementId::Persistent, self.stats.total_deaths, turn);
        self.update_progress(AchievementId::Determined, self.stats.total_deaths, turn);
        self.update_progress(AchievementId::NeverGiveUp, self.stats.total_deaths, turn);
    }

    /// Records game victory
    pub fn record_victory(&mut self, class: CharacterClass, turns: u32, player_level: u32, turn: u32) {
        self.stats.total_games_won += 1;
        let class_name = class.name().to_string();
        *self.stats.games_won_by_class.entry(class_name.clone()).or_insert(0) += 1;

        // Track fastest win
        if self.stats.fastest_win_turns == 0 || turns < self.stats.fastest_win_turns {
            self.stats.fastest_win_turns = turns;
        }

        // Deathless win
        if !self.run_stats.died_this_session {
            self.stats.deathless_wins += 1;
            self.unlock(AchievementId::Immortal, turn);
        }

        // Class mastery achievements
        match class {
            CharacterClass::Warrior => self.unlock(AchievementId::WarriorMaster, turn),
            CharacterClass::Mage => self.unlock(AchievementId::MageMaster, turn),
            CharacterClass::Rogue => self.unlock(AchievementId::RogueMaster, turn),
            CharacterClass::Paladin => self.unlock(AchievementId::PaladinMaster, turn),
            CharacterClass::Ranger => self.unlock(AchievementId::RangerMaster, turn),
            CharacterClass::Necromancer => self.unlock(AchievementId::NecromancerMaster, turn),
        };

        // Jack of all trades
        if self.stats.games_won_by_class.len() >= 6 {
            self.unlock(AchievementId::JackOfAllTrades, turn);
        }

        // Victory count achievements
        self.update_progress(AchievementId::TrueHero, self.stats.total_games_won, turn);
        self.update_progress(AchievementId::LegendaryHero, self.stats.total_games_won, turn);

        // Speed run achievements
        if turns < 1000 {
            self.unlock(AchievementId::Speedrunner, turn);
        }
        if turns < 500 {
            self.unlock(AchievementId::SpeedDemon, turn);
        }
        if turns < 300 {
            self.unlock(AchievementId::LightningFast, turn);
        }

        // Low level achievement
        if player_level <= 10 {
            self.unlock(AchievementId::LowLevel, turn);
        }
    }

    /// Records critical hit
    pub fn record_critical_hit(&mut self, turn: u32) {
        self.stats.critical_hits += 1;
        self.update_progress(AchievementId::CriticalMaster, self.stats.critical_hits, turn);
    }

    /// Records status effect applied
    pub fn record_status_effect_applied(&mut self, effect_name: &str, turn: u32) {
        self.run_stats.status_effects_applied.insert(effect_name.to_string());

        // Check if all status effects have been applied (assuming ~10 different effects)
        if self.run_stats.status_effects_applied.len() >= 10 {
            self.unlock(AchievementId::StatusMaster, turn);
        }
    }

    /// Records key collection
    pub fn record_keys(&mut self, key_count: u32, turn: u32) {
        if key_count >= 10 {
            self.unlock(AchievementId::DoorLord, turn);
        }
    }

    /// Records hunger status
    pub fn record_hunger(&mut self, hunger: i32, max_hunger: i32, turn: u32) {
        if hunger >= max_hunger {
            self.unlock(AchievementId::FullBelly, turn);
        }
    }

    /// Records HP status for survival achievements
    pub fn record_hp_status(&mut self, hp: i32, turn: u32) {
        if hp == 1 {
            self.unlock(AchievementId::Survivor, turn);
        }
    }

    /// Records a kill while at 1 HP
    pub fn record_kill_at_1hp(&mut self, turn: u32) {
        self.unlock(AchievementId::LastStand, turn);
    }

    /// Records fully equipped status
    pub fn record_fully_equipped(&mut self, equipped_slots: usize, legendary_slots: usize, has_both_rings: bool, turn: u32) {
        if equipped_slots >= 9 {
            self.unlock(AchievementId::FullyEquipped, turn);
        }
        if legendary_slots >= 5 {
            self.unlock(AchievementId::LegendaryGear, turn);
        }
        if has_both_rings {
            self.unlock(AchievementId::RingBearer, turn);
        }
    }

    /// Records map reveal
    pub fn record_map_revealed(&mut self, turn: u32) {
        self.stats.floors_fully_revealed += 1;
        self.update_progress(AchievementId::MapRevealer, self.stats.floors_fully_revealed, turn);
    }

    /// Gets all visible achievements (non-hidden or unlocked)
    pub fn get_visible_achievements(&self) -> Vec<&Achievement> {
        self.achievements.values()
            .filter(|a| !a.hidden || a.unlocked)
            .collect()
    }

    /// Gets achievements by category
    pub fn get_achievements_by_category(&self, category: AchievementCategory) -> Vec<&Achievement> {
        self.achievements.values()
            .filter(|a| a.category == category && (!a.hidden || a.unlocked))
            .collect()
    }

    /// Gets total achievement count
    pub fn total_count(&self) -> usize {
        self.achievements.len()
    }

    /// Gets unlocked achievement count
    pub fn unlocked_count(&self) -> usize {
        self.achievements.values().filter(|a| a.unlocked).count()
    }

    /// Gets completion percentage
    pub fn completion_percentage(&self) -> f32 {
        (self.unlocked_count() as f32 / self.total_count() as f32) * 100.0
    }

    /// Gets all unlocked titles
    pub fn get_unlocked_titles(&self) -> Vec<String> {
        self.achievements.values()
            .filter(|a| a.unlocked)
            .filter_map(|a| {
                if let Some(AchievementReward::Title(title)) = &a.reward {
                    Some(title.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets total stat bonuses from achievements
    pub fn get_stat_bonuses(&self) -> HashMap<StatType, i32> {
        let mut bonuses = HashMap::new();

        for achievement in self.achievements.values() {
            if achievement.unlocked {
                if let Some(AchievementReward::StatBonus { stat, amount }) = &achievement.reward {
                    *bonuses.entry(*stat).or_insert(0) += amount;
                }
            }
        }

        bonuses
    }

    /// Gets total starting gold bonus from achievements
    pub fn get_starting_gold_bonus(&self) -> u32 {
        self.achievements.values()
            .filter(|a| a.unlocked)
            .filter_map(|a| {
                if let Some(AchievementReward::StartingGold(amount)) = &a.reward {
                    Some(*amount)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_achievement_creation() {
        let tracker = AchievementTracker::new();
        assert!(tracker.total_count() > 0);
        assert_eq!(tracker.unlocked_count(), 0);
    }

    #[test]
    fn test_achievement_unlock() {
        let mut tracker = AchievementTracker::new();
        let result = tracker.unlock(AchievementId::FirstBlood, 1);
        assert!(result);
        assert!(tracker.is_unlocked(AchievementId::FirstBlood));
    }

    #[test]
    fn test_kill_tracking() {
        let mut tracker = AchievementTracker::new();
        tracker.record_kill(EnemyKind::Rat, false, 1);
        assert_eq!(tracker.stats.total_kills, 1);
        assert!(tracker.is_unlocked(AchievementId::FirstBlood));
    }

    #[test]
    fn test_boss_tracking() {
        let mut tracker = AchievementTracker::new();
        tracker.record_boss_defeat(EnemyKind::BossGoblinKing, 5, 0, 100);
        assert!(tracker.is_unlocked(AchievementId::GoblinKingSlayer));
        assert!(tracker.is_unlocked(AchievementId::FlawlessBoss)); // 0 damage taken
    }

    #[test]
    fn test_progress_tracking() {
        let mut tracker = AchievementTracker::new();
        for _ in 0..10 {
            tracker.record_kill(EnemyKind::Rat, false, 1);
        }
        assert!(tracker.is_unlocked(AchievementId::Apprentice));
    }

    #[test]
    fn test_stat_bonuses() {
        let mut tracker = AchievementTracker::new();
        // Unlock an achievement with stat bonus
        for _ in 0..1000 {
            tracker.record_kill(EnemyKind::Rat, false, 1);
        }
        let bonuses = tracker.get_stat_bonuses();
        assert!(bonuses.get(&StatType::Attack).unwrap_or(&0) > &0);
    }

    #[test]
    fn test_category_filtering() {
        let tracker = AchievementTracker::new();
        let combat = tracker.get_achievements_by_category(AchievementCategory::Combat);
        assert!(!combat.is_empty());
        assert!(combat.iter().all(|a| a.category == AchievementCategory::Combat));
    }
}

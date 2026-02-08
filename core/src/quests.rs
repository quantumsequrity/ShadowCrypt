//! Quest system: quest definitions, objectives, rewards, and tracking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entities::EnemyKind;
use crate::items::{Item, ItemKind, Rarity};

/// Unique identifier for quests
pub type QuestId = u32;

/// Types of objectives that can be part of a quest
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ObjectiveType {
    /// Kill a specific number of any enemies
    KillEnemies(u32),
    /// Kill a specific number of a particular enemy type
    KillEnemyType(EnemyKind, u32),
    /// Kill a specific boss
    DefeatBoss(EnemyKind),
    /// Collect a specific amount of gold
    CollectGold(u32),
    /// Collect a specific item type
    CollectItem(ItemKind, u32),
    /// Collect items of a minimum rarity
    CollectRarityItems(Rarity, u32),
    /// Reach a specific dungeon level
    ReachDungeonLevel(u32),
    /// Reach a specific player level
    ReachPlayerLevel(u32),
    /// Open a number of chests
    OpenChests(u32),
    /// Use shrines a number of times
    UseShrines(u32),
    /// Survive a number of turns
    SurviveTurns(u32),
    /// Deal a total amount of damage
    DealDamage(u32),
    /// Use skills a number of times
    UseSkills(u32),
    /// Explore rooms (visit unique rooms)
    ExploreRooms(u32),
    /// Descend stairs a number of times
    DescendStairs(u32),
    /// Kill enemies without taking any damage
    KillWithoutDamage(u32),
    /// Kill enemies within a time limit (turns)
    KillInTime(u32, u32),
    /// Escort an NPC safely through a dungeon
    EscortNPC(String),
    /// Rescue an NPC from captivity
    RescueNPC(String),
    /// Craft a specific item
    CraftItem(ItemKind),
    /// Enchant a specific item
    EnchantItem(ItemKind),
    /// Find a secret room
    FindSecretRoom(u32),
    /// Talk to an NPC
    TalkToNPC(String),
    /// Deliver an item to an NPC
    DeliverItem(ItemKind, String),
    /// Complete a floor without killing any enemies
    CompleteFloorWithoutKilling(u32),
    /// Survive waves of enemies
    SurviveWaves(u32),
}

impl ObjectiveType {
    /// Returns a human-readable description of the objective
    pub fn description(&self) -> String {
        match self {
            Self::KillEnemies(n) => format!("Kill {} enemies", n),
            Self::KillEnemyType(kind, n) => format!("Kill {} {}", n, kind.name()),
            Self::DefeatBoss(kind) => format!("Defeat the {}", kind.name()),
            Self::CollectGold(n) => format!("Collect {} gold", n),
            Self::CollectItem(kind, n) => format!("Collect {} {}", n, kind.name()),
            Self::CollectRarityItems(rarity, n) => {
                format!("Collect {} {} or better items", n, rarity.prefix().trim())
            }
            Self::ReachDungeonLevel(n) => format!("Reach dungeon level {}", n),
            Self::ReachPlayerLevel(n) => format!("Reach player level {}", n),
            Self::OpenChests(n) => format!("Open {} chests", n),
            Self::UseShrines(n) => format!("Use {} shrines", n),
            Self::SurviveTurns(n) => format!("Survive for {} turns", n),
            Self::DealDamage(n) => format!("Deal {} total damage", n),
            Self::UseSkills(n) => format!("Use skills {} times", n),
            Self::ExploreRooms(n) => format!("Explore {} rooms", n),
            Self::DescendStairs(n) => format!("Descend stairs {} times", n),
            Self::KillWithoutDamage(n) => format!("Kill {} enemies without taking damage", n),
            Self::KillInTime(n, turns) => format!("Kill {} enemies within {} turns", n, turns),
            Self::EscortNPC(name) => format!("Escort {} safely", name),
            Self::RescueNPC(name) => format!("Rescue {}", name),
            Self::CraftItem(kind) => format!("Craft a {}", kind.name()),
            Self::EnchantItem(kind) => format!("Enchant a {}", kind.name()),
            Self::FindSecretRoom(n) => format!("Find {} secret room(s)", n),
            Self::TalkToNPC(name) => format!("Talk to {}", name),
            Self::DeliverItem(kind, name) => format!("Deliver {} to {}", kind.name(), name),
            Self::CompleteFloorWithoutKilling(n) => format!("Complete {} floor(s) without killing", n),
            Self::SurviveWaves(n) => format!("Survive {} waves of enemies", n),
        }
    }

    /// Returns the target value for this objective
    pub fn target(&self) -> u32 {
        match self {
            Self::KillEnemies(n)
            | Self::KillEnemyType(_, n)
            | Self::CollectGold(n)
            | Self::CollectItem(_, n)
            | Self::CollectRarityItems(_, n)
            | Self::ReachDungeonLevel(n)
            | Self::ReachPlayerLevel(n)
            | Self::OpenChests(n)
            | Self::UseShrines(n)
            | Self::SurviveTurns(n)
            | Self::DealDamage(n)
            | Self::UseSkills(n)
            | Self::ExploreRooms(n)
            | Self::DescendStairs(n)
            | Self::KillWithoutDamage(n)
            | Self::FindSecretRoom(n)
            | Self::CompleteFloorWithoutKilling(n)
            | Self::SurviveWaves(n) => *n,
            Self::DefeatBoss(_)
            | Self::EscortNPC(_)
            | Self::RescueNPC(_)
            | Self::CraftItem(_)
            | Self::EnchantItem(_)
            | Self::TalkToNPC(_)
            | Self::DeliverItem(_, _) => 1,
            Self::KillInTime(n, _) => *n,
        }
    }
}

/// Progress tracking for a single objective
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ObjectiveProgress {
    pub objective: ObjectiveType,
    pub current: u32,
    pub completed: bool,
}

impl ObjectiveProgress {
    pub fn new(objective: ObjectiveType) -> Self {
        Self {
            objective,
            current: 0,
            completed: false,
        }
    }

    /// Update progress and return true if newly completed
    pub fn update(&mut self, amount: u32) -> bool {
        if self.completed {
            return false;
        }
        self.current = self.current.saturating_add(amount);
        if self.current >= self.objective.target() {
            self.current = self.objective.target();
            self.completed = true;
            return true;
        }
        false
    }

    /// Set progress to a specific value (for level/gold tracking)
    pub fn set_progress(&mut self, value: u32) -> bool {
        if self.completed {
            return false;
        }
        self.current = value.min(self.objective.target());
        if self.current >= self.objective.target() {
            self.completed = true;
            return true;
        }
        false
    }

    /// Returns progress as a percentage (0-100)
    pub fn percentage(&self) -> u32 {
        let target = self.objective.target();
        if target == 0 {
            return 100;
        }
        ((self.current as f32 / target as f32) * 100.0).min(100.0) as u32
    }

    /// Returns a progress string like "3/10"
    pub fn progress_string(&self) -> String {
        format!("{}/{}", self.current, self.objective.target())
    }
}

/// Types of rewards that can be granted for completing quests
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum QuestReward {
    /// Grant experience points
    Experience(u32),
    /// Grant gold
    Gold(u32),
    /// Grant a specific item
    Item(ItemKind, Rarity),
    /// Grant multiple random items of minimum rarity
    RandomItems(u32, Rarity),
    /// Increase max HP permanently
    MaxHpBonus(i32),
    /// Increase max mana permanently
    MaxManaBonus(i32),
    /// Increase base attack permanently
    AttackBonus(i32),
    /// Increase base defense permanently
    DefenseBonus(i32),
    /// Grant a skill point (for future skill tree)
    SkillPoint(u32),
    /// Unlock access to a special area or feature
    Unlock(String),
}

impl QuestReward {
    /// Returns a human-readable description of the reward
    pub fn description(&self) -> String {
        match self {
            Self::Experience(n) => format!("+{} XP", n),
            Self::Gold(n) => format!("+{} Gold", n),
            Self::Item(kind, rarity) => format!("{}{}", rarity.prefix(), kind.name()),
            Self::RandomItems(n, rarity) => {
                format!("{} random {} items", n, rarity.prefix().trim())
            }
            Self::MaxHpBonus(n) => format!("+{} Max HP", n),
            Self::MaxManaBonus(n) => format!("+{} Max Mana", n),
            Self::AttackBonus(n) => format!("+{} Attack", n),
            Self::DefenseBonus(n) => format!("+{} Defense", n),
            Self::SkillPoint(n) => format!("+{} Skill Point(s)", n),
            Self::Unlock(s) => format!("Unlock: {}", s),
        }
    }
}

/// Quest difficulty/category
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum QuestCategory {
    /// Tutorial/early game quests
    Starter,
    /// Main story progression quests
    Main,
    /// Optional side quests
    Side,
    /// Repeatable daily/challenge quests
    Daily,
    /// Achievement-style quests
    Achievement,
    /// Hidden/secret quests
    Hidden,
    /// Escort quests
    Escort,
    /// Rescue quests
    Rescue,
    /// Mystery/investigation quests
    Mystery,
    /// Crafting quests
    Crafting,
    /// Bounty quests
    Bounty,
    /// Exploration quests
    Exploration,
    /// Collection quests
    Collection,
    /// Survival quests
    Survival,
    /// Arena combat quests
    Arena,
    /// Guild-specific quests
    Guild,
}

impl QuestCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Starter => "Starter",
            Self::Main => "Main Quest",
            Self::Side => "Side Quest",
            Self::Daily => "Daily Challenge",
            Self::Achievement => "Achievement",
            Self::Hidden => "Secret",
            Self::Escort => "Escort",
            Self::Rescue => "Rescue",
            Self::Mystery => "Mystery",
            Self::Crafting => "Crafting",
            Self::Bounty => "Bounty",
            Self::Exploration => "Exploration",
            Self::Collection => "Collection",
            Self::Survival => "Survival",
            Self::Arena => "Arena",
            Self::Guild => "Guild",
        }
    }

    /// Returns color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Starter => 5,      // Green
            Self::Main => 11,        // Yellow
            Self::Side => 7,         // Blue
            Self::Daily => 9,        // Cyan
            Self::Achievement => 13, // Magenta
            Self::Hidden => 4,       // Purple
            Self::Escort => 6,       // Orange
            Self::Rescue => 12,      // Light Red
            Self::Mystery => 3,      // Dark Cyan
            Self::Crafting => 14,    // Light Yellow
            Self::Bounty => 1,       // Red
            Self::Exploration => 10, // Light Green
            Self::Collection => 8,   // Dark Gray
            Self::Survival => 2,     // Dark Green
            Self::Arena => 15,       // White
            Self::Guild => 6,        // Orange
        }
    }
}

/// The current state of a quest
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum QuestState {
    /// Quest is not yet available to the player
    Locked,
    /// Quest is available but not started
    Available,
    /// Quest is active and in progress
    Active,
    /// All objectives completed, ready to turn in
    ReadyToComplete,
    /// Quest has been completed and rewards claimed
    Completed,
    /// Quest failed (for timed quests)
    Failed,
}

/// Definition of a quest
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Quest {
    pub id: QuestId,
    pub name: String,
    pub description: String,
    pub category: QuestCategory,
    pub objectives: Vec<ObjectiveType>,
    pub rewards: Vec<QuestReward>,
    /// Minimum dungeon level to unlock this quest
    pub min_dungeon_level: u32,
    /// Minimum player level to unlock this quest
    pub min_player_level: u32,
    /// Quest IDs that must be completed before this one is available
    pub prerequisites: Vec<QuestId>,
    /// Whether this quest can be repeated
    pub repeatable: bool,
    /// Turn limit for timed quests (0 = no limit)
    pub turn_limit: u32,
}

impl Quest {
    pub fn new(id: QuestId, name: &str, description: &str, category: QuestCategory) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            category,
            objectives: Vec::new(),
            rewards: Vec::new(),
            min_dungeon_level: 1,
            min_player_level: 1,
            prerequisites: Vec::new(),
            repeatable: false,
            turn_limit: 0,
        }
    }

    pub fn with_objective(mut self, objective: ObjectiveType) -> Self {
        self.objectives.push(objective);
        self
    }

    pub fn with_reward(mut self, reward: QuestReward) -> Self {
        self.rewards.push(reward);
        self
    }

    pub fn with_min_dungeon_level(mut self, level: u32) -> Self {
        self.min_dungeon_level = level;
        self
    }

    pub fn with_min_player_level(mut self, level: u32) -> Self {
        self.min_player_level = level;
        self
    }

    pub fn with_prerequisite(mut self, quest_id: QuestId) -> Self {
        self.prerequisites.push(quest_id);
        self
    }

    pub fn with_repeatable(mut self, repeatable: bool) -> Self {
        self.repeatable = repeatable;
        self
    }

    pub fn with_turn_limit(mut self, turns: u32) -> Self {
        self.turn_limit = turns;
        self
    }
}

/// Active quest instance with progress tracking
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ActiveQuest {
    pub quest_id: QuestId,
    pub state: QuestState,
    pub objective_progress: Vec<ObjectiveProgress>,
    pub turns_elapsed: u32,
    pub times_completed: u32,
}

impl ActiveQuest {
    pub fn new(quest: &Quest) -> Self {
        let objective_progress = quest
            .objectives
            .iter()
            .map(|obj| ObjectiveProgress::new(obj.clone()))
            .collect();

        Self {
            quest_id: quest.id,
            state: QuestState::Active,
            objective_progress,
            turns_elapsed: 0,
            times_completed: 0,
        }
    }

    /// Check if all objectives are completed
    pub fn all_objectives_completed(&self) -> bool {
        self.objective_progress.iter().all(|p| p.completed)
    }

    /// Update state based on objectives
    pub fn update_state(&mut self) {
        if self.state == QuestState::Active && self.all_objectives_completed() {
            self.state = QuestState::ReadyToComplete;
        }
    }

    /// Returns overall completion percentage
    pub fn overall_percentage(&self) -> u32 {
        if self.objective_progress.is_empty() {
            return 100;
        }
        let total: u32 = self.objective_progress.iter().map(|p| p.percentage()).sum();
        total / self.objective_progress.len() as u32
    }
}

/// Quest tracker managing all quests for a player
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QuestTracker {
    /// All available quest definitions
    pub quest_definitions: Vec<Quest>,
    /// Currently active quests
    pub active_quests: HashMap<QuestId, ActiveQuest>,
    /// Completed quest IDs (for non-repeatable quests)
    pub completed_quests: Vec<QuestId>,
    /// Total quests completed counter
    pub total_quests_completed: u32,
    /// Rooms explored (for exploration objectives)
    pub rooms_explored: u32,
    /// Total damage dealt (for damage objectives)
    pub total_damage_dealt: u32,
    /// Skills used counter
    pub skills_used: u32,
    /// Chests opened counter
    pub chests_opened: u32,
    /// Shrines used counter
    pub shrines_used: u32,
    /// Stairs descended counter
    pub stairs_descended: u32,
}

impl Default for QuestTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl QuestTracker {
    pub fn new() -> Self {
        let mut tracker = Self {
            quest_definitions: Vec::new(),
            active_quests: HashMap::new(),
            completed_quests: Vec::new(),
            total_quests_completed: 0,
            rooms_explored: 0,
            total_damage_dealt: 0,
            skills_used: 0,
            chests_opened: 0,
            shrines_used: 0,
            stairs_descended: 0,
        };
        tracker.initialize_quests();
        tracker
    }

    /// Initialize all quest definitions
    fn initialize_quests(&mut self) {
        self.quest_definitions = vec![
            // === STARTER QUESTS ===
            Quest::new(1, "First Blood", "Defeat your first enemy.", QuestCategory::Starter)
                .with_objective(ObjectiveType::KillEnemies(1))
                .with_reward(QuestReward::Experience(25))
                .with_reward(QuestReward::Gold(10)),
            Quest::new(
                2,
                "Getting Started",
                "Reach the second floor of the dungeon.",
                QuestCategory::Starter,
            )
            .with_objective(ObjectiveType::ReachDungeonLevel(2))
            .with_reward(QuestReward::Experience(50))
            .with_reward(QuestReward::Item(ItemKind::HealthPotion, Rarity::Common)),
            Quest::new(
                3,
                "Treasure Hunter",
                "Open your first chest.",
                QuestCategory::Starter,
            )
            .with_objective(ObjectiveType::OpenChests(1))
            .with_reward(QuestReward::Experience(30))
            .with_reward(QuestReward::Gold(25)),
            Quest::new(
                4,
                "Pest Control",
                "Clear out 10 rats and spiders.",
                QuestCategory::Starter,
            )
            .with_objective(ObjectiveType::KillEnemies(10))
            .with_reward(QuestReward::Experience(75))
            .with_reward(QuestReward::Item(ItemKind::LeatherArmor, Rarity::Common)),
            Quest::new(
                5,
                "Blessed",
                "Use a shrine for the first time.",
                QuestCategory::Starter,
            )
            .with_objective(ObjectiveType::UseShrines(1))
            .with_reward(QuestReward::Experience(40))
            .with_reward(QuestReward::MaxHpBonus(5)),
            // === MAIN QUESTS ===
            Quest::new(
                10,
                "Goblin Menace",
                "Defeat the Goblin King on level 5.",
                QuestCategory::Main,
            )
            .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossGoblinKing))
            .with_reward(QuestReward::Experience(500))
            .with_reward(QuestReward::Gold(200))
            .with_reward(QuestReward::Item(ItemKind::ShortSword, Rarity::Rare))
            .with_min_dungeon_level(5)
            .with_prerequisite(1),
            Quest::new(
                11,
                "Orcish Uprising",
                "Defeat the Orc Warlord on level 10.",
                QuestCategory::Main,
            )
            .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossOrcWarlord))
            .with_reward(QuestReward::Experience(1000))
            .with_reward(QuestReward::Gold(500))
            .with_reward(QuestReward::Item(ItemKind::ChainMail, Rarity::Rare))
            .with_min_dungeon_level(10)
            .with_prerequisite(10),
            Quest::new(
                12,
                "Vampire's Lair",
                "Defeat the Vampire Lord on level 15.",
                QuestCategory::Main,
            )
            .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossVampireLord))
            .with_reward(QuestReward::Experience(2000))
            .with_reward(QuestReward::Gold(1000))
            .with_reward(QuestReward::Item(ItemKind::RingOfTheVampire, Rarity::Epic))
            .with_min_dungeon_level(15)
            .with_prerequisite(11),
            Quest::new(
                13,
                "Nature's Wrath",
                "Defeat the Forest Guardian on level 20.",
                QuestCategory::Main,
            )
            .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossForestGuardian))
            .with_reward(QuestReward::Experience(3000))
            .with_reward(QuestReward::Gold(1500))
            .with_reward(QuestReward::Item(ItemKind::DragonArmor, Rarity::Epic))
            .with_min_dungeon_level(20)
            .with_prerequisite(12),
            Quest::new(
                14,
                "Frozen Terror",
                "Defeat the Ice Dragon on level 25.",
                QuestCategory::Main,
            )
            .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossIceDragon))
            .with_reward(QuestReward::Experience(5000))
            .with_reward(QuestReward::Gold(2500))
            .with_reward(QuestReward::Item(ItemKind::FrostBlade, Rarity::Legendary))
            .with_min_dungeon_level(25)
            .with_prerequisite(13),
            Quest::new(
                15,
                "The Final Battle",
                "Defeat the Demon King and save the realm!",
                QuestCategory::Main,
            )
            .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossDemonKing))
            .with_reward(QuestReward::Experience(10000))
            .with_reward(QuestReward::Gold(5000))
            .with_reward(QuestReward::Item(ItemKind::DemonSlayer, Rarity::Mythic))
            .with_min_dungeon_level(30)
            .with_prerequisite(14),
            // === SIDE QUESTS ===
            Quest::new(20, "Exterminator", "Kill 50 enemies.", QuestCategory::Side)
                .with_objective(ObjectiveType::KillEnemies(50))
                .with_reward(QuestReward::Experience(200))
                .with_reward(QuestReward::AttackBonus(2)),
            Quest::new(
                21,
                "Undead Hunter",
                "Slay 20 undead creatures.",
                QuestCategory::Side,
            )
            .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Skeleton, 10))
            .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Zombie, 10))
            .with_reward(QuestReward::Experience(300))
            .with_reward(QuestReward::Item(ItemKind::ScrollMassHeal, Rarity::Rare))
            .with_min_dungeon_level(9),
            Quest::new(
                22,
                "Wealthy",
                "Accumulate 1000 gold.",
                QuestCategory::Side,
            )
            .with_objective(ObjectiveType::CollectGold(1000))
            .with_reward(QuestReward::Experience(150))
            .with_reward(QuestReward::Gold(500)),
            Quest::new(23, "Deep Delver", "Reach dungeon level 15.", QuestCategory::Side)
                .with_objective(ObjectiveType::ReachDungeonLevel(15))
                .with_reward(QuestReward::Experience(400))
                .with_reward(QuestReward::MaxHpBonus(15)),
            Quest::new(
                24,
                "Skill Master",
                "Use skills 25 times.",
                QuestCategory::Side,
            )
            .with_objective(ObjectiveType::UseSkills(25))
                .with_reward(QuestReward::Experience(200))
                .with_reward(QuestReward::MaxManaBonus(20)),
            Quest::new(
                25,
                "Explorer",
                "Explore 100 rooms.",
                QuestCategory::Side,
            )
            .with_objective(ObjectiveType::ExploreRooms(100))
            .with_reward(QuestReward::Experience(250))
            .with_reward(QuestReward::Item(ItemKind::ScrollMapping, Rarity::Rare)),
            Quest::new(
                26,
                "Collector",
                "Find 5 rare or better items.",
                QuestCategory::Side,
            )
            .with_objective(ObjectiveType::CollectRarityItems(Rarity::Rare, 5))
            .with_reward(QuestReward::Experience(350))
            .with_reward(QuestReward::RandomItems(2, Rarity::Epic)),
            Quest::new(
                27,
                "Descent",
                "Descend the stairs 20 times.",
                QuestCategory::Side,
            )
            .with_objective(ObjectiveType::DescendStairs(20))
            .with_reward(QuestReward::Experience(300))
            .with_reward(QuestReward::DefenseBonus(3)),
            // === DAILY CHALLENGES ===
            Quest::new(
                30,
                "Daily Slayer",
                "Kill 15 enemies today.",
                QuestCategory::Daily,
            )
            .with_objective(ObjectiveType::KillEnemies(15))
            .with_reward(QuestReward::Experience(100))
            .with_reward(QuestReward::Gold(50))
            .with_repeatable(true),
            Quest::new(
                31,
                "Daily Treasure",
                "Open 5 chests today.",
                QuestCategory::Daily,
            )
            .with_objective(ObjectiveType::OpenChests(5))
            .with_reward(QuestReward::Experience(75))
            .with_reward(QuestReward::RandomItems(1, Rarity::Uncommon))
            .with_repeatable(true),
            Quest::new(
                32,
                "Speed Runner",
                "Descend 5 floors in 500 turns.",
                QuestCategory::Daily,
            )
            .with_objective(ObjectiveType::DescendStairs(5))
            .with_reward(QuestReward::Experience(200))
            .with_reward(QuestReward::Item(ItemKind::BootsOfSpeed, Rarity::Rare))
            .with_turn_limit(500)
            .with_repeatable(true),
            // === ACHIEVEMENTS ===
            Quest::new(
                40,
                "Century Slayer",
                "Kill 100 enemies.",
                QuestCategory::Achievement,
            )
            .with_objective(ObjectiveType::KillEnemies(100))
            .with_reward(QuestReward::Experience(500))
            .with_reward(QuestReward::AttackBonus(5)),
            Quest::new(
                41,
                "Massacre",
                "Kill 500 enemies.",
                QuestCategory::Achievement,
            )
            .with_objective(ObjectiveType::KillEnemies(500))
            .with_reward(QuestReward::Experience(2000))
            .with_reward(QuestReward::Item(ItemKind::DemonSlayer, Rarity::Epic))
            .with_prerequisite(40),
            Quest::new(
                42,
                "Survivor",
                "Survive 1000 turns.",
                QuestCategory::Achievement,
            )
            .with_objective(ObjectiveType::SurviveTurns(1000))
            .with_reward(QuestReward::MaxHpBonus(25))
            .with_reward(QuestReward::DefenseBonus(5)),
            Quest::new(
                43,
                "Damage Dealer",
                "Deal 10000 total damage.",
                QuestCategory::Achievement,
            )
            .with_objective(ObjectiveType::DealDamage(10000))
            .with_reward(QuestReward::Experience(1000))
            .with_reward(QuestReward::AttackBonus(10)),
            Quest::new(
                44,
                "Wealthy Adventurer",
                "Accumulate 10000 gold total.",
                QuestCategory::Achievement,
            )
            .with_objective(ObjectiveType::CollectGold(10000))
            .with_reward(QuestReward::Experience(1500))
            .with_reward(QuestReward::Item(ItemKind::CrownOfKings, Rarity::Legendary)),
            Quest::new(45, "Max Level", "Reach player level 30.", QuestCategory::Achievement)
                .with_objective(ObjectiveType::ReachPlayerLevel(30))
                .with_reward(QuestReward::Experience(5000))
                .with_reward(QuestReward::MaxHpBonus(50))
                .with_reward(QuestReward::MaxManaBonus(50)),
            // === HIDDEN QUESTS ===
            Quest::new(
                50,
                "Shrine Devotee",
                "Use 10 shrines.",
                QuestCategory::Hidden,
            )
            .with_objective(ObjectiveType::UseShrines(10))
            .with_reward(QuestReward::Experience(400))
            .with_reward(QuestReward::MaxHpBonus(10))
            .with_reward(QuestReward::MaxManaBonus(10)),
            Quest::new(
                51,
                "Hoarder",
                "Collect 20 legendary items.",
                QuestCategory::Hidden,
            )
            .with_objective(ObjectiveType::CollectRarityItems(Rarity::Legendary, 20))
            .with_reward(QuestReward::Experience(3000))
            .with_reward(QuestReward::Item(ItemKind::AmuletOfTheGods, Rarity::Mythic)),
        ];

        // Append all predefined quests from the expanded quest database
        self.quest_definitions.extend(predefined_quests());
    }

    /// Get a quest definition by ID
    pub fn get_quest(&self, id: QuestId) -> Option<&Quest> {
        self.quest_definitions.iter().find(|q| q.id == id)
    }

    /// Check if a quest's prerequisites are met
    pub fn prerequisites_met(&self, quest: &Quest) -> bool {
        quest
            .prerequisites
            .iter()
            .all(|prereq_id| self.completed_quests.contains(prereq_id))
    }

    /// Check if a quest is available given current game state
    pub fn is_quest_available(
        &self,
        quest: &Quest,
        dungeon_level: u32,
        player_level: u32,
    ) -> bool {
        if !quest.repeatable && self.completed_quests.contains(&quest.id) {
            return false;
        }
        if self.active_quests.contains_key(&quest.id) {
            return false;
        }
        if dungeon_level < quest.min_dungeon_level {
            return false;
        }
        if player_level < quest.min_player_level {
            return false;
        }
        self.prerequisites_met(quest)
    }

    /// Get all available quests that can be started
    pub fn get_available_quests(&self, dungeon_level: u32, player_level: u32) -> Vec<&Quest> {
        self.quest_definitions
            .iter()
            .filter(|q| self.is_quest_available(q, dungeon_level, player_level))
            .collect()
    }

    /// Start a quest
    pub fn start_quest(&mut self, quest_id: QuestId) -> Option<String> {
        if let Some(quest) = self.get_quest(quest_id).cloned() {
            if self.active_quests.contains_key(&quest_id) {
                return Some("Quest already active.".to_string());
            }
            let active = ActiveQuest::new(&quest);
            self.active_quests.insert(quest_id, active);
            Some(format!("Quest started: {}", quest.name))
        } else {
            None
        }
    }

    /// Auto-start available starter quests
    pub fn auto_start_starter_quests(&mut self, dungeon_level: u32, player_level: u32) {
        let starter_ids: Vec<QuestId> = self
            .quest_definitions
            .iter()
            .filter(|q| {
                q.category == QuestCategory::Starter
                    && self.is_quest_available(q, dungeon_level, player_level)
            })
            .map(|q| q.id)
            .collect();

        for id in starter_ids {
            self.start_quest(id);
        }
    }

    /// Complete a quest and return rewards
    pub fn complete_quest(&mut self, quest_id: QuestId) -> Option<Vec<QuestReward>> {
        if let Some(active) = self.active_quests.get(&quest_id) {
            if active.state != QuestState::ReadyToComplete {
                return None;
            }
        } else {
            return None;
        }

        let quest = self.get_quest(quest_id)?.clone();
        let mut active = self.active_quests.remove(&quest_id)?;

        active.state = QuestState::Completed;
        active.times_completed += 1;
        self.total_quests_completed += 1;

        if !quest.repeatable {
            self.completed_quests.push(quest_id);
        }

        Some(quest.rewards.clone())
    }

    /// Update quest progress for enemy kills
    pub fn on_enemy_killed(&mut self, enemy_kind: EnemyKind) -> Vec<String> {
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                let updated = match &progress.objective {
                    ObjectiveType::KillEnemies(_) => progress.update(1),
                    ObjectiveType::KillEnemyType(kind, _) if *kind == enemy_kind => {
                        progress.update(1)
                    }
                    ObjectiveType::DefeatBoss(kind) if *kind == enemy_kind => progress.update(1),
                    _ => false,
                };
                if updated {
                    messages.push(format!(
                        "Quest progress: {} ({})",
                        progress.objective.description(),
                        progress.progress_string()
                    ));
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for gold collection
    pub fn on_gold_changed(&mut self, total_gold: u32) -> Vec<String> {
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                if let ObjectiveType::CollectGold(_) = &progress.objective {
                    if progress.set_progress(total_gold) {
                        messages.push(format!(
                            "Quest objective complete: {}",
                            progress.objective.description()
                        ));
                    }
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for dungeon level changes
    pub fn on_dungeon_level_changed(&mut self, level: u32) -> Vec<String> {
        self.stairs_descended += 1;
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                let updated = match &progress.objective {
                    ObjectiveType::ReachDungeonLevel(_) => progress.set_progress(level),
                    ObjectiveType::DescendStairs(_) => progress.update(1),
                    _ => false,
                };
                if updated && progress.completed {
                    messages.push(format!(
                        "Quest objective complete: {}",
                        progress.objective.description()
                    ));
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for player level changes
    pub fn on_player_level_changed(&mut self, level: u32) -> Vec<String> {
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                if let ObjectiveType::ReachPlayerLevel(_) = &progress.objective {
                    if progress.set_progress(level) {
                        messages.push(format!(
                            "Quest objective complete: {}",
                            progress.objective.description()
                        ));
                    }
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for chest opening
    pub fn on_chest_opened(&mut self) -> Vec<String> {
        self.chests_opened += 1;
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                if let ObjectiveType::OpenChests(_) = &progress.objective {
                    if progress.update(1) {
                        messages.push(format!(
                            "Quest progress: {} ({})",
                            progress.objective.description(),
                            progress.progress_string()
                        ));
                    }
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for shrine usage
    pub fn on_shrine_used(&mut self) -> Vec<String> {
        self.shrines_used += 1;
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                if let ObjectiveType::UseShrines(_) = &progress.objective {
                    if progress.update(1) {
                        messages.push(format!(
                            "Quest progress: {} ({})",
                            progress.objective.description(),
                            progress.progress_string()
                        ));
                    }
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for skill usage
    pub fn on_skill_used(&mut self) -> Vec<String> {
        self.skills_used += 1;
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                if let ObjectiveType::UseSkills(_) = &progress.objective {
                    if progress.update(1) {
                        messages.push(format!(
                            "Quest progress: {} ({})",
                            progress.objective.description(),
                            progress.progress_string()
                        ));
                    }
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for damage dealt
    pub fn on_damage_dealt(&mut self, damage: u32) -> Vec<String> {
        self.total_damage_dealt += damage;
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                if let ObjectiveType::DealDamage(_) = &progress.objective {
                    if progress.set_progress(self.total_damage_dealt) && progress.completed {
                        messages.push(format!(
                            "Quest objective complete: {}",
                            progress.objective.description()
                        ));
                    }
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for room exploration
    pub fn on_room_explored(&mut self) -> Vec<String> {
        self.rooms_explored += 1;
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                if let ObjectiveType::ExploreRooms(_) = &progress.objective {
                    if progress.set_progress(self.rooms_explored) && progress.completed {
                        messages.push(format!(
                            "Quest objective complete: {}",
                            progress.objective.description()
                        ));
                    }
                }
            }
            active.update_state();
        }

        messages
    }

    /// Update quest progress for item collection
    pub fn on_item_collected(&mut self, item_kind: ItemKind, rarity: Rarity) -> Vec<String> {
        let mut messages = Vec::new();

        for active in self.active_quests.values_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            for progress in &mut active.objective_progress {
                let updated = match &progress.objective {
                    ObjectiveType::CollectItem(kind, _) if *kind == item_kind => progress.update(1),
                    ObjectiveType::CollectRarityItems(min_rarity, _) if rarity >= *min_rarity => {
                        progress.update(1)
                    }
                    _ => false,
                };
                if updated {
                    messages.push(format!(
                        "Quest progress: {} ({})",
                        progress.objective.description(),
                        progress.progress_string()
                    ));
                }
            }
            active.update_state();
        }

        messages
    }

    /// Tick turn-based quest tracking
    pub fn on_turn(&mut self, turn_count: u32) -> Vec<String> {
        let mut messages = Vec::new();
        let mut failed_quests = Vec::new();

        // Build a map of quest turn limits and names to avoid borrow issues
        let quest_info: HashMap<QuestId, (u32, String)> = self
            .quest_definitions
            .iter()
            .map(|q| (q.id, (q.turn_limit, q.name.clone())))
            .collect();

        for (quest_id, active) in self.active_quests.iter_mut() {
            if active.state != QuestState::Active {
                continue;
            }

            active.turns_elapsed += 1;

            // Update survive turns objectives
            for progress in &mut active.objective_progress {
                if let ObjectiveType::SurviveTurns(_) = &progress.objective {
                    if progress.set_progress(turn_count) && progress.completed {
                        messages.push(format!(
                            "Quest objective complete: {}",
                            progress.objective.description()
                        ));
                    }
                }
            }
            active.update_state();

            // Check turn limit
            if let Some((turn_limit, quest_name)) = quest_info.get(quest_id) {
                if *turn_limit > 0 && active.turns_elapsed >= *turn_limit {
                    if !active.all_objectives_completed() {
                        failed_quests.push(*quest_id);
                        messages.push(format!("Quest failed: {} (time limit exceeded)", quest_name));
                    }
                }
            }
        }

        // Mark failed quests
        for quest_id in failed_quests {
            if let Some(active) = self.active_quests.get_mut(&quest_id) {
                active.state = QuestState::Failed;
            }
        }

        messages
    }

    /// Get active quests ready to complete
    pub fn get_completable_quests(&self) -> Vec<QuestId> {
        self.active_quests
            .iter()
            .filter(|(_, a)| a.state == QuestState::ReadyToComplete)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get all active quest IDs
    pub fn get_active_quest_ids(&self) -> Vec<QuestId> {
        self.active_quests
            .iter()
            .filter(|(_, a)| a.state == QuestState::Active || a.state == QuestState::ReadyToComplete)
            .map(|(id, _)| *id)
            .collect()
    }
}

/// Returns 100+ predefined quests organized by category.
/// These are appended to the base quest definitions during initialization.
pub fn predefined_quests() -> Vec<Quest> {
    vec![
        // =====================================================================
        // MAIN STORY CHAIN (13 quests, IDs 100-112)
        // =====================================================================
        Quest::new(
            100,
            "The Awakening",
            "You awaken in a dark chamber with no memory. Discover your latent power by defeating enemies and exploring the first floor.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::KillEnemies(3))
        .with_objective(ObjectiveType::ExploreRooms(5))
        .with_reward(QuestReward::Experience(100))
        .with_reward(QuestReward::Gold(50))
        .with_reward(QuestReward::Item(ItemKind::Dagger, Rarity::Uncommon)),

        Quest::new(
            101,
            "Descent into Darkness",
            "The way forward leads deeper underground. Enter the first true dungeon and survive its perils.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::ReachDungeonLevel(3))
        .with_objective(ObjectiveType::KillEnemies(10))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(100))
        .with_reward(QuestReward::Item(ItemKind::LeatherArmor, Rarity::Uncommon))
        .with_min_dungeon_level(1)
        .with_prerequisite(100),

        Quest::new(
            102,
            "The Shadow Rises",
            "Dark whispers echo through the corridors. An ancient evil stirs in the depths. Learn of the great threat by delving deeper.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::ReachDungeonLevel(5))
        .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossGoblinKing))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(250))
        .with_reward(QuestReward::Item(ItemKind::ShortSword, Rarity::Rare))
        .with_min_dungeon_level(3)
        .with_prerequisite(101),

        Quest::new(
            103,
            "Allies of Light",
            "You cannot face the darkness alone. Seek out companions by talking to the wandering NPCs in the dungeon.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::TalkToNPC("Elder Mage Thandril".to_string()))
        .with_objective(ObjectiveType::TalkToNPC("Captain Varek".to_string()))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(200))
        .with_reward(QuestReward::Unlock("Companion System".to_string()))
        .with_min_dungeon_level(5)
        .with_prerequisite(102),

        Quest::new(
            104,
            "The Lost Temple",
            "Ancient texts speak of a temple hidden deep within the earth. Find the secret rooms that lead to its entrance.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::FindSecretRoom(3))
        .with_objective(ObjectiveType::ReachDungeonLevel(10))
        .with_reward(QuestReward::Experience(800))
        .with_reward(QuestReward::Gold(500))
        .with_reward(QuestReward::Item(ItemKind::AmuletOfProtection, Rarity::Rare))
        .with_min_dungeon_level(7)
        .with_prerequisite(103),

        Quest::new(
            105,
            "Betrayal at the Keep",
            "One of your allies has been corrupted by the shadow. Uncover the traitor by defeating the Orc Warlord and finding evidence.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossOrcWarlord))
        .with_objective(ObjectiveType::CollectItem(ItemKind::AncientRelic, 1))
        .with_reward(QuestReward::Experience(1200))
        .with_reward(QuestReward::Gold(700))
        .with_reward(QuestReward::Item(ItemKind::LongSword, Rarity::Epic))
        .with_min_dungeon_level(10)
        .with_prerequisite(104),

        Quest::new(
            106,
            "Into the Abyss",
            "The path to the source of evil leads through the deepest dungeons. Brave the abyssal depths where demons dwell.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::ReachDungeonLevel(15))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Demon, 10))
        .with_reward(QuestReward::Experience(1500))
        .with_reward(QuestReward::Gold(1000))
        .with_reward(QuestReward::Item(ItemKind::HolyArmor, Rarity::Epic))
        .with_min_dungeon_level(12)
        .with_prerequisite(105),

        Quest::new(
            107,
            "The Dragon's Lair",
            "A great dragon guards the passage to the lower realms. Face the legendary beast and claim its hoard.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossIceDragon))
        .with_reward(QuestReward::Experience(3000))
        .with_reward(QuestReward::Gold(2000))
        .with_reward(QuestReward::Item(ItemKind::DragonArmor, Rarity::Legendary))
        .with_reward(QuestReward::Item(ItemKind::DragonScale, Rarity::Legendary))
        .with_min_dungeon_level(20)
        .with_prerequisite(106),

        Quest::new(
            108,
            "The Lich King's Domain",
            "In the crypts below the dragon's lair, an ancient lich commands an army of undead. Confront the undead lord and shatter his phylactery.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Lich, 1))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Skeleton, 20))
        .with_objective(ObjectiveType::CollectItem(ItemKind::SoulGem, 1))
        .with_reward(QuestReward::Experience(4000))
        .with_reward(QuestReward::Gold(2500))
        .with_reward(QuestReward::Item(ItemKind::VoidStaff, Rarity::Legendary))
        .with_min_dungeon_level(23)
        .with_prerequisite(107),

        Quest::new(
            109,
            "The Demon Gate",
            "A portal to the demon realm has been opened. Fight through waves of demons and seal the gate before the world is consumed.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::SurviveWaves(10))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::DemonLord, 3))
        .with_reward(QuestReward::Experience(6000))
        .with_reward(QuestReward::Gold(3500))
        .with_reward(QuestReward::Item(ItemKind::DemonSlayer, Rarity::Legendary))
        .with_min_dungeon_level(26)
        .with_prerequisite(108),

        Quest::new(
            110,
            "The Final Shadow",
            "The Demon King awaits at the bottom of the abyss. Face the ultimate evil and determine the fate of the world.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::DefeatBoss(EnemyKind::BossDemonKing))
        .with_reward(QuestReward::Experience(10000))
        .with_reward(QuestReward::Gold(5000))
        .with_reward(QuestReward::Item(ItemKind::DemonSlayer, Rarity::Mythic))
        .with_reward(QuestReward::MaxHpBonus(50))
        .with_reward(QuestReward::MaxManaBonus(50))
        .with_min_dungeon_level(30)
        .with_prerequisite(109),

        Quest::new(
            111,
            "Redemption",
            "With the Demon King fallen, the world begins to heal. Choose your ending: retire as a hero, or continue your journey into the unknown.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::TalkToNPC("Elder Mage Thandril".to_string()))
        .with_reward(QuestReward::Experience(5000))
        .with_reward(QuestReward::Gold(10000))
        .with_reward(QuestReward::Unlock("New Game Plus".to_string()))
        .with_prerequisite(110),

        Quest::new(
            112,
            "Eternal Legend",
            "Your legend lives on. Continue exploring the endless depths, facing ever-greater challenges in the post-game dungeon.",
            QuestCategory::Main,
        )
        .with_objective(ObjectiveType::ReachDungeonLevel(50))
        .with_objective(ObjectiveType::KillEnemies(1000))
        .with_reward(QuestReward::Experience(20000))
        .with_reward(QuestReward::Gold(20000))
        .with_reward(QuestReward::Item(ItemKind::AmuletOfTheGods, Rarity::Mythic))
        .with_reward(QuestReward::AttackBonus(25))
        .with_reward(QuestReward::DefenseBonus(25))
        .with_prerequisite(111),

        // =====================================================================
        // BOUNTY QUESTS (15 quests, IDs 200-214)
        // =====================================================================
        Quest::new(
            200,
            "Rat Catcher",
            "The guild has posted a bounty on the rat infestation. Clear them out.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Rat, 15))
        .with_reward(QuestReward::Experience(50))
        .with_reward(QuestReward::Gold(75))
        .with_repeatable(true),

        Quest::new(
            201,
            "Spider Slayer",
            "Venomous spiders threaten the upper tunnels. Exterminate them.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Spider, 15))
        .with_reward(QuestReward::Experience(75))
        .with_reward(QuestReward::Gold(100))
        .with_repeatable(true),

        Quest::new(
            202,
            "Goblin Bounty",
            "A bounty has been placed on goblin raiders. Bring proof of their defeat.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Goblin, 20))
        .with_reward(QuestReward::Experience(100))
        .with_reward(QuestReward::Gold(150))
        .with_min_player_level(3)
        .with_repeatable(true),

        Quest::new(
            203,
            "Skeleton Purge",
            "The restless dead walk the corridors. Put them back to rest.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Skeleton, 20))
        .with_reward(QuestReward::Experience(120))
        .with_reward(QuestReward::Gold(175))
        .with_min_dungeon_level(5)
        .with_repeatable(true),

        Quest::new(
            204,
            "Orc Warhunt",
            "Orc war parties are growing bolder. Thin their numbers.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Orc, 15))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(250))
        .with_min_dungeon_level(7)
        .with_repeatable(true),

        Quest::new(
            205,
            "Troll Trouble",
            "Trolls have been spotted in the middle depths. They must be dealt with.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Troll, 10))
        .with_reward(QuestReward::Experience(300))
        .with_reward(QuestReward::Gold(350))
        .with_min_dungeon_level(8)
        .with_repeatable(true),

        Quest::new(
            206,
            "Ghost Hunter",
            "Spectral entities haunt the abandoned wing. Banish them.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Ghost, 10))
        .with_reward(QuestReward::Experience(250))
        .with_reward(QuestReward::Gold(300))
        .with_min_dungeon_level(10)
        .with_repeatable(true),

        Quest::new(
            207,
            "Vampire Hunt",
            "Vampires prey on the weak. Track and destroy them.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Vampire, 8))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(500))
        .with_min_dungeon_level(12)
        .with_repeatable(true),

        Quest::new(
            208,
            "Demon Purge",
            "Demons have crossed into our realm. Send them back to the abyss.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Demon, 12))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(600))
        .with_min_dungeon_level(18)
        .with_repeatable(true),

        Quest::new(
            209,
            "Elemental Containment",
            "Rogue elementals are destabilizing the dungeon. Destroy them.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::FireElemental, 5))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::IceElemental, 5))
        .with_reward(QuestReward::Experience(450))
        .with_reward(QuestReward::Gold(550))
        .with_min_dungeon_level(15)
        .with_repeatable(true),

        Quest::new(
            210,
            "Wraith Warden",
            "Wraiths drift through walls, terrorizing all. Eliminate them.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Wraith, 8))
        .with_reward(QuestReward::Experience(350))
        .with_reward(QuestReward::Gold(400))
        .with_min_dungeon_level(11)
        .with_repeatable(true),

        Quest::new(
            211,
            "Wolf Pack Culling",
            "Dire wolves have formed dangerous packs. Cull their numbers.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::DireWolf, 10))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(225))
        .with_min_dungeon_level(6)
        .with_repeatable(true),

        Quest::new(
            212,
            "Golem Breaker",
            "Ancient golems guard forbidden passages. Smash them apart.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Golem, 5))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(600))
        .with_min_dungeon_level(16),

        Quest::new(
            213,
            "Hellhound Handler",
            "Hellhounds roam the fire caverns. Put them down before they spread.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Hellhound, 8))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(450))
        .with_min_dungeon_level(14)
        .with_repeatable(true),

        Quest::new(
            214,
            "Mass Extermination",
            "The dungeon is overrun. Kill everything that moves.",
            QuestCategory::Bounty,
        )
        .with_objective(ObjectiveType::KillEnemies(200))
        .with_reward(QuestReward::Experience(1000))
        .with_reward(QuestReward::Gold(1500))
        .with_reward(QuestReward::Item(ItemKind::Greatsword, Rarity::Rare))
        .with_min_player_level(10),

        // =====================================================================
        // EXPLORATION QUESTS (7 quests, IDs 300-306)
        // =====================================================================
        Quest::new(
            300,
            "Cartographer's Apprentice",
            "Map the first five floors by exploring every room.",
            QuestCategory::Exploration,
        )
        .with_objective(ObjectiveType::ExploreRooms(25))
        .with_reward(QuestReward::Experience(150))
        .with_reward(QuestReward::Gold(100))
        .with_reward(QuestReward::Item(ItemKind::ScrollMapping, Rarity::Common)),

        Quest::new(
            301,
            "Hidden Passages",
            "The dungeon holds many secrets. Find hidden rooms concealed behind illusory walls.",
            QuestCategory::Exploration,
        )
        .with_objective(ObjectiveType::FindSecretRoom(3))
        .with_reward(QuestReward::Experience(300))
        .with_reward(QuestReward::Gold(200))
        .with_reward(QuestReward::Item(ItemKind::Compass, Rarity::Rare))
        .with_min_dungeon_level(3),

        Quest::new(
            302,
            "Deep Explorer",
            "Venture into the deepest known levels of the dungeon.",
            QuestCategory::Exploration,
        )
        .with_objective(ObjectiveType::ReachDungeonLevel(20))
        .with_objective(ObjectiveType::ExploreRooms(200))
        .with_reward(QuestReward::Experience(800))
        .with_reward(QuestReward::Gold(500))
        .with_reward(QuestReward::Item(ItemKind::TeleportCrystal, Rarity::Rare))
        .with_min_dungeon_level(10),

        Quest::new(
            303,
            "Master Cartographer",
            "Explore over 500 rooms across all dungeon floors.",
            QuestCategory::Exploration,
        )
        .with_objective(ObjectiveType::ExploreRooms(500))
        .with_reward(QuestReward::Experience(1500))
        .with_reward(QuestReward::Gold(1000))
        .with_reward(QuestReward::Item(ItemKind::ScrollMapping, Rarity::Legendary))
        .with_min_player_level(10),

        Quest::new(
            304,
            "Shrine Pilgrim",
            "Seek out and use shrines scattered throughout the dungeon.",
            QuestCategory::Exploration,
        )
        .with_objective(ObjectiveType::UseShrines(5))
        .with_objective(ObjectiveType::ExploreRooms(50))
        .with_reward(QuestReward::Experience(250))
        .with_reward(QuestReward::MaxHpBonus(10))
        .with_reward(QuestReward::MaxManaBonus(10)),

        Quest::new(
            305,
            "The Secret Vault",
            "Legends tell of a hidden vault containing untold treasures. Find five secret rooms to locate it.",
            QuestCategory::Exploration,
        )
        .with_objective(ObjectiveType::FindSecretRoom(5))
        .with_objective(ObjectiveType::OpenChests(10))
        .with_reward(QuestReward::Experience(600))
        .with_reward(QuestReward::Gold(800))
        .with_reward(QuestReward::RandomItems(3, Rarity::Rare))
        .with_min_dungeon_level(8),

        Quest::new(
            306,
            "Stairway to the Abyss",
            "Descend through 30 flights of stairs, pushing ever deeper into the unknown.",
            QuestCategory::Exploration,
        )
        .with_objective(ObjectiveType::DescendStairs(30))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(400))
        .with_reward(QuestReward::DefenseBonus(5)),

        // =====================================================================
        // COLLECTION QUESTS (10 quests, IDs 400-409)
        // =====================================================================
        Quest::new(
            400,
            "Potion Hoarder",
            "Collect health potions to build a reserve for the difficult fights ahead.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::HealthPotion, 10))
        .with_reward(QuestReward::Experience(100))
        .with_reward(QuestReward::Gold(50))
        .with_reward(QuestReward::Item(ItemKind::FullRestorePotion, Rarity::Rare)),

        Quest::new(
            401,
            "Scroll Scholar",
            "Collect scrolls of various types to study their magic.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::ScrollFireball, 2))
        .with_objective(ObjectiveType::CollectItem(ItemKind::ScrollIceStorm, 2))
        .with_objective(ObjectiveType::CollectItem(ItemKind::ScrollLightning, 2))
        .with_reward(QuestReward::Experience(250))
        .with_reward(QuestReward::Gold(200))
        .with_reward(QuestReward::Item(ItemKind::ScrollMeteor, Rarity::Epic))
        .with_min_player_level(5),

        Quest::new(
            402,
            "Ring Collector",
            "Seek out rings of power scattered throughout the dungeon.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectRarityItems(Rarity::Rare, 3))
        .with_reward(QuestReward::Experience(300))
        .with_reward(QuestReward::Gold(250))
        .with_reward(QuestReward::Item(ItemKind::RingOfTheAncients, Rarity::Epic))
        .with_min_player_level(8),

        Quest::new(
            403,
            "Gold Fever",
            "Amass a fortune in gold from the dungeon's treasures.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectGold(5000))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(2000))
        .with_min_player_level(5),

        Quest::new(
            404,
            "Treasure Chest Marathon",
            "Open chests throughout the dungeon to find rare loot.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::OpenChests(25))
        .with_reward(QuestReward::Experience(300))
        .with_reward(QuestReward::RandomItems(2, Rarity::Rare)),

        Quest::new(
            405,
            "Dragon Trophies",
            "Collect dragon scales as proof of your valor against dragonkind.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::DragonScale, 3))
        .with_reward(QuestReward::Experience(800))
        .with_reward(QuestReward::Gold(1000))
        .with_reward(QuestReward::Item(ItemKind::DragonArmor, Rarity::Legendary))
        .with_min_dungeon_level(20),

        Quest::new(
            406,
            "Soul Harvester",
            "Gather soul gems from defeated powerful enemies.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::SoulGem, 5))
        .with_reward(QuestReward::Experience(600))
        .with_reward(QuestReward::Gold(800))
        .with_reward(QuestReward::Item(ItemKind::AmuletOfDeath, Rarity::Epic))
        .with_min_dungeon_level(15),

        Quest::new(
            407,
            "Relic Hunter",
            "Ancient relics hold the key to forgotten knowledge. Find them all.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::AncientRelic, 3))
        .with_reward(QuestReward::Experience(700))
        .with_reward(QuestReward::Gold(600))
        .with_reward(QuestReward::Unlock("Ancient Lore Library".to_string()))
        .with_min_dungeon_level(12),

        Quest::new(
            408,
            "Epic Collector",
            "Collect items of epic quality or above to prove your worth.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectRarityItems(Rarity::Epic, 10))
        .with_reward(QuestReward::Experience(1000))
        .with_reward(QuestReward::Gold(1500))
        .with_reward(QuestReward::RandomItems(2, Rarity::Legendary))
        .with_min_player_level(15),

        Quest::new(
            409,
            "Demon Hearts",
            "Collect the still-beating hearts of slain demons for the alchemists.",
            QuestCategory::Collection,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::DemonHeart, 5))
        .with_reward(QuestReward::Experience(900))
        .with_reward(QuestReward::Gold(1200))
        .with_reward(QuestReward::Item(ItemKind::DemonArmor, Rarity::Legendary))
        .with_min_dungeon_level(20),

        // =====================================================================
        // CRAFTING QUESTS (7 quests, IDs 500-506)
        // =====================================================================
        Quest::new(
            500,
            "Apprentice Smith",
            "Learn the basics of crafting by forging a simple dagger.",
            QuestCategory::Crafting,
        )
        .with_objective(ObjectiveType::CraftItem(ItemKind::Dagger))
        .with_reward(QuestReward::Experience(100))
        .with_reward(QuestReward::Gold(50))
        .with_reward(QuestReward::Unlock("Basic Crafting".to_string())),

        Quest::new(
            501,
            "Journeyman Armorer",
            "Craft a set of chain mail to prove your skill at the forge.",
            QuestCategory::Crafting,
        )
        .with_objective(ObjectiveType::CraftItem(ItemKind::ChainMail))
        .with_reward(QuestReward::Experience(250))
        .with_reward(QuestReward::Gold(200))
        .with_reward(QuestReward::Unlock("Intermediate Crafting".to_string()))
        .with_min_player_level(5)
        .with_prerequisite(500),

        Quest::new(
            502,
            "Master Weaponsmith",
            "Craft a greatsword using materials gathered from the dungeon depths.",
            QuestCategory::Crafting,
        )
        .with_objective(ObjectiveType::CraftItem(ItemKind::Greatsword))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(400))
        .with_reward(QuestReward::AttackBonus(5))
        .with_min_player_level(10)
        .with_prerequisite(501),

        Quest::new(
            503,
            "Enchanter's Path",
            "Enchant a weapon with magical properties at an enchanting shrine.",
            QuestCategory::Crafting,
        )
        .with_objective(ObjectiveType::EnchantItem(ItemKind::LongSword))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(300))
        .with_reward(QuestReward::Unlock("Enchanting".to_string()))
        .with_min_player_level(8),

        Quest::new(
            504,
            "Frost Forger",
            "Craft and enchant a frost blade using ice dragon materials.",
            QuestCategory::Crafting,
        )
        .with_objective(ObjectiveType::CraftItem(ItemKind::FrostBlade))
        .with_objective(ObjectiveType::EnchantItem(ItemKind::FrostBlade))
        .with_reward(QuestReward::Experience(800))
        .with_reward(QuestReward::Gold(700))
        .with_reward(QuestReward::Item(ItemKind::FrostBlade, Rarity::Legendary))
        .with_min_player_level(15)
        .with_min_dungeon_level(20),

        Quest::new(
            505,
            "Potion Brewer",
            "Brew a full restore potion using rare ingredients.",
            QuestCategory::Crafting,
        )
        .with_objective(ObjectiveType::CraftItem(ItemKind::FullRestorePotion))
        .with_reward(QuestReward::Experience(300))
        .with_reward(QuestReward::Gold(200))
        .with_reward(QuestReward::Item(ItemKind::UltimatePowerPotion, Rarity::Epic))
        .with_min_player_level(7),

        Quest::new(
            506,
            "Legendary Artificer",
            "Craft and enchant a piece of dragon armor, the pinnacle of the smithing arts.",
            QuestCategory::Crafting,
        )
        .with_objective(ObjectiveType::CraftItem(ItemKind::DragonArmor))
        .with_objective(ObjectiveType::EnchantItem(ItemKind::DragonArmor))
        .with_reward(QuestReward::Experience(2000))
        .with_reward(QuestReward::Gold(2000))
        .with_reward(QuestReward::Item(ItemKind::DragonArmor, Rarity::Mythic))
        .with_reward(QuestReward::Unlock("Legendary Crafting".to_string()))
        .with_min_player_level(20)
        .with_prerequisite(502),

        // =====================================================================
        // SURVIVAL QUESTS (5 quests, IDs 600-604)
        // =====================================================================
        Quest::new(
            600,
            "Endurance Test",
            "Survive for 500 turns without dying. A basic test of your staying power.",
            QuestCategory::Survival,
        )
        .with_objective(ObjectiveType::SurviveTurns(500))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(100))
        .with_reward(QuestReward::MaxHpBonus(10)),

        Quest::new(
            601,
            "Pacifist Run",
            "Complete 3 floors without killing a single enemy. Use stealth and cunning.",
            QuestCategory::Survival,
        )
        .with_objective(ObjectiveType::CompleteFloorWithoutKilling(3))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(300))
        .with_reward(QuestReward::Item(ItemKind::ShadowCloak, Rarity::Rare))
        .with_min_player_level(5),

        Quest::new(
            602,
            "Wave Survivor",
            "Survive 5 waves of increasingly difficult enemies in the arena.",
            QuestCategory::Survival,
        )
        .with_objective(ObjectiveType::SurviveWaves(5))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(250))
        .with_reward(QuestReward::DefenseBonus(5))
        .with_min_player_level(8),

        Quest::new(
            603,
            "Untouchable",
            "Kill 10 enemies without taking any damage. Prove your combat mastery.",
            QuestCategory::Survival,
        )
        .with_objective(ObjectiveType::KillWithoutDamage(10))
        .with_reward(QuestReward::Experience(600))
        .with_reward(QuestReward::Gold(400))
        .with_reward(QuestReward::AttackBonus(5))
        .with_min_player_level(10),

        Quest::new(
            604,
            "Eternal Survivor",
            "Survive for 5000 turns. Only the most resilient adventurers accomplish this feat.",
            QuestCategory::Survival,
        )
        .with_objective(ObjectiveType::SurviveTurns(5000))
        .with_reward(QuestReward::Experience(2000))
        .with_reward(QuestReward::Gold(1500))
        .with_reward(QuestReward::MaxHpBonus(30))
        .with_reward(QuestReward::DefenseBonus(10))
        .with_min_player_level(15),

        // =====================================================================
        // ARENA QUESTS (10 quests, IDs 700-709)
        // =====================================================================
        Quest::new(
            700,
            "Arena Initiate",
            "Enter the arena and defeat 5 opponents in single combat.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillEnemies(5))
        .with_reward(QuestReward::Experience(100))
        .with_reward(QuestReward::Gold(100))
        .with_min_player_level(3),

        Quest::new(
            701,
            "Arena Combatant",
            "Prove yourself by defeating 15 arena opponents.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillEnemies(15))
        .with_reward(QuestReward::Experience(250))
        .with_reward(QuestReward::Gold(200))
        .with_reward(QuestReward::Item(ItemKind::ShortSword, Rarity::Uncommon))
        .with_min_player_level(5)
        .with_prerequisite(700),

        Quest::new(
            702,
            "Arena Warrior",
            "Defeat 30 arena opponents including tougher enemies.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillEnemies(30))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Orc, 5))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(350))
        .with_reward(QuestReward::Item(ItemKind::ChainMail, Rarity::Rare))
        .with_min_player_level(8)
        .with_prerequisite(701),

        Quest::new(
            703,
            "Arena Veteran",
            "Survive 3 waves of arena enemies to earn the veteran's badge.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::SurviveWaves(3))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(500))
        .with_reward(QuestReward::AttackBonus(3))
        .with_min_player_level(10)
        .with_prerequisite(702),

        Quest::new(
            704,
            "Arena Champion",
            "Defeat 50 arena opponents and survive 5 waves to claim the champion title.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillEnemies(50))
        .with_objective(ObjectiveType::SurviveWaves(5))
        .with_reward(QuestReward::Experience(800))
        .with_reward(QuestReward::Gold(750))
        .with_reward(QuestReward::Item(ItemKind::PlateMail, Rarity::Epic))
        .with_min_player_level(12)
        .with_prerequisite(703),

        Quest::new(
            705,
            "The Gauntlet",
            "Defeat 20 enemies without taking damage in the arena gauntlet.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillWithoutDamage(20))
        .with_reward(QuestReward::Experience(1000))
        .with_reward(QuestReward::Gold(1000))
        .with_reward(QuestReward::Item(ItemKind::Katana, Rarity::Epic))
        .with_min_player_level(15),

        Quest::new(
            706,
            "Speed Kill Challenge",
            "Defeat 25 enemies within 100 turns in the arena speed trial.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillInTime(25, 100))
        .with_reward(QuestReward::Experience(700))
        .with_reward(QuestReward::Gold(600))
        .with_reward(QuestReward::Item(ItemKind::BootsOfSpeed, Rarity::Epic))
        .with_min_player_level(12),

        Quest::new(
            707,
            "Arena Master",
            "Defeat 100 opponents and survive 10 waves to become an arena master.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillEnemies(100))
        .with_objective(ObjectiveType::SurviveWaves(10))
        .with_reward(QuestReward::Experience(2000))
        .with_reward(QuestReward::Gold(2000))
        .with_reward(QuestReward::Item(ItemKind::Greatsword, Rarity::Legendary))
        .with_min_player_level(18)
        .with_prerequisite(704),

        Quest::new(
            708,
            "Arena Legend",
            "Kill 200 arena opponents, including elite enemies, to become a legend.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillEnemies(200))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::DemonLord, 5))
        .with_reward(QuestReward::Experience(5000))
        .with_reward(QuestReward::Gold(5000))
        .with_reward(QuestReward::Item(ItemKind::DemonSlayer, Rarity::Legendary))
        .with_reward(QuestReward::AttackBonus(10))
        .with_min_player_level(22)
        .with_prerequisite(707),

        Quest::new(
            709,
            "The Immortal",
            "Complete the ultimate arena challenge: kill 50 enemies without taking any damage.",
            QuestCategory::Arena,
        )
        .with_objective(ObjectiveType::KillWithoutDamage(50))
        .with_reward(QuestReward::Experience(10000))
        .with_reward(QuestReward::Gold(10000))
        .with_reward(QuestReward::Item(ItemKind::CrownOfKings, Rarity::Mythic))
        .with_reward(QuestReward::MaxHpBonus(50))
        .with_reward(QuestReward::AttackBonus(15))
        .with_reward(QuestReward::DefenseBonus(15))
        .with_min_player_level(25)
        .with_prerequisite(708),

        // =====================================================================
        // GUILD QUESTS (10 quests, IDs 800-809)
        // =====================================================================
        Quest::new(
            800,
            "Warriors Guild: Initiation",
            "The Warriors Guild demands you prove your strength. Slay 20 enemies with melee weapons.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::KillEnemies(20))
        .with_objective(ObjectiveType::DealDamage(500))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(150))
        .with_reward(QuestReward::Unlock("Warriors Guild Rank 1".to_string()))
        .with_min_player_level(3),

        Quest::new(
            801,
            "Warriors Guild: Champion",
            "Become a champion of the Warriors Guild by defeating powerful foes.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Troll, 5))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Orc, 10))
        .with_objective(ObjectiveType::DealDamage(5000))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(400))
        .with_reward(QuestReward::AttackBonus(5))
        .with_reward(QuestReward::Unlock("Warriors Guild Rank 2".to_string()))
        .with_min_player_level(10)
        .with_prerequisite(800),

        Quest::new(
            802,
            "Mages Guild: Apprentice",
            "The Mages Guild requires you to demonstrate magical aptitude. Use skills extensively.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::UseSkills(30))
        .with_objective(ObjectiveType::CollectItem(ItemKind::ManaPotion, 5))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(150))
        .with_reward(QuestReward::MaxManaBonus(15))
        .with_reward(QuestReward::Unlock("Mages Guild Rank 1".to_string()))
        .with_min_player_level(3),

        Quest::new(
            803,
            "Mages Guild: Archmage",
            "Master the arcane arts to earn the title of Archmage.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::UseSkills(100))
        .with_objective(ObjectiveType::DealDamage(10000))
        .with_reward(QuestReward::Experience(800))
        .with_reward(QuestReward::Gold(600))
        .with_reward(QuestReward::MaxManaBonus(30))
        .with_reward(QuestReward::Item(ItemKind::VoidStaff, Rarity::Legendary))
        .with_reward(QuestReward::Unlock("Mages Guild Rank 2".to_string()))
        .with_min_player_level(15)
        .with_prerequisite(802),

        Quest::new(
            804,
            "Thieves Guild: Pickpocket",
            "The Thieves Guild values stealth and cunning. Open chests and find secret rooms.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::OpenChests(15))
        .with_objective(ObjectiveType::FindSecretRoom(2))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(300))
        .with_reward(QuestReward::Unlock("Thieves Guild Rank 1".to_string()))
        .with_min_player_level(3),

        Quest::new(
            805,
            "Thieves Guild: Shadow Master",
            "Become a master thief by completing floors without killing and amassing gold.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::CompleteFloorWithoutKilling(5))
        .with_objective(ObjectiveType::CollectGold(10000))
        .with_objective(ObjectiveType::FindSecretRoom(10))
        .with_reward(QuestReward::Experience(800))
        .with_reward(QuestReward::Gold(2000))
        .with_reward(QuestReward::Item(ItemKind::ShadowCloak, Rarity::Legendary))
        .with_reward(QuestReward::Unlock("Thieves Guild Rank 2".to_string()))
        .with_min_player_level(15)
        .with_prerequisite(804),

        Quest::new(
            806,
            "Hunters Guild: Tracker",
            "The Hunters Guild wants you to track and eliminate specific creatures.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Wolf, 10))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Spider, 10))
        .with_objective(ObjectiveType::ExploreRooms(50))
        .with_reward(QuestReward::Experience(250))
        .with_reward(QuestReward::Gold(200))
        .with_reward(QuestReward::Unlock("Hunters Guild Rank 1".to_string()))
        .with_min_player_level(5),

        Quest::new(
            807,
            "Hunters Guild: Beast Master",
            "Hunt the most dangerous beasts in the dungeon to earn the Beast Master title.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::DireWolf, 10))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::CaveBear, 5))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::TreeEnt, 5))
        .with_reward(QuestReward::Experience(600))
        .with_reward(QuestReward::Gold(500))
        .with_reward(QuestReward::Item(ItemKind::Bow, Rarity::Epic))
        .with_reward(QuestReward::Unlock("Hunters Guild Rank 2".to_string()))
        .with_min_player_level(12)
        .with_prerequisite(806),

        Quest::new(
            808,
            "Alchemists Guild: Ingredient Gatherer",
            "Gather raw materials and potions for the Alchemists Guild.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::HealthPotion, 5))
        .with_objective(ObjectiveType::CollectItem(ItemKind::ManaPotion, 5))
        .with_objective(ObjectiveType::CollectItem(ItemKind::Mushrooms, 3))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(150))
        .with_reward(QuestReward::Unlock("Alchemists Guild Rank 1".to_string()))
        .with_min_player_level(3),

        Quest::new(
            809,
            "Alchemists Guild: Grand Alchemist",
            "Master the art of alchemy by crafting the most powerful potions.",
            QuestCategory::Guild,
        )
        .with_objective(ObjectiveType::CraftItem(ItemKind::FullRestorePotion))
        .with_objective(ObjectiveType::CraftItem(ItemKind::UltimatePowerPotion))
        .with_objective(ObjectiveType::CollectItem(ItemKind::SoulGem, 3))
        .with_reward(QuestReward::Experience(1000))
        .with_reward(QuestReward::Gold(800))
        .with_reward(QuestReward::Item(ItemKind::UltimatePowerPotion, Rarity::Legendary))
        .with_reward(QuestReward::Unlock("Alchemists Guild Rank 2".to_string()))
        .with_min_player_level(18)
        .with_prerequisite(808),

        // =====================================================================
        // ESCORT QUESTS (IDs 900-902)
        // =====================================================================
        Quest::new(
            900,
            "Escort the Merchant",
            "A traveling merchant needs safe passage through the upper dungeon floors. Protect them from harm.",
            QuestCategory::Escort,
        )
        .with_objective(ObjectiveType::EscortNPC("Merchant Aldwin".to_string()))
        .with_objective(ObjectiveType::DescendStairs(3))
        .with_reward(QuestReward::Experience(300))
        .with_reward(QuestReward::Gold(500))
        .with_reward(QuestReward::Item(ItemKind::RingOfProtection, Rarity::Rare))
        .with_min_player_level(5),

        Quest::new(
            901,
            "Escort the Scholar",
            "An elderly scholar seeks ancient texts in the deep dungeon. Keep them alive while they study.",
            QuestCategory::Escort,
        )
        .with_objective(ObjectiveType::EscortNPC("Scholar Miriel".to_string()))
        .with_objective(ObjectiveType::ExploreRooms(20))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(400))
        .with_reward(QuestReward::SkillPoint(2))
        .with_min_player_level(10),

        Quest::new(
            902,
            "Escort the Princess",
            "The kidnapped princess must be brought to safety through dangerous territory.",
            QuestCategory::Escort,
        )
        .with_objective(ObjectiveType::EscortNPC("Princess Lyria".to_string()))
        .with_objective(ObjectiveType::DescendStairs(5))
        .with_objective(ObjectiveType::KillEnemies(20))
        .with_reward(QuestReward::Experience(1000))
        .with_reward(QuestReward::Gold(2000))
        .with_reward(QuestReward::Item(ItemKind::CrownOfKings, Rarity::Epic))
        .with_min_player_level(15),

        // =====================================================================
        // RESCUE QUESTS (IDs 910-912)
        // =====================================================================
        Quest::new(
            910,
            "Rescue the Prisoner",
            "A prisoner is being held captive by goblins on the lower floors. Free them.",
            QuestCategory::Rescue,
        )
        .with_objective(ObjectiveType::RescueNPC("Prisoner Gareth".to_string()))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Goblin, 10))
        .with_reward(QuestReward::Experience(300))
        .with_reward(QuestReward::Gold(250))
        .with_reward(QuestReward::Item(ItemKind::Key, Rarity::Uncommon))
        .with_min_dungeon_level(3),

        Quest::new(
            911,
            "Save the Healer",
            "A healer has been captured by undead forces. Rescue them before it is too late.",
            QuestCategory::Rescue,
        )
        .with_objective(ObjectiveType::RescueNPC("Healer Anara".to_string()))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Skeleton, 15))
        .with_reward(QuestReward::Experience(500))
        .with_reward(QuestReward::Gold(400))
        .with_reward(QuestReward::MaxHpBonus(20))
        .with_min_dungeon_level(8),

        Quest::new(
            912,
            "Free the Archmage",
            "The legendary Archmage has been imprisoned by demons. Break through their forces and free him.",
            QuestCategory::Rescue,
        )
        .with_objective(ObjectiveType::RescueNPC("Archmage Zorath".to_string()))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Demon, 15))
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::DemonLord, 2))
        .with_reward(QuestReward::Experience(1500))
        .with_reward(QuestReward::Gold(1200))
        .with_reward(QuestReward::Item(ItemKind::Staff, Rarity::Legendary))
        .with_reward(QuestReward::MaxManaBonus(30))
        .with_min_dungeon_level(18),

        // =====================================================================
        // MYSTERY QUESTS (IDs 920-922)
        // =====================================================================
        Quest::new(
            920,
            "The Missing Adventurers",
            "Several adventurers went into the dungeon and never returned. Find clues about their fate.",
            QuestCategory::Mystery,
        )
        .with_objective(ObjectiveType::FindSecretRoom(2))
        .with_objective(ObjectiveType::ExploreRooms(30))
        .with_objective(ObjectiveType::CollectItem(ItemKind::AncientRelic, 1))
        .with_reward(QuestReward::Experience(400))
        .with_reward(QuestReward::Gold(300))
        .with_reward(QuestReward::Item(ItemKind::ScrollIdentify, Rarity::Rare))
        .with_min_dungeon_level(5),

        Quest::new(
            921,
            "The Cursed Artifact",
            "A cursed artifact is causing monsters to become more aggressive. Find and neutralize it.",
            QuestCategory::Mystery,
        )
        .with_objective(ObjectiveType::FindSecretRoom(3))
        .with_objective(ObjectiveType::CollectItem(ItemKind::SoulGem, 2))
        .with_objective(ObjectiveType::TalkToNPC("Spirit of the Vault".to_string()))
        .with_reward(QuestReward::Experience(700))
        .with_reward(QuestReward::Gold(600))
        .with_reward(QuestReward::Item(ItemKind::AmuletOfPower, Rarity::Epic))
        .with_min_dungeon_level(12),

        Quest::new(
            922,
            "The Ancient Prophecy",
            "Decipher an ancient prophecy by finding relics and speaking with dungeon spirits.",
            QuestCategory::Mystery,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::AncientRelic, 5))
        .with_objective(ObjectiveType::TalkToNPC("Oracle of the Deep".to_string()))
        .with_objective(ObjectiveType::FindSecretRoom(5))
        .with_reward(QuestReward::Experience(1500))
        .with_reward(QuestReward::Gold(1200))
        .with_reward(QuestReward::Item(ItemKind::AmuletOfTheGods, Rarity::Legendary))
        .with_reward(QuestReward::Unlock("Prophecy Lore".to_string()))
        .with_min_dungeon_level(20),

        // =====================================================================
        // ADDITIONAL SIDE QUESTS for 100+ total (IDs 950-959)
        // =====================================================================
        Quest::new(
            950,
            "The Bat Cave",
            "A colony of bats has infested the upper levels. Clear them out.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Bat, 20))
        .with_reward(QuestReward::Experience(80))
        .with_reward(QuestReward::Gold(60)),

        Quest::new(
            951,
            "Mushroom Forager",
            "Gather edible mushrooms from the dungeon floors for the camp cook.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::Mushrooms, 5))
        .with_reward(QuestReward::Experience(100))
        .with_reward(QuestReward::Gold(75))
        .with_reward(QuestReward::Item(ItemKind::HeartyStew, Rarity::Uncommon)),

        Quest::new(
            952,
            "Torch Bearer",
            "Carry torches through dark passages to light the way for others.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::Torch, 5))
        .with_objective(ObjectiveType::ExploreRooms(20))
        .with_reward(QuestReward::Experience(120))
        .with_reward(QuestReward::Gold(80))
        .with_reward(QuestReward::Item(ItemKind::VisionPotion, Rarity::Uncommon)),

        Quest::new(
            953,
            "Key Collector",
            "Find keys to unlock sealed chambers throughout the dungeon.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::Key, 5))
        .with_reward(QuestReward::Experience(150))
        .with_reward(QuestReward::Gold(100))
        .with_reward(QuestReward::RandomItems(1, Rarity::Rare)),

        Quest::new(
            954,
            "Slime Cleanup",
            "Slimes are clogging the dungeon corridors. Dissolve them.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Slime, 15))
        .with_reward(QuestReward::Experience(100))
        .with_reward(QuestReward::Gold(90)),

        Quest::new(
            955,
            "Bomb Expert",
            "Collect and use bombs to clear blocked passages.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::CollectItem(ItemKind::Bomb, 5))
        .with_reward(QuestReward::Experience(130))
        .with_reward(QuestReward::Gold(100)),

        Quest::new(
            956,
            "Iron Will",
            "Deal 2000 damage to prove your combat prowess.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::DealDamage(2000))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::AttackBonus(3))
        .with_min_player_level(5),

        Quest::new(
            957,
            "Mana Adept",
            "Use skills 50 times to hone your magical abilities.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::UseSkills(50))
        .with_reward(QuestReward::Experience(250))
        .with_reward(QuestReward::MaxManaBonus(15))
        .with_min_player_level(5),

        Quest::new(
            958,
            "Kobold Extermination",
            "Kobolds have been sabotaging dungeon infrastructure. Remove them.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::KillEnemyType(EnemyKind::Kobold, 15))
        .with_reward(QuestReward::Experience(90))
        .with_reward(QuestReward::Gold(85)),

        Quest::new(
            959,
            "The Deliverer",
            "Deliver a healing potion to a wounded NPC deep in the dungeon.",
            QuestCategory::Side,
        )
        .with_objective(ObjectiveType::DeliverItem(ItemKind::HealthPotion, "Wounded Knight Beron".to_string()))
        .with_reward(QuestReward::Experience(200))
        .with_reward(QuestReward::Gold(150))
        .with_reward(QuestReward::Item(ItemKind::IronShield, Rarity::Rare))
        .with_min_dungeon_level(5),
    ]
}

/// Generates a procedural quest based on the player's level and RNG.
/// Creates random quests with appropriate difficulty and rewards scaled to level.
pub fn generate_procedural_quest(player_level: u32, rng: &mut impl rand::Rng) -> Quest {
    use rand::Rng;

    // Generate a unique ID in the procedural range (10000+)
    let id: u32 = 10000 + rng.gen_range(0..90000);

    // Determine quest category based on weighted random selection
    let category_roll: u32 = rng.gen_range(0..100);
    let category = if category_roll < 25 {
        QuestCategory::Bounty
    } else if category_roll < 40 {
        QuestCategory::Exploration
    } else if category_roll < 55 {
        QuestCategory::Collection
    } else if category_roll < 65 {
        QuestCategory::Survival
    } else if category_roll < 75 {
        QuestCategory::Arena
    } else if category_roll < 85 {
        QuestCategory::Crafting
    } else if category_roll < 92 {
        QuestCategory::Side
    } else {
        QuestCategory::Daily
    };

    // Scale values based on player level
    let level_multiplier = player_level.max(1);
    let base_xp = 50 * level_multiplier;
    let base_gold = 30 * level_multiplier;

    // Generate quest based on category
    let (name, description, objective) = match category {
        QuestCategory::Bounty => {
            let enemy_templates: Vec<(&str, &str, EnemyKind, u32)> = vec![
                ("Pest Cleanup", "Eliminate the pests infesting the area.", EnemyKind::Rat, 10 + player_level),
                ("Spider Scourge", "Clear out the spider nests.", EnemyKind::Spider, 8 + player_level),
                ("Goblin Raiders", "Stop the goblin raiding parties.", EnemyKind::Goblin, 10 + player_level / 2),
                ("Undead Uprising", "Put the restless dead back to rest.", EnemyKind::Skeleton, 10 + player_level / 2),
                ("Orc Incursion", "Push back the orc invasion force.", EnemyKind::Orc, 5 + player_level / 3),
                ("Spectral Threat", "Banish the ghosts haunting the corridors.", EnemyKind::Ghost, 5 + player_level / 3),
                ("Demon Infestation", "Demons have breached the barrier. Destroy them.", EnemyKind::Demon, 3 + player_level / 5),
                ("Troll Problem", "Trolls are blocking the passages. Remove them.", EnemyKind::Troll, 5 + player_level / 4),
            ];
            let idx = rng.gen_range(0..enemy_templates.len());
            let (name, desc, kind, count) = enemy_templates[idx].clone();
            (
                name.to_string(),
                desc.to_string(),
                ObjectiveType::KillEnemyType(kind, count),
            )
        }
        QuestCategory::Exploration => {
            let explore_roll = rng.gen_range(0..3);
            match explore_roll {
                0 => (
                    "Uncharted Territory".to_string(),
                    format!("Explore {} undiscovered rooms.", 15 + player_level * 2),
                    ObjectiveType::ExploreRooms(15 + player_level * 2),
                ),
                1 => (
                    "Hidden Depths".to_string(),
                    format!("Find {} secret rooms concealed in the dungeon.", 1 + player_level / 5),
                    ObjectiveType::FindSecretRoom(1 + player_level / 5),
                ),
                _ => (
                    "Deeper Still".to_string(),
                    format!("Descend {} flights of stairs.", 5 + player_level / 2),
                    ObjectiveType::DescendStairs(5 + player_level / 2),
                ),
            }
        }
        QuestCategory::Collection => {
            let collect_roll = rng.gen_range(0..4);
            match collect_roll {
                0 => (
                    "Treasure Seeker".to_string(),
                    format!("Open {} treasure chests.", 5 + player_level),
                    ObjectiveType::OpenChests(5 + player_level),
                ),
                1 => (
                    "Gold Rush".to_string(),
                    format!("Accumulate {} gold.", 200 * level_multiplier),
                    ObjectiveType::CollectGold(200 * level_multiplier),
                ),
                2 => (
                    "Rare Finder".to_string(),
                    format!("Find {} rare items.", 1 + player_level / 5),
                    ObjectiveType::CollectRarityItems(Rarity::Rare, 1 + player_level / 5),
                ),
                _ => (
                    "Potion Stockpile".to_string(),
                    format!("Collect {} health potions.", 3 + player_level / 3),
                    ObjectiveType::CollectItem(ItemKind::HealthPotion, 3 + player_level / 3),
                ),
            }
        }
        QuestCategory::Survival => {
            let survival_roll = rng.gen_range(0..3);
            match survival_roll {
                0 => (
                    "Endurance Challenge".to_string(),
                    format!("Survive for {} turns.", 200 + player_level * 50),
                    ObjectiveType::SurviveTurns(200 + player_level * 50),
                ),
                1 => (
                    "Flawless Fighter".to_string(),
                    format!("Kill {} enemies without taking damage.", 3 + player_level / 3),
                    ObjectiveType::KillWithoutDamage(3 + player_level / 3),
                ),
                _ => (
                    "Wave Defense".to_string(),
                    format!("Survive {} waves of enemies.", 2 + player_level / 5),
                    ObjectiveType::SurviveWaves(2 + player_level / 5),
                ),
            }
        }
        QuestCategory::Arena => {
            let arena_count = 10 + player_level * 2;
            (
                "Arena Challenge".to_string(),
                format!("Defeat {} opponents in the arena.", arena_count),
                ObjectiveType::KillEnemies(arena_count),
            )
        }
        QuestCategory::Crafting => {
            let craft_items = vec![
                ItemKind::Dagger,
                ItemKind::ShortSword,
                ItemKind::LongSword,
                ItemKind::ChainMail,
                ItemKind::LeatherArmor,
                ItemKind::IronShield,
            ];
            let idx = rng.gen_range(0..craft_items.len());
            let item = craft_items[idx].clone();
            (
                "Forge Request".to_string(),
                format!("Craft a {} for the guild.", item.name()),
                ObjectiveType::CraftItem(item),
            )
        }
        _ => {
            // Side / Daily / other categories - general kill quest
            let kill_count = 10 + player_level * 2;
            (
                "General Contract".to_string(),
                format!("Eliminate {} hostile creatures.", kill_count),
                ObjectiveType::KillEnemies(kill_count),
            )
        }
    };

    // Build the quest with scaled rewards
    let mut quest = Quest::new(id, &name, &description, category)
        .with_objective(objective)
        .with_reward(QuestReward::Experience(base_xp))
        .with_reward(QuestReward::Gold(base_gold))
        .with_min_player_level(player_level.saturating_sub(2).max(1));

    // Add bonus item reward for higher-level quests
    if player_level >= 5 {
        let rarity = if player_level >= 25 {
            Rarity::Legendary
        } else if player_level >= 18 {
            Rarity::Epic
        } else if player_level >= 10 {
            Rarity::Rare
        } else {
            Rarity::Uncommon
        };

        let bonus_items = vec![
            ItemKind::HealthPotion,
            ItemKind::ManaPotion,
            ItemKind::ScrollFireball,
            ItemKind::RingOfStrength,
            ItemKind::AmuletOfHealth,
            ItemKind::BootsOfSpeed,
            ItemKind::IronShield,
            ItemKind::LongSword,
        ];
        let idx = rng.gen_range(0..bonus_items.len());
        quest = quest.with_reward(QuestReward::Item(bonus_items[idx].clone(), rarity));
    }

    // Daily quests are repeatable
    if category == QuestCategory::Daily {
        quest = quest.with_repeatable(true);
    }

    // Add turn limit for some quests
    if rng.gen_range(0..100) < 20 {
        quest = quest.with_turn_limit(300 + player_level * 50);
    }

    quest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_tracker_creation() {
        let tracker = QuestTracker::new();
        assert!(!tracker.quest_definitions.is_empty());
    }

    #[test]
    fn test_start_quest() {
        let mut tracker = QuestTracker::new();
        let result = tracker.start_quest(1);
        assert!(result.is_some());
        assert!(tracker.active_quests.contains_key(&1));
    }

    #[test]
    fn test_kill_progress() {
        let mut tracker = QuestTracker::new();
        tracker.start_quest(1); // First Blood - kill 1 enemy

        let messages = tracker.on_enemy_killed(EnemyKind::Rat);
        assert!(!messages.is_empty());

        let active = tracker.active_quests.get(&1).unwrap();
        assert_eq!(active.state, QuestState::ReadyToComplete);
    }

    #[test]
    fn test_quest_completion() {
        let mut tracker = QuestTracker::new();
        tracker.start_quest(1);
        tracker.on_enemy_killed(EnemyKind::Rat);

        let rewards = tracker.complete_quest(1);
        assert!(rewards.is_some());
        assert!(tracker.completed_quests.contains(&1));
    }

    #[test]
    fn test_objective_progress() {
        let mut progress = ObjectiveProgress::new(ObjectiveType::KillEnemies(5));
        assert_eq!(progress.percentage(), 0);

        progress.update(2);
        assert_eq!(progress.current, 2);
        assert_eq!(progress.percentage(), 40);

        progress.update(3);
        assert!(progress.completed);
        assert_eq!(progress.percentage(), 100);
    }
}

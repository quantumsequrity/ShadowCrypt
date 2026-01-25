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
            | Self::DescendStairs(n) => *n,
            Self::DefeatBoss(_) => 1,
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
        }
    }

    /// Returns color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Starter => 5,     // Green
            Self::Main => 11,       // Yellow
            Self::Side => 7,        // Blue
            Self::Daily => 9,       // Cyan
            Self::Achievement => 13, // Magenta
            Self::Hidden => 4,      // Purple
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
            if let Some(quest) = self.get_quest(*quest_id) {
                if quest.turn_limit > 0 && active.turns_elapsed >= quest.turn_limit {
                    if !active.all_objectives_completed() {
                        failed_quests.push(*quest_id);
                        messages.push(format!("Quest failed: {} (time limit exceeded)", quest.name));
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

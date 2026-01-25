// Quest System for ShadowCrypt Roguelike
// Comprehensive quest system with 100+ quests, procedural generation, and NPC integration

use crossterm::style::Color;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::{ItemKind, Rarity, EnemyKind, CharacterClass};

// ============================================================================
// QUEST TYPE ENUM
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum QuestType {
    Main,           // Story-critical quests
    Side,           // Optional side quests
    Daily,          // Reset daily, random rewards
    Weekly,         // Larger weekly challenges
    Bounty,         // Kill specific targets
    Exploration,    // Discover locations
    Collection,     // Gather items
    Escort,         // Protect NPCs
    Rescue,         // Save prisoners/NPCs
    Delivery,       // Bring items to NPCs
    Crafting,       // Create specific items
    Mystery,        // Solve puzzles/find clues
    Arena,          // Combat challenges
    Guild,          // Faction-specific quests
    Legendary,      // Epic multi-part quests
}

impl QuestType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Main => "Main Quest",
            Self::Side => "Side Quest",
            Self::Daily => "Daily Quest",
            Self::Weekly => "Weekly Challenge",
            Self::Bounty => "Bounty",
            Self::Exploration => "Exploration",
            Self::Collection => "Collection",
            Self::Escort => "Escort Mission",
            Self::Rescue => "Rescue Mission",
            Self::Delivery => "Delivery",
            Self::Crafting => "Crafting Quest",
            Self::Mystery => "Mystery",
            Self::Arena => "Arena Challenge",
            Self::Guild => "Guild Quest",
            Self::Legendary => "Legendary Quest",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Main => Color::Yellow,
            Self::Side => Color::White,
            Self::Daily => Color::Green,
            Self::Weekly => Color::Blue,
            Self::Bounty => Color::Red,
            Self::Exploration => Color::Cyan,
            Self::Collection => Color::Magenta,
            Self::Escort => Color::DarkYellow,
            Self::Rescue => Color::DarkCyan,
            Self::Delivery => Color::Grey,
            Self::Crafting => Color::DarkMagenta,
            Self::Mystery => Color::DarkBlue,
            Self::Arena => Color::DarkRed,
            Self::Guild => Color::DarkGreen,
            Self::Legendary => Color::Rgb { r: 255, g: 215, b: 0 }, // Gold
        }
    }

    pub fn icon(&self) -> char {
        match self {
            Self::Main => '!',
            Self::Side => '?',
            Self::Daily => 'D',
            Self::Weekly => 'W',
            Self::Bounty => 'X',
            Self::Exploration => 'E',
            Self::Collection => 'C',
            Self::Escort => 'P',
            Self::Rescue => 'R',
            Self::Delivery => 'D',
            Self::Crafting => 'F',
            Self::Mystery => 'M',
            Self::Arena => 'A',
            Self::Guild => 'G',
            Self::Legendary => 'L',
        }
    }
}

// ============================================================================
// QUEST DIFFICULTY
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum QuestDifficulty {
    Trivial,
    Easy,
    Normal,
    Hard,
    VeryHard,
    Nightmare,
    Legendary,
}

impl QuestDifficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Trivial => "Trivial",
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::VeryHard => "Very Hard",
            Self::Nightmare => "Nightmare",
            Self::Legendary => "Legendary",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Trivial => Color::Grey,
            Self::Easy => Color::Green,
            Self::Normal => Color::White,
            Self::Hard => Color::Yellow,
            Self::VeryHard => Color::Magenta,
            Self::Nightmare => Color::Red,
            Self::Legendary => Color::Rgb { r: 255, g: 215, b: 0 },
        }
    }

    pub fn xp_multiplier(&self) -> f32 {
        match self {
            Self::Trivial => 0.5,
            Self::Easy => 0.75,
            Self::Normal => 1.0,
            Self::Hard => 1.5,
            Self::VeryHard => 2.0,
            Self::Nightmare => 3.0,
            Self::Legendary => 5.0,
        }
    }

    pub fn gold_multiplier(&self) -> f32 {
        match self {
            Self::Trivial => 0.5,
            Self::Easy => 0.8,
            Self::Normal => 1.0,
            Self::Hard => 1.5,
            Self::VeryHard => 2.5,
            Self::Nightmare => 4.0,
            Self::Legendary => 8.0,
        }
    }

    pub fn recommended_level(&self) -> u32 {
        match self {
            Self::Trivial => 1,
            Self::Easy => 3,
            Self::Normal => 5,
            Self::Hard => 10,
            Self::VeryHard => 15,
            Self::Nightmare => 20,
            Self::Legendary => 25,
        }
    }

    pub fn from_level(level: u32) -> Self {
        match level {
            1..=2 => Self::Trivial,
            3..=5 => Self::Easy,
            6..=9 => Self::Normal,
            10..=14 => Self::Hard,
            15..=19 => Self::VeryHard,
            20..=24 => Self::Nightmare,
            _ => Self::Legendary,
        }
    }
}

// ============================================================================
// QUEST OBJECTIVE TYPES
// ============================================================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ObjectiveType {
    // Kill objectives
    KillEnemyType(EnemyKind, u32),           // Kill X of enemy type
    KillAnyEnemy(u32),                        // Kill X enemies total
    KillBoss(Option<EnemyKind>),              // Kill specific or any boss
    KillWithoutDamage(u32),                   // Kill X enemies without taking damage
    KillInTime(u32, u32),                     // Kill X enemies within Y turns

    // Collection objectives
    CollectItem(ItemKind, u32),               // Collect X of item
    CollectGold(u32),                         // Collect X gold
    CollectRarity(Rarity, u32),               // Collect X items of rarity
    CollectAnyItems(u32),                     // Collect X items total

    // Exploration objectives
    ReachFloor(u32),                          // Reach dungeon floor X
    ExploreRooms(u32),                        // Explore X rooms
    FindSecretRoom,                           // Find a secret room
    DiscoverLocation(String),                 // Find specific location

    // Interaction objectives
    TalkToNPC(String),                        // Talk to specific NPC
    DeliverItem(ItemKind, String),            // Deliver item to NPC
    EscortNPC(String),                        // Escort NPC to safety
    RescueNPC(String),                        // Rescue trapped NPC

    // Skill objectives
    UseSkill(u32),                            // Use skills X times
    CraftItem(ItemKind),                      // Craft specific item
    EnchantItem(u32),                         // Enchant X items
    LevelUp(u32),                             // Reach level X

    // Survival objectives
    SurviveTurns(u32),                        // Survive X turns
    SurviveWithoutHealing(u32),               // Survive X turns without healing
    ReachFloorWithHP(u32, u32),               // Reach floor X with Y% HP

    // Special objectives
    DefeatBossWithinTurns(u32),               // Beat boss within X turns
    CompleteFloorWithoutKilling,              // Pacifist run on floor
    FindAllSecrets(u32),                      // Find X secrets on floor
    OpenAllChests(u32),                       // Open X chests

    // Composite objectives
    Custom(String),                           // Custom description
}

impl ObjectiveType {
    pub fn description(&self) -> String {
        match self {
            Self::KillEnemyType(kind, count) => format!("Kill {} {}", count, kind.name()),
            Self::KillAnyEnemy(count) => format!("Kill {} enemies", count),
            Self::KillBoss(Some(kind)) => format!("Defeat the {}", kind.name()),
            Self::KillBoss(None) => "Defeat a boss".to_string(),
            Self::KillWithoutDamage(count) => format!("Kill {} enemies without taking damage", count),
            Self::KillInTime(kills, turns) => format!("Kill {} enemies within {} turns", kills, turns),

            Self::CollectItem(kind, count) => format!("Collect {} {}", count, kind.name()),
            Self::CollectGold(amount) => format!("Collect {} gold", amount),
            Self::CollectRarity(rarity, count) => format!("Find {} {} items", count, rarity.prefix().trim()),
            Self::CollectAnyItems(count) => format!("Collect {} items", count),

            Self::ReachFloor(floor) => format!("Reach dungeon floor {}", floor),
            Self::ExploreRooms(count) => format!("Explore {} rooms", count),
            Self::FindSecretRoom => "Find a secret room".to_string(),
            Self::DiscoverLocation(name) => format!("Discover {}", name),

            Self::TalkToNPC(name) => format!("Speak with {}", name),
            Self::DeliverItem(kind, name) => format!("Deliver {} to {}", kind.name(), name),
            Self::EscortNPC(name) => format!("Escort {} to safety", name),
            Self::RescueNPC(name) => format!("Rescue {}", name),

            Self::UseSkill(count) => format!("Use skills {} times", count),
            Self::CraftItem(kind) => format!("Craft a {}", kind.name()),
            Self::EnchantItem(count) => format!("Enchant {} items", count),
            Self::LevelUp(level) => format!("Reach level {}", level),

            Self::SurviveTurns(turns) => format!("Survive for {} turns", turns),
            Self::SurviveWithoutHealing(turns) => format!("Survive {} turns without healing", turns),
            Self::ReachFloorWithHP(floor, hp_percent) => format!("Reach floor {} with {}% HP", floor, hp_percent),

            Self::DefeatBossWithinTurns(turns) => format!("Defeat the boss within {} turns", turns),
            Self::CompleteFloorWithoutKilling => "Complete floor without killing".to_string(),
            Self::FindAllSecrets(count) => format!("Find all {} secrets", count),
            Self::OpenAllChests(count) => format!("Open {} chests", count),

            Self::Custom(desc) => desc.clone(),
        }
    }
}

// ============================================================================
// QUEST OBJECTIVE
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestObjective {
    pub objective_type: ObjectiveType,
    pub current_progress: u32,
    pub required_progress: u32,
    pub completed: bool,
    pub optional: bool,
    pub bonus_reward: Option<QuestReward>,
}

impl QuestObjective {
    pub fn new(objective_type: ObjectiveType, required: u32) -> Self {
        Self {
            objective_type,
            current_progress: 0,
            required_progress: required,
            completed: false,
            optional: false,
            bonus_reward: None,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn with_bonus(mut self, reward: QuestReward) -> Self {
        self.bonus_reward = Some(reward);
        self
    }

    pub fn update_progress(&mut self, amount: u32) -> bool {
        if !self.completed {
            self.current_progress = (self.current_progress + amount).min(self.required_progress);
            if self.current_progress >= self.required_progress {
                self.completed = true;
                return true;
            }
        }
        false
    }

    pub fn progress_string(&self) -> String {
        if self.required_progress == 1 {
            if self.completed { "Complete".to_string() } else { "Incomplete".to_string() }
        } else {
            format!("{}/{}", self.current_progress, self.required_progress)
        }
    }

    pub fn description(&self) -> String {
        let status = if self.optional { "[Optional] " } else { "" };
        format!("{}{}", status, self.objective_type.description())
    }
}

// ============================================================================
// QUEST REWARD
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestReward {
    pub xp: u32,
    pub gold: u32,
    pub items: Vec<(ItemKind, Rarity)>,
    pub reputation: HashMap<String, i32>,
    pub skills: Vec<String>,
    pub stat_bonuses: Option<(i32, i32, i32, i32, i32)>, // hp, atk, def, mana, spd
    pub unlock_quest: Option<u32>,
    pub unlock_area: Option<String>,
    pub title: Option<String>,
}

impl QuestReward {
    pub fn new() -> Self {
        Self {
            xp: 0,
            gold: 0,
            items: Vec::new(),
            reputation: HashMap::new(),
            skills: Vec::new(),
            stat_bonuses: None,
            unlock_quest: None,
            unlock_area: None,
            title: None,
        }
    }

    pub fn xp(mut self, amount: u32) -> Self {
        self.xp = amount;
        self
    }

    pub fn gold(mut self, amount: u32) -> Self {
        self.gold = amount;
        self
    }

    pub fn item(mut self, kind: ItemKind, rarity: Rarity) -> Self {
        self.items.push((kind, rarity));
        self
    }

    pub fn reputation(mut self, faction: &str, amount: i32) -> Self {
        self.reputation.insert(faction.to_string(), amount);
        self
    }

    pub fn skill(mut self, skill_name: &str) -> Self {
        self.skills.push(skill_name.to_string());
        self
    }

    pub fn stats(mut self, hp: i32, atk: i32, def: i32, mana: i32, spd: i32) -> Self {
        self.stat_bonuses = Some((hp, atk, def, mana, spd));
        self
    }

    pub fn unlocks_quest(mut self, quest_id: u32) -> Self {
        self.unlock_quest = Some(quest_id);
        self
    }

    pub fn unlocks_area(mut self, area: &str) -> Self {
        self.unlock_area = Some(area.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn description(&self) -> String {
        let mut parts = Vec::new();

        if self.xp > 0 {
            parts.push(format!("{} XP", self.xp));
        }
        if self.gold > 0 {
            parts.push(format!("{} Gold", self.gold));
        }
        for (kind, rarity) in &self.items {
            parts.push(format!("{}{}", rarity.prefix(), kind.name()));
        }
        for (faction, rep) in &self.reputation {
            let sign = if *rep > 0 { "+" } else { "" };
            parts.push(format!("{}{} {} Rep", sign, rep, faction));
        }
        if let Some(ref title) = self.title {
            parts.push(format!("Title: {}", title));
        }

        if parts.is_empty() {
            "No rewards".to_string()
        } else {
            parts.join(", ")
        }
    }

    pub fn scale(&self, multiplier: f32) -> Self {
        Self {
            xp: (self.xp as f32 * multiplier) as u32,
            gold: (self.gold as f32 * multiplier) as u32,
            items: self.items.clone(),
            reputation: self.reputation.clone(),
            skills: self.skills.clone(),
            stat_bonuses: self.stat_bonuses,
            unlock_quest: self.unlock_quest,
            unlock_area: self.unlock_area.clone(),
            title: self.title.clone(),
        }
    }
}

impl Default for QuestReward {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// QUEST STATUS
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum QuestStatus {
    NotStarted,
    Available,
    InProgress,
    ReadyToComplete,
    Completed,
    Failed,
    Expired,
    Abandoned,
}

impl QuestStatus {
    pub fn name(&self) -> &'static str {
        match self {
            Self::NotStarted => "Not Started",
            Self::Available => "Available",
            Self::InProgress => "In Progress",
            Self::ReadyToComplete => "Ready to Complete",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Expired => "Expired",
            Self::Abandoned => "Abandoned",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::NotStarted => Color::DarkGrey,
            Self::Available => Color::White,
            Self::InProgress => Color::Yellow,
            Self::ReadyToComplete => Color::Green,
            Self::Completed => Color::Cyan,
            Self::Failed => Color::Red,
            Self::Expired => Color::DarkRed,
            Self::Abandoned => Color::Grey,
        }
    }
}

// ============================================================================
// QUEST CHOICE (for branching)
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestChoice {
    pub id: u32,
    pub description: String,
    pub consequence: String,
    pub reputation_effects: HashMap<String, i32>,
    pub leads_to_quest: Option<u32>,
    pub modifies_reward: Option<QuestReward>,
    pub required_class: Option<CharacterClass>,
    pub required_level: Option<u32>,
}

impl QuestChoice {
    pub fn new(id: u32, description: &str, consequence: &str) -> Self {
        Self {
            id,
            description: description.to_string(),
            consequence: consequence.to_string(),
            reputation_effects: HashMap::new(),
            leads_to_quest: None,
            modifies_reward: None,
            required_class: None,
            required_level: None,
        }
    }
}

// ============================================================================
// QUEST STRUCT
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub lore: String,
    pub quest_type: QuestType,
    pub difficulty: QuestDifficulty,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestReward,
    pub giver_npc: Option<String>,
    pub turn_in_npc: Option<String>,
    pub deadline: Option<u32>,
    pub start_turn: Option<u32>,
    pub status: QuestStatus,
    pub chain_id: Option<u32>,
    pub chain_position: Option<u32>,
    pub next_quest_id: Option<u32>,
    pub prerequisite_quests: Vec<u32>,
    pub prerequisite_level: Option<u32>,
    pub prerequisite_class: Option<CharacterClass>,
    pub choices: Vec<QuestChoice>,
    pub selected_choice: Option<u32>,
    pub repeatable: bool,
    pub times_completed: u32,
    pub floor_requirement: Option<u32>,
    pub markers: Vec<(usize, usize, char, Color)>,
    pub is_procedural: bool,
}

impl Quest {
    pub fn new(id: u32, name: &str, description: &str, quest_type: QuestType) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            lore: String::new(),
            quest_type,
            difficulty: QuestDifficulty::Normal,
            objectives: Vec::new(),
            rewards: QuestReward::new(),
            giver_npc: None,
            turn_in_npc: None,
            deadline: None,
            start_turn: None,
            status: QuestStatus::NotStarted,
            chain_id: None,
            chain_position: None,
            next_quest_id: None,
            prerequisite_quests: Vec::new(),
            prerequisite_level: None,
            prerequisite_class: None,
            choices: Vec::new(),
            selected_choice: None,
            repeatable: false,
            times_completed: 0,
            floor_requirement: None,
            markers: Vec::new(),
            is_procedural: false,
        }
    }

    pub fn with_lore(mut self, lore: &str) -> Self {
        self.lore = lore.to_string();
        self
    }

    pub fn with_difficulty(mut self, difficulty: QuestDifficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn add_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    pub fn with_reward(mut self, reward: QuestReward) -> Self {
        self.rewards = reward;
        self
    }

    pub fn from_npc(mut self, npc: &str) -> Self {
        self.giver_npc = Some(npc.to_string());
        self
    }

    pub fn turn_in_to(mut self, npc: &str) -> Self {
        self.turn_in_npc = Some(npc.to_string());
        self
    }

    pub fn with_deadline(mut self, turns: u32) -> Self {
        self.deadline = Some(turns);
        self
    }

    pub fn in_chain(mut self, chain_id: u32, position: u32) -> Self {
        self.chain_id = Some(chain_id);
        self.chain_position = Some(position);
        self
    }

    pub fn then_quest(mut self, quest_id: u32) -> Self {
        self.next_quest_id = Some(quest_id);
        self
    }

    pub fn requires_quest(mut self, quest_id: u32) -> Self {
        self.prerequisite_quests.push(quest_id);
        self
    }

    pub fn requires_level(mut self, level: u32) -> Self {
        self.prerequisite_level = Some(level);
        self
    }

    pub fn requires_class(mut self, class: CharacterClass) -> Self {
        self.prerequisite_class = Some(class);
        self
    }

    pub fn add_choice(mut self, choice: QuestChoice) -> Self {
        self.choices.push(choice);
        self
    }

    pub fn repeatable(mut self) -> Self {
        self.repeatable = true;
        self
    }

    pub fn requires_floor(mut self, floor: u32) -> Self {
        self.floor_requirement = Some(floor);
        self
    }

    pub fn add_marker(mut self, x: usize, y: usize, icon: char, color: Color) -> Self {
        self.markers.push((x, y, icon, color));
        self
    }

    pub fn start(&mut self, current_turn: u32) {
        self.status = QuestStatus::InProgress;
        self.start_turn = Some(current_turn);
    }

    pub fn check_completion(&mut self) -> bool {
        let required_complete = self.objectives.iter()
            .filter(|o| !o.optional)
            .all(|o| o.completed);

        if required_complete {
            self.status = QuestStatus::ReadyToComplete;
            true
        } else {
            false
        }
    }

    pub fn complete(&mut self) -> &QuestReward {
        self.status = QuestStatus::Completed;
        self.times_completed += 1;
        &self.rewards
    }

    pub fn fail(&mut self) {
        self.status = QuestStatus::Failed;
    }

    pub fn check_expired(&mut self, current_turn: u32) -> bool {
        if let (Some(deadline), Some(start)) = (self.deadline, self.start_turn) {
            if current_turn - start > deadline {
                self.status = QuestStatus::Expired;
                return true;
            }
        }
        false
    }

    pub fn abandon(&mut self) {
        self.status = QuestStatus::Abandoned;
    }

    pub fn remaining_turns(&self, current_turn: u32) -> Option<u32> {
        if let (Some(deadline), Some(start)) = (self.deadline, self.start_turn) {
            let elapsed = current_turn - start;
            if elapsed < deadline {
                Some(deadline - elapsed)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == QuestStatus::InProgress || self.status == QuestStatus::ReadyToComplete
    }

    pub fn progress_percentage(&self) -> f32 {
        if self.objectives.is_empty() {
            return 100.0;
        }

        let total_required: u32 = self.objectives.iter()
            .filter(|o| !o.optional)
            .map(|o| o.required_progress)
            .sum();

        let total_progress: u32 = self.objectives.iter()
            .filter(|o| !o.optional)
            .map(|o| o.current_progress)
            .sum();

        if total_required == 0 {
            100.0
        } else {
            (total_progress as f32 / total_required as f32) * 100.0
        }
    }
}

// ============================================================================
// QUEST CHAIN
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestChain {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub quest_ids: Vec<u32>,
    pub current_quest_index: usize,
    pub completed: bool,
    pub final_reward: QuestReward,
}

impl QuestChain {
    pub fn new(id: u32, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            quest_ids: Vec::new(),
            current_quest_index: 0,
            completed: false,
            final_reward: QuestReward::new(),
        }
    }

    pub fn add_quest(mut self, quest_id: u32) -> Self {
        self.quest_ids.push(quest_id);
        self
    }

    pub fn with_final_reward(mut self, reward: QuestReward) -> Self {
        self.final_reward = reward;
        self
    }

    pub fn current_quest_id(&self) -> Option<u32> {
        self.quest_ids.get(self.current_quest_index).copied()
    }

    pub fn advance(&mut self) -> bool {
        if self.current_quest_index + 1 < self.quest_ids.len() {
            self.current_quest_index += 1;
            true
        } else {
            self.completed = true;
            false
        }
    }

    pub fn progress_string(&self) -> String {
        format!("{}/{}", self.current_quest_index, self.quest_ids.len())
    }
}

// ============================================================================
// PROCEDURAL QUEST GENERATOR
// ============================================================================

pub struct ProceduralQuestGenerator;

impl ProceduralQuestGenerator {
    pub fn generate_kill_quest(rng: &mut StdRng, player_level: u32, dungeon_level: u32) -> Quest {
        let difficulty = QuestDifficulty::from_level(player_level);
        let enemy_kinds = Self::enemies_for_level(dungeon_level);
        let enemy = *enemy_kinds.choose(rng).unwrap_or(&EnemyKind::Rat);

        let base_count = match difficulty {
            QuestDifficulty::Trivial => rng.gen_range(3..=5),
            QuestDifficulty::Easy => rng.gen_range(5..=8),
            QuestDifficulty::Normal => rng.gen_range(8..=12),
            QuestDifficulty::Hard => rng.gen_range(10..=15),
            QuestDifficulty::VeryHard => rng.gen_range(12..=18),
            QuestDifficulty::Nightmare => rng.gen_range(15..=25),
            QuestDifficulty::Legendary => rng.gen_range(20..=30),
        };

        let kill_count = base_count;
        let base_xp = 50 * player_level;
        let base_gold = 25 * player_level;

        let prefixes = ["Exterminate", "Hunt", "Slay", "Eliminate", "Purge"];
        let name = format!("{} the {}", prefixes.choose(rng).unwrap(), enemy.plural_name());

        let descriptions = [
            format!("The {} have become a menace. Deal with them.", enemy.plural_name()),
            format!("Reports of {} attacks have increased. Put an end to it.", enemy.name()),
            format!("A bounty has been placed on {}. Collect it.", enemy.plural_name()),
            format!("The guild needs {} heads. Get hunting.", enemy.name()),
        ];

        Quest::new(
            rng.gen::<u32>() % 100000 + 50000,
            &name,
            descriptions.choose(rng).unwrap(),
            QuestType::Bounty,
        )
        .with_difficulty(difficulty)
        .add_objective(QuestObjective::new(
            ObjectiveType::KillEnemyType(enemy, kill_count),
            kill_count,
        ))
        .with_reward(
            QuestReward::new()
                .xp((base_xp as f32 * difficulty.xp_multiplier()) as u32)
                .gold((base_gold as f32 * difficulty.gold_multiplier()) as u32)
        )
        .with_deadline(500 + kill_count * 20)
    }

    pub fn generate_collection_quest(rng: &mut StdRng, player_level: u32, _dungeon_level: u32) -> Quest {
        let difficulty = QuestDifficulty::from_level(player_level);

        let collectibles = [
            (ItemKind::HealthPotion, "Health Potions", 3),
            (ItemKind::ManaPotion, "Mana Potions", 3),
            (ItemKind::Gold, "Gold Coins", 100),
            (ItemKind::Torch, "Torches", 5),
        ];

        let (item, item_name, base_count) = collectibles.choose(rng).unwrap();
        let count = (*base_count as f32 * difficulty.xp_multiplier()) as u32;

        let prefixes = ["Gather", "Collect", "Acquire", "Stockpile", "Hoard"];
        let name = format!("{} {}", prefixes.choose(rng).unwrap(), item_name);

        Quest::new(
            rng.gen::<u32>() % 100000 + 60000,
            &name,
            &format!("We need {} {}. Bring them to us.", count, item_name.to_lowercase()),
            QuestType::Collection,
        )
        .with_difficulty(difficulty)
        .add_objective(QuestObjective::new(
            ObjectiveType::CollectItem(*item, count),
            count,
        ))
        .with_reward(
            QuestReward::new()
                .xp((75 * player_level as f32 * difficulty.xp_multiplier()) as u32)
                .gold((50 * player_level as f32 * difficulty.gold_multiplier()) as u32)
        )
        .with_deadline(600)
    }

    pub fn generate_exploration_quest(rng: &mut StdRng, player_level: u32, dungeon_level: u32) -> Quest {
        let difficulty = QuestDifficulty::from_level(player_level);
        let target_floor = dungeon_level + rng.gen_range(1..=3);

        let names = [
            format!("Delve to Floor {}", target_floor),
            format!("Depths of Level {}", target_floor),
            format!("Explore the Unknown"),
        ];

        Quest::new(
            rng.gen::<u32>() % 100000 + 70000,
            names.choose(rng).unwrap(),
            &format!("Venture deeper into the dungeon. Reach floor {}.", target_floor),
            QuestType::Exploration,
        )
        .with_difficulty(difficulty)
        .add_objective(QuestObjective::new(
            ObjectiveType::ReachFloor(target_floor),
            1,
        ))
        .with_reward(
            QuestReward::new()
                .xp((100 * player_level as f32 * difficulty.xp_multiplier()) as u32)
                .gold((75 * player_level as f32 * difficulty.gold_multiplier()) as u32)
        )
    }

    pub fn generate_survival_quest(rng: &mut StdRng, player_level: u32, _dungeon_level: u32) -> Quest {
        let difficulty = QuestDifficulty::from_level(player_level);
        let turns = match difficulty {
            QuestDifficulty::Trivial => rng.gen_range(50..=100),
            QuestDifficulty::Easy => rng.gen_range(100..=150),
            QuestDifficulty::Normal => rng.gen_range(150..=200),
            QuestDifficulty::Hard => rng.gen_range(200..=300),
            QuestDifficulty::VeryHard => rng.gen_range(300..=400),
            QuestDifficulty::Nightmare => rng.gen_range(400..=500),
            QuestDifficulty::Legendary => rng.gen_range(500..=750),
        };

        Quest::new(
            rng.gen::<u32>() % 100000 + 80000,
            "Survival Challenge",
            &format!("Survive for {} turns in the dungeon.", turns),
            QuestType::Daily,
        )
        .with_difficulty(difficulty)
        .add_objective(QuestObjective::new(
            ObjectiveType::SurviveTurns(turns),
            turns,
        ))
        .with_reward(
            QuestReward::new()
                .xp((60 * player_level as f32 * difficulty.xp_multiplier()) as u32)
                .gold((40 * player_level as f32 * difficulty.gold_multiplier()) as u32)
        )
    }

    pub fn generate_daily_quest(rng: &mut StdRng, player_level: u32, dungeon_level: u32) -> Quest {
        match rng.gen_range(0..4) {
            0 => Self::generate_kill_quest(rng, player_level, dungeon_level),
            1 => Self::generate_collection_quest(rng, player_level, dungeon_level),
            2 => Self::generate_exploration_quest(rng, player_level, dungeon_level),
            _ => Self::generate_survival_quest(rng, player_level, dungeon_level),
        }
    }

    pub fn generate_weekly_quest(rng: &mut StdRng, player_level: u32, _dungeon_level: u32) -> Quest {
        let difficulty = QuestDifficulty::from_level(player_level + 2);

        Quest::new(
            rng.gen::<u32>() % 100000 + 90000,
            "Weekly Challenge: Dungeon Master",
            "Complete multiple challenging objectives this week.",
            QuestType::Weekly,
        )
        .with_difficulty(difficulty)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(50), 50))
        .add_objective(QuestObjective::new(ObjectiveType::CollectGold(1000), 1000))
        .add_objective(QuestObjective::new(ObjectiveType::ExploreRooms(30), 30))
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(None), 1).optional())
        .with_reward(
            QuestReward::new()
                .xp(500 * player_level)
                .gold(300 * player_level)
                .item(ItemKind::HealthPotion, Rarity::Rare)
        )
        .with_deadline(5000)
    }

    fn enemies_for_level(level: u32) -> Vec<EnemyKind> {
        match level {
            1..=4 => vec![EnemyKind::Rat, EnemyKind::Bat, EnemyKind::Spider, EnemyKind::Goblin],
            5..=9 => vec![EnemyKind::Skeleton, EnemyKind::Zombie, EnemyKind::Orc, EnemyKind::GiantSpider],
            10..=14 => vec![EnemyKind::Ghost, EnemyKind::Wraith, EnemyKind::Wolf, EnemyKind::DireWolf],
            15..=19 => vec![EnemyKind::Troll, EnemyKind::Vampire, EnemyKind::IceElemental, EnemyKind::FrostGiant],
            20..=24 => vec![EnemyKind::Demon, EnemyKind::DemonLord, EnemyKind::Lich, EnemyKind::FireElemental],
            _ => vec![EnemyKind::DemonLord, EnemyKind::Balrog, EnemyKind::Lich, EnemyKind::AbyssalHorror],
        }
    }
}

// ============================================================================
// QUEST JOURNAL
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestJournal {
    pub quests: HashMap<u32, Quest>,
    pub chains: HashMap<u32, QuestChain>,
    pub active_quests: Vec<u32>,
    pub completed_quests: Vec<u32>,
    pub failed_quests: Vec<u32>,
    pub available_quests: Vec<u32>,
    pub daily_quest_refresh_turn: u32,
    pub weekly_quest_refresh_turn: u32,
    pub total_quests_completed: u32,
    pub total_xp_from_quests: u32,
    pub total_gold_from_quests: u32,
    pub selected_index: usize,
    pub show_completed: bool,
    pub filter: Option<QuestType>,
}

impl QuestJournal {
    pub fn new() -> Self {
        let mut journal = Self {
            quests: HashMap::new(),
            chains: HashMap::new(),
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
            failed_quests: Vec::new(),
            available_quests: Vec::new(),
            daily_quest_refresh_turn: 0,
            weekly_quest_refresh_turn: 0,
            total_quests_completed: 0,
            total_xp_from_quests: 0,
            total_gold_from_quests: 0,
            selected_index: 0,
            show_completed: false,
            filter: None,
        };

        // Initialize with all predefined quests
        let all_quests = create_all_quests();
        for quest in all_quests {
            journal.add_quest(quest);
        }

        // Initialize quest chains
        let all_chains = create_quest_chains();
        for chain in all_chains {
            journal.chains.insert(chain.id, chain);
        }

        journal
    }

    pub fn add_quest(&mut self, mut quest: Quest) {
        let id = quest.id;
        if quest.status == QuestStatus::NotStarted && quest.prerequisite_quests.is_empty() {
            quest.status = QuestStatus::Available;
            self.available_quests.push(id);
        }
        self.quests.insert(id, quest);
    }

    pub fn accept_quest(&mut self, quest_id: u32, current_turn: u32) -> bool {
        if let Some(quest) = self.quests.get_mut(&quest_id) {
            if quest.status == QuestStatus::Available {
                quest.start(current_turn);
                self.available_quests.retain(|&id| id != quest_id);
                self.active_quests.push(quest_id);
                return true;
            }
        }
        false
    }

    pub fn complete_quest(&mut self, quest_id: u32) -> Option<QuestReward> {
        if let Some(quest) = self.quests.get_mut(&quest_id) {
            if quest.status == QuestStatus::ReadyToComplete {
                let reward = quest.complete().clone();

                self.active_quests.retain(|&id| id != quest_id);
                self.completed_quests.push(quest_id);
                self.total_quests_completed += 1;
                self.total_xp_from_quests += reward.xp;
                self.total_gold_from_quests += reward.gold;

                // Unlock next quest in chain
                if let Some(next_id) = quest.next_quest_id {
                    self.unlock_quest(next_id);
                }

                // Check for quest chain completion
                if let Some(chain_id) = quest.chain_id {
                    if let Some(chain) = self.chains.get_mut(&chain_id) {
                        chain.advance();
                    }
                }

                return Some(reward);
            }
        }
        None
    }

    pub fn unlock_quest(&mut self, quest_id: u32) {
        if let Some(quest) = self.quests.get_mut(&quest_id) {
            if quest.status == QuestStatus::NotStarted {
                quest.status = QuestStatus::Available;
                if !self.available_quests.contains(&quest_id) {
                    self.available_quests.push(quest_id);
                }
            }
        }
    }

    pub fn fail_quest(&mut self, quest_id: u32) {
        if let Some(quest) = self.quests.get_mut(&quest_id) {
            quest.fail();
            self.active_quests.retain(|&id| id != quest_id);
            self.failed_quests.push(quest_id);
        }
    }

    pub fn abandon_quest(&mut self, quest_id: u32) {
        if let Some(quest) = self.quests.get_mut(&quest_id) {
            quest.abandon();
            self.active_quests.retain(|&id| id != quest_id);
        }
    }

    pub fn update_progress(&mut self, objective_type: &ObjectiveType, amount: u32) {
        for &quest_id in &self.active_quests.clone() {
            if let Some(quest) = self.quests.get_mut(&quest_id) {
                for objective in &mut quest.objectives {
                    if Self::objectives_match(&objective.objective_type, objective_type) {
                        objective.update_progress(amount);
                    }
                }
                quest.check_completion();
            }
        }
    }

    fn objectives_match(obj1: &ObjectiveType, obj2: &ObjectiveType) -> bool {
        match (obj1, obj2) {
            (ObjectiveType::KillEnemyType(k1, _), ObjectiveType::KillEnemyType(k2, _)) => k1 == k2,
            (ObjectiveType::KillAnyEnemy(_), ObjectiveType::KillAnyEnemy(_)) => true,
            (ObjectiveType::KillAnyEnemy(_), ObjectiveType::KillEnemyType(_, _)) => true,
            (ObjectiveType::CollectItem(k1, _), ObjectiveType::CollectItem(k2, _)) => k1 == k2,
            (ObjectiveType::CollectGold(_), ObjectiveType::CollectGold(_)) => true,
            (ObjectiveType::ReachFloor(f1), ObjectiveType::ReachFloor(f2)) => f1 <= f2,
            (ObjectiveType::ExploreRooms(_), ObjectiveType::ExploreRooms(_)) => true,
            (ObjectiveType::SurviveTurns(_), ObjectiveType::SurviveTurns(_)) => true,
            _ => false,
        }
    }

    pub fn check_deadlines(&mut self, current_turn: u32) {
        for &quest_id in &self.active_quests.clone() {
            if let Some(quest) = self.quests.get_mut(&quest_id) {
                if quest.check_expired(current_turn) {
                    self.active_quests.retain(|&id| id != quest_id);
                    self.failed_quests.push(quest_id);
                }
            }
        }
    }

    pub fn refresh_daily_quests(&mut self, current_turn: u32, player_level: u32, dungeon_level: u32, rng: &mut StdRng) {
        if current_turn >= self.daily_quest_refresh_turn + 1000 {
            self.daily_quest_refresh_turn = current_turn;

            // Remove old daily quests
            self.available_quests.retain(|&id| {
                if let Some(quest) = self.quests.get(&id) {
                    quest.quest_type != QuestType::Daily
                } else {
                    true
                }
            });

            // Generate new daily quests
            for _ in 0..3 {
                let mut quest = ProceduralQuestGenerator::generate_daily_quest(rng, player_level, dungeon_level);
                quest.is_procedural = true;
                quest.status = QuestStatus::Available;
                let id = quest.id;
                self.quests.insert(id, quest);
                self.available_quests.push(id);
            }
        }
    }

    pub fn refresh_weekly_quests(&mut self, current_turn: u32, player_level: u32, dungeon_level: u32, rng: &mut StdRng) {
        if current_turn >= self.weekly_quest_refresh_turn + 7000 {
            self.weekly_quest_refresh_turn = current_turn;

            // Remove old weekly quests
            self.available_quests.retain(|&id| {
                if let Some(quest) = self.quests.get(&id) {
                    quest.quest_type != QuestType::Weekly
                } else {
                    true
                }
            });

            // Generate new weekly quest
            let mut quest = ProceduralQuestGenerator::generate_weekly_quest(rng, player_level, dungeon_level);
            quest.is_procedural = true;
            quest.status = QuestStatus::Available;
            let id = quest.id;
            self.quests.insert(id, quest);
            self.available_quests.push(id);
        }
    }

    pub fn get_active_quests(&self) -> Vec<&Quest> {
        self.active_quests.iter()
            .filter_map(|id| self.quests.get(id))
            .collect()
    }

    pub fn get_available_quests(&self) -> Vec<&Quest> {
        self.available_quests.iter()
            .filter_map(|id| self.quests.get(id))
            .collect()
    }

    pub fn get_quests_for_npc(&self, npc_name: &str) -> Vec<&Quest> {
        self.available_quests.iter()
            .chain(self.active_quests.iter())
            .filter_map(|id| self.quests.get(id))
            .filter(|q| {
                q.giver_npc.as_ref().map(|n| n == npc_name).unwrap_or(false) ||
                q.turn_in_npc.as_ref().map(|n| n == npc_name).unwrap_or(false)
            })
            .collect()
    }

    pub fn get_quest_markers(&self) -> Vec<(usize, usize, char, Color)> {
        let mut markers = Vec::new();
        for &quest_id in &self.active_quests {
            if let Some(quest) = self.quests.get(&quest_id) {
                markers.extend(quest.markers.clone());
            }
        }
        markers
    }

    pub fn quest_count_by_status(&self) -> (usize, usize, usize, usize) {
        (
            self.available_quests.len(),
            self.active_quests.len(),
            self.completed_quests.len(),
            self.failed_quests.len(),
        )
    }
}

impl Default for QuestJournal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PREDEFINED QUESTS (100+ unique quests)
// ============================================================================

pub fn create_all_quests() -> Vec<Quest> {
    let mut quests = Vec::new();

    // ========== MAIN STORY QUESTS (IDs 1-20) ==========
    quests.push(
        Quest::new(1, "The Descent Begins",
            "Enter the dungeon and survive your first encounter with the darkness.",
            QuestType::Main)
        .with_lore("The ancient ShadowCrypt has stood sealed for a thousand years. Now, dark energy seeps from its depths, and you have been chosen to investigate.")
        .with_difficulty(QuestDifficulty::Trivial)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(2), 1))
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(3), 3))
        .with_reward(QuestReward::new().xp(100).gold(50))
        .then_quest(2)
    );

    quests.push(
        Quest::new(2, "Whispers in the Dark",
            "The walls seem to whisper. Find the source of the mysterious voices.",
            QuestType::Main)
        .with_lore("As you delve deeper, you hear whispers that shouldn't exist. Ancient voices speak of a great evil awakening.")
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(5), 1))
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(Some(EnemyKind::BossGoblinKing)), 1))
        .with_reward(QuestReward::new().xp(300).gold(150).item(ItemKind::HealthPotion, Rarity::Uncommon))
        .requires_quest(1)
        .then_quest(3)
    );

    quests.push(
        Quest::new(3, "The Goblin Menace",
            "The Goblin King has fallen, but his army remains. Thin their numbers.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Goblin, 15), 15))
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::GoblinArcher, 10), 10))
        .with_reward(QuestReward::new().xp(400).gold(200))
        .requires_quest(2)
        .then_quest(4)
    );

    quests.push(
        Quest::new(4, "Into the Catacombs",
            "The path leads deeper, into ancient burial grounds where the dead do not rest.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(10), 1))
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Skeleton, 20), 20))
        .with_reward(QuestReward::new().xp(600).gold(300).item(ItemKind::LongSword, Rarity::Rare))
        .requires_quest(3)
        .then_quest(5)
    );

    quests.push(
        Quest::new(5, "The Necromancer's Sanctum",
            "A powerful necromancer has taken residence in the catacombs. End his dark rituals.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(Some(EnemyKind::BossOrcWarlord)), 1))
        .with_reward(QuestReward::new().xp(800).gold(500).item(ItemKind::ManaPotion, Rarity::Rare))
        .requires_quest(4)
        .then_quest(6)
    );

    quests.push(
        Quest::new(6, "Echoes of the Fallen",
            "The necromancer's death has disturbed something ancient. Investigate the sealed chamber.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(15), 1))
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Wraith, 10), 10))
        .with_reward(QuestReward::new().xp(1000).gold(600))
        .requires_quest(5)
        .then_quest(7)
    );

    quests.push(
        Quest::new(7, "The Frozen Depths",
            "Ice and frost pervade these halls. Something cold and ancient awaits.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(Some(EnemyKind::BossVampireLord)), 1))
        .with_reward(QuestReward::new().xp(1500).gold(800).item(ItemKind::FrostBlade, Rarity::Epic))
        .requires_quest(6)
        .then_quest(8)
    );

    quests.push(
        Quest::new(8, "Heart of Ice",
            "The Frost Giant's lair holds secrets of the demon's origin. Find the frozen tome.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(20), 1))
        .add_objective(QuestObjective::new(ObjectiveType::CollectAnyItems(10), 10))
        .with_reward(QuestReward::new().xp(1800).gold(1000))
        .requires_quest(7)
        .then_quest(9)
    );

    quests.push(
        Quest::new(9, "The Dragon's Lair",
            "An ancient dragon guards the path to the demon realm. Prove your worth.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(Some(EnemyKind::BossForestGuardian)), 1))
        .with_reward(QuestReward::new().xp(2500).gold(1500).item(ItemKind::DragonArmor, Rarity::Epic))
        .requires_quest(8)
        .then_quest(10)
    );

    quests.push(
        Quest::new(10, "Gates of the Abyss",
            "The demon realm's entrance lies before you. Steel yourself for what awaits.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(25), 1))
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Demon, 15), 15))
        .with_reward(QuestReward::new().xp(3000).gold(2000))
        .requires_quest(9)
        .then_quest(11)
    );

    quests.push(
        Quest::new(11, "The Demon Lord's Challenge",
            "A Demon Lord blocks your path. Defeat this lieutenant of darkness.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Nightmare)
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(Some(EnemyKind::BossIceDragon)), 1))
        .with_reward(QuestReward::new().xp(4000).gold(2500).item(ItemKind::DemonSlayer, Rarity::Legendary))
        .requires_quest(10)
        .then_quest(12)
    );

    quests.push(
        Quest::new(12, "The Final Descent",
            "Floor 30 awaits. The Demon King's throne room lies at the bottom.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Nightmare)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(30), 1))
        .with_reward(QuestReward::new().xp(5000).gold(3000))
        .requires_quest(11)
        .then_quest(13)
    );

    quests.push(
        Quest::new(13, "End of the Shadow",
            "The Demon King must fall. The fate of the world rests on your shoulders.",
            QuestType::Main)
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(Some(EnemyKind::BossDemonKing)), 1))
        .with_reward(QuestReward::new().xp(10000).gold(10000).title("Savior of the Realm"))
        .requires_quest(12)
    );

    // ========== BOUNTY QUESTS (IDs 101-130) ==========
    quests.push(
        Quest::new(101, "Rat Infestation",
            "The dungeon's lower levels are overrun with rats. Clear them out.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Trivial)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Rat, 10), 10))
        .with_reward(QuestReward::new().xp(50).gold(25))
        .from_npc("Wandering Sage")
    );

    quests.push(
        Quest::new(102, "Spider Extermination",
            "Giant spiders have nested in the caves. Destroy their webs.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Trivial)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Spider, 8), 8))
        .with_reward(QuestReward::new().xp(75).gold(40))
    );

    quests.push(
        Quest::new(103, "Bat Swarm",
            "A colony of bats is terrorizing travelers. Put an end to their menace.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Trivial)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Bat, 12), 12))
        .with_reward(QuestReward::new().xp(60).gold(30))
    );

    quests.push(
        Quest::new(104, "Goblin Scouts",
            "Goblin scouts have been spotted. Eliminate them before they report back.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Goblin, 10), 10))
        .with_reward(QuestReward::new().xp(100).gold(60))
    );

    quests.push(
        Quest::new(105, "Skeleton Patrol",
            "Undead patrols roam the catacombs. Break their ranks.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Skeleton, 12), 12))
        .with_reward(QuestReward::new().xp(120).gold(75))
        .requires_floor(5)
    );

    quests.push(
        Quest::new(106, "Zombie Uprising",
            "The dead are rising. Put them back to rest permanently.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Zombie, 15), 15))
        .with_reward(QuestReward::new().xp(150).gold(100))
        .requires_floor(5)
    );

    quests.push(
        Quest::new(107, "Orc War Party",
            "An orc war party has entered the dungeon. Stop their advance.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Orc, 10), 10))
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::OrcWarrior, 5), 5))
        .with_reward(QuestReward::new().xp(200).gold(150))
        .requires_floor(8)
    );

    quests.push(
        Quest::new(108, "Ghostly Presence",
            "Ghosts haunt the lower halls. Dispel their spirits.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Ghost, 8), 8))
        .with_reward(QuestReward::new().xp(250).gold(200))
        .requires_floor(10)
    );

    quests.push(
        Quest::new(109, "Wraith Hunters",
            "Wraiths are particularly dangerous. Destroy them with care.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Wraith, 6), 6))
        .with_reward(QuestReward::new().xp(300).gold(250))
        .requires_floor(12)
    );

    quests.push(
        Quest::new(110, "Vampire Hunt",
            "A vampire coven threatens the region. End their bloodlust.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Vampire, 5), 5))
        .with_reward(QuestReward::new().xp(400).gold(350))
        .requires_floor(15)
    );

    quests.push(
        Quest::new(111, "Troll Trouble",
            "Trolls have made the caves their home. Evict them permanently.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Troll, 4), 4))
        .with_reward(QuestReward::new().xp(350).gold(300))
        .requires_floor(13)
    );

    quests.push(
        Quest::new(112, "Dark Elf Assassins",
            "Dark elf assassins have infiltrated the dungeon. Hunt them down.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::DarkElf, 8), 8))
        .with_reward(QuestReward::new().xp(350).gold(275))
        .requires_floor(12)
    );

    quests.push(
        Quest::new(113, "Werewolf Pack",
            "A pack of werewolves prowls the night. Silver your weapons.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Werewolf, 4), 4))
        .with_reward(QuestReward::new().xp(450).gold(400))
        .requires_floor(16)
    );

    quests.push(
        Quest::new(114, "Demon Slayer",
            "Demons have crossed into our realm. Send them back to the abyss.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Nightmare)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Demon, 10), 10))
        .with_reward(QuestReward::new().xp(600).gold(500))
        .requires_floor(20)
    );

    quests.push(
        Quest::new(115, "Lich's Bane",
            "A lich threatens to raise an army of undead. Destroy its phylactery.",
            QuestType::Bounty)
        .with_difficulty(QuestDifficulty::Nightmare)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Lich, 2), 2))
        .with_reward(QuestReward::new().xp(800).gold(600))
        .requires_floor(22)
    );

    // ========== EXPLORATION QUESTS (IDs 201-230) ==========
    quests.push(
        Quest::new(201, "Dungeon Delver",
            "Explore the first few floors of the dungeon.",
            QuestType::Exploration)
        .with_difficulty(QuestDifficulty::Trivial)
        .add_objective(QuestObjective::new(ObjectiveType::ExploreRooms(10), 10))
        .with_reward(QuestReward::new().xp(100).gold(50))
    );

    quests.push(
        Quest::new(202, "Cartographer",
            "Map out a significant portion of the dungeon.",
            QuestType::Exploration)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::ExploreRooms(25), 25))
        .with_reward(QuestReward::new().xp(200).gold(100).item(ItemKind::ScrollMapping, Rarity::Common))
    );

    quests.push(
        Quest::new(203, "Secret Seeker",
            "Find hidden rooms within the dungeon.",
            QuestType::Exploration)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::FindSecretRoom, 1))
        .with_reward(QuestReward::new().xp(300).gold(200))
    );

    quests.push(
        Quest::new(204, "Deep Explorer",
            "Venture deep into the dungeon's lower levels.",
            QuestType::Exploration)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(15), 1))
        .with_reward(QuestReward::new().xp(500).gold(300))
    );

    quests.push(
        Quest::new(205, "Abyss Walker",
            "Reach the deepest known levels of the dungeon.",
            QuestType::Exploration)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(25), 1))
        .with_reward(QuestReward::new().xp(1000).gold(600))
    );

    quests.push(
        Quest::new(206, "Room by Room",
            "Methodically explore every corner of the dungeon.",
            QuestType::Exploration)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::ExploreRooms(50), 50))
        .with_reward(QuestReward::new().xp(400).gold(250))
    );

    quests.push(
        Quest::new(207, "Lost Passages",
            "Discover the hidden pathways between floors.",
            QuestType::Exploration)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::FindAllSecrets(3), 3))
        .with_reward(QuestReward::new().xp(600).gold(400))
    );

    // ========== COLLECTION QUESTS (IDs 301-340) ==========
    quests.push(
        Quest::new(301, "Potion Hoarder",
            "Collect health potions for the infirmary.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::CollectItem(ItemKind::HealthPotion, 5), 5))
        .with_reward(QuestReward::new().xp(100).gold(75))
        .from_npc("Temple Healer")
    );

    quests.push(
        Quest::new(302, "Mana Crystal Gathering",
            "The Enchanters Guild needs mana potions for their experiments.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::CollectItem(ItemKind::ManaPotion, 5), 5))
        .with_reward(QuestReward::new().xp(100).gold(80))
        .from_npc("Mystic Enchanter")
    );

    quests.push(
        Quest::new(303, "Gold Rush",
            "Accumulate wealth for the merchant's guild.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::CollectGold(500), 500))
        .with_reward(QuestReward::new().xp(200).item(ItemKind::RingOfProtection, Rarity::Uncommon))
        .from_npc("Traveling Merchant")
    );

    quests.push(
        Quest::new(304, "Treasure Hunter",
            "Find valuable items throughout the dungeon.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::CollectAnyItems(15), 15))
        .with_reward(QuestReward::new().xp(250).gold(150))
    );

    quests.push(
        Quest::new(305, "Rare Finds",
            "Locate rare quality items.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::CollectRarity(Rarity::Rare, 3), 3))
        .with_reward(QuestReward::new().xp(400).gold(300))
    );

    quests.push(
        Quest::new(306, "Epic Discovery",
            "Find epic quality equipment.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::CollectRarity(Rarity::Epic, 2), 2))
        .with_reward(QuestReward::new().xp(800).gold(500))
    );

    quests.push(
        Quest::new(307, "Legendary Acquisition",
            "Discover a legendary artifact.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Nightmare)
        .add_objective(QuestObjective::new(ObjectiveType::CollectRarity(Rarity::Legendary, 1), 1))
        .with_reward(QuestReward::new().xp(1500).gold(1000))
    );

    quests.push(
        Quest::new(308, "Scroll Collector",
            "Gather magical scrolls for study.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::CollectItem(ItemKind::ScrollFireball, 2), 2))
        .add_objective(QuestObjective::new(ObjectiveType::CollectItem(ItemKind::ScrollIceStorm, 2), 2))
        .with_reward(QuestReward::new().xp(300).gold(200))
    );

    quests.push(
        Quest::new(309, "Torch Bearer",
            "Collect torches to light the dark passages.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Trivial)
        .add_objective(QuestObjective::new(ObjectiveType::CollectItem(ItemKind::Torch, 10), 10))
        .with_reward(QuestReward::new().xp(50).gold(30))
    );

    quests.push(
        Quest::new(310, "Wealthy Adventurer",
            "Amass a significant fortune.",
            QuestType::Collection)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::CollectGold(2000), 2000))
        .with_reward(QuestReward::new().xp(600).item(ItemKind::RingOfMana, Rarity::Rare))
    );

    // ========== SKILL/CRAFTING QUESTS (IDs 401-420) ==========
    quests.push(
        Quest::new(401, "Combat Training",
            "Use your combat skills in battle.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::UseSkill(10), 10))
        .with_reward(QuestReward::new().xp(150).gold(75))
    );

    quests.push(
        Quest::new(402, "Skill Master",
            "Demonstrate mastery of your abilities.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::UseSkill(50), 50))
        .with_reward(QuestReward::new().xp(400).gold(200))
    );

    quests.push(
        Quest::new(403, "Level Up",
            "Gain experience and reach a new level.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::LevelUp(5), 5))
        .with_reward(QuestReward::new().xp(200).gold(100))
    );

    quests.push(
        Quest::new(404, "Power Growth",
            "Continue growing stronger.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::LevelUp(10), 10))
        .with_reward(QuestReward::new().xp(500).gold(250))
    );

    quests.push(
        Quest::new(405, "Veteran Status",
            "Become a seasoned adventurer.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::LevelUp(15), 15))
        .with_reward(QuestReward::new().xp(1000).gold(500))
    );

    quests.push(
        Quest::new(406, "Elite Warrior",
            "Reach the pinnacle of power.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::LevelUp(20), 20))
        .with_reward(QuestReward::new().xp(2000).gold(1000))
    );

    quests.push(
        Quest::new(407, "Legendary Hero",
            "Become a living legend.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::LevelUp(25), 25))
        .with_reward(QuestReward::new().xp(5000).gold(2500).title("Living Legend"))
    );

    // ========== SURVIVAL QUESTS (IDs 501-520) ==========
    quests.push(
        Quest::new(501, "Survivor",
            "Survive in the dungeon for an extended period.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::SurviveTurns(200), 200))
        .with_reward(QuestReward::new().xp(300).gold(150))
    );

    quests.push(
        Quest::new(502, "Endurance Test",
            "Test your limits of survival.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::SurviveTurns(500), 500))
        .with_reward(QuestReward::new().xp(600).gold(300))
    );

    quests.push(
        Quest::new(503, "Iron Will",
            "Survive without healing for an extended period.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::SurviveWithoutHealing(100), 100))
        .with_reward(QuestReward::new().xp(800).gold(400).stats(10, 0, 5, 0, 0))
    );

    quests.push(
        Quest::new(504, "Unbreakable",
            "Reach deep floors while maintaining high health.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Nightmare)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloorWithHP(15, 75), 1))
        .with_reward(QuestReward::new().xp(1200).gold(600))
    );

    quests.push(
        Quest::new(505, "Perfect Run",
            "Reach the lower floors in peak condition.",
            QuestType::Side)
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloorWithHP(20, 90), 1))
        .with_reward(QuestReward::new().xp(2000).gold(1000).title("Untouchable"))
    );

    // ========== ARENA/COMBAT QUESTS (IDs 601-630) ==========
    quests.push(
        Quest::new(601, "First Blood",
            "Slay your first enemies.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Trivial)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(5), 5))
        .with_reward(QuestReward::new().xp(50).gold(25))
    );

    quests.push(
        Quest::new(602, "Blood Bath",
            "Prove yourself in combat.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(25), 25))
        .with_reward(QuestReward::new().xp(200).gold(100))
    );

    quests.push(
        Quest::new(603, "Warrior's Path",
            "Walk the path of the warrior.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(50), 50))
        .with_reward(QuestReward::new().xp(400).gold(200))
    );

    quests.push(
        Quest::new(604, "Champion's Trial",
            "Prove your worth as a champion.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(100), 100))
        .with_reward(QuestReward::new().xp(800).gold(400).item(ItemKind::Greatsword, Rarity::Rare))
    );

    quests.push(
        Quest::new(605, "Slayer Supreme",
            "Become a master of death.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(200), 200))
        .with_reward(QuestReward::new().xp(1500).gold(800))
    );

    quests.push(
        Quest::new(606, "Death Incarnate",
            "Leave none standing in your wake.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Nightmare)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(500), 500))
        .with_reward(QuestReward::new().xp(3000).gold(1500).title("Death Incarnate"))
    );

    quests.push(
        Quest::new(607, "Boss Hunter",
            "Defeat the dungeon's bosses.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(None), 1))
        .with_reward(QuestReward::new().xp(500).gold(300))
    );

    quests.push(
        Quest::new(608, "Boss Slayer",
            "Defeat multiple bosses.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::VeryHard)
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(None), 3))
        .with_reward(QuestReward::new().xp(1500).gold(800))
    );

    quests.push(
        Quest::new(609, "Flawless Victory",
            "Kill enemies without taking damage.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::KillWithoutDamage(5), 5))
        .with_reward(QuestReward::new().xp(600).gold(350))
    );

    quests.push(
        Quest::new(610, "Speed Demon",
            "Kill enemies quickly.",
            QuestType::Arena)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillInTime(10, 50), 10))
        .with_reward(QuestReward::new().xp(400).gold(200))
    );

    // ========== NPC/GUILD QUESTS (IDs 701-750) ==========
    quests.push(
        Quest::new(701, "Merchant's Request",
            "Help the traveling merchant with their inventory problems.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::CollectGold(200), 200))
        .with_reward(QuestReward::new().xp(150).reputation("Merchants Guild", 10))
        .from_npc("Traveling Merchant")
    );

    quests.push(
        Quest::new(702, "Healer's Herbs",
            "Gather healing supplies for the temple.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::CollectItem(ItemKind::HealthPotion, 3), 3))
        .with_reward(QuestReward::new().xp(100).reputation("Temple", 10))
        .from_npc("Temple Healer")
    );

    quests.push(
        Quest::new(703, "Blacksmith's Test",
            "Prove your worth to the blacksmith.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(20), 20))
        .with_reward(QuestReward::new().xp(200).reputation("Blacksmith Guild", 15).item(ItemKind::IronHelm, Rarity::Uncommon))
        .from_npc("Master Blacksmith")
    );

    quests.push(
        Quest::new(704, "Alchemist's Ingredients",
            "Gather rare ingredients from the dungeon.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::CollectAnyItems(10), 10))
        .with_reward(QuestReward::new().xp(250).reputation("Alchemist Guild", 15).item(ItemKind::StrengthPotion, Rarity::Rare))
        .from_npc("Eccentric Alchemist")
    );

    quests.push(
        Quest::new(705, "Sage's Wisdom",
            "Prove your understanding of the dungeon.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(10), 1))
        .add_objective(QuestObjective::new(ObjectiveType::ExploreRooms(30), 30))
        .with_reward(QuestReward::new().xp(400).reputation("Sages Council", 20))
        .from_npc("Wandering Sage")
    );

    quests.push(
        Quest::new(706, "Enchanter's Challenge",
            "Gather magical items for enchantment study.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::CollectRarity(Rarity::Rare, 2), 2))
        .with_reward(QuestReward::new().xp(500).reputation("Enchanters Circle", 25).item(ItemKind::ScrollEnchant, Rarity::Rare))
        .from_npc("Mystic Enchanter")
    );

    quests.push(
        Quest::new(707, "Guard's Duty",
            "Help the dungeon guard clear threats.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(30), 30))
        .with_reward(QuestReward::new().xp(300).reputation("Guards", 20))
        .from_npc("Dungeon Guard")
    );

    // ========== LEGENDARY QUESTS (IDs 801-820) ==========
    quests.push(
        Quest::new(801, "The Demon King's Bane",
            "Collect artifacts of power to stand against the Demon King.",
            QuestType::Legendary)
        .with_lore("Ancient prophecies speak of five artifacts that, when combined, grant power to defeat the Demon King.")
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::CollectRarity(Rarity::Legendary, 3), 3))
        .add_objective(QuestObjective::new(ObjectiveType::KillBoss(None), 5))
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(30), 1))
        .with_reward(QuestReward::new().xp(10000).gold(5000).title("Demon's Bane").stats(20, 10, 10, 20, 5))
        .in_chain(1, 1)
    );

    quests.push(
        Quest::new(802, "Keeper of Secrets",
            "Uncover all the hidden secrets of ShadowCrypt.",
            QuestType::Legendary)
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::FindAllSecrets(10), 10))
        .add_objective(QuestObjective::new(ObjectiveType::ExploreRooms(100), 100))
        .with_reward(QuestReward::new().xp(8000).gold(4000).title("Keeper of Secrets"))
    );

    quests.push(
        Quest::new(803, "The Thousand Slain",
            "Become a legend through sheer combat prowess.",
            QuestType::Legendary)
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(1000), 1000))
        .with_reward(QuestReward::new().xp(15000).gold(7500).title("The Thousand Slayer").stats(0, 25, 0, 0, 10))
    );

    quests.push(
        Quest::new(804, "Master of All",
            "Reach the pinnacle of power and mastery.",
            QuestType::Legendary)
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::LevelUp(30), 30))
        .add_objective(QuestObjective::new(ObjectiveType::UseSkill(500), 500))
        .with_reward(QuestReward::new().xp(20000).gold(10000).title("Master of All"))
    );

    quests.push(
        Quest::new(805, "Wealthy Beyond Measure",
            "Amass legendary wealth.",
            QuestType::Legendary)
        .with_difficulty(QuestDifficulty::Legendary)
        .add_objective(QuestObjective::new(ObjectiveType::CollectGold(50000), 50000))
        .with_reward(QuestReward::new().xp(12000).title("Dragon's Hoard").item(ItemKind::CrownOfKings, Rarity::Legendary))
    );

    // ========== TIME-LIMITED QUESTS (IDs 901-920) ==========
    quests.push(
        Quest::new(901, "Timed Extermination",
            "Clear enemies quickly before time runs out.",
            QuestType::Daily)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(15), 15))
        .with_reward(QuestReward::new().xp(200).gold(100))
        .with_deadline(100)
        .repeatable()
    );

    quests.push(
        Quest::new(902, "Speed Run",
            "Reach the next floor quickly.",
            QuestType::Daily)
        .with_difficulty(QuestDifficulty::Hard)
        .add_objective(QuestObjective::new(ObjectiveType::ReachFloor(3), 1))
        .with_reward(QuestReward::new().xp(300).gold(150))
        .with_deadline(150)
        .repeatable()
    );

    quests.push(
        Quest::new(903, "Quick Loot",
            "Gather items before time expires.",
            QuestType::Daily)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::CollectAnyItems(5), 5))
        .with_reward(QuestReward::new().xp(150).gold(75))
        .with_deadline(75)
        .repeatable()
    );

    quests.push(
        Quest::new(904, "Rapid Gold",
            "Collect gold quickly.",
            QuestType::Daily)
        .with_difficulty(QuestDifficulty::Easy)
        .add_objective(QuestObjective::new(ObjectiveType::CollectGold(100), 100))
        .with_reward(QuestReward::new().xp(100).gold(50))
        .with_deadline(100)
        .repeatable()
    );

    // ========== CLASS-SPECIFIC QUESTS (IDs 1001-1050) ==========
    quests.push(
        Quest::new(1001, "Warrior's Honor",
            "Prove yourself as a true warrior.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillAnyEnemy(30), 30))
        .add_objective(QuestObjective::new(ObjectiveType::KillWithoutDamage(3), 3))
        .with_reward(QuestReward::new().xp(400).gold(200).item(ItemKind::Greatsword, Rarity::Rare))
        .requires_class(CharacterClass::Warrior)
    );

    quests.push(
        Quest::new(1002, "Mage's Study",
            "Master the arcane arts through practice.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::UseSkill(30), 30))
        .add_objective(QuestObjective::new(ObjectiveType::CollectItem(ItemKind::ManaPotion, 5), 5))
        .with_reward(QuestReward::new().xp(400).gold(200).item(ItemKind::VoidStaff, Rarity::Rare))
        .requires_class(CharacterClass::Mage)
    );

    quests.push(
        Quest::new(1003, "Rogue's Cunning",
            "Use stealth and skill to overcome your foes.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Goblin, 10), 10))
        .add_objective(QuestObjective::new(ObjectiveType::CollectGold(300), 300))
        .with_reward(QuestReward::new().xp(400).gold(300).item(ItemKind::Dagger, Rarity::Rare))
        .requires_class(CharacterClass::Rogue)
    );

    quests.push(
        Quest::new(1004, "Paladin's Virtue",
            "Smite evil and protect the innocent.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Skeleton, 15), 15))
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Zombie, 15), 15))
        .with_reward(QuestReward::new().xp(400).gold(200).item(ItemKind::HolyArmor, Rarity::Rare))
        .requires_class(CharacterClass::Paladin)
    );

    quests.push(
        Quest::new(1005, "Ranger's Hunt",
            "Track and hunt the beasts of the dungeon.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Wolf, 10), 10))
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Spider, 10), 10))
        .with_reward(QuestReward::new().xp(400).gold(200).item(ItemKind::Bow, Rarity::Rare))
        .requires_class(CharacterClass::Ranger)
    );

    quests.push(
        Quest::new(1006, "Necromancer's Dominion",
            "Master the powers of death and undeath.",
            QuestType::Guild)
        .with_difficulty(QuestDifficulty::Normal)
        .add_objective(QuestObjective::new(ObjectiveType::KillEnemyType(EnemyKind::Ghost, 5), 5))
        .add_objective(QuestObjective::new(ObjectiveType::UseSkill(25), 25))
        .with_reward(QuestReward::new().xp(400).gold(200).item(ItemKind::Scythe, Rarity::Rare))
        .requires_class(CharacterClass::Necromancer)
    );

    quests
}

// ============================================================================
// QUEST CHAINS
// ============================================================================

pub fn create_quest_chains() -> Vec<QuestChain> {
    vec![
        QuestChain::new(1, "The Descent", "The main story of ShadowCrypt")
            .add_quest(1)
            .add_quest(2)
            .add_quest(3)
            .add_quest(4)
            .add_quest(5)
            .add_quest(6)
            .add_quest(7)
            .add_quest(8)
            .add_quest(9)
            .add_quest(10)
            .add_quest(11)
            .add_quest(12)
            .add_quest(13)
            .with_final_reward(
                QuestReward::new()
                    .xp(25000)
                    .gold(15000)
                    .title("Conqueror of ShadowCrypt")
            ),

        QuestChain::new(2, "The Hunter", "Become a legendary monster hunter")
            .add_quest(101)
            .add_quest(102)
            .add_quest(103)
            .add_quest(104)
            .add_quest(105)
            .add_quest(110)
            .add_quest(114)
            .with_final_reward(
                QuestReward::new()
                    .xp(5000)
                    .gold(3000)
                    .title("Master Hunter")
            ),

        QuestChain::new(3, "Explorer's Guild", "Map the depths of ShadowCrypt")
            .add_quest(201)
            .add_quest(202)
            .add_quest(203)
            .add_quest(204)
            .add_quest(205)
            .with_final_reward(
                QuestReward::new()
                    .xp(4000)
                    .gold(2000)
                    .title("Master Cartographer")
            ),

        QuestChain::new(4, "Path of Power", "Grow stronger through combat")
            .add_quest(601)
            .add_quest(602)
            .add_quest(603)
            .add_quest(604)
            .add_quest(605)
            .add_quest(606)
            .with_final_reward(
                QuestReward::new()
                    .xp(8000)
                    .gold(4000)
                    .title("Avatar of War")
            ),
    ]
}

//! Tower and Endless Dungeon Modes
//!
//! Special game modes with infinite progression, challenges, and rewards.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tower/dungeon mode types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TowerMode {
    /// Classic story mode (30 floors)
    StoryMode,
    /// Endless dungeon (infinite floors)
    EndlessDungeon,
    /// Tower defense (waves of enemies)
    TowerDefense,
    /// Boss rush (bosses only)
    BossRush,
    /// Daily challenge (timed, seeded)
    DailyChallenge,
    /// Weekly challenge (harder)
    WeeklyChallenge,
    /// Randomizer mode
    Randomizer,
    /// Arena mode (combat only)
    Arena,
    /// Puzzle mode (traps and puzzles)
    PuzzleMode,
    /// Survival mode (limited resources)
    Survival,
}

impl TowerMode {
    pub fn all() -> &'static [TowerMode] {
        &[
            Self::StoryMode, Self::EndlessDungeon, Self::TowerDefense,
            Self::BossRush, Self::DailyChallenge, Self::WeeklyChallenge,
            Self::Randomizer, Self::Arena, Self::PuzzleMode, Self::Survival,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::StoryMode => "Story Mode",
            Self::EndlessDungeon => "Endless Dungeon",
            Self::TowerDefense => "Tower Defense",
            Self::BossRush => "Boss Rush",
            Self::DailyChallenge => "Daily Challenge",
            Self::WeeklyChallenge => "Weekly Challenge",
            Self::Randomizer => "Randomizer",
            Self::Arena => "Arena",
            Self::PuzzleMode => "Puzzle Dungeon",
            Self::Survival => "Survival Mode",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::StoryMode => "The classic 30-floor adventure. Defeat the Demon King!",
            Self::EndlessDungeon => "Infinite floors with scaling difficulty. How deep can you go?",
            Self::TowerDefense => "Defend against waves of enemies. Build defenses!",
            Self::BossRush => "Fight bosses back-to-back. No rest between fights!",
            Self::DailyChallenge => "A new challenge every day. Compete for high scores!",
            Self::WeeklyChallenge => "Harder weekly challenges with unique modifiers.",
            Self::Randomizer => "Everything is randomized. Expect the unexpected!",
            Self::Arena => "Pure combat. Fight in the arena for glory!",
            Self::PuzzleMode => "Solve puzzles and avoid traps. Brains over brawn!",
            Self::Survival => "Limited resources. Manage carefully to survive!",
        }
    }

    pub fn has_floor_limit(&self) -> bool {
        matches!(self, Self::StoryMode | Self::BossRush | Self::PuzzleMode)
    }

    pub fn max_floors(&self) -> Option<u32> {
        match self {
            Self::StoryMode => Some(30),
            Self::BossRush => Some(10),
            Self::PuzzleMode => Some(20),
            _ => None,
        }
    }

    pub fn modifiers(&self) -> TowerModifiers {
        match self {
            Self::StoryMode => TowerModifiers::default(),
            Self::EndlessDungeon => TowerModifiers {
                difficulty_scaling: 1.05,
                loot_scaling: 1.02,
                xp_scaling: 1.03,
                ..Default::default()
            },
            Self::TowerDefense => TowerModifiers {
                enemy_waves: true,
                build_phase: true,
                gold_bonus: 1.5,
                ..Default::default()
            },
            Self::BossRush => TowerModifiers {
                bosses_only: true,
                no_healing_between: true,
                xp_bonus: 2.0,
                loot_bonus: 2.0,
                ..Default::default()
            },
            Self::DailyChallenge => TowerModifiers {
                seeded: true,
                time_limit: Some(3600),
                score_multiplier: 1.5,
                ..Default::default()
            },
            Self::WeeklyChallenge => TowerModifiers {
                seeded: true,
                difficulty_multiplier: 1.5,
                score_multiplier: 2.0,
                special_rules: vec!["No Potions".into(), "Double Enemies".into()],
                ..Default::default()
            },
            Self::Randomizer => TowerModifiers {
                randomize_items: true,
                randomize_enemies: true,
                randomize_abilities: true,
                ..Default::default()
            },
            Self::Arena => TowerModifiers {
                combat_only: true,
                waves: true,
                gold_bonus: 1.3,
                xp_bonus: 1.5,
                ..Default::default()
            },
            Self::PuzzleMode => TowerModifiers {
                puzzles: true,
                reduced_enemies: true,
                trap_bonus: 2.0,
                ..Default::default()
            },
            Self::Survival => TowerModifiers {
                limited_resources: true,
                no_shops: true,
                hunger_rate: 2.0,
                permadeath: true,
                ..Default::default()
            },
        }
    }
}

/// Modifiers for tower modes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TowerModifiers {
    pub difficulty_multiplier: f32,
    pub difficulty_scaling: f32,
    pub loot_bonus: f32,
    pub loot_scaling: f32,
    pub xp_bonus: f32,
    pub xp_scaling: f32,
    pub gold_bonus: f32,
    pub score_multiplier: f32,
    pub enemy_waves: bool,
    pub waves: bool,
    pub build_phase: bool,
    pub bosses_only: bool,
    pub no_healing_between: bool,
    pub seeded: bool,
    pub time_limit: Option<u32>,
    pub special_rules: Vec<String>,
    pub randomize_items: bool,
    pub randomize_enemies: bool,
    pub randomize_abilities: bool,
    pub combat_only: bool,
    pub puzzles: bool,
    pub reduced_enemies: bool,
    pub trap_bonus: f32,
    pub limited_resources: bool,
    pub no_shops: bool,
    pub hunger_rate: f32,
    pub permadeath: bool,
}

impl Default for TowerModifiers {
    fn default() -> Self {
        Self {
            difficulty_multiplier: 1.0,
            difficulty_scaling: 1.0,
            loot_bonus: 1.0,
            loot_scaling: 1.0,
            xp_bonus: 1.0,
            xp_scaling: 1.0,
            gold_bonus: 1.0,
            score_multiplier: 1.0,
            enemy_waves: false,
            waves: false,
            build_phase: false,
            bosses_only: false,
            no_healing_between: false,
            seeded: false,
            time_limit: None,
            special_rules: vec![],
            randomize_items: false,
            randomize_enemies: false,
            randomize_abilities: false,
            combat_only: false,
            puzzles: false,
            reduced_enemies: false,
            trap_bonus: 1.0,
            limited_resources: false,
            no_shops: false,
            hunger_rate: 1.0,
            permadeath: false,
        }
    }
}

/// Floor data for endless mode
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndlessFloor {
    pub floor: u32,
    pub difficulty: f32,
    pub enemy_count: u32,
    pub has_boss: bool,
    pub theme: String,
    pub special_modifier: Option<FloorModifier>,
    pub cleared: bool,
    pub completion_time: Option<u32>,
}

impl EndlessFloor {
    pub fn new(floor: u32) -> Self {
        let difficulty = 1.0 + (floor as f32 * 0.05);
        let has_boss = floor % 5 == 0;
        let enemy_count = 5 + floor / 2;
        let theme = Self::theme_for_floor(floor);
        let special_modifier = if floor % 10 == 0 {
            Some(FloorModifier::random())
        } else {
            None
        };

        Self {
            floor,
            difficulty,
            enemy_count,
            has_boss,
            theme,
            special_modifier,
            cleared: false,
            completion_time: None,
        }
    }

    fn theme_for_floor(floor: u32) -> String {
        let themes = ["Dungeon", "Cave", "Crypt", "Forest", "Ice", "Volcanic", "Ruins", "Demon"];
        themes[(floor as usize / 5) % themes.len()].to_string()
    }
}

/// Special floor modifiers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FloorModifier {
    DoubleEnemies,
    HalfEnemies,
    DoubleLoot,
    NoLoot,
    FastEnemies,
    SlowEnemies,
    ToughEnemies,
    WeakEnemies,
    DarkFloor,
    BrightFloor,
    Labyrinth,
    OpenArena,
    TrapHeavy,
    SafeFloor,
    EliteOnly,
    SwarmMode,
}

impl FloorModifier {
    pub fn random() -> Self {
        let options = [
            Self::DoubleEnemies, Self::HalfEnemies, Self::DoubleLoot, Self::NoLoot,
            Self::FastEnemies, Self::SlowEnemies, Self::ToughEnemies, Self::WeakEnemies,
            Self::DarkFloor, Self::BrightFloor, Self::Labyrinth, Self::OpenArena,
            Self::TrapHeavy, Self::SafeFloor, Self::EliteOnly, Self::SwarmMode,
        ];
        options[rand::random::<usize>() % options.len()].clone()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::DoubleEnemies => "Horde",
            Self::HalfEnemies => "Sparse",
            Self::DoubleLoot => "Treasure Trove",
            Self::NoLoot => "Barren",
            Self::FastEnemies => "Swift Foes",
            Self::SlowEnemies => "Sluggish Foes",
            Self::ToughEnemies => "Fortified",
            Self::WeakEnemies => "Weakened",
            Self::DarkFloor => "Pitch Black",
            Self::BrightFloor => "Well Lit",
            Self::Labyrinth => "Maze",
            Self::OpenArena => "Open Arena",
            Self::TrapHeavy => "Trapped",
            Self::SafeFloor => "Safe Haven",
            Self::EliteOnly => "Elite Guard",
            Self::SwarmMode => "Swarm",
        }
    }
}

/// Wave data for tower defense
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wave {
    pub wave_number: u32,
    pub enemy_types: Vec<(String, u32)>,
    pub boss: Option<String>,
    pub spawn_rate: f32,
    pub difficulty_multiplier: f32,
    pub completed: bool,
}

impl Wave {
    pub fn new(wave_number: u32) -> Self {
        let mut enemy_types = vec![
            ("Goblin".to_string(), 3 + wave_number),
            ("Skeleton".to_string(), 2 + wave_number / 2),
        ];

        if wave_number >= 5 {
            enemy_types.push(("Orc".to_string(), wave_number / 3));
        }
        if wave_number >= 10 {
            enemy_types.push(("Demon".to_string(), wave_number / 5));
        }

        let boss = if wave_number % 5 == 0 {
            Some(format!("Wave {} Boss", wave_number))
        } else {
            None
        };

        Self {
            wave_number,
            enemy_types,
            boss,
            spawn_rate: 1.0 + (wave_number as f32 * 0.1),
            difficulty_multiplier: 1.0 + (wave_number as f32 * 0.05),
            completed: false,
        }
    }

    pub fn total_enemies(&self) -> u32 {
        self.enemy_types.iter().map(|(_, count)| count).sum::<u32>()
            + if self.boss.is_some() { 1 } else { 0 }
    }
}

/// Tower run statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TowerStats {
    pub highest_floor: u32,
    pub total_floors_cleared: u32,
    pub total_enemies_killed: u32,
    pub total_bosses_killed: u32,
    pub total_gold_earned: u64,
    pub total_xp_earned: u64,
    pub fastest_floor_time: Option<u32>,
    pub best_daily_score: u32,
    pub best_weekly_score: u32,
    pub arena_wins: u32,
    pub survival_record: u32,
}

/// Tower system manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TowerSystem {
    pub mode: TowerMode,
    pub current_floor: u32,
    pub modifiers: TowerModifiers,
    pub floors: HashMap<u32, EndlessFloor>,
    pub waves: Vec<Wave>,
    pub current_wave: u32,
    pub score: u64,
    pub stats: TowerStats,
    pub seed: Option<u64>,
    pub start_time: u32,
    pub elapsed_turns: u32,
}

impl TowerSystem {
    pub fn new(mode: TowerMode) -> Self {
        Self {
            mode,
            current_floor: 1,
            modifiers: mode.modifiers(),
            floors: HashMap::new(),
            waves: Vec::new(),
            current_wave: 0,
            score: 0,
            stats: TowerStats::default(),
            seed: None,
            start_time: 0,
            elapsed_turns: 0,
        }
    }

    pub fn with_seed(mode: TowerMode, seed: u64) -> Self {
        let mut system = Self::new(mode);
        system.seed = Some(seed);
        system.modifiers.seeded = true;
        system
    }

    pub fn current_floor_data(&self) -> EndlessFloor {
        self.floors.get(&self.current_floor)
            .cloned()
            .unwrap_or_else(|| EndlessFloor::new(self.current_floor))
    }

    pub fn advance_floor(&mut self) -> bool {
        if let Some(max) = self.mode.max_floors() {
            if self.current_floor >= max {
                return false;
            }
        }

        if let Some(floor) = self.floors.get_mut(&self.current_floor) {
            floor.cleared = true;
            floor.completion_time = Some(self.elapsed_turns);
        }

        self.current_floor += 1;
        self.stats.total_floors_cleared += 1;

        if self.current_floor > self.stats.highest_floor {
            self.stats.highest_floor = self.current_floor;
        }

        // Generate next floor
        self.floors.insert(self.current_floor, EndlessFloor::new(self.current_floor));
        true
    }

    pub fn difficulty(&self) -> f32 {
        let base = self.modifiers.difficulty_multiplier;
        let scaling = self.modifiers.difficulty_scaling.powf(self.current_floor as f32 - 1.0);
        base * scaling
    }

    pub fn loot_multiplier(&self) -> f32 {
        let base = self.modifiers.loot_bonus;
        let scaling = self.modifiers.loot_scaling.powf(self.current_floor as f32 - 1.0);
        base * scaling
    }

    pub fn xp_multiplier(&self) -> f32 {
        let base = self.modifiers.xp_bonus;
        let scaling = self.modifiers.xp_scaling.powf(self.current_floor as f32 - 1.0);
        base * scaling
    }

    pub fn add_score(&mut self, points: u64) {
        self.score += (points as f32 * self.modifiers.score_multiplier) as u64;
    }

    pub fn tick(&mut self) {
        self.elapsed_turns += 1;
    }

    pub fn is_time_up(&self) -> bool {
        if let Some(limit) = self.modifiers.time_limit {
            self.elapsed_turns >= limit
        } else {
            false
        }
    }

    pub fn start_wave(&mut self) {
        self.current_wave += 1;
        self.waves.push(Wave::new(self.current_wave));
    }

    pub fn complete_wave(&mut self) -> bool {
        if let Some(wave) = self.waves.last_mut() {
            wave.completed = true;
            return true;
        }
        false
    }
}

impl Default for TowerSystem {
    fn default() -> Self {
        Self::new(TowerMode::StoryMode)
    }
}

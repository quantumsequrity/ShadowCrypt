//! Leaderboard and high score system for ShadowCrypt
//!
//! This module provides functionality for:
//! - Score calculation based on gameplay achievements
//! - Persistent storage of high scores
//! - Ranking display and filtering

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};

use crate::classes::CharacterClass;
use crate::game::GameState;

/// Maximum number of entries to keep in the leaderboard
pub const MAX_LEADERBOARD_ENTRIES: usize = 100;

/// Default number of top scores to display
pub const DEFAULT_TOP_SCORES: usize = 10;

/// Score multipliers for calculation
mod score_weights {
    /// Points per dungeon level reached
    pub const DUNGEON_LEVEL: u64 = 1000;
    /// Bonus for defeating bosses (per boss level)
    pub const BOSS_DEFEAT: u64 = 5000;
    /// Points per enemy killed
    pub const ENEMY_KILL: u64 = 10;
    /// Points per 100 gold collected
    pub const GOLD_PER_100: u64 = 50;
    /// Points per player level gained
    pub const PLAYER_LEVEL: u64 = 500;
    /// Victory bonus multiplier (applied to total)
    pub const VICTORY_MULTIPLIER: f64 = 2.0;
    /// Speed bonus: base points divided by (turns / expected_turns)
    pub const SPEED_BONUS_BASE: u64 = 10000;
    /// Expected turns per dungeon level for speed calculation
    pub const EXPECTED_TURNS_PER_LEVEL: u32 = 200;
}

/// A single entry in the leaderboard
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct LeaderboardEntry {
    /// Player's name
    pub name: String,
    /// Calculated total score
    pub score: u64,
    /// Character class used
    pub class: CharacterClass,
    /// Deepest dungeon level reached
    pub dungeon_level: u32,
    /// Total enemies killed
    pub kills: u32,
    /// Total gold collected
    pub gold: u32,
    /// Player's final level
    pub player_level: u32,
    /// Total turns taken
    pub turn_count: u32,
    /// Whether the game was won
    pub victory: bool,
    /// Unix timestamp of when the score was achieved
    pub timestamp: u64,
    /// Detailed score breakdown for display
    pub score_breakdown: ScoreBreakdown,
}

/// Detailed breakdown of score components
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ScoreBreakdown {
    /// Points from dungeon depth
    pub depth_points: u64,
    /// Points from boss defeats
    pub boss_points: u64,
    /// Points from enemy kills
    pub kill_points: u64,
    /// Points from gold collected
    pub gold_points: u64,
    /// Points from player level
    pub level_points: u64,
    /// Speed bonus points
    pub speed_bonus: u64,
    /// Victory multiplier applied
    pub victory_bonus: u64,
}

impl ScoreBreakdown {
    /// Calculate the total from all components
    pub fn total(&self) -> u64 {
        self.depth_points
            + self.boss_points
            + self.kill_points
            + self.gold_points
            + self.level_points
            + self.speed_bonus
            + self.victory_bonus
    }
}

impl LeaderboardEntry {
    /// Create a new leaderboard entry from a finished game
    pub fn from_game(game: &GameState, player_name: &str) -> Self {
        let (score, breakdown) = calculate_score(game);

        Self {
            name: player_name.to_string(),
            score,
            class: game.player.class,
            dungeon_level: game.dungeon_level,
            kills: game.player.kills,
            gold: game.player.gold,
            player_level: game.player.level,
            turn_count: game.turn_count,
            victory: game.victory,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            score_breakdown: breakdown,
        }
    }

    /// Format the entry for display as a single line
    pub fn format_line(&self, rank: usize) -> String {
        let victory_marker = if self.victory { "*" } else { " " };
        format!(
            "{:>3}. {:>12} {:>10} pts  {} Lv{:<2} DL{:<2} {:>5} kills",
            rank,
            truncate_name(&self.name, 12),
            format_number(self.score),
            self.class.name(),
            self.player_level,
            self.dungeon_level,
            self.kills,
        ) + victory_marker
    }

    /// Format a detailed view of the entry
    pub fn format_detailed(&self) -> String {
        let datetime = format_timestamp(self.timestamp);
        let victory_status = if self.victory {
            "VICTORY!"
        } else {
            "Defeated"
        };

        format!(
            r#"
=====================================
  {} - {}
=====================================
  Player: {}
  Class:  {}
  Score:  {} points

  -- Game Stats --
  Dungeon Level: {}
  Player Level:  {}
  Enemies Slain: {}
  Gold Earned:   {}
  Turns Taken:   {}

  -- Score Breakdown --
  Depth Bonus:   {:>8} pts
  Boss Bonus:    {:>8} pts
  Kill Bonus:    {:>8} pts
  Gold Bonus:    {:>8} pts
  Level Bonus:   {:>8} pts
  Speed Bonus:   {:>8} pts
  Victory Bonus: {:>8} pts
  --------------------------
  TOTAL:         {:>8} pts
====================================="#,
            victory_status,
            datetime,
            self.name,
            self.class.name(),
            format_number(self.score),
            self.dungeon_level,
            self.player_level,
            self.kills,
            format_number(self.gold as u64),
            self.turn_count,
            format_number(self.score_breakdown.depth_points),
            format_number(self.score_breakdown.boss_points),
            format_number(self.score_breakdown.kill_points),
            format_number(self.score_breakdown.gold_points),
            format_number(self.score_breakdown.level_points),
            format_number(self.score_breakdown.speed_bonus),
            format_number(self.score_breakdown.victory_bonus),
            format_number(self.score),
        )
    }
}

/// The main leaderboard structure managing all high score entries
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Leaderboard {
    /// All stored entries, sorted by score descending
    entries: Vec<LeaderboardEntry>,
    /// Version for future compatibility
    version: u32,
}

impl Leaderboard {
    /// Create a new empty leaderboard
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            version: 1,
        }
    }

    /// Add a new entry to the leaderboard
    /// Returns the rank (1-indexed) if the entry made it onto the board
    pub fn add_entry(&mut self, entry: LeaderboardEntry) -> Option<usize> {
        // Find insertion position (maintain descending sort by score)
        let pos = self
            .entries
            .iter()
            .position(|e| e.score < entry.score)
            .unwrap_or(self.entries.len());

        // Only add if within max entries limit
        if pos >= MAX_LEADERBOARD_ENTRIES {
            return None;
        }

        self.entries.insert(pos, entry);

        // Trim to max entries
        if self.entries.len() > MAX_LEADERBOARD_ENTRIES {
            self.entries.truncate(MAX_LEADERBOARD_ENTRIES);
        }

        Some(pos + 1) // Return 1-indexed rank
    }

    /// Submit a game result and return the rank if it made the leaderboard
    pub fn submit_game(&mut self, game: &GameState, player_name: &str) -> Option<usize> {
        let entry = LeaderboardEntry::from_game(game, player_name);
        self.add_entry(entry)
    }

    /// Get the top N entries
    pub fn top(&self, count: usize) -> &[LeaderboardEntry] {
        let len = self.entries.len().min(count);
        &self.entries[..len]
    }

    /// Get all entries
    pub fn all_entries(&self) -> &[LeaderboardEntry] {
        &self.entries
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the leaderboard is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get entries filtered by character class
    pub fn by_class(&self, class: CharacterClass) -> Vec<&LeaderboardEntry> {
        self.entries.iter().filter(|e| e.class == class).collect()
    }

    /// Get entries filtered by victory status
    pub fn victories(&self) -> Vec<&LeaderboardEntry> {
        self.entries.iter().filter(|e| e.victory).collect()
    }

    /// Get a specific entry by rank (1-indexed)
    pub fn get_by_rank(&self, rank: usize) -> Option<&LeaderboardEntry> {
        if rank == 0 || rank > self.entries.len() {
            None
        } else {
            self.entries.get(rank - 1)
        }
    }

    /// Check if a score would make it onto the leaderboard
    pub fn would_qualify(&self, score: u64) -> bool {
        if self.entries.len() < MAX_LEADERBOARD_ENTRIES {
            return true;
        }
        self.entries
            .last()
            .map(|e| score > e.score)
            .unwrap_or(true)
    }

    /// Get the minimum score required to make the leaderboard
    pub fn minimum_qualifying_score(&self) -> u64 {
        if self.entries.len() < MAX_LEADERBOARD_ENTRIES {
            0
        } else {
            self.entries.last().map(|e| e.score + 1).unwrap_or(0)
        }
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Format the leaderboard for display
    pub fn format_display(&self, count: usize) -> String {
        if self.entries.is_empty() {
            return String::from(
                r#"
╔═══════════════════════════════════════════════════════════════╗
║                    SHADOWCRYPT LEADERBOARD                     ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║                    No high scores yet!                        ║
║           Complete a game to record your score.               ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝"#,
            );
        }

        let mut output = String::from(
            r#"
╔═══════════════════════════════════════════════════════════════╗
║                    SHADOWCRYPT LEADERBOARD                     ║
╠═══════════════════════════════════════════════════════════════╣
"#,
        );

        for (i, entry) in self.top(count).iter().enumerate() {
            let rank = i + 1;
            let medal = match rank {
                1 => "[1st]",
                2 => "[2nd]",
                3 => "[3rd]",
                _ => "     ",
            };
            let victory = if entry.victory { "W" } else { " " };

            output.push_str(&format!(
                "║ {} {:>3}. {:<12} {:>10} pts {:<10} Lv{:<2} DL{:<2} {} ║\n",
                medal,
                rank,
                truncate_name(&entry.name, 12),
                format_number(entry.score),
                entry.class.name(),
                entry.player_level,
                entry.dungeon_level,
                victory
            ));
        }

        output.push_str(
            "╚═══════════════════════════════════════════════════════════════╝\n",
        );
        output.push_str("  * = Victory   DL = Dungeon Level   Lv = Player Level\n");

        output
    }

    /// Format a compact leaderboard for in-game display
    pub fn format_compact(&self, count: usize) -> String {
        let mut output = String::from("=== HIGH SCORES ===\n");

        if self.entries.is_empty() {
            output.push_str("No scores yet!\n");
            return output;
        }

        for (i, entry) in self.top(count).iter().enumerate() {
            output.push_str(&entry.format_line(i + 1));
            output.push('\n');
        }

        output
    }
}

/// Calculate the score for a game state
/// Returns (total_score, breakdown)
pub fn calculate_score(game: &GameState) -> (u64, ScoreBreakdown) {
    use score_weights::*;

    let mut breakdown = ScoreBreakdown::default();

    // Depth points: deeper = better
    breakdown.depth_points = game.dungeon_level as u64 * DUNGEON_LEVEL;

    // Boss points: count defeated bosses based on level reached
    let boss_levels = [5, 10, 15, 20, 25, 30];
    let bosses_defeated = boss_levels
        .iter()
        .filter(|&&level| game.dungeon_level > level || (game.dungeon_level == level && game.boss_defeated))
        .count() as u64;
    breakdown.boss_points = bosses_defeated * BOSS_DEFEAT;

    // Kill points
    breakdown.kill_points = game.player.kills as u64 * ENEMY_KILL;

    // Gold points
    breakdown.gold_points = (game.player.gold as u64 / 100) * GOLD_PER_100;

    // Level points
    breakdown.level_points = game.player.level as u64 * PLAYER_LEVEL;

    // Speed bonus: reward efficient play
    let expected_turns = game.dungeon_level * EXPECTED_TURNS_PER_LEVEL;
    if game.turn_count > 0 && expected_turns > 0 {
        let efficiency = expected_turns as f64 / game.turn_count as f64;
        if efficiency > 0.5 {
            breakdown.speed_bonus = (SPEED_BONUS_BASE as f64 * efficiency.min(2.0)) as u64;
        }
    }

    // Calculate subtotal before victory bonus
    let subtotal = breakdown.depth_points
        + breakdown.boss_points
        + breakdown.kill_points
        + breakdown.gold_points
        + breakdown.level_points
        + breakdown.speed_bonus;

    // Victory bonus: double the score for winning
    if game.victory {
        breakdown.victory_bonus = subtotal; // Effectively doubles the score
    }

    let total = subtotal + breakdown.victory_bonus;

    (total, breakdown)
}

/// Calculate estimated score from a game (useful for preview)
pub fn estimate_score(game: &GameState) -> u64 {
    calculate_score(game).0
}

/// Error type for leaderboard operations
#[derive(Debug)]
pub enum LeaderboardError {
    IoError(std::io::Error),
    SerializeError(String),
    DeserializeError(String),
}

impl From<std::io::Error> for LeaderboardError {
    fn from(err: std::io::Error) -> Self {
        LeaderboardError::IoError(err)
    }
}

impl std::fmt::Display for LeaderboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeaderboardError::IoError(e) => write!(f, "I/O error: {}", e),
            LeaderboardError::SerializeError(e) => write!(f, "Serialization error: {}", e),
            LeaderboardError::DeserializeError(e) => write!(f, "Deserialization error: {}", e),
        }
    }
}

impl std::error::Error for LeaderboardError {}

/// Get the default leaderboard file path
pub fn default_leaderboard_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".shadowcrypt_leaderboard.dat")
}

/// Save the leaderboard to a file
pub fn save_leaderboard(
    leaderboard: &Leaderboard,
    path: Option<PathBuf>,
) -> Result<(), LeaderboardError> {
    let path = path.unwrap_or_else(default_leaderboard_path);

    // Serialize with bincode
    let data = bincode::serialize(leaderboard)
        .map_err(|e| LeaderboardError::SerializeError(e.to_string()))?;

    // Compress with gzip
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;

    // Write to file
    fs::write(path, compressed)?;

    Ok(())
}

/// Load the leaderboard from a file
pub fn load_leaderboard(path: Option<PathBuf>) -> Result<Leaderboard, LeaderboardError> {
    let path = path.unwrap_or_else(default_leaderboard_path);

    // Check if file exists
    if !path.exists() {
        return Ok(Leaderboard::new());
    }

    // Read file
    let compressed = fs::read(path)?;

    // Decompress
    let mut decoder = GzDecoder::new(&compressed[..]);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    // Deserialize
    let leaderboard: Leaderboard = bincode::deserialize(&data)
        .map_err(|e| LeaderboardError::DeserializeError(e.to_string()))?;

    Ok(leaderboard)
}

/// Check if a leaderboard file exists
pub fn leaderboard_exists(path: Option<PathBuf>) -> bool {
    let path = path.unwrap_or_else(default_leaderboard_path);
    path.exists()
}

/// Delete the leaderboard file
pub fn delete_leaderboard(path: Option<PathBuf>) -> Result<(), LeaderboardError> {
    let path = path.unwrap_or_else(default_leaderboard_path);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

// Helper functions

/// Truncate a name to fit display width
fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        format!("{:<width$}", name, width = max_len)
    } else {
        format!("{}...", &name[..max_len - 3])
    }
}

/// Format a number with thousands separators
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}

/// Format a Unix timestamp as a readable date
fn format_timestamp(timestamp: u64) -> String {
    // Simple date formatting without external dependencies
    let secs_per_day: u64 = 86400;
    let secs_per_year: u64 = 31536000;
    let secs_per_leap_year: u64 = 31622400;

    if timestamp == 0 {
        return String::from("Unknown");
    }

    // Calculate year, month, day from Unix timestamp
    let mut remaining = timestamp;
    let mut year = 1970u32;

    loop {
        let year_secs = if is_leap_year(year) {
            secs_per_leap_year
        } else {
            secs_per_year
        };
        if remaining < year_secs {
            break;
        }
        remaining -= year_secs;
        year += 1;
    }

    let days_in_months: [u64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1u32;
    for days in days_in_months.iter() {
        let month_secs = days * secs_per_day;
        if remaining < month_secs {
            break;
        }
        remaining -= month_secs;
        month += 1;
    }

    let day = (remaining / secs_per_day) + 1;

    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Check if a year is a leap year
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Statistics summary for the leaderboard
#[derive(Clone, Debug, Default)]
pub struct LeaderboardStats {
    pub total_games: usize,
    pub total_victories: usize,
    pub highest_score: u64,
    pub average_score: u64,
    pub deepest_level: u32,
    pub most_kills: u32,
    pub most_gold: u32,
    pub favorite_class: Option<CharacterClass>,
}

impl Leaderboard {
    /// Calculate statistics across all entries
    pub fn statistics(&self) -> LeaderboardStats {
        if self.entries.is_empty() {
            return LeaderboardStats::default();
        }

        let total_games = self.entries.len();
        let total_victories = self.entries.iter().filter(|e| e.victory).count();
        let highest_score = self.entries.first().map(|e| e.score).unwrap_or(0);
        let average_score = self.entries.iter().map(|e| e.score).sum::<u64>() / total_games as u64;
        let deepest_level = self.entries.iter().map(|e| e.dungeon_level).max().unwrap_or(0);
        let most_kills = self.entries.iter().map(|e| e.kills).max().unwrap_or(0);
        let most_gold = self.entries.iter().map(|e| e.gold).max().unwrap_or(0);

        // Find most used class
        let mut class_counts = std::collections::HashMap::new();
        for entry in &self.entries {
            *class_counts.entry(entry.class).or_insert(0) += 1;
        }
        let favorite_class = class_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(class, _)| class);

        LeaderboardStats {
            total_games,
            total_victories,
            highest_score,
            average_score,
            deepest_level,
            most_kills,
            most_gold,
            favorite_class,
        }
    }

    /// Format statistics for display
    pub fn format_statistics(&self) -> String {
        let stats = self.statistics();

        if self.entries.is_empty() {
            return String::from("No statistics available - play some games first!");
        }

        let favorite = stats
            .favorite_class
            .map(|c| c.name())
            .unwrap_or("None");

        format!(
            r#"
╔═══════════════════════════════════════════════════════════════╗
║                    LEADERBOARD STATISTICS                      ║
╠═══════════════════════════════════════════════════════════════╣
║  Total Games Played:    {:>10}                             ║
║  Total Victories:       {:>10}                             ║
║  Win Rate:              {:>9.1}%                             ║
║                                                               ║
║  Highest Score:         {:>10}                             ║
║  Average Score:         {:>10}                             ║
║                                                               ║
║  Deepest Level Reached: {:>10}                             ║
║  Most Kills (single):   {:>10}                             ║
║  Most Gold (single):    {:>10}                             ║
║                                                               ║
║  Favorite Class:        {:>10}                             ║
╚═══════════════════════════════════════════════════════════════╝"#,
            stats.total_games,
            stats.total_victories,
            (stats.total_victories as f64 / stats.total_games as f64) * 100.0,
            format_number(stats.highest_score),
            format_number(stats.average_score),
            stats.deepest_level,
            stats.most_kills,
            format_number(stats.most_gold as u64),
            favorite
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game(level: u32, kills: u32, gold: u32, victory: bool) -> GameState {
        let mut game = GameState::new(CharacterClass::Warrior);
        game.dungeon_level = level;
        game.player.kills = kills;
        game.player.gold = gold;
        game.player.level = level / 2 + 1;
        game.turn_count = level * 150;
        game.victory = victory;
        game
    }

    #[test]
    fn test_score_calculation() {
        let game = create_test_game(10, 50, 1000, false);
        let (score, breakdown) = calculate_score(&game);

        assert!(score > 0);
        assert!(breakdown.depth_points > 0);
        assert!(breakdown.kill_points > 0);
        assert!(breakdown.gold_points > 0);
        assert_eq!(breakdown.total(), score);
    }

    #[test]
    fn test_victory_bonus() {
        let game_loss = create_test_game(10, 50, 1000, false);
        let game_win = create_test_game(10, 50, 1000, true);

        let (score_loss, _) = calculate_score(&game_loss);
        let (score_win, _) = calculate_score(&game_win);

        assert!(score_win > score_loss);
        // Victory should roughly double the score
        assert!(score_win >= score_loss * 2 - 1000); // Allow some tolerance
    }

    #[test]
    fn test_leaderboard_ordering() {
        let mut leaderboard = Leaderboard::new();

        let entry1 = LeaderboardEntry {
            name: "Player1".to_string(),
            score: 5000,
            class: CharacterClass::Warrior,
            dungeon_level: 5,
            kills: 20,
            gold: 500,
            player_level: 3,
            turn_count: 500,
            victory: false,
            timestamp: 1000,
            score_breakdown: ScoreBreakdown::default(),
        };

        let entry2 = LeaderboardEntry {
            name: "Player2".to_string(),
            score: 10000,
            class: CharacterClass::Mage,
            dungeon_level: 10,
            kills: 50,
            gold: 1000,
            player_level: 6,
            turn_count: 1000,
            victory: false,
            timestamp: 2000,
            score_breakdown: ScoreBreakdown::default(),
        };

        leaderboard.add_entry(entry1);
        leaderboard.add_entry(entry2);

        // Higher score should be first
        assert_eq!(leaderboard.entries[0].score, 10000);
        assert_eq!(leaderboard.entries[1].score, 5000);
    }

    #[test]
    fn test_leaderboard_max_entries() {
        let mut leaderboard = Leaderboard::new();

        // Add more than max entries
        for i in 0..MAX_LEADERBOARD_ENTRIES + 10 {
            let entry = LeaderboardEntry {
                name: format!("Player{}", i),
                score: (i as u64 + 1) * 100,
                class: CharacterClass::Warrior,
                dungeon_level: 1,
                kills: 0,
                gold: 0,
                player_level: 1,
                turn_count: 100,
                victory: false,
                timestamp: i as u64,
                score_breakdown: ScoreBreakdown::default(),
            };
            leaderboard.add_entry(entry);
        }

        assert_eq!(leaderboard.len(), MAX_LEADERBOARD_ENTRIES);
    }

    #[test]
    fn test_filter_by_class() {
        let mut leaderboard = Leaderboard::new();

        for class in [CharacterClass::Warrior, CharacterClass::Mage, CharacterClass::Warrior] {
            let entry = LeaderboardEntry {
                name: "Test".to_string(),
                score: 1000,
                class,
                dungeon_level: 5,
                kills: 10,
                gold: 100,
                player_level: 3,
                turn_count: 300,
                victory: false,
                timestamp: 0,
                score_breakdown: ScoreBreakdown::default(),
            };
            leaderboard.add_entry(entry);
        }

        let warriors = leaderboard.by_class(CharacterClass::Warrior);
        assert_eq!(warriors.len(), 2);

        let mages = leaderboard.by_class(CharacterClass::Mage);
        assert_eq!(mages.len(), 1);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(1000000), "1,000,000");
        assert_eq!(format_number(42), "42");
    }

    #[test]
    fn test_truncate_name() {
        assert_eq!(truncate_name("Short", 10), "Short     ");
        assert_eq!(truncate_name("VeryLongPlayerName", 10), "VeryLon...");
    }

    #[test]
    fn test_would_qualify() {
        let mut leaderboard = Leaderboard::new();

        // Empty leaderboard should accept any score
        assert!(leaderboard.would_qualify(0));

        // Add entries up to max
        for i in 0..MAX_LEADERBOARD_ENTRIES {
            let entry = LeaderboardEntry {
                name: format!("Player{}", i),
                score: 1000 + i as u64,
                class: CharacterClass::Warrior,
                dungeon_level: 1,
                kills: 0,
                gold: 0,
                player_level: 1,
                turn_count: 100,
                victory: false,
                timestamp: 0,
                score_breakdown: ScoreBreakdown::default(),
            };
            leaderboard.add_entry(entry);
        }

        // Score lower than minimum should not qualify
        assert!(!leaderboard.would_qualify(500));
        // Score higher than minimum should qualify
        assert!(leaderboard.would_qualify(2000));
    }

    #[test]
    fn test_statistics() {
        let mut leaderboard = Leaderboard::new();

        let entry1 = LeaderboardEntry {
            name: "Winner".to_string(),
            score: 50000,
            class: CharacterClass::Warrior,
            dungeon_level: 30,
            kills: 200,
            gold: 5000,
            player_level: 15,
            turn_count: 5000,
            victory: true,
            timestamp: 1000,
            score_breakdown: ScoreBreakdown::default(),
        };

        let entry2 = LeaderboardEntry {
            name: "Loser".to_string(),
            score: 10000,
            class: CharacterClass::Warrior,
            dungeon_level: 10,
            kills: 50,
            gold: 1000,
            player_level: 5,
            turn_count: 1500,
            victory: false,
            timestamp: 2000,
            score_breakdown: ScoreBreakdown::default(),
        };

        leaderboard.add_entry(entry1);
        leaderboard.add_entry(entry2);

        let stats = leaderboard.statistics();
        assert_eq!(stats.total_games, 2);
        assert_eq!(stats.total_victories, 1);
        assert_eq!(stats.highest_score, 50000);
        assert_eq!(stats.deepest_level, 30);
        assert_eq!(stats.most_kills, 200);
        assert_eq!(stats.favorite_class, Some(CharacterClass::Warrior));
    }
}

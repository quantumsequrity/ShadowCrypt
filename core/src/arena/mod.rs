//! Arena and PvP System
//!
//! A comprehensive arena system featuring multiple arena types, ranked play,
//! tournaments, spectator features, and AI opponents.
//!
//! # Arena Types
//! - Dueling Arena (1v1)
//! - Team Arena (3v3, 5v5)
//! - Battle Royale (last one standing)
//! - Tournament Brackets
//! - Ranked Ladder
//! - Sect Wars
//! - Kingdom Wars
//!
//! # Features
//! - Multiple match types (casual, ranked, wager, death match, honor duel)
//! - Comprehensive ranking system from Bronze to Legend
//! - Spectator system with betting and replays
//! - AI opponents with varied fighting styles

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

// =============================================================================
// Constants
// =============================================================================

/// Base arena points per win
pub const BASE_ARENA_POINTS: u32 = 10;
/// Rating change on win
pub const RATING_WIN_BASE: i32 = 25;
/// Rating change on loss
pub const RATING_LOSS_BASE: i32 = -20;
/// Maximum wager amount
pub const MAX_WAGER: u64 = 100_000;
/// Minimum wager amount
pub const MIN_WAGER: u64 = 100;
/// Battle royale minimum players
pub const BATTLE_ROYALE_MIN_PLAYERS: usize = 10;
/// Battle royale maximum players
pub const BATTLE_ROYALE_MAX_PLAYERS: usize = 100;
/// Tournament minimum participants
pub const TOURNAMENT_MIN_PARTICIPANTS: usize = 8;
/// Maximum spectator bet
pub const MAX_SPECTATOR_BET: u64 = 10_000;

// =============================================================================
// Arena Types
// =============================================================================

/// Types of arenas available in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArenaType {
    /// Classic 1v1 combat
    DuelingArena,
    /// Team-based 3v3 combat
    TeamArena3v3,
    /// Team-based 5v5 combat
    TeamArena5v5,
    /// Free-for-all until one remains
    BattleRoyale,
    /// Single elimination tournament
    TournamentSingle,
    /// Double elimination tournament
    TournamentDouble,
    /// Ranked ladder play
    RankedLadder,
    /// Sect vs Sect combat
    SectWars,
    /// Kingdom vs Kingdom combat
    KingdomWars,
}

impl ArenaType {
    /// Returns all arena types
    pub fn all() -> &'static [ArenaType] {
        &[
            Self::DuelingArena,
            Self::TeamArena3v3,
            Self::TeamArena5v5,
            Self::BattleRoyale,
            Self::TournamentSingle,
            Self::TournamentDouble,
            Self::RankedLadder,
            Self::SectWars,
            Self::KingdomWars,
        ]
    }

    /// Display name for the arena type
    pub fn name(&self) -> &'static str {
        match self {
            Self::DuelingArena => "Dueling Arena",
            Self::TeamArena3v3 => "Team Arena (3v3)",
            Self::TeamArena5v5 => "Team Arena (5v5)",
            Self::BattleRoyale => "Battle Royale",
            Self::TournamentSingle => "Tournament (Single Elimination)",
            Self::TournamentDouble => "Tournament (Double Elimination)",
            Self::RankedLadder => "Ranked Ladder",
            Self::SectWars => "Sect Wars",
            Self::KingdomWars => "Kingdom Wars",
        }
    }

    /// Description of the arena type
    pub fn description(&self) -> &'static str {
        match self {
            Self::DuelingArena => "Classic one-on-one combat to test your skill against a single opponent.",
            Self::TeamArena3v3 => "Coordinate with two allies to defeat the enemy team in tactical combat.",
            Self::TeamArena5v5 => "Large-scale team battles requiring coordination and strategy.",
            Self::BattleRoyale => "A free-for-all where only the last warrior standing claims victory.",
            Self::TournamentSingle => "Single elimination bracket - one loss and you're out.",
            Self::TournamentDouble => "Double elimination bracket - two losses to be eliminated.",
            Self::RankedLadder => "Climb the competitive ladder and prove your worth.",
            Self::SectWars => "Battle alongside your sect members for glory and territory.",
            Self::KingdomWars => "Fight for your kingdom in large-scale warfare.",
        }
    }

    /// Required players per side
    pub fn players_per_side(&self) -> usize {
        match self {
            Self::DuelingArena | Self::RankedLadder => 1,
            Self::TeamArena3v3 => 3,
            Self::TeamArena5v5 => 5,
            Self::BattleRoyale => 1, // Individual
            Self::TournamentSingle | Self::TournamentDouble => 1, // Usually 1v1
            Self::SectWars => 10,
            Self::KingdomWars => 20,
        }
    }

    /// Whether this arena type supports teams
    pub fn is_team_based(&self) -> bool {
        matches!(
            self,
            Self::TeamArena3v3 | Self::TeamArena5v5 | Self::SectWars | Self::KingdomWars
        )
    }

    /// Entry fee for this arena type
    pub fn entry_fee(&self) -> u64 {
        match self {
            Self::DuelingArena => 50,
            Self::TeamArena3v3 => 100,
            Self::TeamArena5v5 => 150,
            Self::BattleRoyale => 500,
            Self::TournamentSingle => 1000,
            Self::TournamentDouble => 1500,
            Self::RankedLadder => 0,
            Self::SectWars => 0, // Free but requires sect membership
            Self::KingdomWars => 0, // Free but requires kingdom citizenship
        }
    }

    /// Minimum level required to enter
    pub fn min_level(&self) -> u32 {
        match self {
            Self::DuelingArena => 5,
            Self::TeamArena3v3 => 10,
            Self::TeamArena5v5 => 15,
            Self::BattleRoyale => 20,
            Self::TournamentSingle => 25,
            Self::TournamentDouble => 25,
            Self::RankedLadder => 30,
            Self::SectWars => 40,
            Self::KingdomWars => 50,
        }
    }
}

// =============================================================================
// Arena Ranks
// =============================================================================

/// Tier within a rank (I is highest, III is lowest)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RankTier {
    III = 0,
    II = 1,
    I = 2,
}

impl RankTier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::I => "I",
            Self::II => "II",
            Self::III => "III",
        }
    }

    pub fn next(&self) -> Option<RankTier> {
        match self {
            Self::III => Some(Self::II),
            Self::II => Some(Self::I),
            Self::I => None,
        }
    }

    pub fn prev(&self) -> Option<RankTier> {
        match self {
            Self::I => Some(Self::II),
            Self::II => Some(Self::III),
            Self::III => None,
        }
    }
}

/// Arena ranks from lowest to highest
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ArenaRankTier {
    Bronze = 0,
    Silver = 1,
    Gold = 2,
    Platinum = 3,
    Diamond = 4,
    Master = 5,
    Grandmaster = 6,
    Champion = 7,
    Legend = 8,
}

impl ArenaRankTier {
    pub fn all() -> &'static [ArenaRankTier] {
        &[
            Self::Bronze,
            Self::Silver,
            Self::Gold,
            Self::Platinum,
            Self::Diamond,
            Self::Master,
            Self::Grandmaster,
            Self::Champion,
            Self::Legend,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Bronze => "Bronze",
            Self::Silver => "Silver",
            Self::Gold => "Gold",
            Self::Platinum => "Platinum",
            Self::Diamond => "Diamond",
            Self::Master => "Master",
            Self::Grandmaster => "Grandmaster",
            Self::Champion => "Champion",
            Self::Legend => "Legend",
        }
    }

    /// Whether this rank has tiers (I, II, III)
    pub fn has_tiers(&self) -> bool {
        matches!(
            self,
            Self::Bronze | Self::Silver | Self::Gold | Self::Platinum | Self::Diamond
        )
    }

    /// Rating threshold for this rank
    pub fn rating_threshold(&self) -> i32 {
        match self {
            Self::Bronze => 0,
            Self::Silver => 400,
            Self::Gold => 800,
            Self::Platinum => 1200,
            Self::Diamond => 1600,
            Self::Master => 2000,
            Self::Grandmaster => 2400,
            Self::Champion => 2800,
            Self::Legend => 3200,
        }
    }

    /// Rating per tier within this rank
    pub fn rating_per_tier(&self) -> i32 {
        if self.has_tiers() {
            (self.next_rank_threshold() - self.rating_threshold()) / 3
        } else {
            0
        }
    }

    fn next_rank_threshold(&self) -> i32 {
        match self {
            Self::Bronze => 400,
            Self::Silver => 800,
            Self::Gold => 1200,
            Self::Platinum => 1600,
            Self::Diamond => 2000,
            Self::Master => 2400,
            Self::Grandmaster => 2800,
            Self::Champion => 3200,
            Self::Legend => 4000,
        }
    }

    pub fn next(&self) -> Option<ArenaRankTier> {
        match self {
            Self::Bronze => Some(Self::Silver),
            Self::Silver => Some(Self::Gold),
            Self::Gold => Some(Self::Platinum),
            Self::Platinum => Some(Self::Diamond),
            Self::Diamond => Some(Self::Master),
            Self::Master => Some(Self::Grandmaster),
            Self::Grandmaster => Some(Self::Champion),
            Self::Champion => Some(Self::Legend),
            Self::Legend => None,
        }
    }

    pub fn prev(&self) -> Option<ArenaRankTier> {
        match self {
            Self::Bronze => None,
            Self::Silver => Some(Self::Bronze),
            Self::Gold => Some(Self::Silver),
            Self::Platinum => Some(Self::Gold),
            Self::Diamond => Some(Self::Platinum),
            Self::Master => Some(Self::Diamond),
            Self::Grandmaster => Some(Self::Master),
            Self::Champion => Some(Self::Grandmaster),
            Self::Legend => Some(Self::Champion),
        }
    }

    /// Color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Bronze => 12,      // DarkYellow (bronze-ish)
            Self::Silver => 1,       // Grey
            Self::Gold => 11,        // Yellow
            Self::Platinum => 9,     // Cyan
            Self::Diamond => 7,      // Blue
            Self::Master => 13,      // Magenta
            Self::Grandmaster => 3,  // Red
            Self::Champion => 5,     // Green
            Self::Legend => 11,      // Bright Yellow (golden)
        }
    }
}

impl Default for ArenaRankTier {
    fn default() -> Self {
        Self::Bronze
    }
}

/// Complete arena rank including tier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaRank {
    pub tier: ArenaRankTier,
    pub sub_tier: Option<RankTier>,
}

impl ArenaRank {
    pub fn new(tier: ArenaRankTier, sub_tier: Option<RankTier>) -> Self {
        let sub_tier = if tier.has_tiers() {
            sub_tier.or(Some(RankTier::III))
        } else {
            None
        };
        Self { tier, sub_tier }
    }

    pub fn from_rating(rating: i32) -> Self {
        let tier = ArenaRankTier::all()
            .iter()
            .rev()
            .find(|t| rating >= t.rating_threshold())
            .copied()
            .unwrap_or(ArenaRankTier::Bronze);

        let sub_tier = if tier.has_tiers() {
            let rating_in_tier = rating - tier.rating_threshold();
            let per_tier = tier.rating_per_tier();
            if per_tier > 0 {
                let tier_num = (rating_in_tier / per_tier).min(2) as u8;
                Some(match tier_num {
                    0 => RankTier::III,
                    1 => RankTier::II,
                    _ => RankTier::I,
                })
            } else {
                Some(RankTier::III)
            }
        } else {
            None
        };

        Self { tier, sub_tier }
    }

    pub fn display_name(&self) -> String {
        match self.sub_tier {
            Some(sub) => format!("{} {}", self.tier.name(), sub.name()),
            None => self.tier.name().to_string(),
        }
    }

    /// Returns default starting rank
    pub fn default_rank() -> Self {
        Self::new(ArenaRankTier::Bronze, Some(RankTier::III))
    }
}

impl Default for ArenaRank {
    fn default() -> Self {
        Self::default_rank()
    }
}

// =============================================================================
// Match Types
// =============================================================================

/// Types of matches that can be played
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchType {
    /// No penalties for losing
    Casual,
    /// Gain or lose rating based on result
    Ranked,
    /// Bet items or gold on the outcome
    Wager,
    /// Permanent death on loss
    DeathMatch,
    /// Reputation at stake
    HonorDuel,
}

impl MatchType {
    pub fn all() -> &'static [MatchType] {
        &[
            Self::Casual,
            Self::Ranked,
            Self::Wager,
            Self::DeathMatch,
            Self::HonorDuel,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Casual => "Casual",
            Self::Ranked => "Ranked",
            Self::Wager => "Wager Match",
            Self::DeathMatch => "Death Match",
            Self::HonorDuel => "Honor Duel",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Casual => "A friendly match with no stakes. Perfect for practice.",
            Self::Ranked => "Compete for rating points and climb the ladder.",
            Self::Wager => "Put your gold or items on the line. Winner takes all.",
            Self::DeathMatch => "The ultimate test. Lose and face permanent death.",
            Self::HonorDuel => "Fight for honor. Your reputation is at stake.",
        }
    }

    /// Whether this match affects rating
    pub fn affects_rating(&self) -> bool {
        matches!(self, Self::Ranked | Self::HonorDuel)
    }

    /// Whether death is permanent
    pub fn is_permadeath(&self) -> bool {
        matches!(self, Self::DeathMatch)
    }

    /// Required level to access this match type
    pub fn min_level(&self) -> u32 {
        match self {
            Self::Casual => 5,
            Self::Ranked => 15,
            Self::Wager => 20,
            Self::DeathMatch => 50,
            Self::HonorDuel => 30,
        }
    }
}

// =============================================================================
// Wager System
// =============================================================================

/// Items or currency wagered in a match
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wager {
    /// Gold amount wagered
    pub gold: u64,
    /// Item IDs wagered
    pub items: Vec<u64>,
    /// Total estimated value
    pub total_value: u64,
}

impl Wager {
    pub fn new() -> Self {
        Self {
            gold: 0,
            items: Vec::new(),
            total_value: 0,
        }
    }

    pub fn with_gold(gold: u64) -> Self {
        Self {
            gold,
            items: Vec::new(),
            total_value: gold,
        }
    }

    pub fn add_gold(&mut self, amount: u64) {
        self.gold += amount;
        self.total_value += amount;
    }

    pub fn add_item(&mut self, item_id: u64, value: u64) {
        self.items.push(item_id);
        self.total_value += value;
    }

    pub fn is_valid(&self) -> bool {
        self.total_value >= MIN_WAGER && self.total_value <= MAX_WAGER
    }
}

impl Default for Wager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Arena Rewards
// =============================================================================

/// Types of arena rewards
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ArenaReward {
    /// Arena points for the arena shop
    ArenaPoints(u32),
    /// Gold currency
    Gold(u64),
    /// Experience points
    Experience(u64),
    /// Exclusive item by ID
    ExclusiveItem { item_id: u64, name: String },
    /// Title unlocked
    Title { id: u32, name: String },
    /// Rating points (for ranked)
    RatingPoints(i32),
    /// Season reward
    SeasonReward { season: u32, tier: ArenaRankTier },
    /// Achievement unlock
    Achievement { id: u32, name: String },
    /// Cosmetic item
    Cosmetic { id: u64, name: String, cosmetic_type: CosmeticType },
}

/// Types of cosmetic rewards
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CosmeticType {
    Skin,
    Aura,
    Border,
    Badge,
    Title,
    Emote,
    Banner,
}

/// Reward tier for rank-based rewards
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankReward {
    pub rank: ArenaRankTier,
    pub arena_points: u32,
    pub gold: u64,
    pub exclusive_items: Vec<String>,
    pub title: Option<String>,
    pub cosmetics: Vec<String>,
}

impl RankReward {
    pub fn for_rank(rank: ArenaRankTier) -> Self {
        match rank {
            ArenaRankTier::Bronze => Self {
                rank,
                arena_points: 100,
                gold: 500,
                exclusive_items: vec![],
                title: None,
                cosmetics: vec!["Bronze Border".to_string()],
            },
            ArenaRankTier::Silver => Self {
                rank,
                arena_points: 250,
                gold: 1000,
                exclusive_items: vec!["Silver Arena Gauntlets".to_string()],
                title: Some("Silver Combatant".to_string()),
                cosmetics: vec!["Silver Border".to_string()],
            },
            ArenaRankTier::Gold => Self {
                rank,
                arena_points: 500,
                gold: 2500,
                exclusive_items: vec!["Golden Arena Blade".to_string()],
                title: Some("Gold Warrior".to_string()),
                cosmetics: vec!["Gold Border".to_string(), "Gold Aura".to_string()],
            },
            ArenaRankTier::Platinum => Self {
                rank,
                arena_points: 1000,
                gold: 5000,
                exclusive_items: vec!["Platinum Arena Set".to_string()],
                title: Some("Platinum Champion".to_string()),
                cosmetics: vec!["Platinum Border".to_string(), "Platinum Aura".to_string()],
            },
            ArenaRankTier::Diamond => Self {
                rank,
                arena_points: 2000,
                gold: 10000,
                exclusive_items: vec!["Diamond Arena Armor".to_string()],
                title: Some("Diamond Elite".to_string()),
                cosmetics: vec![
                    "Diamond Border".to_string(),
                    "Diamond Aura".to_string(),
                    "Diamond Banner".to_string(),
                ],
            },
            ArenaRankTier::Master => Self {
                rank,
                arena_points: 3500,
                gold: 25000,
                exclusive_items: vec!["Master's Blade".to_string(), "Master's Sigil".to_string()],
                title: Some("Arena Master".to_string()),
                cosmetics: vec![
                    "Master Border".to_string(),
                    "Master Aura".to_string(),
                    "Master Banner".to_string(),
                ],
            },
            ArenaRankTier::Grandmaster => Self {
                rank,
                arena_points: 5000,
                gold: 50000,
                exclusive_items: vec![
                    "Grandmaster's Weapon".to_string(),
                    "Grandmaster's Armor".to_string(),
                ],
                title: Some("Grandmaster".to_string()),
                cosmetics: vec![
                    "Grandmaster Border".to_string(),
                    "Grandmaster Aura".to_string(),
                    "Exclusive Emote Pack".to_string(),
                ],
            },
            ArenaRankTier::Champion => Self {
                rank,
                arena_points: 7500,
                gold: 100000,
                exclusive_items: vec![
                    "Champion's Regalia".to_string(),
                    "Champion's Trophy".to_string(),
                ],
                title: Some("Champion of the Arena".to_string()),
                cosmetics: vec![
                    "Champion Border".to_string(),
                    "Champion Aura".to_string(),
                    "Champion Mount Skin".to_string(),
                ],
            },
            ArenaRankTier::Legend => Self {
                rank,
                arena_points: 10000,
                gold: 250000,
                exclusive_items: vec![
                    "Legendary Arena Set".to_string(),
                    "Legend's Crown".to_string(),
                    "Legend's Banner".to_string(),
                ],
                title: Some("Living Legend".to_string()),
                cosmetics: vec![
                    "Legendary Border".to_string(),
                    "Legendary Aura".to_string(),
                    "Legendary Mount".to_string(),
                    "Hall of Legends Entry".to_string(),
                ],
            },
        }
    }
}

// =============================================================================
// Spectator System
// =============================================================================

/// Spectator viewing a match
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spectator {
    pub player_id: u64,
    pub name: String,
    pub bet: Option<SpectatorBet>,
    pub joined_at: u64,
}

/// Bet placed by a spectator
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectatorBet {
    pub amount: u64,
    pub target_player_id: u64,
    pub odds: f32,
    pub potential_payout: u64,
}

impl SpectatorBet {
    pub fn new(amount: u64, target_player_id: u64, odds: f32) -> Self {
        let potential_payout = (amount as f32 * odds) as u64;
        Self {
            amount,
            target_player_id,
            odds,
            potential_payout,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.amount > 0 && self.amount <= MAX_SPECTATOR_BET && self.odds > 0.0
    }
}

/// Match replay data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchReplay {
    pub match_id: u64,
    pub arena_type: ArenaType,
    pub match_type: MatchType,
    pub participants: Vec<ReplayParticipant>,
    pub actions: Vec<ReplayAction>,
    pub winner_id: Option<u64>,
    pub duration_turns: u32,
    pub recorded_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayParticipant {
    pub player_id: u64,
    pub name: String,
    pub class: String,
    pub level: u32,
    pub starting_hp: i32,
    pub starting_mp: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayAction {
    pub turn: u32,
    pub actor_id: u64,
    pub action: CombatAction,
    pub target_id: Option<u64>,
    pub damage_dealt: Option<i32>,
    pub damage_taken: Option<i32>,
    pub status_applied: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CombatAction {
    Attack,
    Skill { name: String },
    Spell { name: String },
    UseItem { name: String },
    Move { x: i32, y: i32 },
    Defend,
    Dodge,
    Flee,
    Surrender,
}

/// Spectator system for watching matches
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SpectatorSystem {
    pub active_spectators: HashMap<u64, Vec<Spectator>>,
    pub betting_pool: HashMap<u64, u64>,
    pub saved_replays: VecDeque<MatchReplay>,
    pub max_saved_replays: usize,
    pub featured_matches: Vec<u64>,
}

impl SpectatorSystem {
    pub fn new() -> Self {
        Self {
            active_spectators: HashMap::new(),
            betting_pool: HashMap::new(),
            saved_replays: VecDeque::new(),
            max_saved_replays: 100,
            featured_matches: Vec::new(),
        }
    }

    pub fn add_spectator(&mut self, match_id: u64, spectator: Spectator) {
        self.active_spectators
            .entry(match_id)
            .or_default()
            .push(spectator);
    }

    pub fn remove_spectator(&mut self, match_id: u64, player_id: u64) {
        if let Some(spectators) = self.active_spectators.get_mut(&match_id) {
            spectators.retain(|s| s.player_id != player_id);
        }
    }

    pub fn place_bet(&mut self, match_id: u64, spectator_id: u64, bet: SpectatorBet) -> bool {
        if !bet.is_valid() {
            return false;
        }

        if let Some(spectators) = self.active_spectators.get_mut(&match_id) {
            if let Some(spectator) = spectators.iter_mut().find(|s| s.player_id == spectator_id) {
                if spectator.bet.is_none() {
                    *self.betting_pool.entry(match_id).or_default() += bet.amount;
                    spectator.bet = Some(bet);
                    return true;
                }
            }
        }
        false
    }

    pub fn calculate_odds(&self, match_id: u64, player_id: u64) -> f32 {
        // Default odds based on total bets
        let total_pool = self.betting_pool.get(&match_id).copied().unwrap_or(0);
        if total_pool == 0 {
            return 2.0; // Default even odds
        }

        let player_bets: u64 = self
            .active_spectators
            .get(&match_id)
            .map(|specs| {
                specs
                    .iter()
                    .filter_map(|s| s.bet.as_ref())
                    .filter(|b| b.target_player_id == player_id)
                    .map(|b| b.amount)
                    .sum()
            })
            .unwrap_or(0);

        if player_bets == 0 {
            return 3.0; // Underdog odds
        }

        (total_pool as f32 / player_bets as f32).max(1.1)
    }

    pub fn save_replay(&mut self, replay: MatchReplay) {
        if self.saved_replays.len() >= self.max_saved_replays {
            self.saved_replays.pop_front();
        }
        self.saved_replays.push_back(replay);
    }

    pub fn get_replay(&self, match_id: u64) -> Option<&MatchReplay> {
        self.saved_replays.iter().find(|r| r.match_id == match_id)
    }

    pub fn spectator_count(&self, match_id: u64) -> usize {
        self.active_spectators
            .get(&match_id)
            .map(|s| s.len())
            .unwrap_or(0)
    }

    pub fn feature_match(&mut self, match_id: u64) {
        if !self.featured_matches.contains(&match_id) {
            self.featured_matches.push(match_id);
        }
    }
}

// =============================================================================
// Arena AI Opponents
// =============================================================================

/// AI opponent fighting styles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AIFightingStyle {
    /// Balanced approach
    Balanced,
    /// Aggressive, high damage
    Aggressive,
    /// Defensive, high survival
    Defensive,
    /// Relies on combos and timing
    Technical,
    /// Uses magic and spells
    Caster,
    /// Unpredictable patterns
    Chaotic,
    /// Counter-focused
    Reactive,
    /// Speed and evasion
    Evasive,
    /// Heavy hits, slow attacks
    Berserker,
    /// Status effects and debuffs
    Controller,
}

impl AIFightingStyle {
    pub fn all() -> &'static [AIFightingStyle] {
        &[
            Self::Balanced,
            Self::Aggressive,
            Self::Defensive,
            Self::Technical,
            Self::Caster,
            Self::Chaotic,
            Self::Reactive,
            Self::Evasive,
            Self::Berserker,
            Self::Controller,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Balanced => "Balanced",
            Self::Aggressive => "Aggressive",
            Self::Defensive => "Defensive",
            Self::Technical => "Technical",
            Self::Caster => "Caster",
            Self::Chaotic => "Chaotic",
            Self::Reactive => "Reactive",
            Self::Evasive => "Evasive",
            Self::Berserker => "Berserker",
            Self::Controller => "Controller",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Balanced => "A well-rounded fighter with no glaring weaknesses.",
            Self::Aggressive => "Relentless offense focused on overwhelming damage.",
            Self::Defensive => "Patient fighter who waits for openings.",
            Self::Technical => "Precision fighter who chains devastating combos.",
            Self::Caster => "Spell-focused fighter with powerful magic.",
            Self::Chaotic => "Unpredictable patterns that are hard to read.",
            Self::Reactive => "Counter-specialist who punishes mistakes.",
            Self::Evasive => "Nimble fighter who avoids damage entirely.",
            Self::Berserker => "Trades defense for devastating power.",
            Self::Controller => "Debilitates enemies with status effects.",
        }
    }

    /// Attack priority (0.0 to 1.0)
    pub fn attack_priority(&self) -> f32 {
        match self {
            Self::Aggressive | Self::Berserker => 0.9,
            Self::Balanced | Self::Technical => 0.6,
            Self::Chaotic => 0.5,
            Self::Caster | Self::Controller => 0.4,
            Self::Defensive | Self::Reactive | Self::Evasive => 0.3,
        }
    }

    /// Defense priority
    pub fn defense_priority(&self) -> f32 {
        match self {
            Self::Defensive => 0.9,
            Self::Reactive | Self::Evasive => 0.7,
            Self::Balanced | Self::Controller => 0.5,
            Self::Technical | Self::Caster => 0.4,
            Self::Chaotic => 0.3,
            Self::Aggressive | Self::Berserker => 0.1,
        }
    }

    /// Skill usage priority
    pub fn skill_priority(&self) -> f32 {
        match self {
            Self::Technical | Self::Caster | Self::Controller => 0.8,
            Self::Balanced | Self::Reactive => 0.5,
            Self::Evasive | Self::Chaotic => 0.4,
            Self::Aggressive | Self::Defensive => 0.3,
            Self::Berserker => 0.2,
        }
    }
}

/// AI difficulty levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AIDifficulty {
    Training = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
    Expert = 4,
    Master = 5,
    Legendary = 6,
}

impl AIDifficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Training => "Training",
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::Expert => "Expert",
            Self::Master => "Master",
            Self::Legendary => "Legendary",
        }
    }

    /// Stat multiplier for this difficulty
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Training => 0.5,
            Self::Easy => 0.75,
            Self::Normal => 1.0,
            Self::Hard => 1.25,
            Self::Expert => 1.5,
            Self::Master => 1.75,
            Self::Legendary => 2.0,
        }
    }

    /// AI reaction time modifier (lower = faster)
    pub fn reaction_modifier(&self) -> f32 {
        match self {
            Self::Training => 2.0,
            Self::Easy => 1.5,
            Self::Normal => 1.0,
            Self::Hard => 0.8,
            Self::Expert => 0.6,
            Self::Master => 0.4,
            Self::Legendary => 0.2,
        }
    }

    /// Chance of AI making optimal decision
    pub fn optimal_chance(&self) -> f32 {
        match self {
            Self::Training => 0.2,
            Self::Easy => 0.4,
            Self::Normal => 0.6,
            Self::Hard => 0.75,
            Self::Expert => 0.85,
            Self::Master => 0.95,
            Self::Legendary => 0.99,
        }
    }

    /// Arena points reward multiplier
    pub fn reward_multiplier(&self) -> f32 {
        match self {
            Self::Training => 0.25,
            Self::Easy => 0.5,
            Self::Normal => 1.0,
            Self::Hard => 1.5,
            Self::Expert => 2.0,
            Self::Master => 3.0,
            Self::Legendary => 5.0,
        }
    }
}

/// Arena AI opponent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArenaAI {
    pub id: u64,
    pub name: String,
    pub title: Option<String>,
    pub level: u32,
    pub fighting_style: AIFightingStyle,
    pub difficulty: AIDifficulty,
    pub base_hp: i32,
    pub base_mp: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub base_speed: i32,
    pub skills: Vec<AISkill>,
    pub special_moves: Vec<SpecialMove>,
    pub weaknesses: Vec<String>,
    pub resistances: Vec<String>,
    pub lore: String,
    pub victory_quote: String,
    pub defeat_quote: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AISkill {
    pub name: String,
    pub damage_multiplier: f32,
    pub mp_cost: i32,
    pub cooldown: u32,
    pub current_cooldown: u32,
    pub effect: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecialMove {
    pub name: String,
    pub description: String,
    pub damage_multiplier: f32,
    pub hp_threshold: f32, // Triggers below this HP percentage
    pub uses_per_fight: u32,
    pub uses_remaining: u32,
}

impl ArenaAI {
    pub fn training_dummy() -> Self {
        Self {
            id: 0,
            name: "Training Dummy".to_string(),
            title: None,
            level: 1,
            fighting_style: AIFightingStyle::Balanced,
            difficulty: AIDifficulty::Training,
            base_hp: 100,
            base_mp: 50,
            base_attack: 5,
            base_defense: 5,
            base_speed: 5,
            skills: vec![],
            special_moves: vec![],
            weaknesses: vec![],
            resistances: vec![],
            lore: "A simple training dummy for practicing combat.".to_string(),
            victory_quote: "...".to_string(),
            defeat_quote: "...".to_string(),
        }
    }

    pub fn create_opponent(name: &str, level: u32, style: AIFightingStyle, difficulty: AIDifficulty) -> Self {
        let mult = difficulty.stat_multiplier();
        Self {
            id: rand::random(),
            name: name.to_string(),
            title: None,
            level,
            fighting_style: style,
            difficulty,
            base_hp: ((100 + level as i32 * 10) as f32 * mult) as i32,
            base_mp: ((50 + level as i32 * 5) as f32 * mult) as i32,
            base_attack: ((10 + level as i32 * 2) as f32 * mult) as i32,
            base_defense: ((5 + level as i32) as f32 * mult) as i32,
            base_speed: ((10 + level as i32 / 2) as f32 * mult) as i32,
            skills: Vec::new(),
            special_moves: Vec::new(),
            weaknesses: Vec::new(),
            resistances: Vec::new(),
            lore: String::new(),
            victory_quote: "A worthy opponent.".to_string(),
            defeat_quote: "Well fought...".to_string(),
        }
    }

    pub fn effective_hp(&self) -> i32 {
        (self.base_hp as f32 * self.difficulty.stat_multiplier()) as i32
    }

    pub fn effective_attack(&self) -> i32 {
        (self.base_attack as f32 * self.difficulty.stat_multiplier()) as i32
    }

    pub fn effective_defense(&self) -> i32 {
        (self.base_defense as f32 * self.difficulty.stat_multiplier()) as i32
    }

    pub fn add_skill(&mut self, skill: AISkill) {
        self.skills.push(skill);
    }

    pub fn add_special_move(&mut self, special: SpecialMove) {
        self.special_moves.push(special);
    }
}

/// Boss challenge in the arena
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BossChallenge {
    pub id: u64,
    pub name: String,
    pub boss: ArenaAI,
    pub required_rank: ArenaRankTier,
    pub entry_fee: u64,
    pub rewards: Vec<ArenaReward>,
    pub time_limit_turns: Option<u32>,
    pub special_rules: Vec<BossRule>,
    pub attempts_remaining: u32,
    pub best_time: Option<u32>,
    pub defeated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BossRule {
    /// No healing items allowed
    NoHealing,
    /// No magic allowed
    NoMagic,
    /// Time limit in turns
    TimeLimit(u32),
    /// Boss regenerates HP
    Regenerating(i32),
    /// Boss has multiple phases
    MultiPhase(u32),
    /// Environmental hazards
    Hazards(String),
    /// Minions spawn periodically
    Minions { interval: u32, count: u32 },
}

// =============================================================================
// Tournament System
// =============================================================================

/// Tournament bracket entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BracketEntry {
    pub player_id: u64,
    pub player_name: String,
    pub seed: u32,
    pub wins: u32,
    pub losses: u32,
    pub eliminated: bool,
}

/// Tournament match in a bracket
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TournamentMatch {
    pub match_id: u64,
    pub round: u32,
    pub match_number: u32,
    pub player1: Option<BracketEntry>,
    pub player2: Option<BracketEntry>,
    pub winner_id: Option<u64>,
    pub scheduled_time: Option<u64>,
    pub completed: bool,
}

/// Tournament bracket
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TournamentBracket {
    pub tournament_id: u64,
    pub name: String,
    pub bracket_type: BracketType,
    pub participants: Vec<BracketEntry>,
    pub matches: Vec<TournamentMatch>,
    pub current_round: u32,
    pub total_rounds: u32,
    pub winner_id: Option<u64>,
    pub prize_pool: u64,
    pub entry_fee: u64,
    pub started: bool,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BracketType {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    Swiss,
}

impl TournamentBracket {
    pub fn new(
        id: u64,
        name: String,
        bracket_type: BracketType,
        entry_fee: u64,
    ) -> Self {
        Self {
            tournament_id: id,
            name,
            bracket_type,
            participants: Vec::new(),
            matches: Vec::new(),
            current_round: 0,
            total_rounds: 0,
            winner_id: None,
            prize_pool: 0,
            entry_fee,
            started: false,
            completed: false,
        }
    }

    pub fn register_participant(&mut self, player_id: u64, player_name: String) -> bool {
        if self.started {
            return false;
        }

        if self.participants.iter().any(|p| p.player_id == player_id) {
            return false;
        }

        self.participants.push(BracketEntry {
            player_id,
            player_name,
            seed: self.participants.len() as u32 + 1,
            wins: 0,
            losses: 0,
            eliminated: false,
        });

        self.prize_pool += self.entry_fee;
        true
    }

    pub fn start_tournament(&mut self) -> bool {
        if self.started || self.participants.len() < TOURNAMENT_MIN_PARTICIPANTS {
            return false;
        }

        self.total_rounds = (self.participants.len() as f32).log2().ceil() as u32;
        self.generate_bracket();
        self.started = true;
        true
    }

    fn generate_bracket(&mut self) {
        // Simple single elimination bracket generation
        let num_participants = self.participants.len();
        let mut match_id = 1u64;

        // First round matches
        for i in (0..num_participants).step_by(2) {
            let player1 = self.participants.get(i).cloned();
            let player2 = self.participants.get(i + 1).cloned();

            self.matches.push(TournamentMatch {
                match_id,
                round: 1,
                match_number: (i / 2 + 1) as u32,
                player1,
                player2,
                winner_id: None,
                scheduled_time: None,
                completed: false,
            });
            match_id += 1;
        }

        self.current_round = 1;
    }

    pub fn record_match_result(&mut self, match_id: u64, winner_id: u64) -> bool {
        if let Some(match_entry) = self.matches.iter_mut().find(|m| m.match_id == match_id) {
            if match_entry.completed {
                return false;
            }

            match_entry.winner_id = Some(winner_id);
            match_entry.completed = true;

            // Update participant records
            if let Some(p1) = &match_entry.player1 {
                if let Some(participant) = self.participants.iter_mut().find(|p| p.player_id == p1.player_id) {
                    if p1.player_id == winner_id {
                        participant.wins += 1;
                    } else {
                        participant.losses += 1;
                        if self.bracket_type == BracketType::SingleElimination {
                            participant.eliminated = true;
                        }
                    }
                }
            }

            if let Some(p2) = &match_entry.player2 {
                if let Some(participant) = self.participants.iter_mut().find(|p| p.player_id == p2.player_id) {
                    if p2.player_id == winner_id {
                        participant.wins += 1;
                    } else {
                        participant.losses += 1;
                        if self.bracket_type == BracketType::SingleElimination {
                            participant.eliminated = true;
                        }
                    }
                }
            }

            // Check if round is complete
            let round_complete = self.matches
                .iter()
                .filter(|m| m.round == self.current_round)
                .all(|m| m.completed);

            if round_complete {
                self.advance_round();
            }

            return true;
        }
        false
    }

    fn advance_round(&mut self) {
        let remaining: Vec<_> = self.participants
            .iter()
            .filter(|p| !p.eliminated)
            .cloned()
            .collect();

        if remaining.len() == 1 {
            self.winner_id = Some(remaining[0].player_id);
            self.completed = true;
            return;
        }

        self.current_round += 1;

        // Generate next round matches
        let mut match_id = self.matches.len() as u64 + 1;
        for i in (0..remaining.len()).step_by(2) {
            let player1 = remaining.get(i).cloned();
            let player2 = remaining.get(i + 1).cloned();

            self.matches.push(TournamentMatch {
                match_id,
                round: self.current_round,
                match_number: (i / 2 + 1) as u32,
                player1,
                player2,
                winner_id: None,
                scheduled_time: None,
                completed: false,
            });
            match_id += 1;
        }
    }

    pub fn get_prize_distribution(&self) -> Vec<(u32, u64)> {
        // Place -> Prize amount
        vec![
            (1, (self.prize_pool as f64 * 0.5) as u64),
            (2, (self.prize_pool as f64 * 0.25) as u64),
            (3, (self.prize_pool as f64 * 0.15) as u64),
            (4, (self.prize_pool as f64 * 0.10) as u64),
        ]
    }
}

// =============================================================================
// Sect Wars
// =============================================================================

/// Sect war event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectWar {
    pub war_id: u64,
    pub attacking_sect_id: u64,
    pub defending_sect_id: u64,
    pub attacking_sect_name: String,
    pub defending_sect_name: String,
    pub territory_at_stake: String,
    pub attacker_points: u64,
    pub defender_points: u64,
    pub battles_fought: u32,
    pub max_battles: u32,
    pub attacker_participants: Vec<WarParticipant>,
    pub defender_participants: Vec<WarParticipant>,
    pub started_at: u64,
    pub ends_at: u64,
    pub winner: Option<WarSide>,
    pub completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarParticipant {
    pub player_id: u64,
    pub player_name: String,
    pub contribution: u64,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarSide {
    Attacker,
    Defender,
}

impl SectWar {
    pub fn new(
        war_id: u64,
        attacking_sect_id: u64,
        defending_sect_id: u64,
        attacking_sect_name: String,
        defending_sect_name: String,
        territory: String,
    ) -> Self {
        Self {
            war_id,
            attacking_sect_id,
            defending_sect_id,
            attacking_sect_name,
            defending_sect_name,
            territory_at_stake: territory,
            attacker_points: 0,
            defender_points: 0,
            battles_fought: 0,
            max_battles: 100,
            attacker_participants: Vec::new(),
            defender_participants: Vec::new(),
            started_at: 0,
            ends_at: 0,
            winner: None,
            completed: false,
        }
    }

    pub fn record_battle_result(&mut self, winner: WarSide, points: u64) {
        match winner {
            WarSide::Attacker => self.attacker_points += points,
            WarSide::Defender => self.defender_points += points,
        }
        self.battles_fought += 1;

        if self.battles_fought >= self.max_battles {
            self.complete_war();
        }
    }

    pub fn complete_war(&mut self) {
        self.completed = true;
        self.winner = if self.attacker_points > self.defender_points {
            Some(WarSide::Attacker)
        } else if self.defender_points > self.attacker_points {
            Some(WarSide::Defender)
        } else {
            None // Draw
        };
    }

    pub fn add_participant(&mut self, side: WarSide, player_id: u64, player_name: String) {
        let participant = WarParticipant {
            player_id,
            player_name,
            contribution: 0,
            kills: 0,
            deaths: 0,
            assists: 0,
        };

        match side {
            WarSide::Attacker => self.attacker_participants.push(participant),
            WarSide::Defender => self.defender_participants.push(participant),
        }
    }

    pub fn update_participant_stats(
        &mut self,
        player_id: u64,
        kills: u32,
        deaths: u32,
        assists: u32,
        contribution: u64,
    ) {
        let participants = self.attacker_participants.iter_mut()
            .chain(self.defender_participants.iter_mut());

        for participant in participants {
            if participant.player_id == player_id {
                participant.kills += kills;
                participant.deaths += deaths;
                participant.assists += assists;
                participant.contribution += contribution;
                break;
            }
        }
    }
}

// =============================================================================
// Kingdom Wars
// =============================================================================

/// Kingdom war event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KingdomWar {
    pub war_id: u64,
    pub kingdom1_id: u64,
    pub kingdom2_id: u64,
    pub kingdom1_name: String,
    pub kingdom2_name: String,
    pub kingdom1_score: u64,
    pub kingdom2_score: u64,
    pub objectives: Vec<WarObjective>,
    pub phase: KingdomWarPhase,
    pub phase_end_time: u64,
    pub total_participants: u32,
    pub started_at: u64,
    pub completed: bool,
    pub winner_kingdom_id: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarObjective {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub objective_type: WarObjectiveType,
    pub points_value: u64,
    pub controller: Option<u64>, // Kingdom ID controlling it
    pub capture_progress: f32,
    pub position: (i32, i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WarObjectiveType {
    /// Control point that can be captured
    ControlPoint,
    /// Resource point generating points over time
    ResourcePoint,
    /// Kill enemy players
    KillTarget,
    /// Destroy enemy structure
    Structure,
    /// Escort objective
    Escort,
    /// Defend for a duration
    Defense,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KingdomWarPhase {
    /// Preparation phase
    Preparation,
    /// Main battle phase
    Battle,
    /// Overtime if scores are close
    Overtime,
    /// War has ended
    Ended,
}

impl KingdomWar {
    pub fn new(
        war_id: u64,
        kingdom1_id: u64,
        kingdom2_id: u64,
        kingdom1_name: String,
        kingdom2_name: String,
    ) -> Self {
        Self {
            war_id,
            kingdom1_id,
            kingdom2_id,
            kingdom1_name,
            kingdom2_name,
            kingdom1_score: 0,
            kingdom2_score: 0,
            objectives: Vec::new(),
            phase: KingdomWarPhase::Preparation,
            phase_end_time: 0,
            total_participants: 0,
            started_at: 0,
            completed: false,
            winner_kingdom_id: None,
        }
    }

    pub fn add_objective(&mut self, objective: WarObjective) {
        self.objectives.push(objective);
    }

    pub fn capture_objective(&mut self, objective_id: u64, kingdom_id: u64) {
        if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == objective_id) {
            obj.controller = Some(kingdom_id);
            obj.capture_progress = 1.0;

            // Award points
            if kingdom_id == self.kingdom1_id {
                self.kingdom1_score += obj.points_value;
            } else if kingdom_id == self.kingdom2_id {
                self.kingdom2_score += obj.points_value;
            }
        }
    }

    pub fn advance_phase(&mut self) {
        self.phase = match self.phase {
            KingdomWarPhase::Preparation => KingdomWarPhase::Battle,
            KingdomWarPhase::Battle => {
                // Check if overtime is needed
                let diff = (self.kingdom1_score as i64 - self.kingdom2_score as i64).abs();
                if diff < 100 {
                    KingdomWarPhase::Overtime
                } else {
                    KingdomWarPhase::Ended
                }
            }
            KingdomWarPhase::Overtime => KingdomWarPhase::Ended,
            KingdomWarPhase::Ended => KingdomWarPhase::Ended,
        };

        if self.phase == KingdomWarPhase::Ended {
            self.complete_war();
        }
    }

    pub fn complete_war(&mut self) {
        self.completed = true;
        self.winner_kingdom_id = if self.kingdom1_score > self.kingdom2_score {
            Some(self.kingdom1_id)
        } else if self.kingdom2_score > self.kingdom1_score {
            Some(self.kingdom2_id)
        } else {
            None // Draw
        };
    }
}

// =============================================================================
// Arena Match
// =============================================================================

/// Active arena match
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArenaMatch {
    pub match_id: u64,
    pub arena_type: ArenaType,
    pub match_type: MatchType,
    pub participants: Vec<MatchParticipant>,
    pub teams: Option<(Vec<u64>, Vec<u64>)>,
    pub wagers: HashMap<u64, Wager>,
    pub current_turn: u32,
    pub max_turns: Option<u32>,
    pub started_at: u64,
    pub ended_at: Option<u64>,
    pub winner_id: Option<u64>,
    pub winning_team: Option<u8>,
    pub match_state: MatchState,
    pub combat_log: Vec<CombatLogEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchParticipant {
    pub player_id: u64,
    pub name: String,
    pub team: Option<u8>,
    pub current_hp: i32,
    pub max_hp: i32,
    pub current_mp: i32,
    pub max_mp: i32,
    pub is_alive: bool,
    pub position: (i32, i32),
    pub kills: u32,
    pub deaths: u32,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub healing_done: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchState {
    Waiting,
    InProgress,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatLogEntry {
    pub turn: u32,
    pub timestamp: u64,
    pub actor_id: u64,
    pub action: String,
    pub target_id: Option<u64>,
    pub result: String,
}

impl ArenaMatch {
    pub fn new(match_id: u64, arena_type: ArenaType, match_type: MatchType) -> Self {
        Self {
            match_id,
            arena_type,
            match_type,
            participants: Vec::new(),
            teams: None,
            wagers: HashMap::new(),
            current_turn: 0,
            max_turns: None,
            started_at: 0,
            ended_at: None,
            winner_id: None,
            winning_team: None,
            match_state: MatchState::Waiting,
            combat_log: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, participant: MatchParticipant) -> bool {
        if self.match_state != MatchState::Waiting {
            return false;
        }

        let max_participants = match self.arena_type {
            ArenaType::DuelingArena | ArenaType::RankedLadder => 2,
            ArenaType::TeamArena3v3 => 6,
            ArenaType::TeamArena5v5 => 10,
            ArenaType::BattleRoyale => BATTLE_ROYALE_MAX_PLAYERS,
            _ => 2,
        };

        if self.participants.len() >= max_participants {
            return false;
        }

        self.participants.push(participant);
        true
    }

    pub fn add_wager(&mut self, player_id: u64, wager: Wager) -> bool {
        if self.match_type != MatchType::Wager {
            return false;
        }

        if !wager.is_valid() {
            return false;
        }

        self.wagers.insert(player_id, wager);
        true
    }

    pub fn start_match(&mut self, timestamp: u64) -> bool {
        if self.match_state != MatchState::Waiting {
            return false;
        }

        let min_participants = match self.arena_type {
            ArenaType::DuelingArena | ArenaType::RankedLadder => 2,
            ArenaType::TeamArena3v3 => 6,
            ArenaType::TeamArena5v5 => 10,
            ArenaType::BattleRoyale => BATTLE_ROYALE_MIN_PLAYERS,
            _ => 2,
        };

        if self.participants.len() < min_participants {
            return false;
        }

        self.match_state = MatchState::InProgress;
        self.started_at = timestamp;
        self.current_turn = 1;
        true
    }

    pub fn process_turn(&mut self) {
        if self.match_state != MatchState::InProgress {
            return;
        }

        self.current_turn += 1;

        // Check for time limit
        if let Some(max) = self.max_turns {
            if self.current_turn > max {
                self.end_by_timeout();
            }
        }
    }

    pub fn record_kill(&mut self, killer_id: u64, victim_id: u64) {
        if let Some(killer) = self.participants.iter_mut().find(|p| p.player_id == killer_id) {
            killer.kills += 1;
        }

        if let Some(victim) = self.participants.iter_mut().find(|p| p.player_id == victim_id) {
            victim.deaths += 1;
            victim.is_alive = false;
        }

        self.check_match_end();
    }

    pub fn record_damage(&mut self, attacker_id: u64, target_id: u64, damage: u64) {
        if let Some(attacker) = self.participants.iter_mut().find(|p| p.player_id == attacker_id) {
            attacker.damage_dealt += damage;
        }

        if let Some(target) = self.participants.iter_mut().find(|p| p.player_id == target_id) {
            target.damage_taken += damage;
            target.current_hp = (target.current_hp - damage as i32).max(0);
        }
    }

    pub fn log_action(&mut self, actor_id: u64, action: String, target_id: Option<u64>, result: String) {
        self.combat_log.push(CombatLogEntry {
            turn: self.current_turn,
            timestamp: 0, // Would be set by game
            actor_id,
            action,
            target_id,
            result,
        });
    }

    fn check_match_end(&mut self) {
        let alive_count = self.participants.iter().filter(|p| p.is_alive).count();

        match self.arena_type {
            ArenaType::DuelingArena | ArenaType::RankedLadder => {
                if alive_count <= 1 {
                    if let Some(winner) = self.participants.iter().find(|p| p.is_alive) {
                        self.end_match(Some(winner.player_id), None);
                    }
                }
            }
            ArenaType::TeamArena3v3 | ArenaType::TeamArena5v5 => {
                if let Some((team1, team2)) = &self.teams {
                    let team1_alive = self.participants.iter()
                        .filter(|p| p.is_alive && team1.contains(&p.player_id))
                        .count();
                    let team2_alive = self.participants.iter()
                        .filter(|p| p.is_alive && team2.contains(&p.player_id))
                        .count();

                    if team1_alive == 0 {
                        self.end_match(None, Some(2));
                    } else if team2_alive == 0 {
                        self.end_match(None, Some(1));
                    }
                }
            }
            ArenaType::BattleRoyale => {
                if alive_count <= 1 {
                    if let Some(winner) = self.participants.iter().find(|p| p.is_alive) {
                        self.end_match(Some(winner.player_id), None);
                    }
                }
            }
            _ => {}
        }
    }

    fn end_by_timeout(&mut self) {
        // Determine winner by remaining HP percentage
        let winner = self.participants.iter()
            .filter(|p| p.is_alive)
            .max_by(|a, b| {
                let a_pct = a.current_hp as f32 / a.max_hp as f32;
                let b_pct = b.current_hp as f32 / b.max_hp as f32;
                a_pct.partial_cmp(&b_pct).unwrap()
            });

        if let Some(w) = winner {
            self.end_match(Some(w.player_id), None);
        } else {
            self.end_match(None, None); // Draw
        }
    }

    pub fn end_match(&mut self, winner_id: Option<u64>, winning_team: Option<u8>) {
        self.match_state = MatchState::Completed;
        self.winner_id = winner_id;
        self.winning_team = winning_team;
    }

    pub fn get_mvp(&self) -> Option<&MatchParticipant> {
        self.participants.iter()
            .max_by_key(|p| p.kills * 3 + p.damage_dealt as u32 / 100)
    }
}

// =============================================================================
// Player Arena Stats
// =============================================================================

/// Player's arena statistics and progress
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerArenaStats {
    pub player_id: u64,
    pub rating: i32,
    pub rank: ArenaRank,
    pub arena_points: u32,
    pub matches_played: u32,
    pub matches_won: u32,
    pub matches_lost: u32,
    pub win_streak: u32,
    pub best_win_streak: u32,
    pub loss_streak: u32,
    pub total_kills: u32,
    pub total_deaths: u32,
    pub total_damage_dealt: u64,
    pub total_damage_taken: u64,
    pub total_healing_done: u64,
    pub tournaments_won: u32,
    pub tournaments_participated: u32,
    pub highest_rank: ArenaRank,
    pub season_stats: HashMap<u32, SeasonStats>,
    pub unlocked_titles: Vec<String>,
    pub equipped_title: Option<String>,
    pub unlocked_cosmetics: Vec<u64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SeasonStats {
    pub season: u32,
    pub peak_rating: i32,
    pub peak_rank: ArenaRank,
    pub matches_played: u32,
    pub matches_won: u32,
    pub rewards_claimed: bool,
}

impl PlayerArenaStats {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            rating: 0,
            rank: ArenaRank::default(),
            ..Default::default()
        }
    }

    pub fn record_win(&mut self, rating_change: i32, arena_points: u32) {
        self.matches_played += 1;
        self.matches_won += 1;
        self.win_streak += 1;
        self.loss_streak = 0;

        if self.win_streak > self.best_win_streak {
            self.best_win_streak = self.win_streak;
        }

        self.rating = (self.rating + rating_change).max(0);
        self.arena_points += arena_points;
        self.update_rank();
    }

    pub fn record_loss(&mut self, rating_change: i32) {
        self.matches_played += 1;
        self.matches_lost += 1;
        self.loss_streak += 1;
        self.win_streak = 0;

        self.rating = (self.rating + rating_change).max(0);
        self.update_rank();
    }

    fn update_rank(&mut self) {
        self.rank = ArenaRank::from_rating(self.rating);

        if self.rank.tier > self.highest_rank.tier ||
           (self.rank.tier == self.highest_rank.tier &&
            self.rank.sub_tier > self.highest_rank.sub_tier) {
            self.highest_rank = self.rank;
        }
    }

    pub fn win_rate(&self) -> f32 {
        if self.matches_played == 0 {
            return 0.0;
        }
        self.matches_won as f32 / self.matches_played as f32
    }

    pub fn kda_ratio(&self) -> f32 {
        if self.total_deaths == 0 {
            return self.total_kills as f32;
        }
        self.total_kills as f32 / self.total_deaths as f32
    }

    pub fn unlock_title(&mut self, title: String) {
        if !self.unlocked_titles.contains(&title) {
            self.unlocked_titles.push(title);
        }
    }

    pub fn equip_title(&mut self, title: &str) -> bool {
        if self.unlocked_titles.contains(&title.to_string()) {
            self.equipped_title = Some(title.to_string());
            return true;
        }
        false
    }

    pub fn update_season_stats(&mut self, season: u32) {
        let stats = self.season_stats.entry(season).or_default();
        stats.season = season;
        stats.matches_played = self.matches_played;
        stats.matches_won = self.matches_won;

        if self.rating > stats.peak_rating {
            stats.peak_rating = self.rating;
            stats.peak_rank = self.rank;
        }
    }
}

// =============================================================================
// Arena Shop
// =============================================================================

/// Item available in the arena shop
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArenaShopItem {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub item_type: ArenaShopItemType,
    pub arena_points_cost: u32,
    pub gold_cost: Option<u64>,
    pub required_rank: Option<ArenaRankTier>,
    pub limited_stock: Option<u32>,
    pub season_exclusive: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArenaShopItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    Cosmetic,
    Title,
    Mount,
    Pet,
}

/// Arena shop
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArenaShop {
    pub items: Vec<ArenaShopItem>,
    pub purchase_history: HashMap<u64, Vec<u64>>, // player_id -> item_ids
    pub current_season: u32,
}

impl ArenaShop {
    pub fn new() -> Self {
        let mut shop = Self::default();
        shop.populate_default_items();
        shop
    }

    fn populate_default_items(&mut self) {
        self.items = vec![
            ArenaShopItem {
                id: 1,
                name: "Arena Champion's Blade".to_string(),
                description: "A fearsome weapon forged for arena champions.".to_string(),
                item_type: ArenaShopItemType::Weapon,
                arena_points_cost: 5000,
                gold_cost: None,
                required_rank: Some(ArenaRankTier::Gold),
                limited_stock: None,
                season_exclusive: None,
            },
            ArenaShopItem {
                id: 2,
                name: "Gladiator's Armor".to_string(),
                description: "Heavy armor worn by the greatest gladiators.".to_string(),
                item_type: ArenaShopItemType::Armor,
                arena_points_cost: 7500,
                gold_cost: None,
                required_rank: Some(ArenaRankTier::Platinum),
                limited_stock: None,
                season_exclusive: None,
            },
            ArenaShopItem {
                id: 3,
                name: "Victory Potion".to_string(),
                description: "Temporarily boosts all stats in arena matches.".to_string(),
                item_type: ArenaShopItemType::Consumable,
                arena_points_cost: 100,
                gold_cost: Some(500),
                required_rank: None,
                limited_stock: None,
                season_exclusive: None,
            },
            ArenaShopItem {
                id: 4,
                name: "Champion's Aura".to_string(),
                description: "A glowing aura that marks you as a true champion.".to_string(),
                item_type: ArenaShopItemType::Cosmetic,
                arena_points_cost: 10000,
                gold_cost: None,
                required_rank: Some(ArenaRankTier::Diamond),
                limited_stock: None,
                season_exclusive: None,
            },
            ArenaShopItem {
                id: 5,
                name: "Legendary Combatant".to_string(),
                description: "An exclusive title for the most dedicated fighters.".to_string(),
                item_type: ArenaShopItemType::Title,
                arena_points_cost: 25000,
                gold_cost: None,
                required_rank: Some(ArenaRankTier::Master),
                limited_stock: Some(100),
                season_exclusive: None,
            },
        ];
    }

    pub fn can_purchase(&self, player_id: u64, item_id: u64, player_points: u32, player_rank: ArenaRankTier) -> Result<(), String> {
        let item = self.items.iter().find(|i| i.id == item_id)
            .ok_or("Item not found")?;

        // Check rank requirement
        if let Some(required) = item.required_rank {
            if (player_rank as u8) < (required as u8) {
                return Err(format!("Requires {} rank", required.name()));
            }
        }

        // Check points
        if player_points < item.arena_points_cost {
            return Err("Not enough arena points".to_string());
        }

        // Check stock
        if let Some(stock) = item.limited_stock {
            let purchased_count = self.purchase_history.values()
                .flat_map(|v| v.iter())
                .filter(|&&id| id == item_id)
                .count() as u32;
            if purchased_count >= stock {
                return Err("Item is out of stock".to_string());
            }
        }

        // Check if already purchased (for non-consumables)
        if item.item_type != ArenaShopItemType::Consumable {
            if let Some(history) = self.purchase_history.get(&player_id) {
                if history.contains(&item_id) {
                    return Err("Already purchased".to_string());
                }
            }
        }

        Ok(())
    }

    pub fn purchase(&mut self, player_id: u64, item_id: u64) -> Option<&ArenaShopItem> {
        let item = self.items.iter().find(|i| i.id == item_id)?;

        self.purchase_history
            .entry(player_id)
            .or_default()
            .push(item_id);

        Some(item)
    }
}

// =============================================================================
// Matchmaking
// =============================================================================

/// Matchmaking queue entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueEntry {
    pub player_id: u64,
    pub player_name: String,
    pub rating: i32,
    pub rank: ArenaRank,
    pub queued_at: u64,
    pub arena_type: ArenaType,
    pub match_type: MatchType,
    pub preferred_opponents: Vec<u64>,
    pub blocked_opponents: Vec<u64>,
}

/// Matchmaking system
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MatchmakingSystem {
    pub queues: HashMap<(ArenaType, MatchType), Vec<QueueEntry>>,
    pub rating_tolerance_base: i32,
    pub rating_tolerance_increase_per_minute: i32,
    pub max_rating_tolerance: i32,
}

impl MatchmakingSystem {
    pub fn new() -> Self {
        Self {
            queues: HashMap::new(),
            rating_tolerance_base: 50,
            rating_tolerance_increase_per_minute: 10,
            max_rating_tolerance: 300,
        }
    }

    pub fn add_to_queue(&mut self, entry: QueueEntry) {
        let key = (entry.arena_type, entry.match_type);
        self.queues.entry(key).or_default().push(entry);
    }

    pub fn remove_from_queue(&mut self, player_id: u64) {
        for queue in self.queues.values_mut() {
            queue.retain(|e| e.player_id != player_id);
        }
    }

    pub fn find_match(
        &mut self,
        arena_type: ArenaType,
        match_type: MatchType,
        current_time: u64,
    ) -> Option<Vec<QueueEntry>> {
        let key = (arena_type, match_type);
        let queue = self.queues.get_mut(&key)?;

        if queue.is_empty() {
            return None;
        }

        let required_players = match arena_type {
            ArenaType::DuelingArena | ArenaType::RankedLadder => 2,
            ArenaType::TeamArena3v3 => 6,
            ArenaType::TeamArena5v5 => 10,
            ArenaType::BattleRoyale => BATTLE_ROYALE_MIN_PLAYERS,
            _ => 2,
        };

        if queue.len() < required_players {
            return None;
        }

        // Find suitable matches based on rating
        let mut matched = Vec::new();
        let mut matched_indices = Vec::new();

        for (i, entry) in queue.iter().enumerate() {
            if matched.len() >= required_players {
                break;
            }

            let wait_minutes = (current_time.saturating_sub(entry.queued_at)) / 60;
            let tolerance = (self.rating_tolerance_base +
                self.rating_tolerance_increase_per_minute * wait_minutes as i32)
                .min(self.max_rating_tolerance);

            let is_suitable = matched.is_empty() || matched.iter().all(|m: &QueueEntry| {
                let rating_diff = (entry.rating - m.rating).abs();
                rating_diff <= tolerance &&
                !entry.blocked_opponents.contains(&m.player_id) &&
                !m.blocked_opponents.contains(&entry.player_id)
            });

            if is_suitable {
                matched.push(entry.clone());
                matched_indices.push(i);
            }
        }

        if matched.len() >= required_players {
            // Remove matched entries from queue (in reverse order to preserve indices)
            for i in matched_indices.into_iter().rev() {
                queue.remove(i);
            }
            Some(matched)
        } else {
            None
        }
    }

    pub fn queue_length(&self, arena_type: ArenaType, match_type: MatchType) -> usize {
        self.queues
            .get(&(arena_type, match_type))
            .map(|q| q.len())
            .unwrap_or(0)
    }

    pub fn estimated_wait_time(&self, arena_type: ArenaType, match_type: MatchType) -> u64 {
        let queue_len = self.queue_length(arena_type, match_type);
        let required = match arena_type {
            ArenaType::DuelingArena | ArenaType::RankedLadder => 2,
            ArenaType::TeamArena3v3 => 6,
            ArenaType::TeamArena5v5 => 10,
            ArenaType::BattleRoyale => BATTLE_ROYALE_MIN_PLAYERS,
            _ => 2,
        };

        if queue_len >= required {
            30 // Seconds
        } else {
            let needed = required - queue_len;
            (needed as u64 * 60).min(300) // Estimate 1 min per player, max 5 mins
        }
    }
}

// =============================================================================
// Arena System (Main Controller)
// =============================================================================

/// Main arena system controller
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArenaSystem {
    /// Active matches
    pub active_matches: HashMap<u64, ArenaMatch>,
    /// Player stats
    pub player_stats: HashMap<u64, PlayerArenaStats>,
    /// Matchmaking system
    pub matchmaking: MatchmakingSystem,
    /// Spectator system
    pub spectator_system: SpectatorSystem,
    /// Arena shop
    pub shop: ArenaShop,
    /// Active tournaments
    pub tournaments: HashMap<u64, TournamentBracket>,
    /// Active sect wars
    pub sect_wars: HashMap<u64, SectWar>,
    /// Active kingdom wars
    pub kingdom_wars: HashMap<u64, KingdomWar>,
    /// Boss challenges
    pub boss_challenges: Vec<BossChallenge>,
    /// AI opponents available
    pub ai_opponents: Vec<ArenaAI>,
    /// Next match ID
    pub next_match_id: u64,
    /// Current season
    pub current_season: u32,
    /// Season end timestamp
    pub season_end_time: u64,
    /// Global arena statistics
    pub global_stats: GlobalArenaStats,
}

/// Global arena statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GlobalArenaStats {
    pub total_matches_played: u64,
    pub total_players_registered: u64,
    pub total_arena_points_distributed: u64,
    pub total_gold_wagered: u64,
    pub most_wins_player_id: Option<u64>,
    pub highest_rating_ever: i32,
    pub longest_win_streak: u32,
}

impl ArenaSystem {
    pub fn new() -> Self {
        let mut system = Self {
            next_match_id: 1,
            current_season: 1,
            shop: ArenaShop::new(),
            matchmaking: MatchmakingSystem::new(),
            spectator_system: SpectatorSystem::new(),
            ..Default::default()
        };

        system.populate_ai_opponents();
        system.populate_boss_challenges();
        system
    }

    fn populate_ai_opponents(&mut self) {
        // Training dummy
        self.ai_opponents.push(ArenaAI::training_dummy());

        // Various AI opponents
        let styles = [
            ("Iron Guard", AIFightingStyle::Defensive, AIDifficulty::Easy),
            ("Swift Shadow", AIFightingStyle::Evasive, AIDifficulty::Normal),
            ("Raging Bull", AIFightingStyle::Berserker, AIDifficulty::Normal),
            ("Arcane Duelist", AIFightingStyle::Caster, AIDifficulty::Hard),
            ("Blade Dancer", AIFightingStyle::Technical, AIDifficulty::Hard),
            ("Chaos Knight", AIFightingStyle::Chaotic, AIDifficulty::Expert),
            ("Counter Master", AIFightingStyle::Reactive, AIDifficulty::Expert),
            ("Grand Champion", AIFightingStyle::Balanced, AIDifficulty::Master),
            ("Legendary Gladiator", AIFightingStyle::Aggressive, AIDifficulty::Legendary),
        ];

        for (i, (name, style, diff)) in styles.iter().enumerate() {
            let level = (i + 1) as u32 * 10;
            let mut ai = ArenaAI::create_opponent(name, level, *style, *diff);
            ai.id = i as u64 + 1;
            self.ai_opponents.push(ai);
        }
    }

    fn populate_boss_challenges(&mut self) {
        let mut boss = ArenaAI::create_opponent(
            "The Colosseum Guardian",
            100,
            AIFightingStyle::Balanced,
            AIDifficulty::Legendary,
        );
        boss.title = Some("Arena Boss".to_string());
        boss.lore = "An ancient construct created to test the mightiest warriors.".to_string();
        boss.add_special_move(SpecialMove {
            name: "Colossal Slam".to_string(),
            description: "A devastating area attack.".to_string(),
            damage_multiplier: 3.0,
            hp_threshold: 0.5,
            uses_per_fight: 3,
            uses_remaining: 3,
        });

        self.boss_challenges.push(BossChallenge {
            id: 1,
            name: "Trial of the Guardian".to_string(),
            boss,
            required_rank: ArenaRankTier::Diamond,
            entry_fee: 5000,
            rewards: vec![
                ArenaReward::ArenaPoints(10000),
                ArenaReward::Gold(100000),
                ArenaReward::Title { id: 100, name: "Guardian Slayer".to_string() },
            ],
            time_limit_turns: Some(100),
            special_rules: vec![BossRule::MultiPhase(3)],
            attempts_remaining: 3,
            best_time: None,
            defeated: false,
        });
    }

    /// Register a new player in the arena system
    pub fn register_player(&mut self, player_id: u64) -> &PlayerArenaStats {
        self.global_stats.total_players_registered += 1;
        self.player_stats.entry(player_id).or_insert_with(|| PlayerArenaStats::new(player_id))
    }

    /// Get player stats
    pub fn get_player_stats(&self, player_id: u64) -> Option<&PlayerArenaStats> {
        self.player_stats.get(&player_id)
    }

    /// Get mutable player stats
    pub fn get_player_stats_mut(&mut self, player_id: u64) -> Option<&mut PlayerArenaStats> {
        self.player_stats.get_mut(&player_id)
    }

    /// Queue for a match
    pub fn queue_for_match(
        &mut self,
        player_id: u64,
        player_name: String,
        arena_type: ArenaType,
        match_type: MatchType,
        current_time: u64,
    ) -> bool {
        let stats = self.player_stats.get(&player_id).cloned()
            .unwrap_or_else(|| PlayerArenaStats::new(player_id));

        let entry = QueueEntry {
            player_id,
            player_name,
            rating: stats.rating,
            rank: stats.rank,
            queued_at: current_time,
            arena_type,
            match_type,
            preferred_opponents: Vec::new(),
            blocked_opponents: Vec::new(),
        };

        self.matchmaking.add_to_queue(entry);
        true
    }

    /// Leave queue
    pub fn leave_queue(&mut self, player_id: u64) {
        self.matchmaking.remove_from_queue(player_id);
    }

    /// Process matchmaking and create matches
    pub fn process_matchmaking(&mut self, current_time: u64) -> Vec<u64> {
        let mut created_matches = Vec::new();

        for arena_type in ArenaType::all() {
            for match_type in MatchType::all() {
                if let Some(players) = self.matchmaking.find_match(*arena_type, *match_type, current_time) {
                    let match_id = self.create_match(*arena_type, *match_type, players, current_time);
                    created_matches.push(match_id);
                }
            }
        }

        created_matches
    }

    /// Create a new match
    pub fn create_match(
        &mut self,
        arena_type: ArenaType,
        match_type: MatchType,
        players: Vec<QueueEntry>,
        current_time: u64,
    ) -> u64 {
        let match_id = self.next_match_id;
        self.next_match_id += 1;

        let mut arena_match = ArenaMatch::new(match_id, arena_type, match_type);

        for (i, player) in players.iter().enumerate() {
            let participant = MatchParticipant {
                player_id: player.player_id,
                name: player.player_name.clone(),
                team: if arena_type.is_team_based() {
                    Some((i % 2) as u8 + 1)
                } else {
                    None
                },
                current_hp: 100 + (player.rating / 10),
                max_hp: 100 + (player.rating / 10),
                current_mp: 50,
                max_mp: 50,
                is_alive: true,
                position: (i as i32 * 5, 0),
                kills: 0,
                deaths: 0,
                damage_dealt: 0,
                damage_taken: 0,
                healing_done: 0,
            };
            arena_match.add_participant(participant);
        }

        if arena_type.is_team_based() {
            let team1: Vec<u64> = players.iter().enumerate()
                .filter(|(i, _)| i % 2 == 0)
                .map(|(_, p)| p.player_id)
                .collect();
            let team2: Vec<u64> = players.iter().enumerate()
                .filter(|(i, _)| i % 2 == 1)
                .map(|(_, p)| p.player_id)
                .collect();
            arena_match.teams = Some((team1, team2));
        }

        arena_match.start_match(current_time);
        self.active_matches.insert(match_id, arena_match);
        self.global_stats.total_matches_played += 1;

        match_id
    }

    /// Challenge an AI opponent
    pub fn challenge_ai(
        &mut self,
        player_id: u64,
        player_name: String,
        ai_index: usize,
        current_time: u64,
    ) -> Option<u64> {
        let ai = self.ai_opponents.get(ai_index)?.clone();

        let match_id = self.next_match_id;
        self.next_match_id += 1;

        let mut arena_match = ArenaMatch::new(match_id, ArenaType::DuelingArena, MatchType::Casual);

        let stats = self.player_stats.get(&player_id);
        let player_participant = MatchParticipant {
            player_id,
            name: player_name,
            team: None,
            current_hp: stats.map(|s| 100 + s.rating / 10).unwrap_or(100),
            max_hp: stats.map(|s| 100 + s.rating / 10).unwrap_or(100),
            current_mp: 50,
            max_mp: 50,
            is_alive: true,
            position: (0, 0),
            kills: 0,
            deaths: 0,
            damage_dealt: 0,
            damage_taken: 0,
            healing_done: 0,
        };

        let ai_participant = MatchParticipant {
            player_id: ai.id,
            name: ai.name.clone(),
            team: None,
            current_hp: ai.effective_hp(),
            max_hp: ai.effective_hp(),
            current_mp: ai.base_mp,
            max_mp: ai.base_mp,
            is_alive: true,
            position: (10, 0),
            kills: 0,
            deaths: 0,
            damage_dealt: 0,
            damage_taken: 0,
            healing_done: 0,
        };

        arena_match.add_participant(player_participant);
        arena_match.add_participant(ai_participant);
        arena_match.start_match(current_time);

        self.active_matches.insert(match_id, arena_match);
        self.global_stats.total_matches_played += 1;

        Some(match_id)
    }

    /// Complete a match and distribute rewards
    pub fn complete_match(&mut self, match_id: u64) -> Option<MatchResult> {
        let arena_match = self.active_matches.remove(&match_id)?;

        if arena_match.match_state != MatchState::Completed {
            return None;
        }

        let winner_id = arena_match.winner_id;

        let mut result = MatchResult {
            match_id,
            winner_id,
            participants: Vec::new(),
            rating_changes: HashMap::new(),
            rewards: HashMap::new(),
        };

        // Calculate rating changes and rewards
        for participant in &arena_match.participants {
            let is_winner = Some(participant.player_id) == winner_id;

            if arena_match.match_type.affects_rating() {
                let rating_change = if is_winner {
                    RATING_WIN_BASE
                } else {
                    RATING_LOSS_BASE
                };

                result.rating_changes.insert(participant.player_id, rating_change);

                if let Some(stats) = self.player_stats.get_mut(&participant.player_id) {
                    let arena_points = if is_winner {
                        BASE_ARENA_POINTS * (1 + stats.win_streak / 5)
                    } else {
                        BASE_ARENA_POINTS / 2
                    };

                    if is_winner {
                        stats.record_win(rating_change, arena_points);
                    } else {
                        stats.record_loss(rating_change);
                    }

                    stats.total_kills += participant.kills;
                    stats.total_deaths += participant.deaths;
                    stats.total_damage_dealt += participant.damage_dealt;
                    stats.total_damage_taken += participant.damage_taken;
                    stats.total_healing_done += participant.healing_done;

                    // Track global stats
                    if stats.rating > self.global_stats.highest_rating_ever {
                        self.global_stats.highest_rating_ever = stats.rating;
                    }
                    if stats.best_win_streak > self.global_stats.longest_win_streak {
                        self.global_stats.longest_win_streak = stats.best_win_streak;
                    }

                    let mut rewards = Vec::new();
                    if is_winner {
                        rewards.push(ArenaReward::ArenaPoints(arena_points));
                        rewards.push(ArenaReward::Experience(100 * participant.kills as u64));
                    }
                    result.rewards.insert(participant.player_id, rewards);
                }
            }

            result.participants.push(ParticipantResult {
                player_id: participant.player_id,
                name: participant.name.clone(),
                kills: participant.kills,
                deaths: participant.deaths,
                damage_dealt: participant.damage_dealt,
                damage_taken: participant.damage_taken,
                is_winner,
            });
        }

        // Handle wagers
        if arena_match.match_type == MatchType::Wager {
            if let Some(winner) = winner_id {
                let total_wager: u64 = arena_match.wagers.values()
                    .map(|w| w.total_value)
                    .sum();
                self.global_stats.total_gold_wagered += total_wager;

                result.rewards.entry(winner).or_default().push(
                    ArenaReward::Gold(total_wager)
                );
            }
        }

        // Create replay
        let replay = MatchReplay {
            match_id,
            arena_type: arena_match.arena_type,
            match_type: arena_match.match_type,
            participants: arena_match.participants.iter().map(|p| ReplayParticipant {
                player_id: p.player_id,
                name: p.name.clone(),
                class: "Unknown".to_string(),
                level: 1,
                starting_hp: p.max_hp,
                starting_mp: p.max_mp,
            }).collect(),
            actions: Vec::new(),
            winner_id,
            duration_turns: arena_match.current_turn,
            recorded_at: arena_match.ended_at.unwrap_or(0),
        };
        self.spectator_system.save_replay(replay);

        Some(result)
    }

    /// Create a tournament
    pub fn create_tournament(
        &mut self,
        name: String,
        bracket_type: BracketType,
        entry_fee: u64,
    ) -> u64 {
        let tournament_id = self.tournaments.len() as u64 + 1;
        let tournament = TournamentBracket::new(tournament_id, name, bracket_type, entry_fee);
        self.tournaments.insert(tournament_id, tournament);
        tournament_id
    }

    /// Register for a tournament
    pub fn register_for_tournament(
        &mut self,
        tournament_id: u64,
        player_id: u64,
        player_name: String,
    ) -> bool {
        if let Some(tournament) = self.tournaments.get_mut(&tournament_id) {
            return tournament.register_participant(player_id, player_name);
        }
        false
    }

    /// Start a sect war
    pub fn start_sect_war(
        &mut self,
        attacking_sect_id: u64,
        defending_sect_id: u64,
        attacking_name: String,
        defending_name: String,
        territory: String,
    ) -> u64 {
        let war_id = self.sect_wars.len() as u64 + 1;
        let war = SectWar::new(
            war_id,
            attacking_sect_id,
            defending_sect_id,
            attacking_name,
            defending_name,
            territory,
        );
        self.sect_wars.insert(war_id, war);
        war_id
    }

    /// Start a kingdom war
    pub fn start_kingdom_war(
        &mut self,
        kingdom1_id: u64,
        kingdom2_id: u64,
        kingdom1_name: String,
        kingdom2_name: String,
    ) -> u64 {
        let war_id = self.kingdom_wars.len() as u64 + 1;
        let war = KingdomWar::new(
            war_id,
            kingdom1_id,
            kingdom2_id,
            kingdom1_name,
            kingdom2_name,
        );
        self.kingdom_wars.insert(war_id, war);
        war_id
    }

    /// Get leaderboard
    pub fn get_leaderboard(&self, limit: usize) -> Vec<LeaderboardEntry> {
        let mut entries: Vec<_> = self.player_stats.values()
            .map(|s| LeaderboardEntry {
                player_id: s.player_id,
                rating: s.rating,
                rank: s.rank,
                wins: s.matches_won,
                losses: s.matches_lost,
                win_rate: s.win_rate(),
            })
            .collect();

        entries.sort_by(|a, b| b.rating.cmp(&a.rating));
        entries.truncate(limit);
        entries
    }

    /// Attempt a boss challenge
    pub fn attempt_boss_challenge(
        &mut self,
        challenge_id: u64,
        player_id: u64,
        player_name: String,
        current_time: u64,
    ) -> Option<u64> {
        let challenge = self.boss_challenges.iter_mut()
            .find(|c| c.id == challenge_id)?;

        if challenge.attempts_remaining == 0 || challenge.defeated {
            return None;
        }

        challenge.attempts_remaining -= 1;

        // Create the match
        let match_id = self.next_match_id;
        self.next_match_id += 1;

        let mut arena_match = ArenaMatch::new(match_id, ArenaType::DuelingArena, MatchType::Casual);

        let player_participant = MatchParticipant {
            player_id,
            name: player_name,
            team: None,
            current_hp: 100,
            max_hp: 100,
            current_mp: 50,
            max_mp: 50,
            is_alive: true,
            position: (0, 0),
            kills: 0,
            deaths: 0,
            damage_dealt: 0,
            damage_taken: 0,
            healing_done: 0,
        };

        let boss_participant = MatchParticipant {
            player_id: challenge.boss.id,
            name: challenge.boss.name.clone(),
            team: None,
            current_hp: challenge.boss.effective_hp(),
            max_hp: challenge.boss.effective_hp(),
            current_mp: challenge.boss.base_mp,
            max_mp: challenge.boss.base_mp,
            is_alive: true,
            position: (10, 0),
            kills: 0,
            deaths: 0,
            damage_dealt: 0,
            damage_taken: 0,
            healing_done: 0,
        };

        arena_match.add_participant(player_participant);
        arena_match.add_participant(boss_participant);
        arena_match.max_turns = challenge.time_limit_turns;
        arena_match.start_match(current_time);

        self.active_matches.insert(match_id, arena_match);

        Some(match_id)
    }

    /// Process end of season
    pub fn end_season(&mut self) -> Vec<(u64, RankReward)> {
        let mut rewards = Vec::new();

        for (player_id, stats) in &mut self.player_stats {
            // Record season stats
            stats.update_season_stats(self.current_season);

            // Calculate reward based on highest rank achieved
            let reward = RankReward::for_rank(stats.highest_rank.tier);
            rewards.push((*player_id, reward));

            // Reset for new season but keep some progress
            stats.rating = (stats.rating * 3) / 4; // Soft reset
            stats.rank = ArenaRank::from_rating(stats.rating);
            stats.win_streak = 0;
            stats.loss_streak = 0;
        }

        self.current_season += 1;
        rewards
    }

    /// Get queue status
    pub fn get_queue_status(&self, arena_type: ArenaType, match_type: MatchType) -> QueueStatus {
        QueueStatus {
            players_in_queue: self.matchmaking.queue_length(arena_type, match_type),
            estimated_wait_seconds: self.matchmaking.estimated_wait_time(arena_type, match_type),
        }
    }
}

/// Match result summary
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchResult {
    pub match_id: u64,
    pub winner_id: Option<u64>,
    pub participants: Vec<ParticipantResult>,
    pub rating_changes: HashMap<u64, i32>,
    pub rewards: HashMap<u64, Vec<ArenaReward>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantResult {
    pub player_id: u64,
    pub name: String,
    pub kills: u32,
    pub deaths: u32,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub is_winner: bool,
}

/// Leaderboard entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub player_id: u64,
    pub rating: i32,
    pub rank: ArenaRank,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: f32,
}

/// Queue status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueStatus {
    pub players_in_queue: usize,
    pub estimated_wait_seconds: u64,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_rank_from_rating() {
        assert_eq!(ArenaRank::from_rating(0).tier, ArenaRankTier::Bronze);
        assert_eq!(ArenaRank::from_rating(500).tier, ArenaRankTier::Silver);
        assert_eq!(ArenaRank::from_rating(1000).tier, ArenaRankTier::Gold);
        assert_eq!(ArenaRank::from_rating(2000).tier, ArenaRankTier::Master);
        assert_eq!(ArenaRank::from_rating(3200).tier, ArenaRankTier::Legend);
    }

    #[test]
    fn test_player_stats_win_loss() {
        let mut stats = PlayerArenaStats::new(1);
        assert_eq!(stats.matches_played, 0);
        assert_eq!(stats.rating, 0);

        stats.record_win(25, 10);
        assert_eq!(stats.matches_won, 1);
        assert_eq!(stats.win_streak, 1);
        assert_eq!(stats.rating, 25);

        stats.record_loss(-20);
        assert_eq!(stats.matches_lost, 1);
        assert_eq!(stats.win_streak, 0);
        assert_eq!(stats.loss_streak, 1);
        assert_eq!(stats.rating, 5);
    }

    #[test]
    fn test_wager_validation() {
        let mut wager = Wager::new();
        assert!(!wager.is_valid()); // Empty wager invalid

        wager.add_gold(MIN_WAGER);
        assert!(wager.is_valid());

        wager.add_gold(MAX_WAGER);
        assert!(!wager.is_valid()); // Over max
    }

    #[test]
    fn test_tournament_registration() {
        let mut tournament = TournamentBracket::new(
            1,
            "Test Tournament".to_string(),
            BracketType::SingleElimination,
            100,
        );

        assert!(tournament.register_participant(1, "Player1".to_string()));
        assert!(!tournament.register_participant(1, "Player1".to_string())); // Duplicate
        assert_eq!(tournament.participants.len(), 1);
        assert_eq!(tournament.prize_pool, 100);
    }

    #[test]
    fn test_ai_difficulty_scaling() {
        let easy_ai = ArenaAI::create_opponent("Easy", 10, AIFightingStyle::Balanced, AIDifficulty::Easy);
        let hard_ai = ArenaAI::create_opponent("Hard", 10, AIFightingStyle::Balanced, AIDifficulty::Hard);

        assert!(easy_ai.effective_hp() < hard_ai.effective_hp());
        assert!(easy_ai.effective_attack() < hard_ai.effective_attack());
    }

    #[test]
    fn test_arena_system_registration() {
        let mut system = ArenaSystem::new();
        let stats = system.register_player(1);
        assert_eq!(stats.player_id, 1);
        assert_eq!(stats.rating, 0);
        assert_eq!(system.global_stats.total_players_registered, 1);
    }

    #[test]
    fn test_matchmaking_queue() {
        let mut mm = MatchmakingSystem::new();

        let entry1 = QueueEntry {
            player_id: 1,
            player_name: "Player1".to_string(),
            rating: 1000,
            rank: ArenaRank::from_rating(1000),
            queued_at: 0,
            arena_type: ArenaType::DuelingArena,
            match_type: MatchType::Ranked,
            preferred_opponents: vec![],
            blocked_opponents: vec![],
        };

        let entry2 = QueueEntry {
            player_id: 2,
            player_name: "Player2".to_string(),
            rating: 1050,
            rank: ArenaRank::from_rating(1050),
            queued_at: 0,
            arena_type: ArenaType::DuelingArena,
            match_type: MatchType::Ranked,
            preferred_opponents: vec![],
            blocked_opponents: vec![],
        };

        mm.add_to_queue(entry1);
        mm.add_to_queue(entry2);

        let matched = mm.find_match(ArenaType::DuelingArena, MatchType::Ranked, 0);
        assert!(matched.is_some());
        assert_eq!(matched.unwrap().len(), 2);
    }

    #[test]
    fn test_spectator_betting() {
        let mut spec_system = SpectatorSystem::new();

        spec_system.add_spectator(1, Spectator {
            player_id: 100,
            name: "Spectator1".to_string(),
            bet: None,
            joined_at: 0,
        });

        let bet = SpectatorBet::new(1000, 1, 2.0);
        assert!(spec_system.place_bet(1, 100, bet));
        assert_eq!(spec_system.betting_pool.get(&1), Some(&1000));
    }

    #[test]
    fn test_sect_war() {
        let mut war = SectWar::new(
            1,
            1,
            2,
            "Sect A".to_string(),
            "Sect B".to_string(),
            "Mountain Peak".to_string(),
        );

        war.record_battle_result(WarSide::Attacker, 100);
        assert_eq!(war.attacker_points, 100);
        assert_eq!(war.battles_fought, 1);

        war.record_battle_result(WarSide::Defender, 150);
        assert_eq!(war.defender_points, 150);
    }
}

//! Comprehensive Statistics Tracking System for ShadowCrypt
//!
//! This module provides extensive statistics tracking across all game activities:
//! - Combat statistics (damage, kills, deaths, skills)
//! - Exploration statistics (floors, rooms, secrets, traps)
//! - Economic statistics (gold, items, crafting)
//! - Time statistics (playtime, floor times, sessions)
//! - Social statistics (NPCs, quests, reputation)
//! - Leaderboards (personal bests, records)
//! - Graphs and visualizations (progress tracking, comparisons)

use std::collections::{HashMap, BTreeMap};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::classes::CharacterClass;
use crate::entities::EnemyKind;

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of floor time records to keep
pub const MAX_FLOOR_TIME_RECORDS: usize = 100;

/// Maximum session history entries
pub const MAX_SESSION_HISTORY: usize = 50;

/// Maximum graph data points
pub const MAX_GRAPH_DATA_POINTS: usize = 1000;

/// Leaderboard categories
pub const LEADERBOARD_CATEGORIES: &[&str] = &[
    "highest_damage",
    "most_kills",
    "fastest_clear",
    "most_gold",
    "deepest_floor",
    "highest_combo",
    "most_crits",
    "longest_session",
];

// ============================================================================
// Combat Statistics
// ============================================================================

/// Tracks all combat-related statistics
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CombatStatistics {
    /// Total damage dealt across all games
    pub total_damage_dealt: u64,
    /// Total damage received across all games
    pub total_damage_received: u64,
    /// Enemies killed by type (enemy kind name -> count)
    pub enemies_killed_by_type: HashMap<String, u64>,
    /// Total enemies killed
    pub total_enemies_killed: u64,
    /// Bosses defeated (boss name -> defeat count)
    pub bosses_defeated: HashMap<String, u32>,
    /// Total boss defeats
    pub total_bosses_defeated: u32,
    /// Total deaths
    pub deaths: u32,
    /// Highest single hit damage
    pub highest_hit: u64,
    /// Highest hit details (damage, enemy, floor)
    pub highest_hit_details: Option<HighestHitRecord>,
    /// Total critical hits landed
    pub critical_hits: u64,
    /// Critical hit rate (crits / total attacks)
    pub critical_hit_rate: f32,
    /// Total attacks made
    pub total_attacks: u64,
    /// Skills used by skill name
    pub skills_used: HashMap<String, u64>,
    /// Total skills used
    pub total_skills_used: u64,
    /// Spells cast by spell name
    pub spells_cast: HashMap<String, u64>,
    /// Total spells cast
    pub total_spells_cast: u64,
    /// Damage dealt by type (physical, fire, ice, etc.)
    pub damage_by_type: HashMap<String, u64>,
    /// Damage received by type
    pub damage_received_by_type: HashMap<String, u64>,
    /// Dodges successful
    pub successful_dodges: u64,
    /// Blocks successful
    pub successful_blocks: u64,
    /// Parries successful
    pub successful_parries: u64,
    /// Kill streaks (streak length -> count)
    pub kill_streaks: HashMap<u32, u32>,
    /// Highest kill streak
    pub highest_kill_streak: u32,
    /// Current kill streak (for active session)
    pub current_kill_streak: u32,
    /// Overkill damage (damage beyond killing blow)
    pub total_overkill_damage: u64,
    /// Status effects applied
    pub status_effects_applied: HashMap<String, u64>,
    /// Status effects received
    pub status_effects_received: HashMap<String, u64>,
    /// Combat time in turns
    pub total_combat_turns: u64,
    /// Average damage per attack
    pub average_damage_per_attack: f32,
    /// Kills per floor average
    pub kills_per_floor_average: f32,
    /// Damage dealt per floor (floor -> damage)
    pub damage_by_floor: HashMap<u32, u64>,
    /// Kills by floor (floor -> kills)
    pub kills_by_floor: HashMap<u32, u64>,
}

/// Record of the highest hit achieved
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighestHitRecord {
    pub damage: u64,
    pub enemy_name: String,
    pub floor: u32,
    pub skill_used: Option<String>,
    pub was_critical: bool,
    pub timestamp: u64,
}

impl CombatStatistics {
    /// Create new combat statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record damage dealt
    pub fn record_damage_dealt(&mut self, amount: u64, damage_type: &str, enemy: &str, floor: u32, skill: Option<&str>, is_critical: bool) {
        self.total_damage_dealt += amount;
        self.total_attacks += 1;

        *self.damage_by_type.entry(damage_type.to_string()).or_insert(0) += amount;
        *self.damage_by_floor.entry(floor).or_insert(0) += amount;

        // Check for highest hit
        if amount > self.highest_hit {
            self.highest_hit = amount;
            self.highest_hit_details = Some(HighestHitRecord {
                damage: amount,
                enemy_name: enemy.to_string(),
                floor,
                skill_used: skill.map(|s| s.to_string()),
                was_critical: is_critical,
                timestamp: current_timestamp(),
            });
        }

        // Track critical hits
        if is_critical {
            self.critical_hits += 1;
        }

        // Update critical hit rate
        self.critical_hit_rate = self.critical_hits as f32 / self.total_attacks as f32;
        self.average_damage_per_attack = self.total_damage_dealt as f32 / self.total_attacks as f32;
    }

    /// Record damage received
    pub fn record_damage_received(&mut self, amount: u64, damage_type: &str) {
        self.total_damage_received += amount;
        *self.damage_received_by_type.entry(damage_type.to_string()).or_insert(0) += amount;
    }

    /// Record an enemy kill
    pub fn record_kill(&mut self, enemy_kind: &str, floor: u32, overkill: u64) {
        self.total_enemies_killed += 1;
        *self.enemies_killed_by_type.entry(enemy_kind.to_string()).or_insert(0) += 1;
        *self.kills_by_floor.entry(floor).or_insert(0) += 1;
        self.total_overkill_damage += overkill;

        // Update kill streak
        self.current_kill_streak += 1;
        if self.current_kill_streak > self.highest_kill_streak {
            self.highest_kill_streak = self.current_kill_streak;
        }
        *self.kill_streaks.entry(self.current_kill_streak).or_insert(0) += 1;
    }

    /// Record a boss defeat
    pub fn record_boss_defeat(&mut self, boss_name: &str) {
        self.total_bosses_defeated += 1;
        *self.bosses_defeated.entry(boss_name.to_string()).or_insert(0) += 1;
    }

    /// Record player death
    pub fn record_death(&mut self) {
        self.deaths += 1;
        self.current_kill_streak = 0;
    }

    /// Record skill usage
    pub fn record_skill_used(&mut self, skill_name: &str) {
        self.total_skills_used += 1;
        *self.skills_used.entry(skill_name.to_string()).or_insert(0) += 1;
    }

    /// Record spell cast
    pub fn record_spell_cast(&mut self, spell_name: &str) {
        self.total_spells_cast += 1;
        *self.spells_cast.entry(spell_name.to_string()).or_insert(0) += 1;
    }

    /// Record status effect applied to enemy
    pub fn record_status_applied(&mut self, effect_name: &str) {
        *self.status_effects_applied.entry(effect_name.to_string()).or_insert(0) += 1;
    }

    /// Record status effect received
    pub fn record_status_received(&mut self, effect_name: &str) {
        *self.status_effects_received.entry(effect_name.to_string()).or_insert(0) += 1;
    }

    /// Record successful dodge
    pub fn record_dodge(&mut self) {
        self.successful_dodges += 1;
    }

    /// Record successful block
    pub fn record_block(&mut self) {
        self.successful_blocks += 1;
    }

    /// Record successful parry
    pub fn record_parry(&mut self) {
        self.successful_parries += 1;
    }

    /// Get kill/death ratio
    pub fn kill_death_ratio(&self) -> f32 {
        if self.deaths == 0 {
            self.total_enemies_killed as f32
        } else {
            self.total_enemies_killed as f32 / self.deaths as f32
        }
    }

    /// Get most killed enemy type
    pub fn most_killed_enemy(&self) -> Option<(&String, &u64)> {
        self.enemies_killed_by_type.iter().max_by_key(|(_, count)| *count)
    }

    /// Get most used skill
    pub fn most_used_skill(&self) -> Option<(&String, &u64)> {
        self.skills_used.iter().max_by_key(|(_, count)| *count)
    }
}

// ============================================================================
// Exploration Statistics
// ============================================================================

/// Tracks all exploration-related statistics
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ExplorationStatistics {
    /// Floors explored (unique floors visited)
    pub floors_explored: u32,
    /// Deepest floor reached
    pub deepest_floor: u32,
    /// Total rooms discovered
    pub rooms_discovered: u64,
    /// Rooms discovered per floor
    pub rooms_by_floor: HashMap<u32, u64>,
    /// Secret areas found
    pub secret_areas_found: u64,
    /// Secret areas by floor
    pub secrets_by_floor: HashMap<u32, u32>,
    /// Total distance traveled (in tiles)
    pub distance_traveled: u64,
    /// Distance traveled per floor
    pub distance_by_floor: HashMap<u32, u64>,
    /// Traps triggered
    pub traps_triggered: u64,
    /// Traps avoided (detected and bypassed)
    pub traps_avoided: u64,
    /// Traps disarmed
    pub traps_disarmed: u64,
    /// Trap types encountered
    pub trap_types_encountered: HashMap<String, u32>,
    /// Chests opened
    pub chests_opened: u64,
    /// Chest types opened (normal, locked, mimic, etc.)
    pub chest_types_opened: HashMap<String, u64>,
    /// Doors opened
    pub doors_opened: u64,
    /// Locked doors opened
    pub locked_doors_opened: u64,
    /// Keys used
    pub keys_used: u64,
    /// Shrines activated
    pub shrines_activated: u64,
    /// Shrine types used
    pub shrine_types: HashMap<String, u32>,
    /// Portals used
    pub portals_used: u64,
    /// Stairs descended
    pub stairs_descended: u64,
    /// Stairs ascended
    pub stairs_ascended: u64,
    /// Hidden walls discovered
    pub hidden_walls_found: u64,
    /// Maps fully revealed
    pub maps_fully_revealed: u32,
    /// Unique locations discovered
    pub unique_locations: HashMap<String, u32>,
    /// Environmental hazards survived
    pub hazards_survived: u64,
    /// Times rested
    pub times_rested: u64,
    /// Exploration percentage per floor
    pub exploration_percent_by_floor: HashMap<u32, f32>,
    /// Average exploration percentage
    pub average_exploration_percent: f32,
}

impl ExplorationStatistics {
    /// Create new exploration statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record floor visited
    pub fn record_floor_visited(&mut self, floor: u32) {
        if floor > self.deepest_floor {
            self.deepest_floor = floor;
        }
        self.floors_explored = self.floors_explored.max(floor);
    }

    /// Record room discovered
    pub fn record_room_discovered(&mut self, floor: u32) {
        self.rooms_discovered += 1;
        *self.rooms_by_floor.entry(floor).or_insert(0) += 1;
    }

    /// Record secret area found
    pub fn record_secret_found(&mut self, floor: u32) {
        self.secret_areas_found += 1;
        *self.secrets_by_floor.entry(floor).or_insert(0) += 1;
    }

    /// Record distance traveled
    pub fn record_movement(&mut self, floor: u32, distance: u64) {
        self.distance_traveled += distance;
        *self.distance_by_floor.entry(floor).or_insert(0) += distance;
    }

    /// Record trap interaction
    pub fn record_trap(&mut self, trap_type: &str, triggered: bool, disarmed: bool) {
        *self.trap_types_encountered.entry(trap_type.to_string()).or_insert(0) += 1;

        if triggered {
            self.traps_triggered += 1;
        } else if disarmed {
            self.traps_disarmed += 1;
            self.traps_avoided += 1;
        } else {
            self.traps_avoided += 1;
        }
    }

    /// Record chest opened
    pub fn record_chest_opened(&mut self, chest_type: &str) {
        self.chests_opened += 1;
        *self.chest_types_opened.entry(chest_type.to_string()).or_insert(0) += 1;
    }

    /// Record door opened
    pub fn record_door_opened(&mut self, locked: bool) {
        self.doors_opened += 1;
        if locked {
            self.locked_doors_opened += 1;
            self.keys_used += 1;
        }
    }

    /// Record shrine activated
    pub fn record_shrine_activated(&mut self, shrine_type: &str) {
        self.shrines_activated += 1;
        *self.shrine_types.entry(shrine_type.to_string()).or_insert(0) += 1;
    }

    /// Record floor exploration percentage
    pub fn record_floor_exploration(&mut self, floor: u32, percent: f32) {
        self.exploration_percent_by_floor.insert(floor, percent);

        // Update average
        if !self.exploration_percent_by_floor.is_empty() {
            let sum: f32 = self.exploration_percent_by_floor.values().sum();
            self.average_exploration_percent = sum / self.exploration_percent_by_floor.len() as f32;
        }

        // Check for full exploration
        if percent >= 100.0 {
            self.maps_fully_revealed += 1;
        }
    }

    /// Get trap avoidance rate
    pub fn trap_avoidance_rate(&self) -> f32 {
        let total = self.traps_triggered + self.traps_avoided;
        if total == 0 {
            1.0
        } else {
            self.traps_avoided as f32 / total as f32
        }
    }
}

// ============================================================================
// Economic Statistics
// ============================================================================

/// Tracks all economic-related statistics
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EconomicStatistics {
    /// Total gold earned
    pub gold_earned: u64,
    /// Gold earned by source (enemies, chests, quests, etc.)
    pub gold_by_source: HashMap<String, u64>,
    /// Total gold spent
    pub gold_spent: u64,
    /// Gold spent by category (items, repairs, services, etc.)
    pub gold_by_expense: HashMap<String, u64>,
    /// Current gold (for active session tracking)
    pub current_gold: u64,
    /// Maximum gold held at once
    pub max_gold_held: u64,
    /// Items looted
    pub items_looted: u64,
    /// Items looted by rarity
    pub items_by_rarity: HashMap<String, u64>,
    /// Items looted by type
    pub items_by_type: HashMap<String, u64>,
    /// Items crafted
    pub items_crafted: u64,
    /// Items crafted by type
    pub crafted_by_type: HashMap<String, u64>,
    /// Items sold
    pub items_sold: u64,
    /// Items sold by type
    pub sold_by_type: HashMap<String, u64>,
    /// Total gold from sales
    pub gold_from_sales: u64,
    /// Items purchased
    pub items_purchased: u64,
    /// Gold spent on purchases
    pub gold_on_purchases: u64,
    /// Items enchanted
    pub items_enchanted: u64,
    /// Items repaired
    pub items_repaired: u64,
    /// Gold spent on repairs
    pub gold_on_repairs: u64,
    /// Most valuable item found
    pub most_valuable_item: Option<MostValuableItem>,
    /// Potions consumed
    pub potions_consumed: u64,
    /// Potions consumed by type
    pub potions_by_type: HashMap<String, u64>,
    /// Scrolls used
    pub scrolls_used: u64,
    /// Scrolls by type
    pub scrolls_by_type: HashMap<String, u64>,
    /// Food consumed
    pub food_consumed: u64,
    /// Items upgraded
    pub items_upgraded: u64,
    /// Items destroyed/lost
    pub items_lost: u64,
    /// Gambling wins
    pub gambling_wins: u64,
    /// Gambling losses
    pub gambling_losses: u64,
    /// Total gold from gambling
    pub gold_from_gambling: i64,
}

/// Record of the most valuable item found
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MostValuableItem {
    pub name: String,
    pub value: u64,
    pub rarity: String,
    pub floor_found: u32,
    pub timestamp: u64,
}

impl EconomicStatistics {
    /// Create new economic statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record gold earned
    pub fn record_gold_earned(&mut self, amount: u64, source: &str) {
        self.gold_earned += amount;
        self.current_gold += amount;
        *self.gold_by_source.entry(source.to_string()).or_insert(0) += amount;

        if self.current_gold > self.max_gold_held {
            self.max_gold_held = self.current_gold;
        }
    }

    /// Record gold spent
    pub fn record_gold_spent(&mut self, amount: u64, expense_type: &str) {
        self.gold_spent += amount;
        self.current_gold = self.current_gold.saturating_sub(amount);
        *self.gold_by_expense.entry(expense_type.to_string()).or_insert(0) += amount;
    }

    /// Record item looted
    pub fn record_item_looted(&mut self, item_type: &str, rarity: &str, value: u64, name: &str, floor: u32) {
        self.items_looted += 1;
        *self.items_by_rarity.entry(rarity.to_string()).or_insert(0) += 1;
        *self.items_by_type.entry(item_type.to_string()).or_insert(0) += 1;

        // Check for most valuable
        if self.most_valuable_item.as_ref().map_or(true, |v| value > v.value) {
            self.most_valuable_item = Some(MostValuableItem {
                name: name.to_string(),
                value,
                rarity: rarity.to_string(),
                floor_found: floor,
                timestamp: current_timestamp(),
            });
        }
    }

    /// Record item crafted
    pub fn record_item_crafted(&mut self, item_type: &str) {
        self.items_crafted += 1;
        *self.crafted_by_type.entry(item_type.to_string()).or_insert(0) += 1;
    }

    /// Record item sold
    pub fn record_item_sold(&mut self, item_type: &str, price: u64) {
        self.items_sold += 1;
        self.gold_from_sales += price;
        *self.sold_by_type.entry(item_type.to_string()).or_insert(0) += 1;
    }

    /// Record potion consumed
    pub fn record_potion_consumed(&mut self, potion_type: &str) {
        self.potions_consumed += 1;
        *self.potions_by_type.entry(potion_type.to_string()).or_insert(0) += 1;
    }

    /// Record scroll used
    pub fn record_scroll_used(&mut self, scroll_type: &str) {
        self.scrolls_used += 1;
        *self.scrolls_by_type.entry(scroll_type.to_string()).or_insert(0) += 1;
    }

    /// Record gambling result
    pub fn record_gambling(&mut self, won: bool, amount: i64) {
        if won {
            self.gambling_wins += 1;
            self.gold_from_gambling += amount;
        } else {
            self.gambling_losses += 1;
            self.gold_from_gambling -= amount.abs();
        }
    }

    /// Get net worth (earned - spent)
    pub fn net_worth(&self) -> i64 {
        self.gold_earned as i64 - self.gold_spent as i64
    }

    /// Get gambling win rate
    pub fn gambling_win_rate(&self) -> f32 {
        let total = self.gambling_wins + self.gambling_losses;
        if total == 0 {
            0.0
        } else {
            self.gambling_wins as f32 / total as f32
        }
    }
}

// ============================================================================
// Time Statistics
// ============================================================================

/// Tracks all time-related statistics
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimeStatistics {
    /// Total playtime in seconds
    pub total_playtime_secs: u64,
    /// Playtime by session
    pub session_playtimes: Vec<SessionRecord>,
    /// Time per floor (floor -> seconds)
    pub time_per_floor: HashMap<u32, u64>,
    /// Fastest floor clear (floor -> seconds)
    pub fastest_floor_clear: HashMap<u32, u64>,
    /// Slowest floor clear
    pub slowest_floor_clear: HashMap<u32, u64>,
    /// Average time per floor
    pub average_time_per_floor: HashMap<u32, f64>,
    /// Longest single session in seconds
    pub longest_session_secs: u64,
    /// Longest session details
    pub longest_session_details: Option<SessionRecord>,
    /// Shortest winning session
    pub shortest_winning_session_secs: u64,
    /// Current session start time
    pub current_session_start: u64,
    /// Current floor start time
    pub current_floor_start: u64,
    /// Total game sessions
    pub total_sessions: u32,
    /// Games completed
    pub games_completed: u32,
    /// Games won
    pub games_won: u32,
    /// Average session length
    pub average_session_length_secs: u64,
    /// Time spent in combat (estimated)
    pub combat_time_secs: u64,
    /// Time spent exploring (estimated)
    pub exploration_time_secs: u64,
    /// Time spent in menus/inventory
    pub menu_time_secs: u64,
    /// Fastest game completion
    pub fastest_completion_secs: u64,
    /// Fastest completion details
    pub fastest_completion_details: Option<SpeedRecord>,
    /// Turn count statistics
    pub total_turns: u64,
    /// Turns per floor
    pub turns_per_floor: HashMap<u32, u64>,
    /// Fastest floor by turns
    pub fastest_floor_turns: HashMap<u32, u64>,
}

/// Record of a game session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionRecord {
    pub start_time: u64,
    pub end_time: u64,
    pub duration_secs: u64,
    pub floors_completed: u32,
    pub final_floor: u32,
    pub character_class: String,
    pub victory: bool,
}

/// Record of a speed achievement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeedRecord {
    pub time_secs: u64,
    pub turns: u64,
    pub character_class: String,
    pub final_floor: u32,
    pub timestamp: u64,
}

impl TimeStatistics {
    /// Create new time statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new session
    pub fn start_session(&mut self) {
        self.current_session_start = current_timestamp();
        self.current_floor_start = self.current_session_start;
        self.total_sessions += 1;
    }

    /// End current session
    pub fn end_session(&mut self, floors_completed: u32, final_floor: u32, class: &str, victory: bool) {
        let end_time = current_timestamp();
        let duration = end_time.saturating_sub(self.current_session_start);

        self.total_playtime_secs += duration;

        let record = SessionRecord {
            start_time: self.current_session_start,
            end_time,
            duration_secs: duration,
            floors_completed,
            final_floor,
            character_class: class.to_string(),
            victory,
        };

        // Check for longest session
        if duration > self.longest_session_secs {
            self.longest_session_secs = duration;
            self.longest_session_details = Some(record.clone());
        }

        // Check for fastest winning session
        if victory {
            self.games_won += 1;
            if self.shortest_winning_session_secs == 0 || duration < self.shortest_winning_session_secs {
                self.shortest_winning_session_secs = duration;
            }
            if self.fastest_completion_secs == 0 || duration < self.fastest_completion_secs {
                self.fastest_completion_secs = duration;
                self.fastest_completion_details = Some(SpeedRecord {
                    time_secs: duration,
                    turns: self.total_turns,
                    character_class: class.to_string(),
                    final_floor,
                    timestamp: end_time,
                });
            }
        }

        // Maintain session history
        self.session_playtimes.push(record);
        if self.session_playtimes.len() > MAX_SESSION_HISTORY {
            self.session_playtimes.remove(0);
        }

        // Update averages
        if self.total_sessions > 0 {
            self.average_session_length_secs = self.total_playtime_secs / self.total_sessions as u64;
        }

        self.games_completed += 1;
    }

    /// Record floor completion
    pub fn record_floor_completed(&mut self, floor: u32, turns: u64) {
        let now = current_timestamp();
        let floor_time = now.saturating_sub(self.current_floor_start);

        // Update time per floor
        *self.time_per_floor.entry(floor).or_insert(0) += floor_time;

        // Update fastest/slowest
        let current_fastest = self.fastest_floor_clear.entry(floor).or_insert(u64::MAX);
        if floor_time < *current_fastest {
            *current_fastest = floor_time;
        }

        let current_slowest = self.slowest_floor_clear.entry(floor).or_insert(0);
        if floor_time > *current_slowest {
            *current_slowest = floor_time;
        }

        // Update turns
        self.total_turns += turns;
        *self.turns_per_floor.entry(floor).or_insert(0) += turns;

        let current_fastest_turns = self.fastest_floor_turns.entry(floor).or_insert(u64::MAX);
        if turns < *current_fastest_turns {
            *current_fastest_turns = turns;
        }

        self.current_floor_start = now;
    }

    /// Get win rate
    pub fn win_rate(&self) -> f32 {
        if self.games_completed == 0 {
            0.0
        } else {
            self.games_won as f32 / self.games_completed as f32
        }
    }

    /// Format playtime as readable string
    pub fn format_playtime(&self) -> String {
        format_duration(self.total_playtime_secs)
    }
}

// ============================================================================
// Social Statistics
// ============================================================================

/// Tracks all social/interaction-related statistics
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SocialStatistics {
    /// NPCs talked to (unique NPC names)
    pub npcs_talked_to: HashMap<String, u32>,
    /// Total NPC interactions
    pub total_npc_interactions: u64,
    /// Quests completed
    pub quests_completed: u64,
    /// Quests completed by type
    pub quests_by_type: HashMap<String, u64>,
    /// Quests failed
    pub quests_failed: u64,
    /// Quests abandoned
    pub quests_abandoned: u64,
    /// Current active quests count
    pub active_quests: u32,
    /// Reputation changes (faction -> total change)
    pub reputation_changes: HashMap<String, i64>,
    /// Companions recruited
    pub companions_recruited: u64,
    /// Companion names recruited
    pub companion_names: HashMap<String, u32>,
    /// Companions lost/died
    pub companions_lost: u64,
    /// Total time with companions (turns)
    pub time_with_companions: u64,
    /// Shops visited
    pub shops_visited: u64,
    /// Shop types visited
    pub shop_types: HashMap<String, u32>,
    /// Items traded with NPCs
    pub items_traded: u64,
    /// Favors completed
    pub favors_completed: u64,
    /// Dialogue options chosen
    pub dialogue_choices: u64,
    /// Unique dialogue branches explored
    pub dialogue_branches: HashMap<String, u32>,
    /// Guilds joined
    pub guilds_joined: u64,
    /// Guild names
    pub guild_names: HashMap<String, u32>,
    /// Guild ranks achieved
    pub guild_ranks: HashMap<String, u32>,
    /// NPCs befriended (high reputation)
    pub npcs_befriended: u64,
    /// NPCs angered (low reputation)
    pub npcs_angered: u64,
    /// Bounties completed
    pub bounties_completed: u64,
    /// Contracts fulfilled
    pub contracts_fulfilled: u64,
}

impl SocialStatistics {
    /// Create new social statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record NPC interaction
    pub fn record_npc_interaction(&mut self, npc_name: &str) {
        self.total_npc_interactions += 1;
        *self.npcs_talked_to.entry(npc_name.to_string()).or_insert(0) += 1;
    }

    /// Record quest completion
    pub fn record_quest_completed(&mut self, quest_type: &str) {
        self.quests_completed += 1;
        *self.quests_by_type.entry(quest_type.to_string()).or_insert(0) += 1;
    }

    /// Record quest failed
    pub fn record_quest_failed(&mut self) {
        self.quests_failed += 1;
    }

    /// Record reputation change
    pub fn record_reputation_change(&mut self, faction: &str, change: i64) {
        *self.reputation_changes.entry(faction.to_string()).or_insert(0) += change;
    }

    /// Record companion recruited
    pub fn record_companion_recruited(&mut self, companion_name: &str) {
        self.companions_recruited += 1;
        *self.companion_names.entry(companion_name.to_string()).or_insert(0) += 1;
    }

    /// Record companion lost
    pub fn record_companion_lost(&mut self) {
        self.companions_lost += 1;
    }

    /// Record shop visit
    pub fn record_shop_visit(&mut self, shop_type: &str) {
        self.shops_visited += 1;
        *self.shop_types.entry(shop_type.to_string()).or_insert(0) += 1;
    }

    /// Record guild joined
    pub fn record_guild_joined(&mut self, guild_name: &str) {
        self.guilds_joined += 1;
        *self.guild_names.entry(guild_name.to_string()).or_insert(0) += 1;
    }

    /// Get quest completion rate
    pub fn quest_completion_rate(&self) -> f32 {
        let total = self.quests_completed + self.quests_failed + self.quests_abandoned;
        if total == 0 {
            1.0
        } else {
            self.quests_completed as f32 / total as f32
        }
    }

    /// Get number of unique NPCs met
    pub fn unique_npcs_met(&self) -> usize {
        self.npcs_talked_to.len()
    }
}

// ============================================================================
// Leaderboards
// ============================================================================

/// Personal bests and records tracking
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Leaderboards {
    /// Personal best records by category
    pub personal_bests: HashMap<String, LeaderboardEntry>,
    /// All-time records (global across all games)
    pub all_time_records: HashMap<String, LeaderboardEntry>,
    /// Speed records (time-based achievements)
    pub speed_records: Vec<SpeedLeaderboardEntry>,
    /// Damage records
    pub damage_records: Vec<DamageLeaderboardEntry>,
    /// Historical records for trending
    pub historical_records: HashMap<String, Vec<HistoricalRecord>>,
    /// Records by class
    pub records_by_class: HashMap<String, ClassRecords>,
}

/// A single leaderboard entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub value: u64,
    pub character_class: String,
    pub floor_achieved: u32,
    pub timestamp: u64,
    pub details: Option<String>,
}

/// Speed-specific leaderboard entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeedLeaderboardEntry {
    pub category: String,
    pub time_secs: u64,
    pub turns: u64,
    pub character_class: String,
    pub timestamp: u64,
}

/// Damage-specific leaderboard entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DamageLeaderboardEntry {
    pub category: String,
    pub damage: u64,
    pub enemy_name: String,
    pub skill_used: Option<String>,
    pub was_critical: bool,
    pub timestamp: u64,
}

/// Historical record for tracking progression
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalRecord {
    pub value: u64,
    pub timestamp: u64,
    pub game_number: u32,
}

/// Records for a specific class
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ClassRecords {
    pub highest_floor: u32,
    pub fastest_clear_secs: u64,
    pub most_damage: u64,
    pub most_kills: u64,
    pub victories: u32,
    pub total_games: u32,
}

impl Leaderboards {
    /// Create new leaderboards
    pub fn new() -> Self {
        Self::default()
    }

    /// Update a personal best
    pub fn update_personal_best(&mut self, category: &str, value: u64, class: &str, floor: u32, details: Option<&str>) -> bool {
        let current = self.personal_bests.get(category);

        if current.map_or(true, |c| value > c.value) {
            self.personal_bests.insert(category.to_string(), LeaderboardEntry {
                value,
                character_class: class.to_string(),
                floor_achieved: floor,
                timestamp: current_timestamp(),
                details: details.map(|s| s.to_string()),
            });

            // Also update all-time if applicable
            self.update_all_time_record(category, value, class, floor, details);

            return true;
        }
        false
    }

    /// Update all-time record
    pub fn update_all_time_record(&mut self, category: &str, value: u64, class: &str, floor: u32, details: Option<&str>) -> bool {
        let current = self.all_time_records.get(category);

        if current.map_or(true, |c| value > c.value) {
            self.all_time_records.insert(category.to_string(), LeaderboardEntry {
                value,
                character_class: class.to_string(),
                floor_achieved: floor,
                timestamp: current_timestamp(),
                details: details.map(|s| s.to_string()),
            });
            return true;
        }
        false
    }

    /// Add speed record
    pub fn add_speed_record(&mut self, category: &str, time_secs: u64, turns: u64, class: &str) {
        self.speed_records.push(SpeedLeaderboardEntry {
            category: category.to_string(),
            time_secs,
            turns,
            character_class: class.to_string(),
            timestamp: current_timestamp(),
        });

        // Sort by time (fastest first)
        self.speed_records.sort_by_key(|r| r.time_secs);

        // Keep only top 100
        if self.speed_records.len() > 100 {
            self.speed_records.truncate(100);
        }
    }

    /// Add damage record
    pub fn add_damage_record(&mut self, category: &str, damage: u64, enemy: &str, skill: Option<&str>, critical: bool) {
        self.damage_records.push(DamageLeaderboardEntry {
            category: category.to_string(),
            damage,
            enemy_name: enemy.to_string(),
            skill_used: skill.map(|s| s.to_string()),
            was_critical: critical,
            timestamp: current_timestamp(),
        });

        // Sort by damage (highest first)
        self.damage_records.sort_by_key(|r| std::cmp::Reverse(r.damage));

        // Keep only top 100
        if self.damage_records.len() > 100 {
            self.damage_records.truncate(100);
        }
    }

    /// Add historical record for trending
    pub fn add_historical_record(&mut self, category: &str, value: u64, game_number: u32) {
        let records = self.historical_records.entry(category.to_string()).or_insert_with(Vec::new);

        records.push(HistoricalRecord {
            value,
            timestamp: current_timestamp(),
            game_number,
        });

        // Keep only last 1000 records
        if records.len() > MAX_GRAPH_DATA_POINTS {
            records.remove(0);
        }
    }

    /// Update class-specific records
    pub fn update_class_records(&mut self, class: &str, floor: u32, time_secs: u64, damage: u64, kills: u64, victory: bool) {
        let records = self.records_by_class.entry(class.to_string()).or_insert_with(ClassRecords::default);

        records.total_games += 1;

        if floor > records.highest_floor {
            records.highest_floor = floor;
        }

        if time_secs > 0 && (records.fastest_clear_secs == 0 || time_secs < records.fastest_clear_secs) {
            records.fastest_clear_secs = time_secs;
        }

        if damage > records.most_damage {
            records.most_damage = damage;
        }

        if kills > records.most_kills {
            records.most_kills = kills;
        }

        if victory {
            records.victories += 1;
        }
    }

    /// Get personal best for category
    pub fn get_personal_best(&self, category: &str) -> Option<&LeaderboardEntry> {
        self.personal_bests.get(category)
    }

    /// Get top speed records
    pub fn get_top_speed_records(&self, count: usize) -> &[SpeedLeaderboardEntry] {
        &self.speed_records[..count.min(self.speed_records.len())]
    }

    /// Get top damage records
    pub fn get_top_damage_records(&self, count: usize) -> &[DamageLeaderboardEntry] {
        &self.damage_records[..count.min(self.damage_records.len())]
    }
}

// ============================================================================
// Graphs and Visualizations
// ============================================================================

/// Data structures for graphs and visualizations
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GraphData {
    /// Progress over time data points
    pub progress_over_time: Vec<ProgressDataPoint>,
    /// Stat comparison data
    pub stat_comparisons: HashMap<String, Vec<ComparisonDataPoint>>,
    /// Kill distribution by enemy type
    pub kill_distribution: HashMap<String, u64>,
    /// Damage distribution by type
    pub damage_distribution: HashMap<String, u64>,
    /// Floor progression data
    pub floor_progression: Vec<FloorProgressionPoint>,
    /// Gold over time
    pub gold_over_time: Vec<GoldDataPoint>,
    /// Experience curve
    pub experience_curve: Vec<ExperienceDataPoint>,
    /// Per-session summaries for graphing
    pub session_summaries: Vec<SessionSummary>,
}

/// A single progress data point
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressDataPoint {
    pub timestamp: u64,
    pub game_number: u32,
    pub floor_reached: u32,
    pub kills: u64,
    pub gold: u64,
    pub playtime_secs: u64,
}

/// Comparison data point for stat comparisons
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComparisonDataPoint {
    pub label: String,
    pub value: f64,
    pub timestamp: u64,
}

/// Floor progression tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloorProgressionPoint {
    pub floor: u32,
    pub time_to_reach_secs: u64,
    pub kills_at_floor: u64,
    pub gold_at_floor: u64,
    pub game_number: u32,
}

/// Gold tracking over time
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoldDataPoint {
    pub timestamp: u64,
    pub gold: u64,
    pub floor: u32,
    pub source: String,
}

/// Experience progression tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExperienceDataPoint {
    pub level: u32,
    pub total_xp: u64,
    pub floor: u32,
    pub timestamp: u64,
}

/// Summary of a game session for graphing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSummary {
    pub game_number: u32,
    pub timestamp: u64,
    pub duration_secs: u64,
    pub final_floor: u32,
    pub total_kills: u64,
    pub total_gold: u64,
    pub character_class: String,
    pub victory: bool,
}

impl GraphData {
    /// Create new graph data
    pub fn new() -> Self {
        Self::default()
    }

    /// Add progress data point
    pub fn add_progress_point(&mut self, game_number: u32, floor: u32, kills: u64, gold: u64, playtime: u64) {
        self.progress_over_time.push(ProgressDataPoint {
            timestamp: current_timestamp(),
            game_number,
            floor_reached: floor,
            kills,
            gold,
            playtime_secs: playtime,
        });

        // Keep only last N points
        if self.progress_over_time.len() > MAX_GRAPH_DATA_POINTS {
            self.progress_over_time.remove(0);
        }
    }

    /// Add floor progression point
    pub fn add_floor_progression(&mut self, floor: u32, time: u64, kills: u64, gold: u64, game_number: u32) {
        self.floor_progression.push(FloorProgressionPoint {
            floor,
            time_to_reach_secs: time,
            kills_at_floor: kills,
            gold_at_floor: gold,
            game_number,
        });
    }

    /// Update kill distribution
    pub fn update_kill_distribution(&mut self, enemy_type: &str, count: u64) {
        *self.kill_distribution.entry(enemy_type.to_string()).or_insert(0) += count;
    }

    /// Update damage distribution
    pub fn update_damage_distribution(&mut self, damage_type: &str, amount: u64) {
        *self.damage_distribution.entry(damage_type.to_string()).or_insert(0) += amount;
    }

    /// Add gold data point
    pub fn add_gold_point(&mut self, gold: u64, floor: u32, source: &str) {
        self.gold_over_time.push(GoldDataPoint {
            timestamp: current_timestamp(),
            gold,
            floor,
            source: source.to_string(),
        });

        if self.gold_over_time.len() > MAX_GRAPH_DATA_POINTS {
            self.gold_over_time.remove(0);
        }
    }

    /// Add experience data point
    pub fn add_experience_point(&mut self, level: u32, total_xp: u64, floor: u32) {
        self.experience_curve.push(ExperienceDataPoint {
            level,
            total_xp,
            floor,
            timestamp: current_timestamp(),
        });
    }

    /// Add session summary
    pub fn add_session_summary(&mut self, game_number: u32, duration: u64, floor: u32, kills: u64, gold: u64, class: &str, victory: bool) {
        self.session_summaries.push(SessionSummary {
            game_number,
            timestamp: current_timestamp(),
            duration_secs: duration,
            final_floor: floor,
            total_kills: kills,
            total_gold: gold,
            character_class: class.to_string(),
            victory,
        });

        if self.session_summaries.len() > MAX_SESSION_HISTORY {
            self.session_summaries.remove(0);
        }
    }

    /// Generate ASCII bar chart for kill distribution
    pub fn kill_distribution_chart(&self, width: usize) -> String {
        generate_bar_chart(&self.kill_distribution, "Kill Distribution", width)
    }

    /// Generate ASCII bar chart for damage distribution
    pub fn damage_distribution_chart(&self, width: usize) -> String {
        generate_bar_chart(&self.damage_distribution, "Damage Distribution", width)
    }

    /// Get average metrics from progress data
    pub fn get_averages(&self) -> (f64, f64, f64) {
        if self.progress_over_time.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let count = self.progress_over_time.len() as f64;
        let avg_floor = self.progress_over_time.iter().map(|p| p.floor_reached as f64).sum::<f64>() / count;
        let avg_kills = self.progress_over_time.iter().map(|p| p.kills as f64).sum::<f64>() / count;
        let avg_gold = self.progress_over_time.iter().map(|p| p.gold as f64).sum::<f64>() / count;

        (avg_floor, avg_kills, avg_gold)
    }
}

// ============================================================================
// Main Statistics System
// ============================================================================

/// The main statistics system that aggregates all statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatisticsSystem {
    /// Combat statistics
    pub combat: CombatStatistics,
    /// Exploration statistics
    pub exploration: ExplorationStatistics,
    /// Economic statistics
    pub economic: EconomicStatistics,
    /// Time statistics
    pub time: TimeStatistics,
    /// Social statistics
    pub social: SocialStatistics,
    /// Leaderboards
    pub leaderboards: Leaderboards,
    /// Graph data
    pub graphs: GraphData,
    /// System version for compatibility
    pub version: u32,
    /// Total games played
    pub total_games: u32,
    /// Current game number
    pub current_game_number: u32,
    /// Statistics creation timestamp
    pub created_at: u64,
    /// Last updated timestamp
    pub last_updated: u64,
}

impl Default for StatisticsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl StatisticsSystem {
    /// Create a new statistics system
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
            combat: CombatStatistics::new(),
            exploration: ExplorationStatistics::new(),
            economic: EconomicStatistics::new(),
            time: TimeStatistics::new(),
            social: SocialStatistics::new(),
            leaderboards: Leaderboards::new(),
            graphs: GraphData::new(),
            version: 1,
            total_games: 0,
            current_game_number: 0,
            created_at: now,
            last_updated: now,
        }
    }

    /// Start a new game session
    pub fn start_game(&mut self, character_class: &str) {
        self.total_games += 1;
        self.current_game_number = self.total_games;
        self.time.start_session();
        self.combat.current_kill_streak = 0;
        self.economic.current_gold = 0;
        self.last_updated = current_timestamp();
    }

    /// End current game session
    pub fn end_game(&mut self, floors_completed: u32, final_floor: u32, class: &str, victory: bool) {
        self.time.end_session(floors_completed, final_floor, class, victory);

        // Update leaderboards
        self.leaderboards.update_personal_best(
            "highest_floor",
            final_floor as u64,
            class,
            final_floor,
            None,
        );

        self.leaderboards.update_personal_best(
            "most_kills",
            self.combat.total_enemies_killed,
            class,
            final_floor,
            None,
        );

        self.leaderboards.update_personal_best(
            "highest_damage",
            self.combat.total_damage_dealt,
            class,
            final_floor,
            None,
        );

        self.leaderboards.update_personal_best(
            "most_gold",
            self.economic.gold_earned,
            class,
            final_floor,
            None,
        );

        // Update class records
        self.leaderboards.update_class_records(
            class,
            final_floor,
            self.time.total_playtime_secs,
            self.combat.total_damage_dealt,
            self.combat.total_enemies_killed,
            victory,
        );

        // Add to graphs
        self.graphs.add_session_summary(
            self.current_game_number,
            self.time.total_playtime_secs,
            final_floor,
            self.combat.total_enemies_killed,
            self.economic.gold_earned,
            class,
            victory,
        );

        self.graphs.add_progress_point(
            self.current_game_number,
            final_floor,
            self.combat.total_enemies_killed,
            self.economic.gold_earned,
            self.time.total_playtime_secs,
        );

        // Add historical records
        self.leaderboards.add_historical_record("floor", final_floor as u64, self.current_game_number);
        self.leaderboards.add_historical_record("kills", self.combat.total_enemies_killed, self.current_game_number);
        self.leaderboards.add_historical_record("gold", self.economic.gold_earned, self.current_game_number);

        self.last_updated = current_timestamp();
    }

    /// Record a complete combat action
    pub fn record_combat(&mut self, damage: u64, damage_type: &str, enemy: &str, floor: u32, skill: Option<&str>, is_critical: bool, is_kill: bool, overkill: u64) {
        self.combat.record_damage_dealt(damage, damage_type, enemy, floor, skill, is_critical);

        if is_kill {
            self.combat.record_kill(enemy, floor, overkill);
            self.graphs.update_kill_distribution(enemy, 1);
        }

        self.graphs.update_damage_distribution(damage_type, damage);
        self.last_updated = current_timestamp();
    }

    /// Record floor completion
    pub fn record_floor_completed(&mut self, floor: u32, turns: u64) {
        self.exploration.record_floor_visited(floor);
        self.time.record_floor_completed(floor, turns);

        self.graphs.add_floor_progression(
            floor,
            self.time.total_playtime_secs,
            self.combat.total_enemies_killed,
            self.economic.gold_earned,
            self.current_game_number,
        );

        self.last_updated = current_timestamp();
    }

    /// Get overall statistics summary
    pub fn get_summary(&self) -> StatisticsSummary {
        StatisticsSummary {
            total_games: self.total_games,
            total_playtime: format_duration(self.time.total_playtime_secs),
            total_kills: self.combat.total_enemies_killed,
            total_deaths: self.combat.deaths,
            total_gold_earned: self.economic.gold_earned,
            deepest_floor: self.exploration.deepest_floor,
            highest_damage: self.combat.highest_hit,
            bosses_defeated: self.combat.total_bosses_defeated,
            win_rate: self.time.win_rate(),
            kill_death_ratio: self.combat.kill_death_ratio(),
            average_floor: self.graphs.get_averages().0,
            quests_completed: self.social.quests_completed,
        }
    }

    /// Format comprehensive statistics display
    pub fn format_display(&self) -> String {
        let summary = self.get_summary();

        format!(
            r#"
================================================================================
                         SHADOWCRYPT STATISTICS
================================================================================

  OVERVIEW
  --------
  Total Games Played:   {:>10}
  Total Playtime:       {:>10}
  Win Rate:             {:>9.1}%

  COMBAT
  ------
  Total Kills:          {:>10}
  Total Deaths:         {:>10}
  K/D Ratio:            {:>10.2}
  Highest Hit:          {:>10}
  Critical Hits:        {:>10}
  Bosses Defeated:      {:>10}

  EXPLORATION
  -----------
  Deepest Floor:        {:>10}
  Rooms Discovered:     {:>10}
  Secrets Found:        {:>10}
  Distance Traveled:    {:>10}

  ECONOMY
  -------
  Gold Earned:          {:>10}
  Gold Spent:           {:>10}
  Items Looted:         {:>10}
  Items Crafted:        {:>10}

  SOCIAL
  ------
  NPCs Met:             {:>10}
  Quests Completed:     {:>10}
  Companions Recruited: {:>10}

================================================================================
"#,
            summary.total_games,
            summary.total_playtime,
            summary.win_rate * 100.0,
            summary.total_kills,
            summary.total_deaths,
            summary.kill_death_ratio,
            summary.highest_damage,
            self.combat.critical_hits,
            summary.bosses_defeated,
            summary.deepest_floor,
            self.exploration.rooms_discovered,
            self.exploration.secret_areas_found,
            self.exploration.distance_traveled,
            summary.total_gold_earned,
            self.economic.gold_spent,
            self.economic.items_looted,
            self.economic.items_crafted,
            self.social.unique_npcs_met(),
            summary.quests_completed,
            self.social.companions_recruited,
        )
    }

    /// Format combat statistics
    pub fn format_combat_stats(&self) -> String {
        let mut output = String::from(
            r#"
================================================================================
                           COMBAT STATISTICS
================================================================================
"#,
        );

        output.push_str(&format!(
            "
  Damage Statistics
  -----------------
  Total Damage Dealt:     {:>12}
  Total Damage Received:  {:>12}
  Highest Single Hit:     {:>12}
  Average Damage/Attack:  {:>12.1}
  Critical Hit Rate:      {:>11.1}%

  Combat Record
  -------------
  Total Attacks:          {:>12}
  Critical Hits:          {:>12}
  Successful Dodges:      {:>12}
  Successful Blocks:      {:>12}
  Successful Parries:     {:>12}

  Kill Statistics
  ---------------
  Total Kills:            {:>12}
  Kill/Death Ratio:       {:>12.2}
  Highest Kill Streak:    {:>12}
  Overkill Damage:        {:>12}

  Skills & Spells
  ---------------
  Total Skills Used:      {:>12}
  Total Spells Cast:      {:>12}
",
            self.combat.total_damage_dealt,
            self.combat.total_damage_received,
            self.combat.highest_hit,
            self.combat.average_damage_per_attack,
            self.combat.critical_hit_rate * 100.0,
            self.combat.total_attacks,
            self.combat.critical_hits,
            self.combat.successful_dodges,
            self.combat.successful_blocks,
            self.combat.successful_parries,
            self.combat.total_enemies_killed,
            self.combat.kill_death_ratio(),
            self.combat.highest_kill_streak,
            self.combat.total_overkill_damage,
            self.combat.total_skills_used,
            self.combat.total_spells_cast,
        ));

        // Add top killed enemies
        output.push_str("\n  Top Killed Enemies\n  ------------------\n");
        let mut kills: Vec<_> = self.combat.enemies_killed_by_type.iter().collect();
        kills.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (enemy, count) in kills.iter().take(10) {
            output.push_str(&format!("  {:.<25} {:>8}\n", enemy, count));
        }

        output.push_str("\n================================================================================\n");
        output
    }

    /// Format leaderboard display
    pub fn format_leaderboards(&self) -> String {
        let mut output = String::from(
            r#"
================================================================================
                            LEADERBOARDS
================================================================================

  PERSONAL BESTS
  --------------
"#,
        );

        for (category, entry) in &self.leaderboards.personal_bests {
            output.push_str(&format!(
                "  {:.<25} {:>12} ({})\n",
                category,
                entry.value,
                entry.character_class
            ));
        }

        output.push_str("\n  TOP SPEED RECORDS\n  -----------------\n");
        for (i, record) in self.leaderboards.get_top_speed_records(5).iter().enumerate() {
            output.push_str(&format!(
                "  {}. {} - {} ({} turns, {})\n",
                i + 1,
                record.category,
                format_duration(record.time_secs),
                record.turns,
                record.character_class
            ));
        }

        output.push_str("\n  TOP DAMAGE RECORDS\n  ------------------\n");
        for (i, record) in self.leaderboards.get_top_damage_records(5).iter().enumerate() {
            let crit = if record.was_critical { " [CRIT]" } else { "" };
            output.push_str(&format!(
                "  {}. {} damage to {}{}\n",
                i + 1,
                record.damage,
                record.enemy_name,
                crit
            ));
        }

        output.push_str("\n================================================================================\n");
        output
    }

    /// Generate ASCII progress chart
    pub fn format_progress_chart(&self) -> String {
        let mut output = String::from(
            r#"
================================================================================
                         PROGRESS OVER TIME
================================================================================

"#,
        );

        if self.graphs.progress_over_time.is_empty() {
            output.push_str("  No progress data available yet. Play some games!\n");
            return output;
        }

        // Create ASCII sparkline for floors
        let floors: Vec<u32> = self.graphs.progress_over_time.iter().map(|p| p.floor_reached).collect();
        output.push_str("  Floor Reached:\n");
        output.push_str(&format!("  {}\n\n", generate_sparkline(&floors, 50)));

        // Create ASCII sparkline for kills
        let kills: Vec<u64> = self.graphs.progress_over_time.iter().map(|p| p.kills).collect();
        output.push_str("  Kills:\n");
        output.push_str(&format!("  {}\n\n", generate_sparkline_u64(&kills, 50)));

        // Create ASCII sparkline for gold
        let gold: Vec<u64> = self.graphs.progress_over_time.iter().map(|p| p.gold).collect();
        output.push_str("  Gold Earned:\n");
        output.push_str(&format!("  {}\n", generate_sparkline_u64(&gold, 50)));

        output.push_str("\n================================================================================\n");
        output
    }
}

/// Summary of overall statistics
#[derive(Clone, Debug)]
pub struct StatisticsSummary {
    pub total_games: u32,
    pub total_playtime: String,
    pub total_kills: u64,
    pub total_deaths: u32,
    pub total_gold_earned: u64,
    pub deepest_floor: u32,
    pub highest_damage: u64,
    pub bosses_defeated: u32,
    pub win_rate: f32,
    pub kill_death_ratio: f32,
    pub average_floor: f64,
    pub quests_completed: u64,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get current Unix timestamp
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Format duration in seconds as readable string
fn format_duration(secs: u64) -> String {
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Generate ASCII bar chart
fn generate_bar_chart(data: &HashMap<String, u64>, title: &str, width: usize) -> String {
    if data.is_empty() {
        return format!("  {} - No data available\n", title);
    }

    let mut output = format!("\n  {}\n  {}\n", title, "-".repeat(title.len()));

    let max_value = *data.values().max().unwrap_or(&1);
    let max_label_len = data.keys().map(|k| k.len()).max().unwrap_or(10).min(20);

    let mut sorted: Vec<_> = data.iter().collect();
    sorted.sort_by_key(|(_, v)| std::cmp::Reverse(*v));

    for (label, value) in sorted.iter().take(10) {
        let bar_width = if max_value > 0 {
            ((**value as f64 / max_value as f64) * width as f64) as usize
        } else {
            0
        };

        let truncated_label = if label.len() > max_label_len {
            format!("{}...", &label[..max_label_len - 3])
        } else {
            format!("{:width$}", label, width = max_label_len)
        };

        output.push_str(&format!(
            "  {} |{} {}\n",
            truncated_label,
            "=".repeat(bar_width),
            value
        ));
    }

    output
}

/// Generate ASCII sparkline from u32 data
fn generate_sparkline(data: &[u32], width: usize) -> String {
    if data.is_empty() {
        return String::from("No data");
    }

    let chars = ['_', '.', '-', '=', '*', '#', '@'];
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&1);
    let range = (max - min).max(1) as f64;

    let step = (data.len() as f64 / width as f64).max(1.0);
    let mut result = String::new();

    for i in 0..width {
        let idx = (i as f64 * step) as usize;
        if idx < data.len() {
            let normalized = ((data[idx] - min) as f64 / range * (chars.len() - 1) as f64) as usize;
            result.push(chars[normalized.min(chars.len() - 1)]);
        }
    }

    result
}

/// Generate ASCII sparkline from u64 data
fn generate_sparkline_u64(data: &[u64], width: usize) -> String {
    if data.is_empty() {
        return String::from("No data");
    }

    let chars = ['_', '.', '-', '=', '*', '#', '@'];
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&1);
    let range = (max - min).max(1) as f64;

    let step = (data.len() as f64 / width as f64).max(1.0);
    let mut result = String::new();

    for i in 0..width {
        let idx = (i as f64 * step) as usize;
        if idx < data.len() {
            let normalized = ((data[idx] - min) as f64 / range * (chars.len() - 1) as f64) as usize;
            result.push(chars[normalized.min(chars.len() - 1)]);
        }
    }

    result
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_statistics() {
        let mut stats = CombatStatistics::new();

        stats.record_damage_dealt(100, "physical", "Goblin", 1, None, false);
        assert_eq!(stats.total_damage_dealt, 100);
        assert_eq!(stats.total_attacks, 1);

        stats.record_damage_dealt(200, "fire", "Orc", 2, Some("Fireball"), true);
        assert_eq!(stats.total_damage_dealt, 300);
        assert_eq!(stats.critical_hits, 1);
        assert_eq!(stats.highest_hit, 200);
    }

    #[test]
    fn test_kill_tracking() {
        let mut stats = CombatStatistics::new();

        stats.record_kill("Goblin", 1, 5);
        stats.record_kill("Goblin", 1, 10);
        stats.record_kill("Orc", 2, 0);

        assert_eq!(stats.total_enemies_killed, 3);
        assert_eq!(stats.enemies_killed_by_type.get("Goblin"), Some(&2));
        assert_eq!(stats.enemies_killed_by_type.get("Orc"), Some(&1));
        assert_eq!(stats.total_overkill_damage, 15);
    }

    #[test]
    fn test_exploration_statistics() {
        let mut stats = ExplorationStatistics::new();

        stats.record_floor_visited(5);
        assert_eq!(stats.deepest_floor, 5);

        stats.record_floor_visited(3);
        assert_eq!(stats.deepest_floor, 5); // Should not decrease

        stats.record_room_discovered(1);
        stats.record_room_discovered(1);
        assert_eq!(stats.rooms_discovered, 2);
    }

    #[test]
    fn test_economic_statistics() {
        let mut stats = EconomicStatistics::new();

        stats.record_gold_earned(100, "enemy");
        stats.record_gold_earned(50, "chest");
        assert_eq!(stats.gold_earned, 150);
        assert_eq!(stats.current_gold, 150);

        stats.record_gold_spent(30, "items");
        assert_eq!(stats.current_gold, 120);
        assert_eq!(stats.gold_spent, 30);
    }

    #[test]
    fn test_time_statistics() {
        let mut stats = TimeStatistics::new();

        stats.start_session();
        assert!(stats.current_session_start > 0);

        stats.record_floor_completed(1, 100);
        assert_eq!(stats.total_turns, 100);
    }

    #[test]
    fn test_leaderboards() {
        let mut leaderboards = Leaderboards::new();

        let updated = leaderboards.update_personal_best("kills", 100, "Warrior", 5, None);
        assert!(updated);

        let updated = leaderboards.update_personal_best("kills", 50, "Mage", 3, None);
        assert!(!updated); // Lower value should not update

        let updated = leaderboards.update_personal_best("kills", 150, "Rogue", 10, None);
        assert!(updated);

        let best = leaderboards.get_personal_best("kills").unwrap();
        assert_eq!(best.value, 150);
    }

    #[test]
    fn test_statistics_system() {
        let mut system = StatisticsSystem::new();

        system.start_game("Warrior");
        assert_eq!(system.total_games, 1);

        system.record_combat(50, "physical", "Goblin", 1, None, false, true, 5);
        assert_eq!(system.combat.total_enemies_killed, 1);

        system.record_floor_completed(1, 50);
        assert_eq!(system.exploration.deepest_floor, 1);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m 1s");
    }

    #[test]
    fn test_sparkline() {
        let data = vec![1, 5, 3, 8, 2];
        let sparkline = generate_sparkline(&data, 5);
        assert_eq!(sparkline.len(), 5);
    }

    #[test]
    fn test_kill_death_ratio() {
        let mut stats = CombatStatistics::new();

        // No deaths
        assert_eq!(stats.kill_death_ratio(), 0.0);

        stats.record_kill("Goblin", 1, 0);
        stats.record_kill("Goblin", 1, 0);
        assert_eq!(stats.kill_death_ratio(), 2.0);

        stats.record_death();
        assert_eq!(stats.kill_death_ratio(), 2.0);
    }

    #[test]
    fn test_graph_data() {
        let mut graphs = GraphData::new();

        graphs.add_progress_point(1, 5, 100, 500, 3600);
        assert_eq!(graphs.progress_over_time.len(), 1);

        graphs.update_kill_distribution("Goblin", 10);
        graphs.update_kill_distribution("Orc", 5);
        assert_eq!(graphs.kill_distribution.get("Goblin"), Some(&10));
    }

    #[test]
    fn test_social_statistics() {
        let mut stats = SocialStatistics::new();

        stats.record_npc_interaction("Merchant Bob");
        stats.record_npc_interaction("Merchant Bob");
        stats.record_npc_interaction("Guard Tom");

        assert_eq!(stats.total_npc_interactions, 3);
        assert_eq!(stats.unique_npcs_met(), 2);
    }
}

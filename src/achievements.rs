// ============================================================================
// ACHIEVEMENTS MODULE
// ============================================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Write, Read as IoRead};
use std::path::PathBuf;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

use crate::{CharacterClass, Rarity};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub unlocked: bool,
    pub unlock_date: Option<String>,
    pub reward: Option<AchievementReward>,
    pub hidden: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementReward {
    Title(String),
    StartingBonus { stat: String, amount: i32 },
    #[allow(dead_code)]
    UnlockSpecies(String),
    UnlockClass(String),
    #[allow(dead_code)]
    CosmeticColor(String),
}

impl Achievement {
    pub fn all_achievements() -> Vec<Achievement> {
        vec![
            // Combat achievements
            Achievement { id: 1, name: "First Blood".into(), description: "Kill your first enemy".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 2, name: "Warrior".into(), description: "Kill 100 enemies".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Warrior".into())), hidden: false },
            Achievement { id: 3, name: "Slayer".into(), description: "Kill 500 enemies".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Slayer".into())), hidden: false },
            Achievement { id: 4, name: "Genocide".into(), description: "Kill 1000 enemies".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::StartingBonus { stat: "ATK".into(), amount: 5 }), hidden: false },
            Achievement { id: 5, name: "Legend".into(), description: "Kill 5000 enemies".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Legendary".into())), hidden: true },

            // Boss achievements
            Achievement { id: 10, name: "Kingslayer".into(), description: "Defeat the Goblin King".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 11, name: "Warlord's Bane".into(), description: "Defeat the Orc Warlord".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 12, name: "Vampire Hunter".into(), description: "Defeat the Vampire Lord".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 13, name: "Nature's Fury".into(), description: "Defeat the Forest Guardian".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 14, name: "Dragon Slayer".into(), description: "Defeat the Ice Dragon".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Dragon Slayer".into())), hidden: false },
            Achievement { id: 15, name: "Demon Vanquisher".into(), description: "Defeat the Demon King".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Demon Vanquisher".into())), hidden: false },
            Achievement { id: 16, name: "Speedrunner".into(), description: "Beat the game in under 1000 turns".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Speed Demon".into())), hidden: true },

            // Exploration achievements
            Achievement { id: 20, name: "Explorer".into(), description: "Explore 50 rooms".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 21, name: "Delver".into(), description: "Reach floor 10".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 22, name: "Deep Diver".into(), description: "Reach floor 20".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 23, name: "Abyssal".into(), description: "Reach floor 30".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },

            // Class achievements
            Achievement { id: 30, name: "Master Warrior".into(), description: "Beat the game as Warrior".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 31, name: "Master Mage".into(), description: "Beat the game as Mage".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 32, name: "Master Rogue".into(), description: "Beat the game as Rogue".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 33, name: "Master Paladin".into(), description: "Beat the game as Paladin".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 34, name: "Master Ranger".into(), description: "Beat the game as Ranger".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 35, name: "Master Necromancer".into(), description: "Beat the game as Necromancer".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 36, name: "Master of All".into(), description: "Beat the game with all 6 classes".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::UnlockClass("Legendary Hero".into())), hidden: true },

            // Misc achievements
            Achievement { id: 50, name: "Wealthy".into(), description: "Collect 10000 gold total".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 51, name: "Collector".into(), description: "Find 50 unique items".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 52, name: "Pacifist".into(), description: "Complete floor 1 without killing".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Pacifist".into())), hidden: true },
            Achievement { id: 53, name: "Survivor".into(), description: "Survive with 1 HP".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 54, name: "Lucky".into(), description: "Find a Mythic item".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },

            // Death achievements
            Achievement { id: 60, name: "First Death".into(), description: "Die for the first time".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 61, name: "Determined".into(), description: "Die 10 times".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 62, name: "Immortal".into(), description: "Beat the game without dying".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Immortal".into())), hidden: true },

            // Level achievements
            Achievement { id: 70, name: "Veteran".into(), description: "Reach player level 10".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 71, name: "Champion".into(), description: "Reach player level 20".into(), unlocked: false, unlock_date: None, reward: None, hidden: false },
            Achievement { id: 72, name: "Legendary".into(), description: "Reach player level 30".into(), unlocked: false, unlock_date: None, reward: Some(AchievementReward::Title("Legendary".into())), hidden: false },
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AchievementStats {
    pub total_kills: u32,
    pub total_deaths: u32,
    pub total_gold: u32,
    #[allow(dead_code)]
    pub rooms_explored: u32,
    pub max_floor_reached: u32,
    pub max_player_level: u32,
    pub unique_items_found: Vec<String>,
    pub classes_beaten: Vec<String>,
    pub bosses_defeated: Vec<String>,
    #[allow(dead_code)]
    pub games_won: u32,
    #[allow(dead_code)]
    pub games_won_without_death: u32,
    #[allow(dead_code)]
    pub floor1_pacifist_run: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTracker {
    pub achievements: Vec<Achievement>,
    pub stats: AchievementStats,
    pub pending_notifications: Vec<String>,
    pub current_session_deaths: u32,
    pub current_session_floor1_kills: u32,
}

impl AchievementTracker {
    pub fn new() -> Self {
        Self {
            achievements: Achievement::all_achievements(),
            stats: AchievementStats::default(),
            pending_notifications: Vec::new(),
            current_session_deaths: 0,
            current_session_floor1_kills: 0,
        }
    }

    pub fn save_path() -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let dir = PathBuf::from(home).join(".shadowcrypt");
        let _ = fs::create_dir_all(&dir);
        dir.join("achievements.bin")
    }

    pub fn load() -> Self {
        let path = Self::save_path();
        if let Ok(data) = fs::read(&path) {
            let mut decoder = GzDecoder::new(&data[..]);
            let mut decompressed = Vec::new();
            if decoder.read_to_end(&mut decompressed).is_ok() {
                if let Ok(tracker) = bincode::deserialize::<AchievementTracker>(&decompressed) {
                    return tracker;
                }
            }
        }
        Self::new()
    }

    pub fn save(&self) {
        let path = Self::save_path();
        if let Ok(data) = bincode::serialize(self) {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            if encoder.write_all(&data).is_ok() {
                if let Ok(compressed) = encoder.finish() {
                    let _ = fs::write(path, compressed);
                }
            }
        }
    }

    pub fn unlock(&mut self, id: u32) -> bool {
        if let Some(ach) = self.achievements.iter_mut().find(|a| a.id == id) {
            if !ach.unlocked {
                ach.unlocked = true;
                ach.unlock_date = Some(chrono_lite_now());
                self.pending_notifications.push(format!("Achievement Unlocked: {}!", ach.name));
                self.save();
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_unlocked(&self, id: u32) -> bool {
        self.achievements.iter().any(|a| a.id == id && a.unlocked)
    }

    pub fn pop_notification(&mut self) -> Option<String> {
        if self.pending_notifications.is_empty() {
            None
        } else {
            Some(self.pending_notifications.remove(0))
        }
    }

    pub fn check_kill_achievements(&mut self, total_kills: u32) {
        self.stats.total_kills = total_kills;
        if total_kills >= 1 { self.unlock(1); }
        if total_kills >= 100 { self.unlock(2); }
        if total_kills >= 500 { self.unlock(3); }
        if total_kills >= 1000 { self.unlock(4); }
        if total_kills >= 5000 { self.unlock(5); }
        self.save();
    }

    pub fn check_boss_achievement(&mut self, boss_name: &str) {
        if !self.stats.bosses_defeated.contains(&boss_name.to_string()) {
            self.stats.bosses_defeated.push(boss_name.to_string());
        }
        match boss_name {
            "GOBLIN KING" => { self.unlock(10); }
            "ORC WARLORD" => { self.unlock(11); }
            "VAMPIRE LORD" => { self.unlock(12); }
            "FOREST GUARDIAN" => { self.unlock(13); }
            "ICE DRAGON" => { self.unlock(14); }
            "DEMON KING" => { self.unlock(15); }
            _ => {}
        }
        self.save();
    }

    pub fn check_floor_achievements(&mut self, floor: u32) {
        if floor > self.stats.max_floor_reached {
            self.stats.max_floor_reached = floor;
        }
        if floor >= 10 { self.unlock(21); }
        if floor >= 20 { self.unlock(22); }
        if floor >= 30 { self.unlock(23); }
        self.save();
    }

    pub fn check_level_achievements(&mut self, player_level: u32) {
        if player_level > self.stats.max_player_level {
            self.stats.max_player_level = player_level;
        }
        if player_level >= 10 { self.unlock(70); }
        if player_level >= 20 { self.unlock(71); }
        if player_level >= 30 { self.unlock(72); }
        self.save();
    }

    pub fn check_gold_achievement(&mut self, total_gold: u32) {
        if total_gold > self.stats.total_gold {
            self.stats.total_gold = total_gold;
        }
        if total_gold >= 10000 { self.unlock(50); }
        self.save();
    }

    pub fn check_item_found(&mut self, item_name: &str, rarity: Rarity) {
        if !self.stats.unique_items_found.contains(&item_name.to_string()) {
            self.stats.unique_items_found.push(item_name.to_string());
        }
        if self.stats.unique_items_found.len() >= 50 { self.unlock(51); }
        if rarity == Rarity::Mythic { self.unlock(54); }
        self.save();
    }

    pub fn check_survival(&mut self, hp: i32) {
        if hp == 1 { self.unlock(53); }
    }

    pub fn record_death(&mut self) {
        self.stats.total_deaths += 1;
        self.current_session_deaths += 1;
        if self.stats.total_deaths >= 1 { self.unlock(60); }
        if self.stats.total_deaths >= 10 { self.unlock(61); }
        self.save();
    }

    pub fn record_victory(&mut self, class: CharacterClass, turns: u32) {
        self.stats.games_won += 1;

        let class_name = class.name().to_string();
        if !self.stats.classes_beaten.contains(&class_name) {
            self.stats.classes_beaten.push(class_name);
        }

        match class {
            CharacterClass::Warrior => { self.unlock(30); }
            CharacterClass::Mage => { self.unlock(31); }
            CharacterClass::Rogue => { self.unlock(32); }
            CharacterClass::Paladin => { self.unlock(33); }
            CharacterClass::Ranger => { self.unlock(34); }
            CharacterClass::Necromancer => { self.unlock(35); }
        }

        if self.stats.classes_beaten.len() >= 6 { self.unlock(36); }
        if turns < 1000 { self.unlock(16); }
        if self.current_session_deaths == 0 {
            self.stats.games_won_without_death += 1;
            self.unlock(62);
        }

        self.save();
    }

    pub fn record_floor1_kill(&mut self) {
        self.current_session_floor1_kills += 1;
    }

    pub fn check_pacifist(&mut self, completed_floor1: bool) {
        if completed_floor1 && self.current_session_floor1_kills == 0 {
            self.stats.floor1_pacifist_run = true;
            self.unlock(52);
            self.save();
        }
    }

    pub fn reset_session(&mut self) {
        self.current_session_deaths = 0;
        self.current_session_floor1_kills = 0;
    }

    pub fn unlocked_count(&self) -> usize {
        self.achievements.iter().filter(|a| a.unlocked).count()
    }

    pub fn total_count(&self) -> usize {
        self.achievements.len()
    }

    pub fn get_visible_achievements(&self) -> Vec<&Achievement> {
        self.achievements.iter()
            .filter(|a| !a.hidden || a.unlocked)
            .collect()
    }
}

pub fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    format!("{:04}-{:02}-{:02}", years, month, day)
}

//! Comprehensive Save/Load System for ShadowCrypt
//!
//! This module provides a complete save/load system with:
//! - Multiple save slots (10 slots + autosave + quicksave)
//! - Auto-save with configurable intervals
//! - Quick save functionality
//! - Cloud save support (stub for future implementation)
//! - Save file integrity verification with checksums
//! - Corruption detection and recovery
//! - Export/import functionality
//! - New Game Plus support
//! - Pause system integration

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};

use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use serde::{Serialize, Deserialize};

use crate::game::GameState;
use crate::classes::CharacterClass;
use crate::items::{Item, Rarity};
use crate::achievements::AchievementTracker;

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of manual save slots
pub const MAX_SAVE_SLOTS: usize = 10;

/// Default auto-save interval in seconds
pub const DEFAULT_AUTOSAVE_INTERVAL: u64 = 300; // 5 minutes

/// Save file magic number for validation
const SAVE_MAGIC: [u8; 4] = [0x53, 0x43, 0x52, 0x59]; // "SCRY"

/// Current save file version
const SAVE_VERSION: u32 = 1;

/// Maximum backup files to keep
const MAX_BACKUPS: usize = 5;

// ============================================================================
// Error Types
// ============================================================================

/// Error type for save/load operations
#[derive(Debug)]
pub enum SaveError {
    /// I/O error during file operations
    IoError(std::io::Error),
    /// Serialization failed
    SerializeError(String),
    /// Deserialization failed
    DeserializeError(String),
    /// Save file is corrupted
    CorruptedSave(String),
    /// Checksum verification failed
    ChecksumMismatch { expected: u32, actual: u32 },
    /// Invalid save slot
    InvalidSlot(usize),
    /// Save file not found
    SaveNotFound(String),
    /// Version mismatch
    VersionMismatch { expected: u32, actual: u32 },
    /// Invalid magic number
    InvalidMagic,
    /// Cloud sync error
    CloudSyncError(String),
    /// Import/export error
    ImportExportError(String),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "I/O error: {}", e),
            Self::SerializeError(e) => write!(f, "Serialization error: {}", e),
            Self::DeserializeError(e) => write!(f, "Deserialization error: {}", e),
            Self::CorruptedSave(e) => write!(f, "Corrupted save: {}", e),
            Self::ChecksumMismatch { expected, actual } => {
                write!(f, "Checksum mismatch: expected {:08x}, got {:08x}", expected, actual)
            }
            Self::InvalidSlot(slot) => write!(f, "Invalid slot: {}", slot),
            Self::SaveNotFound(path) => write!(f, "Save not found: {}", path),
            Self::VersionMismatch { expected, actual } => {
                write!(f, "Version mismatch: expected {}, got {}", expected, actual)
            }
            Self::InvalidMagic => write!(f, "Invalid save file format"),
            Self::CloudSyncError(e) => write!(f, "Cloud sync error: {}", e),
            Self::ImportExportError(e) => write!(f, "Import/export error: {}", e),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<std::io::Error> for SaveError {
    fn from(err: std::io::Error) -> Self {
        SaveError::IoError(err)
    }
}

// ============================================================================
// Save Slot Types
// ============================================================================

/// Types of save slots
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum SaveSlotType {
    /// Manual save slots (0-9)
    Manual(usize),
    /// Auto-save slot
    AutoSave,
    /// Quick save slot
    QuickSave,
    /// Cloud save slot
    CloudSave,
}

impl SaveSlotType {
    /// Get the filename for this slot type
    pub fn filename(&self) -> String {
        match self {
            Self::Manual(slot) => format!("save_{:02}.sav", slot),
            Self::AutoSave => "autosave.sav".to_string(),
            Self::QuickSave => "quicksave.sav".to_string(),
            Self::CloudSave => "cloudsave.sav".to_string(),
        }
    }

    /// Get display name for this slot type
    pub fn display_name(&self) -> String {
        match self {
            Self::Manual(slot) => format!("Save Slot {}", slot + 1),
            Self::AutoSave => "Auto Save".to_string(),
            Self::QuickSave => "Quick Save".to_string(),
            Self::CloudSave => "Cloud Save".to_string(),
        }
    }
}

// ============================================================================
// Save Metadata
// ============================================================================

/// Metadata stored with each save file for preview
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveMetadata {
    /// Save file version
    pub version: u32,
    /// Timestamp when saved (Unix epoch seconds)
    pub timestamp: u64,
    /// Player's character class
    pub character_class: CharacterClass,
    /// Player's level
    pub player_level: u32,
    /// Current dungeon level
    pub dungeon_level: u32,
    /// Total play time in seconds
    pub play_time_seconds: u64,
    /// Current location name
    pub location_name: String,
    /// Player's current HP
    pub player_hp: i32,
    /// Player's max HP
    pub player_max_hp: i32,
    /// Player's gold
    pub player_gold: u32,
    /// Turn count
    pub turn_count: u32,
    /// Whether the game is over
    pub game_over: bool,
    /// Whether the player won
    pub victory: bool,
    /// Number of kills
    pub kills: u32,
    /// Save slot type
    pub slot_type: SaveSlotType,
    /// Optional screenshot data (base64 encoded)
    pub screenshot: Option<String>,
    /// Checksum of the save data
    pub checksum: u32,
}

impl SaveMetadata {
    /// Create metadata from game state
    pub fn from_game_state(state: &GameState, slot_type: SaveSlotType, play_time: u64) -> Self {
        let location = crate::world::DungeonTheme::from_level(state.dungeon_level);

        Self {
            version: SAVE_VERSION,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            character_class: state.player.class,
            player_level: state.player.level,
            dungeon_level: state.dungeon_level,
            play_time_seconds: play_time,
            location_name: format!("{} - Level {}", location.name(), state.dungeon_level),
            player_hp: state.player.hp,
            player_max_hp: state.player.total_max_hp(),
            player_gold: state.player.gold,
            turn_count: state.turn_count,
            game_over: state.game_over,
            victory: state.victory,
            kills: state.player.kills,
            slot_type,
            screenshot: None,
            checksum: 0, // Will be calculated during save
        }
    }

    /// Format play time as HH:MM:SS
    pub fn formatted_play_time(&self) -> String {
        let hours = self.play_time_seconds / 3600;
        let minutes = (self.play_time_seconds % 3600) / 60;
        let seconds = self.play_time_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    /// Format timestamp as readable date
    pub fn formatted_date(&self) -> String {
        // Simple date formatting without external crate
        let days_since_epoch = self.timestamp / 86400;
        let seconds_today = self.timestamp % 86400;
        let hours = seconds_today / 3600;
        let minutes = (seconds_today % 3600) / 60;

        // Very simple date calculation (approximate)
        let year = 1970 + (days_since_epoch / 365);
        let day_of_year = days_since_epoch % 365;
        let month = (day_of_year / 30) + 1;
        let day = (day_of_year % 30) + 1;

        format!("{:04}-{:02}-{:02} {:02}:{:02}", year, month, day, hours, minutes)
    }
}

// ============================================================================
// Complete Save Data
// ============================================================================

/// Complete save data structure
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    /// Save metadata
    pub metadata: SaveMetadata,
    /// Complete game state
    pub game_state: GameState,
    /// Player settings
    pub settings: GameSettings,
    /// World state (explored areas, etc.)
    pub world_state: WorldState,
    /// New Game Plus data (for NG+ progression)
    pub ng_plus_data: Option<NewGamePlusData>,
}

/// Game settings that persist across sessions
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameSettings {
    /// Auto-save enabled
    pub auto_save_enabled: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Sound volume (0-100)
    pub sound_volume: u8,
    /// Music volume (0-100)
    pub music_volume: u8,
    /// Show damage numbers
    pub show_damage_numbers: bool,
    /// Show minimap
    pub show_minimap: bool,
    /// Screen shake enabled
    pub screen_shake: bool,
    /// Auto-pickup items
    pub auto_pickup: bool,
    /// Confirm before dangerous actions
    pub confirm_dangerous: bool,
    /// Key bindings
    pub key_bindings: HashMap<String, String>,
    /// Graphics quality level
    pub graphics_quality: u8,
    /// Colorblind mode
    pub colorblind_mode: Option<String>,
}

impl GameSettings {
    pub fn new() -> Self {
        Self {
            auto_save_enabled: true,
            auto_save_interval: DEFAULT_AUTOSAVE_INTERVAL,
            sound_volume: 80,
            music_volume: 60,
            show_damage_numbers: true,
            show_minimap: true,
            screen_shake: true,
            auto_pickup: false,
            confirm_dangerous: true,
            key_bindings: HashMap::new(),
            graphics_quality: 2, // Medium
            colorblind_mode: None,
        }
    }
}

/// World state tracking
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorldState {
    /// Explored dungeon levels
    pub explored_levels: Vec<u32>,
    /// Fully cleared levels (all enemies killed)
    pub cleared_levels: Vec<u32>,
    /// Killed unique enemies (by ID)
    pub killed_unique_enemies: Vec<u64>,
    /// Discovered secrets
    pub discovered_secrets: Vec<String>,
    /// Unlocked shortcuts
    pub unlocked_shortcuts: Vec<(u32, u32)>, // (from_level, to_level)
    /// NPC relationship states
    pub npc_relationships: HashMap<String, i32>,
    /// Active world events
    pub active_events: Vec<String>,
    /// Completed world events
    pub completed_events: Vec<String>,
}

/// New Game Plus data for carrying progress
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewGamePlusData {
    /// NG+ cycle number (1 = first NG+, 2 = NG++, etc.)
    pub cycle: u32,
    /// Carried over gold percentage
    pub carried_gold: u32,
    /// Carried over items
    pub carried_items: Vec<Item>,
    /// Permanent stat bonuses from achievements
    pub stat_bonuses: HashMap<String, i32>,
    /// Unlocked abilities
    pub unlocked_abilities: Vec<String>,
    /// Enemy difficulty multiplier
    pub difficulty_multiplier: f32,
    /// Loot quality bonus
    pub loot_bonus: f32,
    /// Previous run's final stats
    pub previous_run_stats: PreviousRunStats,
}

/// Stats from the previous run (for NG+)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PreviousRunStats {
    pub final_level: u32,
    pub total_kills: u32,
    pub bosses_defeated: Vec<String>,
    pub play_time: u64,
    pub gold_collected: u32,
}

// ============================================================================
// Pause System
// ============================================================================

/// Pause state for the game
#[derive(Clone, Debug, Default)]
pub struct PauseState {
    /// Whether the game is paused
    pub is_paused: bool,
    /// Time when pause started
    pub pause_start: Option<Instant>,
    /// Total time spent paused this session
    pub total_pause_time: Duration,
    /// Current pause menu selection
    pub menu_selection: usize,
    /// Available pause menu options
    pub menu_options: Vec<PauseMenuOption>,
}

/// Options available in the pause menu
#[derive(Clone, Debug, PartialEq)]
pub enum PauseMenuOption {
    Resume,
    QuickSave,
    ManualSave,
    LoadGame,
    Settings,
    Help,
    MainMenu,
    QuitGame,
}

impl PauseMenuOption {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Resume => "Resume Game",
            Self::QuickSave => "Quick Save",
            Self::ManualSave => "Save Game",
            Self::LoadGame => "Load Game",
            Self::Settings => "Settings",
            Self::Help => "Help",
            Self::MainMenu => "Main Menu",
            Self::QuitGame => "Quit Game",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Resume,
            Self::QuickSave,
            Self::ManualSave,
            Self::LoadGame,
            Self::Settings,
            Self::Help,
            Self::MainMenu,
            Self::QuitGame,
        ]
    }
}

impl PauseState {
    pub fn new() -> Self {
        Self {
            is_paused: false,
            pause_start: None,
            total_pause_time: Duration::ZERO,
            menu_selection: 0,
            menu_options: PauseMenuOption::all(),
        }
    }

    /// Toggle pause state
    pub fn toggle(&mut self) {
        if self.is_paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    /// Pause the game
    pub fn pause(&mut self) {
        if !self.is_paused {
            self.is_paused = true;
            self.pause_start = Some(Instant::now());
            self.menu_selection = 0;
        }
    }

    /// Resume the game
    pub fn resume(&mut self) {
        if self.is_paused {
            if let Some(start) = self.pause_start.take() {
                self.total_pause_time += start.elapsed();
            }
            self.is_paused = false;
        }
    }

    /// Navigate up in the menu
    pub fn menu_up(&mut self) {
        if self.menu_selection > 0 {
            self.menu_selection -= 1;
        } else {
            self.menu_selection = self.menu_options.len() - 1;
        }
    }

    /// Navigate down in the menu
    pub fn menu_down(&mut self) {
        if self.menu_selection < self.menu_options.len() - 1 {
            self.menu_selection += 1;
        } else {
            self.menu_selection = 0;
        }
    }

    /// Get the currently selected option
    pub fn selected_option(&self) -> Option<&PauseMenuOption> {
        self.menu_options.get(self.menu_selection)
    }
}

// ============================================================================
// Save Manager
// ============================================================================

/// Main save manager handling all save/load operations
#[derive(Clone)]
pub struct SaveManager {
    /// Base directory for save files
    save_dir: PathBuf,
    /// Current settings
    pub settings: GameSettings,
    /// Cached metadata for all slots
    slot_metadata: HashMap<SaveSlotType, SaveMetadata>,
    /// Current session play time tracking
    session_start: Option<Instant>,
    /// Accumulated play time from loaded save
    accumulated_play_time: u64,
    /// Last auto-save time
    last_autosave: Option<Instant>,
    /// Pause state
    pub pause_state: PauseState,
    /// World state
    pub world_state: WorldState,
    /// NG+ data
    pub ng_plus_data: Option<NewGamePlusData>,
    /// Cloud sync enabled
    pub cloud_sync_enabled: bool,
    /// Cloud sync status
    pub cloud_sync_status: CloudSyncStatus,
}

/// Cloud sync status
#[derive(Clone, Debug, Default)]
pub struct CloudSyncStatus {
    pub is_syncing: bool,
    pub last_sync: Option<u64>,
    pub sync_error: Option<String>,
    pub pending_upload: bool,
    pub pending_download: bool,
}

impl SaveManager {
    /// Create a new save manager
    pub fn new() -> Self {
        let save_dir = Self::default_save_dir();

        // Ensure save directory exists
        let _ = fs::create_dir_all(&save_dir);

        let mut manager = Self {
            save_dir,
            settings: GameSettings::new(),
            slot_metadata: HashMap::new(),
            session_start: None,
            accumulated_play_time: 0,
            last_autosave: None,
            pause_state: PauseState::new(),
            world_state: WorldState::default(),
            ng_plus_data: None,
            cloud_sync_enabled: false,
            cloud_sync_status: CloudSyncStatus::default(),
        };

        // Load metadata for all slots
        manager.refresh_metadata();

        manager
    }

    /// Get the default save directory
    pub fn default_save_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".shadowcrypt").join("saves")
    }

    /// Start a new game session (for play time tracking)
    pub fn start_session(&mut self) {
        self.session_start = Some(Instant::now());
        self.last_autosave = Some(Instant::now());
    }

    /// Get total play time in seconds
    pub fn get_play_time(&self) -> u64 {
        let session_time = self.session_start
            .map(|start| {
                let elapsed = start.elapsed();
                let pause_time = self.pause_state.total_pause_time;
                if elapsed > pause_time {
                    (elapsed - pause_time).as_secs()
                } else {
                    0
                }
            })
            .unwrap_or(0);

        self.accumulated_play_time + session_time
    }

    /// Refresh metadata cache for all save slots
    pub fn refresh_metadata(&mut self) {
        self.slot_metadata.clear();

        // Check all manual slots
        for slot in 0..MAX_SAVE_SLOTS {
            let slot_type = SaveSlotType::Manual(slot);
            if let Ok(metadata) = self.load_metadata(slot_type) {
                self.slot_metadata.insert(slot_type, metadata);
            }
        }

        // Check special slots
        for slot_type in [SaveSlotType::AutoSave, SaveSlotType::QuickSave, SaveSlotType::CloudSave] {
            if let Ok(metadata) = self.load_metadata(slot_type) {
                self.slot_metadata.insert(slot_type, metadata);
            }
        }
    }

    /// Get path for a save slot
    pub fn slot_path(&self, slot_type: SaveSlotType) -> PathBuf {
        self.save_dir.join(slot_type.filename())
    }

    /// Get backup path for a save slot
    fn backup_path(&self, slot_type: SaveSlotType, index: usize) -> PathBuf {
        let filename = slot_type.filename();
        let backup_name = format!("{}.bak{}", filename, index);
        self.save_dir.join("backups").join(backup_name)
    }

    /// Calculate CRC32 checksum
    fn calculate_checksum(data: &[u8]) -> u32 {
        let mut crc: u32 = 0xFFFFFFFF;
        for byte in data {
            crc ^= *byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        !crc
    }

    /// Create a backup of the save file
    fn create_backup(&self, slot_type: SaveSlotType) -> Result<(), SaveError> {
        let source = self.slot_path(slot_type);
        if !source.exists() {
            return Ok(());
        }

        let backup_dir = self.save_dir.join("backups");
        fs::create_dir_all(&backup_dir)?;

        // Rotate existing backups
        for i in (1..MAX_BACKUPS).rev() {
            let old_backup = self.backup_path(slot_type, i - 1);
            let new_backup = self.backup_path(slot_type, i);
            if old_backup.exists() {
                let _ = fs::rename(&old_backup, &new_backup);
            }
        }

        // Create new backup
        let backup = self.backup_path(slot_type, 0);
        fs::copy(&source, &backup)?;

        Ok(())
    }

    /// Save game to a specific slot
    pub fn save_game(&mut self, state: &GameState, slot_type: SaveSlotType) -> Result<(), SaveError> {
        // Create backup of existing save
        self.create_backup(slot_type)?;

        // Prepare save data
        let play_time = self.get_play_time();
        let mut metadata = SaveMetadata::from_game_state(state, slot_type, play_time);

        let save_data = SaveData {
            metadata: metadata.clone(),
            game_state: state.clone(),
            settings: self.settings.clone(),
            world_state: self.world_state.clone(),
            ng_plus_data: self.ng_plus_data.clone(),
        };

        // Serialize to bincode
        let serialized = bincode::serialize(&save_data)
            .map_err(|e| SaveError::SerializeError(e.to_string()))?;

        // Calculate checksum
        let checksum = Self::calculate_checksum(&serialized);
        metadata.checksum = checksum;

        // Compress with gzip
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&serialized)?;
        let compressed = encoder.finish()?;

        // Build final save file with header
        let mut final_data = Vec::new();
        final_data.extend_from_slice(&SAVE_MAGIC);
        final_data.extend_from_slice(&SAVE_VERSION.to_le_bytes());
        final_data.extend_from_slice(&checksum.to_le_bytes());
        final_data.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        final_data.extend_from_slice(&compressed);

        // Write to file
        let path = self.slot_path(slot_type);
        let mut file = BufWriter::new(File::create(&path)?);
        file.write_all(&final_data)?;
        file.flush()?;

        // Update metadata cache
        self.slot_metadata.insert(slot_type, metadata);

        // Trigger cloud sync if enabled
        if self.cloud_sync_enabled {
            self.cloud_sync_status.pending_upload = true;
        }

        Ok(())
    }

    /// Quick save (single key shortcut)
    pub fn quick_save(&mut self, state: &GameState) -> Result<(), SaveError> {
        self.save_game(state, SaveSlotType::QuickSave)
    }

    /// Auto save (called periodically)
    pub fn auto_save(&mut self, state: &GameState) -> Result<(), SaveError> {
        if !self.settings.auto_save_enabled {
            return Ok(());
        }
        self.save_game(state, SaveSlotType::AutoSave)
    }

    /// Check if auto-save is due
    pub fn should_auto_save(&self) -> bool {
        if !self.settings.auto_save_enabled {
            return false;
        }

        if let Some(last) = self.last_autosave {
            last.elapsed().as_secs() >= self.settings.auto_save_interval
        } else {
            true
        }
    }

    /// Perform auto-save if needed
    pub fn tick_autosave(&mut self, state: &GameState) -> Result<bool, SaveError> {
        if self.should_auto_save() && !self.pause_state.is_paused {
            self.auto_save(state)?;
            self.last_autosave = Some(Instant::now());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Load metadata only (for save slot preview)
    pub fn load_metadata(&self, slot_type: SaveSlotType) -> Result<SaveMetadata, SaveError> {
        let path = self.slot_path(slot_type);
        if !path.exists() {
            return Err(SaveError::SaveNotFound(path.display().to_string()));
        }

        let mut file = BufReader::new(File::open(&path)?);
        let mut header = [0u8; 16]; // magic(4) + version(4) + checksum(4) + size(4)
        file.read_exact(&mut header)?;

        // Validate magic
        if &header[0..4] != &SAVE_MAGIC {
            return Err(SaveError::InvalidMagic);
        }

        // Read version
        let version = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        if version != SAVE_VERSION {
            return Err(SaveError::VersionMismatch { expected: SAVE_VERSION, actual: version });
        }

        // Read checksum and size
        let stored_checksum = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
        let compressed_size = u32::from_le_bytes([header[12], header[13], header[14], header[15]]) as usize;

        // Read compressed data
        let mut compressed = vec![0u8; compressed_size];
        file.read_exact(&mut compressed)?;

        // Decompress
        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        // Verify checksum
        let actual_checksum = Self::calculate_checksum(&decompressed);
        if actual_checksum != stored_checksum {
            return Err(SaveError::ChecksumMismatch {
                expected: stored_checksum,
                actual: actual_checksum
            });
        }

        // Deserialize just enough to get metadata
        let save_data: SaveData = bincode::deserialize(&decompressed)
            .map_err(|e| SaveError::DeserializeError(e.to_string()))?;

        Ok(save_data.metadata)
    }

    /// Load game from a specific slot
    pub fn load_game(&mut self, slot_type: SaveSlotType) -> Result<GameState, SaveError> {
        let path = self.slot_path(slot_type);
        if !path.exists() {
            return Err(SaveError::SaveNotFound(path.display().to_string()));
        }

        let mut file = BufReader::new(File::open(&path)?);
        let mut header = [0u8; 16];
        file.read_exact(&mut header)?;

        // Validate magic
        if &header[0..4] != &SAVE_MAGIC {
            return self.try_recover(slot_type);
        }

        // Read version
        let version = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        if version != SAVE_VERSION {
            return Err(SaveError::VersionMismatch { expected: SAVE_VERSION, actual: version });
        }

        // Read checksum and size
        let stored_checksum = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
        let compressed_size = u32::from_le_bytes([header[12], header[13], header[14], header[15]]) as usize;

        // Read compressed data
        let mut compressed = vec![0u8; compressed_size];
        file.read_exact(&mut compressed)?;

        // Decompress
        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        // Verify checksum
        let actual_checksum = Self::calculate_checksum(&decompressed);
        if actual_checksum != stored_checksum {
            return self.try_recover(slot_type);
        }

        // Deserialize
        let save_data: SaveData = bincode::deserialize(&decompressed)
            .map_err(|e| SaveError::DeserializeError(e.to_string()))?;

        // Restore state
        self.settings = save_data.settings;
        self.world_state = save_data.world_state;
        self.ng_plus_data = save_data.ng_plus_data;
        self.accumulated_play_time = save_data.metadata.play_time_seconds;
        self.start_session();

        Ok(save_data.game_state)
    }

    /// Load the most recent save (any slot)
    pub fn load_last_save(&mut self) -> Result<GameState, SaveError> {
        // Find the most recent save
        let mut most_recent: Option<(SaveSlotType, u64)> = None;

        for (slot_type, metadata) in &self.slot_metadata {
            match most_recent {
                None => most_recent = Some((*slot_type, metadata.timestamp)),
                Some((_, ts)) if metadata.timestamp > ts => {
                    most_recent = Some((*slot_type, metadata.timestamp));
                }
                _ => {}
            }
        }

        if let Some((slot_type, _)) = most_recent {
            self.load_game(slot_type)
        } else {
            Err(SaveError::SaveNotFound("No saves found".to_string()))
        }
    }

    /// Try to recover from backup if main save is corrupted
    fn try_recover(&mut self, slot_type: SaveSlotType) -> Result<GameState, SaveError> {
        // Try each backup in order
        for i in 0..MAX_BACKUPS {
            let backup_path = self.backup_path(slot_type, i);
            if backup_path.exists() {
                // Temporarily swap paths and try loading
                let original_path = self.slot_path(slot_type);
                if fs::copy(&backup_path, &original_path).is_ok() {
                    if let Ok(state) = self.load_game(slot_type) {
                        return Ok(state);
                    }
                }
            }
        }

        Err(SaveError::CorruptedSave(
            "All recovery attempts failed".to_string()
        ))
    }

    /// Get all available save slot metadata
    pub fn get_all_slots(&self) -> Vec<(SaveSlotType, Option<&SaveMetadata>)> {
        let mut slots = Vec::new();

        // Manual slots
        for slot in 0..MAX_SAVE_SLOTS {
            let slot_type = SaveSlotType::Manual(slot);
            slots.push((slot_type, self.slot_metadata.get(&slot_type)));
        }

        // Special slots
        slots.push((SaveSlotType::AutoSave, self.slot_metadata.get(&SaveSlotType::AutoSave)));
        slots.push((SaveSlotType::QuickSave, self.slot_metadata.get(&SaveSlotType::QuickSave)));

        slots
    }

    /// Delete a save file
    pub fn delete_save(&mut self, slot_type: SaveSlotType) -> Result<(), SaveError> {
        let path = self.slot_path(slot_type);
        if path.exists() {
            fs::remove_file(path)?;
            self.slot_metadata.remove(&slot_type);
        }
        Ok(())
    }

    /// Check if a save slot exists
    pub fn slot_exists(&self, slot_type: SaveSlotType) -> bool {
        self.slot_metadata.contains_key(&slot_type)
    }

    /// Export save to a portable format
    pub fn export_save(&self, slot_type: SaveSlotType, export_path: &Path) -> Result<(), SaveError> {
        let source = self.slot_path(slot_type);
        if !source.exists() {
            return Err(SaveError::SaveNotFound(source.display().to_string()));
        }

        // Read the save file
        let data = fs::read(&source)?;

        // Encode as base64 for portability
        let encoded = base64_encode(&data);

        // Write to export file with header
        let mut export = String::new();
        export.push_str("SHADOWCRYPT_SAVE_EXPORT_V1\n");
        export.push_str(&slot_type.filename());
        export.push('\n');
        export.push_str(&encoded);

        fs::write(export_path, export)?;

        Ok(())
    }

    /// Import save from exported format
    pub fn import_save(&mut self, import_path: &Path, slot_type: SaveSlotType) -> Result<(), SaveError> {
        let content = fs::read_to_string(import_path)
            .map_err(|e| SaveError::ImportExportError(e.to_string()))?;

        let lines: Vec<&str> = content.lines().collect();
        if lines.len() < 3 {
            return Err(SaveError::ImportExportError("Invalid export format".to_string()));
        }

        if lines[0] != "SHADOWCRYPT_SAVE_EXPORT_V1" {
            return Err(SaveError::ImportExportError("Unknown export version".to_string()));
        }

        // Decode the save data
        let encoded = lines[2..].join("");
        let decoded = base64_decode(&encoded)
            .map_err(|e| SaveError::ImportExportError(e))?;

        // Write to destination slot
        let dest_path = self.slot_path(slot_type);
        fs::write(&dest_path, decoded)?;

        // Refresh metadata
        self.refresh_metadata();

        Ok(())
    }

    /// Start New Game Plus
    pub fn start_new_game_plus(&mut self, completed_state: &GameState) -> NewGamePlusData {
        let cycle = self.ng_plus_data.as_ref().map(|d| d.cycle + 1).unwrap_or(1);

        // Calculate carried over gold (10% per cycle, max 50%)
        let gold_percent = (cycle * 10).min(50);
        let carried_gold = (completed_state.player.gold * gold_percent) / 100;

        // Carry over best items (legendary+ only)
        let carried_items: Vec<Item> = completed_state.player.inventory
            .iter()
            .filter(|item| item.rarity >= Rarity::Legendary)
            .take(3) // Max 3 items
            .cloned()
            .collect();

        // Get achievement bonuses
        let stat_bonuses = completed_state.achievement_tracker.get_stat_bonuses()
            .into_iter()
            .map(|(stat, val)| (format!("{:?}", stat), val))
            .collect();

        let ng_data = NewGamePlusData {
            cycle,
            carried_gold,
            carried_items,
            stat_bonuses,
            unlocked_abilities: Vec::new(),
            difficulty_multiplier: 1.0 + (cycle as f32 * 0.2), // 20% harder per cycle
            loot_bonus: 1.0 + (cycle as f32 * 0.1), // 10% better loot per cycle
            previous_run_stats: PreviousRunStats {
                final_level: completed_state.player.level,
                total_kills: completed_state.player.kills,
                bosses_defeated: Vec::new(), // Would need to track this
                play_time: self.get_play_time(),
                gold_collected: completed_state.player.gold,
            },
        };

        self.ng_plus_data = Some(ng_data.clone());
        ng_data
    }

    // ========================================================================
    // Cloud Save Stubs (for future implementation)
    // ========================================================================

    /// Enable cloud sync (stub)
    pub fn enable_cloud_sync(&mut self, _api_key: &str) -> Result<(), SaveError> {
        // Stub for future cloud implementation
        self.cloud_sync_enabled = true;
        Ok(())
    }

    /// Disable cloud sync
    pub fn disable_cloud_sync(&mut self) {
        self.cloud_sync_enabled = false;
    }

    /// Sync with cloud (stub)
    pub fn sync_cloud(&mut self) -> Result<(), SaveError> {
        if !self.cloud_sync_enabled {
            return Ok(());
        }

        // Stub: In a real implementation, this would:
        // 1. Check for newer cloud saves
        // 2. Upload local saves if newer
        // 3. Download cloud saves if newer
        // 4. Handle conflicts

        self.cloud_sync_status.is_syncing = false;
        self.cloud_sync_status.last_sync = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );

        Err(SaveError::CloudSyncError("Cloud sync not yet implemented".to_string()))
    }

    /// Upload save to cloud (stub)
    pub fn upload_to_cloud(&mut self, _slot_type: SaveSlotType) -> Result<(), SaveError> {
        Err(SaveError::CloudSyncError("Cloud upload not yet implemented".to_string()))
    }

    /// Download save from cloud (stub)
    pub fn download_from_cloud(&mut self) -> Result<(), SaveError> {
        Err(SaveError::CloudSyncError("Cloud download not yet implemented".to_string()))
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Functions (base64)
// ============================================================================

/// Simple base64 encoding
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = if i + 1 < data.len() { data[i + 1] as usize } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as usize } else { 0 };

        result.push(ALPHABET[(b0 >> 2) & 0x3F] as char);
        result.push(ALPHABET[((b0 << 4) | (b1 >> 4)) & 0x3F] as char);

        if i + 1 < data.len() {
            result.push(ALPHABET[((b1 << 2) | (b2 >> 6)) & 0x3F] as char);
        } else {
            result.push('=');
        }

        if i + 2 < data.len() {
            result.push(ALPHABET[b2 & 0x3F] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

/// Simple base64 decoding
fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    const DECODE: [i8; 128] = [
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,62,-1,-1,-1,63,
        52,53,54,55,56,57,58,59,60,61,-1,-1,-1,-1,-1,-1,
        -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,
        15,16,17,18,19,20,21,22,23,24,25,-1,-1,-1,-1,-1,
        -1,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,
        41,42,43,44,45,46,47,48,49,50,51,-1,-1,-1,-1,-1,
    ];

    let data: String = data.chars().filter(|c| !c.is_whitespace()).collect();
    if data.len() % 4 != 0 {
        return Err("Invalid base64 length".to_string());
    }

    let mut result = Vec::new();
    let bytes = data.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let b0 = DECODE.get(bytes[i] as usize).copied().unwrap_or(-1);
        let b1 = DECODE.get(bytes[i + 1] as usize).copied().unwrap_or(-1);
        let b2 = if bytes[i + 2] == b'=' { 0 } else { DECODE.get(bytes[i + 2] as usize).copied().unwrap_or(-1) };
        let b3 = if bytes[i + 3] == b'=' { 0 } else { DECODE.get(bytes[i + 3] as usize).copied().unwrap_or(-1) };

        if b0 < 0 || b1 < 0 || (bytes[i + 2] != b'=' && b2 < 0) || (bytes[i + 3] != b'=' && b3 < 0) {
            return Err("Invalid base64 character".to_string());
        }

        result.push(((b0 << 2) | (b1 >> 4)) as u8);
        if bytes[i + 2] != b'=' {
            result.push((((b1 & 0x0F) << 4) | (b2 >> 2)) as u8);
        }
        if bytes[i + 3] != b'=' {
            result.push((((b2 & 0x03) << 6) | b3) as u8);
        }

        i += 4;
    }

    Ok(result)
}

// ============================================================================
// Legacy Compatibility Functions
// ============================================================================

/// Get the default save file path (legacy compatibility)
pub fn default_save_path() -> PathBuf {
    SaveManager::default_save_dir().join("quicksave.sav")
}

/// Save the game state to a file (legacy compatibility)
pub fn save_game(state: &GameState, path: Option<PathBuf>) -> Result<(), SaveError> {
    let mut manager = SaveManager::new();

    if let Some(p) = path {
        // Custom path - write directly
        let serialized = bincode::serialize(state)
            .map_err(|e| SaveError::SerializeError(e.to_string()))?;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&serialized)?;
        let compressed = encoder.finish()?;

        fs::write(p, compressed)?;
        Ok(())
    } else {
        manager.quick_save(state)
    }
}

/// Load the game state from a file (legacy compatibility)
pub fn load_game(path: Option<PathBuf>) -> Result<GameState, SaveError> {
    if let Some(p) = path {
        // Custom path - read directly
        let compressed = fs::read(p)?;

        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        let state: GameState = bincode::deserialize(&decompressed)
            .map_err(|e| SaveError::DeserializeError(e.to_string()))?;

        Ok(state)
    } else {
        let mut manager = SaveManager::new();
        manager.load_game(SaveSlotType::QuickSave)
    }
}

/// Check if a save file exists (legacy compatibility)
pub fn save_exists(path: Option<PathBuf>) -> bool {
    let path = path.unwrap_or_else(default_save_path);
    path.exists()
}

/// Delete a save file (legacy compatibility)
pub fn delete_save(path: Option<PathBuf>) -> Result<(), SaveError> {
    let path = path.unwrap_or_else(default_save_path);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::CharacterClass;
    use std::env::temp_dir;

    fn create_test_manager() -> SaveManager {
        let mut manager = SaveManager::new();
        manager.save_dir = temp_dir().join("shadowcrypt_test_saves");
        let _ = fs::create_dir_all(&manager.save_dir);
        manager
    }

    #[test]
    fn test_checksum() {
        let data = b"Hello, World!";
        let checksum = SaveManager::calculate_checksum(data);
        assert_ne!(checksum, 0);

        // Same data should produce same checksum
        let checksum2 = SaveManager::calculate_checksum(data);
        assert_eq!(checksum, checksum2);

        // Different data should produce different checksum
        let checksum3 = SaveManager::calculate_checksum(b"Different data");
        assert_ne!(checksum, checksum3);
    }

    #[test]
    fn test_base64_roundtrip() {
        let original = b"Test data for base64 encoding!";
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(original.to_vec(), decoded);
    }

    #[test]
    fn test_save_metadata() {
        let game = GameState::new(CharacterClass::Warrior);
        let metadata = SaveMetadata::from_game_state(&game, SaveSlotType::Manual(0), 3600);

        assert_eq!(metadata.player_level, 1);
        assert_eq!(metadata.dungeon_level, 1);
        assert_eq!(metadata.play_time_seconds, 3600);
        assert_eq!(metadata.formatted_play_time(), "01:00:00");
    }

    #[test]
    fn test_save_load_roundtrip() {
        let mut manager = create_test_manager();
        manager.start_session();

        let game = GameState::new(CharacterClass::Mage);
        let slot = SaveSlotType::Manual(0);

        // Save
        manager.save_game(&game, slot).expect("Failed to save");
        assert!(manager.slot_exists(slot));

        // Load
        let loaded = manager.load_game(slot).expect("Failed to load");
        assert_eq!(loaded.dungeon_level, game.dungeon_level);
        assert_eq!(loaded.player.class, game.player.class);

        // Cleanup
        let _ = manager.delete_save(slot);
    }

    #[test]
    fn test_pause_state() {
        let mut pause = PauseState::new();

        assert!(!pause.is_paused);

        pause.pause();
        assert!(pause.is_paused);

        pause.menu_down();
        assert_eq!(pause.menu_selection, 1);

        pause.menu_up();
        assert_eq!(pause.menu_selection, 0);

        pause.resume();
        assert!(!pause.is_paused);
    }

    #[test]
    fn test_slot_types() {
        assert_eq!(SaveSlotType::Manual(0).filename(), "save_00.sav");
        assert_eq!(SaveSlotType::Manual(9).filename(), "save_09.sav");
        assert_eq!(SaveSlotType::AutoSave.filename(), "autosave.sav");
        assert_eq!(SaveSlotType::QuickSave.filename(), "quicksave.sav");
    }

    #[test]
    fn test_settings_defaults() {
        let settings = GameSettings::new();

        assert!(settings.auto_save_enabled);
        assert_eq!(settings.auto_save_interval, DEFAULT_AUTOSAVE_INTERVAL);
        assert_eq!(settings.sound_volume, 80);
        assert!(settings.show_damage_numbers);
    }
}

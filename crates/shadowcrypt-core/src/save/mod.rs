//! Save/load system for the ShadowCrypt roguelike
//!
//! This module handles saving and loading game state to/from disk,
//! using binary serialization with optional compression.

use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

/// Error type for save/load operations
#[derive(Debug)]
pub enum SaveError {
    /// IO error during file operations
    IoError(std::io::Error),
    /// Serialization/deserialization error
    SerializationError(String),
    /// Save file not found
    NotFound,
    /// Save file corrupted
    Corrupted,
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Self::NotFound => write!(f, "Save file not found"),
            Self::Corrupted => write!(f, "Save file corrupted"),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<std::io::Error> for SaveError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

/// Result type for save/load operations
pub type SaveResult<T> = Result<T, SaveError>;

/// Save file header for version checking
#[derive(Serialize, Deserialize, Debug)]
struct SaveHeader {
    /// Magic number to identify save files
    magic: u32,
    /// Save format version
    version: u32,
    /// Timestamp when saved
    timestamp: u64,
    /// Character name/class for display
    character_info: String,
    /// Dungeon level for display
    dungeon_level: u32,
}

impl SaveHeader {
    const MAGIC: u32 = 0x53484157; // "SHAW" in ASCII
    const VERSION: u32 = 1;

    fn new(character_info: String, dungeon_level: u32) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            timestamp,
            character_info,
            dungeon_level,
        }
    }

    fn validate(&self) -> SaveResult<()> {
        if self.magic != Self::MAGIC {
            return Err(SaveError::Corrupted);
        }
        if self.version > Self::VERSION {
            return Err(SaveError::SerializationError(
                format!("Save version {} is newer than supported version {}", self.version, Self::VERSION)
            ));
        }
        Ok(())
    }
}

/// Information about a save slot for the UI
#[derive(Debug, Clone)]
pub struct SaveSlotInfo {
    /// Slot number
    pub slot: u32,
    /// Whether this slot has a save
    pub exists: bool,
    /// Character info (class/name)
    pub character_info: String,
    /// Dungeon level
    pub dungeon_level: u32,
    /// When the save was created
    pub timestamp: u64,
    /// Path to the save file
    pub path: PathBuf,
}

/// Manager for save/load operations
pub struct SaveManager {
    /// Base directory for save files
    save_dir: PathBuf,
}

impl SaveManager {
    /// Creates a new save manager with the default save directory
    pub fn new() -> SaveResult<Self> {
        let save_dir = Self::default_save_dir()?;
        Ok(Self { save_dir })
    }

    /// Creates a new save manager with a custom save directory
    pub fn with_dir(save_dir: PathBuf) -> Self {
        Self { save_dir }
    }

    /// Returns the default save directory
    fn default_save_dir() -> SaveResult<PathBuf> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| SaveError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find home directory"
            )))?;

        let path = PathBuf::from(home).join(".shadowcrypt").join("saves");
        Ok(path)
    }

    /// Ensures the save directory exists
    fn ensure_dir(&self) -> SaveResult<()> {
        fs::create_dir_all(&self.save_dir)?;
        Ok(())
    }

    /// Returns the path for a save slot
    fn slot_path(&self, slot: u32) -> PathBuf {
        self.save_dir.join(format!("save_{}.dat", slot))
    }

    /// Saves game data to a slot
    pub fn save<T: Serialize>(&self, slot: u32, data: &T, character_info: &str, dungeon_level: u32) -> SaveResult<()> {
        self.ensure_dir()?;

        let header = SaveHeader::new(character_info.to_string(), dungeon_level);
        let path = self.slot_path(slot);

        // Serialize header
        let header_bytes = bincode::serialize(&header)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;

        // Serialize data
        let data_bytes = bincode::serialize(data)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;

        // Write compressed file
        let file = fs::File::create(&path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());

        // Write header length and header
        let header_len = header_bytes.len() as u32;
        encoder.write_all(&header_len.to_le_bytes())?;
        encoder.write_all(&header_bytes)?;

        // Write data
        encoder.write_all(&data_bytes)?;
        encoder.finish()?;

        Ok(())
    }

    /// Loads game data from a slot
    pub fn load<T: for<'de> Deserialize<'de>>(&self, slot: u32) -> SaveResult<T> {
        let path = self.slot_path(slot);

        if !path.exists() {
            return Err(SaveError::NotFound);
        }

        let file = fs::File::open(&path)?;
        let mut decoder = GzDecoder::new(file);

        // Read header length
        let mut len_bytes = [0u8; 4];
        decoder.read_exact(&mut len_bytes)?;
        let header_len = u32::from_le_bytes(len_bytes) as usize;

        // Read and validate header
        let mut header_bytes = vec![0u8; header_len];
        decoder.read_exact(&mut header_bytes)?;
        let header: SaveHeader = bincode::deserialize(&header_bytes)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;
        header.validate()?;

        // Read data
        let mut data_bytes = Vec::new();
        decoder.read_to_end(&mut data_bytes)?;

        let data: T = bincode::deserialize(&data_bytes)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;

        Ok(data)
    }

    /// Deletes a save slot
    pub fn delete(&self, slot: u32) -> SaveResult<()> {
        let path = self.slot_path(slot);
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    /// Lists all save slots with their info
    pub fn list_slots(&self, max_slots: u32) -> Vec<SaveSlotInfo> {
        (0..max_slots).map(|slot| {
            let path = self.slot_path(slot);
            if path.exists() {
                if let Ok(info) = self.read_slot_info(slot) {
                    return info;
                }
            }
            SaveSlotInfo {
                slot,
                exists: false,
                character_info: String::new(),
                dungeon_level: 0,
                timestamp: 0,
                path,
            }
        }).collect()
    }

    /// Reads just the header info for a slot
    fn read_slot_info(&self, slot: u32) -> SaveResult<SaveSlotInfo> {
        let path = self.slot_path(slot);

        if !path.exists() {
            return Err(SaveError::NotFound);
        }

        let file = fs::File::open(&path)?;
        let mut decoder = GzDecoder::new(file);

        // Read header length
        let mut len_bytes = [0u8; 4];
        decoder.read_exact(&mut len_bytes)?;
        let header_len = u32::from_le_bytes(len_bytes) as usize;

        // Read header
        let mut header_bytes = vec![0u8; header_len];
        decoder.read_exact(&mut header_bytes)?;
        let header: SaveHeader = bincode::deserialize(&header_bytes)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;

        Ok(SaveSlotInfo {
            slot,
            exists: true,
            character_info: header.character_info,
            dungeon_level: header.dungeon_level,
            timestamp: header.timestamp,
            path,
        })
    }

    /// Quick save to the default slot (slot 0)
    pub fn quick_save<T: Serialize>(&self, data: &T, character_info: &str, dungeon_level: u32) -> SaveResult<()> {
        self.save(0, data, character_info, dungeon_level)
    }

    /// Quick load from the default slot (slot 0)
    pub fn quick_load<T: for<'de> Deserialize<'de>>(&self) -> SaveResult<T> {
        self.load(0)
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new().expect("Failed to create save manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        value: i32,
        name: String,
    }

    #[test]
    fn test_save_and_load() {
        let save_dir = temp_dir().join("shadowcrypt_test");
        let manager = SaveManager::with_dir(save_dir.clone());

        let data = TestData {
            value: 42,
            name: "Test".to_string(),
        };

        // Save
        manager.save(0, &data, "Warrior", 5).unwrap();

        // Load
        let loaded: TestData = manager.load(0).unwrap();
        assert_eq!(data, loaded);

        // Cleanup
        let _ = fs::remove_dir_all(save_dir);
    }

    #[test]
    fn test_slot_info() {
        let save_dir = temp_dir().join("shadowcrypt_test_info");
        let manager = SaveManager::with_dir(save_dir.clone());

        let data = TestData {
            value: 123,
            name: "Hero".to_string(),
        };

        manager.save(1, &data, "Mage Level 10", 15).unwrap();

        let slots = manager.list_slots(3);
        assert!(!slots[0].exists);
        assert!(slots[1].exists);
        assert_eq!(slots[1].character_info, "Mage Level 10");
        assert_eq!(slots[1].dungeon_level, 15);

        // Cleanup
        let _ = fs::remove_dir_all(save_dir);
    }
}

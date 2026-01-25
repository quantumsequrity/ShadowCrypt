//! Save/load functionality for game state

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

use crate::game::GameState;

/// Error type for save/load operations
#[derive(Debug)]
pub enum SaveError {
    IoError(std::io::Error),
    SerializeError(String),
    DeserializeError(String),
}

impl From<std::io::Error> for SaveError {
    fn from(err: std::io::Error) -> Self {
        SaveError::IoError(err)
    }
}

/// Get the default save file path
pub fn default_save_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".shadowcrypt_save.dat")
}

/// Save the game state to a file
pub fn save_game(state: &GameState, path: Option<PathBuf>) -> Result<(), SaveError> {
    let path = path.unwrap_or_else(default_save_path);

    // Serialize with bincode
    let data = bincode::serialize(state)
        .map_err(|e| SaveError::SerializeError(e.to_string()))?;

    // Compress with gzip
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;

    // Write to file
    fs::write(path, compressed)?;

    Ok(())
}

/// Load the game state from a file
pub fn load_game(path: Option<PathBuf>) -> Result<GameState, SaveError> {
    let path = path.unwrap_or_else(default_save_path);

    // Read file
    let compressed = fs::read(path)?;

    // Decompress
    let mut decoder = GzDecoder::new(&compressed[..]);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    // Deserialize
    let state: GameState = bincode::deserialize(&data)
        .map_err(|e| SaveError::DeserializeError(e.to_string()))?;

    Ok(state)
}

/// Check if a save file exists
pub fn save_exists(path: Option<PathBuf>) -> bool {
    let path = path.unwrap_or_else(default_save_path);
    path.exists()
}

/// Delete a save file
pub fn delete_save(path: Option<PathBuf>) -> Result<(), SaveError> {
    let path = path.unwrap_or_else(default_save_path);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::CharacterClass;
    use std::env::temp_dir;

    #[test]
    fn test_save_load_roundtrip() {
        let game = GameState::new(CharacterClass::Warrior);
        let save_path = temp_dir().join("test_shadowcrypt_save.dat");

        // Save
        save_game(&game, Some(save_path.clone())).expect("Failed to save");

        // Load
        let loaded = load_game(Some(save_path.clone())).expect("Failed to load");

        // Verify basic data survived
        assert_eq!(loaded.dungeon_level, game.dungeon_level);
        assert_eq!(loaded.player.class, game.player.class);

        // Cleanup
        let _ = delete_save(Some(save_path));
    }
}

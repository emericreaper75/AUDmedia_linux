use crate::queue::RepeatMode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub music_folders: Vec<String>,
    pub volume: f64,
    pub last_played_track: Option<String>,
    pub last_playback_position: Option<u64>,
    pub shuffle: bool,
    pub repeat_mode: RepeatMode,
    pub window_width: i32,
    pub window_height: i32,
    pub window_maximized: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            music_folders: Vec::new(),
            volume: 1.0,
            last_played_track: None,
            last_playback_position: None,
            shuffle: false,
            repeat_mode: RepeatMode::Off,
            window_width: 420,
            window_height: 720,
            window_maximized: false,
        }
    }
}

impl AppConfig {
    /// Returns the path to the configuration file.
    fn config_path() -> PathBuf {
        gtk4::glib::user_config_dir()
            .join("audmedia_linux")
            .join("config.json")
    }

    /// Loads the configuration from disk, returning defaults on any failure.
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                } else {
                    eprintln!("Warning: Corrupted config at {:?}, using defaults.", path);
                }
            } else {
                eprintln!(
                    "Warning: Failed to read config at {:?}, using defaults.",
                    path
                );
            }
        }
        Self::default()
    }

    /// Saves the configuration to disk.
    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create config directory: {}", e);
                    return;
                }
            }
        }

        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = fs::write(&path, json) {
                    eprintln!("Failed to write config file: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to serialize config: {}", e),
        }
    }
}

/// Validates and resolves a saved playback state against the current library.
/// Returns Option<(Track, u64)> containing the verified track and a safe playback position in nanoseconds.
pub fn validate_restored_playback_state(
    config: &AppConfig,
    library: &crate::library::Library,
) -> Option<(crate::library::Track, u64)> {
    let saved_path_str = config.last_played_track.as_ref()?;
    if saved_path_str.trim().is_empty() {
        return None;
    }

    let target_path = PathBuf::from(saved_path_str);
    if !target_path.exists() || !target_path.is_file() {
        return None;
    }

    // Check if the track exists in the library
    let track = library
        .tracks
        .iter()
        .find(|t| t.path == target_path)?
        .clone();

    // Validate position against track duration
    let position = match config.last_playback_position {
        Some(pos) => {
            if let Some(dur) = track.metadata.duration {
                let dur_ns = dur.as_nanos() as u64;
                if pos > dur_ns {
                    0
                } else {
                    pos
                }
            } else {
                pos
            }
        }
        None => 0,
    };

    Some((track, position))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{Library, Track};
    use crate::metadata::TrackMetadata;
    use std::time::Duration;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.music_folders.len(), 0);
        assert_eq!(config.volume, 1.0);
        assert_eq!(config.window_width, 420);
        assert_eq!(config.window_height, 720);
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut config = AppConfig::default();
        config.music_folders.push("/tmp/music".to_string());
        config.volume = 0.5;

        let json = serde_json::to_string(&config).unwrap();
        let loaded: AppConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.music_folders, vec!["/tmp/music".to_string()]);
        assert_eq!(loaded.volume, 0.5);
    }

    #[test]
    fn test_restoration_no_saved_track() {
        let config = AppConfig::default();
        let library = Library::new();
        assert!(validate_restored_playback_state(&config, &library).is_none());
    }

    #[test]
    fn test_restoration_valid_track() {
        let temp_dir =
            std::env::temp_dir().join(format!("audmedia_rest_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let track_file = temp_dir.join("test_song.mp3");
        std::fs::write(&track_file, b"dummy content").unwrap();

        let mut library = Library::new();
        let track = Track {
            path: track_file.clone(),
            metadata: TrackMetadata {
                title: Some("Test Song".to_string()),
                artist: Some("Test Artist".to_string()),
                duration: Some(Duration::from_secs(180)),
                ..Default::default()
            },
        };
        library.tracks.push(track);

        let mut config = AppConfig::default();
        config.last_played_track = Some(track_file.to_string_lossy().to_string());
        config.last_playback_position = Some(10_000_000_000); // 10 seconds

        let restored = validate_restored_playback_state(&config, &library);
        assert!(restored.is_some());
        let (rest_track, rest_pos) = restored.unwrap();
        assert_eq!(rest_track.metadata.title.as_deref(), Some("Test Song"));
        assert_eq!(rest_pos, 10_000_000_000);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_restoration_missing_track_file() {
        let mut library = Library::new();
        let non_existent = PathBuf::from("/tmp/non_existent_track_123456789.mp3");
        let track = Track {
            path: non_existent.clone(),
            metadata: TrackMetadata::default(),
        };
        library.tracks.push(track);

        let mut config = AppConfig::default();
        config.last_played_track = Some(non_existent.to_string_lossy().to_string());

        assert!(validate_restored_playback_state(&config, &library).is_none());
    }

    #[test]
    fn test_restoration_track_not_in_library() {
        let temp_dir =
            std::env::temp_dir().join(format!("audmedia_rest_notlib_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let track_file = temp_dir.join("outside.mp3");
        std::fs::write(&track_file, b"dummy content").unwrap();

        let library = Library::new(); // Empty library
        let mut config = AppConfig::default();
        config.last_played_track = Some(track_file.to_string_lossy().to_string());

        assert!(validate_restored_playback_state(&config, &library).is_none());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_restoration_position_greater_than_duration() {
        let temp_dir =
            std::env::temp_dir().join(format!("audmedia_rest_posdur_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let track_file = temp_dir.join("short_song.mp3");
        std::fs::write(&track_file, b"dummy content").unwrap();

        let mut library = Library::new();
        let track = Track {
            path: track_file.clone(),
            metadata: TrackMetadata {
                title: Some("Short Song".to_string()),
                duration: Some(Duration::from_secs(10)), // 10s duration = 10_000_000_000 ns
                ..Default::default()
            },
        };
        library.tracks.push(track);

        let mut config = AppConfig::default();
        config.last_played_track = Some(track_file.to_string_lossy().to_string());
        config.last_playback_position = Some(999_000_000_000); // 999s position > 10s duration

        let restored = validate_restored_playback_state(&config, &library);
        assert!(restored.is_some());
        let (_rest_track, rest_pos) = restored.unwrap();
        assert_eq!(rest_pos, 0); // Safely clamped to 0

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_restoration_corrupt_config_deserialization() {
        let invalid_json = "{ invalid json content }";
        let config: AppConfig = serde_json::from_str(invalid_json).unwrap_or_default();
        let library = Library::new();

        assert_eq!(config.last_played_track, None);
        assert_eq!(config.last_playback_position, None);
        assert!(validate_restored_playback_state(&config, &library).is_none());
    }
}

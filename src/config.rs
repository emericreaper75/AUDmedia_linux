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

#[cfg(test)]
mod tests {
    use super::*;

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
}

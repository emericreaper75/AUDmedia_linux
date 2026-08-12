//! Main application state manager.

#![allow(dead_code)]

use crate::library::Library;
use crate::player::Player;
use crate::queue::Queue;

/// Application state linking UI, Library, Player, and Queue.
#[derive(Debug)]
pub struct AppState {
    pub library: Library,
    pub player: Player,
    pub queue: Queue,
}

impl AppState {
    /// Creates a new `AppState`.
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            library: Library,
            player: Player::new().map_err(|e| e.to_string())?,
            queue: Queue,
        })
    }
}

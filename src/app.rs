//! Main application state manager.

use crate::library::Library;
use crate::player::Player;
use crate::queue::Queue;

/// Application state linking UI, Library, Player, and Queue.
#[derive(Debug, Default)]
pub struct AppState {
    pub library: Library,
    pub player: Player,
    pub queue: Queue,
}

impl AppState {
    /// Creates a new `AppState`.
    pub fn new() -> Self {
        Self::default()
    }
}

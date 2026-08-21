//! Main application state manager.

#![allow(dead_code)]

use crate::library::Library;
use crate::player::Player;
use crate::queue::Queue;
use crate::config::AppConfig;

/// Application state linking UI, Library, Player, and Queue.
#[derive(Debug)]
pub struct AppState {
    pub library: Library,
    pub player: Player,
    pub queue: Queue,
    pub config: AppConfig,
    pub mpris: Option<std::rc::Rc<mpris_server::Server<crate::mpris::MprisPlayer>>>,
}

impl AppState {
    /// Creates a new `AppState`.
    pub fn new() -> Result<Self, String> {
        let config = AppConfig::load();
        
        let player = Player::new().map_err(|e| e.to_string())?;
        player.set_volume(config.volume);
        
        let mut queue = Queue::new();
        queue.set_repeat_mode(config.repeat_mode);
        queue.set_shuffle(config.shuffle);

        Ok(Self {
            library: Library::new(),
            player,
            queue,
            config,
            mpris: None,
        })
    }
}

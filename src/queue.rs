//! Playback queue management.

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::library::Track;

/// Repeat mode for playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RepeatMode {
    #[default]
    Off,
    One,
    Queue,
}

/// Track queue structure for managing playback order.
#[derive(Debug, Default)]
pub struct Queue {
    tracks: Vec<Track>,
    /// The indices of tracks in playback order.
    play_order: Vec<usize>,
    /// The current position in the `play_order`.
    current_position: Option<usize>,
    repeat_mode: RepeatMode,
    shuffle: bool,
}

impl Queue {
    /// Creates a new `Queue`.
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            play_order: Vec::new(),
            current_position: None,
            repeat_mode: RepeatMode::Off,
            shuffle: false,
        }
    }

    /// Returns the current repeat mode.
    #[allow(dead_code)]
    pub fn repeat_mode(&self) -> RepeatMode {
        self.repeat_mode
    }

    /// Sets the repeat mode.
    pub fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
    }

    /// Returns whether shuffle is enabled.
    pub fn shuffle(&self) -> bool {
        self.shuffle
    }

    /// Enables or disables shuffle mode.
    pub fn set_shuffle(&mut self, shuffle: bool) {
        if self.shuffle == shuffle {
            return;
        }
        self.shuffle = shuffle;
        if self.tracks.is_empty() {
            return;
        }

        let current_track_idx = self.current_index();

        if shuffle {
            // Turning shuffle on
            let mut rng = thread_rng();
            self.play_order.shuffle(&mut rng);

            // If there's a playing track, move it to the front of the shuffled order
            if let Some(curr_idx) = current_track_idx {
                if let Some(pos) = self.play_order.iter().position(|&idx| idx == curr_idx) {
                    self.play_order.swap(0, pos);
                    self.current_position = Some(0);
                }
            }
        } else {
            // Turning shuffle off
            self.play_order = (0..self.tracks.len()).collect();
            if let Some(curr_idx) = current_track_idx {
                self.current_position = Some(curr_idx);
            }
        }
    }

    /// Adds a track to the end of the queue.
    pub fn add_track(&mut self, track: Track) {
        let new_index = self.tracks.len();
        self.tracks.push(track);

        // Always append to the end of the current play order, even if shuffled
        self.play_order.push(new_index);

        if self.current_position.is_none() {
            self.current_position = Some(0);
        }
    }

    /// Returns the tracks currently in the queue, in their original order.
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// Removes a track at the given original index.
    pub fn remove_track(&mut self, index: usize) {
        if index >= self.tracks.len() {
            return;
        }

        let removed_play_pos = self.play_order.iter().position(|&i| i == index).unwrap();

        self.tracks.remove(index);
        self.play_order.remove(removed_play_pos);

        // Update indices in play_order to reflect removal
        for idx in &mut self.play_order {
            if *idx > index {
                *idx -= 1;
            }
        }

        if let Some(curr_pos) = self.current_position {
            if self.tracks.is_empty() {
                self.current_position = None;
            } else if removed_play_pos < curr_pos
                || (removed_play_pos == curr_pos && curr_pos == self.play_order.len())
            {
                self.current_position = Some(curr_pos.saturating_sub(1));
            }
        }
    }

    /// Clears all tracks from the queue.
    pub fn clear(&mut self) {
        self.tracks.clear();
        self.play_order.clear();
        self.current_position = None;
    }

    /// Advances to the next track and returns it.
    pub fn next(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }

        if let Some(mut curr_pos) = self.current_position {
            match self.repeat_mode {
                RepeatMode::One => {
                    // Stay on the same track
                }
                RepeatMode::Queue => {
                    curr_pos = (curr_pos + 1) % self.play_order.len();
                    self.current_position = Some(curr_pos);
                }
                RepeatMode::Off => {
                    if curr_pos + 1 < self.play_order.len() {
                        curr_pos += 1;
                        self.current_position = Some(curr_pos);
                    } else {
                        return None;
                    }
                }
            }
            return self.current();
        }
        None
    }

    /// Moves to the previous track and returns it.
    pub fn previous(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }

        if let Some(mut curr_pos) = self.current_position {
            match self.repeat_mode {
                RepeatMode::One => {
                    // Restart same track (stay on the same track)
                }
                RepeatMode::Queue | RepeatMode::Off => {
                    if curr_pos > 0 {
                        curr_pos -= 1;
                        self.current_position = Some(curr_pos);
                    } else if self.repeat_mode == RepeatMode::Queue {
                        curr_pos = self.play_order.len() - 1;
                        self.current_position = Some(curr_pos);
                    } else {
                        // Off and at start, we could go to start of current track,
                        // but returning None matches previous behavior.
                        return None;
                    }
                }
            }
            return self.current();
        }
        None
    }

    /// Returns the current track, if any.
    pub fn current(&self) -> Option<&Track> {
        self.current_index().and_then(|idx| self.tracks.get(idx))
    }

    /// Returns the current original index.
    pub fn current_index(&self) -> Option<usize> {
        self.current_position
            .and_then(|pos| self.play_order.get(pos).copied())
    }

    /// Returns whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::TrackMetadata;
    use std::path::PathBuf;

    fn create_dummy_track(title: &str) -> Track {
        Track {
            path: PathBuf::from(format!("/fake/{}", title)),
            metadata: TrackMetadata {
                title: Some(title.to_string()),
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_add_tracks() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());

        queue.add_track(create_dummy_track("A"));
        assert_eq!(queue.tracks().len(), 1);
        assert_eq!(queue.current_index(), Some(0));

        queue.add_track(create_dummy_track("B"));
        assert_eq!(queue.tracks().len(), 2);
        assert_eq!(queue.current_index(), Some(0));
    }

    #[test]
    fn test_remove_tracks() {
        let mut queue = Queue::new();
        queue.add_track(create_dummy_track("A"));
        queue.add_track(create_dummy_track("B"));
        queue.add_track(create_dummy_track("C"));

        queue.current_position = Some(1); // Playing B

        // Remove A (before current)
        queue.remove_track(0);
        assert_eq!(queue.tracks().len(), 2);
        assert_eq!(
            queue.current().unwrap().metadata.title.as_deref(),
            Some("B")
        );
        assert_eq!(queue.current_index(), Some(0)); // Now B is at index 0

        // Remove C (after current)
        queue.remove_track(1);
        assert_eq!(queue.tracks().len(), 1);
        assert_eq!(queue.current_index(), Some(0));

        // Remove B (the current track itself, which is the last track)
        queue.remove_track(0);
        assert!(queue.is_empty());
        assert_eq!(queue.current_index(), None);
    }

    #[test]
    fn test_clear() {
        let mut queue = Queue::new();
        queue.add_track(create_dummy_track("A"));
        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.current_index(), None);
    }

    #[test]
    fn test_next() {
        let mut queue = Queue::new();
        queue.add_track(create_dummy_track("A"));
        queue.add_track(create_dummy_track("B"));

        let next = queue.next();
        assert_eq!(next.unwrap().metadata.title.as_deref(), Some("B"));

        let end = queue.next();
        assert!(end.is_none());
        assert_eq!(queue.current_index(), Some(1)); // Stays at end
    }

    #[test]
    fn test_previous() {
        let mut queue = Queue::new();
        queue.add_track(create_dummy_track("A"));
        queue.add_track(create_dummy_track("B"));
        queue.next(); // Now at B

        let prev = queue.previous();
        assert_eq!(prev.unwrap().metadata.title.as_deref(), Some("A"));

        let start = queue.previous();
        assert!(start.is_none());
        assert_eq!(queue.current_index(), Some(0)); // Stays at start
    }

    #[test]
    fn test_shuffle() {
        let mut queue = Queue::new();
        queue.add_track(create_dummy_track("A"));
        queue.add_track(create_dummy_track("B"));
        queue.add_track(create_dummy_track("C"));
        queue.add_track(create_dummy_track("D"));

        // Current track is A (index 0)
        queue.set_shuffle(true);
        assert!(queue.shuffle());
        // Shuffling should keep the current track as the active one, effectively moving it to play_order[0]
        assert_eq!(
            queue.current().unwrap().metadata.title.as_deref(),
            Some("A")
        );

        let mut order = Vec::new();
        order.push(
            queue
                .current()
                .unwrap()
                .metadata
                .title
                .as_deref()
                .unwrap()
                .to_string(),
        );

        while let Some(track) = queue.next() {
            order.push(track.metadata.title.clone().unwrap());
        }

        assert_eq!(order.len(), 4);
        assert!(order.contains(&"A".to_string()));
        assert!(order.contains(&"B".to_string()));
        assert!(order.contains(&"C".to_string()));
        assert!(order.contains(&"D".to_string()));

        // Turning shuffle off should restore original order starting from the currently active track
        // Wait, turning off shuffle restores the sequential order but keeps current track active.
        queue.set_shuffle(false);
        assert!(!queue.shuffle());

        // Let's test that it restores to sequential
        // Current index was whatever track was last playing.
        let curr_idx = queue.current_index().unwrap();
        if curr_idx < 3 {
            let _ = queue.next();
            assert_eq!(queue.current_index(), Some(curr_idx + 1));
        }
    }

    #[test]
    fn test_repeat_mode() {
        let mut queue = Queue::new();
        queue.add_track(create_dummy_track("A"));
        queue.add_track(create_dummy_track("B"));

        queue.set_repeat_mode(RepeatMode::Queue);
        queue.next(); // At B
        let next = queue.next(); // Wrap to A
        assert_eq!(next.unwrap().metadata.title.as_deref(), Some("A"));

        queue.set_repeat_mode(RepeatMode::One);
        let next = queue.next(); // Repeat A
        assert_eq!(next.unwrap().metadata.title.as_deref(), Some("A"));
    }
}

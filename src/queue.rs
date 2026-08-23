//! Playback queue management and unified library/queue sequence traversal.

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::library::{Library, Track};

/// Repeat mode for playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RepeatMode {
    #[default]
    Off,
    One,
    Queue,
}

/// Identifies whether the active track came from the user priority queue or the library sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackSource {
    #[default]
    None,
    Queue,
    Library,
}

/// Track queue structure for managing user queue items and library playback sequence.
#[derive(Debug, Default)]
pub struct Queue {
    /// Temporary priority queue of tracks explicitly added by user.
    queue_tracks: Vec<Track>,
    queue_order: Vec<usize>,
    queue_position: Option<usize>,

    /// Ordered sequence of track indices into Library.tracks.
    library_order: Vec<usize>,
    library_position: Option<usize>,

    /// Source of current playback track.
    current_source: PlaybackSource,

    repeat_mode: RepeatMode,
    shuffle: bool,
}

impl Queue {
    /// Creates a new `Queue`.
    pub fn new() -> Self {
        Self {
            queue_tracks: Vec::new(),
            queue_order: Vec::new(),
            queue_position: None,
            library_order: Vec::new(),
            library_position: None,
            current_source: PlaybackSource::None,
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
    #[allow(dead_code)]
    pub fn shuffle(&self) -> bool {
        self.shuffle
    }

    /// Returns the current playback source (Queue, Library, or None).
    #[allow(dead_code)]
    pub fn current_source(&self) -> PlaybackSource {
        self.current_source
    }

    /// Synchronizes the library play order with the current library size.
    pub fn sync_library(&mut self, library: &Library) {
        let len = library.tracks.len();
        if self.library_order.len() != len {
            let mut order: Vec<usize> = (0..len).collect();
            if self.shuffle {
                let mut rng = thread_rng();
                order.shuffle(&mut rng);
            }
            self.library_order = order;
        }
        if len == 0 {
            self.library_position = None;
            if self.current_source == PlaybackSource::Library {
                self.current_source = PlaybackSource::None;
            }
        }
    }

    /// Sets shuffle mode. Re-orders both user queue and library playback sequences if changed.
    pub fn set_shuffle(&mut self, shuffle: bool) {
        if self.shuffle == shuffle {
            return;
        }
        self.shuffle = shuffle;

        // 1. User queue shuffle
        let curr_queue_idx = self
            .queue_position
            .and_then(|pos| self.queue_order.get(pos).copied());
        if shuffle {
            let mut rng = thread_rng();
            self.queue_order.shuffle(&mut rng);
            if let Some(curr_idx) = curr_queue_idx {
                if let Some(pos) = self.queue_order.iter().position(|&idx| idx == curr_idx) {
                    self.queue_order.swap(0, pos);
                    self.queue_position = Some(0);
                }
            }
        } else {
            self.queue_order = (0..self.queue_tracks.len()).collect();
            if let Some(curr_idx) = curr_queue_idx {
                self.queue_position = Some(curr_idx);
            }
        }

        // 2. Library sequence shuffle
        let curr_lib_idx = self
            .library_position
            .and_then(|pos| self.library_order.get(pos).copied());
        let lib_len = self.library_order.len();
        if shuffle {
            let mut rng = thread_rng();
            self.library_order.shuffle(&mut rng);
            if let Some(curr_idx) = curr_lib_idx {
                if let Some(pos) = self.library_order.iter().position(|&idx| idx == curr_idx) {
                    self.library_order.swap(0, pos);
                    self.library_position = Some(0);
                }
            }
        } else {
            self.library_order = (0..lib_len).collect();
            if let Some(curr_idx) = curr_lib_idx {
                self.library_position = Some(curr_idx);
            }
        }
    }

    /// Sets the current track to a specific library track and switches active source to Library.
    pub fn play_library_track(&mut self, library: &Library, track: &Track) -> Option<usize> {
        self.sync_library(library);
        let track_idx = library.tracks.iter().position(|t| t.path == track.path)?;
        let order_pos = self.library_order.iter().position(|&i| i == track_idx)?;

        self.library_position = Some(order_pos);
        self.current_source = PlaybackSource::Library;
        Some(order_pos)
    }

    /// Adds a track to the end of the user priority queue.
    #[allow(dead_code)]
    pub fn add_track(&mut self, track: Track) {
        let new_index = self.queue_tracks.len();
        self.queue_tracks.push(track);
        self.queue_order.push(new_index);

        if self.queue_position.is_none() {
            self.queue_position = Some(0);
        }
    }

    /// Sets the active playback item to a specific index in the user queue.
    #[allow(dead_code)]
    pub fn play_queue_index(&mut self, index: usize) {
        if index < self.queue_tracks.len() {
            if let Some(pos) = self.queue_order.iter().position(|&i| i == index) {
                self.queue_position = Some(pos);
                self.current_source = PlaybackSource::Queue;
            }
        }
    }

    /// Returns the tracks currently in the user priority queue.
    #[allow(dead_code)]
    pub fn tracks(&self) -> &[Track] {
        &self.queue_tracks
    }

    /// Removes a track from the user priority queue at the given original index.
    #[allow(dead_code)]
    pub fn remove_track(&mut self, index: usize) {
        if index >= self.queue_tracks.len() {
            return;
        }

        let removed_play_pos = match self.queue_order.iter().position(|&i| i == index) {
            Some(pos) => pos,
            None => return,
        };

        self.queue_tracks.remove(index);
        self.queue_order.remove(removed_play_pos);

        for idx in &mut self.queue_order {
            if *idx > index {
                *idx -= 1;
            }
        }

        if let Some(curr_pos) = self.queue_position {
            if self.queue_tracks.is_empty() {
                self.queue_position = None;
                if self.current_source == PlaybackSource::Queue {
                    self.current_source = PlaybackSource::None;
                }
            } else if removed_play_pos < curr_pos
                || (removed_play_pos == curr_pos && curr_pos == self.queue_order.len())
            {
                self.queue_position = Some(curr_pos.saturating_sub(1));
            }
        }
    }

    /// Clears all tracks from the user priority queue.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.queue_tracks.clear();
        self.queue_order.clear();
        self.queue_position = None;
        if self.current_source == PlaybackSource::Queue {
            self.current_source = PlaybackSource::None;
        }
    }

    /// Returns the currently active track from Queue or Library.
    pub fn current<'a>(&'a self, library: &'a Library) -> Option<&'a Track> {
        match self.current_source {
            PlaybackSource::Queue => {
                let pos = self.queue_position?;
                let idx = *self.queue_order.get(pos)?;
                self.queue_tracks.get(idx)
            }
            PlaybackSource::Library => {
                let pos = self.library_position?;
                let idx = *self.library_order.get(pos)?;
                library.tracks.get(idx)
            }
            PlaybackSource::None => None,
        }
    }

    /// Returns the current original index within the active source (Queue or Library).
    pub fn current_index(&self) -> Option<usize> {
        match self.current_source {
            PlaybackSource::Queue => self
                .queue_position
                .and_then(|pos| self.queue_order.get(pos).copied()),
            PlaybackSource::Library => self
                .library_position
                .and_then(|pos| self.library_order.get(pos).copied()),
            PlaybackSource::None => None,
        }
    }

    /// Advances to the next track following playback priority: Queue first, then Library.
    pub fn next<'a>(&'a mut self, library: &'a Library) -> Option<&'a Track> {
        self.sync_library(library);

        if self.repeat_mode == RepeatMode::One {
            return self.current(library);
        }

        match self.current_source {
            PlaybackSource::Queue => {
                if self.queue_tracks.is_empty() {
                    return self.advance_library_next(library);
                }

                let curr_pos = self.queue_position.unwrap_or(0);
                match self.repeat_mode {
                    RepeatMode::Queue => {
                        let next_pos = (curr_pos + 1) % self.queue_order.len();
                        self.queue_position = Some(next_pos);
                        self.current(library)
                    }
                    RepeatMode::Off | RepeatMode::One => {
                        if curr_pos + 1 < self.queue_order.len() {
                            self.queue_position = Some(curr_pos + 1);
                            self.current(library)
                        } else {
                            // Queue EXHAUSTED! Transition to Library
                            self.transition_from_queue_to_library(library)
                        }
                    }
                }
            }
            PlaybackSource::Library => {
                // If user added queue items that have not been played yet
                if let Some(unplayed_pos) = self.unplayed_queue_position() {
                    self.queue_position = Some(unplayed_pos);
                    self.current_source = PlaybackSource::Queue;
                    return self.current(library);
                }
                self.advance_library_next(library)
            }
            PlaybackSource::None => {
                if !self.queue_tracks.is_empty() {
                    self.queue_position = Some(0);
                    self.current_source = PlaybackSource::Queue;
                    self.current(library)
                } else if !library.tracks.is_empty() {
                    self.library_position = Some(0);
                    self.current_source = PlaybackSource::Library;
                    self.current(library)
                } else {
                    None
                }
            }
        }
    }

    /// Moves to the previous track.
    pub fn previous<'a>(&'a mut self, library: &'a Library) -> Option<&'a Track> {
        self.sync_library(library);

        if self.repeat_mode == RepeatMode::One {
            return self.current(library);
        }

        match self.current_source {
            PlaybackSource::Queue => {
                let curr_pos = self.queue_position.unwrap_or(0);
                if curr_pos > 0 {
                    self.queue_position = Some(curr_pos - 1);
                    self.current(library)
                } else if self.repeat_mode == RepeatMode::Queue && !self.queue_order.is_empty() {
                    self.queue_position = Some(self.queue_order.len() - 1);
                    self.current(library)
                } else {
                    // Start of Queue reached! Fall back to Library position
                    self.fallback_from_queue_to_library(library)
                }
            }
            PlaybackSource::Library => {
                if let Some(lib_pos) = self.library_position {
                    if lib_pos > 0 {
                        self.library_position = Some(lib_pos - 1);
                        self.current(library)
                    } else if self.repeat_mode == RepeatMode::Queue
                        && !self.library_order.is_empty()
                    {
                        self.library_position = Some(self.library_order.len() - 1);
                        self.current(library)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            PlaybackSource::None => None,
        }
    }

    /// Helper to transition playback from an exhausted Queue to the Library sequence.
    fn transition_from_queue_to_library<'a>(
        &'a mut self,
        library: &'a Library,
    ) -> Option<&'a Track> {
        // Check if any played track in the queue is present in the library, working backwards from current queue_position
        if let Some(curr_pos) = self.queue_position {
            for pos in (0..=curr_pos).rev() {
                if let Some(&track_idx) = self.queue_order.get(pos) {
                    if let Some(track) = self.queue_tracks.get(track_idx) {
                        if let Some(lib_track_idx) =
                            library.tracks.iter().position(|t| t.path == track.path)
                        {
                            if let Some(lib_pos) =
                                self.library_order.iter().position(|&i| i == lib_track_idx)
                            {
                                if lib_pos + 1 < self.library_order.len() {
                                    self.library_position = Some(lib_pos + 1);
                                    self.current_source = PlaybackSource::Library;
                                    return self.current(library);
                                } else {
                                    self.current_source = PlaybackSource::None;
                                    return None;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Otherwise continue from library_position
        if let Some(lib_pos) = self.library_position {
            if lib_pos + 1 < self.library_order.len() {
                self.library_position = Some(lib_pos + 1);
                self.current_source = PlaybackSource::Library;
                self.current(library)
            } else {
                self.current_source = PlaybackSource::None;
                None
            }
        } else if !self.library_order.is_empty() {
            self.library_position = Some(0);
            self.current_source = PlaybackSource::Library;
            self.current(library)
        } else {
            self.current_source = PlaybackSource::None;
            None
        }
    }

    /// Helper to fall back from the start of the Queue to the Library position.
    fn fallback_from_queue_to_library<'a>(&'a mut self, library: &'a Library) -> Option<&'a Track> {
        if self.library_position.is_some() {
            self.current_source = PlaybackSource::Library;
            self.current(library)
        } else if !self.library_order.is_empty() {
            self.library_position = Some(0);
            self.current_source = PlaybackSource::Library;
            self.current(library)
        } else {
            self.current_source = PlaybackSource::None;
            None
        }
    }

    /// Helper to advance to the next track within the Library sequence.
    fn advance_library_next<'a>(&'a mut self, library: &'a Library) -> Option<&'a Track> {
        if let Some(lib_pos) = self.library_position {
            if lib_pos + 1 < self.library_order.len() {
                self.library_position = Some(lib_pos + 1);
                self.current_source = PlaybackSource::Library;
                self.current(library)
            } else {
                self.current_source = PlaybackSource::None;
                None
            }
        } else if !self.library_order.is_empty() {
            self.library_position = Some(0);
            self.current_source = PlaybackSource::Library;
            self.current(library)
        } else {
            self.current_source = PlaybackSource::None;
            None
        }
    }

    /// Checks if there is an unplayed position in the user queue.
    fn unplayed_queue_position(&self) -> Option<usize> {
        if self.queue_tracks.is_empty() {
            return None;
        }
        match self.queue_position {
            Some(pos) => {
                if pos + 1 < self.queue_order.len() {
                    Some(pos + 1)
                } else {
                    None
                }
            }
            None => Some(0),
        }
    }

    /// Returns whether both Queue and Library are empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.queue_tracks.is_empty() && self.library_order.is_empty()
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

    fn title(t: Option<&Track>) -> Option<&str> {
        t?.metadata.title.as_deref()
    }

    fn create_sample_library(titles: &[&str]) -> Library {
        let mut lib = Library::new();
        for t in titles {
            lib.tracks.push(create_dummy_track(t));
        }
        lib
    }

    // -----------------------------------------------------------------------
    // Case 1: No queue – Library navigation
    // -----------------------------------------------------------------------

    #[test]
    fn test_case_1_no_queue_library_navigation() {
        let library = create_sample_library(&["A", "B", "C", "D"]);
        let mut queue = Queue::new();
        queue.sync_library(&library);

        // User clicks B (index 1)
        queue.play_library_track(&library, &library.tracks[1]);
        assert_eq!(title(queue.current(&library)), Some("B"));

        // Next -> C
        assert_eq!(title(queue.next(&library)), Some("C"));
        // Next -> D
        assert_eq!(title(queue.next(&library)), Some("D"));
        // Next -> None (Library end)
        assert!(queue.next(&library).is_none());

        // Previous back from D -> C
        queue.play_library_track(&library, &library.tracks[3]); // At D
        assert_eq!(title(queue.previous(&library)), Some("C"));
        assert_eq!(title(queue.previous(&library)), Some("B"));
        assert_eq!(title(queue.previous(&library)), Some("A"));
        // Previous at A -> None
        assert!(queue.previous(&library).is_none());
    }

    // -----------------------------------------------------------------------
    // Case 2: Queue exists – Priority queue then Library continuation
    // -----------------------------------------------------------------------

    #[test]
    fn test_case_2_queue_exists_then_library() {
        let library = create_sample_library(&["A", "B", "C", "D", "E"]);
        let mut queue = Queue::new();
        queue.sync_library(&library);

        // User plays B
        queue.play_library_track(&library, &library.tracks[1]);

        // User queues X and Y
        queue.add_track(create_dummy_track("X"));
        queue.add_track(create_dummy_track("Y"));

        // Play X from queue
        queue.play_queue_index(0);
        assert_eq!(title(queue.current(&library)), Some("X"));

        // Next -> Y
        assert_eq!(title(queue.next(&library)), Some("Y"));

        // Next after Y (Queue exhausted) -> Continues into Library at C
        assert_eq!(title(queue.next(&library)), Some("C"));
        assert_eq!(title(queue.next(&library)), Some("D"));
    }

    // -----------------------------------------------------------------------
    // Case 3: Queue contains track present in Library
    // -----------------------------------------------------------------------

    #[test]
    fn test_case_3_queue_contains_library_track() {
        let library = create_sample_library(&["A", "B", "C", "D"]);
        let mut queue = Queue::new();
        queue.sync_library(&library);

        // User queues C and X
        queue.add_track(create_dummy_track("C"));
        queue.add_track(create_dummy_track("X"));

        // Play C from Queue
        queue.play_queue_index(0);
        assert_eq!(title(queue.current(&library)), Some("C"));

        // Next -> X
        assert_eq!(title(queue.next(&library)), Some("X"));

        // Next after X -> Continues through Library from position after C (which is D)
        assert_eq!(title(queue.next(&library)), Some("D"));
    }

    // -----------------------------------------------------------------------
    // Case 4: Queue empty after single queue track
    // -----------------------------------------------------------------------

    #[test]
    fn test_case_4_single_queue_track_to_library() {
        let library = create_sample_library(&["A", "B", "C", "D"]);
        let mut queue = Queue::new();
        queue.sync_library(&library);

        // User is at A in library
        queue.play_library_track(&library, &library.tracks[0]);

        // User queues X
        queue.add_track(create_dummy_track("X"));
        queue.play_queue_index(0);
        assert_eq!(title(queue.current(&library)), Some("X"));

        // Next after X -> continues into Library at B
        assert_eq!(title(queue.next(&library)), Some("B"));
    }

    // -----------------------------------------------------------------------
    // Case 5: Search result starts playback
    // -----------------------------------------------------------------------

    #[test]
    fn test_case_5_search_result_playback() {
        let library = create_sample_library(&["A", "B", "C", "D"]);
        let mut queue = Queue::new();
        queue.sync_library(&library);

        // User searches for B and selects B from library
        queue.play_library_track(&library, &library.tracks[1]);
        assert_eq!(title(queue.current(&library)), Some("B"));

        // Next follows normal library order -> C
        assert_eq!(title(queue.next(&library)), Some("C"));
        // Previous follows normal library order -> B -> A
        assert_eq!(title(queue.previous(&library)), Some("B"));
        assert_eq!(title(queue.previous(&library)), Some("A"));
    }

    // -----------------------------------------------------------------------
    // Case 6: Shuffle and Repeat modes
    // -----------------------------------------------------------------------

    #[test]
    fn test_case_6_repeat_one() {
        let library = create_sample_library(&["A", "B", "C"]);
        let mut queue = Queue::new();
        queue.sync_library(&library);
        queue.set_repeat_mode(RepeatMode::One);

        queue.play_library_track(&library, &library.tracks[1]); // B
        assert_eq!(title(queue.next(&library)), Some("B"));
        assert_eq!(title(queue.next(&library)), Some("B"));
        assert_eq!(title(queue.previous(&library)), Some("B"));
    }

    #[test]
    fn test_case_6_repeat_queue() {
        let library = create_sample_library(&["A", "B"]);
        let mut queue = Queue::new();
        queue.sync_library(&library);

        queue.add_track(create_dummy_track("X"));
        queue.add_track(create_dummy_track("Y"));
        queue.play_queue_index(0);
        queue.set_repeat_mode(RepeatMode::Queue);

        assert_eq!(title(queue.next(&library)), Some("Y"));
        // RepeatQueue wraps back to X
        assert_eq!(title(queue.next(&library)), Some("X"));
    }

    #[test]
    fn test_case_6_shuffle_library() {
        let library = create_sample_library(&["A", "B", "C", "D", "E"]);
        let mut queue = Queue::new();
        queue.set_shuffle(true);
        queue.sync_library(&library);

        // Start at first track in shuffled library
        assert!(queue.next(&library).is_some());
        let mut count = 1;
        while queue.next(&library).is_some() {
            count += 1;
        }
        assert_eq!(count, 5);
    }
}

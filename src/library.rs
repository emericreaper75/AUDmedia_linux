//! Music library management and file scanning.

use crate::metadata::{extract_metadata, TrackMetadata};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A single track in the library.
#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub metadata: TrackMetadata,
}

/// In-memory music library structure.
#[derive(Debug, Default)]
pub struct Library {
    pub tracks: Vec<Track>,
}

impl Library {
    /// Creates a new empty `Library`.
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    /// Scans a directory recursively and builds a `Library`.
    /// `progress_callback` is called periodically with the number of discovered tracks.
    pub fn scan_directory<P: AsRef<Path>, F: FnMut(usize)>(
        path: P,
        mut progress_callback: F,
    ) -> Self {
        let mut tracks = Vec::new();
        let supported_extensions = ["mp3", "flac", "ogg", "m4a", "wav"];

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                        match extract_metadata(path) {
                            Ok(metadata) => {
                                tracks.push(Track {
                                    path: path.to_path_buf(),
                                    metadata,
                                });
                            }
                            Err(e) => {
                                eprintln!("Failed to read metadata for {:?}: {}", path, e);
                                // Add with default metadata so it can still be played
                                tracks.push(Track {
                                    path: path.to_path_buf(),
                                    metadata: TrackMetadata::default(),
                                });
                            }
                        }

                        if tracks.len() % 10 == 0 {
                            progress_callback(tracks.len());
                        }
                    }
                }
            }
        }

        progress_callback(tracks.len());
        Self { tracks }
    }
}

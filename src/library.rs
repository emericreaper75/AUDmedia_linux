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
    pub fn scan_directory<P: AsRef<Path>, C: AsRef<Path>, F: FnMut(usize)>(
        path: P,
        cache_dir: C,
        mut progress_callback: F,
    ) -> Self {
        let mut tracks = Vec::new();
        let supported_extensions = ["mp3", "flac", "ogg", "m4a", "wav"];
        let cache_path = cache_dir.as_ref();

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                        match extract_metadata(path, cache_path) {
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

    /// Searches the library for tracks matching the query in title, artist, or album.
    /// Matching is case-insensitive. Returns an empty vector if the query is empty or whitespace.
    pub fn search(&self, query: &str) -> Vec<&Track> {
        let query_trim = query.trim();
        if query_trim.is_empty() {
            return Vec::new();
        }

        let query_lower = query_trim.to_lowercase();

        self.tracks
            .iter()
            .filter(|track| {
                let matches_title = track
                    .metadata
                    .title
                    .as_deref()
                    .map_or(false, |t| t.to_lowercase().contains(&query_lower));
                let matches_artist = track
                    .metadata
                    .artist
                    .as_deref()
                    .map_or(false, |a| a.to_lowercase().contains(&query_lower));
                let matches_album = track
                    .metadata
                    .album
                    .as_deref()
                    .map_or(false, |a| a.to_lowercase().contains(&query_lower));

                matches_title || matches_artist || matches_album
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_track(title: Option<&str>, artist: Option<&str>, album: Option<&str>) -> Track {
        Track {
            path: PathBuf::from("/fake/path"),
            metadata: TrackMetadata {
                title: title.map(String::from),
                artist: artist.map(String::from),
                album: album.map(String::from),
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_search() {
        let mut library = Library::new();
        library.tracks.push(create_track(
            Some("Bohemian Rhapsody"),
            Some("Queen"),
            Some("A Night at the Opera"),
        ));
        library.tracks.push(create_track(
            Some("Radio Ga Ga"),
            Some("Queen"),
            Some("The Works"),
        ));
        library.tracks.push(create_track(
            Some("Hotel California"),
            Some("Eagles"),
            Some("Hotel California"),
        ));
        library.tracks.push(create_track(
            Some("Stairway to Heaven"),
            Some("Led Zeppelin"),
            Some("Led Zeppelin IV"),
        ));
        library.tracks.push(create_track(None, None, None)); // Empty metadata

        // Exact matches
        let results = library.search("Bohemian Rhapsody");
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].metadata.title.as_deref().unwrap(),
            "Bohemian Rhapsody"
        );

        // Partial matches
        let results = library.search("Ga Ga");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].metadata.title.as_deref().unwrap(), "Radio Ga Ga");

        // Case differences
        let results = library.search("qUeEn");
        assert_eq!(results.len(), 2);

        // Artist search
        let results = library.search("Eagles");
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].metadata.title.as_deref().unwrap(),
            "Hotel California"
        );

        // Album search
        let results = library.search("Opera");
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].metadata.title.as_deref().unwrap(),
            "Bohemian Rhapsody"
        );

        // Empty search
        let results = library.search("");
        assert_eq!(results.len(), 0);
        let results = library.search("   ");
        assert_eq!(results.len(), 0);

        // No results
        let results = library.search("Beatles");
        assert_eq!(results.len(), 0);
    }
}

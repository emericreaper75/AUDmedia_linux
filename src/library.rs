//! Music library management and file scanning.

use crate::metadata::{extract_metadata, TrackMetadata};
use jwalk::WalkDir;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

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
        let start_time = std::time::Instant::now();
        let supported_extensions = ["mp3", "flac", "ogg", "m4a", "wav"];
        let cache_path_buf = cache_dir.as_ref().to_path_buf();

        // 1. Collect all valid audio file paths sequentially
        let paths: Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path())
            .filter(|p| {
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    supported_extensions.contains(&ext.to_lowercase().as_str())
                } else {
                    false
                }
            })
            .collect();

        let paths_len = paths.len();

        // 2. Parallelize metadata extraction on a background thread pool, sending results via a channel
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            paths.into_par_iter().for_each_with(tx, |tx, path| {
                match extract_metadata(&path, &cache_path_buf) {
                    Ok(metadata) => {
                        let _ = tx.send(Track { path, metadata });
                    }
                    Err(e) => {
                        eprintln!(
                            "Skipping invalid or unreadable audio file {:?}: {}",
                            path, e
                        );
                    }
                }
            });
        });

        // 3. Receive tracks on this thread (so we don't have to require Sync/Send on progress_callback)
        let mut tracks = Vec::with_capacity(paths_len);
        for track in rx {
            tracks.push(track);
            if tracks.len() % 10 == 0 {
                progress_callback(tracks.len());
            }
        }

        progress_callback(tracks.len());
        let duration = start_time.elapsed();
        println!(
            "Library scan completed in {:.2?} ({} tracks)",
            duration,
            tracks.len()
        );

        Self { tracks }
    }

    /// Searches the library for tracks matching the query in title, artist, or album.
    /// Matching is case-insensitive. Returns an empty vector if the query is empty or whitespace.
    #[allow(dead_code)]
    pub fn search(&self, query: &str) -> Vec<&Track> {
        let query_trim = query.trim();
        if query_trim.is_empty() {
            return Vec::new();
        }

        let query_lower = query_trim.to_lowercase();

        self.tracks
            .iter()
            .filter(|track| track.metadata.search_index.contains(&query_lower))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_track(title: Option<&str>, artist: Option<&str>, album: Option<&str>) -> Track {
        let mut search_parts = Vec::new();
        if let Some(t) = title {
            search_parts.push(t.to_lowercase());
        }
        if let Some(a) = artist {
            search_parts.push(a.to_lowercase());
        }
        if let Some(a) = album {
            search_parts.push(a.to_lowercase());
        }

        Track {
            path: PathBuf::from("/fake/path"),
            metadata: TrackMetadata {
                title: title.map(String::from),
                artist: artist.map(String::from),
                album: album.map(String::from),
                search_index: search_parts.join(" "),
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

        // Case differences & multiple matching tracks
        let results = library.search("qUeEn");
        assert_eq!(results.len(), 2);
        let titles: Vec<&str> = results
            .iter()
            .map(|t| t.metadata.title.as_deref().unwrap())
            .collect();
        assert!(titles.contains(&"Bohemian Rhapsody"));
        assert!(titles.contains(&"Radio Ga Ga"));

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

    fn create_minimal_wav(path: &Path) {
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&(36u32 + 100u32).to_le_bytes());
        data.extend_from_slice(b"WAVE");
        data.extend_from_slice(b"fmt ");
        data.extend_from_slice(&16u32.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&8000u32.to_le_bytes());
        data.extend_from_slice(&16000u32.to_le_bytes());
        data.extend_from_slice(&2u16.to_le_bytes());
        data.extend_from_slice(&16u16.to_le_bytes());
        data.extend_from_slice(b"data");
        data.extend_from_slice(&100u32.to_le_bytes());
        data.extend(vec![0u8; 100]);

        std::fs::write(path, data).expect("Failed to write test WAV file");
    }

    #[test]
    fn test_scan_directory_validation() {
        let temp_dir = std::env::temp_dir().join(format!("audmedia_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let cache_dir = temp_dir.join("cache");
        std::fs::create_dir_all(&cache_dir).unwrap();

        // 1. Valid audio file
        let valid_wav = temp_dir.join("valid_track.wav");
        create_minimal_wav(&valid_wav);

        // 2. Empty mp3
        let empty_mp3 = temp_dir.join("empty.mp3");
        std::fs::write(&empty_mp3, b"").unwrap();

        // 3. Text file renamed to mp3
        let fake_mp3 = temp_dir.join("fake.mp3");
        std::fs::write(&fake_mp3, b"This is text content, not an mp3 audio stream.").unwrap();

        // 4. Corrupt audio file
        let corrupt_mp3 = temp_dir.join("corrupt.mp3");
        std::fs::write(&corrupt_mp3, &[0xFF, 0xFB, 0x00, 0x00, 0x12, 0x34]).unwrap();

        // 5. Unsupported extension
        let doc_txt = temp_dir.join("document.txt");
        std::fs::write(&doc_txt, b"Plain text file").unwrap();

        // Perform scan
        let library = Library::scan_directory(&temp_dir, &cache_dir, |_| {});

        // Only valid_wav should be present
        assert_eq!(library.tracks.len(), 1);
        assert_eq!(library.tracks[0].path, valid_wav);
        // Missing metadata title should fallback to file stem ("valid_track")
        assert_eq!(
            library.tracks[0].metadata.title.as_deref(),
            Some("valid_track")
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}

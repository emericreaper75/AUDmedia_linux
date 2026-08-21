//! Audio metadata extraction and artwork discovery.

use image::imageops::FilterType;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::time::Duration;

/// Track metadata container.
#[derive(Debug, Clone, Default)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub duration: Option<Duration>,
    pub artwork_path: Option<String>,
    pub search_index: String,
}

pub fn extract_metadata(
    path: &Path,
    cache_dir: &Path,
) -> Result<TrackMetadata, Box<dyn std::error::Error>> {
    let tagged_file = Probe::open(path)?.guess_file_type()?.read()?;

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());

    let mut meta = TrackMetadata {
        duration: Some(tagged_file.properties().duration()),
        ..Default::default()
    };

    if let Some(t) = tag {
        meta.title = t.title().map(|s| s.into_owned());
        meta.artist = t.artist().map(|s| s.into_owned());
        meta.album = t.album().map(|s| s.into_owned());
        meta.track_number = t.track();
        meta.disc_number = t.disk();
        meta.genre = t.genre().map(|s| s.into_owned());

        meta.artwork_path = extract_and_cache_artwork(Some(t), path, cache_dir);
    } else {
        meta.artwork_path = extract_and_cache_artwork(None, path, cache_dir);
    }

    let mut search_parts = Vec::new();
    if let Some(ref t) = meta.title {
        search_parts.push(t.to_lowercase());
    }
    if let Some(ref a) = meta.artist {
        search_parts.push(a.to_lowercase());
    }
    if let Some(ref a) = meta.album {
        search_parts.push(a.to_lowercase());
    }
    meta.search_index = search_parts.join(" ");

    Ok(meta)
}

fn extract_and_cache_artwork(
    tag: Option<&lofty::tag::Tag>,
    path: &Path,
    cache_dir: &Path,
) -> Option<String> {
    // Generate a hash based on the directory path to deduplicate caching
    // since tracks in the same directory usually share the same artwork.
    let parent_dir = path.parent()?.to_string_lossy();
    let mut hasher = Sha256::new();
    hasher.update(parent_dir.as_bytes());
    let hash_bytes = hasher.finalize();
    let hash_str = hash_bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let cached_file_path = cache_dir.join(format!("{}.jpg", hash_str));

    if cached_file_path.exists() {
        return Some(cached_file_path.to_string_lossy().into_owned());
    }

    // Ensure cache dir exists
    if !cache_dir.exists() {
        let _ = std::fs::create_dir_all(cache_dir);
    }

    // Attempt 1: Embedded artwork
    if let Some(t) = tag {
        if let Some(pic) = t.pictures().first() {
            if let Ok(img) = image::load_from_memory(pic.data()) {
                let resized = img.resize_to_fill(256, 256, FilterType::Triangle);
                if resized.save(&cached_file_path).is_ok() {
                    return Some(cached_file_path.to_string_lossy().into_owned());
                }
            }
        }
    }

    // Attempt 2: Local artwork
    let local_names = ["cover.jpg", "cover.png", "folder.jpg", "folder.png"];
    for name in local_names {
        let local_path = path.parent()?.join(name);
        if local_path.exists() {
            if let Ok(img) = image::open(&local_path) {
                let resized = img.resize_to_fill(256, 256, FilterType::Triangle);
                if resized.save(&cached_file_path).is_ok() {
                    return Some(cached_file_path.to_string_lossy().into_owned());
                }
            }
        }
    }

    None
}

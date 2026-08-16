//! Audio metadata extraction and artwork discovery.

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
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
}

pub fn extract_metadata(path: &Path) -> Result<TrackMetadata, Box<dyn std::error::Error>> {
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
        // Try to get year if available in Accessor, otherwise ignore for now
        // Some formats don't have a direct year() method without fetching an Item
    }

    Ok(meta)
}

//! Audio metadata extraction and artwork discovery.

/// Track metadata container.
#[derive(Debug, Clone, Default)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

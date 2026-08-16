//! GStreamer audio player engine.

use gstreamer as gst;
use gstreamer::glib;
use gstreamer::prelude::*;

/// Audio player managing playback state and GStreamer pipeline.
#[derive(Debug)]
pub struct Player {
    pipeline: gst::Element,
    _bus_watch: gst::bus::BusWatchGuard,
}

impl Player {
    /// Creates a new `Player` instance.
    pub fn new() -> Result<Self, glib::Error> {
        let pipeline = gst::ElementFactory::make("playbin")
            .build()
            .map_err(|e| glib::Error::new(glib::FileError::Failed, &e.to_string()))?;

        let bus = pipeline.bus().expect("Pipeline should have a bus");
        let bus_watch = bus
            .add_watch(move |_, msg| {
                match msg.view() {
                    gst::MessageView::Eos(..) => {
                        println!("End of stream reached. (Queue support to be added)");
                    }
                    gst::MessageView::Error(err) => {
                        eprintln!("Playback error: {} ({:?})", err.error(), err.debug());
                    }
                    _ => (),
                }
                glib::ControlFlow::Continue
            })
            .expect("Failed to add bus watch");

        Ok(Self {
            pipeline,
            _bus_watch: bus_watch,
        })
    }

    /// Plays a file from the given URI.
    pub fn play_file(&self, uri: &str) -> Result<(), glib::Error> {
        self.pipeline
            .set_state(gst::State::Null)
            .map_err(|e| glib::Error::new(glib::FileError::Failed, &e.to_string()))?;
        self.pipeline.set_property("uri", uri);
        self.pipeline
            .set_state(gst::State::Playing)
            .map_err(|e| glib::Error::new(glib::FileError::Failed, &e.to_string()))?;
        Ok(())
    }

    /// Pauses playback.
    pub fn pause(&self) {
        let _ = self.pipeline.set_state(gst::State::Paused);
    }

    /// Resumes playback.
    pub fn resume(&self) {
        let _ = self.pipeline.set_state(gst::State::Playing);
    }

    /// Stops playback.
    pub fn stop(&self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }

    /// Returns whether the player is currently playing.
    pub fn is_playing(&self) -> bool {
        let (result, state, _pending) = self.pipeline.state(gst::ClockTime::ZERO);
        if result.is_ok() {
            state == gst::State::Playing
        } else {
            false
        }
    }

    /// Gets the current playback position in nanoseconds.
    pub fn position(&self) -> Option<u64> {
        self.pipeline.query_position::<gst::ClockTime>().map(|t| t.nseconds())
    }

    /// Gets the duration of the current file in nanoseconds.
    pub fn duration(&self) -> Option<u64> {
        self.pipeline.query_duration::<gst::ClockTime>().map(|t| t.nseconds())
    }

    /// Seeks to a specific position in nanoseconds.
    pub fn seek(&self, position_ns: u64) -> Result<(), glib::Error> {
        let flags = gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT;
        let pos = gst::ClockTime::from_nseconds(position_ns);
        if self.pipeline.seek_simple(flags, pos).is_err() {
            return Err(glib::Error::new(glib::FileError::Failed, "Seek failed"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        gst::init().unwrap();
        let player = Player::new();
        assert!(player.is_ok());
    }

    #[test]
    fn test_player_state() {
        gst::init().unwrap();
        let player = Player::new().unwrap();
        assert!(!player.is_playing());

        // Cannot play without valid URI but we can test stop/pause methods don't panic
        player.pause();
        assert!(!player.is_playing());

        player.stop();
        assert!(!player.is_playing());
    }
}

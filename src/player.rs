//! GStreamer audio player engine.

use gstreamer as gst;
use gstreamer::glib;
use gstreamer::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Audio player managing playback state and GStreamer pipeline.
#[derive(Clone)]
pub struct Player {
    pipeline: gst::Element,
    on_eos: Rc<RefCell<Option<Box<dyn FnMut()>>>>,
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player").finish()
    }
}

impl Player {
    /// Creates a new `Player` instance.
    pub fn new() -> Result<Self, glib::Error> {
        let pipeline = gst::ElementFactory::make("playbin")
            .build()
            .map_err(|e| glib::Error::new(glib::FileError::Failed, &e.to_string()))?;

        let bus = pipeline.bus().expect("Pipeline should have a bus");

        let on_eos = Rc::new(RefCell::new(None::<Box<dyn FnMut()>>));
        let on_eos_clone = on_eos.clone();

        let _bus_watch = bus
            .add_watch_local(move |_, msg| {
                match msg.view() {
                    gst::MessageView::Eos(..) => {
                        if let Some(cb) = on_eos_clone.borrow_mut().as_mut() {
                            cb();
                        }
                    }
                    gst::MessageView::Error(err) => {
                        eprintln!("Playback error: {} ({:?})", err.error(), err.debug());
                    }
                    _ => (),
                }
                glib::ControlFlow::Continue
            })
            .expect("Failed to add bus watch");

        Ok(Self { pipeline, on_eos })
    }

    /// Sets the callback to be executed when the player reaches the end of the stream.
    pub fn set_eos_callback<F: FnMut() + 'static>(&self, f: F) {
        *self.on_eos.borrow_mut() = Some(Box::new(f));
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

    /// Sets the volume (0.0 to 1.0).
    pub fn set_volume(&self, volume: f64) {
        self.pipeline.set_property("volume", volume);
    }

    /// Gets the current volume.
    pub fn volume(&self) -> f64 {
        self.pipeline.property("volume")
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
        self.pipeline
            .query_position::<gst::ClockTime>()
            .map(|t| t.nseconds())
    }

    /// Gets the duration of the current file in nanoseconds.
    pub fn duration(&self) -> Option<u64> {
        self.pipeline
            .query_duration::<gst::ClockTime>()
            .map(|t| t.nseconds())
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

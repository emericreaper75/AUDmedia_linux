use mpris_server::{
    zbus::{fdo, Result as ZbusResult},
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, RootInterface, Time,
    TrackId, Volume,
};

pub enum MprisCommand {
    PlayPause,
    Play,
    Pause,
    Next,
    Previous,
}

pub struct MprisPlayer {
    sender: async_channel::Sender<MprisCommand>,
}

impl MprisPlayer {
    pub fn new(sender: async_channel::Sender<MprisCommand>) -> Self {
        Self { sender }
    }
}

impl RootInterface for MprisPlayer {
    async fn identity(&self) -> fdo::Result<String> {
        Ok("AUDmedia".to_string())
    }

    async fn desktop_entry(&self) -> fdo::Result<String> {
        Ok("audmedia".to_string())
    }

    async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
        Ok(vec!["file".into()])
    }

    async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
        Ok(vec![
            "audio/mpeg".into(),
            "audio/flac".into(),
            "audio/ogg".into(),
            "audio/x-wav".into(),
        ])
    }

    async fn can_quit(&self) -> fdo::Result<bool> {
        Ok(false)
    }
    async fn quit(&self) -> fdo::Result<()> {
        Ok(())
    }
    async fn can_raise(&self) -> fdo::Result<bool> {
        Ok(false)
    }
    async fn raise(&self) -> fdo::Result<()> {
        Ok(())
    }
    async fn has_track_list(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn fullscreen(&self) -> fdo::Result<bool> {
        Ok(false)
    }
    async fn set_fullscreen(&self, _fs: bool) -> ZbusResult<()> {
        Ok(())
    }
    async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
        Ok(false)
    }
}

impl PlayerInterface for MprisPlayer {
    async fn next(&self) -> fdo::Result<()> {
        let _ = self.sender.send(MprisCommand::Next).await;
        Ok(())
    }
    async fn previous(&self) -> fdo::Result<()> {
        let _ = self.sender.send(MprisCommand::Previous).await;
        Ok(())
    }
    async fn pause(&self) -> fdo::Result<()> {
        let _ = self.sender.send(MprisCommand::Pause).await;
        Ok(())
    }
    async fn play_pause(&self) -> fdo::Result<()> {
        let _ = self.sender.send(MprisCommand::PlayPause).await;
        Ok(())
    }
    async fn stop(&self) -> fdo::Result<()> {
        let _ = self.sender.send(MprisCommand::Pause).await;
        Ok(())
    }
    async fn play(&self) -> fdo::Result<()> {
        let _ = self.sender.send(MprisCommand::Play).await;
        Ok(())
    }
    async fn seek(&self, _offset: Time) -> fdo::Result<()> {
        Ok(())
    }
    async fn set_position(&self, _track_id: TrackId, _pos: Time) -> fdo::Result<()> {
        Ok(())
    }
    async fn open_uri(&self, _uri: String) -> fdo::Result<()> {
        Ok(())
    }
    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        Ok(PlaybackStatus::Stopped) // This is just initial state, updates are sent via signal
    }
    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        Ok(LoopStatus::None)
    }
    async fn set_loop_status(&self, _ls: LoopStatus) -> ZbusResult<()> {
        Ok(())
    }
    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(PlaybackRate::default())
    }
    async fn set_rate(&self, _r: PlaybackRate) -> ZbusResult<()> {
        Ok(())
    }
    async fn shuffle(&self) -> fdo::Result<bool> {
        Ok(false)
    }
    async fn set_shuffle(&self, _s: bool) -> ZbusResult<()> {
        Ok(())
    }
    async fn metadata(&self) -> fdo::Result<Metadata> {
        Ok(Metadata::default())
    }
    async fn volume(&self) -> fdo::Result<Volume> {
        Ok(Volume::default())
    }
    async fn set_volume(&self, _v: Volume) -> ZbusResult<()> {
        Ok(())
    }
    async fn position(&self) -> fdo::Result<Time> {
        Ok(Time::ZERO)
    }
    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(PlaybackRate::default())
    }
    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(PlaybackRate::default())
    }
    async fn can_go_next(&self) -> fdo::Result<bool> {
        Ok(true)
    }
    async fn can_go_previous(&self) -> fdo::Result<bool> {
        Ok(true)
    }
    async fn can_play(&self) -> fdo::Result<bool> {
        Ok(true)
    }
    async fn can_pause(&self) -> fdo::Result<bool> {
        Ok(true)
    }
    async fn can_seek(&self) -> fdo::Result<bool> {
        Ok(false)
    }
    async fn can_control(&self) -> fdo::Result<bool> {
        Ok(true)
    }
}

pub fn create_metadata(track: &crate::library::Track) -> Metadata {
    let mut builder = Metadata::builder();

    if let Some(title) = &track.metadata.title {
        builder = builder.title(title);
    }

    if let Some(artist) = &track.metadata.artist {
        builder = builder.artist(vec![artist.as_str()]);
    }

    if let Some(album) = &track.metadata.album {
        builder = builder.album(album);
    }

    if let Some(artwork) = &track.metadata.artwork_path {
        // MPRIS requires a URI for artwork, convert file path to file:// URI
        if let Ok(uri) = gtk4::glib::filename_to_uri(artwork, None) {
            builder = builder.art_url(uri.as_str());
        }
    }

    builder.build()
}

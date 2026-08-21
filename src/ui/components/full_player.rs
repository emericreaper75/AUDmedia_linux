use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, Scale};

pub struct FullPlayer {
    pub container: GtkBox,
    pub img_artwork: gtk4::Image,
    pub lbl_title: Label,
    pub lbl_artist: Label,
    pub scale_progress: Scale,
    pub lbl_time: Label,
    pub btn_play_pause: Button,
    pub btn_prev: Button,
    pub btn_next: Button,
    #[allow(dead_code)]
    pub btn_shuffle: Button,
    #[allow(dead_code)]
    pub btn_repeat: Button,
    #[allow(dead_code)]
    pub btn_queue: Button,
    pub btn_close: Button,
}

impl FullPlayer {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(24)
            .margin_start(32)
            .margin_end(32)
            .margin_top(16)
            .margin_bottom(32)
            .css_classes(["background"]) // Solid background for overlay
            .build();

        // Top bar (Close button)
        let top_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Start)
            .build();

        let btn_close = Button::builder()
            .icon_name("go-down-symbolic")
            .css_classes(["flat", "circular"])
            .build();

        top_box.append(&btn_close);

        // Large Artwork
        let img_artwork = gtk4::Image::builder()
            .icon_name("audio-x-generic-symbolic")
            .pixel_size(256)
            .vexpand(true)
            .valign(Align::Center)
            .halign(Align::Center)
            .css_classes(["rounded-corners"])
            .build();

        // Track Info
        let info_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .halign(Align::Center)
            .build();

        let lbl_title = Label::builder()
            .label("No Track")
            .halign(Align::Center)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["title-1"])
            .build();

        let lbl_artist = Label::builder()
            .label("Unknown Artist")
            .halign(Align::Center)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["dim-label", "title-4"])
            .build();

        info_box.append(&lbl_title);
        info_box.append(&lbl_artist);

        // Progress bar and time
        let progress_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .build();

        let scale_progress = Scale::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .draw_value(false)
            .build();

        let time_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .build();

        let lbl_time = Label::builder()
            .label("0:00 / 0:00")
            .css_classes(["dim-label", "numeric"])
            .halign(Align::Center)
            .hexpand(true)
            .build();

        time_box.append(&lbl_time);

        progress_box.append(&scale_progress);
        progress_box.append(&time_box);

        // Main controls row
        let controls_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();

        let btn_prev = Button::builder()
            .icon_name("media-skip-backward-symbolic")
            .css_classes(["flat", "circular", "large"])
            .build();

        let btn_play_pause = Button::builder()
            .icon_name("media-playback-start-symbolic")
            .css_classes(["circular", "suggested-action"])
            .width_request(64)
            .height_request(64)
            .build();

        let btn_next = Button::builder()
            .icon_name("media-skip-forward-symbolic")
            .css_classes(["flat", "circular", "large"])
            .build();

        controls_box.append(&btn_prev);
        controls_box.append(&btn_play_pause);
        controls_box.append(&btn_next);

        // Secondary controls row
        let secondary_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(32)
            .halign(Align::Center)
            .margin_top(16)
            .build();

        let btn_shuffle = Button::builder()
            .icon_name("media-playlist-shuffle-symbolic")
            .css_classes(["flat", "circular"])
            .build();

        let btn_repeat = Button::builder()
            .icon_name("media-playlist-repeat-symbolic")
            .css_classes(["flat", "circular"])
            .build();

        let btn_queue = Button::builder()
            .icon_name("view-list-symbolic")
            .css_classes(["flat", "circular"])
            .build();

        secondary_box.append(&btn_shuffle);
        secondary_box.append(&btn_repeat);
        secondary_box.append(&btn_queue);

        container.append(&top_box);
        container.append(&img_artwork);
        container.append(&info_box);
        container.append(&progress_box);
        container.append(&controls_box);
        container.append(&secondary_box);

        Self {
            container,
            img_artwork,
            lbl_title,
            lbl_artist,
            scale_progress,
            lbl_time,
            btn_play_pause,
            btn_prev,
            btn_next,
            btn_shuffle,
            btn_repeat,
            btn_queue,
            btn_close,
        }
    }
}

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, Scale};

pub struct PlayerControls {
    pub container: GtkBox,
    pub btn_play_pause: Button,
    pub btn_prev: Button,
    pub btn_next: Button,
    pub lbl_title: Label,
    pub lbl_artist: Label,
    pub scale_progress: Scale,
    pub lbl_time: Label,
    pub img_artwork: gtk4::Image,
}

impl PlayerControls {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .css_classes(["background"]) // basic background
            .build();

        // Progress bar and time
        let progress_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_start(12)
            .margin_end(12)
            .margin_top(4)
            .build();

        let scale_progress = Scale::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .draw_value(false)
            .build();

        let lbl_time = Label::builder()
            .label("0:00 / 0:00")
            .css_classes(["dim-label", "numeric"])
            .build();

        progress_box.append(&scale_progress);
        progress_box.append(&lbl_time);

        // Main controls row
        let controls_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_bottom(8)
            .margin_top(4)
            .build();

        // Track Info
        let info_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .valign(Align::Center)
            .build();

        let lbl_title = Label::builder()
            .label("No Track")
            .halign(Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["heading"])
            .build();

        let lbl_artist = Label::builder()
            .label("Unknown Artist")
            .halign(Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["dim-label"])
            .build();

        info_box.append(&lbl_title);
        info_box.append(&lbl_artist);

        let img_artwork = gtk4::Image::builder()
            .icon_name("audio-x-generic-symbolic")
            .pixel_size(48)
            .build();

        let left_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .hexpand(true)
            .valign(Align::Center)
            .build();

        left_box.append(&img_artwork);
        left_box.append(&info_box);

        // Buttons
        let btn_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();

        let btn_prev = Button::builder()
            .icon_name("media-skip-backward-symbolic")
            .css_classes(["flat", "circular"])
            .build();

        let btn_play_pause = Button::builder()
            .icon_name("media-playback-start-symbolic")
            .css_classes(["flat", "circular"])
            .build();

        let btn_next = Button::builder()
            .icon_name("media-skip-forward-symbolic")
            .css_classes(["flat", "circular"])
            .build();

        btn_box.append(&btn_prev);
        btn_box.append(&btn_play_pause);
        btn_box.append(&btn_next);

        // Right spacer to keep buttons centered
        let right_spacer = GtkBox::builder().hexpand(true).build();

        controls_box.append(&left_box);
        controls_box.append(&btn_box);
        controls_box.append(&right_spacer);

        container.append(&progress_box);
        container.append(&controls_box);

        Self {
            container,
            btn_play_pause,
            btn_prev,
            btn_next,
            lbl_title,
            lbl_artist,
            scale_progress,
            lbl_time,
            img_artwork,
        }
    }
}

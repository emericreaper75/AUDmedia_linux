use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, GestureClick, Label, Orientation};

pub struct MiniPlayer {
    pub container: GtkBox,
    pub img_artwork: gtk4::Image,
    pub lbl_title: Label,
    pub lbl_artist: Label,
    pub btn_play_pause: Button,
    pub click_gesture: GestureClick,
}

impl MiniPlayer {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(8)
            .margin_bottom(8)
            .css_classes(["background"]) // basic background
            .build();

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        let img_artwork = gtk4::Image::builder()
            .icon_name("audio-x-generic-symbolic")
            .pixel_size(48)
            .css_classes(["rounded-corners"])
            .build();

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

        let btn_play_pause = Button::builder()
            .icon_name("media-playback-start-symbolic")
            .css_classes(["flat", "circular"])
            .valign(Align::Center)
            .build();

        container.append(&img_artwork);
        container.append(&info_box);
        container.append(&btn_play_pause);

        Self {
            container,
            img_artwork,
            lbl_title,
            lbl_artist,
            btn_play_pause,
            click_gesture,
        }
    }
}

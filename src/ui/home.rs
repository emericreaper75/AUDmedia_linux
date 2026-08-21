use adw::StatusPage;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation};
use libadwaita as adw;

pub struct HomeView {
    pub container: GtkBox,
    pub btn_scan: Button,
}

impl HomeView {
    pub fn new() -> Self {
        let container = GtkBox::builder().orientation(Orientation::Vertical).build();

        let status_page = StatusPage::builder()
            .icon_name("folder-music-symbolic")
            .title("Your library is empty")
            .description("Add a music folder to get started.")
            .vexpand(true)
            .build();

        let btn_scan = Button::builder()
            .label("Add Music Folder")
            .halign(gtk4::Align::Center)
            .css_classes(["pill", "suggested-action"])
            .margin_bottom(32)
            .build();

        let vbox = GtkBox::builder().orientation(Orientation::Vertical).build();

        vbox.append(&status_page);
        vbox.append(&btn_scan);

        container.append(&vbox);

        Self {
            container,
            btn_scan,
        }
    }

    pub fn set_empty(&self, empty: bool) {
        if empty {
            self.container.set_visible(true);
        } else {
            // Ideally we'd show recently added or just hide the empty state.
            // For MVP, we can leave it or update the text.
            let status = self
                .container
                .first_child()
                .unwrap()
                .downcast::<GtkBox>()
                .unwrap()
                .first_child()
                .unwrap()
                .downcast::<StatusPage>()
                .unwrap();
            status.set_title("Welcome back");
            status.set_description(Some("Browse your library below."));
            status.set_icon_name(Some("audio-headphones-symbolic"));
            self.btn_scan.set_visible(false);
        }
    }
}

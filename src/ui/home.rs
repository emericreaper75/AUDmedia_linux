use adw::StatusPage;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation};
use libadwaita as adw;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeState {
    NoFolder,
    Scanning,
    EmptyFolder,
    ScanFailed(String),
    LibraryLoaded(usize),
}

pub struct HomeView {
    pub container: GtkBox,
    pub status_page: StatusPage,
    pub btn_scan: Button,   // Add / Change Folder button
    pub btn_rescan: Button, // Rescan button
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
            .margin_bottom(16)
            .build();

        let btn_rescan = Button::builder()
            .label("Rescan Folder")
            .halign(gtk4::Align::Center)
            .css_classes(["pill"])
            .margin_bottom(16)
            .visible(false)
            .build();

        let button_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .halign(gtk4::Align::Center)
            .margin_bottom(32)
            .build();

        button_box.append(&btn_scan);
        button_box.append(&btn_rescan);

        let vbox = GtkBox::builder().orientation(Orientation::Vertical).build();

        vbox.append(&status_page);
        vbox.append(&button_box);

        container.append(&vbox);

        let view = Self {
            container,
            status_page,
            btn_scan,
            btn_rescan,
        };

        view.set_state(HomeState::NoFolder);
        view
    }

    pub fn set_state(&self, state: HomeState) {
        match state {
            HomeState::NoFolder => {
                self.status_page.set_title("Your library is empty");
                self.status_page
                    .set_description(Some("Add a music folder to get started."));
                self.status_page
                    .set_icon_name(Some("folder-music-symbolic"));
                self.btn_scan.set_label("Add Music Folder");
                self.btn_scan.set_visible(true);
                self.btn_scan.set_sensitive(true);
                self.btn_rescan.set_visible(false);
            }
            HomeState::Scanning => {
                self.status_page.set_title("Scanning...");
                self.status_page
                    .set_description(Some("Please wait while we index your music."));
                self.status_page
                    .set_icon_name(Some("emblem-synchronizing-symbolic"));
                self.btn_scan.set_sensitive(false);
                self.btn_rescan.set_sensitive(false);
            }
            HomeState::EmptyFolder => {
                self.status_page.set_title("No music found");
                self.status_page.set_description(Some(
                    "The selected folder contains no playable audio files.",
                ));
                self.status_page
                    .set_icon_name(Some("dialog-warning-symbolic"));
                self.btn_scan.set_label("Choose Another Folder");
                self.btn_scan.set_visible(true);
                self.btn_scan.set_sensitive(true);
                self.btn_rescan.set_label("Rescan Folder");
                self.btn_rescan.set_visible(true);
                self.btn_rescan.set_sensitive(true);
            }
            HomeState::ScanFailed(ref err) => {
                self.status_page.set_title("Scan failed");
                self.status_page.set_description(Some(err.as_str()));
                self.status_page
                    .set_icon_name(Some("dialog-error-symbolic"));
                self.btn_scan.set_label("Choose Another Folder");
                self.btn_scan.set_visible(true);
                self.btn_scan.set_sensitive(true);
                self.btn_rescan.set_label("Retry Scan");
                self.btn_rescan.set_visible(true);
                self.btn_rescan.set_sensitive(true);
            }
            HomeState::LibraryLoaded(count) => {
                self.status_page.set_title("Welcome back");
                let desc = if count == 1 {
                    "Showing 1 song in your library.".to_string()
                } else {
                    format!("Showing {} songs in your library.", count)
                };
                self.status_page.set_description(Some(&desc));
                self.status_page
                    .set_icon_name(Some("audio-headphones-symbolic"));
                self.btn_scan.set_label("Change Folder");
                self.btn_scan.set_visible(true);
                self.btn_scan.set_sensitive(true);
                self.btn_rescan.set_label("Rescan Folder");
                self.btn_rescan.set_visible(true);
                self.btn_rescan.set_sensitive(true);
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_empty(&self, empty: bool) {
        if empty {
            self.set_state(HomeState::NoFolder);
        } else {
            self.set_state(HomeState::LibraryLoaded(0));
        }
    }
}

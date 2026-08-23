use crate::ui::components::songs::SongsView;
use adw::StatusPage;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation, SearchEntry};
use libadwaita as adw;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchState {
    EmptyQuery,
    NoResults,
    EmptyLibrary,
    Results(usize),
}

pub struct SearchView {
    pub container: GtkBox,
    pub search_entry: SearchEntry,
    pub status_page: StatusPage,
    pub btn_scan: Button,
    pub songs_view: SongsView,
}

impl SearchView {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_start(16)
            .margin_end(16)
            .margin_top(16)
            .margin_bottom(16)
            .build();

        let search_entry = SearchEntry::builder()
            .placeholder_text("Search songs, artists, or albums...")
            .margin_bottom(8)
            .build();

        let status_page = StatusPage::builder()
            .icon_name("system-search-symbolic")
            .title("Search your library")
            .description("Find songs, artists, or albums.")
            .vexpand(true)
            .build();

        let btn_scan = Button::builder()
            .label("Add Music Folder")
            .halign(gtk4::Align::Center)
            .css_classes(["pill", "suggested-action"])
            .margin_bottom(32)
            .visible(false)
            .build();

        let songs_view = SongsView::new();
        songs_view.container.set_vexpand(true);
        songs_view.container.set_visible(false);

        container.append(&search_entry);
        container.append(&status_page);
        container.append(&btn_scan);
        container.append(&songs_view.container);

        let view = Self {
            container,
            search_entry,
            status_page,
            btn_scan,
            songs_view,
        };

        view.set_state(SearchState::EmptyQuery);
        view
    }

    pub fn set_state(&self, state: SearchState) {
        match state {
            SearchState::EmptyQuery => {
                self.status_page.set_title("Search your library");
                self.status_page
                    .set_description(Some("Find songs, artists, or albums."));
                self.status_page
                    .set_icon_name(Some("system-search-symbolic"));
                self.status_page.set_visible(true);
                self.btn_scan.set_visible(false);
                self.songs_view.container.set_visible(false);
            }
            SearchState::NoResults => {
                self.status_page.set_title("No results");
                self.status_page
                    .set_description(Some("Try a different song, artist, or album."));
                self.status_page
                    .set_icon_name(Some("system-search-symbolic"));
                self.status_page.set_visible(true);
                self.btn_scan.set_visible(false);
                self.songs_view.container.set_visible(false);
            }
            SearchState::EmptyLibrary => {
                self.status_page.set_title("Your library is empty");
                self.status_page
                    .set_description(Some("Add a music folder to get started."));
                self.status_page
                    .set_icon_name(Some("folder-music-symbolic"));
                self.status_page.set_visible(true);
                self.btn_scan.set_label("Add Music Folder");
                self.btn_scan.set_visible(true);
                self.songs_view.container.set_visible(false);
            }
            SearchState::Results(_) => {
                self.status_page.set_visible(false);
                self.btn_scan.set_visible(false);
                self.songs_view.container.set_visible(true);
            }
        }
    }
}

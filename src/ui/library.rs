use crate::ui::components::{albums::AlbumsView, artists::ArtistsView, songs::SongsView};
use adw::{ViewStack, ViewSwitcher};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation};
use libadwaita as adw;

pub struct LibraryView {
    pub container: GtkBox,
    pub songs_view: SongsView,
    pub albums_view: AlbumsView,
    pub artists_view: ArtistsView,
}

impl LibraryView {
    pub fn new() -> Self {
        let container = GtkBox::builder().orientation(Orientation::Vertical).build();

        let stack = ViewStack::new();
        stack.set_vexpand(true);

        let switcher = ViewSwitcher::builder()
            .stack(&stack)
            .margin_top(12)
            .margin_bottom(12)
            .halign(gtk4::Align::Center)
            .build();

        let songs_view = SongsView::new();
        let albums_view = AlbumsView::new();
        let artists_view = ArtistsView::new();

        let songs_page = stack.add_titled(&songs_view.container, Some("songs"), "Songs");
        let albums_page = stack.add_titled(&albums_view.container, Some("albums"), "Albums");
        let artists_page = stack.add_titled(&artists_view.container, Some("artists"), "Artists");

        songs_page.set_icon_name(Some("audio-x-generic-symbolic"));
        albums_page.set_icon_name(Some("media-optical-symbolic"));
        artists_page.set_icon_name(Some("avatar-default-symbolic"));

        container.append(&switcher);
        container.append(&stack);

        Self {
            container,
            songs_view,
            albums_view,
            artists_view,
        }
    }
}

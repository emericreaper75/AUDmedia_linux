//! Main application window implementation.

use adw::prelude::*;
use libadwaita as adw;

/// Builds and displays the main GTK4 / libadwaita application window.
pub fn build_ui(app: &adw::Application) {
    let header_bar = adw::HeaderBar::new();

    let label = gtk4::Label::builder()
        .label("AUDmedia Music Player")
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let status_page = adw::StatusPage::builder()
        .title("AUDmedia")
        .description("Lightweight Linux Music Player")
        .icon_name("audio-x-generic-symbolic")
        .child(&label)
        .build();

    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content.append(&header_bar);
    content.append(&status_page);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("AUDmedia")
        .default_width(420)
        .default_height(720)
        .content(&content)
        .build();

    window.present();
}

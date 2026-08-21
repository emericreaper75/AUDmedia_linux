//! AUDmedia Linux Music Player entry point.

mod app;
mod library;
mod metadata;
mod player;
mod queue;
mod ui;
pub mod config;
pub mod mpris;

use libadwaita as adw;
use libadwaita::glib;
use libadwaita::prelude::*;

const APP_ID: &str = "org.audmedia.AUDmedia";

fn main() -> glib::ExitCode {
    // Initialize GStreamer
    if let Err(err) = gstreamer::init() {
        eprintln!("Failed to initialize GStreamer: {err}");
        return glib::ExitCode::FAILURE;
    }

    // Initialize GTK4 / libadwaita application
    let app = adw::Application::builder().application_id(APP_ID).build();

    let app_state = app::AppState::new().expect("Failed to initialize application state");
    let state = std::rc::Rc::new(std::cell::RefCell::new(app_state));

    app.connect_activate(move |app| {
        ui::window::build_ui(app, state.clone());
    });

    app.run()
}

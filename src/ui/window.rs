//! Main application window implementation.

use crate::app::AppState;
use adw::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation};
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

/// Builds and displays the main GTK4 / libadwaita application window.
pub fn build_ui(app: &adw::Application, state: Rc<RefCell<AppState>>) {
    let header_bar = adw::HeaderBar::new();

    let label = Label::builder()
        .label("No file selected")
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let btn_select = Button::builder().label("Select File").build();
    let btn_play = Button::builder().label("Play / Pause").build();
    let btn_stop = Button::builder().label("Stop").build();

    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(gtk4::Align::Center)
        .margin_bottom(24)
        .build();

    button_box.append(&btn_select);
    button_box.append(&btn_play);
    button_box.append(&btn_stop);

    let content = GtkBox::new(Orientation::Vertical, 12);
    content.append(&header_bar);
    content.append(&label);
    content.append(&button_box);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("AUDmedia")
        .default_width(420)
        .default_height(720)
        .content(&content)
        .build();

    let window_clone = window.clone();
    let label_clone = label.clone();
    let state_select = state.clone();

    btn_select.connect_clicked(move |_| {
        let dialog = gtk4::FileDialog::new();
        let label_inner = label_clone.clone();
        let state_inner = state_select.clone();
        
        dialog.open(Some(&window_clone), gtk4::gio::Cancellable::NONE, move |result| {
            if let Ok(file) = result {
                let uri = file.uri();
                let basename = file
                    .basename()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                label_inner.set_label(&format!("Loaded: {}", basename));
                
                if let Err(e) = state_inner.borrow().player.play_file(uri.as_str()) {
                    eprintln!("Playback error: {}", e);
                    label_inner.set_label(&format!("Error: {}", e));
                }
            }
        });
    });

    let state_play = state.clone();
    btn_play.connect_clicked(move |_| {
        let player = &state_play.borrow().player;
        if player.is_playing() {
            player.pause();
        } else {
            player.resume();
        }
    });

    let state_stop = state.clone();
    btn_stop.connect_clicked(move |_| {
        let player = &state_stop.borrow().player;
        player.stop();
    });

    window.present();
}

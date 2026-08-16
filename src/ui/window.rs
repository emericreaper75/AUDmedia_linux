use crate::app::AppState;
use crate::ui::home::HomeView;
use crate::ui::library::LibraryView;
use crate::ui::models::{ItemObject, TrackObject};
use crate::ui::components::player_controls::PlayerControls;
use adw::prelude::*;
use adw::{HeaderBar, ViewStack, ViewSwitcherBar};
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// Builds and displays the main GTK4 / libadwaita application window.
pub fn build_ui(app: &adw::Application, state: Rc<RefCell<AppState>>) {
    let toolbar_view = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let header_bar = HeaderBar::new();
    toolbar_view.append(&header_bar);

    let stack = ViewStack::new();
    let home_view_rc = Rc::new(HomeView::new());
    let library_view_rc = Rc::new(LibraryView::new());

    let home_page = stack.add_titled(&home_view_rc.container, Some("home"), "Home");
    home_page.set_icon_name(Some("go-home-symbolic"));

    let library_page = stack.add_titled(&library_view_rc.container, Some("library"), "Library");
    library_page.set_icon_name(Some("folder-music-symbolic"));

    toolbar_view.append(&stack);

    let switcher_bar = ViewSwitcherBar::builder()
        .stack(&stack)
        .reveal(true)
        .build();
    
    let player_controls = Rc::new(PlayerControls::new());
    toolbar_view.append(&player_controls.container);

    toolbar_view.append(&switcher_bar);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("AUDmedia")
        .default_width(420)
        .default_height(720)
        .content(&toolbar_view)
        .build();

    
    // Player Controls Wiring
    let controls_rc = player_controls.clone();
    
    // Selection changed -> Play song
    let state_for_selection = state.clone();
    let controls_for_selection = controls_rc.clone();
    library_view_rc.songs_view.selection_model.connect_selection_changed(move |model, _, _| {
        if let Some(item) = model.selected_item() {
            if let Ok(track_obj) = item.downcast::<TrackObject>() {
                let track = track_obj.get_track();
                let idx = model.selected();
                
                let uri = glib::filename_to_uri(&track.path, None).unwrap();
                
                let mut app_state = state_for_selection.borrow_mut();
                app_state.current_track_index = Some(idx as usize);
                
                if let Err(e) = app_state.player.play_file(uri.as_str()) {
                    eprintln!("Failed to play: {}", e);
                } else {
                    controls_for_selection.lbl_title.set_label(track.metadata.title.as_deref().unwrap_or("Unknown"));
                    controls_for_selection.lbl_artist.set_label(track.metadata.artist.as_deref().unwrap_or("Unknown Artist"));
                    controls_for_selection.btn_play_pause.set_icon_name("media-playback-pause-symbolic");
                }
            }
        }
    });

    // Play/Pause button
    let state_for_play = state.clone();
    let controls_for_play = controls_rc.clone();
    controls_rc.btn_play_pause.connect_clicked(move |btn| {
        let app_state = state_for_play.borrow();
        if app_state.player.is_playing() {
            app_state.player.pause();
            btn.set_icon_name("media-playback-start-symbolic");
        } else {
            if app_state.current_track_index.is_some() {
                app_state.player.resume();
                btn.set_icon_name("media-playback-pause-symbolic");
            }
        }
    });

    // Previous button
    let state_for_prev = state.clone();
    let lib_view_for_prev = library_view_rc.clone();
    controls_rc.btn_prev.connect_clicked(move |_| {
        let app_state = state_for_prev.borrow();
        if let Some(mut idx) = app_state.current_track_index {
            if idx > 0 {
                idx -= 1;
                lib_view_for_prev.songs_view.selection_model.select_item(idx as u32, true);
            }
        }
    });

    // Next button
    let state_for_next = state.clone();
    let lib_view_for_next = library_view_rc.clone();
    controls_rc.btn_next.connect_clicked(move |_| {
        let app_state = state_for_next.borrow();
        if let Some(mut idx) = app_state.current_track_index {
            let total = app_state.library.tracks.len();
            if idx + 1 < total {
                idx += 1;
                lib_view_for_next.songs_view.selection_model.select_item(idx as u32, true);
            }
        }
    });

    // Progress update timer
    let state_for_timer = state.clone();
    let controls_for_timer = controls_rc.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let app_state = state_for_timer.borrow();
        if app_state.player.is_playing() {
            if let (Some(pos), Some(dur)) = (app_state.player.position(), app_state.player.duration()) {
                if dur > 0 {
                    controls_for_timer.scale_progress.set_range(0.0, dur as f64);
                    controls_for_timer.scale_progress.set_value(pos as f64);

                    let pos_sec = pos / 1_000_000_000;
                    let dur_sec = dur / 1_000_000_000;
                    let pos_str = format!("{}:{:02}", pos_sec / 60, pos_sec % 60);
                    let dur_str = format!("{}:{:02}", dur_sec / 60, dur_sec % 60);
                    controls_for_timer.lbl_time.set_label(&format!("{} / {}", pos_str, dur_str));
                }
            }
        } else {
            // Also update if we reached EOS (queue support placeholder)
            // But for now, we just let it be.
        }
        glib::ControlFlow::Continue
    });

    // Seek interaction
    let state_for_seek = state.clone();
    controls_rc.scale_progress.connect_value_changed(move |scale| {
        let app_state = state_for_seek.borrow();
        let target_ns = scale.value() as u64;
        
        if let Some(pos) = app_state.player.position() {
            let diff = if target_ns > pos { target_ns - pos } else { pos - target_ns };
            if diff > 500_000_000 { // 500ms
                if let Err(e) = app_state.player.seek(target_ns) {
                    eprintln!("Seek error: {}", e);
                }
            }
        }
    });


    // Setup scan button
    let window_clone = window.clone();
    let state_clone = state.clone();

    let home_view_clone = home_view_rc.clone();
    let library_view_clone = library_view_rc.clone();

    home_view_rc.btn_scan.connect_clicked(move |_| {
        let dialog = gtk4::FileDialog::new();
        let window_weak = window_clone.downgrade();
        let state_weak = Rc::downgrade(&state_clone);
        let home_weak = Rc::downgrade(&home_view_clone);
        let lib_weak = Rc::downgrade(&library_view_clone);

        dialog.select_folder(
            Some(&window_clone),
            gtk4::gio::Cancellable::NONE,
            move |result| {
                if let Ok(folder) = result {
                    if let Some(path) = folder.path() {
                        let home = if let Some(h) = home_weak.upgrade() {
                            h
                        } else {
                            return;
                        };
                        let state_rc = if let Some(s) = state_weak.upgrade() {
                            s
                        } else {
                            return;
                        };
                        let lib_rc = if let Some(l) = lib_weak.upgrade() {
                            l
                        } else {
                            return;
                        };

                        // Temporarily change the empty state text to show loading
                        let status = home
                            .container
                            .first_child()
                            .unwrap()
                            .downcast::<gtk4::Box>()
                            .unwrap()
                            .first_child()
                            .unwrap()
                            .downcast::<adw::StatusPage>()
                            .unwrap();
                        status.set_title("Scanning...");
                        status.set_description(Some("Please wait while we index your music."));
                        home.btn_scan.set_sensitive(false);

                        let result_lib = std::sync::Arc::new(std::sync::Mutex::new(None));
                        let result_clone = result_lib.clone();

                        std::thread::spawn(move || {
                            let lib = crate::library::Library::scan_directory(path, |_| {});
                            *result_clone.lock().unwrap() = Some(lib);
                        });

                        gtk4::glib::timeout_add_local(
                            std::time::Duration::from_millis(100),
                            move || {
                                if let Ok(mut lock) = result_lib.try_lock() {
                                    if let Some(lib) = lock.take() {
                                        state_rc.borrow_mut().library = lib;

                                        // Update UI models
                                        let library = &state_rc.borrow().library;

                                        lib_rc.songs_view.store.remove_all();
                                        lib_rc.albums_view.store.remove_all();
                                        lib_rc.artists_view.store.remove_all();

                                        let mut albums = HashSet::new();
                                        let mut artists = HashSet::new();

                                        for track in &library.tracks {
                                            lib_rc
                                                .songs_view
                                                .store
                                                .append(&TrackObject::new(track.clone()));

                                            let album = track
                                                .metadata
                                                .album
                                                .clone()
                                                .unwrap_or_else(|| "Unknown Album".to_string());
                                            let artist =
                                                track.metadata.artist.clone().unwrap_or_else(
                                                    || "Unknown Artist".to_string(),
                                                );

                                            if albums.insert(album.clone()) {
                                                lib_rc
                                                    .albums_view
                                                    .store
                                                    .append(&ItemObject::new(&album, &artist));
                                            }

                                            if artists.insert(artist.clone()) {
                                                lib_rc
                                                    .artists_view
                                                    .store
                                                    .append(&ItemObject::new(&artist, ""));
                                            }
                                        }

                                        home.set_empty(false);
                                        home.btn_scan.set_sensitive(true);

                                        return gtk4::glib::ControlFlow::Break;
                                    }
                                }

                                gtk4::glib::ControlFlow::Continue
                            },
                        );
                    }
                }
            },
        );
    });

    window.present();
}

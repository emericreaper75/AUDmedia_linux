use crate::app::AppState;
use crate::ui::components::mini_player::MiniPlayer;
use crate::ui::components::full_player::FullPlayer;
use crate::ui::home::HomeView;
use crate::ui::library::LibraryView;
use crate::ui::models::{ItemObject, TrackObject};
use crate::ui::queue::QueueView;
use adw::prelude::*;
use gtk4::EventControllerKey;
use adw::{HeaderBar, ViewStack, ViewSwitcherBar};
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

pub fn build_ui(app: &adw::Application, state: Rc<RefCell<AppState>>) {
    let toolbar_view = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let header_bar = HeaderBar::new();
    toolbar_view.append(&header_bar);

    let stack = ViewStack::new();
    let home_view_rc = Rc::new(HomeView::new());
    let library_view_rc = Rc::new(LibraryView::new());
    let queue_view_rc = Rc::new(QueueView::new());

    let home_page = stack.add_titled(&home_view_rc.container, Some("home"), "Home");
    home_page.set_icon_name(Some("go-home-symbolic"));

    let library_page = stack.add_titled(&library_view_rc.container, Some("library"), "Library");
    library_page.set_icon_name(Some("folder-music-symbolic"));

    let queue_page = stack.add_titled(&queue_view_rc.container, Some("queue"), "Queue");
    queue_page.set_icon_name(Some("view-list-symbolic"));

    toolbar_view.append(&stack);

    let switcher_bar = ViewSwitcherBar::builder()
        .stack(&stack)
        .reveal(true)
        .build();

    let mini_player = Rc::new(MiniPlayer::new());
    let full_player = Rc::new(FullPlayer::new());

    toolbar_view.append(&mini_player.container);
    toolbar_view.append(&switcher_bar);


    // Initialize MPRIS server
    let (mpris_sender, mpris_receiver) = async_channel::unbounded();
    let state_for_mpris = state.clone();
    
    glib::MainContext::default().spawn_local(async move {
        let player = crate::mpris::MprisPlayer::new(mpris_sender);
        if let Ok(server) = mpris_server::Server::new("audmedia", player).await {
            state_for_mpris.borrow_mut().mpris = Some(Rc::new(server));
        } else {
            eprintln!("Failed to initialize MPRIS server. Continuing without it.");
        }
    });

    // Overlay to handle full player
    let overlay = gtk4::Overlay::new();
    overlay.set_child(Some(&toolbar_view));
    
    // Set full player invisible initially
    full_player.container.set_visible(false);
    
    // Ensure full player covers the entire window when visible
    // We add it to the overlay
    overlay.add_overlay(&full_player.container);

    let (width, height, maximized) = {
        let c = &state.borrow().config;
        (c.window_width, c.window_height, c.window_maximized)
    };

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("AUDmedia")
        .default_width(width)
        .default_height(height)
        .maximized(maximized)
        .content(&overlay)
        .build();

    let state_for_close = state.clone();
    window.connect_close_request(move |win| {
        let mut app_state = state_for_close.borrow_mut();
        app_state.config.window_width = win.width();
        app_state.config.window_height = win.height();
        app_state.config.window_maximized = win.is_maximized();
        
        if let Some(track) = app_state.queue.current() {
            app_state.config.last_played_track = Some(track.path.to_string_lossy().to_string());
        }
        app_state.config.last_playback_position = app_state.player.position();
        
        app_state.config.save();
        glib::Propagation::Proceed
    });

    // MiniPlayer click -> Open FullPlayer
    let fp_for_open = full_player.container.clone();
    mini_player.click_gesture.connect_pressed(move |_, _, _, _| {
        fp_for_open.set_visible(true);
    });

    // Close button -> Hide FullPlayer
    let fp_for_close = full_player.container.clone();
    full_player.btn_close.connect_clicked(move |_| {
        fp_for_close.set_visible(false);
    });

    // Helper to update both players
    let update_players = {
        let mini = mini_player.clone();
        let full = full_player.clone();
        let state_for_update = state.clone();
        move |track: &crate::library::Track| {
            let title = track.metadata.title.clone().unwrap_or_else(|| "Unknown".to_string());
            let artist = track.metadata.artist.clone().unwrap_or_else(|| "Unknown Artist".to_string());
            let artwork_path = track.metadata.artwork_path.as_ref();
            
            mini.lbl_title.set_label(&title);
            mini.lbl_artist.set_label(&artist);
            full.lbl_title.set_label(&title);
            full.lbl_artist.set_label(&artist);

            if let Some(path) = artwork_path {
                mini.img_artwork.set_from_file(Some(path));
                full.img_artwork.set_from_file(Some(path));
            } else {
                mini.img_artwork.set_icon_name(Some("audio-x-generic-symbolic"));
                full.img_artwork.set_icon_name(Some("audio-x-generic-symbolic"));
            }
            
            // Push to MPRIS
            if let Some(mpris) = &state_for_update.borrow().mpris {
                let server = mpris.clone();
                let metadata = crate::mpris::create_metadata(track);
                glib::MainContext::default().spawn_local(async move {
                    let _ = server.properties_changed([
                        mpris_server::Property::Metadata(metadata),
                        mpris_server::Property::PlaybackStatus(mpris_server::PlaybackStatus::Playing)
                    ]).await;
                });
            }
        }
    };

    let update_play_btn = {
        let mini = mini_player.clone();
        let full = full_player.clone();
        let state_for_update = state.clone();
        move |icon: &str| {
            mini.btn_play_pause.set_icon_name(icon);
            full.btn_play_pause.set_icon_name(icon);
            
            let status = if icon == "media-playback-pause-symbolic" {
                mpris_server::PlaybackStatus::Playing
            } else if icon == "media-playback-start-symbolic" {
                mpris_server::PlaybackStatus::Paused
            } else {
                mpris_server::PlaybackStatus::Stopped
            };
            
            if let Some(mpris) = &state_for_update.borrow().mpris {
                let server = mpris.clone();
                glib::MainContext::default().spawn_local(async move {
                    let _ = server.properties_changed([
                        mpris_server::Property::PlaybackStatus(status)
                    ]).await;
                });
            }
        }
    };

    // Auto-continue logic (EOS)
    let state_for_eos = state.clone();
    let queue_view_for_eos = queue_view_rc.clone();
    let update_players_eos = update_players.clone();
    let update_play_btn_eos = update_play_btn.clone();

    state.borrow().player.set_eos_callback(move || {
        let mut app_state = state_for_eos.borrow_mut();
        if let Some(track) = app_state.queue.next().cloned() {
            let uri = glib::filename_to_uri(&track.path, None).unwrap();

            if let Err(e) = app_state.player.play_file(uri.as_str()) {
                eprintln!("Auto-continue failed: {}", e);
            } else {
                update_players_eos(&track);

                if let Some(idx) = app_state.queue.current_index() {
                    queue_view_for_eos.selection_model.select_item(idx as u32, true);
                }
            }
        } else {
            update_play_btn_eos("media-playback-start-symbolic");
        }
    });

    // Selection changed -> Play song
    let state_for_selection = state.clone();
    let queue_view_for_selection = queue_view_rc.clone();
    let update_players_sel = update_players.clone();
    let update_play_btn_sel = update_play_btn.clone();

    library_view_rc.songs_view.selection_model.connect_selection_changed(move |model, _, _| {
        if let Some(item) = model.selected_item() {
            if let Ok(track_obj) = item.downcast::<TrackObject>() {
                let track = track_obj.get_track();
                let mut app_state = state_for_selection.borrow_mut();

                app_state.queue.clear();
                queue_view_for_selection.store.remove_all();

                app_state.queue.add_track(track.clone());
                queue_view_for_selection.store.append(&TrackObject::new(track.clone()));
                queue_view_for_selection.selection_model.select_item(0, true);

                let uri = glib::filename_to_uri(&track.path, None).unwrap();

                if let Err(e) = app_state.player.play_file(uri.as_str()) {
                    eprintln!("Failed to play: {}", e);
                } else {
                    update_players_sel(&track);
                    update_play_btn_sel("media-playback-pause-symbolic");
                }
            }
        }
    });

    // Play/Pause button setup
    let state_for_play = state.clone();
    let update_play_btn_pp = update_play_btn.clone();
    let toggle_play = move || {
        let app_state = state_for_play.borrow();
        if app_state.player.is_playing() {
            app_state.player.pause();
            update_play_btn_pp("media-playback-start-symbolic");
        } else {
            if app_state.queue.current().is_some() {
                app_state.player.resume();
                update_play_btn_pp("media-playback-pause-symbolic");
            }
        }
    };

    let toggle_play_clone = toggle_play.clone();
    mini_player.btn_play_pause.connect_clicked(move |_| toggle_play_clone());
    let toggle_play_full = toggle_play.clone();
    full_player.btn_play_pause.connect_clicked(move |_| toggle_play_full());

    let action_prev = {
        let state_rc = state.clone();
        let queue_view = queue_view_rc.clone();
        let update_p = update_players.clone();
        let update_btn = update_play_btn.clone();
        move || {
            let mut app_state = state_rc.borrow_mut();
            if let Some(track) = app_state.queue.previous().cloned() {
                let uri = glib::filename_to_uri(&track.path, None).unwrap();
                if let Err(e) = app_state.player.play_file(uri.as_str()) {
                    eprintln!("Failed to play: {}", e);
                } else {
                    update_p(&track);
                    update_btn("media-playback-pause-symbolic");

                    if let Some(idx) = app_state.queue.current_index() {
                        queue_view.selection_model.select_item(idx as u32, true);
                    }
                }
            }
        }
    };

    // Previous button
    let action_prev_btn = action_prev.clone();
    full_player.btn_prev.connect_clicked(move |_| action_prev_btn());

    let action_next = {
        let state_rc = state.clone();
        let queue_view = queue_view_rc.clone();
        let update_p = update_players.clone();
        let update_btn = update_play_btn.clone();
        move || {
            let mut app_state = state_rc.borrow_mut();
            if let Some(track) = app_state.queue.next().cloned() {
                let uri = glib::filename_to_uri(&track.path, None).unwrap();
                if let Err(e) = app_state.player.play_file(uri.as_str()) {
                    eprintln!("Failed to play: {}", e);
                } else {
                    update_p(&track);
                    update_btn("media-playback-pause-symbolic");

                    if let Some(idx) = app_state.queue.current_index() {
                        queue_view.selection_model.select_item(idx as u32, true);
                    }
                }
            }
        }
    };

    // Next button
    let action_next_btn = action_next.clone();
    full_player.btn_next.connect_clicked(move |_| action_next_btn());

    // Progress update timer
    let state_for_timer = state.clone();
    let full_player_timer = full_player.clone();
    
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let app_state = state_for_timer.borrow();
        if app_state.player.is_playing() {
            if let (Some(pos), Some(dur)) = (app_state.player.position(), app_state.player.duration()) {
                if dur > 0 {
                    full_player_timer.scale_progress.set_range(0.0, dur as f64);
                    full_player_timer.scale_progress.set_value(pos as f64);

                    let pos_sec = pos / 1_000_000_000;
                    let dur_sec = dur / 1_000_000_000;
                    let pos_str = format!("{}:{:02}", pos_sec / 60, pos_sec % 60);
                    let dur_str = format!("{}:{:02}", dur_sec / 60, dur_sec % 60);
                    full_player_timer.lbl_time.set_label(&format!("{} / {}", pos_str, dur_str));
                }
            }
        }
        glib::ControlFlow::Continue
    });

    // Seek interaction
    let state_for_seek = state.clone();
    full_player.scale_progress.connect_value_changed(move |scale| {
        let app_state = state_for_seek.borrow();
        let target_ns = scale.value() as u64;

        if let Some(pos) = app_state.player.position() {
            let diff = if target_ns > pos { target_ns - pos } else { pos - target_ns };
            if diff > 500_000_000 {
                if let Err(e) = app_state.player.seek(target_ns) {
                    eprintln!("Seek error: {}", e);
                }
            }
        }
    });

    // Refactored folder scanning function
    let scan_folder = move |path: PathBuf,
                            home: Rc<HomeView>,
                            state_rc: Rc<RefCell<AppState>>,
                            lib_rc: Rc<LibraryView>| {
        let status = home.container.first_child().unwrap()
            .downcast::<gtk4::Box>().unwrap()
            .first_child().unwrap()
            .downcast::<adw::StatusPage>().unwrap();
        status.set_title("Scanning...");
        status.set_description(Some("Please wait while we index your music."));
        home.btn_scan.set_sensitive(false);

        let result_lib = std::sync::Arc::new(std::sync::Mutex::new(None));
        let result_clone = result_lib.clone();

        std::thread::spawn(move || {
            let cache_dir = gtk4::glib::user_cache_dir().join("audmedia_linux").join("artwork");
            let lib = crate::library::Library::scan_directory(path, cache_dir, |_| {});
            *result_clone.lock().unwrap() = Some(lib);
        });

        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(mut lock) = result_lib.try_lock() {
                if let Some(lib) = lock.take() {
                    state_rc.borrow_mut().library = lib;

                    let library = &state_rc.borrow().library;
                    lib_rc.songs_view.store.remove_all();
                    lib_rc.albums_view.store.remove_all();
                    lib_rc.artists_view.store.remove_all();

                    let mut albums = HashSet::new();
                    let mut artists = HashSet::new();

                    let mut song_objs = Vec::new();
                    let mut album_objs = Vec::new();
                    let mut artist_objs = Vec::new();

                    for track in &library.tracks {
                        song_objs.push(crate::ui::models::TrackObject::new(track.clone()).upcast::<gtk4::glib::Object>());

                        let album = track.metadata.album.clone().unwrap_or_else(|| "Unknown Album".to_string());
                        let artist = track.metadata.artist.clone().unwrap_or_else(|| "Unknown Artist".to_string());

                        if albums.insert(album.clone()) {
                            album_objs.push(crate::ui::models::ItemObject::new(&album, &artist, track.metadata.artwork_path.clone()).upcast::<gtk4::glib::Object>());
                        }

                        if artists.insert(artist.clone()) {
                            artist_objs.push(crate::ui::models::ItemObject::new(&artist, "", track.metadata.artwork_path.clone()).upcast::<gtk4::glib::Object>());
                        }
                    }
                    
                    lib_rc.songs_view.store.splice(lib_rc.songs_view.store.n_items(), 0, &song_objs);
                    lib_rc.albums_view.store.splice(lib_rc.albums_view.store.n_items(), 0, &album_objs);
                    lib_rc.artists_view.store.splice(lib_rc.artists_view.store.n_items(), 0, &artist_objs);

                    home.set_empty(false);
                    home.btn_scan.set_sensitive(true);

                    return gtk4::glib::ControlFlow::Break;
                }
            }
            gtk4::glib::ControlFlow::Continue
        });
    };

    // Auto-scan on startup if we have folders configured
    let state_for_startup = state.clone();
    let home_view_startup = home_view_rc.clone();
    let library_view_startup = library_view_rc.clone();
    let scan_folder_clone = scan_folder.clone();
    let folder_to_scan = state_for_startup.borrow().config.music_folders.first().cloned();
    if let Some(first_folder) = folder_to_scan {
        scan_folder_clone(
            PathBuf::from(first_folder),
            home_view_startup,
            state_for_startup,
            library_view_startup,
        );
    }

    // Setup scan button for manual scanning
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
        let scan_folder_manual = scan_folder.clone();

        dialog.select_folder(
            Some(&window_clone),
            gtk4::gio::Cancellable::NONE,
            move |result| {
                if let Ok(folder) = result {
                    if let Some(path) = folder.path() {
                        let home = if let Some(h) = home_weak.upgrade() { h } else { return; };
                        let state_rc = if let Some(s) = state_weak.upgrade() { s } else { return; };
                        let lib_rc = if let Some(l) = lib_weak.upgrade() { l } else { return; };
                        
                        // Update config with the selected folder
                        state_rc.borrow_mut().config.music_folders = vec![path.to_string_lossy().to_string()];
                        state_rc.borrow().config.save();

                        scan_folder_manual(path, home, state_rc, lib_rc);
                    }
                }
            },
        );
    });


    // MPRIS receiver
    let action_toggle_mpris = toggle_play.clone();
    let action_next_mpris = action_next.clone();
    let action_prev_mpris = action_prev.clone();
    
    glib::MainContext::default().spawn_local(async move {
        while let Ok(cmd) = mpris_receiver.recv().await {
            match cmd {
                crate::mpris::MprisCommand::PlayPause => action_toggle_mpris(),
                crate::mpris::MprisCommand::Play => action_toggle_mpris(),
                crate::mpris::MprisCommand::Pause => action_toggle_mpris(),
                crate::mpris::MprisCommand::Next => action_next_mpris(),
                crate::mpris::MprisCommand::Previous => action_prev_mpris(),
            }
        }
    });

    // Keyboard Shortcuts
    let key_controller = EventControllerKey::new();
    let state_for_key = state.clone();
    let action_toggle_key = toggle_play.clone();
    let action_next_key = action_next.clone();
    let action_prev_key = action_prev.clone();
    let fp_for_esc = full_player.container.clone();

    key_controller.connect_key_pressed(move |_, key, _keycode, _modifiers| {
        let key_name = key.name().unwrap_or_default();
        match key_name.as_str() {
            "space" => {
                action_toggle_key();
                glib::Propagation::Stop
            }
            "Left" => {
                action_prev_key();
                glib::Propagation::Stop
            }
            "Right" => {
                action_next_key();
                glib::Propagation::Stop
            }
            "Up" => {
                let app_state = state_for_key.borrow();
                let new_vol = (app_state.player.volume() + 0.05).min(1.0);
                app_state.player.set_volume(new_vol);
                glib::Propagation::Stop
            }
            "Down" => {
                let app_state = state_for_key.borrow();
                let new_vol = (app_state.player.volume() - 0.05).max(0.0);
                app_state.player.set_volume(new_vol);
                glib::Propagation::Stop
            }
            "Escape" => {
                if fp_for_esc.is_visible() {
                    fp_for_esc.set_visible(false);
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            }
            _ => glib::Propagation::Proceed,
        }
    });
    window.add_controller(key_controller);


    window.present();
}

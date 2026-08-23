use crate::app::AppState;
use crate::ui::components::full_player::FullPlayer;
use crate::ui::components::mini_player::MiniPlayer;
use crate::ui::home::HomeView;
use crate::ui::library::LibraryView;
use crate::ui::models::TrackObject;
use crate::ui::queue::QueueView;
use crate::ui::search::SearchView;
use adw::{HeaderBar, ViewStack, ViewSwitcherBar};
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::EventControllerKey;
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
    let search_view_rc = Rc::new(SearchView::new());
    let queue_view_rc = Rc::new(QueueView::new());

    let home_page = stack.add_titled(&home_view_rc.container, Some("home"), "Home");
    home_page.set_icon_name(Some("go-home-symbolic"));

    let library_page = stack.add_titled(&library_view_rc.container, Some("library"), "Library");
    library_page.set_icon_name(Some("folder-music-symbolic"));

    let search_page = stack.add_titled(&search_view_rc.container, Some("search"), "Search");
    search_page.set_icon_name(Some("system-search-symbolic"));

    toolbar_view.append(&stack);

    let switcher_bar = ViewSwitcherBar::builder()
        .stack(&stack)
        .reveal(true)
        .build();

    let mini_player = Rc::new(MiniPlayer::new());
    let full_player = Rc::new(FullPlayer::new());

    let queue_popover = gtk4::Popover::builder()
        .child(&queue_view_rc.container)
        .build();
    queue_popover.set_parent(&full_player.btn_queue);

    full_player.btn_queue.connect_clicked(move |_| {
        queue_popover.popup();
    });

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

        let AppState {
            ref library,
            ref queue,
            ref mut config,
            ref player,
            ..
        } = *app_state;

        if let Some(track) = queue.current(library) {
            config.last_played_track = Some(track.path.to_string_lossy().to_string());
        }
        config.last_playback_position = player.position();

        config.save();
        glib::Propagation::Proceed
    });

    // MiniPlayer click -> Open FullPlayer
    let fp_for_open = full_player.container.clone();
    mini_player
        .click_gesture
        .connect_pressed(move |_, _, _, _| {
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
            let title = track
                .metadata
                .title
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());
            let artist = track
                .metadata
                .artist
                .clone()
                .unwrap_or_else(|| "Unknown Artist".to_string());
            let artwork_path = track.metadata.artwork_path.as_ref();

            mini.lbl_title.set_label(&title);
            mini.lbl_artist.set_label(&artist);
            full.lbl_title.set_label(&title);
            full.lbl_artist.set_label(&artist);

            if let Some(path) = artwork_path {
                mini.img_artwork.set_from_file(Some(path));
                full.img_artwork.set_from_file(Some(path));
            } else {
                mini.img_artwork
                    .set_icon_name(Some("audio-x-generic-symbolic"));
                full.img_artwork
                    .set_icon_name(Some("audio-x-generic-symbolic"));
            }

            // Push to MPRIS
            if let Some(mpris) = &state_for_update.borrow().mpris {
                let server = mpris.clone();
                let metadata = crate::mpris::create_metadata(track);
                glib::MainContext::default().spawn_local(async move {
                    let _ = server
                        .properties_changed([
                            mpris_server::Property::Metadata(metadata),
                            mpris_server::Property::PlaybackStatus(
                                mpris_server::PlaybackStatus::Playing,
                            ),
                        ])
                        .await;
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
                    let _ = server
                        .properties_changed([mpris_server::Property::PlaybackStatus(status)])
                        .await;
                });
            }
        }
    };

    // Helper to reflect shuffle/repeat state in the secondary controls.
    // Reads state and updates button icons/CSS – never borrows state mutably.
    let update_secondary_btns = {
        let full = full_player.clone();
        let state_ref = state.clone();
        move || {
            let (shuffle_on, repeat_mode) = {
                let app_state = state_ref.borrow();
                (app_state.queue.shuffle(), app_state.queue.repeat_mode())
            };

            // Shuffle: add "accent" CSS class when active
            if shuffle_on {
                full.btn_shuffle.add_css_class("accent");
            } else {
                full.btn_shuffle.remove_css_class("accent");
            }

            // Repeat: cycle icon to reflect current mode
            match repeat_mode {
                crate::queue::RepeatMode::Off => {
                    full.btn_repeat
                        .set_icon_name("media-playlist-repeat-symbolic");
                    full.btn_repeat.remove_css_class("accent");
                }
                crate::queue::RepeatMode::Queue => {
                    full.btn_repeat
                        .set_icon_name("media-playlist-repeat-symbolic");
                    full.btn_repeat.add_css_class("accent");
                }
                crate::queue::RepeatMode::One => {
                    full.btn_repeat
                        .set_icon_name("media-playlist-repeat-song-symbolic");
                    full.btn_repeat.add_css_class("accent");
                }
            }
        }
    };

    // Shuffle button
    let state_for_shuffle = state.clone();
    let update_secondary_shuffle = update_secondary_btns.clone();
    full_player.btn_shuffle.connect_clicked(move |_| {
        {
            let mut app_state = state_for_shuffle.borrow_mut();
            let new_shuffle = !app_state.queue.shuffle();
            app_state.queue.set_shuffle(new_shuffle);
        }
        update_secondary_shuffle();
    });

    // Repeat button – cycles Off → Queue → One → Off
    let state_for_repeat = state.clone();
    let update_secondary_repeat = update_secondary_btns.clone();
    full_player.btn_repeat.connect_clicked(move |_| {
        {
            let mut app_state = state_for_repeat.borrow_mut();
            let next_mode = match app_state.queue.repeat_mode() {
                crate::queue::RepeatMode::Off => crate::queue::RepeatMode::Queue,
                crate::queue::RepeatMode::Queue => crate::queue::RepeatMode::One,
                crate::queue::RepeatMode::One => crate::queue::RepeatMode::Off,
            };
            app_state.queue.set_repeat_mode(next_mode);
        }
        update_secondary_repeat();
    });

    // Initialize secondary button appearance from saved config state
    update_secondary_btns();

    // Auto-continue logic (EOS)
    let state_for_eos = state.clone();
    let queue_view_for_eos = queue_view_rc.clone();
    let update_players_eos = update_players.clone();
    let update_play_btn_eos = update_play_btn.clone();

    state.borrow().player.set_eos_callback(move || {
        let (track_opt, uri_res) = {
            let mut app_state = state_for_eos.borrow_mut();
            let AppState {
                ref library,
                ref mut queue,
                ref player,
                ..
            } = *app_state;

            if let Some(track) = queue.next(library).cloned() {
                let uri = glib::filename_to_uri(&track.path, None);
                if let Ok(ref u) = uri {
                    let play_res = player.play_file(u.as_str());
                    let curr_idx = queue.current_index();
                    (Some((track, curr_idx)), Some(play_res))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            }
        };

        if let Some((track, curr_idx)) = track_opt {
            if let Some(Ok(())) = uri_res {
                update_players_eos(&track);

                if let Some(idx) = curr_idx {
                    queue_view_for_eos
                        .selection_model
                        .select_item(idx as u32, true);
                }
            }
        } else {
            update_play_btn_eos("media-playback-start-symbolic");
        }
    });

    // Selection changed -> Play song
    let state_for_selection = state.clone();
    let _queue_view_for_selection = queue_view_rc.clone();
    let update_players_sel = update_players.clone();
    let update_play_btn_sel = update_play_btn.clone();

    library_view_rc
        .songs_view
        .selection_model
        .connect_selection_changed(move |model, _, _| {
            if let Some(item) = model.selected_item() {
                if let Ok(track_obj) = item.downcast::<TrackObject>() {
                    let track = track_obj.get_track();

                    let play_result = {
                        let mut app_state = state_for_selection.borrow_mut();
                        let AppState {
                            ref library,
                            ref mut queue,
                            ref player,
                            ..
                        } = *app_state;

                        queue.play_library_track(library, &track);

                        match glib::filename_to_uri(&track.path, None) {
                            Ok(uri) => Some((player.play_file(uri.as_str()), track.clone())),
                            Err(e) => {
                                eprintln!("Invalid path for selected track: {e}");
                                None
                            }
                        }
                    };

                    if let Some((res, track)) = play_result {
                        if let Err(e) = res {
                            eprintln!("Failed to play: {}", e);
                        } else {
                            update_players_sel(&track);
                            update_play_btn_sel("media-playback-pause-symbolic");
                        }
                    }
                }
            }
        });

    // Play/Pause button setup
    let state_for_play = state.clone();
    let update_play_btn_pp = update_play_btn.clone();
    let toggle_play = move || {
        let (is_playing, has_current) = {
            let app_state = state_for_play.borrow();
            let AppState {
                ref library,
                ref queue,
                ref player,
                ..
            } = *app_state;
            (player.is_playing(), queue.current(library).is_some())
        };

        if is_playing {
            state_for_play.borrow().player.pause();
            update_play_btn_pp("media-playback-start-symbolic");
        } else if has_current {
            state_for_play.borrow().player.resume();
            update_play_btn_pp("media-playback-pause-symbolic");
        }
    };

    let toggle_play_clone = toggle_play.clone();
    mini_player
        .btn_play_pause
        .connect_clicked(move |_| toggle_play_clone());
    let toggle_play_full = toggle_play.clone();
    full_player
        .btn_play_pause
        .connect_clicked(move |_| toggle_play_full());

    let action_prev = {
        let state_rc = state.clone();
        let update_p = update_players.clone();
        let update_btn = update_play_btn.clone();
        move || {
            let (track_and_idx, play_res) = {
                let mut app_state = state_rc.borrow_mut();
                let AppState {
                    ref library,
                    ref mut queue,
                    ref player,
                    ..
                } = *app_state;

                if let Some(track) = queue.previous(library).cloned() {
                    let uri_res = glib::filename_to_uri(&track.path, None);
                    if let Ok(uri) = uri_res {
                        let res = player.play_file(uri.as_str());
                        let idx = queue.current_index();
                        (Some((track, idx)), Some(res))
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                }
            };

            if let (Some((track, _idx)), Some(res)) = (track_and_idx, play_res) {
                if let Err(e) = res {
                    eprintln!("Failed to play: {e}");
                } else {
                    update_p(&track);
                    update_btn("media-playback-pause-symbolic");
                }
            }
        }
    };

    // Previous button
    let action_prev_btn = action_prev.clone();
    full_player
        .btn_prev
        .connect_clicked(move |_| action_prev_btn());

    let action_next = {
        let state_rc = state.clone();
        let update_p = update_players.clone();
        let update_btn = update_play_btn.clone();
        move || {
            let (track_and_idx, play_res) = {
                let mut app_state = state_rc.borrow_mut();
                let AppState {
                    ref library,
                    ref mut queue,
                    ref player,
                    ..
                } = *app_state;

                if let Some(track) = queue.next(library).cloned() {
                    let uri_res = glib::filename_to_uri(&track.path, None);
                    if let Ok(uri) = uri_res {
                        let res = player.play_file(uri.as_str());
                        let idx = queue.current_index();
                        (Some((track, idx)), Some(res))
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                }
            };

            if let (Some((track, _idx)), Some(res)) = (track_and_idx, play_res) {
                if let Err(e) = res {
                    eprintln!("Failed to play: {e}");
                } else {
                    update_p(&track);
                    update_btn("media-playback-pause-symbolic");
                }
            }
        }
    };

    // Next button
    let action_next_btn = action_next.clone();
    full_player
        .btn_next
        .connect_clicked(move |_| action_next_btn());

    // Progress update timer
    let state_for_timer = state.clone();
    let full_player_timer = full_player.clone();

    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let (is_playing, pos, dur) = {
            let app_state = state_for_timer.borrow();
            (
                app_state.player.is_playing(),
                app_state.player.position(),
                app_state.player.duration(),
            )
        };

        if is_playing {
            if let (Some(pos), Some(dur)) = (pos, dur) {
                if dur > 0 {
                    full_player_timer.scale_progress.set_range(0.0, dur as f64);
                    full_player_timer.scale_progress.set_value(pos as f64);

                    let pos_sec = pos / 1_000_000_000;
                    let dur_sec = dur / 1_000_000_000;
                    let pos_str = format!("{}:{:02}", pos_sec / 60, pos_sec % 60);
                    let dur_str = format!("{}:{:02}", dur_sec / 60, dur_sec % 60);
                    full_player_timer
                        .lbl_time
                        .set_label(&format!("{} / {}", pos_str, dur_str));
                }
            }
        }
        glib::ControlFlow::Continue
    });

    // Seek interaction
    let state_for_seek = state.clone();
    full_player
        .scale_progress
        .connect_value_changed(move |scale| {
            let pos = state_for_seek.borrow().player.position();
            let target_ns = scale.value() as u64;

            if let Some(pos) = pos {
                if target_ns.abs_diff(pos) > 500_000_000 {
                    if let Err(e) = state_for_seek.borrow().player.seek(target_ns) {
                        eprintln!("Seek error: {}", e);
                    }
                }
            }
        });

    // Refactored folder scanning function with non-overlapping scan guard
    let is_scanning = Rc::new(std::cell::Cell::new(false));

    let scan_folder = {
        let is_scanning_scan = is_scanning.clone();
        let queue_view_scan = queue_view_rc.clone();
        let update_players_scan = update_players.clone();
        let update_play_btn_scan = update_play_btn.clone();
        move |path: PathBuf,
              home: Rc<HomeView>,
              state_rc: Rc<RefCell<AppState>>,
              lib_rc: Rc<LibraryView>| {
            if is_scanning_scan.get() {
                return;
            }

            if !path.exists() || !path.is_dir() {
                let err_msg = format!(
                    "Folder '{}' does not exist or is inaccessible.",
                    path.display()
                );
                home.set_state(crate::ui::home::HomeState::ScanFailed(err_msg));
                state_rc.borrow_mut().library = crate::library::Library::new();
                lib_rc.songs_view.store.remove_all();
                lib_rc.albums_view.store.remove_all();
                lib_rc.artists_view.store.remove_all();
                return;
            }

            is_scanning_scan.set(true);
            home.set_state(crate::ui::home::HomeState::Scanning);

            let result_lib = std::sync::Arc::new(std::sync::Mutex::new(None));
            let result_clone = result_lib.clone();

            std::thread::spawn(move || {
                let cache_dir = gtk4::glib::user_cache_dir()
                    .join("audmedia_linux")
                    .join("artwork");
                let lib = crate::library::Library::scan_directory(path, cache_dir, |_| {});
                *result_clone.lock().unwrap() = Some(lib);
            });

            let is_scanning_timer = is_scanning_scan.clone();
            let queue_view_timer = queue_view_scan.clone();
            let update_players_timer = update_players_scan.clone();
            let update_play_btn_timer = update_play_btn_scan.clone();

            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if let Ok(mut lock) = result_lib.try_lock() {
                    if let Some(lib) = lock.take() {
                        lib_rc.songs_view.store.remove_all();
                        lib_rc.albums_view.store.remove_all();
                        lib_rc.artists_view.store.remove_all();

                        let mut albums = HashSet::new();
                        let mut artists = HashSet::new();

                        let mut song_objs = Vec::new();
                        let mut album_objs = Vec::new();
                        let mut artist_objs = Vec::new();

                        for track in &lib.tracks {
                            song_objs.push(
                                crate::ui::models::TrackObject::new(track.clone())
                                    .upcast::<gtk4::glib::Object>(),
                            );

                            let album = track
                                .metadata
                                .album
                                .clone()
                                .unwrap_or_else(|| "Unknown Album".to_string());
                            let artist = track
                                .metadata
                                .artist
                                .clone()
                                .unwrap_or_else(|| "Unknown Artist".to_string());

                            if albums.insert(album.clone()) {
                                album_objs.push(
                                    crate::ui::models::ItemObject::new(
                                        &album,
                                        &artist,
                                        track.metadata.artwork_path.clone(),
                                    )
                                    .upcast::<gtk4::glib::Object>(),
                                );
                            }

                            if artists.insert(artist.clone()) {
                                artist_objs.push(
                                    crate::ui::models::ItemObject::new(
                                        &artist,
                                        "",
                                        track.metadata.artwork_path.clone(),
                                    )
                                    .upcast::<gtk4::glib::Object>(),
                                );
                            }
                        }

                        lib_rc.songs_view.store.splice(
                            lib_rc.songs_view.store.n_items(),
                            0,
                            &song_objs,
                        );
                        lib_rc.albums_view.store.splice(
                            lib_rc.albums_view.store.n_items(),
                            0,
                            &album_objs,
                        );
                        lib_rc.artists_view.store.splice(
                            lib_rc.artists_view.store.n_items(),
                            0,
                            &artist_objs,
                        );

                        let track_count = lib.tracks.len();
                        {
                            let mut app_state = state_rc.borrow_mut();
                            app_state.queue.sync_library(&lib);
                            app_state.library = lib;
                        }

                        is_scanning_timer.set(false);
                        if track_count == 0 {
                            home.set_state(crate::ui::home::HomeState::EmptyFolder);
                        } else {
                            home.set_state(crate::ui::home::HomeState::LibraryLoaded(track_count));

                            // Restore playback state if available
                            let (restored_track, restored_pos) = {
                                let app_state = state_rc.borrow();
                                crate::config::validate_restored_playback_state(
                                    &app_state.config,
                                    &app_state.library,
                                )
                            }
                            .unzip();

                            if let (Some(restored_track), Some(restored_pos)) =
                                (restored_track, restored_pos)
                            {
                                if let Ok(uri) = glib::filename_to_uri(&restored_track.path, None) {
                                    let success = {
                                        let mut app_state = state_rc.borrow_mut();
                                        let AppState {
                                            ref library,
                                            ref mut queue,
                                            ref player,
                                            ..
                                        } = *app_state;
                                        queue.play_library_track(library, &restored_track);

                                        let ok = player.prepare_file(uri.as_str()).is_ok();
                                        if ok && restored_pos > 0 {
                                            let _ = player.seek(restored_pos);
                                        }
                                        ok
                                    };

                                    if success {
                                        queue_view_timer.store.remove_all();
                                        queue_view_timer.store.append(
                                            &crate::ui::models::TrackObject::new(
                                                restored_track.clone(),
                                            )
                                            .upcast::<gtk4::glib::Object>(),
                                        );
                                        queue_view_timer.selection_model.select_item(0, true);

                                        update_players_timer(&restored_track);
                                        update_play_btn_timer("media-playback-start-symbolic");
                                    }
                                }
                            }
                        }

                        return gtk4::glib::ControlFlow::Break;
                    }
                }
                gtk4::glib::ControlFlow::Continue
            });
        }
    };

    // Auto-scan on startup if we have folders configured
    let state_for_startup = state.clone();
    let home_view_startup = home_view_rc.clone();
    let library_view_startup = library_view_rc.clone();
    let scan_folder_clone = scan_folder.clone();
    let folder_to_scan = state_for_startup
        .borrow()
        .config
        .music_folders
        .first()
        .cloned();
    if let Some(first_folder) = folder_to_scan {
        scan_folder_clone(
            PathBuf::from(first_folder),
            home_view_startup,
            state_for_startup,
            library_view_startup,
        );
    } else {
        home_view_rc.set_state(crate::ui::home::HomeState::NoFolder);
    }

    // Setup scan button for manual scanning / changing folder
    let window_clone = window.clone();
    let state_clone = state.clone();
    let home_view_clone = home_view_rc.clone();
    let library_view_clone = library_view_rc.clone();
    let scan_folder_manual = scan_folder.clone();
    let is_scanning_scan_btn = is_scanning.clone();

    home_view_rc.btn_scan.connect_clicked(move |_| {
        if is_scanning_scan_btn.get() {
            return;
        }

        let dialog = gtk4::FileDialog::new();
        let state_weak = Rc::downgrade(&state_clone);
        let home_weak = Rc::downgrade(&home_view_clone);
        let lib_weak = Rc::downgrade(&library_view_clone);
        let scan_folder_cb = scan_folder_manual.clone();

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

                        // Update config with the selected folder
                        {
                            let mut app_state = state_rc.borrow_mut();
                            app_state.config.music_folders =
                                vec![path.to_string_lossy().to_string()];
                            app_state.config.save();
                        }

                        scan_folder_cb(path, home, state_rc, lib_rc);
                    }
                }
            },
        );
    });

    // Setup rescan button for re-scanning current folder
    let state_rescan = state.clone();
    let home_rescan = home_view_rc.clone();
    let lib_rescan = library_view_rc.clone();
    let scan_folder_rescan = scan_folder.clone();
    let is_scanning_rescan_btn = is_scanning.clone();

    home_view_rc.btn_rescan.connect_clicked(move |_| {
        if is_scanning_rescan_btn.get() {
            return;
        }

        let folder = state_rescan.borrow().config.music_folders.first().cloned();

        if let Some(path_str) = folder {
            scan_folder_rescan(
                PathBuf::from(path_str),
                home_rescan.clone(),
                state_rescan.clone(),
                lib_rescan.clone(),
            );
        }
    });

    // Search entry input listener
    let state_for_search = state.clone();
    let search_view_for_entry = search_view_rc.clone();

    search_view_rc
        .search_entry
        .connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            let app_state = state_for_search.borrow();

            if app_state.library.tracks.is_empty() {
                search_view_for_entry.set_state(crate::ui::search::SearchState::EmptyLibrary);
                search_view_for_entry.songs_view.store.remove_all();
                return;
            }

            let trimmed = query.trim();
            if trimmed.is_empty() {
                search_view_for_entry.songs_view.store.remove_all();
                search_view_for_entry.set_state(crate::ui::search::SearchState::EmptyQuery);
            } else {
                let matching_tracks = app_state.library.search(trimmed);
                if matching_tracks.is_empty() {
                    search_view_for_entry.songs_view.store.remove_all();
                    search_view_for_entry.set_state(crate::ui::search::SearchState::NoResults);
                } else {
                    let objs: Vec<glib::Object> = matching_tracks
                        .iter()
                        .map(|t| {
                            crate::ui::models::TrackObject::new((*t).clone())
                                .upcast::<glib::Object>()
                        })
                        .collect();

                    search_view_for_entry.songs_view.store.remove_all();
                    search_view_for_entry.songs_view.store.splice(0, 0, &objs);
                    search_view_for_entry.set_state(crate::ui::search::SearchState::Results(
                        matching_tracks.len(),
                    ));
                }
            }
        });

    // Search song selection handler
    let state_for_search_select = state.clone();
    let _queue_view_search = queue_view_rc.clone();
    let update_players_search = update_players.clone();
    let update_play_btn_search = update_play_btn.clone();

    search_view_rc
        .songs_view
        .selection_model
        .connect_selection_changed(move |selection_model, _position, _n_items| {
            let selected_item = selection_model.selected_item();
            if let Some(obj) = selected_item {
                if let Ok(track_obj) = obj.downcast::<TrackObject>() {
                    let track = track_obj.get_track();

                    let play_result = {
                        let mut app_state = state_for_search_select.borrow_mut();
                        let AppState {
                            ref library,
                            ref mut queue,
                            ref player,
                            ..
                        } = *app_state;

                        queue.play_library_track(library, &track);

                        match glib::filename_to_uri(&track.path, None) {
                            Ok(uri) => Some((player.play_file(uri.as_str()), track.clone())),
                            Err(e) => {
                                eprintln!("Invalid path for selected track: {e}");
                                None
                            }
                        }
                    };

                    if let Some((res, track)) = play_result {
                        if let Err(e) = res {
                            eprintln!("Failed to play selected track: {}", e);
                        } else {
                            update_players_search(&track);
                            update_play_btn_search("media-playback-pause-symbolic");
                        }
                    }
                }
            }
        });

    // Search view btn_scan (Add Music Folder on empty library)
    let window_search_scan = window.clone();
    let state_search_scan = state.clone();
    let home_search_scan = home_view_rc.clone();
    let library_search_scan = library_view_rc.clone();
    let scan_folder_search = scan_folder;
    let is_scanning_search_scan = is_scanning;

    search_view_rc.btn_scan.connect_clicked(move |_| {
        if is_scanning_search_scan.get() {
            return;
        }

        let dialog = gtk4::FileDialog::new();
        let state_weak = Rc::downgrade(&state_search_scan);
        let home_weak = Rc::downgrade(&home_search_scan);
        let lib_weak = Rc::downgrade(&library_search_scan);
        let scan_folder_cb = scan_folder_search.clone();

        dialog.select_folder(
            Some(&window_search_scan),
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

                        {
                            let mut app_state = state_rc.borrow_mut();
                            app_state.config.music_folders =
                                vec![path.to_string_lossy().to_string()];
                            app_state.config.save();
                        }

                        scan_folder_cb(path, home, state_rc, lib_rc);
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
                let new_vol = {
                    let app_state = state_for_key.borrow();
                    (app_state.player.volume() + 0.05).min(1.0)
                };
                state_for_key.borrow().player.set_volume(new_vol);
                glib::Propagation::Stop
            }
            "Down" => {
                let new_vol = {
                    let app_state = state_for_key.borrow();
                    (app_state.player.volume() - 0.05).max(0.0)
                };
                state_for_key.borrow().player.set_volume(new_vol);
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

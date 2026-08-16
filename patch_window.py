import sys

with open('src/ui/window.rs', 'r') as f:
    content = f.read()

# 1. Add imports
content = content.replace(
    'use crate::ui::models::{ItemObject, TrackObject};',
    'use crate::ui::models::{ItemObject, TrackObject};\nuse crate::ui::components::player_controls::PlayerControls;'
)

# 2. Add player controls setup
controls_setup = """
    let player_controls = Rc::new(PlayerControls::new());
    toolbar_view.append(&player_controls.container);
"""
content = content.replace(
    'toolbar_view.append(&switcher_bar);',
    controls_setup + '\n    toolbar_view.append(&switcher_bar);'
)

# 3. Add wiring after setup scan button
wiring_code = """
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

"""
content = content.replace(
    '// Setup scan button',
    wiring_code + '\n    // Setup scan button'
)

with open('src/ui/window.rs', 'w') as f:
    f.write(content)

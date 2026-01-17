#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod scheduler;
mod state;
mod commands;
mod tray;

use tauri::Manager;

pub fn run() {
    let app_state = crate::state::AppState::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::start_break,
            commands::pause_breaks,
            commands::resume_breaks,
            commands::skip_break,
            commands::postpone_break,
            commands::complete_break,
            commands::get_break_state,
            commands::update_settings,
            commands::get_settings,
            commands::set_break_window_mode,
            commands::set_normal_window_mode,
        ])
        .setup(|app| {
            // Create a hidden anchor window to prevent app from quitting when all visible windows close
            // This is the most reliable cross-platform solution
            use tauri::{WebviewUrl, WebviewWindowBuilder};

            match WebviewWindowBuilder::new(app, "anchor", WebviewUrl::App("/".into()))
                .title("Restie Anchor")
                .inner_size(1.0, 1.0)
                .visible(false)
                .skip_taskbar(true)
                .build()
            {
                Ok(_) => println!("Created anchor window to keep app alive"),
                Err(e) => println!("Warning: Failed to create anchor window: {:?}", e),
            }

            // Optional: Hide dock icon on macOS for tray-only experience
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                println!("macOS: Hidden dock icon (tray-only app)");
            }

            // Set app handle in scheduler FIRST
            let app_state: tauri::State<crate::state::AppState> = app.state();
            let mut scheduler = app_state.scheduler.lock().unwrap();
            scheduler.set_app_handle(app.handle().clone());
            drop(scheduler);

            // THEN setup tray (so it can read the scheduler state)
            let _tray = tray::setup_tray(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

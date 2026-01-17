use crate::models::{BreakSettings, BreakState};
use tauri::{Manager, State};

#[tauri::command]
pub fn start_break(app_state: State<'_, crate::state::AppState>) -> Result<BreakState, String> {
    let scheduler = app_state.scheduler.lock().unwrap();
    scheduler.start_break();
    Ok(scheduler.get_state())
}

#[tauri::command]
pub fn pause_breaks(app_state: State<'_, crate::state::AppState>) -> Result<BreakState, String> {
    let scheduler = app_state.scheduler.lock().unwrap();
    scheduler.pause();
    Ok(scheduler.get_state())
}

#[tauri::command]
pub fn resume_breaks(app_state: State<'_, crate::state::AppState>) -> Result<BreakState, String> {
    let scheduler = app_state.scheduler.lock().unwrap();
    scheduler.resume();
    Ok(scheduler.get_state())
}

#[tauri::command]
pub fn skip_break(app_state: State<'_, crate::state::AppState>) -> Result<BreakState, String> {
    let scheduler = app_state.scheduler.lock().unwrap();
    scheduler.skip_break();
    Ok(scheduler.get_state())
}

#[tauri::command]
pub fn postpone_break(app_state: State<'_, crate::state::AppState>) -> Result<BreakState, String> {
    let scheduler = app_state.scheduler.lock().unwrap();
    scheduler.postpone_break();
    Ok(scheduler.get_state())
}

#[tauri::command]
pub fn complete_break(app_state: State<'_, crate::state::AppState>) -> Result<BreakState, String> {
    let scheduler = app_state.scheduler.lock().unwrap();
    scheduler.complete_break();
    Ok(scheduler.get_state())
}

#[tauri::command]
pub fn get_break_state(app_state: State<'_, crate::state::AppState>) -> Result<BreakState, String> {
    let scheduler = app_state.scheduler.lock().unwrap();
    Ok(scheduler.get_state())
}

#[tauri::command]
pub fn update_settings(settings: BreakSettings, app_state: State<'_, crate::state::AppState>) -> Result<BreakSettings, String> {
    // Backend validation (safety net)
    if settings.microbreak_interval_minutes < 1 || settings.microbreak_interval_minutes > 60 {
        return Err("Invalid microbreak interval: must be 1-60 minutes".to_string());
    }
    if settings.microbreak_duration_seconds < 5 || settings.microbreak_duration_seconds > 300 {
        return Err("Invalid microbreak duration: must be 5-300 seconds".to_string());
    }
    if settings.longbreak_interval_microbreaks < 1 || settings.longbreak_interval_microbreaks > 10 {
        return Err("Invalid long break interval: must be 1-10 microbreaks".to_string());
    }
    if settings.longbreak_duration_minutes < 1 || settings.longbreak_duration_minutes > 60 {
        return Err("Invalid long break duration: must be 1-60 minutes".to_string());
    }

    println!("Settings validated: {:?}", settings);

    // Update settings
    {
        let mut current_settings = app_state.settings.lock().unwrap();
        *current_settings = settings.clone();
        println!("Settings updated in AppState");
    } // Drop lock before acquiring scheduler lock

    // Immediately reschedule with new settings
    {
        let scheduler = app_state.scheduler.lock().unwrap();
        scheduler.reschedule_with_new_settings();
        println!("Rescheduled breaks with new settings");
    }

    Ok(settings)
}

#[tauri::command]
pub fn get_settings(app_state: State<'_, crate::state::AppState>) -> Result<BreakSettings, String> {
    let settings = app_state.settings.lock().unwrap();
    Ok(settings.clone())
}

#[tauri::command]
pub fn set_break_window_mode(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_fullscreen(true);
        let _ = window.set_decorations(false);
        let _ = window.set_always_on_top(true);
        let _ = window.set_ignore_cursor_events(false);
    }
    Ok(())
}

#[tauri::command]
pub fn set_normal_window_mode(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_fullscreen(false);
        let _ = window.set_decorations(true);
        let _ = window.set_always_on_top(false);
    }
    Ok(())
}

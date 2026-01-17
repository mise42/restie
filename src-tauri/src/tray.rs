use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent, TrayIcon},
    Manager,
    WebviewUrl,
    WebviewWindowBuilder,
    AppHandle,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn format_time_left(ms: u64) -> String {
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    if minutes > 60 {
        let hours = minutes / 60;
        let mins = minutes % 60;
        format!("{}h {}m", hours, mins)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

fn get_tray_tooltip(app: &AppHandle) -> String {
    let app_state = app.state::<crate::state::AppState>();
    let scheduler = app_state.scheduler.lock().unwrap();
    let scheduler_state = scheduler.get_scheduler_state();

    if let Some(scheduled_time) = scheduler_state.scheduled_break_time {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let time_left = scheduled_time.saturating_sub(now);

        let break_type = match scheduler_state.current_break_type {
            Some(crate::models::BreakType::Microbreak) => "Microbreak",
            Some(crate::models::BreakType::Longbreak) => "Long break",
            None => "Break",
        };

        format!("Next: {} in {}", break_type, format_time_left(time_left))
    } else {
        "No break scheduled".to_string()
    }
}

pub fn setup_tray(app: &tauri::App) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let preferences_item = MenuItem::with_id(app, "preferences", "Preferences...", true, None::<&str>)?;
    let test_break_item = MenuItem::with_id(app, "test_break", "Test Break", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&test_break_item, &preferences_item, &quit_item])?;

    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip(&get_tray_tooltip(&app.handle()))
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            "preferences" => {
                println!("Tray: preferences menu item clicked");

                // Check if preferences window already exists
                if let Some(window) = app.get_webview_window("preferences") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    // Create new preferences window
                    if let Ok(_window) = WebviewWindowBuilder::new(
                        app,
                        "preferences",
                        WebviewUrl::App("/preferences".into())
                    )
                    .title("Restie - Preferences")
                    .inner_size(450.0, 450.0)
                    .resizable(true)
                    .center()
                    .build()
                    {
                        // Window will actually close when user clicks X (destroyed, not hidden)
                        // This is intentional - preferences are infrequent, break window serves as app anchor
                    }
                }
            }
            "test_break" => {
                println!("Tray: test_break menu item clicked");

                // Get settings to check fullscreen preference
                let app_state = app.state::<crate::state::AppState>();
                let settings = app_state.settings.lock().unwrap();
                let fullscreen = settings.fullscreen_breaks;
                drop(settings); // Release the lock

                // Check if break window already exists
                if let Some(window) = app.get_webview_window("break") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    // Create new break window with settings from preferences
                    // Both fullscreen and windowed modes are frameless (overlay-like)
                    let builder = WebviewWindowBuilder::new(
                        app,
                        "break",
                        WebviewUrl::App("/break".into())
                    )
                    .title("Restie - Break Time")
                    .decorations(false)
                    .skip_taskbar(true);

                    let builder = if fullscreen {
                        // Use maximized instead of native fullscreen for frameless look
                        // Don't use always_on_top so menu bar remains accessible
                        builder
                            .maximized(true)
                            .resizable(false)
                    } else {
                        builder
                            .inner_size(800.0, 600.0)
                            .center()
                            .always_on_top(true)
                    };

                    let _ = builder.build();
                }
            }
            _ => (),
        })
        .on_tray_icon_event(move |tray, event| {
            match event {
                TrayIconEvent::Enter { .. } => {
                    // User is hovering! Compute fresh tooltip
                    let app = tray.app_handle();
                    let tooltip = get_tray_tooltip(&app);
                    let _ = tray.set_tooltip(Some(&tooltip));
                    println!("Tooltip updated on hover: {}", tooltip);
                }
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    // Double-click on tray icon - show preferences
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("preferences") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(tray)
}

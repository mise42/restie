use crate::models::{BreakSettings, BreakState, BreakType, SchedulerState};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[derive(Clone)]
pub struct BreakScheduler {
    settings: Arc<Mutex<BreakSettings>>,
    state: Arc<Mutex<BreakState>>,
    scheduler_state: Arc<Mutex<SchedulerState>>,
    app_handle: Option<AppHandle>,
}

impl BreakScheduler {
    pub fn new(settings: Arc<Mutex<BreakSettings>>) -> Self {
        Self {
            settings: settings.clone(),
            state: Arc::new(Mutex::new(BreakState::default())),
            scheduler_state: Arc::new(Mutex::new(SchedulerState {
                scheduled_break_time: None,
                current_break_type: None,
                time_left_ms: None,
            })),
            app_handle: None,
        }
    }

    pub fn set_app_handle(&mut self, handle: AppHandle) {
        println!("Setting app handle in scheduler");
        self.app_handle = Some(handle);
        // Schedule first break after app handle is set
        println!("Scheduling first break...");
        self.schedule_next_break();
        println!("First break scheduled");
    }

    fn schedule_next_break(&self) {
        println!("schedule_next_break called");
        let settings = self.settings.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let state = self.state.lock().unwrap();
        let break_type = if state.microbreaks_since_longbreak + 1 >= settings.longbreak_interval_microbreaks {
            BreakType::Longbreak
        } else {
            BreakType::Microbreak
        };

        let interval_ms = match break_type {
            BreakType::Microbreak => settings.microbreak_interval_minutes * 60 * 1000,
            BreakType::Longbreak => settings.longbreak_duration_minutes * 60 * 1000,
        } as u64;

        let scheduled_time = now + interval_ms;

        println!("Scheduling {:?} in {}ms (at timestamp {})", break_type, interval_ms, scheduled_time);

        let mut scheduler_state = self.scheduler_state.lock().unwrap();
        scheduler_state.scheduled_break_time = Some(scheduled_time);
        scheduler_state.current_break_type = Some(break_type.clone());
        scheduler_state.time_left_ms = Some(interval_ms);

        drop(scheduler_state);
        drop(settings);
        drop(state);

        // Start background task to check for break time
        self.start_background_check();
    }

    fn start_background_check(&self) {
        let state = self.state.clone();
        let scheduler_state = self.scheduler_state.clone();
        let settings = self.settings.clone();
        let app_handle = self.app_handle.clone();

        std::thread::spawn(move || {
            let mut last_break_triggered = false;

            loop {
                std::thread::sleep(Duration::from_millis(500));

                let s = state.lock().unwrap();
                if s.is_paused {
                    drop(s);
                    continue;
                }

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                let ss = scheduler_state.lock().unwrap();

                if let Some(scheduled_time) = ss.scheduled_break_time {
                    let time_left = scheduled_time.saturating_sub(now);

                    // Trigger break when time is up (within 1 second window)
                    if time_left <= 500 && !last_break_triggered {
                        // Time for break!
                        println!("Break time reached! Showing break window... (time_left: {}ms)", time_left);

                        // Mark break as triggered
                        last_break_triggered = true;

                        // Get fullscreen setting
                        let fullscreen = settings.lock().unwrap().fullscreen_breaks;

                        if let Some(ref app) = app_handle {
                            // Check if break window already exists
                            if let Some(window) = app.get_webview_window("break") {
                                println!("Break window exists, showing it");
                                let _ = window.show();
                                let _ = window.set_focus();
                            } else {
                                println!("Creating new break window (fullscreen: {})", fullscreen);
                                // Create new break window
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

                                match builder.build() {
                                    Ok(_) => println!("Break window created successfully"),
                                    Err(e) => println!("Failed to create break window: {:?}", e),
                                }
                            }
                        } else {
                            println!("No app handle available");
                        }

                        drop(ss);
                        drop(s);
                        continue;
                    } else if time_left > 500 {
                        // Reset the trigger flag when we're not near break time
                        last_break_triggered = false;
                    }
                }

                drop(ss);
                drop(s);
            }
        });
    }

    pub fn start_break(&self) {
        let mut state = self.state.lock().unwrap();
        state.is_break_active = true;
    }

    pub fn pause(&self) {
        let mut state = self.state.lock().unwrap();
        state.is_paused = true;
    }

    pub fn resume(&self) {
        let mut state = self.state.lock().unwrap();
        state.is_paused = false;
    }

    pub fn skip_break(&self) {
        let mut state = self.state.lock().unwrap();
        state.is_break_active = false;
        state.break_type = None;
        state.skip_count += 1;
        drop(state);

        // Schedule next break after skip
        self.schedule_next_break();
    }

    pub fn postpone_break(&self) {
        let mut state = self.state.lock().unwrap();
        state.postpone_count += 1;
        drop(state);

        // Reschedule for 5 minutes later
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let mut scheduler_state = self.scheduler_state.lock().unwrap();
        scheduler_state.scheduled_break_time = Some(now + (5 * 60 * 1000));
        drop(scheduler_state);

        // Start background check again
        self.start_background_check();
    }

    pub fn complete_break(&self) {
        let mut state = self.state.lock().unwrap();
        state.is_break_active = false;
        state.break_type = None;
        state.break_number += 1;

        // Determine next break type
        let settings = self.settings.lock().unwrap();
        if state.microbreaks_since_longbreak + 1 >= settings.longbreak_interval_microbreaks {
            state.microbreaks_since_longbreak = 0;
        } else {
            state.microbreaks_since_longbreak += 1;
        }
        drop(state);
        drop(settings);

        // Schedule next break after completion
        self.schedule_next_break();
    }

    pub fn reschedule_with_new_settings(&self) {
        println!("Rescheduling breaks with new settings...");

        // Check if break is currently active
        let state = self.state.lock().unwrap();
        if state.is_break_active {
            println!("Break currently active, will reschedule after completion");
            drop(state);
            return; // Option A: wait until break completes
        }
        drop(state);

        // Clear current schedule
        {
            let mut scheduler_state = self.scheduler_state.lock().unwrap();
            scheduler_state.scheduled_break_time = None;
            scheduler_state.current_break_type = None;
            scheduler_state.time_left_ms = None;
            println!("Cleared old schedule");
        }

        // Schedule next break with new settings
        self.schedule_next_break();
        println!("Rescheduled with new settings");
    }

    pub fn get_state(&self) -> BreakState {
        self.state.lock().unwrap().clone()
    }

    pub fn get_scheduler_state(&self) -> SchedulerState {
        self.scheduler_state.lock().unwrap().clone()
    }
}

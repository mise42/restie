use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakSettings {
    pub microbreak_interval_minutes: u32,
    pub microbreak_duration_seconds: u32,
    pub longbreak_interval_microbreaks: u32,
    pub longbreak_duration_minutes: u32,
    pub fullscreen_breaks: bool,
}

impl Default for BreakSettings {
    fn default() -> Self {
        Self {
            microbreak_interval_minutes: 20,
            microbreak_duration_seconds: 20,
            longbreak_interval_microbreaks: 4,
            longbreak_duration_minutes: 5,
            fullscreen_breaks: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakType {
    Microbreak,
    Longbreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakState {
    pub break_type: Option<BreakType>,
    pub break_number: u32,
    pub microbreaks_since_longbreak: u32,
    pub is_paused: bool,
    pub is_break_active: bool,
    pub postpone_count: u32,
    pub skip_count: u32,
}

impl Default for BreakState {
    fn default() -> Self {
        Self {
            break_type: None,
            break_number: 0,
            microbreaks_since_longbreak: 0,
            is_paused: false,
            is_break_active: false,
            postpone_count: 0,
            skip_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerState {
    pub scheduled_break_time: Option<u64>, // Unix timestamp in milliseconds
    pub current_break_type: Option<BreakType>,
    pub time_left_ms: Option<u64>,
}

impl Default for SchedulerState {
    fn default() -> Self {
        Self {
            scheduled_break_time: None,
            current_break_type: None,
            time_left_ms: None,
        }
    }
}

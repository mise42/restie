use crate::models::BreakSettings;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub settings: Arc<Mutex<BreakSettings>>,
    pub scheduler: Arc<Mutex<crate::scheduler::BreakScheduler>>,
}

impl Default for AppState {
    fn default() -> Self {
        let settings = Arc::new(Mutex::new(BreakSettings::default()));
        Self {
            settings: settings.clone(),
            scheduler: Arc::new(Mutex::new(crate::scheduler::BreakScheduler::new(settings))),
        }
    }
}

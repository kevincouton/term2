use term2_core::SessionManager;

pub struct AppState {
    pub sessions: SessionManager,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: SessionManager::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

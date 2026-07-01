use term2_core::{ProfileRegistry, SessionManager};

pub struct AppState {
    pub sessions: SessionManager,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: SessionManager::new(),
        }
    }

    pub fn registry_for(&self, user: &str) -> ProfileRegistry {
        ProfileRegistry::new(user)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

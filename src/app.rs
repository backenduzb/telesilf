use std::sync::Arc;

use crate::states::manager::StateManager;

#[derive(Clone)]
pub struct AppState {
    pub state_manager: Arc<StateManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            state_manager: Arc::new(StateManager::default()),
        }
    }
}

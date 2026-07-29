use std::sync::Arc;

use crate::states::manager::StateManager;

#[derive(Clone)]
pub struct AppState {
    pub state_manager: Arc<StateManager>,
    pub last_replied_dates: dashmap::DashMap<teloxide::types::ChatId, chrono::NaiveDate>,
}

impl AppState {
	pub async fn has_replied_today(&self, chat_id: teloxide::types::ChatId, today: chrono::NaiveDate) -> bool {
        if let Some(last_date) = self.last_replied_dates.get(&chat_id) {
            *last_date == today
        } else {
            false
        }
    }
	
    pub async fn mark_as_replied_today(&self, chat_id: teloxide::types::ChatId, today: chrono::NaiveDate) {
        self.last_replied_dates.insert(chat_id, today);
    }
    
    pub fn new() -> Self {
        Self {
	        state_manager: Arc::new(StateManager::default()),
	        last_replied_dates: dashmap::DashMap::new(),
        }
    }
}

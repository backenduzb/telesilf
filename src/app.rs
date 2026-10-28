use dashmap::DashMap;
use chrono::NaiveDate;
use teloxide::types::ChatId;

use crate::states::manager::StateManager;

#[derive(Clone)]
pub struct AppState {
    pub state_manager: StateManager,
    pub last_replied_dates: DashMap<ChatId, NaiveDate>,
    pub admin: u64,
}

impl AppState {
    pub fn new(admin: u64) -> Self {
        Self {
            state_manager: StateManager::default(),
            last_replied_dates: DashMap::new(),
            admin,
        }
    }

    pub fn has_replied_today(&self, chat_id: ChatId, today: NaiveDate) -> bool {
        self.last_replied_dates
            .get(&chat_id)
            .map(|d| *d == today)
            .unwrap_or(false)
    }

    pub fn mark_as_replied_today(&self, chat_id: ChatId, today: NaiveDate) {
        self.last_replied_dates.insert(chat_id, today);
    }
}

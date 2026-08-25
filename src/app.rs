use std::sync::Arc;

use crate::states::manager::StateManager;
use teloxide::types::ChatId;
use tokio_util::sync::CancellationToken;

/// Chatdagi faol tug'ilgan kun tabrigi haqida ma'lumot.
#[derive(Clone)]
pub struct BirthdayHandle {
    /// Ushbu tabrikning noyob ID-si (eskisini noto'g'ri o'chirib qo'ymaslik uchun).
    pub id: u64,
    /// `/stopbirthday` yoki yangi tabrik boshlanganda bekor qilish tokeni.
    pub token: CancellationToken,
}

#[derive(Clone)]
pub struct AppState {
    pub state_manager: Arc<StateManager>,
    pub last_replied_dates: dashmap::DashMap<ChatId, chrono::NaiveDate>,
    pub birthdays: dashmap::DashMap<ChatId, BirthdayHandle>,
}

impl AppState {
    pub async fn has_replied_today(&self, chat_id: ChatId, today: chrono::NaiveDate) -> bool {
        if let Some(last_date) = self.last_replied_dates.get(&chat_id) {
            *last_date == today
        } else {
            false
        }
    }

    pub async fn mark_as_replied_today(&self, chat_id: ChatId, today: chrono::NaiveDate) {
        self.last_replied_dates.insert(chat_id, today);
    }

    pub fn new() -> Self {
        Self {
            state_manager: Arc::new(StateManager::default()),
            last_replied_dates: dashmap::DashMap::new(),
            birthdays: dashmap::DashMap::new(),
        }
    }
}

use teloxide::types::{ChatId, UserId, *};

use super::{state::State, story::StoryDraft};

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: UserId,
    pub chat_id: ChatId,
    pub business_connection_id: Option<BusinessConnectionId>,
    pub state: State,
    pub story: StoryDraft,
}

impl Session {
    pub fn new(user_id: UserId, chat_id: ChatId) -> Self {
        Self {
            user_id,
            chat_id,
            business_connection_id: None,
            state: State::Idle,
            story: StoryDraft::default(),
        }
    }

    pub fn reset_story(&mut self) {
        self.story.clear();
        self.state = State::Idle;
    }
}

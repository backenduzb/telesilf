use dashmap::DashMap;
use teloxide::types::{BusinessConnectionId, ChatId, MessageId, UserId};

use super::business::{BusinessMessageSnapshot, business_message_key};
use super::session::Session;

#[derive(Default)]
pub struct StateManager {
    sessions: DashMap<UserId, Session>,
    business_messages: DashMap<String, BusinessMessageSnapshot>,
}

impl StateManager {
    pub fn get_or_create(&self, user: UserId, chat: ChatId) -> Session {
        self.sessions
            .entry(user)
            .or_insert_with(|| Session::new(user, chat))
            .clone()
    }

    pub fn get(&self, user: UserId) -> Option<Session> {
        self.sessions.get(&user).map(|session| session.clone())
    }

    pub fn update(&self, session: Session) {
        self.sessions.insert(session.user_id, session);
    }

    pub fn remove(&self, user: UserId) {
        self.sessions.remove(&user);
    }

    pub fn remember_business_message(&self, snapshot: BusinessMessageSnapshot) {
        self.business_messages
            .insert(snapshot.cache_key(), snapshot);
    }

    pub fn take_business_message(
        &self,
        connection_id: &BusinessConnectionId,
        message_id: MessageId,
    ) -> Option<BusinessMessageSnapshot> {
        self.business_messages
            .remove(&business_message_key(connection_id, message_id))
            .map(|(_, snapshot)| snapshot)
    }

    pub fn clear_business_messages_for_connection(&self, connection_id: &BusinessConnectionId) {
        let prefix = format!("{}:", connection_id.0);
        let keys: Vec<String> = self
            .business_messages
            .iter()
            .filter(|entry| entry.key().starts_with(&prefix))
            .map(|entry| entry.key().clone())
            .collect();

        for key in keys {
            self.business_messages.remove(&key);
        }
    }
}

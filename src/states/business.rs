use teloxide::types::{BusinessConnectionId, ChatId, MessageId, UserId};

#[derive(Debug, Clone)]
pub struct BusinessMessageSnapshot {
    pub business_connection_id: String,
    pub message_id: MessageId,
    pub chat_id: ChatId,
    pub user_id: UserId,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub text: Option<String>,
}

impl BusinessMessageSnapshot {
    pub fn cache_key(&self) -> String {
        format!("{}:{}", self.business_connection_id, self.message_id)
    }

    pub fn display_name(&self) -> String {
        match &self.last_name {
            Some(last_name) if !last_name.trim().is_empty() => {
                format!("{} {}", self.first_name, last_name)
            }
            _ => self.first_name.clone(),
        }
    }
}

pub fn business_connection_key(id: &BusinessConnectionId) -> String {
    id.0.clone()
}

pub fn business_message_key(connection_id: &BusinessConnectionId, message_id: MessageId) -> String {
    format!("{}:{}", business_connection_key(connection_id), message_id)
}

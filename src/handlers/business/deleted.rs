use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::types::{BusinessMessagesDeleted, ChatId};

use crate::app::AppState;

fn format_deleted_business_message(snapshot: &crate::states::business::BusinessMessageSnapshot) -> String {
    let username = snapshot
        .username
        .as_deref()
        .map(|name| format!("@{name}"))
        .unwrap_or_else(|| "username yo'q".to_string());

    let mut text = format!(
        "O'chirilgan xabar topildi\n\nFoydalanuvchi:<b>{}</b>\nUsername: <code>{}</code>\nUser ID: <pre>{}</pre>\nChat ID: <pre>{}</pre>\nMessage ID: {}",
        snapshot.display_name(),
        username,
        snapshot.user_id.0,
        snapshot.chat_id.0,
        snapshot.message_id.0
    );

    if let Some(content) = snapshot.text.as_deref() {
        text.push_str("\nXabar:<i>");
        text.push_str(content);
        text.push_str("</i>");
        
    }

    text
}

pub async fn deleted_business_messages(
    bot: Bot,
    deleted: BusinessMessagesDeleted,
    app: Arc<AppState>,
) -> Result<(), teloxide::RequestError> {
    let mut lines = Vec::new();

    for message_id in deleted.message_ids {
        if let Some(snapshot) = app
            .state_manager
            .take_business_message(&deleted.business_connection_id, message_id)
        {
            lines.push(format_deleted_business_message(&snapshot));
        }
    }

    if lines.is_empty() {
        return Ok(());
    }

    bot.send_message(6400925437, lines.join("\n\n---\n\n"))
        .await?;

    Ok(())
}

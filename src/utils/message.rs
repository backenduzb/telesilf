use std::time::Duration;
use teloxide::errors::RequestError;
use teloxide::types::{BusinessConnectionId, MessageId};
use teloxide::{prelude::*, requests::Requester};
use tokio::time::sleep;

pub async fn stream_bs_text(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: String,
    business_connection_id: Option<&str>,
) -> Result<(), RequestError> {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return Ok(());
    }

    let mut current = String::new();
    for (index, word) in words.iter().enumerate() {
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);

        let mut req = bot.edit_message_text(chat_id, message_id, current.clone());
        if let Some(id) = business_connection_id {
            req = req.business_connection_id(BusinessConnectionId(id.to_owned()));
        }
        req.await?;

        if index + 1 != words.len() {
            sleep(Duration::from_millis(150)).await;
        }
    }

    Ok(())
}

pub async fn stream_text(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: String,
) -> Result<(), RequestError> {
    stream_bs_text(bot, chat_id, message_id, text, None).await
}

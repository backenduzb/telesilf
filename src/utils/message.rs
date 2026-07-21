use std::time::Duration;
use teloxide::errors::RequestError;
use teloxide::types::MessageId;
use teloxide::{prelude::*, requests::Requester};
use tokio::time::sleep;

async fn stream_text_inner(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: String,
    business_connection_id: Option<&str>,
) -> Result<(), RequestError> {
    let mut current = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return Ok(());
    }

    for (index, word) in words.iter().enumerate() {
        if !current.is_empty() {
            current.push(' ');
        }

        current.push_str(word);

        let mut req = bot.edit_message_text(chat_id, message_id, current.clone());
        if let Some(id) = business_connection_id {
            req = req.business_connection_id(teloxide::types::BusinessConnectionId(id.to_owned()));
        }

        req.await?;

        if index + 1 != words.len() {
            sleep(Duration::from_millis(25)).await;
        }
    }

    Ok(())
}

pub async fn stream_bs_text(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: String,
    business_connection_id: Option<&str>,
) -> Result<(), RequestError> {
    stream_text_inner(bot, chat_id, message_id, text, business_connection_id).await
}

pub async fn stream_text(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: String,
) -> Result<(), RequestError> {
    stream_text_inner(bot, chat_id, message_id, text, None).await
}

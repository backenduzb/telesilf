use std::time::{Duration, Instant};
use teloxide::types::MessageId;
use teloxide::errors::RequestError;
use teloxide::{
    prelude::*,
    requests::Requester,
};
use tokio::time::sleep;

pub async fn stream_text(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: String,
    business_connection_id: Option<&str>,
) -> Result<(), RequestError> {
    let mut current = String::new();

    for word in text.split_whitespace() {
        if !current.is_empty() {
            current.push(' ');
        }

        current.push_str(word);

        let mut req = bot.edit_message_text(chat_id, message_id, current.clone());

        if let Some(id) = business_connection_id {
            req = req.business_connection_id(
                teloxide::types::BusinessConnectionId(id.to_owned()),
            );
        }

        req.await?;

        sleep(Duration::from_millis(90)).await;
    }

    Ok(())
}
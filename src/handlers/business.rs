use teloxide::prelude::*;
use teloxide::types::BusinessConnectionId;
use teloxide::RequestError;
use teloxide::types::MessageKind;

pub async fn send_business_reply(
    bot: Bot,
    connection_id: BusinessConnectionId,
    customer_chat_id: ChatId,
) -> Result<(), RequestError> {
    bot.send_message(customer_chat_id, "Reply")
        .business_connection_id(connection_id)
        .await?;

    Ok(())
}


pub async fn test(bot: Bot, msg: Message) -> Result<(), RequestError> {
    if let MessageKind::Common(ref msg_common) = msg.kind {
        if let Some(ref biz_id) = msg_common.business_connection_id {
            bot.send_message(msg.chat.id, "bussinessssssssssssssssssssssss")
                .business_connection_id(biz_id.clone())
                .await?;
        }
    }

    Ok(())
}
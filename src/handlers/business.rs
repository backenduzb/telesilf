use teloxide::prelude::*;
use teloxide::types::BusinessConnectionId;
use teloxide::RequestError;
use teloxide::types::MessageKind;
use crate::utils::message::stream_text;

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
	if let MessageKind::Common(ref common) = msg.kind {
        if let Some(biz_id) = &common.business_connection_id {

            if let Some(text) = msg.text() {
            	if let Some(user) = &msg.from {
                    if user.id == UserId(6400925437) {
                            return Ok(()); 
                        }
                }
            	let name = msg
                        .from
                        .as_ref()
                        .map(|u| u.first_name.as_str())
                        .unwrap_or("do'stim");
            	let mut req = bot.send_message(msg.chat.id, "▌");
             
	             if let Some(id) = &common.business_connection_id {
	                 req = req.business_connection_id(id.clone());
	             }
             
             	let sent = req.await?;
                if text.contains("salom") {
	                stream_text(
	                    &bot,
	                    msg.chat.id,
	                    sent.id,
	                    format!("Assalomu alaykum {}! Javohir o'rniga hozir men javobberib turubman marhamat nima kerak bo'lsa so'rashingiz mumkin.", name),
	                    Some(biz_id.0.as_str()),
	                )
	                .await?;
                }
            }
        }
    }

    Ok(())
}
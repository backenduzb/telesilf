use crate::app::AppState;
use crate::handlers::business::connection::{
    remember_business_message_from_message, save_business_connection_from_message,
};
use crate::utils::message::stream_bs_text;
use teloxide::prelude::*;
use teloxide::RequestError;
use teloxide::types::{MessageKind, UserId};
use std::sync::Arc;

pub async fn business_start(
    bot: Bot,
    msg: Message,
    app: Arc<AppState>,
) -> Result<(), RequestError> {
    save_business_connection_from_message(&bot, &msg, &app).await?;
    remember_business_message_from_message(&msg, &app);

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
                let mut req = bot.send_message(msg.chat.id, "...");

                if let Some(id) = &common.business_connection_id {
                    req = req.business_connection_id(id.clone());
                }

                let sent = req.await?;
                if text.contains("salom") {
                    stream_bs_text(
	                    &bot,
	                    msg.chat.id,
	                    sent.id,
	                    format!("Assalomu alaykum {}! hozir men javobberib turubman marhamat nima kerak bo'lsa so'rashingiz mumkin.", name),
	                    Some(biz_id.0.as_str()),
	                )
	                .await?;
                }
            }
        }
    }

    Ok(())
}

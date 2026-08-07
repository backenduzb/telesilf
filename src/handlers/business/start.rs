use crate::app::AppState;
use crate::handlers::business::connection::{
    remember_business_message_from_message, save_business_connection_from_message,
};
use crate::config::settings::Config;
use crate::utils::message::stream_bs_text;
use crate::utils::greeting::is_greeting_or_appeal; 
use chrono::{ Local, TimeZone, Utc};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{MessageKind, UserId};
use teloxide::RequestError;
use crate::handlers::business::info::info_getter;

pub async fn business_start(
    bot: Bot,
    msg: Message,
    app: Arc<AppState>,
) -> Result<(), RequestError> {
	let config = Config::from_env();
    save_business_connection_from_message(&bot, &msg, &app).await?;

    if let MessageKind::Common(ref common) = msg.kind {
        if let Some(biz_id) = &common.business_connection_id {
            if let Some(text) = msg.text() {
                if let Some(user) = &msg.from {
                    if user.id == UserId(config.admin) {
                        remember_business_message_from_message(&msg, &app);
                        return Ok(());
                    }
                }
    
                info_getter(&bot, &msg).await?;
    
                let msg_date_utc = msg.date;
                let msg_date_local = Utc
                    .timestamp_opt(msg_date_utc.timestamp(), 0)
                    .single()
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(Local::now);
    
                let today = Local::now().date_naive();
                let is_today = msg_date_local.date_naive() == today;
    
                let is_first_message_today =
                    is_today && !app.has_replied_today(msg.chat.id, today).await;
    
                remember_business_message_from_message(&msg, &app);
    
                if is_first_message_today && is_greeting_or_appeal(text) {
                    app.mark_as_replied_today(msg.chat.id, today).await;
    
                    let name = msg
                        .from
                        .as_ref()
                        .map(|u| u.first_name.as_str())
                        .unwrap_or("do'stim");
    
                    let mut req = bot.send_message(msg.chat.id, "...");
                    req = req.business_connection_id(biz_id.clone());
    
                    let sent = req.await?;
    
                    stream_bs_text(
                        &bot,
                        msg.chat.id,
                        sent.id,
                        format!(
                            "Assalomu alaykum {}! Hozir men javob berib turibman, marhamat nima kerak bo'lsa so'rashingiz mumkin.\n\nMisol: Silf portfolioni ko'rsat",
                            name
                        ),
                        Some(biz_id.0.as_str()),
                    )
                    .await?;
                }
            }
        }
    }

    Ok(())
}
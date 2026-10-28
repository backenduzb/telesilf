use crate::app::AppState;
use crate::handlers::business::connection::{
    remember_business_message_from_message, save_business_connection_from_message,
};
use crate::handlers::business::info::info_getter;
use crate::utils::greeting::is_greeting_or_appeal;
use crate::utils::message::stream_bs_text;
use chrono::{Local, TimeZone, Utc};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{MessageKind, UserId};
use teloxide::RequestError;

pub async fn business_start(
    bot: Bot,
    msg: Message,
    app: Arc<AppState>,
) -> Result<(), RequestError> {
    save_business_connection_from_message(&bot, &msg, &app).await?;

    let MessageKind::Common(ref common) = msg.kind else {
        return Ok(());
    };
    let Some(biz_id) = &common.business_connection_id else {
        return Ok(());
    };
    let Some(text) = msg.text() else {
        return Ok(());
    };

    if msg.from.as_ref().map(|u| u.id) == Some(UserId(app.admin)) {
        remember_business_message_from_message(&msg, &app);
        return Ok(());
    }

    info_getter(&bot, &msg).await?;

    let msg_date_local = Utc
        .timestamp_opt(msg.date.timestamp(), 0)
        .single()
        .map(|dt| dt.with_timezone(&Local))
        .unwrap_or_else(Local::now);

    let today = Local::now().date_naive();
    let is_first_message_today =
        msg_date_local.date_naive() == today && !app.has_replied_today(msg.chat.id, today);

    remember_business_message_from_message(&msg, &app);

    if is_first_message_today && is_greeting_or_appeal(text) {
        app.mark_as_replied_today(msg.chat.id, today);

        let name = msg
            .from
            .as_ref()
            .map(|u| u.first_name.as_str())
            .unwrap_or("do'stim");

        let sent = bot
            .send_message(msg.chat.id, "...")
            .business_connection_id(biz_id.clone())
            .await?;

        stream_bs_text(
            &bot,
            msg.chat.id,
            sent.id,
            format!(
                "Assalomu alaykum {}! Hozir men javob berib turibman, marhamat nima kerak bo'lsa so'rashingiz mumkin.",
                name
            ),
            Some(biz_id.0.as_str()),
        )
        .await?;
    }

    Ok(())
}

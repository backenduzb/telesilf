use std::sync::Arc;

use crate::app::AppState;
use crate::services::birthday;
use teloxide::prelude::*;
use teloxide::RequestError;

/// `/birthday_for Ism` — ism uchun tug'ilgan kun tabrigini boshlaydi.
pub async fn handle_birthday_for(
    bot: &Bot,
    msg: &Message,
    app: &Arc<AppState>,
    name: String,
) -> Result<(), RequestError> {
    let name = name.trim();

    if name.is_empty() {
        bot.send_message(
            msg.chat.id,
            "Tabriklash uchun ism yozing, masalan:\n/birthday_for Aziz",
        )
        .await?;
        return Ok(());
    }

    let name: String = name.chars().take(20).collect();

    bot.send_message(
        msg.chat.id,
        format!(
            "🎉 {name} uchun tug'ilgan kun tabrigi boshlandi!\nTo'xtatish uchun: /stopbirthday"
        ),
    )
    .await?;

    birthday::start_birthday(app, bot.clone(), msg.chat.id, name);

    Ok(())
}

/// `/stopbirthday` — chatdagi faol tabrikni to'xtatadi.
pub async fn handle_stop_birthday(
    bot: &Bot,
    msg: &Message,
    app: &Arc<AppState>,
) -> Result<(), RequestError> {
    if birthday::stop_birthday(app, msg.chat.id) {
        bot.send_message(msg.chat.id, "✅ Tabriklash to'xtatildi.")
            .await?;
    } else {
        bot.send_message(msg.chat.id, "Hozircha faol tabriklash yo'q edi 🤷")
            .await?;
    }

    Ok(())
}

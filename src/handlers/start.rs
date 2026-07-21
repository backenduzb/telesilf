use crate::app::AppState;
use crate::handlers::business::connection::save_business_connection_from_message;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "list")]
pub enum Command {
    #[command(description = "start")]
    Start,
}

pub async fn start(
    bot: Bot,
    msg: Message,
    cmd: Command,
    app: std::sync::Arc<AppState>,
) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            save_business_connection_from_message(&bot, &msg, &app).await?;
            let name = msg
                .from
                .as_ref()
                .map(|u| u.first_name.clone())
                .unwrap_or_else(|| "Foydalanuvchi".to_string());

            bot.send_message(msg.chat.id, format!("Assalomu alaykum, {name}!"))
                .await?;
        }
    }
    Ok(())
}

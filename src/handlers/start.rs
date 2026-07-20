use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
pub enum Command {
    Start,
}

pub async fn start(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(
                msg.chat.id, 
                "Salom! Men Silf AI botman. Sizga qanday yordam bera olaman?"
            ).await?;
        }
    }
    Ok(())
}
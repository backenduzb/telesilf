use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Mavjud buyruqlar:")]
pub enum Command {
    #[command(description = "Silf botni ishga tushirish")]
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
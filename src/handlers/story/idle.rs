use crate::states::{session::Session, state::State};
use crate::utils::message::stream_text;
use teloxide::prelude::*;

pub async fn handle_idle(
    bot: &Bot,
    msg: &Message,
    session: &mut Session,
) -> Result<(), teloxide::RequestError> {
    let Some(text) = msg.text() else {
        return Ok(());
    };

    let text = text.to_lowercase();

    if text.contains("story") || text.contains("hikoya") {
        session.state = State::WaitingStoryMedia;
        let req = bot.send_message(msg.chat.id, "...").await?;

        stream_text(
            bot,
            msg.chat.id,
            req.id,
            "Marhamat storyni profilingizga yuklay olaman. Rasm yoki video yuboring."
                .to_string(),
        )
        .await?;
    }

    Ok(())
}

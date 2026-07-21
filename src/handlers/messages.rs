use crate::handlers::{
    story::idle::handle_idle,
    story::story::{
        handle_waiting_story_caption, handle_waiting_story_confirm, handle_waiting_story_media,
    },
};
use crate::{app::AppState, states::state::State};
use std::sync::Arc;
use teloxide::prelude::*;

pub async fn message_handler(bot: Bot, msg: Message, app: Arc<AppState>) -> ResponseResult<()> {
    let Some(user) = msg.from.as_ref() else {
        return Ok(());
    };

    let mut session = app.state_manager.get_or_create(user.id, msg.chat.id);

    match session.state {
        State::Idle => {
            handle_idle(&bot, &msg, &mut session).await?;
        }

        State::WaitingStoryMedia => {
            handle_waiting_story_media(&bot, &msg, &mut session).await?;
        }

        State::WaitingStoryCaption => {
            handle_waiting_story_caption(&bot, &msg, &mut session).await?;
        }

        State::WaitingStoryConfirm => {
            handle_waiting_story_confirm(&bot, &msg, &mut session).await?;
        }
    }

    app.state_manager.update(session);

    Ok(())
}

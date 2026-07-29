use crate::services::story::{post_story_multipart, prepare_story_content};
use crate::states::{session::Session, state::State, story::StoryMedia};
use crate::utils::message::stream_text;
use teloxide::prelude::*;

pub async fn handle_waiting_story_media(
    bot: &Bot,
    msg: &Message,
    session: &mut Session,
) -> Result<(), teloxide::RequestError> {
    if let Some(photo) = msg.photo() {
        session.story.media = Some(StoryMedia::Photo(photo.last().unwrap().file.id.clone()));
    } else if let Some(video) = msg.video() {
        session.story.media = Some(StoryMedia::Video(video.file.id.clone()));
    } else {
        let req = bot.send_message(msg.chat.id, "...").await?;

        stream_text(
            bot,
            msg.chat.id,
            req.id,
            "Iltimos rasm yoki video yuboring.".to_string(),
        )
        .await?;

        return Ok(());
    }

    session.story.caption = msg.caption().map(ToOwned::to_owned);
    session.state = State::WaitingStoryAction;

    let req = bot.send_message(msg.chat.id, "...").await?;

    stream_text(
        bot,
        msg.chat.id,
        req.id,
        "Bu rasm yoki videoni nima qilay? Storyga yuklaymi?".to_string(),
    )
    .await?;

    Ok(())
}

pub async fn handle_waiting_story_action(
    bot: &Bot,
    msg: &Message,
    session: &mut Session,
) -> Result<(), teloxide::RequestError> {
    let Some(text) = msg.text() else {
        return Ok(());
    };

    match text.trim().to_lowercase().as_str() {
        "ha" | "xa" | "story" | "storyga yukla" | "yukla" | "yuklayver" => {
            if let Some(caption) = session.story.caption.as_deref() {
                session.state = State::WaitingStoryCaptionDecision;
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "{}\n\nShu captionni ishlataymi?",
                        caption
                    ),
                )
                .await?;
            } else {
                session.state = State::WaitingStoryCaption;
                bot.send_message(msg.chat.id, "Caption yozib yuboring.")
                    .await?;
            }
        }
        "yo'q" | "yoq" | "yo‘q" | "bekor" | "bekor qil" => {
            session.reset_story();
            bot.send_message(msg.chat.id, "Mayli, bu media storyga yuklanmaydi.")
                .await?;
        }
        _ => {
            bot.send_message(
                msg.chat.id,
                "Uzur gapignizga tushuna olmadim 😅",
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn handle_waiting_story_caption_decision(
    bot: &Bot,
    msg: &Message,
    session: &mut Session,
) -> Result<(), teloxide::RequestError> {
    let Some(text) = msg.text() else {
        return Ok(());
    };

    match text.trim().to_lowercase().as_str() {
        "ha" | "xa" | "olaver" | "ishlat" | "ishlataver" => {
            session.state = State::WaitingStoryConfirm;
            bot.send_message(msg.chat.id, "Shu caption bilan yuklayveraymi?")
                .await?;
        }
        "yo'q" | "yoq" | "yo‘q" => {
            session.story.caption = None;
            session.state = State::WaitingStoryCaption;
            bot.send_message(msg.chat.id, "Unaqada yangi caption yozibbering.")
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Videoni tagidagi captionni ishlataveraymi?")
                .await?;
        }
    }

    Ok(())
}

pub async fn handle_waiting_story_caption(
    bot: &Bot,
    msg: &Message,
    session: &mut Session,
) -> Result<(), teloxide::RequestError> {
    let Some(text) = msg.text() else {
        let req = bot.send_message(msg.chat.id, "...").await?;

        stream_text(
            bot,
            msg.chat.id,
            req.id,
            "Descriptionda faqat matn bo'ladiku matn yuboring.".to_string(),
        )
        .await?;

        return Ok(());
    };

    session.story.caption = Some(text.to_owned());

    session.state = State::WaitingStoryConfirm;

    let req = bot.send_message(msg.chat.id, "...").await?;

    stream_text(
        bot,
        msg.chat.id,
        req.id,
        "Yuklayveraymiz tekshirdingizmi? hammasini?".to_string(),
    )
    .await?;

    Ok(())
}

pub async fn handle_waiting_story_confirm(
    bot: &Bot,
    msg: &Message,
    session: &mut Session,
) -> Result<(), teloxide::RequestError> {
    let Some(text) = msg.text() else {
        return Ok(());
    };

    match text.trim().to_lowercase().as_str() {
        "ha" => {
            let Some(business_connection_id) = session.business_connection_id.clone() else {
                bot.send_message(msg.chat.id, "Uzur storyni uplaod qila olmaymas ekanmanda")
                    .await?;

                return Ok(());
            };

            let Some(media) = session.story.media.as_ref() else {
                bot.send_message(msg.chat.id, "malumotlarini topolmadim")
                    .await?;

                return Ok(());
            };

            let prepared = prepare_story_content(bot, media).await?;

            let active_period = session.story.active_period();

            post_story_multipart(
                bot,
                business_connection_id,
                &prepared,
                active_period,
                session.story.caption.as_deref(),
            )
            .await?;

            session.reset_story();

            bot.send_message(
                msg.chat.id,
                "Storyni profilingizga yuklab qo'ydim ko'rishingiz mumkin.",
            )
            .await?;
        }

        "yo'q" => {
            session.reset_story();

            bot.send_message(msg.chat.id, "ignor qildim").await?;
        }

        _ => {
            bot.send_message(msg.chat.id, "Aniq bir qaroringizni ayting.")
                .await?;
        }
    }

    Ok(())
}

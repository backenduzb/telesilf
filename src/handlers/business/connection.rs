use std::sync::Arc;

use teloxide::{
    prelude::*,
    types::{BusinessConnection, BusinessConnectionId, MessageKind},
};

use crate::app::AppState;
use crate::states::business::BusinessMessageSnapshot;

fn sync_session_from_connection(connection: &BusinessConnection, app: &Arc<AppState>) {
    let mut session = app
        .state_manager
        .get_or_create(connection.user.id, ChatId::from(connection.user_chat_id));

    session.chat_id = ChatId::from(connection.user_chat_id);

    if connection.is_enabled {
        session.business_connection_id = Some(connection.id.clone());
    } else {
        session.business_connection_id = None;
        session.reset_story();
    }

    app.state_manager.update(session);
}

pub async fn save_business_connection(connection: &BusinessConnection, app: &Arc<AppState>) {
    sync_session_from_connection(connection, app);
}

pub fn remember_business_message_from_message(msg: &Message, app: &Arc<AppState>) {
    let Some(user) = msg.from.as_ref() else {
        return;
    };

    let MessageKind::Common(common) = &msg.kind else {
        return;
    };

    let Some(connection_id) = &common.business_connection_id else {
        return;
    };

    let text = msg
        .text()
        .map(ToOwned::to_owned)
        .or_else(|| msg.caption().map(ToOwned::to_owned));

    let snapshot = BusinessMessageSnapshot {
        business_connection_id: connection_id.0.clone(),
        message_id: msg.id,
        chat_id: msg.chat.id,
        user_id: user.id,
        username: user.username.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        text,
    };

    app.state_manager.remember_business_message(snapshot);
}

async fn fetch_and_save_business_connection(
    bot: &Bot,
    connection_id: BusinessConnectionId,
    app: &Arc<AppState>,
) -> Result<(), teloxide::RequestError> {
    let connection = bot.get_business_connection(connection_id).await?;
    sync_session_from_connection(&connection, app);
    Ok(())
}

pub async fn save_business_connection_from_message(
    bot: &Bot,
    msg: &Message,
    app: &Arc<AppState>,
) -> Result<(), teloxide::RequestError> {
    let Some(user) = msg.from.as_ref() else {
        return Ok(());
    };

    let MessageKind::Common(common) = &msg.kind else {
        return Ok(());
    };

    let Some(connection_id) = &common.business_connection_id else {
        return Ok(());
    };

    if let Some(session) = app.state_manager.get(user.id) {
        if session.business_connection_id.as_ref() == Some(connection_id) {
            return Ok(());
        }
    }

    fetch_and_save_business_connection(bot, connection_id.clone(), app).await
}

pub async fn clear_business_connection(connection: &BusinessConnection, app: &Arc<AppState>) {
    let mut session = app
        .state_manager
        .get_or_create(connection.user.id, ChatId::from(connection.user_chat_id));

    session.chat_id = ChatId::from(connection.user_chat_id);

    if !connection.is_enabled {
        session.business_connection_id = None;
        session.reset_story();
        app.state_manager
            .clear_business_messages_for_connection(&connection.id);
    }

    app.state_manager.update(session);
}

pub async fn save_business_connection_from_connection(
    connection: BusinessConnection,
    app: Arc<AppState>,
) -> Result<(), teloxide::RequestError> {
    if connection.is_enabled {
        save_business_connection(&connection, &app).await;
        return Ok(());
    };

    clear_business_connection(&connection, &app).await;
    Ok(())
}

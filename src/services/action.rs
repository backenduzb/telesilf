use std::{future::Future, time::Duration}
use teloxide::{
	prelude::*,
	types::ChatAction,
};
use tokio_utill::sync::CancellationToken;

pub async fn with_chat_action<F, T>(
	bot: Bot,
	chat_id: ChatId,
	action: ChatAction,
	future: F,
) -> T where F: Future<Output = T> {
	let cancel = CancellationToken::new();
	let token = cancel.clone();
	let bot2 = bot.clone();

	let handle = tokio::spawn(async move {
		while !token.is_cancelled() {
			let _ = bot2.send_chat_action(chat_id, action.clone()).await;
			tokio::time::sleep(Duration::from_secs(4)).await;
		}
	});
	let result = future.await;

	cancel.cancel();
	let _ = handle.await;

	result
}
use std::sync::Arc;

use crate::app::AppState;
use crate::config::settings::Config;
use crate::routes::set::setup_router;
use teloxide::prelude::*;

pub async fn run_polling(bot: Bot, _config: Config, app: Arc<AppState>) {
    log::info!("Bot Polling rejimida ishga tushmoqda...");

    Dispatcher::builder(bot, setup_router())
        .dependencies(dptree::deps![app])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn run_webhook(bot: Bot, config: Config, app: Arc<AppState>) {
    log::info!("Bot Webhook rejimida ishga tushmoqda...");

    let url_str = config
        .webhook_url
        .expect("Webhook uchun WEBHOOK_URL kiritilmagan!");
    let url = url_str.parse().expect("WEBHOOK_URL formati noto'g'ri");

    let port_num: u16 = config.port.parse().unwrap_or(8080);
    println!("PORT = {}", config.port);
    let address = ([0, 0, 0, 0], port_num).into();

    let listener = teloxide::update_listeners::webhooks::axum(
        bot.clone(),
        teloxide::update_listeners::webhooks::Options::new(address, url),
    )
    .await
    .expect("Webhook tinglovchisini yaratib bo'lmadi");

    Dispatcher::builder(bot, setup_router())
        .dependencies(dptree::deps![app])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("Webhookda xatolik ketdi"),
        )
        .await;
}

use teloxide::prelude::*;
use crate::config::settings::Config; 
use crate::routes::set::setup_router;

pub async fn run_polling(bot: Bot, _config: Config) {
    log::info!("Bot Polling rejimida ishga tushmoqda...");
    
    Dispatcher::builder(bot, setup_router())
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn run_webhook(bot: Bot, config: Config) {
    log::info!("Bot Webhook rejimida ishga tushmoqda...");
    
    let url_str = config.webhook_url.expect("Webhook uchun WEBHOOK_URL kiritilmagan!");
    let url = url_str.parse().expect("WEBHOOK_URL formati noto'g'ri");
    
    let port_num: u16 = config.port.parse().unwrap_or(8080);
    let address = ([127, 0, 0, 1], port_num).into();
    
    let listener = teloxide::update_listeners::webhooks::axum(
        bot.clone(),
        teloxide::update_listeners::webhooks::Options::new(address, url),
    )
    .await
    .expect("Webhook tinglovchisini yaratib bo'lmadi");

    Dispatcher::builder(bot, setup_router())
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("Webhookda xatolik ketdi"), 
        )
        .await;
}
mod app;
mod config;
mod handlers;
mod routes;
mod services;
mod states;
mod utils;

use config::settings::Config;
use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::prelude::*;
use utils::run::{run_polling, run_webhook};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let app = Arc::new(app::AppState::new());
    log::info!("Silf boti tayyorlanmoqda...");

    let config = Config::from_env();

    let bot = Bot::new(&config.bot_token);

    if config.debug {
        run_polling(bot, config, app).await;
    } else {
        run_webhook(bot, config, app).await;
    }
}

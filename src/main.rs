mod handlers;
mod utils;
mod config;
mod routes;

use dotenvy::dotenv;
use teloxide::prelude::*;
use config::settings::Config;
use utils::run::{run_polling, run_webhook};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    
    log::info!("Silf boti tayyorlanmoqda...");

    let config = Config::from_env();
    
    let bot = Bot::new(&config.bot_token);

    if config.debug {
        run_polling(bot, config).await;
    } else {
        run_webhook(bot, config).await;
    }
}
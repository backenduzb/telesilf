use std::env;

#[derive(Clone)]
pub struct Config {
    pub bot_token: String,
    pub debug: bool,
    pub webhook_url: Option<String>,
    pub port: String,
    pub admin: String,
}

impl Config {
    pub fn from_env() -> Self {
        let debug = env::var("DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        Self {
            bot_token: env::var("BOT_TOKEN").expect("BOT_TOKEN topilmadi!"),
            debug,
            webhook_url: env::var("WEBHOOK_URL").ok(),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            admin: env::var("admin").unwrap_or_else(|_| "6400925437".to_string()),
        }
    }
}

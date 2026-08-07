use std::env;

#[derive(Clone)]
pub struct Config {
    pub bot_token: String,
    pub debug: bool,
    pub webhook_url: Option<String>,
    pub port: String,
    pub admin: u64,
}

impl Config {
    pub fn from_env() -> Self {
        let debug = env::var("DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        let admin = env::var("ADMIN")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(6400925437);
        Self {
            bot_token: env::var("BOT_TOKEN").expect("BOT_TOKEN topilmadi!"),
            debug,
            webhook_url: env::var("WEBHOOK_URL").ok(),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            admin,
        }
    }
}

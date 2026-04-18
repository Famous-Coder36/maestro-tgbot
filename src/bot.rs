use reqwest::Client;
use std::collections::HashMap;
use std::sync::OnceLock;

static BOT: OnceLock<TelegramBot> = OnceLock::new();

#[derive(Debug)]
pub struct TelegramBot {
    pub token: String,
    pub client: Client,
}

impl TelegramBot {

    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    pub fn init(token: &str) -> Result<(), &'static str> {
    let bot = TelegramBot::new(token.to_string());

    BOT.set(bot)
        .map_err(|_| "Bot already initialized")?;

    Ok(())
}

    pub fn get() -> &'static TelegramBot {
        BOT.get().expect("Bot not initialized")
    }

    pub async fn bot(
        &self,
        method: &str,
        data: HashMap<String, String>,
    ) -> Result<String, reqwest::Error> {

        let url = format!(
            "https://api.telegram.org/bot{}/{}",
            self.token, method
        );

        let res = self.client
            .post(&url)
            .form(&data)
            .send()
            .await?;

        Ok(res.text().await?)
    }
}


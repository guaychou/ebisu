pub mod message;

use {
    crate::configuration::TelegramConfig,
    crate::error::AppError,
    getset::Getters,
    message::Message,
    reqwest::Client,
    serde::{Deserialize, Serialize},
    serde_json::json,
    tokio::time::Duration,
};
#[derive(Clone)]
pub struct Telegram {
    config: TelegramConfig,
    client: Client,
    bot_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramResponseOkResult {
    message_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramResponseOk {
    ok: bool,
    result: TelegramResponseOkResult,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct TelegramResponseError {
    ok: bool,
    #[getset(get = "pub with_prefix")]
    error_code: u16,
    #[getset(get = "pub with_prefix")]
    description: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TelegramResponse {
    Ok(TelegramResponseOk),
    Err(TelegramResponseError),
}

impl Telegram {
    pub fn new(config: TelegramConfig) -> Self {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let timeout_duration = Duration::new(5, 0);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .connect_timeout(timeout_duration)
            .http2_adaptive_window(true)
            .build()
            .unwrap_or_default();
        let bot_url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            config.get_token()
        );
        return Self {
            config: config,
            client: client,
            bot_url: bot_url,
        };
    }
    pub async fn send_alert(
        &self,
        service: String,
        message: Option<String>,
    ) -> Result<TelegramResponseOk, AppError> {
        let message = message.unwrap_or_else(|| String::from("Restarted, Please check"));
        let text = format!(
            "***ALERT 🔥 🔥 🔥***\n\n***Service***    : {}\n***Message*** : {}",
            service, message
        );
        let message: Message = Message {
            chat_id: self.config.get_chat_id().to_string(),
            text: text,
            parse_mode: String::from("MarkdownV2"),
            disable_notification: false,
        };
        match self
            .client
            .post(self.bot_url.to_string())
            .json(&json!(message))
            .send()
            .await?
            .json::<TelegramResponse>()
            .await?
        {
            TelegramResponse::Ok(data) => return Ok(data),
            TelegramResponse::Err(err) => return Err(err.into()),
        }
    }
}

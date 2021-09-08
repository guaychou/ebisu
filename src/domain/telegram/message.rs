use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub chat_id: String,
    pub text: String,
    pub disable_notification: bool,
    pub parse_mode: String,
}

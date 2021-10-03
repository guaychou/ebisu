use crate::domain::telegram::Telegram;
use crate::error::AppError;
use axum::{extract::Extension, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use validator::*;
use tracing::instrument;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestBody {
    #[validate(length(min = 3, max = 20))]
    service: String,
    #[validate(length(min = 5))]
    message: String,
}

#[instrument(name = "alert_message_handler" skip(telegram))]
pub async fn alert_with_message(
    Json(req): Json<RequestBody>,
    Extension(telegram): Extension<Telegram>,
) -> Result<Json<Value>, AppError> {
    req.validate()?;
    let data = telegram
        .send_alert(
            req.service
                .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), ""),
            Some(
                req.message
                    .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), ""),
            ),
        )
        .await?;
    Ok(json!(data).into())
}

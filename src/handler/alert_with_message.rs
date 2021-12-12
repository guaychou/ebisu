use crate::domain::telegram::Telegram;
use crate::error::AppError;
use axum::{extract::Extension, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::instrument;
use validator::*;

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
    let data = telegram.send_alert(req.service, Some(req.message)).await?;
    Ok(json!(data).into())
}

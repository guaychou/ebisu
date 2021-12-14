use crate::domain::telegram::Telegram;
use crate::error::AppError;
use crate::extractor::JsonExtractor;
use axum::{extract::Extension, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::instrument;
use validator::*;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestBody {
    #[validate(length(min = 3, max = 20))]
    service: String,
}

#[instrument(name = "alert_handler" skip(telegram))]
pub async fn alert(
    JsonExtractor(req): JsonExtractor<RequestBody>,
    Extension(telegram): Extension<Telegram>,
) -> Result<Json<Value>, AppError> {
    req.validate()?;
    let data = telegram.send_alert(req.service, None).await?;
    Ok(json!(data).into())
}

use crate::domain::telegram::Telegram;
use crate::error::AppError;
use axum::{extract::Extension, Json};
use log::info;
use serde::Deserialize;
use serde_json::{json, Value};
use validator::*;
use tracing::instrument;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestBody {
    #[validate(length(min = 3, max = 20))]
    service: String,
}


#[instrument(name = "alert_handler" skip(telegram))]
pub async fn alert(
    Json(req): Json<RequestBody>,
    Extension(telegram): Extension<Telegram>,
) -> Result<Json<Value>, AppError> {
    info!("Getting this data {:#?}", req);
    req.validate()?;
    let data = telegram
        .send_alert(
            req.service
                .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), ""),
            None,
        )
        .await?;
    Ok(json!(data).into())
}

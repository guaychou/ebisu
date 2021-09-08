use std::convert::Infallible;

use axum::{Json, body::{Bytes, Full}, http::{Response}, http::StatusCode, response::IntoResponse};

use crate::domain::telegram::TelegramResponseError;
use serde_json::json;
use validator::ValidationErrors;


#[derive(Debug)]
pub enum AppError {
    Validation(ValidationErrors),
    TelegramError(TelegramResponseError),
}

// /// This makes it possible to use `?` to automatically convert a `UserRepoError`
// /// into an `AppError`.
impl From<ValidationErrors> for AppError {
    fn from(inner: ValidationErrors) -> Self {
        AppError::Validation(inner)
    }
}

impl From<TelegramResponseError> for AppError {
    fn from(inner: TelegramResponseError) -> Self {
        AppError::TelegramError(inner)
    }
}

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, json!(e)),
            AppError::TelegramError(e) => (
                StatusCode::from_u16(*e.get_error_code()).unwrap_or_default(),
                json!(e),
            ),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

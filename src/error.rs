use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::Response,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::domain::telegram::TelegramResponseError;
use reqwest::Error as RequestError;
use serde_json::json;
use tower::BoxError;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    Validation(ValidationErrors),
    TelegramError(TelegramResponseError),
    RequestError(RequestError),
}

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

impl From<RequestError> for AppError {
    fn from(inner: RequestError) -> Self {
        AppError::RequestError(inner)
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
            AppError::RequestError(_) => { 
                (StatusCode::INTERNAL_SERVER_ERROR, json!("Request to telegram api is failed.")) 
            
            },
        };
        let body = Json(json!({
            "code" : status.as_u16(),
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "code" : 408,
                "error" : "Uhh ohh, request time out",
            })),
        ));
    };
    if error.is::<tower::load_shed::error::Overloaded>() {
        return Ok((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "code" : 503,
                "error" : "Uhh ohh, service unavailable",
            })),
        ));
    }

    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "code" : 500,
            "error" : "Uhh ohh, unhandled internal error",
        })),
    ))
}

use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
pub use crate::handlers::JsonExtractor;

pub enum AppError {
    JsonRejection(JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                (rejection.status(), rejection.body_text())
            }
        };

        (status, JsonExtractor(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

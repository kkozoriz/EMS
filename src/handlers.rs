use axum::response::{IntoResponse, Response};
use axum_macros::FromRequest;
use serde::Deserialize;

pub mod post_notifications;
pub mod post_templates;
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(super::errors::AppError))]
pub struct JsonExtractor<T>(pub T);

impl<T> IntoResponse for JsonExtractor<T>
    where
        axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Deserialize)]
pub struct PostTemplate {
    pub name: String,
    pub subject: String,
    pub body: String,
}

use axum::response::{IntoResponse, Response};
use axum_macros::FromRequest;
use serde::Deserialize;

pub mod post_templates;
pub mod post_users;

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
pub struct RequestTemplate {
    pub name: String,
    pub subject: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct RequestUser {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

use crate::AppError;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub name: String,
    pub age: i32,
}

pub async fn show_users(Json(json): Json<Users>) -> Result<Json<Users>, AppError> {

    Ok(Json(json))
}

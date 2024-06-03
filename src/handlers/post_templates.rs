use crate::db::db_models::NewNotificationTemplate;
use crate::db::schema::notification_templates;
use crate::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum_macros::FromRequest;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl};
use serde::Deserialize;
use crate::errors::AppError;

async fn save_new_template(
    pool: Pool<ConnectionManager<PgConnection>>,
    new_template: NewNotificationTemplate,
) -> Result<(), diesel::result::Error> {
    let conn = &mut pool.get().unwrap();

    diesel::insert_into(notification_templates::table)
        .values(&new_template)
        .execute(conn)?;

    Ok(())
}

pub async fn post_templates(
    State(state): State<AppState>,
    JsonExtractor(new_template): JsonExtractor<CreateTemplate>,
) -> Result<(), AppError> {
    let new_temp = NewNotificationTemplate {
        name: new_template.name,
        subject: new_template.subject,
        body: new_template.body,
        created_at: None,
        updated_at: None,
    };

    save_new_template(state.pool, new_temp).await.unwrap();

    Ok(())
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
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
pub struct CreateTemplate {
    pub name: String,
    pub subject: String,
    pub body: String,
}

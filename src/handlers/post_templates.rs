use crate::db::db_models::{NewNotificationTemplate, NotificationTemplate};
use crate::db::schema::notification_templates;
use crate::errors::AppError;
use crate::handlers::RequestTemplate;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

async fn save_new_template(
    pool: Pool<ConnectionManager<PgConnection>>,
    new_template: NewNotificationTemplate,
) -> Result<NotificationTemplate, diesel::result::Error> {
    let conn = &mut pool.get().unwrap();

    diesel::insert_into(notification_templates::table)
        .values(&new_template)
        .returning(NotificationTemplate::as_returning())
        .get_result(conn)
}

pub async fn post_templates(
    State(state): State<AppState>,
    Json(new_template): Json<RequestTemplate>,
) -> Result<Json<NotificationTemplate>, AppError> {
    let new_temp = NewNotificationTemplate {
        name: new_template.name,
        subject: new_template.subject,
        body: new_template.body,
        created_at: None,
        updated_at: None,
    };

    let created_template = save_new_template(state.pool, new_temp).await?;

    Ok(Json(created_template))
}

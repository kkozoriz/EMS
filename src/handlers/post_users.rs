use crate::db::db_models::{NewUser, User};
use crate::db::schema::users;
use crate::errors::AppError;
use crate::handlers::RequestUser;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

async fn save_new_user(
    pool: Pool<ConnectionManager<PgConnection>>,
    new_user: NewUser,
) -> Result<User, diesel::result::Error> {
    let conn = &mut pool.get().unwrap();

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

pub async fn post_user(
    State(state): State<AppState>,
    Json(user): Json<RequestUser>,
) -> Result<Json<User>, AppError> {
    let new_user = NewUser {
        name: user.name,
        email: user.email,
        phone: user.phone,
        created_at: None,
        updated_at: None,
    };

    let created_user = save_new_user(state.pool, new_user).await?;

    Ok(Json(created_user))
}

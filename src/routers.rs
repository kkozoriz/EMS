use axum::handler::Handler;
use crate::handlers::post_templates::post_templates;
use crate::handlers::post_users::post_user;
use crate::AppState;
use axum::routing::post;
use axum::Router;

pub fn app_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(post_templates))
        .route("/user", post(post_user))
        .with_state(app_state)
}

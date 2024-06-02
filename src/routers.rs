use crate::handlers::post_templates::post_templates;
use crate::AppState;
use axum::routing::post;
use axum::Router;

pub fn app_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(post_templates))
        .with_state(app_state)
}

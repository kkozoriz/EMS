mod db;
mod errors;
mod handlers;
mod routers;

use crate::db::get_connection_pool;
use crate::routers::app_router;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;

extern crate pretty_env_logger as logger;
#[macro_use] extern crate log;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    logger::init();

    let pool = get_connection_pool();
    let state = AppState { pool };
    let app = app_router(state.clone()).with_state(state);

    info!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

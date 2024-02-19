mod hello_world;
mod users;
use hello_world::hello_world;

use axum::{
    extract::FromRef, routing::{get, post}, Router
};
use sea_orm::DatabaseConnection;
use users::{create_user, login};
#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub fn create_router(db_conn: DatabaseConnection) -> Router {
      let state=AppState{database:db_conn};
    // build our application with a route
    Router::new()
        .route("/", get(hello_world))
        .route("/users", post(create_user))
        .route("/user/signin", post(login)).with_state(state)
}

mod hello_world;
use hello_world::hello_world;

use axum::{routing::get, Router};


pub fn create_router()->Router{
      // build our application with a route
      Router::new()
      .route("/", get(hello_world))
 
}


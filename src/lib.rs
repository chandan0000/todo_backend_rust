mod entity;
mod utils;
use routes::create_router;
use sea_orm::Database;

mod routes;
pub async fn run(databse_url: &str) {
    let db = Database::connect(databse_url).await.unwrap();

    let app = create_router(db); 

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// cargo watch -c -w src -x run
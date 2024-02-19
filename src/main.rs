use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use todo_backend::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let databse_url = dotenv!("DATABASE_URL");
    run(databse_url).await;
}

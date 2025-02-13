use sqlx::mysql::MySqlPool;
use std::env;
use dotenv::dotenv;

pub async fn create_db_pool() -> MySqlPool  {
    dotenv().ok();
    // load .env file

    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set in .env");

    MySqlPool::connect(&database_url).await.expect("Failed to connect to TiDB")
}
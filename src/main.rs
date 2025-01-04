use leetcode_practice::fixtures::populate_test_data;
use sqlx::PgPool;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    populate_test_data(&pool).await.unwrap();

    let users = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{:?}", users);
    Ok(())
}

#![allow(dead_code)]

use leetcode_practice::fixtures::populate_test_data;
use sqlx::PgPool;
use std::error::Error;

#[derive(Debug)]
struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    populate_test_data(&pool).await?;

    let users: Vec<User> = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", users);

    let _user = sqlx::query!("SELECT * FROM users WHERE id = $1", 1)
        .fetch_one(&pool)
        .await?;

    Ok(())
}

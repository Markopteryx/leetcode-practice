#![allow(dead_code)]

use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::faker::name::en::Name;
use fake::Fake;
use sqlx::PgPool;
use std::error::Error;

pub async fn populate_test_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    for _ in 0..10 {
        let name: String = Name().fake();
        let email: String = SafeEmail().fake();

        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (name, email)
            VALUES ($1, $2)
            RETURNING id
            "#,
            name,
            email
        )
        .fetch_one(pool)
        .await?
        .id;

        for _ in 0..3 {
            let title: String = Sentence(3..5).fake();
            let content: String = Paragraph(3..5).fake();

            sqlx::query!(
                r#"
                INSERT INTO posts (user_id, title, content)
                VALUES ($1, $2, $3)
                "#,
                user_id,
                title,
                content
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

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

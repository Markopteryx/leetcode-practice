use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::faker::name::en::Name;
use fake::Fake;
use sqlx::PgPool;

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

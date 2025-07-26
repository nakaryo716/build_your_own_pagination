use std::env;

use sqlx::PgPool;

use crate::entity::article::Article;

pub mod entity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get database url from environment variable
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://user:password@localhost:15432/app_db".to_string());

    // establish connection pool
    let db_pool = PgPool::connect(&database_url).await?;

    // test database connection
    let s: Vec<Article> = sqlx::query_as(
        r#"
        SELECT id, title, body, created_at FROM articles
        "#,
    )
    .fetch_all(&db_pool)
    .await?;

    s.iter()
        .flat_map(|v| serde_json::to_string(&v))
        .for_each(|v| println!("{v}"));

    Ok(())
}

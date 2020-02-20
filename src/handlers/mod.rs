use std::convert::Infallible;
use std::sync::Arc;

use sqlx::PgPool;
use warp::reply::html;
use warp::Reply;

use crate::models::User;

pub(crate) async fn db_get_user_handler(
    name: String,
    db: Arc<PgPool>,
) -> Result<impl Reply, Infallible> {
    let results = sqlx::query_as!(
        User,
        r#"
SELECT * FROM users
WHERE name = $1
"#,
        &name
    )
    .fetch_all(&*db)
    .await
    .unwrap();
    if !results.is_empty() {
        Ok(html(format!("Hello {}! You're in our DB!", name)))
    } else {
        sqlx::query!(
            r#"
INSERT INTO users
(name, age)
VALUES ($1,$2)
"#,
            name,
            20
        )
            .execute(&*db)
            .await
            .unwrap();
        Ok(html(format!("Hello {} we just added you to our DB!", name)))
    }
}

pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: sqlx::types::time::PrimitiveDateTime,
}

impl User {
    pub async fn create_user_with_email_and_password(
        pool: sqlx::postgres::PgPool,
        email: &str,
        password: String,
    ) -> Result<i32, sqlx::Error> {
        let password_hash = password_auth::generate_hash(password);
        let user = sqlx::query!(
            r#"INSERT INTO users (email, password)
            VALUES ($1, $2)
            RETURNING id
            "#,
            email,
            password_hash
        )
        .fetch_one(&pool)
        .await?;
        Ok(user.id)
    }

    pub async fn get_user_by_email(
        pool: sqlx::postgres::PgPool,
        email: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT * 
            FROM users
            WHERE email=$1
            "#,
            email
        )
        .fetch_one(&pool)
        .await?;
        Ok(user)
    }
}

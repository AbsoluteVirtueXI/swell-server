use sqlx::PgPool;
use std::env;
use crate::models::*;

pub type Db = PgPool;

// TODO: ERROR
pub async fn get_db() -> Db {
    dotenv::dotenv().ok();
    PgPool::builder()
        .max_size(20)
        .build(&env::var("DATABASE_URL").unwrap()).await.unwrap()
}

pub async fn db_user_exists(eth_addr: String, db: &Db) -> User  {
    sqlx::query_as!(User, "SELECT * FROM users WHERE eth_addr = $1", eth_addr)
        .fetch_one(db).await.unwrap()
}

pub async fn db_create_user(eth_addr: String, bio: String, db: &Db) -> u64 {
    sqlx::query!("INSERT INTO users (eth_addr, bio) VALUES ($1, $2)", eth_addr, bio)
        .execute(db).await.unwrap()

}
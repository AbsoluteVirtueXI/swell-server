use sqlx::PgPool;
use std::env;
use crate::models::*;

pub type Db = PgPool;

pub async fn get_db() -> Db {
    dotenv::dotenv().ok();
    PgPool::builder()
        .max_size(20)
        .build(&env::var("DATABASE_URL").unwrap()).await.unwrap()
}

pub async fn db_get_user_id_by_eth_addr(ethd_addr: String, db: &Db) -> Option<User> {

}

pub async fn db_get_user_by_eth_addr(eth_addr: String, db: &Db) -> Option<User>  {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE eth_addr = $1", eth_addr)
        .fetch_one(db).await
    {
        Ok(user) => Some(user),
        Err(e) => None
    }
}

pub async fn db_get_user_by_id(id: i32, db: &Db) -> User {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(db).await.unwrap()
}

pub async fn db_get_user_by_login(login: String, db: &Db) -> User {
    sqlx::query_as!(User, "SELECT * FROM users WHERE login = $1", login)
        .fetch_one(db).await.unwrap()
}

pub async fn db_get_user_by_register_request(register_request: RegisterRequest, db: &Db) -> User {
    sqlx::query_as!(User, "SELECT * FROM users where login = $1 AND eth_addr = $2",
     register_request.login, register_request.eth_addr)
        .fetch_one(db).await.unwrap()
}

pub async fn db_create_user(eth_addr: String, login: String, db: &Db) -> u64 {
    sqlx::query!("INSERT INTO users (eth_addr, login) VALUES ($1, $2)", eth_addr, login)
        .execute(db).await.unwrap()

}

use sqlx::PgPool;
use std::env;
use crate::models::*;
use anyhow::Result;

pub type Db = PgPool;

pub async fn get_db() -> Db {
    dotenv::dotenv().ok();
    PgPool::builder()
        .max_size(20)
        .build(&env::var("DATABASE_URL").unwrap()).await.unwrap()
}



pub async fn db_get_user_by_eth(eth_addr: String, db: &Db) -> Option<User>  {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE eth_addr = $1", eth_addr)
        .fetch_one(db).await {
        Ok(user) => Some(user),
        Err(e) => None,
    }

}

pub async fn db_get_all_videos(db: &Db) -> Vec<Video> {
    sqlx::query_as!(Video, "SELECT * FROM videos").fetch_all(db).await.unwrap()
}

pub async fn db_get_all_items(db: &Db) -> Vec<Item> {
    sqlx::query_as!(Item, "SELECT * FROM items").fetch_all(db).await.unwrap()
}

pub async fn db_get_user_by_id(id: i32, db: &Db) -> Option<User> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(db).await {
        Ok(user) => Some(user),
        Err(e) => None,
    }
}

pub async fn db_get_user_by_login(login: String, db: &Db) -> Option<User> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE login = $1", login)
        .fetch_one(db).await {
        Ok(user) => Some(user),
        Err(e) => None,
    }
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

pub async fn db_get_id(eth_addr: String, db: &Db) -> i32 {
    let res = sqlx::query_as!(User, "SELECT * FROM USERS where eth_addr = $1", eth_addr).fetch_one(db).await.unwrap();
    res.id
}

pub async fn db_add_video(owner_id: i32, title: String, bio: String, price: i32, path: String, db:&Db) -> u64 {
    sqlx::query!("INSERT INTO videos (owner_id, title, bio, price, path) VALUES ($1, $2, $3, $4, $5)",
    owner_id, title, bio, price, path).execute(db).await.unwrap();
    let res = sqlx::query_as!(Video, "SELECT * FROM videos where path = $1", path).fetch_one(db).await.unwrap();
    sqlx::query!("UPDATE users SET videos = array_append(videos, $1) WHERE id = $2", res.id, owner_id).execute(db).await.unwrap()
}

pub async fn db_add_image(owner_id: i32, title: String, bio: String, price: i32, path: String, db: &Db) -> u64 {
    sqlx::query!("INSERT INTO items (owner_id, title, bio, price, path) VALUES ($1, $2, $3, $4, $5)",
    owner_id, title, bio, price, path).execute(db).await.unwrap();
    let res = sqlx::query_as!(Item, "SELECT * FROM items where path = $1", path).fetch_one(db).await.unwrap();
    sqlx::query!("UPDATE users SET items = array_append(items, $1) where id = $2", res.id, owner_id).execute(db).await.unwrap()
}
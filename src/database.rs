/*
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

 */



use sqlx::postgres::PgPool;
use crate::models::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;


#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Database, Error> {
        let pool = PgPool::builder()
            .max_size(20) // maximum number of connections in the pool
            .build(database_url)
            .await?;
        let db = Database { pool };
        Ok(db)
    }
}

impl Database {
    fn _handle_optional_result<T>(sql_res: Result<T, sqlx::Error>) -> Result<Option<T>, sqlx::Error> {
        match sql_res {
            Ok(res) => Ok(Some(res)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e)
        }
    }
}

// Users
impl Database {
    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        let sql_res = sqlx::query_as!(
            User,
            r#"SELECT * FROM users where id = $1"#,
            id
        )
            .fetch_one(&self.pool)
            .await;
        Database::_handle_optional_result(sql_res)
    }

    pub async fn get_user_by_username(&self, username: String) -> Result<Option<User>, sqlx::Error> {
        let sql_res = sqlx::query_as!(
            User,
            r#"SELECT * FROM users where username = $1"#,
            username
        )
            .fetch_one(&self.pool)
            .await;
        Database::_handle_optional_result(sql_res)
    }

    pub async fn get_users_by_pattern(&self, id: i64, mut pattern: String) -> Result<Vec<User>, sqlx::Error> {
        pattern.push_str("%");
        let sql_res = sqlx::query_as!(
            User,
            r#"SELECT * from users WHERE username ILIKE $1 AND id != $2 ORDER BY username"#,
            pattern,
            id,
        ).fetch_all(&self.pool).await?;
        Ok(sql_res)
    }

    pub async fn add_user(&self, user: RegisterInput) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (username, eth_address) VALUES($1, $2) RETURNING *
            "#,
            user.username,
            user.eth_address
        )
            .fetch_one(&self.pool)
            .await
        //Database::_handle_optional_result(sql_res)
    }

    pub async fn get_followers(&self, id: i64) -> Result<Vec<User>, sqlx::Error> {
        let sql_res = sqlx::query_as!(
            User,
            r#"
                WITH followers as (select * from follows where followee_id = $1) select * from users where id IN (select follower_id from followers)
            "#,
            id
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(sql_res)
    }

    pub async fn get_followees(&self, id: i64) -> Result<Vec<User>, sqlx::Error> {
        let sql_res = sqlx::query_as!(
            User,
            r#"
                WITH followees as (select * from follows where follower_id = $1) select * from users where id IN (select followee_id from followees)
            "#,
            id
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(sql_res)
    }

    pub async fn follow(&self, followee_id: i64, follower_id: i64) -> Result<bool, sqlx::Error> {
        let sql_res = sqlx::query!(
            r#"
                INSERT INTO follows (followee_id, follower_id) VALUES($1, $2)
            "#,
            followee_id,
            follower_id
        ).execute(&self.pool).await?;
        Ok(true)
    }

    pub async fn unfollow(&self, followee_id: i64, follower_id: i64) -> Result<bool, sqlx::Error> {
        let sql_res = sqlx::query!(
            r#"
                DELETE from follows where followee_id = $1 and follower_id = $2
            "#,
            followee_id,
            follower_id
        ).execute(&self.pool).await?;
        Ok(true)
    }

    pub async fn db_add_product(&self,
                                seller_id: i64,
                                description: String,
                                price: i64,
                                product_type: String,
                                media_type: String,
                                path: String,
                                thumbnail_path: String) -> Result<bool, sqlx::Error> {
        let sql_res = sqlx::query_as!(Media, r#"
        INSERT INTO medias (path, thumbnail_path, media_type) VALUES ($1, $2, $3) RETURNING *
        "#, path, thumbnail_path, media_type)
            .fetch_one(&self.pool)
            .await.unwrap();
        let sql_res2 = sqlx::query_as!(Product, r#"
        INSERT INTO products(product_type, seller_id, description, price, media_id) VALUES ($1, $2, $3, $4, $5) RETURNING *
        "#, product_type, seller_id, description, price, sql_res.id).fetch_one(&self.pool).await.unwrap();
        Ok(true)
    }
    /*
    pub async fn db_get_all_videos(db: &Db) -> Vec<Video> {
        sqlx::query_as!(Video, "SELECT * FROM videos").fetch_all(db).await.unwrap()
    }*/

    pub async fn db_get_products_feed(&self, id: i64) -> Result<Vec<Feed>, sqlx::Error> {
        let sql_res = sqlx::query_as!(Feed,
        r#"
            SELECT products.id, products.seller_id, users.username, users.avatar, products.product_type, products.description,
            products.price, products.views, products.likes, medias.path, medias.thumbnail_path, medias.media_type, medias.created_at
            FROM products INNER JOIN users ON products.seller_id = users.id INNER JOIN medias ON products.media_id = medias.id
            WHERE products.buyers_id = 0 ORDER BY products.created_at DESC
        "#).fetch_all(&self.pool).await?;
        Ok(sql_res)
    }

    pub async fn db_get_my_products_feed(&self, id: i64) -> Result<Vec<Feed>, sqlx::Error> {
        let sql_res = sqlx::query_as!(Feed,
        r#"
            SELECT products.id, products.seller_id, users.username, users.avatar, products.product_type, products.description,
            products.price, products.views, products.likes, medias.path, medias.thumbnail_path, medias.media_type, medias.created_at
            FROM products INNER JOIN users ON products.seller_id = users.id INNER JOIN medias ON products.media_id = medias.id
            WHERE users.id = $1 ORDER BY products.created_at DESC
        "#, id).fetch_all(&self.pool).await?;
        Ok(sql_res)
    }

    pub async fn db_add_message(&self, user_id: i64, input: SendMessageInput) -> Result<bool, sqlx::Error> {
        let sql_res = sqlx::query!(
            r#"INSERT INTO messages (sender, receiver, content) VALUES($1, $2, $3)"#,
            user_id, input.receiver, input.content
        ).execute(&self.pool).await?;
        Ok(true)
    }


    pub async fn db_get_all_threads(&self, user_id: i64) -> Result<Vec<Thread>, sqlx::Error> {
        let sql_res = sqlx::query_as!(Thread,
            r#"
                SELECT users.id, users.username, users.avatar, m.content, m.created_at
                FROM messages m
                INNER JOIN users ON users.id = CASE WHEN m.sender = $1 THEN m.receiver ELSE m.sender END
                WHERE $1 IN (m.sender, m.receiver) AND
                (LEAST(m.sender, m.receiver), GREATEST(m.sender, m.receiver), m.created_at) IN
                (SELECT LEAST(m2.sender, m2.receiver), GREATEST(m2.sender, m2.receiver), MAX(m2.created_at)
                FROM messages m2
                GROUP BY LEAST(m2.sender, m2.receiver), GREATEST(m2.sender, m2.receiver)
                ) ORDER BY m.created_at DESC
            "#, user_id
        ).fetch_all(&self.pool).await?;
        Ok(sql_res)
    }

    pub async fn db_get_all_messages(&self, input: AllMessagesInput) -> Result<Vec<Message>, sqlx::Error> {
        let sql_res = sqlx::query_as!(Message,
        r#"
            SELECT * FROM messages WHERE sender = $1 AND receiver = $2 OR sender = $2 AND receiver = $1 ORDER BY created_at ASC
        "#, input.user1, input.user2
        ).fetch_all(&self.pool).await?;
        Ok(sql_res)
    }
}


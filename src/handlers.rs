use crate::models::*;
use crate::db::*;
use std::convert::Infallible;
use warp::http::StatusCode;
use crate::json_extractor;

pub async fn handle_is_registered(eth_addr: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    match db_get_user_by_eth(eth_addr, &db).await {
        Some(value) => Ok(warp::reply::json(&value)),
        None => Ok(warp::reply::json(&ErrorMessage {
            code: 404u16,
            message: String::from("User not found"),
        }))
    }
}

pub async fn handle_register(request: RegisterRequest, db: Db) -> Result<impl warp::Reply, Infallible> {
    let row = db_create_user(request.eth_addr, request.login, &db).await;
    if row != 0 {
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }

}

pub async fn handle_get_user_by_id(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
    match db_get_user_by_id(id, &db).await {
        Some(value) => Ok(warp::reply::json(&value)),
        None => Ok(warp::reply::json(&ErrorMessage {
            code: 404u16,
            message: String::from("User not found"),
        }))
    }
}

pub async fn handle_get_user_by_eth(eth: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    match db_get_user_by_eth(eth, &db).await {
        Some(value) => Ok(warp::reply::json(&value)),
        None => Ok(warp::reply::json(&ErrorMessage {
            code: 404u16,
            message: String::from("User not found"),
        }))
    }
}

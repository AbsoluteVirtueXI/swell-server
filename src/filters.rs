use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::db::*;
use crate::models::*;
use crate::handlers::*;

pub fn rest_swell(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    rest_is_registered(db.clone()).or(rest_register(db.clone()))
}


pub fn rest_is_registered(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("is_registered")
        .and(warp::get())
        .and(warp::query::<IsRegisteredRequest>())
        .and(with_db(db))
        .and_then(handle_is_registered)
}

pub fn rest_register(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(json_body<>())
        .and(with_db(db))
        .and_then(handle_register)
}

fn json_body<T>() -> impl Filter<Extract= (T,), Error = warp::Rejection> + Clone {
    warp::body::json()
}


/// Make the db accessible within filter
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
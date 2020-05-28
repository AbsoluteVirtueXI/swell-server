use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::db::*;
use crate::models::*;
use crate::handlers::*;
use crate::json_extractor::*;



pub fn rest_swell(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    rest_is_registered(db.clone())
        .or(rest_register(db.clone()))
        .or(warp::post().and(warp::path("files")).and(warp::fs::dir("files/")))
}


pub fn rest_is_registered(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("is_registered" / u64)
        .map(|eth_addr: u64| eth_addr.to_string())
        .and(warp::get())
        //.and(warp::path::param())
        .and(with_db(db))
        .and_then(handle_is_registered)
}


pub fn rest_register(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(json_body_register())
        .and(with_db(db))
        .and_then(handle_register)
}


/// Make the db accessible within filter
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
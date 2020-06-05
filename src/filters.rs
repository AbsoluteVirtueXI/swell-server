use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::db::*;
use crate::models::*;
use crate::handlers::*;
use crate::json_extractor::*;



pub fn rest_swell(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    rest_is_registered(db.clone())
        .or(rest_register(db.clone()))
        .or(rest_get_user_by_id(db.clone()))
        .or(warp::path("files")
            .and(warp::get())
            .and(warp::fs::dir("files/")))
        .or(rest_get_id(db.clone()))
        .or(rest_upload_video(db.clone()))
}

pub fn rest_upload_video(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("upload_video")
        .and(warp::multipart::form())
        .and_then(deserialize_form_data)
        .and(with_db(db))
        .and_then(save_video_file)
}

pub fn rest_get_id(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_id" / String)
        .and(warp::get())
        //.and(warp::path::param())
        .and(with_db(db))
        .and_then(handle_get_id)
}

pub fn rest_get_user_by_id(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_user_by_id" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handle_get_user_by_id)
}

pub fn rest_get_user_by_eth(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_user_by_eth" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handle_get_user_by_eth)
}

pub fn rest_is_registered(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("is_registered" / String)
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
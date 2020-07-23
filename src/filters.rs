use warp::Filter;
use crate::json_extractor::*;
use crate::database::*;
use crate::handlers::*;
use warp::{
    filters::multipart::{FormData, Part},
    reject, Buf, Rejection
};


use serde::{Deserialize, Serialize};

/*
use crate::database::*;
use crate::models::*;
use crate::handlers::*;

*/

/// Make the db accessible within filter
fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn rest_swell(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    rest_register(db.clone())
        .or(rest_get_user_by_id(db.clone()))
        .or(rest_get_my_profile(db.clone()))
        .or(rest_get_user_by_username(db.clone()))
        .or(rest_upload_product(db.clone()))
        .or(rest_get_products_feed(db.clone()))
        .or(rest_get_my_products_feed(db.clone()))
        .or(rest_get_all_messages(db.clone()))
        .or(rest_get_my_threads(db.clone()))
        .or(rest_send_message(db.clone()))
        .or(warp::path("files")
            .and(warp::get())
            .and(warp::fs::dir("files/")))
}

pub fn rest_register(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(json_body_register())
        .and(with_db(db))
        .and_then(handle_register)
}


pub fn rest_get_user_by_id(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_user_by_id" / i64)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handle_get_user_by_id)
}

pub fn rest_get_my_profile(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_my_profile")
        .and(warp::header::<String>("Authorization"))
        .and(with_db(db))
        .and_then(handle_get_my_profile)
}

pub fn rest_get_user_by_username(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_user_by_username" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handle_get_user_by_username)
}

pub fn rest_upload_product(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    println!("IN rest_upload_product");
    warp::path!("upload_product")
        .and(warp::post())
        .and(warp::header::<String>("Authorization"))
        .and(warp::multipart::form())
        .and_then(deserialize_form_data)
        .and(with_db(db))
        .and_then(save_media_file)
}

pub fn rest_get_products_feed(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_products_feed")
        .and(warp::get())
        .and(warp::header::<String>("Authorization"))
        .and(with_db(db))
        .and_then(handle_get_products_feed)
}

pub fn rest_get_my_products_feed(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_my_products_feed")
        .and(warp::get())
        .and(warp::header::<String>("Authorization"))
        .and(with_db(db))
        .and_then(handle_get_my_products_feed)
}

pub fn rest_get_all_messages(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_all_messages")
        .and(warp::post())
        .and(warp::header::<String>("Authorization"))
        .and(json_body_all_messages())
        .and(with_db(db))
        .and_then(handle_get_all_messages)
}

pub fn rest_send_message(db: Database) -> impl
Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("send_message")
        .and(warp::post())
        .and(warp::header::<String>("Authorization"))
        .and(json_body_send_messages())
        .and(with_db(db))
        .and_then(handle_send_message)
}

pub fn rest_get_my_threads(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_my_threads")
        .and(warp::get())
        .and(warp::header::<String>("Authorization"))
        .and(with_db(db))
        .and_then(handle_get_my_threads)
}

/*
pub fn rest_upload_item(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("upload_item")
        .and(warp::multipart::form())
        .and_then(deserialize_form_data)
        .and(with_db(db))
        .and_then(save_image_file)
}


pub fn rest_swell(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    rest_is_registered(db.clone())
        .or(rest_register(db.clone()))
        .or(rest_get_user_by_id(db.clone()))
        .or(warp::path("files")
            .and(warp::get())
            .and(warp::fs::dir("files/")))
        .or(rest_get_id(db.clone()))
        .or(rest_upload_video(db.clone()))
        .or(rest_upload_item(db.clone()))
        .or(rest_get_all_videos(db.clone()))
        .or(rest_get_all_items(db.clone()))
}

pub fn rest_get_all_items(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_all_items")
        .and(with_db(db))
        .and_then(handle_get_all_items)
}

pub fn rest_get_all_videos(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get_all_videos")
        .and(with_db(db))
        .and_then(handle_get_all_videos)
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



 */
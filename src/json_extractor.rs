use crate::models::*;
use warp::Filter;

pub fn json_body_register() -> impl Filter<Extract= (RegisterInput,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

pub fn json_body_all_messages() -> impl Filter<Extract= (AllMessagesInput,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

pub fn json_body_send_messages() -> impl Filter<Extract= (SendMessageInput,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

pub fn json_body_pattern() -> impl Filter<Extract= (Pattern,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

pub fn json_body_buy_products() -> impl Filter<Extract= (BuyProducts,), Error = warp::Rejection> + Clone {
    warp::body::json()
}
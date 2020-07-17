use crate::models::*;
use warp::Filter;

pub fn json_body_register() -> impl Filter<Extract= (RegisterInput,), Error = warp::Rejection> + Clone {
    warp::body::json()
}
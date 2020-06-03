use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub eth_addr: String,
    pub bio: String,
    pub czar: i32,
    pub videos: Vec<i32>,
    pub videos_bought: Vec<i32>,
    pub liked: Vec<i32>,
}

pub struct Video {
    pub id: i32,
    pub owner_id: i32,
    pub path: String,
    pub views: i32,
    pub liked: i32,
    pub price: i32,
}

// Request
#[derive(Deserialize)]
pub struct IsRegisteredRequest{
    eth_addr: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub login: String,
    pub eth_addr: String,
}

#[derive(Serialize)]
pub struct Response {
    pub code: u16,
    pub message: String
}

#[derive(Serialize)]
pub struct Eth2Id {
    pub id: u64
}
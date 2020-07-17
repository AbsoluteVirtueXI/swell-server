use serde::{Deserialize, Serialize, Deserializer};
use warp::{
    filters::multipart::{FormData, Part},
    reject, Buf, Rejection,
};

use chrono::{DateTime, Utc, TimeZone};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub eth_address: String,
    pub bio : String,
    pub quadreum: i64,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub eth_address: String,
}

#[derive(Serialize)]
pub struct Response {
    pub code: u16,
    pub data: String
}

/*
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Video {
    pub id: i32,
    pub owner_id: i32,
    pub path: String,
    pub bio: String,
    pub title: String,
    pub views: i32,
    pub liked: i32,
    pub price: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub id: i32,
    pub owner_id: i32,
    pub path: String,
    pub bio: String,
    pub title: String,
    pub views: i32,
    pub liked: i32,
    pub price: i32,
}


// Request
#[derive(Deserialize)]
pub struct IsRegisteredRequest{
    eth_addr: String,
}


#[derive(Serialize)]
pub struct Eth2Id {
    pub id: u64
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UploadVideoFormData {
    pub owner_id: i32,
    pub title: String,
    pub bio: String,
    pub price: i32,
    pub content: Vec<u8>,
}


#[derive(Debug)]
pub struct FileError{
    message: MessageError,
}

impl reject::Reject for FileError {}


#[derive(Debug)]
pub enum MessageError {
    NoFormData,
    ErrorFileOperation,
}

pub enum PartType {
    OwnerId(i32),
    Title(String),
    Bio(String),
    Price(i32),
    FilePart(Part),
    NoFormData,
}

pub struct ResultData {
    pub owner_id: i32,
    pub title: String,
    pub bio: String,
    pub price: i32,
    pub file_part: Option<Part>,
}

impl ResultData {
    pub fn new() -> Self {
        ResultData {
            owner_id: 0,
            title: "".to_string(),
            bio: "".to_string(),
            price: 0,
            file_part: None,
        }
    }
}*/
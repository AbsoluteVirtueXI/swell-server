use sqlx::Type;

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


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Product {
    pub id: i64,
    pub product_type: String,
    pub seller_id: i64,
    pub buyers_id: i64,
    pub description: String,
    pub price: i64,
    pub media_id: i64,
    pub views: i64,
    pub likes: i64,
    pub created_at: DateTime<Utc>,

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Media {
    pub id: i64,
    pub path: String,
    pub thumbnail_path: String,
    pub media_type: String,
    pub created_at: DateTime<Utc>,

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Feed {
    //id of the product
    pub id: i64,
    //id of the user
    pub seller_id: i64,
    pub username: String,
    pub avatar: String,
    pub product_type: String,
    pub description: String,
    pub price: i64,
    pub views: i64,
    pub likes: i64,
    pub path: String,
    pub thumbnail_path: String,
    pub media_type: String,
    pub created_at: DateTime<Utc>,
}

/*
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Thread {
    pub id: i64,
    pub src_id: i64,
    pub src_username: String,
    pub src_avatar: String,
}

 */


/*
CREATE TABLE medias(
id BIGSERIAL PRIMARY KEY NOT NULL,
path TEXT NOT NULL,
media_type MEDIA_TYPE NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
*/

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
*/

#[derive(Deserialize, Serialize, Clone)]
pub struct UploadProductFormData {
    pub seller_id: i32,
    pub description: String,
    pub product_type: String,
    pub price: i64,
    pub content: Vec<u8>,
    pub media_type: String
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
    SellerId(i64),
    Description(String),
    ProductType(String),
    Price(i64),
    FilePart(Part),
    MediaType(String),
    NoFormData,
}


pub struct ResultData {
    pub seller_id: i64,
    pub description: String,
    pub product_type: String,
    pub price: i64,
    pub file_part: Option<Part>,
    pub media_type: String
}

impl ResultData {
    pub fn new() -> Self {
        ResultData {
            seller_id: 0,
            description: "".to_string(),
            product_type: "".to_string(),
            price: 0,
            file_part: None,
            media_type: "".to_string(),
        }
    }
}
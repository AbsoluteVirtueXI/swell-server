use serde::{Deserialize, Serialize};

//#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub eth_addr: String,
    pub bio: String,
}

// Request
#[derive(Deserialize)]
pub struct IsRegisteredRequest{
    eth_addr: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub
}
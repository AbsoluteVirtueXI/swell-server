use crate::models::*;
use crate::database::*;

use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{
    filters::multipart::{FormData, Part},
    reject, Buf, Rejection,
};

use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde_json;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::prelude::*;

/*
use warp::http::StatusCode;
use crate::json_extractor;
use warp::{
    filters::multipart::{FormData, Part},
    reject, Buf, Rejection,
};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde_json;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::prelude::*;
*/


pub async fn handle_register(input: RegisterInput, db: Database) -> Result<impl warp::Reply, Infallible> {
    let sql_res = db.add_user(input).await;
    let code;
    let data;
    match sql_res {
        Ok(user) => {
            code = 200;
            data = serde_json::to_string(&user).unwrap();
        }
        Err(e) => {
            code = 403;
            data = format!("{}", e);
        }
    }
    Ok(warp::reply::json(&Response { code, data }))
}

pub async fn handle_get_user_by_id(id: i64, db: Database) -> Result<impl warp::Reply, Infallible> {
    let sql_res = db.get_user_by_id(id).await;
    let code;
    let data;
    match sql_res {
        Ok(user_opt) => {
            match user_opt {
                Some(user) => {
                    code = 200;
                    data = serde_json::to_string(&user).unwrap();
                }
                None => {
                    code = 404;
                    data = String::from("User not found");
                }
            }
        }
        Err(e) => {
            code = 403;
            data = format!("{}", e);
        }
    }
    Ok(warp::reply::json(&Response { code, data }))
}

pub async fn handle_get_my_profile(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.get_user_by_id(id).await;
            match sql_res {
                Ok(user_opt) => {
                    match user_opt {
                        Some(user) => {
                            code = 200;
                            data = serde_json::to_string(&user).unwrap();
                        }
                        None => {
                            code = 404;
                            data = String::from("User not found");
                        }
                    }
                }
                Err(e) => {
                    code = 403;
                    data = format!("{}", e);
                }
            }
        }
    }
    Ok(warp::reply::json(&Response { code, data }))
}

pub async fn handle_get_user_by_username(username: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let sql_res = db.get_user_by_username(username).await;
    let code;
    let data;
    match sql_res {
        Ok(user_opt) => {
            match user_opt {
                Some(user) => {
                    code = 200;
                    data = serde_json::to_string(&user).unwrap();
                }
                None => {
                    code = 404;
                    data = String::from("User not found");
                }
            }
        }
        Err(e) => {
            code = 403;
            data = format!("{}", e);
        }
    }
    Ok(warp::reply::json(&Response { code, data }))
}



pub async fn deserialize_form_data(id: String, form_data: FormData) -> Result<ResultData, Rejection> {
    let mut result_data = ResultData::new();
    let parts: Vec<PartType> = form_data
        .then(|part| async {
            let mut part = part.unwrap();
            match part.name() {
                "content" => PartType::FilePart(part),
                "description" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    PartType::Description(value)
                },
                "seller_id" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = value.parse::<i64>().unwrap();
                    PartType::SellerId(value)
                },
                "price" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = value.parse::<i64>().unwrap();
                    PartType::Price(value)
                },
                "media_type" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    PartType::MediaType(value)
                },
                "product_type" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    PartType::ProductType(value)
                }
                _ => PartType::NoFormData,
            }
        })
        .collect::<Vec<PartType>>()
        .await;

    for part in parts {
        match part {
            PartType::FilePart(file_part) => {
                result_data.file_part = Some(file_part);
            }

            PartType::Description(description) => {
                result_data.description = description;
            }
            PartType::Price(price) => {
                result_data.price = price;
            }
            PartType::SellerId(id) => {
                result_data.seller_id = id;
            }
            PartType::ProductType(product_type) => {
                result_data.product_type = product_type;
            }
            PartType::MediaType(media_type) => {
                result_data.media_type = media_type;
            }
            PartType::NoFormData => (),
        };
    }

    Ok(result_data)
}

pub async fn save_media_file(product: ResultData, db: Database) -> Result<impl warp::Reply, Infallible> {
    print("IN SAVE MEDIA");
    let uuid = Uuid::new_v4().to_string();
    let extension = if product.media_type == "VIDEO" {
        String::from("mp4")
    } else {
        String::from("jpg")
    };
    let file_path = format!("files/{}.{}", uuid, { extension });
    let data_buf = product.file_part.unwrap().data().await.unwrap().unwrap();
    let data_bytes = data_buf.bytes();
    let mut file = File::create(file_path.clone()).await.unwrap();
    file.write_all(data_bytes).await.unwrap();
    let res = db.db_add_product(product.seller_id,
                             product.description, product.price,
                             product.product_type,
                             product.media_type,
                             file_path).await;
    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/*
pub async fn handle_get_id(eth_addr: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let id = db_get_id(eth_addr, &db).await;
    Ok(warp::reply::json(&Response{
        code: 200u16,
        message: format!("{}", id)
    }))
}

pub async fn handle_is_registered(eth_addr: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    match db_get_user_by_eth(eth_addr, &db).await {
        Some(_) => Ok(StatusCode::OK),
        None => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn handle_register(request: RegisterRequest, db: Db) -> Result<impl warp::Reply, Infallible> {
    let row = db_create_user(request.eth_addr, request.login, &db).await;
    if row != 0 {
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }

}

pub async fn handle_get_user_by_id(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
    match db_get_user_by_id(id, &db).await {
        Some(value) => Ok(warp::reply::json(&value)),
        None => Ok(warp::reply::json(&Response {
            code: 404u16,
            message: String::from("User not found"),
        }))
    }
}

pub async fn handle_get_all_videos(db: Db) -> Result<impl warp::Reply, Infallible> {
    let lst_video = db_get_all_videos(&db).await;
    Ok(warp::reply::json(&lst_video))
}

pub async fn handle_get_all_items(db: Db) -> Result<impl warp::Reply, Infallible> {
    let lst_item = db_get_all_items(&db).await;
    Ok(warp::reply::json(&lst_item))
}

pub async fn handle_get_user_by_eth(eth: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    match db_get_user_by_eth(eth, &db).await {
        Some(value) => Ok(warp::reply::json(&value)),
        None => Ok(warp::reply::json(&Response {
            code: 404u16,
            message: String::from("User not found"),
        }))
    }
}


pub async fn deserialize_form_data(form_data: FormData) -> Result<ResultData, Rejection> {
    let mut result_data = ResultData::new();
    let parts: Vec<PartType> = form_data
        .then(|part| async {
            let mut part = part.unwrap();
            match part.name() {
                "content" => PartType::FilePart(part),
                "title" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    PartType::Title(value)
                },
                "bio" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    PartType::Bio(value)
                },
                "owner_id" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = value.parse::<i32>().unwrap();
                    PartType::OwnerId(value)
                },
                "price" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = value.parse::<i32>().unwrap();
                    PartType::Price(value)
                }
                _ => PartType::NoFormData,
            }
        })
        .collect::<Vec<PartType>>()
        .await;

    for part in parts {
        match part {
            PartType::FilePart(file_part) => {
                result_data.file_part = Some(file_part);
            }
            PartType::Title(title) => {
                result_data.title = title;
            }
            PartType::Bio(bio) => {
                result_data.bio = bio;
            }
            PartType::Price(price) => {
                result_data.price = price;
            }
            PartType::OwnerId(id) => {
                result_data.owner_id = id;
            }
            PartType::NoFormData => (),
        };
    }

    Ok(result_data)
}

pub async fn save_video_file(video: ResultData, db: Db) -> Result<impl warp::Reply, Infallible> {
    let uuid = Uuid::new_v4().to_string();
    let file_path = format!("files/{}.mp4", uuid);
    let data_buf = video.file_part.unwrap().data().await.unwrap().unwrap();
    let data_bytes = data_buf.bytes();
    let mut file = File::create(file_path.clone()).await.unwrap();
    file.write_all(data_bytes).await.unwrap();
    let row = db_add_video(video.owner_id, video.title, video.bio, video.price, file_path, &db).await;
    if row != 0 {
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}

pub async fn save_image_file(image: ResultData, db: Db) -> Result<impl warp::Reply, Infallible> {
    let uuid = Uuid::new_v4().to_string();
    let file_path = format!("files/{}.jpg", uuid);
    let data_buf = image.file_part.unwrap().data().await.unwrap().unwrap();
    let data_bytes = data_buf.bytes();
    let mut file = File::create(file_path.clone()).await.unwrap();
    file.write_all(data_bytes).await.unwrap();
    let row = db_add_image(image.owner_id, image.title, image.bio, image.price, file_path, &db).await;
    if row != 0 {
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}
 */

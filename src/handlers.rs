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
use crate::ffmpeg_utils::*;

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

pub async fn handle_get_followers(user_id: i64, id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;

    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.get_followers(user_id).await;
            match sql_res {
                Ok(users) => {
                    code = 200;
                    data = serde_json::to_string(&users).unwrap();
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

pub async fn handle_get_followees(user_id: i64, id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;

    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.get_followees(user_id).await;
            match sql_res {
                Ok(users) => {
                    code = 200;
                    data = serde_json::to_string(&users).unwrap();
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

pub async fn handle_follow(user_id: i64, id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    match id.parse::<i64>() {
        Err(e) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(id) => {
            let res = db.follow(user_id, id).await;
            match res {
                Ok(_) => Ok(StatusCode::CREATED),
                Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

pub async fn handle_unfollow(user_id: i64, id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    match id.parse::<i64>() {
        Err(e) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(id) => {
            let res = db.unfollow(user_id, id).await;
            match res {
                Ok(_) => Ok(StatusCode::CREATED),
                Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}


pub async fn handle_get_users_by_pattern(id: String, pattern: Pattern, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;

    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.get_users_by_pattern(id, pattern).await;
            match sql_res {
                Ok(users) => {
                    code = 200;
                    data = serde_json::to_string(&users).unwrap();
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
    println!("In deserialize");
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
                }
                "seller_id" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = value.parse::<i64>().unwrap();
                    PartType::SellerId(value)
                }
                "price" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = value.parse::<i64>().unwrap();
                    PartType::Price(value)
                }
                "media_type" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    PartType::MediaType(value)
                }
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
    println!("result data: {} {} {}", result_data.seller_id, result_data.price, result_data.description);
    Ok(result_data)
}

pub async fn save_media_file(product: ResultData, db: Database) -> Result<impl warp::Reply, Infallible> {
    println!("IN SAVE MEDIA");
    let thumbnail_path;
    let uuid = Uuid::new_v4().to_string();
    let extension = if product.media_type == "VIDEO" {
        thumbnail_path = format!("files/{}.png", uuid);
        String::from("mp4")
    } else {
        thumbnail_path = String::from("");
        String::from("jpg")
    };
    let file_path = format!("files/{}.{}", uuid, extension.clone());
    let data_buf = product.file_part.unwrap().data().await.unwrap().unwrap();
    let data_bytes = data_buf.bytes();
    let mut file = File::create(file_path.clone()).await.unwrap();
    file.write_all(data_bytes).await.unwrap();
    let res = db.db_add_product(product.seller_id,
                                product.description, product.price,
                                product.product_type,
                                product.media_type,
                                file_path.clone(), thumbnail_path.clone()).await;
    if !thumbnail_path.is_empty() {
        let status = create_thumbnail(file_path.clone(), thumbnail_path.clone()).await;
    }

    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn deserialize_form_profile(id: String, form_data: FormData) -> Result<ProfileData, Rejection> {
    println!("In deserialize");
    let mut result_data = ProfileData::new();
    let parts: Vec<ProfilePartType> = form_data
        .then(|part| async {
            let mut part = part.unwrap();
            match part.name() {
                "avatar" => ProfilePartType::FilePart(part),
                "bio" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = if value.is_empty() {
                        String::from("Hello, i am new on Squarrin")
                    } else {
                        value
                    };
                    ProfilePartType::Bio(value)
                }
                "id" => {
                    let part_bytes = part.data().await.unwrap().unwrap();
                    let value = std::str::from_utf8(part_bytes.bytes()).unwrap().to_string();
                    let value = value.parse::<i64>().unwrap();
                    ProfilePartType::Id(value)
                }
                _ => ProfilePartType::NoFormData,
            }
        })
        .collect::<Vec<ProfilePartType>>()
        .await;

    for part in parts {
        match part {
            ProfilePartType::FilePart(file_part) => {
                result_data.file_part = Some(file_part);
            }

            ProfilePartType::Bio(bio) => {
                result_data.bio = bio;
            }
            ProfilePartType::Id(id) => {
                result_data.id = id;
            }
            ProfilePartType::NoFormData => (),
        };
    }
    println!("result data: {} {}", result_data.id, result_data.bio);
    Ok(result_data)
}

pub async fn save_profile(profile: ProfileData, db: Database) -> Result<impl warp::Reply, Infallible> {
    println!("IN SAVE Profile");
    println!("{}", profile.bio);
    let uuid = Uuid::new_v4().to_string();
    let file_path = format!("files/{}.jpg", uuid);
    let data_buf = profile.file_part.unwrap().data().await.unwrap().unwrap();
    let data_bytes = data_buf.bytes();
    let mut file = File::create(file_path.clone()).await.unwrap();
    file.write_all(data_bytes).await.unwrap();
    let res = db.db_update_profile(profile.id,
                                profile.bio,
                                file_path.clone()).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn handle_get_products_feed(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.db_get_products_feed(id).await;
            match sql_res {
                Ok(feeds) => {
                    code = 200;
                    data = serde_json::to_string(&feeds).unwrap();
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

pub async fn handle_get_my_products_feed(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.db_get_my_products_feed(id).await;
            match sql_res {
                Ok(feeds) => {
                    code = 200;
                    data = serde_json::to_string(&feeds).unwrap();
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

pub async fn handle_get_products_feed_by_user(user_id: i64, id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.db_get_my_products_feed(user_id).await;
            match sql_res {
                Ok(feeds) => {
                    code = 200;
                    data = serde_json::to_string(&feeds).unwrap();
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

pub async fn handle_get_my_threads(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.db_get_all_threads(id).await;
            match sql_res {
                Ok(threads) => {
                    code = 200;
                    data = serde_json::to_string(&threads).unwrap();
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

pub async fn handle_get_all_messages(id: String, input: AllMessagesInput, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            if id != input.user1 && id != input.user2 {
                code = 401;
                data = String::from("Unauthorized")
            } else {
                let sql_res = db.db_get_all_messages(input).await;
                match sql_res {
                    Ok(messages) => {
                        code = 200;
                        data = serde_json::to_string(&messages).unwrap();
                    }
                    Err(e) => {
                        code = 403;
                        data = format!("{}", e);
                    }
                }
            }
        }
    }
    Ok(warp::reply::json(&Response { code, data }))
}

pub async fn handle_send_message(id: String, input: SendMessageInput, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.db_add_message(id,input).await;
            match sql_res {
                Ok(res) => {
                    code = 200;
                    data = serde_json::to_string(&res).unwrap();
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

pub async fn handle_buy_products(id: String, buy_products: BuyProducts, db: Database) -> Result<impl warp::Reply, Infallible> {
    let code;
    let data;
    match id.parse::<i64>() {
        Err(_) => {
            code = 403;
            data = String::from("Bad token format")
        }
        Ok(id) => {
            let sql_res = db.db_buy_products(id,buy_products).await;
            match sql_res {
                Ok(res) => {
                    code = 200;
                    data = serde_json::to_string(&res).unwrap();
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

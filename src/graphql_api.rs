use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Deferred, EmptyMutation, EmptySubscription, Object, QueryBuilder, Schema};
use async_graphql_warp::{BadRequest, GQLResponseStream};
use http::StatusCode;
use std::convert::Infallible;
use std::time::Duration;
use tokio::time;
use warp::{http::Response, Filter, Rejection, Reply};

use async_graphql::{Context, EmptySubscription, Schema, Upload, ID};
use futures::lock::Mutex;
use slab::Slab;

pub type FilesSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[async_graphql::SimpleObject]
#[derive(Clone)]
pub struct FileInfo {
    id: ID,
    filename: String,
    mimetype: Option<String>,
}

pub type Storage = Mutex<Slab<FileInfo>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn uploads(&self, ctx: &Context<'_>) -> Vec<FileInfo> {
        let storage = ctx.data::<Storage>().lock().await;
        storage.iter().map(|(_, file)| file).cloned().collect()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn single_upload(&self, ctx: &Context<'_>, file: Upload) -> FileInfo {
        let mut storage = ctx.data::<Storage>().lock().await;
        println!("files count: {}", storage.len());
        let entry = storage.vacant_entry();
        let info = FileInfo {
            id: entry.key().into(),
            filename: file.filename().to_string(),
            mimetype: file.content_type().map(ToString::to_string),
        };
        entry.insert(info.clone());
        info
    }

    async fn multiple_upload(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Vec<FileInfo> {
        let mut infos = Vec::new();
        let mut storage = ctx.data::<Storage>().lock().await;
        for file in files {
            let entry = storage.vacant_entry();
            let info = FileInfo {
                id: entry.key().into(),
                filename: file.filename().to_string(),
                mimetype: file.content_type().map(ToString::to_string),
            };
            entry.insert(info.clone());
            infos.push(info)
        }
        infos
    }
}

struct Comment {
    user: String,
    text: String,
}

#[Object]
impl Comment {
    async fn user(&self) -> &str {
        time::delay_for(Duration::from_secs(2)).await;
        &self.user
    }

    async fn text(&self) -> &str {
        &self.text
    }
}

struct Book {
    id: i32,
    title: String,
    author: String,
}

#[Object]
impl Book {
    async fn title(&self) -> &str {
        &self.title
    }

    async fn author(&self) -> &str {
        &self.author
    }

    async fn comments(&self) -> Deferred<Option<Vec<Comment>>> {
        let comments = if self.id == 1 {
            vec![
                Comment {
                    user: "John".to_string(),
                    text: "I liked it".to_string(),
                },
                Comment {
                    user: "Mary".to_string(),
                    text: "It is a book".to_string(),
                },
            ]
        } else if self.id == 2 {
            vec![
                Comment {
                    user: "Alberta".to_string(),
                    text: "Amazing :-)".to_string(),
                },
                Comment {
                    user: "Joanna".to_string(),
                    text: "Excellent".to_string(),
                },
            ]
        } else {
            Vec::new()
        };

        Some(comments).into()
    }
}

struct Query;

#[Object]
impl Query {
    async fn books(&self) -> Vec<Book> {
        vec![
            Book {
                id: 1,
                title: "Harry Potter and the Chamber of Secrets".to_string(),
                author: "J.K. Rowling".to_string(),
            },
            Book {
                id: 2,
                title: "Jurassic Park".to_string(),
                author: "Michael Crichton".to_string(),
            },
            Book {
                id: 3,
                title: "Moby Dick".to_string(),
                author: "Herman Melville".to_string(),
            },
        ]
    }
}


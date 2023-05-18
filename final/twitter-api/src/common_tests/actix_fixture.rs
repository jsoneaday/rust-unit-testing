use crate::{
    common::{
        app_state::AppState, 
        entities::base::DbRepo
    }, 
    routes::{
        messages::message_route::{get_message, create_message, get_messages}, 
        profiles::profile_route::{create_profile, get_profile_by_user, get_profile}
    }
};
use std::env;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPool, Postgres, Pool, FromRow};
use actix_web::{
    App,
    web,
    Error,
    test, 
    dev::{Service, ServiceResponse}
};
use actix_http::Request;
use fake::Fake;
use fake::faker::lorem::en::Sentence;
use std::ops::Range;

pub const PUBLIC_GROUP_TYPE: i32 = 1;
pub const CIRCLE_GROUP_TYPE: i32 = 2;

#[allow(unused)]
#[derive(Deserialize, FromRow)]
pub struct MessageResponse {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub original_msg_id: i64,
    pub responding_msg_id: i64
}

#[derive(Debug, Clone)]
pub enum FixtureError {
    MissingData(String),
    QueryFailed(String)
}
impl std::error::Error for FixtureError{}
impl std::fmt::Display for FixtureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingData(msg) => write!(f, "{}", msg),
            Self::QueryFailed(msg) => write!(f, "{}", msg)
        }
    }
}

pub async fn get_conn_pool() -> Pool<Postgres> {
    dotenv().ok();
    let postgres_host = env::var("POSTGRES_HOST").unwrap();
    let postgres_port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_user = env::var("POSTGRES_USER").unwrap();
    let postgres_db = env::var("POSTGRES_DB").unwrap();

    PgPool::connect(&format!("postgres://{postgres_user}:{postgres_password}@{postgres_host}:{postgres_port}/{postgres_db}")).await.unwrap()
}

#[allow(unused)]
pub async fn get_app_state() -> AppState {   
    AppState {
        client: reqwest::Client::new(),
        conn: get_conn_pool().await,
        db_repo: DbRepo
    }
}

#[allow(unused)]
pub async fn get_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    test::init_service(
        App::new()
            .app_data(web::Data::new(get_app_state().await))
            .service(
                web::scope("/v1")
                    .service(
                        web::resource("/msg")
                            .route(web::get().to(get_message))
                            .route(web::post().to(create_message))
                    )
                    .service(
                      web::resource("/msgs")
                          .route(web::get().to(get_messages))   
                    )
                    .service(get_profile)
                    .service(get_profile_by_user)
                    .service(
                        web::resource("/profile")
                            .route(web::post().to(create_profile))
                    )
            )
    ).await
}

pub fn create_random_body(prefix: Option<String>) -> String {
    let mut body: String = match prefix {
        Some(pref) => pref,
        None => "".to_string()
    };

    for _ in [..4] {
        let random_sentence: String = Sentence(Range{ start: 5, end: 6 }).fake();
        body = format!("{}. {}", body, random_sentence);
    }
    body
}

mod model {
    pub mod message;
}

use std::env;
use dotenv::dotenv;
use model::message::{MessageQueryResult, EntityId};
use sqlx::{Postgres, Pool};
//use sqlx::Postgres;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let postgres_host = env::var("POSTGRES_HOST").unwrap();
    let postgres_port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_user = env::var("POSTGRES_USER").unwrap();
    let postgres_db = env::var("POSTGRES_DB").unwrap();

    let postgres_url = format!("postgres://{postgres_user}:{postgres_password}@{postgres_host}:{postgres_port}/{postgres_db}");    
    let conn = sqlx::postgres::PgPool::connect(&postgres_url).await.unwrap();

    let migrate = sqlx::migrate!("./migrations").run(&conn).await;
    match migrate {
        Ok(()) => println!("sqlx migration success"),
        Err(e) => println!("sqlx migration error: {:?}", e)
    };

    let id = add_message(&conn, "First entry".to_string(), "Hello world".to_string()).await;
    println!("added message {}", id.as_ref().unwrap());
    let item = get_message(&conn, id.unwrap()).await;
    println!("get message {}", item.unwrap().unwrap().title);
}

async fn add_message(conn: &Pool<Postgres>, title: String, body: String) -> Result<i64, sqlx::Error> {
    let id_result = sqlx::query_as::<_, EntityId>(
        "insert into message (title, body) values ($1, $2) returning id"
    )
    .bind(title)
    .bind(body)
    .fetch_one(conn)
    .await;

    match id_result {
        Ok(entity) => Ok(entity.id),
        Err(e) => Err(e)
    }
}

async fn get_message(conn: &Pool<Postgres>, id: i64) -> Result<Option<MessageQueryResult>, sqlx::Error> {
    sqlx::query_as::<_, MessageQueryResult>(
        "select * from message where id = $1"
    )
    .bind(id)
    .fetch_optional(conn)
    .await
}
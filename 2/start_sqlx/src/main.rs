mod model {
    pub mod message;
}

use std::env;
use dotenv::dotenv;

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

    println!("completed");
}

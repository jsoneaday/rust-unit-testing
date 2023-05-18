use sqlx::{Pool, Postgres};

use super::entities::base::DbRepo;

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub conn: Pool<Postgres>,
    pub db_repo: DbRepo
}
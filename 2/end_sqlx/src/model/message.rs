use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(FromRow, Deserialize)]
pub struct EntityId {
    pub id: i64
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct MessageQueryResult {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub body: Option<String>
}
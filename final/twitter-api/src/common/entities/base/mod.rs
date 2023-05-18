use serde::Deserialize;
use sqlx::{FromRow};

#[allow(unused)]
#[derive(FromRow, Deserialize)]
pub struct EntityId {
    pub id: i64
}

#[derive(Clone)]
pub struct DbRepo;
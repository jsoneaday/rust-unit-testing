use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct ProfileQuery {
    pub id: i64
}

#[derive(Deserialize)]
pub struct ProfileByUserNameQuery {
    pub user_name: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileShort {
    pub id: i64,
    pub user_name: String,
    pub full_name: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileCreateJson {
    pub user_name: String,
    pub full_name: String,
    pub description: String,
    pub region: Option<String>,
    pub main_url: Option<String>,
    pub avatar: Vec<u8>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponder {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub description: String,
    pub region: Option<String>,
    pub main_url: Option<String>,
    pub avatar: Vec<u8>
}
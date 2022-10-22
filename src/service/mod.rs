use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use serde::Deserialize;

use crate::API_URL;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub url: String,
    pub expired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct RespRecords {
    pub data: Vec<Record>,
}

pub async fn fetch_records() -> RespRecords {
    let url = format!("{}/api/records", API_URL);
    Request::get(&url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

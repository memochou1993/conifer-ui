use chrono::{DateTime, Utc};
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};

use crate::API_URL;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Record {
    pub id: String,
    pub url: String,
    pub token: String,
    pub expired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Record {
    pub fn new(url: String) -> Self {
        Self {
            id: String::from(""),
            url,
            token: String::from(""), // FIXME
            expired_at: chrono::offset::Utc::now(),
            created_at: chrono::offset::Utc::now(),
            updated_at: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Deserialize)]
pub struct RespRecordStore {
    pub data: Record,
}

pub async fn store_record(record: &Record) -> RespRecordStore {
    let url = format!("{}/api/records", API_URL);
    let headers = Headers::new();
    headers.append("Content-Type", "application/json");
    let body = serde_json::to_string(record).unwrap();
    Request::post(&url)
        .body(body)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

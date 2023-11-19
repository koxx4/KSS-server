use chrono::{DateTime, Utc};
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentKssEvent {
    pub _id: bson::oid::ObjectId,
    pub name: String,
    pub count: i64,
    pub image: bson::Binary,
    pub confidence: f64,
    pub date: bson::DateTime,
    pub important: bool,
    pub bounding_boxes: Vec<Vec<i64>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KssEvent {
    pub id: String,
    pub name: String,
    pub count: i64,
    pub confidence: f64,
    pub date: DateTime<Utc>,
    pub important: bool,
    pub bounding_boxes: Vec<Vec<i64>>
}
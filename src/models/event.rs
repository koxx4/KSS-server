use std::collections::HashMap;

use chrono::{DateTime, Utc};
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentKssEvent {
    pub _id: bson::oid::ObjectId,
    pub objects: HashMap<String, u32>,
    pub image_id: bson::oid::ObjectId,
    pub confidence: f64,
    pub date: bson::DateTime,
    pub important: bool,
    pub read: bool,
    pub bounding_boxes: Vec<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KssEventDto {
    pub id: String,
    pub objects: HashMap<String, u32>,
    pub image_id: String,
    pub confidence: f64,
    pub date: DateTime<Utc>,
    pub important: bool,
    pub read: bool,
    pub bounding_boxes: Vec<Vec<i64>>,
}

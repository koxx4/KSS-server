use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    pub _id: ObjectId,
    pub system_on: bool,
    pub input_threshold: i32,
    pub output_threshold: i32,
    pub events_config: Vec<EventConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserPreferencesDto {
    pub input_threshold: i32,
    pub output_threshold: i32,
    pub events_config: Vec<EventConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventConfig {
    pub event_name: String,
    pub important: bool,
    pub precision_threshold: i32,
}

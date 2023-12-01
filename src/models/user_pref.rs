use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentUserPreferences {
    pub _id: u32,
    pub system_on: bool,
    pub input_threshold: i32,
    pub output_threshold: i32,
    pub events_config: Vec<PersistentEventConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentEventConfig {
    pub event_name: String,
    pub important: bool,
    pub precision_threshold: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConfig {
    pub event_name: String,
    pub important: bool,
    pub precision_threshold: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub input_threshold: i32,
    pub output_threshold: i32,
    pub events_config: Vec<EventConfig>,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub important_objects: Vec<String>,
	pub camera_viewport: [i64; 2]
}
use chrono::{DateTime, Utc};
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentKssEvent {
    pub _id: bson::oid::ObjectId,
    pub objects: Vec<PersistentKssEventObject>,
    pub image_id: bson::oid::ObjectId,
    pub date: bson::DateTime,
    pub important: bool,
    pub read: bool,
}

impl PersistentKssEvent {
    pub fn to_dto(&self) -> KssEventDto {
        let objects_dto = self
            .objects
            .iter()
            .map(|obj| KssEventObjectDto {
                name: obj.name.clone(),
                count: obj.count,
                avg_confidence: obj.avg_confidence,
                ncid: obj.ncid.clone(),
            })
            .collect::<Vec<KssEventObjectDto>>();

        let avg_confidence = if self.objects.is_empty() {
            0.0
        } else {
            self.objects
                .iter()
                .map(|obj| obj.avg_confidence as f64)
                .sum::<f64>()
                / self.objects.len() as f64
        };

        KssEventDto {
            id: self._id.to_hex(),
            objects: objects_dto,
            avg_confidence,
            image_id: self.image_id.to_hex(),
            date: self.date.to_chrono(),
            read: self.read,
            important: self.important
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentKssEventObject {
    pub name: String,
    pub count: u32,
    pub avg_confidence: f32,
    pub ncid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KssEventObjectDto {
    pub name: String,
    pub count: u32,
    pub avg_confidence: f32,
    pub ncid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KssEventDto {
    pub id: String,
    pub objects: Vec<KssEventObjectDto>,
    pub avg_confidence: f64,
    pub image_id: String,
    pub date: DateTime<Utc>,
    pub important: bool,
    pub read: bool,
}

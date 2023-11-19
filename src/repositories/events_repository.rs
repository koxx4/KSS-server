use futures::TryStreamExt;
use mongodb::{Client, bson::doc};
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use mongodb::options::FindOptions;
use crate::models::event::{KssEvent, PersistentKssEvent};

const KSS_EVENTS_COLLECTION: &'static str = "kss-events";

pub async fn get_unread_events_count(client: &Client) -> Result<u64> {
    let db = client.database("kss");
    let collection = db.collection::<PersistentKssEvent>(KSS_EVENTS_COLLECTION);
    let filter = doc! {"read": false};

    collection.count_documents(filter, None).await
}

pub async fn get_all_kss_events(client: &Client, page: i64, limit: i64) -> Vec<KssEvent> {

    let db = client.database("kss");
    let collection= db.collection::<PersistentKssEvent>(KSS_EVENTS_COLLECTION);

    let skip: i64 = (page - 1) * limit;

    let date_sort = doc! {"date": -1};

    let mut cursor = collection.find(
        None,
        Some(FindOptions::builder().skip(skip as u64).limit(limit).sort(date_sort).build())
    ).await.unwrap();

    let mut events: Vec<KssEvent> = vec!();

    while let Some(e) = cursor.try_next().await.unwrap() {

        let event: KssEvent = KssEvent {
            id: e._id.to_string(),
            name: e.name,
            count: e.count,
            confidence: e.confidence,
            date: e.date.to_chrono(),
            important: e.important,
            bounding_boxes: e.bounding_boxes
        };

        events.push(event);
    }

    events
}

pub async fn get_event_image(client: &Client, event_id: ObjectId) -> Option<Vec<u8>> {
    let db = client.database("kss");
    let collection = db.collection::<PersistentKssEvent>(KSS_EVENTS_COLLECTION);

    let filter = doc! { "_id": event_id };

    let event = collection.find_one(filter, None).await.unwrap();

    match event {
        Some(e) => Some(e.image.bytes),
        None => None,
    }
}
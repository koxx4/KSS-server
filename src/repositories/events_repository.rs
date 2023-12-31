use crate::models::event::{KssEventDto, PersistentKssEvent};
use futures::{TryStreamExt, AsyncReadExt};
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use mongodb::options::{FindOptions, GridFsBucketOptions};
use mongodb::{bson::doc, Client};

use super::constants::{KSS_DB, KSS_EVENTS_COLLECTION};

pub async fn get_unread_events_count(client: &Client) -> Result<u64> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentKssEvent>(KSS_EVENTS_COLLECTION);
    let filter = doc! {"read": false};

    collection.count_documents(filter, None).await
}

pub async fn get_all_kss_events(client: &Client, page: i64, limit: i64) -> Vec<KssEventDto> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentKssEvent>(KSS_EVENTS_COLLECTION);

    let skip: i64 = (page - 1) * limit;

    let date_sort = doc! {"date": -1};

    let mut cursor = collection
        .find(
            None,
            Some(
                FindOptions::builder()
                    .skip(skip as u64)
                    .limit(limit)
                    .sort(date_sort)
                    .build(),
            ),
        )
        .await
        .unwrap();

    let mut events: Vec<KssEventDto> = vec![];

    while let Some(e) = cursor.try_next().await.unwrap() {
        events.push(e.to_dto());
    }

    events
}

pub async fn mark_events_as_read(client: &Client, event_ids: &[ObjectId]) -> u64 {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentKssEvent>(KSS_EVENTS_COLLECTION);

    // Utworzenie filtru do wyszukiwania dokumentów z określonymi ID
    let filter = doc! {
        "_id": {
            "$in": event_ids
        }
    };

    // Ustawienie pola "read" na true
    let update = doc! {
        "$set": { "read": true }
    };

    // Aktualizacja dokumentów
    collection
        .update_many(filter, update, None)
        .await
        .unwrap()
        .modified_count
}

pub async fn get_event_image(client: &Client, event_image_id: ObjectId) -> Option<Vec<u8>> {
    let img = client.database(KSS_DB)
    .gridfs_bucket(GridFsBucketOptions::default())
    .open_download_stream(bson::Bson::ObjectId(event_image_id))
    .await;

    match img {
        Ok(mut stream) => {
            let mut buffer = Vec::new();

            match stream.read_to_end(&mut buffer).await {
                Ok(_) => Some(buffer),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

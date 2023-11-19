use std::str::FromStr;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use crate::models::event::KssEvent;
use crate::repositories::events_repository;

pub async fn get_unread_count(client: &Client) -> u64 {
    events_repository::get_unread_events_count(client).await.unwrap_or(0)
}

pub async fn get_all_kss_events(client: &Client, page: i64, limit: i64) -> Vec<KssEvent> {
    events_repository::get_all_kss_events(client, page, limit).await
}

pub async fn get_image_for_event(client: &Client, event_id: &str) -> Option<Vec<u8>> {
    events_repository::get_event_image(
        client,
        ObjectId::from_str(event_id).unwrap()
    ).await
}
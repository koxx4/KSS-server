use crate::models::event::KssEventDto;
use crate::repositories::events_repository;
use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use std::str::FromStr;

pub async fn get_unread_count(client: &Client) -> u64 {
    events_repository::get_unread_events_count(client)
        .await
        .unwrap_or(0)
}

pub async fn get_all_kss_events(client: &Client, page: i64, limit: i64) -> Vec<KssEventDto> {
    events_repository::get_all_kss_events(client, page, limit).await
}

pub async fn mark_kss_events_as_read(client: &Client, event_ids: &[&str]) {
    let bson_ids: Vec<ObjectId> = event_ids
        .iter()
        .map(|&id| ObjectId::from_str(id).unwrap())
        .collect();

    let modified_count = events_repository::mark_events_as_read(client, &bson_ids).await;

    info!("Marked {} events as read", modified_count);
}

pub async fn get_image_for_event(client: &Client, event_image_id: &str) -> Option<Vec<u8>> {
    events_repository::get_event_image(client, ObjectId::from_str(event_image_id).unwrap_or_default()).await
}

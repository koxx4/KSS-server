use log::debug;
use mongodb::bson::doc;
use mongodb::options::ReturnDocument;
use mongodb::{bson, Client};
use mongodb::{error::Result, options::FindOneAndUpdateOptions};

use crate::models::user_pref::{
    EventConfig, PersistentEventConfig, PersistentUserPreferences, UserPreferences,
};

use super::constants::{KSS_DB, USER_PREFERENCES_COLLECTION};

pub async fn get_user_preferences(client: &Client) -> Result<Option<UserPreferences>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentUserPreferences>(USER_PREFERENCES_COLLECTION);

    match collection.find_one(doc! {"_id": 1}, None).await {
        Ok(Some(pref)) => Ok(Some(UserPreferences {
            input_threshold: pref.input_threshold,
            output_threshold: pref.output_threshold,
            events_config: pref
                .events_config
                .iter()
                .map(|e| EventConfig {
                    event_name: e.event_name.clone(),
                    important: e.important,
                    precision_threshold: e.precision_threshold,
                })
                .collect(),
        })),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn set_user_preferences(
    client: &Client,
    user_pref: &UserPreferences,
) -> Result<Option<PersistentUserPreferences>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentUserPreferences>(USER_PREFERENCES_COLLECTION);

    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .return_document(ReturnDocument::After)
        .build();

    let update_doc = doc! {
        "input_threshold": user_pref.input_threshold,
        "output_threshold": user_pref.output_threshold,
        "events_config": bson::to_bson(&user_pref.events_config.iter()
            .map(|e| PersistentEventConfig {
            event_name: e.event_name.clone(),
            important: e.important,
            precision_threshold: e.precision_threshold
        }).collect::<Vec<PersistentEventConfig>>()).unwrap()
    };

    let update = doc! {"$set": update_doc};

    debug!("Update {:#?}", update);

    collection
        .find_one_and_update(doc! {"_id": 1}, update, Some(options))
        .await
}

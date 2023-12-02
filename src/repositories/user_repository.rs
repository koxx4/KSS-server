use log::debug;
use mongodb::bson::doc;
use mongodb::options::ReturnDocument;
use mongodb::{bson, Client};
use mongodb::{error::Result, options::FindOneAndUpdateOptions};

use crate::models::user_pref::{
    EventConfig, PersistentEventConfig, PersistentPushToken, PersistentUserPreferences,
    UserPreferences,
};
use crate::repositories::constants::USER_PUSH_TOKENS;

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

pub async fn get_power_status(client: &Client) -> Result<Option<bool>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentUserPreferences>(USER_PREFERENCES_COLLECTION);

    match collection.find_one(doc! {"_id": 1}, None).await {
        Ok(Some(pref)) => Ok(Some(pref.system_on)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn set_power_status(
    client: &Client,
    power_status: bool,
) -> Result<Option<PersistentUserPreferences>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentUserPreferences>(USER_PREFERENCES_COLLECTION);

    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .return_document(ReturnDocument::After)
        .build();

    let update_doc = doc! {
        "system_on": power_status
    };

    let update = doc! {"$set": update_doc};

    debug!("Update {:#?}", update);

    collection
        .find_one_and_update(doc! {"_id": 1}, update, Some(options))
        .await
}

pub async fn set_push_token(
    client: &Client,
    push_token_value: &str,
) -> Result<Option<PersistentPushToken>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<PersistentPushToken>(USER_PUSH_TOKENS);

    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .return_document(ReturnDocument::After)
        .build();

    let ppt = PersistentPushToken {
        token: push_token_value.to_owned(),
        date: bson::DateTime::now(),
    };

    let update_doc = bson::to_document(&ppt).unwrap();

    let update = doc! {"$set": update_doc};

    debug!("Update {:#?}", update);

    collection
        .find_one_and_update(doc! {"token": push_token_value}, update, Some(options))
        .await
}

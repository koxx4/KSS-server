use log::debug;
use bson::{bson, Bson};
use mongodb::bson::doc;
use mongodb::{bson, Client};
use mongodb::{error::Result, options::FindOneAndUpdateOptions};

use crate::models::user_pref::{UserPreferences, UserPreferencesDto};

use super::constants::{KSS_DB, USER_PREFERENCES_COLLECTION};

pub async fn get_user_preferences(client: &Client) -> Result<Option<UserPreferencesDto>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<UserPreferences>(USER_PREFERENCES_COLLECTION);

    match collection.find_one(None, None).await {
        Ok(Some(pref)) => Ok(Some(UserPreferencesDto {
            input_threshold: pref.input_threshold,
            output_threshold: pref.output_threshold,
            events_config: pref.events_config,
        })),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn set_user_preferences(
    client: &Client,
    user_pref: &UserPreferencesDto,
) -> Result<Option<UserPreferences>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<UserPreferences>(USER_PREFERENCES_COLLECTION);

    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .build();

    let update = doc! {"$set": bson!(user_pref)};

    debug!("Update {:#?}", update);

    collection
        .find_one_and_update(doc! {"_id": 1}, update, Some(options))
        .await
}

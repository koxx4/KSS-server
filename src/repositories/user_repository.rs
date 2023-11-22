use mongodb::error::Result;
use mongodb::results::InsertOneResult;
use mongodb::Client;

use crate::models::user_pref::UserPreferences;

use super::constants::{KSS_DB, USER_PREFERENCES_COLLECTION};

pub async fn get_user_preferences(client: &Client) -> Result<Option<UserPreferences>> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<UserPreferences>(USER_PREFERENCES_COLLECTION);

    collection.find_one(None, None).await
}

pub async fn set_user_preferences(
    client: &Client,
    user_pref: &UserPreferences,
) -> Result<InsertOneResult> {
    let db = client.database(KSS_DB);
    let collection = db.collection::<UserPreferences>(USER_PREFERENCES_COLLECTION);

    collection.insert_one(user_pref, None).await
}

use actix_web::web::Json;
use actix_web::{web, post, get, HttpResponse};
use mongodb::Client;
use log::{error, debug};

use crate::models::user_pref::{UserPreferences, PushTokenRequest};
use crate::repositories::user_repository::{get_user_preferences, set_user_preferences, get_power_status, set_power_status, set_push_token};

#[get("/api/kss/preferences")]
async fn get_user_preferences_api(db: web::Data<Client>) -> Json<UserPreferences> {

    debug!("Requested user preferences");

    get_user_preferences(db.as_ref())
        .await
		.map(|prefs| Json(prefs.unwrap()))
        .map_err(|e| {
            error!("Error while retreiving user preferences: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }).unwrap()
}

#[get("/api/kss/preferences/power")]
async fn get_power_status_api(db: web::Data<Client>) -> Json<bool> {

    debug!("Requested user preferences");

    get_power_status(db.as_ref())
        .await
		.map(|status| Json(status.unwrap()))
        .map_err(|e| {
            error!("Error retreiving power status: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }).unwrap()
}

#[post("/api/kss/preferences")]
async fn set_user_preferences_api(
    data: web::Json<UserPreferences>,
    db: web::Data<Client>,
) -> HttpResponse {
    let preferences = data.into_inner();

    debug!("Retreived user pref to save {:#?}", preferences);

    set_user_preferences(db.as_ref(), &preferences)
        .await
		.map(|pref| {
            debug!("Upserted pred: {:#?}", pref);
            HttpResponse::Ok().finish()
        })
        .map_err(|e| {
            error!("Error while saving user preferences: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }).unwrap()
}

#[post("/api/kss/preferences/power")]
async fn set_power_status_api(
    data: web::Json<bool>,
    db: web::Data<Client>,
) -> HttpResponse {
    let power_on = data.into_inner();

    debug!("Retreived power status to set: {:#?}", power_on);

    set_power_status(db.as_ref(), power_on)
        .await
		.map(|pref| {
            debug!("Upserted pred: {:#?}", pref);
            HttpResponse::Ok().finish()
        })
        .map_err(|e| {
            error!("Error while saving power on status: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }).unwrap()
}

#[post("/api/kss/preferences/pushToken")]
async fn set_user_push_token(
    data: web::Json<PushTokenRequest>,
    db: web::Data<Client>,
) -> HttpResponse {
    let token = data.into_inner();

    debug!("Retreived push token to save {:#?}", token);

    set_push_token(db.as_ref(), &token.token)
        .await
		.map(|pref| {
            debug!("Upserted push token: {:#?}", pref);
            HttpResponse::Ok().finish()
        })
        .map_err(|e| {
            error!("Error while saving push token: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }).unwrap()
}
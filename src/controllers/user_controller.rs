use actix_web::{web, post, get, HttpResponse};
use mongodb::Client;
use log::error;

use crate::models::user_pref::UserPreferences;
use crate::repositories::user_repository::{get_user_preferences, set_user_preferences};

#[get("/api/user/preferences")]
async fn get_user_preferences_api(db: web::Data<Client>) -> HttpResponse {

    get_user_preferences(db.as_ref())
        .await
		.map(|_| HttpResponse::Ok().finish())
        .map_err(|e| {
            error!("Błąd zapisu preferencji: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }).unwrap()
}

#[post("/api/user/preferences")]
async fn set_user_preferences_api(
    data: web::Json<UserPreferences>,
    db: web::Data<Client>,
) -> HttpResponse {
    let preferences = data.into_inner();
    set_user_preferences(db.as_ref(), &preferences)
        .await
		.map(|_| HttpResponse::Ok().finish())
        .map_err(|e| {
            error!("Błąd zapisu preferencji: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }).unwrap()
}

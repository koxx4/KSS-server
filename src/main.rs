mod controllers;
mod models;
mod repositories;
mod services;

use crate::controllers::events_controller::{get_event_image, get_events, get_unread_events_count};
use crate::controllers::health_controller::health_check;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use controllers::user_controller::{get_user_preferences_api, set_user_preferences_api, get_power_status_api, set_power_status_api, set_user_push_token};
use env_logger::Env;
use models::user_pref::{PersistentEventConfig, PersistentUserPreferences};
use mongodb::bson::{self, doc};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::{Client, Collection};

async fn initialize_user_preferences(client: &Client) {
    let collection: Collection<PersistentUserPreferences> =
        client
            .database("kss")
            .collection::<PersistentUserPreferences>("user-preferences");

    let event_names = vec![
        "Fire",
        "Smoke",
        "Human",
        "Other",
        "Open pot",
        "Open pot boiling",
        "Closed pot",
        "Closed pot boiling",
        "Dish",
        "Gas",
        "Pan",
        "Closed pan",
    ];
    let events_config: Vec<PersistentEventConfig> = event_names
        .into_iter()
        .map(|name| PersistentEventConfig {
            event_name: name.to_string(),
            important: false,
            precision_threshold: 80,
        })
        .collect();

    let user_pref = doc! {
        "system_on": false,
        "input_threshold": 3,
        "output_threshold": 3,
        "events_config": bson::to_bson(&events_config).unwrap()
    };

    let options = FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    let update = doc! { "$setOnInsert": bson::to_bson(&user_pref).unwrap() };
    let query = doc! { "_id": 1 };

    if collection
        .find_one(query.clone(), None)
        .await
        .unwrap()
        .is_none()
    {
        collection
            .find_one_and_update(query, update, options)
            .await
            .unwrap();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let client = Client::with_uri_str("mongodb://localhost:27017/?replicaSet=rs0")
        .await
        .unwrap();

    // Initializes user preferences on startup if they did not exist before
    initialize_user_preferences(&client).await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(Data::new(client.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(health_check)
            .service(get_events)
            .service(get_unread_events_count)
            .service(get_event_image)
            .service(get_user_preferences_api)
            .service(set_user_preferences_api)
            .service(get_power_status_api)
            .service(set_power_status_api)
            .service(set_user_push_token)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

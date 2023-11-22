mod controllers;
mod models;
mod repositories;
mod services;

use crate::controllers::events_controller::{get_event_image, get_events, get_unread_events_count};
use crate::controllers::events_ws_controller::ws_check_new;
use crate::controllers::health_controller::health_check;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let client = Client::with_uri_str("mongodb://localhost:27017/?replicaSet=rs0")
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Umożliwia żądania z dowolnego źródła
            .allow_any_method() // Umożliwia wszystkie metody HTTP
            .allow_any_header() // Umożliwia wszystkie nagłówki
            .max_age(3600);

        App::new()
            .app_data(Data::new(client.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(health_check)
            .service(get_events)
            .service(get_unread_events_count)
            .service(get_event_image)
            .route("api/kss/ws/check-new", web::get().to(ws_check_new))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

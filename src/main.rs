use std::str::FromStr;
use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, bson};
use serde::{Deserialize, Serialize};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest, web};
use actix_web::middleware::Logger;
use actix_web::web::Json;
use env_logger::Env;
use mongodb::bson::oid::ObjectId;
use log::info;
use mongodb::options::FindOptions;


#[derive(Deserialize)]
struct PaginationParams {
    page: Option<i64>,
    limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistentKssEvent {
    _id: ObjectId,
    name: String,
    count: i64,
    image: bson::Binary,
    confidence: f64,
    date: bson::DateTime,
    important: bool,
    bounding_boxes: Vec<Vec<i64>>
}

#[derive(Debug, Serialize, Deserialize)]
struct KssEvent {
    id: String,
    name: String,
    count: i64,
    confidence: f64,
    date: DateTime<Utc>,
    important: bool,
    bounding_boxes: Vec<Vec<i64>>
}

async fn get_all_kss_events(client: Client, page: i64, limit: i64) -> Vec<KssEvent> {

    let db = client.database("kss");
    let collection= db.collection::<PersistentKssEvent>("kss-events");

    let skip: i64 = (page - 1) * limit;

    let date_sort = doc! {"date": -1};

    let mut cursor = collection.find(
        None,
        Some(FindOptions::builder().skip(skip as u64).limit(limit).sort(date_sort).build())
    ).await.unwrap();

    let mut events: Vec<KssEvent> = vec!();

    while let Some(e) = cursor.try_next().await.unwrap() {

        let event: KssEvent = KssEvent {
            id: e._id.to_string(),
            name: e.name,
            count: e.count,
            confidence: e.confidence,
            date: e.date.to_chrono(),
            important: e.important,
            bounding_boxes: e.bounding_boxes
        };

        events.push(event);
    }

    events
}


#[get("/api/kss/events/latest")]
async fn get_events(query_params: web::Query<PaginationParams>) -> Json<Vec<KssEvent>> {

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let kss_events = get_all_kss_events(client, page, limit).await;

    Json(kss_events)
}

#[get("/api/kss/health")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/api/kss/events/{id}/image")]
async fn get_event_image(req: HttpRequest) -> impl Responder {

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let event_id = req.match_info().get("id").unwrap_or("");
    info!("event id: {}", event_id);
    let db = client.database("kss");
    let collection = db.collection::<PersistentKssEvent>("kss-events");

    match ObjectId::from_str(event_id) {
        Ok(oid) => {
            let filter = doc! { "_id": oid };
            info!("{}", filter);
            match collection.find_one(filter, None).await {
                Ok(Some(event)) => {
                    HttpResponse::Ok().content_type("image/jpeg").body(event.image.bytes)
                }
                Ok(None) => HttpResponse::NotFound().finish(),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(get_events)
            .service(get_event_image)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

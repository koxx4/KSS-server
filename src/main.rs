use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, bson};
use serde::{Deserialize, Serialize};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;

#[derive(Debug, Serialize, Deserialize)]
struct KssEvent {
    name: String,
    count: i64,
    image: bson::Binary,
    confidence: f64,
    date: bson::DateTime,
    important: bool,
    bounding_boxes: Vec<Vec<i64>>
}

async fn get_all_kss_events(client: Client) -> Vec<KssEvent> {

    let db = client.database("kss");

    let collection= db.collection::<KssEvent>("kss-events");
    let start_time = Utc::now() - chrono::Duration::minutes(10);
    let bson_dt = bson::DateTime::from_chrono(start_time);

    let _filter = doc! {
        "timestamp": {
            "$gte": bson_dt
        }
    };

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut events: Vec<KssEvent> = vec!();

    while let Some(e) = cursor.try_next().await.unwrap() {
       events.push(e)
    }

    events
}

#[get("/health")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/api/kss/events")]
async fn get_events() -> Json<Vec<KssEvent>> {

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let kss_events = get_all_kss_events(client).await;

    Json(kss_events)
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_events)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

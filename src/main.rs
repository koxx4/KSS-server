use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, bson};
use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    timestamp: bson::DateTime,
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("kss");

    let collection= db.collection::<Event>("kss-events");
    let start_time = Utc::now() - chrono::Duration::minutes(10);
    let bson_dt = bson::DateTime::from_chrono(start_time);

    let filter = doc! {
        "timestamp": {
            "$gte": bson_dt
        }
    };

    let mut cursor = collection.find(filter, None).await.unwrap();

    while let Some(e) = cursor.try_next().await.unwrap() {
        println!("event-name: {}, when?: {}", e.name, e.timestamp);
    }

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

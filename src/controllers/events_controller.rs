use actix_web::{get, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json, Query};
use mongodb::Client;
use serde::Deserialize;
use crate::models::event::KssEvent;
use crate::services::events_service::{get_all_kss_events, get_image_for_event};

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<i64>,
    limit: Option<i64>,
}

#[get("/api/kss/events/latest")]
async fn get_events(query_params: Query<PaginationParams>, client: Data<Client>) -> Json<Vec<KssEvent>> {

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let kss_events = get_all_kss_events(client.get_ref(), page, limit).await;

    Json(kss_events)
}


#[get("/api/kss/events/{id}/image")]
async fn get_event_image(req: HttpRequest, client: Data<Client>) -> impl Responder {

    let event_id = req.match_info().get("id").unwrap_or("");
    let img = get_image_for_event(client.get_ref(), event_id).await;

    match img {
        Some(bytes) => HttpResponse::Ok().content_type("image/jpeg").body(bytes),
        None => HttpResponse::NotFound().finish(),
    }
}
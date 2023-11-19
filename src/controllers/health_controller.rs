use actix_web::get;

#[get("/api/kss/health")]
pub async fn health_check() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().finish()
}

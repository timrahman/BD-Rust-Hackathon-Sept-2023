use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    log::info!("Looking healthy √");
    HttpResponse::Ok().finish()
}

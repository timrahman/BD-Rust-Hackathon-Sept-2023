use actix_web::HttpResponse;

pub async fn users() -> HttpResponse {
    log::info!("Looking for users √");
    HttpResponse::Ok().finish()
}

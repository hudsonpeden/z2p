use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    println!("Health check");
    HttpResponse::Ok().finish()
}

use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    let response = json!({
        "status": "healthy",
        "service": "nginx_wrapper_api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime": "unknown" 
    });
    
    HttpResponse::Ok().json(response)
}


#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "pong",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
       .service(ping);
}

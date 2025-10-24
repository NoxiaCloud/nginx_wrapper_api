use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;


#[get("/test")]
pub async fn test() -> impl Responder {
    let response = json!({
        "status": "ok",
        "message": "API is running correctly",
        "endpoint": "/test",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    });
    
    HttpResponse::Ok().json(response)
}


#[get("/info")]
pub async fn api_info() -> impl Responder {
    let response = json!({
        "name": "nginx_wrapper_api",
        "description": "REST API for managing NGINX safely and programmatically",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": [
            "/test",
            "/info", 
            "/health",
            "/ping"
        ],
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(response)
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(test)
       .service(api_info);
}

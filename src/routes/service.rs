use actix_web::{ post, get, HttpResponse, Responder };
use serde_json::json;
use std::process::Command;

fn invoke_systemctl(action: &str) -> Result<Option<String>, String> {
    let output = Command::new("systemctl")
        .arg(action)
        .arg("nginx")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(if stdout.is_empty() { None } else { Some(stdout) })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr)
    }
}

#[post("/service/start")]
pub async fn start() -> impl Responder {
    match invoke_systemctl("start") {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Successfully started NGINX"})),
        Err(err) =>
            HttpResponse::InternalServerError().json(
                json!({"message": "Failed to start NGINX", "error": err})
            ),
    }
}

#[post("/service/stop")]
pub async fn stop() -> impl Responder {
    match invoke_systemctl("stop") {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Successfully stopped NGINX"})),
        Err(err) =>
            HttpResponse::InternalServerError().json(
                json!({"message": "Failed to stop NGINX", "error": err})
            ),
    }
}

#[post("/service/reload")]
pub async fn reload() -> impl Responder {
    match invoke_systemctl("reload") {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Successfully reloaded NGINX"})),
        Err(err) =>
            HttpResponse::InternalServerError().json(
                json!({"message": "Failed to reload NGINX", "error": err})
            ),
    }
}

#[post("/service/restart")]
pub async fn restart() -> impl Responder {
    match invoke_systemctl("restart") {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Successfully restarted NGINX"})),
        Err(err) =>
            HttpResponse::InternalServerError().json(
                json!({"message": "Failed to restart NGINX", "error": err})
            ),
    }
}

#[get("/service/status")]
pub async fn status() -> impl Responder {
    match invoke_systemctl("status") {
        Ok(Some(stdout)) =>
            HttpResponse::Ok().json(json!({"message": "NGINX status fetched successfully", "status": stdout})),
        Ok(None) => HttpResponse::Ok().json(json!({"message": "NGINX status fetched successfully"})),
        Err(err) =>
            HttpResponse::InternalServerError().json(
                json!({"message": "Failed to fetch NGINX status", "error": err})
            ),
    }
}

pub fn cfg_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(start).service(stop).service(reload).service(restart).service(status);
}

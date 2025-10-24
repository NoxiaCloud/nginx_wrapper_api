mod routes;

use actix_web::{ App, HttpServer };
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::{ dev::ServiceRequest, Error as ActixError };
use actix_web::error::ErrorUnauthorized;
use routes::init_routes;
use std::env;
use std::path::PathBuf;
use num_cpus;

fn load_dotenv() {
    let exe_dir: PathBuf = env::current_exe().unwrap().parent().unwrap().to_path_buf();

    let dotenv_path = exe_dir.join(".env");
    if dotenvy::from_path(&dotenv_path).is_ok() {
        println!("Loaded .env from {:?}", dotenv_path);
    } else {
        println!("No .env found at {:?}, using defaults or system env", dotenv_path);
    }
}

async fn auth(
    req: ServiceRequest,
    _credentials: BearerAuth
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let expected_key = env::var("API_KEY").expect("API_KEY must be set in .env");

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let valid = auth_header.strip_prefix("Bearer ").map_or(false, |token| token == expected_key);

    if valid {
        Ok(req)
    } else {
        Err((ErrorUnauthorized("Invalid API key"), req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_dotenv();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env
        ::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let workers = env
        ::var("WORKERS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or_else(|| num_cpus::get());

    let addr = format!("{}:{}", host, port);
    println!("Server running at http://{}", addr);

    HttpServer::new(|| { App::new().wrap(HttpAuthentication::bearer(auth)).configure(init_routes) })
        .workers(workers)
        .bind(addr)?
        .run().await
}

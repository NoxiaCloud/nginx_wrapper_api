mod routes;

use actix_web::{
    App,
    HttpServer,
    dev::ServiceRequest,
    Error as ActixError,
    error::ErrorUnauthorized,
};
use actix_web_httpauth::{ middleware::HttpAuthentication, extractors::bearer::BearerAuth };
use routes::init_routes;
use std::env;
use num_cpus;

fn load_dotenv() -> Result<(), Box<dyn std::error::Error>> {
    if dotenvy::dotenv().is_ok() {
        println!("Loaded .env from current directory");
        return Ok(());
    }

    if let Ok(exe_dir) = env::current_exe() {
        if let Some(parent) = exe_dir.parent() {
            let dotenv_path = parent.join(".env");
            if dotenvy::from_path(&dotenv_path).is_ok() {
                println!("Loaded .env from {:?}", dotenv_path);
                return Ok(());
            }
        }
    }

    println!("No .env file found, using system environment variables");
    Ok(())
}

async fn auth(
    req: ServiceRequest,
    _credentials: BearerAuth
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let expected_key = match env::var("API_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            eprintln!("Warning: API_KEY not set or empty, authentication disabled");
            return Ok(req);
        }
    };

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    let is_valid = !token.is_empty() && token == expected_key;

    if is_valid {
        Ok(req)
    } else {
        Err((ErrorUnauthorized("Authentication required"), req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = load_dotenv() {
        eprintln!("Warning: Failed to load .env file: {}", e);
    }

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let port: u16 = port_str
        .parse()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid PORT value: '{}'. Must be a number between 1-65535", port_str)
            )
        })?;

    let workers = env
        ::var("WORKERS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&w| w > 0 && w <= 100)
        .unwrap_or_else(|| num_cpus::get());

    let addr = format!("{}:{}", host, port);
    println!("Starting server..");
    println!("Address: http://{}", addr);
    println!("Workers: {}", workers);

    HttpServer::new(|| { App::new().wrap(HttpAuthentication::bearer(auth)).configure(init_routes) })
        .workers(workers)
        .bind(&addr)?
        .run().await
}

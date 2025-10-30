mod routes;

use actix_web::middleware::Logger as ActixLogger;
use actix_web::{
    App,
    HttpServer,
    dev::ServiceRequest,
    Error as ActixError,
    error::ErrorUnauthorized,
};
use actix_web_httpauth::{ middleware::HttpAuthentication, extractors::bearer::BearerAuth };
use routes::init_routes;
use std::{ env, fs, path::Path };
use num_cpus;
use std::sync::Arc;
use flexi_logger::{ Logger, FileSpec, Duplicate };
use log::{ info, warn, error };

fn load_dotenv() -> Result<(), Box<dyn std::error::Error>> {
    if dotenvy::dotenv().is_ok() {
        info!("Loaded .env from current directory");
        return Ok(());
    }

    if let Ok(exe_dir) = env::current_exe() {
        if let Some(parent) = exe_dir.parent() {
            let dotenv_path = parent.join(".env");
            if dotenvy::from_path(&dotenv_path).is_ok() {
                info!("Loaded .env from {:?}", dotenv_path);
                return Ok(());
            }
        }
    }

    warn!("WARNING: No .env file found, using system environment variables");
    Ok(())
}

async fn auth(
    req: ServiceRequest,
    credentials: BearerAuth,
    expected_key: Arc<String>
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let token = credentials.token();
    let expected_key = expected_key.as_str();

    if token.is_empty() || expected_key.is_empty() || token != expected_key {
        Err((ErrorUnauthorized("Authentication required"), req))
    } else {
        Ok(req)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = load_dotenv() {
        warn!("WARNING: Failed to load .env file: {}", e);
    }

    unsafe {
        env::set_var(
            "RUST_LOG",
            env
                ::var("RUST_LOG")
                .unwrap_or_else(|_| {
                    "actix_web=trace,actix_server=trace,node_agent=trace".into()
                })
        );
    }

    let log_dir = env
        ::var("LOG_DIR")
        .ok()
        .filter(|s| !s.is_empty());
    if let Some(ref dir) = log_dir {
        let path = Path::new(dir);
        fs::create_dir_all(path).expect("Failed to create log directory");
    }

    Logger::try_with_env_or_str("trace")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory(log_dir.as_deref().unwrap_or("./logs"))
                .basename("node-agent")
                .suffix("log")
        )
        .duplicate_to_stdout(Duplicate::All)
        .format(|writer, now, record| {
            write!(
                writer,
                "[{}][{}] {}",
                now.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                &record.args()
            )
        })
        .start()
        .unwrap();

    info!("Logging initialized in {}", log_dir.as_deref().unwrap_or("./logs"));

    let api_key = match env::var("API_KEY") {
        Ok(ref key) if !key.is_empty() => Arc::new(key.clone()),
        _ => {
            error!("ERROR: API_KEY not set, server will not start.");
            std::process::exit(1);
        }
    };

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let port: u16 = port_str.parse().unwrap_or_else(|_| {
        error!("Invalid PORT value: '{}'. Must be a number between 1-65535.", port_str);
        std::process::exit(1);
    });

    let workers = env
        ::var("WORKERS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&w| w > 0 && w <= 100)
        .unwrap_or_else(num_cpus::get);

    let addr = format!("{}:{}", host, port);

    let expected_key = api_key.clone();

    HttpServer::new(move || {
        let expected_key = expected_key.clone();
        let auth_middleware = HttpAuthentication::bearer(move |req, credentials|
            auth(req, credentials, expected_key.clone())
        );

        App::new().wrap(ActixLogger::default()).wrap(auth_middleware).configure(init_routes)
    })
        .workers(workers)
        .bind(&addr)?
        .run().await
}

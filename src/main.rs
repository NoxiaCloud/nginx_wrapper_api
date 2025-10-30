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
use std::{ env, fs, sync::Arc };
use flexi_logger::{ Logger, FileSpec, Duplicate };
use log::{ info, error };

#[inline(always)]
async fn auth(
    req: ServiceRequest,
    credentials: BearerAuth,
    expected_key: Arc<String>
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let token = credentials.token();

    if token.len() != expected_key.len() || token.is_empty() {
        return Err((ErrorUnauthorized("Authentication required"), req));
    }

    if token.as_bytes() != expected_key.as_bytes() {
        return Err((ErrorUnauthorized("Authenticaation required"), req));
    }
    Ok(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap_or_else(|e| {
        eprintln!("ERROR: Failed to load environment variables: {}", e);
        std::process::exit(1);
    });

    unsafe {
        env::set_var(
            "RUST_LOG",
            env
                ::var("RUST_LOG")
                .unwrap_or_else(|_| "actix_web=trace,actix_server=trace,node_agent=trace".into())
        );
    }

    let log_dir = env
        ::var("LOG_DIR")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "./logs".to_string());

    fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    Logger::try_with_env_or_str("trace")
        .unwrap()
        .log_to_file(FileSpec::default().directory(&log_dir).basename("node-agent").suffix("log"))
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

    info!("Logging initialized in {}", log_dir);

    let api_key = Arc::new(
        env
            ::var("API_KEY")
            .ok()
            .filter(|k| !k.is_empty())
            .unwrap_or_else(|| {
                error!("API_KEY not set, server will not start.");
                std::process::exit(1);
            })
    );

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env
        ::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| {
            error!("Invalid PORT value. Must be a number between 1-65535.");
            8080
        });

    let workers = env
        ::var("WORKERS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&w| w > 0 && w <= 100)
        .unwrap_or_else(num_cpus::get);

    let addr = format!("{}:{}", host, port);
    let expected_key = api_key;

    HttpServer::new(move || {
        let expected_key = expected_key.clone();
        let auth_middleware = HttpAuthentication::bearer(move |req, credentials| {
            auth(req, credentials, expected_key.clone())
        });

        App::new().wrap(ActixLogger::default()).wrap(auth_middleware).configure(init_routes)
    })
        .workers(workers)
        .bind(&addr)?
        .run().await
}

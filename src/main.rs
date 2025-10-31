mod routes;

use actix_web::middleware::Logger as ActixLogger;
use actix_web::{
    App,
    HttpServer,
};
use routes::init_routes;
use std::{ env, fs };
use flexi_logger::{ Logger, FileSpec, Duplicate };
use log::{ info, error };

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

    HttpServer::new(move || { App::new().wrap(ActixLogger::default()).configure(init_routes) })
        .workers(workers)
        .bind(&addr)?
        .run().await
}

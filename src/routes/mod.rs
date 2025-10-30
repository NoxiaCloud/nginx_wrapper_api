pub mod system;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    system::metrics::cfg_routes(cfg);
}

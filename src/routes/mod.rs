pub mod service;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    service::cfg_routes(cfg);
}

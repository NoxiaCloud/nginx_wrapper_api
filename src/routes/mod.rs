pub mod test;
pub mod health;

use actix_web::web;


pub fn init_routes(cfg: &mut web::ServiceConfig) {

    test::configure_routes(cfg);
    health::configure_routes(cfg);
}

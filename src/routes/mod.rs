pub mod system;

use actix_web::{ web, dev::ServiceRequest, Error as ActixError, error::ErrorUnauthorized };
use actix_web_httpauth::{ middleware::HttpAuthentication, extractors::bearer::BearerAuth };
use std::sync::Arc;
use std::env;
use log::error;

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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let api_key = Arc::new(
        env::var("API_KEY").unwrap_or_else(|_| {
            error!("API_KEY not set. Server will not start.");
            std::process::exit(1);
        })
    );

    let auth_middleware = HttpAuthentication::bearer(move |req, creds| {
        auth(req, creds, api_key.clone())
    });

    cfg.service(
        web
            ::scope("/system")
            .wrap(auth_middleware.clone())
            .configure(|cfg| {
                system::metrics::cfg_routes(cfg);
                system::network::cfg_routes(cfg);
            })
    );
}

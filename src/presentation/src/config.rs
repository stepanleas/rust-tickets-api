use crate::api;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/tickets")
            .service(api::list_all)
            .service(api::find_one)
            .service(api::create)
            .service(api::update)
            .service(api::delete)
    );
    cfg.service(
        web::scope("/api/health")
            .service(api::startup)
            .service(api::ready)
            .service(api::live)
    );
}
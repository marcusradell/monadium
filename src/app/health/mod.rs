use actix_web::{web, HttpResponse, Responder};

async fn live() -> impl Responder {
    HttpResponse::Ok()
}

async fn ready() -> impl Responder {
    HttpResponse::Ok()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("/live", web::get().to(live))
            .route("/ready", web::get().to(ready)),
    );
}

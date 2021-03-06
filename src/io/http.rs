use crate::io::db;
use crate::io::jwt;
use actix_web::{middleware, web, App, HttpServer};

pub async fn init(
    db_pool: db::Pool,
    jwt: jwt::Jwt,
    address: String,
    configure_list: Vec<fn(&mut web::ServiceConfig)>,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let mut scope = web::scope("/api");

        for configure in configure_list.clone() {
            scope = scope.configure(configure);
        }
        App::new()
            .data(db_pool.clone())
            .data(jwt.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(scope)
    })
    .bind(address)?
    .run();

    println!("Server started.");

    server.await
}

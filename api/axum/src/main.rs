use api_lib::{
    io::{self, Config},
    setup,
};
use tracing::info;

#[tokio::main]
async fn main() {
    io::tracing::init();

    let config = Config::init();

    let app_router = setup(config).await;

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Booting server on http://{}", address);

    let server = axum::Server::bind(&address).serve(app_router.into_make_service());

    info!("Server ready.");

    server.await.expect("Server failed.");
}

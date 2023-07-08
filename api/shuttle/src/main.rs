use api_lib::io::Config;
use tracing::info;

#[shuttle_runtime::main]
async fn shuttle(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let config = Config::init_shuttle(secrets);

    let app_router = api_lib::setup(config).await;

    info!("Server ready.");

    Ok(app_router.into())
}

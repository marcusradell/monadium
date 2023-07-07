use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn _init() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default tracing subscriber.");
}

pub struct Config {
    pub database_url: String,
    pub migrate_db: bool,
}

impl Config {
    pub fn init_shuttle(secrets: shuttle_secrets::SecretStore) -> Self {
        let database_url = secrets
            .get("DATABASE_URL")
            .expect("Missing secret DATABASE_URL");

        let migrate_db = secrets.get("MIGRATE_DB").expect("Missing MIGRATE_DB") == "ON";

        Self {
            database_url,
            migrate_db,
        }
    }

    pub fn init() -> Self {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Missing secret DATABASE_URL");

        let migrate_db = env::var("MIGRATE_DB").expect("Missing MIGRATE_DB") == "ON";

        Self {
            database_url,
            migrate_db,
        }
    }
}

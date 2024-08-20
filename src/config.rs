use dotenv::dotenv;
use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub max_server_backups: usize,
    pub max_local_backups: usize,
    pub local_backups_location: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            api_url: env::var("API_URL").expect("API_URL must be set"),
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
            max_server_backups: env::var("MAX_SERVER_BACKUPS")
                .expect("MAX_SERVER_BACKUPS must be set")
                .parse()
                .expect("MAX_SERVER_BACKUPS must be a valid usize"),
            max_local_backups: env::var("MAX_LOCAL_BACKUPS")
                .expect("MAX_LOCAL_BACKUPS must be set")
                .parse()
                .expect("MAX_LOCAL_BACKUPS must be a valid usize"),
            local_backups_location: env::var("LOCAL_BACKUPS_LOCATION").expect("LOCAL_BACKUPS_LOCATION must be set"),
        }
    }

    pub fn setup_logging() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    }
}
                
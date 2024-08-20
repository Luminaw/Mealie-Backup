use chrono::DateTime;
use chrono::Utc;
use dotenv::dotenv;
use get_backup::{AllBackups, BackupService};
use std::env;
use tokio::{self, io::AsyncWriteExt};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod get_backup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    FmtSubscriber::builder()
        .compact()
        .without_time()
        .with_max_level(Level::TRACE)
        .init();

    dotenv().ok();
    
    let url = env::var("API_URL").expect("API_URL must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let max_server_backups = env::var("MAX_SERVER_BACKUPS")
        .expect("MAX_SERVER_BACKUPS must be set")
        .parse::<usize>()
        .expect("MAX_SERVER_BACKUPS must be a valid usize");
    
    info!("Loaded environment variables");
    info!("Starting backup download...");

    let backup_service = BackupService::new(url, api_key);

    backup_service.create_backup(Some("en-US".to_string())).await?;
    
    // Retrieve all backups and get the first one
    let all_backups = backup_service.get_all_backups(Some("en-US".to_string())).await?;
    let backup = all_backups.backups.first().ok_or("No backups found")?;
    let backup_name = &backup.name;

    // Get the backup token and download the backup
    let backup_token = backup_service.get_backup(&backup_name.clone(), Some("en-US".to_string())).await?;
    let backup_data = backup_service.download_backup(backup_token).await?;

    save_backup(backup_name, backup_data).await.expect("Failed to save backup");

    info!("Backup downloaded and saved successfully");

    cleanup_old_backups(&all_backups, &backup_service, max_server_backups).await.expect("Failed to cleanup old backups");

    Ok(())
}

async fn save_backup(backup_name: &str, backup_data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tokio::fs::File::create(backup_name).await?;
    file.write_all(&backup_data).await?;
    Ok(())
}

async fn cleanup_old_backups(all_backups: &AllBackups, backup_service: &BackupService, max_server_backups: usize) -> Result<(), Box<dyn std::error::Error>> {
    if all_backups.backups.len() >= max_server_backups {
        // Find the oldest backup
        if let Some(oldest_backup) = all_backups.backups.iter().min_by_key(|b| {
            DateTime::parse_from_rfc3339(&b.date).unwrap_or_else(|_| DateTime::<Utc>::MIN_UTC.into())
        }) {
            // Delete the oldest backup
            backup_service.delete_backup(oldest_backup.name.clone(), Some("en-US".to_string())).await?;
            info!("Deleted oldest backup: {}", oldest_backup.name);
        }
    }

    Ok(())
}

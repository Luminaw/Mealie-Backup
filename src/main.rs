use chrono::DateTime;
use chrono::Utc;
use config::Config;
use get_backup::{AllBackups, BackupService};
use std::fs;
use tokio::{self, io::AsyncWriteExt};
use tracing::info;

pub mod get_backup;
pub mod config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let set_config = Config::new();
    
    info!("Loaded environment variables");
    info!("Starting backup download...");

    let backup_service = BackupService::new(set_config.api_url, set_config.api_key);

    backup_service.create_backup(Some("en-US".to_string())).await?;
    
    // Retrieve all backups and get the first one
    let all_backups = backup_service.get_all_backups(Some("en-US".to_string())).await?;
    let backup = all_backups.backups.first().ok_or("No backups found")?;
    let backup_name = &backup.name;

    // Get the backup token and download the backup
    let backup_token = backup_service.get_backup(&backup_name.clone(), Some("en-US".to_string())).await?;
    let backup_data = backup_service.download_backup(backup_token).await?;

    save_backup(backup_name, &set_config.local_backups_location, backup_data).await.expect("Failed to save backup");

    info!("Backup downloaded and saved successfully");

    cleanup_old_backups(&all_backups, &backup_service, set_config.max_server_backups).await.expect("Failed to cleanup old backups");
    cleanup_old_local_backups(&set_config.local_backups_location, set_config.max_local_backups).await?;

    Ok(())
}

async fn save_backup(backup_name: &str, local_backups_location: &str, backup_data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tokio::fs::File::create(format!("{}/{}", local_backups_location, backup_name)).await?;
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

async fn cleanup_old_local_backups(local_backups_location: &str, max_local_backups: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut backups: Vec<_> = fs::read_dir(local_backups_location)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();

    if backups.len() > max_local_backups {
        backups.sort_by_key(|entry| entry.metadata().and_then(|meta| meta.created()).unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH));

        for entry in backups.iter().take(backups.len() - max_local_backups) {
            fs::remove_file(entry.path())?;
            info!("Deleted local backup: {:?}", entry.path());
        }
    }

    Ok(())
}

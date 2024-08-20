# Mealie Backup Service

This project provides a backup service for the Mealie application. It allows you to create, download, and manage backups of your Mealie data using a REST API.

## Features

- Create new backups
- Download existing backups
- Delete old backups if the number of backups exceeds a specified limit

## Prerequisites

- Rust (latest stable version)
- Cargo (latest stable version)

## Setup

1. **Clone the repository:**

    ```sh
    git clone https://github.com/yourusername/mealie-backup-service.git
    cd mealie-backup-service
    ```

2. **Create a `.env` file in the root directory with the following content:**

    ```env
    API_URL=http://your-mealie-api-url
    API_KEY=your-api-key
    MAX_SERVER_BACKUPS=5
    MAX_LOCAL_BACKUPS=14
    LOCAL_BACKUPS_LOCATION=/path/to/local/backups
    ```

3. **Install dependencies:**

    ```sh
    cargo build --release
    ```

## Usage

1. **Run the backup service:**

    ```sh
    cargo run --release
    ```

2. **Create a new backup:**

    The service will automatically create a new backup when it starts.

3. **Download the latest backup:**

    The service will download the latest backup and save it to a file.

4. **Delete old backups:**

    If the number of backups exceeds the `MAX_SERVER_BACKUPS` limit, the oldest backup from the server will be deleted automatically.

## Scheduling the Backup Service

### Windows Task Scheduler

1. **Build the program** to create an executable:
    ```sh
    cargo build --release
    ```

2. **Create a batch file** to run the executable. Create a file similar to [run_backup.bat](/scripts/run_backup.bat).

3. **Create a Task Scheduler task** using the `schtasks` command. Open Command Prompt with administrative privileges and run the following command:
    ```sh
    schtasks /create /tn "MealieBackupTask" /tr "C:\path\to\run_backup.bat" /sc daily /st 00:00
    ```

### Linux Cron Job

1. **Build the Rust program** to create an executable:
    ```sh
    cargo build --release
    ```

2. **Create a shell script** to run the executable. Create a file similar to [run_backup.sh](scripts/run_backup.sh)

3. **Make the shell script executable**:
    ```sh
    chmod +x /path/to/project/run_backup.sh
    ```

4. **Create a cron job** to run the script every day at midnight. Open the crontab editor:
    ```sh
    crontab -e
    ```

    Add the following line to the crontab file:
    ```sh
    0 0 * * * /path/to/project/run_backup.sh
    ```

    This line schedules the `run_backup.sh` script to run every day at midnight.
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

pub async fn heartbeat_task(path: PathBuf) {
    let interval = Duration::from_secs(60);

    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let _ = fs::write(&path, now.to_string());

        sleep(interval).await;
    }
}

use chrono::{DateTime, Utc};
use tokio::process::Command;

pub async fn checkout_from(date: &DateTime<Utc>) -> bool {
    let commit = get_commit(date).await;
    let commit = commit.trim();
    if commit.is_empty() {
        return false;
    }

    checkout(&commit).await
}

pub async fn checkout_main() -> bool {
    checkout("main").await
}

async fn get_commit(date: &DateTime<Utc>) -> String {
    let output = Command::new("git")
        .arg("rev-list")
        .arg("-1")
        .arg(format!("--before={}", date.format("%Y-%m-%d %H:%M:%S")))
        .arg("main")
        .output()
        .await
        .unwrap()
        .stdout;

    String::from_utf8_lossy(&output).to_string()
}

async fn checkout(commit: &str) -> bool {
    let output = Command::new("git")
        .arg("checkout")
        .arg(commit)
        .output()
        .await
        .unwrap();

    output.status.success()
}

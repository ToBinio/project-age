use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
    vec,
};
use tokio::{fs, process::Command, sync::Semaphore};

use async_recursion::async_recursion;

pub async fn get_all_files() -> Vec<PathBuf> {
    let semaphore = Arc::new(Semaphore::new(num_cpus::get()));

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(ProgressStyle::default_spinner());
    pb.set_message("Finding files...");

    let files = get_files(PathBuf::from("."), semaphore.clone()).await;

    pb.finish_with_message(format!("{} files found", files.len()));
    files
}

#[async_recursion]
async fn get_files(path: PathBuf, semaphore: Arc<Semaphore>) -> Vec<PathBuf> {
    let mut files = vec![];
    let mut dirs = vec![];

    let _permit = semaphore.acquire().await.unwrap();

    let mut dir = fs::read_dir(path).await.unwrap();
    while let Ok(Some(entry)) = dir.next_entry().await {
        let path = entry.path();
        let entry_type = entry.file_type().await.unwrap();

        if entry_type.is_file() {
            files.push(path);
        } else if entry_type.is_dir() && !is_ignored(&path).await {
            dirs.push(path);
        }
    }

    drop(_permit);

    let tasks = dirs.into_iter().map(|dir| {
        let semaphore = semaphore.clone();
        async move { get_files(dir, semaphore).await }
    });

    files.extend(join_all(tasks).await.into_iter().flatten());
    files
}

async fn is_ignored(path: &Path) -> bool {
    if path.file_name().unwrap().to_string_lossy().starts_with(".") {
        return true;
    }

    Command::new("git")
        .arg("check-ignore")
        .arg(path.to_str().unwrap())
        .output()
        .await
        .unwrap()
        .status
        .success()
}

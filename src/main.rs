use futures::future::join_all;
use std::{path::PathBuf, sync::Arc, vec};
use tokio::{fs, process::Command, sync::Semaphore};

use async_recursion::async_recursion;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(10));

    let files = get_files(PathBuf::from("."), semaphore.clone()).await;
    println!("files: {:?}", files.len());
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

    let tasks = dirs
        .into_iter()
        .map(|dir| {
            let semaphore = semaphore.clone();
            async move { get_files(dir, semaphore).await }
        })
        .collect::<Vec<_>>();

    files.extend(join_all(tasks).await.into_iter().flatten());
    files
}

async fn is_ignored(path: &PathBuf) -> bool {
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

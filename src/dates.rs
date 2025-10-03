use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use chrono::{DateTime, Datelike};
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{process::Command, sync::Semaphore};

pub async fn parse_dates(files: &[PathBuf]) -> HashMap<String, i32> {
    let semaphore = Arc::new(Semaphore::new(num_cpus::get()));

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar());

    let tasks = files.iter().map(|file| {
        let semaphore = semaphore.clone();
        let pb = pb.clone();

        async move {
            let _permit = semaphore.acquire().await.unwrap();

            let output = blame(file).await;
            let data = get_dates(&output);

            pb.inc(1);
            data
        }
    });

    let dates = join_all(tasks).await.into_iter().flatten();

    let mut map = HashMap::new();
    for date in dates {
        *map.entry(date).or_insert(0) += 1;
    }

    pb.finish_and_clear();

    map
}

async fn blame(path: &Path) -> String {
    let output = Command::new("git")
        .arg("blame")
        .arg("--line-porcelain")
        .arg(path.to_str().unwrap())
        .output()
        .await
        .unwrap()
        .stdout;

    String::from_utf8_lossy(&output).to_string()
}

fn get_dates(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.starts_with("committer-time"))
        .map(|line| line.split(" ").nth(1).unwrap().to_string())
        .map(|line| line.parse().unwrap())
        .map(|date| DateTime::from_timestamp_secs(date).unwrap())
        .map(|date| format!("{}-{:02}", date.year(), date.month()))
        .collect()
}

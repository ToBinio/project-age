use crate::{dates::parse_dates, files::get_all_files};

mod dates;
mod files;

#[tokio::main]
async fn main() {
    let files = get_all_files().await;
    println!("files: {:?}", files.len());

    let map = parse_dates(&files).await;
    for (date, count) in map {
        println!("{:?}: {}", date, count);
    }
}

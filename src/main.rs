use crate::{dates::parse_dates, files::get_all_files, plot::plot_data};

mod dates;
mod files;
mod plot;

#[tokio::main]
async fn main() {
    let files = get_all_files().await;
    let map = parse_dates(&files).await;
    plot_data(map);
}

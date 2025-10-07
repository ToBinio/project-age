use std::collections::HashMap;

use chrono::{Months, Utc};

use crate::{checkout::checkout_main, dates::parse_dates, files::get_all_files, plot::plot_data};

mod checkout;
mod dates;
mod files;
mod plot;

#[tokio::main]
async fn main() {
    let mut data = vec![];

    let mut now = Utc::now();
    let map = get_data().await;

    data.push((now.clone(), map));

    now = now.checked_sub_months(Months::new(1)).unwrap();
    while checkout::checkout_from(&now).await {
        let map = get_data().await;
        data.push((now.clone(), map));

        now = now.checked_sub_months(Months::new(1)).unwrap();
    }

    plot_data(data);

    checkout_main().await;
}

async fn get_data() -> HashMap<String, i32> {
    let files = get_all_files().await;
    parse_dates(&files).await
}

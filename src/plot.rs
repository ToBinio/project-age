use std::collections::HashMap;

use chrono::{DateTime, Utc};
use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    prelude::{BitMapBackend, IntoDrawingArea, IntoSegmentedCoord},
    series::Histogram,
    style::{Color, Palette, Palette99, WHITE},
};

//TODO - flip data 90°

pub fn plot_data(data: Vec<(DateTime<Utc>, HashMap<String, i32>)>) {
    let mut keys = data
        .iter()
        .map(|(_, v)| v)
        .flat_map(|map| map.iter().map(|(k, _)| k.to_string()))
        .collect::<Vec<_>>();
    keys.sort();
    keys.dedup();

    let mut result = vec![];
    for index in (0..data.len()).rev() {
        let mut values = vec![];
        for key in keys.iter() {
            let mut result = (key, 0);

            for date in data[0..index].iter() {
                result.1 += date.1.get(key).unwrap_or(&0);
            }

            values.push(result);
        }

        result.push((data[index].0, values));
    }

    let max = result
        .iter()
        .flat_map(|(_, values)| values.iter().map(|(_, value)| *value))
        .max()
        .unwrap_or(0);

    let root_area = BitMapBackend::new("./age.png", (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Age of Lines", ("sans-serif", 40))
        .build_cartesian_2d(keys.into_segmented(), 0..max)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    for (index, (_date, values)) in result.into_iter().enumerate() {
        let color = Palette99::pick(index).mix(0.5);

        ctx.draw_series(
            Histogram::vertical(&ctx)
                .style(color.filled())
                .margin(1)
                .data(values.into_iter()),
        )
        .unwrap();
    }

    root_area.present().unwrap();
}

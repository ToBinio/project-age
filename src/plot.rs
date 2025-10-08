use std::collections::HashMap;

use chrono::{DateTime, Utc};
use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    prelude::{BitMapBackend, IntoDrawingArea, IntoSegmentedCoord, Rectangle},
    series::Histogram,
    style::{BLACK, Color, Palette, Palette99, WHITE},
};

type TimeFrame = DateTime<Utc>;
type LineAgeKey = String;

pub fn plot_data(data: Vec<(TimeFrame, HashMap<LineAgeKey, i32>)>) {
    let mut line_age_keys = data
        .iter()
        .map(|(_, v)| v)
        .flat_map(|map| map.iter().map(|(k, _)| k.to_string()))
        .collect::<Vec<_>>();
    line_age_keys.sort();
    line_age_keys.dedup();
    line_age_keys.reverse();

    let mut result = vec![];
    for index in 0..line_age_keys.len() {
        let mut values = vec![];
        for (time_frame, data) in data.iter() {
            let mut result = (time_frame.format("%Y-%m").to_string(), 0);

            for line_age_key in line_age_keys[index..line_age_keys.len()].iter() {
                result.1 += data.get(line_age_key).unwrap_or(&0);
            }

            values.push(result);
        }

        result.push((line_age_keys[index].to_string(), values));
    }

    let keys = data
        .iter()
        .map(|(date, _)| date.format("%Y-%m").to_string())
        .rev()
        .collect::<Vec<_>>();

    println!("{:?} - {:?}", line_age_keys, keys);

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

    for (index, (date, values)) in result.iter().enumerate() {
        let color = Palette99::pick(index);

        let data = values
            .iter()
            .map(|(label, value)| (label, *value))
            .collect::<Vec<_>>();

        ctx.draw_series(
            Histogram::vertical(&ctx)
                .style(color.filled())
                .margin(1)
                .data(data),
        )
        .unwrap()
        .label(date)
        .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], color.filled()));
    }

    ctx.configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();
    root_area.present().unwrap();
}

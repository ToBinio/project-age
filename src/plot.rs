use std::collections::HashMap;

use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    prelude::{BitMapBackend, IntoDrawingArea, IntoSegmentedCoord},
    series::Histogram,
    style::{Color, WHITE, full_palette::BLUEGREY},
};

pub fn plot_data(data: HashMap<String, i32>) {
    let mut data = data.into_iter().collect::<Vec<_>>();
    data.sort_by(|a, b| a.0.cmp(&b.0));

    let max = data.iter().map(|(_, v)| *v).max().unwrap_or(0);
    let keys = data.iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>();

    let root_area = BitMapBackend::new("./age.png", (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Age of Lines", ("sans-serif", 40))
        .build_cartesian_2d(keys.into_segmented(), 0..max)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        Histogram::vertical(&ctx)
            .style(BLUEGREY.filled())
            .margin(1)
            .data(data.iter().map(|(k, v)| (k, *v))),
    )
    .unwrap();

    root_area.present().unwrap();
}

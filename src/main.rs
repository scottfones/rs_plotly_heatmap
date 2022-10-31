// use plotly::{common::Title, layout::Axis, HeatMap, ImageFormat, Layout, Plot};
use plotly::{layout::{Axis, Margin}, HeatMap, Layout, Plot};

fn main() -> std::io::Result<()> {
    let z = vec![
        0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 45, 45, 45, 45, 60, 45, 45, 45, 45, 10, 10, 10, 10, 10,
        0, 0, 0, 0, 0,
    ];
    let z_n: i32 = z.len().try_into().unwrap();
    let x: Vec<i32> = (1..z_n).collect();
    let y: Vec<i32> = vec![0; z.len()];
    let trace = HeatMap::new(x, y, z).zmax(100).zmin(0);

    let x_axis = Axis::new().tick_values(vec![]);
    let y_axis = Axis::new().tick_values(vec![]);
    let c_layout = Layout::new()
        .width(1200)
        .height(400)
        .x_axis(x_axis)
        .y_axis(y_axis)
        // .title(Title::new("Advection: 1D Heatmap"))
        .margin(Margin::new().left(0).right(0).top(0).bottom(0));
        // .plot_background_color(NamedColor::Black)
        // .paper_background_color(NamedColor::Black);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(c_layout);

    // plot.save("a1d_0.png", ImageFormat::PNG,  400, 100, 1.0);
    plot.show();

    Ok(())
    // println!("{}", plot.to_inline_html(Some("advection_1d heatmap")));
}

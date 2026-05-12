use clap::Parser;
use kuva::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}

fn main() {
    println!("Hello, world!");
    let x_left = -1.0;
    let x_right = 1.0;
    let potential = |x: f64| (x - x_left) * (x - x_left) * (x - x_right) * (x - x_right);
    let xs = (0..100)
        .map(|ix| -2.0 + 0.04 * ix as f64)
        .collect::<Vec<f64>>();
    let ys = xs.iter().map(|x| potential(*x)).collect::<Vec<f64>>();
    let data: Vec<(f64, f64)> = xs.into_iter().zip(ys.into_iter()).collect();
    let plot = ScatterPlot::new()
        .with_data(data)
        .with_color("steelblue")
        .with_size(5.0);

    let plots: Vec<Plot> = vec![plot.into()];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("My Plot")
        .with_x_label("X")
        .with_y_label("Y");

    let pdf = render_to_pdf(plots, layout).unwrap();
    std::fs::write("my_plot.pdf", pdf).unwrap();
}

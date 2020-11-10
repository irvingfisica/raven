use std::error::Error;
use std::process;
use ordered_float::OrderedFloat;
use plotters::prelude::*;

use raven::RawFrame;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let datos = RawFrame::from_arg(1)?;

    let col_x = "Reservations";
    let col_y = "Pizzas";

    let extent_x: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil(col_x)?;
    let extent_y: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil(col_y)?;

    let x_range = extent_x.0.into_inner()..extent_x.1.into_inner();
    let y_range = extent_y.0.into_inner()..extent_y.1.into_inner();


    let drawing_area = BitMapBackend::new("./imagenes/test.png", (1024, 768)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
                    .caption("Pizzas!", ("Arial", 30))
                    .margin(10)
                    .set_label_area_size(LabelAreaPosition::Left, 50)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(x_range,y_range)?;

    chart.configure_mesh()
            .y_desc("Pizzas")
            .x_desc("Reservaciones")
            .axis_desc_style(("sans-serif", 20))
            .draw().unwrap();

    chart.draw_series(
        LineSeries::new(datos.pair_col_fil_sorted(col_x,col_y)? ,&RED),
    )?;

    chart.draw_series(
        AreaSeries::new(datos.pair_col_fil_sorted(col_x,col_y)?,0.0,&BLUE.mix(0.2)),
    )?;

    chart.draw_series(datos.pair_col_fil(col_x, col_y).unwrap().map(|point| Circle::new(point, 3, &RED)))
        .unwrap();

    Ok(())
}

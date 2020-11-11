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

    let col_x = "longitud";
    let col_y = "latitud";

    let extent_x: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil(col_x)?;
    let extent_y: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil(col_y)?;

    let x_range = extent_x.0.into_inner()..extent_x.1.into_inner();
    let y_range = extent_y.0.into_inner()..extent_y.1.into_inner();


    let drawing_area = BitMapBackend::new("./imagenes/test_2.png", (1024, 768)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
                    .build_cartesian_2d(x_range,y_range)?;

    // chart.draw_series(datos.pair_col_fil(col_x, col_y).unwrap().map(|point| Circle::new(point, 1, &RED)))
    //     .unwrap();

    chart.draw_series(datos.slice_col_imp(vec![col_x, col_y],vec![0.0,0.0]).unwrap().map(|point| Circle::new((point[0],point[1]), 1, &RED)))
        .unwrap();

    Ok(())
}

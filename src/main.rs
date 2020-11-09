use std::error::Error;
use std::process;
use  ordered_float::OrderedFloat;

use raven::RawFrame;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let datos = RawFrame::from_arg(1)?;

    let maximo: OrderedFloat<f64> = datos.max_num_fil("latitud").unwrap();
    let minimo: OrderedFloat<f64> = datos.min_num_fil("latitud").unwrap();
    let extent: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil("longitud").unwrap();

    println!("{}",maximo.into_inner());
    println!("{}",minimo);
    println!("{:?}",extent);

    Ok(())
}

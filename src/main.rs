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

    let col_a = "col_a";
    let col_b = "col_b";

    let maximo: OrderedFloat<f64> = datos.max_num_fil(col_a).unwrap();
    let minimo: OrderedFloat<f64> = datos.min_num_fil(col_a).unwrap();
    let extent: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil(col_b).unwrap();

    println!("{}",maximo.into_inner());
    println!("{}",minimo);
    println!("{:?}",extent);
    println!("{:?}",extent.0.into_inner()..extent.1.into_inner());

    let pares_f: Vec<(f64,f64)> = datos.pair_col_fil(col_a,col_b).unwrap().collect();
    let pares_i: Vec<(f64,f64)> = datos.pair_col_imp(col_a,col_b,0.0,0.0).unwrap().collect();

    println!("{:?}",pares_f);
    println!("{:?}",pares_i);

    Ok(())
}

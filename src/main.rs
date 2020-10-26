use std::error::Error;
use std::process;

use raven::RawFrame;
use raven::Datum;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let datos = RawFrame::from_arg(1)?;

    println!("{:#?}",datos.columns);
    println!("{:#?}",datos.records[0]);
    println!("{:#?}",datos.records[1]);

    // println!("{:?}",datos.col_index("ageb"));

    // let valoresn: Vec<Datum> = datos.column("ageb")?.collect();

    // println!("{:?}",valoresn);

    // println!("{:?}",datos.column("latitud")?.last());
    // println!("{:?}",datos.column("longitud")?.last());

    let valoresn: Vec<f64> = datos.col_float_fil("Pizzas")?.collect();
    println!("{:?}",valoresn);

    Ok(())
}

use std::error::Error;
use std::process;

use raven::RawFrame;

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

    println!("{:?}",datos.col_index("nom_vial"));
    println!("{:?}",datos.col_index("nom_vial"));

    // let valorest: Vec<Option<&str>> = datos.get_column("per_ocu")?.collect();
    let valoresn: Vec<Option<f64>> = datos.get_column_numeric("latitud")?.collect();

    println!("{:?}",valoresn);


    Ok(())
}

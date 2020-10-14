use std::error::Error;
use std::process;

use raven::adquisicion;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let datos = adquisicion::leer_datos()?;

    println!("{:?}",datos);

    Ok(())
}

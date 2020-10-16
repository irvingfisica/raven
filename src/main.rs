use std::error::Error;
use std::process;

use raven::Frame;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let datos = Frame::from_arg(1)?;

    println!("{:#?}",datos.registros[0]);
    println!("{:#?}",datos.columns());

    Ok(())
}

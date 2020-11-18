use std::error::Error;
use std::process;

use ravencol::RawFrame;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut datos1 = RawFrame::from_arg(1)?;
    let datos2 = RawFrame::from_arg(1)?;

    datos1.concat(datos2)?;

    println!("{:?}",datos1.records);

    Ok(())
}

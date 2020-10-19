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

    Ok(())
}

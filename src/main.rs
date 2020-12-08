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
    let datos = RawFrame::from_arg(1)?;

    let mcvec: Vec<f64> = datos.column_major_vector(vec!["col_a","col_b"],vec![0.0,0.0])?;
    println!("{:?}",mcvec);

    let mrvec: Vec<f64> = datos.row_major_vector(vec!["col_a","col_b"],vec![0.0,0.0])?;
    println!("{:?}",mrvec);

    Ok(())
}

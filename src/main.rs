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

    println!("{:#?}",datos.columns.get(1).unwrap());
    // println!("{:#?}",datos.records[1]);

    // println!("Columna 'col_a' está en la posición: {:?}",datos.col_index("col_b"));

    // let col_dat_test_a: Vec<Datum> = datos.column("col_a")?.collect();
    // let col_dat_test_b: Vec<Datum> = datos.column("col_b")?.collect();
    // println!("{:?}",col_dat_test_a);
    // println!("{:?}",col_dat_test_b);

    // let col_typ_test: Vec<Option<i32>> = datos.col_type("col_a")?.collect();
    // println!("{:?}",col_typ_test);

    // let col_imp_test: Vec<i32> = datos.col_imp("col_b",0)?.collect();
    // println!("{:?}",col_imp_test);

    // let col_fil_test: Vec<i32> = datos.col_fil("col_a")?.collect();
    // println!("{:?}",col_fil_test);

    Ok(())
}

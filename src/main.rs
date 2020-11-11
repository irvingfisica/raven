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

    println!("{:?}",datos.columns);

    // let it_bool = datos.col_imp("entidad",String::from("")).unwrap().map(|ele|{ele == "CIUDAD DE MÉXICO"});

    // let salida_2: Vec<(f64,f64)> = raven::utils::bool_filter(it_bool,datos.pair_col_imp("longitud","latitud",0.0,0.0).unwrap()).collect();

    // println!("{:?}",salida_2);

    let vec_test: Vec<Vec<f64>> = datos.slice_col_fil(vec!["Reservations","Pizzas"]).unwrap().collect();

    println!("{:?}",vec_test);

    Ok(())
}

use std::error::Error;
use std::process;

// use raven::Frame;
use raven::lectura;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // let datos = Frame::from_arg(1)?;

    // println!("{:#?}",datos.registros[0]);
    // println!("{:#?}",datos.columns());

    let archivo = lectura::leer_argumento(1)?;

    // let datos = lectura::get_data_src(archivo)?;
    // let datos = lectura::get_data_vec(archivo)?;
    // let datos = lectura::get_data_hsm(archivo)?;
    // let datos = lectura::get_data_brc(archivo)?;
    // let datos = lectura::get_data_src_h(archivo)?;
    let datos = lectura::get_data_brc_h(archivo)?;

    println!("{:?}",datos[0]);
    println!("{:?}",datos[1]);

    Ok(())
}

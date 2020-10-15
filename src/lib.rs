use std::error::Error;
use std::ffi::OsString;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Dato {
    Float(f64),
    Text(String),
    None,
}

#[derive(Debug)]
pub struct Registro {
    datos: HashMap<String, Dato>,
}

impl Registro {
    pub fn from_hash(hash_cadenas: HashMap<String, Option<String>>) -> Registro {
        
        let mut datos: HashMap<String, crate::Dato> = HashMap::new();

        for (key, val) in hash_cadenas.iter() {
            let parsed: crate::Dato = match val {
                None => crate::Dato::None,
                Some(cadena) => {
                    let float_try: Result<f64,_> = cadena.parse();
                    match float_try {
                        Ok(float) => crate::Dato::Float(float),
                        Err(_) => crate::Dato::Text(cadena.to_string()),
                    }
                },
            };

            datos.insert(key.to_string(),parsed);
            
        };

        crate::Registro{datos}

    }
}

#[derive(Debug)]
pub struct Frame {
    registros: Vec<Registro>,
}

impl Frame {
    pub fn from_vec(registros: Vec<crate::Registro>) -> Frame {
        crate::Frame{registros}
    }

    pub  fn from_os_string(file_path: OsString) -> Result<crate::Frame, Box<dyn Error>> {

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let mut registros: Vec<crate::Registro> = Vec::new();

        for fila in rdr.deserialize() {

            let fila_leida: HashMap<String, Option<String>> = fila?;

            let registro = crate::Registro::from_hash(fila_leida);
            registros.push(registro);

        };

        Ok(crate::Frame::from_vec(registros))
        
    }

    pub fn from_arg(n: usize) -> Result<crate::Frame, Box<dyn Error>> {

        let ruta = crate::lectura::leer_argumento(n)?;
        let datos = crate::Frame::from_os_string(ruta)?;

        Ok(datos)

    }
}

pub mod lectura {

    use std::env;
    use std::error::Error;
    use std::ffi::OsString;

    pub fn leer_argumento(n: usize) -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(n) {
            Some(file_path) => Ok(file_path),
            None => Err(From::from("No se pudo leer el argumento"))
        }
    }

}
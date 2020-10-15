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

#[derive(Debug)]
pub struct Frame {
    registros: Vec<Registro>,
}

pub mod lectura {

    use std::env;
    use std::error::Error;
    use std::ffi::OsString;
    use std::collections::HashMap;

    fn leer_argumento() -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(1) {
            Some(file_path) => Ok(file_path),
            None => Err(From::from("No se pudo leer el argumento"))
        }
    }

    pub  fn leer_registros(file_path: OsString) -> Result<crate::Frame, Box<dyn Error>> {

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let mut registros: Vec<crate::Registro> = Vec::new();

        for fila in rdr.deserialize() {

            let fila_leida: HashMap<String, Option<String>> = fila?;
            let mut datos: HashMap<String, crate::Dato> = HashMap::new();

            for (key, val) in fila_leida.iter() {
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
            registros.push(crate::Registro{datos});
        };

        Ok(crate::Frame{registros})
        
    }

    pub fn leer_datos() ->Result<crate::Frame, Box<dyn Error>> {
        
        let ruta = leer_argumento()?;
        let datos = leer_registros(ruta)?;

        Ok(datos)
    }

}
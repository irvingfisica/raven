pub mod adquisicion {

    use std::env;
    use std::error::Error;
    use std::ffi::OsString;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum Dato {
        Float(f64),
        Text(String),
        None,
    }

    pub type Registro = HashMap<String, Dato>;

    fn leer_argumento() -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(1) {
            Some(file_path) => Ok(file_path),
            None => Err(From::from("No se pudo leer el argumento"))
        }
    }

    pub fn leer_registros(file_path: OsString) -> Result<Vec<Registro>, Box<dyn Error>> {

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let mut datos: Vec<Registro> = Vec::new();

        for result in rdr.deserialize() {

            let record_leido: HashMap<String, Option<String>> = result?;
            let mut record_escrito: Registro = HashMap::new();

            for (key, val) in record_leido.iter() {
                let parsed: Dato = match val {
                    None => Dato::None,
                    Some(cadena) => {
                        let float_try: Result<f64,_> = cadena.parse();
                        match float_try {
                            Ok(float) => Dato::Float(float),
                            Err(_) => Dato::Text(cadena.to_string()),
                        }
                    },
                };

                record_escrito.insert(key.to_string(),parsed);
                
            };
            datos.push(record_escrito);
        };

        Ok(datos)
        
    }

    pub fn leer_datos() ->Result<Vec<Registro>, Box<dyn Error>> {
        
        let ruta = leer_argumento()?;
        let datos = leer_registros(ruta)?;

        Ok(datos)
    }

}
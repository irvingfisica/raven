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
    pub datos: HashMap<String, Dato>,
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

    pub fn columns(&self) -> Vec<String> {
        self.datos.keys().map(|key| {key.clone()}).collect()
    }
}

#[derive(Debug)]
pub struct Frame {
    pub registros: Vec<Registro>,
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

        // let headers = rdr.headers();
        // println!("{:?}", headers);

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

    pub fn columns(&self) -> Vec<String> {
        self.registros[0].columns()
    }
}

pub mod lectura {

    use std::env;
    use std::error::Error;
    use std::ffi::OsString;
    use std::collections::HashMap;

    pub fn leer_argumento(n: usize) -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(n) {
            Some(file_path) => Ok(file_path),
            None => Err(From::from("No se pudo leer el argumento"))
        }
    }

    pub fn get_data_src(file_path: OsString) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {

        let mut vector: Vec<csv::StringRecord> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        for result in rdr.records() {
            let record = result?;
            vector.push(record);
        }

        Ok(vector)
    }

    pub fn get_data_vec(file_path: OsString) -> Result<Vec<Vec<String>>, Box<dyn Error>> {

        let mut vector: Vec<Vec<String>> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        for result in rdr.deserialize() {
            let record: Vec<String> = result?;
            vector.push(record);
        }

        Ok(vector)
    }

    pub fn get_data_hsm(file_path: OsString) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {

        let mut vector: Vec<HashMap<String, String>> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        for result in rdr.deserialize() {
            let record: HashMap<String, String> = result?;
            vector.push(record);
        }

        Ok(vector)
    }

    pub fn get_data_brc(file_path: OsString) -> Result<Vec<csv::ByteRecord>, Box<dyn Error>> {

        let mut vector: Vec<csv::ByteRecord> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        for result in rdr.byte_records() {
            let record = result?;
            vector.push(record);
        }

        Ok(vector)
    }


}
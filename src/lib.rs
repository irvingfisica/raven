use std::error::Error;
use std::ffi::OsString;

#[derive(Debug)]
pub struct RawFrame {
    pub records: Vec<csv::StringRecord>,
    pub columns: csv::StringRecord,
}

impl RawFrame {
    pub fn from_os_string(file_path: OsString) -> Result<crate::RawFrame, Box<dyn Error>> {

        let (columns,records) = crate::reading::get_data_src_h(file_path)?;

        Ok(crate::RawFrame{columns, records})

    }

    pub fn from_arg(n: usize) -> Result<crate::RawFrame, Box<dyn Error>> {

        let ruta = crate::reading::read_arg(n)?;
        let datos = crate::RawFrame::from_os_string(ruta)?;

        Ok(datos)

    }

    pub fn col_index(&self, column: &str) -> Option<usize> {
        let cadena = String::from(column);
        self.columns.iter().position(|col| col == cadena)
    }

    pub fn get_column(&self, column: &str) -> Result<impl Iterator<Item=Option<&str>> + '_,Box<dyn Error>>{

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            record.get(position)
        }))


    }

    pub fn get_column_numeric(&self, column: &str) -> Result<impl Iterator<Item=Option<f64>> + '_,Box<dyn Error>>{
    
        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            let result:Option<f64> = match record.get(position) {
                None => None,
                Some(cadena) => match cadena.parse() {
                    Ok(num) => Some(num),
                    _ => None,
                }
            };

            result
        }))
    }

}

pub mod reading {

    use std::env;
    use std::error::Error;
    use std::ffi::OsString;
    use std::collections::HashMap;

    pub fn read_arg(n: usize) -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(n) {
            Some(file_path) => Ok(file_path),
            None => Err(From::from("No se pudo leer el argumento"))
        }
    }

    pub fn get_data_src(file_path: OsString) -> Result<(csv::StringRecord,Vec<csv::StringRecord>), Box<dyn Error>> {

        let mut vector: Vec<csv::StringRecord> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let columns = rdr.headers()?.clone();

        for result in rdr.records() {
            let record = result?;
            vector.push(record);
        }

        Ok((columns,vector))
    }

    pub fn get_data_vec(file_path: OsString) -> Result<(csv::StringRecord,Vec<Vec<String>>), Box<dyn Error>> {

        let mut vector: Vec<Vec<String>> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let columns = rdr.headers()?.clone();

        for result in rdr.deserialize() {
            let record: Vec<String> = result?;
            vector.push(record);
        }

        Ok((columns,vector))
    }

    pub fn get_data_hsm(file_path: OsString) -> Result<(csv::StringRecord,Vec<HashMap<String, String>>), Box<dyn Error>> {

        let mut vector: Vec<HashMap<String, String>> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let columns = rdr.headers()?.clone();

        for result in rdr.deserialize() {
            let record: HashMap<String, String> = result?;
            vector.push(record);
        }

        Ok((columns,vector))
    }

    pub fn get_data_brc(file_path: OsString) -> Result<(csv::StringRecord,Vec<csv::ByteRecord>), Box<dyn Error>> {

        let mut vector: Vec<csv::ByteRecord> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let columns = rdr.headers()?.clone();

        for result in rdr.byte_records() {
            let record = result?;
            vector.push(record);
        }

        Ok((columns,vector))
    }

    pub fn get_data_src_h(file_path: OsString) -> Result<(csv::StringRecord,Vec<csv::StringRecord>), Box<dyn Error>> {

        let mut vector: Vec<csv::StringRecord> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let columns = rdr.byte_headers()?.clone();
        let columns = csv::StringRecord::from_byte_record_lossy(columns);

        let mut iter = rdr.into_records();

        loop {
            let row = match iter.next() {
                Some(rec) => rec,
                None => break,
            };

            let record = match row {
                Ok(rec) => rec,
                Err(_) => continue,
            };

            vector.push(record);
        }

        Ok((columns,vector))
    }

    pub fn get_data_brc_h(file_path: OsString) -> Result<(csv::StringRecord,Vec<csv::StringRecord>), Box<dyn Error>> {

        let mut vector: Vec<csv::StringRecord> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

            let columns = rdr.byte_headers()?.clone();
            let columns = csv::StringRecord::from_byte_record_lossy(columns);

        let mut iter = rdr.into_byte_records();

        loop {
            let row = match iter.next() {
                Some(rec) => rec,
                None => break,
            };

            let record = match row {
                Ok(rec) => csv::StringRecord::from_byte_record_lossy(rec),
                Err(_) => continue,
            };

            vector.push(record);
        }

        Ok((columns,vector))
    }

}
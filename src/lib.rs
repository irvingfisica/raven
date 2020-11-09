//! Raven
//!
//! `Raven` is a collection of utilities for processing data for data analysis. Up to now it can reads data only from CSV files.
//! 
use std::error::Error;
use std::ffi::OsString;

/// Enum to contain a datum, it can be an Integer, a Float, an String or None.
#[derive(Debug, PartialEq)]
pub enum Datum<'a> {
    Integer(i32),
    Float(f64),
    NotNumber(&'a str),
    None
}

/// Main data struct. It contains a vec of StringRecords and the name of the columns from the CSV file.
/// 
/// The normal way of creating a RawFrame is from a CSV file. This file will be parsed with CSV crate functions.
/// The struct has a creator method using os_strings as path for the CSV file and a creator method form terminal arg in position n.
/// 
/// A RawFrame is similir to a DataFrame. It is a tabular data structure. 
/// It is possible to operate over columns which are created with methods.
/// All the column extraction methods return iterators, the main objective extracting a column is to operate with it.
/// The intention is ti produce an iterator which is consumed in the column calculation.
/// 
/// In order to computo over columns it is important to decide the type of datum to use in the calculation and if the operation
/// needs full columns or only parsed valid columns for the required type.
/// 
/// Once the type of the column is decided it is necessary to decide what to do with the data which is not possible to parse to this type.
/// Three options are provided. Keep it blank through Option types, impute it with a none_value or filter them out of the column.
#[derive(Debug)]
pub struct RawFrame {
    pub records: Vec<csv::StringRecord>,
    pub columns: csv::StringRecord,
}

impl RawFrame {
    /// Creates a RawFrame from an os_string.
    /// 
    /// # Arguments
    ///
    /// * `file_path` - An OsString that holds the path of CSV file
    ///
    /// # Examples
    ///
    /// ```
    /// use raven::RawFrame;
    /// use std::ffi::OsString;
    ///
    /// let path = OsString::from("./datos_test/test.csv");
    /// let datos = RawFrame::from_os_string(path).unwrap();
    /// ```
    pub fn from_os_string(file_path: OsString) -> Result<crate::RawFrame, Box<dyn Error>> {

        let (columns,records) = crate::reading::get_data_src_h(file_path)?;

        Ok(crate::RawFrame{columns, records})

    }

    /// Creates a RawFrame from terminal argument in position n.
    /// 
    /// # Arguments
    ///
    /// * `n` - A usize that holds the position of the terminal argument on which is the path of CSV file
    pub fn from_arg(n: usize) -> Result<crate::RawFrame, Box<dyn Error>> {

        let ruta = crate::reading::read_arg(n)?;
        let datos = crate::RawFrame::from_os_string(ruta)?;

        Ok(datos)

    }

    /// Returns the position index for column in RawFrame or None if column does not exists.
    /// 
    /// # Arguments
    ///
    /// * `column` - A string slice that holds the name of the column
    ///
    /// # Examples
    ///
    /// ```
    /// use raven::RawFrame;
    /// use std::ffi::OsString;
    ///
    /// fn get_data() -> raven::RawFrame {
    ///     let path = OsString::from("./datos_test/test.csv");
    ///     let datos = RawFrame::from_os_string(path).unwrap();
    ///     datos
    /// } 
    /// 
    /// let datos = get_data();
    /// 
    /// assert_eq!(datos.col_index("col_b"),Some(1));
    /// ```
    pub fn col_index(&self, column: &str) -> Option<usize> {
        let cadena = String::from(column);
        self.columns.iter().position(|col| col == cadena)
    }

    /// Returns a full column of Datum. 
    /// The column is in a consumible iterator. Each element has Datum type. All the valid rows are included.
    /// The Datum type mixes several posibilities of types, this generates a general column.
    /// For simple operations or plotting use specific type columns.
    /// 
    /// # Arguments
    ///
    /// * `column` - A string slice that holds the name of the column
    ///
    /// # Examples
    ///
    /// ```
    /// use raven::RawFrame;
    /// use raven::Datum;
    /// use std::ffi::OsString;
    ///
    /// fn get_data() -> raven::RawFrame {
    ///     let path = OsString::from("./datos_test/test.csv");
    ///     let datos = RawFrame::from_os_string(path).unwrap();
    ///     datos
    /// }
    /// 
    /// let datos = get_data();
    /// 
    /// let col: Vec<Datum> = datos.column("col_a").unwrap().collect();
    /// ```
    pub fn column(&self, column: &str) -> Result<impl Iterator<Item=Datum> + '_,Box<dyn Error>>{
    
        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => Datum::None,
                Some(cadena) => match cadena.parse::<i32>() {
                    Ok(num) => Datum::Integer(num),
                    _ => match cadena.parse::<f64>() {
                        Ok(num) => Datum::Float(num),
                        _ => Datum::NotNumber(cadena)
                    },
                }
            }
        }))
    }

    /// Returns a full column of a generic type. 
    /// The column is in a consumible iterator. Each element has Option<T> type. All the valid rows are included.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method feels repetitive with methods that returns specific type columns because was created after the definition of those.
    /// In the future the specific type columns will dissapear.
    /// 
    /// # Arguments
    ///
    /// * `column` - A string slice that holds the name of the column
    ///
    /// # Examples
    ///
    /// ```
    /// use raven::RawFrame;
    /// use raven::Datum;
    /// use std::ffi::OsString;
    ///
    /// fn get_data() -> raven::RawFrame {
    ///     let path = OsString::from("./datos_test/test.csv");
    ///     let datos = RawFrame::from_os_string(path).unwrap();
    ///     datos
    /// }
    /// 
    /// let datos = get_data();
    /// 
    /// let col: Vec<Option<i32>> = datos.col_type("col_a").unwrap().collect();
    /// ```
    pub fn col_type<T>(&self, column: &str) -> Result<impl Iterator<Item=Option<T>> + '_,Box<dyn Error>>
    where T: std::str::FromStr
    {

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => None,
                Some(cadena) => match cadena.parse::<T>() {
                    Ok(num) => Some(num),
                    _ => None
                } 
            }
        }))

    }

    /// Returns a filtered column of generic type filtering for only the possible to parse data. 
    /// The column is in a consumible iterator. Each element has T type. Only the valid parsed rows are included.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method has a variable number of elements related to the rows in the RawDataframe, use it with caution.
    /// This method feels repetitive with methods that returns specific type columns because was created after the definition of those.
    /// In the future the specific type columns will dissapear.
    /// 
    /// # Arguments
    ///
    /// * `column` - A string slice that holds the name of the column
    ///
    /// # Examples
    ///
    /// ```
    /// use raven::RawFrame;
    /// use raven::Datum;
    /// use std::ffi::OsString;
    ///
    /// fn get_data() -> raven::RawFrame {
    ///     let path = OsString::from("./datos_test/test.csv");
    ///     let datos = RawFrame::from_os_string(path).unwrap();
    ///     datos
    /// }
    /// 
    /// let datos = get_data();
    /// 
    /// let col: Vec<i32> = datos.col_fil("col_a").unwrap().collect();
    /// ```
    pub fn col_fil<T>(&self, column: &str) -> Result<impl Iterator<Item=T> + '_,Box<dyn Error>>
    where T: std::str::FromStr
    {
        
        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().filter_map(move |record| {
            match record.get(position) {
                None => None,
                Some(cadena) => cadena.parse::<T>().ok()
            }
        }))
    }

    /// Returns a full column of a generic type imputing none_val in the impossible to parse data. 
    /// The column is in a consumible iterator. Each element has T type. All the valid rows are included.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method feels repetitive with methods that returns specific type columns because was created after the definition of those.
    /// In the future the specific type columns will dissapear.
    /// 
    /// # Arguments
    ///
    /// * `column` - A string slice that holds the name of the column
    ///
    /// # Examples
    ///
    /// ```
    /// use raven::RawFrame;
    /// use raven::Datum;
    /// use std::ffi::OsString;
    ///
    /// fn get_data() -> raven::RawFrame {
    ///     let path = OsString::from("./datos_test/test.csv");
    ///     let datos = RawFrame::from_os_string(path).unwrap();
    ///     datos
    /// } 
    /// 
    /// let datos = get_data();
    /// 
    /// let col: Vec<i32> = datos.col_imp("col_a",0).unwrap().collect();
    /// ```
    pub fn col_imp<T>(&self, column: &str, none_val:T) -> Result<impl Iterator<Item=T> + '_,Box<dyn Error>>
    where T: std::str::FromStr + Copy + 'static
    {

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => none_val,
                Some(cadena) => match cadena.parse::<T>() {
                    Ok(num) => num,
                    _ => none_val
                }
            }
        })) 

    }

    /// Returns a full column of strs. 
    /// The column is in a consumible iterator. Each element has Option<str> type. All the valid rows are included.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn column_str(&self, column: &str) -> Result<impl Iterator<Item=Option<&str>> + '_,Box<dyn Error>>{

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            record.get(position)
        }))

    }

    /// Returns a full column of ints. 
    /// The column is in a consumible iterator. Each element has Option<i32> type. All the valid rows are included.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn column_int(&self, column: &str) -> Result<impl Iterator<Item=Option<i32>> + '_,Box<dyn Error>>{

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => None,
                Some(cadena) => match cadena.parse::<i32>() {
                    Ok(num) => Some(num),
                    _ => None
                } 
            }
        }))

    }

    /// Returns a full column of ints imputing none_val in the impossible to parse data.
    /// The column is in a consumible iterator. Each element has i32 type. All the valid rows are included.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one. 
    pub fn col_int_imp(&self, column: &str, none_val: i32) -> Result<impl Iterator<Item=i32> + '_,Box<dyn Error>>{

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => none_val,
                Some(cadena) => match cadena.parse::<i32>() {
                    Ok(num) => num,
                    _ => none_val
                }
            }
        })) 

    }

    /// Returns a filtered column of ints filtering for only the possible to parse data.
    /// The column is in a consumible iterator. Each element has i32 type. Only the valid rows are included.
    /// This method has a variable number of elements related to the rows in the RawDataframe, use it with caution.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn col_int_fil(&self, column: &str) -> Result<impl Iterator<Item=i32> + '_,Box<dyn Error>>{
        
        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().filter_map(move |record| {
            match record.get(position) {
                None => None,
                Some(cadena) => cadena.parse::<i32>().ok()
            }
        }))
    }

    /// Returns a full column of floats. 
    /// The column is in a consumible iterator. Each element has Option<f64> type. All the valid rows are included.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn column_float(&self, column: &str) -> Result<impl Iterator<Item=Option<f64>> + '_,Box<dyn Error>>{

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => None,
                Some(cadena) => match cadena.parse::<f64>() {
                    Ok(num) => Some(num),
                    _ => None
                } 
            }
        }))

    }

    /// Returns a full column of floats imputing none_val in the impossible to parse data.
    /// The column is in a consumible iterator. Each element has f64 type. All the valid rows are included.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn col_float_imp(&self, column: &str, none_val: f64) -> Result<impl Iterator<Item=f64> + '_,Box<dyn Error>>{

        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => none_val,
                Some(cadena) => match cadena.parse::<f64>() {
                    Ok(num) => num,
                    _ => none_val
                }
            }
        })) 

    }

    /// Returns a filtered column of floats filtering for only the possible to parse data.
    /// The column is in a consumible iterator. Each element has f64 type. Only the valid parsed rows are included.
    /// This method has a variable number of elements related to the rows in the RawDataframe, use it with caution.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn col_float_fil(&self, column: &str) -> Result<impl Iterator<Item=f64> + '_,Box<dyn Error>>{
        
        let position = match self.col_index(column) {
            Some(n) => n,
            None => return Err(From::from("No existe la columna"))
        };

        Ok(self.records.iter().filter_map(move |record| {
            match record.get(position) {
                None => None,
                Some(cadena) => cadena.parse::<f64>().ok()
            }
        }))
    }

}

pub mod reading {
    //! Auxiliar module for reading CSV files.

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
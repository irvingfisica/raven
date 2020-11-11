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

    /// Returns the position index for column in RawFrame or Error if column does not exists.
    fn col_position(&self, column: &str) -> Result<usize,Box<dyn Error>> {
        match self.col_index(column) {
            Some(n) => Ok(n),
            None => Err(From::from("No existe la columna"))
        }
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
    
        let position = self.col_position(column)?;

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

        let position = self.col_position(column)?;

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
        
        let position = self.col_position(column)?;

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
    /// * `none_val` - value for imputing the impossible to parse values
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
    where T: std::str::FromStr + Clone + 'static
    {

        let position = self.col_position(column)?;

        Ok(self.records.iter().map(move |record| {
            match record.get(position) {
                None => none_val.clone(),
                Some(cadena) => match cadena.parse::<T>() {
                    Ok(num) => num,
                    _ => none_val.clone()
                }
            }
        })) 

    }

    /// Returns a full column of strs. 
    /// The column is in a consumible iterator. Each element has Option<str> type. All the valid rows are included.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn column_str(&self, column: &str) -> Result<impl Iterator<Item=Option<&str>> + '_,Box<dyn Error>>{

        let position = self.col_position(column)?;

        Ok(self.records.iter().map(move |record| {
            record.get(position)
        }))

    }

    /// Returns a full column of ints. 
    /// The column is in a consumible iterator. Each element has Option<i32> type. All the valid rows are included.
    /// This method will dissapear in future versions because a generic one exists. Try to use the generic one.
    pub fn column_int(&self, column: &str) -> Result<impl Iterator<Item=Option<i32>> + '_,Box<dyn Error>>{

        let position = self.col_position(column)?;

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

        let position = self.col_position(column)?;

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
        
        let position = self.col_position(column)?;

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

        let position = self.col_position(column)?;

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

        let position = self.col_position(column)?;

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
        
        let position = self.col_position(column)?;

        Ok(self.records.iter().filter_map(move |record| {
            match record.get(position) {
                None => None,
                Some(cadena) => cadena.parse::<f64>().ok()
            }
        }))
    }

    /// Returns the maximum value of a column. The type is generic for comparable types, in order to compare floats is necessary to define std::cmp::Ord or use ordered-float crate or similar
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
    /// let maximo: i32 = datos.max_num_fil("col_a").unwrap();
    /// ```
    pub fn max_num_fil<T>(&self, column: &str) -> Result<T,Box<dyn Error>>
    where T: std::str::FromStr + std::cmp::Ord
    {

        let iter = self.col_fil(column)?;

        match iter.max() {
            None => Err(From::from("No se encontró el máximo")),
            Some(val) => Ok(val)
        }

    }

    /// Returns the minimum value of a column. The type is generic for comparable types, in order to compare floats is necessary to define std::cmp::Ord or use ordered-float crate or similar
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
    /// let minimo: i32 = datos.min_num_fil("col_a").unwrap();
    /// ```
    pub fn min_num_fil<T>(&self, column: &str) -> Result<T,Box<dyn Error>>
    where T: std::str::FromStr + std::cmp::Ord
    {

        let iter = self.col_fil(column)?;

        match iter.min() {
            None => Err(From::from("No se encontró el mínimo")),
            Some(val) => Ok(val)
        }

    }

    /// Returns the extent of range of a column. A tuple with minimum and maximum. The type is generic for comparable types, in order to compare floats is necessary to define std::cmp::Ord or use ordered-float crate or similar
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
    /// let extent: (i32,i32) = datos.extent_num_fil("col_a").unwrap();
    /// ```
    pub fn extent_num_fil<T>(&self, column: &str) -> Result<(T,T),Box<dyn Error>>
    where T: std::str::FromStr + std::cmp::Ord
    {
        let maximo = self.max_num_fil(column)?;
        let minimo = self.min_num_fil(column)?;

        Ok((minimo,maximo))
    }

    /// Returns a pair of columns of generic type filtering for rows where both values can be parsed. 
    /// The result is in a consumible iterator. Each element is a tuple of T type.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method has a variable number of elements related to the rows in the RawDataframe, use it with caution.
    /// This method is mainly used for compute operations between two columns and to generate a pair of coordinates to plot.
    /// 
    /// # Arguments
    ///
    /// * `xcolumn` - A string slice that holds the name of first the column
    /// * `ycolumn` - A string slice that holds the name of second the column
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
    /// let pairs: Vec<(f64,f64)> = datos.pair_col_fil("col_a","col_b").unwrap().collect();
    /// ```
    pub fn pair_col_fil<T>(&self, xcolumn: &str, ycolumn: &str) -> Result<impl Iterator<Item=(T,T)> + '_,Box<dyn Error>>
    where T: std::str::FromStr
    {

        let xposition = self.col_position(xcolumn)?;

        let yposition = self.col_position(ycolumn)?;

        Ok(self.records.iter().filter_map(move |record| {
            let xval = match record.get(xposition) {
                None => None,
                Some(cadena) => match cadena.parse::<T>() {
                    Ok(num) => Some(num),
                    _ => None
                } 
            };

            let yval = match record.get(yposition) {
                None => None,
                Some(cadena) => match cadena.parse::<T>() {
                    Ok(num) => Some(num),
                    _ => None
                } 
            };
            
            match (xval,yval) {
                (Some(valx),Some(valy)) => Some((valx,valy)),
                _ => None,
            }
        }))

    }

    /// Returns a pair of columns of generic type imputing in the impossible to parse data none_val_x for the first column and none_val_y for the second column. 
    /// The result is in a consumible iterator. Each element is a tuple of T type. All the valid rows are included.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method is mainly used for compute operations between two columns and to generate a pair of coordinates to plot.
    /// 
    /// # Arguments
    ///
    /// * `xcolumn` - A string slice that holds the name of first the column
    /// * `ycolumn` - A string slice that holds the name of second the column
    /// 
    /// * `none_val_x` - value for imputing the impossible to parse values for the first column
    /// * `none_val_y` - value for imputing the impossible to parse values for the second column
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
    pub fn pair_col_imp<T>(&self, xcolumn: &str, ycolumn: &str, none_val_x:T, none_val_y:T) -> Result<impl Iterator<Item=(T,T)> + '_,Box<dyn Error>>
    where T: std::str::FromStr + Clone + 'static
    {

        let xposition = self.col_position(xcolumn)?;

        let yposition = self.col_position(ycolumn)?;

        Ok(self.records.iter().map(move |record| {
            let xval = match record.get(xposition) {
                None => none_val_x.clone(),
                Some(cadena) => match cadena.parse::<T>() {
                    Ok(num) => num,
                    _ => none_val_x.clone()
                } 
            };

            let yval = match record.get(yposition) {
                None => none_val_y.clone(),
                Some(cadena) => match cadena.parse::<T>() {
                    Ok(num) => num,
                    _ => none_val_y.clone()
                } 
            };
            
            (xval,yval)
        }))

    }

    /// Returns a slice of columns of generic type imputing in the impossible to parse data the values in the imp_vals Vec. 
    /// The result is in a consumible iterator. Each element is a vec of T type. All the valid rows are included.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method is mainly used for compute operations between two columns and to generate a pair of coordinates to plot.
    /// 
    /// # Arguments
    ///
    /// * `columns` - A Vec of string slices that holds the names of the columns to get
    /// 
    /// * `imp_vals` - values for imputing the impossible to parse values, it has the same order of columns
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
    /// let col: Vec<i32> = datos.slice_col_imp(vec!["col_a","col_b"],vec![0,0]).unwrap().collect();
    /// ```
    pub fn slice_col_imp<T>(&self, columns: Vec<&str>, imp_vals: Vec<T>) -> Result<impl Iterator<Item=Vec<T>> + '_,Box<dyn Error>> 
    where T: std::str::FromStr + Clone + 'static
    {

        let positions = columns.iter().map(|col| self.col_position(col).unwrap()).collect::<Vec<usize>>();

        Ok(self.records.iter().map(move |record| {
            positions.iter().zip(imp_vals.iter()).map(|tup|{
                match record.get(*tup.0) {
                    None => tup.1.clone(),
                    Some(cadena) => match cadena.parse::<T>() {
                        Ok(num) => num,
                        _ => tup.1.clone()
                    }
                }
            }).collect::<Vec<T>>()
        }))
    }

    /// Returns a slice of columns of generic type filtering for rows where all values can be parsed. 
    /// The result is in a consumible iterator. Each element is a vec of T type.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method has a variable number of elements related to the rows in the RawDataframe, use it with caution.
    /// 
    /// # Arguments
    ///
    /// * `columns` -A Vec of string slices that holds the names of the columns to get
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
    /// let pairs: Vec<(f64,f64)> = datos.slice_col_fil(vec!["col_a","col_b"]).unwrap().collect();
    /// ```
    pub fn slice_col_fil<T>(&self, columns: Vec<&str>) -> Result<impl Iterator<Item=Vec<T>> + '_,Box<dyn Error>> 
    where T: std::str::FromStr + Clone
    {

        let positions = columns.iter().map(|col| self.col_position(col).unwrap()).collect::<Vec<usize>>();

        Ok(self.records.iter().filter_map(move |record| {
            let row = positions.iter().map(|pos|{
                match record.get(*pos) {
                    None => None,
                    Some(cadena) => match cadena.parse::<T>() {
                        Ok(num) => Some(num),
                        _ => None
                    }
                }
            }).collect::<Vec<Option<T>>>();

            match row.iter().all(|ele| ele.is_some()) {
                true => Some(row.iter().map(move |val| val.as_ref().cloned().unwrap()).collect::<Vec<T>>()),
                false=> None
            }
        }))

    }

    /// Returns a pair of columns of generic type sorted by values on first column. Imputing in the impossible to parse data none_val_x for the first column and none_val_y for the second column. 
    /// The result is in a consumible iterator. Each element is a tuple of T type.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method has a variable number of elements related to the rows in the RawDataframe, use it with caution.
    /// This method has a different order related to the rows in the RawDataframe, use it with caution.
    /// This method is mainly used for compute operations between two columns and to generate a pair of coordinates to plot.
    /// 
    /// # Arguments
    ///
    /// * `xcolumn` - A string slice that holds the name of first the column
    /// * `ycolumn` - A string slice that holds the name of second the column
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
    /// let pairs: Vec<(f64,f64)> = datos.pair_col_imp_sorted("col_a","col_b").unwrap().collect();
    /// ```
    pub fn pair_col_fil_sorted<T>(&self, xcolumn: &str, ycolumn: &str) -> Result<impl Iterator<Item=(T,T)> + '_,Box<dyn Error>>
    where T: std::str::FromStr + std::cmp::PartialOrd + 'static
    {

        let mut temp_vec: Vec<(T,T)> = self.pair_col_fil(xcolumn, ycolumn).unwrap().collect();
        temp_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Ok(temp_vec.into_iter())

    }

    /// Returns a pair of columns of generic type sorted by values on first column. Filtering for rows where both values can be parsed. 
    /// The result is in a consumible iterator. Each element is a tuple of T type.
    /// The generic type is specified in the definition of the variable in which the iterator will bind.
    /// This method has a variable number of elements related to the rows in the RawDataframe, use it with caution.
    /// This method has a different order related to the rows in the RawDataframe, use it with caution.
    /// This method is mainly used for compute operations between two columns and to generate a pair of coordinates to plot.
    /// 
    /// # Arguments
    ///
    /// * `xcolumn` - A string slice that holds the name of first the column
    /// * `ycolumn` - A string slice that holds the name of second the column
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
    /// let pairs: Vec<(f64,f64)> = datos.pair_col_fil_sorted("col_a","col_b").unwrap().collect();
    /// ```
    pub fn pair_col_imp_sorted<T>(&self, xcolumn: &str, ycolumn: &str, none_val_x:T, none_val_y:T) -> Result<impl Iterator<Item=(T,T)> + '_,Box<dyn Error>>
    where T: std::str::FromStr + std::cmp::PartialOrd + Copy + 'static
    {

        let mut temp_vec: Vec<(T,T)> = self.pair_col_imp(xcolumn, ycolumn, none_val_x, none_val_y).unwrap().collect();
        temp_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Ok(temp_vec.into_iter())

    }


}

pub mod utils {
    //! Auxiliar module with handy methods.

    pub fn bool_filter<T>(boolean_iter: impl Iterator<Item=bool>, target_iter: impl Iterator<Item=T>) -> impl Iterator<Item=T>{

        boolean_iter.zip(target_iter).filter(|tup| tup.0).map(|truetup| truetup.1)

    } 

}

pub mod writing {
    //! Auxiliar module for writing CSV files.

    use std::ffi::OsString;
    use std::error::Error;

    /// Write a csv file from an iter. It is necessary that the elements of the iter are vecs of a defined type. In order to write an iter obtained from column producer methods of RawFrame it must be casted to a type first
    pub fn to_csv_iter<T>(path: OsString, columns: Vec<&str>, iterador: impl Iterator<Item=Vec<T>>) -> Result<(), Box<dyn Error>>
    where T: std::convert::AsRef<[u8]>
    {

        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record(columns)?;

        for record in iterador {
            wtr.write_record(record)?;
        }

        wtr.flush()?;

        Ok(())

    }
}

pub mod reading {
    //! Auxiliar module for reading CSV files.

    use std::env;
    use std::error::Error;
    use std::ffi::OsString;
    use std::collections::HashMap;

    /// Returns an OsString for terminal argument in position n or an error if it is not possible to read it
    pub fn read_arg(n: usize) -> Result<OsString, Box<dyn Error>> {
        match env::args_os().nth(n) {
            Some(file_path) => Ok(file_path),
            None => Err(From::from("No se pudo leer el argumento"))
        }
    }

    /// Returns a tuple with column names and a Vec of rows in a csv file. Each row is represented as a csv::StringRecord
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

    /// Returns a tuple with column names and a Vec of rows in a csv file. Each row is represented as a Vec<String>
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

    /// Returns a tuple with column names and a Vec of rows in a csv file. Each row are represented as a HashMap
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

    /// Returns a tuple with column names and a Vec of rows in a csv file. Each row are represented as a csv::ByteRecord
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

    /// Returns a tuple with column names and a Vec of rows in a csv file. Each row are represented as a csv::StringRecord
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

    /// Returns a tuple with column names and a Vec of rows in a csv file. Each row are represented as a csv::StringRecord
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
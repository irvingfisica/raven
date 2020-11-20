# RavenCol

Tabular data manipulation in Rust.

[![CratesIo](https://img.shields.io/crates/v/ravencol.svg)](https://crates.io/crates/ravencol) [![Documentacion](https://docs.rs/ravencol/badge.svg)](https://docs.rs/ravencol/)

[Documentation](https://docs.rs/ravencol/)

RavenCol allows consuming tabular data in a simple way and doing operations on it. One of the most used formats in terms of data is tabular data and CSV files, due to this in this first stage all functions are focused on consuming CSV files and obtaining tabular data structures that allow to operate on them.

The main structure is the RawFrame. From the construction of a RawFrame, iterators of its columns or of sets of columns can be generated on which it is possible to perform more complex operations. Most of the methods associated with the RawFrame return iterators which allow to use all the capabilities of Rust in terms of iterators to calculate on the RawFrame and its components.

Many functionality that would be desirable in RawFrames are achievable using Rust's iterator capabilities.

## RawFrame construction

Currently RawFrames can only be built from CSV files. To build a RawFrame the path to the file is required. There are 2 functions to create RawFrames:

- RawFrame from an OsString: `RawFrame::from_os_string(file_path: OsString)`
- RawFrame from the nth argument when the binary is executed: `RawFrame::from_arg(n: usize)`

### Example of loading a CSV file
~~~rust
let path = OsString::from("./datos_test/test.csv");
let datos = RawFrame::from_os_string(path).unwrap();
~~~

Each RawFrame has two elements:
- `columns` where the names of the columns obtained from the first row of the CSV file are stored
- `records` where all records are stored as a vector of rows.

If our data is in several files there is the possibility of concatenating RawFrames, this operation modifies the base RawFrame and consumes the target RawFrame. Up to now the function to concatenate only checks that the number of columns is the same in both DataFrames, It is the user's responsibility to assure that the columns of both RawFrames have semantic sense and are in the same order.

### Example to load all files from a directory

The following example assumes that the path to the directory is provided in the execution position 1 argument and that it is filled with only CSV files with the same kind of columns
~~~rust
let directorio = reading::read_arg(1)?;

let mut paths = fs::read_dir(directorio)?.map(|path| path.unwrap().path().into_os_string());
let mut base = RawFrame::from_os_string(paths.next().unwrap())?;

loop {

    let file = match paths.next() {
        Some(rec) => rec,
        None => break,
    };

    base.concat(RawFrame::from_os_string(file)?)?;

}
~~~

## Column creation

Once a RawFrame exists, its columns can be used. RawFrames are thought to be data structures that are kept in memory and that function as a source of columns with which to operate. The columns can be obtained through accessor functions and generate consumable iterators.

The general philosophy of RavenCol is to have the data in a RawFrame and from there generate consumable iterators to calculate on the data.

When generating a column from a RawFrame we must think of two things:
- The type of data that my column will contain
- What to do with data that cannot be represented in that data type

Column accessor functions allow you to define columns of a specific type of data, however specifying the type of data to use is the responsibility of whoever calls the function as the functions are defined in terms of generic data types.

Once the data type has been decided there are 3 main possibilities to deal with data that is not representable in that data type. What to do with this data will define the type of function to use for the column: Rows in which it is not possible to obtain valid data can be filtered or they can be assigned with a value that we define.

### Create Option columns

You can obtain a column where each of the datum is inside an Option, in such a way that when a datum is not valid its value is None. To create a column of this style, use the `col_type (column)` function where `column` is the name of the column to be obtained.

~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<Option<i32>> = datos.col_type("col_a").unwrap().collect();
~~~

### Create columns of data by filtering invalid data

A column can be obtained where only those data that can be represented in the chosen data type are kept, in such a way that when a dastum is not valid it is not included in the resulting iterator. To create a column of this style, use the function `col_fil (column)` where `column` is the name of the column to obtain. When using this method the obtained iterator does not have the same number of elements as the number of rows in the RawFrame, it must be used with care.

~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<i32> = datos.col_fil("col_a").unwrap().collect();
~~~

### Create data columns by imputing invalid data

A column can be obtained where each of the datum that cannot be represented in the chosen data type is replaced by a value that is provided as a parameter to the method used, in such a way that when a datum is not valid, the value to impute will represent it. To create a column of this style, the function `col_imp (column, none_val)` is used where `column` is the name of the column to be obtained and` none_val` is the value to be substituted or imputed.

~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<i32> = datos.col_imp("col_a",0).unwrap().collect();
~~~

### Create generic columns

You can also create a generic column trying to identify if the data in each row of the column to be generated is an integer, a float, a string or a null value. For this we use the Enum Datum that was created to represent a generic data.
~~~rust
Datum {
    Integer(i32),
    Float(f64),
    NotNumber(&str),
    None
}
~~~

To create a generic column of this type we use the function `column (column)` where the argument `column` is the name of the column to obtain.
~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<Datum> = datos.column("col_a").unwrap().collect();
~~~

### Create column sets

There are times when iterators that contain data sets from multiple columns are needed. For example, to plot points we would need pairs of coordinates. Within RavenCol there are methods to obtain these sets of data. The logic is the same, select the type of data and define what to do with the values ​​that it is not possible to represent in that type. Up to now all values ​​must have the same type, if structures with different types of data are needed it is possible to use the Datum type and then process it.

To create this type of multicolumns, the `slice_col_fil (columns)` and `slice_col_imp (columns, imp_vals)` methods are provided where columns is a vector with the names of the columns and imp_vals is a vector with the values ​​to be imputed for each column.

A special case is the creation of pairs of columns since they are used in the creation of plots, for those cases we have the methods `pair_col_fil (xcolumn, ycolumn)` and `pair_col_imp (xcolumn, ycolumn, none_val_x, none_val_y)` both methods return iterators with tuples of values. For the special case of plots of joined points, i.e. line plotting, methods are provided that order the elements of the iterator considering the first column. These methods are `pair_col_fil_sorted (xcolumn, ycolumn)` and `pair_col_imp_sorted (xcolumn, ycolumn, none_val_x, none_val_y)`. Sorting capabilities are basic, always in terms of the first column and always in ascending order. If more complex orderings are required they can be done with the iterator manipulation capabilities provided by Rust.

### Example of how to plot using [plotters](https://github.com/38/plotters)

~~~rust
use std::error::Error;
use std::process;
use ordered_float::OrderedFloat;
use plotters::prelude::*;

use ravencol::RawFrame;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {

    let path = OsString::from("./datos_test/pizzas.csv");
    let datos = RawFrame::from_os_string(path).unwrap();

    let col_x = "Reservations";
    let col_y = "Pizzas";

    let extent_x: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil(col_x)?;
    let extent_y: (OrderedFloat<f64>,OrderedFloat<f64>) = datos.extent_num_fil(col_y)?;

    let x_range = extent_x.0.into_inner()..extent_x.1.into_inner();
    let y_range = extent_y.0.into_inner()..extent_y.1.into_inner();


    let drawing_area = BitMapBackend::new("./test.png", (1024, 768)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
                    .caption("Pizzas!", ("Arial", 30))
                    .margin(10)
                    .set_label_area_size(LabelAreaPosition::Left, 50)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(x_range,y_range)?;

    chart.configure_mesh()
            .y_desc("Pizzas")
            .x_desc("Reservaciones")
            .axis_desc_style(("sans-serif", 20))
            .draw().unwrap();

    chart.draw_series(
        LineSeries::new(datos.pair_col_fil_sorted(col_x,col_y)? ,&RED),
    )?;

    chart.draw_series(
        AreaSeries::new(datos.pair_col_fil_sorted(col_x,col_y)?,0.0,&BLUE.mix(0.2)),
    )?;

    chart.draw_series(datos.pair_col_fil(col_x, col_y).unwrap().map(|point| Circle::new(point, 3, &RED)))
        .unwrap();

    Ok(())
}
~~~
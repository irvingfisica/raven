# RavenCol

Manipulación de datos tabulares en Rust.

[![CratesIo](https://img.shields.io/crates/v/ravencol.svg)](https://crates.io/crates/ravencol) [![Documentacion](https://docs.rs/ravencol/badge.svg)](https://docs.rs/ravencol/)

[Documentation](https://docs.rs/ravencol/)

RavenCol permite consumir datos tabulares de forma simple y hacer operaciones sobre ellos. Uno de los formatos más usados en cuanto a datos son los datos tabulares y los archivos CSV, debido a esto en esta primera etapa todas las funciones se concentrean en consumir archivos CSV y obtener estructuras de datos tabulares que permitan operar sobre ellos.

La estructura principal es el RawFrame. A partir de la construcción de un RawFrame se pueden generar iteradores de sus columnas o de conjuntos de columnas sobre las cuales es posible realizar operaciones más complejas. La mayoría de los métodos asociados al RawFrame regresan iteradores lo que permite usar todas las capacidades de Rust en términos de iteradores para calcular sobre el RawFrame y sus componentes.

Muchas funcionalidades que serían deseables en los RawFrames son alcanzables usando las capacidades de Rust en cuanto a iteradores.

## Construcción del RawFrame

Actualmente se pueden construir RawFrames solamente desde archivos CSVs. Para construir un RawFrame es necesaria la ruta al archivo. Existen 2 funciones para crear RawFrames:

- RawFrame desde un OsString: `RawFrame::from_os_string(file_path: OsString)`
- RawFrame desde el argumento n de la terminal: `RawFrame::from_arg(n: usize)`

### Ejemplo de carga de un archivo CSV
~~~rust
let path = OsString::from("./datos_test/test.csv");
let datos = RawFrame::from_os_string(path).unwrap();
~~~

Cada RawFrame tiene dos elementos:
- `columns` en donde se guarda el nombre de las columnas obtenido de la primera fila del archivo CSV 
- `records` en donde se guardan todos los registros como un vector de filas.

Si nuestros datos se encuentran en varios archivos existe la posibilidad de concatenar RawFrames, esta operación modifica el RawFrame base y consume el RawFrame objetivo. Por ahora la función para concatenar solamente checa que el número de columnas sea el mismo en ambos DataFrames, por ahora es responsabilidad del usuario que las columnas de ambos RawFrames tengan sentido semantico y estén en el mismo orden. 

### Ejemplo para cargar todos los archivos desde un directorio

En el siguiente ejemplo se asume que la ruta al directorio se proporciona en el argumento posición 1 de la ejecución y que está lleno de únicamente archivos CSV con el mismo tipo de columnas
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

## Creación de columnas

Una vez que existe un RawFrame se pueden usar sus columnas. Los RawFrames están pensados para ser estructuras de datos que se mantienen en memoria y que funcionan como fuente de columnas con las cuales se puede operar. Las columnas se pueden obtener a traves de funciones accesoras y generan iteradores consumibles.

La filosofía general de RavenCol es tener los datos en un RawFrame y a partir de ahí generar iteradores consumibles para calcular sobre los datos.

Al generar una columna de un RawFrame debemos pensar en dos cosas:
- El tipo de datos que contendrá mi columna
- Que hacer con los datos que no se pueden representar en ese tipo de datos

Las funciones accesoras de columnas permiten definir columnas de un tipo específico de datos, sin embargo especificar el tipo de datos a usar es responsabilidad de quien llama a la función pues las funciones están definidas en términos de tipos de datos genéricos.

Una vez decidido el tipo de datos hay 3 posibilidades principales para tratar con los datos que no son representables en ese tipo de datos. Que hacer con estos datos definirá el tipo de función a usar para la columna: Se pueden filtrar las filas en las cuales no es posible obtener un dato válido o se pueden imputar con algún valor que nosotros definamos.

### Crear columnas de Options

Se puede obtener una columna en donde cada uno de los datos está dentro de un Option, de tal manera que cuando un dato no es válido su valor es None. Para crear una columna de ese estilo se utiliza la función `col_type(column)` donde `column` es el nombre de la columna a obtener.

~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<Option<i32>> = datos.col_type("col_a").unwrap().collect();
~~~

### Crear columnas de datos filtrando datos no válidos

Se puede obtener una columna en donde solamente se mantengan aquellos datos que es posible representar en el tipo de datos elegido, de tal manera que cuando un dato no es válido no se incluye en el iterador resultante. Para crear una columna de ese estilo se utiliza la función `col_fil(column)` donde `column` es el nombre de la columna a obtener. Al utilizar este método el iterador obtenido no tiene el mismo número de elementos que el número de filas en el RawFrame, se debe usar con cuidado.

~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<i32> = datos.col_fil("col_a").unwrap().collect();
~~~

### Crear columnas de datos imputando datos no válidos

Se puede obtener una columna en donde cada uno de los datos que no se pueden representar en el tipo de datos elegidos es sustituido por un valor que se proporciona como parámetro al método utilizado, de tal manera que cuando un dato no es válido el valor a imputar lo representará. Para crear una columna de ese estilo se utiliza la función `col_imp(column, none_val)` donde `column` es el nombre de la columna a obtener y `none_val` es el valor a sustituir o imputar.

~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<i32> = datos.col_imp("col_a",0).unwrap().collect();
~~~

### Crear columnas genéricas

También se puede crear una columna genérica tratando de identificar si el dato en cada fila de la columna a generar es un entero, un flotante, una cadena o un valor nulo. Para esto usamos el Enum Datum que fue creado para representar un dato genérico.
~~~rust
Datum {
    Integer(i32),
    Float(f64),
    NotNumber(&str),
    None
}
~~~

Para crear una columna genérica de este tipo usamos la funcion `column(column)` donde el argumento `column` es el nombre de la columna a obtener.
~~~rust
fn get_data() -> ravencol::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}

let datos = get_data();

let columna: Vec<Datum> = datos.column("col_a").unwrap().collect();
~~~

### Creación de conjuntos de columnas

Hay ocasiones en donde se necesitan iteradores que contengan conjuntos de datos provenientes de varias columnas. Por ejemplo para graficar puntos necesitaríamos parejas de coordenadas. Dentro de RavenCol existen métodos para obtener estos conjuntos de valores. La lógica es la misma, seleccionar el tipo de datos y definir que hacer con los valores que no es posible representar en ese tipo. Por ahora todos los valores deben tener el mismo tipo, si se busca crear estructuras con diferentes tipos de datos es posible usar el tipo Datum y luego procesarlo.

Para realizar este tipo de multicolumnas se proporcionan los métodos `slice_col_fil(columns)` y `slice_col_imp(columns,imp_vals)` donde columns es un vector con los nombres de las columnas y imp_vals es un vector con los valores a imputar para cada columna.

Un caso especial es la creación de pares de columnas ya que se utilizan en la creación de gráficas, para esos casos se cuenta con los métodos `pair_col_fil(xcolumn, ycolumn)` y `pair_col_imp(xcolumn, ycolumn, none_val_x, none_val_y)` ambos métodos regresan iteradores con tuplas de valores. Para el caso en especial de gráficas de puntos unidos y conformación de líneas se proporcionan métodos que ordenan los elementos del iterador considerando la primera columna. Estos métodos son `pair_col_fil_sorted(xcolumn, ycolumn)` y `pair_col_imp_sorted(xcolumn, ycolumn, none_val_x, none_val_y)`. Las capacidades de ordenamiento son básicas, siempre en términos de la primera columna y siempre en orden ascendente. Si se requieren ordenamientos más complejos se pueden realizar con las capacidades de manipulación de iteradores que proporciona Rust.

### Ejemplo de como graficar usando [plotters](https://github.com/38/plotters)

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
use raven::RawFrame;
use raven::Datum;
use std::ffi::OsString;

#[test]
fn columns_test(){

    let datos = get_data();

    assert_eq!(datos.columns.get(1),Some("col_b"));
}

#[test]
fn row_test(){

    let datos = get_data();

    assert_eq!(datos.records[1].get(0),Some("13"));
    assert_eq!(datos.records[1].get(1),Some("25a"));
}

#[test]
fn index_test(){
    let datos = get_data();
    assert_eq!(datos.col_index("col_b"),Some(1));
}

#[test]
fn full_col_test(){
    let datos = get_data();

    let mut iter = datos.column("col_b").unwrap();

    assert_eq!(iter.next().unwrap(), Datum::Integer(18));
    assert_eq!(iter.next().unwrap(), Datum::NotNumber("25a"));
    assert_eq!(iter.next().unwrap(), Datum::Float(23.0));
    assert_eq!(iter.next().unwrap(), Datum::None);
    assert_eq!(iter.next().unwrap(), Datum::Integer(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn type_col_test(){
    let datos = get_data();

    let col_typ_test: Vec<Option<i32>> = datos.col_type("col_a").unwrap().collect();
    let mut iter = col_typ_test.iter();

    assert_eq!(iter.next(), Some(&None));
    assert_eq!(iter.next().unwrap().unwrap(), 13);
    assert_eq!(iter.next().unwrap().unwrap(), 13);
    assert_eq!(iter.next().unwrap().unwrap(), 12);
    assert_eq!(iter.next().unwrap().unwrap(), 25);
    assert_eq!(iter.next(), None);
}

#[test]
fn type_col_test_b(){
    let datos = get_data();

    let col_typ_test: Vec<Option<i32>> = datos.col_type("col_b").unwrap().collect();
    let mut iter = col_typ_test.iter();

    assert_eq!(iter.next().unwrap().unwrap(), 18);
    assert_eq!(iter.next(), Some(&None));
    assert_eq!(iter.next(), Some(&None));
    assert_eq!(iter.next(), Some(&None));
    assert_eq!(iter.next().unwrap().unwrap(), 3);
    assert_eq!(iter.next(), None);
}

#[test]
fn imp_col_test(){
    let datos = get_data();

    let col_imp_test: Vec<i32> = datos.col_imp("col_a",0).unwrap().collect();
    let mut iter = col_imp_test.iter();

    assert_eq!(*iter.next().unwrap(), 0);
    assert_eq!(*iter.next().unwrap(), 13);
    assert_eq!(*iter.next().unwrap(), 13);
    assert_eq!(*iter.next().unwrap(), 12);
    assert_eq!(*iter.next().unwrap(), 25);
    assert_eq!(iter.next(), None);
}

#[test]
fn imp_col_test_b(){
    let datos = get_data();

    let col_imp_test: Vec<i32> = datos.col_imp("col_b",0).unwrap().collect();
    let mut iter = col_imp_test.iter();

    assert_eq!(*iter.next().unwrap(), 18);
    assert_eq!(*iter.next().unwrap(), 0);
    assert_eq!(*iter.next().unwrap(), 0);
    assert_eq!(*iter.next().unwrap(), 0);
    assert_eq!(*iter.next().unwrap(), 3);
    assert_eq!(iter.next(), None);
}

#[test]
fn fil_col_test(){
    let datos = get_data();

    let col_fil_test: Vec<i32> = datos.col_fil("col_a").unwrap().collect();
    let mut iter = col_fil_test.iter();

    assert_eq!(*iter.next().unwrap(), 13);
    assert_eq!(*iter.next().unwrap(), 13);
    assert_eq!(*iter.next().unwrap(), 12);
    assert_eq!(*iter.next().unwrap(), 25);
    assert_eq!(iter.next(), None);
}

#[test]
fn fil_col_test_b(){
    let datos = get_data();

    let col_fil_test: Vec<i32> = datos.col_fil("col_b").unwrap().collect();
    let mut iter = col_fil_test.iter();

    assert_eq!(*iter.next().unwrap(), 18);
    assert_eq!(*iter.next().unwrap(), 3);
    assert_eq!(iter.next(), None);
}

fn get_data() -> raven::RawFrame {
    let path = OsString::from("./datos_test/test.csv");
    let datos = RawFrame::from_os_string(path).unwrap();
    datos
}
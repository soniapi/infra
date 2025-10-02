use infra::{establish_connection, create_object};
use calamine::{open_workbook, Xlsx, Reader, DataType};
use std::any::type_name;
use std::io;

fn convert(data: &DataType) -> Option<f32> {
    match data {
        DataType::Float(f) => Some(*f as f32),
        _ => None,
    }
}

/*fn print_type<T>(_: &T) {
    println!("Type is: {}", type_name::<T>());
}*/

fn main() {
    let connection = &mut establish_connection();
    let mut excel: Xlsx<_> = open_workbook("/Users/soniapignorel/Data/Object.xlsx").unwrap();
    println!("Please enter the name of the Excel file sheet which cells you'd like to deserialize and add into to a PostgreSQL database table:"); 
    let mut sheet_name = String::new();
    io::stdin()
        .read_line(&mut sheet_name) 
        .expect("Read line failed");
    let trimmed_sheet_name = sheet_name.trim(); 

    if let Some(Ok(range)) = excel.worksheet_range(&trimmed_sheet_name) {
        for row in range.rows().skip(1).take(10) {
            println!("row[0]={:?}, row[1]={:?}, row[2]={:?}, row[3]={:?}", row[0].as_datetime(), row[1].as_string(), &convert(&row[2]).as_ref().unwrap(), &convert(&row[3]).as_ref().unwrap());
            let object = create_object(connection, &row[0].as_datetime().as_ref().unwrap(), &row[1].as_string().as_ref().unwrap(), &convert(&row[2]).as_ref().unwrap(), &convert(&row[3]).as_ref().unwrap());
        }
    }
    else {
        println!("Can't find the Excel sheet name you provided.");
    }
}


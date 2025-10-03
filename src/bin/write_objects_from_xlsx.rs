use calamine::{DataType, Xlsx, open_workbook, Reader};
use std::any::type_name;
use std::io::{self, Write};
use infra::{establish_connection, create_object};

fn convert(data: &DataType) -> Option<f32> {
    match data {
        DataType::Float(f) => Some(*f as f32),
        _ => None,
    }
}

fn print_type<T>(_: &T) {
    println!("Type is: {}", type_name::<T>());
}

fn inputs() -> (String, String) {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    stdout.flush().expect("Failed to flush stdout");
    println!("Enter your file path:"); 
    let mut f = String::new();
    stdin.read_line(&mut f).expect("Read line failed");
    let trimmed_f = f.trim().to_string();
    println!("Your input:{:?}", trimmed_f);

    stdout.flush().expect("Failed to flush stdout");
    println!("Enter your file tab:");
    let mut t = String::new();
    stdin.read_line(&mut t).expect("Read line failed");
    let trimmed_t = t.trim().to_string();
    println!("Your input:{:?}", trimmed_t);

    (trimmed_f, trimmed_t) 
}

fn main() {
    let connection = &mut establish_connection();
    let (f, t) = inputs();
    let mut excel: Xlsx<_> = open_workbook(f).unwrap();

    if let Some(Ok(range)) = excel.worksheet_range(&t) {
        for row in range.rows().skip(1).take(10) {
            println!("Check you PostgreSQL table for below object insertion");
            println!("row[0]={:?}, row[1]={:?}, row[2]={:?}, row[3]={:?}", row[0].as_datetime(), row[1].as_string(), &convert(&row[2]).as_ref().unwrap(), &convert(&row[3]).as_ref().unwrap());
            create_object(connection, &row[0].as_datetime().as_ref().unwrap(), &row[1].as_string().as_ref().unwrap(), &convert(&row[2]).as_ref().unwrap(), &convert(&row[3]).as_ref().unwrap());
        }
    }
    else {
        println!("Can't find the file.");
    }
}


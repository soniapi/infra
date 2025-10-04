use calamine::DataType;
use std::any::type_name;
use std::io::{self, Write};

pub fn convert(data: &DataType) -> Option<f32> {
    match data {
        DataType::Float(f) => Some(*f as f32),
        _ => None,
    }
}

pub fn print_type<T>(_: &T) {
    println!("Type is: {}", type_name::<T>());
}

pub fn inputs() -> (String, String) {
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


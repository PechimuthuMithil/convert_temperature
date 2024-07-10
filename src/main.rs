use std::io;

fn main() {
    let mut scale = String::new();
    let mut temp = String::new();

    println!("Enter the temperature scale (F or C):");
    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read the temperature scale.");
    
    println!("Enter the temperature value:");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the temperature value");

    let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature value entered.");
                return;
            }
    };

    let scale = scale.trim().to_uppercase();
    let result = match scale.as_str() {
        "F" => {
            ((temp - 32.0) * 5.0 / 9.0, "C")
        },
        "C" => {
            ((temp * 9.0 / 5.0) + 32.0, "F")
        },
        _ => {
            println!("Invalid temperature scale entered.");
            return;
        }
    };

    println!("Converted temperature: {}°{}", result.0, result.1);
}

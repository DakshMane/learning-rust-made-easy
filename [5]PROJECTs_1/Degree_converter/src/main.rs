use std::io;

fn main() {
    println!("FOR DEGREE TO FAHRENHEIT SELECT 1");
    println!("FOR FAHRENHEIT TO DEGREE SELECT 2");

    loop {
        let mut choice = String::new();

        // Read input
        if io::stdin().read_line(&mut choice).is_err() {
            println!("Failed to read input. Exiting...");
            break;
        }

        // Parse input
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Exiting...");
                break;
            }
        };

        if choice == 1 {
            println!("Enter Celsius:");
            let mut celsius = String::new();
            io::stdin().read_line(&mut celsius).unwrap();
            let celsius: f32 = celsius.trim().parse().unwrap();
            println!("Fahrenheit = {}", cel_to_fah(celsius));
        } else if choice == 2 {
            println!("Enter Fahrenheit:");
            let mut fahrenheit = String::new();
            io::stdin().read_line(&mut fahrenheit).unwrap();
            let fahrenheit: f32 = fahrenheit.trim().parse().unwrap();
            println!("Celsius = {}", fah_to_cel(fahrenheit));
        } else {
            println!("Invalid choice. Exiting...");
            break;
        }
    }
}

fn cel_to_fah(x: f32) -> f32 {
    (x * 9.0 / 5.0) + 32.0
}

fn fah_to_cel(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

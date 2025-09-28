use std::io;

fn main() {
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Error reading ..");

    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", &string[0..i]); // print first word slice
            return; // exit after printing
        }
    }

    // if no space found, print the whole trimmed string
    println!("{}", string.trim());
}

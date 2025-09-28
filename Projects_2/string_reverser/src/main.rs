use std::io;

fn main() {
    let mut s = String::new() ; 

    io::stdin().read_line(&mut s ).expect("Error reading the line ") ; 
    
    let reversed : String = s.trim().chars().rev().collect() ; 
    println!("{}", reversed) ; 
}


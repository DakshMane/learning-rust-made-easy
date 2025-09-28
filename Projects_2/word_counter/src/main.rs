use std::io;

fn main() {
    println!("Give your sentence ");

    let mut string = String::new() ; 

    io::stdin().read_line(&mut string).expect("Error reading the sentence .. ") ; 

// this counts chars 
    // let size = string.trim().len() ; 
    //correction we want word count this counts words .. 
    let size = string.trim().split_whitespace().count() ; 

    println!("The size of the sentence is {}" , size) ; 
}

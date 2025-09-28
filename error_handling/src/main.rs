use std::{fs::File, io::ErrorKind};

fn main() {
    /*
    Sometimes bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the panic! macro. There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by 
    explicitly calling the panic! macro. */

    // panic!("crash and burn ") ; 
    let greeting_file_Result = File::open("hello.txt") ; 

    // let greeting_file = match  greeting_file_Result {
    //     Ok(file) => file , 
    //     Err(error) => panic!("problem opening the file : {:?}",error) 
    // }  ; 

    // HANDLING MORE BETTER WAY , IF FILE NOT EXISTING ,MAKE ONE

    let greeting_file = match  greeting_file_Result {
        Ok(file) => file , 
        Err(error) => match  error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc , 
                Err(e) => panic!("problem creating the file : {:?}",e)
            } , 
            _ => {
                panic!("problem opening the file : {:?}",error ) ; 
            }
            
        }
    } ; 
}

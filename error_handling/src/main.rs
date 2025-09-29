use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    /*
    Sometimes bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the panic! macro. There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by
    explicitly calling the panic! macro. */

    // panic!("crash and burn ") ;
    let greeting_file_Result = File::open("hello.txt");

    // let greeting_file = match  greeting_file_Result {
    //     Ok(file) => file ,
    //     Err(error) => panic!("problem opening the file : {:?}",error)
    // }  ;

    // HANDLING MORE BETTER WAY , IF FILE NOT EXISTING ,MAKE ONE

    let greeting_file = match greeting_file_Result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file : {:?}", e),
            },
            _ => {
                panic!("problem opening the file : {:?}", error);
            }
        },
    };

    // let greeting_file = File::open("hello1.txt")?; gives error only valid for func hreturn type .. aving result

    // shortcut ..
    // let greeting_file2 = File::open("hello.txt").expect("Not in the project ... ") ;

    /*When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code */
}

// this func can be written in shorter way ..
// fn read_username_from_file() -> Result<String , io::Error> {
//     let username_file_result = File::open("hello.txt") ;

//     let mut username_file = match  username_file_result {
//         Ok(file) => file ,
//         Err(e) => return Err(e) ,
//     };

//     let mut username = String::new() ;

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username) ,
//         Err(e) => Err(e) ,
//     }
//     }

fn read_username_from_file() -> Result<String, io::Error> {
    /*In the context of Listing 9-7, the ? at the end of the File::open call will return the value inside an Ok to the variable username_file. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.

    The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the ? */
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)

    /*The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function */
}

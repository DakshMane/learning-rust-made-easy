// fn main() {
//     println!("Hello, world!");



// /*
// Ownership Rules

// Keep these rules in mind as we work through the examples that illustrate them:
//     Each value in Rust has an owner.
//     There can only be one owner at a time.
//     When the owner goes out of scope, the value will be dropped.
// //  */

// //  {                      // s is not valid here, since it's not yet declared
// //         let mut  s = "hello";   // s is valid from this point forward
// //         s[0] = "j" ; 


// //         // do stuff with s
// //     }   
// //     s.len() ; gives error coz s out of scope ..                   // this scope is now over, and s is no longer valid
// }

// fn main() {
//      let s = String::from("hello"); 
//     takes_ownership(s);

//     let x = 5 ; 
//     makes_copy(x);


//     // doesnt gives error coz integer has Copy trait .. 
//     // only types stored on stack have Copy trait by default .. 
//     let y = x * 5 ; 
    
//     // s.len() ;  gives error coz value of s is now borrowed  ...
// }

// fn takes_ownership(some_string : String ) {
//     println!("{}" , some_string) ; 
// }

// fn makes_copy(some_integer : i32) {
//     println!("{}", some_integer);
// } 

fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
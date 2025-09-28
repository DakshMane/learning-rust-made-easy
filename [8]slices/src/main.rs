// fn main() {

// //     let mut  s = String::from("hello world");
// // let word = first_word(&s); // word will get the value 5

// //     s.clear(); // this empties the String, making it equal to ""

//     // word still has the value 5 here, but s no longer has any content that we
//     // could meaningfully use with the value 5, so word is now totally invalid!l


//     let s = String::from("Hello World") ; 

//     let hello = &s[0..5] ; 
//     //Rather than a reference to the entire String, hello is a reference to a portion of the String, specified in the extra [0..5] bit
//     let world = &s[6..11]  ; 
// }
/*
This program compiles without any errors and would also do so if we used word after calling s.clear(). Because word isn’t connected to the state of s at all, word still contains the value 5. We could use that value 5 with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in word.

Having to worry about the index in word getting out of sync with the data in s is tedious and error prone! Managing these indices is even more brittle if we write a second_word function. Its signature would have to look like this:

fn second_word(s: &String) -> (usize, usize) {

Now we’re tracking a starting and an ending index, and we have even more values that were calculated from data in a particular state but aren’t tied to that state at all. We have three unrelated variables floating around that need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices. */

// fn first_word(s : &String) -> usize {
//     let bytes= s.as_bytes() ; 
// // enumerate generates index and element in tuple
//     for (i , &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//         return i ; 
//        }
// }

//     s.len()  

// }

fn main() {
    let mut s = String::from("Hello World") ; 

    let word = first_word(&s) ; 

    // s.clear(); // mutable borrow occurs here .. , then in print again immutable borrow 

    println!("the first word is : {}" , word) ; 

    /*
     borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. The println! after the call to clear uses the reference in word,
     so the immutable reference must still be active at that point.  */
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes() ; 

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i] ; 
        }
     }

     &s[..]
}

/*
Summary

The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control. */
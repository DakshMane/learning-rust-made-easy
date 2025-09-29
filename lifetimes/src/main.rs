use std::fmt::Display;

fn main() {
    /*Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a type has the behavior we want,
    lifetimes ensure that references are valid as long as we need them to be. */

    /*
    The main aim of lifetimes is to prevent dangling references,
    which cause a program to reference data other than the data it’s intended to reference. */
    // let r;

    //     {
    //         /*
    //         `x` does not live long enough
    // borrowed value does not live long enough */
    //         let x = 5;
    //         r = &x;
    //     }

    // println!("r: {r}");
    // 'b denotes lifetime of x and 'a denotes of r ..

    let x = 5; // ----------+-- 'b
    //           |
    let r = &x; // --+-- 'a  |
    //   |       |
    println!("r: {r}"); //   |       |
    // --+       |
    // ----------+

    let string1 = String::from("long string is long");
    /*
    string1 is valid until the end of the outer scope,
     string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. Run this code and you’ll see that the borrow checker approves; it will compile and print
    The longest string is long string is long */
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    /*
    This gives error coz the string2 doesnt last till the outer bound ..

     let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");


     */

    // Static Lifetime ..
    // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime";

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {result}");
}

// ? Using lifetime Annotations in Func Signatures

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }

    /* ! The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a */
}

// using type params , trait bounds and liftimes together ..

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

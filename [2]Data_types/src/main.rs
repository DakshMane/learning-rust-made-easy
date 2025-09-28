fn main() {
    // println!("Hello, world!");
    // let guess  = "42".parse().expect("Not number ") ;  gives error coz type must be known 
//     let mut  i : u8  = 1 ; 
//    for j   in 1..=300 {
//         i += 1 ; 
//    }
// // INTEGER OVERFLOW RESULTING IN PANICKING ... 
//    print!("{}",i)


    // let y = 3.0 ; 
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    // let tup = (500 , 6.4 , 1) ; 
    //     let first_element = tup.0 ; // accessing the first element
    // let (x , y , z ) = tup ; // destructuring the tup 


    // println!(" value of y is {}" , y )

    let a = [1,2,3,4] ; 
    //accessing the element  
    let first = a[0] ; 
    
}
//Keep in mind that Rust is a statically typed language,
//  which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as when we converted a String to a numeric type using parse 
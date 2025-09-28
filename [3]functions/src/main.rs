fn main() {
    // println!("Hello, world!");

    // let y = another_function(6) ; 
    // println!("The value of y is: {}", y);

    // let x = (let y = 6 );   is wrong coz let y doesnt returns any value 

    let y = {
        let x = 3 ; 
        x + 1 
    } ; // x + 1 doesnt have ; coz its a return statement ...
    
    println!("The value of y is: {}", y);
}

fn another_function ( x : i32) -> i32 {
    x*5 
}
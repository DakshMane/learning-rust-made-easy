fn main() {
    // let v: Vec<i32>  =  Vec::new() ; // type annotation rqrd 

    // let v = vec![1, 2, 3];

    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // reading elements of vectors .. 

    let mut v = vec![1, 2, 3 ,4 ,5 ] ; 

    let third : &i32 = &v[2] ; 
    println!("The third element is {third}") ; 

    let third : Option<&i32> = v.get(2) ; 

    match third {
        Some(third) => println!("The third element is {third}") , 
        None => println!("There is no third element")
    }

    //   let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];
    // this gives error coz immutable ref was taken and then mutable borrow tried .. 

    // v.push(6);

    // println!("The first element is: {first}");
    
    // ITERATING 

    for i in &mut v {

        /*To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i , 
         before we can use the += operator.  */
        *i += 50 ; 
    }

    // USING ENUM TO STORE MULTIPLE TYPES
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];  

    // how did u knew i was going to type this ?
    // 

}

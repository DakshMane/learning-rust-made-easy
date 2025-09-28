fn main() {
    let a = [10,20,30,40] ; 

    // let mut index = 0 ; 
    // while index <  5 {
    //     println!("the value is : {}" , a[index]) ; 

    //     index += 1 ; 
    // }

    for element in a {
        println!("the value is : {}" , element) ; 
    }

    for index in 0..5 {
        println!("the value is : {}" , a[index]) ; 
    }
}

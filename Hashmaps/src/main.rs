use std::collections::HashMap;

fn main() {


    // Creating new HashMap 

    let mut scores = HashMap::new() ; 
    // scores.insert(String::from("Blue"), 10) ; 
    // scores.insert(String::from("Yellow"), 50) ; 

    // Accessing values in HashMap

    let team_name = String::from("Blue") ; 
    let score = scores.get(&team_name).copied().unwrap_or(0) ; 

    println!("{:?}", score) ;


    //Iterating values in HashMap

    // for (key , value) in &scores {
    //     println!("{} : {}", key , value) ;
    // }

    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");


    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values,
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
         // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // only blue -> 25 will be registered as one key can have only one value .. 

     scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
println!("{scores:?}");


    // if key has existing value then it persists but if not then this new value will be its value .. 
 scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    }
   


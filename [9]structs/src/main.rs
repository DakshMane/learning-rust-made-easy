// fn main() {
//     // instantiating struct 
//     let mut  user1 = User {
//         active : true , 
//         username : String::from("daksh") , 
//         email : String::from("daksh@gmail.com") , 
//         sign_in_count : 1 , 
//     } ; 
//         // only possible if mut variable ... 
//         user1.email = String::from("daksh2005@gmail.com"); 

//         // creating instances from other instances

//         let user2 = User {
//             active : user1.active , 
//             username : user1.username.clone() , // coz String dont have Copy trait thus it gives error in user3 
//             email : String::from("user1@example") , 
//             sign_in_count : user1.sign_in_count , 
//         } ; 

//         let user3 = User {
        
//             email : String::from("user3@gmail.com"), 
//             ..user1  
            
//         } ; 
// }

// struct User {
//     active : bool , 
//     username : String , 
//     email : String , 
//     sign_in_count : u64 , 
// }

// fn build_user(email : String , username : String) -> User{
//     // using Init shorthand as struct fields and func params name are same .. 

//     User { active: true, username, email, sign_in_count: 1 }
// }


// USING TUPLE STRUCT 

// struct  Color (i32 , i32 , i32) ; 
// struct Point(i32 , i32 , i32) ; 
// fn main() {
//     let black = Color(0,0,0)  ;
//     let origin = Point(0,0,0) ; 
// }

//Unit-like Structs 
/*
 define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section.
  Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
 */

 struct AlwaysEqual ; 

 /*
 In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s also possible for structs to store references to data owned by something else,
but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, 
like the following; this won’t work: */


// Adding functions to work with structs 


// #[derive(Debug)]
// // lets printing in the terminal .. 
// struct  Rectangle {
//     width : u32 , 
//     height : u32 , 
// }

// fn main() {

//     let rec1 = Rectangle {
//         width : 30 , height : 50 
//     } ;
//     println!("rect1 is {rec1:?}");

//     println!("The area of rectange is {}" , area(&rec1)) ; 

//     dbg!(&rec1); 

// }

// fn area (rectangle : &Rectangle ) -> u32 {

//     /*Our area function is now defined with one parameter, 
//     which we’ve named rectangle, whose type is an immutable borrow
//      of a struct Rectangle instance */

//      /*This way, main retains its ownership and can continue using rect1,
//       which is the reason we use the & in the function signature and
//        where we call the function. */
//     rectangle.width * rectangle.height
// }

// Implementing methods for structs ... 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {

        /*
        We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, 
        and we just want to read the data in the struct, 
        not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does,
         we’d use &mut self as the first parameter. 
         Having a method that takes ownership of the instance by using just self as the first parameter is rare;
          this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the
           original instance after the transformation. */
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0 
    }

    fn can_hold(&self , other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }

    // tried by own .. 
    fn can_hold_with_option(&self , other : &Rectangle) -> Option<bool> {
       Some(self.width > other.width && self.height > other.height) 
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() 
   { println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );}

    let rec2 = Rectangle {
        width : 20 , 
        height : 40 , 
    } ; 

     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rec2));

    match rect1.can_hold_with_option(&rec2) {
        Some(true) => println!("rec1 can hold rec2" ) , 
        Some(false) => println!("rec1 cant hold rec2") , 
        None => println!("no result" ) , 
    }
   
}
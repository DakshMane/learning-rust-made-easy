// enum IpAddrKind {
//     V4 , V6 
// }

// enum IpAddrKindDetailed {
//     V4(u8,u8,u8,u8) , 
//     V6(String) , 
// } 

// fn route (ip_kind : IpAddrKind) {}

// struct IpAddr {
//     kind : IpAddrKind , 
//     address : String , 
// }

// enum Message {
//     Quit , 
//     Move {x : i32 , y : i32} , 
//     Write (String) , 
//     ChangeColor(i32 , i32 , i32) , 
// }
// //Message enum is equivalent to creating 4 unit structs like below :- 
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// fn main() {
//     // initiating 


//     let home2 = IpAddrKindDetailed::V4(127, 0, 0, 1) ; 
//     let home = IpAddr {
//         kind : IpAddrKind::V4 , 
//         address : String::from("127.0.0.1") 
//     } ; 
//     let four = IpAddrKind::V4 ; 
//     let six = IpAddrKind::V6 ; 

// }



enum Coin {
    Penny , Nickel , Dime , Quarter(UsState), 
}

#[derive(Debug)]
enum UsState {
    Alabama , Alaska 
}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None , 
        Some(i) => Some(i + 1 ) , 
    }
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1 , 
        Coin::Nickel => 5 , 
        Coin::Dime => 10 , 
        Coin::Quarter(state) => {
            println!("State quarter from {state:?} !") ; 
            25 
        } , 
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hand(){}

impl  UsState {
    fn existed_in(&self , year : u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819 , 
            UsState::Alaska => year >= 1959 , 
        }
    }
}


//, but it has pushed the work into the body of the if let statement, and if the work to be done is more complicated, 
// it might be hard to follow exactly how the top-level branches relate. 
// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }

fn describe_state_quarter(coin :Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None ; 
    } ; 
    if state.existed_in(1990) {
        Some(format!("{state:?} is pretty old, for America!"))

    }
    else {
        Some(format!("{state:?} is relatively new.")) 
    }
}


fn main() {

     let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9 ; 
    match dice_roll {
        3 => add_fancy_hat() , 
        7 => remove_fancy_hand() , 
        _ =>  () , 
    }
    // let some_number = Some(5) ; 
    // let some_char = Some("e") ; 

    // let absent_num: Option<i32> = None ; 

    //  let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // gives error coz option(i8) is added with i8 ..
    // let sum = x + y;


  // Learning if let block control flow   
    let config_max = Some(3u8) ; 
    // match config_max {
    //     Some(max) => println!("The max is configured to be {max}") , 
    //     _ => () , 
    // }
    // we want to print only if config_max var has value,
    //but we have to add _ => () just coz of expression which is annoying instead use:


    if let Some(max) = config_max {
        println!("The max is configured to be {max}") ; 
    }

    // one more example :
    let coin = Coin::Penny ; 
    let mut count = 0 ; 
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?} !") ; 
        
    }
    else {
        count += 1 ; 
    }
    }
    //
    



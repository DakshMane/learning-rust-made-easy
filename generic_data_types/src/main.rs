// fn main() {
//     /*
//     Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.
//      Before diving into generics syntax, let’s first look at how to remove duplication in a way that doesn’t involve generic types by extracting a function that replaces specific values with a placeholder that represents multiple values. */
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {largest}");

//     // for implementing same we would copy the code again and again , which isnt a good practice rather we make function ..
// }

// This will be only valid for integer list , for char we have  to make another func
// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
// // like this .. while the func are same just names and types are different ..

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");
// }

// To paramaterize this function to be valid for other data types ..
// restriction on trait added to do comparision ..
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // direct comparision cannot be allowed to type &T . have to add restriction
        if item > largest {
            largest = item;
        }
    }

    largest
}

// also can be used in struct definition ..

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// In Method Definitions ..
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // fn distance_from_origin(&self) -> f32 {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // } powi works for integer not for generic types ..
}
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let doesnt_work = Point { x: 5, y: 4.0 }; // error ony same data types
    let now_works = Point2 { x: 5, y: 4.0 }; // works   
    println!("p.x is {}", integer.x());
}

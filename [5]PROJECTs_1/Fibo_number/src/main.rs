use std::io;

fn main() {

    // fibonacci series printing till n ... 
    let mut number = String::new(); 

    io::stdin().read_line(&mut number).expect("ERROR OCCURRED"); 

    let number: i32 = number.trim().parse().expect("ERROR OCCURRED"); 

    println!("Fibonacci series up to {} terms:", number);
    fibo_series(number);
}

fn fibo_series(n: i32) {
    let mut a = 0;
    let mut b = 1;

    for i in 1..=n {
        print!("{} ", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!(); // new line after printing series
}

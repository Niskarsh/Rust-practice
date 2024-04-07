// fn main() {
//     let x: u8; // Don't change this line!
//     x = 0;
//     println!("x is: {}", x);
// }

// fn main() {
//     let pi: f32;
//     pi = 3.14159; // This value represents pi 
//     println!("pi is: {}", pi);
// }

// fn main() {
//     let a: i32 = -15;
//     let b: i32 = 170;
//     let name: &str = "Michael";
//     println!("name is: {}, and the multiplication result is {}", name, a * b);

// }


// fn main() {
//     type book = (String, String, i32); // Add your code here to this line

//     let book1: book = (
//         String::from("Rust Programming Langauge"),
//         String::from("RUST Community"),
//         2010,
//     );
//     println!(
//         "Book name: {}, Author: {}, Year {}",
//         book1.0, book1.1, book1.2
//     );

//     let book2: book = (
//         String::from("Rust by Example"),
//         String::from("Steve Klabnik and Carol Nichols"),
//         2015,
//     );
//     println!(
//         "Book name: {}, Authors: {}, Year {}",
//         book2.0, book2.1, book2.2
//     );
// }


// add_3(x): This function should add three to the input variable ‘x’ and the return the resultant value.
// add_5(x): This function should add five to the input variable ‘x’ and the return the resultant value.
// times(x,y): This function should compute the multiplication of the inputs ‘x’ and ‘y’ and return the resultant value.

// fn add_3 (x: i32) -> i32 {
//     let sum = x+3;
//     sum
// }

// fn add_5(y: i32) -> i32 {
//     let sum = y+5;
//     sum
// }

// fn times(x: i32,y: i32) -> i32{
//     let mul = x*y;
//     mul
// }

// fn main() {
//     let x = 3;
//     let y = 4;
//     println!(
//         "The result of x+3 times y+5 is {}",
//         times(add_3(x), add_5(y))
//     );
//     print!("{}", x)
// }



//Problem 4: Add the definition of the quadruple function below by calling the double function twice inside the quadruple function.   

fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    // your code here //  
    let quad = double(double(x));
    quad
}

fn main() {
    println!(
        "For 1: the expected value is 4 while the output is {}",
        quadruple(1)
    );

    println!(
        "For 2: the expected value is 8 while the output is {}",
        quadruple(2)
    );

    println!(
        "For 3: the expected value is 12 while the output is {}",
        quadruple(3)
    );

    println!(
        "For 4: the expected value is 16 while the output is {}",
        quadruple(4)
    );
}

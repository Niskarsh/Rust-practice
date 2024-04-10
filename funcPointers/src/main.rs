// Problem 1: Complete the function signature for `sum_of_squares`. 
// It must not contain any generics. 

// fn add(x: u32, y: u32) -> u32 {
//     x + y
// }

// fn square(x: u32) -> u32 {
//     x * x
// }

// fn sum_of_squares(num: u32 , sq: fn(u32) -> u32, add: fn(u32, u32) -> u32) -> u32 { 
//     let mut result = 0;
//     for i in 1..=num {
//         result = add(result, sq(i));
//     }
//     result
// }

// fn main() {
//     let num: u32 = 4;
//     let sum = sum_of_squares(num, square, add);
//     println!("Sum of squares from 1 to {} = {}", num, sum);
// }

// Problem 2: Update the function signature so that it uses function pointers instead of closures. 
// You will also add a square function.

fn invoker<O>(operation: O, operand: i32) -> i32 // This needs to be updated
where
    O: Fn(i32) -> i32,
{
    operation(operand)
}

/* A square function needs to be added here */ 
fn square (x: i32) -> i32 {
    x * x
}

fn main() {
    // let square = |x: i32| x * x;

    let result = invoker(&square, 4);
    println!("Result is: {}", result);
}


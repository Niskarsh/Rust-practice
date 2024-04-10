// Problem 1: Complete the code by adding the closure definition

fn main() {
    let x = 10;
    let add_to_x = |y| y+x; /* Add closure definition here */

    let result = add_to_x(5);
    println!("Result: {}", result);
}

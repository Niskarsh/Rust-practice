// // Problem 1: Fix the code so that it compiles

// fn main() {
//     let mut vec_1 = vec![4, 5, 6, 9, 8];
//     for i in &mut vec_1 { // fix this line by making a call to relevant function
//         *i = *i * 2;
//     }
//     println!("{:?}", vec_1);
// }

// Problem 1: Convert the code based on the combinators  

fn main() {
    let numbers = vec![1, 2];
    let mut result = 0;

    result = numbers
    .iter()
    .map(|x|(*x)*(*x))
    .sum();
    /* The code in the for loop needs to be replaced */ 
    // for &num in &numbers {
    //     if num % 2 != 0 {
    //         let squared_num = num * num;
    //         result += squared_num;
    //     }
    // }

    println!("Result without combinators: {}", result);
}
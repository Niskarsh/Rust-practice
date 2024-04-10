// Problem 1: Fix the code so that it compiles

fn main() {
    let mut vec_1 = vec![4, 5, 6, 9, 8];
    for i in &mut vec_1 { // fix this line by making a call to relevant function
        *i = *i * 2;
    }
    println!("{:?}", vec_1);
}

// Problem 1: Fix the code by moving the indicated line to approperiate location

// fn main() {
//     let mut some_str = String::from("I am String");
//     let ref1 = &some_str;
//     println!("{ref1}"); // move this line only
//     let ref2 = &mut some_str;
//     ref2.push_str(" additional information");   
//     println!("{ref2}");
// }


// Problem 1: Fix the code by moving the indicated line to approperiate location

// fn identity(a: &i32) -> &i32 {
//     a
// }

// fn main() {
//     let mut x_ref: Option<&'static i32> = None;
//     {
//         let x:&'static i32 = &7;
//         x_ref = Some(identity(x));
//     }
//     assert_eq!(*x_ref.unwrap(), 7); // Issue at this line
// }

// Problem 1: Fix the code by moving the indicated line to approperiate location

// fn identity(a: &i32) -> &i32 {
//     a
// }

// fn main() {
//     let mut x_ref: Option<&i32> = None;
//     {
//         let x = 7;
//         x_ref = Some(identity(&x));
//         assert_eq!(*x_ref.unwrap(), 7); // Issue at this line
//     }
    
// }

//Problem 3: Fix the code by moving the indicated line to approperiate place 

// fn option(opt: Option<&i32>) -> &i32 {
//     opt.unwrap()
// }
// fn main() {
//     let y = 4; // move this line only 
//     let answer = { 
        
//         option(Some(&y)) 
//     };
//     assert_eq!(answer, &4);
// }

// Problem 1: Add lifetime annotations in the function signature

fn some_if_greater<'a, 'b>(number: &'a i32, greater_than: &'b i32) -> Option<&'a i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}
fn main() {
    let num_1 = 7;
    let greater_val = 4;
    let test = some_if_greater(&num_1, &greater_val);
    println!("{:?}", test);
}

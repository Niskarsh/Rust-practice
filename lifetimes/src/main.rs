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

fn identity(a: &i32) -> &i32 {
    a
}

fn main() {
    let mut x_ref: Option<&i32> = None;
    {
        let x = 7;
        x_ref = Some(identity(&x));
        assert_eq!(*x_ref.unwrap(), 7); // Issue at this line
    }
    
}

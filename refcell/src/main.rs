// Problem 1: fill in the code for TODO's

// use std::cell::RefCell;
// fn main() {
//     let data: RefCell<Option<i32>> = RefCell::new(Some(42));

//     // TODO: Use borrow_mut to safely modify the value inside the RefCell to None.
//     let mut a = data.borrow_mut();
//     *a = Some(34);
//     drop(a);

//     if *(data.borrow()) != None {
//         println!("Final value: {:?}", data.borrow());
//     } else {
//         println!("No value present.");
//     }
// }


// Problem 2: Fix the lines indicated in the code so that it compiles


// use std::cell::RefCell;
// #[derive(Debug)]
// struct Car {
//     model: String,
//     price: u32,
//     status: RefCell<&'static str>,
// }

// impl Car {
//     fn new(model: &str, price: u32) -> Self {
//         Car {
//             model: model.to_owned(),
//             price,
//             status: RefCell::new("Available"),
//         }
//     }

//     fn sold(&mut self) {
//         let new_status: &'static str = match self.price {
//             0..=50000 => "Sold - Economy",
//             50001..=100000 => "Sold - Mid Range",
//             _ => "Sold - Luxury",
//         };
//         self.status = RefCell::new(new_status); // fix this line
//     }
// }

// fn main() {
//     let mut car = Car::new("Sedan", 75000);
//     car.sold();
//     println!("Car Status: {:?}", car.status); // Fix this line 
// }


// Problem 3: The code below will compile. However it will panic at execution time.
// Task 1: Your first ttask is to add some code so that the it does not panice at execution time 
// Task 2: The value at the last print line will not be displayed. 
// Instead of value, <Borrowed> will be displayed. 
// Add appropriate code so that the value of x is being displayed. 

use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let x_ref1 = x.borrow();
    let x_ref2 = x.borrow();
    println!("x_ref1: {}, x_ref2: {}", x_ref1, x_ref2);
    
    /* Code for Task 1 */
    drop(x_ref1);
    drop(x_ref2);
    let mut x_ref3 = x.borrow_mut();
    *x_ref3 = 6;
    
    /* Code for Task 2 */
    drop(x_ref3);
    println!("Stored value: {:?}", x);
}


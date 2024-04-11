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


use std::cell::RefCell;
#[derive(Debug)]
struct Car {
    model: String,
    price: u32,
    status: RefCell<&'static str>,
}

impl Car {
    fn new(model: &str, price: u32) -> Self {
        Car {
            model: model.to_owned(),
            price,
            status: RefCell::new("Available"),
        }
    }

    fn sold(&mut self) {
        let new_status: &'static str = match self.price {
            0..=50000 => "Sold - Economy",
            50001..=100000 => "Sold - Mid Range",
            _ => "Sold - Luxury",
        };
        self.status = RefCell::new(new_status); // fix this line
    }
}

fn main() {
    let mut car = Car::new("Sedan", 75000);
    car.sold();
    println!("Car Status: {:?}", car.status); // Fix this line 
}

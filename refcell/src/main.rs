// Problem 1: fill in the code for TODO's

use std::cell::RefCell;
fn main() {
    let data: RefCell<Option<i32>> = RefCell::new(Some(42));

    // TODO: Use borrow_mut to safely modify the value inside the RefCell to None.
    let mut a = data.borrow_mut();
    *a = Some(34);
    drop(a);

    if *(data.borrow()) != None {
        println!("Final value: {:?}", data.borrow());
    } else {
        println!("No value present.");
    }
}

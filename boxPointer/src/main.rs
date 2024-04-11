// // Problem 1: Fix the code below so that it compiles 

// enum BinaryTree {
//     Leaf,
//     Node(i32, Box<BinaryTree>, Box<BinaryTree>),
// }

// fn main() {}

// Problem 2: Fix the code by completing the function signature

struct Wrapper {
    data: String,
}

fn modify_data(mut wrapper: Box<Wrapper>) -> Box<Wrapper> {
    wrapper.data = String::from("Modified");
    wrapper
}

fn main() {
    let original_wrapper = Box::new(Wrapper {
        data: String::from("Original"),
    });
    let modified_wrapper = modify_data(original_wrapper);
}

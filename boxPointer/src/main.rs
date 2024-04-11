// // Problem 1: Fix the code below so that it compiles

// enum BinaryTree {
//     Leaf,
//     Node(i32, Box<BinaryTree>, Box<BinaryTree>),
// }

// fn main() {}

// Problem 2: Fix the code by completing the function signature

// struct Wrapper {
//     data: String,
// }

// fn modify_data(mut wrapper: Box<Wrapper>) -> Box<Wrapper> {
//     wrapper.data = String::from("Modified");
//     wrapper
// }

// fn main() {
//     let original_wrapper = Box::new(Wrapper {
//         data: String::from("Original"),
//     });
//     let modified_wrapper = modify_data(original_wrapper);
// }

// Problem 3: Complete the code below

#[derive(Debug)]
enum ListNode<T> {
    /*TODO: Declare an enum variant called Node, with Box pointer for the next node of type 'T' */
    ///*TODO: Another variant for the placeholder for the end of the list
    Node(T, Box<ListNode<T>>),
    None,
}

fn main() {
    // Create a linked list representing: Node(1, Node(2, Node(3, Node(4, None))))
    let list = ListNode::Node(
        1,
        Box::new(ListNode::Node(
            2,
            Box::new(ListNode::Node(
                3,
                Box::new(ListNode::Node(4, Box::new(ListNode::None))),
            )),
        )),
    );
    println!("{:?}", list);
}

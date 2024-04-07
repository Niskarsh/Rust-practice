// // Problem 1: Fix the code below so that it compiles

// fn main() {
//     let s1: String = String::from("this is me, ");
//     let s2: &str = "Nouman";
//     some_function(&s1, s2); // Something is wrong here
//     println!("{} {}", s1, s2);
// }

// fn some_function(a1: &String, a2: &str) {
//     println!("{} {}", *a1, a2);
// }


// Problem 2: 

/* 
Fix the code below. By solving this problem you will be able to understand 
the change of ownership inside a loop 
*/ 

// fn main() {
//     let mut my_vec = vec![1, 2, 3, 4, 5];
//     let mut temp;

//     while !my_vec.is_empty() {
//         temp = &my_vec; // Something wrong on this line
//         println!("Elements in temporary vector are: {:?}", temp);


//         if let Some(last_element) = my_vec.pop() { // pop() is used to remove an element from the vec
//             println!("Popped element: {}", last_element);
//         }
//     }
// }


// Problem 3: Fix the code so that it compiles.


fn main() {
    
        let str1 = generate_string();
    
    let str2 = str1;   // Something wrong with this line
}

fn generate_string() -> String {
    let some_string = String::from("I will generate a string");
    some_string
}



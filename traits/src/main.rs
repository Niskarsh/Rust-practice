// Problem 1: Define a generic enum named 'Operation' that represents basic mathematical operations
// (e.g., Addition, Subtraction, Multiplication, Division). 
// Each variant should store two values of the same type.

/*
Define enum here
*/

// enum Operation<T> {
//     Addition(T, T),
//     Multiplication(T, T),
//     Subtraction(T, T),
//     Division(T, T),
// }

// // impl<T> Operation<T> {
// //     fn Addition (m: T, n: T) -> T {
// //         m+n
// //     }
// //  }

// fn main() {
//     let op_1 = Operation::Addition(5, 10);
//     let op_2 = Operation::Multiplication(3.5, 2.0);
//     let op_3 = Operation::Subtraction(3.5, 2.0);
//     let op_4 = Operation::Division(2, 3);
// }

// Problem 2: Fix the code below so that it compiles 


// struct Container<T> {
//     value: T,
// }

// impl<T> Container<T> {
//     fn create(value: T) -> Container<T> {       // something wrong here 
//         Container { value }
//     }
// }

// impl Container<i32> {
//     fn display(&self) {
//         println!("The value inside the container is: {}", self.value);
//     }

//     // fn create(value: i32) -> Container<i32> {
//     //     Container { value }
//     // }
// }

// fn main(){}

// Problem 3: Generalize the function take_and_return() so that the it can accept any type.  

struct User {
    name: String,
    id: u32,
}

fn take_and_return<T>(user: T) -> T { // this line needs updation
    user
}

fn main() {
    let user1 = User {
        name: "Alice".to_string(),
        id: 199,
    };
    let _user2 = take_and_return(user1);

    let str1 = String::from("Hello folks");
    let _str2 = take_and_return(str1); // we want this to compile
}
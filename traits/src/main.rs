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

// struct User {
//     name: String,
//     id: u32,
// }

// fn take_and_return<T>(user: T) -> T { // this line needs updation
//     user
// }

// fn main() {
//     let user1 = User {
//         name: "Alice".to_string(),
//         id: 199,
//     };
//     let _user2 = take_and_return(user1);

//     let str1 = String::from("Hello folks");
//     let _str2 = take_and_return(str1); // we want this to compile
// }

// Problem 1: Fix the code so that it compiles. 

// trait Sound {
//     fn animal_sound(&self) -> String {
//         "I dont have sound!".to_string()
//     }
// }
// struct Dog;
// struct Cat;
// struct Fish;

// impl Sound for Dog {
//     fn animal_sound(&self) -> String {
//         "woof".to_string()
//     }
// }
// impl Sound for Cat {
//     fn animal_sound(&self) -> String {
//         "meow".to_string()
//     }
// }
// impl Sound for Fish {} // fish does not make any sound so we should not implement the 
//                        //fn animal_sound(). This will make compiler unhappy 
// fn main() {
//     let dog = Dog;
//     let cat = Cat;
//     let fish = Fish;
//     println!("Dog Sound: {}", dog.animal_sound());
//     println!("Cat Sound: {}", cat.animal_sound());
//     println!("Fish Sound: {}", fish.animal_sound());
// }


// Problem 2: Fix the code by adding implementation for the Vehicle trait for the Bus and Bicycle types

trait Vehicle {
    fn speed(&self) -> f64 {
        0.0
    }
}

struct Car {
    model: String,
    speed: f64,
}

impl Vehicle for Car {
    fn speed(&self) -> f64 {
        self.speed
    }
}

// Do not change the struct definitions
struct Bicycle {
    brand: String,
}
impl Vehicle for Bicycle {}

struct Bus {
    model: String,
    speed: f64,
}

impl Vehicle for Bus {
    fn speed(&self) -> f64 {
        self.speed
    }
}

fn main() {
    let car = Car {
        model: "Camry".to_string(),
        speed: 120.0,
    };
    let bicycle = Bicycle {
        brand: "MountainBike".to_string(),
    };
    let bus = Bus {
        model: "BMC".to_string(),
        speed: 100.0,
    };

    car.speed();
    bicycle.speed();
    bus.speed();
}

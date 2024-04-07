// Problem 1:
/* In this exercise, you will be working on creating a student management system
using Rust. The system should allow you to store and retrieve student information
based on their unique ID. For ease of work, the student structure is already
created in the code below

Next, create a StudentManager structure containing a field of student, which
will essentially be a hashmap where the key part will be an integer representing
unique ID of student and the value part will be the complete details of the
students contained in the student structure.

The StudentManager should implement the following methods:
1. new() -> Self: A constructor that initializes an empty student manager.

2. add_student(&mut self, student: Student) -> Result<(), String>:
Adds a student to the manager.
If the student's ID already exists, return an error message.
Otherwise, add the student to the manager and return Ok.

3. get_student(&self, id: i32) -> Option<&Student>: Retrieves a student
from the manager based on their ID.
If the student is found, return Some(student). Otherwise, return None.

Your task is to implement the StudentManager structure, and the mentioned methods.
Additionally, provide a sample usage of the student management system by adding
a few students and retrieving their information using the get_student() method.
*/

// use std::collections::HashMap;

// #[derive(Debug)]
// struct Student {
//     id: i32,
//     name: String,
//     grade: String,
// }

// struct StudentManager {
//     students: HashMap<i32, Student>,
// }

// impl StudentManager {
//     fn new() -> Self {
//         let students = HashMap::new();
//         Self { students }
//     }
//     fn add_student(&mut self, student: Student) -> Result<(), String> {
//         if self.students.contains_key(&student.id) {
//             Err("Student already there".to_string())
//         } else {
//             self.students.insert(student.id, student);
//             Ok(())
//         }
//     }
//     //     get_student(&self, id: i32) -> Option<&Student>: Retrieves a student
//     // from the manager based on their ID.
//     // If the student is found, return Some(student). Otherwise, return None.
//     fn get_student(&self, id: i32) -> Option<&Student> {
//         self.students.get(&id)
//     }
// }

// fn main() {
//     let mut students = StudentManager::new();
//     let signup = students.add_student(Student { id: 1, name: "Nik".to_string(), grade: "A".to_string() });
//     match signup {
//         Ok(()) => {},
//         Err(err) => print!("Something broke: {}", err),
//     }
//     let student = students.get_student(1);
//     match student {
//         Some(s) => print!("{:?}", s),
//         None => {},
//     }

// }

//Problem 2:

/* The code below contains a student struct with three fields.
Your taks is to create a hashmap called student_database in the main function,
which will store instances of the Student structure. The keys of the hashmap
should be unique student IDs, represented as integers while the values will be
instances of the student struct.

Implement a function called add_student() that takes the student's ID, name, age, and grade as parameters.
The function should add a new entry to the student_database hashmap if the student ID doesn't already exist.
If the student ID already exists in the hashmap, the function should not add a new entry.

Use the skeleton of the function given below.
*/

// use std::collections::HashMap;
// struct Student {
//     name: String,
//     age: i32,
//     grade: String,
// }

// fn add_student(
//     student_database: &mut HashMap<i32, Student>,
//     id: i32,
//     name: String,
//     age: i32,
//     grade: String,
// ) {

//     // Your code here
//     let check_entry = student_database.get(&id);
//     if let None = check_entry {
//         student_database.insert(id, Student {
//             name,
//             age,
//             grade
//         });
//     }
// }

// fn main() {
//     let mut student_database: HashMap<i32, Student> = HashMap::new();
//     add_student(
//         &mut student_database,
//         1,
//         String::from("John"),
//         17,
//         String::from("Grade 11"),
//     );

//     add_student(
//         &mut student_database,
//         2,
//         String::from("Sarah"),
//         16,
//         String::from("Grade 10"),
//     );

//     // Printing the student database

//     for (id, student) in &student_database {
//         println!("Student ID: {}", id);
//         println!("Name: {}", student.name);
//         println!("Age: {}", student.age);
//         println!("Grade: {}", student.grade);
//         println!("------------------");
//     }
// }

//Problem 3: Modify the code by eliminating all the let statements

// use std::simd::SimdElement;

struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn new_fruit() -> Self {
        Self {
            apples: 10,
            bananas: 5,
        }
    }

    fn increase_fruit (&mut self) -> Self {
        Self {
            apples: self.apples*2,
            bananas: self.bananas*2,
        }
    }

    fn print_fruit (&self) {
        println!(
            "You have {} apples and {} bananas",
            self.apples, self.bananas
        );
    }
}

// fn increase_fruit(fruit: Fruit) -> Fruit {
//     let fruit = Fruit {
//         apples: fruit.apples * 2,
//         bananas: fruit.bananas * 3,
//     };
//     fruit
// }

// fn new_fruit() -> Fruit {
//     let fruit = Fruit {
//         apples: 10,
//         bananas: 5,
//     };
//     fruit
// }

// fn print_fruit(fruit: Fruit) {
//     println!(
//         "You have {} apples and {} bananas",
//         fruit.apples, fruit.bananas
//     );
// }

fn main() {
    let mut some_fruit = Fruit::new_fruit();

    some_fruit.increase_fruit();
    some_fruit.print_fruit();
    // print_fruit(updated_fruit);
}

// Problem 1: Compile the code by adding the definition for the next method

// struct Counter {
//     current: u32,
//     max: u32,
// }

// impl Counter {
//     fn new(max: u32) -> Counter {
//         Counter { current: 0, max }
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> { 
//         /* Add code here */
//         let c = self.current.clone();
//         if c < self.max {
//             self.current = self.current + 1;
//             Some(c)
//         } else {
//             None
//         }
         
//     }
// }

// fn main() {
//     let mut counter = Counter::new(3);
//     assert!(matches!(counter.next(), Some(0)));
//     assert!(matches!(counter.next(), Some(1)));
//     assert!(matches!(counter.next(), Some(2)));
//     assert!(matches!(counter.next(), None));
// }

// Problem 2: Complete the into_iter function definition 

// struct Person {
//     name: String,
//     age: u32,
//     occupation: String,
// }

// impl IntoIterator for Person {
//     type Item = String;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         /* Your code here 
//         Hint: Should return a vector of Strings, 
//         representing the fields of the struct  */
//         vec![self.name, self.occupation].into_iter()
//     }
// }

// fn main() {
//     let person = Person {
//         name: "Alice".to_string(),
//         age: 30,
//         occupation: "Software Engineer".to_string(),
//     };

//     let mut person_iterator = person.into_iter();

//     while let Some(property) = person_iterator.next() {
//         println!("{}", property);
//     }
// }

// Problem 3: Complete the into_iter function definition. 
// Also add the type for the associated type item

struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8; // this needs to be fixed 
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        /* The function needs to be completed */ 
        vec![self.r, self.g, self.b].into_iter()
    }
}

fn main() {
    let p = Pixel {
        r: 54,
        g: 23,
        b: 74,
    };
    let p = p.into_iter();

    for component in p {
        println!("{}", component);
    }
}

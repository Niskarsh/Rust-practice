// Problem 1: Fix the code by moving the indicated line to approperiate location

// fn main() {
//     let mut some_str = String::from("I am String");
//     let ref1 = &some_str;
//     println!("{ref1}"); // move this line only
//     let ref2 = &mut some_str;
//     ref2.push_str(" additional information");   
//     println!("{ref2}");
// }


// Problem 1: Fix the code by moving the indicated line to approperiate location

// fn identity(a: &i32) -> &i32 {
//     a
// }

// fn main() {
//     let mut x_ref: Option<&'static i32> = None;
//     {
//         let x:&'static i32 = &7;
//         x_ref = Some(identity(x));
//     }
//     assert_eq!(*x_ref.unwrap(), 7); // Issue at this line
// }

// Problem 1: Fix the code by moving the indicated line to approperiate location

// fn identity(a: &i32) -> &i32 {
//     a
// }

// fn main() {
//     let mut x_ref: Option<&i32> = None;
//     {
//         let x = 7;
//         x_ref = Some(identity(&x));
//         assert_eq!(*x_ref.unwrap(), 7); // Issue at this line
//     }
    
// }

//Problem 3: Fix the code by moving the indicated line to approperiate place 

// fn option(opt: Option<&i32>) -> &i32 {
//     opt.unwrap()
// }
// fn main() {
//     let y = 4; // move this line only 
//     let answer = { 
        
//         option(Some(&y)) 
//     };
//     assert_eq!(answer, &4);
// }

// Problem 1: Add lifetime annotations in the function signature

// fn some_if_greater<'a, 'b>(number: &'a i32, greater_than: &'b i32) -> Option<&'a i32> {
//     if number > greater_than {
//         Some(number)
//     } else {
//         None
//     }
// }
// fn main() {
//     let num_1 = 7;
//     let greater_val = 4;
//     let test = some_if_greater(&num_1, &greater_val);
//     println!("{:?}", test);
// }

// Problem 1: The code is compiling. However, I want you to remove the 
// extra lifetimes parameters from function signatures.  

struct DataHolder<'a> {
    data: Vec<&'a str>,
}

impl<'a> DataHolder<'a> {
    fn initialize() -> Self {
        DataHolder { data: Vec::new() }
    }

    fn add_data(&mut self, item: &'a str) {
        self.data.push(item);
    }

    fn extract_data_containing_substring(&mut self, sub: & str) -> & str {
        for i in 0..self.data.len() {
            if self.data[i].contains(sub) {
                return self.data.remove(i);
            }
        }
        panic!("Data containing substring not found");
    }
}

fn main() {
    let mut my_data = DataHolder::initialize();
    my_data.add_data("Apple");
    my_data.add_data("Banana");
    my_data.add_data("Cherry");
    my_data.add_data("Date");
    let extracted = my_data.extract_data_containing_substring("na");
    println!("Extracted: {}", extracted);
    assert_eq!(my_data.data.len(), 3);
}

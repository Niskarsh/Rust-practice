// Problem 1: Fix the code below so that it compiles 

// mod m1 {
//     struct A {
//         d: m2::D,
//     }
//     mod m2 {
//         pub enum D {
//             B,
//             C,
//         }
//     }
// }

// fn main(){}


// fn main() {
//     println!("Hello, world!");
// }


//Problem 2: Fix the code below so that it compiles 

// mod m1 {
    
//     struct A {
//         d: m2::D,
//     }
//     pub mod m2 {
//         pub enum D {
//             B,
//             C,
//         }
//     }
// }

// mod m3 {
//     struct C {
//         e: crate::m1::m2::D,
//     }
// }


// fn main(){}


// Problem #3: Re-export the items properly so that the code compiles

mod graphics {

    // Re-export the 'show_area' function for easier access
    // Re-export the 'calculate_area' function for easier access
    pub use shapes::calculate_area;
    pub use display::show_area;

    pub mod shapes {
        pub fn calculate_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }
    pub mod display {
        pub fn show_area(shape: &str, area: f64) {
            println!("The area of the {} is: {}", shape, area);
        }
    }
}

use graphics::calculate_area; // fix this line
use graphics::show_area; // fix this line
fn main() {
    let radius = 3.0;
    let area = calculate_area(radius);

    show_area("circle", area);
}

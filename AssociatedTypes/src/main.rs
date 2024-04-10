// ----------------------------------------------
// 		Associated Types in Traits
// ----------------------------------------------

// use std::i32;

// #[derive(Debug)]
// struct Km {
//     value: u32,
// }

// #[derive(Debug)]
// struct Kmh {
//     value: u32,
// }

// #[derive(Debug)]
// struct Miles {
//     value: u32,
// }

// #[derive(Debug)]
// struct Mph {
//     value: u32,
// }

// trait Distance {}

// trait DistanceMetric {
//     // fn getDistance(&self, time: i32) -> Box<dyn Distance>;
// }

// fn getDistance(distanceMetric: &dyn DistanceMetric, time: u32) -> Box<dyn Distance> {
    
// }


// // trait DistanceThreeHours {
// //     type Distance;
// //     fn distance_in_three_hours(&self) -> Self::Distance;
// // }

// // impl DistanceMetric for Mph {
// //     fn getDistance(&self, time: i32) -> Box<dyn Distance> {
// //         Box::new(Miles { value: (self.value * time) })
// //     }
// // }

// fn distanceThreeHours(&dyn DistanceMetric) -> DistanceMetric {

// }

// impl DistanceThreeHours for Kmh {
//     type Distance = Km;
//     fn distance_in_three_hours(&self) -> Self::Distance {
//         Self::Distance {
//             value: self.value * 3,
//         }
//     }
// }

// impl DistanceThreeHours for Mph {
//     type Distance = Miles;
//     fn distance_in_three_hours(&self) -> Self::Distance {
//         Self::Distance {
//             value: self.value * 3,
//         }
//     }
// }
// fn main() {
//     let speed_Kmh = Kmh { value: 90 };
//     let distance_Km = speed_Kmh.distance_in_three_hours();

//     println!(
//         "At {:?}, you will travel {:?} in 3 hours",
//         speed_Kmh, distance_Km
//     );

//     let speed_Mph = Mph { value: 90 };
//     let distance_Miles = speed_Mph.distance_in_three_hours();
//     println!(
//         "At {:?}, you will travel {:?}, in 3 hours",
//         speed_Mph, distance_Miles
//     );
// }


// Problem 1: Add the correct types for the associated type 'item' in the implemenation blocks
trait Container {
    type Item;

    fn add_item(&mut self, item: Self::Item);
    fn get_item(&self) -> Option<&Self::Item>;
}

struct VecContainer_i32 {
    items: Vec<i32>,
}

impl Container for VecContainer_i32 {
    type Item = i32; // This line needs a fix

    fn add_item(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn get_item(&self) -> Option<&i32> {
        self.items.last()
    }
}

struct OptionContainer<T> {
    item: Option<T>,
}

impl<T> Container for OptionContainer<T> {
    type Item = T; // This line needs a fix

    fn add_item(&mut self, item: T) {
        self.item = Some(item);
    }

    fn get_item(&self) -> Option<&T> {
        self.item.as_ref()
    }
}

fn main() {
    let mut vec_container = VecContainer_i32 { items: Vec::new() };
    vec_container.add_item(42);
    vec_container.add_item(123);

    if let Some(last_item) = vec_container.get_item() {
        println!("Last item in VecContainer: {}", last_item);
    } else {
        println!("VecContainer is empty");
    }

    let mut option_container = OptionContainer { item: None };
    option_container.add_item("Hello, Rust!");

    if let Some(only_item) = option_container.get_item() {
        println!("Only item in OptionContainer: {}", only_item);
    } else {
        println!("OptionContainer is empty");
    }
}

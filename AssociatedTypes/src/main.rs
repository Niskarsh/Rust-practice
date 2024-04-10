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

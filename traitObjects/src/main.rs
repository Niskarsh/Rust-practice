// Problem 1: fix the 'describe_animal' function signature and also fix the calls to it in main

// trait Animal {
//     fn name(&self) -> &str;
//     fn make_sound(&self);
//     // fn describe_animal(&self) { // This line needs a fix
//     //     println!("The {} says:", self.name());
//     //     self.make_sound();
//     // }
// }

// struct Lion {
//     name: String,
// }

// impl Animal for Lion {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn make_sound(&self) {
//         println!("Roar!");
//     }
// }

// struct Penguin {
//     name: String,
// }

// impl Animal for Penguin {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn make_sound(&self) {
//         println!("Honk!");
//     }
// }

// fn describe_animal(animal: &dyn Animal) {
//     println!("The {} says:", animal.name());
//     animal.make_sound();
// }


// fn main() {
//     let lion = Lion {
//         name: "Simba".to_string(),
//     };
//     let penguin = Penguin {
//         name: "Happy Feet".to_string(),
//     };

//     // The calls to function needs fixes
//     // lion.describe_animal();
//     // penguin.describe_animal();
//     describe_animal(&lion);
//     describe_animal(&penguin);
// }


// Problem 2: Fix the code by making suitable changes to the signatures 
// of the functions 'get_vehicle' and 'operate_vehicle' 

// trait Vehicle {
//     fn start_engine(&self) -> String;
//     fn drive(&self) -> String;
// }

// struct Car;

// struct Bicycle;

// impl Vehicle for Car {
//     fn start_engine(&self) -> String {
//         "🚗 Engine started".to_string()
//     }

//     fn drive(&self) -> String {
//         "🚗 Driving the car".to_string()
//     }
// }

// impl Vehicle for Bicycle {
//     fn start_engine(&self) -> String {
//         "🚲 No engine to start for a bicycle".to_string()
//     }

//     fn drive(&self) -> String {
//         "🚲 Pedaling the bicycle".to_string()
//     }
// }

// fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> { // This line needs a fix 
//     match vehicle_type {
//         "car" => Box::new(Car),
//         "bicycle" => Box::new(Bicycle),
//         _ => panic!("No vehicle of that type available"),
//     }
// }

// fn operate_vehicle(driver: &dyn Vehicle) { // This line needs a fix 
//     println!("{}", driver.start_engine());
//     println!("{}", driver.drive());
// }

// fn main() {
//     // Do not change code in main
//     let my_vehicle = get_vehicle("car"); 
//     operate_vehicle(&*my_vehicle);

//     let my_vehicle = get_vehicle("bicycle");
//     operate_vehicle(my_vehicle.as_ref());
// }


// Problem 3: Fix the code by adding a proper type annotation for the vector in main

trait Vehicle {
    fn start_engine(&self) -> String;
    fn drive(&self) -> String;
}

struct Car;

struct Bicycle;

impl Vehicle for Car {
    fn start_engine(&self) -> String {
        "🚗 Engine started".to_string()
    }

    fn drive(&self) -> String {
        "🚗 Driving the car".to_string()
    }
}

impl Vehicle for Bicycle {
    fn start_engine(&self) -> String {
        "🚲 No engine to start for a bicycle".to_string()
    }

    fn drive(&self) -> String {
        "🚲 Pedaling the bicycle".to_string()
    }
}

fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
    match vehicle_type {
        "car" => Box::new(Car),
        "bicycle" => Box::new(Bicycle),
        _ => panic!("No vehicle of that type available"),
    }
}

fn operate_vehicle(driver: &dyn Vehicle) {
    println!("{}", driver.start_engine());
    println!("{}", driver.drive());
}

fn main() {
    let vehicle_1 = get_vehicle("car");
    let vehicle_2 = get_vehicle("car");
    let vehicle_3 = get_vehicle("bicycle");

    let vehicles = vec![vehicle_1.as_ref(), vehicle_2.as_ref(), vehicle_3.as_ref()]; // Fix this line

    for v in vehicles {
        operate_vehicle(v);
    }
}



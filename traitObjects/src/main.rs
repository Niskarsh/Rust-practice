// Problem 1: fix the 'describe_animal' function signature and also fix the calls to it in main

trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
    // fn describe_animal(&self) { // This line needs a fix
    //     println!("The {} says:", self.name());
    //     self.make_sound();
    // }
}

struct Lion {
    name: String,
}

impl Animal for Lion {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Roar!");
    }
}

struct Penguin {
    name: String,
}

impl Animal for Penguin {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Honk!");
    }
}

fn describe_animal(animal: &dyn Animal) {
    println!("The {} says:", animal.name());
    animal.make_sound();
}


fn main() {
    let lion = Lion {
        name: "Simba".to_string(),
    };
    let penguin = Penguin {
        name: "Happy Feet".to_string(),
    };

    // The calls to function needs fixes
    // lion.describe_animal();
    // penguin.describe_animal();
    describe_animal(&lion);
    describe_animal(&penguin);
}

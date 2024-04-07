// Problem 1: Fix the code so that it compiles.


// fn first_character(chars: &Vec<char>) -> Option<char> {
//     if chars.len() > 0 {
//         Some(chars[0])
//     } else {
//         None
//     }
// }

// fn main() {
//     let my_chars = vec!['a', 'b', 'c', 'd'];
//     match first_character(&my_chars) {
//         Some(character) => println!("First character: {character}"),
//         None => println!("Empty array"),
//     }
// }

// Problem 2: Fix the code so that it compiles.


fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None
}

fn main() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("User's name: {fruit}");
    }
}

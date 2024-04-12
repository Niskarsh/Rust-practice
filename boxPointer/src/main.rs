// // Problem 1: Fix the code below so that it compiles

// enum BinaryTree {
//     Leaf,
//     Node(i32, Box<BinaryTree>, Box<BinaryTree>),
// }

// fn main() {}

// Problem 2: Fix the code by completing the function signature

// struct Wrapper {
//     data: String,
// }

// fn modify_data(mut wrapper: Box<Wrapper>) -> Box<Wrapper> {
//     wrapper.data = String::from("Modified");
//     wrapper
// }

// fn main() {
//     let original_wrapper = Box::new(Wrapper {
//         data: String::from("Original"),
//     });
//     let modified_wrapper = modify_data(original_wrapper);
// }

// Problem 3: Complete the code below

// #[derive(Debug)]
// enum ListNode<T> {
//     /*TODO: Declare an enum variant called Node, with Box pointer for the next node of type 'T' */
//     ///*TODO: Another variant for the placeholder for the end of the list
//     Node(T, Box<ListNode<T>>),
//     None,
// }

// fn main() {
//     // Create a linked list representing: Node(1, Node(2, Node(3, Node(4, None))))
//     let list = ListNode::Node(
//         1,
//         Box::new(ListNode::Node(
//             2,
//             Box::new(ListNode::Node(
//                 3,
//                 Box::new(ListNode::Node(4, Box::new(ListNode::None))),
//             )),
//         )),
//     );
//     println!("{:?}", list);
// }

// Problem 4: Fix the code by adding the type annotation 

// #[derive(Default)]
// struct AudioSample;
// #[derive(Default)]
// struct ImageFile;

// trait Media {
//     fn prin(&self);
//     fn new() -> Self
//     where
//         Self: Sized;
// }

// impl<T> Media for T {
//     fn new () -> T {
//         T::new()
//     }
//     fn prin(&self) {
//         print!("Hi");
//     }
// }


// impl AudioSample {
//     fn new() -> Self {
//         Self
//     }
// }

// impl ImageFile {
//     fn new() -> Self {
//         Self
//     }
// }


// fn main() {
//     let audio_1 = AudioSample::new();
//     let audio_2 = Box::new(AudioSample::new());

//     let audio_3 = audio_1;
//     let audio_4 = audio_2;

//     let image_1 = Box::new(ImageFile::new());

//     let media_collection: Vec<Box<dyn Media>> = vec![Box::new(audio_3), audio_4, image_1]; // Fix this line
//     for media in &media_collection {
//         (*media).prin()
//     }
// }

#[derive(Default)]
struct AudioSample;
#[derive(Default)]
struct ImageFile;

trait Media {
    fn prin(&self);
    fn new() -> Self
    where
        Self: Sized;
}

impl<T: Default> Media for dyn U {
    fn new() -> Self {
        Default::default()
    }
    fn prin(&self) {
        print!("Hi");
    }
}

// impl<T> Default for T {
//     fn default() -> T {
//         T
//     }
// }

// impl ImageFile {
//     fn new() -> Self {
//         ImageFile
//     }
// }

fn main() {
    let audio_1 = AudioSample::new();
    let audio_2 = Box::new(AudioSample::new());

    let audio_3 = audio_1;
    let audio_4 = audio_2;

    let image_1 = Box::new(ImageFile::new());

    let media_collection: Vec<Box<dyn Media>> = vec![Box::new(audio_3), audio_4, image_1];
    for media in &media_collection {
        media.prin();
    }
}

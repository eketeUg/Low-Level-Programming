// fn main() {
//     let mut x = 5;
//     println!("the value of x is: {}", x);
//     x = 6;
//     println!("the value of x is: {x}");
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("the value of x is: {x}");
//     }
//     println!("the value of x is: {x}");
// }

// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("the value of spaces is: {spaces}");
// }

// integer type

// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }


// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// The variable tup binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
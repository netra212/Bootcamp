// --------------------
// Constants & Statics
// --------------------
const MAX_PLAYERS: u8 = 10;
static mut CASION_NAME: &str = "Rusty Casion"; // can be changed but unsafe to do on static variable. 

fn main() {
    // TODO:
    println!("Hello, world!");

    // creation
    let a = 5.0;

    // mutability
    // In rust, variable are immutable by default.
    let mut b = 5;
    b = 10;

    // shadowing
    let c = 10;
    let c = 20; // second variable will shadow first_variable. 

    // scope
    let d = 30;
    println!("d is: {d}");

    // boolean.
    let b1 = true;

    // unsigned integers.
    let i1: u8 = 1;

    // signed integers.
    let i7: i8 = 1;

    // floating point numbers.
    let f1: f32 = 0.2;

    // platforms specific integers.
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str, and string.
    let c1: char = 'c';
    let s1: &str = "Hello";
    let s2: String = String::from("hello");

    // arrays.
    let a1 = [1, 2, 3, 4, 5];

    let i1 = a1[4];

    // Type aliasing.
    type age = u8;
    let a1: age = 57;

    // ---------------------
    // Constants & Statics:
    // ---------------------
    let a2 = 10;
    let b2 = 10;

    let c = CASION_NAME;
    let d = CASION_NAME;

    // function calling....
    my_function(2);

    // Control flow in rust....
    let d1 = 56;

    if d1 < 5 {
        println!("less than 5");
    }

    // loop
    let var = 'outer: loop {
        println!("Loop Foreever.");
        loop {
            break 'outer;
        }
    }

    // while loop
    while d1 < 100 {
        println!("I am greater than 100.");
    }
}

fn my_function(x: i32) {
    println!("my_function called with: {}", x);
}

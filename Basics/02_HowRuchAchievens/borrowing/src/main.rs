fn main() {
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
    let s1 = String::from("Rust"); // heap allocated string.
    let r1 = &s1;
    print_string(r1);
    println!("s1 is: {s1}");
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str("is awesome");
    p1
}

fn print_string(p1: &String) {
    println!("{p1}");
}

// *p1 -> asterik infront of any variable means de-referencing.

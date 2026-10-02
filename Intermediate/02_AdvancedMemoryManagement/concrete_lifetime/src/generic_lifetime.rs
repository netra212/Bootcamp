fn main() {
    let player1 = String::from("player 1");
    let player2 = String::from("player 2");

    let result = first_turn(player1.as_str(), player2.as_str());
    println!("Player going first is: {}", result);

    let player3 = String::from("player 3");
    {
        let player4 = String::from("player 4");
    }
}

// here a is not a concrete lifetime, rather than a relational lifetime.
// lifetime of return value is equal to shortest lifetime.
fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() { p1 } else { p2 }
}

// static lifetime : define a references that can live during the entire program.eg: string slices.
fn first_turn(p1: &str, p2: &str) -> &'static str {
    let s: &'static str = "Let's Get Rusty!"; // s will be valid for the entire duration of the program. 
    p1;
}

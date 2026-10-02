fn main() {
    let s1 = String::from("Let's Get Rusty!"); // The lifetime of s1 start from line 2 and ends on line 6 and s1 will be dropped.
    println!("s1: {s1}");
    // Borrow checker check the lifetime of the value.
    // Concrete Lifetime: is a time during value exist at a particular memory location.

    // let s2 = s1;
    // println!("s1: {s1}");

    // Lifetime with references.
    // let r1;
    {
        // let s1 = String::from("Let's Get Rusty!");
        // r1 = &s1;
    }
    // println!("r1: {r1}"); // r1 will be invalid here. we can fixed by moving this println statement inside the inner scope {}

    // Lifetime with mutable and immutable references.
    //
    let mut s2 = String::from("Let's Get Rusty.");
    let r2 = &s2;
    println!("r1: {r2}"); // r2 lives till line 22. 
    let r3 = &mut s2;
    r3.push_str("!");

    // Borrowing rule: There can be only one mutable reference at one time but can be multiple immutable references at a given time.
}

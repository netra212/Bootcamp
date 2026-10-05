fn main() {
    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    //
    let v2 = vec![1, 2, 3];

    let s = &v[0]; // can panic if invalid index passed. 

    // safer way.
    // let s = v.remove(0);

    // get method for indexing.
    let s = v.get(0); // much more safer.

    if let Some(e) = s {
        println!("{e}");
    }

    // taking an mutable reference of an vector.
    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}");
    }

    // for loop that consume an vector.
    let v3 = vec![];
    for s in v.into_iter() {
        v3.push(s);
    }

    let i = v.get(0);
} // vector v is dropped here. 

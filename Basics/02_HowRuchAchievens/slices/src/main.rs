fn main() {
    // slices are references to a contigous sequence of elements in a collection.
    let tweet = String::from("This is my twee & it's very very long.");
    let trim_tweet: &str = &tweet[..20]; // string slice.
    println!("{trim_tweet}");
    // String Types.
    // String
    // - growable, heap allocated string (UTF-8 encoded.)
    // str
    // - Immutable sequence of UTF-8 bytes somewhere in memory (stack, heap, or static memory).
    // - Handle behind a reference (&str) because length of sequence is unknown at compile time.
}

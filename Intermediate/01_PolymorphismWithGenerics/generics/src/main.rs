struct BrowserCommand<T> {
    name: String, // name of the command.
    payload: T,   // data associated with the command.
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new(
        "nagivate".to_owned(),
        "https://www.letgetrusty.com".to_owned(),
    );

    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);

    cmd1.print_payload();
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),   // T is the type store.
    Errr(E), // E is the error.
}

fn serialize_payload<T>(payload: T) -> String {
    // convert payload to JSON string...
    "placeholder".to_owned()
}

// Monomorphization.
fn serialize_payload_string(payload: &String) -> String {
    // convert payload to JSON string...
    "placeholder".to_owned()
}
fn serialize_payload_i32(payload: &i32) -> String {
    // convert payload to JSON string...
    "placeholder".to_owned()
}

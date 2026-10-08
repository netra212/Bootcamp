
// Macros ?
// - referred as syntax extensions. 
// - reduce the amount of code that we have to write and maintain. 
// - Increase compile time. 

// Declarative Macros. 
use macros::*;
use syn::Data;
use std::{collections::HashMap, fmt::format, hash::Hash};


// procedural macros import. 


// ---------------------------------------------
// Logging
// ---------------------------------------------
trait Log {
    fn info(&self, msg:&str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug)]
struct Database {
    url: String, 
    connections: u32,
}

impl Database {
    fn new(url: String) -> Database {
        Database {url, connections
        :0}
    }

    fn connect(&mut self) {
        self.info(format!("New Connection to {}", self.url().as_str()));
        self.connections += 1;
        if self.connections >= 100 {
            // a lot of connections. 
            self.warn(format!("100 or more connections open!").as_str());
        }
    }
}

// Procedural Macros. 
#[derive(Debug)]
struct Product {
    name: String, 
    price: u32,
}

fn main() {
    hello!();

    let mut scores2 = HashMap::new();
    scores2.insert("Red team".to_owned(), 3);
    scores2.insert("Black team".to_owned(), 1);
    scores2.insert("Blue team".to_owned(), 4);
    scores2.insert("Gorg team".to_owned(), 3);


    let scores2 = map!(
        "Red_Team".to_owned() => 3, 
        "Blue_Team".to_owned() => 4, 
        "star_Team".to_owned() => 6
    );

    // Implementation of Procedural Macros 
    // log_info(println!("Hello"));
    let mut db = Database::new("localhost:5433".to_owned());
    db.connect();

    // Procedural macros attributes. 
    let laptop = Product {
        name: "MacBook Pro".to_owned(), 
        price: 2000
    };

    buy_product(laptop, 20);
}

#[log_call(verbose)]
fn buy_product(product: Product, discount: i32) {
    // ....
}

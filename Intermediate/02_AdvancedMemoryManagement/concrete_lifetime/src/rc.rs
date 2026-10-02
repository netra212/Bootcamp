use std::rc::Rc;

struct Database {}

struct AuthService {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}

// Rc -> can be used in single threaded application.
// Rc -> Rc smart pointer only allow immutable shared value.
fn main() {
    // First wrap database instance in Rc smart pointer.
    //
    // When db is zero, then Rc dropped the value it hold.
    let db = Rc::new(Database {}); // reference count: 1
    // In this case, we want both service to share same database.
    // which means, One database instance must be shared into two services at same time.
    // But in rust, that is not allowed,
    // to counter that, we implement Rc-> Reference count smart pointer.
    //
    // calling clone
    let auth_service = AuthService { db: Rc::clone(&db) }; // clone does not make a new clone of database, rather it increment the reference count. // reference count: 2
    let content_service = ContentService { db: Rc::clone(&db) }; // reference count: 3
} // At the end of the main function, all three variable will be dropped, when content_service dropped, Rc smart pointer inside a db field also dropped, and value of reference count will be decrement by 1.  

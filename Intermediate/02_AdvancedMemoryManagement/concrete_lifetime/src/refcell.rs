use std::rc::Rc;
use std::std::RefCell;

struct Database {
    max_connection: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

// RefCell: RefCell smart pointer allow mutable and immutable shared value.
// RefCell uses the interior mutablility design pattern which allows the mutably borrowed data even though there is immutable reference to that data. This breaks the compile time borrowing rule.
fn main() {
    // Wrapping database into RefCell to allow both mutable and immutable reference of share value.
    let db = Rc::new(RefCell::new(Database {
        max_connection: 100,
    }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    let mut r1 = db.borrow_mut(); // mutable reference. 
    let r2 = db.borrow_mut();

    r1.max_connection = 200;
}

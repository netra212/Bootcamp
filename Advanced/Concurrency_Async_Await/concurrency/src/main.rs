// use std::rc::Rc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::{thread, time::Duration};
use std::sync::Arc;
use tokio::time::sleep;

#[derive(Debug)]
struct Database {
    connections: Vec<i32>,
}

// Imagine If we want to share an instance of database
impl Database {
    fn new() -> Database {
        Database {
            connections: vec![],
        }
    }

    fn connect(&mut self, id: i32) {
        self.connections.push(id)
    }
}

#[tokio::main] // async code will be executed by tokio runtime. 
async fn main() {
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned Thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("Main Thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // In this case, Main thread will panic.

    // moving values into thread.
    let s = "Let's Get Rusty".to_owned();
    let handle = thread::spawn(move || {
        println!("{s}");
    });

    // Message passing between thread.

    // Either we can pass message between threads.
    // Eitehr we can share state.
    // mpsc: mult-producer, single-consumer FIFO queue communication primitives.

    let (tx, rx) = mpsc::channel();

    let sentences = [
        "Hello world".to_owned(),
        "something".to_owned(),
        "Beautiful person".to_owned(),
        "Rust is easy".to_owned(),
    ];

    for s in sentences {
        let tx_clone = tx.clone(); // cloning sender. 
        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx);

    for sentence in rx {
        println!("{sentence}");
    }

    // ---------------------------------------------------------
    //
    // Implementation of share state with Mutex
    // Mutex: used for sharing instance between two or more state.
    let db = Arc::new(Mutex::new(Database::new())); // db is mutex which wrap the database. Arc: atomic reference counting: 
    // Rc: reference counting. 
    let mut handles = vec![];

    for i in 0..10 {
        // Arc: allow shared ownership of data across multiple threads. 
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });// db_lock will be drop here because it goes out of scope.
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // let db_lock = db.lock().unwrap();
    // println!("{db_lock}");

    // send + sync trait. 
    // Both of these are marker trait, they don't do anything, rather they are just a label on type to let the compiler know that certain properties are enforece by developers. Both of these trait are mark as unsafe because compiler cannot verify that whether these properties are enforced. 

    // Implementation of async.
    //
    // let f = my_function();
    // println!("Let's Learn Rust");
    // f.await;

    // tokio task
    let mut handles2 = vec![];
    
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles2.push(handle);
    }

    for handle in handles2 {
        handle.await.unwrap();
    }

}

// Concurrency:-
// When different parts of program execute independently.
// Time slicing: Execution of these parts is interleaved on a single core.
// Parallel Execution: Execution of these parts happens at the same time using multiple cores.


// 
async fn my_function(i: i32) {
    println!("{i} an async function!");
    let s1 = read_from_database().await;
    println!("{i} -> First result: {s1}");
    let s2 = read_from_database().await;
    println!("{i} -> Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(10)).await;
    "DB Reult".to_owned()
}



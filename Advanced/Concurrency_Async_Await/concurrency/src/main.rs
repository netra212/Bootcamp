use std::{thread, time::Duration};
use std::sync::mpsc;

fn main() {
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
        "Rust is easy".to_owned()
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
    
}

// Concurrency:-
// When different parts of program execute independently.
// Time slicing: Execution of these parts is interleaved on a single core.
// Parallel Execution: Execution of these parts happens at the same time using multiple cores.

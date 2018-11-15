extern crate rand;
use rand::prelude::*;
use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    // Create a thread safe Mutex value initialized to 0.
    let val = Arc::new(Mutex::new(0));    
    // Create a range for the random number generator
    let mut range = thread_rng();
    for _ in 0..100 {
        let mutex = Arc::clone(&val);
        thread::spawn(move || {
            let rx = mutex.lock().unwrap();
            println!("Received: {}", *rx);
            thread::sleep(Duration::from_millis(250));
        }).join();
        
        // Gain access to the Mutex
        let mut tx = val.lock().unwrap();
        // Assign the value to a random generated value.
        *tx = rand::random::<u8>();
        print!("Produced: {}\t", *tx);
        thread::sleep(Duration::from_millis(250));
    }
    println!("\nDone.");
}

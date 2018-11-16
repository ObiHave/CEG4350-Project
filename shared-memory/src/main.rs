//extern crate circular_queue;
extern crate rand;
//use rand::prelude::*;
use std::thread;
use std::sync::{RwLock, Mutex, Arc};
use std::time::Duration;
//use circular_queue::CircularQueue;

fn main() {
    // Create a thread safe Mutex value initialized to 0.
    let val = Arc::new(RwLock::new(0));
    let mutex1 = Arc::clone(&val);
    thread::spawn(move || {
        for _ in 0..100 {
        // Gain access to the Mutex
        let mut tx = mutex1.write().unwrap();
        // Assign the value to a random generated value.
        *tx = rand::random::<u8>();
        print!("Produced: {}\t", *tx);
        } 
    });
    let mutex2 = Arc::clone(&val);
    thread::spawn(move || {
        for _ in 0..100 {
        let rx = mutex2.read().unwrap();
        println!("Received: {}", *rx);
        thread::sleep(Duration::from_millis(250));
        }
    }).join();
    
    println!("\nDone.");
}

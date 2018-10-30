extern crate rand;
use rand::prelude::*;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Create a range for the random number generator
    let mut range = thread_rng();
    // Create a channel for message passing.
    let (tx, rx) = mpsc::channel();
      
    thread::spawn(move || {
        for recieved in rx {
            println!("Recieved: {}", recieved);
        }   
    });
    // Generate and send the random produced numbers
    let mut i = 0;
    while i != 100 {
        let x: u8 = range.gen_range(0,100);
        print!("Produced: {}\t", x);
        tx.send(x).unwrap();
        i = i + 1;
        thread::sleep(Duration::from_millis(1000));
    }
    println!("\nDone.");
}

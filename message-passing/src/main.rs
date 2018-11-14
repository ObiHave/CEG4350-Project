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
    // Generates a Vector of 100 random unsigned integers
    let mut vals: Vec<u8> = (0..100).map(|_| range.gen_range(0,100)).collect();
    
    // Spawn a thread to recieve the message and print it to the terminal
    thread::spawn(move || {
        for recieved in rx {
            println!("Recieved: {}", recieved);
        }   
    });
    // Generate and send the random produced numbers
    for _ in 0..100 {
        // Pop the last item in the random data vector
        let x: u8 = vals.pop().unwrap();
        // Print the data to the terminal and send it to the receiver
        print!("Produced: {}\t", x);
        tx.send(x).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
    println!("\nDone.");
}

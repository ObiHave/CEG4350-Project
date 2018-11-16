extern crate rand;
use rand::prelude::*;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Create a channel for message passing.
    let (tx, rx) = mpsc::channel();
    // Generates a Vector of 100 random unsigned integers
    let mut vals: Vec<u8> = (0..100).map(|_| rand::random::<u8>).collect();
    
    // Spawn a thread to recieve the message and print it to the terminal
    thread::spawn(move || {
        for recieved in rx {
            println!("Recieved: {}", recieved);
        }   
    });
    // Generate and send the random produced numbers
    for _ in 0..100 {
        // Pop the last item off the random data vector
        let x: u8 = vals.pop().unwrap();
        // Print the data to the terminal
        print!("Produced: {}\t", x);
        // Send the data to the reciever
        tx.send(x).unwrap();

        thread::sleep(Duration::from_millis(250));
    }
    println!("\nDone.");
}

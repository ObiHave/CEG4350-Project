extern crate rand;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Create a channel for pipe transfers.
    let (tx, rx) = mpsc::channel();
    // Generates a Vector of 100 random 8-bit unsigned integers
    let mut vals: Vec<u8> = (0..100).map(|_| rand::random::<u8>()).collect();
    
    // Spawn a thread to recieve the data and print it to the terminal
    thread::spawn(move || {
        for recieved in rx {
            println!("Recieved: {}", recieved);
        }   
    });
    // Spawn a thread to "produce" data, print it to the terminal, and send it to the receiver.
    // This thread is joined so that the program does not finish executing before all 100 data is produced.
    thread::spawn(move || {
        // Loop for the length of the vector
        for _ in 0..vals.len() {
            // Pop the last item off the random data vector
            let x: u8 = vals.pop().unwrap();
            // Print the data to the terminal
            print!("Produced: {}\t", x);
            // Send the data to the reciever
            tx.send(x).unwrap();

            thread::sleep(Duration::from_millis(250));
        }
    }).join();
    
    println!("\nDone.");
}

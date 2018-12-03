extern crate rand;
use rand::Rng;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Create a channel for message passing, with a maximum buffer size of 5 items.
    let (tx, rx) = mpsc::sync_channel(5);
    // Generates a Vector of 100 random 8-bit unsigned integers
    let mut vals: Vec<u8> = (0..100).map(|_| rand::random::<u8>()).collect();
    
    // Spawn a thread to receive the data and print it to the terminal
    thread::spawn(move || {
        for received in rx {
            println!("\t\tReceived: {}", received); 

            // Paused thread execution for a random amount of time.
            let pause: u16 = rand::thread_rng().gen_range(0, 1000);
            thread::sleep(Duration::from_millis(pause.into())); 
        }   
        println!("Received 100 values. Consumer terminating.");
    });
    // Spawn a thread to "produce" data, print it to the terminal, and send it to the receiver.
    // This thread is joined so that the program does not finish executing before all 100 data is produced.
    thread::spawn(move || {
        // Loop for the length of the vector
        for _ in 0..vals.len() {
            // Pop the last item off the random data vector
            let x: u8 = vals.pop().unwrap();
            // Print the data to the terminal
            println!("Produced: {}\t", x);
            // Send the data to the reciever
            tx.send(x).unwrap();
            let pause: u16 = rand::thread_rng().gen_range(0, 500);
            thread::sleep(Duration::from_millis(pause.into()));
        }
        println!("Produced 100 values. Producer terminating.");
    }).join();
    
    println!("\nDone.");
}

extern crate rand;
use rand::prelude::*;
use std::thread;
use std::net::UdpSocket;
use std::time::Duration;

fn main() {    
    // Create a range for the random number generator
    let mut range = thread_rng();
    // Bind the sockets, localhost ports 8080 and 8081, or throw the expected error.
    let mut producer_socket = UdpSocket::bind("127.0.0.1:8080").expect("Couldn't bind producer socket.");
    let mut consumer_socket = UdpSocket::bind("127.0.0.1:8081").expect("Couldn't bind consumer socket.");
    // Keep track of the addresses of the sockets
    let prod_name = producer_socket.local_addr().unwrap();
    let cons_name = consumer_socket.local_addr().unwrap();

    // Create a buffer to hold the data sent/received. Size of 1 item, initialized to 0. 
    let mut buffer = [0, 1];

    // Generates a Vector of 100 random unsigned integers
    let vals: Vec<u8> = (0..100).map(|_| range.gen()).collect();
    // Spawn a thread to produce the data.
    thread::spawn(move || {
        // Loop for the length of the vector.
        for i in 0..vals.len() {
            // Print the value produced to the terminal.
            print!("Produced: {}\t", vals[i]);
            // Send the data over the socket to the consumer socket.
            let _ = producer_socket.send_to(&vals[i..i+1], cons_name);

            thread::sleep(Duration::from_millis(250));
        }
        println!("Produced 100 values. Producer terminating.");
    });
    // Spawn a thread to consume the data.
    // This thread is joined so that the program does not finish executing before all 100 data is consumed.
    thread::spawn(move || {
        // Loop 100 times to receive 100 data.
        for _ in 0..100 {
            // Receive the data over the socket
            consumer_socket.recv_from(&mut buffer).expect("Failed to receive message.");
            // Print the data received to the screen.
            println!("Received: {}", buffer[0]);

            thread::sleep(Duration::from_millis(250));
        }
        println!("Received 100 values. Consumer terminating.");
    }).join();
    

    println!("\nDone.");
}

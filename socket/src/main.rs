extern crate rand;
use rand::prelude::*;
use std::thread;
use std::net::UdpSocket;
use std::time::Duration;

fn main() {    
    // Create a range for the random number generator
    let mut range = thread_rng();
    // Bind the sockets, localhost ports 8080 and 8081
    let producer_socket = UdpSocket::bind("127.0.0.1:8080").expect("Couldn't bind producer socket.");
    let consumer_socket = UdpSocket::bind("127.0.0.1:8081").expect("Couldn't bind consumer socket.");
    let consumer_addr = consumer_socket.local_addr().unwrap();

    //Create a buffer with capacity to hold one recieved data item.
    let mut buffer = [0];

    // Generates a Vector of 100 random unsigned 8-bit integers
    let vals: Vec<u8> = (0..100).map(|_| range.gen()).collect();

    thread::spawn(move || {
        for i in 0..100 {
            print!("Produced: {}\t", vals[i]);
            // Send the data over the socket
            let _ = producer_socket.send_to(&vals[i..i+1], consumer_addr);
            thread::sleep(Duration::from_millis(250));
        }
    });
    
    for _ in 0..100 {
        // Receive the data over the socket
        consumer_socket.recv_from(&mut buffer).expect("Failed to receive message.");
        println!("Received: {}", buffer[0]);
    }
    println!("Done.");
    
}

extern crate rand;
use rand::prelude::*;
use std::thread;
use std::net::UdpSocket;
use std::time::Duration;

fn main() {    
    // Create a range for the random number generator
    let mut range = thread_rng();
    // Bind the sockets, localhost ports 8080 and 8081
    let mut producer_socket = UdpSocket::bind("127.0.0.1:8080").expect("Couldn't bind producer socket.");
    let mut consumer_socket = UdpSocket::bind("127.0.0.1:8081").expect("Couldn't bind consumer socket.");
    let prod_name = producer_socket.local_addr().unwrap();
    let cons_name = consumer_socket.local_addr().unwrap();

    //Create a buffer to hold the data sent/received
    let mut buffer = [0, 100];

    // Generates a Vector of 100 random unsigned integers
    let vals: Vec<u8> = (0..100).map(|_| range.gen()).collect();

    thread::spawn(move || {
        // Send the data over the socket
        let amt = producer_socket.send_to(&vals[..], cons_name);
        println!("{}: Sent {} bytes to {}.", prod_name, amt.unwrap(), cons_name);
    });
   // Generate and send the random produced numbers
    //Receive the data over the socket
    let (size, peer) = consumer_socket.recv_from(&mut buffer).expect("Failed to receive message.");
    println!("{}: Received {} bytes from {}", cons_name, size, peer);
    println!("\nDone.");
}

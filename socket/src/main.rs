extern crate rand;
use rand::prelude::*;
use std::thread;
use std::net::UdpSocket;
use std::time::Duration;

fn main() {
    // Create a range for the random number generator
    let mut range = thread_rng();
    // Bind the socket, localhost port 8080
    let socket = UdpSocket::bind("127.0.0.0.1::8080").unwrap();
    println!("Listening on {}", socket.local_addr().unwrap());

    //Create a buffer to hold the data sent/received
    let mut buffer = [0, 100];

    thread::spawn(move || {
         //Receive the data over the socket
         let (size, peer) = socket.recv_from(&mut buffer).expect("Failed to receive message.");
         for i in 0..size {
             println!("Received: {}", buffer[i]);
         }
    });
    // Generate and send the random produced numbers
    let mut i = 0;

    while i != 100 {
        let vals: Vec<u8> = (0..100).map(|_| range.gen_range(0,100)).collect();
      //  print!("Produced: {}\t", vals);
        // Send the data over the socket
        let _amt = socket.send_to(&vals.as_slice(), socket.local_addr().unwrap());
        i = i + 1;
        thread::sleep(Duration::from_millis(1000));
    }
    println!("\nDone.");
}

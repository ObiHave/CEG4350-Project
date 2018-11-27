extern crate rand;
extern crate queue;
use queue::Queue;
use rand::Rng;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let queue: Queue<u8> = Queue::with_capacity(10);
    // Create a thread safe Mutex value initialized to 0.
    let val = Arc::new(Mutex::new(queue));
 
    
    
        let mutex1 = val.clone();
        let producer = thread::spawn(move || {
            for _ in 0..100 {
                // Gain access to the Mutex
                {
                    let mut tx = mutex1.lock().expect("The write Mutex was poisoned!");
                    // Assign the value to a random generated value.
                    let x: u8 = rand::random::<u8>();
                    while tx.len() > 10 {}
                    tx.queue(x);
                    println!("Produced: {}\t", x);
                }              
                let num: u16 = rand::thread_rng().gen_range(0, 501);
                thread::sleep(Duration::from_millis(num.into())); 
            }
            println!("Dat's a hundo.");
        });

    let mutex2 = val.clone();
    let consumer = thread::spawn(move || {

        loop {

            {
                let mut rx = mutex2.lock().expect("The read Mutex was poisoned!");
                if let Some(item) = rx.dequeue() {
                    println!("\t\t\tReceived: {} \t | Queue Length: {}", item, rx.len());
                }
            }
            let num: u16 = rand::thread_rng().gen_range(0, 501);
            thread::sleep(Duration::from_millis(num.into()));     
        } 
    }).join();

    println!("\nDone.");
}

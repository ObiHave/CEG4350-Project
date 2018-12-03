extern crate rand;
extern crate queue;
use queue::Queue;
use rand::Rng;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    // Create a ring buffer "queue" with a maximum size of 10.
    let queue: Queue<u8> = Queue::with_capacity(10);
    // Create a thread safe Mutex value initialized to track the queue.
    let mutex = Arc::new(Mutex::new(queue));    
    
    // In order to have persistent Mutexes across threads in Rust, the mutex must be cloned.
    // This cloned mutex is then borrowed by the producer thread.
    let pmutex = mutex.clone();
    // Create a new execution thread, which solely executes the producer_thread function.
    thread::spawn(move || producer_thread(pmutex));

    
    // Clone the mutex to be borrowed by the consumer thread.
    let cmutex = mutex.clone();
    // Create a new execution thread, which solely executes the consumer_thread function.
    // This thread is joined so that the program does not finish executing before all 100 data is consumed.
    thread::spawn(move || consumer_thread(cmutex)).join();

    println!("Done.");
}

// Generates 100 random 8-bit unsigned integers and adds them to the queue.
fn producer_thread(mutex: Arc<Mutex<Queue<u8>>>) {
    // Instantiate a counter to only produce 100 data
    let mut tx_count = 0;
    
    loop {
        {
            // Attempt to gain access to the mutex, or print out the expected error message.
            let mut tx = mutex.lock().expect("The write Mutex was poisoned!");
            // If the queue is full, go to the next iteration of the loop and try again.
            if tx.len() > 9 {continue}
            // Generate a random 8-bit unsigned integer.
            let x: u8 = rand::random::<u8>();
            // Add the random number to the first spot in the queue.
            tx.queue(x);
            // Print out the number produced.
            println!("Produced: {}\t", x);
            // Keep track of the amount of data generated.
            tx_count += 1;
            // If 100 data has been produced, break the loop.
            if tx_count == 99 {break}
        }           
        // Create a random number between 0 and 500.   
        let pause: u16 = rand::thread_rng().gen_range(0, 500);
        // Paused thread execution for a random amount of time.
        thread::sleep(Duration::from_millis(pause.into())); 
    }
    println!("Produced 100 values. Producer terminating.");
}

fn consumer_thread(mutex: Arc<Mutex<Queue<u8>>>) {
    // Instantiate a counter to only consume 100 data
    let mut rx_count = 0;
    loop {

        {
            // Attempt to gain access to the mutex, or print out the expected error message.
            let mut rx = mutex.lock().expect("The read Mutex was poisoned!");
            // If there is something in the queue that is able to be taken out, consume it.
            if let Some(item) = rx.dequeue() {
                // Keep track of the amount of data consumed.
                rx_count += 1;
                println!("\t\t\tReceived: {} \t | Queue Length: {}", item, rx.len());
            }
            // If 100 data has been consumed, break the loop.
            if rx_count == 99 {break}
        }
        // Paused thread execution for a random amount of time, between 0 and 500 milliseconds.
        let pause: u16 = rand::thread_rng().gen_range(0, 500);        
        thread::sleep(Duration::from_millis(pause.into()));     
    }
    println!("Received 100 values. Consumer terminating."); 
}
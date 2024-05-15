use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

use rand::random;
use synchronization_primitives::semaphore::Semaphore;

const THREADS: u32 = 10;
const LIMIT: u32 = 2;

fn main() {
    let semaphore = Arc::new(Semaphore::new(LIMIT));
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let semaphore = semaphore.clone();
        let counter = counter.clone();
        let handle = spawn(move || {
            pre();
            semaphore.wait();
            inside(&counter);
            semaphore.signal();
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn pre() {
    sleep(Duration::from_millis(random::<u64>() % 5000));
}

fn inside(counter: &Mutex<u32>) {
    {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        println!("Inside! - {}", counter);
    }
    sleep(Duration::from_millis(random::<u64>() % 1000));
    {
        let mut counter = counter.lock().unwrap();
        *counter -= 1;
        println!("Leaving! - {}", counter);
    }
}

use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

use rand::random;
use synchronization_primitives::monitor::Monitor;

const THREADS: u32 = 10;
const LIMIT: u32 = 2;

fn main() {
    let semaphore = Arc::new(Monitor::new(LIMIT));
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let semaphore = semaphore.clone();
        let counter = counter.clone();
        let handle = spawn(move || {
            pre();
            wait(&semaphore);
            inside(&counter);
            signal(&semaphore);
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

fn wait(semaphore: &Monitor<u32>) {
    let mut guard = semaphore.lock().unwrap();

    while *guard == 0 {
        guard.wait().unwrap();
    }

    *guard -= 1;
}

fn signal(semaphore: &Monitor<u32>) {
    *semaphore.lock().unwrap() += 1
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

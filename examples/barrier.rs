use std::{
    sync::Arc,
    thread::{sleep, spawn},
    time::Duration,
};

use rand::random;
use synchronization_primitives::barrier::Barrier;

const THREADS: u32 = 10;
const ROUNDS: u32 = 3;

fn main() {
    let barrier = Arc::new(Barrier::new(10));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let barrier = barrier.clone();
        let handle = spawn(move || {
            for _ in 0..ROUNDS {
                pre();
                barrier.wait();
                post();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn pre() {
    sleep(Duration::from_millis(random::<u64>() % 5000));

    println!("Before!");
}

fn post() {
    println!("After!");
}

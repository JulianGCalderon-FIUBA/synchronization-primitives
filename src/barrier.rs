use std::sync::{Condvar, Mutex};

pub struct Barrier {
    limit: u32,
    counter: Mutex<(u32, u32)>,
    waiting: Condvar,
}

pub type Monitor<T> = (Mutex<T>, Vec<Condvar>);

impl Barrier {
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            counter: Mutex::new((0, limit)),
            waiting: Condvar::new(),
        }
    }

    pub fn wait(&self) -> bool {
        let mut counter = self.counter.lock().unwrap();
        counter.0 += 1;

        let mut counter = self
            .waiting
            .wait_while(counter, |&mut counter| counter.0 < self.limit)
            .unwrap();

        counter.1 -= 1;
        if counter.1 == 0 {
            counter.0 = 0;
            counter.1 = self.limit;
        }

        if counter.1 == self.limit - 1 {
            self.waiting.notify_all();
            true
        } else {
            false
        }
    }
}

use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    waiting: Condvar,
    limit: Mutex<u32>,
}

impl Semaphore {
    pub fn new(limit: u32) -> Self {
        Self {
            waiting: Condvar::new(),
            limit: Mutex::new(limit),
        }
    }

    pub fn wait(&self) {
        let limit = self.limit.lock().expect("mutex will never be poisoned");
        let mut limit = self
            .waiting
            .wait_while(limit, |&mut limit| limit == 0)
            .expect("mutex will never be poisoned");

        *limit -= 1;
    }

    pub fn signal(&self) {
        *self.limit.lock().expect("mutex will never be poisoned") += 1;
        self.waiting.notify_one();
    }
}

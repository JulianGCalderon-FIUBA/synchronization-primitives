use std::sync::{Condvar, Mutex};

struct BarrierState {
    pub waiting: u32,
    pub left: u32,
}

pub struct Barrier {
    limit: u32,
    state: Mutex<BarrierState>,
    condition: Condvar,
}

pub type Monitor<T> = (Mutex<T>, Vec<Condvar>);

impl Barrier {
    pub fn new(limit: u32) -> Self {
        let state = BarrierState {
            waiting: 0,
            left: limit,
        };

        Self {
            limit,
            state: Mutex::new(state),
            condition: Condvar::new(),
        }
    }

    pub fn wait(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        state.waiting += 1;

        let mut state = self
            .condition
            .wait_while(state, |state| state.waiting < self.limit)
            .unwrap();

        state.left -= 1;
        if state.left == 0 {
            state.waiting = 0;
            state.left = self.limit;
        }

        if state.left == self.limit - 1 {
            self.condition.notify_all();
            true
        } else {
            false
        }
    }
}

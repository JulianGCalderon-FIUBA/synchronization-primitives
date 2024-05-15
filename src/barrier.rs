use std::sync::{Condvar, Mutex};

struct BarrierState {
    pub waiting: u32,
    pub leaving: u32,
}

pub struct Barrier {
    limit: u32,
    state: Mutex<BarrierState>,
    condition: Condvar,
}

impl Barrier {
    pub fn new(limit: u32) -> Self {
        let state = BarrierState {
            waiting: 0,
            leaving: limit,
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

        state.leaving -= 1;

        // if last (first to leave)
        if state.leaving == self.limit - 1 {
            self.condition.notify_all();
            return true;
        }

        // if last to leave
        if state.leaving == 0 {
            state.waiting = 0;
            state.leaving = self.limit;
        }

        return false;
    }
}

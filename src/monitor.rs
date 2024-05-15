use std::{
    ops::{Deref, DerefMut},
    sync::{Condvar, Mutex, MutexGuard, PoisonError},
};

pub struct Monitor<T> {
    data: Mutex<T>,
    cond: Condvar,
}

impl<T> Monitor<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Mutex::new(data),
            cond: Condvar::new(),
        }
    }

    pub fn lock<'m>(&'m self) -> Result<MonitorGuard<'m, T>, PoisonError<MutexGuard<'m, T>>> {
        Ok(MonitorGuard {
            guard: Some(self.data.lock()?),
            cond: &self.cond,
        })
    }
}

pub struct MonitorGuard<'m, T> {
    guard: Option<MutexGuard<'m, T>>,
    cond: &'m Condvar,
}

impl<'m, T> MonitorGuard<'m, T> {
    pub fn wait(&mut self) -> Result<(), PoisonError<MutexGuard<'m, T>>> {
        let guard = self.cond.wait(self.guard.take().unwrap())?;
        self.guard = Some(guard);

        Ok(())
    }
}

impl<'m, T> Deref for MonitorGuard<'m, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.guard.as_ref().unwrap()
    }
}

impl<'m, T> DerefMut for MonitorGuard<'m, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.guard.as_mut().unwrap()
    }
}
use crate::kernel::get_kernel;
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

/// A simple spinlock mutex, unlocked after the inner `Guard` is dropped
pub struct Mutex<T> {
    obj: UnsafeCell<T>,
    lock: AtomicBool,
}

// as long as T is safe to Send, its Mutex can be Send + Sync
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    const MAX_SPINS: u64 = 1_000_000_000;

    pub const fn new(obj: T) -> Self {
        Self {
            obj: UnsafeCell::new(obj),
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&'_ self) -> Guard<'_, T> {
        let mut spins = 0;

        while self
            .lock
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            spins += 1;

            if spins >= Self::MAX_SPINS {
                panic!("Deadlock detected in spinlock mutex");
            }
        }

        Guard::new(self)
    }

    fn unlock(&self) {
        self.lock.store(false, Ordering::SeqCst);
    }
}

pub struct Guard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> Guard<'a, T> {
    fn new(mutex: &'a Mutex<T>) -> Self {
        get_kernel().cpu_me().increment_preempt_counter();
        Self { mutex }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
        get_kernel().cpu_me().decrement_preempt_counter();
    }
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.obj.get() }
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.obj.get() }
    }
}

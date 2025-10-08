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
    pub const fn new(obj: T) -> Self {
        Self {
            obj: UnsafeCell::new(obj),
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        // TODO: now that scheduling is implemented, shared resources (like UART) can't
        // be locked, or they will likely cause a deadlock

        // while self
        //     .lock
        //     .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        //     .is_err()
        // {}

        Guard::new(self)
    }

    fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

pub struct Guard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> Guard<'a, T> {
    fn new(mutex: &'a Mutex<T>) -> Self {
        Self { mutex }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
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

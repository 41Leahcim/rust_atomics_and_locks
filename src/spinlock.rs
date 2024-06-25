use std::{cell::UnsafeCell, hint::spin_loop, ops::{Deref, DerefMut}, sync::atomic::{AtomicBool, Ordering}};

pub struct SpinLock<T>{
    locked: AtomicBool,
    value: UnsafeCell<T>
}

unsafe impl<T: Send> Sync for SpinLock<T>{}

impl<T> SpinLock<T>{
    pub const fn new(value: T) -> Self{
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value)
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn lock(&self) -> Guard<T>{
        while self.locked.swap(true, Ordering::Acquire){
            spin_loop();
        }
        Guard{ lock: self }
    }
}

pub struct Guard<'guard, T>{
    lock: &'guard SpinLock<T>
}

impl<T> Drop for Guard<'_, T>{
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> Deref for Guard<'_, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: The existence of this guard guarantees we've exclusively locked the lock
        unsafe{ &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: The existence of this guard guarantees we've exclusively locked the lock
        unsafe{ &mut *self.lock.value.get() }
    }
}

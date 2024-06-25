use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicU8, Ordering},
};

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T: Send> Sync for Channel<T> {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    /// # Panics
    /// When trying to send more than one message.
    pub fn send(&self, message: T) {
        assert!(
            self.state
                .compare_exchange(EMPTY, WRITING, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok(),
            "Can't send more than one message!"
        );
        unsafe { (*self.message.get()).write(message) };
        self.state.store(READY, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Relaxed) == READY
    }

    /// # Panics
    /// If no message is available yet, or if the message was already consumed.
    ///
    /// Tip: Use `is_ready` to check first.
    pub fn receive(&self) -> T {
        assert!(
            self.state
                .compare_exchange(READY, READING, Ordering::Acquire, Ordering::Relaxed)
                .is_ok(),
            "No message available!"
        );

        // Safety: We've just checked (and reset) the ready flag
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() };
        }
    }
}

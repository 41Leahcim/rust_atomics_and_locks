use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T: Send> Sync for Channel<T> {}

impl<T> Channel<T> {
    const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    fn send(&self, message: T) {
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Ordering::Release);
    }

    fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    /// # Panics
    /// If no message is available yet, or if the message was already consumed.
    ///
    /// Tip: Use `is_ready` to check first.
    fn receive(&self) -> T {
        assert!(
            self.ready.swap(false, Ordering::Acquire),
            "No message available!"
        );

        // Safety: We've just checked (and reset) the ready flag
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() };
        }
    }
}

pub struct Sender<T>(Arc<Channel<T>>);

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        self.0.send(message);
    }
}

pub struct Receiver<T>(Arc<Channel<T>>);

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.0.is_ready()
    }

    /// # Panics
    /// If no message is available yet, or if the message was already consumed.
    ///
    /// Tip: Use `is_ready` to check first.
    pub fn receive(self) -> T {
        self.0.receive()
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel::new());
    (Sender(channel.clone()), Receiver(channel))
}

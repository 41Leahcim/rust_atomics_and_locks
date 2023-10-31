use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn main() {
    loop {
        [
            thread::spawn(|| Y.store(X.load(Ordering::Relaxed), Ordering::Relaxed)),
            thread::spawn(|| X.store(Y.load(Ordering::Relaxed), Ordering::Relaxed)),
        ]
        .into_iter()
        .for_each(|thread| thread.join().unwrap());
        assert_eq!(X.load(Ordering::Relaxed), 0); // Might fail?
        assert_eq!(Y.load(Ordering::Relaxed), 0); // Might fail?
    }
}

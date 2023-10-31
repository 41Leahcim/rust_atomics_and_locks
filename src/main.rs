use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

/// May happen before b
fn a() {
    X.store(10, Ordering::Relaxed); // happens first

    // happens next, may appear to happen first from the other thread
    Y.store(20, Ordering::Relaxed);
}

/// May happen before a
fn b() {
    let y = Y.load(Ordering::Relaxed); // happens first
    let x = X.load(Ordering::Relaxed); // happens next
    println!("{x} {y}"); // when done
}

fn main() {
    for _ in 0..20 {
        let threads = [thread::spawn(a), thread::spawn(b)];
        for thread in threads {
            thread.join().unwrap();
        }
        X.store(0, Ordering::Relaxed);
        Y.store(20, Ordering::Relaxed);
    }
}

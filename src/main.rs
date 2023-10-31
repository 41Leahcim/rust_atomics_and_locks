use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    for _ in 0..100_000 {
        X.store(1, Ordering::Relaxed);
        let t = thread::spawn(f);
        X.store(2, Ordering::Relaxed); // f might load this value
        t.join().unwrap();

        // Won't cause panics in f
        X.store(3, Ordering::Relaxed);
    }
}

fn f() {
    let x = X.load(Ordering::Relaxed);
    assert!(x == 1 || x == 2);
}

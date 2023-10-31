use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Ordering::Relaxed);
    X.fetch_add(10, Ordering::Relaxed);
}

fn a1() {
    X.fetch_add(5, Ordering::Relaxed);
}

fn a2() {
    X.fetch_add(10, Ordering::Relaxed);
}

fn b() {
    let a = X.load(Ordering::Relaxed);
    let b = X.load(Ordering::Relaxed);
    let c = X.load(Ordering::Relaxed);
    let d = X.load(Ordering::Relaxed);
    println!("{a} {b} {c} {d}");
}

fn main() {
    for _ in 0..20 {
        let threads = [
            thread::spawn(a),
            thread::spawn(b),
            thread::spawn(a1),
            thread::spawn(a2),
        ];
        for thread in threads.into_iter().rev() {
            thread.join().unwrap();
        }
    }
}

use core::sync::atomic::{fence, AtomicU16, AtomicU32, AtomicU8, Ordering};
use std::thread;

static A: AtomicU8 = AtomicU8::new(0);
static B: AtomicU16 = AtomicU16::new(0);
static C: AtomicU32 = AtomicU32::new(0);

fn releasing() {
    fence(Ordering::Release);
    A.store(1, Ordering::Relaxed);
    B.store(2, Ordering::Relaxed);
    C.store(3, Ordering::Relaxed);
}

fn acquiring() {
    let a = A.load(Ordering::Relaxed);
    let b = B.load(Ordering::Relaxed);
    let c = C.load(Ordering::Relaxed);
    fence(Ordering::Acquire);
    println!("{a}, {b}, {c}");
}

fn main() {
    thread::scope(|s| {
        s.spawn(releasing);
        s.spawn(acquiring);
    });
}

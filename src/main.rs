use std::{
    hint::black_box,
    ops::Deref,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    thread,
    time::Instant,
};

/// A 64-byte aligned structm change to 128 if needed
#[repr(align(64))]
struct Aligned(AtomicU64);

impl Deref for Aligned {
    type Target = AtomicU64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Performance: 219.901429ms
fn single_loading_thread() {
    static A: AtomicU64 = AtomicU64::new(0);
    black_box(&A);
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Ordering::Relaxed));
    }
    println!("single loading thread: {:?}", start.elapsed());
}

/// Performance: 298.063858ms
fn two_loading_threads() {
    static A: AtomicU64 = AtomicU64::new(0);
    static FINISHED: AtomicBool = AtomicBool::new(false);
    black_box(&A);
    let thread = thread::spawn(|| {
        while !FINISHED.load(Ordering::Relaxed) {
            black_box(A.load(Ordering::Relaxed));
        }
    });
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Ordering::Relaxed));
    }
    println!("Two loading threads: {:?}", start.elapsed());
    FINISHED.store(true, Ordering::Relaxed);
    thread.join().unwrap();
}

/// Performance: 1.585377923s
fn one_storing_thread_one_loading_thread() {
    static A: AtomicU64 = AtomicU64::new(0);
    static FINISHED: AtomicBool = AtomicBool::new(false);
    black_box(&A);
    let thread = thread::spawn(|| {
        while !FINISHED.load(Ordering::Relaxed) {
            A.store(0, Ordering::Relaxed);
        }
    });
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Ordering::Relaxed));
    }
    println!(
        "One storing thread + one loading thread: {:?}",
        start.elapsed()
    );
    FINISHED.store(true, Ordering::Relaxed);
    thread.join().unwrap();
}

/// Performance: 1.107514258s - 2.622151533s
fn three_atomics() {
    static A: [AtomicU64; 3] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
    static FINISHED: AtomicBool = AtomicBool::new(false);
    black_box(&A);
    let thread = thread::spawn(|| {
        while !FINISHED.load(Ordering::Relaxed) {
            A[0].store(0, Ordering::Relaxed);
            A[2].store(0, Ordering::Relaxed);
        }
    });
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A[1].load(Ordering::Relaxed));
    }
    println!("3 atomics: {:?}", start.elapsed());
    FINISHED.store(true, Ordering::Relaxed);
    thread.join().unwrap();
}

/// Separate cache-lines: 260.807534ms
fn separate_cache_lines() {
    static A: [Aligned; 3] = [
        Aligned(AtomicU64::new(0)),
        Aligned(AtomicU64::new(0)),
        Aligned(AtomicU64::new(0)),
    ];
    static FINISHED: AtomicBool = AtomicBool::new(false);
    black_box(&A);
    let thread = thread::spawn(|| {
        while !FINISHED.load(Ordering::Relaxed) {
            A[0].store(0, Ordering::Relaxed);
            A[2].store(0, Ordering::Relaxed);
        }
    });
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A[1].load(Ordering::Relaxed));
    }
    println!("Separate cache lines: {:?}", start.elapsed());
    FINISHED.store(true, Ordering::Relaxed);
    thread.join().unwrap();
}

fn main() {
    single_loading_thread();
    two_loading_threads();
    one_storing_thread_one_loading_thread();
    three_atomics();
    separate_cache_lines();
}

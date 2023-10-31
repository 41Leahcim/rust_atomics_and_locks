use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

/// # Safety
/// Only one thread is accessing DATA at a time, because it's only writte to
/// once before and only read after setting the ready flag.
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        // Nothing else is accessing DATA, because the READY flag hasn't been set yet
        unsafe { DATA = 123 };
        READY.store(true, Ordering::Release); // Everything from before this store ..
    });

    // .. is visible after this loads `true`.
    while !READY.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting...");
    }

    // Nothing is mutating DATA, because READY is set
    println!("{}", unsafe { DATA });
}

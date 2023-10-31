use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    thread,
    time::Duration,
};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Ordering::Relaxed);
        READY.store(true, Ordering::Release); // Everything from before this store ..
    });

    // .. is visible after this loads `true`.
    while !READY.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting...");
    }
    println!("{}", DATA.load(Ordering::Relaxed));
}

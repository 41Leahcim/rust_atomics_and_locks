use core::sync::atomic::{fence, Ordering};
use std::{sync::atomic::AtomicBool, thread, time::Duration};

static mut DATA: [u64; 10] = [0; 10];
static READY: [AtomicBool; 10] = [const{AtomicBool::new(false)}; 10];

const fn some_calculation<T>(value: T) -> T {
    value
}

fn main() {
    for i in 0..READY.len() {
        thread::spawn(move || {
            let data = some_calculation(i as u64);
            unsafe { DATA[i] = data };
            READY[i].store(true, Ordering::Release);
        });
    }
    thread::sleep(Duration::from_millis(500));
    
    fence(Ordering::Acquire);
    for value in unsafe { DATA.iter() }
        .zip(READY.iter().map(|ready| ready.load(Ordering::Relaxed)))
        .filter_map(|(value, is_ready)| is_ready.then_some(*value))
    {
        println!("{value}");
    }
}

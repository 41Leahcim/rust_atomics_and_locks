use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
};

fn increment(a: &AtomicU32) {
    let mut current = a.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        // Compare_exchange could be replaced by compare_exchange_weak for better performance
        match a.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}

fn main() {
    let value = AtomicU32::new(0);
    thread::scope(|s| {
        const MAX: usize = 1_000_000;
        let mut threads = Vec::with_capacity(MAX);
        let mut max_length = 0;
        for _ in 0..MAX {
            threads.push(s.spawn(|| increment(&value)));
            max_length = max_length.max(threads.len());
            for i in (0..threads.len()).rev() {
                if threads[i].is_finished() && threads.remove(i).join().is_ok() {
                    println!("{}", value.load(Ordering::Relaxed));
                }
            }
        }
        println!("{max_length}");
    });
}

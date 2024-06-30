use std::{
    hint::spin_loop,
    sync::atomic::{compiler_fence, AtomicBool, AtomicUsize, Ordering},
    thread,
};

fn main() {
    let locked = AtomicBool::new(false);
    let counter = AtomicUsize::new(0);
    thread::scope(|scope| {
        // Spawn four threads, that each iterate a million times.
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..1_000_000 {
                    // Acquire the lock using the wrong memory order
                    while locked.swap(true, Ordering::Relaxed) {
                        spin_loop();
                    }
                    compiler_fence(Ordering::Acquire);

                    // Non-tatomically increment the counter, while holding the lock.
                    let old = counter.load(Ordering::Relaxed);
                    let new = old + 1;
                    counter.store(new, Ordering::Relaxed);

                    // Release the lock, using the wrong memory ordering
                    compiler_fence(Ordering::Release);
                    locked.store(false, Ordering::Relaxed);
                }
            });
        }
    });

    println!("{}", counter.into_inner());
}

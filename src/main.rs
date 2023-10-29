use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
};

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Ordering::Relaxed);
    loop {
        assert!(id < 1000, "Too many IDs!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

fn main() {
    const MAX: usize = 1_001;
    let mut threads = Vec::with_capacity(MAX);
    for _ in 0..MAX {
        threads.push(thread::spawn(allocate_new_id));
        for i in (0..threads.len()).rev() {
            if threads[i].is_finished() {
                if let Ok(id) = threads.remove(i).join() {
                    println!("{id}");
                }
            }
        }
    }

    while let Some(thread) = threads.pop() {
        if let Ok(id) = thread.join() {
            println!("{id}");
        }
    }
}

use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
};

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .expect("Too many IDs!")
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

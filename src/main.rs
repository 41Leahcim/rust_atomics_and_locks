use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
};

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

fn main() {
    let mut threads = vec![];
    for _ in 0..1002 {
        threads.push(thread::spawn(allocate_new_id));
        for i in (0..threads.len()).rev() {
            if threads[i].is_finished() {
                let thread = threads.remove(i);
                if let Ok(id) = thread.join() {
                    println!("{id}");
                }
            }
        }
    }
}

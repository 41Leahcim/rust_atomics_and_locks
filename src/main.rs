use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let queue = Mutex::new(VecDeque::new());

    // This Condition Variable is needed to notify the thread an item is available
    let not_empty = Condvar::new();

    thread::scope(|s| {
        // Consuming thread
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            // Retrieve an item from the VecDeque
            let item = loop {
                // Return the item if available, otherwise wait for a new item
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };

            // Drop the MutexGuard
            drop(q);

            // Print the item
            dbg!(item);
        });

        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })
}

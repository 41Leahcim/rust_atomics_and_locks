use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

fn process_item<T: Debug>(item: T) {
    println!("{item:?}")
}

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to process all 100 times
        for t in 0..4 {
            s.spawn(move || {
                for i in t * 25..(t + 1) * 25 {
                    // Assuming this takes some time
                    process_item(i);
                    num_done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working... {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

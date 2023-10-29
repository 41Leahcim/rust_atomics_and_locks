use std::{
    fmt::Debug,
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

fn process_item<T: Debug + TryInto<u64> + Copy>(item: T)
where
    <T as TryInto<u64>>::Error: Debug,
{
    thread::sleep(Duration::from_millis(item.try_into().unwrap()));
    println!("{item:?}")
}

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // A background thread to process all 100 times
        for t in 0..4 {
            s.spawn(move || {
                for i in t * 25..(t + 1) * 25 {
                    let start = Instant::now();

                    // Assuming this takes some time
                    process_item(i);

                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            match n {
                0 => println!("Working... nothing done yet."),
                100 => break,
                1..=99 => println!(
                    "Working... {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                ),
                _ => println!("I am doing too much"),
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

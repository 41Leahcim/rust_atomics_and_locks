use std::{sync::Mutex, thread, time::Duration};

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                // Lock the mutex, other threads won't have access to it
                let mut guard = n.lock().unwrap();

                // Add 100 times to the value in it
                for _ in 0..100 {
                    *guard += 1;
                }

                // Wait a second, increasing the total run-time by the number of threads in seconds
                // So 10 seconds
                thread::sleep(Duration::from_secs(1));

                // The mutex will only be unlocked after the current scope (here)
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}

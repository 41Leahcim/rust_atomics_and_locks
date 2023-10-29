use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

fn calculate_x() -> u64 {
    // Randomly generate a value >= 1
    rand::random::<u64>().max(1)
}

fn get_x() -> u64 {
    // Initialize an atomic u64 to 0
    static X: AtomicU64 = AtomicU64::new(0);

    // Load the value
    let mut x = X.load(Ordering::Relaxed);

    // Recalculate the value for x, if it is 0
    if x == 0 {
        x = calculate_x();
        X.store(x, Ordering::Relaxed);
    }

    // Return the value
    x
}

fn main() {
    // Start 2 threads retrieving x
    let t1 = thread::spawn(get_x);
    let t2 = thread::spawn(get_x);

    // Panic if they aren't equal, which happens sometimes
    // This can be solved with std::sync::Once or std::sync::OnceLock
    assert_eq!(t1.join().unwrap(), t2.join().unwrap());

    // Print the value of x
    println!("{}", get_x());
}

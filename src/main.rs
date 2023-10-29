use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

fn generate_random_key() -> u64 {
    rand::random()
}

/// Using Once or OnceCell is better when keys take long to generate
fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        // Compare_exchange_weak shouldn't be used here
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => {
                println!("{:?} generated the key", thread::current().id());
                new_key
            }
            Err(k) => {
                println!(
                    "Another thread than {:?} generated a key after load",
                    thread::current().id()
                );
                k
            }
        }
    } else {
        key
    }
}

fn main() {
    let t1 = thread::spawn(get_key);
    let t2 = thread::spawn(get_key);
    assert_eq!(t1.join().unwrap(), t2.join().unwrap());
}

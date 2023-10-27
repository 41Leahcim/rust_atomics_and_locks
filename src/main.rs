use std::{sync::Arc, thread};

fn main() {
    // Thread safe reference counter
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    let t1 = thread::spawn(move || dbg!(b));
    dbg!(a);
    t1.join().unwrap();
}

use std::{sync::Arc, thread};

fn main() {
    // Thread safe reference counter
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}

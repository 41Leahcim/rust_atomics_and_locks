use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new([1, 2, 3]);

    let t1 = thread::spawn({
        // Create a clone of Arc a with the same name, shadowing the original
        let a = a.clone();

        // move the clone to the new thread
        move || dbg!(a)
    });

    // The original a is still available
    dbg!(a);
    t1.join().unwrap();
}

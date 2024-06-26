use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use rust_atomics_and_locks::arc::Arc;

const VALUE: &str = "hello";

#[test]
fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Create two Arcs sharing an object containing a string
    // and a DetectDrop, to detect when it's dropped.
    let x = Arc::new((VALUE, DetectDrop));
    let y = x.clone();

    // Send x to another thread and use it there.
    let t = thread::spawn(move || assert_eq!(x.0, VALUE));

    // In parallel, y should still be usable
    assert_eq!(y.0, VALUE);

    // Wait for the thread to finish
    t.join().unwrap();

    // One Arc, x, should be dropped by now.
    // We still have y, so the object shouldn't have been dropped yet.
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);

    // Drop the remaining `Arc`
    drop(y);

    // Now that `y` is dropped too,
    // the object should have been dropped.
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
}

#[test]
fn test2() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Create an Arc with 2 weak pointers
    let x = Arc::new((VALUE, DetectDrop));
    let y = Arc::downgrade(&x);
    let z = Arc::downgrade(&x);

    // Send x to another thread and use it there.
    let t = thread::spawn(move || assert_eq!(y.upgrade().unwrap().0, VALUE));

    // In parallel, y should still be usable
    assert_eq!(x.0, VALUE);

    // Wait for the thread to finish
    t.join().unwrap();

    // THe data shouldn't be dropped yet, and the weak pointer should be upgradable.
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);
    assert!(z.upgrade().is_some());

    // Drop the remaining `Arc`
    drop(x);

    // Now the data should be dropped, and the
    // weak pointer should no longer be upgradable.
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
    assert!(z.upgrade().is_none());
}

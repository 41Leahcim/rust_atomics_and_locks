use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let a = AtomicI32::new(100); // a = 100
    let b = a.fetch_add(23, Ordering::Relaxed); // a = 123, b = 100
    let c = a.load(Ordering::Relaxed); // a = 123, c = 123

    assert_eq!(b, 100);
    assert_eq!(c, 123);
}

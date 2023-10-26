// A thread unsafe reference counter
use std::rc::Rc;

fn main() {
    // Create a new reference counter
    let a = Rc::new([1, 2, 3]);

    // Cloning a reference counter, will increase the reference count.
    let b = a.clone();

    // Both reference counters point to the same memory.
    assert_eq!(a.as_ptr(), b.as_ptr());
}

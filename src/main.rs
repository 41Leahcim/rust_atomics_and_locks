use std::{cell::RefCell, fmt::Debug};

fn f<T>(v: &RefCell<Vec<T>>)
where
    i32: TryInto<T>,
    <i32 as TryInto<T>>::Error: Debug,
{
    // Take a mutable reference to more easily add a new element
    v.borrow_mut().push(1.try_into().unwrap());
}

fn main() {
    // Cells and RefCells can only be used within a single thread
    let values = RefCell::new(vec![0_i32]);
    f(&values);
    println!("{:?}", values.take());
}

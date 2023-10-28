use std::{cell::Cell, fmt::Debug};

fn f<T>(v: &Cell<Vec<T>>)
where
    i32: TryInto<T>,
    <i32 as TryInto<T>>::Error: Debug,
{
    // The old vec is returned and replaced with an empty vec
    let mut v2 = v.take();
    v2.push(1.try_into().unwrap());
    v.set(v2);
}

fn main() {
    let values = Cell::new(vec![0_i32]);
    f(&values);
    println!("{:?}", values.take());
}

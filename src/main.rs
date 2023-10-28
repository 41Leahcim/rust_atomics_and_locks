use std::{fmt::Debug, ops::AddAssign};

fn main() {
    let a = 0;
    let mut b = 0;
    f(&a, &mut b);
}

fn x() {}

fn f<T: AddAssign<T> + PartialEq + Copy>(a: &T, b: &mut T)
where
    i32: TryInto<T>,
    <i32 as std::convert::TryInto<T>>::Error: Debug,
{
    // Assign before the value of a
    let before = *a;

    // Subtract 1 from the value of b
    *b += 1.try_into().unwrap();

    // Assign after the value of a
    let after = *a;

    // The value of a can't have changed because of the immutable reference and borrow checker
    if before != after {
        x();
    }
}

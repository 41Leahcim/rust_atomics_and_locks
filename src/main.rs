use std::{cell::Cell, env::args, fmt::Debug, io, ops::Add};

fn x() {
    println!("Value changed");
}

// Copy or clone required for T as Cell returns the actual value inside it, instead of a reference
fn f<T: Copy + Add<T, Output = T> + PartialEq>(a: &Cell<T>, b: &Cell<T>)
where
    i32: TryInto<T>,
    <i32 as TryInto<T>>::Error: Debug,
{
    // Take and store the value of a
    let before = a.get();

    // Increment the value of b
    b.set(b.get() + 1.try_into().unwrap());

    // Take the value of a
    let after = a.get();

    // Call x, if a has changed.
    // This will happen if a reference to the same cell was passed to a and b
    if before != after {
        x();
    }
}

fn main() {
    // Read a usize value from the cli args or screen and store it in a cell
    let a = Cell::<usize>::new(
        args()
            .nth(1)
            .as_deref()
            .map_or_else(
                || {
                    let mut buffer = String::new();
                    eprint!("Enter an integer between {} and {}:", i32::MIN, i32::MAX);
                    io::stdin().read_line(&mut buffer).unwrap();
                    buffer.trim().parse()
                },
                str::parse,
            )
            .unwrap(),
    );

    // Create a new cell if there was a second value passed
    // Otherwise pass a
    if let Some(value) = args().nth(2) {
        let b = Cell::new(value.parse::<usize>().unwrap());
        f(&a, &b);
    } else {
        f(&a, &a);
    }
}

use std::{env::args, io};

fn main() {
    // Read an index
    let index: usize = args()
        .nth(1)
        .as_deref()
        .map_or_else(
            || {
                let mut buffer = String::new();
                eprint!("Enter a number between 0 and 2 inclusive: ");
                io::stdin().read_line(&mut buffer).unwrap();
                buffer.trim().parse()
            },
            str::parse,
        )
        .unwrap();

    // Make sure the index is between 0 and 2 inclusive, as the index won't be checked
    let index = index.clamp(0, 2);

    // Create the data
    let a = [123, 456, 789];

    // Get the requested element
    let b = unsafe { a.get_unchecked(index) };

    // Print the requested element
    println!("{b}");
}

use std::thread;

// A static variable isn't owned by any thread.
// It is owned by the entire program.
static X: [i32; 3] = [1, 2, 3];

fn main() {
    // This compiles, as it only borrows a static variable.
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}

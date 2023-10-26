use std::thread;

fn main() {
    // Leaking memory, means it will live forever.
    // The box::leak returns a static mutable reference to that memory.
    // Mutable references can implicitly be turned into shared references.
    // Leaked memory will never be deallocated which can cause the program to use up all memory.
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    // The memory living forever allows us to pass it to threads.
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
}

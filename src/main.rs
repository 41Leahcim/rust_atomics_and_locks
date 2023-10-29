use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

fn some_work() {}

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work.
    let background_thread = thread::spawn(|| {
        while !STOP.load(Ordering::Relaxed) {
            some_work();
        }
    });

    // Use the main thread to listen for user input.
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Commands: help, stop"),
            "stop" => break,
            cmd => println!("Unknown command: {cmd:?}\nCommands: help, stop"),
        }
    }

    // Inform the background thread, it needs to stop!
    STOP.store(true, Ordering::Relaxed);

    // Wait until the background thread finishes
    background_thread.join().unwrap();
}
